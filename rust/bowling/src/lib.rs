#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    // Frame status
    current_frame: usize,
    frames: Vec<Frame>,
    // Game status
    game_complete: bool,
    total_score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 1,
            frames: vec![Frame::new(1)],
            game_complete: false,
            total_score: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.frames[self.current_frame - 1].pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.game_complete {
            return Err(Error::GameComplete);
        }

        let mut result = FrameResult::InProgress;
        for frame in self.frames.iter_mut() {
            if frame.number == self.current_frame as u16 {
                result = frame.roll(pins);
            } else {
                self.total_score += frame.bonus_roll(pins);
            }
        }

        match result {
            FrameResult::Complete => {
                if self.current_frame == 10 {
                    self.total_score += self.frames[9].score;
                    self.game_complete = true;
                } else {
                    self.current_frame += 1;
                    self.frames.push(Frame::new(self.current_frame as u16))
                }
            }
            FrameResult::InProgress => (),
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.game_complete {
            true => Some(self.total_score),
            false => None,
        }
    }
}

enum FrameResult {
    InProgress,
    Complete,
    //     Open,
    //     Spare,
    //     Strike,
}

struct Frame {
    number: u16,
    pins: u16,
    rolls: u16,
    bonus: u16,
    score: u16,
    fill: bool,
}

impl Frame {
    fn new(number: u16) -> Self {
        Frame {
            number: number,
            pins: 10,
            rolls: 2,
            bonus: 0,
            score: 0,
            fill: false,
        }
    }

    fn roll(&mut self, pins: u16) -> FrameResult {
        self.score += pins;
        self.pins -= pins;
        self.rolls -= 1;

        if self.number == 10 && self.pins == 0 && !self.fill {
            self.pins = 10;
            self.rolls += 1;
            self.fill = true;
        }

        match self.rolls {
            0 => {
                self.bonus = match self.pins {
                    0 => 2,
                    _ => 1,
                };
                FrameResult::Complete
            }
            _ => match self.pins {
                0 => {
                    if self.number == 10 {
                        self.pins = 10;
                        return FrameResult::InProgress;
                    }
                    self.bonus = 3;
                    FrameResult::Complete
                }
                _ => FrameResult::InProgress,
            },
        }
    }

    fn bonus_roll(&mut self, pins: u16) -> u16 {
        if self.bonus == 0 {
            0
        } else if self.bonus == 1 {
            self.bonus -= 1;
            self.score
        } else {
            self.bonus -= 1;
            self.score += pins;
            0
        }
    }
}
