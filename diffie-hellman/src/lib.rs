pub fn private_key(p: u64) -> u64 {
    // Simple deterministic way to get a number between 2 and p-1
    // For testing purposes only, not cryptographically secure
    2 + (p >> 2) % (p - 2)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    let mut base: u128 = (base % modulus) as u128;
    let mut exp = exponent;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus as u128;
        }
        base = (base * base) % modulus as u128;
        exp >>= 1;
    }
    result as u64
}
