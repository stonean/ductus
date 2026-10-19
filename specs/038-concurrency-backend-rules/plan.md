# 038 — Backend concurrency rules Plan

Implements [038 — Backend concurrency rules](spec.md).

## Overview

A rule-introducing, markdown-tier feature — the 034/037 path exactly. It adds one rule file, `framework/rules/concurrency-backend.md`, on the **existing** `BE` surface (so no `scripts/lint-rule-ids.sh` change and no `data-model.md`), and wires it into the `/ductus` Shared Files manifest. The §Shared Files note is already count-free (036), so no count to update.

Four categories ship — `RACE`, `LOCK`, `TXN`, `COORD` — with thirteen rules, eight of them MUST. Verification is **design-time commitment** framing enforced by `/ductus:analyze` against feature artifacts, matching 034/037.

## Technical Decisions

### The rule file — `framework/rules/concurrency-backend.md`

Modeled on `performance-backend.md` / `observability-backend.md` (same surface, same analyze-time framing):

- **Header.** Title `# Concurrency Rules — Backend`; an intro scoping it to server-side concurrency correctness; the RFC 2119 note; the ID-format / category-declaration line stating IDs follow `BE-{CATEGORY}-{NNN}` with categories `RACE` (shared-state races), `LOCK` (locking/deadlock), `TXN` (transaction isolation), `COORD` (distributed coordination), and a pointer to `specs/008-security-rules/data-model.md` for the schema; a default-**SHOULD** paragraph (MUST reserved for scale-independent correctness/corruption hazards) noting these verify design-time commitments enforced by `/{project}:analyze`; and the opt-out paragraph the other backend rule files carry — a project without a backend need not opt out, because `/{project}:analyze` loads every rule file present and these rules fire only on a spec or plan that meets their Verification trigger — stating what each of `[rules] surfaces`, `[[review.disabled-rule-files]]`, and `[pinned] files` does and does not exclude.
- **Categories disjoint** from the other backend files (verified: `RACE`/`LOCK`/`TXN`/`COORD` appear in none).

### Rule set

Thirteen rules across four `## BE-{CATEGORY}` sections. Verification is phrased as a design-time commitment a spec/plan must make.

| ID | Sev | Statement gist |
| --- | --- | --- |
| `BE-RACE-001` | **MUST** | Shared mutable state reachable from more than one concurrent execution context is protected by a synchronization mechanism (lock / atomic / single-owner / serialized); unsynchronized concurrent access with at least one write is a data race. |
| `BE-RACE-002` | SHOULD | Prefer eliminating shared mutable state (immutability, confinement, message-passing) over guarding it, to shrink the surface that can race. |
| `BE-RACE-003` | **MUST** | A check-then-act sequence on shared state (test-then-set, lazy initialization, create-if-absent, check-then-insert) executes atomically — one lock across check and act, or one atomic operation; a uniqueness or existence rule on persisted data is enforced by the data store (unique or foreign-key constraint, insert-on-conflict, conditional put), not by a preceding read; a check-then-act whose act is harmless to repeat MAY be non-atomic, each access still synchronized per `BE-RACE-001`, and Verification accepts a plan stating the repeat is harmless. |
| `BE-LOCK-001` | SHOULD | The concurrency-control strategy for a contended resource is stated — optimistic (version/CAS) vs. pessimistic (lock) — with rationale. |
| `BE-LOCK-002` | SHOULD | When multiple locks are held at once, they are acquired in one consistent global order, each acquisition waits no longer than a bounded lock timeout, and each lock is released within a bounded hold time, to avoid deadlock and unbounded waits. |
| `BE-TXN-001` | SHOULD | Each transaction states its isolation level explicitly, with the anomalies that level prevents on the chosen engine, rather than silently relying on the engine default — which differs by engine and by deployment configuration, as do the guarantees behind the same level name. |
| `BE-TXN-002` | **MUST** | A concurrent read-modify-write of persisted state prevents lost updates (optimistic version check, `SELECT … FOR UPDATE`, an atomic in-place or conditional write, or an isolation level that detects the conflict on the named engine and aborts one writer); a bare read-then-write is silent corruption. |
| `BE-TXN-003` | **MUST** | A transaction that reads a set of rows, decides, then writes — enforcing an invariant across rows — guards against write skew and phantoms: serializable isolation on the chosen engine, locks on every row the decision read (a materialized lock row when it rests on absent rows), or an enforcing database constraint; snapshot isolation does not suffice. |
| `BE-TXN-004` | SHOULD | A transaction that can abort on a concurrency conflict (serialization failure, deadlock victim, failed version check) treats the abort as expected, with a stated handling path: bounded whole-transaction retry with fresh reads, or the conflict reported to the caller. |
| `BE-COORD-001` | **MUST** | A distributed lock, lease, or leader election relied on for correctness carries a fencing token that the protected resource validates — a lease alone is insufficient, because a paused holder, a delayed request, or a clock that jumps can let a holder act after its lease has expired (split-brain double-write); a lock taken only for efficiency MAY omit fencing. |
| `BE-COORD-002` | **MUST** | An operation invoked under at-least-once delivery, automatic retry, or a scheduler that can start it more than once for one scheduled time (a Kubernetes CronJob, an in-process scheduler running in every replica) or start a run while the previous run is still in progress is idempotent or dedup-guarded, and the delivery semantics are stated — at-most-once, at-least-once, or exactly-once with the boundary within which it holds. |
| `BE-COORD-003` | **MUST** | An operation that commits a database change and publishes a message, or writes a second datastore that must stay consistent, makes the two atomic (transactional outbox, change-data capture, or a single source of truth) — never two independent writes. |
| `BE-COORD-004` | **MUST** | An order-dependent consumer assumes no ordering wider than its broker guarantees: it states the ordering scope it relies on (partition, key, or message group) and either keeps ordered messages in one scope by key with serial per-scope processing or discards out-of-order messages by a per-entity sequence or version check. |

Cross-references, not restatements: `BE-COORD-002` cites `api-backend.md` `BE-IDEMP` (the HTTP idempotency contract) and `reliability-backend.md` `BE-RETRY-001` (retry mechanics) and `BE-RETRY-005` (exhausted redelivery); `BE-TXN-004` cites `BE-RETRY-001`; `BE-COORD-004` cites `BE-RETRY-005` (halting an ordered stream rather than dead-lettering out of it); `BE-TXN-002` cites `api-backend.md` `BE-COND-001` (a read-modify-write across two HTTP requests); `BE-LOCK-002` cites `configuration-cross.md` `CFG-CONST-003` for the lock timeout and hold bound, each a named constant or environment variable; `BE-RACE-003` cites `performance-backend.md` `BE-CACHE-003` for concurrent misses on an expensive cache entry (stampede), the case its harmless-to-repeat MAY leaves to performance. No rule reaches connection pooling, so none cites `performance-backend.md` `BE-POOL-*`.

### Manifest wiring — `framework/bootstrap/ductus.md`

Add a `concurrency-backend.md` → `specs/rules/concurrency-backend.md` row to the `### ductus-owned shared files` table, slotted between `api-backend.md` and `configuration-cross.md` (alphabetical: `concurrency` sorts before `configuration` — `conc` < `conf`). The `-backend.md` suffix means 024's loader selects it under the `backend` surface and 033's filter includes it. The §Shared Files note is count-free, so no count edit.

### What this feature does NOT touch

- **`scripts/lint-rule-ids.sh`** — `BE` is already registered; `BE-RACE-001` etc. match the regex unchanged.
- **`data-model.md`** — no new surface and no new schema; categories are declared in the file header (016 policy), schema is 008's. No data-model artifact (matching 034/037).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/concurrency-backend.md` | Create | The concurrency rule set (`RACE`/`LOCK`/`TXN`/`COORD`, thirteen rules, eight MUST) |
| `framework/bootstrap/ductus.md` | Modify | Add the manifest row (between `api-backend` and `configuration-cross`) |

## Trade-offs

- **Four categories, `COORD` kept.** Distributed coordination fits the design-time-commitment model (fencing tokens, delivery semantics are per-feature commitments), so it stays rather than splitting to a separate distributed-systems set (clarify resolution).
- **`TXN` here, not a future data-handling set.** Isolation levels manage concurrent-access anomalies — concurrency, not data-at-rest (clarify resolution).
- **Eight MUSTs.** Concurrency is unusually corruption-dense: data races (`RACE-001`), non-atomic check-then-act (`RACE-003`), lost updates (`TXN-002`), write skew and phantoms (`TXN-003`), missing fencing on a lock relied on for correctness (`COORD-001`), non-idempotent duplicates under retry, redelivery, or a duplicating scheduler (`COORD-002`), dual writes (`COORD-003`), and message ordering assumed wider than the broker guarantees (`COORD-004`) each silently corrupt state regardless of scale — two concurrent actors, two messages, or one crash at the wrong moment suffice. **Decision update, 2026-10-19 (rule-file review):** the set launched with four MUSTs (`RACE-001`, `TXN-002`, `COORD-001`, `COORD-002`); the review added `RACE-003`, `TXN-003`, `COORD-003`, and `COORD-004` under the same scale-independence test, delivering the check-then-act and write-skew concerns the spec's category table had named with no rule behind them, and narrowed `COORD-001`'s MUST to locks relied on for correctness, so a lock taken only for efficiency MAY omit fencing. **Decision update, 2026-10-19 (second rule-file review):** `RACE-003`'s MUST is narrowed the same way — a check-then-act whose act is harmless to repeat (two callers computing and storing the same value) MAY leave the check and the act non-atomic, each access still synchronized per `RACE-001`, because the race then costs work, not correctness; the plan states that the repeat is harmless instead of naming an atomicity mechanism. The MUST count stays eight. The contextual choices — optimistic vs. pessimistic, isolation-level selection, race-surface reduction, lock ordering and timeouts, and conflict-abort handling (`TXN-004`, SHOULD because the choice between retrying the transaction and reporting the conflict to the caller is contextual) — stay SHOULD.
- **Idempotency not duplicated.** Retry-safety cites `BE-IDEMP` rather than introducing a colliding `IDEMP` category (the spec's own boundary); `COORD-002` owns the deduplication obligation only where `BE-IDEMP` cannot reach — a queue consumer or scheduled job, which carries no `Idempotency-Key`.
- **Known limitation.** Analyze-time verification checks that a spec/plan *commits* to these guards; it cannot prove the guard is correctly implemented (a lock taken in the wrong order, a fencing token not actually validated) — that residual is for `/ductus:review` and tests, the standard design-time-rule limitation.

## Cross-spec impact

The spec/plan reference 008, 016, 024, 033, 034 (deps) and cite `BE-IDEMP` and `BE-COND-001` (api-backend), `BE-RETRY-001` and `BE-RETRY-005` (reliability-backend), `CFG-CONST-003` (configuration-cross), and `BE-CACHE-003` (performance-backend, from `BE-RACE-003`); `BE-POOL-*` (performance-backend) is named only as a boundary no rule reaches. None need an edit: all are cited, not changed; 024/033 already select `-backend.md` files; 034/037 are sibling precedents, unaffected. Informational; does not block.
