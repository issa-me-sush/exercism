enum Relationship {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> Comparison {
    let len1 = list1.len();
    let len2 = list2.len();

    if len1 == len2 && list1 == list2 {
        return Comparison::Equal;
    } else if len1 <= len2 && (0..=len2 - len1).any(|i| &list2[i..i + len1] == list1) {
        return Comparison::Sublist;
    } else if len1 >= len2 && (0..=len1 - len2).any(|i| &list1[i..i + len2] == list2) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
