# Observability Rules — Backend

Enforceable observability rules for server-side metrics, distributed tracing, and health signaling. These rules apply to projects adopting `ductus` whose surface includes a backend.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `BE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `METRIC` (metrics), `TRACE` (distributed tracing), `HEALTH` (health endpoints). See `specs/008-security-rules/data-model.md` for the full schema.

Observability rules default to **SHOULD** — the right metrics, spans, and probes are context-dependent. A rule is **MUST** only when its absence blinds operators in a way that prevents detection or diagnosis of an outage regardless of scale. These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts. Logging and audit-trail rules live in the `BE-LOG` category of `security-backend.md`, not here; `BE-TRACE-003` governs only which trace identifiers a log record carries, extending `BE-LOG-006`.

Projects without a backend need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger. In `.ductus/config.toml`, a `[rules] surfaces` list without `backend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## BE-METRIC — Metrics

### BE-METRIC-001

> Each request-handling path SHOULD commit to RED metrics — request **r**ate, **e**rror rate, and **d**uration (latency distribution) — so a degradation is detectable from request-side signals.

**Rationale:** Rate, errors, and duration are the minimal signal set that distinguishes "healthy," "slow," and "failing" for any request handler. A path with no RED metrics is invisible on a dashboard: an error spike or latency regression is noticed only when a user complains.

**Verification:** Any spec or plan that introduces a request-handling path (HTTP route, RPC handler, message consumer) SHOULD name the RED metrics it emits. `/{project}:analyze` flags request-handling features whose plan commits to no rate/error/duration instrumentation.

**Source:** The RED method (Tom Wilkie, created 2015) — Grafana Labs, *The RED Method: How to Instrument Your Services* (2018); Google SRE book, ch. 6 *Monitoring Distributed Systems* (the four golden signals: latency, traffic, errors, saturation); Prometheus documentation — *Instrumentation* (online-serving systems: queries, errors, latency).

### BE-METRIC-002

> Each managed resource — a connection pool, or a worker or thread pool and the work queue that feeds it — SHOULD commit to USE metrics: **u**tilization, **s**aturation, and **e**rror count.

**Rationale:** Resource exhaustion — every pooled connection checked out, a work backlog that keeps growing — reaches the RED metrics of `BE-METRIC-001` only as its symptom, rising latency or errors, which does not say which resource ran out. USE metrics name it: saturation is the work a resource has but cannot yet service, usually queued, which is where a backlog becomes visible.

**Verification:** Any spec or plan that introduces a bounded resource pool, queue, or worker set SHOULD name its USE metrics. `/{project}:analyze` flags resource-introducing plans with no saturation or utilization signal.

**Source:** The USE method (Brendan Gregg) — *The USE Method* (brendangregg.com; first published in ACM Queue as "Thinking Methodically about Performance", 2012), which applies to software resources such as thread pools, mutex locks, and process, thread, and file-descriptor capacity.

### BE-METRIC-003

> Metric label/tag sets SHOULD be bounded-cardinality — no unbounded or caller-controlled value (user ID, request ID, raw URL path, email) used as a label.

**Rationale:** Each distinct label combination is a separate time series; an unbounded label explodes storage and query cost and can exhaust the metrics backend — the resource-exhaustion class `performance-backend.md` addresses. This rule governs cardinality only: a sensitive value such as an email address is barred from labels whatever its cardinality by `security-backend.md` `BE-LOG-008`.

**Verification:** Any spec or plan that introduces metric labels SHOULD confirm each label is drawn from a bounded set — a request path is labelled by its route template (`/users/{id}`), not the raw path. `/{project}:analyze` flags plans that propose high-cardinality identifiers (user/request IDs, raw paths) as label values.

**Source:** Prometheus documentation — *Metric and label naming* (every unique label combination is a new time series; no labels for user IDs, email addresses, or other unbounded sets of values) and *Instrumentation* (do not overuse labels); OpenTelemetry HTTP semantic conventions (Stable) — `http.route`, the matched route template, which must be low-cardinality.

### BE-METRIC-004

> Latency SHOULD be recorded as a histogram, which aggregates across instances — with exponential buckets created dynamically at a chosen resolution, or with fixed buckets chosen to cover the expected latency range — and reported by percentile, not by a mean alone or by per-instance precomputed quantiles.

**Rationale:** A mean hides the slow tail users actually feel: the Google SRE book's example is a web service averaging 100 ms at 1,000 requests per second in which 1% of requests might easily take 5 seconds. Precomputed quantiles (a client-side summary) are computed per instance and cannot be meaningfully combined — averaging per-instance quantiles yields statistically nonsensical values — so a fleet-wide percentile needs a histogram. Recording the distribution is what makes the "duration" of `BE-METRIC-001` able to show a tail regression.

**Verification:** Any spec or plan that commits to latency metrics (per `BE-METRIC-001`) SHOULD name the distribution type and the percentiles it reports. `/{project}:analyze` flags plans whose only latency signal is a mean, or a per-instance summary on a service that runs more than one instance.

**Source:** Prometheus documentation — *Histograms and summaries* (summaries are not aggregatable; native histograms, whose exponential buckets follow a configured resolution, are preferred where available, and classic histograms take buckets picked for the expected range); Google SRE book, ch. 6 *Monitoring Distributed Systems* ("Worrying About Your Tail").

### BE-METRIC-005

> Each scheduled or batch job SHOULD emit the timestamp of its last successful run, together with its run outcome and duration, so a job that stops running or keeps failing is detectable.

**Rationale:** A scheduled job handles no requests, so the RED metrics of `BE-METRIC-001` never see it, and a job that silently stops being scheduled emits no error at all — the missing signal is the failure. Time since last success is the one signal that exposes both a job that keeps failing and a job that no longer runs, once it exceeds the job's expected interval.

**Verification:** Any spec or plan that introduces a scheduled, periodic, or batch job (cron task, scheduled worker, recurring export) SHOULD name its last-success timestamp metric and its outcome and duration signals. `/{project}:analyze` flags job-introducing plans that commit to no last-success signal.

**Source:** Prometheus documentation — *Instrumentation* (batch jobs: the last time the job succeeded is its key metric, alongside its overall runtime and the last time it completed, successfully or not).

## BE-TRACE — Distributed tracing

### BE-TRACE-001

> Trace context MUST be propagated across every service boundary the feature crosses — inbound context extracted and continued (or, for a consumed message, linked), outbound context injected into every downstream call and published message (e.g. W3C `traceparent` / `tracestate`, carried in message headers or attributes across a broker) — so a single request stays correlatable end-to-end. At a trust boundary — a request from an untrusted external caller, or a call out to an external system — a service MAY restart the trace instead of continuing the inbound context.

**Rationale:** Without propagation a distributed failure fragments into disconnected per-service spans, and the broken hop is invisible until an incident forces a manual reconstruction. `security-backend.md` `BE-LOG-006` already requires a correlation ID to cross every hop and reach every log line, and an opaque one (`X-Request-ID`) satisfies it; this rule requires that what crosses the boundary be trace context — the trace ID, the parent ID (the caller's span), and the trace flags — so each service's spans join one trace rather than merely sharing a tag. A consumed message is linked to the context it was created in rather than parented to it, the default the OpenTelemetry messaging conventions use. The restart permission follows W3C Trace Context, which lets a service restart the trace at a trust boundary and advises keeping `tracestate` out of requests to external systems. The absence makes cross-service failures undebuggable regardless of scale, which is why it is MUST.

**Verification:** Any spec or plan that adds or crosses a service boundary (calls another service, enqueues to or consumes from a broker) MUST commit to extracting inbound and injecting outbound trace context (on the message itself for a broker hop) — or, for a hop that is a trust boundary, state that the trace restarts there. `/{project}:analyze` flags cross-service plans that do not state trace-context propagation.

**Source:** W3C Trace Context (W3C Recommendation, 23 November 2021) — `traceparent` / `tracestate`; its privacy and security considerations (restarting a trace at a trust boundary; `tracestate` in requests to external systems); OpenTelemetry Semantic Conventions — messaging spans (Development status: a producer attaches a creation context to each message, and consumer spans link to it).

### BE-TRACE-002

> Significant units of work — outbound calls, database queries, expensive computation — SHOULD be wrapped in a span with a low-cardinality name (the operation or route template, never an embedded ID) carrying meaningful attributes (target, operation, result).

**Rationale:** Spans around significant work turn a trace into a latency breakdown, making the slow hop in a request obvious. Without them a trace shows that a request was slow but not where. A span name that embeds an ID (`get_user/314159`) defeats grouping spans by operation, the same way a high-cardinality metric label does (`BE-METRIC-003`). Which values may not appear in span attributes at all is `security-backend.md` `BE-LOG-008`.

**Verification:** Any spec or plan that introduces significant units of work SHOULD name the spans it creates and their key attributes. `/{project}:analyze` flags plans for externally-dependent or compute-heavy work that commit to no spans, and span names that interpolate per-request values.

**Source:** OpenTelemetry Trace API — span names (`get_user`, not `get_user/314159`); OpenTelemetry HTTP semantic conventions (Stable) — span name `{method} {target}`, with `http.route` as a server span's target; OpenTelemetry Semantic Conventions, *Recording errors* (Development status) — span status `Error` on a failed operation.

### BE-TRACE-003

> A log record emitted within a traced operation SHOULD carry the active trace ID and span ID — as the `BE-LOG-006` correlation ID or beside it — so each log line joins the trace and span that produced it.

**Rationale:** `security-backend.md` `BE-LOG-006` requires a correlation ID in every log line but leaves the primitive open; an opaque request ID satisfies it yet cannot be joined to a trace, so an operator who has found the slow or failing span must still search logs by time window. Carrying the trace and span IDs makes the pivot between a span and its log lines direct, in both directions. This rule governs only which identifiers a log record carries; what is logged, and how, stays in `BE-LOG`.

**Verification:** Any spec or plan for a service that commits to both tracing (per `BE-TRACE-001` / `BE-TRACE-002`) and logging SHOULD state that log records carry the trace ID and span ID. `/{project}:analyze` flags traced services whose logging commitment names only an opaque request ID.

**Source:** OpenTelemetry Logs Data Model (Stable) — `TraceId` / `SpanId` fields; W3C Trace Context.

### BE-TRACE-004

> A service that records traces SHOULD state the sampling policy for the traces it starts, and SHOULD respect the inbound `sampled` flag for the traces it continues (parent-based sampling), so a request recorded upstream is recorded at every hop.

**Rationale:** Independent per-service sampling decisions produce broken traces: the caller recorded the request but a downstream hop dropped its spans, leaving a hole exactly where the investigation needs one. Respecting the propagated flag keeps a sampled trace whole. Stating the root policy — a ratio, a rate limit, or tail-based retention at a collector — makes trace volume a deliberate choice and tells an operator whether a missing trace means the request never happened or was not sampled. The flag is the caller's recommendation, not a command: W3C Trace Context warns that a public-facing service which naively continues any trace with the flag set invites abuse, so at an untrusted edge the service restarts the trace (`BE-TRACE-001`) and applies its own root policy.

**Verification:** Any spec or plan that introduces tracing for a service SHOULD name the sampling policy for root spans and state that continued traces honor the inbound `sampled` flag. `/{project}:analyze` flags tracing plans that state no sampling policy, or that sample each service independently of the propagated decision.

**Source:** W3C Trace Context — `trace-flags` (`sampled`: a component making a recording decision should respect it) and its security considerations; OpenTelemetry Trace SDK — `ParentBased` sampler (the SDK default is `ParentBased(root=AlwaysOn)`); OpenTelemetry *Sampling* concepts (head and tail sampling); Jaeger sampling documentation (probabilistic and rate-limiting samplers).

## BE-HEALTH — Health endpoints

### BE-HEALTH-001

> A service MUST expose a readiness signal distinct from its liveness signal, and that reflects the reachability of the dependencies it needs to serve, so an instance that cannot serve is not sent traffic. A service deployed on a platform that offers no per-instance readiness hook — for example AWS Lambda, which sends an execution environment invocations only after its Init phase ends and whose function configuration defines no readiness probe — MAY omit the readiness signal, provided its spec or plan names the platform and states what stands in for readiness (such as the platform's own initialization gate) or that nothing does.

**Rationale:** A missing readiness signal — or one that returns healthy unconditionally — lets the orchestrator route traffic to instances that cannot reach their database, cache, or upstreams, turning a bad deploy or a dependency blip into served errors with no signal. Kubernetes treats a container that defines no readiness probe as ready, and a rolling update counts a new instance available once it is ready and does not remove old instances until a sufficient number of new ones have come up, so readiness is what holds a rollout back from replacing healthy instances with broken ones; its absence ships silent bad deploys regardless of scale, which is why it is MUST. Readiness that is the liveness check either restarts instances over a dependency outage or ignores dependencies altogether, which is why the two are distinct. Which dependencies are critical — and so gate readiness — is classified under `reliability-backend.md` `BE-BULK-002`; a readiness response that reports dependency state redacts sensitive configuration values per `configuration-cross.md` `CFG-ENV-006`. The exception covers a platform with nothing for a readiness signal to drive: on AWS Lambda an execution environment's Init phase ends when its runtime and extensions signal ready, and only then does it receive an invocation, so initialization is the gate the platform applies to each environment before it serves — the function configuration defines no probe that re-checks a dependency once an environment is serving. Requiring the spec to name the platform and its stand-in keeps that missing gate a stated decision an operator can see rather than a silent gap. The exception turns on the platform, not on being serverless: a Cloud Run service accepts a readiness probe, which stops routing new requests to an instance that fails it, so a service there commits to readiness as usual.

**Verification:** Any spec or plan for a deployable service MUST commit to a readiness endpoint or signal distinct from liveness, and state which dependencies gate readiness — or, for a service on a platform that offers no per-instance readiness hook, name the platform and state what stands in for readiness, or that nothing does, instead of committing to a readiness endpoint. `/{project}:analyze` flags service plans with no readiness signal and no such named platform and stand-in, or with readiness conflated with liveness.

**Source:** Kubernetes documentation — liveness, readiness, and startup probes (a failed readiness probe removes the Pod from Service endpoints; a probe a container does not define counts as success; for an app with strict back-end dependencies, readiness additionally checks each required service while liveness passes when the app itself is healthy); *Deployments* (a Pod counts as available once ready; readiness probe failures can stall a rollout); AWS Lambda Developer Guide — *Understanding the Lambda execution environment lifecycle* (the Init phase ends when the runtime and all extensions send a `Next` API request, and the `Invoke` event is sent in response to `Next`); AWS Lambda API Reference — `CreateFunction` (the function configuration has no readiness or health-probe setting); Google Cloud Run documentation — *Configure container health checks for services* (startup, liveness, and readiness probes; an instance failing its readiness probe stops receiving new requests without being shut down).

### BE-HEALTH-002

> A liveness probe SHOULD reflect only whether the process is alive and able to make progress (not deadlocked) and SHOULD NOT fail on dependency unavailability; a slow-starting service SHOULD cover initialization with a startup probe rather than a lengthened liveness delay or threshold.

**Rationale:** Conflating liveness with dependency reachability causes restart storms during a downstream outage — restarting the container cannot fix an unreachable database, so the restarts amplify the incident; Kubernetes cautions that a liveness probe must truly indicate unrecoverable failure, such as a deadlock, and that an incorrect one can lead to cascading failures. A startup probe holds off the liveness and readiness checks until it succeeds, so slow initialization is covered without tuning the liveness probe itself for the worst-case startup, which Kubernetes notes can compromise its fast response to deadlocks. Keeping readiness distinct from liveness is `BE-HEALTH-001`'s obligation; this rule governs what liveness itself checks.

**Verification:** Any spec or plan that defines health probes SHOULD keep liveness independent of dependency reachability and use a startup probe for slow initialization. `/{project}:analyze` flags plans where liveness fails on dependency unavailability, or where slow initialization is absorbed by a long liveness initial delay instead of a startup probe. Probe intervals and timeouts are operator-tunable values governed by `configuration-cross.md` `CFG-CONST-*` / `CFG-ENV-*`.

**Source:** Kubernetes documentation — liveness, readiness, and startup probes (the liveness caution on cascading failures; a startup probe defers liveness and readiness checks until it succeeds); *Configure Liveness, Readiness and Startup Probes* ("Protect slow starting containers with startup probes").
