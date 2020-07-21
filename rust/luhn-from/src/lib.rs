pub struct Luhn {
    sin: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = &self.sin.replace(" ", "");
        if code.len() < 2 || code.find(|c: char| !c.is_numeric()) != None {
            return false;
        }

        code.chars()
            .rev()
            .enumerate()
            .map(|(i, c)| match i % 2 == 1 {
                true => double(c.to_digit(10).unwrap()),
                false => c.to_digit(10).unwrap(),
            })
            .sum::<u32>()
            % 10
            == 0
    }
}

fn double(n: u32) -> u32 {
    2 * n
        - match n < 5 {
            true => 0,
            false => 9,
        }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
// impl<'a> From<&'a str> for Luhn {
//     fn from(input: &'a str) -> Self {
//         unimplemented!("From the given input '{}' create a new Luhn struct.", input);
//     }
// }

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            sin: input.to_string(),
        }
    }
}
