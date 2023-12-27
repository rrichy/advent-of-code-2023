use regex::Regex;

use crate::timed;

pub fn solve(input: String) {
    println!("Day 12");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn part_one(input: String) -> usize {
    // let mut sum = 0;
    // input.lines().for_each(|l| {
    //     let (hay, pat) = l.split_once(' ').unwrap();
    //     let pats = pat.split(',');

    //     for (i, n) in pats.enumerate() {
    //         let pattern = String::new();

    //         pattern.extends(format!(r"(#|?){{}}", n));
    //         if i < pats.count() - 1 {
    //             pattern.extend(r"\.+");
    //         }

    //         let re = Regex::new()
    //     }
    // })
    0
}

fn part_two(input: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        //        assert_eq!(part_one(read_txt_file(12, crate::TextEnum::Input)), 11911);
    }

    #[test]
    fn part_two_test() {
        //        assert_eq!(
        //            part_two(read_txt_file(12, crate::TextEnum::Input)),
        //            10151663816849
        //        );
    }
}
