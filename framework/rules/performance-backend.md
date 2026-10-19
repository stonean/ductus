# Performance Rules — Backend

Enforceable performance rules for server-side request handling, data access, caching, and resource management. These rules apply to projects adopting `ductus` whose surface includes a backend.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `BE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `QUERY` (query efficiency), `CACHE` (caching), `POOL` (connection pooling), `PAYLOAD` (payload budgets), `ASYNC` (async offloading). See `specs/008-security-rules/data-model.md` for the full schema.

Performance rules default to **SHOULD** — thresholds are context-dependent, so most are advisory. A rule is **MUST** only when its absence is a denial-of-service or resource-exhaustion risk regardless of scale. These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts. Server-side deadlines, downstream-call timeouts, retries, circuit breakers, and bulkheads are reliability concerns covered by `reliability-backend.md` (`BE-TIMEOUT`, `BE-RETRY`, `BE-BREAKER`, `BE-BULK`), not here; the one timeout this file owns is connection-pool acquisition (`BE-POOL-002`), which `BE-TIMEOUT-001` defers to.

Projects without a backend need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger. In `.ductus/config.toml`, a `[rules] surfaces` list without `backend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## BE-QUERY — Query Efficiency

### BE-QUERY-001

> A spec or plan that fetches data related to each item of a collection MUST commit to a batched access pattern (eager-load/join, a single `IN`/`ANY` query, or a dataloader) rather than one query per item. Per-item queries inside a loop over a collection MUST NOT be used.

**Rationale:** The N+1 query pattern turns one logical read into one extra query per record; over a user-controlled or growing collection the query count is unbounded, and because each query is its own round trip, latency and database load grow linearly with result size while the request holds its pooled connection for the whole sequence. Batching makes the query count independent of collection length.

**Verification:** Any spec or plan that renders, serializes, or aggregates a collection together with related data (orders with their line items, users with their roles) MUST state how the related data is fetched in bulk. `/{project}:analyze` flags collection-handling plans that describe a per-item lookup, a query inside a loop, or that omit the access-pattern commitment.

**Source:** Rails Guides, *Active Record Query Interface*, "N + 1 Queries Problem" section; GraphQL DataLoader (per-tick batching, per-request instances)

### BE-QUERY-002

> A spec or plan that introduces a query filtering, sorting, or joining on a column SHOULD name the index that supports it, or state that the access is covered by the primary key or a unique constraint.

**Rationale:** A filter on an unindexed column forces a full scan, and a sort that no index order satisfies forces an explicit sort operation over the matching rows — costs that grow with table size, fine in development and an outage in production once the table is large. Joins are the common blind spot: PostgreSQL, for one, does not create an index on a foreign key's referencing columns when the constraint is declared. Column order matters too: a composite (multicolumn) index is most efficient when the query constrains its leading columns, and in some databases it cannot serve a search on a later column alone. Naming the index at design time makes the access path reviewable before the slow query ships.

**Verification:** Any spec or plan that introduces a non-trivial query (a `WHERE`, `ORDER BY`, or `JOIN` on a non-primary-key column) SHOULD name the supporting index. `/{project}:analyze` emits a warning when a query design describes filtering or sorting on a column without naming an index or stating primary-key or unique-constraint coverage.

**Source:** Markus Winand, *Use The Index, Luke* — Concatenated Indexes, Indexing Order By; PostgreSQL documentation, Foreign Keys, Multicolumn Indexes

### BE-QUERY-003

> A read that can match an unbounded number of rows MUST bound how many it materializes at once. Endpoints that return collections to clients MUST paginate per `BE-PAGE` (api-backend.md); internal reads MUST cap the result set with a named maximum (per `CFG-CONST-003`) or iterate it in bounded batches (keyset pagination or a server-side streaming cursor).

**Rationale:** An unbounded read loads the entire matching set into memory — a single large tenant or an unfiltered query can exhaust application memory and saturate the database. An explicit limit bounds the worst case regardless of how much data exists; a job that legitimately touches every row keeps the same bound by iterating in batches rather than loading the whole set. Batching by `OFFSET` bounds memory but not work: PostgreSQL, for one, still computes every row an `OFFSET` skips, so each successive batch reads more than the last, which is why keyset iteration or a cursor is the batch form named here.

**Verification:** Any spec or plan that reads a collection MUST commit to a row bound — pagination for client-facing endpoints (deferring to `BE-PAGE`), or a named maximum or batch size for internal reads. `/{project}:analyze` flags collection reads with no limit, no pagination commitment, and no bounded-batch iteration.

**Source:** OWASP API Security Top 10 2023, API4:2023 Unrestricted Resource Consumption; Nygard, *Release It!* — Unbounded Result Sets; PostgreSQL documentation, LIMIT and OFFSET; `api-backend.md` §BE-PAGE

### BE-QUERY-004

> A spec or plan that writes many rows in one operation (bulk import, batch update, fan-out insert) SHOULD issue set-based or batched statements — a multi-row `INSERT`, a set-based `UPDATE`, a bulk-load API such as `COPY` — in chunks of a named maximum size, rather than one statement per row or a single unbounded statement or transaction.

**Rationale:** One statement per row pays a round trip and a parse for every row — and a commit too, when each row is committed separately, which PostgreSQL's bulk-loading guidance warns does a lot of work for each row added — so a bulk write's latency grows linearly with its size: the write-side twin of the N+1 read in `BE-QUERY-001`. On a live system the opposite extreme fails too: one unbounded statement or transaction holds every lock it takes until it ends (PostgreSQL, for one, normally holds a row or table lock until the transaction ends), blocking concurrent writers to those rows for its whole duration; while it stays open, PostgreSQL cannot vacuum away recently-dead row versions it might still see, contributing to table bloat; and a late failure rolls back all of its work. Bounded chunks cap both the round trips and the lock duration. First populating a database is the different case PostgreSQL's guidance addresses, recommending `COPY` or a single transaction there.

**Verification:** Any spec or plan that inserts, updates, or deletes a collection of rows SHOULD describe its batched write mechanism and name the chunk size as a named constant per `CFG-CONST-003`. `/{project}:analyze` emits a warning when a bulk-write design describes a per-row statement in a loop, or a single unbounded write with no chunking.

**Source:** PostgreSQL documentation, Populating a Database (Disable Autocommit, Use `COPY`), `INSERT` (multirow `VALUES`), Explicit Locking, Client Connection Defaults (`idle_in_transaction_session_timeout`)

## BE-CACHE — Caching

### BE-CACHE-001

> Every cache entry MUST have an explicit time-to-live or a documented invalidation trigger, and the cache MUST be size-bounded — a maximum entry count or memory limit, enforced by an eviction policy. Caches that grow without expiry, invalidation, or eviction MUST NOT be used.

**Rationale:** Expiry and a size bound cover different failures, so the rule needs both. A TTL bounds staleness but not memory — a cache keyed by an unbounded input space (user IDs, query strings) holds every distinct key written within one TTL, so its footprint grows with traffic and can exhaust memory — and a size bound caps memory but not staleness, so an entry nothing expires is served long after its source changes. An eviction policy with no capacity behind it bounds nothing: Redis, for one, sets no memory limit by default on 64-bit systems. A named TTL or invalidation rule plus an enforced size limit caps both.

**Verification:** Any spec or plan that introduces a cache MUST name its TTL and/or the event that invalidates an entry, and MUST bound its size (a maximum entry count or memory limit, and the eviction policy that enforces it). `/{project}:analyze` flags caching plans that omit the expiry/invalidation commitment, name an eviction policy without a capacity, or describe an unbounded in-memory map used as a cache.

**Source:** Redis documentation, Key eviction (`maxmemory`, `maxmemory-policy`); AWS, *Database Caching Strategies Using Redis* — Cache Validity

### BE-CACHE-002

> Cache keys SHOULD incorporate every input that affects the cached value, including the authorization scope (user, tenant, role) when the value is principal-specific.

**Rationale:** A key that omits an input that changes the value serves the wrong result — and when the omitted input is the principal, it serves one user's data to another. The scope enters the key as an identifier (a user or tenant ID), never as a session token, API key, or other secret, which OWASP's cache-key guidance keeps out of keys. The authorization requirement itself is `BE-AUTHZ-002` / `BE-AUTHZ-005` in security-backend.md, and `BE-AUTHZ-003` forbids caching permission state across requests outright; this rule is the caching-design reminder that keys must be scope-complete.

**Verification:** Any spec or plan that caches principal-specific or input-dependent data SHOULD enumerate the inputs composing the cache key and confirm the authorization scope is among them. `/{project}:analyze` emits a warning when a cache over per-user or per-tenant data describes a key that omits the scoping dimension.

**Source:** OWASP Web Cache Security Cheat Sheet — Design a Complete Cache Key; OWASP Authorization Cheat Sheet — Validate the Permissions on Every Request

### BE-CACHE-003

> Caches fronting expensive computation SHOULD protect against stampede (thundering herd): concurrent misses for the same key SHOULD be coalesced (single-flight, or a short lock or lease) or pre-empted by refreshing before expiry (probabilistic early expiration, stale-while-revalidate), and entries written together SHOULD carry jittered TTLs so they do not expire together.

**Rationale:** When a hot key expires, every concurrent request misses at once and recomputes the same expensive value simultaneously, spiking load on the very backend the cache exists to protect. Coalescing collapses the herd to a single recomputation, and early refresh avoids the miss altogether. The same spike happens across many keys when entries loaded in bulk (a warm-up, a batch refresh) share one fixed TTL and expire in the same instant; random jitter on the TTL spreads those expirations out.

**Verification:** Any spec or plan that caches the result of an expensive or slow operation (an aggregate query, a third-party call) SHOULD describe its stampede protection, and a cache populated in bulk SHOULD state its TTL jitter. `/{project}:analyze` emits a warning when a cache over expensive work omits any coalescing or early-refresh strategy, or when a bulk-populated cache uses a single fixed TTL.

**Source:** Vattani, Chierichetti & Lowenstein, *Optimal Probabilistic Cache Stampede Prevention* (PVLDB 8(8), 2015); Nishtala et al., *Scaling Memcache at Facebook* (NSDI 2013) — leases; AWS, *Database Caching Strategies Using Redis* — Cache Validity (TTL jitter); RFC 5861 §3 (`stale-while-revalidate`)

### BE-CACHE-004

> A response to a safe request (`GET`, `HEAD`) that `BE-API-001` (security-backend.md) does not already require to be `no-store` SHOULD declare its caching policy explicitly in `Cache-Control` — a freshness lifetime (`max-age`, or `s-maxage` for shared caches) or `no-cache` to require revalidation — and a cacheable response SHOULD carry a validator (`ETag` or `Last-Modified`) so a client can revalidate with a conditional request answered by `304 Not Modified`.

**Rationale:** A response with no explicit freshness can still be cached: RFC 9111 lets a cache assign it a heuristic lifetime when its status code is heuristically cacheable (a `200` included), so an unmarked response is reused for an unpredictable time — stale for some clients, never reused for others. Declaring the policy makes reuse deliberate, and a validator turns a repeat fetch of an unchanged resource into a body-less `304`, saving serialization and bandwidth. Sensitive and authenticated responses stay `no-store` per `BE-API-001`, and build-emitted frontend assets and the HTML documents that reference them follow `performance-frontend.md` `FE-LOAD-007`; this rule governs the rest.

**Verification:** Any spec or plan that introduces a `GET` endpoint SHOULD state each response's `Cache-Control` policy (or that it is `no-store` per `BE-API-001`) and, for a cacheable response, its validator and conditional-request (`304`) support. `/{project}:analyze` emits a warning when a `GET`-serving design states no caching policy, or states a cacheable policy with no validator.

**Source:** RFC 9111 §4.2.2 (heuristic freshness), §4.3 (validation), §5.2 (Cache-Control); RFC 9110 §8.8.2–8.8.3 (Last-Modified, ETag), §13.1.2 (If-None-Match), §15.1 (heuristically cacheable status codes), §15.4.5 (304 Not Modified)

## BE-POOL — Connection Pooling

### BE-POOL-001

> Connections to databases, caches, and external services MUST be obtained from a pool, or from a shared long-lived client that multiplexes requests over its connections, not opened per request. A pool MUST have a bounded maximum size, set by a named constant (per `CFG-CONST-003`) rather than a bare literal.

**Rationale:** Opening a connection per request repeats the setup — a socket, a handshake, authentication — on every call and, under load, opens more connections than the backend can accept, exhausting database connection slots and collapsing the service. A pool amortizes that setup, and a bounded one caps concurrent connections. The bound has to be set rather than assumed, because not every pool has one by default: Go's `database/sql`, for one, places no limit on open connections unless `SetMaxOpenConns` is called. A shared multiplexed client (an HTTP/2 or gRPC channel, a pipelining cache client) achieves the same reuse with fewer connections, which is why gRPC's guidance is to reuse channels rather than create one per call.

**Verification:** Any spec or plan that connects to a database or external service MUST commit to pooled or shared multiplexed connections and name the pool-size constant. `/{project}:analyze` flags data-access plans that describe per-request connection creation, a pool with no maximum size, or an unnamed/bare-literal pool size.

**Source:** Microsoft, *SQL Server Connection Pooling (ADO.NET)*; Go `database/sql` documentation (`DB.SetMaxOpenConns`); gRPC documentation, Performance Best Practices; `configuration-cross.md` §CFG-CONST

### BE-POOL-002

> Connection-pool acquisition MUST use a bounded wait timeout. A caller that cannot obtain a connection within the timeout MUST fail fast with an error rather than block indefinitely.

**Rationale:** An unbounded acquisition wait turns pool saturation into a pile-up: requests block holding threads, the thread pool exhausts, and the outage cascades. A bounded wait converts saturation into fast, sheddable errors that protect the rest of the system. Pool defaults differ — HikariCP's `connectionTimeout` defaults to 30 seconds and then throws, while Apache Commons DBCP's `maxWaitMillis` defaults to waiting indefinitely — so the bound has to be stated rather than inherited.

**Verification:** Any spec or plan that introduces a connection pool MUST name the acquisition-timeout value (a named constant per `CFG-CONST-003`) and the fail-fast behavior on timeout. `/{project}:analyze` flags pool configurations that omit an acquisition timeout or describe blocking until a connection is free.

**Source:** Nygard, *Release It!* — Blocked Threads, Fail Fast; HikariCP configuration (`connectionTimeout`); Apache Commons DBCP configuration (`maxWaitMillis`)

### BE-POOL-003

> Pooled connections MUST be returned to the pool on every code path, including error and early-return paths. Acquisition and release MUST be paired by a construct that runs on scope exit (context manager, `defer`, try-with-resources, RAII).

**Rationale:** A connection leaked on an error path is never reused; enough leaks drain the pool and the service stops serving while orphaned connections sit idle — Microsoft's pooling guidance warns that connections not explicitly closed might not be returned to the pool. Pairing release to scope exit via an unwinding construct makes the leak structurally impossible: each of the constructs named runs its release whichever path leaves the scope — normal completion, an early return, or an error.

**Verification:** Any spec or plan that acquires a pooled resource MUST commit to a scope-bound release mechanism that fires on error paths. `/{project}:analyze` flags resource-acquisition plans that describe manual release without an unwinding guarantee, or that release only on the success path.

**Source:** Microsoft, *SQL Server Connection Pooling (ADO.NET)*; Oracle, *The Java Tutorials* — The try-with-resources Statement; *Effective Go* — Defer; PEP 343 (the `with` statement); cppreference, RAII

### BE-POOL-004

> A connection pool's maximum size SHOULD be derived from the capacity of the backend it connects to rather than from expected request concurrency: the per-instance maximum multiplied by the largest number of instances that can run at once (autoscaling ceiling plus rolling-deploy overlap), plus the connections every other client of that backend holds, SHOULD stay within the backend's connection limit.

**Rationale:** Pools are sized per process, but the backend sees their sum. A pool sized in isolation is multiplied by every replica — and by old and new replicas running side by side during a rolling deploy — so a scale-out or a deploy can exhaust the database's connection slots (PostgreSQL's `max_connections`, for one) and refuse connections to every client at once. A larger pool is not faster either: HikariCP's pool-sizing guidance, taking the PostgreSQL project's `(core_count × 2) + effective_spindle_count` formula as a starting point, argues for a small pool saturated with threads waiting for connections, because a core runs one thread at a time, so connections beyond what the database's cores — and its waits on disk and network I/O — can keep busy add context-switching overhead rather than throughput.

**Verification:** Any spec or plan that introduces a connection pool, or changes a service's replica count or autoscaling bounds, SHOULD state the pool maximum, the maximum instance count, and the backend connection limit the total must stay within — or the connection proxy (PgBouncer, RDS Proxy) that multiplexes them. `/{project}:analyze` emits a warning when a pool size is stated without reference to the backend's connection limit and the instance count.

**Source:** HikariCP, *About Pool Sizing*; PostgreSQL documentation, Connection Settings (`max_connections`); PgBouncer, Features (pooling modes); AWS, *Amazon RDS Proxy* documentation

### BE-POOL-005

> A database transaction or a checked-out pooled connection SHOULD NOT be held open across a call to another service or any other wait whose duration this service does not control (an outbound HTTP or RPC call, a third-party API, a wait on user input); the call SHOULD be made before the transaction begins, or after it commits and the connection is released — through a transactional outbox per `BE-COORD-003` (concurrency-backend.md) when the call publishes an event or writes a second datastore that must stay consistent with the commit.

**Rationale:** A transaction occupies one database connection from its start to its commit — the span for which PgBouncer's transaction pooling mode assigns a server connection — so a downstream call made inside it holds that connection, and every lock the transaction has taken (PostgreSQL, for one, normally holds a row or table lock until the transaction ends), for the downstream's whole latency. Pool occupancy then tracks the slowest dependency rather than the database: a dependency that slows toward its timeout (`BE-TIMEOUT-001`, reliability-backend.md) drains a pool sized for short queries, and requests that never touch that dependency queue behind it for a connection (`BE-POOL-002`). While the transaction sits open and idle, PostgreSQL also cannot vacuum away recently-dead row versions it might still see, contributing to table bloat. Moving such a call after the commit as an independent second step is what `BE-COORD-003` forbids for an event or a second datastore; the outbox moves it after the commit without giving up atomicity.

**Verification:** Any spec or plan in which one request or job both runs database work and calls another service SHOULD state that the call is made outside any open transaction and without a checked-out connection, or name the outbox (`BE-COORD-003`) that defers it. `/{project}:analyze` emits a warning when a design places a downstream or third-party call between a transaction's begin and commit, or holds a pooled connection across one.

**Source:** PostgreSQL documentation, Explicit Locking, Client Connection Defaults (`idle_in_transaction_session_timeout`); PgBouncer, Features (transaction pooling); `concurrency-backend.md` §BE-COORD

## BE-PAYLOAD — Payload Budgets

### BE-PAYLOAD-001

> Response payloads MUST have a bounded maximum size. Endpoints that return collections able to exceed a small fixed maximum MUST paginate per `BE-PAGE` (api-backend.md) rather than return an unbounded list in a single response.

**Rationale:** An unbounded response serializes the entire result set into memory on the server and forces the client to receive it all — a large tenant or an unfiltered list exhausts server memory, saturates bandwidth, and can OOM the client. A size bound (via pagination or an explicit cap) makes the worst-case response independent of how much data exists.

**Verification:** Any spec or plan that returns a collection or a variable-size body MUST commit to a maximum response size or to pagination. `/{project}:analyze` flags response designs that return unbounded lists or omit a size/pagination bound. Pagination commitments defer to `BE-PAGE`.

**Source:** OWASP API Security Top 10 2023, API4:2023 Unrestricted Resource Consumption; `api-backend.md` §BE-PAGE

### BE-PAYLOAD-002

> Endpoints returning large or composite resources SHOULD support field selection (sparse fieldsets) so clients can request only the fields they need.

**Rationale:** Returning every field of a large resource on every request wastes serialization time, bandwidth, and client memory when the caller needs only a few. Field selection lets the cost track what the client actually uses.

**Verification:** Any spec or plan that returns a large or deeply-nested resource SHOULD describe a field-selection mechanism (a `fields=` parameter or field mask, a GraphQL selection set, a projection, or a view enumeration that selects among named response shapes). `/{project}:analyze` emits a warning when a large-resource endpoint offers no way to limit returned fields.

**Source:** Google AIP-157 (Partial responses); JSON:API v1.1, Sparse Fieldsets

### BE-PAYLOAD-003

> Large text or JSON responses SHOULD be compressed (gzip, brotli, or zstd) when the client advertises support via `Accept-Encoding`, and a response whose encoding is negotiated SHOULD carry `Vary: Accept-Encoding`. A response that reflects attacker-influenced input in the same body as a secret (a CSRF token, a session-bound value) SHOULD NOT be compressed unless the secret is masked per response. Already-compressed media types MAY be sent without further encoding.

**Rationale:** Text and JSON are highly redundant — MDN cites size reductions of up to 70% for some documents — so sending them uncompressed wastes bandwidth and adds latency for every client, especially on mobile networks. `Vary: Accept-Encoding` tells shared caches the stored body depends on that request header, so a compressed body is not replayed to a client that cannot decode it. Compression has one security exception: when a body carries both a secret and attacker-reflected input, its compressed length leaks the secret byte by byte (BREACH), so such responses stay uncompressed or mask the secret per response. Images, audio, and video are already compressed; a second pass usually gains nothing and can make the file larger.

**Verification:** Any spec or plan covering response handling or edge configuration that serves sizable text/JSON SHOULD commit to negotiated compression with `Vary: Accept-Encoding`, and, where a compressed response carries a secret beside reflected input, state how the secret is protected. `/{project}:analyze` emits a warning when a text/JSON-serving surface omits any compression commitment, or compresses without `Vary: Accept-Encoding`.

**Source:** RFC 9110 §8.4 (Content-Encoding), §12.5.3 (Accept-Encoding), §12.5.5 (Vary); Gluck, Harris & Prado, *BREACH: Reviving the CRIME Attack* (2013); MDN, *Compression in HTTP*

### BE-PAYLOAD-004

> Request or response bodies that can be large (file uploads and downloads, exports, bulk responses) SHOULD be streamed in bounded chunks rather than buffered whole in memory, so per-request memory is bounded by the chunk size rather than the body size.

**Rationale:** A handler that buffers a body before processing it holds the entire payload in memory, so memory use is body size multiplied by concurrent requests — a few concurrent large uploads or exports can exhaust the heap even when each is within its size limit. Streaming caps per-request memory at the buffer size and gets the first bytes to the client or to storage sooner. The size limits themselves are `BE-INPUT-006` (requests, security-backend.md) and `BE-PAYLOAD-001` (responses); this rule governs how a within-limit body is handled.

**Verification:** Any spec or plan that accepts or returns a body large relative to per-request memory (uploads, downloads, exports, generated reports) SHOULD state that the body is streamed — chunked transfer, multipart streaming, or a direct-to-object-storage upload — and name the buffer or chunk size as a named constant per `CFG-CONST-003`. `/{project}:analyze` emits a warning when a large-body design reads the whole body into memory before processing or writing it.

**Source:** OWASP API Security Top 10 2023, API4:2023 Unrestricted Resource Consumption (maximum allocable memory); `security-backend.md` §BE-INPUT-006

## BE-ASYNC — Async Offloading

### BE-ASYNC-001

> Work that is slow or unbounded — including a third-party call made only for its side effects — MUST NOT block the synchronous request path. It MUST be offloaded to a background job or queue, with the client receiving an asynchronous acknowledgment (e.g., `202 Accepted` per `BE-STATUS-001`) and, when the client needs the outcome, a way to observe completion. A third-party call whose result the response depends on MAY stay on the request path when it is bounded by a timeout per `BE-TIMEOUT-001` (reliability-backend.md).

**Rationale:** Holding a request thread while a slow or third-party operation runs ties up a worker and a connection for the whole duration; enough concurrent slow requests exhaust the worker pool and the service stops accepting new work. Offloading frees the request path to return immediately and lets the slow work proceed under its own concurrency limit. A third-party call the response cannot be formed without is the exception: offloading it only moves the wait into client polling, so it stays synchronous, bounded by a timeout and, for a failure-prone dependency, a circuit breaker (`BE-TIMEOUT-001`, `BE-BREAKER-001`). The status-monitor contract for observing offloaded work is `api-backend.md` `BE-LRO-001`.

**Verification:** Any spec or plan that introduces slow or unbounded processing (report generation, media transcoding, bulk import, a side-effect-only third-party call such as sending an email) MUST commit to offloading it to a background job and returning an async acknowledgment. `/{project}:analyze` flags request handlers that perform slow or unbounded work, or side-effect-only third-party calls, synchronously without an offload commitment.

**Source:** Azure Architecture Center, Asynchronous Request-Reply pattern; `api-backend.md` §BE-STATUS
