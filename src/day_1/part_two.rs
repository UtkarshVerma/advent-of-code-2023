use crate::answer::Answer;

pub fn solve(_: &str) -> Answer {
    // let answer = input
    //     .lines()
    //     .map(|_| {
    //         let first = 10;
    //         let last = 20;

    //         10 * first + last
    //     })
    //     .sum();

    Answer::Unsolved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve(input), Answer::Number(281));
    }
}
