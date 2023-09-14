use color_eyre::Result;

mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::run_cli()?;
    Ok(())
}
