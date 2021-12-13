use advent_of_code_2021::{
    parsing,
    util::{self, FixedCollector},
};
use std::{path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-eight.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SevenSeg(u8);

impl FromStr for SevenSeg {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.chars().try_fold(0, |sum, ch| {
            let bit = match ch {
                'a' => 1,
                'b' => 2,
                'c' => 3,
                'd' => 4,
                'e' => 5,
                'f' => 6,
                'g' => 7,
                other => eyre::bail!("Invalid character {}", other),
            };
            Ok(sum + (1 << bit))
        })?;

        Ok(SevenSeg(value))
    }
}

impl SevenSeg {
    fn len(self) -> u32 {
        self.0.count_ones()
    }

    fn contains(self, other: Self) -> bool {
        other.0 & self.0 == other.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Input {
    digits: [SevenSeg; 10],
    display_value: [SevenSeg; 4],
}

impl FromStr for Input {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(" | ")
            .ok_or_else(|| eyre::format_err!("Invalid format"))?;
        let digits = first
            .split(" ")
            .map(|seg| seg.parse())
            .collect::<Result<FixedCollector<_, 10>, _>>()?
            .0?;
        let display_value = second
            .split(" ")
            .map(|seg| seg.parse())
            .collect::<Result<FixedCollector<_, 4>, _>>()?
            .0?;
        Ok(Input {
            digits,
            display_value,
        })
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = parsing::line_separated::<Input, _>(args.file)?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input)?);

    Ok(())
}

fn part_one(input: &[Input]) -> usize {
    input
        .iter()
        .cloned()
        .flat_map(|input| input.display_value.into_iter())
        .map(|seg| seg.len())
        .filter(|&len| len == 2 || len == 4 || len == 3 || len == 7)
        .count()
}

fn find_with_len<const N: usize>(len: u32, input: &Input) -> eyre::Result<[SevenSeg; N]> {
    input
        .digits
        .iter()
        .copied()
        .filter(|&x| x.len() == len)
        .collect::<FixedCollector<_, N>>()
        .0
}

fn part_two(input: &[Input]) -> eyre::Result<usize> {
    input
        .iter()
        .map(|input| {
            // Easy ones
            let [one] = find_with_len(2, input)?;
            let [four] = find_with_len(4, input)?;
            let [seven] = find_with_len(3, input)?;
            let [eight] = find_with_len(7, input)?;

            // Six segments lit up
            let (zero, six, nine) = match find_with_len::<3>(6, input)? {
                [nine, a, b] | [a, nine, b] | [a, b, nine] if nine.contains(four) => match (a, b) {
                    (zero, six) | (six, zero) if zero.contains(one) => (zero, six, nine),
                    _ => eyre::bail!("Can't find a 0"),
                },
                _ => eyre::bail!("Can't find a 9"),
            };

            // Five segments lit up
            let (two, three, five) = match find_with_len::<3>(5, input)? {
                [three, a, b] | [a, three, b] | [a, b, three] if three.contains(one) => {
                    match (a, b) {
                        (two, five) | (five, two) if six.contains(five) => (two, three, five),
                        _ => eyre::bail!("Can't find a 5"),
                    }
                }
                _ => eyre::bail!("Can't find a 3"),
            };

            let digits = [zero, one, two, three, four, five, six, seven, eight, nine];

            let [a, b, c, d] = input
                .display_value
                .into_iter()
                .map(|this_digit| digits.iter().position(|&other| this_digit == other))
                .collect::<Option<FixedCollector<_, 4>>>()
                .ok_or_else(|| eyre::format_err!("Invalid digit found"))?
                .0?;

            Ok(a * 1000 + b * 100 + c * 10 + d)
        })
        .try_fold(0, |x, y| y.map(|y| x + y))
}

#[cfg(test)]
mod test {
    use super::Input;
    use advent_of_code_2021::tools::{MoreItertools, StringTools};

    fn get_example_input() -> Vec<Input> {
        let raw = r"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ";
        raw.lines_good()
            .parsed()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    #[test]
    fn example_part_one() {
        let example_input = get_example_input();
        assert_eq!(26, super::part_one(&example_input));
    }

    #[test]
    fn example_part_two() {
        let example_input = get_example_input();
        assert_eq!(61229, super::part_two(&example_input).unwrap());
    }
}
