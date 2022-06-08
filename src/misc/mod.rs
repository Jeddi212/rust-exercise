pub mod snip_snap;

use crate::util::heading::*;
use crate::util::user_input::*;

pub fn run() {
    title("Snip-snap", "Please input a number!");
    prompt();

    let input = str_to_i32(read_new());

    snip_snap::snip_snap(input);
}