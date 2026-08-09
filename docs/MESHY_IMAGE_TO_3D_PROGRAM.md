# Meshy image-to-3D delivery program

This document maps the Meshy image-to-3D program across Linear, GitHub Project 1, repositories, pull requests, deployment evidence, and manufacturing-release controls.

## Planning records

- Linear foundation: **DEN-2465** — complete.
- Linear operationalization: **DEN-2506** — in progress.
- Linear milestone: **Meshy image-to-3D operationalization**.
- Linear project: **github.com/daedalus-fab**.
- Linear architecture document: **Meshy image-to-3D provider integration and operationalization — DEN-2465 / DEN-2506**.
- Organization tracking issue: `daedalus-fab/.github#9`.
- GitHub Project: **daedalus-fab-project**, organization project 1.
- Slack coordination: `#daedalus-fab`.

DEN-2506 is the cross-repository outcome. A repository item may move to Done after its pull request merges and exact-head CI evidence is recorded, but DEN-2506 remains open until deployment, durable production storage, fabrication review, product surfaces, recovery evidence, and the approved live smoke are complete.

## Current execution summary

| Workstream | Evidence | State |
| --- | --- | --- |
| Shared provider client | `fabrication-server.rs#5` → `f62ea9bed671270d02e7217ea0bf20dc2755c48a` | Done |
| Provider-neutral API | `daedalus-api-server.rs#8` → `632623740efdc196113734d7df1f883c5f756b6a` | Done |
| Organization/project documentation | `.github#11` → `599384e5ee89c8e75e7ea3e0efeb56d3e4d59549` | Done |
| Durable worker and verified directory/PVC archive contract | `fabrication-server.rs#7` → `06686e967097b0e1eb05963d2373fd97e0244255` | Done |
| Deployment pin and real superproject compile | `ORESoftware/k8s-cluster#1077` | Blocked by DEN-1537 and DEN-2284 |
| Production S3/R2 archive | DEN-2506 follow-up | Backlog |
| Webhook and SSE | DEN-2506 follow-up | Backlog |
| Candidate `fab_designs` and deterministic preflight | DEN-2506 follow-up | Backlog |
| Clients, paired desktops, MCP, and e2e | DEN-2506 follow-up | Backlog |
| Credit-capped live smoke | DEN-2506 follow-up | Blocked until production controls exist |

A dated execution snapshot is available in `MESHY_DELIVERY_STATUS_2026-08-07.md`.

## Repository allocation

| Repository | Responsibility | Current evidence |
| --- | --- | --- |
| `fabrication-server.rs` | Shared `dd-meshy-client`, resumable `dd-meshy-job`, Fiducia-fenced worker integration, durable downloads, checksums, verified archival, mesh repair, and additive preflight. | PR #5 and PR #7 merged. |
| `daedalus-api-server.rs` | Shared-auth identity, provider-neutral API, owner-scoped jobs, local IDs, task-ID binding, normalized status, and release blocking. | PR #8 merged. |
| `daedalus-clients` | Typed client methods for create, status, progress, and review operations. | Follow-up under DEN-2506. |
| Flutter and Rust desktop workbenches | Asset selection, progress, previews, dimension evidence, and explicit candidate disposition. | Follow-up under DEN-2506. |
| `daedalus-fab-mcp-server.rs` | Provider-neutral tools routed through Daedalus authorization and job control. | Follow-up under DEN-2506. |
| `daedalus-fab-e2e` | Cross-repository create, progress, ingest, candidate, review-blocking, and recovery tests. | Follow-up under DEN-2506. |
| `ORESoftware/k8s-cluster` | Exact deployment gitlink, protected secret delivery, real shared-path compilation, JetStream consumer wiring, and runtime configuration. | PR #1077 open and blocked by protected CI configuration. |
| shared definitions and object storage | Canonical database/NATS definitions, S3/R2 archive contract, IAM, retention, and lifecycle. | Follow-up under DEN-2506 and durable-execution work. |

Provider HTTP is implemented once in `dd-meshy-client`. The API and durable worker consume the shared client instead of implementing bearer auth, endpoints, provider errors, retry metadata, or task models again.

## Delivered durable worker contract

Merged PR `fabrication-server.rs#7` adds:

- a standalone `dd-meshy-job` stage engine;
- a persisted `SubmissionCheckpoint::Prepared` before a potentially billable create;
- an in-memory submission permit that cannot survive a process restart;
- refusal to automatically recreate when a persisted intent has no provider task ID;
- ambiguous-outcome checkpoints for transport, timeout, and unreadable create responses;
- explicit `existing_provider_task` adoption for reconciliation;
- Fiducia-fenced RDS ownership, heartbeats, checkpoints, completion, and retry classification;
- requested-output filtering and provider task-ID binding;
- allowlisted HTTPS artifact downloads on port 443, with redirects disabled and streamed byte limits;
- source/output SHA-256, byte count, media type, expiry, and consumed-credit evidence;
- basic GLB/STL/3MF/OBJ/FBX/USDZ signature validation;
- deterministic non-identifying object keys;
- idempotent verified directory/PVC archival and conflicting-byte rejection;
- one checkpoint per archived format;
- immutable `needs_review`, `release_state = blocked`, and `machine_ready = false` results.

This is credit-spend protection, not a claim that Meshy itself provides a proven idempotency key. A create whose result is unknown requires reconciliation before another create may occur.

### Exact worker validation

Validated head:

```text
4e107049073f2f2b902c13f3de50818cf2427f35
```

- Formal methods run `31051099240` — passed.
- Durable Meshy workflow run `31051099270` — passed.
- Root CI run `31051099550` — passed.

No live Meshy task was submitted and no provider credits were consumed by these tests.

## Deployment-superproject pin

`ORESoftware/k8s-cluster#1077` now pins the merged fabrication-server commit and uses a narrow compile gate at the real path-dependency layout.

Current exact head:

```text
6ad142abf7b7a737babca47028c30c677ae64c76
```

The PR:

- pins `remote/deployments/fabrication-server-rs` to `06686e967097b0e1eb05963d2373fd97e0244255`;
- is semantically rebased onto current k8s `main`;
- checks out only the exact `remote/libs`, public DES, and fabrication-server dependencies;
- uses the dedicated read-only deploy key for `remote/libs`;
- uses owner-scoped GitHub App installation tokens for the fabrication-server gitlink;
- verifies each mode-160000 gitlink and clean nested checkout;
- compiles both `meshy-job-worker` and the existing `dd-fabrication-server` regression target when protected credentials are present.

Passing checks on the current head:

- secret scan `31209319898`;
- cluster e2e `31209320003`;
- OpenAPI contracts `31209320189`;
- workflow branch trigger contract `31209320327`.

The focused compile gate fails before checkout because the repository does not expose `K8S_SUBMODULE_APP_ID` or `K8S_SUBMODULE_APP_PRIVATE_KEY`. The repository-wide private-source gate separately remains blocked because the non-empty `K8S_LIBS_DEPLOY_KEY` cannot authenticate to `ORESoftware/k8s-libs-and-shared-defs`.

Canonical blockers:

- **DEN-1537** / `ORESoftware/k8s-cluster#886` — restore owner-scoped GitHub App secrets;
- **DEN-2284** / `ORESoftware/k8s-cluster#997` — rotate or repair the dedicated read-only `remote/libs` deploy-key pair.

DEN-2332 was canceled as a duplicate of DEN-1537. No PAT fallback, credential-bearing URL, long-lived installation token, or broadened repository allowlist was introduced.

## GitHub Project 1 representation

Add each executable repository issue or pull request to `daedalus-fab-project` and link it to DEN-2506.

Recommended field values:

| Field | Meshy program use |
| --- | --- |
| Linear issue | `DEN-2506` for operationalization; `DEN-2465` for the completed provider foundation. |
| Workstream | provider, API, worker, storage, preflight, clients, desktop, MCP, e2e, or operations. |
| Repository | Repository that owns the next executable transition. |
| Status | Backlog, Ready, In progress, In review, Blocked, or Done. |
| Risk | On track, At risk, or Blocked. |
| Evidence | Pull request, exact-head workflow run, archive receipt, recovery drill, or deployment record. |

Recommended saved view:

- **Meshy delivery** — filter by Linear issue DEN-2506, group by workstream, then repository.

Current item state should mirror the execution summary above: provider/API/docs/worker Done, k8s pin Blocked, and remaining product/operation work Backlog.

State rules:

1. Branch or issue created: add the repository item to Project 1.
2. Pull request opened: set the item to In review when review/CI is the next gate.
3. External protected-configuration failure: retain the PR item, set Status to Blocked and Risk to Blocked, and link the canonical infrastructure issue.
4. Pull request merged: mark that repository item Done.
5. Keep DEN-2506 In Progress while any cross-repository acceptance criterion remains open.
6. Attach durable-ingestion, preflight, smoke-test, and recovery evidence to Linear before completing DEN-2506.

## Manufacturing invariant

A Meshy task that reports success is not released manufacturing geometry.

```json
{
  "provider_success": "candidate_geometry_only",
  "review_state": "needs_review",
  "release_state": "blocked",
  "machine_ready": false
}
```

No Project item may claim machine-ready completion merely because a provider task completed.

## Remaining operational acceptance criteria

DEN-2506 is complete only after all of the following are demonstrated:

1. Protected k8s checkout credentials are restored, PR #1077 compiles both binaries against exact gitlinks, and the deployment pin merges.
2. The production JetStream consumer invokes the durable worker and ACKs only after the corresponding RDS transition commits.
3. Progress uses server-sent events with bounded polling fallback.
4. Webhook notifications trigger an authenticated re-fetch of authoritative provider/RDS state.
5. Requested outputs are copied into immutable S3/R2 storage before provider URL expiry, with least-privilege IAM and lifecycle policy.
6. Source and output hashes, byte counts, media types, model/options, timestamps, expiry, and consumed credits are retained.
7. A review-blocked candidate `fab_designs` record is created from verified archive receipts.
8. STL/3MF normalization, units and scale evidence, topology, normals, manifold/watertightness, wall-thickness, repair, and orientation checks run.
9. An explicit accept, reconstruct, regenerate, or reject decision is recorded.
10. Retry/backoff, dead-letter/reconciliation handling, metrics, alerts, quotas, and provider budget controls are present.
11. Generated clients, paired desktop review flows, MCP tools, and cross-repository e2e/recovery coverage are delivered.
12. One explicitly approved, credit-capped live smoke task completes through durable ingestion and remains machine-release blocked.

## Pull-request linking and coordination

Use the organization convention from `LINEAR_AND_GITHUB_PROJECTS.md`:

- include `DEN-2506` in the pull-request title or body;
- use `Related: DEN-2506` for repository slices that must not close the parent outcome;
- attach the pull request and final workflow evidence to Linear;
- merge only after the exact proposed commit has passed its required checks;
- reuse organization tracker #9 and `#daedalus-fab` for parallel-agent coordination;
- avoid duplicate credential, storage, webhook, preflight, client, desktop, MCP, or e2e tickets.

## Current execution links

- Provider foundation: https://github.com/daedalus-fab/fabrication-server.rs/pull/5
- API orchestration: https://github.com/daedalus-fab/daedalus-api-server.rs/pull/8
- Durable worker: https://github.com/daedalus-fab/fabrication-server.rs/pull/7
- Deployment pin: https://github.com/ORESoftware/k8s-cluster/pull/1077
- Organization tracker: https://github.com/daedalus-fab/.github/issues/9
- GitHub App blocker: https://github.com/ORESoftware/k8s-cluster/issues/886
- Shared-libs deploy-key blocker: https://github.com/ORESoftware/k8s-cluster/issues/997
- Linear operationalization: https://linear.app/denman/issue/DEN-2506/fabrication-serverrs-operationalize-meshy-generation-jobs-and-durable
