use advent_of_code_2021::{parsing, util, SubmarineCommand};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-two.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;

    let lines = parsing::line_separated::<SubmarineCommand, _>(args.file)?;

    println!("Part one: {}", part_one(&lines[..]));
    println!("Part two: {}", part_two(&lines[..]));

    Ok(())
}

fn part_one(input: &[SubmarineCommand]) -> isize {
    let h_pos: isize = input
        .iter()
        .copied()
        .filter_map(|input| match input {
            SubmarineCommand::Forward(x) => Some(x),
            _ => None,
        })
        .sum();

    let v_pos: isize = input
        .iter()
        .copied()
        .filter_map(|input| match input {
            SubmarineCommand::Down(x) => Some(x),
            SubmarineCommand::Up(x) => Some(-x),
            _ => None,
        })
        .sum();

    h_pos * v_pos
}

fn part_two(input: &[SubmarineCommand]) -> isize {
    let (h_pos, v_pos, _aim) = input
        .iter()
        .cloned()
        .fold((0, 0, 0), |(h_pos, v_pos, aim), item| match item {
            SubmarineCommand::Forward(forward) => (h_pos + forward, v_pos + aim * forward, aim),
            SubmarineCommand::Down(down) => (h_pos, v_pos, aim + down),
            SubmarineCommand::Up(up) => (h_pos, v_pos, aim - up),
        });

    h_pos * v_pos
}

#[cfg(test)]
mod test {
    use crate::SubmarineCommand;

    const EXAMPLE_INPUT: &'static [SubmarineCommand] = &[
        SubmarineCommand::Forward(5),
        SubmarineCommand::Down(5),
        SubmarineCommand::Forward(8),
        SubmarineCommand::Up(3),
        SubmarineCommand::Down(8),
        SubmarineCommand::Forward(2),
    ];

    #[test]
    fn example_part_one() {
        assert_eq!(150, super::part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(900, super::part_two(EXAMPLE_INPUT));
    }
}
