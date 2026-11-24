use std::{error::Error, io, path::Path, process};

use forth_lab::{
    DB_FIFO, FifoGuard, decode_response, encode_request, open_queue_for_reading, read_frame,
    send_frame,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Client {}", process::id());
    let mut input: String = String::new();

    _ = io::stdin()
        .read_line(&mut input)
        .expect("Error while reading input");

    let id: i32 = input.trim().parse()?;

    let reply = FifoGuard::create(format!("/tmp/db_client_{}", process::id()))?;
    let reply_path = reply.path().to_str().expect("path is ASCII");

    let mut queue = open_queue_for_reading(reply.path())?;

    send_frame(Path::new(DB_FIFO), &encode_request(id, reply_path))?;

    let answer = decode_response(&read_frame(&mut queue)?)?;
    println!("{id}: {answer}");

    Ok(())
}
