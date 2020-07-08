pub fn abbreviate(phrase: &str) -> String {
    let cleaned: String = phrase
        .chars()
        .map(|c| {
            if !c.is_alphabetic() && c != '\'' {
                ' '
            } else {
                c
            }
        })
        .collect();
    let cleaned2 = &cleaned;
    cleaned2
        .split_ascii_whitespace()
        .map(|w| {
            let mut iter = w.chars();
            let mut past_c = iter.next().unwrap();
            let mut new_string = past_c.to_string();
            for c in iter {
                if past_c.is_lowercase() && c.is_uppercase() {
                    new_string.push(c);
                }
                past_c = c;
            }
            new_string.to_uppercase()
        })
        .collect()
}
