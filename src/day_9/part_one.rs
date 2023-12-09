use crate::answer::Answer;

fn compute_sequence_difference(sequence: &[i32]) -> Vec<i32> {
    sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

pub fn solve(input: &str) -> Answer {
    let answer = input
        .lines()
        .map(|line| {
            let sequence = line
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect::<Vec<i32>>();

            let mut sum = 0;
            let mut differences = sequence;
            while differences.iter().any(|x| *x != 0) {
                let last = differences.last().copied().unwrap_or_default();
                differences = compute_sequence_difference(&differences);
                sum += last;
            }

            sum
        })
        .sum();

    Answer::SignedNumber(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(solve(input), Answer::SignedNumber(114));
    }

    #[test]
    fn test_sequence_operations() {
        let input = vec![1, 3, 6, 10, 15, 21, 28];
        assert_eq!(compute_sequence_difference(&input), vec![2, 3, 4, 5, 6, 7]);
    }
}
