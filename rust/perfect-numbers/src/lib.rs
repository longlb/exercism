#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let diff: i64 = num as i64 - (1..num).filter(|n| num % n == 0).sum::<u64>() as i64;
    if diff < 0 {
        Some(Classification::Abundant)
    } else if diff > 0 {
        Some(Classification::Deficient)
    } else {
        Some(Classification::Perfect)
    }
}
