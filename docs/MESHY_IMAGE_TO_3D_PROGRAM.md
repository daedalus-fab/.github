# Meshy image-to-3D delivery program

This document maps the Meshy image-to-3D program across Linear, GitHub Project 1, repositories, pull requests, and release evidence.

## Planning records

- Linear foundation: **DEN-2465** — complete.
- Linear operationalization: **DEN-2506** — in progress.
- Linear milestone: **Meshy image-to-3D operationalization**.
- Linear project: **github.com/daedalus-fab**.
- Linear architecture document: **Meshy image-to-3D provider integration and operationalization — DEN-2465 / DEN-2506**.
- Organization tracking issue: `daedalus-fab/.github#9`.
- GitHub Project: **daedalus-fab-project**, organization project 1.

DEN-2506 remains the cross-repository outcome. Individual repository items may move to Done after merge and CI, but DEN-2506 remains open until the operational acceptance criteria below are verified.

## Repository allocation

| Repository | Responsibility | Current evidence |
| --- | --- | --- |
| `fabrication-server.rs` | Shared `dd-meshy-client`, bounded workers, durable downloads, checksums, provenance, mesh repair, and additive preflight. | PR #5 merged for the provider client. |
| `daedalus-api-server.rs` | Shared-auth identity, provider-neutral API, owner-scoped job persistence, local IDs, task-ID binding, normalized status, and release blocking. | PR #8. |
| `daedalus-clients` | Typed client methods for create, status, progress, and review operations. | Follow-up under DEN-2506. |
| Flutter and Rust desktop workbenches | Asset selection, progress, previews, dimension evidence, and explicit candidate disposition. | Follow-up under DEN-2506. |
| `daedalus-fab-mcp-server.rs` | Provider-neutral tools routed through Daedalus authorization and job control. | Follow-up under DEN-2506. |
| `daedalus-fab-e2e` | Cross-repository create, progress, ingest, candidate, review-blocking, and recovery tests. | Follow-up under DEN-2506. |
| `k8s-cluster` and shared definitions | Deployment wiring, durable queue/database definitions, secret delivery, and observability. | Follow-up under DEN-2506 and durable-execution work. |

The API server consumes the shared provider crate instead of implementing provider HTTP a second time. Provider success is normalized as candidate geometry only.

## GitHub Project 1 representation

Add each executable repository issue or pull request to `daedalus-fab-project` and link it to DEN-2506.

Recommended field values:

| Field | Meshy program use |
| --- | --- |
| Linear issue | `DEN-2506` for operationalization; `DEN-2465` for the completed client foundation. |
| Workstream | provider, API, worker, storage, preflight, clients, desktop, MCP, e2e, or operations. |
| Repository | Repository that owns the next executable transition. |
| Status | Backlog, Ready, In progress, In review, Blocked, or Done. |
| Risk | On track, At risk, or Blocked. |
| Evidence | Pull request, workflow run, durable artifact record, drill, or deployment record. |

Recommended saved view:

- **Meshy delivery** — filter by Linear issue DEN-2506, group by workstream, then repository.

State rules:

1. Branch or issue created: add the repository item to Project 1.
2. Pull request opened: set the repository item to In review when review/CI is the next gate.
3. CI failure: retain In review and mark Risk as At risk.
4. Pull request merged: mark that repository item Done.
5. Keep DEN-2506 In Progress while any cross-repository acceptance criterion remains open.
6. Attach durable-ingestion, preflight, smoke-test, and recovery evidence to Linear before completing DEN-2506.

## Manufacturing invariant

A Meshy task that reports success is not released manufacturing geometry.

```json
{
  "provider_success": "candidate_geometry_only",
  "review_state": "needs_review",
  "release_state": "blocked"
}
```

No Project item may claim machine-ready completion merely because a provider task completed.

## Operational acceptance criteria

DEN-2506 is complete only after all of the following are demonstrated:

1. Billable task creation is idempotent and executes through a bounded worker lane.
2. Progress uses server-sent events with bounded polling fallback.
3. Webhook notifications trigger an authenticated re-fetch of authoritative provider state.
4. Requested outputs are copied into Daedalus-controlled storage before provider URL expiry.
5. Source and output hashes, byte counts, media types, model/options, timestamps, expiry, and consumed credits are recorded.
6. A review-blocked candidate `fab_designs` record is created.
7. STL/3MF normalization, units and scale evidence, topology, manifold/watertightness, wall-thickness, and orientation checks run.
8. An explicit accept, reconstruct, regenerate, or reject decision is recorded.
9. Retry/backoff, dead-letter handling, metrics, quotas, and provider budget controls are present.
10. Generated clients, paired desktop review flows, MCP tools, and cross-repository e2e coverage are delivered.
11. One explicitly approved, credit-capped live smoke task completes through durable ingestion and remains machine-release blocked.

## Pull-request linking

Use the organization convention from `LINEAR_AND_GITHUB_PROJECTS.md`:

- include `DEN-2506` in the pull-request title or body;
- use `Related: DEN-2506` for repository slices that must not close the parent outcome;
- attach the pull request and final workflow evidence to Linear;
- merge only after the exact proposed commit has passed its required checks.

## Current execution links

- Foundation provider client: https://github.com/daedalus-fab/fabrication-server.rs/pull/5
- API orchestration: https://github.com/daedalus-fab/daedalus-api-server.rs/pull/8
- Organization issue: https://github.com/daedalus-fab/.github/issues/9
- Linear operationalization: https://linear.app/denman/issue/DEN-2506/fabrication-serverrs-operationalize-meshy-generation-jobs-and-durable
