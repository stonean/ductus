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
> AC2, AC5, AC6, AC9 and AC12 carry the narrowing.

Split the adopter pre-commit hook into two files so `/ductus` can keep its generators in sync without ever overwriting code the adopter added to their own pre-commit hook.

## Problem

Spec 017 shipped `framework/bootstrap/hooks/pre-commit` to adopters at `.githooks/pre-commit` with `update` strategy and a `# managed-by: ductus` sentinel on line 2. On every `/ductus` run, ductus overwrites that file in full whenever the sentinel is present (see [017-derive-dont-ask](../017-derive-dont-ask/spec.md) §Generators and Hooks; item 6 of the §Hook Installation ladder as it stood before this spec, a section now in `framework/bootstrap/ductus-procedure.md`).

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

The detection ladder lives in `framework/bootstrap/ductus-procedure.md` §Hook Installation, which is the current ladder; this spec does not restate it. What this spec changed there: it replaced the seven-item ladder with four cases — already wired, a custom hooks directory, a third-party hook system (`.husky/`, `.pre-commit-config.yaml`, `lefthook.yml`, or `lefthook-local.yml`), and no conflicts, which sets `core.hooksPath`. The manifest passes write the inner file (`update`) and the outer file (`create`) whichever case applies. The previous "existing `.githooks/pre-commit` from a prior `/ductus` run, detected by sentinel" branch went away — the outer file is no longer detected by sentinel because ductus doesn't own it. What decides whether the hook is wired is instead whether the outer file invokes the inner one: an outer file that does not is a hook of the project's own, which the `create` pass leaves in place, and wiring it would run that hook and never ductus's passes, so `/ductus` skips wiring with the manual integration snippet, as it does for a third-party hook system. Migration of pre-existing ductus-installed hooks is handled separately (see §Migration below).

### Manual integration snippet

When detection skips wiring, `/ductus` reports the manual integration snippet in `framework/bootstrap/ductus-procedure.md` §Hook Installation. This spec changed the hook it names from the outer file to the inner one: from `./.githooks/pre-commit` to `./.githooks/ductus-pre-commit`, which is safe to call from another hook runner.

### Pinning

`.ductus/config.toml` `pinned.files` continues to apply. Pinning the inner file (`.githooks/ductus-pre-commit`) freezes it at the pinned version — `/ductus` will not overwrite. Pinning the outer file is a no-op (it's already `create`-strategy and never overwritten after first run); listing it in `pinned.files` is harmless.

## Migration

Existing adopters who already ran `/ductus` from spec 017 have a ductus-owned `.githooks/pre-commit` with the `# managed-by: ductus` sentinel on line 2. On the first `/ductus` run after this lands:

1. Detect the sentinel on line 2 of `.githooks/pre-commit`.
2. Rename the file: `git mv .githooks/pre-commit .githooks/ductus-pre-commit` if the file is tracked, otherwise plain `mv`.
3. Apply the `update` strategy to the renamed file with the new shipped contents (which are byte-identical to the pre-rename contents for adopters who never edited theirs — so the rename is the only on-disk change for the common case).
4. Apply the `create` strategy for the new outer `.githooks/pre-commit`, writing the stub above. Because the old file has been renamed, the destination no longer exists and `create` proceeds.
5. Report `migrated pre-commit hook: .githooks/pre-commit → .githooks/ductus-pre-commit; created adopter-owned .githooks/pre-commit stub`.

If `.githooks/pre-commit` exists but does **not** carry the sentinel, leave it alone — it's an adopter file, and the detection ladder decides: it wires the hook when the file invokes the inner hook, and otherwise skips wiring with the manual integration snippet. The new layout still installs `.githooks/ductus-pre-commit` via the manifest in this case (it's the inner file, useful even when not wired).

Adopters who edited their ductus-installed pre-commit despite the sentinel: their edits live in the renamed file (now `ductus-pre-commit`). The next `/ductus` will overwrite that file with the shipped version, dropping their edits. This is the same fate those edits had under the prior design — the migration does not make things worse, and the post-migration model gives adopters a safe place (the new outer file) to put edits going forward.

## Affected Surfaces

- `framework/bootstrap/hooks/pre-commit` — split into two files. Its contents before this spec became `framework/bootstrap/hooks/ductus-pre-commit`, and the adopter-owned outer file's initial content took its place at the same path.
- `framework/bootstrap/hooks/install.sh` — deleted. The two install actions (`git config core.hooksPath .githooks` and `chmod +x` on both hook files) are inlined into §Hook Installation, now in `framework/bootstrap/ductus-procedure.md`. The conflict-detection logic the script duplicated already lives in `/ductus`'s detection ladder, and a manual `bash install.sh` would not regenerate the outer file (only `/ductus` writes it via the manifest), so the satellite script's only remaining role goes away.
- `framework/bootstrap/ductus-procedure.md` §Hook Installation — rewrite the detection ladder per §Design above; update the manual integration snippet path; add the migration step.
- `framework/bootstrap/ductus.md` §Shared Files — replace the single `framework/bootstrap/hooks/pre-commit` → `.githooks/pre-commit` (`update`) row with two rows: inner file (`update`) and outer file (`create`).
- ductus's own `.githooks/pre-commit` — unaffected. This is the dogfood repo's hook; it lives under a different ownership model (ductus's own repo, edited by maintainers).
- CI safety net (017 AC24) — unaffected. The dry-run check still exercises the same generators; the indirection through the new outer file does not change what runs.

## Edge Cases

- **Pre-existing `.githooks/ductus-pre-commit` blocks the migration rename.** A partial prior migration or an adopter's hand-created file at the inner path can pre-occupy the destination of `git mv .githooks/pre-commit .githooks/ductus-pre-commit`. The migration aborts the rename, reports `migration skipped: .githooks/ductus-pre-commit already exists; resolve manually`, and proceeds with the manifest passes regardless. The `update`-strategy pass overwrites the pre-existing inner file with the shipped contents; the old `.githooks/pre-commit` (still carrying the sentinel) is left in place but the new §Hook Installation ladder in `framework/bootstrap/ductus-procedure.md` no longer treats sentinels in the outer file as ductus-managed, so it is treated as adopter-owned going forward. The adopter resolves the duplicate manually.

- **`core.hooksPath` is unset during migration.** Possible when an adopter manually edited git config or never ran `git config core.hooksPath .githooks` (e.g., 017's `install.sh` was skipped). The detection ladder runs after the rename, and with nothing else wired its no-conflicts case sets `core.hooksPath` to the project's hooks directory — a no-op if already wired, corrective if unset — while its earlier cases handle the conflict case (`core.hooksPath` pointing elsewhere).

- **Executable bits on the new files.** `update` and `create` strategies write file content but do not chmod. After the manifest passes complete, §Hook Installation in `framework/bootstrap/ductus-procedure.md` runs `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit` once. Subsequent `/ductus` runs re-chmod (idempotent) — covers the case where an adopter's git config or worktree configuration dropped the executable bit.

- **Outer file deleted, inner file remains.** Adopter or tooling removed `.githooks/pre-commit` while leaving `.githooks/ductus-pre-commit` in place. With `core.hooksPath` still set, git silently skips the missing pre-commit hook on every commit and generators stop running. The next `/ductus` run's `create`-strategy pass detects the missing destination and rewrites the outer stub, restoring the wiring. No new code path needed; this falls out of `create` strategy's normal "skip if present, write if missing" semantics.

- **Signpost-link pollution of predecessor's `dependencies:`.** The signpost block introduced by AC10 contains an inline markdown link from 017 to 018 (the convention for forward-pointing signposts on done specs). Without a fix, `gen-spec-deps.sh` would add that link's target to 017's `dependencies:` frontmatter on the first commit after the signpost is inserted — wrong, because 017 was implemented before 018 existed and cannot depend on it. AC13 fixes this at the framework level: the generator skips block-quoted lines (`^[[:space:]]*>`) when scanning for sibling-spec links, so any forward-pointer inside a blockquote is excluded by construction. The fix is general — every future signpost benefits — and removes a discipline trap that would otherwise require authors to remember a special non-link form for signpost references. Empirical verification: six existing done specs (000, 003, 006, 007, 008, 011) carry retroactively-added signpost-style blockquotes pointing at later specs that superseded or renamed something in the predecessor. Pre-fix, those forward-pointers were treated as dependencies and rewritten into the predecessors' frontmatter on every commit. The fix corrects the frontmatter on all six on the next generator run.

- **Migration rename fails partway through.** If `git mv` succeeds but the run aborts before the manifest passes (network failure during a later step, user `^C`, etc.), the legacy file is now at `.githooks/ductus-pre-commit` and `.githooks/pre-commit` does not exist. The next `/ductus` run's detection ladder fires its already-wired case (`core.hooksPath` already names the project's hooks directory), the `update` pass is a no-op for the inner file (content matches), and the `create` pass writes the new outer stub — self-healing without operator intervention. If `git mv` itself fails (permissions, repo locked, file in use), the migration aborts the rename, reports `migration failed: could not rename .githooks/pre-commit; resolve manually` and continues with the manifest passes. The `update` strategy still installs `.githooks/ductus-pre-commit` (writing it from scratch since the destination doesn't exist); the `create` strategy sees `.githooks/pre-commit` still in place and skips. Adopter ends up with both files: the legacy sentinel'd outer and the new ductus-owned inner, idle until the outer invokes it — so the detection ladder reports the skip with the manual integration snippet rather than the hook wired (AC14). Adopter completes the migration manually by editing the outer to invoke the inner hook beside it, as the shipped stub does.

## Acceptance Criteria

- [x] AC1: `framework/bootstrap/hooks/ductus-pre-commit` ships with the framework, contains the `# managed-by: ductus` sentinel on line 2, and runs the adopter-relevant generators (currently `.ductus/scripts/gen-spec-deps.sh`) plus the existing `git add` staging — superseded by 022-deterministic-runtime: the shell generators it names were promoted to runtime primitives, so `.ductus/scripts/` no longer exists — the derivation this criterion delivered now runs as `derive-dependencies` and `derive-references`
- [x] AC2: A second shipped file (`framework/bootstrap/hooks/pre-commit`, replacing the file currently at that path) holds the initial content for the adopter-owned outer hook: invokes `./.githooks/ductus-pre-commit` and contains no `# managed-by: ductus` sentinel — narrowed by 059-project-in-a-repository-subdirectory: the stub invokes the inner hook beside it, found from its own location (`"$(dirname "${BASH_SOURCE[0]}")"`) rather than from the repository root, which is that same file for a project at the repository root and the project's own inner hook for one in a subdirectory
- [x] AC3: `framework/bootstrap/ductus.md` §Shared Files manifest lists `framework/bootstrap/hooks/ductus-pre-commit` → `.githooks/ductus-pre-commit` with `update` strategy
- [x] AC4: `framework/bootstrap/ductus.md` §Shared Files manifest lists the new outer-hook source → `.githooks/pre-commit` with `create` strategy
- [x] AC5: `framework/bootstrap/ductus-procedure.md` §Hook Installation detection ladder is rewritten to the four-item form in §Design above; the "existing `.githooks/pre-commit` from a prior `/ductus` run, detected by sentinel" branch is removed — narrowed by 059-project-in-a-repository-subdirectory: the ladder names the hooks directory from the git work tree and gained a case that rewires the `.githooks` value a `/ductus` run before 059 wrote from a subdirectory; for a project at the repository root that case never matches, and the four this spec wrote are its cases
- [x] AC6: `framework/bootstrap/ductus-procedure.md` §Hook Installation manual integration snippet references `./.githooks/ductus-pre-commit`, not `./.githooks/pre-commit` — narrowed by 059-project-in-a-repository-subdirectory: the snippet names the inner hook from the work tree's root, where hook runners run, so for a project in a subdirectory it carries the project's path, `{P}`, ahead of `.githooks`
- [x] AC7: `framework/bootstrap/ductus-procedure.md` includes a migration subsection: when `.githooks/pre-commit` exists with the `# managed-by: ductus` sentinel on line 2 and no `.githooks/ductus-pre-commit` exists, rename the file and apply the manifest passes; report the migration in the post-scaffolding summary
- [x] AC8: A `/ductus` run on an adopter project that previously installed the spec-017 hook produces: a renamed inner file at `.githooks/ductus-pre-commit` and a fresh outer `.githooks/pre-commit` stub; subsequent `/ductus` runs leave the outer file untouched even if the adopter added lines to it
- [x] AC9: A `/ductus` run on a fresh project (no existing hook) produces both files with executable bits set, sets `core.hooksPath .githooks`, and leaves only the inner file (`.githooks/ductus-pre-commit`) carrying the `# managed-by: ductus` sentinel — narrowed by 059-project-in-a-repository-subdirectory: `.githooks` is the value for a project at the repository root; a project in a subdirectory gets `{P}.githooks`, its hooks directory named from the work tree (059's AC9)
- [x] AC10: Spec 017 carries a signpost block immediately after its H1 (before the lead paragraph) pointing at 018, naming the superseded surfaces (the adopter pre-commit ownership model; `framework/bootstrap/hooks/pre-commit` strategy; the new `ductus-pre-commit` inner file; the `install.sh` deletion; AC21–AC23). The 017 body and ACs are not edited beyond the inserted signpost block, per the frozen-archaeology rule the constitution carried when 018 shipped (removed by 023's `living-specs` scenario; a body edit today takes the `done → in-progress` back-edge instead)
- [x] AC11: `/ductus` end-to-end run executed against a sandbox adopter directory (existing-install case and fresh-install case) produces the file layouts described in AC8 and AC9 with no manual intervention
- [x] AC12: `framework/bootstrap/hooks/install.sh` is deleted; its install actions (`git config core.hooksPath .githooks` and `chmod +x` on the two hook files) are inlined into `framework/bootstrap/ductus-procedure.md` §Hook Installation; no other artifact references the deleted file — narrowed by 059-project-in-a-repository-subdirectory: the value set is `{P}.githooks`, which is `.githooks` at the repository root
- [x] AC13: `.ductus/scripts/gen-spec-deps.sh` excludes block-quoted lines (lines matching `^[[:space:]]*>`) when extracting sibling-spec links from spec bodies. Signpost-style references inside a blockquote do not pollute the predecessor spec's `dependencies:` frontmatter. Effect: 017's `dependencies` returns to `[]` post-signpost, AND any pre-existing done spec whose `dependencies` was polluted by retroactively-added signpost blockquotes (000, 003, 006, 007, 008, 011) is corrected on the next generator run. The corrections remove forward-pointers to specs that were implemented later — those are signposts, not implement-time dependencies — superseded by 022-deterministic-runtime: the shell generators it names were promoted to runtime primitives, so `.ductus/scripts/` no longer exists — the derivation this criterion delivered now runs as `derive-dependencies` and `derive-references`
- [x] AC14: `/ductus` reports the hook installed or already wired only when a non-comment line of `.githooks/pre-commit` names `ductus-pre-commit`; otherwise it does not wire, leaves `core.hooksPath` as it found it, and reports the skip with the manual integration snippet

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Q1 (inner file name):** `.githooks/ductus-pre-commit`. The `ductus-` prefix matches existing convention (`.ductus/config.toml`, `# managed-by: ductus`) and reads naturally as "the ductus hook called by pre-commit." The alternative `.githooks/pre-commit-ductus` would sort adjacent to `pre-commit` in `ls`, but the `.githooks/` directory rarely holds enough files for sort order to matter and naming for convention beats naming for sort.
- **Q2 (outer file initial content):** Keep the friendly version with two comment blocks. The hook fires for every contributor on every commit; anyone opening the file should be able to tell immediately that their edits persist and where to add steps. Since `create` strategy writes the file once, the "noise after customization" cost is bounded — adopters can delete the comments if they're noisy, or keep them since the boundaries they describe stay accurate. Concretely, the initial content has (1) a top comment explaining the file is adopter-owned and where to add checks, (2) the inner hook's invocation, prefixed by a comment explaining that file is ductus-owned. No decorative comments beyond those two blocks.
- **Q3 (`install.sh` retention):** Delete `framework/bootstrap/hooks/install.sh`. The script had a useful semantic role under the spec-017 design (a single point that "installed" the hook by setting `core.hooksPath` and chmodding the file), but under the split-file design that role collapses: install is no longer separable from the manifest pass because the outer file is `create`-strategy and only `/ductus` writes it via the manifest. A manual `bash install.sh` from a fresh clone could not regenerate the outer file. The two install actions (`git config core.hooksPath .githooks` and `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit`) are inlined into §Hook Installation, now in `framework/bootstrap/ductus-procedure.md`. The conflict-detection logic the script duplicated already lives in `/ductus`'s detection ladder before invocation, so nothing is lost.
- **Q4 (migration detection scope):** Strict line-2 sentinel check. The shipped 017 hook always puts `# managed-by: ductus` on line 2; an unmodified adopter file matches. Adopters who edited the file enough to push the sentinel off line 2 (added shebang flags, prepended custom comments, etc.) are treated as adopter-owned: the migration does not auto-rename, the manual integration warning fires, and the new `.githooks/ductus-pre-commit` is still installed via the manifest. Bias is toward false negatives — they cost a one-time warning and manual step; false positives (auto-renaming a customized file) destroy work and are the exact case this spec exists to prevent.
- **Q5 (017 signpost):** A single signpost block at the top of `specs/017-derive-dont-ask/spec.md`, inserted immediately after the H1 and before the lead paragraph. The block links to 018, names the superseded surfaces (the adopter pre-commit ownership model; `framework/bootstrap/hooks/pre-commit` is now adopter-owned `create`-strategy, not ductus-owned `update`-strategy; the new `framework/bootstrap/hooks/ductus-pre-commit` holds the ductus-owned generator orchestration; `install.sh` is deleted; 017's AC21–AC23 reflect the pre-018 design). The 017 body, ACs, and resolved-question entries are not edited beyond the inserted signpost block, per the frozen-archaeology rule the constitution carried at the time (since removed by 023's `living-specs` scenario).
- **Q6 (post-migration follow-on):** No CI grace pass needed. ductus's own repo `.githooks/pre-commit` is the maintainer file at the ductus repo root, untouched by this spec; CI generators don't span the hook layout. Adopter projects: the migration is a `/ductus`-triggered commit event, not a CI event. Adopters who pull the new ductus and run CI before re-running `/ductus` keep building green against the old file (which still works). On the `/ductus` re-run, `git mv` renames the inner file (byte-identical contents for unmodified adopters) and `create` writes the new outer stub — both are explicit changes, not silent CI failures. Generator dry-runs in CI only watch generator outputs, which don't differ across the rename.
