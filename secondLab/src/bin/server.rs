use std::io;

use secondLab::{BUFFER, write_to_file_clean};

fn main() -> io::Result<()> {
    write_to_file_clean(BUFFER, "")?;

    loop {
        
    }
    
    Ok(())
}