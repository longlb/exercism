#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() > second_list.len()
        && substrings(first_list, second_list.len()).contains(&second_list)
    {
        Comparison::Superlist
    } else if second_list.len() > first_list.len()
        && substrings(second_list, first_list.len()).contains(&first_list)
    {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn substrings<T: PartialEq>(list: &[T], len: usize) -> Vec<&[T]> {
    let mut substrings = Vec::new();
    for i in 0..list.len() - len + 1 {
        substrings.push(&list[i..i + len])
    }
    substrings
}
