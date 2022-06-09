pub fn factorial(n: u128) -> u128 {
    // if                else
    if n <= 1 { return 1; } n * factorial(n - 1)
}