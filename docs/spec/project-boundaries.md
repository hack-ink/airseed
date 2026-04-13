# Project Boundaries

Purpose: Define what `airseed` is allowed to do, what it intentionally does not do, and which
safety boundaries future implementation must preserve.

Status: normative

Read this when: You are implementing or reviewing mnemonic generation, key derivation, output
handling, dependency choices, or environment assumptions.

Not this document: The current scaffold binary behavior or the operator workflow. Use
`docs/spec/cli.md` for the checked-in binary contract and
`docs/runbook/offline-generation.md` for the recommended environment sequence.

Defines:

- the product scope for `airseed`
- the cryptography and dependency posture
- the default handling rules for mnemonic, seed, and private-key material
- the environment and threat-model boundaries

## Product scope

- `airseed` exists to generate mnemonic phrases offline and derive the minimum wallet material the
  operator needs locally.
- `airseed` is not a full wallet, custody platform, browser extension, sync client, or portfolio
  application.
- `airseed` is intended for air-gapped or near-air-gapped use.

## Cryptography and dependencies

- Do not implement mnemonic generation, seed derivation, or key derivation cryptography from
  scratch in this repository.
- Prefer maintained crates with a small and reviewable surface.
- A minimal implementation is expected to use crates in the class of `bip39`, `bip32`,
  `rand_core::OsRng`, and `zeroize`.
- Any future widening of the cryptographic dependency surface must be documented explicitly.

## Secret handling

- Mnemonics, seeds, and derived private keys are secret material.
- The default flow must not write secret material to disk.
- The default flow must not send secret material over the network.
- The default flow must not require screenshots, clipboard export, or cloud backup.
- The default operator path should optimize for human transcription and verification.

## Environment boundaries

- For real funds, the recommended path remains a hardware wallet.
- On general-purpose computers, this project may reduce residue risk but cannot prove that no
  traces exist.
- Docker is not treated as a no-trace boundary.
- Virtual machines are lower-trust than bare-metal boot on a trusted live OS.
- The current documented no-hardware route is Tails booted from USB on compatible x86-64
  hardware.

## Implementation posture

- Keep the CLI small and auditable.
- Avoid background daemons, telemetry, embedded update channels, or long-running services.
- Keep chain-specific logic separate from seed-generation logic.
- If the repo later adds file export, QR export, or clipboard output, that requires explicit
  opt-in and a spec update.

## Change rule

- Any change that broadens network access, persistence, secret-export surface, or cryptographic
  implementation responsibility must update this spec and the related decision record in the same
  change.
