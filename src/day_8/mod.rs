use std::collections::HashMap;

use crate::timed;

pub fn solve(input: String) {
    println!("Day Eight");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn part_one(input: String) -> usize {
    let mut steps = 0;
    let mut node_cache = HashMap::new();

    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    nodes.lines().for_each(|line| {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        node_cache.insert(key, (left, right));
    });

    let mut current = "AAA";
    let mut index = 0;
    while current.ne("ZZZ") {
        if instructions.chars().nth(index).unwrap() == 'L' {
            current = node_cache.get(current).unwrap().0;
        } else {
            current = node_cache.get(current).unwrap().1;
        }
        steps += 1;
        index = steps % instructions.chars().count();
    }

    steps
}

fn part_two(input: String) -> usize {
    let mut steps = 0;
    let mut node_cache = HashMap::new();
    let mut current = vec![];

    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    nodes.lines().for_each(|line| {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        if key.ends_with("A") {
            current.push((key, 0));
        }

        node_cache.insert(key, (left, right));
    });

    let mut index = 0;
    while current.iter().any(|&(_, v)| v == 0) {
        steps += 1;

        for node in current.iter_mut() {
            if node.1 != 0 {
                continue;
            }

            node.0 = {
                if instructions.chars().nth(index).unwrap() == 'L' {
                    node_cache.get(node.0).unwrap().0
                } else {
                    node_cache.get(node.0).unwrap().1
                }
            };

            if node.0.ends_with("Z") {
                node.1 = steps;
            }
        }

        index = steps % instructions.chars().count();
    }

    current
        .iter()
        .fold(1, |acc, &(_, v)| (acc * v) / gcd(acc, v))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(8, crate::TextEnum::Input)), 11911);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(
            part_two(read_txt_file(8, crate::TextEnum::Input)),
            10151663816849
        );
    }
}
