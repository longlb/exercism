pub fn is_armstrong_number(num: u32) -> bool {
    let temp: Vec<u32> = split_to_vec(num);
    let result: u32 = temp.iter().map(|x| x.pow(temp.len() as u32)).sum();
    result == num
}

fn split_to_vec(num: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut divisor: u32 = 10;
    let mut n_cop: u32 = num;
    while n_cop != 0 {
        let x = n_cop % divisor;
        result.push(x / (divisor / 10));
        n_cop -= x;
        divisor *= 10;
    }
    result
}
