# 034 — Backend performance rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Author `framework/rules/performance-backend.md`

- [x] Write the file header: title, RFC 2119 note, `BE-{CATEGORY}-{NNN}` ID format, the five category abbreviations (`QUERY`/`CACHE`/`POOL`/`PAYLOAD`/`ASYNC`), the `See specs/008-security-rules/data-model.md for the full schema` pointer, and the pin note.
- [x] Write the 13 rules across the five `## BE-{CATEGORY}` sections per the plan's rule list, each with Statement (RFC 2119), Rationale, Verification (design-time commitment), and Source.
- [x] Cross-reference rather than restate: `BE-PAGE` (pagination), `BE-AUTHZ-002`/`BE-AUTHZ-005` (cache-key tenant isolation), `BE-STATUS-001` (202 for async), `CFG-CONST-003` (named tunable constants).
- Done when: the file exists with 13 well-formed rules, 8 MUST / 5 SHOULD, each MUST a DoS/exhaustion case.

## 2. Register the file in the `/ductus` manifest

- [x] Add `framework/rules/performance-backend.md → specs/rules/performance-backend.md` to the `### ductus-owned shared files` table in `framework/bootstrap/ductus.md` (alphabetical position, strategy: update).
- Done when: the manifest row is present; `/ductus` would install the file, and 033's `-backend` surface filter selects it.

## 3. Validate

- [x] `scripts/lint-rule-ids.sh` passes (all IDs well-formed, categories disjoint, no duplicates).
- [x] `scripts/lint-rule-filenames.sh` passes (the `-backend.md` suffix).
- [x] `npx markdownlint-cli2` and `scripts/audit/*` pass (also lint-frontmatter, lint-tool-coverage, lint-procedure-parseability — all green).
- [x] Category abbreviations `QUERY`/`CACHE`/`POOL`/`PAYLOAD`/`ASYNC` confirmed disjoint from `security-backend.md` and `api-backend.md`.
- Done when: all lints/audits green.

## 4. Review and complete

- [x] Run `/ductus:review` over the change set; resolve any MUST findings. (0 MUST / 0 SHOULD / 0 low-confidence — clean; see `review.md`.)
- Done when: `/ductus:review` reports no blocking violations and the spec can advance to `done`.

## 5. Rule-file review 2026-10-19: land new and amended backend performance rules

- [x] `BE-QUERY-004` added (SHOULD): bulk writes use set-based or batched statements in chunks of a named maximum size (`CFG-CONST-003`), not one statement per row or one unbounded statement or transaction.
- [x] `BE-CACHE-004` added (SHOULD): a safe-request response `BE-API-001` does not require to be `no-store` declares an explicit `Cache-Control` policy, and a cacheable one carries a validator (`ETag` / `Last-Modified`) for `304` revalidation; frontend assets defer to `performance-frontend.md`.
- [x] `BE-POOL-004` added (SHOULD): pool maximum × the largest concurrent instance count (autoscaling ceiling plus rolling-deploy overlap), plus the backend's other clients, stays within the backend's connection limit, or a connection proxy multiplexes them.
- [x] `BE-PAYLOAD-004` added (SHOULD): large request and response bodies are streamed in bounded chunks with a named buffer size, citing `BE-INPUT-006` and `BE-PAYLOAD-001` for the size limits.
- [x] `BE-ASYNC-001` amended (owner decision kept): slow or unbounded work and side-effect-only third-party calls MUST be offloaded; a third-party call the response depends on MAY stay synchronous when bounded per `BE-TIMEOUT-001`; cites `BE-BREAKER-001` and `BE-LRO-001`; Source is the Azure Asynchronous Request-Reply pattern.
- [x] `BE-CACHE-001` amended: the size bound is a maximum entry count or memory limit enforced by an eviction policy, and an eviction policy without a capacity is flagged; Source is Redis key eviction and AWS caching strategies.
- [x] `BE-CACHE-002` amended: Rationale cites `BE-AUTHZ-003` (no cross-request permission caching).
- [x] `BE-CACHE-003` amended: coalescing (single-flight, lock, lease) or early refresh (probabilistic early expiration, stale-while-revalidate), plus jittered TTLs for entries written together.
- [x] `BE-QUERY-001` amended: Source is the Rails Guides N + 1 section and GraphQL DataLoader.
- [x] `BE-QUERY-002` amended: primary-key or unique-constraint coverage satisfies the rule; Rationale covers sorts, unindexed foreign-key columns, and composite-index leading columns; Source is Use The Index, Luke and PostgreSQL Foreign Keys.
- [x] `BE-QUERY-003` amended: an unbounded read bounds the rows it materializes; internal reads cap with a named maximum (`CFG-CONST-003`) or iterate in bounded batches; Source is OWASP API4:2023.
- [x] `BE-POOL-001` amended: a shared long-lived multiplexing client satisfies the rule as well as a pool; Source is HikariCP About Pool Sizing.
- [x] `BE-PAYLOAD-001` amended: Source is OWASP API4:2023.
- [x] `BE-PAYLOAD-003` amended: negotiated compression carries `Vary: Accept-Encoding`; a body reflecting attacker input beside a secret is not compressed unless the secret is masked per response (BREACH); already-compressed media MAY skip encoding.
- [x] File header amended: the reliability boundary names `reliability-backend.md` (`BE-TIMEOUT`/`BE-RETRY`/`BE-BREAKER`/`BE-BULK`) and keeps pool acquisition (`BE-POOL-002`) here; the opt-out paragraph is rewritten around analyze loading every file, `[rules] surfaces`, `[[review.disabled-rule-files]]`, and `[pinned] files`.
- [x] SLO exclusion recorded in spec.md Resolved Questions (owner decision 2026-10-19, pointing to 037's reasoning); the category set stays five.
- [x] spec.md (category table, severity posture, Boundaries, AC6, new criterion, Resolved Questions) and plan.md (rule list, severity summary, trade-offs) describe the rule file as it is on disk.

- **Done when**: `framework/rules/performance-backend.md` on disk carries every rule and header change above as described, spec.md and plan.md describe it, and `scripts/lint-rule-ids.sh` passes.

## 6. Rule-file review 2026-10-19 (second pass): land BE-POOL-005 and amended backend performance rules

- [x] `BE-POOL-005` added (SHOULD / SHOULD NOT): a database transaction or a checked-out pooled connection is not held open across a call to another service or any other wait this service does not control (an outbound HTTP or RPC call, a third-party API, a wait on user input); the call comes before the transaction begins, or after it commits and the connection is released, through a transactional outbox per `BE-COORD-003` (concurrency-backend.md) when it publishes an event or writes a second datastore that must stay consistent with the commit. The Rationale cites `BE-TIMEOUT-001` and `BE-POOL-002`; the Source is PostgreSQL Explicit Locking and Client Connection Defaults (`idle_in_transaction_session_timeout`), PgBouncer Features (transaction pooling) and `concurrency-backend.md` §BE-COORD. The transaction-pooling and vacuum claims were checked against pgbouncer.org/features.html and the PostgreSQL Client Connection Defaults page
- [x] `BE-POOL-001` amended (MUST tightened): a pool MUST have a bounded maximum size, set by a named constant (`CFG-CONST-003`) rather than a bare literal, and the Verification flags a pool with no maximum size. The Rationale says Go's `database/sql` places no limit on open connections unless `SetMaxOpenConns` is called (checked against pkg.go.dev: the default is 0, unlimited) and that gRPC's guidance is to reuse channels. The Source is Microsoft SQL Server Connection Pooling (ADO.NET), Go `database/sql` (`DB.SetMaxOpenConns`), gRPC Performance Best Practices and `configuration-cross.md` §CFG-CONST, superseding the HikariCP About Pool Sizing source task 5 recorded for this rule (HikariCP now backs `BE-POOL-002` and `BE-POOL-004`)
- [x] `BE-PAYLOAD-001` amended (MUST narrowed): pagination per `BE-PAGE` is required of endpoints returning collections able to exceed a small fixed maximum, rather than of every collection endpoint; the bounded-maximum-size MUST is unchanged
- [x] `BE-PAYLOAD-002` amended (SHOULD widened): the Verification accepts a `fields=` parameter or field mask, a GraphQL selection set, a projection, or a view enumeration selecting among named response shapes. The Source is Google AIP-157 (Partial responses) and JSON:API v1.1 Sparse Fieldsets, checked against google.aip.dev/157, which supports partial responses through field masks and through view enums with at least `BASIC` and `FULL` values
- [x] `BE-QUERY-004` amended (scoped to live systems): the Rationale places the failure of one unbounded statement or transaction on a live system (locks held until transaction end, vacuum held back, a late rollback) and names first populating a database as the different case PostgreSQL's guidance addresses with `COPY` or a single transaction. The Source is PostgreSQL Populating a Database (Disable Autocommit, Use `COPY`), `INSERT` (multirow `VALUES`), Explicit Locking and Client Connection Defaults (`idle_in_transaction_session_timeout`); the population guidance was checked against postgresql.org/docs/current/populate.html
- [x] `BE-QUERY-003` as it stands on disk: the bounded-batch form is keyset pagination or a server-side streaming cursor, the Rationale says `OFFSET` bounds memory but not work, and the Source adds Nygard, *Release It!* (Unbounded Result Sets) and PostgreSQL LIMIT and OFFSET to OWASP API4:2023 and `BE-PAGE`
- [x] Sources on `BE-QUERY-002`, `BE-CACHE-002`, `BE-CACHE-003`, `BE-CACHE-004`, `BE-POOL-002`, `BE-POOL-003`, `BE-POOL-004`, `BE-PAYLOAD-003` and `BE-PAYLOAD-004` each name specific documents; no Source in the file cites a body of literature or unnamed guides. Verify-only: neither spec.md nor plan.md restates a Source
- [x] spec.md: the `QUERY`, `POOL` and `PAYLOAD` rows of the category table match the file; the Severity posture puts a pool with no maximum on the MUST side and `BE-POOL-005` on the SHOULD side; a Boundaries bullet names `concurrency-backend.md` `BE-COORD-003` as the owner of atomic commit-and-publish, referring to spec 038 in plain text because 038 depends on 034; AC6 adds `FE-LOAD-007` (from `BE-CACHE-004`), `BE-COORD-003` (from `BE-POOL-005`) and `BE-TIMEOUT-001` from `BE-POOL-005`; AC9 covers `BE-POOL-005`, labelled by `label-criteria` with `next-criterion` now 10
- [x] plan.md: the rule list gains a `BE-POOL-005` row, the `BE-POOL-001`, `BE-QUERY-003`, `BE-QUERY-004`, `BE-PAYLOAD-001` and `BE-PAYLOAD-002` gists match the file, and the severity summary records the second review and the count of 18 rules, 8 MUST and 10 SHOULD
- [x] Verify: `scripts/lint-rule-ids.sh`, `npx markdownlint-cli2` on the changed spec files and `scripts/lint-frontmatter.sh` pass, and the read-only `derive-dependencies` reports 034's dependencies unchanged and no cycle

- **Done when**: 034's spec.md and plan.md describe `framework/rules/performance-backend.md` as it stands after the second 2026-10-19 rule-file review: 18 rules, 8 MUST and 10 SHOULD, with `BE-POOL-005` listed, cited in AC6's terms and covered by AC9, and the five-category set unchanged. Re-run `/ductus:review` and `/ductus:analyze` before 034 returns to `done`, since the rule file changed after both were recorded.
