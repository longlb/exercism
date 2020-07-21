/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = &code.replace(" ", "");
    if code.len() < 2 || code.find(|c: char| !c.is_numeric()) != None {
        return false;
    }

    code.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match i % 2 == 1 {
            true => double(c.to_digit(10).unwrap()),
            false => c.to_digit(10).unwrap(),
        })
        .sum::<u32>()
        % 10
        == 0
}

fn double(n: u32) -> u32 {
    2 * n
        - match n < 5 {
            true => 0,
            false => 9,
        }
}
