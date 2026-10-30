# 037 — Backend observability rules Plan

Implements [037 — Backend observability rules](spec.md).

## Overview

A rule-introducing, markdown-tier feature — the 034-performance-backend path exactly. It adds one rule file, `framework/rules/observability-backend.md`, on the **existing** `BE` surface (so no `scripts/lint-rule-ids.sh` change and no `data-model.md` — unlike 036, which introduced a new `QUAL` surface), and wires it into the `/ductus` Shared Files manifest. The §Shared Files note is already count-free (036), so no count to update.

Three categories ship — `METRIC`, `TRACE`, `HEALTH` — with exactly two MUST rules; every other rule is SHOULD. Verification is **design-time commitment** framing (what a spec/plan must state), enforced by `/ductus:analyze` against feature artifacts, matching how the 034 performance rules work.

## Technical Decisions

### The rule file — `framework/rules/observability-backend.md`

Modeled on `performance-backend.md` (same surface, same analyze-time design-time-commitment framing):

- **Header.** Title `# Observability Rules — Backend` and one paragraph: the scope (server-side observability beyond logging, for a backend surface); these verify design-time commitments enforced by `/{project}:analyze`; a rule is **MUST** only when its absence prevents detection or diagnosis of an outage regardless of scale, the rest **SHOULD**; logging and audit-trail rules live in `security-backend.md` §BE-LOG, and `BE-TRACE-003` governs only which trace identifiers a log record carries, extending `BE-LOG-006`; the ID-format / category-declaration line stating IDs follow `BE-{CATEGORY}-{NNN}` with categories `METRIC` (metrics), `TRACE` (distributed tracing), `HEALTH` (health endpoints); and a pointer to constitution §rules for severity, ID permanence, and how review selects or excludes the file (008's `rules-are-as-short-as-their-required-content`).
- **Categories disjoint** from `security-backend.md`, `api-backend.md`, `performance-backend.md` (verified: `METRIC`/`TRACE`/`HEALTH` appear in none).

### Rule set

The rules below, across three `## BE-{CATEGORY}` sections. Verification is phrased as a design-time commitment a spec/plan must make (not a code grep), per AC #4.

| ID | Sev | Statement gist |
| --- | --- | --- |
| `BE-METRIC-001` | SHOULD | Each request-handling path commits to RED metrics (request **r**ate, **e**rror rate, **d**uration). |
| `BE-METRIC-002` | SHOULD | Each managed resource — a connection pool, or a worker or thread pool and the work queue that feeds it — commits to USE metrics (utilization, saturation, errors). |
| `BE-METRIC-003` | SHOULD | Metric label sets are bounded-cardinality — no unbounded/user-controlled values as labels; a request path is labelled by its route template, not the raw path. Governs cardinality only: a sensitive value is barred from labels whatever its cardinality by `security-backend.md` `BE-LOG-008`, or `configuration-cross.md` `CFG-ENV-006` for a configuration value. |
| `BE-METRIC-004` | SHOULD | Latency is recorded as a histogram, which aggregates across instances — exponential buckets created dynamically at a chosen resolution, or fixed buckets chosen to cover the expected range — and reported by percentile, not by a mean alone or, on a multi-instance service, by per-instance precomputed quantiles. |
| `BE-METRIC-005` | SHOULD | Each scheduled or batch job emits its last-success timestamp with its run outcome and duration, so a job that stops running or keeps failing is detectable. |
| `BE-TRACE-001` | **MUST** | Trace context is propagated across every service boundary the feature crosses — extracted and continued inbound (linked, for a consumed message), injected into every downstream call and published message (W3C `traceparent` / `tracestate`, in message headers or attributes across a broker); its absence makes distributed failures undebuggable. At a trust boundary — a request from an untrusted external caller, or a call out to an external system — a service MAY restart the trace instead of continuing the inbound context, and Verification accepts a plan stating that the trace restarts at such a hop. Extends/cites `security-backend.md` `BE-LOG-006`, which already carries a correlation ID across every hop in any primitive: this rule requires that what crosses be trace context — the trace ID, the parent ID (the caller's span), and the trace flags — so each service's spans join one trace. |
| `BE-TRACE-002` | SHOULD | Significant work (external calls, DB queries, expensive computation) is wrapped in a span with a low-cardinality name (the operation or route template, never an embedded ID) and meaningful attributes. Which values may not appear in span attributes at all is `security-backend.md` `BE-LOG-008`, or `configuration-cross.md` `CFG-ENV-006` for a configuration value. |
| `BE-TRACE-003` | SHOULD | A log record emitted within a traced operation carries the active trace ID and span ID — as the `BE-LOG-006` correlation ID or beside it — so each log line joins its trace and span. Governs only the identifiers; what is logged stays in `BE-LOG`. |
| `BE-TRACE-004` | SHOULD | A service that records traces states the sampling policy for the traces it starts and respects the inbound `sampled` flag for the traces it continues (parent-based sampling), so a request recorded upstream is recorded at every hop. |
| `BE-HEALTH-001` | **MUST** | The service exposes a readiness signal distinct from liveness, and readiness reflects required-dependency reachability — so a not-ready instance is not routed traffic (its absence ships silent bad deploys). A service on a platform that offers no per-instance readiness hook — the example is AWS Lambda, where an execution environment is invoked only after its Init phase ends and the function configuration defines no readiness probe — MAY omit the signal, provided its spec or plan names the platform and states what stands in for readiness or that nothing does; Verification accepts that statement in place of a readiness endpoint (owner decision 2026-10-19). Which dependencies are critical, and so gate readiness, is classified under `reliability-backend.md` `BE-BULK-002`; a readiness response that reports dependency state redacts sensitive configuration values per `configuration-cross.md` `CFG-ENV-006`. |
| `BE-HEALTH-002` | SHOULD | Liveness reflects only that the process is alive and making progress, and does not fail on dependency unavailability, to avoid restart storms during a downstream outage; a slow-starting service covers initialization with a startup probe rather than a lengthened liveness delay or threshold. Keeping readiness distinct from liveness is `BE-HEALTH-001`'s obligation. |

Operator-tunable values these rules name (the probe intervals and timeouts in `BE-HEALTH-002`) are governed by `configuration-cross.md` `CFG-*`, which governs how they are named and validated. Cited, not restated.

### Manifest wiring — `framework/bootstrap/ductus.md`

Add `| \`framework/rules/observability-backend.md\` | \`specs/rules/observability-backend.md\` |` to the `### ductus-owned shared files` table, slotted between `configuration-cross.md` and `performance-backend.md` (alphabetical). The `-backend.md` suffix means 024's loader selects it under the `backend` surface and 033's surface filter includes it when `backend` is configured — no change needed in either. The §Shared Files note is count-free, so no count edit.

### What this feature does NOT touch

- **`scripts/lint-rule-ids.sh`** — `BE` is already a registered surface; the regex accepts `BE-METRIC-001` etc. unchanged.
- **`data-model.md`** — no new surface and no new schema; the file follows 008's rule schema and declares its categories in its own header (016 policy). No data-model artifact (matching 034).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/observability-backend.md` | Create | The observability rule set (`METRIC`/`TRACE`/`HEALTH`, two MUST, the rest SHOULD) |
| `framework/bootstrap/ductus.md` | Modify | Add the manifest row (between `configuration-cross` and `performance-backend`) |

## Trade-offs

- **Three categories vs. five.** Chose `METRIC`/`TRACE`/`HEALTH`; deferred `SLO`/`ALERT` (clarify resolution, reaffirmed by owner decision 2026-10-19 — see the spec's Resolved Questions) — they are operational policy, not per-feature design commitments `/ductus:analyze` can check against a spec/plan. The `BE-{CATEGORY}` grammar admits them later.
- **Analyze-time vs. review-time verification.** Chose analyze-time design-time commitments (like 034), not code-pattern review (like 036's `QUAL-STUB-001`). Observability is something a spec/plan *commits to* up front; the absence of a metric or probe is a planning gap, caught best against artifacts.
- **Two MUSTs only.** Readiness-distinct-from-liveness and trace-context propagation are the two absences that blind operators regardless of scale; everything else is contextual coverage (SHOULD). Keeping the MUST set tight avoids forcing instrumentation that may not fit every feature.
- **Known limitation.** `/ductus:analyze` checks that a spec/plan *commits* to these; it cannot verify the commitment is actually implemented in code — that residual gap is what `/ductus:review` and tests cover, the same limitation every design-time rule carries.

## Cross-spec impact

The spec/plan reference 008, 016, 024, 033, 034 (deps) and cite `BE-LOG-006` and `BE-LOG-008` (security-backend; `BE-LOG-008` from `BE-METRIC-003` and `BE-TRACE-002`), `CFG-*` and `CFG-ENV-006` (configuration-cross; `CFG-ENV-006` from `BE-HEALTH-001`, `BE-METRIC-003` and `BE-TRACE-002`), and `reliability-backend.md` `BE-BULK-002` (from `BE-HEALTH-001`). None need an edit: `BE-LOG-006` is cited/extended, not changed; `BE-LOG-008`, `CFG-*`, `CFG-ENV-006`, and `BE-BULK-002` are cited; 024/033 already select `-backend.md` files by suffix/surface; 034 is the sibling precedent, unaffected. The `BE-LOG-008` link runs both ways: `security-backend.md` `BE-LOG-008` cites `BE-METRIC-003` (cardinality for cost; `BE-LOG-008` bars sensitive values whatever their cardinality), and `BE-METRIC-003` cites it back. Informational; does not block.
