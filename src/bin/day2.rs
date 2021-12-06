use advent_of_code_2021::{parsing, util};
use std::{path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-two.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

/// A command that can be given to the submarine.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SubmarineCommand {
    /// Go forward by a distance.
    Forward(isize),
    /// Go down by a distance - remember that for a submarine, down **increases** depth (or aim, or
    /// whatever).
    Down(isize),
    /// Go up by a distance - remember that for a submarine, up **decreases** depth (or aim, or
    /// whatever).
    Up(isize),
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
    use super::SubmarineCommand;

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
impl FromStr for SubmarineCommand {
    type Err = eyre::Report;

    fn from_str(s: &str) -> eyre::Result<Self> {
        let (direction, magnitude) = s
            .split_once(' ')
            .ok_or_else(|| eyre::format_err!("Invalid command format"))?;
        let magnitude: isize = magnitude.parse()?;
        let output = match direction {
            "forward" => SubmarineCommand::Forward(magnitude),
            "down" => SubmarineCommand::Down(magnitude),
            "up" => SubmarineCommand::Up(magnitude),
            _ => eyre::bail!("Invalid direction"),
        };

        Ok(output)
    }
}

#[cfg(test)]
mod parse_test {
    use super::SubmarineCommand;

    #[test]
    fn parses_submarine_command() {
        assert_eq!(SubmarineCommand::Forward(7), "forward 7".parse().unwrap());
        assert_eq!(SubmarineCommand::Down(3), "down 3".parse().unwrap());
        assert_eq!(SubmarineCommand::Up(10), "up 10".parse().unwrap());

        "invalid 1".parse::<SubmarineCommand>().unwrap_err();
        "forward      3".parse::<SubmarineCommand>().unwrap_err();
        "".parse::<SubmarineCommand>().unwrap_err();
        "forward ".parse::<SubmarineCommand>().unwrap_err();
    }
}
