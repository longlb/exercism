use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    match invalid(key) {
        true => None,
        false => Some(
            s.chars()
                .zip(key.chars().cycle())
                .map(|(s, k)| shift(s, k as i8 - 97))
                .collect::<String>(),
        ),
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    match invalid(key) {
        true => None,
        false => Some(
            s.chars()
                .zip(key.chars().cycle())
                .map(|(s, k)| shift(s, -(k as i8 - 97)))
                .collect::<String>(),
        ),
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut key = String::new();
    let mut rng = thread_rng();
    for _ in 0..100 {
        key.push(rng.gen_range::<u8>(97, 122) as char)
    }
    (key.clone(), encode(&key, s).unwrap())
}

fn invalid(key: &str) -> bool {
    key.chars()
        .filter(|c| c.is_uppercase() || c.is_numeric())
        .count()
        > 0
        || key.is_empty()
}

fn shift(c: char, key: i8) -> char {
    match c.is_uppercase() {
        true => (((c as i8 - 65 + key + 26) % 26) + 65) as u8 as char,
        false => (((c as i8 - 97 + key + 26) % 26) + 97) as u8 as char,
    }
}
