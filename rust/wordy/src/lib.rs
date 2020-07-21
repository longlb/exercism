pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is") {
        return None;
    }

    let banned_words = vec!["by", "to", "the", "power"];
    let ops = command
        .trim_end_matches("?")
        .split_ascii_whitespace()
        .filter(|word| !banned_words.contains(word))
        .collect::<Vec<&str>>();

    calculado(&ops[2..ops.len()])
}

fn calculado(ops: &[&str]) -> Option<i32> {
    match ops.len() {
        0 => return None,
        1 => {
            return match ops[0].parse::<i32>() {
                Ok(x) => Some(x),
                Err(_) => None,
            }
        }
        _ => (),
    }

    let strt: i32 = match calculado(&ops[0..ops.len() - 2]) {
        Some(x) => x,
        None => return None,
    };

    let end: i32 = match remove_suff(ops[ops.len() - 1]).parse() {
        Ok(x) => x,
        Err(_) => return None,
    };

    match ops[ops.len() - 2] {
        "plus" => Some(strt + end),
        "minus" => Some(strt - end),
        "multiplied" => Some(strt * end),
        "divided" => Some(strt / end),
        "raised" => Some(strt.pow(end as u32)),
        _ => None,
    }
}

fn remove_suff<'a>(num: &'a str) -> &'a str {
    let mut num = num;
    for pat in vec!["st", "nd", "rd", "th"] {
        num = num.trim_end_matches(pat);
    }
    num
}
