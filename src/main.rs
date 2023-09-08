use map::Map;

pub enum LEVEL {
    EASY,
    MEDIUM,
    HARD,
}

pub mod map;

fn main() {
    let m = Map::new(LEVEL::EASY);

    m.print();
}


