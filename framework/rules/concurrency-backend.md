# Concurrency Rules — Backend

Concurrency-correctness rules for server-side shared state, locking, transactions, and distributed coordination, for projects whose surface includes a backend. They verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts. A rule is **MUST** only when its absence is a correctness or corruption hazard regardless of scale — a defect two concurrent actors are enough to trigger; the rest are **SHOULD**. Rule IDs follow the format `BE-{CATEGORY}-{NNN}`. Categories: `RACE` (shared-state races), `LOCK` (locking and deadlock avoidance), `TXN` (transaction isolation), `COORD` (distributed coordination). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## BE-RACE — Shared-state races

### BE-RACE-001

> Shared mutable state reachable from more than one concurrent execution context MUST be protected by a synchronization mechanism — a lock, an atomic primitive, single-owner/actor confinement, or serialized access; unsynchronized concurrent access in which at least one access is a write is a data race.

**Verification:** Any spec or plan that introduces state shared across threads, goroutines, async tasks, or request handlers MUST name how that state is synchronized. `/{project}:analyze` flags plans that introduce concurrent access to shared mutable state without a stated synchronization mechanism. This rule governs each access: a compound check-then-act left non-atomic, though each access is synchronized, is `BE-RACE-003`'s.

**Source:** The Go Memory Model (a data race is a write concurrent with another read or write of the same location); Java Language Specification §17.4.5 (happens-before order and data races) and §17.4.1 (conflicting accesses).

### BE-RACE-002

> Designs SHOULD prefer eliminating shared mutable state — immutability, confinement to a single owner, or message-passing — over guarding it, to shrink the surface that can race.

**Verification:** Any spec or plan that introduces shared mutable state SHOULD justify why it is shared rather than confined or immutable. `/{project}:analyze` flags plans that add shared mutable state where confinement or immutability would serve.

**Source:** Effective Go, *Concurrency* — Share by communicating ("Do not communicate by sharing memory; instead, share memory by communicating").

### BE-RACE-003

> A check-then-act sequence on shared state — test-then-set, lazy initialization, create-if-absent, check-then-insert — MUST execute atomically, under one lock held across both the check and the act or as a single atomic operation (compare-and-set, put-if-absent, a conditional write); a uniqueness or existence rule on persisted data MUST be enforced by the data store itself (a unique or foreign-key constraint, an insert-on-conflict, a conditional put), not by a read that precedes the write. A check-then-act whose act is harmless to repeat — two callers computing and storing the same value — MAY leave the check and the act non-atomic, each access still synchronized per `BE-RACE-001`.

**Verification:** Any spec or plan that acts on the result of a check against shared state — creating something if absent, initializing on first use, enforcing a uniqueness or existence rule — MUST name the mechanism that makes the check and the act atomic, or state that repeating the act is harmless, and for a uniqueness or existence rule on persisted data MUST name the constraint or conditional write that enforces it. `/{project}:analyze` flags plans that describe a check followed by a separate act with no atomicity mechanism and no statement that repeating the act is harmless, and plans that enforce uniqueness with a lookup before the insert. Wrapping a check-then-insert in a transaction does not make it atomic: an ordinary read at the default isolation level of PostgreSQL, MySQL/InnoDB, SQL Server, or Oracle locks no row that does not yet exist. **Not flagged:** a stampede of concurrent misses on an expensive cache entry, which `performance-backend.md` `BE-CACHE-003` owns; a lost update in a read-modify-write of an existing persisted record, which `BE-TXN-002` owns; and write skew on an invariant over a set of rows, which `BE-TXN-003` owns.

**Source:** CWE-367, Time-of-check Time-of-use (TOCTOU) Race Condition (its shared resources include "variables in multithreaded programs"); Goetz et al., *Java Concurrency in Practice* (2006), ch. 2 "Thread Safety", 2.2 "Atomicity"; PostgreSQL documentation, *INSERT* — `ON CONFLICT`.

## BE-LOCK — Locking and deadlock avoidance

### BE-LOCK-001

> A spec or plan that introduces a contended resource SHOULD state its concurrency-control strategy — optimistic (version / compare-and-set) or pessimistic (locking) — with the rationale for the choice.

**Verification:** Any spec or plan that introduces a contended resource SHOULD name and justify its optimistic-vs-pessimistic choice. `/{project}:analyze` flags contention-bearing plans that commit to neither.

**Source:** Microsoft, SQL Server *Transaction locking and row versioning guide* — pessimistic and optimistic concurrency control.

### BE-LOCK-002

> When more than one lock is held at once, locks SHOULD be acquired in a single consistent global order, each acquisition SHOULD wait no longer than a bounded lock timeout, and each lock SHOULD be released within a bounded hold time, to avoid deadlock and unbounded waits.

**Verification:** Any spec or plan that acquires more than one lock SHOULD state the global acquisition order, the acquisition timeout, and the hold-time bound. `/{project}:analyze` flags multi-lock plans with no stated ordering, timeout, or hold bound. **Not flagged:** a lock timeout or hold bound written as a bare literal, which `configuration-cross.md` `CFG-CONST-003` owns.

**Source:** PostgreSQL documentation, *Explicit Locking* — Deadlocks ("acquire locks on multiple objects in a consistent order"); Microsoft, SQL Server *Deadlocks guide* ("Access objects in the same order"); MySQL documentation, InnoDB deadlocks ("use the same order of operations").

## BE-TXN — Transaction isolation

### BE-TXN-001

> Each transaction SHOULD state its isolation level explicitly rather than relying on the engine's default, which varies by database and is often weaker than assumed.

**Verification:** Any spec or plan that introduces a multi-statement transaction SHOULD name the isolation level it requires and the anomalies that level prevents on the chosen engine. `/{project}:analyze` flags transactional plans that do not state an isolation level.

**Source:** Berenson et al., "A Critique of ANSI SQL Isolation Levels" (SIGMOD 1995) — the ANSI SQL-92 levels and the anomalies each permits; PostgreSQL *Transaction Isolation*, MySQL *Transaction Isolation Levels*, SQL Server *SET TRANSACTION ISOLATION LEVEL* and *Transaction locking and row versioning guide*, and Oracle *Database Concepts* — Data Concurrency and Consistency documentation; Jepsen, *MySQL 8.0.34* (lost update under REPEATABLE READ).

### BE-TXN-002

> A concurrent read-modify-write of persisted state MUST prevent lost updates — via an optimistic version/ETag check, a `SELECT … FOR UPDATE` row lock, an atomic in-place or conditional write (`UPDATE … SET n = n + 1`, compare-and-set), or an isolation level that detects the conflict and aborts one writer; a bare read-then-write under concurrency is silent data corruption.

**Verification:** Any spec or plan that introduces a concurrent update to shared persisted state MUST name its lost-update guard (version check, row lock, atomic update, or a conflict-detecting isolation level on the named engine). `/{project}:analyze` flags read-modify-write plans that commit to none. An isolation level is a guard only on an engine where that level detects the conflict — PostgreSQL's REPEATABLE READ and SERIALIZABLE do; MySQL/InnoDB's REPEATABLE READ does not. **Not flagged:** a conflict abort with no handling path, which `BE-TXN-004` owns; and a lost update across a read-modify-write spanning two HTTP requests — a client's read, then its write — which `api-backend.md` `BE-COND-001` owns.

**Source:** Berenson et al., "A Critique of ANSI SQL Isolation Levels" (SIGMOD 1995) — lost update (P4); PostgreSQL documentation, *Transaction Isolation* ("could not serialize access due to concurrent update"); Jepsen, *MySQL 8.0.34* ("MySQL Repeatable Read transactions cannot safely read a value and then write it").

### BE-TXN-003

> A transaction that reads a set of rows, decides on the basis of that read, and then writes — enforcing an invariant across rows such as "at least one on call", "no overlapping bookings", or "total within a limit" — MUST guard against write skew and phantoms: by running every transaction that reads or writes the invariant's rows at an isolation level that is serializable on the chosen engine, by locking every row the decision read (materializing a lock row when the decision rests on the absence of rows), or by a database constraint that enforces the invariant; snapshot isolation, including PostgreSQL's REPEATABLE READ, does not prevent write skew.

**Verification:** Any spec or plan whose transaction enforces an invariant spanning more than one row, or conditions a write on the absence of matching rows, MUST name its write-skew guard — the serializable isolation level on the named engine for every transaction that reads or writes the invariant's rows, the rows locked (or the materialized lock row), or the enforcing constraint. `/{project}:analyze` flags multi-row-invariant plans that rely on a default or snapshot isolation level with no lock or constraint. The commitment is to serializable behavior on the chosen engine, not to the keyword: Oracle documents its SERIALIZABLE as reading a transaction-start snapshot and raising `ORA-08177` when a transaction updates a row another transaction changed and committed after it began — the snapshot-isolation behavior under which Fekete et al. observed non-serializable executions on Oracle. A lost-update guard on the row being written (`BE-TXN-002`) is not a write-skew guard, because the two transactions write different rows. This rule governs invariants across rows: a check-then-insert enforcing uniqueness on a single key is `BE-RACE-003`'s.

**Source:** Berenson et al., "A Critique of ANSI SQL Isolation Levels" (SIGMOD 1995) — phantom (P3), which snapshot isolation does not preclude, and write skew (A5B), which it permits; Fekete et al., "Making Snapshot Isolation Serializable" (ACM TODS 30(2), 2005) — Oracle's SERIALIZABLE as a form of snapshot isolation, and materializing a conflict or promoting a read with `SELECT FOR UPDATE` to prevent write skew; PostgreSQL documentation, *Transaction Isolation*; MySQL documentation, *Transaction Isolation Levels*; Oracle *Database Concepts* — Data Concurrency and Consistency.

### BE-TXN-004

> A transaction that can abort on a concurrency conflict — a serialization failure (SQLSTATE `40001`), a deadlock victim, or a failed optimistic version check — SHOULD treat that abort as an expected outcome with a stated handling path: retry the whole transaction from its start, re-reading its inputs, under a bounded retry policy, or report the conflict to the caller.

**Verification:** Any spec or plan that runs transactions at an isolation level that aborts on conflict on the chosen engine (PostgreSQL REPEATABLE READ or SERIALIZABLE, for example), relies on deadlock detection, or uses optimistic version checks SHOULD state how a conflict abort is handled — whole-transaction retry, or the conflict response returned to the caller. `/{project}:analyze` flags plans that adopt a conflict-aborting guard with no stated abort handling. A whole-transaction retry rolls back explicitly first rather than re-issuing the failed statement — Oracle rolls back only the statement that hit a deadlock, leaving the transaction open with its earlier work. **Not flagged:** a retry missing its attempt bound, backoff, or jitter, or repeating a side effect outside the database with no idempotency basis, which `reliability-backend.md` `BE-RETRY-001` owns.

**Source:** PostgreSQL documentation, *Transaction Isolation* ("applications using this level must be prepared to retry transactions due to serialization failures") and *Serialization Failure Handling*; Microsoft, SQL Server *Deadlocks guide* (the victim's transaction is rolled back with error 1205); MySQL documentation, InnoDB deadlocks; Oracle *Database Concepts* — Data Concurrency and Consistency (statement-level rollback on deadlock).

## BE-COORD — Distributed coordination

### BE-COORD-001

> A distributed lock, lease, or leader election relied on for correctness — where two simultaneous holders would corrupt state — MUST carry a fencing token that the protected resource validates, rejecting stale holders; a lease or lock alone is insufficient, because a paused holder, a delayed request, or a clock that jumps can let a holder act after its lease has expired. A lock taken only for efficiency, where a second holder merely duplicates work, MAY omit fencing.

**Verification:** Any spec or plan that introduces a distributed lock or leader election MUST state whether it guards correctness or only efficiency, and for a correctness lock MUST commit to a monotonic fencing token enforced at the protected resource. `/{project}:analyze` flags distributed-lock plans that rely on lease expiry alone without stating that the lock guards efficiency only.

**Source:** Kleppmann, "How to do distributed locking" (fencing tokens; locks for efficiency vs. correctness).

### BE-COORD-002

> An operation invoked under at-least-once delivery, automatic retry, or a scheduler that can start it more than once for one scheduled time (a Kubernetes CronJob, an in-process scheduler running in every replica) or start a run while the previous run is still in progress MUST be idempotent or deduplication-guarded, with its delivery semantics stated — at-most-once, at-least-once, or exactly-once together with the boundary within which that guarantee holds. A deduplication record kept in the effect's own database MUST be committed in the same transaction as the effect.

**Verification:** Any spec or plan that consumes from a broker, exposes a retried operation, schedules a recurring job, or relies on at-least-once delivery MUST commit to idempotency or deduplication and state the delivery semantics. `/{project}:analyze` flags retried, queue-consuming, or scheduled-job plans with no idempotency or dedup commitment, exactly-once claims that name no boundary, and a database-write effect whose deduplication record is committed in a separate transaction on the same database. An exactly-once claim discharges the idempotency or deduplication commitment only for effects inside its named boundary: a database write, email, or payment call outside it still sees duplicates. A scheduled job's deduplication is a run record claimed atomically per scheduled time (`BE-RACE-003`) or a per-item claim that leaves a duplicate or overlapping run nothing to redo; a lock held only while a run executes is not one, since it stops runs overlapping but not a second run for the same time after the first finishes. This rule owns the deduplication of a queue consumer or scheduled job, which carries no `Idempotency-Key`. **Not flagged:** a deduplication record kept in a datastore other than the effect's, a dual write, which `BE-COORD-003` owns; an HTTP operation missing its idempotency contract (an `Idempotency-Key`, idempotent `PUT`/`DELETE`), which `api-backend.md` `BE-IDEMP` owns; unsound retry mechanics — attempt bounds, backoff, and jitter — which `reliability-backend.md` `BE-RETRY-001` owns; and a message redelivered indefinitely or discarded silently once its redelivery bound is exhausted, which `BE-RETRY-005` owns.

**Source:** Kubernetes documentation, *CronJob* — CronJob limitations, Job creation ("the Jobs that you define should be idempotent"), and Concurrency policy; Apache Kafka documentation, *Design* — Message Delivery Semantics ("Exactly-once delivery for other destination systems generally requires cooperation with such systems").

### BE-COORD-003

> An operation that must both commit a database change and publish a message or event, or write to a second datastore that must stay consistent with the first, MUST make the two atomic — a transactional outbox written in the same database transaction and relayed afterwards, change-data capture from the database log, or a single source of truth that the other system consumes; it MUST NOT perform them as two independent writes.

**Verification:** Any spec or plan in which one operation both writes to a database and publishes to a broker, or writes to a second datastore or search index that must stay consistent with it, MUST name the mechanism that makes the two atomic. `/{project}:analyze` flags plans that commit a transaction and then publish, or publish and then commit, as separate steps with no outbox, change-data capture, or single source of truth. **Not flagged:** a consumer that double-applies a message the relay published more than once, which `BE-COORD-002` owns.

**Source:** Richardson, microservices.io, "Pattern: Transactional outbox" ("2PC is not an option"; "The Message relay might publish a message more than once"); Kleppmann, "Using logs to build a solid data infrastructure (or: why dual writes are a bad idea)".

### BE-COORD-004

> A consumer whose correctness depends on message order MUST NOT assume ordering wider than its broker guarantees: it MUST state the ordering scope it relies on (a partition, key, or message group — not the whole topic or queue) and either route messages that must stay ordered into one scope by key, keep the producer from reordering them on retry, and process each scope serially, or detect and discard out-of-order messages with a per-entity sequence number or version check.

**Verification:** Any spec or plan that introduces a message consumer whose outcome depends on message order MUST state the ordering scope it relies on and either the partitioning key, the producer setting that keeps a retry from reordering within a scope (for Kafka, idempotence enabled, its default, or one request in flight), and serial per-scope processing that preserve it, or the stale-message guard. `/{project}:analyze` flags order-dependent consumers that assume topic- or queue-wide ordering, or that name neither a partitioning key nor a sequence or version check. The stale-message guard is a sequence number or version assigned per entity, not a wall-clock timestamp. **Not flagged:** a message dead-lettered out of an ordered stream, which `reliability-backend.md` `BE-RETRY-005` owns.

**Source:** Apache Kafka documentation, *Introduction* (events with the same key go to the same partition; a partition's events are read in the order written) and producer configuration (`retries`); Amazon SQS Developer Guide, *FIFO queue delivery logic* (ordering per message group ID); RabbitMQ documentation, *Queues* guide (message ordering); Kleppmann, "How to do distributed locking" (clock jumps).
