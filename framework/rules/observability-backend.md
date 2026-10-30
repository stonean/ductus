# Observability Rules — Backend

Observability rules for server-side metrics, distributed tracing, and health signaling, for projects whose surface includes a backend. They verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts. A rule is **MUST** only when its absence blinds operators in a way that prevents detection or diagnosis of an outage regardless of scale; the rest are **SHOULD**. Logging and audit-trail rules live in the `BE-LOG` category of `security-backend.md`, not here; `BE-TRACE-003` governs only which trace identifiers a log record carries, extending `BE-LOG-006`. Rule IDs follow the format `BE-{CATEGORY}-{NNN}`. Categories: `METRIC` (metrics), `TRACE` (distributed tracing), `HEALTH` (health endpoints). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## BE-METRIC — Metrics

### BE-METRIC-001

> Each request-handling path SHOULD commit to RED metrics — request **r**ate, **e**rror rate, and **d**uration (latency distribution).

**Verification:** Any spec or plan that introduces a request-handling path (HTTP route, RPC handler, message consumer) SHOULD name the RED metrics it emits. `/{project}:analyze` flags request-handling features whose plan commits to no rate/error/duration instrumentation.

**Source:** The RED method — Grafana Labs, *The RED Method: How to Instrument Your Services* (2018); Google SRE book, ch. 6 *Monitoring Distributed Systems* (the four golden signals); Prometheus documentation — *Instrumentation* (online-serving systems).

### BE-METRIC-002

> Each managed resource — a connection pool, or a worker or thread pool and the work queue that feeds it — SHOULD commit to USE metrics: **u**tilization, **s**aturation, and **e**rror count.

**Verification:** Any spec or plan that introduces a bounded resource pool, a worker set, or the work queue that feeds one SHOULD name its USE metrics. `/{project}:analyze` flags resource-introducing plans with no saturation or utilization signal. Saturation is the work a resource has but cannot yet service, usually queued.

**Source:** The USE method (Brendan Gregg) — *The USE Method* (brendangregg.com; first published in ACM Queue as "Thinking Methodically about Performance", 2012).

### BE-METRIC-003

> Metric label/tag sets SHOULD be bounded-cardinality — no unbounded or caller-controlled value (user ID, request ID, raw URL path, email) used as a label.

**Verification:** Any spec or plan that introduces metric labels SHOULD confirm each label is drawn from a bounded set — a request path is labelled by its route template (`/users/{id}`), not the raw path. `/{project}:analyze` flags plans that propose high-cardinality identifiers (user/request IDs, raw paths) as label values. This rule governs cardinality only: a sensitive value used as a label, such as an email address, is `security-backend.md` `BE-LOG-008`'s whatever its cardinality, or `configuration-cross.md` `CFG-ENV-006`'s when it is a configuration value.

**Source:** Prometheus documentation — *Metric and label naming* and *Instrumentation* (do not overuse labels); OpenTelemetry HTTP semantic conventions (Stable) — `http.route`.

### BE-METRIC-004

> Latency SHOULD be recorded as a histogram — with exponential buckets created dynamically at a chosen resolution, or with fixed buckets chosen to cover the expected latency range — and reported by percentile, not by a mean alone or, on a service that runs more than one instance, by per-instance precomputed quantiles.

**Verification:** Any spec or plan that commits to latency metrics (per `BE-METRIC-001`) SHOULD name the distribution type and the percentiles it reports. `/{project}:analyze` flags plans whose only latency signal is a mean, or a per-instance summary (precomputed quantiles) on a service that runs more than one instance.

**Source:** Prometheus documentation — *Histograms and summaries* (summaries are not aggregatable; native histograms, whose exponential buckets follow a configured resolution, are preferred where available, and classic histograms take buckets picked for the expected range); Google SRE book, ch. 6 *Monitoring Distributed Systems* ("Worrying About Your Tail").

### BE-METRIC-005

> Each scheduled or batch job SHOULD emit the timestamp of its last successful run, together with its run outcome and duration.

**Verification:** Any spec or plan that introduces a scheduled, periodic, or batch job (cron task, scheduled worker, recurring export) SHOULD name each signal the Statement requires. `/{project}:analyze` flags job-introducing plans that commit to no last-success signal.

**Source:** Prometheus documentation — *Instrumentation* (batch jobs).

## BE-TRACE — Distributed tracing

### BE-TRACE-001

> Trace context MUST be propagated across every service boundary the feature crosses — inbound context extracted and continued (or, for a consumed message, linked), outbound context injected into every downstream call and published message (e.g. W3C `traceparent` / `tracestate`, carried in message headers or attributes across a broker). At a trust boundary — a request from an untrusted external caller, or a call out to an external system — a service MAY restart the trace instead of continuing the inbound context.

**Verification:** Any spec or plan that adds or crosses a service boundary (calls another service, enqueues to or consumes from a broker) MUST commit to extracting inbound and injecting outbound trace context (on the message itself for a broker hop) — or, for a hop that is a trust boundary, state that the trace restarts there. `/{project}:analyze` flags cross-service plans that do not state trace-context propagation. Trace context is the trace ID, the parent ID (the caller's span), and the trace flags: an opaque correlation ID (`X-Request-ID`) is not trace context, though it satisfies `security-backend.md` `BE-LOG-006`, which owns a correlation ID missing from a hop or a log line.

**Source:** W3C Trace Context (W3C Recommendation, 23 November 2021) — `traceparent` / `tracestate`; its privacy and security considerations (restarting a trace at a trust boundary; `tracestate` in requests to external systems); OpenTelemetry Semantic Conventions — messaging spans (Development status).

### BE-TRACE-002

> Significant units of work — outbound calls, database queries, expensive computation — SHOULD be wrapped in a span with a low-cardinality name (the operation or route template, never an embedded ID) carrying meaningful attributes (target, operation, result).

**Verification:** Any spec or plan that introduces significant units of work SHOULD name the spans it creates and their key attributes. `/{project}:analyze` flags plans for externally-dependent or compute-heavy work that commit to no spans, and span names that interpolate per-request values. **Not flagged:** a sensitive value in a span attribute, which `security-backend.md` `BE-LOG-008` owns, or `configuration-cross.md` `CFG-ENV-006` when it is a configuration value.

**Source:** OpenTelemetry Trace API — span names (`get_user`, not `get_user/314159`); OpenTelemetry HTTP semantic conventions (Stable) — span name `{method} {target}`, with `http.route` as a server span's target; OpenTelemetry Semantic Conventions, *Recording errors* (Development status) — span status `Error` on a failed operation.

### BE-TRACE-003

> A log record emitted within a traced operation SHOULD carry the active trace ID and span ID — as the `BE-LOG-006` correlation ID or beside it.

**Verification:** Any spec or plan for a service that commits to both tracing (per `BE-TRACE-001` / `BE-TRACE-002`) and logging SHOULD state that log records carry the trace ID and span ID. `/{project}:analyze` flags traced services whose logging commitment names only an opaque request ID. This rule governs only which identifiers a log record carries: what is logged, and how, is `security-backend.md` `BE-LOG`'s.

**Source:** OpenTelemetry Logs Data Model (Stable) — `TraceId` / `SpanId` fields; W3C Trace Context.

### BE-TRACE-004

> A service that records traces SHOULD state the sampling policy for the traces it starts, and SHOULD respect the inbound `sampled` flag for the traces it continues (parent-based sampling).

**Verification:** Any spec or plan that introduces tracing for a service SHOULD name the sampling policy for root spans — such as a ratio, a rate limit, or tail-based retention at a collector — and state that continued traces honor the inbound `sampled` flag. `/{project}:analyze` flags tracing plans that state no sampling policy, or that sample each service independently of the propagated decision. **Not flagged:** a service that restarts the trace at an untrusted edge, per `BE-TRACE-001`, and applies its own root policy there.

**Source:** W3C Trace Context — `trace-flags` (`sampled`) and its security considerations; OpenTelemetry Trace SDK — `ParentBased` sampler; OpenTelemetry *Sampling* concepts (head and tail sampling); Jaeger sampling documentation (probabilistic and rate-limiting samplers).

## BE-HEALTH — Health endpoints

### BE-HEALTH-001

> A service MUST expose a readiness signal distinct from its liveness signal, and that reflects the reachability of the dependencies it needs to serve. A service deployed on a platform that offers no per-instance readiness hook — for example AWS Lambda, which sends an execution environment invocations only after its Init phase ends and whose function configuration defines no readiness probe — MAY omit the readiness signal, provided its spec or plan names the platform and states what stands in for readiness (such as the platform's own initialization gate) or that nothing does.

**Verification:** Any spec or plan for a deployable service MUST commit to a readiness endpoint or signal distinct from liveness, and state which dependencies gate readiness — or, for a service on a platform that offers no per-instance readiness hook, name the platform and state what stands in for readiness, or that nothing does, instead of committing to a readiness endpoint. `/{project}:analyze` flags service plans with no readiness signal and no such named platform and stand-in, with readiness conflated with liveness, or whose readiness signal leaves out a dependency the plan treats as critical to one of its operations. The exception covers a platform with nothing for a readiness signal to drive, and turns on the platform, not on being serverless: a Cloud Run service accepts a readiness probe, which stops routing new requests to an instance that fails it, so a service there commits to readiness as usual. **Not flagged:** a dependency not classified as critical or non-critical, or one that gates readiness though critical to none of the service's operations, which `reliability-backend.md` `BE-BULK-002` owns; and a readiness response that reports dependency state with a sensitive configuration value unredacted, which `configuration-cross.md` `CFG-ENV-006` owns.

**Source:** Kubernetes documentation — liveness, readiness, and startup probes; AWS Lambda Developer Guide — *Understanding the Lambda execution environment lifecycle* (the Init phase ends when the runtime and all extensions send a `Next` API request, and the `Invoke` event is sent in response to `Next`); AWS Lambda API Reference — `CreateFunction` (the function configuration has no readiness or health-probe setting); Google Cloud Run documentation — *Configure container health checks for services* (startup, liveness, and readiness probes; an instance failing its readiness probe stops receiving new requests without being shut down).

### BE-HEALTH-002

> A liveness probe SHOULD reflect only whether the process is alive and able to make progress (not deadlocked) and SHOULD NOT fail on dependency unavailability; a slow-starting service SHOULD cover initialization with a startup probe rather than a lengthened liveness delay or threshold.

**Verification:** Any spec or plan that defines health probes SHOULD keep liveness independent of dependency reachability and use a startup probe for slow initialization. `/{project}:analyze` flags plans where liveness fails on dependency unavailability, or where slow initialization is absorbed by a long liveness initial delay instead of a startup probe. This rule governs what liveness itself checks: readiness conflated with liveness is `BE-HEALTH-001`'s. A combined probe that checks dependencies is also `BE-HEALTH-001`'s, a separate defect: the conflation, where this rule's is liveness failing on a dependency. Probe intervals and timeouts are operator-tunable values governed by `configuration-cross.md` `CFG-CONST-*` / `CFG-ENV-*`.

**Source:** Kubernetes documentation — liveness, readiness, and startup probes (the liveness caution on cascading failures); *Configure Liveness, Readiness and Startup Probes* ("Protect slow starting containers with startup probes").
