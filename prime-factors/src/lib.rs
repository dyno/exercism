pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut n = n;

    for p in 2.. {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
        if n == 1 {
            break;
        }
    }

    factors
}
