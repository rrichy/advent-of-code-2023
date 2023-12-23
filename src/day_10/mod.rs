use std::collections::HashSet;

use crate::timed;

pub fn solve(input: String) {
    println!("Day 10");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
    c: char,
}

impl Point {
    fn new(x: usize, y: usize, c: char) -> Self {
        Self { x, y, c }
    }
}

struct Sketch {
    map: Vec<String>,
    starting_point: Point,
    visited: Vec<Point>,
}

impl Sketch {
    fn build(map: Vec<String>) -> Self {
        let mut starting_point = Point::new(0, 0, 'S');

        for (y, line) in map.iter().enumerate() {
            if let Some(x) = line.find("S") {
                starting_point = Point::new(x, y, 'S');
            }
        }

        Self {
            map,
            starting_point,
            visited: vec![starting_point],
        }
    }

    fn travel_loop(&mut self) -> usize {
        let points = self.neighbors(&self.starting_point);
        for point in points {
            self.travel_to(point);
        }

        self.visited.len()
    }

    fn travel_to(&mut self, p: Point) -> () {
        if self.visited.contains(&p) {
            return;
        }
        self.visited.push(p);

        let points = self.neighbors(&p);
        if points.len() < 2 {
            self.visited.pop();
        } else {
            for point in points {
                self.travel_to(point);
            }
        }
    }

    fn neighbors(&self, p: &Point) -> Vec<Point> {
        let mut n = vec![];
        if p.x > 0
            && "-J7S".contains(p.c)
            && "-LFS".contains(self.map[p.y].chars().nth(p.x - 1).unwrap())
        {
            n.push(Point::new(
                p.x - 1,
                p.y,
                self.map[p.y].chars().nth(p.x - 1).unwrap(),
            ));
        }

        if p.x < self.map[0].len()
            && "-LFS".contains(p.c)
            && "-J7S".contains(self.map[p.y].chars().nth(p.x + 1).unwrap())
        {
            n.push(Point::new(
                p.x + 1,
                p.y,
                self.map[p.y].chars().nth(p.x + 1).unwrap(),
            ));
        }

        if p.y > 0
            && "|LJS".contains(p.c)
            && "|7FS".contains(self.map[p.y - 1].chars().nth(p.x).unwrap())
        {
            n.push(Point::new(
                p.x,
                p.y - 1,
                self.map[p.y - 1].chars().nth(p.x).unwrap(),
            ));
        }

        if p.y < self.map.len()
            && "|7FS".contains(p.c)
            && "|LJS".contains(self.map[p.y + 1].chars().nth(p.x).unwrap())
        {
            n.push(Point::new(
                p.x,
                p.y + 1,
                self.map[p.y + 1].chars().nth(p.x).unwrap(),
            ));
        }

        n
    }
}

fn part_one(input: String) -> usize {
    let mut sketch = Sketch::build(input.lines().map(|line| line.to_string()).collect());

    sketch.travel_loop() / 2
}

fn part_two(input: String) -> usize {
    let mut sketch = Sketch::build(input.lines().map(|line| line.to_string()).collect());

    sketch.travel_loop();

    for point in sketch.visited.iter() {
        // println!("{:#?}", point);
        // sketch.map[point.y] = sketch.map[point.y].as_str();
        if let Some(l) = sketch.map.get_mut(point.y) {
            // if let Some(c) = l.chars().nth(point.x) {
            // *c = 'X';

            // Create a new string with the modified character
            let modified_string = l
                .chars()
                .enumerate()
                .map(|(i, c)| if i == point.x { 'X' } else { c })
                .collect::<String>();

            // Update the vector with the modified string
            *l = modified_string;

            // println!("{}", l);
            // }
        }
    }

    println!("{:#?}", sketch.map);
    0
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(10, crate::TextEnum::Input)), 6838);
    }

    #[test]
    fn part_two_test() {
        //        assert_eq!(
        //            part_two(read_txt_file(10, crate::TextEnum::Input)),
        //            10151663816849
        //        );
    }
}
