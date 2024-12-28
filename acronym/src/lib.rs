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
