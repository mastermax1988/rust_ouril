mod board;
use crate::board::Board;

fn main() {
    let mut b = Board::new();
    b.print();
    b.turn(5);
    b.print();
    b.turn(3);
    b.print();
}
