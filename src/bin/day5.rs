use advent_of_code_2021::{parsing, util};
use std::{cmp, collections::HashSet, path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-five.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl FromStr for Line {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or_else(|| eyre::format_err!("Invalid line format"))?;
        let to_coord = |string: &str| -> eyre::Result<_> {
            let (x, y) = string
                .split_once(',')
                .ok_or_else(|| eyre::format_err!("Invalid coordinate format"))?;
            let x = x.parse()?;
            let y = y.parse()?;

            Ok((x, y))
        };

        let start = to_coord(start)?;
        let end = to_coord(end)?;

        let output = Line { start, end };
        Ok(output)
    }
}

impl Line {
    fn is_non_diagonal(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn points(self) -> impl Iterator<Item = (u32, u32)> {
        let min = cmp::min(self.start, self.end);
        let max = cmp::max(self.start, self.end);

        let range = if self.start.0 == self.end.0 {
            (min.1)..=(max.1)
        } else if self.start.1 == self.end.1 {
            (min.0)..=(max.0)
        } else {
            (min.0)..=(max.0)
        };

        range.map(move |item| {
            if self.start.0 == self.end.0 {
                (self.start.0, item)
            } else if self.start.1 == self.end.1 {
                (item, self.start.1)
            } else {
                let delta = item - min.0;
                if max.1 > min.1 {
                    (item, min.1 + delta)
                } else {
                    (item, min.1 - delta)
                }
            }
        })
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;

    let lines = parsing::line_separated::<Line, _>(args.file)?;

    println!("Part one: {}", part_one(lines.iter().copied()));
    println!("Part two: {}", part_two(lines.iter().copied()));
    Ok(())
}

fn count_overlaps(input: impl Iterator<Item = Line>) -> usize {
    let mut seen = HashSet::new();
    let mut seen_twice = HashSet::new();

    input
        .flat_map(|line| line.points())
        .filter(|&point| {
            if seen.contains(&point) {
                // Returns true if the point **is not** in seen_twice yet, meaning this is the
                // second time we're seeing it.
                seen_twice.insert(point)
            } else {
                seen.insert(point);
                false
            }
        })
        .count()
}

fn part_one(input: impl Iterator<Item = Line>) -> usize {
    count_overlaps(input.filter(|line| line.is_non_diagonal()))
}

fn part_two(input: impl Iterator<Item = Line>) -> usize {
    count_overlaps(input)
}

#[cfg(test)]
mod test {
    use super::Line;

    const EXAMPLE_INPUT: &'static [Line] = &[
        Line { start: (0, 9), end: (5, 9) },
        Line { start: (8, 0), end: (0, 8) },
        Line { start: (9, 4), end: (3, 4) },
        Line { start: (2, 2), end: (2, 1) },
        Line { start: (7, 0), end: (7, 4) },
        Line { start: (6, 4), end: (2, 0) },
        Line { start: (0, 9), end: (2, 9) },
        Line { start: (3, 4), end: (1, 4) },
        Line { start: (0, 0), end: (8, 8) },
        Line { start: (5, 5), end: (8, 2) },
    ];

    #[test]
    fn example_part_one() {
        assert_eq!(5, super::part_one(EXAMPLE_INPUT.iter().copied()))
    }

    #[test]
    fn example_part_two() {
        assert_eq!(12, super::part_two(EXAMPLE_INPUT.iter().copied()))
    }
}
