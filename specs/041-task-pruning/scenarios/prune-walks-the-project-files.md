---
section: "Behavior"
---

# Prune-walks-the-project-files

## Context

[075](../../075-agents-md-holds-what-every-session-needs/spec.md) defines what `AGENTS.md`, `system.md`, `events.md` and `errors.md` hold and where everything else goes, and names `/{project}:prune` as the command that reduces them. Until now prune reduced one spec directory at a time. Its plan-section moves also sent project-only knowledge into `AGENTS.md` by default, so the command that reduced spec artifacts grew the project's largest file.

## Behavior

- **`--project`.** `/{project}:prune --project` walks the four project files and needs no session target. It is refused with `--all`, `--reset` or `--force`, as `--all` refuses `--force`. `--all` itself is unchanged.
- **`prune-project`.** A new primitive does the deterministic half, as `prune-plan` does for a plan.
  - Its preview segments each file into entries. Per file it returns the state (`present`, `absent`, `unreadable`), the bytes, and whether the file is over `[artifacts] read-size-bytes`. Per entry it returns the file, enclosing `##` heading, line range, kind, bytes and digest, never the entry's text.
  - Its apply removes or replaces listed entries by digest, and refuses the whole write when any listed entry changed since the preview. It can create a missing `system.md`, `events.md` or `errors.md` from the template compiled into it. Writes are atomic and leave no backup.
- **Entries.** An entry is a heading section at `###` or deeper, or, inside a `##` section's own body, a top-level list item with its nested lines, a table body row, or another blank-line-separated block. HTML comments and the `ductus:memory-routing` and `ductus:constitutions` regions are never entries, and `##` headings are structure. `data-model.md` records the grammar.
- **Catalog entries.** For an `events.md` or `errors.md` entry, the preview gives its identifier (the first code span of its heading, or of its table row's first cell) and the literal search behind any removal proposal: the files searched (tracked files outside the spec root) and each one containing the identifier. The walk shows that search, and no hits reads *no literal occurrence found*.
- **The walk.** For each entry, the host proposes a home from 075's table and the operator confirms. The host writes the home, then calls apply, the order step 8 uses for a plan section.
  - A procedure goes to `docs/` or a README beside the code. A skill is named, never written.
  - A one-feature decision goes to that spec's Resolved Questions, or to an existing scenario the operator names, never a new scenario. Editing a `done` spec reopens it, and the reopen is named before the write.
  - An organization rule is named with its shared constitution's `repo`, and leaves `AGENTS.md` only once the operator confirms it has landed there.
  - Nothing is written outside the project.
- **Every-adopter entries.** One that the installed framework (`.ductus/constitution.md`, `.ductus/reference/`, the runtime) already states becomes a pointer to that section. One no longer true at the installed version is deleted with its reason. One still true stays as one line, and the run's report carries a drafted upstream report with the project's names removed. Prune files nothing upstream.
- **Scope.** Reading source for the search, and writing a confirmed entry into its home (a code comment included), widen the command's Scope Boundaries, which state both.
- **Plan sections.** Step 8 routes a plan section's project-only knowledge by the same definitions, and no longer names `AGENTS.md` as its default home.
- **Tests.** Runtime tests cover segmentation, digests, stale refusal, home creation and the literal search. A recorded `claude -p` run of `/{project}:prune --project` over a fixture project shows the proposals 075's AC6, AC11 and AC12 describe. Each test fails with its behavior removed.

## Edge Cases

- A missing project file is reported as absent, never as clean.
- An entry that changed between preview and move is not moved; the walk previews it again.
- A project with no `system.md`: prune names the home, and creates it from the template only on confirmation.
- An entry whose home is in another repository is named, and nothing is written there.
- `specs/inbox.md` is not a subject.
- On the markdown-only path, a missing home is created with its `#` heading only, because an adopter's tree holds no copy of the project templates.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
