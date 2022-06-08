pub fn factorial(n: i32) {
    let mut result: i32 = n;
    for i in 1..n {
        result *= i;
    }
    println!("Factorial: {}", result);
}