pub struct PrimeIter {
    primes: Vec<u64>,
}

impl Default for PrimeIter {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimeIter {
    pub fn new() -> Self {
        PrimeIter { primes: vec![] }
    }

    pub fn is_prime(&self, n: u64) -> bool {
        for p in &self.primes {
            if n % *p == 0 {
                return false;
            }
            if p * p > n {
                break;
            }
        }
        true
    }
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.primes.is_empty() {
            self.primes.push(2);
        } else {
            let mut current = self.primes.last().unwrap() + 1;
            while !self.is_prime(current) {
                current += 1;
            }
            self.primes.push(current);
        }

        self.primes.last().copied()
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;

    for prime in PrimeIter::new() {
        while n % prime == 0 {
            factors.push(prime);
            n /= prime;
        }
        if prime > n || n == 1 {
            break;
        }
    }
    factors
}
