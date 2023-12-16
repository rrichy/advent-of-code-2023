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

#[derive(Debug, Clone)]
struct Bound {
    source: Range<usize>,
    destination: Range<usize>,
}

impl Bound {
    fn src_to_dest(&self, src: usize) -> Option<usize> {
        if self.source.contains(&src) {
            let diff = src - self.source.start;
            return Some(self.destination.start + diff);
        }

        None
    }

    fn dest_to_src(&self, dest: usize) -> Option<usize> {
        if self.destination.contains(&dest) {
            let diff = dest - self.destination.start;
            return Some(self.source.start + diff);
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Mapper {
    src: usize,
    dest: usize,
    length: usize,
}

impl Mapper {
    fn src_contains(&self, seed: usize) -> bool {
        self.src <= seed && self.last_src() > seed
    }

    fn dest_contains(&self, seed: usize) -> bool {
        self.dest <= seed && self.last_dest() > seed
    }

    fn src_to_dest(&self, src: usize) -> Option<usize> {
        if true {
            let diff = src - self.src;
            return Some(self.dest + diff);
        }

        None
    }

    fn dest_to_src(&self, dest: usize) -> Option<usize> {
        if self.dest_contains(dest) {
            let diff = dest - self.dest;
            return Some(self.src + diff);
        }

        None
    }

    fn last_src(&self) -> usize {
        self.src + self.length - 1
    }

    fn last_dest(&self) -> usize {
        self.dest + self.length - 1
    }
}

struct Almanac2 {
    funcs: Vec<Vec<Mapper>>,
    // composite: Vec<Bound>,
}

impl Almanac2 {
    fn build(maps: String) -> Self {
        let mut funcs = vec![];
        maps.split("\n\n").for_each(|map| {
            let mut _func = vec![];
            map.lines().skip(1).for_each(|piece| {
                let (dest, src, length): (usize, usize, usize) = piece
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();

                _func.push(Mapper { src, dest, length });
            });

            funcs.push(_func);
        });

        Self { funcs }
    }

    fn seed_to_loc(&self, seed: usize) -> usize {
        let mut _seed = seed;
        for map in &self.funcs {
            if let Some(m) = map.iter().find(|m| m.src_contains(_seed)) {
                if let Some(n_seed) = m.src_to_dest(_seed) {
                    _seed = n_seed;
                }
            }
        }
        _seed
    }
}

struct SeedRange(usize, usize);

fn part_two(input: String) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec()
        .chunks(2)
        .map(|chunk| SeedRange(chunk[0], chunk[1]))
        .collect_vec();

    let almanac = Almanac2::build(maps.to_string());
    let mut min_loc = usize::MAX;

    seeds.iter().for_each(|seed_range| {
        for seed in seed_range.0..seed_range.0 + seed_range.1 {
            let loc = almanac.seed_to_loc(seed);
            if loc < min_loc {
                min_loc = loc;
                println!("{}", min_loc);
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
        assert_eq!(part_two(read_txt_file(5, crate::TextEnum::Input)), 81956384);
    }
}

/* The efficient way :

For each seed range :

For each map :

We take each range and split them on intersections with given map

Then we store the result of the map.

And again until there is no range remaining.
seeds, *maps = open('input').read().split('\n\n')
seeds = [int(seed) for seed in seeds.split()[1:]]
maps = [[list(map(int, line.split())) for line in m.splitlines()[1:]] for m in maps]

locations = []
for i in range(0, len(seeds), 2):
    ranges = [[seeds[i], seeds[i + 1] + seeds[i]]]
    results = []
    for _map in maps:
        while ranges:
            start_range, end_range = ranges.pop()
            for target, start_map, r in _map:
                end_map = start_map + r
                offset = target - start_map
                if end_map <= start_range or end_range <= start_map:  # no overlap
                    continue
                if start_range < start_map:
                    ranges.append([start_range, start_map])
                    start_range = start_map
                if end_map < end_range:
                    ranges.append([end_map, end_range])
                    end_range = end_map
                results.append([start_range + offset, end_range + offset])
                break
            else:
                results.append([start_range, end_range])
        ranges = results
        results = []
    locations += ranges
print(min(loc[0] for loc in locations))
*/