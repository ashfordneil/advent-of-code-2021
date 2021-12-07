use advent_of_code_2021::{parsing, util};
use std::{collections::VecDeque, path::PathBuf};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-six.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let input = parsing::comma_separated::<usize, _>(args.file)?;

    println!("Part one: {}", let_them_grow(&input, 80));
    println!("Part two: {}", let_them_grow(&input, 256));

    Ok(())
}

fn let_them_grow(input: &[usize], days: u32) -> u64 {
    let mut state = VecDeque::from([0u64; 9]);
    input.iter().copied().for_each(|age| {
        state[age] += 1;
    });

    for _ in 0..days {
        // bring every bucket one day closer to the front, and also find the fish that are ready
        let about_to_pop = state.pop_front().unwrap();
        // reset the parents
        state[6] += about_to_pop;
        // add the children to the back of the queue (which is always the "8 days to go" bucket)
        state.push_back(about_to_pop)
    }

    state.into_iter().sum()
}

#[cfg(test)]
mod test {
    const EXAMPLE_INPUT: &'static [usize] = &[3, 4, 3, 1, 2];

    #[test]
    fn example_part_one() {
        assert_eq!(5934, super::let_them_grow(EXAMPLE_INPUT, 80))
    }

    #[test]
    fn example_part_two() {
        assert_eq!(26984457539, super::let_them_grow(EXAMPLE_INPUT, 256))
    }
}
