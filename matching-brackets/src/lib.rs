use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{')]);

    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if let Some(&expected) = pairs.get(&c) {
                    if stack.pop() != Some(expected) {
                        return false;
                    }
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
