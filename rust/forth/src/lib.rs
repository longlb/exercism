use maplit::hashmap;
use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone)]
enum Op {
    Num(Value),
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
}

pub struct Forth {
    data: Vec<Value>,
    ops: HashMap<String, Vec<Op>>,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            data: Vec::new(),
            ops: hashmap!["+".to_string() => vec![Op::Add],
                          "-".to_string() => vec![Op::Sub],
                          "*".to_string() => vec![Op::Mul],
                          "/".to_string() => vec![Op::Div],
                          "dup".to_string() => vec![Op::Dup],
                          "drop".to_string() => vec![Op::Drop],
                          "swap".to_string() => vec![Op::Swap],
                          "over".to_string() => vec![Op::Over],],
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.data.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut input = input.split_ascii_whitespace();
        while let Some(op) = input.next() {
            if let Ok(x) = op.parse::<Value>() {
                self.data.push(x)
            } else if op == ":" {
                self.create_op(&mut input)?;
            } else {
                self.operate(op)?;
            }
        }
        Ok(())
    }

    fn create_op<'a, I>(&mut self, input: &mut I) -> ForthResult
    where
        I: Iterator<Item = &'a str>,
    {
        let new_word = self.op_word_valid(input.next())?.to_lowercase();
        let mut opers: Vec<Op> = Vec::new();
        let mut concluded = false;

        while let Some(new_op) = input.next() {
            if new_op == ";" {
                self.ops
                    .entry(new_word)
                    .and_modify(|o| {
                        o.clear();
                        o.append(&mut opers)
                    })
                    .or_insert(opers);
                concluded = true;
                break;
            } else if let Ok(x) = new_op.parse::<Value>() {
                opers.push(Op::Num(x));
            } else {
                opers.append(&mut self.get_ops(new_op)?);
            }
        }

        if !concluded {
            return Err(Error::InvalidWord);
        }
        Ok(())
    }

    fn op_word_valid<'a>(&self, op: Option<&'a str>) -> Result<&'a str, Error> {
        match op {
            None => Err(Error::InvalidWord),
            Some(x) => match x.parse::<Value>().is_ok() {
                true => Err(Error::InvalidWord),
                false => Ok(x),
            },
        }
    }

    fn operate(&mut self, op: &str) -> ForthResult {
        for oper in self.get_ops(op)? {
            self.base_op(oper)?;
        }
        Ok(())
    }

    fn get_ops(&self, op: &str) -> Result<Vec<Op>, Error> {
        match self.ops.get(&op.to_ascii_lowercase()) {
            Some(x) => Ok(x.clone()),
            None => Err(Error::UnknownWord),
        }
    }

    fn base_op(&mut self, op: Op) -> ForthResult {
        match op {
            Op::Num(x) => {
                self.data.push(x);
                Ok(())
            }
            Op::Add => self.add(),
            Op::Sub => self.sub(),
            Op::Mul => self.mul(),
            Op::Div => self.div(),
            Op::Dup => self.dup(),
            Op::Drop => self.drop(),
            Op::Swap => self.swap(),
            Op::Over => self.over(),
        }
    }

    fn top(&mut self) -> Result<Value, Error> {
        if self.data.len() < 1 {
            Err(Error::StackUnderflow)
        } else {
            Ok(self.data.pop().unwrap())
        }
    }

    fn add(&mut self) -> ForthResult {
        let (right, left) = (self.top()?, self.top()?);
        self.data.push(left + right);
        Ok(())
    }

    fn sub(&mut self) -> ForthResult {
        let (right, left) = (self.top()?, self.top()?);
        self.data.push(left - right);
        Ok(())
    }

    fn mul(&mut self) -> ForthResult {
        let (right, left) = (self.top()?, self.top()?);
        self.data.push(left * right);
        Ok(())
    }

    fn div(&mut self) -> ForthResult {
        let (right, left) = (self.top()?, self.top()?);
        if right == 0 {
            return Err(Error::DivisionByZero);
        }
        self.data.push(left / right);
        Ok(())
    }

    fn dup(&mut self) -> ForthResult {
        let top = self.top()?;
        self.data.push(top);
        self.data.push(top);
        Ok(())
    }

    fn drop(&mut self) -> ForthResult {
        self.top()?;
        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let (right, left) = (self.top()?, self.top()?);
        self.data.push(right);
        self.data.push(left);
        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let (right, left) = (self.top()?, self.top()?);
        self.data.push(left);
        self.data.push(right);
        self.data.push(left);
        Ok(())
    }
}
