#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (shorter, longer) = if first_list.len() <= second_list.len() {
        (first_list, second_list)
    } else {
        (second_list, first_list)
    };

    if shorter == longer {
        Comparison::Equal
    } else if longer.windows(shorter.len()).any(|window| window == shorter) {
        if shorter.len() == longer.len() {
            Comparison::Equal
        } else if shorter.len() == second_list.len() {
            Comparison::Sublist
        } else {
            Comparison::Superlist
        }
    } else {
        Comparison::Unequal
    }
}
