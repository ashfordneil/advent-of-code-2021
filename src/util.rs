//! Utilities for writing shorter scripts each day.

use std::{iter::FromIterator, mem::MaybeUninit, ptr};

use structopt::StructOpt;

/// Set everything up to start running. The logging is maybe premature at this stage, but I'd rather
/// not have to add it when I'm in the middle of solving a problem.
pub fn setup<A: StructOpt>() -> eyre::Result<A> {
    color_eyre::install()?;
    env_logger::try_init()?;

    Ok(A::from_args())
}

pub struct FixedCollector<T, const N: usize>(pub eyre::Result<[T; N]>);

impl<T, const N: usize> FromIterator<T> for FixedCollector<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(input: I) -> Self {
        let mut input = input.into_iter();
        // Safety: assuming that the outside is init doesn't really mean anything, because the inside
        // still isn't
        let mut half_filled = unsafe {
            let alloc: MaybeUninit<[MaybeUninit<T>; N]> = MaybeUninit::uninit();
            let cast = alloc.assume_init();
            cast
        };

        for i in 0..N {
            if let Some(next) = input.next() {
                half_filled[i].write(next);
            } else {
                // Safety: we know that all values below i have already been written, so assuming init
                // is allowed
                unsafe {
                    for j in 0..i {
                        ptr::drop_in_place(half_filled[j].as_mut_ptr());
                    }
                }
                return FixedCollector(Err(eyre::format_err!("Too few elements to collect")));
            }
        }

        // Safety: The whole array has been filled by now, so assuming the whole thing is init should be
        // fine. The only issue is that we can't do that with stable rust yet, so instead we have to go
        // for a sketchy pointer cast.
        let output = unsafe { (&half_filled as *const _ as *const [T; N]).read() };

        if input.next().is_some() {
            return FixedCollector(Err(eyre::format_err!("Too many elements to collect")));
        } else {
            FixedCollector(Ok(output))
        }
    }
}
