pub mod factorial;

use crate::util::error_output;
use crate::util::user_input::*;

pub fn run() {
    welcome();

    let input = read_new();
    println!();

    choose(input);
}

fn choose(input: String) {
    match input.as_str() {
        "1" => factorial(),
        _ => print!("Not implemented yet"),
    }
}

fn welcome() {
    println!("Math\nPlease select the option");
    println!("-----------------------------");
    menu();
}

fn menu() {
    print!(
        "
        Math ::: Option
        1. Factorial
        ~> "
    );
}

fn factorial() {
    println!("Factorial\nPlease input a number");
    print!("~> ");

    let input = str_to_i32(read_new());

    error_output::validate(input);

    factorial::factorial(input);
}