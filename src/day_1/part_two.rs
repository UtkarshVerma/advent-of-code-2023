use crate::answer::Answer;

pub fn solve(input: &str) -> Answer {
    let answer = input
        .lines()
        .map(|line| {
            let first = line
                .char_indices()
                .find_map(|(i, char)| {
                    if char.is_ascii_digit() {
                        return Some(char.to_digit(10).unwrap_or_default());
                    }

                    find_number_string(&line[..=i])
                })
                .unwrap_or_default();

            let last = line
                .char_indices()
                .rev()
                .find_map(|(i, char)| {
                    if char.is_ascii_digit() {
                        return Some(char.to_digit(10).unwrap_or_default());
                    }

                    find_number_string(&line[i..])
                })
                .unwrap_or_default();

            10 * first + last
        })
        .sum();

    Answer::Number(answer)
}

fn find_number_string(string: &str) -> Option<u32> {
    let number_strings = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for i in 0..string.len() {
        for (number, number_string) in number_strings.iter().enumerate() {
            if (string[i..]).starts_with(number_string) {
                return Some(number as u32);
            }
        }
    }

    None
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

        let input = "qnshr9threeonefour";
        assert_eq!(solve(input), Answer::Number(94));
    }

    #[test]
    fn test_find_number_string() {
        assert_eq!(find_number_string("seven"), Some(7));
        assert_eq!(find_number_string("onseven"), Some(7));
        assert_eq!(find_number_string("twthreefour"), Some(3));
    }
}
