pub fn snip_snap(n: i32) {
    for i in 1..=n {
        match (i%3, i%5) {
            (0, 0) => print!("snip-snap"),
            (0, _) => print!("snip"),
            (_, 0) => print!("snap"),
            (_, _) => print!("{}", i)
        }
        print!(" ")
    }
}