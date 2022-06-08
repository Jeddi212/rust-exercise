use colored::*;

pub fn validate(input: i32) {
    if input < 0 {
        eprintln!("\n{} ::: Your number [{}] is smaller than 0", "ERROR".red(), input);
        std::process::exit(0x0100);
    }
}