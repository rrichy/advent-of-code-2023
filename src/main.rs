use std::{env, process};

use advent_of_code_2023::{
    day_1,
    day_2,
    day_3,
    // day_10, day_11, day_4, day_5, day_6, day_7, day_8, day_9,
    Config,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let day = config.day.value();

    println!("Advent of Code 2022 - Day {}", day);

    match day {
        1 => day_1::solve(),
        2 => day_2::solve(),
        3 => day_3::solve(),
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
