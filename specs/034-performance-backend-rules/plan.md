# 034 — Backend performance rules Plan

Implements [034 — Backend performance rules](spec.md).

## Overview

Author `framework/rules/performance-backend.md` (five categories) following the canonical rule schema, and register it in the `/ductus` Shared Files manifest so it installs under the backend surface. Pure markdown-tier change — no runtime. Verification clauses are design-time commitments (what a spec/plan MUST state), consistent with `/ductus:analyze` auditing artifacts. Default severity SHOULD; MUST only for the DoS/exhaustion-regardless-of-scale cases.

## Technical Decisions

### Categories and abbreviations

`BE-` prefix (backend surface), five categories declared in the file header, all disjoint from `security-backend.md` and `api-backend.md`:

`QUERY` (query efficiency), `CACHE` (caching), `POOL` (connection pooling), `PAYLOAD` (payload budgets), `ASYNC` (async offloading).

### Rule list

| ID | Sev | Statement gist |
| --- | --- | --- |
| `BE-QUERY-001` | MUST | No N+1: a plan that fetches related data for a collection MUST batch (eager-load / join / `IN` / dataloader), not query per item |
| `BE-QUERY-002` | SHOULD | A plan that filters/sorts/joins on a column SHOULD name the supporting index (or state primary-key or unique-constraint coverage) |
| `BE-QUERY-003` | MUST | A read that can match an unbounded number of rows MUST bound how many it materializes: endpoints paginate (cite `BE-PAGE`); internal reads cap with a named maximum (cite `CFG-CONST-003`) or iterate in bounded batches by keyset or a server-side cursor, since `OFFSET` bounds memory but not work |
| `BE-QUERY-004` | SHOULD | A bulk write SHOULD use set-based or batched statements in chunks of a named maximum size, not one statement per row or one unbounded statement/transaction; the unbounded-transaction failure is a live-system concern, and first populating a database is the different case PostgreSQL's guidance addresses with `COPY` or a single transaction |
| `BE-CACHE-001` | MUST | Cache entries MUST have an explicit TTL or a documented invalidation trigger, and the cache MUST be size-bounded (a maximum entry count or memory limit enforced by an eviction policy) |
| `BE-CACHE-002` | SHOULD | Cache keys SHOULD incorporate every input that affects the value, including auth scope/tenant (cite `BE-AUTHZ-002`/`BE-AUTHZ-005` for the authorization requirement and `BE-AUTHZ-003` for the ban on caching permission state) |
| `BE-CACHE-003` | SHOULD | Caches fronting expensive work SHOULD coalesce concurrent misses (single-flight / lock / lease) or refresh before expiry, and jitter TTLs of entries written together, to prevent stampede |
| `BE-CACHE-004` | SHOULD | A safe-request response `BE-API-001` does not require to be `no-store` SHOULD declare an explicit `Cache-Control` policy, and a cacheable one SHOULD carry a validator for conditional (`304`) revalidation |
| `BE-POOL-001` | MUST | Connections to DBs/external services MUST come from a pool or a shared multiplexing client, not be opened per request; a pool MUST have a bounded maximum size, set by a named constant (cite `CFG-CONST-003`) |
| `BE-POOL-002` | MUST | Pool acquisition MUST have a bounded timeout so a saturated pool fails fast rather than hanging requests |
| `BE-POOL-003` | MUST | Pooled connections MUST be released on every path, including error paths |
| `BE-POOL-004` | SHOULD | The most connections one instance holds across its pools and processes × the largest concurrent instance count, plus the backend's other clients, SHOULD stay within the connections the backend admits, net of its reserved slots; a connection proxy MAY stand in for the pools that go through it |
| `BE-POOL-005` | SHOULD | A transaction or checked-out pooled connection SHOULD NOT be held open across a call to another service or other wait this service does not control; the call SHOULD come before the transaction begins or after it commits and the connection is released, through a transactional outbox (cite `BE-COORD-003`) when it publishes an event or writes a second datastore that must stay consistent with the commit |
| `BE-PAYLOAD-001` | MUST | Response payloads MUST have a bounded maximum size; endpoints returning collections able to exceed a small fixed maximum MUST paginate (cite `BE-PAGE`) rather than return unbounded lists |
| `BE-PAYLOAD-002` | SHOULD | Endpoints returning large resources SHOULD support field selection — a `fields=` parameter or field mask, a GraphQL selection set, a projection, or a view enumeration selecting among named response shapes |
| `BE-PAYLOAD-003` | SHOULD | Large text/JSON responses SHOULD be compressed when the client advertises support, with `Vary: Accept-Encoding`; a body reflecting attacker input beside a secret SHOULD NOT be compressed unless the secret is masked per response (BREACH) |
| `BE-PAYLOAD-004` | SHOULD | Large request/response bodies SHOULD be streamed in bounded chunks, not buffered whole (cite `BE-INPUT-006` for the request-size limit) |
| `BE-ASYNC-001` | MUST | Slow or unbounded work, including a side-effect-only third-party call, MUST NOT block the synchronous request path; offload to a background job, with an async acknowledgment when the request's own outcome waits on the job (cite `BE-STATUS-001` `202`, `BE-LRO-001` for observing completion). A third-party call the response depends on MAY stay synchronous when bounded per `BE-TIMEOUT-001` |

Every MUST is a DoS/exhaustion-regardless-of-scale case; every other rule is SHOULD (tunable efficiency / correctness-cross-referenced). `BE-QUERY-004`, `BE-CACHE-004`, `BE-POOL-004`, and `BE-PAYLOAD-004`, and the amendments to the other rules, landed with the 2026-10-19 rule-file review; `BE-POOL-005` landed with that day's second review, which also bounded the pool maximum in `BE-POOL-001`, narrowed `BE-PAYLOAD-001`'s pagination trigger, widened `BE-PAYLOAD-002`'s mechanisms, and scoped `BE-QUERY-004`'s unbounded-transaction failure to a live system, leaving 18 rules, 8 MUST and 10 SHOULD; an `SLO` category that review briefly added was removed by owner decision (spec Resolved Questions). `BE-ASYNC` carries one rule; background-job retry-safety/idempotency is `concurrency-backend.md` `BE-COORD-002` (spec 038), and request deadlines/timeouts are **reliability** concerns in `reliability-backend.md` (spec 039; per the spec's Boundaries), so they are intentionally absent here.

### Verification style

Every Verification clause targets a **design-time commitment** in a spec or plan (e.g., "any plan that introduces a list-rendering endpoint MUST state its query strategy and the indexes it relies on; `/ductus:analyze` flags list-endpoint plans that omit the query/index commitment"), never a source-code grep — matching how `/ductus:analyze` audits artifacts.

### Installation

Add one row to the `### ductus-owned shared files` table in `framework/bootstrap/ductus.md`: `framework/rules/performance-backend.md → specs/rules/performance-backend.md` (strategy: update). Spec 033's surface filter selects it under `backend` automatically via the `-backend.md` suffix — no 033 change needed. No `data-model.md`: the schema/format lives in 008; the category abbreviations live in the file header per the constitution's per-file policy.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/performance-backend.md` | Create | The performance rule set |
| `framework/bootstrap/ductus.md` | Modify | Add the rule file to the Shared Files manifest |

## Trade-offs

- **Deadlines/timeouts/retries/circuit-breakers deferred** to `reliability-backend.md` (since landed by spec 039) — keeps this set purely throughput/latency (resolved at clarify). Pool-acquisition timeout (`BE-POOL-002`) stays here.
- **A dependent third-party call may stay synchronous** — `BE-ASYNC-001` offloads slow or unbounded work and side-effect-only third-party calls, but lets a call the response depends on stay on the request path when bounded per `BE-TIMEOUT-001`; offloading it would only move the wait into client polling (owner decision 2026-10-19, spec Resolved Questions).
- **Background-job idempotency** (retry-safety) left to another file as well — `api-backend.md` `BE-IDEMP` covers HTTP idempotency, and the job-level version is `concurrency-backend.md` `BE-COORD-002` (spec 038), which makes an operation run under at-least-once delivery, retry, or a scheduler idempotent or deduplication-guarded.
- **Scope-complete cache keys is SHOULD, not MUST** — the unconditional authorization requirement (tenant isolation) is already `security-backend.md` `BE-AUTHZ-002`/`BE-AUTHZ-005` (MUST); the perf-cache rule cites it and stays SHOULD to keep every MUST in this file a DoS/exhaustion case.
- **No numeric thresholds** — rules require a bound to exist and be named (deferring the value to `CFG-CONST-003` constants), rather than hard-coding numbers that would be wrong for some projects.
