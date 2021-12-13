use advent_of_code_2021::{data::Coordinate, tools::StringTools, util};
use std::{cmp::Reverse, collections::HashMap, fs, ops::Index, path::PathBuf, str::FromStr};

use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-nine.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

struct CaveSystem {
    data: Vec<Vec<u32>>,
}

impl FromStr for CaveSystem {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines_good()
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        let mut tmp = [0u8; 4];
                        let string = ch.encode_utf8(&mut tmp);
                        string.parse::<u32>()
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let row_len = data
            .get(0)
            .ok_or_else(|| eyre::format_err!("No lines found"))?
            .len();

        if row_len == 0 {
            eyre::bail!("Row must not be empty")
        }

        if !data.iter().all(|row| row.len() == row_len) {
            eyre::bail!("All rows must be the same length")
        }

        Ok(CaveSystem { data })
    }
}

impl CaveSystem {
    pub fn max_point(&self) -> Coordinate {
        Coordinate(self.data[0].len(), self.data.len())
    }
}

impl Index<Coordinate> for CaveSystem {
    type Output = u32;

    fn index(&self, Coordinate(x, y): Coordinate) -> &Self::Output {
        self.data.index(y).index(x)
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = fs::read_to_string(args.file)?.parse::<CaveSystem>()?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

fn part_one(input: &CaveSystem) -> u32 {
    Coordinate::iter(input.max_point())
        .filter(|&coord| {
            coord
                .manhattan_neighbours(input.max_point())
                .all(|other_coord| input[coord] < input[other_coord])
        })
        .map(|coord| input[coord] + 1)
        .sum()
}

fn find_basin(system: &CaveSystem, start: Coordinate, value: u32) -> Coordinate {
    let next_step = start
        .manhattan_neighbours(system.max_point())
        .min_by_key(|&coord| system[coord]);

    if let Some(coord) = next_step {
        let other_val = system[coord];
        // Thankfully, the input is set up so that there's always at least one strictly decreasing
        // path from a point to its basin - and even better: in situations where there's more than
        // one path, they all go to the same point. As such, we can just follow the path down
        // directly without needing any branching or risking any infinite loops.
        if other_val < value {
            return find_basin(system, coord, other_val);
        }
    }

    start
}

fn part_two(input: &CaveSystem) -> u32 {
    let mut basins = HashMap::new();
    Coordinate::iter(input.max_point())
        .filter(|&coord| input[coord] != 9)
        .for_each(|coord| {
            let basin = find_basin(input, coord, input[coord]);
            *basins.entry(basin).or_insert(0) += 1;
        });

    basins
        .into_values()
        .map(Reverse)
        .k_smallest(3)
        .map(|Reverse(x)| x)
        .product()
}

#[cfg(test)]
mod test {
    use super::CaveSystem;

    const EXAMPLE_INPUT: &'static str = r"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn example_input_one() {
        let input = EXAMPLE_INPUT.parse::<CaveSystem>().unwrap();
        assert_eq!(15, super::part_one(&input));
    }

    #[test]
    fn example_input_two() {
        let input = EXAMPLE_INPUT.parse::<CaveSystem>().unwrap();
        assert_eq!(1134, super::part_two(&input));
    }
}
