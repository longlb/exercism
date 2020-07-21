pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut result = vec![String::new(); self.rails];
        let mut trigger = true;
        let mut rail = 0;

        for c in text.chars() {
            result[rail].push(c);
            match trigger {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == self.rails - 1 || rail == 0 {
                trigger = !trigger;
            }
        }
        result.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut mapping = vec![Vec::new(); self.rails];
        let cipher_lens = self.row_lengths(cipher.len());
        let mut rail = 0;

        for c in cipher.chars() {
            mapping[rail].push(c);
            if mapping[rail].len() == cipher_lens[rail] {
                rail += 1;
            }
        }

        for item in &mut mapping {
            item.reverse();
        }

        let mut result = String::new();
        let mut trigger = true;
        rail = 0;
        for _ in 0..cipher.len() {
            result.push(mapping[rail].pop().unwrap());
            match trigger {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == self.rails - 1 || rail == 0 {
                trigger = !trigger;
            }
        }
        result
    }

    fn row_lengths(&self, cipher_len: usize) -> Vec<usize> {
        let mut row_lengths = vec![0; self.rails];
        let mut trigger = true;
        let mut rail = 0;

        for _ in 0..cipher_len {
            row_lengths[rail] += 1;
            match trigger {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == self.rails - 1 || rail == 0 {
                trigger = !trigger;
            }
        }
        row_lengths
    }
}
