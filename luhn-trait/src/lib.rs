pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let s = self.to_string();
        let (count, sum) = s
            .chars()
            .filter(|&c| c.is_digit(10))
            .rev()
            .enumerate()
            .map(|(i, c)| c.to_digit(10).unwrap() * (if i % 2 == 0 { 1 } else { 2 }))
            .map(|c| if c > 9 { c - 9 } else { c })
            .fold((0, 0), |acc, c| (acc.0 + 1, acc.1 + c));

        count > 1 && sum % 10 == 0
    }
}
