use crate::answer::Answer;
use crate::day_1;
use crate::day_2;

pub struct Day {
    pub part_one: Answer,
    pub part_two: Answer,
}

impl Day {
    pub fn new(number: u8) -> Self {
        let input = &std::fs::read_to_string(format!("inputs/day_{}", number,)).unwrap_or_default();

        let solvers = match number {
            1 => day_1::SOLVERS,
            2 => day_2::SOLVERS,
            _ => Default::default(),
        };

        Day {
            part_one: (solvers.part_one)(input),
            part_two: (solvers.part_two)(input),
        }
    }
}
