use crate::answer::Answer;

#[derive(Clone)]
struct Range(usize, usize);

#[derive(Debug, PartialEq)]
struct RangeMap(usize, usize, usize);

impl Range {
    fn get_overlap(&self, other: &Range) -> Option<Range> {
        let begin = self.0.max(other.0);
        let end = self.1.min(other.1);

        if begin >= end {
            return None;
        }

        Some(Range(begin, end))
    }

    fn map(&self, mappings: &[RangeMap]) -> Vec<RangeMap> {
        let mut range = self.clone();
        let results = match mappings
            .iter()
            .try_fold(Vec::new(), |mut results, mapping| {
                let source = Range(mapping.0, mapping.0 + mapping.2);
                let dest = Range(mapping.1, mapping.1 + mapping.2);
                let overlap = self.get_overlap(&source);
                if let Some(overlap) = overlap {
                    if overlap.0 > range.0 {
                        results.push(RangeMap(range.0, range.0, overlap.0 - range.0));
                    }

                    let overlap_size = overlap.1 - overlap.0;
                    results.push(RangeMap(
                        overlap.0,
                        overlap.0 - source.0 + dest.0,
                        overlap_size,
                    ));

                    if overlap.1 <= range.1 {
                        range = Range(overlap.1, range.1);
                    }
                }

                if range.0 >= range.1 {
                    return Err(results);
                }

                Ok(results)
            }) {
            Ok(mut results) => {
                results.push(RangeMap(range.0, range.0, range.1 - range.0));
                results
            }
            Err(results) => results,
        };

        results
    }
}

pub fn solve(input: &str) -> Answer {
    let mut iter = input.lines();
    let ranges = iter
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect::<Vec<usize>>();
    let seeds = ranges.chunks(2).fold(Vec::new(), |mut seeds, chunk| {
        seeds.push(Range(chunk[0], chunk[0] + chunk[1]));
        seeds
    });
    iter.next();

    let mut ranges = seeds.clone();
    while iter.next().is_some() {
        let mut mappings = iter.by_ref().take_while(|line| !line.is_empty()).fold(
            Vec::new(),
            |mut mappings, line| {
                let parts = line
                    .split_ascii_whitespace()
                    .map(|part| part.parse().unwrap())
                    .collect::<Vec<usize>>();

                mappings.push(RangeMap(parts[1], parts[0], parts[2]));
                mappings
            },
        );
        mappings.sort_by(|a, b| a.0.cmp(&b.0));
        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        let range_maps = ranges.iter().fold(Vec::new(), |mut ranges, range| {
            ranges.extend(range.map(&mappings));

            ranges
        });

        ranges = range_maps
            .iter()
            .map(|map| Range(map.1, map.1 + map.2))
            .collect();
    }

    let answer = ranges.iter().map(|range| range.0).min().unwrap() as u32;

    Answer::Number(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_map() {
        let range = Range(10, 30);

        let mappings = vec![RangeMap(15, 30, 5), RangeMap(20, 50, 6)];
        assert_eq!(
            range.map(&mappings),
            vec![
                RangeMap(10, 10, 5),
                RangeMap(15, 30, 5),
                RangeMap(20, 50, 6),
                RangeMap(26, 26, 4),
            ]
        );

        let mappings = vec![RangeMap(15, 30, 5), RangeMap(20, 50, 20)];
        assert_eq!(
            range.map(&mappings),
            vec![
                RangeMap(10, 10, 5),
                RangeMap(15, 30, 5),
                RangeMap(20, 50, 10),
            ]
        );
    }

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

        assert_eq!(solve(input), Answer::Number(46));
    }
}
