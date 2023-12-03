pub mod answer;
pub mod day;
pub mod solver;

pub mod day_1;

use day::Day;

fn main() {
    let days = [Day::new(1)];
    for (i, day) in days.iter().enumerate() {
        println!(
            "Day {})\tPart one: {}\n\t\tPart two: {}",
            i + 1,
            day.part_one,
            day.part_two
        );
    }
}
