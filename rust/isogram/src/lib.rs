use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut used_c = HashSet::new();

    for c in candidate
        .chars()
        .filter(|&c| c != ' ' && c != '-')
        .map(|c| c.to_ascii_lowercase())
    {
        if used_c.contains(&c) {
            return false;
        }
        used_c.insert(c);
    }
    true
}
