# Reliability Rules — Backend

Reliability rules for server-side resilience under partial failure — timeouts and deadlines, retries and retry-exhausted messages, circuit breakers, graceful shutdown, backpressure, and graceful degradation. They verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts. A rule is **MUST** only when its absence is an availability or cascading-failure risk regardless of scale; the rest are **SHOULD**. Rule IDs follow the format `BE-{CATEGORY}-{NNN}`. Categories: `TIMEOUT` (timeouts and deadlines), `RETRY` (retries and dead-lettering), `BREAKER` (circuit breakers), `DRAIN` (graceful shutdown), `BULK` (backpressure, bulkheads, and graceful degradation). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## BE-TIMEOUT — Timeouts and deadlines

### BE-TIMEOUT-001

> Every outbound network call, downstream-service call, or blocking wait on another process or host MUST be made under a bounded timeout — no call waits unboundedly for a response.

**Verification:** Any spec or plan that introduces an outbound call or a blocking wait on another process or host MUST name the timeout that bounds it, as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`; `/{project}:analyze` flags one that adds a downstream or remote call with no stated timeout, an unnamed client-library default included. **Not flagged:** local file I/O. This rule bounds the caller's wait on the call: an unbounded wait for a pooled connection is `performance-backend.md`'s `BE-POOL-002`, and a datastore query left running on the server is `BE-TIMEOUT-003`'s.

**Source:** Nygard, *Release It!* — Timeouts; gRPC documentation — Deadlines; Requests documentation — Timeouts.

### BE-TIMEOUT-002

> A request that fans out to downstream calls SHOULD carry an end-to-end deadline that is propagated to those calls and shrinks as time is consumed.

**Verification:** Any spec or plan that fans out to multiple downstream calls SHOULD name the request deadline, as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`, and how it is propagated; `/{project}:analyze` flags one that bounds each call but not the overall request.

**Source:** Google SRE book, ch. 22 "Addressing Cascading Failures" — deadline propagation; gRPC documentation — Deadlines (deadline propagation).

### BE-TIMEOUT-003

> A database or other datastore query issued on a request path or by a worker SHOULD be bounded on the server — by the datastore's own time limit for a statement or operation, set no longer than the caller's timeout or remaining deadline — or, where the datastore has no such limit for that kind of statement, by a client timeout whose expiry cancels the query on the server rather than only ending the caller's wait.

**Verification:** Any spec or plan that issues datastore queries on a request path or from a worker SHOULD name the server-side time limit those queries run under, as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`, and show it no longer than the caller's timeout (`BE-TIMEOUT-001`) or remaining deadline (`BE-TIMEOUT-002`), or, for a query no server-side limit covers — a MySQL write, which `max_execution_time` does not reach, for one — state that the client timeout cancels the query on the server; `/{project}:analyze` flags a data-access plan whose only bound on a query is a client-side wait — an application-level timer that merely stops waiting sends no cancel at all — or whose server-side limit outlasts the caller's timeout. A client that drops its connection does not stop the query either: unless `client_connection_check_interval` is set (its default, 0, disables the check), PostgreSQL detects the lost connection only when it next waits for, receives or sends data on the socket. Each datastore's limit has its own reach: PostgreSQL's `statement_timeout` applies to every statement, is measured from when the command arrives at the server, and is off by default (0), and PostgreSQL advises against setting it in `postgresql.conf` because it would affect all sessions, so it is set for the sessions that issue these queries; MySQL's `max_execution_time`, or the per-statement `MAX_EXECUTION_TIME` optimizer hint, applies only to read-only `SELECT` statements — and neither applies to a `SELECT` in a stored program — so writes need another bound; MongoDB's `maxTimeMS` terminates an operation that exceeds it, though only at one of the operation's designated interrupt points. SQL Server's `remote query timeout` governs only the remote queries SQL Server itself sends, not the queries it receives; its query timeout is set in the client driver, which cancels the query when the timeout expires — with that timeout at 0, the engine runs the query to completion however long it takes, short of a Resource Governor `REQUEST_MAX_CPU_TIME_SEC` limit, which caps the CPU time a request can use rather than how long it runs and, on SQL Server, aborts the request only when trace flag 2422 is enabled. This rule bounds the query itself: a caller waiting on it without bound is `BE-TIMEOUT-001`'s, an unbounded wait for the pooled connection an abandoned query keeps checked out is `performance-backend.md`'s `BE-POOL-002`, and a transaction or checked-out connection held across a call to another service is `BE-POOL-005`'s.

**Source:** PostgreSQL documentation — Client Connection Defaults (`statement_timeout`) and Connections and Authentication (`client_connection_check_interval`); MySQL 8.4 Reference Manual — Server System Variables (`max_execution_time`) and Optimizer Hints (Statement Execution Time Optimizer Hints); MongoDB Manual — `cursor.maxTimeMS()`; Microsoft Learn — "Server configuration: remote query timeout", "Troubleshoot query timeout errors", and "CREATE WORKLOAD GROUP (Transact-SQL)" (`REQUEST_MAX_CPU_TIME_SEC`).

## BE-RETRY — Retries and dead-lettering

### BE-RETRY-001

> Automatic retries MUST be bounded by a maximum attempt count, spaced with exponential backoff plus jitter, and limited to idempotent operations (see `api-backend.md` `BE-IDEMP`).

**Verification:** Any spec or plan that introduces automatic retries MUST state the attempt bound, the backoff-with-jitter policy, and the idempotency basis for the retried operation, with the attempt bound and backoff parameters as named constants or environment variables per `configuration-cross.md` `CFG-CONST-003`; `/{project}:analyze` flags a retry plan missing a bound, jitter, or an idempotency basis. This rule governs the retry mechanics: an at-least-once consumer that does not handle a duplicate delivery is `concurrency-backend.md`'s `BE-COORD-002`.

**Source:** Marc Brooker, "Exponential Backoff And Jitter", AWS Architecture Blog (2015); Google SRE book, ch. 22 "Addressing Cascading Failures" (limit retries per request; randomized exponential backoff); RFC 9110 §9.2.2 (idempotent methods and automatic retry).

### BE-RETRY-002

> Retries SHOULD be governed by a retry budget (e.g. a cap on retries as a fraction of requests) and coordinated with circuit breaking.

**Verification:** Any spec or plan that introduces retries against a shared downstream SHOULD state its retry budget, as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`, and how it composes with circuit breaking; `/{project}:analyze` flags a retry plan with no aggregate cap.

**Source:** Google SRE book, ch. 21 "Handling Overload" (per-request and per-client retry budgets) and ch. 22 "Addressing Cascading Failures" (server-wide retry budget).

### BE-RETRY-003

> Automatic retries SHOULD be limited to failures classified as transient — timeouts, connection failures, and overload or unavailability responses such as `429` and `503` — and SHOULD wait at least the delay a server-supplied `Retry-After` names before the next attempt; permanent errors, malformed requests, and an overload response the downstream marks as not worth retrying SHOULD NOT be retried.

**Verification:** Any spec or plan that introduces automatic retries SHOULD name which failures it retries and which it surfaces immediately, and state that a `Retry-After` (or the downstream's equivalent throttling hint) sets the minimum delay before the next attempt; `/{project}:analyze` flags a retry plan that retries every failure indiscriminately, ignores a downstream's throttling signal, or retries an overload the downstream marks as not worth retrying. This rule decides which failures earn a retry and when: the attempt bound and backoff are `BE-RETRY-001`'s, a rate-limited response sent without `429` and `Retry-After` is `security-backend.md`'s `BE-API-004`, and a status code used against its registered semantics is `api-backend.md`'s `BE-STATUS-001`.

**Source:** Google SRE book, ch. 22 "Addressing Cascading Failures" (do not retry permanent errors or malformed requests) and ch. 21 "Handling Overload" ("overloaded; don't retry" error); RFC 9110 §10.2.3 (`Retry-After`); RFC 6585 §4 (`429 Too Many Requests`, which may carry `Retry-After`).

### BE-RETRY-004

> Retries for a failing dependency SHOULD be issued at one layer of the request path only — the layer immediately above the one that failed — and SHOULD NOT be repeated by the layers above it, which receive the exhausted failure as one they do not retry.

**Verification:** Any spec or plan that adds retries on a call path whose callers or dependencies also retry SHOULD name the one layer that retries and how a retry-exhausted failure is signalled upward so it is not retried again; `/{project}:analyze` flags a plan that stacks retries at more than one layer of the same call path.

**Source:** Google SRE book, ch. 21 "Handling Overload" (retry only at the layer immediately above the rejecting one) and ch. 22 "Addressing Cascading Failures" (avoid amplifying retries across levels; the 64-attempt example).

### BE-RETRY-005

> A message whose processing still fails when its redelivery bound is exhausted MUST be moved to a dead-letter destination that retains it long enough to be acted on, where the team responsible for the consumer can review and redrive it — or, where strict ordering forbids skipping it, MUST halt its stream and record the halt, with the message that caused it, where that team can review it — and MUST NOT be redelivered indefinitely or discarded silently.

**Verification:** Any spec or plan that consumes from a queue, stream, or message broker MUST state the redelivery bound, as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`, the dead-letter destination and its retention, long enough to be acted on (or, for a strictly ordered stream, the halt disposition and where the halt is recorded), and how the responsible team reviews and redrives what lands there; `/{project}:analyze` flags a consumer plan that names no redelivery bound or no disposition for retry-exhausted messages, or a disposition that discards the message or halts without recording the halt and the message that caused it. **Not flagged:** a plan that states no monitoring or alerting on the dead-letter destination or the halt record, since how the team watches them is its own operational choice. This rule governs the disposition of a retry-exhausted message: a redelivery that is not duplicate-safe is `concurrency-backend.md`'s `BE-COORD-002`, and a halting consumer's unstated or over-wide ordering scope is `BE-COORD-004`'s.

**Source:** Amazon SQS Developer Guide — "Using dead-letter queues in Amazon SQS" (`maxReceiveCount` redrive policy, FIFO ordering, retention period); Hohpe & Woolf, *Enterprise Integration Patterns* — Dead Letter Channel.

## BE-BREAKER — Circuit breakers

### BE-BREAKER-001

> Calls to a failure-prone downstream SHOULD be guarded by a circuit breaker that opens on a failure threshold (a count or rate of failures), fails fast (or falls back) while open, and half-opens to probe recovery.

**Verification:** Any spec or plan that calls a downstream with a meaningful failure rate SHOULD state whether a circuit breaker guards it and the open/half-open policy, with its thresholds as named constants or environment variables per `configuration-cross.md` `CFG-CONST-003`; `/{project}:analyze` flags an integration with a failure-prone downstream that commits to no breaker or fallback.

**Source:** Nygard, *Release It!* — Circuit Breaker; Martin Fowler, "CircuitBreaker" (bliki).

## BE-DRAIN — Graceful shutdown

### BE-DRAIN-001

> On receiving a termination signal, a service MUST stop accepting new work, finish or safely hand off in-flight work within a bounded grace period, and only then exit. The grace period, together with any delay that runs after the platform's forced-kill countdown starts (such as a pre-stop hook or an in-app wait after the signal), MUST fit inside the forced-kill deadline.

**Verification:** Any spec or plan for a deployable service MUST commit to the Statement's graceful-shutdown sequence and name the grace period, as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`, which together with the delays the Statement names fits inside the platform's forced-kill deadline; `/{project}:analyze` flags a service plan that does not address shutdown draining, or whose grace period, together with the delays the Statement names, can outlast the platform's forced-kill deadline. **Not flagged:** intake kept open for `BE-DRAIN-002`'s routing-propagation delay before it stops.

**Source:** Kubernetes documentation — Pod Lifecycle, Termination of Pods and Container Lifecycle Hooks.

### BE-DRAIN-002

> At the start of shutdown, the instance SHOULD leave routing — through the orchestrator's endpoint removal, a failing readiness signal (see `observability-backend.md` `BE-HEALTH-001`), or explicit deregistration — and SHOULD keep accepting requests for a bounded propagation delay before it stops intake.

**Verification:** Any spec or plan that defines graceful shutdown SHOULD commit to each obligation in the Statement, with the propagation delay as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`; `/{project}:analyze` flags a shutdown sequence that closes intake as soon as the termination signal arrives, with no step that first removes the instance from routing. A propagation delay that runs after the forced-kill countdown starts and, with the grace period, outlasts the forced-kill deadline is `BE-DRAIN-001`'s.

**Source:** Kubernetes documentation — Pod Lifecycle, Termination of Pods (terminating endpoints) and Container Lifecycle Hooks (`preStop`); AWS Prescriptive Guidance — "Configure container lifecycle hooks" (Amazon EKS); Amazon EKS Best Practices Guide — Load Balancing, "Gracefully handle the client requests".

## BE-BULK — Backpressure, bulkheads, and graceful degradation

### BE-BULK-001

> Failure-prone or resource-heavy work SHOULD be isolated behind a bulkhead — bounded concurrency or a bounded queue — that sheds load (rejects or degrades) when saturated rather than queueing unboundedly.

**Verification:** Any spec or plan that introduces a failure-prone or resource-heavy dependency SHOULD commit to each obligation in the Statement, with the bulkhead's bounds as named constants or environment variables per `configuration-cross.md` `CFG-CONST-003`; `/{project}:analyze` flags a plan that introduces such a dependency with no stated bound or saturation behavior, or that shares one unbounded resource pool across independent workloads. This rule bounds a work queue and isolates workloads: slow work left on the request path is `performance-backend.md`'s `BE-ASYNC-001`; an unbounded connection pool or acquisition wait is `BE-POOL-001`'s or `BE-POOL-002`'s; a shed request not answered with an overload status is `api-backend.md`'s `BE-STATUS-001` (`503` for saturation) or, where the limit is a per-client rate, `security-backend.md`'s `BE-API-004` (`429`); and a bulkhead's pool or queue with no USE metrics is `observability-backend.md`'s `BE-METRIC-002`.

**Source:** Nygard, *Release It!*, 2nd ed. — Bulkheads, Shed Load, Create Back Pressure; RFC 9110 §15.6.4 (`503` for a temporary overload); RFC 6585 §4 (`429` for rate limiting).

### BE-BULK-002

> Each downstream dependency of an operation SHOULD be classified as critical or non-critical to it, and the failure of a non-critical dependency SHOULD degrade the response — omitting that part, or serving a stale or static default — rather than fail the whole operation; the degraded response SHOULD NOT come through a second distributed mechanism that can itself fail or add load during the outage. The degraded path SHOULD be tested, and a dependency critical to none of the service's operations SHOULD NOT gate its readiness signal (`observability-backend.md` `BE-HEALTH-001`).

**Verification:** Any spec or plan that adds a downstream dependency to an operation SHOULD state whether the operation can complete without it and, if it can, the degraded behavior when that dependency fails; `/{project}:analyze` flags a plan that fails the whole operation on the loss of a dependency the plan itself treats as optional, that states no criticality for an operation's dependencies, that degrades through a second distributed mechanism or leaves the degraded path untested, or that lets a dependency critical to none of the service's operations gate the readiness signal of `observability-backend.md` `BE-HEALTH-001`. A circuit breaker's fallback while open (`BE-BREAKER-001`) that serves a non-critical dependency's degraded response is held to the degraded-response conditions above; one that denies the request on a security decision, as `security-backend.md` `BE-ERR-005` requires, is not a degraded response.

**Source:** AWS Well-Architected Framework, Reliability Pillar — REL05-BP01 "Implement graceful degradation to transform applicable hard dependencies into soft dependencies"; Jacob Gabrielson, "Avoiding fallback in distributed systems", Amazon Builders' Library (2019); Google SRE book, ch. 22 "Addressing Cascading Failures" (graceful degradation).
