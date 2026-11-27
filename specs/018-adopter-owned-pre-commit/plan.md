# 018 — Adopter-Owned Pre-Commit Plan

Implements [018 — Adopter-Owned Pre-Commit](spec.md).

## Overview

Three on-disk changes to `framework/bootstrap/hooks/`, one substantive rewrite of `framework/bootstrap/ductus-procedure.md` §Hook Installation plus `framework/bootstrap/ductus.md` §Shared Files, and a signpost block on spec 017. The work is plumbing — no new behavior the framework didn't already do, just a different ownership model for the hook file. Risk surface is the detection ladder's precondition for a wired outcome, which skips wiring a `.githooks/pre-commit` that does not invoke the inner hook. No data model, no event contracts, no error codes.

## Technical Decisions

### Rename, don't recreate, the inner file

`framework/bootstrap/hooks/ductus-pre-commit` is the former `framework/bootstrap/hooks/pre-commit`, moved with `git mv` rather than copied and deleted, so git blame and `git log --follow` carry across. It keeps the sentinel on line 2, runs the frontmatter derivations (`ductus derive-dependencies` and `derive-references`), and stages their output.

### Outer stub is a fresh file

The new `framework/bootstrap/hooks/pre-commit` is written from scratch as the adopter-owned outer stub. Content per `spec.md` §Design > Outer file. This file replaces the renamed one at the same source path; from git's perspective, it's a delete+add (because content differs significantly from the renamed file). Acceptable — the file is fundamentally a different artifact under the new model, and adopter projects don't have history on it anyway.

### Hook Installation rewrite

Two substantive edits to the section, each motivated by the spec:

1. **Detection ladder.** **§Hook Installation** in `framework/bootstrap/ductus-procedure.md` has five cases: already wired, the `.githooks` value a pre-059 run wrote from a project in a repository subdirectory (rewired), a custom hooks path, a third-party hook system, and no conflicts (fresh install). Every third-party hook system (`.husky/`, `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook-local.yml`) is one branch with skip-and-warn behavior. No case detects `.githooks/pre-commit` by sentinel — under this model it is never ductus-managed, so its presence says nothing about ductus.

2. **Inline `core.hooksPath` + `chmod +x`.** `framework/bootstrap/hooks/install.sh` is deleted; its two actions move into `framework/bootstrap/ductus-procedure.md`'s §Hook Installation section's fresh-install path: `git config core.hooksPath {P}.githooks` (`.githooks` at the repository root) and `chmod +x` on the hook files. Both are idempotent on re-runs.

### Manual integration snippet path change

The snippet (printed when the detection ladder hits a third-party hook system or non-`.githooks` `core.hooksPath`) changes from `./.githooks/pre-commit` to `./.githooks/ductus-pre-commit`. Two callsites: the prose snippet in `framework/bootstrap/ductus-procedure.md` §Hook Installation > Manual integration snippet, and the post-scaffolding output's per-condition skip message. Both update. The reason: a project whose own hook runner, not `core.hooksPath`, runs its hooks must call the file ductus keeps current, and the inner hook is the one `/ductus` updates on every run and is safe to call from another runner, while the outer stub is the adopter's own (**§Manual integration snippet** in `specs/018-adopter-owned-pre-commit/spec.md`).

### Spec 017 signpost

Inserted as a top-of-file block-quote, after the H1 and before the lead paragraph. Markdown form:

```markdown
> **Signpost (post-018):** The adopter pre-commit hook design described in
> this spec was superseded by [018-adopter-owned-pre-commit](../018-adopter-owned-pre-commit/spec.md).
> The shipped file at `framework/bootstrap/hooks/pre-commit` is now an
> adopter-owned outer stub written via `create` strategy, not a ductus-owned
> hook updated via `update` strategy. The ductus-owned generator orchestration
> moved to `framework/bootstrap/hooks/ductus-pre-commit`. `install.sh` is
> deleted; install actions inline in `ductus-procedure.md` §Hook Installation. AC21–AC23
> reflect the pre-018 design.
```

The 017 body, ACs, and resolved-question entries are not edited. The block-quote form makes the signpost visually distinct from 017's normal content.

### Verification approach

The end-to-end ACs (AC9, AC11) require a real `/ductus` run against a sandbox project to verify the fresh-install path. The verification is manual — run `/ductus` against a temp directory (a clean repo with no hooks) and inspect the resulting file layout. The ductus repo's own CI does not exercise this code path (`.github/workflows/generators.yml` runs the generators and fails on a changed tree, and no workflow boots a fresh adopter project: framework-checks' `scripts/tests/test-install.sh` tests only that `install.sh` writes the bootstrap command). Manual sandbox verification is the discharge criterion for those ACs.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/hooks/pre-commit` | Rename → `ductus-pre-commit`, then re-create at original path with new content | Old contents become the inner file (ductus-owned); new contents at the same path become the outer stub (adopter-owned) |
| `framework/bootstrap/hooks/ductus-pre-commit` | Create (via rename) | Ductus-owned generator orchestration; sentinel on line 2 preserved |
| `framework/bootstrap/hooks/install.sh` | Delete | Replaced by inlined actions in §Hook Installation |
| `framework/bootstrap/ductus.md` | Modify | §Shared Files manifest (split 1 row → 2) |
| `framework/bootstrap/ductus-procedure.md` | Modify | §Hook Installation (rewrite ladder, inline `core.hooksPath` + `chmod`); update Manual integration snippet path |
| `specs/017-derive-dont-ask/spec.md` | Modify (signpost only) | Insert block-quote signpost after the H1; do not touch body, ACs, or resolved questions |
| `specs/018-adopter-owned-pre-commit/plan.md` | Create | This file |
| `specs/018-adopter-owned-pre-commit/tasks.md` | Create | Task breakdown |

The CI workflow at `.github/workflows/` (if present) is not touched — CI runs the generators and fails on a changed tree (`.github/workflows/generators.yml`), and the rename is byte-identical for the inner file, so CI sees no diff.

## Trade-offs

- **Two files for adopters to inspect.** When debugging a failed pre-commit, the adopter has to know the wiring goes outer → inner. Mitigated by the comment blocks on each file naming the ownership boundary, and by the manual-integration snippet pointing at the inner file as the public entry point.
- **No migration from a spec-017 hook.** No release shipped a spec-017 hook on its own (`spec.md` §Migration), and a project that still holds one gets the skip report of `framework/bootstrap/ductus-procedure.md` §Hook Installation, with the manual integration snippet.
- **`install.sh` deletion drops a (theoretical) manual reinstall path.** Adopters who deleted their hooks and don't want to re-run `/ductus` would have lost a single command (`bash framework/bootstrap/hooks/install.sh`). The replacement is two commands (`git config core.hooksPath .githooks; chmod +x .githooks/*`). This is fine on an assumption: that no adopter re-runs `install.sh` standalone. An adopter's history is out of reach from this repository, so it cannot be checked here, and no report has named the path.
- **The outer file's `set -euo pipefail` propagates the inner file's failures unchanged.** A failing generator in the inner file still aborts the commit, which is the desired behavior. The trade-off this avoids: trying to make the outer file "smart" about partial failures (e.g., letting the adopter's checks run even when ductus's checks fail). Out of scope and would invert the ownership model.
