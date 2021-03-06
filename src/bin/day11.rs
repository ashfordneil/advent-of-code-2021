use advent_of_code_2021::{
    data::Coordinate,
    tools::StringTools,
    util::{self, FixedCollector},
};
use std::{
    collections::VecDeque,
    fs,
    ops::{Index, IndexMut},
    path::PathBuf,
    str::FromStr,
};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-eleven.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Copy, Clone)]
struct OctopusGrid {
    levels: [[u8; 10]; 10],
}

impl FromStr for OctopusGrid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .lines_good()
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        let mut tmp = [0u8; 4];
                        let string = ch.encode_utf8(&mut tmp);
                        string.parse::<u8>()
                    })
                    .collect::<Result<FixedCollector<_, 10>, _>>()?
                    .0
            })
            .collect::<Result<FixedCollector<_, 10>, _>>()?
            .0?;

        Ok(OctopusGrid { levels })
    }
}

impl Index<Coordinate> for OctopusGrid {
    type Output = u8;

    fn index(&self, Coordinate(x, y): Coordinate) -> &Self::Output {
        self.levels.index(x).index(y)
    }
}

impl IndexMut<Coordinate> for OctopusGrid {
    fn index_mut(&mut self, Coordinate(x, y): Coordinate) -> &mut Self::Output {
        self.levels.index_mut(x).index_mut(y)
    }
}

impl OctopusGrid {
    const MAX_POINT: Coordinate = Coordinate(10, 10);

    // Returns the number of flashes
    fn step(&mut self) -> usize {
        for coord in Coordinate::iter(Self::MAX_POINT) {
            self[coord] += 1;
        }

        // Things only enter this list once they have reached length 9
        let mut flashing = Coordinate::iter(Self::MAX_POINT)
            .filter(|&coord| self[coord] > 9)
            .collect::<VecDeque<_>>();

        while let Some(next) = flashing.pop_front() {
            for neighbour in next.all_neighbours(Self::MAX_POINT) {
                if self[neighbour] <= 9 {
                    self[neighbour] += 1;
                    if self[neighbour] > 9 {
                        flashing.push_back(neighbour);
                    }
                }
            }
        }

        Coordinate::iter(Self::MAX_POINT)
            .filter(|&coord| {
                if self[coord] > 9 {
                    self[coord] = 0;
                    true
                } else {
                    false
                }
            })
            .count()
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let raw = fs::read_to_string(args.file)?;
    let input = OctopusGrid::from_str(&raw)?;

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

fn part_one(mut input: OctopusGrid) -> usize {
    (0..100).map(|_| input.step()).sum()
}

fn part_two(mut input: OctopusGrid) -> u32 {
    (1..).find(|_| input.step() == 100).unwrap()
}

#[cfg(test)]
mod test {
    use crate::OctopusGrid;

    const EXAMPLE_INPUT: OctopusGrid = OctopusGrid {
        levels: [
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ],
    };

    #[test]
    fn example_part_one() {
        assert_eq!(1656, super::part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(195, super::part_two(EXAMPLE_INPUT));
    }
}
