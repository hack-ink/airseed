//! `airseed` binary.

mod cli;
mod wallet;
mod prelude {
	pub use color_eyre::{Result, eyre};
}

use clap::Parser;

use crate::{cli::Cli, prelude::Result};

fn main() -> Result<()> {
	color_eyre::install()?;

	Cli::parse().run()
}
