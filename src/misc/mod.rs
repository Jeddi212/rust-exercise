pub mod snip_snap;

use crate::util::error_output;
use crate::util::heading::*;
use crate::util::user_input::*;

pub fn run() {
    
    title("Misc","Please select the option");
    menu("Misc","Option","1. Snip-snap");
    prompt();

    let input = read_new();
    println!();

    choose(input);
}

fn snip_snap() {
    title("Snip-snap", "Please input a number!");
    prompt();

    let input = str_to_i32(read_new());

    snip_snap::snip_snap(input);
}

fn choose(input: String) {
    match input.as_str() {
        "1" => snip_snap(),
        _ => error_output::not_implemented(),
    }
}