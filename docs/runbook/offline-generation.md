# Offline Generation Runbook

Goal: Choose and execute the safest currently practical workflow for generating a mnemonic with
minimal host residue when no hardware wallet is available.

Read this when: You need the current recommended operator path or need to decide between hardware
wallets, Tails, a virtual machine, and Docker.

Inputs: Access to a compatible x86-64 PC; a Windows machine that can prepare a Tails USB; the
official Tails install guides; a reviewed offline binary.

Depends on: `docs/spec/project-boundaries.md`; `docs/decisions/offline-generation-environment.md`

Verification: You can justify the environment choice before generation, and if you proceed, you
finish with a handwritten mnemonic and a powered-off live system.

## 1. Prefer the stronger option first

- If real funds are involved and a hardware wallet is available, stop here and use the hardware
  wallet instead.
- If no hardware wallet is available, continue with the live-USB route below.

## 2. Use Tails on compatible x86-64 hardware

- Use the Windows machine to prepare a Tails USB by following the official Windows install guide.
- As of April 13, 2026, Tails documents an install flow that uses an intermediary Tails and a
  primary Tails on Windows.
- Do not plan around Apple Silicon Macs for this workflow. As of April 13, 2026, the official
  macOS guide says Tails does not work on Macs with Apple chips.
- Boot Tails on compatible x86-64 hardware rather than using Docker or a virtual machine.

## 3. Treat weaker isolation options as fallbacks only

- Use a virtual machine only if the host operating system and virtualization software are trusted
  and leaving traces on the host disk is acceptable.
- Do not treat Docker as an air gap or a no-trace environment. Docker isolation still depends on
  the host, the Docker daemon normally runs with elevated privileges, and Docker tmpfs mounts can
  still be written to swap.

## 4. Keep the generation session minimal

- Keep the live session offline if the chosen workflow allows it.
- Run only the minimal reviewed generator.
- Prefer `airseed generate` for new material.
- If you must re-derive from an existing mnemonic, prefer piping it into `airseed derive --stdin`
  instead of passing it directly on the command line.
- Handwrite the mnemonic instead of photographing, screenshotting, syncing, or copying it into a
  notes app.
- Shut the machine down after the session is complete.

## 5. Keep expectations aligned with the current implementation

- The checked-in binary now implements generation and derivation, but it is still a minimal tool
  rather than a wallet.
- The repository is not presented as audited software.
- For larger or long-lived holdings, a hardware wallet remains the stronger recommendation.
