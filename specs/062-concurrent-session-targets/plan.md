# 062 — Concurrent session targets Plan

Implements [062 — Concurrent session targets](spec.md).

## Overview

Session resolution moves out of the host and into the runtime, because only the
runtime process can see the environment the identity lives in. A new session
core module owns identity, storage, locking, adoption, notices and expiry.
Four surfaces route through it: `write-session`, two new primitives
(`resolve-session` and `retarget-sessions`), the `dashboard` session read, and
the `ductus exec` context seed. The shared default, `.ductus/session.toml`,
keeps its path, keys and resolution chain. Per-process targets are added
beside it as one file per identity under `.ductus/sessions/`. The command
files stop reading the session file by hand and invoke `resolve-session`. The
markdown-only path keeps reading the default and says why. The constitution's
§concurrent-features is rewritten, and 010 gets its signpost.

## Technical Decisions

### Resolution moves into the runtime

Today every targeted command resolves its target with a host read of the
session file. `amend`, `analyze`, `clarify`, `fold`, `implement`, `plan`,
`prune`, `review` and `target` all instruct "Read `.ductus/session.toml`" or
"use the session target from `.ductus/session.toml`" (for example
`framework/commands/clarify.md:22`, `framework/commands/implement.md:98`,
`framework/commands/target.md:29`). Only writes go through a primitive
(`write-session`). A host read cannot be made per-process: the identity is an
environment variable, and the host may not read its environment on the
markdown-only path (`framework/constitution.md:617`). On the MCP path it could
only do so by the model running a shell command before every command, which is
the diligence dependency the spec's Resolved Questions reject.

The runtime can read the environment: each agent process spawns its own
`ductus mcp` server, and that server's environment carries the agent's session
identity (spec, Resolved Questions, grounded 2026-09-29). So resolution becomes
a primitive, `resolve-session`, and the commands invoke it.

### One session core, four callers

A new module, `runtime/src/session.rs`, holds the whole contract: identity
resolution, per-process paths, the lock, the per-process record, adoption,
notices and the expiry sweep. It is called by `write-session`,
`resolve-session`, `retarget-sessions`, `dashboard` and the exec seed, and by
nothing else. Today the read logic exists twice, in `dashboard`
(`runtime/src/primitives/dashboard.rs:737-761`) and in the exec seed
(`runtime/src/main.rs:523-535`), and the write logic once more in
`runtime/src/primitives/write_session.rs:59-123`. Pin-on-first-resolution
and once-only notices only hold if every reader follows the same rule, so
duplicated read logic would reproduce the defect this spec removes.

The module takes the identity as a **parameter**. Only the edges (each session
primitive's `run`, which the MCP handlers and the CLI call, and the exec seed)
ask for the process's own, through
`session::process_identity`, which answers from the identity variables the
binary's `main` captured once at startup (`session::init_process_identity`;
`CFG-ENV-001`). A per-call read of the environment was tried first and
rejected (review): a test binary never captures the variables, so in-process
tests and walks are unidentified by construction, whatever shell runs them.
Tests then pass identities directly instead of mutating process environment,
which is `unsafe` in edition 2024 (`runtime/Cargo.toml:4`) and racy across
parallel tests. `identity_from_env` itself takes a lookup closure, so the
Claude Code rule (AC13) is unit-testable with an injected environment.

### Identity

`identity_from_env(lookup)` returns `Ok(None)`, `Ok(Some(Identity))`, or an
error:

1. `DUCTUS_SESSION`, trimmed. Empty means unset (AC14). Otherwise the value is
   sanitized with the slug rule `create-feature` applies to titles and branch
   identifiers (`derive_slug`, `runtime/src/primitives/create_feature.rs:248`).
   That function is promoted to `pub(crate)` in `runtime/src/primitives/mod.rs`
   so both callers share one rule, rather than a second copy drifting from it.
   A value that sanitizes to nothing is an `InvalidArgument` naming
   `DUCTUS_SESSION` (AC14).
2. A platform table of environment variable names, checked in order. It holds
   one entry today, `CLAUDE_CODE_SESSION_ID` (source `claude-code`), the only
   one verified (spec, Resolved Questions). A new agent is one more row, added
   when its variable is verified rather than guessed. The value is sanitized
   the same way; a UUID passes through unchanged.
3. Otherwise `None`.

Both sources are environment variables, so the standard precedence of flag,
then environment, then file, then default holds by construction
(`CFG-ENV-007`): the identity has no flag or file source, `DUCTUS_SESSION`
outranks the platform variable, and no identity is the default.

`Identity` carries the sanitized `key` (the file stem) and a `source` (`named`
or the platform source). Notices use its display label: the name for a named
session, and `claude-code:` plus the first eight characters of the key for a
platform one.

### Storage: confine each process to its own file

- **Shared default:** `.ductus/session.toml`, unchanged. It keeps the same
  keys, the same `SessionRecord` field order
  (`runtime/src/primitives/write_session.rs:219-235`) and the same
  `SESSION_CHAIN` (`runtime/src/schema/paths.rs:76`), so AC3 and 022's
  per-project resolution table both stay true.
- **Per-process targets:** `.ductus/sessions/{key}.toml`, one file per
  identity. The schema is in [data-model.md](data-model.md).

One file per identity rather than one shared file with a table per identity is
the `BE-RACE-002` choice. A process's own resolutions and writes touch only its
own file and the default. Only a removal (`retarget-sessions`) and the expiry
sweep touch another identity's file, and a malformed file damages one session
instead of all of them.

The new paths are constants in `runtime/src/schema/paths.rs` beside
`SESSION_FILE`: `SESSIONS_DIR` and `SESSIONS_LOCK`. Family
`runtime-hardcoded-paths` (`scripts/audit/runtime-hardcoded-paths.sh`) holds
the runtime to declaring paths there.

**Per-process targets need the `.ductus/` layout.** When the active session
file is a legacy tier (`.govern/session.toml` or the root
`.govern.session.toml`, `runtime/src/schema/paths.rs:69-73`), the core treats
the process as unidentified. Otherwise it would create `.ductus/` state while
a legacy file lingers, which 042's cutover rule forbids
(`runtime/src/schema/paths.rs:159-164`). A legacy layout exists only until the
next `/ductus` run migrates it.

### Synchronization: one advisory lock (`BE-RACE-001`)

Every resolution and write that touches `.ductus/sessions/` or the default
holds an exclusive advisory lock on `.ductus/sessions/.lock`, taken with
`std::fs::File::try_lock` in a retry loop bounded by the named constant
`SESSION_LOCK_TIMEOUT` (ten seconds, `BE-TIMEOUT-001`). The critical sections
are a few small reads and one or two atomic writes, so a wait that long means
the holder is stalled, and the call fails naming the lock file rather than
hanging every agent behind it. The poll (`BE-RETRY-001`) is bounded by that
timeout, at most about a thousand attempts at `LOCK_RETRY`'s ten milliseconds,
and retries only `try_lock`, which changes nothing when it fails. The interval
is fixed rather than a jittered backoff on purpose: the contenders are a few
local processes polling a kernel lock held for milliseconds, with no
downstream for synchronized retries to overload, which is what that rule's
backoff and jitter exist to prevent. The MCP handlers of the three session tools run
on the blocking pool, so a wait never holds an async worker. Writes inside the
lock still use the existing tempfile + rename helper (`write_atomic`,
`runtime/src/primitives/mod.rs:988`), so a reader outside the lock, such as a
markdown-only host reading the default, never sees a torn file.

On the `.ductus/` layout a write or retarget takes the lock on every call,
the unidentified path included. An exception for a repo nobody had used
per-process targets in was tried first and rejected (review, `BE-TXN-002`):
the first identified write is what creates `.ductus/sessions/`, so an
unidentified read-modify-write that skipped the lock because the directory
did not exist yet could interleave with it and lose `cli-config-dir` or the
default's target. The directory is self-ignoring, so taking the lock leaves
`git status` clean (AC7). Only the read-only peek (`dashboard`, the exec seed)
locks just when the directory already exists, so a view creates nothing. On a
legacy layout nothing is per process and no lock is taken.

The lock is what makes AC4 and AC18 hold. Two processes writing at once each
land their own file, and the default takes the later write, which is its
meaning. The expiry sweep re-reads `used-at` under the same lock that
resolutions refresh it under, so it cannot remove a target that was used after
it looked.

`File::lock` is stable from Rust 1.89. The declared `rust-version` is 1.88
(`runtime/Cargo.toml:5`) and the pinned toolchain is 1.97.0
(`runtime/rust-toolchain.toml`), so the change is a one-line MSRV bump to
1.89. It adds no dependency.

### `resolve-session`: the one read, with its side effects

`resolve-session` takes no arguments and returns the calling process's target,
where it came from, and any notices. Under the lock:

- **No identity:** the core reads the default and returns
  `source: "default"`. It writes nothing. A malformed default is an
  operational error naming the file, as `dashboard` already treats it
  (`runtime/src/primitives/dashboard.rs:743-746`, test at `:1262`).
- **Identity, own file present:**
  - A **cleared** file returns `source: "cleared"` and no target (AC20).
  - Otherwise it returns `source: "own"` and the target.
  - Either way the core refreshes `used-at`, delivers and clears any pending
    removal notice, and computes the co-targeting peers: other unexpired
    per-process files naming the same feature. If the peer set differs from
    the file's `seen-peers`, it emits a co-target notice and records the new
    set (AC19). All of this is one atomic rewrite of the own file.
- **Identity, own file absent:**
  - When the default holds a target, the core writes the own file from it,
    returns `source: "adopted"`, and emits an adoption notice (AC17).
  - When the default is empty, it returns `source: "none"` and writes nothing,
    since nothing is pinned (spec, §The shared default).
- **Own file malformed:** an operational error naming the file. Never
  adoption (AC22).

The read has side effects by necessity. Pin-on-first-resolution, once-only
notices and use-based expiry are all stateful, and a pure read can satisfy
none of them. The cost is one small atomic write to a gitignored file per
command.

### `write-session`: own file plus default

The existing three write shapes stay (`runtime/src/primitives/write_session.rs:74-110`):

- **Target write:**
  - The core writes the default exactly as today, preserving `cli-config-dir`
    (AC6, AC16).
  - With an identity it also writes the own file: the target, `used-at`,
    `cleared` removed, and `seen-peers` set to the current peer set. The
    writer is told about peers in the result, so the next resolution does not
    tell it again.
  - The result gains `identity` and `peers`. Each peer entry carries a label,
    a feature, an optional scenario, and `last-used` (AC19).
- **Clear write:**
  - With an identity, the own file becomes `cleared = true` with no target
    keys.
  - The default is cleared too (target block removed, `cli-config-dir` kept),
    as today. The default holds the most recent target change in the working
    tree. Leaving it set would let a restarted agent resurrect a target its
    operator just cleared.
- **Host-config write:** default only, unchanged. `/ductus` records
  `cli-config-dir` this way (`framework/bootstrap/ductus.md:40`).

After a target or clear write, the core runs the expiry sweep. It deletes
per-process files whose `used-at` is more than seven days old, never the
caller's own. The window is `session::IDLE_EXPIRY`, its one definition
(`CFG-CONST-001`): the exec walker's expired-session line reads it rather than
restating the number. It reports files it could not parse in `unreadable` and leaves
them in place: a file that cannot be read cannot be proven idle.

### `retarget-sessions`: removal without stranding

A new primitive replaces fold's `write-session` re-target
(`framework/commands/fold.md:75`, step 13) and consolidate's host-conditional
clear (`framework/commands/consolidate.md:77`, step 6). Consolidate's clear
step currently reads the session file by hand to decide whether to act.

- **Arguments:** `from` (the removed feature), then either a new target
  (`feature`, `path`, optional `scenario` and `scenario-path`) or `clear`, and
  `cause` (`fold` or `consolidate`).
- **Under the lock:** it rewrites every per-process file and the default that
  name `from`. The caller's own file is rewritten without a notice. Every other
  file gets a pending removal notice naming the cause, the old and new target,
  and the caller's label (AC21). Files naming other features are not written
  (AC5).

It is a separate primitive rather than a `write-session` mode. That keeps
`write-session` meaning "write *my* session" and keeps the cross-session
sweep, the only operation that writes another identity's target, in one
auditable place.

### `dashboard` and the exec seed

- **`dashboard`:** `load_session_target` resolves through the core's
  **peek**, not its resolve: it pins no adoption, refreshes no `used-at`, and
  consumes no notice, so the primitive keeps its read-only contract. The peek
  computes the notices the process's next resolution would deliver — a
  pending removal notice, a changed co-target set, a pending adoption — and
  the dashboard shows them without consuming them; the next command's
  `resolve-session` delivers them. The payload gains `session-identity`,
  `session-source` and `session-notices`, and `rendered-markdown` renders the
  process label on the target line and each notice beneath it (AC8).
  `framework/commands/status.md` describes the new line and the markdown-only
  derivation, which is the default only. A delivering read was tried first
  and rejected (task 12): `dashboard` is also called for data —
  `scripts/audit/lib.sh` enumerates the corpus with it — and every such call
  pinned the caller's session and would consume its notices unseen.
- **Exec seed:** `runtime/src/main.rs:523-535` seeds from the core's resolve
  instead of parsing the session path, so `ductus exec` from an agent's shell
  tool carries that agent's identity (AC23). The retarget exceptions in
  `runtime/src/interpreter/mod.rs:303-316` are unchanged, because they govern
  which keys a walk may overwrite, not where the seed came from.

### Gitignore: two layers

`/.ductus/sessions/` is added to the framework-managed block in
`framework/templates/project/gitignore` (beside `/.ductus/session.toml`,
line 31) and to this repository's `.gitignore` (line 10), satisfying AC7.
`/ductus` installs that block with `merge-managed-block`
(`framework/bootstrap/ductus.md:36`), so adopters receive it on their next
run with no migration.

The core also writes `.ductus/sessions/.gitignore` containing `*` whenever it
holds the session lock and finds the file absent — under the lock, so racing
first writers create it once
(`scenarios/first-writers-create-the-ignore-file-once.md`). That covers an
adopter whose runtime was updated before the managed block: per-process files
never show up as untracked, whatever state the root `.gitignore` is in.

### Command files

Every command that resolves its target by hand switches to "Invoke
`resolve-session`; display any notices it returns; on the markdown-only path
read the shared default `.ductus/session.toml`
([§concurrent-features](../../framework/constitution.md#concurrent-features))".
That means `amend`, `analyze`, `clarify`, `groom`, `implement`, `plan`,
`prune`, `review`, `specify` (Context) and `target` (the no-argument display).
The `target --clear` step keeps `write-session` clear mode (AC20). `fold` and
`consolidate` switch to `retarget-sessions`. `help` updates its Session target
glossary entry.

Every file is mirrored to `.claude/commands/ductus/`, and the two new tools
are added to `framework/runtime-tools.txt` and to the permission lists in
`framework/bootstrap/configure/claude.md` and
`framework/bootstrap/configure/auggie.md`. That is the file set the last
primitive addition touched (commit `a0185c22`).

### Constitution and the 010 signpost

§concurrent-features (`framework/constitution.md:762-773`) is rewritten to
cover:

- per-process targets, identity resolution, and the shared default with pin
  on first resolution (AC10);
- worktrees kept for isolating edits (AC25); the note on git's
  one-branch-per-worktree limit was dropped by `050-constitution`'s cut to
  one read;
- the two open bounds: unidentified processes collide, and mixed agent CLIs
  share one `cli-config-dir` (AC25);
- the markdown-only reduction (AC24).

The removal paragraphs extend "the session" to "every session in the working
tree", and the teammate bound stays. 010's §Parallel milestones verdict and
Resolved Question 5 get a post-completion note linking here (AC11). That note
discharges the declared impact.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/session.rs` | Create | Session core: identity, paths, lock, per-process record, resolve, write, retarget, sweep |
| `runtime/src/lib.rs` | Modify | Register the `session` module |
| `runtime/src/schema/paths.rs` | Modify | `SESSIONS_DIR`, `SESSIONS_LOCK` constants |
| `runtime/src/primitives/create_feature.rs` | Modify | Move `derive_slug` out for shared use |
| `runtime/src/primitives/mod.rs` | Modify | Shared `derive_slug`; register new primitive modules |
| `runtime/src/primitives/write_session.rs` | Modify | Route through the core; own file, peers, sweep |
| `runtime/src/primitives/resolve_session.rs` | Create | `resolve-session` primitive |
| `runtime/src/primitives/retarget_sessions.rs` | Create | `retarget-sessions` primitive |
| `runtime/src/primitives/dashboard.rs` | Modify | Resolve through the core; identity and notices in payload and render |
| `runtime/src/main.rs` | Modify | CLI subcommands; exec seed through the core |
| `runtime/src/mcp/server.rs` | Modify | MCP tools for the two new primitives |
| `runtime/src/interpreter/mod.rs` | Modify | Dispatch the two new primitives |
| `runtime/src/schema/primitives.rs` | Modify | Args and result schemas |
| `runtime/src/schema/registry.rs` | Modify | Register the new primitives |
| `runtime/Cargo.toml` | Modify | `rust-version` 1.88 → 1.89 for `File::lock` |
| `runtime/tests/concurrent_sessions.rs` | Create | Spawned-binary tests with per-child environments |
| `runtime/tests/golden/*.jsonl` | Modify | Only if the exec seed change alters a golden stream |
| `framework/runtime-tools.txt` | Modify | Add `resolve-session`, `retarget-sessions` |
| `framework/bootstrap/configure/claude.md` | Modify | Permission entries for the new tools |
| `framework/bootstrap/configure/auggie.md` | Modify | Permission entries for the new tools |
| `framework/templates/project/gitignore` | Modify | `/.ductus/sessions/` |
| `.gitignore` | Modify | `/.ductus/sessions/` |
| `framework/commands/{amend,analyze,clarify,consolidate,fold,groom,help,implement,plan,prune,review,specify,status,target}.md` | Modify | Resolve through `resolve-session`; removal through `retarget-sessions` |
| `.claude/commands/ductus/*.md` | Modify | Mirror the command edits |
| `framework/constitution.md` | Modify | Rewrite §concurrent-features |
| `specs/010-agent-autonomy/spec.md` | Modify | Signpost on the Parallel milestones verdict |
| `docs/runtime.md` | Modify | The environment-variable inventory (`CFG-ENV-002`; review) |
| `runtime/src/primitives/fetch_archive.rs` | Modify | Its allowlist variable captured once at startup (task 19) |
| `README.md` | Modify | Its pointer to `docs/runtime.md` names the inventory (review) |
| `AGENTS.md` | Modify | Gotcha: session identity in tests, captured once at startup |
| `framework/bootstrap/ductus.md`, `framework/bootstrap/govern.md` | Modify | §Session state names the per-process targets (review) |
| `runtime/tests/common/mod.rs` and the tests that spawn the binary | Modify | One shared `ductus_command` that clears the identity variables (review) |

## Trade-offs

- **One shared file with a table per identity: rejected.** Every write would
  read, modify and replace state every session shares. It would need the same
  lock anyway, and one malformed file would take every session down. Per-file
  confinement is `BE-RACE-002`'s preferred form.
- **Identity passed as an argument by the host: rejected.** The markdown-only
  host cannot read its environment. On the MCP path the model would have to
  fetch and pass the identity on every call, a diligence dependency.
- **A locking crate (`fs2`, `fd-lock`) or a lock file created with
  `create_new`: rejected.** The crate adds a dependency for what the standard
  library now provides. A `create_new` lock file needs stale-lock recovery when
  a process dies holding it, whereas an advisory lock is released by the OS.
  The cost is the MSRV bump to 1.89.
- **A side-effect-free `resolve-session`: rejected.** Pinning, once-only
  notices and use-based expiry are all state changes on read. Accepted cost:
  one atomic write per resolution to a gitignored file.
- **Folding the removal sweep into `write-session`: rejected.** It would make
  the one primitive every command calls also the one that writes other
  sessions' state.
- **Leaving the default set on clear: rejected.** A restarted agent would adopt
  the target its operator had just cleared.
- **Known limitation: the platform table has one row.** Only
  `CLAUDE_CODE_SESSION_ID` is verified. Augment, OpenCode and Antigravity
  sessions get isolation only through `DUCTUS_SESSION` until their variables
  are verified and added.
- **Known limitation: legacy layouts get no per-process targets.** A
  pre-migration project behaves as unidentified until `/ductus` migrates it.
- **Known limitation: notices arrive only through resolution.** A session that
  never resolves its target again, because it runs no targeted command, never
  sees them. They wait in its file until it does, or until it expires.
- **Known limitation: idle expiry cannot tell idle from dead.** A session idle
  longer than seven days loses its target and adopts the default when it
  resumes. The adoption is announced, but it may not name the feature the
  session was on.
