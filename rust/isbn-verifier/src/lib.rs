/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_iter = isbn.chars().filter(|&c| c != '-');
    if isbn_iter.clone().count() != 10 {
        return false;
    }

    let mut total = 0;
    let mut mult = 10;
    for c in isbn_iter {
        match "0123456789X".contains(c) {
            true => {
                if mult != 1 && c == 'X' {
                    return false;
                }
                total += digs(c) * mult;
                mult -= 1;
            }
            false => return false,
        }
    }

    total % 11 == 0
}

fn digs(c: char) -> u32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'X' => 10,
        _ => 404,
    }
}
