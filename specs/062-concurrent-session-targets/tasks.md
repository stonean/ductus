# 062 — Concurrent session targets Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Session core: identity, storage, lock

- [x] Bump `rust-version` in `runtime/Cargo.toml` from 1.88 to 1.89 for `std::fs::File::lock`
- [x] Add `SESSIONS_DIR` and `SESSIONS_LOCK` to `runtime/src/schema/paths.rs` beside `SESSION_FILE`
- [x] Move `derive_slug` from `runtime/src/primitives/create_feature.rs` to a shared `pub(crate)` home in `runtime/src/primitives/mod.rs`; `create-feature` calls the shared one
- [x] Create `runtime/src/session.rs` and register it in `runtime/src/lib.rs`: `identity_from_env(lookup)` (`DUCTUS_SESSION` → platform table with `CLAUDE_CODE_SESSION_ID` → none; empty is unset; slug-sanitized; empty after sanitizing is an `InvalidArgument` naming the variable)
- [x] Per-process record type per the data model; parse errors name the file
- [x] Lock helper over `.ductus/sessions/.lock`; directory creation writes `.ductus/sessions/.gitignore` containing `*`
- [x] Legacy-layout guard: when the active session file is a legacy tier, the core treats the process as unidentified
- [x] Unit tests with injected lookups: precedence, same-name sharing, empty-is-unset, sanitizing, refusal, Claude Code variable, legacy guard

- **Done when**: `cargo test` covers identity resolution for AC12, AC13 and AC14, and the legacy guard, all passing; `cargo clippy` is clean at the new MSRV.

## 2. Session core: resolve, write, sweep

- [x] `resolve(repo, identity)`: the five sources (`own`, `adopted`, `default`, `cleared`, `none`); `used-at` refresh; adoption notice; co-target notice against `seen-peers`; pending removal notice delivered and removed; malformed own file is an error, never adoption
- [x] `write_target` / `write_clear` / host-config: own file plus default per the plan; `seen-peers` set at write time; peers returned
- [x] Expiry sweep after target and clear writes: remove per-process files idle more than seven days, never the caller's own, re-reading `used-at` under the lock; report unparseable files without deleting them
- [x] Unit tests: adoption and pin (a later write by another identity leaves the pinned target alone), empty default pins nothing, cleared never adopts, co-target notice fires once and again only when the peer set changes, expiry boundary, a target used after the sweep began is kept, malformed own file errors, unidentified process reads and writes the default only, default always holds the latest change, `cli-config-dir` preserved through every write

- **Done when**: unit tests covering AC6, AC15, AC16, AC17, AC18, AC19, AC20 and AC22 pass against the core directly.

## 3. `write-session` through the core

- [x] Rewrite `runtime/src/primitives/write_session.rs` to delegate to the core, taking the identity from the environment at the edge; keep the three write shapes and their validation
- [x] Extend `WriteSessionResult` in `runtime/src/schema/primitives.rs` with `identity`, `own-path`, `peers`, `expired`, `unreadable`
- [x] Keep the default's bytes identical to today's for an unidentified write, so the existing `write_session` tests and the parity byte-equality check still pass unchanged

- **Done when**: the existing `write_session` tests pass unmodified, new tests cover the identified target and clear writes, and AC3's single-agent shape is asserted byte-for-byte against the default.

## 4. `resolve-session` primitive

- [x] Create `runtime/src/primitives/resolve_session.rs`, with args and result in `runtime/src/schema/primitives.rs` per the data model
- [x] Register it in `runtime/src/schema/registry.rs`, `runtime/src/mcp/server.rs`, the CLI in `runtime/src/main.rs`, and the interpreter dispatch in `runtime/src/interpreter/mod.rs`
- [x] Add `resolve-session` to `framework/runtime-tools.txt` and its permission entry to `framework/bootstrap/configure/claude.md` and `framework/bootstrap/configure/auggie.md`

- **Done when**: `ductus resolve-session` and the MCP tool return the data model's shape; `scripts/lint-tool-coverage.sh` and `scripts/audit/run-all.sh` pass.

## 5. `retarget-sessions` primitive

- [x] Create `runtime/src/primitives/retarget_sessions.rs`: re-target or clear every per-process file and the default naming `from`, under the lock; pending notice on every file but the caller's own; argument validation per the data model
- [x] Register it everywhere `resolve-session` was registered in task 4, including `framework/runtime-tools.txt` and both permission lists
- [x] Tests: fold re-targets every session naming the spec and the default; consolidate clears them; sessions naming other features are not written; the caller gets no notice and the others do; unparseable files are reported

- **Done when**: tests covering AC5 and AC21 pass, and the audit families from task 4 pass.

## 6. `dashboard` and the exec seed through the core

- [x] `load_session_target` in `runtime/src/primitives/dashboard.rs` resolves through the core; the payload gains `identity`, `source` and `notices`; `rendered-markdown` shows the session label on the target line and each notice beneath it
- [x] The exec seed in `runtime/src/main.rs` resolves through the core instead of reading the session path
- [x] Regenerate any golden stream under `runtime/tests/golden/` that the seed change alters, and confirm each diff is only the seed

- **Done when**: dashboard tests cover AC8, the exec seed test covers AC23's two cases, and `cargo test` passes including the walker goldens.

## 7. Cross-process integration tests

- [x] Create `runtime/tests/concurrent_sessions.rs`, spawning the built binary with per-child environments (`DUCTUS_SESSION`, `CLAUDE_CODE_SESSION_ID`, neither)
- [x] Two identities: a target write in one leaves the other's resolved target unchanged, for a plain target write and for each side-effect shape the commands use (AC1); each resolves its own feature (AC2)
- [x] Concurrent writers: many children with distinct identities write at once, and each own file holds its own target (AC4)
- [x] Two children with only `CLAUDE_CODE_SESSION_ID` set, to different values, hold separate targets (AC13); two with neither share the default (AC15)

- **Done when**: the integration tests for AC1, AC2, AC4, AC13 and AC15 pass under `cargo test`.

## 8. Gitignore

- [x] Add `/.ductus/sessions/` to the managed block in `framework/templates/project/gitignore`, with a comment, beside `/.ductus/session.toml`
- [x] Add `/.ductus/sessions/` to this repository's `.gitignore`

- **Done when**: after running two identified sessions in this repository, `git status` shows nothing under `.ductus/` (AC7), and the manifest and template audit families pass.

## 9. Command files

- [x] Replace hand reads of the session file with "Invoke `resolve-session`; display its notices; markdown-only: read the shared default (§concurrent-features)" in `amend`, `analyze`, `clarify`, `groom`, `implement`, `plan`, `prune`, `review`, `specify` and `target` under `framework/commands/`
- [x] `fold` step 13 and `consolidate` step 6 invoke `retarget-sessions`; consolidate's step drops its hand read of the session
- [x] `status` describes the target line's session label, the notices, and the markdown-only derivation; `help` updates the Session target glossary entry
- [x] Mirror every edit to `.claude/commands/ductus/`

- **Done when**: no file under `framework/commands/` instructs a hand read of the session file outside a markdown-only branch; the mirrors are byte-identical; `scripts/audit/run-all.sh` passes (AC24's command-side statement).

## 10. Constitution and the 010 signpost

- [x] Rewrite §concurrent-features in `framework/constitution.md`: per-process targets and identity; the shared default and pin on first resolution; `git worktree` kept for isolating edits, with the one-branch-per-worktree limit; the two open bounds; the markdown-only reduction; removal extended to every session in the working tree, keeping the teammate bound
- [x] Add a post-completion note to `specs/010-agent-autonomy/spec.md` §Parallel milestones and Resolved Question 5 linking to this spec

- **Done when**: AC10, AC24 and AC25 hold in the constitution text; 010 carries the back-link that discharges 062's declared impact; anchors resolve (`resolve-anchor`); markdownlint is clean.

## 11. Verification sweep

- [x] `cargo build --release`, `cargo test`, `cargo clippy --all-targets` under `runtime/`
- [x] `scripts/audit/run-all.sh`
- [x] Manual check in this repository: two Claude Code sessions targeting different specs each keep their own through `/ductus:target` and `/ductus:status`, and a `retarget-sessions` call against a scratch spec both targets re-points or clears both and notifies the other session

- **Done when**: every command above passes, and each acceptance criterion maps to a passing test or a verified text change.

## 12. `dashboard` resolves read-only

- [x] `runtime/src/primitives/dashboard.rs` resolves through `session::peek`, not `session::resolve`: it pins no adoption, refreshes no `used-at`, and consumes no notice — restoring the module's read-only contract. Found in task 11's manual check: `scripts/audit/lib.sh` calls `ductus dashboard` to enumerate the corpus, so a delivering resolution silently pinned the auditing session and would consume its notices before any command showed them
- [x] `session::peek` computes the notices the next resolution would deliver — a pending removal notice, a changed co-target set, and a pending adoption (worded as pending) — without writing, so `/ductus:status` still shows them
- [x] Update the dashboard tests to assert read-only behavior (repeat calls identical, no own file created), and `framework/commands/status.md` to say the view shows the notices the next command delivers rather than consuming them

- **Done when**: a `dashboard` call writes nothing under `.ductus/sessions/`, `/ductus:status` still renders the session label and pending notices, the next `resolve-session` delivers those notices, and `cargo test` and `scripts/audit/run-all.sh` pass.

## 13. Session lock: a bounded wait, taken by every write

- [x] `session::lock` acquires with `File::try_lock` in a retry loop bounded by a named `SESSION_LOCK_TIMEOUT` constant, and on expiry returns an error naming `.ductus/sessions/.lock` (review: BE-TIMEOUT-001)
- [x] The `write-session`, `resolve-session` and `retarget-sessions` MCP handlers run through `dispatch_blocking`, as `dashboard` does, so a lock wait never holds an async worker
- [x] `write` and `retarget` hold the lock on every call, the unidentified path included, closing the window where `.ductus/sessions/` does not exist yet and an unidentified read-modify-write of the default races the first identified one (review: BE-TXN-002); `lock_if_shared` stays only for the read-only `peek`. Update the AC3 tests and the plan's lock paragraph to match
- [x] Test: with the lock held, a second acquirer fails within the timeout with the lock file named (review observation: no test exercised the lock)

- **Done when**: a held lock makes a second acquirer fail, naming the lock file, within the timeout; an unidentified write takes the lock; `git status` still shows nothing under `.ductus/`; `cargo test` passes.

## 14. Session identity read once at startup; an environment-variable inventory

- [x] The binary reads the session identity variables once at startup — in `main`, for the CLI, `exec` and `mcp` — and caches the result; primitives read the cached value, and an in-process caller that never initialized it is unidentified, so in-process tests and walks are unidentified by construction (review: CFG-ENV-001; AGENTS.md session-identity gotcha on `runtime/tests/walker.rs`)
- [x] An invalid `DUCTUS_SESSION` still fails each session primitive with the message naming the variable, rather than stopping the MCP server and every tool with it
- [x] Add one canonical inventory of every environment variable the runtime reads — `DUCTUS_SESSION`, `CLAUDE_CODE_SESSION_ID`, `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` — with each one's purpose, whether it is required, and its default; name it in the plan's Affected Files (review: CFG-ENV-002)

- **Done when**: no primitive reads the environment per call; the in-process walker test runs unidentified in a shell that carries a platform session id; the inventory lists every variable `runtime/src` reads; `cargo test` passes.

## 15. Allowlist validation of the feature and scenario names written into sessions

- [x] `retarget-sessions` checks `from` and `feature` against the feature-directory grammar (`parse_feature_dir`) and `scenario` against the slug grammar (`validate_slug`) before touching any session (review: BE-INPUT-002)
- [x] `write-session` applies the same checks to its `feature` and `scenario`, since a target it writes reaches other sessions through the shared default
- [x] Tests: a newline, a path separator or an empty value in any of them is refused with an error naming the argument, and nothing is written

- **Done when**: no name outside the feature-directory or slug grammar reaches a session file or a notice through either primitive; `cargo test` passes.

## 16. Resolution reports unreadable peer files; the sweep's keep guard is tested

- [x] `Resolution` carries the peer files `due_notices` could not parse, and `resolve-session` and the `dashboard` payload report them as `unreadable`, as `write-session` and `retarget-sessions` already do, so "no co-target" and "could not check" differ (review: QUAL-CLAIM-001; AC22)
- [x] A test drives the sweep with a stale own file and asserts the caller's own file is kept — or, if no caller can reach that guard, the guard is removed and the test renamed to what it checks (review: QUAL-TEST-001)

- **Done when**: a corrupt peer file is named in `resolve-session`'s and `dashboard`'s results; the keep-guard test fails with the guard removed (or the guard is gone); `cargo test` passes.

## 17. Commands deliver every session notice and write result

- [x] `target.md` step 1 invokes `resolve-session` whatever the argument and displays its notices before any write, so `/{project}:target X` and `--clear` no longer discard a pending fold or consolidation notice (review observation; AC21), and `ductus exec` dispatching it unconditionally becomes the intended behavior (review: AGENTS.md backtick-dispatch gotcha)
- [x] Every `write-session` step — target, specify, groom, amend — displays the result's `peers` (label, feature, scenario, last used) and any `expired` or `unreadable` sessions, so the writing session is told who shares its feature (review observation; AC19, AC22)
- [x] fold's and consolidate's report steps name the sessions `retarget-sessions` re-pointed or cleared and any `unreadable` file (review observation; AC21, AC22)
- [x] Regenerate `.claude/commands/ductus/`, and re-bless any walker golden the step change alters, confirming the diff is only that step

- **Done when**: no command drops a notice or a write result the spec promises; the mirrors are in sync; `cargo test` and `scripts/audit/run-all.sh` pass.

## 18. `ductus exec` shows session notices and is tested with an identity

- [x] After dispatching `resolve-session`, the exec walker emits each returned notice into its stream, so an exec walk shows what its resolution consumes (review observation; AC19, AC21)
- [x] An `exec_subprocess` test runs `ductus exec` with `CLAUDE_CODE_SESSION_ID` set and asserts the walk acts on that process's own target, not the shared default's (review observation; AC23)

- **Done when**: an identified exec walk with a pending notice emits it; the identified exec test passes and fails if `run_exec` stops passing the identity; `cargo test` passes.

## 19. Disposition out-of-spec finding: fetch-archive reads its allowlist variable on every call

- [x] `runtime/src/primitives/fetch_archive.rs:296` — `host_is_insecure_allowed` calls `std::env::var("DUCTUS_FETCH_ALLOW_INSECURE_HOSTS")` per call, which CFG-ENV-001 forbids (read once at startup and cache); surfaced implementing task 14, which reads the session variables once at startup — fixed

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task.

## 20. A write names the own record it replaces and never loses a notice; `/target` continues past a malformed own file

- [x] `write` tells a missing own record (no notice to deliver) from one that does not parse: the unparseable file is reported in `unreadable`, as replaced, rather than treated as absent (review: QUAL-CLAIM-001)
- [x] `write` stores the process's own record last — after the shared default and the sweep — so an error part-way leaves the pending notice in place for the next resolution instead of losing it undelivered (review observation)
- [x] `target.md` step 1: when `resolve-session` fails because this process's own session file does not parse and an argument was supplied, report the named file and continue — the target write replaces it (review observation)
- [x] Tests: a write over a malformed own file names it; a write whose default store fails leaves the own record, and its notice, untouched

- **Done when**: no write treats an unparseable own record as absent or loses a pending notice on a part-way failure; `/target X` repairs a malformed own file; `cargo test` passes.

## 21. Session names are held to a visible-ASCII allowlist

- [x] `validate_session_feature` admits only visible ASCII (`is_ascii_graphic`) with no path separator, so bidi and zero-width characters such as U+202E and U+200B are refused along with newlines (review: BE-INPUT-002); `data-model.md` states the rule and the legacy names it refuses
- [x] Test: U+202E and U+200B in a feature name are refused, naming the argument

- **Done when**: no character outside visible ASCII reaches a session file or notice through a stored name; `cargo test` passes.

## 22. The environment-variable inventory lists the proxy variables

- [x] `docs/runtime.md` lists `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY` and `NO_PROXY` (with their lowercase forms), which `fetch-archive`'s HTTP client reads on each fetch, with each one's purpose and default; its completeness and read-once statements say which variables are captured at startup and which the client reads per fetch until 048's `fetch-archive-reads-its-proxy-once` resolves it (review: CFG-ENV-002)
- [x] `main`'s comment names what it captures rather than claiming every variable the runtime reads

- **Done when**: every environment variable the runtime or its HTTP client reads is in the inventory, and no text claims a read-once that does not hold.

## 23. The identity-capture test holds in any environment; the lock poll's pacing is explained

- [x] `a_process_that_never_captured_its_environment_is_unidentified` re-runs itself as a child test process with `CLAUDE_CODE_SESSION_ID` set, so it fails in any environment — CI included — if `process_identity` reads the environment per call (review: QUAL-TEST-001)
- [x] `LOCK_RETRY` records why the lock poll uses a short fixed interval rather than a jittered backoff: it polls a local advisory lock held for milliseconds, bounded by `SESSION_LOCK_TIMEOUT` (review: BE-RETRY-001, low confidence)

- **Done when**: the capture test fails under a per-call environment read even when the parent shell carries no identity; `cargo test` passes.

## 24. A malformed shared default is reported, repairable, and never blocks an exec walk

- [x] `write` reports a shared default that exists but does not parse in `unreadable`, as replaced — its `cli-config-dir` is not carried forward — rather than treating it as absent (review: QUAL-CLAIM-001; AC22)
- [x] `target.md` step 1 continues past a malformed shared default as well as a malformed own file when an argument was supplied, since the write replaces it (review observation: a regression from step 1 running whatever the argument)
- [x] The exec seed reads the shared default leniently, as before 062, whenever the process has no own record, so a malformed default no longer halts every walk; it stays strict on the process's own file (AC22), and `target.md` says an exec walk halts on a malformed own file (review observations)
- [x] Tests: a write over a malformed default names it; an exec walk with a malformed default and no own record completes

- **Done when**: no session file is replaced unreported; `/target X` repairs a malformed default; `ductus exec` runs with a malformed default; `cargo test` passes.

## 25. The environment-variable inventory is accurate about the HTTP client

- [x] `docs/runtime.md`'s `REQUEST_METHOD` row says that when it is set the HTTP client ignores every proxy variable, not only `HTTP_PROXY` (review: CFG-ENV-002)
- [x] The inventory lists `SSL_CERT_FILE` and `SSL_CERT_DIR`, which the HTTP client's certificate verifier reads on each fetch on Linux and other non-Apple Unix, replacing the system store when set (review: CFG-ENV-002)
- [x] The carve-out names what it leaves out and why — variables the Rust standard library, the async runtime and git read the same way for every program (`HOME`, `TMPDIR`, `RUST_BACKTRACE`, `TOKIO_WORKER_THREADS`) — so it separates them from the HTTP client's configuration it lists
- [x] 048's `fetch-archive-reads-its-proxy-once` states the CGI guard correctly and names the certificate variables among the per-fetch reads (review observation)

- **Done when**: every row matches the locked dependencies' behavior, and the inventory's scope statement draws a line its own rows respect.

## 26. First writers create the sessions ignore file once

The fix and its test landed in `9a256750`. The first item is checked once a `runtime` run for a sha containing that commit shows `concurrent_writers_each_keep_their_own_target` passing on windows-latest, ubuntu-latest and macos-latest. If Windows still fails, read the failing assertion before changing anything: the test had never passed there, so a second Windows-only failure may be waiting behind the first.

- [x] Implement the behavior described in `scenarios/first-writers-create-the-ignore-file-once.md`
- [x] `lock_within` in `runtime/src/session.rs` creates the sessions directory, takes the lock, and only then writes `.gitignore` when absent
- [x] A test that fails when the ignore file is written before the lock: a contender that times out on a held lock has not written it

- **Done when**: the ignore file is written only under the session lock, the test is proven red against the old order, and `runtime` is green on windows-latest, ubuntu-latest and macos-latest

## 27. Narrow the removal-notice claim to per-process sessions

- [x] AC21 and the Removal strands no process section: the shared default is changed without a notice, because every unidentified process reads it, and the acting command's report names it
- [x] The constitution's §concurrent-features, fold.md and consolidate.md say the same

- **Done when**: every statement of the rule matches session::retarget, and 062's analyze records this reopen

## 28. The session-lock poll backs off with jitter (BE-RETRY-001)

- [x] Found by the 2026-10-27 analysis: the poll retried `try_lock` at a fixed `LOCK_RETRY` of ten milliseconds, and the plan argued `BE-RETRY-001`'s backoff and jitter away for a local lock, though the rule's Statement has no such scope. Fixed, not routed (owner's delegated disposition, 2026-10-27)
- [x] `lock_within` (`runtime/src/session.rs`) waits a uniformly drawn time up to a bound that starts at `LOCK_RETRY_FIRST` and doubles to `LOCK_RETRY_CAP`, still stopping at the deadline, and retries only `TryLockError::WouldBlock`; the plan's Synchronization section states it
- [x] A test shows the bound doubling to the cap and every wait within it, and a mutation per behavior in `scripts/mutation/session-lock-backoff.py` is killed

- **Done when**: the poll backs off exponentially with full jitter up to its cap, stops at SESSION_LOCK_TIMEOUT, retries only a held lock, and its mutations are killed
