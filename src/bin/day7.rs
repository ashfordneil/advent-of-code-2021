use advent_of_code_2021::{parsing, util};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-seven.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = parsing::comma_separated::<isize, _>(args.file)?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

fn find_fuel(input: &[isize], cost: impl Fn(isize, isize) -> isize) -> Option<isize> {
    let min = input.iter().copied().min()?;
    let max = input.iter().copied().max()?;

    (min..max)
        .map(|position| {
            input
                .iter()
                .copied()
                .map(|crab| cost(position, crab))
                .sum::<isize>()
        })
        .min()
}

fn part_one(input: &[isize]) -> isize {
    find_fuel(input, |position, crab| (position - crab).abs()).unwrap()
}

fn part_two(input: &[isize]) -> isize {
    find_fuel(input, |position, crab| {
        let dist = (position - crab).abs();
        dist * (dist + 1) / 2
    })
    .unwrap()
}

#[cfg(test)]
mod test {
    const EXAMPLE_INPUT: &'static [isize] = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn example_part_one() {
        assert_eq!(37, super::part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(168, super::part_two(EXAMPLE_INPUT));
    }
}
