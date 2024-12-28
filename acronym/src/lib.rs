pub fn abbreviate(phrase: &str) -> String {
    let (acc, _) = phrase
        .chars()
        .fold((String::new(), ' '), |(mut acc, prev), c| {
            if c.is_alphabetic()
                && (prev.is_whitespace()
                    || prev == '-'
                    || prev == '_'
                    || (prev.is_lowercase() && c.is_uppercase()))
            {
                acc.push(c.to_ascii_uppercase());
            }
            (acc, c)
        });

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbreviate() {
        assert_eq!(abbreviate("mTLS"), "MT");
        assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
    }
}
