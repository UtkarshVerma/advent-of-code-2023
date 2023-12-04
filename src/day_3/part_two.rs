use crate::answer::Answer;

type Part = u32;

fn find_parts_in_window(line: &str, left: usize, right: usize) -> Vec<Part> {
    let find_part_start = |(i, char): (usize, char)| {
        if char.is_ascii_digit() {
            let begin = if i == left {
                line[..left]
                    .char_indices()
                    .rev()
                    .find_map(|(j, char)| {
                        if !char.is_ascii_digit() {
                            return Some(j + 1);
                        }

                        None
                    })
                    .unwrap_or(0)
            } else {
                i
            };

            return Some(begin);
        }

        None
    };

    let mut parts: Vec<Part> = vec![];
    let mut start = left;
    while let Some(begin) = line
        .char_indices()
        .skip(start)
        .take_while(|(j, _)| *j < right)
        .find_map(find_part_start)
    {
        let end = line
            .char_indices()
            .skip(begin)
            .find_map(|(i, char)| {
                if !char.is_ascii_digit() {
                    return Some(i);
                }

                None
            })
            .unwrap_or(line.len());

        start = end;

        let part_str = &line[begin..end];
        if let Ok(part) = part_str.parse::<Part>() {
            parts.push(part);
        }
    }

    parts
}

pub fn solve(input: &str) -> Answer {
    let schematic = input.lines().collect::<Vec<&str>>();

    let answer = schematic
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let mut sum = 0;
            let mut iter = line.char_indices();
            while let Some(gear_index) = iter.find_map(|(j, char)| {
                if char == '*' {
                    return Some(j);
                }

                None
            }) {
                let left = ((gear_index as isize) - 1).max(0) as usize;
                let right = (gear_index + 2).min(line.len());

                let mut parts: Vec<Part> = vec![];
                if i != 0 {
                    let index = i - 1;
                    parts.extend(find_parts_in_window(schematic[index], left, right));
                }

                parts.extend(find_parts_in_window(schematic[i], left, right));

                if i + 1 < schematic.len() {
                    let index = i + 1;
                    parts.extend(find_parts_in_window(schematic[index], left, right));
                }

                if parts.len() != 2 {
                    return 0;
                }

                sum += parts.iter().product::<Part>()
            }

            sum
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve(input), Answer::Number(467835));

        let input = "\
..829.
..*...
.878.6";
        assert_eq!(solve(input), Answer::Number(727862));

        let input = "\
.......
460*989
.......";
        assert_eq!(solve(input), Answer::Number(454940));
    }
}
