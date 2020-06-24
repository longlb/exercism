pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut finalsum = 0;
    for i in 1..limit {
        let x: Vec<&u32> = factors
            .into_iter()
            .filter(|x| **x > 0)
            .filter(|x| i % **x == 0)
            .collect();
        if x.len() > 0 {
            finalsum += i;
        }
    }
    finalsum
}
