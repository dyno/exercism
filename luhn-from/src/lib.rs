pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
            .chars()
            .filter(|&c| !c.is_whitespace())
            .rev()
            .enumerate()
            .try_fold((0 as u32, 0 as u32), |(count, sum), (i, c)| {
                c.to_digit(10).map(|d| {
                    let v = if i % 2 == 0 {
                        d
                    } else {
                        let doubled = d * 2;
                        if doubled > 9 {
                            doubled - 9
                        } else {
                            doubled
                        }
                    };
                    (count + 1, sum + v)
                })
            })
            .map(|(count, sum)| count > 1 && sum % 10 == 0)
            .unwrap_or_else(|| false)
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
