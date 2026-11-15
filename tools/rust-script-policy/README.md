# Rust-first script and workflow policy

This first-party Rust CLI validates repository-local policy without Python.

```sh
cargo fmt --manifest-path tools/rust-script-policy/Cargo.toml --check
cargo clippy --manifest-path tools/rust-script-policy/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path tools/rust-script-policy/Cargo.toml --all-targets
cargo run --quiet --manifest-path tools/rust-script-policy/Cargo.toml -- all .
```

Commands: `scripts <root>` rejects new tracked `.py`/`.pyw` files;
`workflows <root>` checks immutable external Action/Docker pins;
`json <file>` validates JSON syntax; `all <root>` runs all three.

## Python automation migration inventory

Four existing scripts are temporarily grandfathered **only in the public
`daedalus-fab/.github` repository**, not in other repositories:

- `scripts/repository_relationships_lib.py` — graph construction and privacy
- `scripts/validate_baseline.py` — baseline governance and file checks
- `scripts/validate_repository_relationships.py` — graph semantics
- `scripts/validate_workflow_security.py` — immutable action pins (the live
  workflow migrates to this Rust CLI)

New Python scripts/tests are forbidden; shrink the Rust allowlist as each
script is replaced. Preserve contract behavior and negative tests before
retiring existing tooling. Python application code needs an explicitly
reviewed exception in its owning product repository. Organization-wide policy
does **not** automatically install a required CI check in sibling repos.

Scope: additive governance on the current repository. Does not delete legacy
scripts, alter protected branches, or waive existing CI or security gates.
