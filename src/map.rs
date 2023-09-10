use druid::{Data, Lens};
use rand::Rng;

#[derive(Clone, Data, Lens)]
pub struct Map {
    pub map_size: u32,
    number_of_mines: u32,
    #[data(eq)]
    mines: Vec<(u32, u32)>,
    #[data(eq)]
    real_map: Vec<Vec<i32>>,
    #[data(eq)]
    pub playing_map: Vec<Vec<i32>>,
    number_of_flags: u32,
    number_of_revealed: u32,
    pub is_lost: bool,
}

impl Map {
    pub fn new(level: super::LEVEL) -> Self {
        let map_size;
        let number_of_mines;

        match level {
            super::LEVEL::EASY => {
                map_size = 9;
                number_of_mines = 10;
            }
            super::LEVEL::MEDIUM => {
                map_size = 16;
                number_of_mines = 50;
            }
            super::LEVEL::HARD => {
                map_size = 24;
                number_of_mines = 150;
            }
        }

        let mines: Vec<(u32, u32)> = Vec::<(u32, u32)>::with_capacity(number_of_mines as usize);
        let real_map = vec![vec![0; map_size as usize]; map_size as usize];
        let playing_map = vec![vec![0; map_size as usize]; map_size as usize];
        let number_of_flags = 0;
        let number_of_revealed = 0;

        let mut new_map = Self {
            map_size,
            number_of_mines,
            mines,
            real_map,
            playing_map,
            number_of_flags,
            number_of_revealed,
            is_lost: false,
        };

        new_map.place_bombs();

        new_map
    }

    fn place_bombs(&mut self) {
        let mut rng = rand::thread_rng();

        let mut i = 0;
        while i != self.number_of_mines {
            let x = rng.gen_range(0..self.map_size);
            let y = rng.gen_range(0..self.map_size);

            if self.real_map[x as usize][y as usize] != -1 {
                self.real_map[x as usize][y as usize] = -1;
                self.mines.push((x, y));
                i += 1;
            }
        }

        for i in 0..self.map_size {
            for j in 0..self.map_size {
                self.find_number(i, j);
            }
        }
    }

    fn find_number(&mut self, x: u32, y: u32) {
        if self.real_map[x as usize][y as usize] == -1 {
            return;
        }

        let mut count = 0;

        for i in x as i32 - 1..=x as i32 + 1 {
            for j in y as i32 - 1..=y as i32 + 1 {
                if i < 0 || j < 0 || i as u32 >= self.map_size || j as u32 >= self.map_size {
                    continue;
                }

                if self.real_map[i as usize][j as usize] == -1 {
                    count += 1;
                }
            }
        }

        self.real_map[x as usize][y as usize] = count;
    }

    pub fn print(&self, x: u32, y: u32) -> String {
        let c: char = match self.playing_map[x as usize][y as usize] {
            0 => '-',
            1 => '⚑',
            2 => match self.real_map[x as usize][y as usize]
                .to_string()
                .parse()
                .unwrap()
            {
                -1 => '*',
                0 => ' ',
                x => x.to_string().parse().unwrap(),
            },
            _ => ' ',
        };

        c.to_string()
    }

    pub fn reveal(&mut self, x: u32, y: u32) -> bool {
        if self.real_map[x as usize][y as usize] == -1 {
            return false;
        }

        if self.playing_map[x as usize][y as usize] == 2 {
            return true;
        }

        self.playing_map[x as usize][y as usize] = 2;
        self.number_of_revealed += 1;

        if self.real_map[x as usize][y as usize] == 0 {
            if self.real_map[x as usize][y as usize] == 0 {
                for i in x as i32 - 1..=x as i32 + 1 {
                    for j in y as i32 - 1..=y as i32 + 1 {
                        if i < 0 || j < 0 || i as u32 >= self.map_size || j as u32 >= self.map_size
                        {
                            continue;
                        }

                        self.reveal(i as u32, j as u32);
                    }
                }
            }
        }

        true
    }

    pub fn flag(&mut self, x: u32, y: u32) {
        self.playing_map[x as usize][y as usize] = 1;
        self.number_of_flags += 1;
    }

    pub fn game_won(&self) -> bool {
        if self.number_of_revealed + self.number_of_mines == self.map_size * self.map_size {
            return true;
        }

        false
    }

    pub fn game_lost(&mut self) {
        for i in 0..self.map_size {
            for j in 0..self.map_size {
                if self.real_map[i as usize][j as usize] == -1 {
                    self.playing_map[i as usize][j as usize] = 2;
                }
            }
        }

        self.is_lost = true;
    }
}
