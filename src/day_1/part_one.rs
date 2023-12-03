use crate::answer::Answer;

pub fn solve(input: &str) -> Answer {
    let answer = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|char| char.is_ascii_digit())
                .unwrap_or_default()
                .to_digit(10)
                .unwrap_or_default();

            let last = line
                .chars()
                .rev()
                .find(|char| char.is_ascii_digit())
                .unwrap_or_default()
                .to_digit(10)
                .unwrap_or_default();

            10 * first + last
        })
        .sum();

    Answer::Number(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve(input), Answer::Number(142));
    }
}
