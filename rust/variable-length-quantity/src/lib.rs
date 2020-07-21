#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for n in values.iter().rev() {
        let mut num = *n;
        result.insert(0, (num & 0x7f) as u8);
        num >>= 7;
        while num != 0 {
            result.insert(0, (num & 0xff | 0x80) as u8);
            num >>= 7;
        }
    }

    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = Vec::new();
    let mut val: u64 = 0;

    for &n in bytes {
        val <<= 7;
        val += n as u64 & 0x7f;

        if val != val as u32 as u64 {
            return Err(Error::Overflow);
        }
        if n & 0x80 == 0 {
            result.push(val as u32);
            val = 0;
        }
    }
    match val > 0 || result.is_empty() {
        true => Err(Error::IncompleteNumber),
        false => Ok(result),
    }
}
