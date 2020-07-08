pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut spiral = Spiral::new(size);
    for i in 0..size.pow(2) {
        spiral.set(i + 1);
        spiral.advance();
    }
    spiral.spiral
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Spiral {
    spiral: Vec<Vec<u32>>,
    direction: Direction,
    size: usize,
    x: usize,
    y: usize,
}

impl Spiral {
    fn new(size: u32) -> Self {
        Spiral {
            spiral: vec![vec![0; size as usize]; size as usize],
            direction: Direction::Right,
            size: size as usize,
            x: 0,
            y: 0,
        }
    }

    fn set(&mut self, num: u32) {
        self.spiral[self.x][self.y] = num;
    }

    fn advance(&mut self) {
        if !self.forward_okay() {
            self.turn_right();
        }
        match self.direction {
            Direction::Right => self.y += 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
            Direction::Up => self.x -= 1,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn forward_okay(&self) -> bool {
        match self.direction {
            Direction::Right => self.y + 1 < self.size && self.spiral[self.x][self.y + 1] == 0,
            Direction::Down => self.x + 1 < self.size && self.spiral[self.x + 1][self.y] == 0,
            Direction::Left => self.y != 0 && self.spiral[self.x][self.y - 1] == 0,
            Direction::Up => self.x != 0 && self.spiral[self.x - 1][self.y] == 0,
        }
    }
}
