# 037 — Backend observability rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Author `framework/rules/observability-backend.md`

- [x] Write the file header: title (`# Observability Rules — Backend`), an intro scoping it to server-side observability beyond logging, the RFC 2119 note, the `BE-{CATEGORY}-{NNN}` ID-format / category-declaration line (categories `METRIC`/`TRACE`/`HEALTH`, with the `See specs/008-security-rules/data-model.md` pointer), the default-SHOULD + design-time-commitment paragraph, and the backend pin/surface note.
- [x] Write the seven launch rules across the three `## BE-{CATEGORY}` sections per the plan's rule table: `BE-METRIC-001/002/003` (SHOULD), `BE-TRACE-001` (MUST) / `BE-TRACE-002` (SHOULD), `BE-HEALTH-001` (MUST) / `BE-HEALTH-002` (SHOULD) — each with Statement (one RFC 2119 keyword), Rationale, and a design-time-commitment Verification clause, plus Source where apt.
- [x] Cross-reference rather than restate: `BE-TRACE-001` extends/cites `security-backend.md` `BE-LOG-006`; `BE-METRIC-003` cites `performance-backend.md` for the cardinality-exhaustion angle; tunable values cite `configuration-cross.md` `CFG-*`.
- Done when: the file exists with seven well-formed launch rules (2 MUST / 5 SHOULD), categories disjoint from the other backend files, each MUST a detection/diagnosis-blocking absence.

## 2. Register the file in the `/ductus` manifest

- [x] Add `framework/rules/observability-backend.md → specs/rules/observability-backend.md` to the `### ductus-owned shared files` table in `framework/bootstrap/ductus.md`, slotted between `configuration-cross.md` and `performance-backend.md` (strategy: update).
- Done when: the manifest row is present; the `-backend.md` suffix makes 024's loader select it under the `backend` surface and 033's filter include it. (The §Shared Files note is already count-free — no count edit.)

## 3. Validate

- [x] `scripts/lint-rule-ids.sh` passes (`BE-METRIC`/`BE-TRACE`/`BE-HEALTH` IDs well-formed; categories disjoint from `security-backend.md` / `api-backend.md` / `performance-backend.md`; no duplicates).
- [x] `scripts/lint-rule-filenames.sh` passes (the `-backend.md` suffix).
- [x] `npx markdownlint-cli2`, the other `scripts/lint-*.sh`, and `scripts/audit/*` pass (frontmatter, tool-coverage, procedure-parseability, manifest-parity, ssot-invariants, cross-doc-consistency — all green).
- Done when: all lints/audits green.

## 4. Review and complete

- [x] Run `/ductus:review` over the change set; resolve any MUST findings.
- Done when: `/ductus:review` reports no blocking violations and the spec can advance to `done`.

## 5. Rule-file review 2026-10-19: land new and amended observability rules

- [x] Add `BE-METRIC-004` (SHOULD): latency recorded as an aggregatable histogram and reported by percentile, not by mean or per-instance precomputed quantiles.
- [x] Add `BE-METRIC-005` (SHOULD): each scheduled or batch job emits its last-success timestamp, run outcome, and duration.
- [x] Add `BE-TRACE-003` (SHOULD): a log record within a traced operation carries the active trace ID and span ID, as or beside the `BE-LOG-006` correlation ID.
- [x] Add `BE-TRACE-004` (SHOULD): state the sampling policy for traces the service starts and respect the inbound `sampled` flag for traces it continues.
- [x] Amend `BE-TRACE-001` (MUST): consumed messages link to inbound context, outbound context is injected into published messages via broker headers or attributes, and the rationale distinguishes trace context from `BE-LOG-006`'s opaque correlation ID.
- [x] Amend `BE-TRACE-002`: spans carry a low-cardinality name (operation or route template, never an embedded ID), and analyze flags span names that interpolate per-request values.
- [x] Amend `BE-METRIC-001` Source: adds the Google SRE four golden signals.
- [x] Amend `BE-METRIC-003`: route-template labelling example in Verification; Prometheus Source added.
- [x] Amend `BE-HEALTH-001` rationale: which dependencies are critical, and so gate readiness, is classified under `reliability-backend.md` `BE-BULK-002`.
- [x] Amend `BE-HEALTH-002`: liveness SHOULD NOT fail on dependency unavailability; slow initialization uses a startup probe rather than a lengthened liveness delay or threshold.
- [x] Update the header paragraphs: the `BE-TRACE-003` / `BE-LOG-006` scoping sentence, and the backend applicability note (analyze loads every file; `[rules] surfaces`, `[[review.disabled-rule-files]]`, and `[pinned] files` each scoped to what they actually stop).
- [x] Sync spec, plan, and tasks to the file on disk; record the owner decision (2026-10-19) reaffirming that SLOs and alerting stay out of rule files; `scripts/lint-rule-ids.sh` passes.

- **Done when**: `framework/rules/observability-backend.md` carries every rule listed above with Statement, Rationale, Verification, and Source; no new category is introduced; exactly two rules (`BE-TRACE-001`, `BE-HEALTH-001`) are MUST; the spec's scope table, Boundaries, and Resolved Questions and the plan's rule table match the file; `scripts/lint-rule-ids.sh` passes.

## 6. Rule-file review 2026-10-19 (second pass): land amended observability rules

- [x] `BE-METRIC-001` (SHOULD) Source: the RED method dated (Tom Wilkie, created 2015) and cited to Grafana Labs, *The RED Method: How to Instrument Your Services* (2018); the Google SRE book's four golden signals named (ch. 6); Prometheus *Instrumentation* added
- [x] `BE-METRIC-002` (SHOULD): the Statement's example narrowed to a connection pool, or a worker or thread pool and the work queue that feeds it, with the cache example removed; the Rationale rewritten — exhaustion reaches the RED metrics of `BE-METRIC-001` only as a symptom, and saturation is the work a resource has but cannot yet service; Source expanded
- [x] `BE-METRIC-003` (SHOULD): the Rationale governs cardinality only and cites `security-backend.md` `BE-LOG-008` for sensitive values barred from labels whatever their cardinality; Source adds the Prometheus label guidance and OpenTelemetry `http.route`
- [x] `BE-METRIC-004` (SHOULD): latency recorded as a histogram with exponential buckets created dynamically at a chosen resolution, or fixed buckets chosen to cover the expected range, and reported by percentile, not by a mean alone or by per-instance precomputed quantiles; the Rationale uses the Google SRE book's tail example; Source expanded
- [x] `BE-METRIC-005` (SHOULD) Source: details the Prometheus batch-job guidance (last success, overall runtime, last completion)
- [x] `BE-TRACE-001` (MUST): the Statement adds a MAY — at a trust boundary (a request from an untrusted external caller, or a call out to an external system) a service may restart the trace instead of continuing the inbound context — and Verification accepts a plan stating that the trace restarts at such a hop; the Rationale names the parent ID (the caller's span), explains linking a consumed message, and grounds the restart in W3C Trace Context; Source expanded
- [x] `BE-TRACE-002` (SHOULD): the Rationale cites `security-backend.md` `BE-LOG-008` for values barred from span attributes; Source adds the span-name example, the OpenTelemetry HTTP conventions, and *Recording errors*
- [x] `BE-TRACE-003` (SHOULD) Source: the OpenTelemetry Logs Data Model marked Stable
- [x] `BE-TRACE-004` (SHOULD): the Rationale treats the inbound `sampled` flag as the caller's recommendation, so at an untrusted edge the service restarts the trace (`BE-TRACE-001`) and applies its own root policy; Source expanded
- [x] `BE-HEALTH-001` (MUST): the Rationale explains from Kubernetes behavior why readiness holds a rollout back and why it is distinct from liveness, and cites `configuration-cross.md` `CFG-ENV-006` for redacting sensitive configuration values from a readiness response that reports dependency state; Source expanded
- [x] `BE-HEALTH-002` (SHOULD): the Rationale adds the Kubernetes liveness caution on cascading failures and says a startup probe avoids tuning the liveness probe for the worst-case startup; Source expanded
- [x] File header: logging and audit-trail rules placed in the `BE-LOG` category of `security-backend.md` (wording only)
- [x] `spec.md`: the scope table's `METRIC` row names the histogram's exponential or fixed buckets and "not by a mean alone", and its `TRACE` row the trust-boundary restart; a Boundaries bullet names `BE-LOG-008` and `CFG-ENV-006`; AC6's parenthetical adds both; the `BE-LOG-006` Resolved Question names the parent ID (the caller's span) and the restart. SLOs and alerting stay excluded (owner decision unchanged)
- [x] `plan.md`: the rule-table gists for `BE-METRIC-002`, `BE-METRIC-003`, `BE-METRIC-004`, `BE-TRACE-001`, `BE-TRACE-002` and `BE-HEALTH-001` match the file; Cross-spec impact cites `BE-LOG-008` in both directions and `CFG-ENV-006`
- [x] Verify: `npx markdownlint-cli2` on the changed spec files and `scripts/lint-frontmatter.sh` pass, and a read-only `derive-dependencies` reports no cycle

- **Done when**: 037's spec and plan describe `framework/rules/observability-backend.md` as it stands after the second 2026-10-19 rule-file review: eleven rules, `BE-TRACE-001` and `BE-HEALTH-001` the only MUSTs, `BE-TRACE-001`'s trust-boundary restart and `BE-METRIC-004`'s bucket choices reflected, and `BE-LOG-008` / `CFG-ENV-006` cited. No rule ID, category, or tier changed. Re-run `/ductus:review` before 037 returns to `done`.

## 7. Rule-file review 2026-10-19 (owner decision): BE-HEALTH-001 exception for platforms without a readiness hook

- [x] `BE-HEALTH-001` (MUST): the Statement adds a MAY — a service on a platform that offers no per-instance readiness hook (for example AWS Lambda) may omit the readiness signal, provided its spec names the platform and states what stands in for readiness or that nothing does
- [x] `BE-HEALTH-001` Rationale, Verification, and Source: the Rationale grounds the exception in Lambda's Init-before-Invoke gate and cites Cloud Run, whose services accept readiness probes, as the contrast; Verification accepts a spec that names such a platform and its stand-in instead of a readiness endpoint, and analyze flags a plan with neither; Source adds the Lambda execution environment lifecycle, the `CreateFunction` API reference, and Cloud Run health checks, each fetched
- [x] `spec.md`: the scope table's `HEALTH` row and a Boundaries bullet carry the exception; a Resolved Question records the owner decision (2026-10-19)
- [x] `plan.md`: the `BE-HEALTH-001` rule-table gist carries the exception
- [x] Verify: `scripts/lint-rule-ids.sh`, `npx markdownlint-cli2` on the rule file and the changed spec files, `scripts/audit/shipped-anchor-resolution.sh`, and `scripts/lint-frontmatter.sh` pass

- **Done when**: `BE-HEALTH-001` carries the platform exception as a MAY beside its MUST, with Rationale, Verification, and Source consistent with it and every external claim fetched from primary documentation; 037's spec and plan match the file; no rule ID, category, or tier changed, and `BE-TRACE-001` and `BE-HEALTH-001` remain the only MUSTs. Re-run `/ductus:review` before 037 returns to `done`.
