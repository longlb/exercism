#[derive(Debug)]
pub struct ChessPosition {
    rank: i32, // Vertical
    file: i32, // Horizontal
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !ChessPosition::in_range(rank) || !ChessPosition::in_range(file) {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }

    fn in_range(num: i32) -> bool {
        num > -1 && num < 8
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.same_rank(other) || self.same_file(other) || self.same_diag(other) {
            true
        } else {
            false
        }
    }

    fn same_rank(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank
    }

    fn same_file(&self, other: &Queen) -> bool {
        self.position.file == other.position.file
    }

    fn same_diag(&self, other: &Queen) -> bool {
        let slope =
            (self.position.file - other.position.file) / (self.position.rank - other.position.rank);
        slope == 1 || slope == -1
    }
}
