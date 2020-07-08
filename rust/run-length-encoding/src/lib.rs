pub fn encode(source: &str) -> String {
    if source == "" {
        return "".to_string();
    }
    let mut encoded = String::new();
    let mut char_iter = source.chars();
    let mut current: char = char_iter.next().unwrap();
    let mut num_curr: u32 = 1;

    for c in char_iter {
        match c == current {
            true => num_curr += 1,
            false => {
                attach_c(&mut encoded, current, num_curr);
                current = c;
                num_curr = 1;
            }
        }
    }
    attach_c(&mut encoded, current, num_curr);
    encoded
}

fn attach_c(encoded: &mut String, current: char, num_curr: u32) {
    if num_curr > 1 {
        encoded.push_str(&num_curr.to_string());
    }
    encoded.push(current);
}

pub fn decode(source: &str) -> String {
    if source == "" {
        return "".to_string();
    }
    let mut decoded = String::new();
    let mut num_rep = String::new();

    for c in source.chars() {
        match c.is_numeric() {
            true => num_rep.push(c),
            false => {
                if num_rep == "" {
                    decoded.push(c);
                } else {
                    decoded.push_str(
                        &vec![c; num_rep.parse().unwrap()]
                            .into_iter()
                            .collect::<String>(),
                    );
                    num_rep = String::new();
                }
            }
        }
    }

    decoded
}
