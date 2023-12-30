use std::thread::current;

use itertools::Itertools;

use crate::timed;

pub fn solve(input: String) {
    println!("Day 14");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

#[derive(PartialEq, Clone)]
enum Rock {
    Round,
    Cube,
    Ground,
}

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone)]
struct ParabolicDish {
    dish: Vec<String>,
}

impl ParabolicDish {
    fn new(input: String) -> Self {
        let dish = input.lines().map(|line| line.to_string()).collect();

        Self { dish }
    }

    fn roll(&mut self, direction: Direction) -> () {
        let width: usize = self.dish[0].len();
        if direction == Direction::North {
            let mut ground_col_indexes: Vec<Option<usize>> = vec![None; width];

            for row_index in 0..self.dish.len() {
                for (col_index, ch) in self.dish[row_index].clone().char_indices() {
                    match ch {
                        '.' => {
                            if ground_col_indexes[col_index].is_none() {
                                ground_col_indexes[col_index] = Some(row_index);
                            }
                        }
                        'O' => {
                            if let Some(i) = ground_col_indexes[col_index] {
                                let mut current_row = self.dish[row_index].clone();
                                let mut prev_row = self.dish[i].clone();

                                current_row.replace_range(col_index..(col_index + 1), ".");
                                prev_row.replace_range(col_index..(col_index + 1), "O");

                                *self.dish.get_mut(i).unwrap() = prev_row.clone();
                                *self.dish.get_mut(row_index).unwrap() = current_row.clone();

                                ground_col_indexes[col_index] = Some(i + 1);
                            } else {
                                ground_col_indexes[col_index] = None;
                            }
                        }
                        _ => {
                            ground_col_indexes[col_index] = None;
                        }
                    }
                }
            }
        }

        if direction == Direction::South {
            let mut ground_col_indexes: Vec<Option<usize>> = vec![None; width];

            for row_index in (0..=(self.dish.len() - 1)).rev() {
                for (col_index, ch) in self.dish[row_index].clone().char_indices() {
                    match ch {
                        '.' => {
                            if ground_col_indexes[col_index].is_none() {
                                ground_col_indexes[col_index] = Some(row_index);
                            }
                        }
                        'O' => {
                            if let Some(i) = ground_col_indexes[col_index] {
                                let mut current_row = self.dish[row_index].clone();
                                let mut prev_row = self.dish[i].clone();

                                current_row.replace_range(col_index..(col_index + 1), ".");
                                prev_row.replace_range(col_index..(col_index + 1), "O");

                                *self.dish.get_mut(i).unwrap() = prev_row.clone();
                                *self.dish.get_mut(row_index).unwrap() = current_row.clone();

                                ground_col_indexes[col_index] = Some(i + 1);
                            } else {
                                ground_col_indexes[col_index] = None;
                            }
                        }
                        _ => {
                            ground_col_indexes[col_index] = None;
                        }
                    }
                }
            }
        }
    }

    fn get_load(&self) -> usize {
        let height = self.dish.len();

        self.dish
            .iter()
            .enumerate()
            .fold(0, |load_sum, (index, current)| {
                load_sum
                    + (height - index)
                        * current.chars().fold(0, |round_count, rock| {
                            if rock == 'O' {
                                return round_count + 1;
                            }

                            round_count
                        })
            })
    }
}

fn part_one(input: String) -> usize {
    let mut dish = ParabolicDish::new(input);

    dish.roll(Direction::North);

    dish.get_load()
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
        assert_eq!(part_one(read_txt_file(14, crate::TextEnum::Input)), 108792);
    }

    #[test]
    fn part_two_test() {
        //        assert_eq!(
        //            part_two(read_txt_file(14, crate::TextEnum::Input)),
        //            10151663816849
        //        );
    }
}
