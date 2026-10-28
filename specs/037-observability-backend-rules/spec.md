---
status: in-progress
dependencies: [008-security-rules, 016-cross-cutting-rules, 024-rule-loader, 033-rule-surface-setting, 034-performance-backend-rules]
next-criterion: 8
---

# 037 — Backend observability rules

Introduce `framework/rules/observability-backend.md`, an enforceable rule set covering server-side observability concerns beyond logging and audit. The file follows the canonical rule schema and is installed and enforced under the **backend** surface. This is a rule-introducing feature spec — the same path [008-security-rules](../008-security-rules/spec.md) used — and one of the three Task-9 backend sets that follow [034-performance-backend-rules](../034-performance-backend-rules/spec.md).

## Motivation

The constitution names **observability** a governance-recognized cross-cutting category (§rules), and the principles list calls the system to be **Supportable** and **Observable**. When this spec was written, the backend surface covered only the logging/audit slice via `security-backend.md` §BE-LOG; the rest of observability — metrics, distributed tracing, health/readiness, SLOs, alerting — had nowhere to be promoted to the rules tier. Each was re-litigated per feature or discovered missing only during an incident, when the absence of a metric, span, or readiness probe is what turns a degradation into an outage.

This spec closes that gap for metrics, distributed tracing, and health/readiness — SLOs and alerting are excluded as operational policy (Resolved Questions): a backend observability rule set citable by ID (`BE-{CATEGORY}-{NNN}`) that `/ductus:analyze` checks against specs and plans (design-time observability commitments) and `/ductus:review` references where applicable.

## Rule set scope

`observability-backend.md` uses the **backend** surface (`-backend.md` suffix, `BE-` ID prefix), with NEW categories disjoint from every category the other `-backend.md` rule files declare in their headers (`security-backend.md`, `api-backend.md`, `performance-backend.md`, and the rest). Category set (resolved at clarify — three categories ship; `SLO` and `ALERT` are deferred as operational policy, see Resolved Questions):

| Category | Abbrev | Concern |
| --- | --- | --- |
| Metrics | `METRIC` | RED (rate/errors/duration) for request handling; USE (utilization/saturation/errors) for resources; bounded-cardinality labels; latency as an aggregatable histogram — exponential buckets at a chosen resolution, or fixed buckets covering the expected range — reported by percentile, not by a mean alone; last-success, outcome, and duration for scheduled and batch jobs |
| Distributed tracing | `TRACE` | trace-context propagation across service boundaries, broker hops included, with a trace that may restart at a trust boundary; low-cardinality spans around significant work; trace and span IDs on log records; a stated sampling policy that honors the inbound `sampled` flag |
| Health endpoints | `HEALTH` | readiness distinct from liveness and reflecting dependency reachability, which a service on a platform with no per-instance readiness hook may omit by naming the platform and what stands in for readiness; liveness that does not fail on a dependency outage; startup probes for slow initialization |

### Severity posture

Observability rules default to **SHOULD** (the right metrics, spans, and probes are context-dependent). A rule is **MUST** only when its absence blinds operators in a way that prevents detection or diagnosis of an outage *regardless of scale* — e.g. no readiness probe (silent bad deploys), broken trace-context propagation (undebuggable distributed failures).

### Boundaries (cross-reference, do not duplicate)

- **Logging and audit trail** are already `security-backend.md` §BE-LOG — this set cites `BE-LOG-006` (and, below, `BE-LOG-008`) and covers the rest of observability. `BE-LOG-006` already requires a correlation ID to cross every hop and reach every log line, in any primitive; `TRACE` extends it rather than restating it, on *what* is carried — trace context across the boundary (`BE-TRACE-001`), the trace and span IDs on a log record (`BE-TRACE-003`). What is logged, and how, stays in §BE-LOG.
- **Sensitive values in telemetry** are `security-backend.md` `BE-LOG-008` and `configuration-cross.md` `CFG-ENV-006` — `BE-METRIC-003` governs label cardinality only and points to `BE-LOG-008` for the values barred from labels whatever their cardinality, `BE-TRACE-002` points there for the values barred from span attributes, and `BE-HEALTH-001` cites `CFG-ENV-006` for redacting sensitive configuration values from a readiness response that reports dependency state.
- **Operator-tunable values** (the probe intervals and timeouts `BE-HEALTH-002` names) are `configuration-cross.md` `CFG-CONST-*` / `CFG-ENV-*`, which govern how they are named and validated; this set cites them rather than restating them.
- **Platforms without a readiness hook** sit outside `BE-HEALTH-001`'s MUST: a service on a platform that offers no per-instance readiness hook (the rule's example is AWS Lambda) MAY omit the readiness signal, provided its spec or plan names the platform and states what stands in for readiness, or that nothing does. The exception turns on the platform, not on being serverless — a Cloud Run service accepts a readiness probe, so it commits to readiness as usual.

## Acceptance Criteria

- [x] AC1: `framework/rules/observability-backend.md` exists, ends in the `-backend.md` suffix, and follows the canonical rule schema (`### {ID}` headings; Statement / Verification, no Rationale; RFC 2119 language) per [008-security-rules](../008-security-rules/spec.md)'s data-model.
- [x] AC2: Every rule ID uses the `BE-{CATEGORY}-{NNN}` format with an observability category disjoint from the `security-backend.md`, `api-backend.md`, and `performance-backend.md` category sets; `scripts/lint-rule-ids.sh` passes.
- [x] AC3: The file header declares the observability category abbreviations per the per-file category-declaration policy ([016-cross-cutting-rules](../016-cross-cutting-rules/spec.md)).
- [x] AC4: The rule set covers, at minimum, metrics, distributed tracing, and health endpoints — each with a Verification clause expressed as a **design-time commitment** the spec/plan must make (not a code-pattern grep), consistent with how `/ductus:analyze` audits artifacts.
- [x] AC5: Each MUST rule is one whose absence prevents detection or diagnosis of an outage regardless of scale; contextual observability trade-offs are SHOULD. The split is evident from the Statements.
- [x] AC6: Rules whose surface overlaps an existing rule cite it rather than restating it (`BE-LOG-006` for tracing/correlation, `BE-LOG-008` for sensitive values in metric labels and span attributes, `CFG-*` for tunable config, and `CFG-ENV-006` for redaction in a readiness response).
- [x] AC7: The file is added to the `/ductus` **Shared Files** manifest in `framework/bootstrap/ductus.md` and is selected under the `backend` surface by `/ductus:review`, composing with [033-rule-surface-setting](../033-rule-surface-setting/spec.md) and [024-rule-loader](../024-rule-loader/spec.md).

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Exact category set and abbreviations.** Resolved: **ship three categories — `METRIC`, `TRACE`, `HEALTH`** (the minimum AC #4 requires), each disjoint from the existing `BE-` category sets in `security-backend.md` / `api-backend.md` / `performance-backend.md`. `SLO` and `ALERT` are deferred (see next question). The `BE-{CATEGORY}-{NNN}` grammar leaves room to add them later.
- **Do SLOs and alerting belong in a rules file?** Resolved: **deferred — out of scope for this rule set.** SLOs and alerting are operational policy, not per-feature design commitments: `/ductus:analyze` verifies a rule by checking whether a spec/plan makes a commitment, but a feature spec does not "define an SLO" or "author an alert" — those live at the service/ops layer. They do not fit the design-time-commitment verification model the other backend rules use. They can be promoted later if a concrete per-feature commitment shape emerges. **Reaffirmed by owner decision, 2026-10-19:** SLOs and alerting stay out of rule files. The 2026-10-19 rule-file review briefly added latency-objective rules to `performance-backend.md` and a security-alerting rule to `security-backend.md`; both were removed on this basis. The signals an objective or an alert would be built on stay in scope — `BE-METRIC-004` requires latency to be recorded as a percentile-reportable distribution, and `BE-METRIC-005` a scheduled job's last-success time — but no rule sets a target or requires an alert.
- **MUST vs. SHOULD default.** Resolved: **default SHOULD; MUST only when the absence prevents detection or diagnosis of an outage regardless of scale.** Mirrors 034's performance posture. The two MUSTs at launch: a readiness probe (its absence ships silent bad deploys) and trace-context propagation (its absence makes distributed failures undebuggable). Metric/span *coverage* choices stay SHOULD.
- **Overlap with `BE-LOG-006`.** Resolved: **`TRACE` extends and cites `BE-LOG-006`, never duplicates it.** `BE-LOG-006` owns the correlation ID: assigned at the edge, carried across every downstream call and queue message, and present in every log line, audit entry, and error response — in any primitive, so an opaque `X-Request-ID` satisfies it. `TRACE` adds what that leaves open. `BE-TRACE-001` requires that what crosses each boundary be trace context — the trace ID, the parent ID (the caller's span), and the trace flags — so each service's spans join one trace rather than merely sharing a tag, while letting a service restart the trace at a trust boundary instead of continuing the inbound context; `BE-TRACE-003` requires a log record emitted within a traced operation to carry the trace ID and span ID, as the correlation ID or beside it, so a log line joins its span. `BE-TRACE-002` (spans around significant work) and `BE-TRACE-004` (sampling) have no `BE-LOG` counterpart. What is logged, and how, stays in `security-backend.md` §BE-LOG; tunable config such as probe intervals and timeouts cites `CFG-*`.
- **Does `BE-HEALTH-001`'s readiness MUST bind a service whose platform offers no readiness hook?** Resolved by owner decision, 2026-10-19: **no — such a service MAY omit the readiness signal, provided its spec or plan names the platform and states what stands in for readiness (such as the platform's own initialization gate), or that nothing does.** The Verification trigger, "any spec or plan for a deployable service", otherwise reached services on platforms where nothing consumes a readiness signal. The MAY sits beside the MUST in one Statement, as `BE-TRACE-001`'s trust-boundary restart does, so the rule stays MUST and the set keeps its two MUSTs. The rule names AWS Lambda as the example, from fetched primary documentation: an execution environment receives invocations only after its Init phase ends, and the `CreateFunction` configuration has no readiness probe setting. Cloud Run was considered and rejected as the example, because its services accept readiness probes; the Verification cites it as the contrast that the exception turns on the platform, not on being serverless.
