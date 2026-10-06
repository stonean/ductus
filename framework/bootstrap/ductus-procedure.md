---
description: The /ductus sections that run after the framework archive is extracted.
---

# ductus — archive half

The half of the `/ductus` procedure that cannot run before the framework archive
exists. It ships **in** that archive and is read from the extracted tree at
`{framework-root}/framework/bootstrap/ductus-procedure.md`, where
`{framework-root}` is the framework root §Archive fetch and extract derives
from the extraction.

Three things follow, and each is the point of the split:

- **It is never installed.** `install.sh` does not place it, the `ductus`
  self-install step does not write it, and no manifest names it. What an adopter
  curls, installs into every agent, loads into context at every invocation, and
  byte-compares on every run is `framework/bootstrap/ductus.md` alone.
- **It is never stale.** There is no self-update check for this file and none is
  needed: it is re-fetched from the archive on every run, so an installed copy
  cannot fall behind it.
- **Nothing dispatches from here.** `## Instructions` in
  `framework/bootstrap/ductus.md` is the numbered walker for the whole run, and
  it stays installed. This file is reference prose the host reads when the run
  reaches these sections.

A run that halts in the pre-flight phase never reads this file — which is exactly
the set of sections §Pre-flight abort already skips.

**Where a named section lives.** The reverse of the rule the installed half
states: a **bolded section name** or `§name` below that has no heading in *this*
file is in `framework/bootstrap/ductus.md`, the half the adopter has installed —
§Pre-flight Phase, §Derived values, §Shared Files, §Per-Agent Scaffolding,
§Project Configuration, §Collect Project Inputs, §Archive fetch and extract and
§Pre-flight abort among them. That file is already loaded whenever this one is
read, because reaching this file means the run got past the archive fetch.

## Pre-run Migrations

Adopter-side cleanup for conventions that have been removed or renamed since the adopter's last `/ductus` run. Driven by a machine-readable registry at `framework/migrations.toml` (one `[[migrations]]` entry per active removal); per-entry procedure bodies live at `framework/migrations/{id}.md`. Spec [027 — Bootstrap Migration Registry](https://github.com/stonean/ductus/blob/main/specs/027-bootstrap-migration-registry/spec.md) defines the contract.

### Procedure

1. Read `framework/migrations.toml` from the fetched archive. If the file is missing or malformed (TOML parse error), abort with `Failed to read framework/migrations.toml; cannot run pre-run migrations.` and do not continue.
2. Resolve the **active config file** once for this run (write policy: §Project Configuration) and read its `[migrations].last_applied` (treat an absent `[migrations]` section as null). Every marker write-back in step 7 targets this same resolved file, so the read and the write-backs agree even though the config file is itself a migration target; the one step that changes the resolution mid-run is the `govern-dir-consolidate` procedure, whose config move makes `.ductus/config.toml` the active file for every later step (its procedure file says so).
3. Filter the registry to entries where both:

   - `introduced_in` is greater than `last_applied`'s `introduced_in` (SemVer comparison, lex tie-break on `id`); when `last_applied` is null, every entry qualifies.
   - Either `sunset_after` is absent, or the current ductus release version is less than `sunset_after` (SemVer comparison).

4. If the filtered list is empty, emit nothing and proceed to the next bootstrap section.
5. Otherwise, prompt once with text of the form:

   ```text
   N framework migrations are pending since your last /ductus run:
     - {id} (introduced {introduced_in})
     ...
   Apply now? (Y/n)
   ```

6. On decline, emit `warning: N migrations skipped; pipeline commands may fail on legacy artifacts until applied. Re-run /ductus to apply.` and proceed without filesystem changes.
7. On confirm, for each filtered entry in order:

   1. Read `framework/migrations/{id}.md` from the fetched archive.
   2. Execute its `## Procedure` steps. The procedure file owns idempotency (step 1 of every procedure exits silently when the target artifact is absent), per-file user prompts (when applicable), and the post-scaffolding summary line.
   3. After the procedure completes successfully, update the active config file's `[migrations].last_applied = "{id}"` atomically (tempfile + rename, matching the rest of the config file's write semantics) — the file resolved at step 2, or `.ductus/config.toml` once the `govern-dir-consolidate` procedure has moved it. The update happens **per entry**, not at end of batch — an aborted batch resumes from the next-pending entry on the following `/ductus` run.
   4. If a procedure aborts (rare — only via explicit user "stop everything" path inside the procedure file), halt the loop. The retained `last_applied` value points at the last-completed entry; the next run resumes.

8. After the loop, invoke `check-orphaned-references` (MCP: `check-orphaned-references`) once and report each finding. Each migration is authored against the layout as it stood at its own `introduced_in` and is correct there; nothing validates the **composition**, and an adopter far enough behind runs several in one batch, so the composition is what they actually execute. A later entry moving a path an earlier entry wrote into an **adopter-owned** file — `create` strategy, so the manifest never overwrites it, and unpinned, so the pinned-invoker warning never fires — leaves a reference pointing at nothing, and nothing errors: a dangling `@import` yields a constitution that is simply not loaded, and a hook calling a moved generator fails at commit time, far from the run that broke it. Report `Orphaned reference: {referrer}:{line} names {target}, which does not exist; most likely orphaned by migration {id}` — the `{id}` clause comes from the finding's `migration`, available here because the registry is in the fetched archive (the result's `attribution` reads `registry`). Findings in `skipped` are referrers that could not be read; surface them as unexamined rather than folding them into a clean count. **The batch does not halt**: the migrations that applied are correct and re-running is safe, and the adopter may have hand-edited the reference, so this reports and never repairs. Run it on **every** batch, not only multi-entry ones — a single migration can orphan a reference just as a chain can, and scoping this to chains would make it run least often in the case it was written for. When the batch applied nothing, there is nothing to verify and nothing is emitted — not a clean bill of health for files it never examined. The same primitive is `/{project}:analyze`'s durable adopter-facing surface, where it runs without the registry; see [027 — Bootstrap Migration Registry](https://github.com/stonean/ductus/blob/main/specs/027-bootstrap-migration-registry/spec.md)'s `migration-chain-reference-integrity`.

### Stale-reference behavior

If the active config file's `[migrations].last_applied` references an `id` that no longer exists in the active registry (the entry was sunsetted since the adopter's last run), treat the field as "before the oldest active entry" and run every active entry. Emit one warning: `last_applied was "{retired_id}" which has been retired; see CHANGELOG.md for its recipe.` Adopters far enough behind to hit a sunsetted entry apply it manually from `CHANGELOG.md`.

### Duplicate-id and reference-integrity guard

If `framework/migrations.toml` contains two entries with the same `id`, or if any entry's `procedure_file` references a path that doesn't exist in the fetched archive, abort the loop before applying anything with a clear error. `/audit`'s Family 10 (`scripts/audit/migration-coverage.sh`) catches these at maintainer time; this guard is the runtime safety net.

## Frontmatter Migration

If `specs/` does not exist (first run), skip this section — there is nothing to migrate.

Bring existing spec and scenario files into the YAML frontmatter format declared in `framework/constitution.md` §text-first-artifacts. Migration is idempotent: re-running on an already-migrated project produces no further metadata changes.

This section runs **after the Pre-flight Phase** so that a stale-ductus abort cannot leave migration changes from old rules on the working tree. The new ductus's migration logic — which may differ — is the only logic that ever writes migration changes.

### Precheck

Run `git status --porcelain -- specs/` (project-relative). If the output is non-empty, refuse with:

> Migration requires a clean working tree under `specs/`. Commit or stash your changes, then re-run.

Exit before any modifications. Unrelated in-flight work outside `specs/` does not block migration.

### Walk

For each file matching one of:

- `specs/**/spec.md`
- `specs/**/scenarios/*.md`

Determine whether the file needs migration:

- Read the first non-blank line of the file. If it is `---`, the file already has frontmatter — skip with reason "already frontmatter."
- Otherwise, scan the first few lines after the heading for bold-prefix metadata patterns (`**Status:**`, `**Dependencies:**`, `**spec-ref:**`). If at least one is found, the file needs migration.
- If no bold-prefix lines are present and no frontmatter exists, skip with reason "no metadata to migrate."

Skip files that appear in `.ductus/config.toml` `pinned.files` with reason "pinned." The adopter is responsible for migrating pinned files manually.

### Convert

For each file that needs migration:

**Spec files** (`spec.md`):

- Extract `**Status:** {value}` and `**Dependencies:** {value}` from the body.
- For dependencies, parse the comma-separated slug list. The literal value `none` becomes an empty list (`[]`).
- Preserve any additional bold-prefix fields the project may have added (e.g., `**Track:** lightweight` becomes `track: lightweight` under the open-schema rule).
- Construct the YAML frontmatter block:

  ```yaml
  ---
  status: {value}
  dependencies: [{slug, slug, ...}]
  ---
  ```

- Remove the bold-prefix lines from the body.
- Insert the frontmatter block at the very top of the file, with one blank line separating it from the heading.

**Scenario files** (`scenarios/{slug}.md`):

- Extract `**spec-ref:** {value}` from the body.
- Construct the YAML frontmatter block:

  ```yaml
  ---
  spec-ref: "{value}"
  ---
  ```

  Quote the `spec-ref` value because it conventionally contains an em-dash and spaces.

  **`spec-ref`, not `section`, is deliberate here — do not "correct" it.** A scenario's required field
  is `section` (§text-first-artifacts), and `spec-ref` is the legacy fallback the constitution keeps for
  pre-017 scenarios. A project still on bold-prefix metadata *is* pre-017, and its `**spec-ref:**` value
  names the parent feature *and* the section, which is what `spec-ref` means and is not what `section`
  means. Writing that value under `section` would rename the field while silently changing what it
  asserts; writing it under `spec-ref` is lossless, and every reader resolves it. The hard fail fires
  only when both keys are missing, so the migrated file is valid.

- Remove the bold-prefix line from the body.
- Insert the frontmatter block at the very top of the file, with one blank line separating it from the heading.

### Edge cases

- **Partially migrated file** (frontmatter present and bold-prefix lines also present in body): the precheck above treats this as "already frontmatter" and skips. The user may run a manual cleanup pass; the migration does not attempt mixed-state recovery.
- **Malformed bold-prefix metadata** (e.g., missing `**Status:**` line, typo in field name, unparseable value): log a warning to the summary as `skipped (malformed metadata): {file path}` with a brief reason. The user repairs manually before re-running.
- **Bold-prefix metadata with custom fields**: preserved as additional frontmatter fields under the open-schema rule.

### Summary

Print a per-file summary at the end of the migration step:

- `migrated: {file path}` for converted files
- `skipped (already frontmatter): {file path}` for files that were already in the new format
- `skipped (pinned): {file path}` for files listed in `.ductus/config.toml`
- `skipped (no metadata to migrate): {file path}` for files without recognizable metadata
- `skipped (malformed metadata): {file path} — {reason}` for files that could not be parsed

The user reviews the result via `git diff` and commits or aborts via `git restore`. No backup directory is created — git is the recovery mechanism.

## Security Audit (brownfield)

Run a one-time security audit when the project newly receives a security rule file alongside existing feature specs. This is the brownfield-adoption hook described in `specs/008-security-rules/spec.md` — it reports each gap with the spec it belongs to and **writes nothing**, so the adopter sees every gap at adoption, and each one resurfaces as a finding to disposition when `/{project}:analyze` next runs on its spec, rather than every legacy spec failing at once.

### Trigger

Run the audit only when **both** conditions hold after the **Shared Files** manifest pass has completed:

1. At least one of `specs/rules/security-backend.md` or `specs/rules/security-frontend.md` was **newly created** by the manifest pass (the destination file did not exist before this run). A file that was merely updated or unchanged does not trigger the audit.
2. The project contains at least one feature spec directory under `specs/`, in either of the two forms `.ductus/constitution.md` §numbering defines — sequential (`000-skeleton`) or branch-scoped (`1234.1-retry-budget`). Do **not** re-derive the digit convention here: §numbering states that the membership rule is defined in exactly one place and that a surface reading the spec corpus calls it rather than restating it, because a second copy is how the two forms drift apart. An earlier wording of this step said "zero-padded, three-digit prefix", which excluded every branch-scoped directory and, past 999, sequential ones too.

If either condition fails, skip this section silently — no output, no finding. This covers the two routine cases:

- **Greenfield adoption** — no feature spec directories exist under `specs/`, so the audit has nothing to scan against.
- **Routine re-run** — the rule files were created on a prior run; the manifest pass reports them as "updated" or "unchanged" rather than "created".

### Loading rule files

For each rule file that passed the trigger:

1. Read the file from its destination path (`specs/rules/security-backend.md` or `specs/rules/security-frontend.md`).
2. Apply the same integrity checks `/{project}:analyze` uses for the security-rule check section: well-formed level-3 headings of the form `### {ID}`, the three required body fields (Statement, Rationale, Verification), an ID matching `{FE|BE}-{CATEGORY}-{NNN}` — the ID is the heading, so it is checked as the heading rather than as a fourth field — and no duplicate IDs within the file. `Source` is **not** checked: the constitution's §rules names four required fields (ID, Statement, Rationale, Verification) and `Source` is not among them, and it is absent from 4 of 11 shipped rule files' rules (`concurrency-backend` 4/8, `configuration-cross` 7/11, `observability-backend` 5/7, `reliability-backend` 5/8), so requiring it would make those files unloadable. It is universal only on the two security files this section reads, which is what made the overstatement invisible.
3. If a file fails any integrity check, report `Security audit: {path} failed to load — {reason}; skipping audit for this file.` and continue with the other rule file (if applicable). Do not abort the surrounding `ductus` run.

This mirrors `/{project}:analyze`'s posture — partial or guessed-at parsing produces unreliable findings, so an unloadable file is treated as absent for audit purposes.

### Per-rule check

For each rule that loaded successfully:

1. Identify the artifacts in scope: `spec.md`, `plan.md`, and any `scenarios/*.md` under each feature spec directory in the spec root (both forms, per the trigger above).
2. Read the rule's **Verification** field. The field describes the trigger — what makes the rule applicable to a given artifact — and the commitment the artifact must include when triggered.
3. For each artifact whose content fires the rule's trigger but does not include the required commitment, produce one finding.

Rules whose Verification trigger does not fire for any artifact produce no finding (the contextual-application property — silently inert when no spec exercises the rule's surface).

### Reporting findings

The audit writes no file. Each finding is reported as one line under the spec it belongs to:

```text
{Rule ID}: {affected artifact path} does not address — {one-line summary}
```

The `{one-line summary}` describes the gap concretely (e.g., `does not name a memory-hard password hashing algorithm`, `does not specify an output encoding strategy`). Leading each line with the rule ID groups related findings and gives the adopter a stable handle for cross-referencing.

**Nothing is written to `specs/inbox.md`.** The inbox holds the todos a person logs, and no command writes a finding there, because it is the one destination no gate reads (`.ductus/constitution.md` §brownfield-inbox, Finding dispositions). A finding here names its spec, and `/{project}:analyze` applies the same rule's Verification trigger to a spec whenever it runs, so the gap comes back — as a finding that run must fix or route, since a MUST-tier gap is blocking and a blocking finding is never discarded; a SHOULD-tier gap may also be discarded with its reason — when the spec is next worked on. The accepted cost is that a legacy spec nobody touches is never re-checked; that is the brownfield stance of letting adoption spread through the areas being worked on. An adopter who wants a particular gap tracked sooner records it with `/{project}:log`.

Because nothing is written, re-triggering the audit — after deleting and re-installing a rule file — reports the same findings again and changes no file.

### Audit summary

Collect the findings by spec. **Post-Scaffolding Output** reports them; when there are none, the audit-summary block is omitted entirely.

## Hook Installation

After **Per-Agent Scaffolding** completes, manage the project's git pre-commit hook so generated artifacts (currently spec `dependencies:` and `references:` frontmatter, future generators if added) stay in sync on every commit.

Two files participate, with different ownership models:

- **`.githooks/ductus-pre-commit`** is ductus-owned. Placed by the **Shared Files** manifest with `update` strategy; carries the `# managed-by: ductus` sentinel on line 2; rewritten on every `/ductus` run unless pinned in `.ductus/config.toml`. Holds the derivation orchestration (currently `ductus derive-dependencies --write --staged` and `ductus derive-references --write --staged` plus output staging). Both run with `--staged` so a commit only rewrites the specs it touches, never unrelated ones. An unreachable runtime **halts** the commit rather than skipping the pass: these primitives produce derived frontmatter the commit captures, so a silent skip would land values they had already superseded. That is safe to make blocking because the hook cannot outrun the binary — `/ductus` wires `core.hooksPath` and acquires the runtime in the same run, and `core.hooksPath` is local git config a clone never carries, so a fresh clone has no hook until `/ductus` has run. `git commit --no-verify` is the deliberate bypass. It runs from its project root, the directory above its own, whatever directory git starts it in, and lists staged specs from there, so a project that is a subdirectory of its repository behaves as one at the repository root (spec 059). It additionally runs `ductus label-criteria` once per staged spec — the acceptance-criterion labelling pass (spec 013), the backstop for a criterion typed by hand in an editor. That step keeps a *swallowed* failure, and the reason is blast radius rather than optionality: a missing `AC{n}` label is caught by `/ductus:analyze` and assigned on the next pass, so nothing wrong is committed, while a stale derived index is committed wrong data.
- **`.githooks/pre-commit`** is adopter-owned. Placed by the manifest with `create` strategy on first install; never overwritten thereafter. Initial content invokes the inner hook beside it, found from the stub's own location rather than from the repository root; adopters add their own pre-commit checks above or below that invocation.

This section's job is to wire git up to actually run the outer hook without clobbering whatever hook system the project already uses. Git runs a hook from the root of the work tree and reads a relative `core.hooksPath` from there (githooks(5)), which is not the project root when the project is a subdirectory of its repository (spec 059). So the value written names the hooks directory from the work tree. Compute two values before the ladder, from the project root:

- **`P`** — the project's path from the work tree: `git rev-parse --show-prefix`. Empty at the work tree's root, else ending in `/`.
- **`H`** — the hooks directory named from the work tree: `{P}.githooks`. At the work tree's root it is `.githooks`, the value every earlier `/ductus` wrote, so a project at its repository's root walks the ladder exactly as before.

Detection runs in this order — first match wins:

1. **`core.hooksPath` is `H`** — already wired up. The manifest passes have already written `.githooks/ductus-pre-commit` (`update`) and, on first run, `.githooks/pre-commit` (`create`). Run `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit` to ensure both files are executable. Report `pre-commit hook already wired up`.
2. **`P` is non-empty, `core.hooksPath` is `.githooks`, and the work tree's root has no `.githooks` directory** — the value a `/ductus` run before spec 059 wrote from this subdirectory, naming a directory that does not exist, so no hook has ever run. Rewire it as case 5 does, under the same precondition, and report `pre-commit hook rewired: .githooks → {H}`. When the work tree's root does have a `.githooks` directory, the value is someone else's and case 3 applies.
3. **`core.hooksPath` points at any other path** — the project uses a custom hooks dir. Skip wiring; report a warning with the manual integration snippet below. This includes another ductus project's hooks directory in the same repository: a repository has one `core.hooksPath`, so the second project is warned rather than wired.
4. **A third-party hook system is detected** — any of `.husky/`, `.pre-commit-config.yaml`, `lefthook.yml`, or `lefthook-local.yml` exists at the project root **or** at the work tree's root (`git rev-parse --show-toplevel`). Skip wiring; report a warning with the manual integration snippet below. The work tree's root counts because a subdirectory project cannot see a marker there from where it stands, and `pre-commit` installs into `.git/hooks` without setting `core.hooksPath`, so no earlier case would notice it before this one set the value over it.
5. **No conflicts** — run `git config core.hooksPath {H}` and `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit`. Report `pre-commit hook installed`.

**Precondition for a wired outcome (cases 1, 2 and 5).** When no non-comment line of `.githooks/pre-commit` names `ductus-pre-commit`, the file does not invoke the inner hook: it is a hook of the project's own, which the manifest's `create` pass left in place because the file already existed, or a spec-017 file a blocked or failed migration rename left behind. Wiring it would run that hook and never ductus's passes, and in case 5 would activate a hook the project had not activated. So do not wire and do not report the hook wired: leave `core.hooksPath` as found, and report `pre-commit hook skipped — .githooks/pre-commit does not invoke ductus-pre-commit; see manual integration snippet above` with the manual integration snippet below. The shipped stub and the one before spec 059 both name `ductus-pre-commit` on their invocation line, and the `ductus-rename` migration re-points an older stub's `govern-pre-commit` line before this ladder runs, so a project whose outer hook is ductus's walks the ladder as before.

**Precondition for wiring (cases 2 and 5).** When `P` is non-empty and `.githooks/pre-commit` still invokes the inner hook with the line the stub shipped before spec 059, `./.githooks/ductus-pre-commit`, do not wire. That line runs from the work tree's root, where this project's inner hook is not, so every commit would fail on it; and the file is adopter-owned, so `/ductus` does not edit it. Report `pre-commit hook not wired: .githooks/pre-commit runs ./.githooks/ductus-pre-commit from the repository root — replace that line with "$(dirname "${BASH_SOURCE[0]}")/ductus-pre-commit" and re-run /ductus`. At the work tree's root the line works as it always has, and the precondition does not apply.

The detection ladder no longer treats `.githooks/pre-commit` itself as a ductus-managed file — under the new model the outer file is adopter-owned, so its presence is not a signal that ductus installed it. Migration of pre-existing ductus-installed hooks (from spec-017 adopters) is handled by the **Migration from spec-017 hook** subsection below, which runs before the detection ladder.

The two frontmatter derivations are **runtime primitives**, not shipped scripts: `ductus derive-dependencies` and `ductus derive-references` (spec 022, `adopter-generator-promotion`). They arrive with the runtime `/ductus` acquires, so there is nothing to scaffold, refresh, or pin — a generator fix reaches adopters through the version bump that ships the binary. The pre-existing `.ductus/scripts/` entries were removed by the `generator-primitives` migration.

### Migration from spec-017 hook

Adopters who installed the pre-commit hook under spec 017 have a single ductus-managed file at `.githooks/pre-commit` carrying the `# managed-by: ductus` sentinel on line 2. The new layout splits that file into a ductus-owned inner script and an adopter-owned outer stub at the same path. Migration runs **before** the detection ladder above and **before** the manifest passes for the two hook files, so the manifest's `update`/`create` strategies see the post-rename layout.

Trigger:

- `.githooks/pre-commit` exists, AND
- the file's line 2 is exactly `# managed-by: ductus`, AND
- `.githooks/ductus-pre-commit` does **not** exist.

When all three hold, perform the rename:

1. Determine whether the file is tracked: `git ls-files --error-unmatch .githooks/pre-commit` (exit code 0 = tracked).
2. If tracked: `git mv .githooks/pre-commit .githooks/ductus-pre-commit`. If untracked: `mv .githooks/pre-commit .githooks/ductus-pre-commit`.
3. Continue with the detection ladder and the manifest passes. The renamed inner file is byte-identical to upstream for unmodified adopters, so the `update` strategy on `.githooks/ductus-pre-commit` is a no-op; the `create` strategy on `.githooks/pre-commit` writes the new outer stub since the path is now empty.
4. Append to the post-scaffolding summary: `migrated pre-commit hook: .githooks/pre-commit → .githooks/ductus-pre-commit; created adopter-owned .githooks/pre-commit stub`.

Recovery branches:

- **Pre-existing `.githooks/ductus-pre-commit` blocks the rename.** If the inner-file destination already exists when the trigger fires, abort the rename without renaming anything. Report `migration skipped: .githooks/ductus-pre-commit already exists; resolve manually` and continue with the detection ladder and manifest passes. The `update` strategy overwrites the pre-existing inner with the shipped contents; the existing `.githooks/pre-commit` (still carrying the sentinel) is left in place but is no longer detected as ductus-managed by the new ladder, so it is treated as adopter-owned going forward. The adopter resolves the duplicate manually.
- **`git mv` fails (permissions, repo locked, file in use).** Report `migration failed: could not rename .githooks/pre-commit; resolve manually` and continue with the detection ladder and manifest passes. The `update` strategy installs `.githooks/ductus-pre-commit` from scratch (destination doesn't exist); the `create` strategy sees `.githooks/pre-commit` still in place and skips. The adopter ends up with both files (the legacy sentinel'd outer, and the new ductus-owned inner idle, since the outer does not invoke it — so the ladder's wired-outcome precondition reports the skip with the manual integration snippet) and completes the migration manually by editing the outer to invoke the inner hook beside it, as the shipped stub does.

If any of the trigger conditions does not hold, skip the migration silently — the detection ladder handles the case.

### Manual integration snippet (for skip cases)

When detection skips installation (cases 3 and 4 above, or an outer hook that does not invoke the inner one), report this message to the user, with `{P}` substituted (empty when the project is the repository root):

> The `ductus` pre-commit hook was not wired up because your project already uses an existing hook system. To get automatic spec-deps regeneration on every commit, add this line to your existing pre-commit chain:
>
> ```bash
> ./{P}.githooks/ductus-pre-commit
> ```
>
> Hook runners run from the repository root, so the path names the hook from there. The shipped hook script finds its project from its own location, and is idempotent and safe to call from another hook runner.

### Pinning

Both hook files are subject to `.ductus/config.toml` `pinned.files`, but the meaning differs by ownership:

- **`.githooks/ductus-pre-commit`** is the only file pinning is meaningful for. A pinned inner file uses `skip` strategy instead of `update` — `/ductus` does not overwrite it across releases. Useful when an adopter has customized ductus's generator orchestration and does not want it reset.
- **`.githooks/pre-commit`** is `create`-strategy and never overwritten after first run regardless of pinning. Listing it in `pinned.files` is harmless but has no effect.

The Hook Installation section above still runs and may set `core.hooksPath` regardless of pinning.

## What This Command Does NOT Do

- Modify `README.md` — the project's README is its own
- Create feature specs — the user does that via `/{project}:specify`
- Fill in AGENTS.md content — that requires project-specific knowledge
- Fill in system.md content — that requires architectural decisions
- Make git commits — the user decides when to commit
- Write findings to `specs/inbox.md` — the security audit reports its gaps and writes nothing; the inbox holds what a person logs
- Run `/{project}:configure` — that happens after adoption, interactively
- Delete an agent's adopted tree — manual cleanup

## Edge Cases

- **Unknown agent key in `--agents=`** — stop before scaffolding; report the unknown key with the list of valid keys.
- **All supported agents already adopted with `--add-agent`** — show the prompt with all agents pre-selected; if the user confirms with no additions, treat it as a routine update and continue silently.
- **The agent's settings file already has entries beyond the bootstrap** (§Permission Setup resolves it per layout — `settings.local.json`, `settings.json`, or the repo-root `opencode.json`) — only add the curl/ls bootstrap entries if missing. Do not overwrite, deduplicate, or reorder entries added by `/{project}:configure` or by the user.
- **`ductus.md` content already matches the version on disk** — when the manifest's `update` strategy compares fetched content to the installed file, identical content reports as "unchanged" and avoids a redundant write. Same rule applies to per-project `configure.md` and other update-strategy files.
- **Pinned `ductus.md` in `.ductus/config.toml`** — the manifest's `update` strategy still skips the file (no overwrite), and the **Pre-flight Phase**'s self-update check never writes pinned files even on the stale-detect path. The check byte-compares anyway: matching upstream → recorded as `current`, no output; divergent from upstream → recorded as `pinned-divergent`, the run continues, and a single advisory line is printed in the post-scaffolding output. A pinned `ductus.md` will not pick up upstream changes until the pin is removed, but the user is told once when the pin is currently suppressing real divergence.
- **Self-update check sees a stale `ductus` in an unselected adopted agent** — the check is scoped to selected agents only. The unselected agent's stale copy is not diffed, not written, and does not trigger the abort; it will be detected the next time the user runs `/ductus` against it.
- **Self-update small fetch fails** — clean abort with the error message defined in **Pre-flight Phase → Self-update check → Small fetch**. No `ductus.md` writes occur, and the archive fetch is skipped. The user re-runs after the transient failure clears.
- **Archive fetch or extract fails** — clean abort with the error message defined in **File Fetching → Archive fetch and extract**. The pre-flight phase has already passed by this point, so no additional `ductus.md` or ductus-wiring writes are pending; the user re-runs after the transient failure clears.
- **A required source file is absent from the extracted archive** — warn `Source not found in archive: {source-path}; skipping.` and continue with the remaining manifest entries. Preserves the per-entry "do not abort on a single fetch error" guarantee at the entry level even though the archive itself is fetched once.
- **First-run prompt with no detected dirs and only one supported agent** — the prompt still appears (the agent must be explicitly chosen), but the single agent is pre-selected. Confirming is one keystroke.
- **Running `ductus.md` cannot infer its own install path** — fall back to no pre-selection in the first-run prompt. The user picks explicitly.
- **ductus runtime not live (State B)** — the **Pre-flight Phase** acquires the pinned release into the store, materializes the pointer, and registers the runtime per the agent's `mechanism` (writes the MCP config for a `write-file` agent, or surfaces the registration command for a `surface-instruction` agent), then joins the **deferred-restart set** and continues the run through the CLI at `{pointer-path}`, surfacing in the **Closing restart** at the end.
- **ductus wiring file is malformed JSON** — the wiring write does not touch the file. `/ductus` skips wiring and warns the user to repair it; the runtime is still acquired and the pointer still materialized, so the next run wires it once the file parses. A hand-maintained config is never clobbered.
- **ductus store probe cannot run or is denied** — the run is classified as State B and acquisition proceeds. Acquisition is idempotent, so a false negative costs a version comparison rather than a redundant download. Detection never hard-fails on a host without shell.
- **Stale `ductus.md` on an adopter who has never wired ductus** — both pre-flight checks contribute writes (a fresh `ductus.md` and the ductus wiring), but the **Pre-flight abort** emits one combined message and the user restarts once, not twice.

## Post-Scaffolding Output

After scaffolding, display:

- The source line **Source resolution** emitted in pre-flight, repeated — `Source: {source-label}` — so the summary says which release the files above came from
- Summary of files created, updated, unchanged, skipped, pinned, merged, and removed — grouped by agent for per-agent files, with shared files in their own group
- Placeholder substitutions applied, as `apply-manifest` reports them: the total alongside the number of entries that ran substitution (`N substitutions across M files`). Both numbers, never just the total — zero across twenty substituted files is a malformed or incomplete map, while zero across zero is a manifest of pinned and skipped files behaving correctly, and one number cannot tell an operator which they are looking at.
- For each scaffolded agent, the agent's `rules_file_note` from the registry
- Hook installation status — one line: `pre-commit hook installed`, `pre-commit hook already wired up`, `pre-commit hook rewired: .githooks → {H}`, `pre-commit hook not wired: …` (the pre-059 outer-stub precondition, with its replacement line), `pre-commit hook skipped — .githooks/pre-commit does not invoke ductus-pre-commit; see manual integration snippet above`, or `pre-commit hook skipped — existing {husky|lefthook|pre-commit-py|core.hooksPath} detected; see manual integration snippet above`. When the spec-017 → spec-018 migration ran, append the migration summary line described in §Hook Installation > Migration from spec-017 hook (or the relevant recovery-branch warning if the rename was skipped or failed).
- Any fetch failures encountered
- Pinned `ductus.md` advisory (if applicable — see below)
- Security audit summary (if applicable — see below)
- ductus runtime tip (failed acquisition only — see below)
- Next steps (varies by mode):

### Closing restart

When the **deferred-restart set** is non-empty — State B wired `ductus` this run — append this after the summary, as the last thing the run says:

> **`ductus` was wired in this run and the work above is complete.** Every primitive ran through the CLI at `{pointer-path}`. Start a new session so the MCP server loads and the pipeline commands call the same primitives as tools.

The restart is for the **tool surface**, not for unfinished work — that distinction is the whole point of moving this here, and the message says which so the operator does not re-run `/ductus` expecting it to do more. One restart, not two: migrations, the archive fetch, Shared Files, and every scaffolding step already happened. An adopter carrying a `ductus.md` that predates the Pre-flight Phase still pays the separate self-update hop, because a copy that cannot execute this phase cannot be talked into it.

Two variants:

- **The wiring was skipped** because the agent's MCP file is not valid JSON: acquisition still happened and the pointer exists, so the work above still completed by CLI. Say the registration was skipped and name the file, rather than promising a tool surface that will not appear.
- **A `surface-instruction` agent** (Auggie, Antigravity) never gets an MCP file written by `/ductus` at all: keep surfacing the one-line registration command here, after the work rather than instead of it.

Nothing is emitted when the set is empty — a State A run has no restart to announce, and a line saying so would be noise on every run.

### ductus runtime tip

When acquisition was attempted and **failed** (the run halts, so this line accompanies the halt rather than a completed scaffold), append one line after the file summary:

> Tip: acquisition failed, so this run halted before the deterministic path was available — there is no markdown-only mode to degrade into. Recover either by placing the pinned binary into the store by hand (the halt message above names the store path and the release URL) or by setting `[runtime] path` in `.ductus/config.toml` to a binary you supply, then re-run `/ductus`. Your `PATH` is not consulted. See [The runtime](https://github.com/stonean/ductus#the-runtime) in the ductus README.

Omit the tip in **State A** (the runtime is already live) and in a successful **State B** (the runtime was acquired and wired, so there is nothing to tip about). State B's file disclosure rides the **Closing restart**, not this output.

### Pinned `ductus.md` advisory

If the **Pre-flight Phase** recorded any selected agent as `pinned-divergent` (the installed `ductus` file — the §Derived values **`ductus` install path** row: `{config_dir}/commands/ductus.md` for `claude-style`, `{config_dir}/command/ductus.md` for `opencode`, `{config_dir}/skills/ductus/SKILL.md` for `antigravity`, `{config_dir}/prompts/ductus.md` for `pi` — is listed in `.ductus/config.toml` `pinned.files` and differs from upstream), append one advisory line per divergent agent after the file summary and before next steps:

> {agent}: ductus.md pinned, upstream has changed.

The advisory is omitted when no agent is `pinned-divergent` — adopters whose pinned version still matches upstream see nothing; adopters with no pin see nothing. The check's `stale` path aborts before this output is ever produced, so the advisory is only ever about pinned files.

### Security audit summary

If the **Security Audit (brownfield)** section ran and produced one or more findings, append this block to the file summary — rendering each command in **the adopted agent's own invocation form**, per the §Derived values **Invocation** row, exactly as the next-steps list below does. The colon form here is the `claude-style` default; printing it to an Antigravity or OpenCode adopter names a command that does not exist on their agent:

```text
Security audit: {N} gap(s) in {M} existing spec(s) — nothing was written.
  {NNN-feature}
    {Rule ID}: {affected artifact path} does not address — {one-line summary}
Each gap resurfaces when /{project}:analyze next runs on its spec. Run it on a spec to fix or route its gaps now (a SHOULD-tier gap may also be discarded), or /{project}:log one to track it.
```

One `{NNN-feature}` group per spec with a finding, each listing every finding for that spec — no cap, because a truncated list reads as the complete one. Omit the block when:

- The audit did not run (trigger conditions did not fire — greenfield run, or routine re-run with rule files already present), OR
- The audit ran but produced no findings (no rule's Verification trigger fired against any existing artifact).

This summary hands off to `/{project}:analyze`, which dispositions each gap in the run that finds it.

### First run (no existing `specs/` directory)

---

**ductus adopted successfully.**

Adopted agents: {comma-separated `name` of selected agents}.

Next steps (render each command in **the adopted agent's own invocation form** — the §Derived values **Invocation** row: `/{project}:<name>` for `claude-style`, `/{project}-<name>` for `antigravity`, `/{project}/<name>` for `opencode`, `/{project}-<name>` for `pi`. The colon form below is the `claude-style` default; printing it to an Antigravity, Pi, or OpenCode adopter names six commands that do not exist on their agent. When several agents were adopted, render the list once per agent under its name):

1. Run `/{project}:configure` in each adopted agent to apply the full permission set.
2. Fill in `AGENTS.md` — tech stack, project structure, code style, testing conventions, gotchas.
3. Fill in `specs/system.md` — architecture, request lifecycle, shared infrastructure.
4. Use `/{project}:log` to record any known issues or bugs into `specs/inbox.md`.
5. Run `/{project}:groom` to walk the inbox and route each item to its proper spec or scenario.
6. Create your first feature spec: `/{project}:specify {feature description}`.
7. The deterministic runtime is already acquired and wired by this run — you do not install it. See [The runtime](https://github.com/stonean/ductus#the-runtime) in the ductus README for how it is pinned and upgraded.

To adopt an additional agent later, re-run `/ductus --add-agent`.

Tip: `specs/` is plain markdown and works in any PKM tool (Obsidian, Logseq, Foam) or as a published site (Quartz, MkDocs). Pick whichever fits your workflow, or none.

---

### Update mode (existing `specs/` directory detected)

---

**ductus updated successfully.**

Updated agents: {comma-separated `name` of selected agents}.

Review changes to updated files and commit when ready. To adopt an additional agent, re-run `/ductus --add-agent`.

Tip: `specs/` is plain markdown and works in any PKM tool (Obsidian, Logseq, Foam) or as a published site (Quartz, MkDocs). `/ductus` keeps the deterministic runtime current on every run — see [The runtime](https://github.com/stonean/ductus#the-runtime) in the ductus README.

---

## Idempotency

This command is safe to run again. Files with `update` strategy are always overwritten with the latest `ductus` version — unless pinned in `.ductus/config.toml`, in which case they are skipped. Files with `create` strategy skip existing files. The `.gitignore` merge checks for the `# ductus` marker before appending. `skip` strategy files are never overwritten.

Re-runs are additive across agents — adopting a new agent leaves existing agents' files untouched.

## Directory Creation

Create intermediate directories as needed (e.g., `specs/`, `specs/templates/`, and — by layout — `{config_dir}/commands/{project}/` for `claude-style`, `{config_dir}/command/{project}/` for `opencode`, `{config_dir}/skills/` and `{config_dir}/rules/` for `antigravity`, or `{config_dir}/prompts/` and `{config_dir}/extensions/` for `pi`).

Throughout this command, every `specs/…` destination — the §Shared Files manifest rows (`specs/system.md`, `specs/inbox.md`, `specs/rules/…`, …), the directories created here, and the spec-root named in the §Post-Scaffolding Output — is written under the configured `[paths] specs-root` (default `specs`, resolved in §Collect Project Inputs). The literal `specs/` paths in the manifest tables and prose are the documented default; substitute the configured name when the operator has set one. This keeps the manifest readable while honoring the override (spec 040).
