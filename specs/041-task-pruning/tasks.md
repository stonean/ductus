---
next-task: 30
---

# 041 — Spec Directory Pruning Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 25. prune-project's opens are contained (BE-INPUT-004, from 075's analysis)

- [x] `prune-project`'s reads and writes of the four project files (`AGENTS.md`, and `system.md`, `events.md` and `errors.md` under the spec root), the `create_dir_all` of a created file's parent, and its search's reads of tracked files go through `validate_contained` (`runtime/src/primitives/mod.rs`) against the project root, rather than `repo.join` behind `validate_specs_root`'s lexical screen: a project file or a parent directory that is a symbolic link leaving the project is refused, and a tracked file the search would read through one is left unread and named, never read or written through. The owner deferred 075's BE-INPUT-004 (MUST) analysis finding here on 2026-11-15, since 075's plan says 041 owns `prune-project`'s design; 075's `analysis.md` records the deferral
- [x] 041's data model §`prune-project` and `PRUNE_PROJECT`'s description state what each case reports
- [x] A test with an outward link for each case (a linked `AGENTS.md`, a linked spec root, a tracked file the search reads) shows it refused or left unread, and turns red under its mutant

- **Done when**: No open prune-project makes follows a symbolic link out of the project, proven by tests their mutants turn red; the whole `cargo test` passes; and 041's next review reads the change.

## 26. prune-plan's single-feature result is bounded, and the design states both prune primitives' bounds (BE-PAYLOAD-001, from 041's analysis)

- [x] `prune-plan`'s single-feature `sections` is held to `[artifacts] read-size-bytes` through a `lists-file`, as `prune-tasks`' single-feature `sections` and both walks already are (spec 080); `PRUNE_PLAN`'s description and every reader of the result state or read the file form
- [x] 041's `data-model.md` (the two schema sections and §Walking every spec) and `plan.md` state both primitives' bounds; the plan no longer reads as if the lists had none
- [x] A test proves the inline form under the read size and the file form past it for a single feature, failing under its mutant

- **Done when**: Every list `prune-tasks` and `prune-plan` return is bounded and the design says so, proven by a test its mutant turns red; the whole `cargo test` passes; and 041's next review reads the change.

## 27. prune's applies guard their read-modify-write (BE-TXN-002, from 041's analysis; owner decision)

- [x] `prune-tasks`', `prune-plan`'s and `prune-project`'s applies guard the window between reading a file and renaming the result over it, by a compare before persist: the file is read again just before the rename, and the write is refused when it changed since the apply read it, so an `append-task`, `mark-task` or other writer landing in between is never dropped. Today each reads, computes and calls `write_atomic` with no guard (`prune_tasks.rs` `reduce`, then `Reduction::write`), and only `append-task` takes a lock (022 task 35). The guard is one shared helper in `runtime/src/primitives/mod.rs`, 022's surface, which 022's plan states. It holds one project-wide lock, taken through `session.rs`'s `lock_file_waiting`, across the re-read and the rename, so two runtime writers cannot both pass the compare; 022's plan states the lock's order beside `append-task`'s, its timeout and its hold bound (BE-LOCK-001, BE-LOCK-002), and the residual it leaves, a writer outside the runtime landing between the re-read and the rename (owner, 2026-11-16). The owner decided on 2026-11-15 to fix 041's analysis finding (BE-TXN-002, MUST) here rather than waive it by a departure entry. 041's review of this task (2026-11-16) found the guard atomic only among the writers that go through it: `append-task`, `mark-task`, `merge-managed-block` and `rewrite-spec-links` still write a prune's files without it, so one landing between the re-read and the rename is replaced. The owner deferred that BE-TXN-002 finding to 022 task 38, which puts the last of them on the guard
- [x] 041's spec Edge Cases, *Concurrent invocations*, names the guard in place of "the last writer wins" and drops the claim that every runtime primitive that rewrites a spec artifact does so; 041's data model and plan state what a refused write reports; `PRUNE_TASKS`', `PRUNE_PLAN`'s and `PRUNE_PROJECT`'s descriptions state it; and the plan's sentence that every list either prune primitive returns is held to the read size names the lists it covers, since `stale-sections` and `prune-project`'s `stale-entries`, `created` and `kept` echo the caller's own input
- [x] A test per primitive in which the file changes between the apply's read and its write shows the write refused and the other writer's change kept, each failing under its mutant

Owed after this task, in order: 041's review of its change; the pre-done criteria verification; then a full analysis, since the task edits 041's design subjects. The 2026-11-15 analysis was judged but never recorded, and its verdicts go stale with this edit. What it found carries into the next run. `BE-TXN-002` turns `addressed` once the guard lands, and `BE-PAYLOAD-001` and `BE-RACE-003` were judged `addressed`. Two step-15 grounding claims in `plan.md` want a citation in the same design edit: that other `tasks.md` readers are line scanners, and that an adopter's tree holds no copy of the project templates. Of the stored grounding decisions, all but the one on a `done` spec's artifacts being committed still stand, so carry them under their stored keys.

- **Done when**: No prune apply overwrites a change made between its read and its write, proven by tests their mutants turn red; the whole `cargo test` passes; and 041's next review reads the change.

## 28. An open fenced block or comment is not examined

- [x] Implement the behavior described in `scenarios/an-open-fenced-block-or-comment-is-not-examined.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 29. `/analyze` says what a `prune-plan` result with an `unexamined-tail` means (found while implementing task 28)

- [ ] State it where the step reads `sections`, in as few bytes as the content allows, and measure `framework/commands/analyze.md` and `framework/reference/analyze.md` before and after

- **Done when**: the step of `framework/commands/analyze.md` that reads `prune-plan`, or the reference section it cites, tells the host that a result carrying `unexamined-tail` examined only part of the plan and is reported so, never read as a plan with no section outside the design record
