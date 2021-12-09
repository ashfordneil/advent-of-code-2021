use advent_of_code_2021::util;
use std::{cmp::Reverse, collections::HashMap, path::PathBuf, str::FromStr};

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
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
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

type Coord = (usize, usize);

impl CaveSystem {
    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (Coord, u32)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &val)| ((x, y), val)))
    }

    pub fn neighbours(&self, (x, y): Coord) -> impl Iterator<Item = (Coord, u32)> {
        let above = y.checked_sub(1).and_then(|y2| {
            let &value = self.data.get(y2)?.get(x)?;
            Some(((x, y2), value))
        });
        let below = y.checked_add(1).and_then(|y2| {
            let &value = self.data.get(y2)?.get(x)?;
            Some(((x, y2), value))
        });
        let left = x.checked_sub(1).and_then(|x2| {
            let &value = self.data.get(y)?.get(x2)?;
            Some(((x2, y), value))
        });
        let right = x.checked_add(1).and_then(|x2| {
            let &value = self.data.get(y)?.get(x2)?;
            Some(((x2, y), value))
        });

        let raw = [left, above, right, below];
        raw.into_iter().filter_map(|x| x)
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = std::fs::read_to_string(args.file)?.parse::<CaveSystem>()?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

fn part_one(input: &CaveSystem) -> u32 {
    input
        .iter()
        .filter(|&(coord, val)| {
            input
                .neighbours(coord)
                .all(|(_coord, other_val)| val < other_val)
        })
        .map(|(_coord, val)| val + 1)
        .sum()
}

fn find_basin(system: &CaveSystem, start: Coord, value: u32) -> Coord {
    let next_step = system
        .neighbours(start)
        .min_by_key(|&(_coord, other_val)| other_val);

    if let Some((coord, other_val)) = next_step {
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
    input
        .iter()
        .filter(|&(_coord, val)| val != 9)
        .for_each(|(coord, val)| {
            let basin = find_basin(input, coord, val);
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
