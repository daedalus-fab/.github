## Change summary

Describe the user-visible behavior, repositories/components affected, compatibility impact, and rollback path. Mark non-applicable checks as `N/A` with a reason.

## Review gates

### Change control and dependencies

- [ ] This work is on a topic branch; no direct default-branch commit is required.
- [ ] This pull request is the proposed path into the default branch; generated tools, bots, migrations, and deployments do not write directly to protected branches.
- [ ] The scope is small enough to review, or staged delivery and follow-up pull requests are identified.
- [ ] Cross-repository dependencies are pinned by immutable commit, lockfile, or released Zed package.
- [ ] Shared functionality is imported from its owning repository, and changes respect repository ownership boundaries.
- [ ] Public contracts, generated language types, ORM models, and fixtures are checked against the independent TypeSpec and JSON Schema sources; consumer compatibility was checked.
- [ ] Breaking changes include migration, rollback, and staged rollout notes.

### SQL, persistence, and state

- [ ] No SQL changes, or every declaration has a stable `<organization>.<domain>` namespace, `<domain>_` object prefix where a shared PostgreSQL schema is required, and an explicit owning repository.
- [ ] Domain SQL may remain with its owning org, but identity, ordering, checksums, drift detection, and promotion are registered through `declarative-migrations`.
- [ ] Application startup validates schema compatibility and does not apply production DDL.
- [ ] Destructive changes, tenant isolation, RLS/authorization, idempotency, and state-machine invariants have evidence.

### Infrastructure and security

- [ ] Application manifests remain app-owned; cluster composition is delegated to `oresoftware/k8s-cluster` and shared components to `oresoftware/k8s-libs-and-shared-defs`.
- [ ] Superproject composition follows the organization's reviewed inventory; applications do not become a second cluster control plane.
- [ ] Workloads use least privilege, restricted pod security, explicit network policy, non-root execution, immutable images, and bounded resources where applicable.
- [ ] Secrets, credentials, personal data, and user content are excluded from source, logs, fixtures, and build artifacts.
- [ ] Authentication/authorization failures are fail-closed and sensitive operations are auditable.
- [ ] Conflicts were resolved semantically using both histories; no destructive Git recovery, force pushes, or history rewrites were used.

### Verification and observability

- [ ] Zed lifecycle hooks run deterministic format, lint, build, contract, and publish checks.
- [ ] Unit, integration, adversarial, migration, and end-to-end tests cover the changed behavior in the appropriate test organization or isolated environment, with teardown evidence where applicable.
- [ ] ORES OTEL trace/correlation propagation is present where applicable, with secret and user-content capture disabled by default.
- [ ] Test evidence, residual risks, follow-up work, and any intentionally deferred repositories are listed below.

## Validation evidence and residual risk

Provide exact commands, checks, fixtures, test-org run links, migration/drift results, and known limitations. Explain checks that could not run. Exclude credentials, customer data, private-repository inventory, and sensitive telemetry from the evidence.

## Salvage check

If this PR supersedes or replaces an older one, say which, and name at least one
concrete thing carried forward from it (a test, a fixture, an error message, a
pin, a doc paragraph). See [`docs/pr-salvage-policy.md`](../docs/pr-salvage-policy.md).

- [ ] Supersedes nothing, **or** the salvaged item is named above.
