fn push_encoded(acc: &mut String, cc: Option<char>, count: usize) {
    if count > 1 {
        acc.push_str(&count.to_string());
    }
    if let Some(cc) = cc {
        acc.push(cc);
    }
}

pub fn encode(source: &str) -> String {
    let (mut acc, cc, count) = source.chars().fold(
        (String::new(), None::<char>, 0),
        |(mut acc, cc, count), c| match cc {
            None => (acc, Some(c), 1),
            Some(cc) if cc == c => (acc, Some(c), count + 1),
            _ => {
                push_encoded(&mut acc, cc, count);
                (acc, Some(c), 1)
            }
        },
    );

    push_encoded(&mut acc, cc, count);
    acc
}

pub fn decode(source: &str) -> String {
    let (acc, _) =
        source
            .chars()
            .fold((String::new(), String::new()), |(mut acc, mut count), c| {
                if c.is_ascii_digit() {
                    count.push(c);
                    (acc, count)
                } else {
                    let count = if count.is_empty() {
                        1
                    } else {
                        count.parse().unwrap_or(1)
                    };
                    acc.push_str(&c.to_string().repeat(count));
                    (acc, String::new())
                }
            });
    acc
}
