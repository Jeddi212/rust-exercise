use colored::*;

pub fn title(title: &str, sub_title: &str) {

println!(
    "{} ::: {}",
    title.yellow().bold(),
    sub_title
);

    line();
}

pub fn menu(title: &str, sub_title: &str, extra: &str) {
    print!(
        "
        {} ::: {}
        {}
        ",
        title.yellow().bold(),
        sub_title,
        extra
    );
}

pub fn result_output(result: String) {
    println!("{} {}", 
        "==".yellow().bold(), 
        result.cyan().bold());
}

pub fn prompt() {
    print!("{} ", "~>".green().bold());
}

pub fn line() {
    println!("{}", "--------------------------------------------".blue());
}