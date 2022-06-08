pub mod snip_snap;

use crate::util::user_input::*;

pub fn run() {
    println!("Snip-Snap\nPlease input some number!");
    print!("~> ");

    let input = str_to_i32(read_new());

    snip_snap::snip_snap(input);
}