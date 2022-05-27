extern crate core;
mod board;

use crate::board::Board;

use text_io::read;

fn main() {
    let mut b = Board::new();
    b.print();
    while true {
        let line: String = read!("{}\n");
        let nr: usize;
        if let Ok(x) = line.parse() {
            nr = x
        } else {
            continue;
        };
        if nr < 6 {
            b.turn(nr);
            b.print();
        }
    }
}
