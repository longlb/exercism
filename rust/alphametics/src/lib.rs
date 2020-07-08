use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Split and organize the input into addends and the sum.
    let mut input_iter = input.split(" == ");
    let addends: Vec<&str> = input_iter.next().unwrap().split(" + ").collect();
    let final_sum: &str = input_iter.next().unwrap();

    // Initialize the solution map and unique characters stack.
    let mut solution = HashMap::new();
    let mut uniques: Vec<char> = input
        .chars()
        .filter(|c| !" +=".contains(*c))
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();

    // Generate permutations until a solution is found.
    gen_permutations(&addends, final_sum, &mut solution, &mut uniques);

    match solution.len() {
        0 => None,
        _ => Some(solution),
    }
}

fn gen_permutations(
    addends: &Vec<&str>,
    final_sum: &str,
    map: &mut HashMap<char, u8>,
    uniques: &mut Vec<char>,
) {
    let letter = uniques.pop().unwrap();
    for i in 0..10 {
        if let None = map.values().find(|&&vals| vals == i) {
            map.insert(letter, i);
            if uniques.is_empty() {
                if no_leading_zeroes(addends, final_sum, map)
                    && verify_solution(addends, final_sum, map)
                {
                    return;
                }
            } else {
                gen_permutations(addends, final_sum, map, uniques);
                if uniques.is_empty() {
                    return;
                }
            }
            map.remove(&letter);
        }
    }
    uniques.push(letter);
}

fn verify_solution(addends: &Vec<&str>, final_sum: &str, map: &HashMap<char, u8>) -> bool {
    let addends_total: u64 = addends.iter().map(|addend| str_to_num(addend, map)).sum();
    addends_total == str_to_num(final_sum, map)
}

fn no_leading_zeroes(addends: &Vec<&str>, final_sum: &str, map: &HashMap<char, u8>) -> bool {
    let zero = match map.iter().filter(|v| *v.1 == 0 as u8).next() {
        Some(c) => *c.0,
        None => return true,
    };

    if addends
        .iter()
        .filter(|word| word.chars().next().unwrap() == zero)
        .count()
        > 0
    {
        return false;
    }

    final_sum.chars().next().unwrap() != zero
}

fn str_to_num(s: &str, map: &HashMap<char, u8>) -> u64 {
    let mut num_rep = "".to_string();
    for c in s.chars() {
        num_rep += &map.get(&c).unwrap().to_string();
    }
    num_rep.parse::<u64>().unwrap()
}
