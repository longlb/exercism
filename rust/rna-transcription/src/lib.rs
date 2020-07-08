#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !"ACGT".contains(c) {
                return Err(i);
            }
        }
        Ok(DNA {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            rna: self
                .dna
                .as_str()
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => c,
                })
                .collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !"ACGU".contains(c) {
                return Err(i);
            }
        }
        Ok(RNA {
            rna: rna.to_string(),
        })
    }
}
