use std::{
    io::{self, ErrorKind}, path::Path, thread, time::Duration,
};

use forth_lab::{
    DB_FIFO, FifoGuard, Row, decode_request, encode_response, install_server_handlers, lookup,
    open_queue_for_reading, read_frame, send_frame, stop_requested,
};

fn main() -> Result<(), io::Error> {
    println!("Server started");
    
    let db = vec![
        Row::new(1, "Aboba"),
        Row::new(2, "Jack"),
        Row::new(3, "John"),
        Row::new(4, "Alex"),
        Row::new(5, "Elie"),
        Row::new(6, "Olivia"),
    ];

    let guard = FifoGuard::create(Path::new(DB_FIFO))?;
    let mut db_fifo = open_queue_for_reading(guard.path())?;

    install_server_handlers()?;

    while !stop_requested() {
        let body = match read_frame(&mut db_fifo) {
            Ok(b) => b,
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => {
                eprintln!("bad frame: {e}");
                continue;
            }
        };

        let (id, path) = match decode_request(&body) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("bad request: {e}");
                continue;
            }
        };
        
        println!("request id={id} from {path}, processing...");
        thread::sleep(Duration::from_secs(5));

        let name = lookup(&db, id);

        let response = encode_response(name);

        if let Err(e) = send_frame(Path::new(path.as_str()), &response) {
            eprintln!("cannot reply to {path}: {e}");
        }
    }

    Ok(())
}
