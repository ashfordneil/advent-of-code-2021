use advent_of_code_2021::{
    data::Coordinate,
    tools::{MoreItertools, StringTools},
    util,
};
use std::{collections::HashSet, fs, path::PathBuf, str::FromStr};

use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-thirteen.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug)]
struct Manual {
    points: Vec<Coordinate>,
    folds: Vec<Fold>,
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    Y(usize),
    X(usize),
}

impl FromStr for Manual {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, folds) = s
            .split_once("\n\n")
            .ok_or_else(|| eyre::format_err!("Invalid input"))?;
        let points = points
            .lines_good()
            .parsed()
            .collect::<Result<Vec<Coordinate>, _>>()?;
        let folds = folds
            .lines_good()
            .parsed()
            .collect::<Result<Vec<Fold>, _>>()?;

        let output = Manual { points, folds };
        Ok(output)
    }
}

impl FromStr for Fold {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tail = s
            .strip_prefix("fold along ")
            .ok_or_else(|| eyre::format_err!("Invalid fold instruction"))?;
        let (dir, val) = tail
            .split_once('=')
            .ok_or_else(|| eyre::format_err!("Missing ="))?;
        let val = val.parse()?;
        let output = match dir {
            "x" => Fold::X(val),
            "y" => Fold::Y(val),
            _ => eyre::bail!("Invalid axis to fold along"),
        };
        Ok(output)
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = fs::read_to_string(args.file)?;
    let input = input.parse()?;

    println!("Part one: {}", part_one(&input));
    println!("Part two: {:#?}", part_two(&input));

    Ok(())
}

impl Manual {
    pub fn points<'a>(&'a self) -> impl 'a + Iterator<Item = Coordinate> {
        self.points.iter().copied()
    }

    pub fn folds<'a>(&'a self) -> impl 'a + Iterator<Item = Fold> {
        self.folds.iter().copied()
    }
}

impl Fold {
    fn reflect(val: usize, fold: usize) -> usize {
        if val < fold {
            val
        } else if val > fold {
            fold - (val - fold)
        } else {
            unreachable!("They said nothing would appear on the folding line")
        }
    }

    pub fn fold(self, Coordinate(x, y): Coordinate) -> Coordinate {
        match self {
            Fold::X(fx) => Coordinate(Self::reflect(x, fx), y),
            Fold::Y(fy) => Coordinate(x, Self::reflect(y, fy)),
        }
    }

    pub fn get_x(self) -> Option<usize> {
        if let Fold::X(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn get_y(self) -> Option<usize> {
        if let Fold::Y(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
}

fn part_one(input: &Manual) -> usize {
    let first_fold = input.folds[0];
    input
        .points()
        .map(|coord| first_fold.fold(coord))
        .unique()
        .count()
}

fn part_two(input: &Manual) -> Vec<String> {
    let all_points = input
        .points()
        .map(|coord| input.folds().fold(coord, |coord2, fold| fold.fold(coord2)))
        .collect::<HashSet<_>>();

    let max_x = input.folds().filter_map(Fold::get_x).last().unwrap();
    let max_y = input.folds().filter_map(Fold::get_y).last().unwrap();

    (0..max_y)
        .map(|y| {
            (0..max_x)
                .map(|x| {
                    if all_points.contains(&Coordinate(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::Manual;

    const EXAMPLE_INPUT: &'static str = r"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    ";

    fn get_input() -> Manual {
        EXAMPLE_INPUT.parse().unwrap()
    }

    #[test]
    fn example_part_one() {
        assert_eq!(17, super::part_one(&get_input()));
    }

    #[test]
    fn example_part_two() {
        let output = super::part_two(&get_input());
        assert_eq!(
            &["#####", "#...#", "#...#", "#...#", "#####", ".....", "....."],
            &output[..]
        );
    }
}
