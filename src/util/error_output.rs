use colored::*;

const ERROR_TITLE: &str = "ERROR";

pub fn validate_minimum(input: i32, minimum: i32) {
    if input < minimum {
        eprintln!(
            "\n{} ::: Your number [{}] is smaller than {}", 
            ERROR_TITLE.red().bold(), 
            input.to_string().yellow(), 
            minimum.to_string().yellow()
        );
        std::process::exit(0x0100);
    }
}

pub fn validate_maximum(input: i32, maximum: i32) {
    if input > maximum {
        eprintln!(
            "\n{} ::: Your number [{}] is greater than {}", 
            ERROR_TITLE.red().bold(), 
            input.to_string().yellow(), 
            maximum.to_string().yellow()
        );
        std::process::exit(0x0100);
    }
}

pub fn not_implemented() {
    eprint!("{} ::: Not implemented yet", ERROR_TITLE.red());
}