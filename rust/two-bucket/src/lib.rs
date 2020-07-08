#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // Set up the buckets and the stats.
    let mut moves = 0;
    let mut buck1 = BucketContents::new(capacity_1);
    let mut buck2 = BucketContents::new(capacity_2);

    match *start_bucket {
        Bucket::One => {
            if capacity_2 == goal {
                return Some(BucketStats {
                    moves: 2,
                    goal_bucket: Bucket::Two,
                    other_bucket: capacity_1,
                });
            }
        }
        Bucket::Two => {
            if capacity_1 == goal {
                return Some(BucketStats {
                    moves: 2,
                    goal_bucket: Bucket::One,
                    other_bucket: capacity_2,
                });
            }
        }
    }

    while buck1.contents != goal && buck2.contents != goal {
        match *start_bucket {
            Bucket::One => {
                if buck1.is_empty() {
                    buck1.fill();
                } else if buck2.is_full() {
                    buck2.empty()
                } else {
                    buck1.pour(&mut buck2);
                }
            }
            Bucket::Two => {
                if buck2.is_empty() {
                    buck2.fill();
                } else if buck1.is_full() {
                    buck1.empty()
                } else {
                    buck2.pour(&mut buck1);
                }
            }
        }
        if moves == 255 {
            return None;
        }
        moves += 1;
    }

    let (goal_bucket, other_bucket) = match buck1.contents == goal {
        true => (Bucket::One, buck2.contents),
        false => (Bucket::Two, buck1.contents),
    };

    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
}

struct BucketContents {
    capacity: u8,
    contents: u8,
}

impl BucketContents {
    fn new(capacity: u8) -> Self {
        BucketContents {
            capacity,
            contents: 0,
        }
    }

    fn pour(&mut self, other: &mut BucketContents) {
        while other.contents != other.capacity && self.contents != 0 {
            self.contents -= 1;
            other.contents += 1;
        }
    }

    fn fill(&mut self) {
        self.contents = self.capacity;
    }

    fn empty(&mut self) {
        self.contents = 0;
    }

    fn is_empty(&self) -> bool {
        self.contents == 0
    }

    fn is_full(&self) -> bool {
        self.contents == self.capacity
    }
}
