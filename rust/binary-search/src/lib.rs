pub fn find(array: &[i32], key: i32) -> Option<usize> {
    println!("{:?}", array);
    if array.is_empty() || key > array[array.len() - 1] || key < array[0] {
        return None;
    }

    let mid = array.len() / 2;
    if key == array[mid] {
        Some(mid)
    } else if key < array[mid] {
        find(&array[0..mid], key)
    } else {
        match find(&array[mid..array.len()], key) {
            Some(x) => Some(x + mid),
            None => None,
        }
    }
}
