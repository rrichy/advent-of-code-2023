use std::time::Instant;

use regex::Regex;

use crate::read_txt_file;

pub fn solve() {
    println!("Day One");

    part_one();
    part_two();
}

fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(1, crate::TextEnum::Input);

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

    println!("Result: {:?}", result);

    println!("Solved in: {:?}", start.elapsed());
    0
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(1, crate::TextEnum::Input);

    let mut result = 0;

    for line in input.lines() {
        let mut calibration_value = "".to_string();
        // let mut normalized = String::from("");
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

    println!("Result: {:?}", result);

    println!("Solved in: {:?}", start.elapsed());
    0
}
