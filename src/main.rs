pub mod math;
pub mod misc;
pub mod util;

use crate::util::user_input::*;

fn main() {
    welcome();

    let input = read_new();
    println!();

    choose(input);
}

fn choose(input: String) {
    match input.as_str() {
        "1" => misc::run(),
        "2" => math::run(),
        _ => print!("Not implemented yet"),
    }
}

fn welcome() {
    println!("Hello, this is rust exercise!");
    println!("-----------------------------");
    menu();
}

fn menu() {
    print!(
        "
        Menu
        1. Misc
        2. Math
        ~> "
    );
}
