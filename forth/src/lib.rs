use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Clone)]
enum Op {
    Executable(fn(&mut Forth) -> Result),
    Scriptable(HashMap<String, Op>, String),
}

pub struct Forth {
    stack: Vec<Value>,
    ops: HashMap<String, Op>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        let mut ops = HashMap::new();
        ops.insert("+".to_string(), Op::Executable(Self::add));
        ops.insert("-".to_string(), Op::Executable(Self::sub));
        ops.insert("*".to_string(), Op::Executable(Self::mul));
        ops.insert("/".to_string(), Op::Executable(Self::div));
        ops.insert("DUP".to_string(), Op::Executable(Self::dup));
        ops.insert("DROP".to_string(), Op::Executable(Self::drop));
        ops.insert("SWAP".to_string(), Op::Executable(Self::swap));
        ops.insert("OVER".to_string(), Op::Executable(Self::over));

        Self {
            stack: Vec::new(),
            ops,
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut stk: Vec<String> = Vec::new();
        let mut in_user_defined = false;

        for word in input.split_whitespace().map(|w| w.to_uppercase()) {
            match word.as_str() {
                ":" => {
                    // start of user-defined word
                    if in_user_defined {
                        // don't support nested user-defined words
                        return Err(Error::InvalidWord);
                    }
                    in_user_defined = true;
                    stk.clear()
                }
                ";" => {
                    // end of user-defined word
                    in_user_defined = false;
                    self.redefine_op(&stk)?
                }
                _ if in_user_defined => stk.push(word),
                _ if word.parse::<Value>().is_ok() => self.stack.push(word.parse().unwrap()),
                _ => {
                    let ops_snapshot = self.ops.clone();
                    self.eval_with_defined_ops(word.as_str(), &ops_snapshot)?
                }
            }
        }

        Ok(())
    }

    fn eval_with_defined_ops(&mut self, word: &str, ops_snapshot: &HashMap<String, Op>) -> Result {
        if word.parse::<Value>().is_ok() {
            self.stack.push(word.parse().unwrap());
            return Ok(());
        }
        if !ops_snapshot.contains_key(word) {
            return Err(Error::UnknownWord);
        }
        let op = ops_snapshot.get(word).unwrap().clone();
        match op.clone() {
            Op::Executable(f) => f(self)?,
            Op::Scriptable(ops_snapshot, s) => {
                let s1 = s.clone();
                for wd in s1.split_whitespace() {
                    self.eval_with_defined_ops(wd, &ops_snapshot)?;
                }
            }
        }
        Ok(())
    }

    fn redefine_op(&mut self, stk: &Vec<String>) -> Result {
        if stk.len() < 2 {
            return Err(Error::InvalidWord);
        }

        let name = stk[0].to_ascii_uppercase();
        if name.parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
        }

        let code = stk[1..]
            .iter()
            .map(|s| s.to_ascii_uppercase())
            .collect::<Vec<_>>()
            .join(" ");

        let ops_snapshot = self.ops.clone();
        self.ops.insert(name, Op::Scriptable(ops_snapshot, code));
        Ok(())
    }

    fn add(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a + b);
        Ok(())
    }

    fn sub(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a - b);
        Ok(())
    }

    fn mul(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a * b);
        Ok(())
    }

    fn div(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        if b == 0 {
            return Err(Error::DivisionByZero);
        }
        self.stack.push(a / b);
        Ok(())
    }

    fn dup(&mut self) -> Result {
        if self.stack.len() < 1 {
            return Err(Error::StackUnderflow);
        }
        let a = self.stack[self.stack.len() - 1];
        self.stack.push(a);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        if self.stack.len() < 1 {
            return Err(Error::StackUnderflow);
        }
        self.stack.pop();
        Ok(())
    }

    fn swap(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(b);
        self.stack.push(a);
        Ok(())
    }

    fn over(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let a = self.stack[self.stack.len() - 2];
        self.stack.push(a);
        Ok(())
    }
}
