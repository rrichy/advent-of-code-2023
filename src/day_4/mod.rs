use std::collections::HashMap;

use regex::Regex;

use crate::timed;

pub fn solve(input: String) {
    println!("Day Four");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn part_one(input: String) -> i32 {
    let mut total_points = 0;
    input.lines().for_each(|line| {
        let (winning, hand) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let mut card_points = 0;

        winning.split_whitespace().for_each(|w| {
            let pattern = Regex::new(&format!(r"\b{}\b", w)).unwrap();

            if let Some(_) = pattern.find(hand) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        });

        total_points += card_points;
    });

    total_points
}

fn part_two(input: String) -> i32 {
    let mut total_scratchcards = 0;
    let mut copies_map = HashMap::new();
    input.lines().enumerate().for_each(|(line_index, line)| {
        let card_no = line_index + 1;
        let total_current_card = 1 + copies_map.get(&card_no).unwrap_or(&0);
        total_scratchcards += total_current_card;

        let (winning, hand) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let mut card_matches = 0;

        winning.split_whitespace().for_each(|w| {
            let pattern = Regex::new(&format!(r"\b{}\b", w)).unwrap();

            if let Some(_) = pattern.find(hand) {
                card_matches += 1;
                copies_map
                    .entry(&card_no + card_matches)
                    .and_modify(|copies| *copies += total_current_card)
                    .or_insert(total_current_card);
            }
        });
    });

    total_scratchcards
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(4, crate::TextEnum::Input)), 25651);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(4, crate::TextEnum::Input)), 19499881);
    }
}
