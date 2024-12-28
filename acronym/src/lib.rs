pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|s| s.chars().filter(|c| c.is_alphabetic()).next())
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>()
}
