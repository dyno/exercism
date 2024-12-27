/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let filtered: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    // Check length and valid digits
    if filtered.len() <= 1 {
        return false;
    }

    if !filtered.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let sum: u32 = filtered
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum();

    sum % 10 == 0
}
