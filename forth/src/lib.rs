use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Clone)]
struct VersionedOp {
    name: String,
    version: u32,
}
impl From<&str> for VersionedOp {
    fn from(s: &str) -> Self {
        let (name, version) = s
            .split_once('@')
            .map(|(n, v)| (n.to_string(), v.parse::<u32>().unwrap()))
            .unwrap_or_else(|| (s.to_string(), 0));

        Self { name, version }
    }
}
impl ToString for VersionedOp {
    fn to_string(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }
}

#[derive(Clone)]
enum Op {
    Executable(fn(&mut Forth) -> Result),
    Scriptable(String),
}

pub struct Forth {
    stack: Vec<Value>,
    ops: HashMap<String, Op>,       // name -> Op
    heads: HashMap<String, String>, // name -> name@version
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
            heads: HashMap::new(),
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
                    if in_user_defined {
                        // don't support nested user-defined words
                        return Err(Error::InvalidWord);
                    }
                    in_user_defined = true;
                    stk.clear()
                }
                ";" => {
                    in_user_defined = false;
                    self.redefine_op(&stk)?
                }
                _ if in_user_defined => stk.push(word),
                _ => self.eval_with_defined_ops(word.as_str())?,
            }
        }

        Ok(())
    }

    fn eval_with_defined_ops(&mut self, word: &str) -> Result {
        if word.parse::<Value>().is_ok() {
            self.stack.push(word.parse().unwrap());
            return Ok(());
        }

        let versioned_word = self.heads.get(word).map(|s| s.as_str()).unwrap_or(word);
        if !self.ops.contains_key(versioned_word) {
            return Err(Error::UnknownWord);
        }

        let op = self.ops.get(versioned_word).unwrap();
        match op {
            Op::Executable(f) => f(self)?,
            Op::Scriptable(s) => {
                let s1 = s.clone();
                for wd in s1.split_whitespace() {
                    self.eval_with_defined_ops(wd)?;
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
            .map(|s| self.heads.get(&s).unwrap_or(&s).to_string())
            .collect::<Vec<_>>()
            .join(" ");

        // Increment the version for the word
        let versioned_name = VersionedOp {
            name: name.to_string(),
            version: self
                .heads
                .get(&name)
                .map(|v| VersionedOp::from(v.as_str()).version + 1)
                .unwrap_or(1),
        }
        .to_string();

        self.ops
            .insert(versioned_name.clone(), Op::Scriptable(code));
        self.heads.insert(name.clone(), versioned_name.clone());

        Ok(())
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn top(&mut self) -> std::result::Result<Value, Error> {
        self.stack.last().cloned().ok_or(Error::StackUnderflow)
    }

    fn add(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(a + b);
        Ok(())
    }

    fn sub(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(a - b);
        Ok(())
    }

    fn mul(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(a * b);
        Ok(())
    }

    fn div(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        if b == 0 {
            return Err(Error::DivisionByZero);
        }
        self.stack.push(a / b);
        Ok(())
    }

    fn dup(&mut self) -> Result {
        let a = self.pop()?;
        self.stack.push(a);
        self.stack.push(a);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        self.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(b);
        self.stack.push(a);
        Ok(())
    }

    fn over(&mut self) -> Result {
        let a = self.pop()?;
        let b = self.top()?;
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }
}
