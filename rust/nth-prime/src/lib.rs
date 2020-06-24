pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    } else if n == 1 {
        return 3;
    }

    let mut primed = 2;
    let mut num = 5;
    loop {
        if primed == n {
            return num;
        }
        num += 2;
        if is_prime_over5(num) {
            primed += 1;
        }
    }
}

fn is_prime_over5(n: u32) -> bool {
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut num = 5;

    while num * num <= n {
        if n % num == 0 || n % (num + 2) == 0 {
            return false;
        }
        num = num + 6;
    }

    true
}
