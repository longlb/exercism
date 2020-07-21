use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.pairs.get(codon) {
            Some(x) => Some(*x),
            None => None,
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins = Vec::new();
        let mut codon = String::new();

        for c in rna.chars() {
            codon.push(c);
            if codon.len() == 3 {
                match self.name_for(&codon) {
                    Some("stop codon") => return Some(proteins),
                    Some(x) => proteins.push(x),
                    None => return None,
                }
                codon = String::new();
            }
        }
        match codon.is_empty() {
            true => Some(proteins),
            false => None,
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.iter().cloned().collect::<HashMap<&'a str, &'a str>>(),
    }
}
