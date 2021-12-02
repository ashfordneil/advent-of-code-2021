//! Another year, another attempt at Advent of Code
//! All of the goodies are in ./src/bin/dayX.rs, this crate is just utilities to make the scripts
//! shorter.

pub mod parsing;
pub mod util;

/// A command that can be given to the submarine.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SubmarineCommand {
    /// Go forward by a distance.
    Forward(isize),
    /// Go down by a distance - remember that for a submarine, down **increases** depth (or aim, or
    /// whatever).
    Down(isize),
    /// Go up by a distance - remember that for a submarine, up **decreases** depth (or aim, or
    /// whatever).
    Up(isize),
}
