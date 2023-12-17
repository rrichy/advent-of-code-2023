use std::cmp::Ordering;

use itertools::Itertools;

use crate::timed;

pub fn solve(input: String) {
    println!("Day Seven");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

const CARD_RANK: &str = "23456789TJQKA";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    Fullhouse,
    Four,
    Five,
}

impl HandType {
    fn get_hand_type(hand: &str) -> Self {
        let counts = hand.chars().counts();
        match counts.len() {
            1 => Self::Five,
            2 => {
                if counts.values().contains(&2) {
                    Self::Fullhouse
                } else {
                    Self::Four
                }
            }
            3 => {
                if counts.values().contains(&2) {
                    Self::TwoPair
                } else {
                    Self::Three
                }
            }
            4 => Self::OnePair,
            _ => Self::High,
        }
    }
}

#[derive(PartialEq, Eq)]
struct HandBid {
    hand: String,
    hand_type: HandType,
    bid: usize,
}

impl HandBid {
    fn new(line: &str) -> Self {
        let (hand, bid) = line.split_once(" ").unwrap();

        Self {
            hand: hand.to_string(),
            hand_type: HandType::get_hand_type(hand),
            bid: bid.parse().unwrap(),
        }
    }
}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (i, self_ch) in self.hand.char_indices() {
                    let other_ch = other.hand.chars().nth(i).unwrap();
                    if self_ch == other_ch {
                        continue;
                    }
                    let self_rank = CARD_RANK.find(self_ch).unwrap();
                    let other_rank = CARD_RANK.find(other_ch).unwrap();
                    return self_rank.cmp(&other_rank);
                }
                return Ordering::Equal;
            }
            ord => ord,
        }
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_one(input: String) -> usize {
    let mut hand_bids: Vec<HandBid> = input.lines().map(HandBid::new).collect();
    hand_bids.sort();

    hand_bids
        .iter()
        .enumerate()
        .fold(0, |sum, (index, hand_bid)| sum + (index + 1) * hand_bid.bid)
}

const CARD_RANK_OVERRULED: &str = "J23456789TQKA";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandTypeOverruled {
    High,
    OnePair,
    TwoPair,
    Three,
    Fullhouse,
    Four,
    Five,
}

impl HandTypeOverruled {
    fn get_hand_type(hand: &str) -> Self {
        let mut counts = hand.chars().counts();
        if let Some(_) = counts.get_mut(&'J') {
            if let Some((ch, _)) = counts
                .iter()
                .max_by_key(|&(ch, &c)| if ch.eq(&'J') { 0 } else { c })
            {
                let hand = hand.replace('J', &ch.to_string());
                counts = hand.chars().counts();
            }
        }

        match counts.len() {
            1 => Self::Five,
            2 => {
                if counts.values().contains(&2) {
                    Self::Fullhouse
                } else {
                    Self::Four
                }
            }
            3 => {
                if counts.values().contains(&2) {
                    Self::TwoPair
                } else {
                    Self::Three
                }
            }
            4 => Self::OnePair,
            _ => Self::High,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct HandBidOverruled {
    hand: String,
    hand_type: HandTypeOverruled,
    bid: usize,
}

impl HandBidOverruled {
    fn new(line: &str) -> Self {
        let (hand, bid) = line.split_once(" ").unwrap();

        Self {
            hand: hand.to_string(),
            hand_type: HandTypeOverruled::get_hand_type(hand),
            bid: bid.parse().unwrap(),
        }
    }
}

impl Ord for HandBidOverruled {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (i, self_ch) in self.hand.char_indices() {
                    let other_ch = other.hand.chars().nth(i).unwrap();
                    if self_ch == other_ch {
                        continue;
                    }
                    let self_rank = CARD_RANK_OVERRULED.find(self_ch).unwrap();
                    let other_rank = CARD_RANK_OVERRULED.find(other_ch).unwrap();
                    return self_rank.cmp(&other_rank);
                }
                return Ordering::Equal;
            }
            ord => ord,
        }
    }
}

impl PartialOrd for HandBidOverruled {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_two(input: String) -> usize {
    let mut hand_bids: Vec<HandBidOverruled> = input.lines().map(HandBidOverruled::new).collect();
    hand_bids.sort();

    hand_bids
        .iter()
        .enumerate()
        .fold(0, |sum, (index, hand_bid)| sum + (index + 1) * hand_bid.bid)
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(
            part_one(read_txt_file(7, crate::TextEnum::Input)),
            251545216
        );
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(7, crate::TextEnum::Input)), 250384185);
    }
}
