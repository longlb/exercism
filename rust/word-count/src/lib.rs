use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let new_words = words
        .chars()
        .filter_map(|c| match is_punc(c) {
            true => None,
            false => Some(match c == ',' {
                true => ' ',
                false => c.to_ascii_lowercase(),
            }),
        })
        .collect::<String>();

    let mut word_counts: HashMap<String, u32> = HashMap::new();
    for word in new_words.split_whitespace() {
        let counter = word_counts
            .entry(word.trim_matches(|c| c == '\'' || c == '\"').to_string())
            .or_insert(0);
        *counter += 1;
    }

    word_counts
}

fn is_punc(c: char) -> bool {
    ".!?;:&@$%^&".contains(c)
}
