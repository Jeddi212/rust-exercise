use colored::*;

pub fn title(title: &str, sub_title: &str) {

println!(
    "{} ::: {}",
    title.yellow(),
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
        title.yellow(),
        sub_title,
        extra
    );
}

pub fn prompt() {
    print!("{} ", "~>".green());
}

pub fn line() {
    println!("{}", "--------------------------------------------".blue());
}