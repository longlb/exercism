pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddles = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if is_saddle(row, col, input) {
                saddles.push((row, col));
            }
        }
    }
    saddles
}

fn is_saddle(row: usize, col: usize, input: &[Vec<u64>]) -> bool {
    let rowmax = *input[row].iter().max().unwrap();
    let colmin = input.iter().map(|row| row[col]).min().unwrap();
    let val = input[row][col];

    rowmax == val && colmin == val
}
