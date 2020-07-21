use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut valid_trips = HashSet::new();
    let mut m = 2;

    while m <= sum / 4 {
        for n in 1..m {
            if prim_coprimes(m, n) {
                let mut k = 1;
                let mut new_sum = 0;
                while new_sum < sum {
                    let mut trip = euclid_k(m, n, k);
                    new_sum = trip.iter().sum::<u32>();

                    if new_sum == sum {
                        trip.sort();
                        valid_trips.insert(trip);
                        break;
                    } else {
                        k += 1;
                    }
                }
            }
        }
        m += 1;
    }

    valid_trips
}

fn euclid_k(m: u32, n: u32, k: u32) -> [u32; 3] {
    [
        k * (m.pow(2) - n.pow(2)),
        k * 2 * m * n,
        k * (m.pow(2) + n.pow(2)),
    ]
}

fn prim_coprimes(m: u32, n: u32) -> bool {
    gcd(m, n) == 1 && (m % 2 == 0 || n % 2 == 0)
}

fn gcd(a: u32, b: u32) -> u32 {
    match a == 0 {
        true => b,
        false => gcd(b % a, a),
    }
}
