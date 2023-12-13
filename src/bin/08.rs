use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(8);

struct Node {
    key: String,
    left: String,
    right: String,
}

fn parse_node(node: &str) -> Node {
    let re = Regex::new(r"\b([0-9A-Z]+)\s*=\s*\(([0-9A-Z]+),\s*([0-9A-Z]+)\)").unwrap();

    re.captures(node)
        .map(|caps| Node {
            key: caps
                .get(1)
                .map_or(String::new(), |m| m.as_str().to_string()),
            left: caps
                .get(2)
                .map_or(String::new(), |m| m.as_str().to_string()),
            right: caps
                .get(3)
                .map_or(String::new(), |m| m.as_str().to_string()),
        })
        .unwrap()
}

fn count_steps_p1(
    pattern: &Vec<char>,
    nodes: &HashMap<String, Node>,
    starting_pos: &str,
    terminating_pos: &str,
) -> Option<u64> {
    let mut pos = starting_pos.to_string();
    for (i, side) in pattern.iter().cycle().enumerate() {
        pos = match side {
            'L' => nodes.get(&pos).unwrap().left.clone(),
            'R' => nodes.get(&pos).unwrap().right.clone(),
            _ => panic!(),
        };

        if pos.ends_with(terminating_pos) {
            return Some(i as u64 + 1);
        }
    }
    return None;
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: Vec<u64>) -> u64 {
    numbers.into_iter().fold(1, |acc, num| lcm(acc, num))
}

fn count_steps_p2(pattern: &Vec<char>, nodes: &HashMap<String, Node>) -> Option<u64> {
    let mut pos: Vec<&String> = nodes.keys().filter(|k| k.ends_with("A")).sorted().collect();

    let repetition_periods: Vec<u64> = pos
        .iter()
        .map(|&p| count_steps_p1(pattern, nodes, p, "Z").unwrap())
        .collect();

    // lowest common multiple
    Some(lcm_of_vec(repetition_periods))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (pattern, nodes) = input
        .trim()
        .split_once("\n\n")
        .map(|(pattern, nodes)| {
            let pattern = pattern.chars().collect::<Vec<char>>();
            let nodes: HashMap<String, Node> = nodes
                .split('\n')
                .map(|node| parse_node(node))
                .map(|node| (node.key.clone(), node))
                .collect();

            (pattern, nodes)
        })
        .unwrap();

    count_steps_p1(&pattern, &nodes, "AAA", "ZZZ")
}

pub fn part_two(input: &str) -> Option<u64> {
    let (pattern, nodes) = input
        .trim()
        .split_once("\n\n")
        .map(|(pattern, nodes)| {
            let pattern = pattern.chars().collect::<Vec<char>>();
            let nodes: HashMap<String, Node> = nodes
                .split('\n')
                .map(|node| parse_node(node))
                .map(|node| (node.key.clone(), node))
                .collect();

            (pattern, nodes)
        })
        .unwrap();

    count_steps_p2(&pattern, &nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
