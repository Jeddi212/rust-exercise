pub fn factorial(n: i32) -> u128 {
    let mut result: u128 = n as u128;
    for i in 1..n as u128 {
        result *= i;
    }
    result
}