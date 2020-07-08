pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    if upper_bound > 1 {
        let mut range = vec![true; upper_bound as usize - 1];
        let mut i = 2;

        while i as f64 <= (upper_bound as f64).sqrt() {
            if range[i - 2] {
                let mut j = i.pow(2);
                while j <= upper_bound as usize {
                    range[j - 2] = false;
                    j += i;
                }
            }
            i += 1;
        }

        for x in 0..range.len() {
            if range[x] {
                primes.push(x as u64 + 2)
            }
        }
    }

    primes
}
