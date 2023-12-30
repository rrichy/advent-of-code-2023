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
    loads: Vec<usize>,
}

trait DishSort {
    fn sort_asc(&self) -> Self;
    fn sort_desc(&self) -> Self;
}

impl DishSort for String {
    /**
     * Sorts a line of a dish in ascending order
     * Ex. O..O#..OO.O -> ..OO#...OOO
     */
    fn sort_asc(&self) -> Self {
        self.split('#')
            .map(|segment| segment.chars().sorted().collect::<String>())
            .join("#")
    }

    /**
     * Sorts a line of a dish in ascending order
     * Ex. O..O#..OO.O -> OO..#OOO...
     */
    fn sort_desc(&self) -> Self {
        self.split('#')
            .map(|segment| segment.chars().sorted().rev().collect::<String>())
            .join("#")
    }
}

impl ParabolicDish {
    fn new(input: String) -> Self {
        let dish = input.lines().map(|line| line.to_string()).collect();

        Self {
            dish,
            loads: vec![],
        }
    }

    fn roll(&mut self, direction: Direction) -> () {
        let width: usize = self.dish[0].len();
        if direction == Direction::North {
            for col_index in 0..width {
                let col = self
                    .dish
                    .iter()
                    .map(|row| row.chars().nth(col_index).unwrap())
                    .collect::<String>()
                    .sort_desc();

                for (row_index, row) in self.dish.iter_mut().enumerate() {
                    row.replace_range(col_index..(col_index + 1), &col[row_index..(row_index + 1)]);
                }
            }

            // This is faster
            // let mut ground_indexes: Vec<Option<usize>> = vec![None; width];

            // for row_index in 0..self.dish.len() {
            //     for (col_index, ch) in self.dish[row_index].clone().char_indices() {
            //         match ch {
            //             '.' => {
            //                 if ground_indexes[col_index].is_none() {
            //                     ground_indexes[col_index] = Some(row_index);
            //                 }
            //             }
            //             'O' => {
            //                 if let Some(i) = ground_indexes[col_index] {
            //                     let mut current_row = self.dish[row_index].clone();
            //                     let mut prev_row = self.dish[i].clone();

            //                     current_row.replace_range(col_index..(col_index + 1), ".");
            //                     prev_row.replace_range(col_index..(col_index + 1), "O");

            //                     *self.dish.get_mut(i).unwrap() = prev_row.clone();
            //                     *self.dish.get_mut(row_index).unwrap() = current_row.clone();

            //                     ground_indexes[col_index] = Some(i + 1);
            //                 } else {
            //                     ground_indexes[col_index] = None;
            //                 }
            //             }
            //             _ => {
            //                 ground_indexes[col_index] = None;
            //             }
            //         }
            //     }
            // }
        }

        if direction == Direction::South {
            for col_index in 0..width {
                let col = self
                    .dish
                    .iter()
                    .map(|row| row.chars().nth(col_index).unwrap())
                    .collect::<String>()
                    .sort_asc();

                for (row_index, row) in self.dish.iter_mut().enumerate() {
                    row.replace_range(col_index..(col_index + 1), &col[row_index..(row_index + 1)]);
                }
            }
        }

        if direction == Direction::West {
            for row in self.dish.iter_mut() {
                *row = row.sort_asc()
            }
        }

        if direction == Direction::East {
            for row in self.dish.iter_mut() {
                *row = row.sort_desc()
            }
        }

        // self.loads.push(value)
        // println!("{:#?}", self.dish);
    }

    fn cycle(&mut self) -> () {
        self.roll(Direction::North);
        self.roll(Direction::East);
        self.roll(Direction::South);
        self.roll(Direction::West);

        self.insert_load();
    }

    fn insert_load(&mut self) -> () {
        let height = self.dish.len();

        let load = self
            .dish
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
            });

        self.loads.push(load);
    }
}

fn part_one(input: String) -> usize {
    let mut dish = ParabolicDish::new(input);

    dish.roll(Direction::North);
    dish.insert_load();

    *dish.loads.last().unwrap()
}

fn part_two(input: String) -> usize {
    let mut dish = ParabolicDish::new(input);

    let mut cycle: Option<Vec<usize>> = None;
    'c: loop {
        // populate the list
        dish.cycle();

        // with an increasing cycle_size, chunk the list with the size of cycle_size
        'cur_cycle: for cycle_size in 5..(dish.loads.len() / 2) {
            // window the chunked list and compare if equal
            for (left, right) in dish.loads.chunks(cycle_size).tuple_windows() {
                if left.len() != cycle_size || right.len() != cycle_size {
                    continue;
                }

                // if the left and right of a windowed list is equal and so is the rest, cycle has been started
                if left == right {
                    if cycle.is_none() {
                        println!("here");
                        cycle = Some(left.to_vec());
                    }
                } else {
                    if cycle.is_some() {
                        cycle = None;
                        break 'cur_cycle;
                    }
                }
            }

            if cycle.is_some() {
                break 'c;
            }
        }

        if cycle.is_some() {
            break;
        }
    }

    let cycle = cycle.unwrap();
    let cycle_start_index = dish
        .loads
        .windows(cycle.len())
        .enumerate()
        .find(|a| a.1 == cycle)
        .unwrap()
        .0;

    cycle[(1_000_000_000 - cycle_start_index - 1) % cycle.len()]
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
        assert_eq!(part_two(read_txt_file(14, crate::TextEnum::Input)), 99118);
    }
}
