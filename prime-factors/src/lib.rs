pub fn factors(n: u64) -> Vec<u64> {
    // "This should calculate the prime factors of {n}"
    let mut factors = Vec::new();
    let mut n = n;
    let mut factor = 2;
    while n != 1 {
        while n % factor == 0 {
            factors.push(factor);
            n /= factor;
        }
        factor += 1;
    }
    factors
}
