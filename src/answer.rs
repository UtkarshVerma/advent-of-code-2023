use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Answer {
    Number(u32),

    Unsolved,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Answer::Number(value) => write!(f, "{}", value),
            Answer::Unsolved => write!(f, "Unsolved"),
        }
    }
}
