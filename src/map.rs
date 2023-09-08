use rand::Rng;

pub struct Map {
    map_size: u32,
    number_of_mines: u32,
    mines: Vec<(u32, u32)>,
    real_map: Vec<Vec<i32>>,
}

impl Map {
    pub fn new(level: super::LEVEL) -> Self {
        let map_size;
        let number_of_mines;

        match level {
            super::LEVEL::EASY => {
                map_size = 9;
                number_of_mines = 10;
            },
            super::LEVEL::MEDIUM => {
                map_size = 16;
                number_of_mines = 50;
            },
            super::LEVEL::HARD => {
                map_size = 24;
                number_of_mines = 150;
            },
        }

        let mines: Vec<(u32, u32)> = Vec::<(u32, u32)>::with_capacity(number_of_mines as usize);
        let real_map;

        real_map = vec![vec![0; map_size as usize]; map_size as usize];

        let mut new_map = Self {
            map_size,
            number_of_mines,
            mines,
            real_map
        };

        new_map.place_bombs();

        new_map

    }

    fn place_bombs(&mut self) {
        let mut rng = rand::thread_rng();

        let mut i = 0;
        while i != self.number_of_mines {
            let x = rng.gen_range(0..self.map_size - 1);
            let y = rng.gen_range(0..self.map_size - 1);

            if self.real_map[x as usize][y as usize] != -1 {
                self.real_map[x as usize][y as usize] = -1;
                self.mines.push((x, y));
                i += 1;
            }
        }

        for i in 0..self.map_size - 1 {
            for j in 0..self.map_size - 1 {
                self.find_number(i, j);
            }
        }
    }

    fn find_number(&mut self, x: u32, y: u32) {
        if self.real_map[x as usize][y as usize] == -1 {
            return;
        }

        let mut count = 0;

        for i in x as i32 - 1..x as i32 + 1 {
            for j in y as i32 - 1..y as i32 + 1 {
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

    pub fn print(&self) {
        for i in 0..self.map_size - 1 {
            for j in 0..self.map_size - 1 {
                let c: char = match self.real_map[i as usize][j as usize] {
                    -1 => '*',
                    0 => '_',  //  ⚑
                    x => x.to_string().parse().unwrap(),
                };

                print!("{} ", c);
            }
            println!("");
        }

        for (x, y) in &self.mines {
            println!("{} {}", x, y)
        }
    }
}
