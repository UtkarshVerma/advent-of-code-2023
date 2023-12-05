use std::collections::HashSet;

use crate::answer::Answer;

struct CardCounter {
    counts: Vec<u32>,
}

impl CardCounter {
    fn new() -> Self {
        CardCounter { counts: vec![] }
    }

    fn increment(&mut self, card_number: usize) {
        match self.counts.get_mut(card_number - 1) {
            Some(value_ptr) => *value_ptr += 1,
            None => {
                for _ in self.counts.len()..card_number - 1 {
                    self.counts.push(0);
                }
                self.counts.push(1);
            }
        }
    }

    fn get(&self, card_number: usize) -> u32 {
        self.counts
            .get(card_number - 1)
            .copied()
            .unwrap_or_default()
    }
}

pub fn solve(input: &str) -> Answer {
    let counter = input.lines().fold(CardCounter::new(), |mut counter, line| {
        let mut parts = line.split(": ");
        let card_num = parts
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut numbers = parts.next().unwrap_or_default().split(" | ");

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
        let matches = our_wins.count();

        // One original
        counter.increment(card_num);

        // Copies won
        for i in 1..=matches {
            for _ in 0..counter.get(card_num) {
                counter.increment(card_num + i);
            }
        }

        counter
    });

    let answer = counter.counts.iter().sum();

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

        assert_eq!(solve(input), Answer::Number(30));
    }
}
