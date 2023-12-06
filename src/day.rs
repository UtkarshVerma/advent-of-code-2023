use std::fmt;

use crate::answer::Answer;
use crate::solver::Solvers;

pub struct Day {
    pub number: u8,

    pub part_one: Answer,
    pub part_two: Answer,
}

impl Day {
    pub fn new(number: u8, solvers: &Solvers) -> Self {
        let input = &std::fs::read_to_string(format!("inputs/day_{}", number,)).unwrap_or_default();

        Day {
            number,
            part_one: (solvers.part_one)(input),
            part_two: (solvers.part_two)(input),
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Day {})\tPart one: {}\n\t\tPart two: {}",
            self.number, self.part_one, self.part_two
        )
    }
}
