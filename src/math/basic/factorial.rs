pub fn factorial(n: u128) -> u128 {
    // let mut result: u128 = n;
    // for i in 1..n as u128 {
    //     result *= i;
    // }
    // result

    // Functional Programming Approach
    (1..=n).product()
}