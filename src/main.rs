mod answer;
mod day;
mod solver;

macro_rules! count_modules {
    ($($module:ident),*) => {
        <[()]>::len(&[$(count_modules!(subst $module)),*])
    };
    (subst $_:ident) => {()};
}

macro_rules! load_days {
    ($($module:ident),*) => {
        $(mod $module;)*

        const SOLVERS: [solver::Solvers; count_modules!($($module),*)] = [
            $($module::SOLVERS,)*
        ];
    };
}

use day::Day;

// NOTE: This does not work for discontinuous chunks.
load_days!(day_1, day_2, day_3, day_4, day_5, day_6, day_7);

fn main() {
    let days = SOLVERS
        .iter()
        .enumerate()
        .map(|(i, solvers)| Day::new((i + 1) as u8, solvers))
        .collect::<Vec<Day>>();

    println!(
        "{}",
        days.iter()
            .map(|day| day.to_string())
            .collect::<Vec<String>>()
            .join("\n\n")
    );
}
