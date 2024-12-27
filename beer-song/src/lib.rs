pub fn verse(n: u32) -> String {
    match n {
        0 => r#"No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
"#
        .to_string(),
        1 => r#"1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.
"#
        .to_string(),
        2 => r#"2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.
"#
        .to_string(),
        _ => format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\n\
             Take one down and pass it around, {} bottles of beer on the wall.\n",
            n - 1
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let verses: Vec<String> = (end..=start).rev().map(verse).collect();
    verses.join("\n")
}
