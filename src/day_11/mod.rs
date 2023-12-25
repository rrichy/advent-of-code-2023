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

struct Universe {
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    expansion_size: usize,
    locations: Vec<Point>,
}

impl Universe {
    fn new(map: Vec<&str>, expansion_size: usize) -> Self {
        let empty_rows = Universe::get_empty_rows(&map);
        let empty_cols = Universe::get_empty_cols(&map);
        let locations = Universe::get_locations(&map);

        Self {
            empty_rows,
            empty_cols,
            expansion_size,
            locations,
        }
    }

    fn get_empty_rows(raw_map: &Vec<&str>) -> Vec<usize> {
        raw_map
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (row_index, row)| {
                if !row.contains('#') {
                    acc.push(row_index);
                }
                acc
            })
    }

    fn get_empty_cols(raw_map: &Vec<&str>) -> Vec<usize> {
        let mut cols = vec![];
        let width = raw_map.iter().next().unwrap().chars().count();
        for col_index in 0..width {
            let col = raw_map.iter().fold(String::new(), |mut acc, line| {
                acc.push(line.chars().nth(col_index).unwrap());
                acc
            });

            if !col.contains('#') {
                cols.push(col_index);
            }
        }

        cols
    }

    fn get_locations(map: &Vec<&str>) -> Vec<Point> {
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
                let mut distance = p.distance(q);
                let range_col = if p.x > q.x { q.x..=p.x } else { p.x..=q.x };
                let range_row = if p.y > q.y { q.y..=p.y } else { p.y..=q.y };

                self.empty_cols.iter().for_each(|col_index| {
                    if range_col.contains(col_index) {
                        distance -= 1;
                        distance += self.expansion_size;
                    }
                });

                self.empty_rows.iter().for_each(|row_index| {
                    if range_row.contains(row_index) {
                        distance -= 1;
                        distance += self.expansion_size;
                    }
                });

                galaxy_pairs.push((p, q, distance));
            }
        });

        galaxy_pairs
    }
}

fn part_one(input: String) -> usize {
    let galaxy = Universe::new(input.lines().collect(), 2);

    galaxy.get_pairs().iter().fold(0, |acc, &(_, _, d)| acc + d)
}

fn part_two(input: String) -> usize {
    let galaxy = Universe::new(input.lines().collect(), 1_000_000);

    galaxy.get_pairs().iter().fold(0, |acc, &(_, _, d)| acc + d)
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
        assert_eq!(
            part_two(read_txt_file(11, crate::TextEnum::Input)),
            702152204842
        );
    }
}
