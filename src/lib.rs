use std::time::Instant;
use std::{env::current_dir, fs, io, process};

#[derive(Clone, Copy)]
pub enum TextEnum {
    Sample,
    SampleTwo,
    Input,
}

pub struct Day {
    value: u32,
}

impl Day {
    pub fn build(value: u32) -> Result<Day, &'static str> {
        if value < 1 || value > 31 {
            return Err("Input a valid day. There are only 31 days in December!");
        }

        Ok(Day { value })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

pub struct Config {
    pub day: Day,
    pub text: TextEnum,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let value: u32;
        let text: TextEnum;

        if args.len() < 2 {
            println!("Not enough arguments!");
            value = loop {
                println!("Enter which day to solve:");

                let mut day = String::new();

                io::stdin()
                    .read_line(&mut day)
                    .expect("Failed to read line");

                match day.trim().parse::<u32>() {
                    Ok(num) => break num,
                    Err(_) => continue,
                }
            };

            text = loop {
                println!(
                    "Enter which puzzle input to use. [input, sample, sample1]. (default: input)"
                );

                let mut text = String::new();

                io::stdin()
                    .read_line(&mut text)
                    .expect("Failed to read line");

                match text.trim() {
                    "sample" => break TextEnum::Sample,
                    "sample2" => break TextEnum::SampleTwo,
                    "input" => break TextEnum::Input,
                    _ => {
                        println!("Invalid text argument was supplied, using the default input");
                        break TextEnum::Input;
                    }
                }
            };

            println!("Using the default input.txt");
        } else {
            value = args[1].parse().expect("Expected an integer");
            if let Some(t) = args.get(2) {
                text = match t.parse::<String>().unwrap_or_default().as_str() {
                    "sample" => TextEnum::Sample,
                    "sample2" => TextEnum::SampleTwo,
                    "input" => TextEnum::Input,
                    _ => {
                        println!("Invalid text argument was supplied, using the default input.txt");
                        TextEnum::Input
                    }
                }
            } else {
                println!("No text argument was supplied, using the default input.txt");
                text = TextEnum::Input;
            }
        }

        let day = Day::build(value).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1);
        });

        Ok(Config { day, text })
    }

    pub fn solve(&self) -> () {
        let day = self.day.value;
        println!("Advent of Code 2023 - Day {}", day);
        let input = read_txt_file(day, self.text);
        match day {
            1 => day_1::solve(input),
            2 => day_2::solve(input),
            3 => day_3::solve(input),
            // 4 => day_4::solve(),
            // 5 => day_5::solve(),
            // 6 => day_6::solve(),
            // 7 => day_7::solve(),
            // 8 => day_8::solve(),
            // 9 => day_9::solve(),
            // 10 => day_10::solve(),
            // 11 => day_11::solve(),
            _ => panic!("Day {} has not yet been solved.", day),
        }
    }
}

pub fn read_txt_file(day: u32, filetype: TextEnum) -> String {
    let cwd = current_dir().expect("Failed to get the current working directory");

    let file = match filetype {
        TextEnum::Sample => "sample.txt",
        TextEnum::SampleTwo => "sample2.txt",
        TextEnum::Input => "input.txt",
    };

    fs::read_to_string(cwd.join(format!("src/day_{}/{}", day, file)))
        .expect("sample.txt does not exists!")
}

pub fn timed<F, R>(func: F)
where
    F: FnOnce() -> R,
    R: std::fmt::Display,
{
    let start = Instant::now();
    let result = func();

    println!("Result: {}, solved in: {:?}", result, start.elapsed());
}

mod day_1;
// pub mod day_10;
// pub mod day_11;
mod day_2;
mod day_3;
// pub mod day_4;
// pub mod day_5;
// pub mod day_6;
// pub mod day_7;
// pub mod day_8;
// pub mod day_9;
