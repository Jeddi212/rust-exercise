pub mod basic;
pub mod recursive;

use crate::util::error_output;
use crate::util::heading::*;
use crate::util::user_input::*;

pub fn run() {
    title("Math","Please select the option");
    menu("Math","Option","1. Basic
        2. Recursive");
    prompt();

    let input = read_new();
    println!();

    choose(input);
}

fn choose(input: String) {
    match input.as_str() {
        "1" => basic::run(),
        "2" => recursive::run(),
        _ => error_output::not_implemented(),
    }
}

