pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if is_silence(msg) {
        "Fine. Be that way!"
    } else if is_yelling(msg) && is_question(msg) {
        "Calm down, I know what I'm doing!"
    } else if is_yelling(msg) {
        "Whoa, chill out!"
    } else if is_question(msg) {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_yelling(message: &str) -> bool {
    let all_chars = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    !all_chars.is_empty() && all_chars.iter().all(|&c| c.is_uppercase())
}

fn is_silence(message: &str) -> bool {
    message.is_empty()
}
