pub struct PrimeIter {
    current: u64,
    primes: Vec<u64>,
}

impl PrimeIter {
    pub fn new() -> Self {
        PrimeIter {
            current: 2,
            primes: Vec::new(),
        }
    }

    fn is_prime(&self, n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let sqrt = (n as f64).sqrt() as u64;
        for &prime in self.primes.iter().take_while(|&&p| p <= sqrt) {
            if n % prime == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.is_prime(self.current) {
            self.current += 1;
        }
        let prime = self.current;
        self.primes.push(prime);
        self.current += 1;
        Some(prime)
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
