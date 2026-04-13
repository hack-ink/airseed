use clap::{
	Args, Parser, Subcommand,
	builder::{
		Styles,
		styling::{AnsiColor, Effects},
	},
};

use crate::{
	prelude::{Result, eyre},
	wallet,
};

/// `airseed` CLI.
#[derive(Debug, Parser)]
#[command(
	version = concat!(
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("VERGEN_GIT_SHA"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	rename_all = "kebab",
	styles = styles(),
)]
pub struct Cli {
	#[command(subcommand)]
	command: Command,
}
impl Cli {
	pub fn run(&self) -> Result<()> {
		let output = match &self.command {
			Command::Generate(arguments) => wallet::generate(
				wallet::validate_word_count(arguments.words)?,
				&wallet::GenerationOptions {
					path: arguments.path().to_owned(),
					show_mnemonic: true,
					show_seed: arguments.show_seed(),
					show_private_key: arguments.show_private_key(),
				},
			)?,
			Command::Derive(arguments) => {
				let options = wallet::GenerationOptions {
					path: arguments.path().to_owned(),
					show_mnemonic: arguments.show_mnemonic(),
					show_seed: arguments.show_seed(),
					show_private_key: arguments.show_private_key(),
				};

				match (&arguments.mnemonic, arguments.stdin) {
					(Some(mnemonic), false) => wallet::derive(mnemonic, &options)?,
					(None, true) => {
						let mnemonic = wallet::read_secret_from_stdin()?;

						wallet::derive_secret(mnemonic, &options)?
					},
					(Some(_), true) => {
						return Err(eyre::eyre!(
							"Use either --mnemonic or --stdin for derive, not both."
						));
					},
					(None, false) => {
						return Err(eyre::eyre!(
							"Provide a mnemonic with --mnemonic or pipe one in with --stdin."
						));
					},
				}
			},
		};

		println!("{}", output.render());

		Ok(())
	}
}

#[derive(Debug, Subcommand)]
enum Command {
	/// Generate a fresh mnemonic and derive the default EVM wallet material from it.
	Generate(GenerateArgs),
	/// Derive wallet material from an existing mnemonic.
	Derive(DeriveArgs),
}

#[derive(Debug, Args)]
struct GenerateArgs {
	/// Number of words in the generated BIP39 mnemonic.
	#[arg(long, default_value_t = 24)]
	words: usize,
	#[command(flatten)]
	common: CommonArgs,
}
impl GenerateArgs {
	fn path(&self) -> &str {
		&self.common.path
	}

	fn show_seed(&self) -> bool {
		self.common.show_seed
	}

	fn show_private_key(&self) -> bool {
		self.common.show_private_key
	}
}

#[derive(Debug, Args)]
struct DeriveArgs {
	/// Existing mnemonic phrase. This may be retained by shell history on shared systems.
	#[arg(long, value_name = "MNEMONIC", conflicts_with = "stdin")]
	mnemonic: Option<String>,
	/// Read the mnemonic phrase from standard input.
	#[arg(long, conflicts_with = "mnemonic")]
	stdin: bool,
	/// Echo the mnemonic back to stdout. Off by default to reduce terminal residue.
	#[arg(long)]
	show_mnemonic: bool,
	#[command(flatten)]
	common: CommonArgs,
}
impl DeriveArgs {
	fn path(&self) -> &str {
		&self.common.path
	}

	fn show_seed(&self) -> bool {
		self.common.show_seed
	}

	fn show_mnemonic(&self) -> bool {
		self.show_mnemonic
	}

	fn show_private_key(&self) -> bool {
		self.common.show_private_key
	}
}

#[derive(Debug, Args)]
struct CommonArgs {
	/// BIP32 derivation path used to derive the child private key.
	#[arg(long, default_value = wallet::DEFAULT_DERIVATION_PATH)]
	path: String,
	/// Include the BIP39 seed in the output.
	#[arg(long)]
	show_seed: bool,
	/// Include the derived private key in the output.
	#[arg(long)]
	show_private_key: bool,
}

fn styles() -> Styles {
	Styles::styled()
		.header(AnsiColor::Red.on_default() | Effects::BOLD)
		.usage(AnsiColor::Red.on_default() | Effects::BOLD)
		.literal(AnsiColor::Blue.on_default() | Effects::BOLD)
		.placeholder(AnsiColor::Green.on_default())
}

#[cfg(test)]
mod tests {
	use clap::Parser;

	use crate::{cli::Cli, wallet};

	#[test]
	fn parse_generate_defaults() {
		let cli = Cli::parse_from(["airseed", "generate"]);
		let command = match cli.command {
			super::Command::Generate(arguments) => arguments,
			_ => panic!("expected generate command"),
		};

		assert_eq!(command.words, 24);
		assert_eq!(command.path(), wallet::DEFAULT_DERIVATION_PATH);
		assert!(!command.show_seed());
		assert!(!command.show_private_key());
	}

	#[test]
	fn parse_derive_unsafe_mnemonic() {
		let cli = Cli::parse_from([
			"airseed",
			"derive",
			"--mnemonic",
			"test test test test test test test test test test test junk",
			"--show-private-key",
		]);
		let command = match cli.command {
			super::Command::Derive(arguments) => arguments,
			_ => panic!("expected derive command"),
		};

		assert!(command.mnemonic.is_some());
		assert!(!command.stdin);
		assert!(!command.show_mnemonic());
		assert!(command.show_private_key());
	}
}
