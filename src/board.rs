#[derive(Copy, Clone)]
pub struct Board {
    field: [u8; 12],
    current_player: usize,
    score: [u8; 2],
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [4; 12],
            current_player: 0,
            score: [0; 2],
        }
    }

    pub fn turn(&mut self, mut start: usize) -> bool {
        if start > 11 {
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
        self.current_player += 1;
        self.current_player %= 2;
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
        println!("Next player: {}", self.current_player);
        println!()
    }
}
