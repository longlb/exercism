pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = Vec::new();

    let limit = alpha_index(c);
    let dims = limit * 2 - 1;
    for i in 0..limit {
        let inner = match i == 0 {
            true => "A".to_string(),
            false => format!("{c}{:^mid$}{c}", ' ', mid = i * 2 - 1, c = index_alpha(i)),
        };
        if i + 1 != limit {
            diamond.insert(i, format!("{:^dims$}", inner, dims = dims));
        }
        diamond.insert(i, format!("{:^dims$}", inner, dims = dims));
    }
    diamond
}

fn alpha_index(c: char) -> usize {
    (c as usize - 65) + 1
}

fn index_alpha(n: usize) -> char {
    (n + 65) as u8 as char
}
