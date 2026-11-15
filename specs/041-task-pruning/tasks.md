---
next-task: 27
---

# 041 — Spec Directory Pruning Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 25. prune-project's opens are contained (BE-INPUT-004, from 075's analysis)

- [x] `prune-project`'s reads and writes of the four project files (`AGENTS.md`, and `system.md`, `events.md` and `errors.md` under the spec root), the `create_dir_all` of a created file's parent, and its search's reads of tracked files go through `validate_contained` (`runtime/src/primitives/mod.rs`) against the project root, rather than `repo.join` behind `validate_specs_root`'s lexical screen: a project file or a parent directory that is a symbolic link leaving the project is refused, and a tracked file the search would read through one is left unread and named, never read or written through. The owner deferred 075's BE-INPUT-004 (MUST) analysis finding here on 2026-11-15, since 075's plan says 041 owns `prune-project`'s design; 075's `analysis.md` records the deferral
- [x] 041's data model §`prune-project` and `PRUNE_PROJECT`'s description state what each case reports
- [x] A test with an outward link for each case (a linked `AGENTS.md`, a linked spec root, a tracked file the search reads) shows it refused or left unread, and turns red under its mutant

- **Done when**: No open prune-project makes follows a symbolic link out of the project, proven by tests their mutants turn red; the whole `cargo test` passes; and 041's next review reads the change.

## 26. prune-plan's single-feature result is bounded, and the design states both prune primitives' bounds (BE-PAYLOAD-001, from 041's analysis)

- [ ] `prune-plan`'s single-feature `sections` is held to `[artifacts] read-size-bytes` through a `lists-file`, as `prune-tasks`' single-feature `sections` and both walks already are (spec 080); `PRUNE_PLAN`'s description and every reader of the result state or read the file form
- [ ] 041's `data-model.md` (the two schema sections and §Walking every spec) and `plan.md` state both primitives' bounds; the plan no longer reads as if the lists had none
- [ ] A test proves the inline form under the read size and the file form past it for a single feature, failing under its mutant

- **Done when**: Every list `prune-tasks` and `prune-plan` return is bounded and the design says so, proven by a test its mutant turns red; the whole `cargo test` passes; and 041's next review reads the change.
