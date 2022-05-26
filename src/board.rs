pub struct Board {
    field: [u8; 12],
    current_player: u8,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [4; 12],
            current_player: 1,
        }
    }

    pub fn turn(&mut self, mut start: usize) -> bool {
        if start > 11 {
            return false;
        }
        let mut index = match self.current_player {
            1 => start,
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

        self.current_player += 1;
        self.current_player %= 2;

        true
    }

    pub fn print(&self) {
        for i in 0..6 {
            print!("{} | ", self.field[11 - i]);
        }
        println!();
        for i in 0..6 {
            print!("{} | ", self.field[i]);
        }
        println!();
        println!()
    }
}
