#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    a: u64,
    b: u64,
    val: u64,
    pal: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            a,
            b,
            val: a * b,
            pal: (a * b)
                .to_string()
                .as_str()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap(),
        }
    }

    pub fn value(&self) -> u64 {
        self.val
    }

    pub fn pal_val(&self) -> u64 {
        self.pal
    }

    pub fn is_pal(&self) -> bool {
        self.val == self.pal
    }

    pub fn insert(&mut self, _a: u64, _b: u64) {
        // self.a = a;
        // self.b = b;
        // Worthless function for what the original problem asks for.
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let mut palindromes: Vec<Palindrome> = Vec::new();

    for i in min..=max {
        for r in min..=max {
            let pali = Palindrome::new(r, i);
            if pali.is_pal() {
                palindromes.push(pali);
            }
        }
    }

    if palindromes.is_empty() {
        return None;
    }

    let min = palindromes
        .iter()
        .min_by(|x, y| x.pal_val().cmp(&y.pal_val()))
        .unwrap();
    let max = palindromes
        .iter()
        .max_by(|x, y| x.pal_val().cmp(&y.pal_val()))
        .unwrap();

    Some((min.clone(), max.clone()))
}
