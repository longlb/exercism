pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut colz = n;
    let mut steps = 0;

    while colz != 1 {
        match colz % 2 == 0 {
            true => colz /= 2,
            false => colz = colz * 3 + 1,
        }
        steps += 1;
    }

    Some(steps)
}
