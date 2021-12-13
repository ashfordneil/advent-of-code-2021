use advent_of_code_2021::{data::Coordinate, parsing, tools::StringTools, util};
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
    start: Coordinate,
    end: Coordinate,
}

impl FromStr for Line {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_parse(" -> ")?;

        let output = Line { start, end };
        Ok(output)
    }
}

impl Line {
    fn is_non_diagonal(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn points(self) -> impl Iterator<Item = Coordinate> {
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
                Coordinate(self.start.0, item)
            } else if self.start.1 == self.end.1 {
                Coordinate(item, self.start.1)
            } else {
                let delta = item - min.0;
                if max.1 > min.1 {
                    Coordinate(item, min.1 + delta)
                } else {
                    Coordinate(item, min.1 - delta)
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
    use advent_of_code_2021::data::Coordinate;

    const EXAMPLE_INPUT: &'static [Line] = &[
        Line { start: Coordinate(0, 9), end: Coordinate(5, 9) },
        Line { start: Coordinate(8, 0), end: Coordinate(0, 8) },
        Line { start: Coordinate(9, 4), end: Coordinate(3, 4) },
        Line { start: Coordinate(2, 2), end: Coordinate(2, 1) },
        Line { start: Coordinate(7, 0), end: Coordinate(7, 4) },
        Line { start: Coordinate(6, 4), end: Coordinate(2, 0) },
        Line { start: Coordinate(0, 9), end: Coordinate(2, 9) },
        Line { start: Coordinate(3, 4), end: Coordinate(1, 4) },
        Line { start: Coordinate(0, 0), end: Coordinate(8, 8) },
        Line { start: Coordinate(5, 5), end: Coordinate(8, 2) },
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
