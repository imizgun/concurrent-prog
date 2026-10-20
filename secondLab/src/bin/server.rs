use std::{fs, io};

use secondLab::{BUFFER, CLIENT, LOCKFILE, assemble_message, remove_on_exit, wait_for_response, write_to_file_clean};

fn main() -> io::Result<()> {
    println!("Server runs\n");
    remove_on_exit(LOCKFILE.to_string());
    write_to_file_clean(BUFFER, "")?;
    
    loop {
        let buffer_content = wait_for_response(BUFFER)?;
        let mut lines = buffer_content.lines();

        if let Some(first_line) = lines.next() {
            if !first_line.starts_with(CLIENT) {
                println!("No client file was provided");
                continue;
            }

            let client_name = first_line.to_string();

            println!("Clinet {} says:\n{}", client_name, lines.map(|x| format!("{}\n", x)).collect::<String>());

            println!();

            let mut input = String::new();

            assemble_message(&mut input)?;

            write_to_file_clean(&client_name, &input)?;
            write_to_file_clean(BUFFER, "")?;

            fs::remove_file(LOCKFILE)?;
        }
    }
}