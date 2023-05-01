use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    dictionary: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            dictionary: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace();
        while let Some(token) = tokens.next() {
            match token.to_uppercase().as_str() {
                "+" | "-" | "*" | "/" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match token {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => {
                            if b == 0 {
                                return Err(Error::DivisionByZero);
                            }
                            a / b
                        }
                        _ => unreachable!(),
                    };
                    self.stack.push(result);
                }
                "DUP" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    let top = self.stack.last().cloned().unwrap();
                    self.stack.push(top);
                }
                "DROP" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.pop();
                }
                "SWAP" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }
                "OVER" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let len = self.stack.len();
                    let value = self.stack[len - 2];
                    self.stack.push(value);
                }
                _ => {
                    if let Ok(number) = token.parse::<Value>() {
                        self.stack.push(number);
                    } else if self.dictionary.contains_key(token) {
                        let words = self.dictionary.get(token).unwrap().join(" ");
                        self.eval(&words)?;
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }
        Ok(())
    }
}
