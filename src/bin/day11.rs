use advent_of_code_2021::util::{self, FixedCollector};
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
    levels: [[u32; 10]; 10],
}

impl FromStr for OctopusGrid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        ch.to_digit(10)
                            .ok_or_else(|| eyre::format_err!("Invalid char: {:?}", ch))
                    })
                    .collect::<Result<FixedCollector<_, 10>, _>>()?
                    .0
            })
            .collect::<Result<FixedCollector<_, 10>, _>>()?
            .0?;

        Ok(OctopusGrid { levels })
    }
}

type Coordinate = (usize, usize);

impl Index<Coordinate> for OctopusGrid {
    type Output = u32;

    fn index(&self, (x, y): Coordinate) -> &Self::Output {
        self.levels.index(x).index(y)
    }
}

impl IndexMut<Coordinate> for OctopusGrid {
    fn index_mut(&mut self, (x, y): Coordinate) -> &mut Self::Output {
        self.levels.index_mut(x).index_mut(y)
    }
}

impl OctopusGrid {
    fn iter() -> impl Iterator<Item = Coordinate> {
        (0..10).flat_map(|x| (0..10).map(move |y| (x, y)))
    }

    fn neighbours((x, y): Coordinate) -> impl Iterator<Item = Coordinate> {
        let mut output = Vec::new();
        for dx in -1..=1 {
            if (dx == -1 && x == 0) || (dx == 1 && x == 9) {
                continue;
            }
            for dy in -1..=1 {
                if (dy == -1 && y == 0) || (dy == 1 && y == 9) || (dx == 0 && dy == 0) {
                    continue;
                }

                let x = (x as isize + dx) as usize;
                let y = (y as isize + dy) as usize;
                output.push((x, y))
            }
        }

        output.into_iter()
    }

    // Returns the number of flashes
    fn step(&mut self) -> usize {
        for coord in Self::iter() {
            self[coord] += 1;
        }

        // Things only enter this list once they have reached length 9
        let mut flashing = Self::iter()
            .filter(|&coord| self[coord] > 9)
            .collect::<VecDeque<_>>();

        while let Some(next) = flashing.pop_front() {
            for neighbour in Self::neighbours(next) {
                if self[neighbour] <= 9 {
                    self[neighbour] += 1;
                    if self[neighbour] > 9 {
                        flashing.push_back(neighbour);
                    }
                }
            }
        }

        Self::iter()
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
