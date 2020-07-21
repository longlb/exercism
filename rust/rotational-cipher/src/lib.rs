pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c.is_alphabetic() {
            true => shift(c, key),
            false => c,
        })
        .collect::<String>()
}

fn shift(c: char, key: i8) -> char {
    match c.is_uppercase() {
        true => (((c as i8 - 65 + key + 26) % 26) + 65) as u8 as char,
        false => (((c as i8 - 97 + key + 26) % 26) + 97) as u8 as char,
    }
}
