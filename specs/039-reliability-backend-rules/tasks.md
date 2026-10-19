# 039 — Backend reliability rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Author `framework/rules/reliability-backend.md`

- [x] Write the file header: title (`# Reliability Rules — Backend`), an intro scoping it to server-side resilience under partial failure, the RFC 2119 note, the `BE-{CATEGORY}-{NNN}` ID-format / category-declaration line (categories `TIMEOUT`/`RETRY`/`BREAKER`/`DRAIN`/`BULK`, with the `See specs/008-security-rules/data-model.md` pointer), the default-SHOULD + design-time-commitment paragraph, and the backend pin/surface note.
- [x] Write the eight rules across the five `## BE-{CATEGORY}` sections per the plan's rule table: `BE-TIMEOUT-001` (MUST) / `BE-TIMEOUT-002` (SHOULD), `BE-RETRY-001` (MUST) / `BE-RETRY-002` (SHOULD), `BE-BREAKER-001` (SHOULD), `BE-DRAIN-001` (MUST) / `BE-DRAIN-002` (SHOULD), `BE-BULK-001` (SHOULD) — each with Statement (one RFC 2119 keyword), Rationale, and a design-time-commitment Verification clause, plus Source where apt.
- [x] Cross-reference rather than restate: `BE-RETRY-001` cites `api-backend.md` `BE-IDEMP`; `BE-DRAIN-002` cites `observability-backend.md` `BE-HEALTH-001`; `BE-BULK-001` cites `performance-backend.md` `BE-ASYNC`/`BE-POOL-*`; `BE-TIMEOUT-001` cites `BE-POOL-002`; tunable values cite `configuration-cross.md` `CFG-*`.
- Done when: the file exists with eight well-formed rules (3 MUST / 5 SHOULD), categories disjoint from the other backend files, each MUST a scale-independent availability/cascading-failure risk, and the four 034-deferred concerns (deadlines, timeouts, retries, breakers) all present.

## 2. Register the file in the `/ductus` manifest

- [x] Add `framework/rules/reliability-backend.md → specs/rules/reliability-backend.md` to the `### ductus-owned shared files` table in `framework/bootstrap/ductus.md`, slotted between `quality-cross.md` and `security-backend.md` (alphabetical; strategy: update).
- Done when: the manifest row is present; the `-backend.md` suffix makes 024's loader select it under the `backend` surface and 033's filter include it. (The §Shared Files note is count-free — no count edit.)

## 3. Validate

- [x] `scripts/lint-rule-ids.sh` passes (`BE-TIMEOUT`/`BE-RETRY`/`BE-BREAKER`/`BE-DRAIN`/`BE-BULK` IDs well-formed; categories disjoint from the other backend files; no duplicates).
- [x] `scripts/lint-rule-filenames.sh` passes (the `-backend.md` suffix).
- [x] `npx markdownlint-cli2`, the other `scripts/lint-*.sh`, and `scripts/audit/*` pass (frontmatter, tool-coverage, procedure-parseability, manifest-parity, ssot-invariants, cross-doc-consistency — all green).
- Done when: all lints/audits green.

## 4. Review and complete

- [x] Run `/ductus:review` over the change set; resolve any MUST findings.
- Done when: `/ductus:review` reports no blocking violations and the spec can advance to `done`.

## 5. Rule-file review 2026-10-19: land new and amended reliability rules

- [x] `BE-TIMEOUT-001` (MUST) amended: bounds outbound calls and blocking waits on another process or host, local file I/O out of scope; the timeout is a named constant or environment variable per `CFG-CONST-003`, and an unnamed client-library default is flagged
- [x] `BE-TIMEOUT-002` (SHOULD) amended: the Rationale adds propagating the deadline as a remaining timeout; Source added
- [x] `BE-RETRY-001` (MUST) amended: the attempt bound and backoff parameters are named constants or environment variables per `CFG-CONST-003`; Source revised
- [x] `BE-RETRY-002` (SHOULD) amended: Source added
- [x] `BE-RETRY-003` (SHOULD) added: retry only failures classified as transient, waiting at least a server-supplied `Retry-After`; cites `BE-API-004` and `BE-STATUS-001`
- [x] `BE-RETRY-004` (SHOULD) added: retry a failing dependency at one layer of the call path only
- [x] `BE-RETRY-005` (MUST, owner-confirmed 2026-10-19) added: a message that exhausts its redelivery bound is dead-lettered, or halts its strictly ordered stream and alerts, and is never redelivered indefinitely or discarded silently; cites `BE-COORD-002` and `CFG-CONST-003`
- [x] `BE-BREAKER-001` (SHOULD) amended: opens on a failure threshold (a count or rate of failures); thresholds cite `CFG-CONST-003`; Source extended
- [x] `BE-DRAIN-001` (MUST) amended: names the grace period, which with any pre-stop delay fits inside the platform's forced-kill deadline, as a named constant or environment variable per `CFG-CONST-003`; Source revised
- [x] `BE-DRAIN-002` (SHOULD) rewritten: leave routing and keep accepting requests for a bounded propagation delay before closing intake, counted within the grace period
- [x] `BE-BULK-001` (SHOULD) amended: states the `BE-ASYNC-001`/`BE-POOL-001`/`BE-POOL-002` boundary, the overload status per `BE-STATUS-001`, saturation via `BE-METRIC-002`, and bounds per `CFG-CONST-003`
- [x] `BE-BULK-002` (SHOULD) added: classify each dependency critical or non-critical, and degrade rather than fail the operation when a non-critical one fails; cites `BE-HEALTH-001` and `BE-BREAKER-001`
- [x] File header amended: `RETRY` declared as retries and dead-lettering, `BULK` as backpressure, bulkheads, and graceful degradation; the MUST examples name the poison message; the opt-out paragraph states what `[rules] surfaces`, `[[review.disabled-rule-files]]` and `[pinned] files` each do
- [x] `spec.md` and `plan.md` reconciled to the file: category table, Severity posture and its 2026-10-19 decision update (four MUSTs), category-set decision update, Boundaries, AC6's `CFG-CONST-003` citation list, the `api-backend.md` category enumeration, and AC9 for the added rules; the plan's counts, header description, rule table, cross-references, trade-offs and cross-spec impact
- [x] Verify: `scripts/lint-rule-ids.sh` passes, `npx markdownlint-cli2` is clean on the changed files, and `scripts/lint-frontmatter.sh` passes

- **Done when**: `framework/rules/reliability-backend.md` carries the four added rules and the revised launch eight, each with Statement / Rationale / Verification / Source; its header declares the widened `RETRY` and `BULK` concerns; `spec.md` and `plan.md` describe the twelve-rule, four-MUST set the file holds; and `scripts/lint-rule-ids.sh` accepts every ID. Re-run `/ductus:review` before 039 advances back to `done`.

## 6. Rule-file review 2026-10-19 (second pass): land amended reliability rules

- [x] `BE-TIMEOUT-002` (SHOULD): Verification makes the request deadline an operator-tunable value, a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`
- [x] `BE-RETRY-001` (MUST) Source: adds the Google SRE book, ch. 22 (limit retries per request; randomized exponential backoff)
- [x] `BE-RETRY-002` (SHOULD): the Statement's example is a cap on retries as a fraction of requests, replacing the token bucket; Verification makes the budget a named constant or environment variable per `CFG-CONST-003`
- [x] `BE-RETRY-003` (SHOULD): the Rationale adds a downstream's "overloaded; don't retry" signal, which is surfaced rather than retried; Source adds the SRE book's ch. 21 and notes that a `429` may carry `Retry-After`
- [x] `BE-RETRY-004` (SHOULD): the Rationale's multiplication example follows the SRE book (three layers each issuing three retries, four attempts, so 64 attempts); Source names the 64-attempt example
- [x] `BE-RETRY-005` (MUST): the Rationale requires the dead-letter destination to be retained long enough to be acted on (an SQS dead-lettered message keeps its original enqueue timestamp) and back-references `concurrency-backend.md` `BE-COORD-004` for the ordering scope a consumer relies on; Source adds FIFO ordering and retention
- [x] `BE-DRAIN-001` (MUST): the Rationale gives the Kubernetes termination sequence — the `terminationGracePeriodSeconds` countdown starts, any `preStop` hook runs, the container's stop signal is sent, and SIGKILL follows when the countdown expires; Source adds Container Lifecycle Hooks
- [x] `BE-DRAIN-002` (SHOULD): the Rationale speaks of the stop signal rather than SIGTERM; Verification makes the propagation delay a named constant or environment variable per `CFG-CONST-003`; Source adds the Amazon EKS Best Practices Guide on load-balancer deregistration
- [x] `BE-BULK-001` (SHOULD): a shed request's overload status is `503` for saturation, or `429` where the limit is a per-client rate, citing `security-backend.md` `BE-API-004` beside `api-backend.md` `BE-STATUS-001`; Source adds RFC 9110 §15.6.4 and RFC 6585 §4 and names *Release It!*, 2nd ed.
- [x] `BE-BULK-002` (SHOULD): the Rationale says the degraded path should not be a second distributed mechanism that can itself fail or add load during the outage; Source adds Gabrielson, "Avoiding fallback in distributed systems" (Amazon Builders' Library, 2019)
- [x] `spec.md`: AC6's `CFG-CONST-003` citation list adds `BE-TIMEOUT-002`, `BE-RETRY-002` and `BE-DRAIN-002` (nine rules); the operator-tunable Boundaries bullet adds request deadlines, retry budgets and routing-propagation delays; the duplicate-delivery bullet adds `BE-RETRY-005`'s `BE-COORD-004` citation; the status-codes bullet has `BE-BULK-001` cite `BE-API-004` as well as `BE-STATUS-001`
- [x] `plan.md`: the `BE-RETRY-002` row drops the token bucket; the cross-reference sentence adds `BE-RETRY-005` → `BE-COORD-004`, `BE-BULK-001` → `BE-API-004`, and the nine `CFG-CONST-003` citers with the three new tunables; Cross-spec impact adds `BE-COORD-004`
- [x] `tasks.md` task 1: its cross-reference item no longer claims `BE-BREAKER-001` cites `BE-POOL-002`, which no committed version of the file has done; `BE-TIMEOUT-001` is the citer
- [x] Verify: `npx markdownlint-cli2` on the changed spec files and `scripts/lint-frontmatter.sh` pass, and a read-only `derive-dependencies` reports no cycle

- **Done when**: 039's spec and plan describe `framework/rules/reliability-backend.md` as it stands after the second 2026-10-19 rule-file review: twelve rules, four MUST, nine rules citing `CFG-CONST-003`, `BE-RETRY-005` back-referencing `BE-COORD-004`, and `BE-BULK-001` citing `BE-API-004`. No rule ID, category, or tier changed. Re-run `/ductus:review` before 039 returns to `done`.

## 7. Rule-file review 2026-10-19 (owner decisions): BE-RETRY-005 without alerting; BE-TIMEOUT-003 server-side statement timeout

- [x] `BE-RETRY-005` (MUST, stays MUST) reworded per the owner decision that alerting does not belong in rule files: the Statement requires a retained dead-letter destination where the team responsible for the consumer can review and redrive the message, or, for a strictly ordered stream, a halt recorded with the message that caused it — no longer "monitored" or "alert"; the Rationale adds that how the team watches either is its own operational choice (the `FE-CSP-005` pattern); the Verification asks for the destination's retention, where a halt is recorded, and how the team reviews and redrives, replacing "how dead-lettered messages reach an operator"
- [x] `BE-TIMEOUT-003` (SHOULD) added: a datastore query issued on a request path or by a worker is bounded on the server by the datastore's own time limit, no longer than the caller's timeout or remaining deadline, or, where none covers it, by a client timeout that cancels it on the server; the Rationale states PostgreSQL `statement_timeout` (default 0, per-session advice), MySQL `max_execution_time` (read-only `SELECT` statements only), MongoDB `maxTimeMS`, SQL Server's client-driver query timeout (no server-side statement timeout named), PostgreSQL's uncertain out-of-band cancel and `client_connection_check_interval`, and why SHOULD; it cites `BE-POOL-002`, `BE-POOL-005` and `CFG-CONST-003`, each vendor claim fetched from the publisher's documentation
- [x] `BE-TIMEOUT-001` (MUST) Rationale: points to `BE-TIMEOUT-003` for the server-side limit that stops a datastore query
- [x] `spec.md`: the `TIMEOUT` and `RETRY` rows of the category table, the pooling and operator-tunable Boundaries bullets, AC6 (ten `CFG-CONST-003` citers), AC9's `BE-RETRY-005` wording, AC10 added for `BE-TIMEOUT-003`, the MUST-vs-SHOULD Resolved Question updated, and the owner decision on `BE-RETRY-005` recorded as a dated Resolved Question
- [x] `plan.md`: thirteen rules, the `BE-TIMEOUT-003` and `BE-RETRY-005` rows of the rule table, the cross-reference sentence (`BE-TIMEOUT-003` → `BE-POOL-002`/`BE-POOL-005`, ten `CFG-CONST-003` citers), Trade-offs, Affected Files and Cross-spec impact
- [x] Verify: `scripts/lint-rule-ids.sh`, `npx markdownlint-cli2` on the rule file and the changed spec files, `scripts/audit/shipped-anchor-resolution.sh` and `scripts/lint-frontmatter.sh` pass, and a read-only `derive-dependencies` reports no cycle

- **Done when**: `framework/rules/reliability-backend.md` carries `BE-RETRY-005` with no monitoring or alerting requirement and `BE-TIMEOUT-003` as a SHOULD with Statement / Rationale / Verification / Source; 039's spec and plan describe the thirteen-rule, four-MUST set with ten rules citing `CFG-CONST-003`; the owner decision is recorded as a dated Resolved Question; and every lint above passes. Re-run `/ductus:review` before 039 returns to `done`.
