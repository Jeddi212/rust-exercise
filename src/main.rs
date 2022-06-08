pub mod math;
pub mod misc;
pub mod util;

use crate::util::error_output;
use crate::util::heading::*;
use crate::util::user_input::*;

fn main() {
    title("Hello", "this is rust exercise");
    menu("Menu", "🦀", "1. Misc
        2. Math"
    );
    prompt();

    let input = read_new();
    println!();

    choose(input);
}

fn choose(input: String) {
    match input.as_str() {
        "1" => misc::run(),
        "2" => math::run(),
        _ => error_output::not_implemented(),
    }
}
