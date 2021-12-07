//! Utilities for parsing input files.

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

/// Fetch data from a file, split it up by comma delimiters, and parse it.
pub fn comma_separated<T: FromStr, P: AsRef<Path>>(input: P) -> eyre::Result<Vec<T>>
    where
        Report: From<T::Err>,
{
    let raw = fs::read_to_string(input)?;
    let output = raw
        .split(',')
        .map(|item| item.trim())
        .map(|item| item.parse::<T>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(output)
}
