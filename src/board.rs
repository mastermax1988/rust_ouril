#[derive(Copy, Clone)]
pub struct Board {
    field: [u8; 12],
    current_player: usize,
    score: [u8; 2],
    game_running: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [4; 12],
            current_player: 0,
            score: [0; 2],
            game_running: true,
        }
    }

    pub fn turn(&mut self, mut start: usize) -> bool {
        if start > 11 {
            return false;
        }
        if !self.check_move_legal(start) {
            return false;
        }
        let mut index = match self.current_player {
            0 => start,
            _ => start + 6,
        };

        start = index;
        if self.field[index] == 0 {
            return false;
        }
        let mut count: usize = self.field[index] as usize;

        self.field[index] = 0;
        while count > 0 {
            index += 1;
            index %= 12;
            if index == start {
                continue;
            }
            self.field[index] += 1;
            count -= 1;
        }
        self.score(index);

        self.set_next_player();

        true
    }

    fn set_next_player(&mut self) {
        if self.score[0] > 12 || self.score[1] > 12 {
            self.game_running = false;
            return;
        }
        self.current_player += 1;
        self.current_player %= 2;
        let start = match self.current_player {
            0 => 6,
            _ => 0,
        };
        for i in 0..6 {
            if self.field[i + start] > 0 {
                return;
            }
        }
        self.game_running = false;
        for i in 0..11 {
            self.score[match i {
                0..=5 => 0,
                _ => 1,
            }] += self.field[i];
        }
    }

    fn check_move_legal(&self, start: usize) -> bool {
        if self.field[start] == 0 {
            return false;
        }
        let enemy_index = match self.current_player {
            0 => 6,
            _ => 0,
        };

        for i in 0..6 {
            if self.field[i + enemy_index] > 0 {
                return true; //enemy field not empty
            }
        }
        if (self.field[start] + start as u8)
            > match self.current_player {
                0 => 5,
                _ => 0,
            }
        {
            return true; //turn into enemy field
        }
        if self.valid_move_exists() {
            return false; //valid move exists, but is not executed
        }

        true
    }

    fn valid_move_exists(&self) -> bool {
        for i in 0..6 {
            if self.field[i + self.current_player * 6] > (6 - i as u8) {
                return true; //valid move exists
            }
        }
        false
    }

    fn score(&mut self, end: usize) {
        if (self.current_player == 0 && end < 6) || (self.current_player == 1 && end > 5) {
            return;
        }
        let backup_field = self.field.clone();
        let backup_score = self.score.clone();
        let min = match self.current_player {
            0 => 6,
            _ => 0,
        };
        let mut index = end;
        while (self.field[index] == 2 || self.field[index] == 3) && index >= min {
            self.score[self.current_player] += self.field[index];
            self.field[index] = 0;
            if index > 0 {
                index -= 1;
            } else {
                break;
            }
        }
        let start = match self.current_player {
            0 => 6,
            _ => 0,
        };
        for i in 0..6 {
            if self.field[i + start] > 0 {
                return;
            }
        }
        println!("gran slam!");
        self.score = backup_score;
        self.field = backup_field;
    }

    pub fn print(&self) {
        println!("{} : {}", self.score[0], self.score[1]);
        for i in 0..6 {
            print!("{} | ", self.field[11 - i]);
        }
        println!();
        for i in 0..6 {
            print!("{} | ", self.field[i]);
        }
        println!();
        if !self.game_running {
            println!("Game over");
            return;
        }
        println!("Next player: {}", self.current_player);
        println!()
    }
}

#[cfg(test)]
mod test {
    use crate::Board;

    #[test]
    fn valid_move() {
        let mut b = Board::new();
        b.field = [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0];
        assert!(!b.check_move_legal(0));
        assert!(b.check_move_legal(5));
        b.turn(5);
        assert_eq!(b.field[6], 2);
        b.field = [0, 0, 0, 0, 0, 6, 1, 2, 0, 1, 1, 1];
        b.current_player = 0;
        assert!(b.check_move_legal(5));
        b.turn(5);
        assert_eq!(b.score[0], 6);
        b.field = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        b.current_player = 0;
        assert!(b.check_move_legal(0));
        b.turn(0);
    }
    #[test]
    fn next_player() {
        let mut b = Board::new();
        b.field = [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0];
        b.set_next_player();
        assert_eq!(b.current_player, 1);
        b.set_next_player();
        assert!(!b.game_running);
        assert_eq!(b.score[1], 1);
    }
}
