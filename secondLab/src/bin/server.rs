use std::io;

use secondLab::{BUFFER, CLIENT, wait_for_response, write_to_file_clean};

fn main() -> io::Result<()> {
    write_to_file_clean(BUFFER, "")?;
    let mut client_name = String::new();
    
    loop {
        let buffer_content = wait_for_response(BUFFER)?;
        let mut lines = buffer_content.lines();

        if let Some(first_line) = lines.next() {
            if !first_line.starts_with(CLIENT) {
                println!("No client file was provided");
                continue;
            }

            client_name = first_line.to_string();

            let messages = lines.map(|x| x.to_string()).collect::<Vec<String>>();
            println!("{:?}", messages);

            println!()

            
        }
    }
}