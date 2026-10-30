# Performance Rules — Backend

Performance rules for server-side request handling, data access, caching, and resource management. They verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts. A rule is **MUST** only when its absence is a denial-of-service or resource-exhaustion risk regardless of scale; the rest are **SHOULD**. Server-side deadlines, downstream-call timeouts, retries, circuit breakers, and bulkheads are `reliability-backend.md`'s (`BE-TIMEOUT`, `BE-RETRY`, `BE-BREAKER`, `BE-BULK`); the one timeout this file owns is connection-pool acquisition (`BE-POOL-002`), which `BE-TIMEOUT-001` defers to. Rule IDs follow the format `BE-{CATEGORY}-{NNN}`. Categories: `QUERY` (query efficiency), `CACHE` (caching), `POOL` (connection pooling), `PAYLOAD` (payload budgets), `ASYNC` (async offloading). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## BE-QUERY — Query Efficiency

### BE-QUERY-001

> A spec or plan that fetches data related to each item of a collection MUST commit to a batched access pattern (eager-load/join, a single `IN`/`ANY` query, or a dataloader) rather than one query per item. Per-item queries inside a loop over a collection MUST NOT be used.

**Verification:** Any spec or plan that renders, serializes, or aggregates a collection together with related data MUST state how the related data is fetched in bulk; `/{project}:analyze` flags one that describes a per-item lookup or a query inside a loop, or omits the access-pattern commitment.

**Source:** Rails Guides, *Active Record Query Interface*, "N + 1 Queries Problem" section; GraphQL DataLoader (per-tick batching, per-request instances)

### BE-QUERY-002

> A spec or plan that introduces a query filtering, sorting, or joining on a column SHOULD name the index that supports it, or state that the access is covered by the primary key or a unique constraint.

**Verification:** `/{project}:analyze` flags a spec or plan that introduces a `WHERE`, `ORDER BY`, or `JOIN` on a non-primary-key column without naming the supporting index or stating primary-key or unique-constraint coverage.

**Source:** Markus Winand, *Use The Index, Luke* — Concatenated Indexes, Indexing Order By; PostgreSQL documentation, Foreign Keys, Multicolumn Indexes

### BE-QUERY-003

> A read that can match an unbounded number of rows MUST bound how many it materializes at once. Endpoints that return collections to clients MUST paginate per `BE-PAGE` (api-backend.md); internal reads MUST cap the result set with a named maximum (per `CFG-CONST-003`) or iterate it in bounded batches (keyset pagination or a server-side streaming cursor).

**Verification:** Any spec or plan that reads a collection MUST commit to a row bound — pagination, whose shape is `BE-PAGE`'s, for a client-facing endpoint, or a named maximum or batch size for an internal read; `/{project}:analyze` flags a collection read with no limit, no pagination commitment, and no bounded-batch iteration. Batching by `OFFSET` is not one of the Statement's bounded-batch forms. **Not flagged:** a client-facing list endpoint's missing or malformed pagination, which `api-backend.md`'s `BE-PAGE` owns.

**Source:** OWASP API Security Top 10 2023, API4:2023 Unrestricted Resource Consumption; Nygard, *Release It!* — Unbounded Result Sets; PostgreSQL documentation, LIMIT and OFFSET; `api-backend.md` §BE-PAGE

### BE-QUERY-004

> A spec or plan that writes many rows in one operation SHOULD issue set-based or batched statements — a multi-row `INSERT`, a set-based `UPDATE`, a bulk-load API such as `COPY` — in chunks of a named maximum size, rather than one statement per row or a single unbounded statement or transaction. A write that first populates a database, rather than writing to a live one, MAY run as one statement or transaction.

**Verification:** Any spec or plan that inserts, updates, or deletes a collection of rows SHOULD describe its batched write mechanism and name the chunk size as a named constant per `CFG-CONST-003`; `/{project}:analyze` flags one that describes a per-row statement in a loop, or a single unbounded write with no chunking. **Not flagged:** an initial population the Statement's MAY covers.

**Source:** PostgreSQL documentation, Populating a Database (Disable Autocommit, Use `COPY`), `INSERT` (multirow `VALUES`)

## BE-CACHE — Caching

### BE-CACHE-001

> Every cache entry MUST have an explicit time-to-live or a documented invalidation trigger, and the cache MUST be size-bounded — a maximum entry count or memory limit, enforced by an eviction policy.

**Verification:** Any spec or plan that introduces a cache MUST commit to each obligation in the Statement; `/{project}:analyze` flags one that omits the expiry or invalidation commitment, names an eviction policy without a capacity, or describes an unbounded in-memory map used as a cache.

**Source:** Redis documentation, Key eviction (`maxmemory`, `maxmemory-policy`); AWS, *Database Caching Strategies Using Redis* — Cache Validity

### BE-CACHE-002

> Cache keys SHOULD incorporate every input that affects the cached value, including the authorization scope (user, tenant, role) when the value is principal-specific, entered as an identifier such as a user or tenant ID and never as a session token, API key, or other secret.

**Verification:** Any spec or plan that caches principal-specific or input-dependent data SHOULD enumerate the inputs composing the cache key and confirm the authorization scope is among them; `/{project}:analyze` flags a cache over per-user or per-tenant data whose key omits the scoping dimension or enters it as a secret. This rule governs the key: serving a cached value without a server-side, object-level authorization check falls under `security-backend.md`'s `BE-AUTHZ-002` and `BE-AUTHZ-005`, and caching permission state across requests under `BE-AUTHZ-003`.

**Source:** OWASP Web Cache Security Cheat Sheet — Design a Complete Cache Key; OWASP Authorization Cheat Sheet — Validate the Permissions on Every Request

### BE-CACHE-003

> Caches fronting expensive computation SHOULD protect against stampede (thundering herd): concurrent misses for the same key SHOULD be coalesced (single-flight, or a short lock or lease) or pre-empted by refreshing before expiry (probabilistic early expiration, stale-while-revalidate), and entries written together SHOULD carry jittered TTLs.

**Verification:** Any spec or plan that caches the result of an expensive or slow operation SHOULD describe its stampede protection, and a cache populated in bulk SHOULD state its TTL jitter; `/{project}:analyze` flags a cache over expensive work that omits any coalescing or early-refresh strategy, or a bulk-populated cache with a single fixed TTL.

**Source:** Vattani, Chierichetti & Lowenstein, *Optimal Probabilistic Cache Stampede Prevention* (PVLDB 8(8), 2015); Nishtala et al., *Scaling Memcache at Facebook* (NSDI 2013) — leases; AWS, *Database Caching Strategies Using Redis* — Cache Validity (TTL jitter); RFC 5861 §3 (`stale-while-revalidate`)

### BE-CACHE-004

> A response to a safe request (`GET`, `HEAD`) that `BE-API-001` (security-backend.md) does not already require to be `no-store` SHOULD declare its caching policy explicitly in `Cache-Control` — a freshness lifetime (`max-age`, or `s-maxage` for shared caches) or `no-cache` to require revalidation — and a cacheable response SHOULD carry a validator (`ETag` or `Last-Modified`) so a client can revalidate with a conditional request answered by `304 Not Modified`.

**Verification:** Any spec or plan that introduces a `GET` endpoint SHOULD state each response's `Cache-Control` policy (or that it is `no-store` per `BE-API-001`) and, for a cacheable response, its validator and conditional-request (`304`) support; `/{project}:analyze` flags a `GET`-serving design that states no caching policy, or a cacheable policy with no validator. **Not flagged:** an authenticated or sensitive-data response, which `security-backend.md` `BE-API-001` requires to be `no-store` and owns; and, where `performance-frontend.md` is also loaded, the cache policy of build-emitted frontend static assets and of the HTML documents that reference them, which its `FE-LOAD-007` owns.

**Source:** RFC 9111 §4.3 (validation), §5.2 (Cache-Control); RFC 9110 §8.8.2–8.8.3 (Last-Modified, ETag), §13.1.2 (If-None-Match), §15.4.5 (304 Not Modified)

## BE-POOL — Connection Pooling

### BE-POOL-001

> Connections to databases, caches, and external services MUST be obtained from a pool, or from a shared long-lived client that multiplexes requests over its connections, not opened per request. A pool MUST have a bounded maximum size, set by a named constant (per `CFG-CONST-003`) rather than a bare literal.

**Verification:** Any spec or plan that connects to a database, cache, or external service MUST commit to each obligation in the Statement; `/{project}:analyze` flags one that describes per-request connection creation or a pool with no maximum size. **Not flagged:** a pool size written as a bare literal, which `configuration-cross.md` `CFG-CONST-003` owns.

**Source:** Microsoft, *SQL Server Connection Pooling (ADO.NET)*; gRPC documentation, Performance Best Practices; `configuration-cross.md` §CFG-CONST

### BE-POOL-002

> Connection-pool acquisition MUST use a bounded wait timeout, set explicitly rather than inherited from the pool's default. A caller that cannot obtain a connection within the timeout MUST fail fast with an error rather than block indefinitely.

**Verification:** Any spec or plan that introduces a connection pool MUST name the acquisition-timeout value, as a named constant per `CFG-CONST-003`, and the fail-fast behavior on timeout; `/{project}:analyze` flags a pool configuration that omits an acquisition timeout or describes blocking until a connection is free.

**Source:** Nygard, *Release It!* — Blocked Threads, Fail Fast

### BE-POOL-003

> Pooled connections MUST be returned to the pool on every code path, including error and early-return paths. Acquisition and release MUST be paired by a construct that runs on scope exit (context manager, `defer`, try-with-resources, RAII).

**Verification:** Any spec or plan that acquires a pooled resource MUST commit to a scope-bound release mechanism that fires on error paths; `/{project}:analyze` flags one that describes manual release without an unwinding guarantee, or release only on the success path.

**Source:** Microsoft, *SQL Server Connection Pooling (ADO.NET)*; Oracle, *The Java Tutorials* — The try-with-resources Statement; *Effective Go* — Defer; PEP 343 (the `with` statement); cppreference, RAII

### BE-POOL-004

> A connection pool's maximum size SHOULD be derived from the capacity of the backend it connects to rather than from expected request concurrency: the most connections to that backend one instance can hold at once, across all its pools and processes, multiplied by the largest number of instances that can run at once (autoscaling ceiling plus deploy overlap), plus the most connections every other client of that backend can hold at once, SHOULD stay within the connections the backend admits for the service and those clients, net of any slots it reserves for administrative roles. A connection proxy that multiplexes client connections onto its own pool of backend connections, such as PgBouncer or RDS Proxy, MAY stand in, in that sum, for the pools that connect through it, counted as the most backend connections the proxy can hold at once across all its instances and pools.

**Verification:** Any spec or plan that introduces a connection pool or changes a term of the Statement's sum or its bound — a pool's or a proxy's size, an instance's processes, the replica count, autoscaling bounds or deploy overlap, or the backend's connection limit or instance size — SHOULD state the connections the backend admits for the service and the backend's other clients, net of slots it reserves for administrative roles, the most connections every other client of the backend can hold at once, and, for each of the service's pools to that backend, either the most connections it holds per instance, across all of an instance's processes, with the largest number of instances that can run at once, deploy overlap included, or, for a pool that connects through a connection proxy such as PgBouncer or RDS Proxy, the most backend connections the proxy can hold at once across all its instances and pools; `/{project}:analyze` flags a pool size stated without reference to the backend's connection limit and either the instance count or the most backend connections such a proxy can hold.

**Source:** HikariCP, *About Pool Sizing*; PostgreSQL documentation, Connection Settings (`max_connections`); PgBouncer, Features (pooling modes); AWS, *Amazon RDS Proxy* documentation

### BE-POOL-005

> A database transaction or a checked-out pooled connection SHOULD NOT be held open across a call to another service or any other wait whose duration this service does not control (an outbound HTTP or RPC call, a third-party API, a wait on user input); the call SHOULD be made before the transaction begins, or after it commits and the connection is released — through a transactional outbox per `BE-COORD-003` (concurrency-backend.md) when the call publishes an event or writes a second datastore that must stay consistent with the commit.

**Verification:** Any spec or plan in which one request or job both runs database work and calls another service SHOULD state that the call is made outside any open transaction and without a checked-out connection, or name the `BE-COORD-003` outbox that defers it; `/{project}:analyze` flags a design that places a downstream or third-party call between a transaction's begin and commit, or holds a pooled connection across one. This rule governs connection and lock occupancy, not atomicity: publishing an event or writing a second datastore as an independent step after the commit is `concurrency-backend.md`'s `BE-COORD-003`, and the call's own missing timeout is `reliability-backend.md`'s `BE-TIMEOUT-001`.

**Source:** `concurrency-backend.md` §BE-COORD

## BE-PAYLOAD — Payload Budgets

### BE-PAYLOAD-001

> Response payloads MUST have a bounded maximum size. Endpoints that return collections able to exceed a small fixed maximum MUST paginate per `BE-PAGE` (api-backend.md) rather than return an unbounded list in a single response.

**Verification:** Any spec or plan that returns a collection or a variable-size body MUST commit to a maximum response size or to pagination, whose shape is `BE-PAGE`'s; `/{project}:analyze` flags a response design that returns an unbounded list or omits a size or pagination bound. **Not flagged:** a client-facing list endpoint's missing or malformed pagination, which `api-backend.md`'s `BE-PAGE` owns.

**Source:** OWASP API Security Top 10 2023, API4:2023 Unrestricted Resource Consumption; `api-backend.md` §BE-PAGE

### BE-PAYLOAD-002

> Endpoints returning large or composite resources SHOULD support field selection (sparse fieldsets).

**Verification:** Any spec or plan that returns a large or deeply-nested resource SHOULD describe a field-selection mechanism (a `fields=` parameter or field mask, a GraphQL selection set, a projection, or a view enumeration that selects among named response shapes); `/{project}:analyze` flags a large-resource endpoint that offers no way to limit returned fields.

**Source:** Google AIP-157 (Partial responses); JSON:API v1.1, Sparse Fieldsets

### BE-PAYLOAD-003

> Large text or JSON responses SHOULD be compressed (gzip, brotli, or zstd) when the client advertises support via `Accept-Encoding`, and a response whose encoding is negotiated SHOULD carry `Vary: Accept-Encoding`. A response that reflects attacker-influenced input in the same body as a secret (a CSRF token, a session-bound value) SHOULD NOT be compressed unless the secret is masked per response. Already-compressed media types MAY be sent without further encoding.

**Verification:** Any spec or plan covering response handling or edge configuration that serves sizable text or JSON SHOULD commit to negotiated compression with `Vary: Accept-Encoding`; `/{project}:analyze` flags one that omits any compression commitment, compresses without `Vary: Accept-Encoding`, or compresses a response carrying a secret beside reflected input without stating how the secret is protected.

**Source:** RFC 9110 §8.4 (Content-Encoding), §12.5.3 (Accept-Encoding), §12.5.5 (Vary); Gluck, Harris & Prado, *BREACH: Reviving the CRIME Attack* (2013); MDN, *Compression in HTTP*

### BE-PAYLOAD-004

> Request or response bodies that can be large SHOULD be streamed in bounded chunks rather than buffered whole in memory.

**Verification:** Any spec or plan that accepts or returns a body large relative to per-request memory, such as an upload, download, export, or generated report, SHOULD state that the body is streamed — chunked transfer, multipart streaming, or a direct-to-object-storage upload — and name the buffer or chunk size as a named constant per `CFG-CONST-003`; `/{project}:analyze` flags a large-body design that reads the whole body into memory before processing or writing it. This rule governs how a within-limit body is handled: a body with no size limit is `security-backend.md`'s `BE-INPUT-006` for a request and `BE-PAYLOAD-001`'s for a response.

**Source:** OWASP API Security Top 10 2023, API4:2023 Unrestricted Resource Consumption (maximum allocable memory); `security-backend.md` §BE-INPUT-006

## BE-ASYNC — Async Offloading

### BE-ASYNC-001

> Work that is slow or unbounded — including a third-party call made only for its side effects — MUST NOT block the synchronous request path. It MUST be offloaded to a background job or queue; when the request's own outcome waits on that job, the client MUST receive an asynchronous acknowledgment (e.g., `202 Accepted` per `BE-STATUS-001`) and, when it needs the outcome, a way to observe completion. A third-party call whose result the response depends on MAY stay on the request path when it is bounded by a timeout per `BE-TIMEOUT-001` (reliability-backend.md).

**Verification:** Any spec or plan that introduces slow or unbounded processing, such as report generation, media transcoding, or bulk import, or a side-effect-only third-party call, such as sending an email, MUST commit to each obligation in the Statement; `/{project}:analyze` flags a request handler that performs either synchronously without an offload commitment, or that offloads work whose outcome the client needs with no way to observe completion. **Not flagged:** a response that keeps the status `BE-STATUS-001` gives it, such as `201` for a created account, when only a side effect is offloaded. This rule governs where the work runs and that a client needing the outcome can observe it: a status monitor lacking the states, result, failure envelope or `Retry-After` that `api-backend.md`'s `BE-LRO-001` names is that rule's, and a synchronous call to a failure-prone dependency with no circuit breaker is `reliability-backend.md`'s `BE-BREAKER-001`.

**Source:** Azure Architecture Center, Asynchronous Request-Reply pattern; `api-backend.md` §BE-STATUS
