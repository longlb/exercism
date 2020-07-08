pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    num_all(n, 1000000000000000000)
}

fn num_all(n: u64, num: u64) -> String {
    println!("n: {}, num: {}", n, num);
    if num == 1 {
        return num1_999(n);
    }
    let base = n / num; // First hundred
    let rem = num_all(n % num, num / 1000); // the rest, recursice

    match base {
        0 => rem,
        _ => (num1_999(base) + " " + &scale(num) + " " + &rem)
            .trim()
            .to_string(),
    }
}

fn num1_999(n: u64) -> String {
    let hunds = n / 100;
    let tens_ones = num1_99(n % 100);

    match hunds {
        0 => tens_ones,
        _ => (digit(hunds) + " hundred " + &tens_ones).trim().to_string(),
    }
}

fn num1_99(n: u64) -> String {
    let tens = n / 10;
    let ones = n % 10;
    match n {
        1..=9 => digit(n),
        10..=14 => unique(n),
        15..=19 => prefix(ones) + "teen",
        20..=99 => match ones {
            0 => prefix(tens) + "ty",
            _ => prefix(tens) + "ty-" + &digit(ones),
        },
        _ => "".to_string(),
    }
}

fn digit(n: u64) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    }
    .to_string()
}

fn unique(n: u64) -> String {
    match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        _ => "",
    }
    .to_string()
}

fn prefix(n: u64) -> String {
    match n {
        2 => "twen",
        3 => "thir",
        4 => "for",
        5 => "fif",
        6 => "six",
        7 => "seven",
        8 => "eigh",
        9 => "nine",
        _ => "",
    }
    .to_string()
}

fn scale(n: u64) -> String {
    match n {
        1000 => "thousand",
        1000000 => "million",
        1000000000 => "billion",
        1000000000000 => "trillion",
        1000000000000000 => "quadrillion",
        1000000000000000000 => "quintillion",
        _ => "",
    }
    .to_string()
}
