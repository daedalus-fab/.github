# Meshy delivery status — 2026-08-07

This file is a durable execution snapshot for the Meshy image-to-3D program. The living program definition remains `MESHY_IMAGE_TO_3D_PROGRAM.md`; Linear DEN-2506 remains the cross-repository outcome.

## Merged slices

| Slice | Pull request | Merge commit |
| --- | --- | --- |
| Shared provider client | `daedalus-fab/fabrication-server.rs#5` | `f62ea9bed671270d02e7217ea0bf20dc2755c48a` |
| Provider-neutral API | `daedalus-fab/daedalus-api-server.rs#8` | `632623740efdc196113734d7df1f883c5f756b6a` |
| Project operating documentation | `daedalus-fab/.github#11` | `599384e5ee89c8e75e7ea3e0efeb56d3e4d59549` |
| Durable worker and verified archival | `daedalus-fab/fabrication-server.rs#7` | `06686e967097b0e1eb05963d2373fd97e0244255` |

## Durable worker evidence

The exact PR #7 head `4e107049073f2f2b902c13f3de50818cf2427f35` passed:

- Formal methods run `31051099240`;
- durable Meshy workflow run `31051099270`;
- root CI run `31051099550`.

Delivered behavior includes:

- a persisted submission intent before any potentially billable Meshy create;
- no automatic re-create after process loss or an ambiguous create response;
- explicit task adoption for reconciliation;
- Fiducia-fenced RDS checkpoints and lease heartbeats;
- allowlisted HTTPS artifact downloads with redirects disabled and streamed byte caps;
- source/output SHA-256, byte count, media type, expiry, and consumed-credit evidence;
- idempotent verified directory/PVC archive receipts;
- immutable `needs_review`, `release_state = blocked`, and `machine_ready = false` results.

No live Meshy task was submitted and no provider credits were consumed by CI.

## Deployment pin

`ORESoftware/k8s-cluster#1077` pins fabrication-server merge `06686e967097b0e1eb05963d2373fd97e0244255` and is based on current k8s `main` at exact head `6ad142abf7b7a737babca47028c30c677ae64c76`.

Passing checks:

- secret scan `31209319898`;
- cluster e2e `31209320003`;
- OpenAPI contracts `31209320189`;
- workflow branch trigger contract `31209320327`.

The exact path-dependency compile remains blocked before checkout by protected configuration:

- DEN-1537 / `ORESoftware/k8s-cluster#886`: `K8S_SUBMODULE_APP_ID` and `K8S_SUBMODULE_APP_PRIVATE_KEY` are absent;
- DEN-2284 / `ORESoftware/k8s-cluster#997`: the dedicated non-empty `K8S_LIBS_DEPLOY_KEY` does not authenticate to the pinned `remote/libs` repository.

DEN-2332 was canceled as a duplicate of DEN-1537. No PAT fallback, long-lived installation token, credential-bearing URL, or broadened private-repository allowlist was introduced.

## Project-state mirror

| Workstream | Evidence | Project state |
| --- | --- | --- |
| Provider | fabrication-server #5 | Done |
| API | daedalus-api-server #8 | Done |
| Project documentation | .github #11 | Done |
| Durable worker / verified PVC archive contract | fabrication-server #7 | Done |
| Deployment pin / exact superproject compile | k8s-cluster #1077 | Blocked by DEN-1537 and DEN-2284 |
| Production S3/R2 archive | DEN-2506 follow-up | Backlog |
| Webhook and SSE | DEN-2506 follow-up | Backlog |
| Candidate design and deterministic preflight | DEN-2506 follow-up | Backlog |
| Clients, paired desktops, MCP, and e2e | DEN-2506 follow-up | Backlog |
| Credit-capped live smoke | DEN-2506 follow-up | Blocked until production controls exist |

## Coordination

- Linear issue: DEN-2506
- Organization tracker: `daedalus-fab/.github#9`
- GitHub Project: `daedalus-fab-project`, organization project 1
- Slack channel: `#daedalus-fab`

Parallel agents must reuse the existing issue and tracker, link overlapping pull requests, and avoid duplicate credential, storage, webhook, preflight, client, desktop, MCP, or e2e tickets.
