use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".contains(nucleotide) {
        Err(nucleotide)
    } else {
        let mut total = 0;
        for c in dna.chars() {
            if nucleotide == c {
                total += 1;
            } else if !"ACGT".contains(c) {
                return Err(c);
            }
        }
        Ok(total)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_map = HashMap::new();
    for nucleotide in "ACGT".chars() {
        dna_map.insert(nucleotide, count(nucleotide, dna)?);
    }
    Ok(dna_map)
}
