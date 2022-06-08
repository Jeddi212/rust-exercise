pub mod factorial;

use colored::*;
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

    if input < 0 {
        eprintln!("\n{} ::: Your number [{}] is smaller than 0", "ERROR".red(), input);
        std::process::exit(0x0100);
    }

    factorial::factorial(input);
}