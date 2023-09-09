use map::Map;
use std::io;

pub enum LEVEL {
    EASY,
    MEDIUM,
    HARD,
}

pub mod map;

pub mod gui;

fn main() {
    let mut m = Map::new(LEVEL::EASY);

    gui::main();

    return;

    loop {
        m.print();

        println!("Enter your move: (row, column)");

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut numbers = line.split_whitespace();

        let x: u32 = match numbers.next() {
            Some(value) => match value.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid positive integer input");
                    continue;
                },
            },
            None => {
                eprintln!("First number is missing.");
                continue;
            },
        };

        let y: u32 = match numbers.next() {
            Some(value) => match value.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid positive integer input");
                    continue;
                },
            },
            None => {
                eprintln!("Second number is missing.");
                continue;
            },
        };

        if m.reveal(x, y) == false {
            m.game_lost();

            println!("You lost!");

            break;
        }

        m.game_won();
    }
}
