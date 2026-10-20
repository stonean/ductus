# Concurrency Rules — Backend

Enforceable concurrency-correctness rules for server-side shared state, locking, transactions, and distributed coordination. These rules apply to projects adopting `ductus` whose surface includes a backend.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `BE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `RACE` (shared-state races), `LOCK` (locking and deadlock avoidance), `TXN` (transaction isolation), `COORD` (distributed coordination). See `specs/008-security-rules/data-model.md` for the full schema.

Concurrency rules default to **SHOULD** where the right approach is contextual. A rule is **MUST** only when its absence is a correctness or corruption hazard regardless of scale — a defect two concurrent actors are enough to trigger. These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts.

Projects without a backend need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger. In `.ductus/config.toml`, a `[rules] surfaces` list without `backend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## BE-RACE — Shared-state races

### BE-RACE-001

> Shared mutable state reachable from more than one concurrent execution context MUST be protected by a synchronization mechanism — a lock, an atomic primitive, single-owner/actor confinement, or serialized access; unsynchronized concurrent access in which at least one access is a write is a data race.

**Rationale:** A data race corrupts state non-deterministically and stays invisible until a production interleaving exposes it. It is a correctness hazard regardless of scale — two concurrent contexts are enough — which is why it is MUST. Synchronizing each access does not make a compound check-then-act atomic; that is `BE-RACE-003`.

**Verification:** Any spec or plan that introduces state shared across threads, goroutines, async tasks, or request handlers MUST name how that state is synchronized. `/{project}:analyze` flags plans that introduce concurrent access to shared mutable state without a stated synchronization mechanism.

**Source:** The Go Memory Model (a data race is a write concurrent with another read or write of the same location); Java Language Specification §17.4.5 (happens-before order and data races) and §17.4.1 (conflicting accesses).

### BE-RACE-002

> Designs SHOULD prefer eliminating shared mutable state — immutability, confinement to a single owner, or message-passing — over guarding it, to shrink the surface that can race.

**Rationale:** Every guarded shared variable is a future race waiting for one missed lock. Removing the sharing removes the hazard class outright, rather than relying on perpetual discipline. SHOULD, because confinement is not always the simpler design — Effective Go itself notes that a reference count may be best done by putting a mutex around an integer.

**Verification:** Any spec or plan that introduces shared mutable state SHOULD justify why it is shared rather than confined or immutable. `/{project}:analyze` flags plans that add shared mutable state where confinement or immutability would serve.

**Source:** Effective Go, *Concurrency* — Share by communicating ("Do not communicate by sharing memory; instead, share memory by communicating").

### BE-RACE-003

> A check-then-act sequence on shared state — test-then-set, lazy initialization, create-if-absent, check-then-insert — MUST execute atomically, under one lock held across both the check and the act or as a single atomic operation (compare-and-set, put-if-absent, a conditional write); a uniqueness or existence rule on persisted data MUST be enforced by the data store itself (a unique or foreign-key constraint, an insert-on-conflict, a conditional put), not by a read that precedes the write. A check-then-act whose act is harmless to repeat — two callers computing and storing the same value — MAY leave the check and the act non-atomic, each access still synchronized per `BE-RACE-001`.

**Rationale:** Between the check and the act another actor can invalidate what was observed — two requests both see a username free and both insert it, two workers both find a job unclaimed and both run it, two processes both find a file absent and both create it. Synchronizing each step separately satisfies `BE-RACE-001` and still leaves this gap open, and an ordinary read at the default isolation level of PostgreSQL, MySQL/InnoDB, SQL Server, or Oracle locks no row that does not yet exist, so wrapping a check-then-insert in a transaction does not close it either. A data-store mechanism closes it: PostgreSQL documents that `INSERT … ON CONFLICT DO UPDATE` guarantees an atomic insert-or-update outcome even under high concurrency, and a constraint holds against every writer, whereas PostgreSQL states its serializable guarantee only for concurrent Serializable transactions. Two concurrent actors are enough, which is why it is MUST. Where repeating the act is harmless — a cache entry built twice with the same value — the race costs work, not correctness, so it MAY stay non-atomic; concurrent misses on an expensive cache entry are `performance-backend.md` `BE-CACHE-003`'s stampede concern. A read-modify-write of an existing persisted record is `BE-TXN-002`; an invariant over a set of rows is `BE-TXN-003`.

**Verification:** Any spec or plan that acts on the result of a check against shared state — creating something if absent, initializing on first use, enforcing a uniqueness or existence rule — MUST name the mechanism that makes the check and the act atomic, or state that repeating the act is harmless, and for a uniqueness or existence rule on persisted data MUST name the constraint or conditional write that enforces it. `/{project}:analyze` flags plans that describe a check followed by a separate act with no atomicity mechanism and no statement that repeating the act is harmless, and plans that enforce uniqueness with a lookup before the insert.

**Source:** CWE-367, Time-of-check Time-of-use (TOCTOU) Race Condition (its shared resources include "variables in multithreaded programs"); Goetz et al., *Java Concurrency in Practice* (2006), ch. 2 "Thread Safety", 2.2 "Atomicity"; PostgreSQL documentation, *INSERT* — `ON CONFLICT`.

## BE-LOCK — Locking and deadlock avoidance

### BE-LOCK-001

> A spec or plan that introduces a contended resource SHOULD state its concurrency-control strategy — optimistic (version / compare-and-set) or pessimistic (locking) — with the rationale for the choice.

**Rationale:** Optimistic and pessimistic control fail in opposite directions: optimistic control lets every party proceed and rolls back the loser of a conflict, which grows costly as contention rises, while pessimistic control makes contenders wait on locks, which costs throughput even when conflicts are rare. SQL Server's documentation frames the choice the same way — pessimistic where contention is high and locking costs less than rollback, optimistic where contention is low and an occasional rollback costs less than locking. Choosing implicitly hides the trade-off and the operational behavior under load.

**Verification:** Any spec or plan that introduces a contended resource SHOULD name and justify its optimistic-vs-pessimistic choice. `/{project}:analyze` flags contention-bearing plans that commit to neither.

**Source:** Microsoft, SQL Server *Transaction locking and row versioning guide* — pessimistic and optimistic concurrency control.

### BE-LOCK-002

> When more than one lock is held at once, locks SHOULD be acquired in a single consistent global order, each acquisition SHOULD wait no longer than a bounded lock timeout, and each lock SHOULD be released within a bounded hold time, to avoid deadlock and unbounded waits.

**Rationale:** Inconsistent acquisition order deadlocks reliably once two code paths take the same locks in opposite orders, and PostgreSQL calls a consistent acquisition order generally the best defense against deadlocks, a practice SQL Server and MySQL also recommend; an acquisition timeout turns a deadlock or a stuck holder into a recoverable error instead of an indefinite stall — PostgreSQL documents that, short of a detected deadlock, a transaction waiting for a lock waits indefinitely; and an unbounded hold time serializes throughput and stacks waiters behind a slow critical section. The lock timeout and the hold bound are operator-tunable values, each a named constant or environment variable per `configuration-cross.md` `CFG-CONST-003`.

**Verification:** Any spec or plan that acquires more than one lock SHOULD state the global acquisition order, the acquisition timeout, and the hold-time bound. `/{project}:analyze` flags multi-lock plans with no stated ordering, timeout, or hold bound.

**Source:** PostgreSQL documentation, *Explicit Locking* — Deadlocks ("acquire locks on multiple objects in a consistent order"); Microsoft, SQL Server *Deadlocks guide* ("Access objects in the same order"); MySQL documentation, InnoDB deadlocks ("use the same order of operations").

## BE-TXN — Transaction isolation

### BE-TXN-001

> Each transaction SHOULD state its isolation level explicitly rather than relying on the engine's default, which varies by database and is often weaker than assumed.

**Rationale:** The default isolation level — READ COMMITTED in PostgreSQL, SQL Server, and Oracle, REPEATABLE READ in MySQL/InnoDB — silently determines which anomalies are possible, and it is deployment configuration as much as engine choice: SQL Server's READ COMMITTED uses locking by default but row versioning when `READ_COMMITTED_SNAPSHOT` is on, which is the default on Azure SQL Database but not on Azure SQL Managed Instance. The same level name also carries different guarantees across engines — PostgreSQL's REPEATABLE READ aborts a transaction that would overwrite a concurrent update, while MySQL/InnoDB's permits the lost update. An implicit default is an unstated correctness assumption that breaks when the database or its configuration changes.

**Verification:** Any spec or plan that introduces a multi-statement transaction SHOULD name the isolation level it requires and the anomalies that level prevents on the chosen engine. `/{project}:analyze` flags transactional plans that do not state an isolation level.

**Source:** Berenson et al., "A Critique of ANSI SQL Isolation Levels" (SIGMOD 1995) — the ANSI SQL-92 levels and the anomalies each permits; PostgreSQL *Transaction Isolation*, MySQL *Transaction Isolation Levels*, SQL Server *SET TRANSACTION ISOLATION LEVEL* and *Transaction locking and row versioning guide*, and Oracle *Database Concepts* — Data Concurrency and Consistency documentation; Jepsen, *MySQL 8.0.34* (lost update under REPEATABLE READ).

### BE-TXN-002

> A concurrent read-modify-write of persisted state MUST prevent lost updates — via an optimistic version/ETag check, a `SELECT … FOR UPDATE` row lock, an atomic in-place or conditional write (`UPDATE … SET n = n + 1`, compare-and-set), or an isolation level that detects the conflict and aborts one writer; a bare read-then-write under concurrency is silent data corruption.

**Rationale:** Two transactions that read, then write, the same record without a guard silently discard one update — no error is raised, and the loss is scale-independent (two concurrent writers suffice). This is the canonical lost-update anomaly. An isolation level is a guard only on an engine where that level detects the conflict — PostgreSQL's REPEATABLE READ and SERIALIZABLE do; MySQL/InnoDB's REPEATABLE READ does not — and the abort it raises needs a handling path (`BE-TXN-004`), so the plan names the guard rather than assuming one. A read-modify-write spanning two HTTP requests — a client's read, then its write — is outside any one transaction; that window is `api-backend.md` `BE-COND-001`.

**Verification:** Any spec or plan that introduces a concurrent update to shared persisted state MUST name its lost-update guard (version check, row lock, atomic update, or a conflict-detecting isolation level on the named engine). `/{project}:analyze` flags read-modify-write plans that commit to none.

**Source:** Berenson et al., "A Critique of ANSI SQL Isolation Levels" (SIGMOD 1995) — lost update (P4); PostgreSQL documentation, *Transaction Isolation* ("could not serialize access due to concurrent update"); Jepsen, *MySQL 8.0.34* ("MySQL Repeatable Read transactions cannot safely read a value and then write it").

### BE-TXN-003

> A transaction that reads a set of rows, decides on the basis of that read, and then writes — enforcing an invariant across rows such as "at least one on call", "no overlapping bookings", or "total within a limit" — MUST guard against write skew and phantoms: by running every transaction that reads or writes the invariant's rows at an isolation level that is serializable on the chosen engine, by locking every row the decision read (materializing a lock row when the decision rests on the absence of rows), or by a database constraint that enforces the invariant; snapshot isolation, including PostgreSQL's REPEATABLE READ, does not prevent write skew.

**Rationale:** Two transactions that each read the same set, each see the invariant hold, and each write a different row both commit — and together break the invariant, with no error raised. A lost-update guard on the row being written (`BE-TXN-002`) does not help, because the two transactions write different rows, and how far a locking read reaches into rows that do not yet exist varies by engine — PostgreSQL's `SELECT … FOR UPDATE` locks the rows it retrieves, while InnoDB's locking reads at REPEATABLE READ can also lock the index range scanned against inserts — so a decision resting on absent rows names a materialized lock row, a constraint, or serializable isolation. Isolation-level names do not transfer between engines: PostgreSQL implements its REPEATABLE READ as snapshot isolation and documents it as permitting serialization anomalies that its SERIALIZABLE level prevents, while Oracle documents its SERIALIZABLE as reading a transaction-start snapshot and raising `ORA-08177` when a transaction updates a row another transaction changed and committed after it began — the snapshot-isolation behavior under which Fekete et al. observed non-serializable executions on Oracle. The commitment is therefore to serializable behavior on the chosen engine, not to the keyword — and from every transaction that touches the invariant, since PostgreSQL states its guarantee for a set of concurrent Serializable transactions, and its own example of two transactions that each read what the other writes lets both commit if either runs at Repeatable Read. Two concurrent transactions suffice, which is why it is MUST. A uniqueness rule on a single key is `BE-RACE-003`.

**Verification:** Any spec or plan whose transaction enforces an invariant spanning more than one row, or conditions a write on the absence of matching rows, MUST name its write-skew guard — the serializable isolation level on the named engine for every transaction that reads or writes the invariant's rows, the rows locked (or the materialized lock row), or the enforcing constraint. `/{project}:analyze` flags multi-row-invariant plans that rely on a default or snapshot isolation level with no lock or constraint.

**Source:** Berenson et al., "A Critique of ANSI SQL Isolation Levels" (SIGMOD 1995) — phantom (P3), which snapshot isolation does not preclude, and write skew (A5B), which it permits; Fekete et al., "Making Snapshot Isolation Serializable" (ACM TODS 30(2), 2005) — Oracle's SERIALIZABLE as a form of snapshot isolation, and materializing a conflict or promoting a read with `SELECT FOR UPDATE` to prevent write skew; PostgreSQL documentation, *Transaction Isolation*; MySQL documentation, *Transaction Isolation Levels*; Oracle *Database Concepts* — Data Concurrency and Consistency.

### BE-TXN-004

> A transaction that can abort on a concurrency conflict — a serialization failure (SQLSTATE `40001`), a deadlock victim, or a failed optimistic version check — SHOULD treat that abort as an expected outcome with a stated handling path: retry the whole transaction from its start, re-reading its inputs, under a bounded retry policy, or report the conflict to the caller.

**Rationale:** Serializable and snapshot isolation, deadlock detection, and optimistic concurrency all resolve a conflict by failing one party, so the failure is part of the design rather than an exceptional error — PostgreSQL documents that applications using REPEATABLE READ or SERIALIZABLE must be prepared to retry transactions on serialization failure (SQLSTATE `40001`) and that deadlock failures (`40P01`) may also be retried. A plan that adopts one of these guards (`BE-LOCK-001`, `BE-TXN-002`, `BE-TXN-003`) with no handling path turns every conflict into a user-visible failure. The retry re-runs the complete transaction with fresh reads, because the conflict invalidated what the transaction read — PostgreSQL directs retrying "the complete transaction, including all logic that decides which SQL to issue and/or which values to use." Engines differ in what the conflict rolls back: PostgreSQL rolls back the transaction, SQL Server and InnoDB roll back a deadlock victim's whole transaction, and Oracle rolls back only the statement that hit a deadlock, leaving the transaction open with its earlier work, so the handling path rolls back explicitly before it retries rather than re-issuing the failed statement. The transaction body must carry no side effect outside the database, or that effect repeats on every attempt. The retry mechanics — attempt bound, backoff, jitter — are `reliability-backend.md` `BE-RETRY-001`. SHOULD, because the choice between retrying and reporting the conflict is contextual.

**Verification:** Any spec or plan that runs transactions at an isolation level that aborts on conflict on the chosen engine (PostgreSQL REPEATABLE READ or SERIALIZABLE, for example), relies on deadlock detection, or uses optimistic version checks SHOULD state how a conflict abort is handled — whole-transaction retry with its bound, or the conflict response returned to the caller. `/{project}:analyze` flags plans that adopt a conflict-aborting guard with no stated abort handling.

**Source:** PostgreSQL documentation, *Transaction Isolation* ("applications using this level must be prepared to retry transactions due to serialization failures") and *Serialization Failure Handling*; Microsoft, SQL Server *Deadlocks guide* (the victim's transaction is rolled back with error 1205); MySQL documentation, InnoDB deadlocks; Oracle *Database Concepts* — Data Concurrency and Consistency (statement-level rollback on deadlock).

## BE-COORD — Distributed coordination

### BE-COORD-001

> A distributed lock, lease, or leader election relied on for correctness — where two simultaneous holders would corrupt state — MUST carry a fencing token that the protected resource validates, rejecting stale holders; a lease or lock alone is insufficient, because a paused holder, a delayed request, or a clock that jumps can let a holder act after its lease has expired. A lock taken only for efficiency, where a second holder merely duplicates work, MAY omit fencing.

**Rationale:** Without fencing, a GC pause, a page fault, a network packet delayed in transit, or a clock jump lets a node act on a lease that has already expired and passed to another node, so both write — split-brain corruption. A monotonic fencing token enforced at the resource makes the resource reject the stale writer, which is why it is MUST; Kleppmann notes this requires the protected resource to take an active role in checking tokens — the client cannot enforce it alone. Kleppmann distinguishes locks taken for efficiency, where an occasional second holder only repeats work, from locks taken for correctness; only the latter need fencing, which keeps this MUST on the corruption case alone.

**Verification:** Any spec or plan that introduces a distributed lock or leader election MUST state whether it guards correctness or only efficiency, and for a correctness lock MUST commit to a monotonic fencing token enforced at the protected resource. `/{project}:analyze` flags distributed-lock plans that rely on lease expiry alone without stating that the lock guards efficiency only.

**Source:** Kleppmann, "How to do distributed locking" (fencing tokens; locks for efficiency vs. correctness).

### BE-COORD-002

> An operation invoked under at-least-once delivery, automatic retry, or a scheduler that can start it more than once for one scheduled time (a Kubernetes CronJob, an in-process scheduler running in every replica) or start a run while the previous run is still in progress MUST be idempotent or deduplication-guarded, with its delivery semantics stated — at-most-once, at-least-once, or exactly-once together with the boundary within which that guarantee holds.

**Rationale:** Retries and at-least-once brokers deliver duplicates by design; a non-idempotent handler double-applies them — a double charge, a double ship, a doubled balance — which is silent corruption regardless of scale. Schedulers duplicate too: Kubernetes documents that a CronJob may create two Jobs for one scheduled time, or none, and directs that Jobs be idempotent, and its default `concurrencyPolicy: Allow` lets a run start while the previous one is still running; an in-process scheduler fires once per replica. For a scheduled job, the deduplication is a run record claimed atomically per scheduled time (`BE-RACE-003`) or a per-item claim that leaves a duplicate or overlapping run nothing to redo; a lock held only while a run executes stops runs overlapping but not a second run for the same time after the first finishes. A broker's exactly-once guarantee has a boundary: Kafka's covers reading, processing, and writing data on Kafka topics, and its documentation states that exactly-once delivery to other destination systems generally requires their cooperation — storing the consumer's position with the output — so a database write, email, or payment call outside that boundary still sees duplicates, and an exactly-once claim that names no boundary does not discharge this rule. Where the effect is a database write, its deduplication record is committed in the same transaction; recorded separately, it is a dual write (`BE-COORD-003`). Stating the delivery semantics makes the duplicate-handling obligation explicit. For an HTTP operation the idempotency contract is `api-backend.md` `BE-IDEMP` (an `Idempotency-Key`, idempotent `PUT`/`DELETE`), which this rule builds on rather than restates; a queue consumer or scheduled job has no such header, so this rule owns its deduplication obligation, while the retry *mechanics* — attempt bounds, backoff, and jitter — are `reliability-backend.md` `BE-RETRY-001`, and a message that exhausts its redelivery bound is `BE-RETRY-005`'s concern.

**Verification:** Any spec or plan that consumes from a broker, exposes a retried operation, schedules a recurring job, or relies on at-least-once delivery MUST commit to idempotency or deduplication and state the delivery semantics. `/{project}:analyze` flags retried, queue-consuming, or scheduled-job plans with no idempotency or dedup commitment, and exactly-once claims that name no boundary.

**Source:** Kubernetes documentation, *CronJob* — CronJob limitations, Job creation ("the Jobs that you define should be idempotent"), and Concurrency policy; Apache Kafka documentation, *Design* — Message Delivery Semantics ("Exactly-once delivery for other destination systems generally requires cooperation with such systems").

### BE-COORD-003

> An operation that must both commit a database change and publish a message or event, or write to a second datastore that must stay consistent with the first, MUST make the two atomic — a transactional outbox written in the same database transaction and relayed afterwards, change-data capture from the database log, or a single source of truth that the other system consumes; it MUST NOT perform them as two independent writes.

**Rationale:** Two independent writes fail independently: a crash between them leaves the database changed with no event published, or an event published for a change that rolled back, and two concurrent writers whose writes reach the two systems in different orders leave them permanently disagreeing — with no error raised either way. Two concurrent writers, or a single crash at the wrong moment, are enough, which is why it is MUST. Two-phase commit across the database and the broker is often not an option — either might not support it — so an outbox or change-data capture makes the database transaction the single commit point. A relay can publish a message more than once, so its consumers still discharge `BE-COORD-002`.

**Verification:** Any spec or plan in which one operation both writes to a database and publishes to a broker, or writes to a second datastore or search index that must stay consistent with it, MUST name the mechanism that makes the two atomic. `/{project}:analyze` flags plans that commit a transaction and then publish, or publish and then commit, as separate steps with no outbox, change-data capture, or single source of truth.

**Source:** Richardson, microservices.io, "Pattern: Transactional outbox" ("2PC is not an option"; "The Message relay might publish a message more than once"); Kleppmann, "Using logs to build a solid data infrastructure (or: why dual writes are a bad idea)".

### BE-COORD-004

> A consumer whose correctness depends on message order MUST NOT assume ordering wider than its broker guarantees: it MUST state the ordering scope it relies on (a partition, key, or message group — not the whole topic or queue) and either route messages that must stay ordered into one scope by key, keep the producer from reordering them on retry, and process each scope serially, or detect and discard out-of-order messages with a per-entity sequence number or version check.

**Rationale:** Brokers order messages, where they order them at all, only within a partition, key, or message group — Kafka guarantees order within a topic-partition and writes events with the same key to the same partition, and Amazon SQS FIFO orders messages within a message group, not across groups — and even that order can be lost: RabbitMQ documents that with multiple consumers on a queue any redelivery can change order, Kafka that producer retries can reorder records within a partition when idempotence is disabled and more than one request is in flight, and a consumer that processes one scope's messages in parallel, or re-processes them after a crash, can apply an older message after a newer one. A handler that applies an older "status changed" or "balance set" message after a newer one silently overwrites newer state with older; two messages for one entity are enough, which is why it is MUST. The stale-message guard is a sequence number or version assigned per entity, not a wall-clock timestamp — clocks can jump forwards or back. Dead-lettering one message out of an ordered stream breaks the same sequence; `reliability-backend.md` `BE-RETRY-005` halts such a stream and records the halt instead.

**Verification:** Any spec or plan that introduces a message consumer whose outcome depends on message order MUST state the ordering scope it relies on and either the partitioning key, the producer setting that keeps a retry from reordering within a scope (for Kafka, idempotence enabled, its default, or one request in flight), and serial per-scope processing that preserve it, or the stale-message guard. `/{project}:analyze` flags order-dependent consumers that assume topic- or queue-wide ordering, or that name neither a partitioning key nor a sequence or version check.

**Source:** Apache Kafka documentation, *Introduction* (events with the same key go to the same partition; a partition's events are read in the order written) and producer configuration (`retries`); Amazon SQS Developer Guide, *FIFO queue delivery logic* (ordering per message group ID); RabbitMQ documentation, *Queues* guide (message ordering); Kleppmann, "How to do distributed locking" (clock jumps).
