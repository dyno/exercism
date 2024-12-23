use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&&word2| is_anagram(word, word2))
        .copied()
        .collect()
}

fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.len() != word2.len() {
        return false;
    }
    let word1_lower = word1.to_lowercase();
    let word2_lower = word2.to_lowercase();
    if word1_lower == word2_lower {
        return false;
    }
    let mut chars1: Vec<char> = word1_lower.chars().collect();
    let mut chars2: Vec<char> = word2_lower.chars().collect();
    chars1.sort_unstable();
    chars2.sort_unstable();

    chars1 == chars2
}
