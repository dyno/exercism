pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let mut digits = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
        .iter()
        .try_fold(0u32, |acc, &digit| {
            digit
                .checked_pow(digits.len() as u32)
                .and_then(|power| acc.checked_add(power))
        })
        .map_or(false, |sum| sum == num)
}
