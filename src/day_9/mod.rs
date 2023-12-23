use itertools::Itertools;

use crate::timed;

pub fn solve(input: String) {
    println!("Day Nine");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn part_one(input: String) -> i64 {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let histories: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let a = next_sequence_item(&histories);
        sum += a;
    });

    sum
}

fn next_sequence_item(v: &Vec<i64>) -> i64 {
    if v.iter().all(|&i| i == 0) {
        return 0;
    }

    let mut next_sequence = vec![];

    for (a, b) in v.iter().tuple_windows() {
        next_sequence.push(b - a);
    }

    v.last().unwrap() + next_sequence_item(&next_sequence)
}

fn part_two(input: String) -> i64 {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let histories: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let a = prev_sequence_item(&histories);
        sum += a;
    });

    sum
}

fn prev_sequence_item(v: &Vec<i64>) -> i64 {
    if v.iter().all(|&i| i == 0) {
        return 0;
    }

    let mut next_sequence = vec![];

    for (a, b) in v.iter().tuple_windows() {
        next_sequence.push(b - a);
    }

    v.first().unwrap() - prev_sequence_item(&next_sequence)
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(
            part_one(read_txt_file(9, crate::TextEnum::Input)),
            1992273652
        );
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(9, crate::TextEnum::Input)), 1012);
    }
}
