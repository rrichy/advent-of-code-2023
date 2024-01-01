use std::collections::HashSet;

use crate::timed;

pub fn solve(input: String) {
    println!("Day 16");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn forward(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }

    fn col(&self) -> usize {
        self.x.try_into().unwrap()
    }

    fn row(&self) -> usize {
        self.y.try_into().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, grid: char) -> Self {
        if grid == '/' {
            return match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
        }

        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    position: Position,
    direction: Direction,
    side: usize,
}

impl Beam {
    fn new(x: usize, y: usize, direction: Direction, mirrors: &Vec<String>) -> Self {
        Self {
            position: Position::new(x as i32, y as i32),
            direction,
            side: mirrors.len(),
        }
    }

    fn spawn(&self, position: Position, &direction: &Direction) -> Self {
        Self {
            position,
            direction,
            side: self.side,
        }
    }

    fn forward(&self) -> Option<Beam> {
        let pos = self.position.forward(&self.direction);
        if pos.x < 0 || pos.x >= (self.side as i32) || pos.y < 0 || pos.y >= (self.side as i32) {
            return None;
        }

        Some(self.spawn(pos, &self.direction))
    }

    fn turn(&mut self, grid: char) -> () {
        self.direction = self.direction.turn(grid);
    }
}

#[derive(Debug)]
struct Contraption {
    mirrors: Vec<String>,
    // history: HashSet<Beam>,
    // energized: Vec<Vec<char>>,
}

impl Contraption {
    fn new(input: String) -> Self {
        let mirrors: Vec<String> = input.lines().map(|l| l.to_string()).collect();
        Self { mirrors }
    }

    fn energize(&mut self, init_beam: Beam) -> usize {
        let mut history: HashSet<Beam> = HashSet::new();
        let mut energized: Vec<Vec<char>> = self
            .mirrors
            .iter()
            .map(|str| str.chars().collect())
            .collect();
        let mut beams = vec![init_beam];

        while beams.len() > 0 {
            let mut beam = beams.pop().unwrap();

            history.insert(beam);
            energized[beam.position.row()][beam.position.col()] = '#';

            let advance_vertically =
                beam.direction == Direction::Up || beam.direction == Direction::Down;
            let grid = self.mirrors[beam.position.row()]
                .chars()
                .nth(beam.position.col())
                .unwrap();

            if grid == '/' || grid == '\\' {
                beam.turn(grid);
            }

            if (grid == '|' && !advance_vertically) || (grid == '-' && advance_vertically) {
                let mut beam_2 = beam.clone();
                beam.turn('/');
                beam_2.turn('\\');

                if let Some(beam) = beam_2.forward() {
                    if !history.contains(&beam) {
                        beams.push(beam);
                    }
                }
            }

            if let Some(beam) = beam.forward() {
                if !history.contains(&beam) {
                    beams.push(beam);
                }
            }
        }

        energized.iter().fold(0, |e, chars| {
            e + chars.iter().fold(0, |e, ch| {
                if ch == &'#' {
                    return e + 1;
                }
                e
            })
        })
    }
}

fn part_one(input: String) -> usize {
    let mut contraption = Contraption::new(input);
    contraption.energize(Beam::new(0, 0, Direction::Right, &contraption.mirrors))

    // contraption.energized.iter().for_each(|l| {
    //     let l = l.iter().fold(String::new(), |mut s, &ch| {
    //         s.push(ch);
    //         s
    //     });

    //     println!("{:?}", l);
    // });
    // contraption.energized.iter().fold(0, |e, chars| {
    //     e + chars.iter().fold(0, |e, ch| {
    //         if ch == &'#' {
    //             return e + 1;
    //         }
    //         e
    //     })
    // })
}

fn part_two(input: String) -> usize {
    let mut contraption = Contraption::new(input);
    let mut max = 0;

    let last_index = contraption.mirrors.len() - 1;
    for index in 0..=last_index {
        let left =
            contraption.energize(Beam::new(0, index, Direction::Right, &contraption.mirrors));
        if left > max {
            max = left;
        }

        let top = contraption.energize(Beam::new(index, 0, Direction::Down, &contraption.mirrors));
        if top > max {
            max = top;
        }

        let right = contraption.energize(Beam::new(
            last_index,
            index,
            Direction::Left,
            &contraption.mirrors,
        ));
        if right > max {
            max = right;
        }

        let down = contraption.energize(Beam::new(
            index,
            last_index,
            Direction::Up,
            &contraption.mirrors,
        ));
        if down > max {
            max = down;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(16, crate::TextEnum::Input)), 8323);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(16, crate::TextEnum::Input)), 8491);
    }
}
