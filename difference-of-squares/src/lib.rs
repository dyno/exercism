pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    sum * sum
}


// By observation and mathematical reasoning, the result of summing  
// i^k (a k-th degree polynomial) is typically a (k+1)-th degree polynomial.
pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
