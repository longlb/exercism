use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    input
        .join("")
        .par_chars()
        .filter(|c| c.is_alphabetic())
        .fold(
            || HashMap::new(),
            |mut acc, c| {
                let counter = acc.entry(c.to_ascii_lowercase()).or_insert(0 as usize);
                *counter += 1;
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                for (k, v) in map {
                    acc.entry(k).and_modify(|i| *i += v).or_insert(v);
                }
                acc
            },
        )
}
