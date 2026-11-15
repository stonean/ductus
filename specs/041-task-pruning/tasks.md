---
next-task: 26
---

# 041 — Spec Directory Pruning Tasks

Tasks derived from the [plan](plan.md). Complete in order. Phase A ships the
`prune-tasks` runtime primitive; Phase B adds the `/{project}:prune` command on
top and registers it. The primitive (Phase A) must build and pass its parity
test before the command (Phase B) wires to it.

## 25. prune-project's opens are contained (BE-INPUT-004, from 075's analysis)

- [ ] `prune-project`'s reads and writes of the four project files (`AGENTS.md`, and `system.md`, `events.md` and `errors.md` under the spec root), the `create_dir_all` of a created file's parent, and its search's reads of tracked files go through `validate_contained` (`runtime/src/primitives/mod.rs`) against the project root, rather than `repo.join` behind `validate_specs_root`'s lexical screen: a project file or a parent directory that is a symbolic link leaving the project is refused, and a tracked file the search would read through one is left unread and named, never read or written through. The owner deferred 075's BE-INPUT-004 (MUST) analysis finding here on 2026-11-15, since 075's plan says 041 owns `prune-project`'s design; 075's `analysis.md` records the deferral
- [ ] 041's data model §`prune-project` and `PRUNE_PROJECT`'s description state what each case reports
- [ ] A test with an outward link for each case (a linked `AGENTS.md`, a linked spec root, a tracked file the search reads) shows it refused or left unread, and turns red under its mutant

- **Done when**: No open prune-project makes follows a symbolic link out of the project, proven by tests their mutants turn red; the whole `cargo test` passes; and 041's next review reads the change.
