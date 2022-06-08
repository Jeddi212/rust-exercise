use crate::util::user_input::*;

pub fn run() {
    println!("Snip-Snap\nPlease input some number!");
    print!("~> ");

    let input = str_to_i32(read_new());

    for i in 1..=input {
        match (i%3, i%5) {
            (0, 0) => print!("snip-snap"),
            (0, _) => print!("snip"),
            (_, 0) => print!("snap"),
            (_, _) => print!("{}", i)
        }
        print!(" ")
    }
}