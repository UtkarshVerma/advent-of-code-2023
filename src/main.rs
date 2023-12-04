mod answer;
mod day;
mod solver;

mod day_1;
mod day_2;
mod day_3;

use day::Day;

fn main() {
    let days = (1..=3).map(Day::new).collect::<Vec<Day>>();
    for (i, day) in days.iter().enumerate() {
        println!(
            "Day {})\tPart one: {}\n\t\tPart two: {}\n",
            i + 1,
            day.part_one,
            day.part_two
        );
    }
}
