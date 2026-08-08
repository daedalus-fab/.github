# DEN-2506 — Meshy worker and Cloudflare R2 operations

## Canonical tracking

| Surface | Item | State |
| --- | --- | --- |
| Linear project | `github.com/daedalus-fab` | Active |
| Linear issue | `DEN-2506` | In Progress |
| Source repository | `daedalus-fab/fabrication-server.rs` | Source merged |
| Source pull request | `fabrication-server.rs#8` | Merged |
| Source merge | `246e63a791b12041888f8986c40677c87497e47a` | Pinned |
| Independent CI | `gha-indie-worker/gha-indie-worker.rs#20` | Validation evidence |
| Kubernetes integration | `ORESoftware/k8s-cluster#1077` | Open |
| Organization coordination | `daedalus-fab/.github#9` | Active umbrella |

The GitHub organization project should contain one delivery item for DEN-2506, linked to the Linear issue and the three implementation/validation/deployment PRs above. Do not create parallel duplicate project items for the same operational outcome.

## Delivered source scope

- typed Meshy provider client and durable job state machine;
- billable-create ambiguity protection and persistent checkpoints;
- immutable Cloudflare R2/S3 archive with conditional create;
- native SHA-256 upload and checksum-enabled replay verification;
- independently locked `dd-meshy-worker` package;
- checksum-protected compact resolver artifact;
- minimal non-root worker image;
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

Nothing in the Meshy provider or R2 archive authorizes fabrication.

## Project status convention

| Delivery state | Linear | GitHub project |
| --- | --- | --- |
| Source implementation under review | In Review | In review |
| Source merged; deployment/provisioning incomplete | In Progress | In progress |
| Blocked by runner, secret, bucket, or account administration | In Progress with explicit blocker comment | Blocked field or status note |
| Bucket, secrets, image digest, and review-blocked smoke certified | Done | Done |

DEN-2506 must remain In Progress while any operational gate below is open.

## Remaining operational gates

1. Restore or replace hosted runners for the `daedalus-fab` source organization.
2. Provide the Kubernetes integration workflow read access to the private source repository.
3. Create the dedicated `daedalus-meshy-artifacts` R2 bucket.
4. Create Object Read & Write credentials scoped to that bucket.
5. Install RDS, Fiducia, Meshy, and R2 values through the secret manager.
6. Publish and pin the worker image digest.
7. Run one explicitly approved, credit-capped live task.
8. Verify GLB/STL/3MF archive receipts, native R2 checksums, durable checkpoints, and the unchanged review-blocked result.

The Cloudflare account API token is an operator credential. It must never appear in source, project fields, issue bodies, workflow YAML, ConfigMaps, images, command lines, or worker pods.

## Validation note

Source-organization Actions were rejected before runner assignment by account billing policy, so those red checks executed no code. Independent CI validated the exact public AWS SDK surface, conditional/checksum operations, exact worker dependency resolution, locked metadata/compilation, crate-alias subject pattern, and resolver-artifact publication without using credentials, Cloudflare, Meshy, or provider credits.
