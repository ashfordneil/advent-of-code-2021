//! Utilities for writing shorter scripts each day.

use structopt::StructOpt;

/// Set everything up to start running. The logging is maybe premature at this stage, but I'd rather
/// not have to add it when I'm in the middle of solving a problem.
pub fn setup<A: StructOpt>() -> eyre::Result<A> {
    color_eyre::install()?;
    env_logger::try_init()?;

    Ok(A::from_args())
}
