# Desktop application allocation

Verified **2026-08-05**.

Daedalus Fab uses the paired native desktop application standard:

- Rust: [`daedalus-fab/daedalus-desktop.rs`](https://github.com/daedalus-fab/daedalus-desktop.rs) — **planned**, not yet verified as a published repository.
- Flutter: [`daedalus-fab/daedalus-ui.dart`](https://github.com/daedalus-fab/daedalus-ui.dart) — **live**, with current native desktop coverage verified for macOS.

The Rust URL is an allocation target, not proof that the remote exists. Linux and Windows status for either implementation must be verified explicitly rather than inferred.

The live Flutter repository records the companion contract in [`COMPANION_DESKTOP.md`](https://github.com/daedalus-fab/daedalus-ui.dart/blob/main/COMPANION_DESKTOP.md), merged through [PR #4](https://github.com/daedalus-fab/daedalus-ui.dart/pull/4).

## Product boundary

Both implementations should support semantic parity for project/model files, local imports and exports, fabrication jobs, machine and device connections, previews, offline workflows, credential storage, diagnostics, failure recovery, and per-platform hardware integration.

The Rust and Flutter implementations remain independently buildable, testable, releasable applications. Shared schemas, clients, file formats, fixtures, sample projects, and conformance tests should be versioned deliberately.

## Feature-delivery rule

Every desktop-facing change must inspect both implementations, define shared acceptance criteria, update both or record an explicit no-change rationale, and report Rust and Flutter status separately for macOS, Linux, and Windows.

## Project routing

- GitHub Project: [`daedalus-fab-project` — Project 1](https://github.com/orgs/daedalus-fab/projects/1)
- Linear project: `github.com/daedalus-fab`
- Central registry: [`ORESoftware/project-registry`](https://github.com/ORESoftware/project-registry/blob/main/registry/desktop-applications.json)
- Portfolio rollout: [`DEN-2469`](https://linear.app/denman/issue/DEN-2469/roll-out-paired-rust-flutter-desktop-repositories-across-the-portfolio)

Repository creation, renames, transfers, archival, or platform-status changes must update this document, Linear, the central registry, and both companion repositories together.
