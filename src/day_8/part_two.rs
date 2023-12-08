use std::collections::HashMap;

use crate::answer::Answer;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn traverse_nodes(node: &str, nodes: &HashMap<&str, (&str, &str)>, steps: &str) -> usize {
    let mut i = 0_usize;
    let mut node_name = node;
    while !node_name.ends_with('Z') {
        let node = nodes.get(&node_name).unwrap();

        let step = steps.as_bytes()[i % steps.len()] as char;
        node_name = match step {
            'L' => node.0,
            'R' => node.1,

            _ => unreachable!(),
        };

        i += 1;
    }

    i
}

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

    let count = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .fold(1, |count, k| lcm(count, traverse_nodes(k, &nodes, steps)));

    Answer::LargeNumber(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(solve(input), Answer::Number(6));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(100, 50), 100);
        assert_eq!(lcm(50, 100), 100);
        assert_eq!(lcm(7, 50), 350);
        assert_eq!(lcm(11567, 19099), 821257);
    }
}
