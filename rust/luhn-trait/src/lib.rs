pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
// impl<'a> Luhn for &'a str {
//     fn valid_luhn(&self) -> bool {
//         unimplemented!("Determine if '{}' is a valid credit card number.", self);
//     }
// }

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string().replace(" ", "");
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
