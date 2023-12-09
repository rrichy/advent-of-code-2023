use regex::Regex;

use crate::timed;

pub fn solve(input: String) {
    println!("Day One");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn part_one(input: String) -> u32 {
    let mut result = 0;
    input.lines().for_each(|line| {
        let mut calibration_value = "".to_string();
        line.chars().for_each(|c| {
            if c.is_numeric() {
                if calibration_value.len() == 2 {
                    calibration_value.pop();
                }
                calibration_value.push(c);
            }
        });

        if calibration_value.len() == 1 {
            calibration_value.push(calibration_value.chars().next().unwrap());
        }

        let calibration_value: u32 = calibration_value.parse::<u32>().unwrap();
        result += calibration_value;
    });

    result
}

fn part_two(input: String) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut calibration_value = "".to_string();
        let rev = line.chars().rev().collect::<String>();

        let first = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine")
            .unwrap()
            .find(line)
            .unwrap()
            .as_str();
        let last = Regex::new(r"[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin")
            .unwrap()
            .find(&rev)
            .unwrap()
            .as_str();

        match first {
            "one" => calibration_value.push('1'),
            "two" => calibration_value.push('2'),
            "three" => calibration_value.push('3'),
            "four" => calibration_value.push('4'),
            "five" => calibration_value.push('5'),
            "six" => calibration_value.push('6'),
            "seven" => calibration_value.push('7'),
            "eight" => calibration_value.push('8'),
            "nine" => calibration_value.push('9'),
            x => calibration_value.push(x.chars().next().unwrap()),
        }

        match last {
            "eno" => calibration_value.push('1'),
            "owt" => calibration_value.push('2'),
            "eerht" => calibration_value.push('3'),
            "ruof" => calibration_value.push('4'),
            "evif" => calibration_value.push('5'),
            "xis" => calibration_value.push('6'),
            "neves" => calibration_value.push('7'),
            "thgie" => calibration_value.push('8'),
            "enin" => calibration_value.push('9'),
            x => calibration_value.push(x.chars().next().unwrap()),
        }

        let calibration_value: u32 = String::from_iter(vec![
            calibration_value.chars().nth(0).unwrap(),
            calibration_value.chars().last().unwrap(),
        ])
        .parse::<u32>()
        .unwrap();
        result += calibration_value;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(1, crate::TextEnum::Input)), 55816);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(1, crate::TextEnum::Input)), 54980);
    }
}
