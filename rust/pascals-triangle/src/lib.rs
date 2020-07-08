pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = Vec::new();
        let mut last_row: Vec<u32> = Vec::new();

        for i in 0..row_count {
            let mut row = vec![1];

            if i >= 1 {
                for index in 0..(i - 1) as usize {
                    row.push(last_row[index] + last_row[index + 1]);
                }
                row.push(1);
                last_row = row.clone();
            }

            triangle.push(row);
        }
        PascalsTriangle { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
