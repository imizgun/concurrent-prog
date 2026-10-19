use std::{fs::{self, OpenOptions}, io::{self, ErrorKind, Write}, thread::sleep, time::Duration};

pub const BUFFER: &'static str = "buffer";
pub const LOCKFILE: &'static str = "lockfile";
pub const CLIENT: &'static str = "client";

pub fn write_to_file_clean(file_path: &str, contents: &str) -> io::Result<()> {
    fs::write(file_path, contents)?;
    Ok(())
}

pub fn create_atomic(path: &str) -> io::Result<()> {
    loop {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
        {
            Ok(mut f) => {
                writeln!(f, "{}", std::process::id())?;
                return Ok(());
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                println!("Serwer zajęty, proszę czekać");
                sleep(Duration::from_secs(1));
            }
            Err(e) => return Err(e),
        }
    }
}