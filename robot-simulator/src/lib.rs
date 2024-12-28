// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

use Direction::*;
impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: match self.d {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: match self.d {
                North => West,
                East => North,
                South => East,
                West => South,
            },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.d {
            North => (self.x, self.y + 1),
            East => (self.x + 1, self.y),
            South => (self.x, self.y - 1),
            West => (self.x - 1, self.y),
        };
        Self { x, y, d: self.d }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, c| match c {
            'L' => acc.turn_left(),
            'R' => acc.turn_right(),
            'A' => acc.advance(),
            _ => acc,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
