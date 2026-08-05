# Linear and GitHub Projects operating model

This document defines how the `daedalus-fab` GitHub organization coordinates planning in Linear with repository execution and evidence in GitHub.

## Canonical systems

| Concern | System of record |
| --- | --- |
| Product intent, priority, ownership, dependencies, milestones, and status narrative | Linear |
| Source, commits, pull requests, reviews, CI, releases, artifacts, and runtime evidence | GitHub repositories |
| Cross-repository execution view for the organization | GitHub Project `daedalus-fab-project` (organization project 1) |
| Organization-wide process and routing policy | `daedalus-fab/.github` |

The Linear workspace currently uses the **Denman** team and the **DEN** issue key. The umbrella Linear project for organization routing is `github.com/daedalus-fab`. Delivery programs may use a narrower Linear project while remaining represented on the same GitHub Project.

## Current durable-execution program

The durable fabrication job program uses:

- Linear project: **Durable fabrication job execution & recovery**.
- Primary implementation issue: **DEN-2525**.
- Canonical schema promotion: **DEN-2526**.
- Worker checkpoint integration: **DEN-2527**.
- Recovery drills and observability: **DEN-2528**.
- JetStream consumers and dead-letter handling: **DEN-2529**.
- This operating documentation: **DEN-2530**.
- Implementation repository: `daedalus-fab/fabrication-server.rs`.
- Canonical database and NATS definitions: `ORESoftware/k8s-libs-and-shared-defs`.
- NATS deployment source: `ORESoftware/k8s-cluster`.

A delivery issue may span repositories, but every repository change must have its own pull request and CI evidence. The Linear issue records the cross-repository outcome and dependency graph.

## Delivery-program documents

Program-specific documents refine this organization-wide model without replacing it:

- [Meshy image-to-3D delivery program](MESHY_IMAGE_TO_3D_PROGRAM.md) — maps DEN-2465/DEN-2506 across the shared provider client, API, durable ingestion, preflight, clients, paired desktop workbenches, MCP, e2e, and GitHub Project 1.

## Required linking convention

Every non-trivial branch, pull request, and merge commit must identify its Linear issue.

- Preferred branch: `feature/<topic>` or the Linear-generated `.../den-####-...` branch.
- Pull request title: `[DEN-####] Imperative summary`.
- Pull request body: include `Tracks DEN-####`; use `Related: DEN-####` for supporting work that should not close the issue.
- Commit messages: include the issue key for changes that will be reviewed independently.
- Linear issue: attach the implementation branch or pull request and list cross-repository blockers/relations.

Do not mark a Linear issue complete merely because one pull request merged when its acceptance criteria include schema rollout, deployment, drills, or evidence in another repository.

## GitHub Project fields

Project 1 should expose at least these fields:

| Field | Purpose |
| --- | --- |
| Status | Backlog, Ready, In progress, In review, Blocked, Done |
| Linear issue | `DEN-####` identifier or issue URL |
| Workstream | API, worker, database, messaging, infrastructure, observability, documentation |
| Repository | Repository that owns the next executable change |
| Priority | Urgent, High, Medium, Low |
| Risk | On track, At risk, Blocked |
| Owner | Person accountable for the next transition |
| Target | Milestone or delivery date when one is committed |

Recommended saved views:

1. **Ready** — status Ready, grouped by repository.
2. **Active** — In progress or In review, grouped by owner.
3. **Review and CI** — open pull requests and failing/pending checks.
4. **Blocked** — blocked items grouped by blocker or external system.
5. **Reliability** — database, messaging, locks, recovery, and observability work.
6. **Recently completed** — completed during the last two weeks, grouped by Linear project.

## State synchronization

Use automation where it is deterministic, and require human judgment where one merge is not the whole outcome.

- Issue or draft PR created: add it to Project 1 and set Backlog or In progress as appropriate.
- Pull request marked ready: set the GitHub Project item to In review and the Linear issue to In Review when that PR is the next gating action.
- CI failure: retain In review but set Risk to At risk; link the failing run or corrective PR.
- Pull request merged: set the repository item to Done. Complete the Linear issue only when all acceptance criteria and cross-repository dependencies are satisfied.
- Pull request closed without merge: return the project item to Ready/Blocked and record the reason in Linear.
- Production or recovery evidence: attach the workflow run, artifact, dashboard, or incident/drill record to the Linear issue.

Avoid two-way automations that continuously overwrite manually curated priority, dependencies, or project health.

## Durable job-control review gates

Changes to job execution must demonstrate all of the following before the parent program is completed:

- RDS is the canonical execution/checkpoint source of truth.
- NATS JetStream is used for delivery and fan-out, not as the sole owner of job progress.
- Fiducia leases provide coarse distributed ownership and monotonically increasing fencing tokens.
- PostgreSQL uses transaction-scoped advisory locks only for short state transitions.
- No database transaction spans slow provider or fabrication work.
- JetStream ACK occurs only after the corresponding RDS commit.
- Stalled jobs resume from the latest committed checkpoint.
- Stale owners and lower fencing tokens cannot checkpoint or complete work.
- Recovery drills and operational metrics exist before production rollout is considered complete.

## Permissions and ownership

Install GitHub integrations at the organization level with least privilege:

- Metadata and contents: read for discovery; write only for repositories where the integration must create branches or commits.
- Pull requests and issues: read/write when creating or updating execution records.
- Actions: read for check evidence; write only when reruns or dispatches are explicitly required.
- Organization projects: read/write for Project 1 synchronization.
- Administration: do not grant unless repository or project creation is an explicit task.

The Linear integration needs access to the Denman team and relevant projects/issues. Secrets and personal access tokens must never be committed, pasted into issue bodies, or stored in project fields.

## Operating loop

1. Define outcome and acceptance criteria in Linear.
2. Split cross-repository work into linked Linear issues.
3. Add each executable issue/PR to GitHub Project 1.
4. Implement through a branch and pull request with the issue key.
5. Treat repository CI and review as authoritative evidence for code changes.
6. Record rollout, migration, and recovery evidence in Linear.
7. Complete the repository item after merge; complete the Linear issue after the full outcome is verified.
8. Reconcile conflicts semantically against current architecture and recent commit history; never resolve by blindly choosing one side.

## Troubleshooting

### Organization repositories are not visible

Confirm the GitHub App is installed on `daedalus-fab` and has access to the required repositories. Repository-level access does not imply organization Project access.

### Project 1 cannot be updated

Verify the integration has organization-project read/write permission and that the authenticated user can administer `daedalus-fab-project`.

### Linear project or team cannot be found

Use the workspace's current **Denman** team and **DEN** issue key. Search by exact project name before creating a replacement to avoid duplicate planning projects.

### PR merged but Linear remains open

Check the issue acceptance criteria and related blockers. Schema promotion, deployment, recovery drills, or evidence may still be outstanding even after the implementation PR merges.

### GitHub and Linear statuses disagree

GitHub is authoritative for repository facts; Linear is authoritative for delivery status. Correct the stale side and add a short note explaining the reconciliation rather than allowing automation to oscillate the values.
