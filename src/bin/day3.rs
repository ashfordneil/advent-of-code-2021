use advent_of_code_2021::{parsing, util};
use std::{cmp::Ordering, iter, path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-three.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Copy, Clone)]
struct DiagnosticReport(u32);

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;

    let lines = parsing::line_separated::<DiagnosticReport, _>(args.file)?;

    println!("Part one: {}", part_one(&lines[..])?);
    println!("Part two: {}", part_two(&lines[..])?);

    Ok(())
}

/// We have a bunch of binary numbers, that we got from strings, but we need to find out how long
/// they were as strings. Step 1) hope that at least one of them had a 1 in the leading position.
/// Step 2) find out what the highest set bit is across all the binary numbers.
fn get_highest_bit(input: &[DiagnosticReport]) -> eyre::Result<u32> {
    let output = input
        .iter()
        .map(|DiagnosticReport(inner)| inner.next_power_of_two() / 2)
        .max()
        .ok_or_else(|| eyre::format_err!("The input was empty"))?;

    Ok(output)
}

/// Walk through a binary number, yielding a bitfield for each bit position in the number. For
/// example, this will spit out 16, 8, 4, 2, 1 if you start it with 16. By bitwise AND-ing these
/// masks with a number, you can walk through the bits of the number one at a time, in descending
/// order of significance.
fn walk_bits(highest_bit: u32) -> impl Iterator<Item = u32> {
    iter::successors(Some(highest_bit), move |&x| {
        let next = x / 2;
        if next == 0 {
            None
        } else {
            Some(next)
        }
    })
}

/// Find the most common bit value to be at a place within a set of numbers. Returns Some(true) if
/// more than half of the numbers have a 1 in that place, Some(false) if less than half do, and None
/// if it's equal.
fn most_common(mask: u32, set: &[DiagnosticReport]) -> Option<bool> {
    let matches = set
        .iter()
        .copied()
        .filter(|DiagnosticReport(item)| item & mask != 0)
        .count();

    match Ord::cmp(&(matches * 2), &set.len()) {
        Ordering::Less => Some(false),
        Ordering::Equal => None,
        Ordering::Greater => Some(true),
    }
}

fn part_one(input: &[DiagnosticReport]) -> eyre::Result<u32> {
    let highest_bit = get_highest_bit(input)?;

    let gamma: u32 = walk_bits(highest_bit)
        .filter(|&bit| most_common(bit, input) == Some(true))
        .sum();

    let epsilon = (highest_bit * 2 - 1) ^ gamma;

    Ok(gamma * epsilon)
}

fn part_two(input: &[DiagnosticReport]) -> eyre::Result<u32> {
    let highest_bit = get_highest_bit(input)?;

    let oxygen = {
        let mut candidates = input.to_vec();
        for bit in walk_bits(highest_bit) {
            if candidates.len() == 1 {
                break;
            }
            if most_common(bit, &candidates).unwrap_or(true) {
                candidates.retain(|DiagnosticReport(item)| item & bit != 0);
            } else {
                candidates.retain(|DiagnosticReport(item)| item & bit == 0);
            }
        }

        match &candidates[..] {
            &[one] => one,
            _ => eyre::bail!("Too many items left over"),
        }
    };

    let co2 = {
        let mut candidates = input.to_vec();
        for bit in walk_bits(highest_bit) {
            if candidates.len() == 1 {
                break;
            }
            if !most_common(bit, &candidates).unwrap_or(true) {
                candidates.retain(|DiagnosticReport(item)| item & bit != 0)
            } else {
                candidates.retain(|DiagnosticReport(item)| item & bit == 0)
            }
        }

        match &candidates[..] {
            &[one] => one,
            _ => eyre::bail!("Too many items left over"),
        }
    };

    Ok(oxygen.0 * co2.0)
}

#[cfg(test)]
mod test {
    use super::DiagnosticReport;

    fn get_example_input() -> eyre::Result<Vec<DiagnosticReport>> {
        let output = vec![
            "00100".parse()?,
            "11110".parse()?,
            "10110".parse()?,
            "10111".parse()?,
            "10101".parse()?,
            "01111".parse()?,
            "00111".parse()?,
            "11100".parse()?,
            "10000".parse()?,
            "11001".parse()?,
            "00010".parse()?,
            "01010".parse()?,
        ];
        Ok(output)
    }

    #[test]
    fn example_part_one() -> eyre::Result<()> {
        assert_eq!(198, super::part_one(&get_example_input()?[..])?);
        Ok(())
    }

    #[test]
    fn example_part_two() -> eyre::Result<()> {
        assert_eq!(230, super::part_two(&get_example_input()?[..])?);
        Ok(())
    }
}

impl FromStr for DiagnosticReport {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let output = u32::from_str_radix(s, 2)?;
        Ok(DiagnosticReport(output))
    }
}
