/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let code =
        plaintext
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| match c.is_ascii_alphabetic() {
                true => char_encode(c.to_ascii_lowercase(), a, b),
                false => c,
            });

    let mut result = String::new();
    for (i, c) in code.enumerate() {
        if i % 5 == 0 {
            result.push(' ')
        }
        result.push(c);
    }

    Ok(result.trim().to_string())
}

fn char_encode(c: char, a: i32, b: i32) -> char {
    num_to_char((((a * char_to_num(c) + b) % 26) + 26) % 26)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(ciphertext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| match c.is_ascii_alphabetic() {
            true => char_decode(c, a, b),
            false => c,
        })
        .collect::<String>())
}

fn char_decode(c: char, a: i32, b: i32) -> char {
    num_to_char((((mmi(a, 26) * (char_to_num(c) - b)) % 26) + 26) % 26)
}

fn gcd(a: i32, b: i32) -> i32 {
    match a == 0 {
        true => b,
        false => gcd(b % a, a),
    }
}

fn mmi(a: i32, b: i32) -> i32 {
    for i in 1..b {
        if a * i % b % b == 1 {
            return i;
        }
    }
    0
}

fn char_to_num(c: char) -> i32 {
    c as i32 - 97
}

fn num_to_char(n: i32) -> char {
    (n as u8 + 97) as char
}
