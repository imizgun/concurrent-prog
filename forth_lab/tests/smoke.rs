use forth_lab::*;
use std::path::PathBuf;

#[test]
fn roundtrip_through_fifo() {
    let path = PathBuf::from("/tmp/forth_lab_smoke_fifo");
    let _ = std::fs::remove_file(&path);
    let guard = FifoGuard::create(path.clone()).unwrap();
    let mut q = open_queue_for_reading(guard.path()).unwrap();

    // dwa zapytania wpisane przed jakimkolwiek odczytem — tak jak w UWAGA 2
    send_frame(&path, &encode_request(3, "/tmp/cli_a")).unwrap();
    send_frame(&path, &encode_request(99, "/tmp/cli_b")).unwrap();

    let (id1, p1) = decode_request(&read_frame(&mut q).unwrap()).unwrap();
    let (id2, p2) = decode_request(&read_frame(&mut q).unwrap()).unwrap();
    assert_eq!((id1, p1.as_str()), (3, "/tmp/cli_a"));
    assert_eq!((id2, p2.as_str()), (99, "/tmp/cli_b"));

    let db = vec![Row::new(1, "Aboba"), Row::new(3, "John")];
    assert_eq!(lookup(&db, 3), "John");
    assert_eq!(lookup(&db, 99), NOT_FOUND);

    send_frame(&path, &encode_response(lookup(&db, id2))).unwrap();
    assert_eq!(
        decode_response(&read_frame(&mut q).unwrap()).unwrap(),
        NOT_FOUND
    );
}

#[test]
fn read_blocks_instead_of_eof() {
    use std::io::Write;
    let path = PathBuf::from("/tmp/forth_lab_smoke_eof");
    let _ = std::fs::remove_file(&path);
    let guard = FifoGuard::create(path.clone()).unwrap();
    let mut q = open_queue_for_reading(guard.path()).unwrap();

    // klient przychodzi i odchodzi: bez O_RDWR read() dostałby tu EOF
    {
        let mut w = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
        w.write_all(&encode_request(1, "/tmp/x")).unwrap();
    }
    assert!(read_frame(&mut q).is_ok());

    // kolejka pusta, wszyscy pisarze odeszli -> read musi zasnąć, nie zwrócić EOF
    let handle = std::thread::spawn(move || read_frame(&mut q).map(|_| ()));
    std::thread::sleep(std::time::Duration::from_millis(300));
    assert!(!handle.is_finished(), "read() zwrócił EOF zamiast zablokować się");
    send_frame(&path, &encode_response("x")).unwrap();
    handle.join().unwrap().unwrap();
}
