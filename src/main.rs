pub mod gui;  //  Module for the GUI
pub mod map;  //  Module for the game logic

use map::Map;
use std::io;

//  All possible levels
pub enum LEVEL {
    EASY,
    MEDIUM,
    HARD,
}

fn main() {
    let op: u32;

    //  Menu. Read lines until the input is correct, then start the gui
    loop {
        println!("Choose dificulty:");
        println!("1 -> EASY");
        println!("2 -> MEDIUM");
        println!("3 -> HARD");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => {
                if num > 3 || num == 0 {
                    eprintln!("Please input a valid option!");
                    continue;
                }

                num
            }
            Err(_) => {
                eprintln!("Please input a valid option!");
                continue;
            }
        };

        op = input;

        break;
    }

    unsafe {
        match op {
            1 => gui::main(LEVEL::EASY),
            2 => gui::main(LEVEL::MEDIUM),
            3 => gui::main(LEVEL::HARD),
            _ => (),
        }
    }
}
