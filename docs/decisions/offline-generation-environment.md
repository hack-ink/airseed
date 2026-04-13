# Offline Generation Environment

Status: accepted

Date: 2026-04-13

Context:

- The repository is intended to become a minimal offline mnemonic and key-material generator, not a
  full wallet.
- The operator goal is to see the mnemonic directly, handwrite it, and leave as little residue on
  general-purpose computers as practical.
- The currently available hardware includes an Apple Silicon Mac and a Windows machine.
- As of April 13, 2026, the official Tails documentation says Tails does not work on Macs with
  Apple chips, while the Windows install flow still supports preparing a Tails USB.

Decision:

- For real funds, recommend a hardware wallet as the primary path.
- When no hardware wallet is available, recommend preparing a Tails USB from Windows and booting
  Tails on compatible x86-64 hardware as the best currently practical route.
- Do not position Docker as a no-trace or air-gapped secret-generation environment.
- Treat virtual machines as a lower-trust fallback only when the host and virtualization layer are
  trusted and leaving traces on disk is acceptable.
- Keep the repository scoped to a minimal generator that uses maintained crates instead of
  hand-rolled wallet cryptography.

Alternatives considered:

- Implement a full wallet from scratch.
  Rejected because it widens the review surface and moves too much cryptographic and operational
  responsibility into this repository.
- Use Docker to minimize residue.
  Rejected because Docker security still depends on the host and the Docker daemon, and Docker
  tmpfs mounts can still persist to swap.
- Use a virtual machine as the primary recommendation.
  Rejected because Tails documents that the host and virtualization software can monitor the guest
  and that traces are likely to remain on disk.
- Use the Apple Silicon Mac as the Tails boot target.
  Rejected because the official Tails macOS install page says Tails does not work on Apple-chip
  Macs.

Consequences:

- Repository docs must not imply that Docker provides a no-trace boundary.
- Repository docs must state clearly that commodity machines cannot prove the absence of traces.
- The repo can document and eventually implement the minimal generator, but it should not claim
  production readiness until the real generation path exists and matches the secret-handling
  boundary.
