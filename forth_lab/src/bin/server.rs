use forth_lab::{DB_FIFO, Row};
use nix::libc::mkfifo;

fn main() -> ! {
    let db = vec![
        Row::new(1, "Aboba"),
        Row::new(2, "Jack"),
        Row::new(3, "John"),
        Row::new(4, "Alex"),
        Row::new(5, "Elie"),
        Row::new(6, "Olivia"),
    ];
    
    loop {
    }
}