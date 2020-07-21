use num_bigint::{BigInt, Sign};
use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg, Sub};
/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, Clone)]
pub struct Decimal {
    num: BigInt,
    scale: isize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        Some(
            Self {
                num: input.replace(".", "").parse::<BigInt>().unwrap(),
                scale: match input.find('.') {
                    Some(x) => input.len() - 1 - x,
                    None => 0,
                } as isize,
            }
            .scale_zeroes(),
        )
    }

    fn scale_up(mut self, scale: isize) -> Self {
        while self.scale != scale {
            self.num *= 10;
            self.scale += 1;
        }
        self
    }

    fn scale_zeroes(mut self) -> Self {
        while self.has_trailing_zeros() && self.scale > 0 {
            self.num /= 10;
            self.scale -= 1;
        }
        self
    }

    fn has_trailing_zeros(&self) -> bool {
        self.num.modpow(&BigInt::from(1), &BigInt::from(10)) == BigInt::from(0)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.scale.cmp(&other.scale) {
            Ordering::Equal => self.num.partial_cmp(&other.num),
            _ => Some(match self.clone().sub(other.clone()).num.sign() {
                Sign::NoSign => Ordering::Equal,
                Sign::Plus => Ordering::Greater,
                Sign::Minus => Ordering::Less,
            }),
        }
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(self) -> Self {
        Self {
            num: -self.num,
            scale: self.scale,
        }
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(mut self, mut other: Self) -> Self {
        match self.scale > other.scale {
            true => other = other.scale_up(self.scale),
            false => self = self.scale_up(other.scale),
        }

        Self {
            num: self.num + other.num,
            scale: self.scale,
        }
        .scale_zeroes()
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Self) -> Self {
        self.add(-other)
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Self) -> Self {
        Self {
            num: self.num * other.num,
            scale: self.scale + other.scale,
        }
        .scale_zeroes()
    }
}
