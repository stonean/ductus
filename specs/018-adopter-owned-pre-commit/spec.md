---
status: done
dependencies: [017-derive-dont-ask]
next-criterion: 15
---

# 018 — Adopter-Owned Pre-Commit

> **Signpost (post-059):** [059-project-in-a-repository-subdirectory](../059-project-in-a-repository-subdirectory/spec.md)
> made the hooks work for a project in a subdirectory of its repository. The
> outer stub invokes the inner hook by its own location rather than from the
> repository root, and `/ductus` sets `core.hooksPath` to the hooks directory
> named from the git work tree, `{P}.githooks`, with a ladder case that rewires
> the `.githooks` value an earlier `/ductus` wrote from a subdirectory. At the
> repository root `{P}` is empty and every path in this spec is unchanged.
> AC2, AC5, AC6, AC9 and AC12 state the subdirectory form.
>
> **Signpost:** [084 — Reopen for owed code, review every code change](../084-reopen-for-owed-code-review-every-code-change/spec.md)
> shipped a pre-push hook pair beside this spec's pre-commit pair, split the
> same way: `.githooks/ductus-pre-push`, which `/ductus` overwrites unless it
> is pinned, and the `.githooks/pre-push` stub it installs once, which calls
> it. For each ref a push updates, the hook runs `check-unreviewed-code` at
> the commit it pushes and exits as the check does: blocking by default, and
> warning under `[review] unreviewed-push = "warn"`.

Split the adopter pre-commit hook into two files so `/ductus` can keep its generators in sync without ever overwriting code the adopter added to their own pre-commit hook.

## Problem

Spec 017 shipped `framework/bootstrap/hooks/pre-commit` to adopters at `.githooks/pre-commit` with `update` strategy and a sentinel comment on line 2. On every `/ductus` run, ductus overwrites that file in full whenever the sentinel is present (see [017-derive-dont-ask](../017-derive-dont-ask/spec.md) §Generators and Hooks; item 6 of the §Hook Installation ladder as it stood before this spec, a section now in `framework/bootstrap/ductus-procedure.md`).

The hook is the natural place for adopters to wire in their own pre-commit logic — lint, type-check, test, format, license-header check. Today, any line an adopter adds to `.githooks/pre-commit` is silently destroyed by the next `/ductus`. The framework can't tell adopter-added lines from its own and treats the whole file as ductus-owned.

The fix: ductus should not own `.githooks/pre-commit` at all. It owns a separate inner file the outer hook invokes. The outer hook is created on first run and never overwritten thereafter — adopters edit it freely.

## Design

### File layout

| Path | Owner | Strategy | Contents |
| --- | --- | --- | --- |
| `.githooks/pre-commit` | Adopter | `create` (first run only) | Stub script that invokes the inner hook; adopter adds their own lines around the invocation |
| `.githooks/ductus-pre-commit` | ductus | `update` | The derivation orchestration that ships with the framework, `framework/bootstrap/hooks/ductus-pre-commit` |

`/ductus` only ever overwrites the inner file. The outer file is created on first run and skipped on every subsequent run, like every other `create`-strategy file in the manifest.

### Outer file (initial content)

The initial content is `framework/bootstrap/hooks/pre-commit`, the file this spec made the adopter-owned stub: a top comment saying the file is the adopter's and where to add their own checks, then the invocation of the inner hook under a comment saying that file is ductus-owned.

No `# managed-by: ductus` sentinel — this file is not managed by ductus after creation. The two comment blocks signal the ownership boundaries (this file is adopter-owned; the inner file is ductus-owned) so an adopter opening it for the first time knows where edits will persist.

### Inner file (ductus-owned)

The inner file's body is what `framework/bootstrap/hooks/pre-commit` shipped before this spec: run the adopter-relevant generators, then `git add` their outputs. The `# managed-by: ductus` sentinel stays on line 2 of the inner file as the marker that ductus owns it.

### Hook Installation logic

The detection ladder lives in `framework/bootstrap/ductus-procedure.md` §Hook Installation, which is the current ladder; this spec does not restate it. The ladder's cases are: already wired, a custom hooks directory, a third-party hook system (`.husky/`, `.pre-commit-config.yaml`, `lefthook.yml`, or `lefthook-local.yml`), and no conflicts, which sets `core.hooksPath`, plus the case from spec 059 that rewires the `.githooks` value an earlier `/ductus` wrote from a subdirectory. The manifest passes write the inner file (`update`) and the outer file (`create`) whichever case applies. No case detects the outer file by sentinel, because ductus doesn't own it. What decides whether the hook is wired is instead whether the outer file invokes the inner one: an outer file that does not is a hook of the project's own, which the `create` pass leaves in place, and wiring it would run that hook and never ductus's passes, so `/ductus` skips wiring with the manual integration snippet, as it does for a third-party hook system. A spec-017 hook gets no migration (§Migration below): the precondition for a wired outcome in §Hook Installation skips wiring it.

### Manual integration snippet

When detection skips wiring, `/ductus` reports the manual integration snippet in `framework/bootstrap/ductus-procedure.md` §Hook Installation. This spec changed the hook it names from the outer file to the inner one: from `./.githooks/pre-commit` to `./.githooks/ductus-pre-commit`, which is safe to call from another hook runner.

### Pinning

`.ductus/config.toml` `pinned.files` continues to apply. Pinning the inner file (`.githooks/ductus-pre-commit`) freezes it at the pinned version — `/ductus` will not overwrite. Pinning the outer file is a no-op (it's already `create`-strategy and never overwritten after first run); listing it in `pinned.files` is harmless.

## Migration

Retired on 2026-11-18 (owner decision). This spec shipped a migration that split a spec-017 `.githooks/pre-commit` into the inner and outer files. No release shipped that hook on its own: v0.1.0 and v0.2.0 predate it, and the first tag after it already carries this spec's split (`git merge-base --is-ancestor` puts `007b5960`, which added the hook, in neither v0.1.0 nor v0.2.0, and both it and `ff4eae28` in v0.3.0). Every copy carries the sentinel of the project's earlier name, which no `/ductus` trigger has matched since the rename to ductus (`git show 007b5960` gives the hook `# managed-by: govern`, and `git grep 'managed-by: govern' -- framework/` finds nothing that reads it). A project that still holds one is told so by the wired-outcome precondition of `framework/bootstrap/ductus-procedure.md` §Hook Installation (AC14): `/ductus` leaves `core.hooksPath` as it found it and reports the skip with the manual integration snippet.

## Affected Surfaces

- `framework/bootstrap/hooks/pre-commit` — split into two files. Its contents before this spec became `framework/bootstrap/hooks/ductus-pre-commit`, and the adopter-owned outer file's initial content took its place at the same path.
- `framework/bootstrap/hooks/install.sh` — deleted. The two install actions (`git config core.hooksPath .githooks` and `chmod +x` on both hook files) are inlined into §Hook Installation, now in `framework/bootstrap/ductus-procedure.md`. The conflict-detection logic the script duplicated already lives in `/ductus`'s detection ladder, and a manual `bash install.sh` would not regenerate the outer file (only `/ductus` writes it via the manifest), so the satellite script's only remaining role goes away.
- `framework/bootstrap/ductus-procedure.md` §Hook Installation — rewrite the detection ladder per §Design above; update the manual integration snippet path.
- `framework/bootstrap/ductus.md` §Shared Files — replace the single `framework/bootstrap/hooks/pre-commit` → `.githooks/pre-commit` (`update`) row with two rows: inner file (`update`) and outer file (`create`).
- ductus's own `.githooks/pre-commit` — unaffected. This is the dogfood repo's hook; it lives under a different ownership model (ductus's own repo, edited by maintainers).
- CI safety net (017 AC24) — unaffected. The dry-run check still exercises the same generators; the indirection through the new outer file does not change what runs.

## Edge Cases

- **Executable bits on the new files.** The manifest passes copy each hook's mode from its source, which git tracks as executable (`mirror_source_mode` in `runtime/src/primitives/apply_manifest.rs`; both hooks are `100755` in `git ls-files -s framework/bootstrap/hooks/`). After the manifest passes complete, §Hook Installation in `framework/bootstrap/ductus-procedure.md` still runs `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit` once, and every later `/ductus` run re-chmods (idempotent), which covers an adopter's git config or worktree that dropped the executable bit.

- **Outer file deleted, inner file remains.** Adopter or tooling removed `.githooks/pre-commit` while leaving `.githooks/ductus-pre-commit` in place. With `core.hooksPath` still set, git silently skips the missing pre-commit hook on every commit and generators stop running, since git runs only the executable hooks present in that directory (`githooks(5)`). The next `/ductus` run's `create`-strategy pass detects the missing destination and rewrites the outer stub, restoring the wiring. No new code path needed; this falls out of `create` strategy's normal "skip if present, write if missing" semantics.

- **Signpost-link pollution of predecessor's `dependencies:`.** The signpost block introduced by AC10 contains an inline markdown link from 017 to 018 (the convention for forward-pointing signposts on done specs). Were it read as a dependency, `derive-dependencies` would add that link's target to 017's `dependencies:` frontmatter on the first commit after the signpost is inserted — wrong, because 017 was implemented before 018 existed and cannot depend on it. AC13 holds this at the framework level: the primitive skips blockquote-prefixed lines when scanning for sibling-spec links, so any forward-pointer inside a blockquote is excluded by construction. The fix is general — every future signpost benefits — and removes a discipline trap that would otherwise require authors to remember a special non-link form for signpost references. Empirical verification (`15c9ad04`, whose message lists each spec's shed dependencies): six existing done specs (000, 003, 006, 007, 008, 011) carry retroactively-added signpost-style blockquotes pointing at later specs that superseded or renamed something in the predecessor, and the blockquote skip corrected the frontmatter on all six.

## Acceptance Criteria

- [x] AC1: `framework/bootstrap/hooks/ductus-pre-commit` ships with the framework, contains the `# managed-by: ductus` sentinel on line 2, and runs the frontmatter derivation primitives `ductus derive-dependencies` and `ductus derive-references` plus the existing `git add` staging
- [x] AC2: A second shipped file, `framework/bootstrap/hooks/pre-commit`, holds the initial content for the adopter-owned outer hook: it invokes the inner hook beside it, found from its own location (`"$(dirname "${BASH_SOURCE[0]}")"`) rather than from the repository root, which is the project's own inner hook whether the project is at the repository root or in a subdirectory, and contains no `# managed-by: ductus` sentinel
- [x] AC3: `framework/bootstrap/ductus.md` §Shared Files manifest lists `framework/bootstrap/hooks/ductus-pre-commit` → `.githooks/ductus-pre-commit` with `update` strategy
- [x] AC4: `framework/bootstrap/ductus.md` §Shared Files manifest lists the new outer-hook source → `.githooks/pre-commit` with `create` strategy
- [x] AC5: `framework/bootstrap/ductus-procedure.md` §Hook Installation detection ladder names the hooks directory from the git work tree and has the cases in §Design above: already wired, a custom hooks directory, a third-party hook system, no conflicts, and one that rewires the `.githooks` value an earlier `/ductus` run wrote from a subdirectory, which never matches for a project at the repository root; no case detects the outer file by sentinel
- [x] AC6: `framework/bootstrap/ductus-procedure.md` §Hook Installation manual integration snippet references `./{P}.githooks/ductus-pre-commit`, not the outer `pre-commit`: it names the inner hook from the work tree's root, where hook runners run, so `{P}`, the project's path from the work tree, is empty for a project at the repository root and precedes `.githooks` for one in a subdirectory
- [x] AC8: `/ductus` runs after the first leave the outer file `.githooks/pre-commit` untouched even if the adopter added lines to it, since its `create` strategy never rewrites an existing file
- [x] AC9: A `/ductus` run on a fresh project (no existing hook) produces both files with executable bits set, sets `core.hooksPath` to `{P}.githooks`, the hooks directory named from the work tree (`.githooks` for a project at the repository root), and leaves only the inner file (`.githooks/ductus-pre-commit`) carrying the `# managed-by: ductus` sentinel
- [x] AC10: Spec 017 carries a signpost block immediately after its H1 (before the lead paragraph) pointing at 018, naming the changed surfaces (the adopter pre-commit ownership model; `framework/bootstrap/hooks/pre-commit` strategy; the new `ductus-pre-commit` inner file; the `install.sh` deletion; AC21–AC23)
- [x] AC11: `/ductus` end-to-end run executed against a sandbox adopter directory (fresh-install case) produces the file layout described in AC9 with no manual intervention
- [x] AC12: `framework/bootstrap/hooks/install.sh` is deleted; its install actions (`git config core.hooksPath {P}.githooks`, which is `.githooks` at the repository root, and `chmod +x` on the hook files) are inlined into `framework/bootstrap/ductus-procedure.md` §Hook Installation; no other artifact references the deleted file
- [x] AC13: `derive-dependencies` excludes blockquote-prefixed lines when extracting sibling-spec links from spec bodies. Signpost-style references inside a blockquote do not pollute the predecessor spec's `dependencies:` frontmatter: 017's `dependencies` stays `[]` with the signpost, and a forward-pointer to a spec implemented later is a signpost, not an implement-time dependency
- [x] AC14: `/ductus` reports the hook installed or already wired only when a non-comment line of `.githooks/pre-commit` names `ductus-pre-commit`; otherwise it does not wire, leaves `core.hooksPath` as it found it, and reports the skip with the manual integration snippet

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Q1 (inner file name):** `.githooks/ductus-pre-commit`. The `ductus-` prefix matches existing convention (`.ductus/config.toml`, `# managed-by: ductus`) and reads naturally as "the ductus hook called by pre-commit." The alternative `.githooks/pre-commit-ductus` would sort adjacent to `pre-commit` in `ls`, but the `.githooks/` directory rarely holds enough files for sort order to matter and naming for convention beats naming for sort.
- **Q2 (outer file initial content):** Keep the friendly version with two comment blocks. The hook fires for every contributor on every commit; anyone opening the file should be able to tell immediately that their edits persist and where to add steps. Since `create` strategy writes the file once, the "noise after customization" cost is bounded — adopters can delete the comments if they're noisy, or keep them since the boundaries they describe stay accurate. Concretely, the initial content has (1) a top comment explaining the file is adopter-owned and where to add checks, (2) the inner hook's invocation, prefixed by a comment explaining that file is ductus-owned. No decorative comments beyond those two blocks.
- **Q3 (`install.sh` retention):** Delete `framework/bootstrap/hooks/install.sh`. The script had a useful semantic role under the spec-017 design (a single point that "installed" the hook by setting `core.hooksPath` and chmodding the file), but under the split-file design that role collapses: install is no longer separable from the manifest pass because the outer file is `create`-strategy and only `/ductus` writes it via the manifest. A manual `bash install.sh` from a fresh clone could not regenerate the outer file. The two install actions (`git config core.hooksPath .githooks` and `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit`) are inlined into §Hook Installation, now in `framework/bootstrap/ductus-procedure.md`. The conflict-detection logic the script duplicated already lives in `/ductus`'s detection ladder before invocation, so nothing is lost.
- **Q4 (migration detection scope):** No migration exists (§Migration), so no sentinel check decides anything: a spec-017 hook is never renamed, whatever its line 2 says. The wired-outcome precondition of `framework/bootstrap/ductus-procedure.md` §Hook Installation skips wiring it, the manual integration warning fires, and the new `.githooks/ductus-pre-commit` is still installed via the manifest. The bias is toward a one-time warning and manual step over auto-renaming a customized file, which destroys work and is the exact case this spec exists to prevent.
- **Q5 (017 signpost):** A single signpost block at the top of `specs/017-derive-dont-ask/spec.md`, immediately after the H1 and before the lead paragraph. The block links to 018, names the changed surfaces (the adopter pre-commit ownership model; `framework/bootstrap/hooks/pre-commit` is adopter-owned `create`-strategy, not ductus-owned `update`-strategy; `framework/bootstrap/hooks/ductus-pre-commit` holds the ductus-owned generator orchestration; `install.sh` is deleted; 017's AC21–AC23 reflect the pre-018 design).
- **Q6 (CI grace pass):** None. ductus's own repo `.githooks/pre-commit` is the maintainer file at the ductus repo root, untouched by this spec, and CI generators don't span the hook layout: generator dry-runs in CI only watch generator outputs, which the hook split does not change.
- **Why split from derive-dont-ask:** `017-derive-dont-ask` delivered the frontmatter derivations and the hook that runs them, and was done before this spec existed. This spec changed who owns that hook. Folding it into 017 would put the ownership split beside criteria that describe the hook as 017 shipped it, so 017 carries a signpost to this spec instead (AC10).
