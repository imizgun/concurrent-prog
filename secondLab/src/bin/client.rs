use std::{
    fs::{self},
    io::{self, stdin},
    process
};

use secondLab::{BUFFER, CLIENT, ESC, LOCKFILE, create_atomic, wait_for_response, write_to_file_clean};

fn main() -> io::Result<()> {
    let id = process::id();
    let client_file = format!("{}_{}", CLIENT, id);

    write_to_file_clean(&client_file, "")?;

    println!("Client {}, write messages: ", id);

    let mut input = String::new();
    input.push_str(&client_file);

    for line in stdin().lines() {
        let str = line?;

        if str == "end" {
            break;
        } else {
            input.push_str(&str);
            input.push('\n');
        }
    }
    input.push(ESC as char);

    create_atomic(LOCKFILE)?;

    fs::write(BUFFER, &input)?;

    let res = wait_for_response(BUFFER)?;

    println!("{}", res);

    fs::remove_file(client_file)?;

    Ok(())
}
