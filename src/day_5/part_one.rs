use crate::answer::Answer;

pub fn solve(input: &str) -> Answer {
    let mut iter = input.lines();
    let seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect::<Vec<usize>>();
    iter.next();

    let mut locations = seeds.clone();
    while iter.next().is_some() {
        let mappings = iter.by_ref().take_while(|line| !line.is_empty()).fold(
            Vec::new(),
            |mut mappings, line| {
                let parts = line
                    .split_ascii_whitespace()
                    .map(|part| part.parse().unwrap())
                    .collect::<Vec<usize>>();

                mappings.push((parts[0], parts[1], parts[2]));

                mappings
            },
        );

        for location in locations.iter_mut() {
            if let Some(mapping) = mappings.iter().find_map(|(dest, source, size)| {
                if (*source..*source + size).contains(location) {
                    return Some(*dest + *location - *source);
                }

                None
            }) {
                *location = mapping;
            }
        }
    }

    Answer::Number(locations.iter().min().copied().unwrap() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(solve(input), Answer::Number(35));
    }
}
