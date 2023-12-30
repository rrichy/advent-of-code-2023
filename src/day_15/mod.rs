use crate::timed;

pub fn solve(input: String) {
    println!("Day 15");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |acc, ch| ((acc + (ch as usize)) * 17) % 256)
}

fn part_one(input: String) -> usize {
    input.split(',').fold(0, |acc, step| acc + hash(step))
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add(u32),
    Subtract,
}

#[derive(Debug, Clone)]
struct Step {
    label: String,
    box_index: usize,
    operation: Operation,
}

impl Step {
    fn new(step: &str) -> Self {
        let mut _step: (String, usize, char, Option<u32>) = ("".to_string(), 0, '\0', None);
        step.chars().for_each(|ch| {
            if ch == '=' || ch == '-' {
                _step.2 = ch;
            }

            if _step.2 == '\0' {
                _step.0.push(ch);
                _step.1 = ((_step.1 + (ch as usize)) * 17) % 256;
            } else {
                _step.3 = ch.to_digit(10);
            }
        });

        Self {
            label: _step.0,
            box_index: _step.1,
            operation: if _step.2 == '=' {
                Operation::Add(_step.3.unwrap())
            } else {
                Operation::Subtract
            },
        }
    }
}

fn part_two(input: String) -> usize {
    let mut map: Vec<Vec<Step>> = vec![vec![]; 256];

    input.split(',').for_each(|step| {
        let step = Step::new(step);

        if let Operation::Add(length) = step.operation {
            if let Some(bbox) = map.get_mut(step.box_index) {
                if let Some(step) = bbox.iter_mut().find(|_s| _s.label == step.label) {
                    step.operation = Operation::Add(length);
                } else {
                    bbox.push(step);
                }
            } else {
                map[step.box_index] = vec![step.clone()];
            }
        } else {
            if let Some(bbox) = map.get_mut(step.box_index) {
                if let Some(index) = bbox.iter().position(|_s| _s.label == step.label) {
                    bbox.splice(index..(index + 1), []);
                }
            }
        }
    });

    map.iter().enumerate().fold(0, |acc, (box_index, steps)| {
        acc + steps.iter().enumerate().fold(0, |acc, (step_index, step)| {
            if let Operation::Add(length) = step.operation {
                return acc + (box_index + 1) * (step_index + 1) * (length as usize);
            }
            acc
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::read_txt_file;

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(read_txt_file(15, crate::TextEnum::Input)), 498538);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(read_txt_file(15, crate::TextEnum::Input)), 286278);
    }
}
