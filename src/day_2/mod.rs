use crate::timed;

pub fn solve(input: String) {
    println!("Day Two");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

struct GameSetup {
    red: u32,
    green: u32,
    blue: u32,
}

fn part_one(input: String) -> u32 {
    let setup = GameSetup {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum_ids: u32 = 0;

    input.lines().for_each(|line| {
        let (game_id, sets) = line.split_once(": ").unwrap();
        let game_id = game_id.split_once(" ").unwrap().1.parse::<u32>().unwrap();

        let sets: Vec<&str> = sets.split("; ").collect();

        for set in sets {
            let colors: Vec<&str> = set.split(", ").collect();

            for color in colors {
                let (count, color) = color.split_once(" ").unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if count > setup.red {
                            return;
                        }
                    }
                    "green" => {
                        if count > setup.green {
                            return;
                        }
                    }
                    "blue" => {
                        if count > setup.blue {
                            return;
                        }
                    }
                    _ => (),
                }
            }
        }

        sum_ids += game_id;
    });

    sum_ids
}

fn part_two(input: String) -> u32 {
    let mut sum: u32 = 0;

    input.lines().for_each(|line| {
        let sets: Vec<&str> = line.split_once(": ").unwrap().1.split("; ").collect();

        let mut setup = GameSetup {
            red: 0,
            green: 0,
            blue: 0,
        };
        for set in sets {
            let colors: Vec<&str> = set.split(", ").collect();

            for color in colors {
                let (count, color) = color.split_once(" ").unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if count > setup.red {
                            setup.red = count;
                        }
                    }
                    "green" => {
                        if count > setup.green {
                            setup.green = count;
                        }
                    }
                    "blue" => {
                        if count > setup.blue {
                            setup.blue = count;
                        }
                    }
                    _ => (),
                }
            }
        }

        sum += setup.red * setup.green * setup.blue;
    });

    sum
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(2, crate::TextEnum::Input)), 2105);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(2, crate::TextEnum::Input)), 72422);
    }
}
