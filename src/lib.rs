use std::time::Instant;
use std::{env::current_dir, fs, io, process};

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
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let value: u32;

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
        } else {
            value = args[1].parse().expect("Expected an integer");
        }

        let day = Day::build(value).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1);
        });

        Ok(Config { day })
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

pub mod day_1;
// pub mod day_10;
// pub mod day_11;
pub mod day_2;
pub mod day_3;
// pub mod day_4;
// pub mod day_5;
// pub mod day_6;
// pub mod day_7;
// pub mod day_8;
// pub mod day_9;
