pub mod factorial;

use crate::util::error_output;
use crate::util::user_input::*;
use crate::util::heading::*;

pub fn run() {
    title("Math","Please select the option");
    menu("Math","Option","1. Factorial");
    prompt();

    let input = read_new();
    println!();

    choose(input);
}

fn choose(input: String) {
    match input.as_str() {
        "1" => factorial(),
        _ => error_output::not_implemented(),
    }
}

fn factorial() {
    title("Factorial", "Please input a number");
    prompt();

    let input = str_to_i32(read_new());

    error_output::validate_minimum(input, 0);
    error_output::validate_maximum(input, 34);

    result_output(factorial::factorial(input).to_string()); 
}