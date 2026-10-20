use std::{fs, io};

use second_lab::{
    BUFFER, CLIENT, LOCKFILE, assemble_message, wait_for_response, write_to_file_clean,
};

fn check_client_connection(client_file: &str) -> io::Result<bool> {
    let connected = fs::exists(client_file)?;

    if !connected {
        println!("Client '{}' disconnected", client_file);
    }
    return Ok(connected);
}

fn release_lock() -> io::Result<()> {
    write_to_file_clean(BUFFER, "")?;
    fs::remove_file(LOCKFILE)?;
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Server runs\n");

    if !fs::exists(LOCKFILE)? {
        write_to_file_clean(BUFFER, "")?;
    }

    loop {
        let buffer_content = wait_for_response(BUFFER)?;
        let mut lines = buffer_content.lines();

        if let Some(first_line) = lines.next() {
            if !first_line.starts_with(CLIENT) {
                println!("No client file was provided");
                release_lock()?;
                continue;
            }

            let client_name = first_line.to_string();

            println!(
                "Client '{}' says:\n{}",
                client_name,
                lines.map(|x| format!("{}\n", x)).collect::<String>()
            );

            println!();

            let mut input = String::new();

            assemble_message(&mut input)?;
            println!();

            if check_client_connection(&client_name)? {
                write_to_file_clean(&client_name, &input)?;
            }
            release_lock()?;
        }
    }
}
