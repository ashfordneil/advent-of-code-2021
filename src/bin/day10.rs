use advent_of_code_2021::{parsing, util};
use std::{path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-ten.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
    Round,
    Square,
    Squiggle,
    Triangle,
}

impl Token {
    fn syntax_score(self) -> u32 {
        match self {
            Token::Round => 3,
            Token::Square => 57,
            Token::Squiggle => 1197,
            Token::Triangle => 25_137,
        }
    }

    // Watch out for overflow
    fn autocomplete_score(self) -> u64 {
        match self {
            Token::Round => 1,
            Token::Square => 2,
            Token::Squiggle => 3,
            Token::Triangle => 4,
        }
    }
}

struct Line(Vec<(Token, bool)>);

impl FromStr for Line {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .trim()
            .chars()
            .map(|ch| {
                let token = match ch {
                    '(' | ')' => Token::Round,
                    '[' | ']' => Token::Square,
                    '{' | '}' => Token::Squiggle,
                    '<' | '>' => Token::Triangle,
                    _ => eyre::bail!("Invalid char {:?}", ch),
                };

                let dir = match ch {
                    '(' | '[' | '{' | '<' => true,
                    ')' | ']' | '}' | '>' => false,
                    _ => unreachable!(),
                };

                Ok((token, dir))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Line(inner))
    }
}

enum Error {
    Corrupt(Token),
    Incomplete(Vec<Token>),
}

impl Line {
    fn find_error(&self) -> Error {
        let mut stack = Vec::new();
        let corruption = self.0.iter().copied().find(|&(ch, dir)| {
            if dir {
                stack.push(ch);
                false
            } else {
                let matching = stack.pop().expect("Too many closing brackets");
                matching != ch
            }
        });

        if let Some((ch, _)) = corruption {
            Error::Corrupt(ch)
        } else {
            stack.reverse();
            Error::Incomplete(stack)
        }
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = parsing::line_separated::<Line, _>(args.file)?;

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &[Line]) -> u32 {
    input
        .iter()
        .filter_map(|line| match line.find_error() {
            Error::Corrupt(ch) => Some(ch),
            _ => None,
        })
        .map(Token::syntax_score)
        .sum()
}

fn part_two(input: &[Line]) -> u64 {
    let mut all_scores = input
        .iter()
        .filter_map(|line| match line.find_error() {
            Error::Incomplete(rest) => Some(rest),
            _ => None,
        })
        .map(|rest| {
            rest.into_iter()
                .fold(0, |acc, ch| 5 * acc + ch.autocomplete_score())
        })
        .collect::<Vec<_>>();

    all_scores.sort();

    all_scores[all_scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::Line;
    use advent_of_code_2021::tools::{StringTools, MoreItertools};

    fn get_input() -> Vec<Line> {
        const EXAMPLE_INPUT: &'static str = r"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        ";

        EXAMPLE_INPUT
            .lines_good()
            .parsed()
            .collect::<Result<Vec<_>, _>>()
            .expect("Unable to parse input")
    }

    #[test]
    fn example_part_one() {
        let input = get_input();
        assert_eq!(26_397, super::part_one(&input));
    }

    #[test]
    fn example_part_two() {
        let input = get_input();
        assert_eq!(288957, super::part_two(&input));
    }
}
