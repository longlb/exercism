#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    } else if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    } else if let Some(x) = string_digits.find(|c: char| !c.is_numeric()) {
        return Err(Error::InvalidDigit(string_digits.chars().nth(x).unwrap()));
    }

    let mut max = 0;
    for slice in string_digits.chars().collect::<Vec<char>>().windows(span) {
        let x = slice
            .iter()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .product::<u64>();
        max = if x > max { x } else { max };
    }
    Ok(max)
}
