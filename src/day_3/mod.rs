use std::time::Instant;

use crate::read_txt_file;

pub fn solve() {
    println!("Day Three");

    part_one();
    part_two();
}

fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(3, crate::TextEnum::Input);

    let mut sum: u32 = 0;

    let schema: Vec<&str> = input.lines().collect();

    for (row_index, line) in schema.iter().enumerate() {
        let mut pointer = 0;
        loop {
            match &line[pointer..].find(char::is_numeric) {
                Some(digit_start_index) => {
                    let absolute_start_index = pointer + *digit_start_index;
                    let number: String = line[absolute_start_index..]
                        .chars()
                        .take_while(|&c| c.is_numeric())
                        .collect();

                    pointer += digit_start_index + number.len();

                    for (pos, ch) in number.char_indices() {
                        let col_index: i32 = (absolute_start_index + pos).try_into().unwrap();
                        let row_index: i32 = row_index.try_into().unwrap();

                        let should_add = vec![
                            is_part_schema(&col_index - 1, &row_index - 1, &schema),
                            is_part_schema(col_index, &row_index - 1, &schema),
                            is_part_schema(&col_index + 1, &row_index - 1, &schema),
                            is_part_schema(&col_index - 1, row_index, &schema),
                            is_part_schema(&col_index + 1, row_index, &schema),
                            is_part_schema(&col_index - 1, &row_index + 1, &schema),
                            is_part_schema(col_index, &row_index + 1, &schema),
                            is_part_schema(&col_index + 1, &row_index + 1, &schema),
                        ]
                        .iter()
                        .any(|res| res == &true);

                        if should_add {
                            let number = number.parse::<u32>().unwrap();
                            sum += number;
                            break;
                        }
                    }
                }
                None => break,
            }
        }
    }

    println!("Result: {:?}", sum);

    println!("Solved in: {:?}", start.elapsed());
    sum
}

fn is_part_schema(col_index: i32, row_index: i32, schema: &Vec<&str>) -> bool {
    if col_index < 0 || row_index < 0 {
        return false;
    }

    let y: usize = row_index.try_into().unwrap();
    let x: usize = col_index.try_into().unwrap();
    match schema.get(y) {
        Some(&line) => match line.chars().nth(x) {
            Some(ch) => {
                if !ch.is_numeric() && ch.ne(&'.') {
                    return true;
                }
            }
            None => (),
        },
        None => {}
    }

    return false;
}

#[derive(Clone, Copy, Debug)]
struct PartNumber {
    col_index: usize,
    row_index: usize,
    value: u32,
    length: usize,
}

impl PartialEq for PartNumber {
    fn eq(&self, other: &Self) -> bool {
        self.col_index == other.col_index
            && self.row_index == other.row_index
            && self.value == other.value
            && self.length == other.length
    }
}

struct GearParts(PartNumber, PartNumber);

#[derive(Debug)]
struct Gear {
    col_index: usize,
    row_index: usize,
    parts: (Option<PartNumber>, Option<PartNumber>),
}

impl Gear {
    fn new(col_index: usize, row_index: usize) -> Self {
        Gear {
            col_index,
            row_index,
            parts: (None, None),
        }
    }

    fn add_part(&mut self, part_number: PartNumber) -> () {
        match self.parts.0 {
            Some(pt) => {
                if pt == part_number {
                    return;
                }
            }
            None => {
                self.parts.0 = Some(part_number);
                return;
            }
        }

        if self.parts.1.is_none() {
            self.parts.1 = Some(part_number);
        }
    }
}

struct Schema {
    width: usize,
    height: usize,
    lines: Vec<String>,
}

impl Schema {
    fn is_out_of_range(&self, col: i32, row: i32) -> bool {
        col < 0
            || col >= self.width.try_into().unwrap()
            || row < 0
            || row >= self.height.try_into().unwrap()
    }
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(3, crate::TextEnum::Input);
    let mut sum: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();
    let schema = Schema {
        width: lines[0].len(),
        height: lines.len(),
        lines: lines.iter().map(|s| s.to_string()).collect(),
    };

    for (absolute_row_index, line) in schema.lines.iter().enumerate() {
        let mut pointer = 0;
        loop {
            match line[pointer..].find("*") {
                Some(relative_gear_index) => {
                    let absolute_gear_index = pointer + relative_gear_index;
                    pointer = absolute_gear_index + 1;

                    let mut gear = Gear::new(absolute_gear_index, absolute_row_index);
                    find_gear_adjacents(&mut gear, &schema);

                    if let (Some(part_1), Some(part_2)) = gear.parts {
                        sum += part_1.value * part_2.value;
                    }
                    // match find_gear_adjacents(gear, &schema) {
                    //     Some(GearParts(part_1, part_2)) => sum += part_1.value * part_2.value,
                    //     None => (),
                    // }
                }
                None => break,
            }
        }
    }

    println!("Result: {:?}", sum);

    println!("Solved in: {:?}", start.elapsed());
    sum
}

fn find_gear_adjacents(gear: &mut Gear, schema: &Schema) -> () {
    let col_index: i32 = gear.col_index.try_into().unwrap();
    let row_index: i32 = gear.row_index.try_into().unwrap();

    if !schema.is_out_of_range(col_index, row_index - 1) {
        let row_index = row_index - 1;
        let line = &schema.lines[gear.row_index - 1];

        match line.chars().nth(gear.col_index) {
            Some(ch) => {
                if ch.is_numeric() {
                    match get_part_number_from_subdigit(col_index, row_index, &schema) {
                        Some(pn) => gear.add_part(pn),
                        None => (),
                    }
                }
            }
            None => (),
        }

        if col_index - 1 >= 0 {
            match line.chars().nth(gear.col_index - 1) {
                Some(ch) => {
                    if ch.is_numeric() {
                        match get_part_number_from_subdigit(col_index - 1, row_index, &schema) {
                            Some(pn) => gear.add_part(pn),
                            None => (),
                        }
                    }
                }
                None => (),
            }
        }

        match line.chars().nth(gear.col_index + 1) {
            Some(ch) => {
                if ch.is_numeric() {
                    match get_part_number_from_subdigit(col_index + 1, row_index, &schema) {
                        Some(pn) => gear.add_part(pn),
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    if !schema.is_out_of_range(col_index, row_index + 1) {
        let row_index = row_index + 1;
        let line = &schema.lines[gear.row_index + 1];

        match line.chars().nth(gear.col_index) {
            Some(ch) => {
                if ch.is_numeric() {
                    match get_part_number_from_subdigit(col_index, row_index, &schema) {
                        Some(pn) => gear.add_part(pn),
                        None => (),
                    }
                }
            }
            None => (),
        }

        if col_index - 1 >= 0 {
            match line.chars().nth(gear.col_index - 1) {
                Some(ch) => {
                    if ch.is_numeric() {
                        match get_part_number_from_subdigit(col_index - 1, row_index, &schema) {
                            Some(pn) => gear.add_part(pn),
                            None => (),
                        }
                    }
                }
                None => (),
            }
        }

        match line.chars().nth(gear.col_index + 1) {
            Some(ch) => {
                if ch.is_numeric() {
                    match get_part_number_from_subdigit(col_index + 1, row_index, &schema) {
                        Some(pn) => gear.add_part(pn),
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    let line = &schema.lines[gear.row_index];
    if gear.col_index >= 0 {
        match line.chars().nth(gear.col_index - 1) {
            Some(ch) => {
                if ch.is_numeric() {
                    match get_part_number_from_subdigit(col_index - 1, row_index, &schema) {
                        Some(pn) => gear.add_part(pn),
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    if gear.col_index < schema.width - 1 {
        match line.chars().nth(gear.col_index + 1) {
            Some(ch) => {
                if ch.is_numeric() {
                    match get_part_number_from_subdigit(col_index + 1, row_index, &schema) {
                        Some(pn) => gear.add_part(pn),
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
}

fn get_part_number_from_subdigit(
    col_index: i32,
    row_index: i32,
    schema: &Schema,
) -> Option<PartNumber> {
    if schema.is_out_of_range(col_index, row_index) {
        return None;
    }

    let row_index: usize = row_index.try_into().unwrap();
    let col_index: usize = col_index.try_into().unwrap();

    let line = &schema.lines[row_index];
    match line.chars().nth(col_index) {
        Some(ch) => {
            if ch.is_numeric() {
                let start_index = col_index
                    - line[..col_index]
                        .chars()
                        .rev()
                        .take_while(|&c| c.is_numeric())
                        .count();

                let number: String = line[start_index..]
                    .chars()
                    .take_while(|&c| c.is_numeric())
                    .collect();
                return Some(PartNumber {
                    col_index: start_index,
                    row_index,
                    value: number.parse::<u32>().unwrap(),
                    length: number.len(),
                });
            }
        }
        None => (),
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 528799);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 84907174);
    }
}
