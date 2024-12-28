pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let input: Vec<char> = self.0.chars().filter(|&c| !c.is_whitespace()).collect();

        if input.iter().any(|&c| !c.is_digit(10)) {
            return false;
        }

        let (count, sum) = input
            .iter()
            .filter_map(|c| c.to_digit(10))
            .rev()
            .enumerate()
            .map(|(i, d)| if i % 2 == 1 { d * 2 } else { d })
            .map(|d| if d > 9 { d - 9 } else { d })
            .fold((0, 0), |(count, sum), d| (count + 1, sum + d));

        count > 1 && sum % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
