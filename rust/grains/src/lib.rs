pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    let x: u64 = 2;
    x.pow(s - 1)
}

pub fn total() -> u64 {
    let mut finalsum = 0;
    for i in 1..=64 {
        finalsum += square(i);
    }
    finalsum
}
