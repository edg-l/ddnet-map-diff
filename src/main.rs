#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

use color_eyre::Result;

mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::run()?;
    Ok(())
}
