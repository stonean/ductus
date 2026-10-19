# 038 — Backend concurrency rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Author `framework/rules/concurrency-backend.md`

- [x] Write the file header: title (`# Concurrency Rules — Backend`), an intro scoping it to server-side concurrency correctness, the RFC 2119 note, the `BE-{CATEGORY}-{NNN}` ID-format / category-declaration line (categories `RACE`/`LOCK`/`TXN`/`COORD`, with the `See specs/008-security-rules/data-model.md` pointer), the default-SHOULD + design-time-commitment paragraph, and the backend pin/surface note.
- [x] Write the eight launch rules across the four `## BE-{CATEGORY}` sections per the plan's rule table: `BE-RACE-001` (MUST) / `BE-RACE-002` (SHOULD), `BE-LOCK-001/002` (SHOULD), `BE-TXN-001` (SHOULD) / `BE-TXN-002` (MUST), `BE-COORD-001` (MUST) / `BE-COORD-002` (MUST) — each with Statement (one RFC 2119 keyword), Rationale, and a design-time-commitment Verification clause, plus Source where apt.
- [x] Cross-reference rather than restate: `BE-COORD-002` cites `api-backend.md` `BE-IDEMP`; tunable values cite `configuration-cross.md` `CFG-*`. No rule reaches connection pooling, so none cites `performance-backend.md` `BE-POOL-*`.
- Done when: the file exists with the eight launch rules well-formed (4 MUST / 4 SHOULD), categories disjoint from the other backend files, each MUST a scale-independent corruption hazard.

## 2. Register the file in the `/ductus` manifest

- [x] Add `framework/rules/concurrency-backend.md → specs/rules/concurrency-backend.md` to the `### ductus-owned shared files` table in `framework/bootstrap/ductus.md`, slotted between `api-backend.md` and `configuration-cross.md` (alphabetical; strategy: update).
- Done when: the manifest row is present; the `-backend.md` suffix makes 024's loader select it under the `backend` surface and 033's filter include it. (The §Shared Files note is count-free — no count edit.)

## 3. Validate

- [x] `scripts/lint-rule-ids.sh` passes (`BE-RACE`/`BE-LOCK`/`BE-TXN`/`BE-COORD` IDs well-formed; categories disjoint from the other backend files; no duplicates).
- [x] `scripts/lint-rule-filenames.sh` passes (the `-backend.md` suffix).
- [x] `npx markdownlint-cli2`, the other `scripts/lint-*.sh`, and `scripts/audit/*` pass (frontmatter, tool-coverage, procedure-parseability, manifest-parity, ssot-invariants, cross-doc-consistency — all green).
- Done when: all lints/audits green.

## 4. Review and complete

- [x] Run `/ductus:review` over the change set; resolve any MUST findings.
- Done when: `/ductus:review` reports no blocking violations and the spec can advance to `done`.

## 5. Rule-file review 2026-10-19: land new and amended concurrency rules

- [x] Header: replace the backend pin/surface note with the opt-out paragraph — a project without a backend need not opt out, since `/{project}:analyze` loads every rule file present and these rules fire only on their Verification trigger — stating what `[rules] surfaces`, `[[review.disabled-rule-files]]`, and `[pinned] files` each exclude.
- [x] Amend `BE-RACE-001`: a data race is unsynchronized concurrent access in which at least one access is a write (write/write counts, not only read-write); the Rationale points a compound check-then-act to `BE-RACE-003`; Source names the Go Memory Model and JLS §17.4.1 / §17.4.5.
- [x] Add `BE-RACE-003` (MUST): a check-then-act on shared state executes atomically (one lock across check and act, or one atomic operation), and a uniqueness or existence rule on persisted data is enforced by the data store (unique constraint, insert-on-conflict, conditional put), not a preceding read.
- [x] Amend `BE-LOCK-002`: one global acquisition order, a bounded acquisition timeout, and a bounded hold time, with Verification asking for all three; the lock timeout and hold bound cite `configuration-cross.md` `CFG-CONST-003`.
- [x] Amend `BE-TXN-001`: the Rationale names each engine's default level, a default set by deployment configuration (SQL Server `READ_COMMITTED_SNAPSHOT`), and one level name carrying different guarantees across engines; Verification asks for the anomalies prevented on the chosen engine; Source widened to the engine documentation.
- [x] Amend `BE-TXN-002`: the lost-update guards add an atomic in-place write and an isolation level that detects the conflict on the named engine; the abort handling points to `BE-TXN-004`; a read-modify-write across two HTTP requests points to `api-backend.md` `BE-COND-001`.
- [x] Add `BE-TXN-003` (MUST): a transaction enforcing an invariant across rows guards against write skew and phantoms (serializable isolation on the chosen engine, locks on every row read or a materialized lock row, or an enforcing constraint); snapshot isolation does not suffice.
- [x] Add `BE-TXN-004` (SHOULD): a conflict abort (serialization failure, deadlock victim, failed version check) has a stated handling path — bounded whole-transaction retry with fresh reads, or the conflict reported to the caller; retry mechanics cite `reliability-backend.md` `BE-RETRY-001`.
- [x] Amend `BE-COORD-001`: the fencing-token MUST covers a lock, lease, or leader election relied on for correctness; a lock taken only for efficiency MAY omit fencing; Verification requires the plan to say which.
- [x] Amend `BE-COORD-002`: a scheduler that can start a job more than once for one scheduled time joins retry and at-least-once delivery; the stated semantics include at-most-once and exactly-once with its boundary; a message exhausting its redelivery bound cites `BE-RETRY-005`; Source names the Kubernetes CronJob documentation.
- [x] Add `BE-COORD-003` (MUST): a database change and its publish, or a second datastore kept consistent, are made atomic (transactional outbox, change-data capture, or a single source of truth), never two independent writes.
- [x] Add `BE-COORD-004` (MUST): an order-dependent consumer states its ordering scope (partition, key, or message group) and either routes by key with serial per-scope processing or discards out-of-order messages by sequence or version; dead-lettering out of an ordered stream cites `BE-RETRY-005`.
- [x] `scripts/lint-rule-ids.sh` passes: thirteen IDs well-formed, no duplicates, `RACE`/`LOCK`/`TXN`/`COORD` disjoint from every other backend rule file's categories.

- **Done when**: `framework/rules/concurrency-backend.md` carries thirteen well-formed rules (8 MUST / 5 SHOULD), each new MUST a scale-independent corruption hazard and each cross-link resolving to an existing rule, and the spec's category table and AC6 and the plan's rule table and §Trade-offs match the file.

## 6. Rule-file review 2026-10-19 (second pass): land amended concurrency rules

- [x] `BE-RACE-002` (SHOULD): the Rationale says why it is SHOULD — confinement is not always the simpler design, as Effective Go notes of a reference count kept by a mutex around an integer; Source added (Effective Go, *Concurrency* — Share by communicating)
- [x] `BE-RACE-003` (MUST): the Statement adds a MAY — a check-then-act whose act is harmless to repeat (two callers computing and storing the same value) may leave the check and the act non-atomic, each access still synchronized per `BE-RACE-001` — and Verification accepts a plan stating that repeating the act is harmless; the accepted data-store mechanisms add a foreign-key constraint; the Rationale cites PostgreSQL `INSERT … ON CONFLICT` and `performance-backend.md` `BE-CACHE-003` for cache stampede; Source expanded
- [x] `BE-LOCK-001` (SHOULD): the Rationale states how optimistic and pessimistic control each fail, matching how SQL Server's documentation frames the choice; Source added (SQL Server *Transaction locking and row versioning guide*)
- [x] `BE-LOCK-002` (SHOULD): the Rationale cites the PostgreSQL, SQL Server and MySQL lock-ordering guidance and PostgreSQL's indefinite lock wait, and makes the lock timeout and hold bound each a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`; Source added
- [x] `BE-TXN-001` (SHOULD): the Rationale notes that `READ_COMMITTED_SNAPSHOT` is the default on Azure SQL Database but not on Azure SQL Managed Instance; Source widened (Berenson et al., the SQL Server row-versioning guide, Oracle *Database Concepts*)
- [x] `BE-TXN-002` (MUST) Source: adds PostgreSQL *Transaction Isolation* and Jepsen *MySQL 8.0.34*
- [x] `BE-TXN-003` (MUST): the Rationale says how far a locking read reaches into absent rows varies by engine (PostgreSQL vs. InnoDB) and describes Oracle's SERIALIZABLE (`ORA-08177`) as snapshot isolation; Source adds Fekete et al. (ACM TODS 2005) and the MySQL and Oracle documentation
- [x] `BE-TXN-004` (SHOULD): the Rationale is engine-specific — PostgreSQL rolls back the transaction, SQL Server and InnoDB a deadlock victim's whole transaction, Oracle only the statement that hit the deadlock — so the handling path rolls back explicitly and retries the whole transaction rather than re-issuing the failed statement; it is SHOULD because the choice between retrying and reporting the conflict is contextual; Source adds PostgreSQL *Serialization Failure Handling* and the SQL Server, MySQL and Oracle documentation
- [x] `BE-COORD-001` (MUST): the Statement's reason clause reads "a paused holder, a delayed request, or a clock that jumps"; the Rationale adds that the protected resource must check the token itself; Source names locks for efficiency vs. correctness
- [x] `BE-COORD-002` (MUST): the trigger reads a scheduler that can start the operation more than once for one scheduled time (a Kubernetes CronJob, an in-process scheduler running in every replica) or start a run while the previous run is still in progress; the Rationale's scheduled-job deduplication is a run record claimed atomically per scheduled time (`BE-RACE-003`) or a per-item claim, Kafka's exactly-once boundary is stated, and a database effect's deduplication record commits in the same transaction (`BE-COORD-003`); Source adds the CronJob concurrency policy and Kafka *Message Delivery Semantics*
- [x] `BE-COORD-003` (MUST): the Rationale says two-phase commit across the database and the broker is often not an option; Source quotes the transactional-outbox pattern
- [x] `BE-COORD-004` (MUST): the Rationale adds Amazon SQS FIFO message groups and the RabbitMQ and Kafka reordering cases, and makes the stale-message guard a per-entity sequence number or version, not a wall-clock timestamp; Source adds Kafka producer `retries`, SQS FIFO delivery, the RabbitMQ *Queues* guide, and Kleppmann on clock jumps
- [x] `spec.md`: the scope table's `RACE` row adds the harmless-to-repeat exception and its `COORD` row the overlapping scheduled run; a Boundaries bullet names `performance-backend.md` `BE-CACHE-003`, and the operator-tunable bullet reads a named constant or an environment variable per `CFG-CONST-003`; AC6 adds `BE-CACHE-003` and its claim that the file mentions no pools or connections still holds; the MUST-vs-SHOULD Resolved Question records the `BE-RACE-003` narrowing
- [x] `plan.md`: the rule-table gists for `BE-RACE-003`, `BE-COORD-001` and `BE-COORD-002` match the file; the cross-reference paragraph and Cross-spec impact add `BE-CACHE-003`; §Trade-offs records the `RACE-003` narrowing as a decision update dated 2026-10-19 and gives `TXN-004`'s SHOULD the file's reason
- [x] Verify: `npx markdownlint-cli2` on the changed spec files and `scripts/lint-frontmatter.sh` pass, and a read-only `derive-dependencies` reports no cycle

- **Done when**: 038's spec and plan describe `framework/rules/concurrency-backend.md` as it stands after the second 2026-10-19 rule-file review: thirteen rules, eight MUST, every rule with a Source, `BE-RACE-003`'s harmless-to-repeat MAY recorded as a decision update, and `BE-CACHE-003` cited. No rule ID, category, or tier changed. Re-run `/ductus:review` before 038 returns to `done`.
