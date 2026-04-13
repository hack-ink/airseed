use std::io::{self, Read as _};

use bip32::{DerivationPath, XPrv};
use bip39::{Language, Mnemonic};
use rand_core::OsRng;
use sha3::{Digest, Keccak256};
use zeroize::Zeroizing;

use crate::prelude::{Result, eyre};

pub const DEFAULT_DERIVATION_PATH: &str = "m/44'/60'/0'/0/0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerationOptions {
	pub path: String,
	pub show_mnemonic: bool,
	pub show_seed: bool,
	pub show_private_key: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletMaterial {
	pub mnemonic: Option<String>,
	pub derivation_path: String,
	pub address: String,
	pub show_mnemonic: bool,
	pub seed_hex: Option<String>,
	pub private_key_hex: Option<String>,
}
impl WalletMaterial {
	pub fn render(&self) -> String {
		let mut lines = Vec::new();

		if self.show_mnemonic {
			let mnemonic = self
				.mnemonic
				.as_deref()
				.expect("show_mnemonic requires mnemonic text to be present");

			lines.push(format!("Mnemonic: {mnemonic}"));
			lines.push(String::new());
			lines.push(String::from("Mnemonic (numbered):"));

			for (index, word) in mnemonic.split_whitespace().enumerate() {
				lines.push(format!("{:>2}. {word}", index + 1));
			}

			lines.push(String::new());
		}

		lines.push(format!("Derivation path: {}", self.derivation_path));
		lines.push(format!("Ethereum address: {}", self.address));

		if let Some(seed_hex) = &self.seed_hex {
			lines.push(format!("BIP39 seed: {seed_hex}"));
		}
		if let Some(private_key_hex) = &self.private_key_hex {
			lines.push(format!("Private key: {private_key_hex}"));
		}

		lines.join("\n")
	}
}

pub fn generate(word_count: usize, options: &GenerationOptions) -> Result<WalletMaterial> {
	let mut rng = OsRng;
	let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, word_count)?;

	derive_material(mnemonic, options)
}

pub fn derive(mnemonic: &str, options: &GenerationOptions) -> Result<WalletMaterial> {
	let normalized = normalize_mnemonic(mnemonic);

	derive_normalized_mnemonic(normalized.as_str(), options)
}

pub fn derive_secret(
	mnemonic: Zeroizing<String>,
	options: &GenerationOptions,
) -> Result<WalletMaterial> {
	let normalized = normalize_mnemonic(mnemonic.as_str());

	derive_normalized_mnemonic(normalized.as_str(), options)
}

pub fn read_secret_from_stdin() -> Result<Zeroizing<String>> {
	let mut buffer = Zeroizing::new(String::new());

	io::stdin().read_to_string(&mut buffer)?;

	if buffer.trim().is_empty() {
		return Err(eyre::eyre!("Stdin did not contain a mnemonic."));
	}

	Ok(buffer)
}

pub fn validate_word_count(word_count: usize) -> Result<usize> {
	match word_count {
		12 | 15 | 18 | 21 | 24 => Ok(word_count),
		_ => Err(eyre::eyre!(
			"Unsupported mnemonic length {word_count}. Use one of: 12, 15, 18, 21, 24."
		)),
	}
}

fn derive_normalized_mnemonic(
	normalized: &str,
	options: &GenerationOptions,
) -> Result<WalletMaterial> {
	let mnemonic = Mnemonic::parse_normalized(normalized)?;

	derive_material(mnemonic, options)
}

fn derive_material(mnemonic: Mnemonic, options: &GenerationOptions) -> Result<WalletMaterial> {
	let path = options
		.path
		.parse::<DerivationPath>()
		.map_err(|error| eyre::eyre!("Invalid derivation path {}: {error}", options.path))?;
	let seed = Zeroizing::new(mnemonic.to_seed(""));
	let derived_key = XPrv::derive_from_path(seed.as_slice(), &path)?;
	let private_key = Zeroizing::new(derived_key.to_bytes());
	let public_key = derived_key.private_key().verifying_key().to_encoded_point(false);
	let address = checksum_ethereum_address(ethereum_address_bytes(public_key.as_bytes())?);
	let seed_hex = options.show_seed.then(|| encode_prefixed_hex(seed.as_slice()));
	let private_key_hex =
		options.show_private_key.then(|| encode_prefixed_hex(private_key.as_slice()));
	let mnemonic = options.show_mnemonic.then(|| mnemonic.to_string());

	Ok(WalletMaterial {
		mnemonic,
		derivation_path: options.path.clone(),
		address,
		show_mnemonic: options.show_mnemonic,
		seed_hex,
		private_key_hex,
	})
}

fn ethereum_address_bytes(encoded_public_key: &[u8]) -> Result<[u8; 20]> {
	if encoded_public_key.len() != 65 {
		return Err(eyre::eyre!(
			"Expected an uncompressed public key with 65 bytes, received {} bytes.",
			encoded_public_key.len()
		));
	}

	let digest = Keccak256::digest(&encoded_public_key[1..]);
	let mut address = [0_u8; 20];

	address.copy_from_slice(&digest[12..]);

	Ok(address)
}

fn checksum_ethereum_address(address: [u8; 20]) -> String {
	let lowercase = encode_hex(&address);
	let hash = Keccak256::digest(lowercase.as_bytes());
	let mut checksummed = String::from("0x");

	for (index, ch) in lowercase.chars().enumerate() {
		if ch.is_ascii_digit() {
			checksummed.push(ch);

			continue;
		}

		let nibble = if index % 2 == 0 { hash[index / 2] >> 4 } else { hash[index / 2] & 0x0f };

		if nibble >= 8 {
			checksummed.push(ch.to_ascii_uppercase());
		} else {
			checksummed.push(ch);
		}
	}

	checksummed
}

fn normalize_mnemonic(input: &str) -> Zeroizing<String> {
	let mut normalized = Zeroizing::new(String::with_capacity(input.len()));

	for word in input.split_whitespace() {
		if !normalized.is_empty() {
			normalized.push(' ');
		}

		for ch in word.chars() {
			normalized.extend(ch.to_lowercase());
		}
	}

	normalized
}

fn encode_prefixed_hex(bytes: &[u8]) -> String {
	format!("0x{}", encode_hex(bytes))
}

fn encode_hex(bytes: &[u8]) -> String {
	const HEX: &[u8; 16] = b"0123456789abcdef";

	let mut encoded = String::with_capacity(bytes.len() * 2);

	for byte in bytes {
		encoded.push(char::from(HEX[(byte >> 4) as usize]));
		encoded.push(char::from(HEX[(byte & 0x0f) as usize]));
	}

	encoded
}

#[cfg(test)]
mod tests {
	use zeroize::Zeroizing;

	use crate::wallet::{self, DEFAULT_DERIVATION_PATH, GenerationOptions};

	#[test]
	fn derive_matches_known_hardhat_account() {
		let material = wallet::derive(
			"test test test test test test test test test test test junk",
			&GenerationOptions {
				path: String::from(DEFAULT_DERIVATION_PATH),
				show_mnemonic: false,
				show_seed: false,
				show_private_key: true,
			},
		)
		.expect("known mnemonic should derive");

		assert_eq!(material.address, "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
		assert_eq!(
			material.private_key_hex.as_deref(),
			Some("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
		);
	}

	#[test]
	fn derive_secret_matches_known_hardhat_account() {
		let material = wallet::derive_secret(
			Zeroizing::new(String::from(
				"test test test test test test test test test test test junk",
			)),
			&GenerationOptions {
				path: String::from(DEFAULT_DERIVATION_PATH),
				show_mnemonic: false,
				show_seed: false,
				show_private_key: true,
			},
		)
		.expect("known mnemonic should derive");

		assert_eq!(material.address, "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
		assert_eq!(
			material.private_key_hex.as_deref(),
			Some("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
		);
	}

	#[test]
	fn checksum_address_matches_eip55_case() {
		let mut address = [0_u8; 20];

		address.copy_from_slice(&[
			0xf3, 0x9f, 0xd6, 0xe5, 0x1a, 0xad, 0x88, 0xf6, 0xf4, 0xce, 0x6a, 0xb8, 0x82, 0x72,
			0x79, 0xcf, 0xff, 0xb9, 0x22, 0x66,
		]);

		assert_eq!(
			wallet::checksum_ethereum_address(address),
			"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
		);
	}

	#[test]
	fn normalize_mnemonic_collapses_whitespace() {
		assert_eq!(wallet::normalize_mnemonic("  TEST\n test\tjunk  ").as_str(), "test test junk");
	}

	#[test]
	fn validate_word_count_rejects_invalid_lengths() {
		let error = wallet::validate_word_count(13).expect_err("13 words should be rejected");

		assert!(error.to_string().contains("Unsupported mnemonic length 13"));
	}

	#[test]
	fn encode_hex_uses_lowercase() {
		assert_eq!(wallet::encode_hex(&[0xab, 0xcd, 0xef]), "abcdef");
	}

	#[test]
	fn render_hides_mnemonic_when_disabled() {
		let material = wallet::derive(
			"test test test test test test test test test test test junk",
			&GenerationOptions {
				path: String::from(DEFAULT_DERIVATION_PATH),
				show_mnemonic: false,
				show_seed: false,
				show_private_key: false,
			},
		)
		.expect("known mnemonic should derive");
		let rendered = material.render();

		assert!(!rendered.contains("Mnemonic:"));
		assert!(rendered.contains("Ethereum address:"));
	}

	#[test]
	fn render_shows_mnemonic_when_enabled() {
		let material = wallet::derive(
			"test test test test test test test test test test test junk",
			&GenerationOptions {
				path: String::from(DEFAULT_DERIVATION_PATH),
				show_mnemonic: true,
				show_seed: false,
				show_private_key: false,
			},
		)
		.expect("known mnemonic should derive");
		let rendered = material.render();

		assert!(rendered.contains("Mnemonic: test test test"));
		assert!(rendered.contains("Mnemonic (numbered):"));
	}
}
