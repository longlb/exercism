pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let heads = "[{(";
    let tails = "]})";

    for c in string.chars() {
        if heads.contains(c) {
            stack.push(c);
        } else if tails.contains(c) {
            if stack.len() > 0 && stack[stack.len() - 1] == partner(c) {
                stack.pop();
            } else {
                return false;
            }
        }
    }
    match stack.len() {
        0 => true,
        _ => false,
    }
}

fn partner(ptnr: char) -> char {
    match ptnr {
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '(' => ')',
        ')' => '(',
        _ => '0',
    }
}
