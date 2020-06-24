#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    score_latest: Option<u32>,
    high_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut score_new = Vec::new();
        let mut score_latest = None;
        let mut high_scores = Vec::with_capacity(3);

        for i in scores {
            match high_scores.len() {
                0 => high_scores.push(*i),
                1 => {
                    if *i > high_scores[0] {
                        high_scores.push(0);
                        high_scores[1] = high_scores[0];
                        high_scores[0] = *i;
                    } else {
                        high_scores.push(*i)
                    }
                }
                2 => {
                    if *i > high_scores[1] {
                        high_scores.push(0);
                        high_scores[2] = high_scores[1];
                        high_scores[1] = *i;
                        if *i > high_scores[0] {
                            high_scores[1] = high_scores[0];
                            high_scores[0] = *i;
                        }
                    } else {
                        high_scores.push(*i)
                    }
                }
                _ => {
                    if *i > high_scores[2] {
                        high_scores[2] = *i;
                        if *i > high_scores[1] {
                            high_scores[2] = high_scores[1];
                            high_scores[1] = *i;
                            if *i > high_scores[0] {
                                high_scores[1] = high_scores[0];
                                high_scores[0] = *i;
                            }
                        }
                    }
                }
            }
            score_new.push(*i);
            score_latest = Some(*i);
        }

        HighScores {
            scores: score_new,
            score_latest,
            high_scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.score_latest
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.high_scores.len() {
            0 => None,
            _ => Some(self.high_scores[0]),
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.high_scores.clone()
    }
}
