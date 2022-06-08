use colored::*;

pub fn snip_snap(n: i32) {
    for i in 1..=n {
        match (i%3, i%5) {
            (0, 0) => print!("{}", "snip-snap".truecolor(22, 222, 171).bold()),
            (0, _) => print!("{}", "snip".truecolor(211, 211, 211)),
            (_, 0) => print!("{}", "snap".truecolor(211, 211, 211)),
            (_, _) => print!("{}", i.to_string().truecolor(60, 60, 60))
        }
        print!(" ")
    }
}