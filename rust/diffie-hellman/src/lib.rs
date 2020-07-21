use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    big_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    big_pow(b_pub, a, p)
}

fn big_pow(base: u64, exp: u64, modu: u64) -> u64 {
    if modu == 1 {
        return 0;
    }
    let mut x = 1;
    for _ in 0..exp {
        x = (x * base) % modu;
    }
    x
}
