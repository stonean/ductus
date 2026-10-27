---
status: in-progress
dependencies: [010-agent-autonomy]
cross-spec-impact: [010-agent-autonomy]
next-criterion: 26
---

# 062 — Concurrent session targets

Two or more agent processes working in the same working tree each hold their
own session target, so a target written by one never changes the feature
another acts on. An operator running a single agent keeps the single-target
behavior: the agent resumes the most recently set target as before. What it
sees differently is its session labeled, its adoption of that target
announced, and, after a restart and until the earlier session's target
expires, that earlier session named as sharing the feature.

## Motivation

The session file, `.ductus/session.toml`, held one target per working tree
([constitution §concurrent-features](../../framework/constitution.md#concurrent-features)).
Every targeted command resolved its feature from that file, and several
commands wrote it as a side effect of their own work: `/ductus:target`
explicitly, and `/ductus:specify`, `/ductus:groom`, `/ductus:amend`'s scenario
route, `/ductus:fold` and `/ductus:consolidate` along the way.

Two agent processes in one working tree therefore shared one target. When one
process wrote it, the other's next command resolved the new value and acted on
a feature its operator had never chosen. Nothing reported this, because the
target was valid, just not theirs. That alone made running two agents at once
unreliable.

[010-agent-autonomy](../010-agent-autonomy/spec.md) had declined multi-target
sessions (its Parallel milestones section and Resolved Question 5). The verdict rested on two
premises: that working on two features at once was rare, and that
`git worktree` and platform isolation already covered it. The first did not
hold: for the operator who raised this, running two or more agent processes at
once was the common case. The second carried a cost 010 did not weigh: by default
`git worktree add` refuses a branch already checked out by another worktree
(git-worktree(1), under `--force`), so under trunk-based work two worktrees
could not both be on `main` without overriding that safeguard.

010's objection still has to be answered, and this spec answers it rather than
dismissing it: with more than one target in play, which one does a command act
on?

## Behavior

### One target per agent process

Each agent process resolves and writes its own target. A target write in one
process, whether explicit or a side effect, leaves every other process's target
as it was. A command acts on the target of the process that runs it and never
on another's.

This is how 010's ambiguity is closed: a process still has exactly one target,
so "which target" has one answer inside it. Within a process the semantics are
unchanged: one target, and a pipeline that is serial within a feature. A
target keeps today's shape, a feature with an optional scenario.

### Process identity

A process is told apart by an identity carried in the environment it was
launched with, resolved in this order:

1. **`DUCTUS_SESSION`**, when the operator set it at launch.
2. **The platform's own session identity**, when the agent passes one to the
   processes it spawns. On Claude Code this is `CLAUDE_CODE_SESSION_ID`.
3. **None.** The process uses the shared default, exactly as before this spec.

Two processes launched with the same `DUCTUS_SESSION` share one target; that
is how an operator groups processes deliberately. An empty `DUCTUS_SESSION` is
treated as unset. A value holding anything other than lowercase letters,
digits and hyphens is sanitized by the rule that derives a slug from a title,
and a value that sanitizes to nothing is refused with a message naming the
variable. An identity is never used unsanitized to locate session state.

`ductus exec` inherits the environment of whatever launched it. Run from an
agent's shell tool, it carries that agent's identity and acts on that agent's
target; run from a terminal with no identity, it uses the shared default.

On the markdown-only path the host may use only its file tools
([§runtime-host-integration](../../framework/constitution.md#host-integration-for-agent-runtimes)),
so it has no sanctioned way to read its environment. That path resolves and
writes the shared default only, as before this spec, and says so. The
reduction is documented rather than silent, as the two-paths guarantee
requires; it lasts only for the window before the runtime is registered.

### The shared default

`.ductus/session.toml` remains the shared default. Every target write, in any
process, also sets it, so it always holds the most recently set target in the
working tree. A process with no identity reads and writes only the default.

An identified process that has no target of its own **adopts** the default on
its first resolution and announces the adoption. From then on only its own
writes change its target. A restarted agent, which comes back with a new
platform identity, therefore resumes the target most recently set in the
working tree, as a single agent does today. When the default is empty there is
nothing to adopt, the process has no target, and nothing is pinned.

Two processes that have no identity share the default and collide exactly as
before. Nothing detects that case: telling a second live unnamed process apart
needs a process-liveness check the framework cannot make portably.
`DUCTUS_SESSION` is the remedy.

### Idle expiry

A per-process target that has been neither resolved nor written for seven days
is removed by the next target write any process makes. A working agent keeps
its target however long it runs, because resolving counts as use. A target used
within the window at the moment of removal is never removed. No background work
runs.

### Clearing

A cleared target leaves its process with **no target**, whether the process
cleared it or a consolidation did. Targeted commands in that process stop and
ask for `/ductus:target`, as they do today. A cleared process does not adopt
the default: that would turn a clear into a re-target nobody chose.

### Two sessions on one feature

Two sessions may target the same feature. The session making the target write
is told, at write time, which other unexpired sessions also target that feature
and when each last used its target. Each of those sessions is told once, at its
next ductus command, and again only when the set of sessions sharing its
feature changes. Neither is blocked. Notice is delivered at a command and never
pushed, because ductus has no channel into an agent process that is not running
one.

### Removal strands no process

§concurrent-features requires that a command removing the targeted feature's
directory not leave the session pointing at it, and it names one bound: a
teammate's session is unreachable. Sessions held by other processes in the same
working tree are reachable, so the rule extends to all of them. A fold
re-targets every session naming the folded spec to its upstream home. A
consolidation clears every session naming the removed spec. The shared default
follows the same rule. Each affected per-process session is told at its next
command what happened to its target and which session's command did it. The
shared default is changed without a notice: every process without an identity
reads it, so there is no single reader to tell, and the acting command's report
names it among the sessions it changed.

### Concurrent writes lose nothing

Two processes writing their targets at the same moment each end up with the
target they wrote. The existing atomic-write guarantee protects one file from
a torn write, but it does not by itself prevent a lost update when two writers
read, modify and replace shared state. The same holds for idle expiry: a
removal never takes a target that was used after the removing process
examined it.

### Unreadable session state

A file holding session state that cannot be parsed is reported by name and is
never treated as absent. Treating it as absent would make an identified process
adopt the default, silently asserting a target no check established.

### Agent configuration stays shared

The per-contributor `cli-config-dir` is not per process. It stays in the shared
default, written only by `/ductus`. With two different agent CLIs running in
one working tree, the runtime reads command files from the agent `/ductus` last
recorded.

### Scope

This spec isolates the **target**, and nothing else. Two processes in one
working tree still share its files, its git index and its commits. Keeping
their edits apart is still what `git worktree` and platform isolation are for,
and the constitution keeps pointing there for it. Each worktree already has its
own session file, so the two compose.

## Acceptance Criteria

- [x] AC1: With two agent processes in one working tree, a target write in the first, whether by `/ductus:target` or as a side effect of `/ductus:specify`, `/ductus:groom`, `/ductus:amend`, `/ductus:fold` or `/ductus:consolidate`, leaves the target the second process resolves unchanged
- [x] AC2: With two processes targeting different features, every targeted command run in either process operates on that process's own feature
- [x] AC3: An operator running a single agent process with no new configuration reads and writes `.ductus/session.toml` with the keys it holds today, and no migration runs for them
- [x] AC4: Two processes writing their targets concurrently each retain the target they wrote; neither write is lost to the other
- [x] AC5: After `/ductus:fold` or `/ductus:consolidate` removes a feature directory, no session target in the working tree names that directory, and every target naming another feature still names the same feature and scenario afterward
- [x] AC6: The per-contributor `cli-config-dir` survives every target write in every process, as it does for the single session today
- [x] AC7: Every file holding session state is gitignored, both in the gitignore template the framework ships and in this repository's own gitignore
- [x] AC8: The pipeline view (`/ductus:status`) identifies the target of the process that ran it
- [x] AC12: An operator-set `DUCTUS_SESSION` takes precedence over the platform's session identity: two processes with different values hold separate targets, and two with the same value share one
- [x] AC13: On Claude Code, two agent processes launched with no `DUCTUS_SESSION` hold separate targets, each keyed by the `CLAUDE_CODE_SESSION_ID` its agent passes to its MCP server
- [x] AC14: An empty `DUCTUS_SESSION` is treated as unset; a value holding characters other than lowercase letters, digits and hyphens is sanitized by the slug rule before use; a value that sanitizes to nothing is refused with a message naming the variable
- [x] AC15: A process with no identity resolves and writes only the shared default, and two such processes share one target exactly as before this spec
- [x] AC16: Every target write in any process also sets the shared default, so an agent restarted with a new identity resolves the target most recently set in the working tree
- [x] AC17: An identified process with no target of its own adopts the shared default on its first resolution and announces the adoption, and a later target write by another process leaves the adopted target unchanged
- [x] AC18: A per-process target neither resolved nor written for seven days is removed by the next target write in any process, and a target used within that window is never removed
- [x] AC19: When a target write names a feature another unexpired session also targets, the writing session is told which sessions and when each last used its target, and each of those sessions is told once at its next ductus command; neither command is blocked
- [x] AC20: A cleared target, whether cleared by its own process or by a consolidation, leaves the process with no target: its targeted commands stop and ask for `/ductus:target`, and it does not adopt the shared default
- [x] AC21: After `/ductus:fold` removes a staging spec, every session that targeted it targets the upstream spec; after `/ductus:consolidate` removes a spec, every session that targeted it is cleared; each affected per-process session is told at its next command what happened to its target and which session's command did it, and the shared default, which has no single reader to tell, is changed without a notice and named in the acting command's report
- [x] AC22: A file holding session state that cannot be parsed is reported by name, and the process does not treat it as absent or adopt the default in its place
- [x] AC23: `ductus exec` run from an agent's shell tool resolves that agent's target, and run from a terminal with no identity it resolves the shared default
- [x] AC24: The markdown-only path states that, having no sanctioned way to read its environment, it resolves and writes the shared default only
- [x] AC10: The concurrent-features section of `framework/constitution.md` is amended to describe per-process targets in place of a single target by design, in the same change that ships the behavior
- [x] AC25: The concurrent-features section of `framework/constitution.md` keeps `git worktree` and platform isolation as the answer for isolating working-tree edits, notes that by default git refuses to check out one branch in two worktrees at once, and states the two bounds this spec leaves open: processes with no identity share one target, and with two different agent CLIs in one working tree the runtime reads command files from the agent `/ductus` last recorded **Superseded in part by `050-constitution`** (scenario `the-constitution-fits-one-read`, 2026-10-27): to fit the constitution to one read, its cut dropped the note that git refuses one branch in two worktrees by default, and the rest of this criterion holds.
- [x] AC11: `specs/010-agent-autonomy/spec.md` carries a signpost on its Parallel milestones verdict linking to this spec, discharging the declared cross-spec impact

## Applicable Rules

- `BE-RACE-001` — session state reachable from more than one process must name its synchronization mechanism
- `BE-RACE-002` — prefer confining each process's target to that process over guarding one shared target

## Open Questions

(none — all resolved; see Resolved Questions below)

## Resolved Questions

- **How does a process claim its own target?** By an identity carried in the
  environment the agent was launched with, resolved in order: an operator-set
  `DUCTUS_SESSION`; otherwise the platform's own session identity, where the
  agent passes one to the processes it spawns; otherwise none, and the process
  resolves the shared `.ductus/session.toml` as before. The environment is the
  carrier because it is set once at launch and inherited by everything the
  agent spawns, so the runtime and the host's shell see the same value without
  the model remembering to pass a name to every command, which is a diligence
  dependency §design-principles rejects. Grounded 2026-09-29: every agent
  process spawns its own `ductus mcp` server (nine running concurrently in one
  `ps` listing, each under a different parent), and Claude Code places the same
  `CLAUDE_CODE_SESSION_ID` in its MCP server's environment and in its shell
  tool's. What the other supported agents expose was not examined; an agent
  that exposes nothing still gets isolation through `DUCTUS_SESSION`.
- **What does a process resolve before it has a target of its own?** The
  shared default, which every target write also updates, so the default always
  holds the working tree's most recently set target and a new or restarted
  agent starts where the last one left off, as a single agent does today. An
  identified process **pins** the target the first time it resolves one: after
  that, only its own writes change it. Without the pin, a process that started
  on the inherited default would be moved by the next write any other process
  made, which is the defect this spec exists to remove, one step removed.
  Processes with no identity share the default and collide exactly as before.
  Nothing detects that case, because telling a second live unnamed process
  apart needs a process-liveness check the framework cannot make portably; the
  bound is stated, and `DUCTUS_SESSION` is the remedy.
- **When is a process's target removed?** When it has gone unused for seven
  days, by the next target write any process makes. Use means the target was
  resolved or written, so a working agent keeps its target however long it
  runs. Idle expiry stands in for a liveness check the framework cannot make:
  the identity is a session identifier, not a process one, and with platform
  identities isolating every agent session automatically, targets would
  otherwise accumulate without bound. No background work runs; removal rides
  on writes that happen anyway. A process may still clear its own target at
  any time. Adopting the default is **always announced**, whether on a first
  resolution or after an expiry, so a session resumed after a week that finds
  its target gone says which target it adopted rather than moving silently.
- **May two processes target the same feature?** Yes, with a notice to both
  sides. Refusing would block legitimate parallel work on one feature, such as
  implementing it in one session while reviewing, analyzing or clarifying a
  scenario of it in another. The hazards are already bounded elsewhere: status
  transitions are guarded on the expected current value, so a transition
  computed from a stale read fails loudly, and concurrent edits to the same
  files are working-tree sharing, which §Scope leaves out. The session making
  the target write is told at write time which other sessions also target that
  feature and when each last used its target. The session that was already
  there is told **once**, at its next ductus command, and again only when the
  set of sessions sharing its feature changes. It is the one that most needs
  telling, since the writer chose to join. Notice is delivered at a command
  and never pushed: ductus has no channel into an agent process that is not
  running a command. A session idle but not yet expired still counts, which is
  why the notice carries when each session last used its target.
- **On removal, is another process's target re-targeted or cleared?** The
  same rule the acting process follows, applied to every session in the
  working tree that names the removed spec. §concurrent-features grounds each
  outcome in the content, not in who ran the command: a fold moves the content
  to its upstream home, so where the work went is a fact for every session;
  a consolidation's surviving spec is one nobody chose to work on, which holds
  more strongly still for a session that did not run the command. So a fold
  re-targets every session naming the folded spec, and a consolidation clears
  every one. Each affected session is told once, at its next command, through
  the same notice as concurrent targeting: what happened to its target, and
  which session did it. **Cleared means no target, never "adopt the
  default."** Under pin-on-first-resolution a cleared session would otherwise
  pick up whatever the default holds, turning a clear into a re-target nobody
  chose; its commands instead stop and ask for `/ductus:target`, as today. The
  same holds when a process clears its own target. The shared default is
  itself cleared when it named the removed spec.
- **Is `cli-config-dir` per process as well?** No. It stays the
  per-contributor agent choice in the shared session file. It is written only
  by `/ductus`, never by a target write, and the runtime reads it
  (`runtime/src/host.rs:86`) for one purpose: choosing which agent's installed
  command files to read, in `ductus exec` (`runtime/src/main.rs:485`), in the
  interpreter payload (`runtime/src/interpreter/payload.rs:1155`) and in the
  corpus link check (`runtime/src/primitives/check_corpus_links.rs:63`). This
  spec is about targets, and the case a per-process value would serve, two
  different agent CLIs installed in one project and running at once, is
  narrower than the one that motivated it. The bound is stated: with two
  different agent CLIs in one working tree, the runtime reads command files
  from the agent `/ductus` last recorded. A later spec can key it by process
  the same way if that case bites.
- **Does the worktree guidance stay?** Yes, reframed. Per-process targets make
  one working tree shareable by several agents; `git worktree` and platform
  isolation remain the answer when their *edits* need isolating too, which
  this spec leaves shared. The two compose, since each worktree already has
  its own gitignored session file. §concurrent-features changes from "a single
  target by design, use worktrees for concurrency" to "each agent process holds
  its own target; isolate edits with worktrees or the platform when needed",
  and notes that by default git refuses to check out one branch in two worktrees at once,
  so the trade-off is explicit.

**Why split from agent-autonomy:** [010](../010-agent-autonomy/spec.md) was a
capability evaluation — it weighed GSD-2's features and declined multi-target
sessions — and owns none of the session machinery. This spec reverses that
decline and builds the runtime behavior it declined, so folding it into 010
would reopen an evaluation to carry an implementation. The routing was chosen
at `/ductus:specify` over a scenario on 010. The overlap in Affected Files is
incidental: both amend shared framework documents, the constitution and the
agents' permission lists.
