use std::fmt::{Display, Formatter, Result};

const BASE: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman {
    n: u32,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let mut num = self.n;
        for &(b, c) in BASE.iter() {
            while num >= b {
                write!(_f, "{}", c)?;
                num -= b;
            }
        }
        Ok(())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman { n: num }
    }
}
