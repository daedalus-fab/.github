# Desktop application allocation

Verified **2026-08-06**.

Daedalus Fab uses the paired desktop application standard:

- Rust: [`daedalus-fab/daedalus-desktop.rs`](https://github.com/daedalus-fab/daedalus-desktop.rs) — **planned**, not yet verified as a published repository.
- Flutter, current: [`daedalus-fab/daedalus-ui.dart`](https://github.com/daedalus-fab/daedalus-ui.dart) — **live**, with current native desktop coverage verified for macOS.
- Flutter canonical rename target: `daedalus-fab/daedalus-flutter` — planned naming normalization; do not describe as published until the repository is renamed and verified.

Linux and Windows status for either implementation must be verified explicitly rather than inferred.

## Why both Rust and Flutter remain active

The Rust and Flutter workbenches remain first-class side-by-side implementations so the project can compare native hardware integration, graphics performance, file and device workflows, platform coverage, accessibility, developer velocity, Flutter mobile reuse, release engineering, and long-term maintenance using the same fabrication features.

Every desktop-facing feature must inspect both implementations, share acceptance criteria and fixtures, and normally update both. A one-sided change requires a documented no-change rationale and parity gap.

## Rust desktop kit: Rust + Qt through FFI

**Selected strategy:** Rust domain/core logic with Qt presentation and platform integration through a narrow typed FFI boundary, preferably CXX-Qt or an equivalently reviewed bridge.

**WebView policy:** prohibited.

Daedalus is a fabrication/CAD-style workbench with project/model files, previews, device and machine connections, multi-window behavior, native dialogs, graphics surfaces, and potentially demanding hardware integration. Qt provides mature native windowing, accessibility, localization, graphics, file, and device facilities, while Rust remains responsible for validation, domain state, persistence, concurrency, and security-sensitive logic.

FFI rules:

- expose narrow typed commands and immutable/value-oriented models;
- avoid broad QObject exposure and untyped QVariant maps at security boundaries;
- keep machine credentials, project validation, job state, and persistence in Rust;
- keep Qt focused on presentation, native events, accessibility, graphics, and device UI; and
- add ABI/ownership/threading tests around the bridge.

The Rust repository must contain `docs/DESKTOP_TOOLKIT.md` with the Qt major-version policy, FFI ownership rules, rendering choice, supported devices, deep links, tests, packaging, and Flutter companion.

## HTTPS-first deep linking

Canonical form:

```text
https://<verified-daedalus-owned-host>/open/<route>?<bounded-query>
```

Fallback scheme:

```text
daedalus://<route>?<bounded-query>
```

Deep-link routes belong in `daedalus-interfaces` and must be shared by Rust, Flutter, clients, and the browser fallback.

Required behavior:

- support cold-start and already-running delivery;
- validate the exact host, route, project/job/device identifiers, action, and query bounds before crossing FFI;
- never place machine credentials, private project data, model payloads, access tokens, or signed job commands in URLs;
- use one-time audience-bound codes for invitations, imports, or authenticated handoffs;
- require confirmation before opening externally supplied projects or connecting to devices; and
- test macOS, Windows, Linux, Android, and iOS routes plus browser fallback.

Qt receives OS URL/file-open events, but only the shared Rust parser may authorize and dispatch the route.

## Product boundary

Both implementations should support semantic parity for project/model files, imports and exports, fabrication jobs, machine/device connections, previews, offline workflows, credentials, diagnostics, recovery, deep links, and platform-specific hardware integration.

Shared schemas, clients, file formats, route fixtures, sample projects, simulated devices, and conformance tests must be versioned deliberately.

## Repository-local documentation

The live Flutter repository records the companion contract in [`COMPANION_DESKTOP.md`](https://github.com/daedalus-fab/daedalus-ui.dart/blob/main/COMPANION_DESKTOP.md), introduced through [PR #4](https://github.com/daedalus-fab/daedalus-ui.dart/pull/4).

Central toolkit assignments: [`rust-desktop-strategies.md`](https://github.com/ORESoftware/project-registry/blob/main/docs/rust-desktop-strategies.md).

## Project routing

- GitHub Project: [`daedalus-fab-project` — Project 1](https://github.com/orgs/daedalus-fab/projects/1)
- Linear project: `github.com/daedalus-fab`
- Protected portfolio registry: maintained outside this public repository; public documentation must not expose private locators, credentials, or protected registry paths
- Portfolio rollout: [`DEN-2469`](https://linear.app/denman/issue/DEN-2469/roll-out-paired-rust-flutter-desktop-repositories-across-the-portfolio)

Repository creation, Flutter renaming, toolkit/FFI changes, deep-link changes, transfers, archival, or platform-status changes must update this document, Linear, the protected registry/strategy, and both companion repositories together.
