use druid::{Data, Lens};
use rand::Rng;

//  The class used for implementing game logic
#[derive(Clone, Data, Lens)]
pub struct Map {
    pub map_size: u32,  //  Size of the map
    number_of_mines: u32,  //  Number of mines
    #[data(eq)]
    mines: Vec<(u32, u32)>,  //  A vector with the position of the mines
    #[data(eq)]
    real_map: Vec<Vec<i32>>,  //  The real map generated
    #[data(eq)]
    pub playing_map: Vec<Vec<i32>>,  //  What the player will see during the game
    number_of_flags: u32,
    number_of_revealed: u32,
    pub is_ended: bool,  //  Flag that tells us if the game has ended
}

impl Map {
    //  Constructor - parameter is a level type
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

        //  Initialize all fields with default values, than randomly place the mines througth the map
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
            is_ended: false,
        };

        new_map.place_mines();

        new_map
    }

    //  Function that randomly places the mines through the map
    fn place_mines(&mut self) {
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

    //  For a specific cell, find the number of neighboring cells with mines
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

    //  Print a cell in the mode the player will see during the game
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

    //  Reveal a cell. If the cell has a mine return false
    pub fn reveal(&mut self, x: u32, y: u32) -> bool {
        if self.real_map[x as usize][y as usize] == -1 {
            return false;
        }

        if self.playing_map[x as usize][y as usize] == 2 {
            return true;
        }

        self.playing_map[x as usize][y as usize] = 2;
        self.number_of_revealed += 1;

        //  If we found a cell with 0 neighborings cells with mines, reveal all others cells with 0 mines near by we find
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

    //  Flag a cell
    pub fn flag(&mut self, x: u32, y: u32) {
        self.playing_map[x as usize][y as usize] = 1;
        self.number_of_flags += 1;
    }

    //  Function that calculates if the game is won
    pub fn game_won(&mut self) -> bool {
        if self.number_of_revealed + self.number_of_mines == self.map_size * self.map_size {
            self.is_ended = true;

            return true;
        }

        false
    }

    //  After the game has been lost, reveale all the mines
    pub fn game_lost(&mut self) {
        for i in 0..self.map_size {
            for j in 0..self.map_size {
                if self.real_map[i as usize][j as usize] == -1 {
                    self.playing_map[i as usize][j as usize] = 2;
                }
            }
        }

        self.is_ended = true;
    }
}
