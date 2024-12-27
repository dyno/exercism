/// Determines the relationship between two lists.
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Checks if one list is contained within another list.
fn is_sublist<T: PartialEq>(shorter: &[T], longer: &[T]) -> bool {
    if shorter.is_empty() {
        true
    } else {
        longer.windows(shorter.len())
            .any(|window| window == shorter)
    }
}

/// Compares two lists to determine their relationship.
///
/// Returns a `Comparison` enum indicating whether the first list is:
/// - Equal to the second list
/// - A sublist of the second list
/// - A superlist of the second list
/// - Unequal and neither a sublist nor superlist
pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    let first_in_second = first.len() <= second.len() && is_sublist(first, second);
    let second_in_first = second.len() <= first.len() && is_sublist(second, first);

    match (first_in_second, second_in_first) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
