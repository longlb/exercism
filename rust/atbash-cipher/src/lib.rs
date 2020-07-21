/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let code = plain
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| match c.is_numeric() {
            true => c,
            false => conv_opp(c.to_ascii_lowercase()),
        });

    let mut result = String::new();
    for (i, c) in code.enumerate() {
        if i % 5 == 0 && i != 0 {
            result.push(' ')
        }
        result.push(c);
    }
    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| match c.is_numeric() {
            true => c,
            false => conv_opp(c),
        })
        .collect()
}

fn conv_opp(c: char) -> char {
    ((c as i16 - 97 - 25).abs() + 97) as u8 as char
}
