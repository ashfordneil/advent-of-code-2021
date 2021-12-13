use std::str::FromStr;

/// Some extra utility methods to call on strings, to shorten the codebase elsewhere.
pub trait StringTools<'a> {
    /// A similar method to .lines() that's defined directly on &str, but with two improvements:
    ///
    /// 1. It automatically trims whitespace from the returned lines
    /// 2. It automatically removes any empty lines (good for when there's trailing newlines at the
    ///    end of the string, for example).
    fn lines_good(self) -> LinesGood<'a>;

    /// Split a string into two, at a delimiter, and then parse each half.
    fn split_parse<A, B>(self, delimiter: &str) -> eyre::Result<(A, B)>
    where
        A: FromStr,
        B: FromStr,
        eyre::Report: From<A::Err> + From<B::Err>;
}

/// See `StringTools::lines_good`.
pub struct LinesGood<'a> {
    inner: std::str::Lines<'a>,
}

impl<'a> Iterator for LinesGood<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.inner.next()?;
            let trimmed = next.trim();
            if !trimmed.is_empty() {
                break Some(trimmed);
            }
        }
    }
}

impl<'a> StringTools<'a> for &'a str {
    fn lines_good(self) -> LinesGood<'a> {
        LinesGood {
            inner: self.lines(),
        }
    }

    fn split_parse<A, B>(self, delimiter: &str) -> color_eyre::Result<(A, B)>
    where
        A: FromStr,
        B: FromStr,
        eyre::Report: From<A::Err> + From<B::Err>,
    {
        let (a, b) = self
            .split_once(delimiter)
            .ok_or_else(|| eyre::format_err!("Missing delimiter"))?;

        let a = a.parse()?;
        let b = b.parse()?;

        Ok((a, b))
    }
}
