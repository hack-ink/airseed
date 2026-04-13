<div align="center">

# airseed

Offline-first wallet seed generator for air-gapped environments.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language Checks](https://github.com/hack-ink/airseed/actions/workflows/language.yml/badge.svg?branch=main)](https://github.com/hack-ink/airseed/actions/workflows/language.yml)
[![Release](https://github.com/hack-ink/airseed/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/airseed/actions/workflows/release.yml)

</div>

## Feature Highlights

- Generate fresh BIP39 mnemonics from a small offline-first Rust CLI.
- Derive the default EVM wallet material from a fresh or existing mnemonic.
- Intended for air-gapped workflows, not a full wallet.
- Explicit safety boundary: no hand-rolled cryptography, no default secret persistence, and no
  network path in the generation flow.
- Current strongest no-hardware route documented in this repo: prepare a Tails USB on Windows and
  boot it on compatible x86-64 hardware.

## Status

- Project direction: minimal offline mnemonic and key material generator.
- Current implementation state: `generate` and `derive` commands are implemented.
- Current output surface: English BIP39 mnemonic, default `m/44'/60'/0'/0/0` child key material,
  and EVM address derivation.
- Safety status: usable as a minimal tool, but still unaudited and not positioned as the safest
  choice for real funds.
- Production recommendation today: use a hardware wallet for real assets.
- Current fallback recommendation without hardware wallet: boot Tails from USB, generate offline,
  handwrite the mnemonic, and power the machine off.

## Usage

### Installation

#### Build from Source

```sh
git clone https://github.com/hack-ink/airseed
cd airseed
cargo build --release
```

### Interaction

Generate a fresh 24-word mnemonic and derive the default EVM address:

```sh
cargo run -- generate
```

Generate a 12-word mnemonic and include the derived private key:

```sh
cargo run -- generate --words 12 --show-private-key
```

Derive the same wallet material from an existing mnemonic through standard input:

```sh
printf '%s\n' 'test test test test test test test test test test test junk' \
  | cargo run -- derive --stdin --show-private-key
```

Echo the mnemonic during `derive` only when you intentionally want it in stdout:

```sh
printf '%s\n' 'test test test test test test test test test test test junk' \
  | cargo run -- derive --stdin --show-mnemonic
```

Cross-check the derived address with `ethers.js` in a separate implementation:

```sh
npm install
printf '%s\n' 'test test test test test test test test test test test junk' \
  | npm run cross-check -- --stdin --expected 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
```

### Safety Notes

- The target tool should stay minimal, auditable, and offline-first.
- Do not implement wallet cryptography from scratch. Use maintained crates in the class of
  `bip39`, `bip32`, `rand_core::OsRng`, and `zeroize`.
- Prefer `derive --stdin` over passing a mnemonic directly in command arguments, because shell
  history may retain command lines.
- `derive --stdin` and `npm run cross-check -- --stdin` do not echo the mnemonic unless
  `--show-mnemonic` is explicitly set.
- Prefer `cross-check` over `double check` when you are verifying the same mnemonic and path in an
  independent implementation.
- Do not treat Docker as a no-trace boundary.
- Treat virtual machines as a lower-trust fallback than a live USB boot.
- On commodity computers, the goal is to reduce residue, not to prove the complete absence of
  traces.

### Documentation

- [Project boundaries](docs/spec/project-boundaries.md)
- [Offline generation runbook](docs/runbook/offline-generation.md)
- [Environment decision record](docs/decisions/offline-generation-environment.md)

### References

- [Tails home](https://tails.net/)
- [Install Tails from macOS](https://tails.net/install/mac/)
- [Install Tails from Windows](https://tails.net/install/windows/)
- [Running Tails in a virtual machine](https://tails.net/doc/advanced_topics/virtualization/index.en.html)
- [Docker Engine security](https://docs.docker.com/engine/security/)
- [Docker tmpfs mounts](https://docs.docker.com/engine/storage/tmpfs/)
- [Ethereum security guidance](https://ethereum.org/security/)

## Development

### Architecture

- `src/cli.rs` defines the command-line interface and command dispatch.
- `src/wallet.rs` owns mnemonic generation, key derivation, and Ethereum address formatting.
- `docs/spec/project-boundaries.md` defines the future implementation boundary.
- `docs/runbook/offline-generation.md` defines the recommended operator workflow.

### Checks

```sh
cargo make checks
cargo make test
cargo run -- --help
```

## Support Me

If you find this project helpful and would like to support its development, you can buy me a coffee!

Your support is greatly appreciated and motivates me to keep improving this project.

- **Fiat**
    - [Ko-fi](https://ko-fi.com/hack_ink)
    - [Afdian](https://afdian.com/a/hack_ink)
- **Crypto**
    - **Bitcoin**
        - `bc1pedlrf67ss52md29qqkzr2avma6ghyrt4jx9ecp9457qsl75x247sqcp43c`
    - **Ethereum**
        - `0x3e25247CfF03F99a7D83b28F207112234feE73a6`
    - **Polkadot**
        - `156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y`

Thank you for your support!

## Appreciation

We would like to extend our heartfelt gratitude to the following projects and contributors:

- The Rust community for their continuous support and development of the Rust ecosystem.

## Additional Acknowledgements

- TODO

<div align="right">

### License

<sup>Licensed under [GPL-3.0](LICENSE).</sup>

</div>
