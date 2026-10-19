use std::{fs, io, process};

use secondLab::{BUFFER, CLIENT, LOCKFILE, create_atomic, write_to_file_clean};

fn main() -> io::Result<()> {
    let id = process::id(); 
    let client_file = format!("{}_{}", CLIENT, id);

    write_to_file_clean(&client_file, "")?;

    println!("Client {}, write messages: ", id);

    let mut input = String::new();

    _ = io::stdin().read_line(&mut input)
        .expect("Error when reading input");

    // client message
    while input != "end\n" {
        create_atomic(LOCKFILE)?;

        fs::write(BUFFER, &input)?;
        
        input.clear();
        
        _ = io::stdin().read_line(&mut input)
            .expect("Error when reading input");
    }

    fs::write(BUFFER, &input)?;

    // waiting for the response 
    loop {
        
    }
    
    Ok(())
}