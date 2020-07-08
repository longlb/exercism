pub struct Triangle {
    side1: u64,
    side2: u64,
    side3: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let new_tri = Triangle {
            side1: sides[0],
            side2: sides[1],
            side3: sides[2],
        };

        if new_tri.is_valid() {
            Some(new_tri)
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.side1 == self.side2 && self.side2 == self.side3 && self.side3 == self.side1
    }

    pub fn is_scalene(&self) -> bool {
        self.side1 != self.side2 && self.side2 != self.side3 && self.side3 != self.side1
    }

    pub fn is_isosceles(&self) -> bool {
        self.side1 == self.side2 || self.side2 == self.side3 || self.side3 == self.side1
    }

    fn is_valid(&self) -> bool {
        self.side1 < self.side2 + self.side3
            && self.side2 < self.side1 + self.side3
            && self.side3 < self.side1 + self.side2
    }
}
