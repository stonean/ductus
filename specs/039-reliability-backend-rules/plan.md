# 039 — Backend reliability rules Plan

Implements [039 — Backend reliability rules](spec.md).

## Overview

A rule-introducing, markdown-tier feature — the 034/037/038 path exactly. It adds one rule file, `framework/rules/reliability-backend.md`, on the **existing** `BE` surface (so no `scripts/lint-rule-ids.sh` change and no `data-model.md`), and wires it into the `/ductus` Shared Files manifest. The §Shared Files note is already count-free (036), so no count to update. Landing this set resolves 034's forward-reference to a future `reliability-backend.md` (AC #8).

Five categories ship — `TIMEOUT`, `RETRY`, `BREAKER`, `DRAIN`, `BULK` — with thirteen rules, four of them MUST — eight and three at launch; the 2026-10-19 rule-file review added `BE-RETRY-003`, `BE-RETRY-004`, `BE-RETRY-005` (MUST) and `BE-BULK-002` and revised the launch eight in place (task 5), and that day's owner decisions added `BE-TIMEOUT-003` and reworded `BE-RETRY-005` without its monitoring and alerting requirement (task 7). Verification is **design-time commitment** framing enforced by `/ductus:analyze` against feature artifacts, matching 034/037/038.

## Technical Decisions

### The rule file — `framework/rules/reliability-backend.md`

Modeled on `performance-backend.md` / `observability-backend.md` / `concurrency-backend.md`:

- **Header.** Title `# Reliability Rules — Backend` and one paragraph: the scope (server-side resilience under partial failure); these verify design-time commitments enforced by `/{project}:analyze`; a rule is **MUST** only for a scale-independent availability or cascading-failure risk, the rest **SHOULD**; the ID-format / category-declaration line stating IDs follow `BE-{CATEGORY}-{NNN}` with categories `TIMEOUT` (timeouts and deadlines), `RETRY` (retries and dead-lettering), `BREAKER` (circuit breakers), `DRAIN` (graceful shutdown), `BULK` (backpressure, bulkheads, and graceful degradation); and a pointer to constitution §rules for severity, ID permanence, and how review selects or excludes the file (008's `rules-are-as-short-as-their-required-content`).
- **Categories disjoint** from the other backend files (verified: `TIMEOUT`/`RETRY`/`BREAKER`/`DRAIN`/`BULK` appear in none).

### Rule set

Thirteen rules across five `## BE-{CATEGORY}` sections. Verification is phrased as a design-time commitment a spec/plan must make.

| ID | Sev | Statement gist |
| --- | --- | --- |
| `BE-TIMEOUT-001` | **MUST** | Every outbound or downstream call, or blocking wait on another process or host, has a bounded timeout — no unbounded waits (an unbounded wait exhausts threads/connections and cascades); an unnamed client-library default does not count, and local file I/O is out of scope. |
| `BE-TIMEOUT-002` | SHOULD | An end-to-end request deadline is established and propagated to downstream calls, so per-call timeouts respect the caller's remaining budget. |
| `BE-TIMEOUT-003` | SHOULD | A datastore query issued on a request path or by a worker is bounded on the server — by the datastore's own time limit (PostgreSQL `statement_timeout`; MySQL `max_execution_time`, read-only `SELECT` statements only; MongoDB `maxTimeMS`), no longer than the caller's timeout or remaining deadline — or, where no server-side limit covers it, by a client timeout that cancels it on the server, since a client-side wait alone can leave the query running and holding its connection and locks. |
| `BE-RETRY-001` | **MUST** | Automatic retries are bounded (max attempts) with exponential backoff + jitter, and apply only to idempotent operations (cite `api-backend.md` `BE-IDEMP`) — unbounded/un-jittered retries are a retry storm that amplifies an outage. |
| `BE-RETRY-002` | SHOULD | Retries are governed by a retry budget (e.g. a cap on retries as a fraction of requests) and coordinated with circuit breaking, so a degraded downstream is not hammered by aggregate retries. |
| `BE-RETRY-003` | SHOULD | Only failures classified as transient (timeouts, connection failures, `429`/`503`) are retried, waiting at least a server-supplied `Retry-After`; permanent errors and malformed requests are not retried. |
| `BE-RETRY-004` | SHOULD | A failing dependency is retried at one layer of the call path only — the layer immediately above the failure — so attempts do not multiply across layers. |
| `BE-RETRY-005` | **MUST** | A message that still fails when its redelivery bound is exhausted goes to a retained dead-letter destination where the responsible team can review and redrive it — or, where strict ordering forbids skipping it, halts its stream with the halt recorded alongside the message that caused it — and is never redelivered indefinitely or discarded silently; no monitoring or alerting is required (owner decision, 2026-10-19). |
| `BE-BREAKER-001` | SHOULD | Calls to a failure-prone downstream are guarded by a circuit breaker that opens on a failure threshold (a count or rate of failures), fails fast while open, and half-opens to probe recovery. |
| `BE-DRAIN-001` | **MUST** | On a termination signal the service stops accepting new work and drains in-flight work within a bounded grace period before exiting — no dropped in-flight requests on deploy; the grace period, with any delay that runs after the forced-kill countdown starts, fits inside the platform's forced-kill deadline. |
| `BE-DRAIN-002` | SHOULD | At the start of shutdown the instance leaves routing (endpoint removal, a failing readiness signal per `observability-backend.md` `BE-HEALTH-001`, or deregistration) and keeps accepting requests for a bounded propagation delay before it closes intake. |
| `BE-BULK-001` | SHOULD | Failure-prone or resource-heavy work is isolated behind a bulkhead (bounded concurrency / bounded queue) and sheds load when saturated rather than queueing unboundedly, to degrade gracefully. |
| `BE-BULK-002` | SHOULD | Each downstream dependency is classified critical or non-critical to the operation; a failing non-critical one degrades the response (omitted, stale, or a static default) rather than failing the whole operation. |

Cross-references, not restatements: `BE-RETRY-001` cites `api-backend.md` `BE-IDEMP`, and it and `BE-RETRY-005` cite `concurrency-backend.md` `BE-COORD-002`; `BE-RETRY-005` also cites `concurrency-backend.md` `BE-COORD-004` for the ordering scope a consumer relies on; `BE-RETRY-003` cites `security-backend.md` `BE-API-004` and `api-backend.md` `BE-STATUS-001`; `BE-DRAIN-002` and `BE-BULK-002` cite `observability-backend.md` `BE-HEALTH-001`; `BE-BULK-001` cites `performance-backend.md` `BE-ASYNC-001`/`BE-POOL-001`/`BE-POOL-002`, `BE-STATUS-001`, `security-backend.md` `BE-API-004` (a `429` where the limit is a per-client rate), and `observability-backend.md` `BE-METRIC-002`; `BE-TIMEOUT-001` cites `performance-backend.md` `BE-POOL-002` for the pool-acquisition-timeout interaction, and `BE-TIMEOUT-003` cites `BE-POOL-002` for a pooled connection an abandoned query keeps checked out and `BE-POOL-005` to separate holding a transaction across a downstream call from bounding the query; tunables (timeout durations, request deadlines, server-side query time limits, retry and redelivery bounds, retry budgets, breaker thresholds, grace periods, routing-propagation delays, bulkhead bounds) cite `configuration-cross.md` `CFG-CONST-003` from ten rules — `BE-TIMEOUT-001`, `BE-TIMEOUT-002`, `BE-TIMEOUT-003`, `BE-RETRY-001`, `BE-RETRY-002`, `BE-RETRY-005`, `BE-BREAKER-001`, `BE-DRAIN-001`, `BE-DRAIN-002` and `BE-BULK-001`.

### Manifest wiring — `framework/bootstrap/ductus.md`

Add a `reliability-backend.md` → `specs/rules/reliability-backend.md` row to the `### ductus-owned shared files` table, slotted between `quality-cross.md` and `security-backend.md` (alphabetical). The `-backend.md` suffix means 024's loader selects it under the `backend` surface and 033's filter includes it. The §Shared Files note is count-free, so no count edit.

### What this feature does NOT touch

- **`scripts/lint-rule-ids.sh`** — `BE` is already registered; `BE-TIMEOUT-001` etc. match the regex unchanged.
- **`data-model.md`** — no new surface and no new schema; categories are declared in the file header (016 policy), schema is 008's. No data-model artifact (matching 034/037/038).
- **`performance-backend.md`** — `BE-POOL-002` is cited, not moved; 034's body already forward-references this set, so no edit to 034 is required (the forward-reference resolves by this file existing).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/reliability-backend.md` | Create | The reliability rule set (`TIMEOUT`/`RETRY`/`BREAKER`/`DRAIN`/`BULK`, thirteen rules, four MUST) |
| `framework/bootstrap/ductus.md` | Modify | Add the manifest row (between `quality-cross` and `security-backend`) |

## Trade-offs

- **Five categories; `DEADLINE` folded into `TIMEOUT`.** Per-call timeout and end-to-end deadline propagation are the same concern (bounding the wait) at two scopes — one category, two rules — rather than a sixth category (clarify resolution).
- **`BULK` kept.** Backpressure/load-shedding is the graceful-degradation primitive named by the constitution's reliability principle and fits the design-time-commitment model; it cites the performance set where it overlaps rather than re-deriving (clarify resolution).
- **Four MUSTs.** Unbounded waits (`TIMEOUT-001`), retry storms (`RETRY-001`), poison messages redelivered forever or silently dropped (`RETRY-005`, added 2026-10-19 and kept MUST by the owner), and no-drain deploys (`DRAIN-001`) are the absences that cause availability loss or cascading failure regardless of scale; breaker adoption, deadline propagation, server-side query time limits (`TIMEOUT-003`, whose caller-side cascade `TIMEOUT-001` already covers), retry budgeting, retry classification, single-layer retries, bulkheading, and graceful degradation are contextual (SHOULD).
- **Known limitation.** Analyze-time verification checks that a spec/plan *commits* to these resilience measures; it cannot prove the timeout is actually wired or the drain actually waits — that residual is for `/ductus:review` and tests, the standard design-time-rule limitation.

## Cross-spec impact

The spec/plan reference 008, 016, 024, 033, 034 (deps) and cite `BE-IDEMP`/`BE-STATUS-001` (api-backend), `BE-POOL-001`/`BE-POOL-002`/`BE-POOL-005`/`BE-ASYNC-001` (performance-backend), `BE-HEALTH-001`/`BE-METRIC-002` (observability-backend, 037), `BE-COORD-002`/`BE-COORD-004` (concurrency-backend, 038), `BE-API-004` (security-backend, 008), `CFG-*` (configuration-cross). None need an edit: all are cited, not changed. 034 forward-references this set in prose; that reference resolves by this file existing — no 034 edit needed (and editing a `done` 034 body would trip its own back-edge). 024/033 already select `-backend.md` files. Informational; does not block.
