pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut n_cop: u64 = n;
    let mut divisor: u64 = 0;
    while n_cop > 1 {
        if is_prime(divisor) {
            while n_cop % divisor == 0 {
                n_cop /= divisor;
                result.push(divisor);
            }
        }
        divisor += 1;
    }
    result
}

fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: u64 = 5;

    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
