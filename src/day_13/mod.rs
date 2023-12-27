use itertools::Itertools;

use crate::timed;

pub fn solve(input: String) {
    println!("Day 13");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn part_one(input: String) -> usize {
    let mut sum = 0;
    input.split("\n\n").for_each(|note| {
        let rows: Vec<String> = note.lines().map(|l| l.to_string()).collect();

        // check reflection horizontally
        for (up, down) in rows.iter().enumerate().tuple_windows() {
            // possible reflection point
            if is_reflection_point(&rows, up.0, down.0) {
                sum += 100 * down.0;
                return;
            }
        }

        // rotate right
        let mut cols = vec!["".to_string(); rows[0].len()];
        rows.iter().rev().for_each(|row| {
            row.chars().enumerate().for_each(|(i, ch)| {
                cols[i].push(ch);
            })
        });

        // check reflection horizontally
        for (up, down) in cols.iter().enumerate().tuple_windows() {
            // possible reflection point
            if is_reflection_point(&cols, up.0, down.0) {
                sum += down.0;
                return;
            }
        }
    });

    sum
}

fn is_reflection_point(collection: &Vec<String>, up: usize, down: usize) -> bool {
    let size_up = down;
    let size_down = collection.len() - down;
    let n_iteration = if size_up < size_down {
        size_up
    } else {
        size_down
    };

    for n in 0..n_iteration {
        let up_v = &collection[up - n];
        let down_v = &collection[down + n];

        if up_v != down_v {
            return false;
        }
    }

    true
}

fn is_equal(a: &String, b: &String, with_smudge: bool) -> (bool, bool) {
    if with_smudge {
        let mut smudge = 0;
        for (i, ch) in a.char_indices() {
            if ch != b.chars().nth(i).unwrap() {
                smudge += 1;
                if smudge > 1 {
                    return (false, false);
                }
            }
        }

        return (true, smudge == 1);
    }

    (a.eq(b), false)
}

fn is_start_of_reflection_point(collection: &Vec<String>, up: usize, down: usize) -> bool {
    let size_up = down;
    let size_down = collection.len() - down;
    let n_iteration = if size_up < size_down {
        size_up
    } else {
        size_down
    };

    let mut smudge = 0;
    for n in 0..n_iteration {
        let up_v = &collection[up - n];
        let down_v = &collection[down + n];

        let (equal, smudged) = is_equal(up_v, down_v, true);

        if smudged {
            smudge += 1;
        }

        if smudge > 1 || !equal {
            return false;
        }
    }

    smudge == 1
}

fn part_two(input: String) -> usize {
    let mut sum = 0;
    input.split("\n\n").for_each(|note| {
        let rows: Vec<String> = note.lines().map(|l| l.to_string()).collect();

        // check reflection horizontally
        for (up, down) in rows.iter().enumerate().tuple_windows() {
            // possible reflection point
            if is_start_of_reflection_point(&rows, up.0, down.0) {
                sum += 100 * down.0;
                return;
            }
        }

        // rotate right
        let mut cols = vec!["".to_string(); rows[0].len()];
        rows.iter().rev().for_each(|row| {
            row.chars().enumerate().for_each(|(i, ch)| {
                cols[i].push(ch);
            })
        });

        // check reflection horizontally
        for (up, down) in cols.iter().enumerate().tuple_windows() {
            // possible reflection point
            if is_start_of_reflection_point(&cols, up.0, down.0) {
                sum += down.0;
                return;
            }
        }
    });

    sum
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(13, crate::TextEnum::Input)), 37718);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(13, crate::TextEnum::Input)), 40995);
    }
}
