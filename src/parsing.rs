//! Utilities for parsing input files.

use crate::SubmarineCommand;
use std::{fs, path::Path, str::FromStr};

use eyre::Report;

/// Fetch data from a file, split it up line by line, and parse it. I'm hoping the input formats
/// they give stay simple enough to be parsed by something like this function, but we'll have to
/// see how we go.
pub fn line_separated<T: FromStr, P: AsRef<Path>>(input: P) -> eyre::Result<Vec<T>>
where
    Report: From<T::Err>,
{
    let raw = fs::read_to_string(input)?;
    let output = raw
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse::<T>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(output)
}
impl FromStr for SubmarineCommand {
    type Err = eyre::Report;

    fn from_str(s: &str) -> eyre::Result<Self> {
        let (direction, magnitude) = s
            .split_once(' ')
            .ok_or_else(|| eyre::format_err!("Invalid command format"))?;
        let magnitude: isize = magnitude.parse()?;
        let output = match direction {
            "forward" => SubmarineCommand::Forward(magnitude),
            "down" => SubmarineCommand::Down(magnitude),
            "up" => SubmarineCommand::Up(magnitude),
            _ => eyre::bail!("Invalid direction"),
        };

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use crate::SubmarineCommand;

    #[test]
    fn parses_submarine_command() {
        assert_eq!(SubmarineCommand::Forward(7), "forward 7".parse().unwrap());
        assert_eq!(SubmarineCommand::Down(3), "down 3".parse().unwrap());
        assert_eq!(SubmarineCommand::Up(10), "up 10".parse().unwrap());

        "invalid 1".parse::<SubmarineCommand>().unwrap_err();
        "forward      3".parse::<SubmarineCommand>().unwrap_err();
        "".parse::<SubmarineCommand>().unwrap_err();
        "forward ".parse::<SubmarineCommand>().unwrap_err();
    }
}
