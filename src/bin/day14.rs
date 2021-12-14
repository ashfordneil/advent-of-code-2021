use advent_of_code_2021::{
    tools::{MoreItertools, StringTools},
    util::{self, FixedCollector},
};
use std::{
    collections::{BTreeMap, HashMap},
    fs, iter,
    iter::Peekable,
    path::PathBuf,
    str::FromStr,
};

use itertools::{Either, Itertools};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-fourteen.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Copy, Clone)]
struct Rule {
    pattern: (char, char),
    insert: char,
}

impl FromStr for Rule {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, extra) = s
            .split_once(" -> ")
            .ok_or_else(|| eyre::format_err!("Invalid pattern"))?;
        let pattern = {
            let [a, b] = pattern.chars().collect::<FixedCollector<_, 2>>().0?;
            (a, b)
        };
        let [insert] = extra.chars().collect::<FixedCollector<_, 1>>().0?;
        Ok(Rule { pattern, insert })
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let file = fs::read_to_string(args.file)?;
    let mut lines = file.as_str().lines_good();

    let template = lines
        .next()
        .ok_or_else(|| eyre::format_err!("Missing start line"))?;
    let rules = lines.parsed().collect::<Result<Vec<_>, _>>()?;

    println!("Part one: {}", part_one(&template, &rules));
    println!("Part two: {}", part_two(&template, &rules));

    Ok(())
}

struct Foldinator {
    data: BTreeMap<char, Vec<(char, char)>>,
}

impl Foldinator {
    fn get_insert(
        &self,
        first: char,
        it: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Option<char> {
        let second_step = self.data.get(&first)?;
        let second = it.peek()?;
        let (_pattern, insert) = second_step
            .iter()
            .find(|(pattern, _insert)| second == pattern)?;
        Some(*insert)
    }
    fn fold_polymer<'a>(
        &'a self,
        input: impl 'a + Iterator<Item = char>,
    ) -> impl 'a + Iterator<Item = char> {
        input
            .peekable()
            .batching(|it| {
                let first = it.next()?;
                let output = match self.get_insert(first, it) {
                    Some(insert) => Either::Left(iter::once(first).chain(iter::once(insert))),
                    None => Either::Right(iter::once(first)),
                };
                Some(output)
            })
            .flat_map(|x| x)
    }
}

fn part_one(input: &str, rules: &[Rule]) -> u64 {
    let mut foldinator = Foldinator {
        data: BTreeMap::new(),
    };
    for rule in rules {
        foldinator
            .data
            .entry(rule.pattern.0)
            .or_insert_with(Vec::new)
            .push((rule.pattern.1, rule.insert));
    }

    let computed_line = input.chars();
    // Can't be a loop because the type of computed_line changes every time we do this
    // Shout out to LLVM for taking this giant mess of nested iterators and making it quick
    let computed_line = foldinator.fold_polymer(computed_line); // 1
    let computed_line = foldinator.fold_polymer(computed_line); // 2
    let computed_line = foldinator.fold_polymer(computed_line); // 3
    let computed_line = foldinator.fold_polymer(computed_line); // 4
    let computed_line = foldinator.fold_polymer(computed_line); // 5
    let computed_line = foldinator.fold_polymer(computed_line); // 6
    let computed_line = foldinator.fold_polymer(computed_line); // 7
    let computed_line = foldinator.fold_polymer(computed_line); // 8
    let computed_line = foldinator.fold_polymer(computed_line); // 9
    let computed_line = foldinator.fold_polymer(computed_line); // 10

    let mut freqs = HashMap::new();
    for letter in computed_line {
        *freqs.entry(letter).or_insert(0) += 1;
    }

    let max = *freqs.values().max().unwrap();
    let min = *freqs.values().min().unwrap();

    max - min
}

fn part_two(input: &str, rules: &[Rule]) -> u64 {
    let mut digrams = HashMap::new();
    for digram in input.chars().tuple_windows() {
        *digrams.entry(digram).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut additions = HashMap::new();
        for rule in rules {
            if let Some(count) = digrams.remove(&rule.pattern) {
                *additions.entry((rule.pattern.0, rule.insert)).or_insert(0) += count;
                *additions.entry((rule.insert, rule.pattern.1)).or_insert(0) += count;
            }
        }

        for (digram, count) in additions {
            *digrams.entry(digram).or_insert(0) += count;
        }
    }

    let mut doubled_freqs = HashMap::new();
    for ((start, end), count) in digrams {
        *doubled_freqs.entry(start).or_insert(0) += count;
        *doubled_freqs.entry(end).or_insert(0) += count;
    }

    let max = (doubled_freqs.values().max().unwrap() + 1) / 2;
    let min = (doubled_freqs.values().min().unwrap() + 1) / 2;

    max - min
}

#[cfg(test)]
mod test {
    use super::Rule;
    use advent_of_code_2021::tools::{MoreItertools, StringTools};

    const EXAMPLE_TEMPLATE: &'static str = "NNCB";
    fn example_rules() -> Vec<Rule> {
        let raw = r"
            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        ";

        raw.lines_good()
            .parsed()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    #[test]
    fn example_part_one() {
        assert_eq!(1588, super::part_one(EXAMPLE_TEMPLATE, &example_rules()));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(
            2188189693529,
            super::part_two(EXAMPLE_TEMPLATE, &example_rules())
        );
    }
}
