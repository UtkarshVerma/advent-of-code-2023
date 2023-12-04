use crate::answer::Answer;
use std::cmp::max;

struct BallCounts {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn solve(input: &str) -> Answer {
    let answer = input
        .lines()
        .map(|line| {
            let sets_string = line.split(": ").last().unwrap_or_default();
            let max_ball_count = sets_string.split("; ").fold(
                BallCounts {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut counts, set_string| {
                    let set_count = set_string.split(", ").fold(
                        BallCounts {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |mut count, ball_string| {
                            let mut ball_iter = ball_string.split_ascii_whitespace();
                            let ball_count: u32 = ball_iter
                                .next()
                                .unwrap_or_default()
                                .parse()
                                .unwrap_or_default();
                            let ball_color = ball_iter.next().unwrap_or_default();

                            match ball_color {
                                "red" => count.red += ball_count,
                                "green" => count.green += ball_count,
                                "blue" => count.blue += ball_count,
                                _ => (),
                            }

                            count
                        },
                    );

                    counts.red = max(counts.red, set_count.red);
                    counts.green = max(counts.green, set_count.green);
                    counts.blue = max(counts.blue, set_count.blue);

                    counts
                },
            );

            max_ball_count.red * max_ball_count.green * max_ball_count.blue
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
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve(input), Answer::Number(2286));
    }
}
