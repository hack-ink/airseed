# Workspace Layout Reference

Purpose: Explain the current `airseed` repository layout, which files own which concerns, and
which paths are tracked source versus generated or local runtime state.

Read this when: You are deciding where a change belongs, checking whether the current layout still
matches the implementation, or routing a docs/code question to the right file.

Sources: `Cargo.toml`; `Makefile.toml`; `package.json`; `README.md`; `src/main.rs`; `src/cli.rs`;
`src/wallet.rs`; `scripts/cross-check-address.mjs`; `docs/spec/project-boundaries.md`;
`docs/spec/cli.md`

Depends on: `docs/spec/project-boundaries.md`; `docs/spec/cli.md`

Covers: The tracked workspace layout, ownership boundaries, and the local directories that should
not be treated as repository source.

## Current top-level layout

| Path | Role |
| --- | --- |
| `src/main.rs` | Binary entrypoint and error-reporting bootstrap |
| `src/cli.rs` | CLI argument surface, subcommands, and output dispatch |
| `src/wallet.rs` | Mnemonic generation, key derivation, and EVM address formatting |
| `scripts/cross-check-address.mjs` | Independent `ethers.js` address derivation for cross-checking mnemonic output |
| `Cargo.toml` | Package metadata and Rust dependency graph |
| `package.json` | Minimal Node dependency surface for the cross-check script |
| `Makefile.toml` | Repo-native lint, test, and formatting entrypoints |
| `README.md` | Public overview, safety posture, and build entrypoint |
| `docs/spec/project-boundaries.md` | Normative product scope and secret-handling limits |
| `docs/spec/cli.md` | Normative current CLI contract |
| `docs/runbook/offline-generation.md` | Recommended offline environment and operator sequence |
| `docs/decisions/offline-generation-environment.md` | Durable rationale for the current environment recommendation |
| `docs/` | Agent-facing docs split into `spec`, `runbook`, `reference`, and `decisions` |
| `.github/` | Repository automation such as Dependabot and workflows |

## Documentation placement

- `README.md`: public project overview and setup guidance
- `docs/spec/`: normative contracts for current behavior
- `docs/runbook/`: repeatable execution and validation steps
- `docs/reference/`: current layout and implementation notes
- `docs/decisions/`: durable rationale for accepted tradeoffs

## Local-only and generated directories

These paths are intentionally not part of the tracked source layout:

- `target/`: Rust build outputs and local artifacts
- `.worktrees/`: local git worktree lanes
- `.workspaces/`: local clone-backed workspace lanes from older workflows
- `.codex/`: local agent/runtime state

## Structure assessment

The current repository layout is intentionally shallow:

- source lives under `src/`
- package and automation metadata live at the top level
- durable docs live under `docs/`

The current code under `src/` is now the minimal implementation surface for the tool:

- `src/cli.rs` owns the user-facing command shape
- `src/wallet.rs` owns the deterministic derivation logic and formatting helpers
- `src/main.rs` stays thin and delegates immediately into the CLI

The repository also carries a minimal Node script surface for cross-checking:

- `scripts/cross-check-address.mjs` derives the same EVM address through `ethers.js`
- `package.json` exists only to support that independent cross-check path
