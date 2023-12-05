use std::collections::HashSet;

use crate::answer::Answer;

pub fn solve(input: &str) -> Answer {
    let answer = input
        .lines()
        .map(|line| {
            let mut numbers = line.split(": ").last().unwrap_or_default().split(" | ");

            let parse_num = |num_str: &str| num_str.parse::<u32>().unwrap_or_default();
            let winning_nums = numbers
                .next()
                .unwrap_or_default()
                .split_whitespace()
                .map(parse_num)
                .collect::<HashSet<u32>>();
            let our_nums = numbers
                .next()
                .unwrap_or_default()
                .split_whitespace()
                .map(parse_num)
                .collect::<HashSet<u32>>();

            let our_wins = winning_nums.intersection(&our_nums);
            let matches = our_wins.count() as u32;

            if matches > 0 {
                2_u32.pow(matches - 1)
            } else {
                0
            }
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve(input), Answer::Number(13));
    }
}
