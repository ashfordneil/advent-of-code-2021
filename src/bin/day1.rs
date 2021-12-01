use advent_of_code_2021::{parsing, util};
use std::path::PathBuf;

use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-one.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;

    let lines = parsing::line_separated::<usize, _>(&args.file)?;

    println!("Part one: {}", part_one(lines.iter().copied()));
    println!("Part two: {}", part_two(lines.iter().copied()));

    Ok(())
}

fn part_one(input: impl Iterator<Item = usize>) -> usize {
    input.tuple_windows().filter(|(lhs, rhs)| lhs < rhs).count()
}

fn part_two(input: impl Iterator<Item = usize>) -> usize {
    let partially_summed = input.tuple_windows().map(|(a, b, c)| a + b + c);
    part_one(partially_summed)
}

#[cfg(test)]
mod test {
    const EXAMPLE_INPUT: &'static [usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn example_part_one() {
        assert_eq!(7, super::part_one(EXAMPLE_INPUT.iter().copied()));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(5, super::part_two(EXAMPLE_INPUT.iter().copied()));
    }
}
