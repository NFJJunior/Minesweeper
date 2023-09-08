use rand::Rng;

pub struct Map {
    map_size: u32,
    number_of_mines: u32,
    mines: Vec<(i32, i32)>,
    real_map: Vec<Vec<i32>>,
}

impl Map {
    pub fn new(level: super::LEVEL) -> Self {
        let map_size;
        let number_of_mines;
        let mines: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
        let real_map;

        match level {
            super::LEVEL::EASY => {
                map_size = 10;
                number_of_mines = 10;
            },
            super::LEVEL::MEDIUM => {
                map_size = 20;
                number_of_mines = 20;
            },
            super::LEVEL::HARD => {
                map_size = 30;
                number_of_mines = 30;
            },
        }

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
            let x = rng.gen_range(0..(self.map_size - 1));
            let y = rng.gen_range(0..(self.map_size - 1));

            if self.real_map[x as usize][y as usize] != -1 {
                self.real_map[x as usize][y as usize] = -1;
                i += 1;
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.number_of_mines - 1 {
            for j in 0..self.number_of_mines - 1 {
                let c: char = match self.real_map[i as usize][j as usize] {
                    -1 => '*',
                    0 => '⚑',
                    x => x.to_string().parse().unwrap(),
                };

                print!("{} ", c);
            }
            println!("");
        }
    }
}
