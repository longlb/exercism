pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|word| pigify(word))
        .collect::<Vec<String>>()
        .join(" ")
}

fn pigify(input: &str) -> String {
    if rule_1(input) {
        return input.to_string() + "ay";
    } else if input.contains("qu") {
        let temp = input.split("qu").collect::<Vec<&str>>();
        if temp[0].chars().filter(|&c| is_vowel(c)).count() == 0 {
            return temp[1..].join("") + temp[0] + "quay";
        }
    }
    let mut prefix = String::new();
    let mut suffix = String::new();
    let mut trigger = false;

    for (i, c) in input.chars().enumerate() {
        if i > 0 && c == 'y' {
            trigger = true;
        }
        if is_vowel(c) {
            trigger = true;
        }
        match trigger {
            true => prefix.push(c),
            false => suffix.push(c),
        }
    }

    prefix + &suffix + "ay"
}

fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}

fn rule_1(input: &str) -> bool {
    let mut ci = input.chars();
    let one = ci.next().unwrap();
    let two = ci.next().unwrap();
    is_vowel(one)
        || match (one, two) {
            ('x', 'r') | ('y', 't') => true,
            _ => false,
        }
}

// fn rule_3(input: &str) -> {}
