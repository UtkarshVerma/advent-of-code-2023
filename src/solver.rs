use crate::answer::Answer;

type Solver = fn(input: &str) -> Answer;

pub struct Solvers {
    pub part_one: Solver,
    pub part_two: Solver,
}

impl Default for Solvers {
    fn default() -> Self {
        Solvers {
            part_one: default,
            part_two: default,
        }
    }
}

pub fn default(_: &str) -> Answer {
    Answer::Unsolved
}
