use std::collections::HashMap;

use crate::answer::Answer;

pub fn solve(input: &str) -> Answer {
    let mut iter = input.lines();
    let steps = iter.next().unwrap();
    let nodes = iter.skip(1).fold(HashMap::new(), |mut map, line| {
        let mut iter = line.splitn(2, " = ");
        let key = iter.next().unwrap();
        let tuple_string = &iter.next().unwrap();
        let values = &tuple_string[1..tuple_string.len() - 1]
            .split(", ")
            .collect::<Vec<&str>>();

        map.insert(key, (values[0], values[1]));
        map
    });

    let mut i = 0_usize;
    let mut node_name = "AAA";
    while node_name != "ZZZ" {
        let node = nodes.get(&node_name).unwrap();

        let step = steps.as_bytes()[i % steps.len()] as char;
        node_name = match step {
            'L' => node.0,
            'R' => node.1,

            _ => unreachable!(),
        };

        i += 1;
    }

    Answer::Number(i as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(solve(input), Answer::Number(2));

        let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(solve(input), Answer::Number(6));
    }
}
