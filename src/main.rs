pub mod gui;
pub mod map;

use map::Map;
use std::io;

pub enum LEVEL {
    EASY,
    MEDIUM,
    HARD,
}

fn main() {
    unsafe {
        gui::main(LEVEL::EASY);
    }
}
