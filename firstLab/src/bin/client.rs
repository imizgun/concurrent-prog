use std::io::{self, Write};

use first_lab::{remove_if_exists, wait_and_take, write_atomic, DATA_FILE, RESULT_FILE};

fn main() -> io::Result<()> {
    print!("Enter an integer: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let num: i64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("That's not an integer: {:?}", input.trim());
            std::process::exit(1);
        }
    };

    remove_if_exists(RESULT_FILE)?;

    write_atomic(DATA_FILE, &num.to_string())?;

    let result = wait_and_take(RESULT_FILE)?;
    println!("Result: {}", result.trim());

    Ok(())
}
