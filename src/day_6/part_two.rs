use crate::answer::Answer;

pub fn solve(input: &str) -> Answer {
    let mut iter = input.lines();
    let time = iter
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = iter
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let d = ((time.pow(2) - 4 * distance) as f64).sqrt();
    let t1 = ((time as f64) - d) / 2.0;
    let t2 = ((time as f64) + d) / 2.0;
    let floor_t1 = t1 as u64;
    let floor_t2 = t2 as u64;

    let mut possibilities = floor_t2 - floor_t1;
    if (floor_t2 as f64) == t2 {
        possibilities -= 1;
    }

    Answer::Number(possibilities as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(solve(input), Answer::Number(71503));
    }
}
