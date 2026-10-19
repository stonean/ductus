# Reliability Rules — Backend

Enforceable reliability rules for server-side resilience under partial failure — timeouts and deadlines, retries and retry-exhausted messages, circuit breakers, graceful shutdown, backpressure, and graceful degradation. These rules apply to projects adopting `ductus` whose surface includes a backend. They land the deadlines, downstream-call timeouts, retries, and circuit breakers that `performance-backend.md` deliberately deferred.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `BE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `TIMEOUT` (timeouts and deadlines), `RETRY` (retries and dead-lettering), `BREAKER` (circuit breakers), `DRAIN` (graceful shutdown), `BULK` (backpressure, bulkheads, and graceful degradation). See `specs/008-security-rules/data-model.md` for the full schema.

Reliability rules default to **SHOULD** where the approach is contextual. A rule is **MUST** only when its absence is an availability or cascading-failure risk regardless of scale — an unbounded downstream wait, a retry storm, a poison message redelivered forever or silently dropped, a deploy that drops in-flight work. These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts.

Projects without a backend need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger. In `.ductus/config.toml`, a `[rules] surfaces` list without `backend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## BE-TIMEOUT — Timeouts and deadlines

### BE-TIMEOUT-001

> Every outbound network call, downstream-service call, or blocking wait on another process or host MUST be made under a bounded timeout — no call waits unboundedly for a response.

**Rationale:** An unbounded wait on a slow or hung dependency holds the calling thread, connection, or goroutine indefinitely; under any load that exhausts the pool and the failure cascades back to every caller — independent of scale, since a single hung dependency suffices. Many clients default to no bound — gRPC sets no deadline unless one is given, and Python `requests` does not time out unless a timeout is set — so the bound has to be stated rather than inherited. Pool-acquisition timeouts are governed by `performance-backend.md` `BE-POOL-002`, and the server-side limit that stops a datastore query by `BE-TIMEOUT-003`; this rule bounds the caller's wait on the call itself. Local file I/O is out of scope: a per-call timeout is generally unavailable for it.

**Verification:** Any spec or plan that introduces an outbound call or a blocking wait on another process or host MUST name the timeout that bounds it, as a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003` rather than a bare literal. `/{project}:analyze` flags plans that add a downstream or remote call with no stated timeout, an unnamed client-library default included.

**Source:** Nygard, *Release It!* — Timeouts; gRPC documentation — Deadlines (no deadline by default); Requests documentation — Timeouts.

### BE-TIMEOUT-002

> A request that fans out to downstream calls SHOULD carry an end-to-end deadline that is propagated to those calls and shrinks as time is consumed, so total work respects the caller's budget.

**Rationale:** Independent per-call timeouts can sum to far more than the client will wait. A propagated deadline caps the whole request and lets downstreams abandon work the caller has already given up on, freeing resources sooner. Propagating it as a remaining timeout, with the elapsed time already deducted — as gRPC does — also sidesteps clock skew between hosts.

**Verification:** Any spec or plan that fans out to multiple downstream calls SHOULD name the request deadline and how it is propagated; the deadline is an operator-tunable value, a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags fan-out plans that bound each call but not the overall request.

**Source:** Google SRE book, ch. 22 "Addressing Cascading Failures" — deadline propagation; gRPC documentation — Deadlines (deadline propagation).

### BE-TIMEOUT-003

> A database or other datastore query issued on a request path or by a worker SHOULD be bounded on the server — by the datastore's own time limit for a statement or operation, set no longer than the caller's timeout or remaining deadline — or, where the datastore has no such limit for that kind of statement, by a client timeout whose expiry cancels the query on the server rather than only ending the caller's wait.

**Rationale:** `BE-TIMEOUT-001` bounds how long the caller waits; it does not bound the query. A client-side timeout ends the wait, and the query stops only if the driver also cancels it on the server — and an application-level timer that merely stops waiting sends no cancel at all. Even a cancel is not certain: PostgreSQL takes it on a separate connection, it "might or might not have any effect", and the client has no direct way to tell whether it succeeded. A client that drops its connection does not stop the query either: unless `client_connection_check_interval` is set (its default, 0, disables the check), PostgreSQL detects the lost connection only when it next waits for, receives or sends data on the socket. Left running, the abandoned query keeps its database connection busy, holds the locks it has taken — PostgreSQL normally holds a row or table lock until the end of the transaction — and consumes the database's CPU and I/O, while a retry of the timed-out call (`BE-RETRY-001`) starts a second copy beside it; where its pooled connection is still checked out, later requests wait for one until `performance-backend.md` `BE-POOL-002`'s acquisition timeout fails them. A server-side limit ends the query whether or not the client is still there. Each datastore's limit has its own reach: PostgreSQL's `statement_timeout` applies to every statement, is measured from when the command arrives at the server, and is off by default (0), and PostgreSQL advises against setting it in `postgresql.conf` because it would affect all sessions, so it is set for the sessions that issue these queries; MySQL's `max_execution_time`, or the per-statement `MAX_EXECUTION_TIME` optimizer hint, applies only to read-only `SELECT` statements — and neither applies to a `SELECT` in a stored program — so writes need another bound; MongoDB's `maxTimeMS` terminates an operation that exceeds it, though only at one of the operation's designated interrupt points. SQL Server's `remote query timeout` governs only the remote queries SQL Server itself sends, not the queries it receives; its query timeout is set in the client driver, which cancels the query when the timeout expires — with that timeout at 0, the engine runs the query to completion however long it takes, short of a Resource Governor `REQUEST_MAX_CPU_TIME_SEC` limit, which caps the CPU time a request can use rather than how long it runs and, on SQL Server, aborts the request only when trace flag 2422 is enabled. It is SHOULD, not MUST: the scale-independent cascade — a caller waiting without bound — is already `BE-TIMEOUT-001`'s MUST, while what an abandoned query costs depends on its weight and on whether the driver already cancels it. Holding a transaction or a checked-out connection across a call to another service is `performance-backend.md` `BE-POOL-005`; this rule bounds the query itself.

**Verification:** Any spec or plan that issues datastore queries on a request path or from a worker SHOULD name the server-side time limit those queries run under and show it no longer than the caller's timeout (`BE-TIMEOUT-001`) or remaining deadline (`BE-TIMEOUT-002`), or, for a query no server-side limit covers — a MySQL write, which `max_execution_time` does not reach, for one — state that the client timeout cancels the query on the server; the limit is an operator-tunable value, a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags data-access plans whose only bound on a query is a client-side wait, or whose server-side limit outlasts the caller's timeout.

**Source:** PostgreSQL documentation — Client Connection Defaults (`statement_timeout`), Connections and Authentication (`client_connection_check_interval`), Message Flow, "Canceling Requests in Progress", and Explicit Locking; MySQL 8.4 Reference Manual — Server System Variables (`max_execution_time`) and Optimizer Hints (Statement Execution Time Optimizer Hints); MongoDB Manual — `cursor.maxTimeMS()`; Microsoft Learn — "Server configuration: remote query timeout", "Troubleshoot query timeout errors", and "CREATE WORKLOAD GROUP (Transact-SQL)" (`REQUEST_MAX_CPU_TIME_SEC`).

## BE-RETRY — Retries and dead-lettering

### BE-RETRY-001

> Automatic retries MUST be bounded by a maximum attempt count, spaced with exponential backoff plus jitter, and limited to idempotent operations (see `api-backend.md` `BE-IDEMP`).

**Rationale:** Unbounded or fixed-interval retries synchronize across callers and amplify a transient downstream blip into a self-sustaining retry storm that prevents recovery; backoff with jitter de-correlates and bounds the added load, and retrying a non-idempotent operation double-applies it. The storm is scale-independent — even a small fleet retrying in lockstep overwhelms a single recovering dependency — which is why it is MUST. This rule owns the retry mechanics (bound, backoff, jitter); the delivery-side duplicate-handling obligation for at-least-once consumers is `concurrency-backend.md` `BE-COORD-002`.

**Verification:** Any spec or plan that introduces automatic retries MUST state the attempt bound, the backoff-with-jitter policy, and the idempotency basis for the retried operation; the attempt bound and backoff parameters are operator-tunable values, named constants or environment variables per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags retry plans missing a bound, jitter, or an idempotency basis.

**Source:** Marc Brooker, "Exponential Backoff And Jitter", AWS Architecture Blog (2015); Google SRE book, ch. 22 "Addressing Cascading Failures" (limit retries per request; randomized exponential backoff); RFC 9110 §9.2.2 (idempotent methods and automatic retry).

### BE-RETRY-002

> Retries SHOULD be governed by a retry budget (e.g. a cap on retries as a fraction of requests) and coordinated with circuit breaking, so a degraded downstream is not hammered by aggregate retries.

**Rationale:** Per-call backoff bounds one caller's retries but not the system-wide aggregate; a budget caps total retry amplification and, paired with a breaker, stops retrying a dependency that is already known to be down.

**Verification:** Any spec or plan that introduces retries against a shared downstream SHOULD state its retry budget and how it composes with circuit breaking; the budget is an operator-tunable value, a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags retry plans with no aggregate cap.

**Source:** Google SRE book, ch. 21 "Handling Overload" (per-request and per-client retry budgets) and ch. 22 "Addressing Cascading Failures" (server-wide retry budget).

### BE-RETRY-003

> Automatic retries SHOULD be limited to failures classified as transient — timeouts, connection failures, and overload or unavailability responses such as `429` and `503` — and SHOULD wait at least the delay a server-supplied `Retry-After` names before the next attempt; permanent errors and malformed requests SHOULD NOT be retried.

**Rationale:** A permanent failure — a validation error, a `400`, a `401` or `403` — can never succeed on retry, so every attempt is pure added load that also delays the error the caller needs. A `Retry-After` on a `429` or `503` is the downstream stating when it can take the request again; retrying sooner spends the attempt bound against a dependency that has already refused. A downstream can also mark an overload as not worth retrying at all — the SRE book describes an "overloaded; don't retry" error, distinct from the "task overloaded" error that does trigger retries — and that signal is surfaced, not retried. `BE-RETRY-001` bounds and spaces retries; this rule decides which failures earn one and honors the wait the server asked for. Sending `429` with `Retry-After` when rate limiting is `security-backend.md` `BE-API-004`, and status semantics are `api-backend.md` `BE-STATUS-001`.

**Verification:** Any spec or plan that introduces automatic retries SHOULD name which failures it retries and which it surfaces immediately, and state that a `Retry-After` (or the downstream's equivalent throttling hint) sets the minimum delay before the next attempt. `/{project}:analyze` flags retry plans that retry every failure indiscriminately or that ignore a downstream's throttling signal.

**Source:** Google SRE book, ch. 22 "Addressing Cascading Failures" (do not retry permanent errors or malformed requests) and ch. 21 "Handling Overload" ("overloaded; don't retry" error); RFC 9110 §10.2.3 (`Retry-After`); RFC 6585 §4 (`429 Too Many Requests`, which may carry `Retry-After`).

### BE-RETRY-004

> Retries for a failing dependency SHOULD be issued at one layer of the request path only — the layer immediately above the one that failed — and SHOULD NOT be repeated by the layers above it, which receive the exhausted failure as one they do not retry.

**Rationale:** Attempts multiply across layers: one request at the top reaches the lowest layer as many times as the product of the attempt counts at each layer in between — the SRE book's example is a backend, frontend, and JavaScript layer each issuing three retries (four attempts), so a single user action becomes 64 attempts on an already-overloaded database. A per-call bound (`BE-RETRY-001`) and a per-process budget (`BE-RETRY-002`) each cap one layer, not the product; assigning retries to a single layer is what keeps the total bounded by one layer's attempt count.

**Verification:** Any spec or plan that adds retries on a call path whose callers or dependencies also retry SHOULD name the one layer that retries and how a retry-exhausted failure is signalled upward so it is not retried again. `/{project}:analyze` flags plans that stack retries at more than one layer of the same call path.

**Source:** Google SRE book, ch. 21 "Handling Overload" (retry only at the layer immediately above the rejecting one) and ch. 22 "Addressing Cascading Failures" (avoid amplifying retries across levels; the 64-attempt example).

### BE-RETRY-005

> A message whose processing still fails when its redelivery bound is exhausted MUST be moved to a retained dead-letter destination where the team responsible for the consumer can review and redrive it — or, where strict ordering forbids skipping it, MUST halt its stream and record the halt, with the message that caused it, where that team can review it — and MUST NOT be redelivered indefinitely or discarded silently.

**Rationale:** A poison message — one that fails on every delivery — redelivered without limit blocks every message behind it on an ordered queue or partition and burns consumer capacity on an unordered one; a single such message is enough, which is why it is MUST. Discarding it instead silently drops the work it carried, the same loss `BE-DRAIN-001` prevents at shutdown. A dead-letter destination keeps the message for diagnosis and redrive while the queue keeps moving, provided it is retained long enough to be acted on: in an Amazon SQS standard queue a dead-lettered message keeps its original enqueue timestamp, so SQS advises giving the dead-letter queue a longer retention period than its source. Where order is load-bearing, moving one message aside breaks the sequence — Amazon SQS advises against a dead-letter queue on a FIFO queue whose order matters — so halting is the safe disposition there, and recording the halt with the message that caused it is what lets the team find and clear it rather than the stream stalling unexplained; the ordering scope a consumer relies on is `concurrency-backend.md` `BE-COORD-004`. How the team watches the dead-letter destination or the halt record is its own operational choice; this rule requires only that what it needs to review and redrive is kept where it can. Duplicate-safe redelivery is `concurrency-backend.md` `BE-COORD-002`.

**Verification:** Any spec or plan that consumes from a queue, stream, or message broker MUST state the redelivery bound, the dead-letter destination and its retention (or, for a strictly ordered stream, the halt disposition and where the halt is recorded), and how the responsible team reviews and redrives what lands there; the redelivery bound is a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags consumer plans that name no redelivery bound or no disposition for retry-exhausted messages, and flags a disposition that discards the message or halts without recording the halt and the message that caused it.

**Source:** Amazon SQS Developer Guide — "Using dead-letter queues in Amazon SQS" (`maxReceiveCount` redrive policy, FIFO ordering, retention period); Hohpe & Woolf, *Enterprise Integration Patterns* — Dead Letter Channel.

## BE-BREAKER — Circuit breakers

### BE-BREAKER-001

> Calls to a failure-prone downstream SHOULD be guarded by a circuit breaker that opens on a failure threshold (a count or rate of failures), fails fast (or falls back) while open, and half-opens to probe recovery.

**Rationale:** Without a breaker, every caller keeps trying a dead dependency — tying up timeouts, threads, and retries and slowing the whole service. Failing fast while open sheds that load and gives the dependency room to recover.

**Verification:** Any spec or plan that calls a downstream with a meaningful failure rate SHOULD state whether a circuit breaker guards it and the open/half-open policy; its thresholds are operator-tunable values, named constants or environment variables per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags integrations with failure-prone downstreams that commit to no breaker or fallback.

**Source:** Nygard, *Release It!* — Circuit Breaker; Martin Fowler, "CircuitBreaker" (bliki).

## BE-DRAIN — Graceful shutdown

### BE-DRAIN-001

> On receiving a termination signal, a service MUST stop accepting new work, finish or safely hand off in-flight work within a bounded grace period, and only then exit.

**Rationale:** Exiting immediately on SIGTERM drops in-flight requests and abandons partial work on every deploy or scale-down — a recurring, scale-independent availability hit, since a single in-flight request is enough to be lost. A bounded grace period prevents both dropped work and unbounded shutdown hangs. Orchestrators enforce their own bound — Kubernetes starts a `terminationGracePeriodSeconds` countdown (30 seconds by default), runs any `preStop` hook, sends the container's stop signal (SIGTERM unless the image sets another), and sends SIGKILL to whatever is still running when the countdown expires — so a drain that outlasts that deadline is cut off mid-request just as surely as no drain. Stopping intake need not be instantaneous: `BE-DRAIN-002` keeps it open for a routing-propagation delay first.

**Verification:** Any spec or plan for a deployable service MUST commit to a graceful-shutdown sequence — stop intake, drain in-flight work within a bounded grace period, then exit — and name the grace period, which together with any pre-stop delay fits inside the platform's forced-kill deadline; the grace period is a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags service plans that do not address shutdown draining, or whose drain can outlast the platform's kill deadline.

**Source:** Kubernetes documentation — Pod Lifecycle, Termination of Pods (stop signal, `terminationGracePeriodSeconds` default 30 seconds, then SIGKILL) and Container Lifecycle Hooks (the grace-period countdown begins before the `preStop` hook runs).

### BE-DRAIN-002

> At the start of shutdown, the instance SHOULD leave routing — through the orchestrator's endpoint removal, a failing readiness signal (see `observability-backend.md` `BE-HEALTH-001`), or explicit deregistration — and SHOULD keep accepting requests for a bounded propagation delay before it stops intake, so traffic stops arriving before the listener closes.

**Rationale:** Routing changes reach load balancers asynchronously. In Kubernetes, the kubelet and the EndpointSlice controller learn of a Pod's termination at the same time, so the stop signal — absent a `preStop` hook — can arrive before kube-proxy, ingress controllers, or cloud load balancers stop sending the Pod new requests; an instance that closes its listener on the signal refuses those requests, re-introducing the dropped-request problem the drain was meant to prevent. Outside an orchestrator, failing readiness or deregistering is what removes the instance from rotation at all. A `preStop` delay, or an in-app wait after the signal, bridges the gap, and it counts against the grace period of `BE-DRAIN-001`.

**Verification:** Any spec or plan that defines graceful shutdown SHOULD state how the instance leaves routing and the propagation delay it waits before closing intake, counted within the grace period; the delay is an operator-tunable value, a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags shutdown sequences that close intake as soon as the termination signal arrives, with no step that first removes the instance from routing.

**Source:** Kubernetes documentation — Pod Lifecycle, Termination of Pods (terminating endpoints) and Container Lifecycle Hooks (`preStop`); AWS Prescriptive Guidance — "Configure container lifecycle hooks" (Amazon EKS); Amazon EKS Best Practices Guide — Load Balancing, "Gracefully handle the client requests" (load-balancer deregistration lags the termination signal).

## BE-BULK — Backpressure, bulkheads, and graceful degradation

### BE-BULK-001

> Failure-prone or resource-heavy work SHOULD be isolated behind a bulkhead — bounded concurrency or a bounded queue — that sheds load (rejects or degrades) when saturated rather than queueing unboundedly.

**Rationale:** A single slow dependency without isolation consumes all threads and memory and takes the whole service down (cascading failure); a bulkhead confines the blast radius to that dependency, and shedding load when saturated preserves partial availability instead of collapsing under an unbounded backlog. Offloading slow work is `performance-backend.md` `BE-ASYNC-001` and bounding connection pools is `BE-POOL-001` / `BE-POOL-002`; neither bounds a work queue or isolates one workload from another, which is what this rule adds. A shed request is answered with an overload status the caller can act on, per `api-backend.md` `BE-STATUS-001` — `503` for saturation, or `429` where the limit is a per-client rate (`security-backend.md` `BE-API-004`) — and saturation is observable through the USE metrics of `observability-backend.md` `BE-METRIC-002`.

**Verification:** Any spec or plan that introduces a failure-prone or resource-heavy dependency SHOULD state its bulkhead (bounded concurrency or queue) and its load-shedding behavior at saturation; the bounds are named constants or environment variables per `configuration-cross.md` `CFG-CONST-003`. `/{project}:analyze` flags plans that introduce such a dependency with no stated bound or saturation behavior, or that share one unbounded resource pool across independent workloads.

**Source:** Nygard, *Release It!*, 2nd ed. — Bulkheads, Shed Load, Create Back Pressure; RFC 9110 §15.6.4 (`503` for a temporary overload); RFC 6585 §4 (`429` for rate limiting).

### BE-BULK-002

> Each downstream dependency of an operation SHOULD be classified as critical or non-critical to it, and the failure of a non-critical dependency SHOULD degrade the response — omitting that part, or serving a stale or static default — rather than fail the whole operation.

**Rationale:** A dependency the operation could do without, left as a hard dependency, turns its outage into an outage of everything that calls it — the landing page that errors because the recommendations service is down. Degrading confines the failure to the feature that needs it. The degraded path should be simpler than the primary one (omit, cached value, static default) rather than a second distributed mechanism that can itself fail or add load during the outage, and it needs a test, since otherwise it first runs during an incident. The classification also scopes readiness: only critical dependencies should gate the readiness signal of `observability-backend.md` `BE-HEALTH-001`, or a soft failure takes the instance out of rotation. A breaker's fallback while open (`BE-BREAKER-001`) is one way to deliver the degraded response.

**Verification:** Any spec or plan that adds a downstream dependency to an operation SHOULD state whether the operation can complete without it and, if it can, the degraded behavior when that dependency fails. `/{project}:analyze` flags plans that fail the whole operation on the loss of a dependency the plan itself treats as optional, or that state no criticality for an operation's dependencies.

**Source:** AWS Well-Architected Framework, Reliability Pillar — REL05-BP01 "Implement graceful degradation to transform applicable hard dependencies into soft dependencies"; Jacob Gabrielson, "Avoiding fallback in distributed systems", Amazon Builders' Library (2019); Google SRE book, ch. 22 "Addressing Cascading Failures" (graceful degradation).
