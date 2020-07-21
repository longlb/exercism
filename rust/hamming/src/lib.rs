/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut hamming = 0;
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    for _ in 0..s1.len() {
        if iter1.next().unwrap() != iter2.next().unwrap() {
            hamming += 1;
        }
    }
    Some(hamming)
}
