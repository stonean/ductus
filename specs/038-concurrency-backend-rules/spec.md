---
status: in-progress
dependencies: [008-security-rules, 016-cross-cutting-rules, 024-rule-loader, 033-rule-surface-setting, 034-performance-backend-rules]
next-criterion: 9
---

# 038 — Backend concurrency rules

Introduce `framework/rules/concurrency-backend.md`, an enforceable rule set covering server-side concurrency correctness concerns. The file follows the canonical rule schema and is installed and enforced under the **backend** surface. This is a rule-introducing feature spec — the same path [008-security-rules](../008-security-rules/spec.md) used — and one of the three Task-9 backend sets that follow [034-performance-backend-rules](../034-performance-backend-rules/spec.md).

## Motivation

The constitution names **concurrency** a governance-recognized cross-cutting category (§rules) — and names it twice. When this spec was written, the backend surface had **zero** coverage of it. Concurrency defects — shared-state races, lost updates under the wrong isolation level, deadlocks from inconsistent lock ordering, double-applied non-idempotent retries, distributed locks without fencing tokens — are among the costliest to diagnose because they are non-deterministic and often invisible until production load. There was nowhere to promote these recurring hazards to the rules tier, so each was re-litigated per feature or caught only after a corruption incident.

This spec closes that gap: a backend concurrency rule set citable by ID (`BE-{CATEGORY}-{NNN}`) that `/ductus:analyze` checks against specs and plans (design-time concurrency commitments).

## Rule set scope

`concurrency-backend.md` uses the **backend** surface (`-backend.md` suffix, `BE-` ID prefix), with NEW categories disjoint from every category the other `-backend.md` rule files declare in their headers (`security-backend.md`, `api-backend.md`, `performance-backend.md`, and the rest). Category set (resolved at clarify — all four ship; see Resolved Questions). Each concern names the rule that delivers it:

| Category | Abbrev | Concern |
| --- | --- | --- |
| Shared-state races | `RACE` | guarded shared mutable state (`BE-RACE-001`); sharing eliminated rather than guarded where it can be (`BE-RACE-002`); no check-then-act without atomicity unless its act is harmless to repeat, and uniqueness or existence on persisted data enforced by the data store (`BE-RACE-003`) |
| Locking | `LOCK` | optimistic vs. pessimistic choice stated (`BE-LOCK-001`); consistent lock ordering to avoid deadlock, a bounded acquisition timeout, and a bounded hold time (`BE-LOCK-002`) |
| Transaction isolation | `TXN` | explicit isolation level per transaction, with the anomalies it prevents on the chosen engine (`BE-TXN-001`); lost updates guarded (`BE-TXN-002`); write skew and phantoms guarded (`BE-TXN-003`); conflict aborts handled as an expected outcome (`BE-TXN-004`) |
| Distributed coordination | `COORD` | distributed locks relied on for correctness carry fencing tokens (`BE-COORD-001`); duplicates from retry, redelivery, or a scheduler — one that fires more than once for a scheduled time or starts a run while the previous one is in progress — are idempotent or deduplicated, with delivery semantics stated — at-most-once, at-least-once, or exactly-once within a named boundary (`BE-COORD-002`); no dual write — a database change and its publish made atomic (`BE-COORD-003`); no message ordering assumed wider than the broker guarantees (`BE-COORD-004`) |

Idempotency of retried operations is **not** a new category here — for an HTTP operation it is already `api-backend.md` `BE-IDEMP`, cross-referenced in the Boundaries below rather than redefined (a duplicate `IDEMP` category would collide with the existing `BE-IDEMP` namespace). A queue consumer or scheduled job carries no `Idempotency-Key`, so its deduplication obligation is `BE-COORD-002`'s, which builds on `BE-IDEMP` rather than restating it.

### Severity posture

Concurrency rules default to **SHOULD** where the right approach is contextual. A rule is **MUST** only when its absence is a correctness/corruption hazard *regardless of scale* — for example, unguarded shared mutable state on a concurrent path, a distributed lock relied on for correctness without a fencing token, a non-idempotent operation placed behind an automatic retry, or a database change and its publish performed as two independent writes.

### Boundaries (cross-reference, do not duplicate)

- **Idempotency of API operations** is already `api-backend.md` `BE-IDEMP` — this set cites it for the retry-safety case rather than restating it.
- **A read-modify-write spanning two HTTP requests** — a client's read, then its write — is `api-backend.md` `BE-COND-001`; `BE-TXN-002` covers the read-modify-write inside one transaction and points there for the cross-request window.
- **Retry mechanics and exhausted redelivery** — attempt bounds, backoff, and jitter — are `reliability-backend.md` `BE-RETRY-001`, and a message that exhausts its redelivery bound, including one in an ordered stream, is `BE-RETRY-005`; `BE-COORD-002`, `BE-COORD-004`, and `BE-TXN-004` cite them rather than restate them.
- **Connection pooling and acquisition bounds** are `performance-backend.md` `BE-POOL-*` — no concurrency rule reaches pool sizing or connection acquisition, so the set neither cites nor restates them.
- **Cache stampede** — concurrent misses on an expensive cache entry — is `performance-backend.md` `BE-CACHE-003`; `BE-RACE-003` lets a check-then-act whose act is harmless to repeat, such as a cache entry built twice with the same value, stay non-atomic and points there for the stampede concern rather than restating it.
- **Operator-tunable values** (lock acquisition timeouts, lock hold bounds) are each a named constant or an environment variable per `configuration-cross.md` `CFG-CONST-003`, which `BE-LOCK-002` cites.

## Acceptance Criteria

- [x] AC1: `framework/rules/concurrency-backend.md` exists, ends in the `-backend.md` suffix, and follows the canonical rule schema (`### {ID}` headings; Statement / Rationale / Verification; RFC 2119 language) per [008-security-rules](../008-security-rules/spec.md)'s data-model.
- [x] AC2: Every rule ID uses the `BE-{CATEGORY}-{NNN}` format with a concurrency category disjoint from the `security-backend.md`, `api-backend.md`, and `performance-backend.md` category sets; `scripts/lint-rule-ids.sh` passes.
- [x] AC3: The file header declares the concurrency category abbreviations per the per-file category-declaration policy ([016-cross-cutting-rules](../016-cross-cutting-rules/spec.md)).
- [x] AC4: The rule set covers, at minimum, shared-state races, locking/deadlock avoidance, and transaction isolation — each with a Verification clause expressed as a **design-time commitment** the spec/plan must make (not a code-pattern grep), consistent with how `/ductus:analyze` audits artifacts.
- [x] AC5: Each MUST rule is one whose absence is a correctness/corruption hazard regardless of scale; contextual trade-offs are SHOULD. The split is evident from the Statements.
- [x] AC6: Rules whose surface overlaps an existing rule cite it rather than restating it. As delivered the citations are `api-backend.md` `BE-IDEMP` (retry safety) and `BE-COND-001` (a read-modify-write across two HTTP requests), `reliability-backend.md` `BE-RETRY-001` (retry mechanics) and `BE-RETRY-005` (exhausted redelivery), `configuration-cross.md` `CFG-CONST-003` (tunable lock timeouts and hold bounds), and, since the second 2026-10-19 rule-file review, `performance-backend.md` `BE-CACHE-003` (cache stampede, from `BE-RACE-003`). The `BE-POOL-*` citation this criterion anticipated is **not** present and was not needed: no rule in the set reaches connection pooling — the file does not mention pools or connections at all — so there is nothing to cite and nothing was restated.
- [x] AC7: The file is added to the `/ductus` **Shared Files** manifest in `framework/bootstrap/ductus.md` and is selected under the `backend` surface by `/ductus:review`, composing with [033-rule-surface-setting](../033-rule-surface-setting/spec.md) and [024-rule-loader](../024-rule-loader/spec.md).
- [x] AC8: Every concern the §Rule set scope category table names is delivered by at least one rule in `framework/rules/concurrency-backend.md`, and the table cites each delivering rule by ID, so no concern is promised without a rule behind it.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Exact category set and abbreviations.** Resolved: **ship all four — `RACE`, `LOCK`, `TXN`, `COORD`**, each disjoint from the existing `BE-` category sets in `security-backend.md` / `api-backend.md` / `performance-backend.md`. Idempotency stays `api-backend.md` `BE-IDEMP` (cited, not redefined).
- **Distributed coordination (`COORD`) in scope or deferred?** Resolved: **kept in scope.** Distributed coordination fits the design-time-commitment model (unlike 037's deferred SLO/ALERT): a spec that introduces a distributed lock can commit to fencing tokens, and one with a message consumer can commit to delivery semantics (at-most-once, at-least-once + idempotent, or exactly-once within a named boundary). The fencing-token rule is a corruption-preventing MUST; deferring it would fragment a coherent concurrency concern.
- **Boundary with transaction isolation vs. a future data-handling set.** Resolved: **`TXN` lives here.** Isolation levels exist to manage *concurrent*-access anomalies (lost update, write skew) — that is concurrency, not data-at-rest. A future data-handling rule set would own validation / retention / PII, not isolation semantics.
- **MUST vs. SHOULD default.** Resolved: **default SHOULD; MUST only when the absence is a correctness/corruption hazard regardless of scale.** Mirrors 034/037. The MUSTs at launch: unguarded shared mutable state on a concurrent path (`BE-RACE-001`), a lost update (`BE-TXN-002`), a distributed lock without a fencing token (`BE-COORD-001`), and a non-idempotent operation behind an automatic retry (`BE-COORD-002`, citing `BE-IDEMP`). The 2026-10-19 rule-file review applied the same test to add a non-atomic check-then-act (`BE-RACE-003`), write skew and phantoms (`BE-TXN-003`), a dual write (`BE-COORD-003`), and message ordering assumed wider than the broker guarantees (`BE-COORD-004`), and narrowed `BE-COORD-001` to locks relied on for correctness (the plan's Trade-offs section). **Updated 2026-10-19 (second rule-file review):** `BE-RACE-003`'s MUST is narrowed the same way — a check-then-act whose act is harmless to repeat MAY leave the check and the act non-atomic, each access still synchronized per `BE-RACE-001`, because a repeated harmless act costs work, not correctness. Contextual choices (optimistic vs. pessimistic locking, isolation-level selection, lock ordering and timeouts, conflict-abort handling) stay SHOULD.
