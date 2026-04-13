# CLI Contract

Purpose: Define the current normative command-line contract for the checked-in `airseed` binary.

Status: normative

Read this when: You are implementing, reviewing, or replacing the current binary entrypoint,
argument surface, output surface, or mnemonic-derivation behavior.

Not this document: The product safety boundary or the recommended operator workflow. Use
`docs/spec/project-boundaries.md` for scope and secret-handling rules,
`docs/runbook/offline-generation.md` for the current environment recommendation, and
`docs/decisions/` for durable tradeoffs.

Defines:

- the current single-binary CLI shape
- the supported subcommands and their default behavior
- the current output surface for generated and derived wallet material

## Binary shape

- The repository currently builds a single binary entrypoint from `src/main.rs`.
- The CLI surface is defined through `clap::Parser` and `clap::Subcommand` in `src/cli.rs`.
- The binary name is `airseed`.
- The checked-in binary installs `color-eyre` and then dispatches directly into the CLI command.
- The binary does not configure file logging, telemetry, or a background runtime.
- `--version` prints the package version, git SHA when available, and target triple.

## Subcommands

- `generate`
  - generates a fresh English BIP39 mnemonic
  - accepts `--words` with one of `12`, `15`, `18`, `21`, or `24`
  - derives wallet material at the selected `--path`, defaulting to `m/44'/60'/0'/0/0`
  - prints the mnemonic and Ethereum address
  - prints the BIP39 seed only when `--show-seed` is set
  - prints the derived private key only when `--show-private-key` is set
- `derive`
  - derives wallet material from an existing mnemonic
  - accepts either `--mnemonic <TEXT>` or `--stdin`
  - does not print the mnemonic by default
  - prints the mnemonic only when `--show-mnemonic` is set
  - uses the same `--path`, `--show-seed`, and `--show-private-key` flags as `generate`

## Output surface

- `generate` renders the mnemonic as both a single line and numbered words in output order.
- The derivation path is always printed.
- The address is rendered as an EIP-55-checksummed Ethereum address.
- `derive` suppresses mnemonic output by default to reduce terminal residue.
- Secret material other than the generated mnemonic is printed only when the relevant flag is
  explicitly set.

## Change rule

- Any change to the supported subcommands, default derivation path, chain output, or secret-output
  flags must update this spec in the same change.
