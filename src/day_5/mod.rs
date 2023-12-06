use crate::solver::Solvers;

mod part_one;
mod part_two;

pub const SOLVERS: Solvers = Solvers {
    part_one: part_one::solve,
    part_two: part_two::solve,
};
