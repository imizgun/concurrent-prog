use first_lab::{remove_if_exists, wait_and_take, write_atomic, DATA_FILE, RESULT_FILE};

fn main() -> std::io::Result<()> {
    remove_if_exists(DATA_FILE)?;
    remove_if_exists(RESULT_FILE)?;

    println!("Server started");

    loop {
        let num_str = wait_and_take(DATA_FILE)?;

        let response = match num_str.trim().parse::<i64>() {
            Ok(x) => match math_func(x) {
                Some(res) => {
                    println!("f({x}) = {res}");
                    res.to_string()
                }
                None => {
                    eprintln!("Overflow for x = {x}");
                    "ERROR: overflow".to_string()
                }
            },
            Err(_) => {
                eprintln!("Invalid data: {num_str:?}");
                "ERROR: not a number".to_string()
            }
        };

        write_atomic(RESULT_FILE, &response)?;
    }
}

fn math_func(x: i64) -> Option<i64> {
    let x2 = x.checked_mul(x)?;
    let x3 = x2.checked_mul(x)?;
    x3.checked_add(x2)?.checked_sub(x.checked_mul(2)?)
}
