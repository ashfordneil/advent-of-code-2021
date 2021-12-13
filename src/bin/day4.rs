use advent_of_code_2021::{
    tools::MoreItertools,
    util::{self, FixedCollector},
};
use std::{fs, path::PathBuf, str::FromStr};

use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-four.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Copy, Clone)]
struct BingoBoard {
    numbers: [[u32; 5]; 5],
    complete: [[bool; 5]; 5],
}

impl FromStr for BingoBoard {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let complete = [[false; 5]; 5];
        let numbers = s
            .lines()
            .map(|line| {
                let row = line
                    .split_whitespace()
                    .map(|num| num.trim().parse::<u32>())
                    .collect::<Result<FixedCollector<_, 5>, _>>()?
                    .0?;
                Ok::<_, eyre::Report>(row)
            })
            .collect::<Result<FixedCollector<_, 5>, _>>()?
            .0?;

        Ok(BingoBoard { complete, numbers })
    }
}

impl BingoBoard {
    pub fn is_complete(&self) -> bool {
        let has_row = self
            .complete
            .iter()
            .any(|row| row.iter().copied().all(|x| x));
        let has_col = (0..5).any(|col| self.complete.iter().copied().all(|row| row[col]));

        has_row || has_col
    }

    pub fn score(&self, last_number: u32) -> u32 {
        let rest: u32 = itertools::zip(self.numbers, self.complete)
            .flat_map(|(row_numbers, row_complete)| itertools::zip(row_numbers, row_complete))
            .filter(|&(_number, complete)| !complete)
            .map(|(number, _complete)| number)
            .sum();
        last_number * rest
    }

    pub fn insert(&mut self, new_number: u32) {
        itertools::zip(&mut self.numbers, &mut self.complete)
            .flat_map(|(row_numbers, row_complete)| itertools::zip(row_numbers, row_complete))
            .for_each(|(number, complete)| {
                if *number == new_number {
                    *complete = true;
                }
            });
    }
}

fn parse_bingo(input: &str) -> eyre::Result<(Vec<u32>, Vec<BingoBoard>)> {
    let (bingo_numbers, boards) = input
        .split_once("\n\n")
        .ok_or_else(|| eyre::format_err!("Not enough lines in the input"))?;
    let bingo_numbers = bingo_numbers
        .trim()
        .split(',')
        .parsed()
        .collect::<Result<Vec<_>, _>>()?;
    let boards = boards
        .split("\n\n")
        .map(|board| board.trim())
        .parsed()
        .collect::<Result<Vec<_>, _>>()?;

    Ok((bingo_numbers, boards))
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = fs::read_to_string(args.file)?;

    let (bingo_numbers, boards) = parse_bingo(&input)?;

    println!(
        "Part one: {}",
        part_one(bingo_numbers.iter().copied(), boards.clone())?
    );
    println!("Part two: {}", part_two(bingo_numbers.into_iter(), boards)?);

    Ok(())
}

fn part_one(
    mut bingo_numbers: impl Iterator<Item = u32>,
    mut boards: Vec<BingoBoard>,
) -> eyre::Result<u32> {
    bingo_numbers
        .find_map(|number| {
            for board in &mut boards {
                board.insert(number);
                if board.is_complete() {
                    return Some(board.score(number));
                }
            }

            None
        })
        .ok_or_else(|| eyre::format_err!("No bingo board completed"))
}

fn part_two(
    bingo_numbers: impl Iterator<Item = u32>,
    mut boards: Vec<BingoBoard>,
) -> eyre::Result<u32> {
    let last_solved = bingo_numbers
        .batching(|numbers| {
            numbers.find_map(|number| {
                let mut last_score = None;
                for board in &mut boards {
                    board.insert(number);
                    if board.is_complete() {
                        last_score = Some(board.score(number));
                    }
                }
                boards.retain(|board| !board.is_complete());

                last_score
            })
        })
        .last()
        .ok_or_else(|| eyre::format_err!("No bingo board completed"))?;

    if !boards.is_empty() {
        eyre::bail!("Some bingo boards left unfinished")
    }

    Ok(last_solved)
}

#[cfg(test)]
mod test {
    const INPUT_FILE: &'static str = r"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    ";

    #[test]
    fn parses_correctly() {
        let (numbers, boards) = super::parse_bingo(INPUT_FILE).unwrap();
        assert_eq!(27, numbers.len());
        assert_eq!(7, numbers[0]);

        assert_eq!(3, boards.len());
        assert_eq!(
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19]
            ],
            boards[0].numbers
        );
    }

    #[test]
    fn example_part_one() {
        let (numbers, boards) = super::parse_bingo(INPUT_FILE).unwrap();
        assert_eq!(4512, super::part_one(numbers.into_iter(), boards).unwrap());
    }

    #[test]
    fn example_part_two() {
        let (numbers, boards) = super::parse_bingo(INPUT_FILE).unwrap();
        assert_eq!(1924, super::part_two(numbers.into_iter(), boards).unwrap());
    }
}
