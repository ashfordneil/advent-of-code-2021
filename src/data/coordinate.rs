use crate::tools::StringTools;
use std::str::FromStr;

/// A set of (x, y) coordinates.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Coordinate(pub usize, pub usize);

impl FromStr for Coordinate {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_parse(",")?;
        Ok(Coordinate(x, y))
    }
}

impl Coordinate {
    /// Iterate over all possible coordinates within a grid, where the grid is defined by it's two
    /// corner points `(0,0)` and `max_point`.
    pub fn iter(max_point: Self) -> impl Iterator<Item = Self> {
        let Coordinate(max_x, max_y) = max_point;
        (0..max_x).flat_map(move |x| (0..max_y).map(move |y| Coordinate(x, y)))
    }

    /// Find all of the manhattan neighbours of this point (IE no diagonals) that lie within the
    /// grid defined by the corner points `(0,0)` and `max_point`.
    pub fn manhattan_neighbours(self, max_point: Self) -> impl Iterator<Item = Self> {
        let Coordinate(x, y) = self;
        let Coordinate(max_x, max_y) = max_point;

        let mut output = Vec::new();
        if x > 0 {
            output.push(Coordinate(x - 1, y));
        }
        if x + 1 < max_x {
            output.push(Coordinate(x + 1, y));
        }
        if y > 0 {
            output.push(Coordinate(x, y - 1));
        }
        if y + 1 < max_y {
            output.push(Coordinate(x, y + 1));
        }

        output.into_iter()
    }

    /// Find all of the neighbours of this point (including diagonals) that lie within the grid
    /// defined by the corner points `(0,0)` and `max_point`.
    pub fn all_neighbours(self, max_point: Self) -> impl Iterator<Item = Self> {
        let Coordinate(x, y) = self;
        let Coordinate(max_x, max_y) = max_point;

        let mut output = Vec::new();

        for dx in -1..=1 {
            if x == 0 && dx == -1 || (x + 1 == max_x && dx == 1) {
                continue;
            }

            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if y == 0 && dy == -1 || (y + 1 == max_y && dy == 1) {
                    continue;
                }

                output.push(Coordinate((x as isize + dx) as usize, (y as isize + dy) as usize))
            }
        }

        output.into_iter()
    }
}
