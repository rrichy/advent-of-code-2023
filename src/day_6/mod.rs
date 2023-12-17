use crate::timed;

pub fn solve(input: String) {
    println!("Day Six");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    fn count_ways_to_win(&self) -> usize {
        let mut win_count = 0;
        for charge_time in 0..=self.time {
            if self.distance < (self.time - charge_time) * charge_time {
                win_count += 1;
            }
        }
        win_count
    }
}

fn part_one(input: String) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();
    let time: Vec<usize> = time
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    let distance: Vec<usize> = distance
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();

    let mut result = 1;

    for (i, &t) in time.iter().enumerate() {
        let race = Race::new(t, distance[i]);
        result *= race.count_ways_to_win();
    }

    result
}

fn part_two(input: String) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();
    let time: usize = time
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance: usize = distance
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();

    let race = Race::new(time, distance);
    race.count_ways_to_win()
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(6, crate::TextEnum::Input)), 1710720);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(6, crate::TextEnum::Input)), 35349468);
    }
}
