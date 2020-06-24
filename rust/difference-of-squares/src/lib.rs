pub fn square_of_sum(n: u32) -> u32 {
    let mut finalsum = 0;
    for i in 1..=n {
        finalsum += i;
    }
    finalsum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut finalsum = 0;
    for i in 1..=n {
        finalsum += i.pow(2);
    }
    finalsum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
