pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        Vec::new()
    } else {
        Minefield::new(minefield).convert()
    }
}

struct Minefield {
    field: Vec<Vec<char>>,
    dir: Vec<(isize, isize)>,
    row_dim: usize,
    col_dim: usize,
}

impl Minefield {
    fn new(minefield: &[&str]) -> Self {
        Self {
            field: minefield
                .iter()
                .map(|line| line.chars().collect())
                .collect(),
            dir: vec![
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, 0),
                (1, -1),
            ],
            row_dim: minefield.len(),
            col_dim: minefield[0].len(),
        }
    }

    fn convert(&self) -> Vec<String> {
        let mut new_board = self.field.clone();

        for row in 0..self.row_dim {
            for col in 0..self.col_dim {
                new_board[row][col] = match self.field[row][col] {
                    ' ' => self.num_mines(row, col),
                    '*' => '*',
                    _ => ' ',
                };
            }
        }

        new_board
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<String>>()
    }

    fn num_mines(&self, row: usize, col: usize) -> char {
        let mut surrounding_mines: u8 = 0;
        for (r, c) in &self.dir {
            let nr: isize = row as isize + r;
            let nc: isize = col as isize + c;
            if self.in_bounds(nr, nc) && self.is_mine(nr as usize, nc as usize) {
                surrounding_mines += 1;
            }
        }
        match surrounding_mines {
            0 => ' ',
            _ => (surrounding_mines + 48) as char,
        }
    }

    fn in_bounds(&self, row: isize, col: isize) -> bool {
        row > -1 && col > -1 && row < self.row_dim as isize && col < self.col_dim as isize
    }

    fn is_mine(&self, row: usize, col: usize) -> bool {
        self.field[row][col] == '*'
    }
}
