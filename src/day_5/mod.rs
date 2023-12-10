use std::ops::Range;

use itertools::Itertools;

use crate::timed;

pub fn solve(input: String) {
    println!("Day Five");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

#[derive(Debug)]
struct Map(usize, usize, usize);

impl Map {
    fn in_range(&self, seed: usize) -> bool {
        (self.0..self.0 + self.2).contains(&seed)
    }

    fn to_map(&self, seed: usize) -> usize {
        let diff = seed - self.0;
        self.1 + diff
    }
}

#[derive(Debug)]
struct Almanac {
    seed_to_soil: Vec<Map>,
    soil_to_fert: Vec<Map>,
    fert_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temp: Vec<Map>,
    temp_to_hum: Vec<Map>,
    hum_to_loc: Vec<Map>,
}

impl Almanac {
    fn seed_to_loc(&self, seed: usize) -> usize {
        let soil = self
            .seed_to_soil
            .iter()
            .find(|&m| m.in_range(seed))
            .map_or(seed, |m| m.to_map(seed));

        let fert = self
            .soil_to_fert
            .iter()
            .find(|&m| m.in_range(soil))
            .map_or(soil, |m| m.to_map(soil));

        let water = self
            .fert_to_water
            .iter()
            .find(|&m| m.in_range(fert))
            .map_or(fert, |m| m.to_map(fert));

        let light = self
            .water_to_light
            .iter()
            .find(|&m| m.in_range(water))
            .map_or(water, |m| m.to_map(water));

        let temp = self
            .light_to_temp
            .iter()
            .find(|&m| m.in_range(light))
            .map_or(light, |m| m.to_map(light));

        let hum = self
            .temp_to_hum
            .iter()
            .find(|&m| m.in_range(temp))
            .map_or(temp, |m| m.to_map(temp));

        let loc = self
            .hum_to_loc
            .iter()
            .find(|&m| m.in_range(hum))
            .map_or(hum, |m| m.to_map(hum));

        // println!("seed {seed}, soil {soil}, fert {fert}, water {water}, light {light}, temp {temp}, hum {hum}, loc {loc}");

        loc
    }
}

fn part_one(input: String) -> usize {
    let mut almanac = Almanac {
        seed_to_soil: vec![],
        soil_to_fert: vec![],
        fert_to_water: vec![],
        water_to_light: vec![],
        light_to_temp: vec![],
        temp_to_hum: vec![],
        hum_to_loc: vec![],
    };

    let seeds: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut current_map: &str = "";
    input.lines().skip(2).for_each(|line| {
        if line.contains("map:") {
            current_map = line;
        } else if line.chars().next().unwrap_or(' ').is_numeric() {
            let (dest, src, range): (usize, usize, usize) = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();

            let map = Map(src, dest, range);

            match current_map {
                "seed-to-soil map:" => almanac.seed_to_soil.push(map),
                "soil-to-fertilizer map:" => almanac.soil_to_fert.push(map),
                "fertilizer-to-water map:" => almanac.fert_to_water.push(map),
                "water-to-light map:" => almanac.water_to_light.push(map),
                "light-to-temperature map:" => almanac.light_to_temp.push(map),
                "temperature-to-humidity map:" => almanac.temp_to_hum.push(map),
                "humidity-to-location map:" => almanac.hum_to_loc.push(map),
                _ => (),
            }
        }
    });

    let mut min_loc = usize::MAX;

    seeds.iter().for_each(|&seed| {
        let loc = almanac.seed_to_loc(seed);
        if loc < min_loc {
            min_loc = loc;
        }
    });

    min_loc
}

fn part_two(input: String) -> usize {
    let mut almanac = Almanac {
        seed_to_soil: vec![],
        soil_to_fert: vec![],
        fert_to_water: vec![],
        water_to_light: vec![],
        light_to_temp: vec![],
        temp_to_hum: vec![],
        hum_to_loc: vec![],
    };

    let seeds: Vec<Range<usize>> = input
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec()
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect_vec();

    let mut current_map: &str = "";
    input.lines().skip(2).for_each(|line| {
        if line.contains("map:") {
            current_map = line;
        } else if line.chars().next().unwrap_or(' ').is_numeric() {
            let (dest, src, range): (usize, usize, usize) = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();

            let map = Map(src, dest, range);

            match current_map {
                "seed-to-soil map:" => almanac.seed_to_soil.push(map),
                "soil-to-fertilizer map:" => almanac.soil_to_fert.push(map),
                "fertilizer-to-water map:" => almanac.fert_to_water.push(map),
                "water-to-light map:" => almanac.water_to_light.push(map),
                "light-to-temperature map:" => almanac.light_to_temp.push(map),
                "temperature-to-humidity map:" => almanac.temp_to_hum.push(map),
                "humidity-to-location map:" => almanac.hum_to_loc.push(map),
                _ => (),
            }
        }
    });

    let mut min_loc = usize::MAX;

    seeds.iter().for_each(|seed_range| {
        for seed in seed_range.clone().into_iter() {
            let loc = almanac.seed_to_loc(seed);
            if loc < min_loc {
                min_loc = loc;
            }
        }
    });

    min_loc
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(
            part_one(read_txt_file(5, crate::TextEnum::Input)),
            218513636
        );
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(5, crate::TextEnum::Input)), 19499881);
    }
}
