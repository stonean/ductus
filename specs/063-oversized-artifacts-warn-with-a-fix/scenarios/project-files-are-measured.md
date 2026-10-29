---
section: "The check"
---

# Project-files-are-measured

## Context

[075](../../075-agents-md-holds-what-every-session-needs/spec.md) makes `[artifacts] read-size-bytes` the limit for four project-level files too: `AGENTS.md`, and `system.md`, `events.md` and `errors.md` under the spec root. It asks this check to measure them for the shipped pre-commit hook and for `/{project}:analyze`. Until now the check's subjects were one feature's spec artifacts only, so nothing measured the file every session loads.

## Behavior

- **A second subject set.** `project-files` measures the four files against the same threshold. `feature` becomes optional, and a call naming neither is an argument error. A feature-only call returns what it returns today.
- **The result.** `threshold`, `examined`, `oversized`, `skipped` and `notices` stay at the top level. The spec keys (`feature`, `status`, `decisions`, `path`) and a `project-files` list are two halves that share no key. Each `project-files` record names the file, its state (`measured`, `absent` or `unreadable`) and its bytes.
- **The fix.** A project file over the threshold joins `oversized` under its own kind. Its one fix is `/{project}:prune --project`, it carries no `on-done`, and its warning and message are worded as a spec subject's are.
- **Absent is reported.** A project file that does not exist has state `absent`, though a missing spec subject is never reported: a project with no `events.md` must read differently from one whose `events.md` was measured. A file that exists but cannot be read goes in `skipped` under `artifact-unreadable`.
- **Staged mode.** `staged` needs `project-files` and excludes `feature`. It measures each file's index copy and its `HEAD` copy, adding `head-bytes` and `refused`. A file is refused when its staged copy is over the threshold and either has no `HEAD` copy or is larger than it. The CLI exits 1 when any file is refused or unreadable, and names each one's path, size, threshold and fix on stderr. Blob paths go through `ProjectRepository::discover`, so a project in a repository subdirectory reads the right blobs.
- **`/{project}:analyze`.** Step 17 passes `project-files` with the feature, and the exec walker seeds it for `analyze`. Each oversized project file becomes an advisory `artifact-size` finding naming `/{project}:prune --project`, and a stored discard in that feature's `analysis.md` decides it as it decides a spec subject. `/{project}:clarify` and `/{project}:plan` keep the feature-only call.
- `data-model.md` and `framework/reference/analyze.md` §Artifact size record the halves, the record's fields, and the new kind and fix.
- Tests prove that one byte over is reported, the threshold itself is not, and absent and unreadable each get their own result. Each test fails with its behavior removed.

## Edge Cases

- A commit that deletes a project file leaves no staged copy, so the file is `absent` and not refused.
- A commit that leaves an over-threshold file unchanged or smaller is not refused.
- In a repository with no commits, no staged file has a `HEAD` copy, so one over the threshold is refused.
- This lands on `main` no earlier than `041-task-pruning`'s `/{project}:prune --project`, which every message here names.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
