# Daedalus Fab web/API connection patterns

Status: organization architecture guidance, tracked by [DEN-4265](https://linear.app/denman/issue/DEN-4265/document-daedalus-fab-webapi-connection-patterns).

This policy applies to traditional operator web/BFF, API, job-control, inventory, machine-status, and background services. Machine safety and command authority remain explicit regardless of transport.

## Four supported avenues

| Avenue | Appropriate use | Boundary |
| --- | --- | --- |
| Direct database read | Named, stable, non-sensitive public/reference or read-model projection | Never operator identity, machine commands, job ownership, customer designs, inventory reservation, safety state, or writes; require distinct `SELECT`-only, `READ ONLY`, non-owner, `NOBYPASSRLS` access |
| Stateless HTTP/JSON | Default synchronous web-to-API path | Required for authorization, job submission, machine commands, inventory changes, and every mutation |
| Stateful TCP | Authorized machine-status or measured low-latency telemetry stream after API authorization | Never command, safety, persistence, or authorization authority; require ADR, mTLS/device identity, bounded frames, deadlines, backpressure, sequence handling, and reconnect policy |
| NATS/message queue | Durable post-commit job effects, scheduling signals, notifications, and telemetry fan-out | Never login, operator approval, safety interlock, command acknowledgement, or immediate response; require an outbox and idempotent consumers |

HTTP is the default. TCP is appropriate for bounded machine status only after authorization; machine commands and state changes remain API mutations. NATS carries durable effects, not command authority.

## Decision and ownership

1. Customer-private data, product authorization, job submission, machine commands, inventory reservation, and all mutations use HTTP.
2. Immediate authoritative responses use HTTP with versioned interfaces, bounded bodies/timeouts, correlation context, and idempotency.
3. Post-commit work is inserted into a transactional outbox and delivered through NATS.
4. A measured status/telemetry stream may use TCP after an ADR and short-lived API authorization.
5. Direct reads remain limited to named safe projections under a dedicated restricted role.

The web/BFF owns HTML, opaque secure sessions, CSRF, and authorization-code plus PKCE. The API owns product authorization, commands, safety preconditions, and state transitions. A core/data package owns typed mappings and transaction helpers. The canonical migration repository owns DDL; services verify compatibility and never migrate production at startup.

Shared Auth proves identity and assurance, not job, design, machine, or facility permission. Validate realm, issuer, audience, tenant, app/client, scopes, session, freshness, and assurance. Protected introspection keeps the service credential and user's token distinct. Never log tokens, cookies, codes, PKCE verifiers, customer designs, device credentials, or raw introspection results.

Use immutable Shared Auth and service dependencies. `opto-sync` supports only declared synchronization/outbox workflows, `ores-otel` provides redacted observability and trace propagation, and `zed-pkg` records dependency provenance. They do not bypass API authorization or device safety controls.

## Operational requirements

- Bound bodies, frames, deadlines, retries, queues, and buffers; propagate trace and correlation context.
- Re-authorize a TCP stream on reconnect or expiry; reject stale sequence/state assumptions.
- Make HTTP commands idempotent and message consumers duplicate-safe.
- Fail closed; no direct-query fallback for a failed API authorization or safety decision.
- Code comments identify the avenue and why its command/safety constraints are satisfied.
- Every TCP or direct-read exception has an ADR, owner, measurements, and review/expiry date.

This document is the durable organization policy; repository ADRs may impose stricter controls.
