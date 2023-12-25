use crate::timed;

pub fn solve(input: String) {
    println!("Day 11");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, p: &Point) -> usize {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}

struct Galaxy {
    map: Vec<String>,
    locations: Vec<Point>,
}

impl Galaxy {
    fn new(raw_map: String) -> Self {
        let rows = Galaxy::get_empty_rows(&raw_map);
        let cols = Galaxy::get_empty_cols(&raw_map);
        let map = Galaxy::expand_map(&raw_map, rows, cols);
        let locations = Galaxy::get_locations(&map);

        Self { map, locations }
    }

    fn get_empty_rows(raw_map: &String) -> Vec<usize> {
        raw_map
            .lines()
            .enumerate()
            .fold(vec![], |mut acc, (row_index, row)| {
                if !row.contains('#') {
                    acc.push(row_index);
                }
                acc
            })
    }

    fn get_empty_cols(raw_map: &String) -> Vec<usize> {
        let mut cols = vec![];
        let width = raw_map.lines().next().unwrap().chars().count();
        for col_index in 0..width {
            let col = raw_map.lines().fold(String::new(), |mut acc, line| {
                acc.push(line.chars().nth(col_index).unwrap());
                acc
            });

            if !col.contains('#') {
                cols.push(col_index);
            }
        }

        cols
    }

    fn expand_map(raw_map: &String, rows: Vec<usize>, cols: Vec<usize>) -> Vec<String> {
        raw_map
            .lines()
            .enumerate()
            .fold(vec![], |mut acc, (row_index, row)| {
                let mut new_row = String::new();
                row.char_indices().for_each(|(col_index, ch)| {
                    new_row.push(ch);
                    if cols.contains(&col_index) {
                        new_row.push(ch);
                    }
                });

                if rows.contains(&row_index) {
                    acc.push(new_row.clone());
                }
                acc.push(new_row);
                acc
            })
    }

    fn get_locations(map: &Vec<String>) -> Vec<Point> {
        map.iter().enumerate().fold(vec![], |mut acc, (y, row)| {
            row.char_indices().for_each(|(x, ch)| {
                if ch == '#' {
                    acc.push(Point::new(x, y));
                }
            });
            acc
        })
    }

    fn get_pairs(&self) -> Vec<(&Point, &Point, usize)> {
        let mut galaxy_pairs = vec![];
        self.locations.iter().enumerate().for_each(|(index, p)| {
            for q in self.locations.iter().skip(index + 1) {
                galaxy_pairs.push((p, q, p.distance(q)));
            }
        });

        galaxy_pairs
    }
}

fn part_one(input: String) -> usize {
    let galaxy = Galaxy::new(input);

    galaxy.get_pairs().iter().fold(0, |acc, &(_, _, d)| acc + d)
}

fn part_two(input: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(11, crate::TextEnum::Input)), 9609130);
    }

    #[test]
    fn part_two_test() {
        //        assert_eq!(
        //            part_two(read_txt_file(11, crate::TextEnum::Input)),
        //            10151663816849
        //        );
    }
}
