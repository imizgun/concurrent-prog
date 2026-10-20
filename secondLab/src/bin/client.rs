use std::{fs::{self}, io::{self}, process
};

use secondLab::{BUFFER, CLIENT, LOCKFILE, assemble_message, create_atomic, remove_on_exit, wait_for_response, write_to_file_clean};

fn main() -> io::Result<()> {
    let id = process::id();
    let client_file = format!("{}_{}", CLIENT, id);

    write_to_file_clean(&client_file, "")?;
    remove_on_exit(client_file.clone());

    println!("Client {}, write messages: ", id);

    let mut input = String::new();
    input.push_str(&format!("{}\n", client_file));

    assemble_message(&mut input)?;

    println!("\nServer response: ");

    create_atomic(LOCKFILE)?;

    fs::write(BUFFER, &input)?;

    let res = wait_for_response(&client_file)?;

    println!("{}", res);

    fs::remove_file(client_file)?;

    Ok(())
}
