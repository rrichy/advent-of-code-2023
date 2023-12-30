#!/bin/bash

DIR="$PWD/src/day_$1" 

if test $DIR; then
    echo "File already exist!"
    exit 1
fi

echo "Creating template files"
mkdir -p $DIR && touch "$DIR/sample.txt" "$DIR/input.txt"

echo -e "use crate::timed;

pub fn solve(input: String) {
    println!(\"Day $1\");

    timed(|| part_one(input.clone()));
    timed(|| part_two(input));
}

fn part_one(input: String) -> usize {
    0
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
//        assert_eq!(part_one(read_txt_file($1, crate::TextEnum::Input)), 11911);
    }

    #[test]
    fn part_two_test() {
//        assert_eq!(
//            part_two(read_txt_file($1, crate::TextEnum::Input)),
//            10151663816849
//        );
    }
}
" > "$DIR/mod.rs"

echo "Adding created template files to lib.rs"

SEARCH_LINE="_ => panic!(\"Day {} has not yet been solved.\", day),"

sed -i "/$SEARCH_LINE/i $1 => day_$1::solve(input)," $PWD/src/lib.rs

echo "mod day_$1;" >> "$PWD/src/lib.rs"

cargo fmt