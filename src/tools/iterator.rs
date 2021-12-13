use std::marker::PhantomData;
use std::str::FromStr;

/// An extension to `Itertools` with some extra methods that are useful just for me.
pub trait MoreItertools: Iterator {
    /// Call .parse on each item of the iterator, emitting the output.
    fn parsed<'a, T>(self) -> Parsing<Self, T> where Self: Sized + Iterator<Item = &'a str> {
        Parsing { inner: self, _phantom: PhantomData }
    }
}

/// See `MoreItertools::parsed`.
pub struct Parsing<I, T> {
    inner: I,
    _phantom: PhantomData<T>,
}

impl <'a, I, T> Iterator for Parsing<I, T> where I: Iterator<Item = &'a str>, T: FromStr {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let outcome = next.parse::<T>();
        Some(outcome)
    }
}

impl<I: Iterator> MoreItertools for I {}