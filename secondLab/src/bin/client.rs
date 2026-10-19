use std::{fs, io::{self, ErrorKind}, process, str::Lines, thread, time::Duration};

use secondLab::{BUFFER, CLIENT, LOCKFILE, create_atomic, write_to_file_clean};

fn main() -> io::Result<()> {
    let id = process::id();
    let client_file = format!("{}_{}", CLIENT, id);

    write_to_file_clean(&client_file, "")?;

    println!("Client {}, write messages: ", id);

    let mut input = String::new();

    _ = io::stdin()
        .read_line(&mut input)
        .expect("Error when reading input");

    // client message
    while input != "end\n" {
        create_atomic(LOCKFILE)?;

        fs::write(BUFFER, &input)?;

        input.clear();

        _ = io::stdin()
            .read_line(&mut input)
            .expect("Error when reading input");
    }

    fs::write(BUFFER, &input)?;

    let mut last_line = String::new();

    // waiting for the response
    while last_line != "end\n" {
        let str = fs::read_to_string(&client_file);

        match str {
            Ok(s) => last_line = s.lines().last().unwrap_or_default().to_string(),
            Err(_) => {
                thread::sleep(Duration::from_secs(1));
            }
        }
        
        
    }

    fs::remove_file(client_file)?;

    Ok(())
}
