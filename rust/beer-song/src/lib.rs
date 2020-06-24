pub fn verse(n: u32) -> String {
    if n == 0 {
        format!(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        )
    } else {
        format!(
            "{} on the wall, {}.\nTake {} down and pass it around, {} on the wall.\n",
            string_bottles(n),
            string_bottles(n),
            match n {
                1 => "it",
                _ => "one",
            },
            string_bottles(n - 1)
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    let mut num = start;
    loop {
        result.push_str(&verse(num));
        if num == end {
            break;
        }
        result.push_str("\n");
        num -= 1;
    }
    result
}

fn string_bottles(n: u32) -> String {
    match n {
        0 => "no more bottles of beer".to_string(),
        1 => "1 bottle of beer".to_string(),
        _ => format!("{} bottles of beer", n),
    }
}
