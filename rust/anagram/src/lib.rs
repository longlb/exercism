use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter_map(|&ana| match is_anagram(word, ana) {
            true => Some(ana),
            false => None,
        })
        .collect::<HashSet<&'a str>>()
}

fn is_anagram(word: &str, ana: &str) -> bool {
    word.to_lowercase() != ana.to_lowercase() && sorted_str(word) == sorted_str(ana)
}

fn sorted_str(s: &str) -> Vec<char> {
    let mut s = s.to_lowercase().chars().collect::<Vec<char>>();
    s.sort();
    s
}
