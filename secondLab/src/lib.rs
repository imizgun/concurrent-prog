use std::{fs::{self, OpenOptions}, io::{self, ErrorKind, Write, stdin}, thread::{self, sleep}, time::Duration};

pub const BUFFER: &'static str = "buffer";
pub const LOCKFILE: &'static str = "lockfile";
pub const CLIENT: &'static str = "client";
pub const ESC: u8 = 0x18;

pub fn write_to_file_clean(file_path: &str, contents: &str) -> io::Result<()> {
    fs::write(file_path, contents)?;
    Ok(())
}

pub fn wait_for_response(path: &str) -> io::Result<String> {
    loop {
        let content = fs::read(path)?;

        if let Some(n) = content.iter().position(|b| *b == ESC) {
            return Ok(String::from_utf8_lossy(&content[..n]).into_owned());
        }
        thread::sleep(Duration::from_millis(500));
    }
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


pub fn assemble_message(message: &mut String) -> io::Result<()> {
    for line in stdin().lines() {
        let str = line?;

        if str == "end" {
            break;
        } else {
            message.push_str(&str);
            message.push('\n');
        }
    }
    message.push(ESC as char);
    Ok(())
}

pub fn remove_on_exit(path: String) {
    ctrlc::set_handler(move || {
        let _ = fs::remove_file(&path);
        std::process::exit(130);
    })
    .expect("Nie udało się ustawić obsługi sygnału");
}
