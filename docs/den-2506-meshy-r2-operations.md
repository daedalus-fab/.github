# DEN-2506 — Meshy worker and Cloudflare R2 operations

## Canonical tracking

| Surface | Item | State |
| --- | --- | --- |
| Linear project | `github.com/daedalus-fab` | Active |
| Linear issue | `DEN-2506` | In Progress |
| Source repository | `daedalus-fab/fabrication-server.rs` | Source and publication policy merged |
| Immutable R2/standalone-worker pull request | `fabrication-server.rs#8` | Merged as `cfff5cbd3932929d1ea7513a7387e5c4cc1053a5` |
| Trusted OCI publication pull request | `fabrication-server.rs#9` | Merged as `b143a4f04fd91696aafcef8a0a522da825fd14c7` |
| Independently reviewed source pin | `cfff5cbd3932929d1ea7513a7387e5c4cc1053a5` | Pinned by Kubernetes PR |
| Independent CI | `gha-indie-worker/gha-indie-worker.rs#20` | Validation evidence |
| Kubernetes integration | `ORESoftware/k8s-cluster#1077` | Open; credential-free contract green |
| Organization coordination | `daedalus-fab/.github#9` | Active umbrella |

The GitHub organization project should contain one delivery item for DEN-2506, linked to the Linear issue and the implementation, validation, publication, and deployment PRs above. Do not create parallel duplicate project items for the same operational outcome.

## Dependency semantics

Credential rotation and deploy-key retirement are parallel remediation work. DEN-2284 is related history and is not a prerequisite for this independently packaged worker lane.

The remaining private-source compile dependency is DEN-1537: the Kubernetes repository must receive the owner-scoped GitHub App pair `K8S_SUBMODULE_APP_ID` and `K8S_SUBMODULE_APP_PRIVATE_KEY`. The integration workflow accepts no PAT fallback and does not use `K8S_LIBS_DEPLOY_KEY` or `REMOTE_DEV_GH_PAT`.

## Delivered source scope

- typed Meshy provider client and durable job state machine;
- billable-create ambiguity protection and persistent checkpoints;
- immutable Cloudflare R2/S3 archive with conditional create;
- native SHA-256 upload and checksum-enabled replay verification;
- independently locked `dd-meshy-worker` package;
- checksum-protected compact resolver artifact;
- minimal non-root worker image;
- trusted GHCR publication policy with SBOM, provenance, exact-digest pullback, runtime verification, and vulnerability scan;
- directory/PVC recovery fallback;
- credential, endpoint, receipt-prefix, and object-key hardening;
- retention and release-boundary documentation.

## Release boundary

Every provider output remains candidate geometry regardless of provider completion or storage durability:

```json
{
  "review_state": "needs_review",
  "release_state": "blocked",
  "machine_ready": false
}
```

Nothing in the Meshy provider, R2 archive, image publication, or Kubernetes integration authorizes fabrication.

## Project status convention

| Delivery state | Linear | GitHub project |
| --- | --- | --- |
| Source implementation under review | In Review | In review |
| Source merged; deployment/provisioning incomplete | In Progress | In progress |
| Blocked by runner, scoped app permission, secret, bucket, or account administration | In Progress with explicit blocker comment | Blocked field or status note |
| Bucket, secrets, image digest, and review-blocked smoke certified | Done | Done |

DEN-2506 must remain In Progress while any operational gate below is open.

## Current evidence

At Kubernetes PR `ORESoftware/k8s-cluster#1077` head `6e821518f996e5df285844c00a6b0e6d2ed2cba2`:

- the credential-free source contract passed;
- secret scan, cluster E2E contracts, OpenAPI contracts, and workflow-trigger contracts passed;
- the private exact-source compile job stopped at its first precondition because the owner-scoped GitHub App pair was absent;
- no Meshy, R2, Cloudflare, AWS, database, Fiducia, or participant credential was used;
- no provider task or live deployment was performed.

That precondition failure is actionable under DEN-1537 and is not a credential-rotation blocker.

## Remaining operational gates

1. Restore or replace hosted runners for the `daedalus-fab` source organization so the trusted publication workflow can produce exact-digest evidence.
2. Complete DEN-1537 by installing the owner-scoped GitHub App pair in the Kubernetes repository and rerun the exact-source compile/test/image job.
3. Reconcile the current `k8s-cluster` base conflict on PR #1077 without changing the reviewed source pin or credential boundary.
4. Create the dedicated `daedalus-meshy-artifacts` R2 bucket.
5. Create Object Read & Write credentials scoped only to that bucket.
6. Install RDS, Fiducia, Meshy, and R2 values through the secret manager.
7. Publish, scan, and pin the exact worker image digest.
8. Run one explicitly approved, credit-capped live task.
9. Verify GLB/STL/3MF archive receipts, native R2 checksums, durable checkpoints, and the unchanged review-blocked result.

The Cloudflare account API token is an operator credential. It must never appear in source, project fields, issue bodies, workflow YAML, ConfigMaps, images, command lines, or worker pods.

## Validation note

Source-organization Actions were rejected before runner assignment by account billing policy, so those red checks executed no code. Independent CI validated the exact public AWS SDK surface, conditional/checksum operations, exact worker dependency resolution, locked metadata/compilation, crate-alias subject pattern, and resolver-artifact publication without using credentials, Cloudflare, Meshy, or provider credits.
