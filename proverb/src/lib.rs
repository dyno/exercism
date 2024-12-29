pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    list.windows(2)
        .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
        .chain(std::iter::once(format!("And all for the want of a {}.", list[0])))
        .collect::<Vec<String>>()
        .join("\n")
}
