use advent_of_code_2021::{parsing, util::{self, FixedCollector}};
use std::{path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-???.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = parsing::line_separated::<isize, _>(args.file)?;

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &[isize]) -> isize {
    0
}

fn part_two(input: &[isize]) -> isize {
    0
}

#[cfg(test)]
mod test {
    const EXAMPLE_INPUT: &'static [isize] = &[ 1, 2, 3 ];

    #[test]
    fn example_part_one() {
        assert_eq!(0, super::part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(0, super::part_two(EXAMPLE_INPUT));
    }
}