---
status: done
dependencies: [026-framework-self-audit]
next-criterion: 34
---

# 027 — Bootstrap Migration Registry

Replace the monotonically-growing prose-encoded Pre-run Migrations section in [framework/bootstrap/ductus.md](../../framework/bootstrap/ductus.md) with a machine-readable registry of convention removals. Adopters record the last migration they applied; bootstrap runs only newer entries. After a per-entry sunset, migrations drop from the active registry but stay documented in an adopter-facing changelog. A new [`/audit`](../026-framework-self-audit/spec.md) family fails when a removed convention has no registry entry, so future removals can't ship without an adopter path.

## Motivation

Every convention removal `ductus` has shipped — `governance` → `ductus` rename, `# Governance` gitignore marker, `spec-and-plan.md` sunset, rule-file relocation, `skills/` → `workflows/` rename, post-005 workflow filename rename, `configuration.md` → `configuration-cross.md` — added a bespoke prose section to `framework/bootstrap/ductus.md`, as that file's ancestor `framework/bootstrap/govern.md` stood before `71d72ee1` (`git show 71d72ee1^:framework/bootstrap/govern.md`, its `## Pre-run Migrations`). Each section scans the adopter's filesystem on **every** `/ductus` re-run, forever. A fresh adopter who installed at the latest ductus release still pays the cost of looking for legacy artifacts that have not existed in the framework for months.

Two compounding problems follow:

1. **Bootstrap work grows monotonically.** The Pre-run Migrations section can only grow because there is no sunset path. Every release that removes a convention adds another permanent scan.
2. **Future removals are unsafe.** Nothing prevents a maintainer from deleting a template, command, or filename convention without writing an adopter migration. Adopters silently break on the next pipeline command.

This spec introduces (a) a machine-readable registry of convention removals, (b) an adopter-side `last_applied_migration` field in `.ductus/config.toml` so bootstrap can skip already-applied entries, (c) a sunset mechanism that prunes old entries from the active registry into an adopter-facing changelog, and (d) an `/audit` family that enforces the registry as the only way to ship a removal.

## Registry Shape

A new file at `framework/migrations.toml` lists every active convention removal. Each entry carries:

- **id** — stable slug the adopter's `.ductus/config.toml` references.
- **introduced_in** — the ductus version (or framework commit) that shipped the removal.
- **sunset_after** — the ductus version past which this migration drops from the active registry into the adopter changelog.
- **summary** — one-line human-readable description (e.g., "spec-and-plan.md → spec.md").
- **procedure_file** — path to the markdown procedure at `framework/migrations/{id}.md`; there is no declarative-step form.
- **idempotency** — every entry must be a no-op when its target artifact is absent. This is an invariant, not a field.

## Adopter State

`.ductus/config.toml` gains a single new field:

```toml
[migrations]
last_applied = "<entry-id>"
```

When the field is absent, bootstrap treats it as "no migrations have been applied" and runs every active entry — except on a **greenfield** run, one that began with no config file at any tier and no spec root, which has nothing to migrate: it applies nothing, asks nothing, and seeds the field with the newest active entry's id (`framework/bootstrap/ductus-procedure.md` §Pre-run Migrations, steps 3–4, since spec 071's task 20; until then a brand-new project was prompted to approve every active entry, each a no-op). A **first adoption**, which Pre-flight Checks decides from finding no signal an earlier `ductus` run leaves, does the same though it keeps a spec root of its own: that spec root and any root `constitution.md` are the project's, so no entry has anything to convert (scenario `a-first-adoption-runs-no-migration`). When present, bootstrap runs only entries newer than the recorded id in registry order, which is `introduced_in` by SemVer comparison and then `id`. After all applicable entries succeed, bootstrap updates the field to the id of the newest entry in the active registry. An unrecognized key in the `[migrations]` section is named and does not fail the run: `/ductus` names it in its summary's `unknown config key` line (`framework/bootstrap/ductus-procedure.md` §Post-Scaffolding Output), and a runtime reader of the section warns on stderr, since `runtime/src/schema/config_keys.rs` declares the section's keys as `last_applied` alone and its `warn_unknown_keys` reports any other (019's scenario `an-unknown-config-key-is-named` and 022's scenario `config-readers-name-unknown-keys` own that behavior).

**Scope: an entry applies once per repository, not once per contributor.** The marker lives in the committed config file, so the first contributor to run `/ductus` applies the pending entries and commits the advanced marker; everyone who pulls afterwards reads it, sees nothing pending, and skips. That is correct for repo-shared state — the edit genuinely happened once, for everyone.

The rule that follows is about remedy, not about paths: **a migration must not be the sole remedy for a defect in per-contributor state.** Touching gitignored per-contributor state is fine when a skipped contributor self-heals through ordinary use — `session-file-consolidate` and `ductus-rename` both move a session file, and a teammate who skips either has their next `/{project}:target` write the active path anyway under the newest-tier rule, while the procedure exits silently when the legacy file is absent. It is not fine when nothing else would ever perform the cleanup: that reaches one person and is then marked done for the whole repo, permanently, with no later run able to correct it.

A cleanup that must reach every contributor belongs on the command that owns the per-contributor file, which runs per-contributor and can be idempotent rather than marker-driven. The worked example is `/ductus:configure` step 4, which retires formerly-canonical permission entries from the gitignored `{cli-config-dir}/settings.local.json` (spec 023, `configure-retires-formerly-canonical-entries`); `/ductus` could not have hosted it in any case, since it never writes the full permission set (`framework/bootstrap/ductus-procedure.md` §What This Command Does NOT Do). See `scenarios/migrations-apply-once-per-repo.md`.

## Bootstrap Loop

The archive half's Pre-run Migrations, in `framework/bootstrap/ductus-procedure.md`, is a single procedure that:

1. Reads `framework/migrations.toml` from the fetched archive, and aborts before applying anything when two entries share an `id`, an entry's `procedure_file` is missing from the archive, or an entry's `id` is not a slug or its `procedure_file` is not exactly `framework/migrations/{id}.md` (its duplicate-id and reference-integrity guard, which tests the slug and exact-path rule before it probes a `procedure_file`, so it probes only an entry that passes). The path step 7.1 then reads stays under `framework/migrations/`: a slug holds no `/`, `\` or `..`, and the staging tree holds no link to carry it out, since `extract-archive` extracts no link entry and refuses an entry that a link under its destination would carry outside it (`runtime/src/primitives/extract_archive.rs`'s module doc; `BE-INPUT-004`).
2. Reads `.ductus/config.toml` `[migrations].last_applied`.
3. For each registry entry newer than `last_applied`, executes the entry's procedure file, `framework/migrations/{id}.md`.
4. After each entry's procedure completes, updates `.ductus/config.toml` `[migrations].last_applied` to that entry's `id`, before the next entry begins (AC9).
5. Reports in the post-scaffolding summary each line an applied entry's procedure emits, its summary line among them, verbatim and in order; the registry's `summary` field describes the entry and is not printed.

Entries past their `sunset_after` version are excluded from this loop entirely — they live in the adopter changelog only.

## Sunset and Adopter Changelog

When an entry's `sunset_after` version is older than the current ductus release, the entry is removed from `framework/migrations.toml` and its text is appended to the adopter-facing changelog, `CHANGELOG.md` at the repo root, with its target paths. Adopters who are far enough behind to need an expired migration must apply it manually using the archive entry as a recipe. Family 10's 10e fails while an active entry is past its `sunset_after`, so the archival does not wait on anyone remembering it.

The sunset window is per-entry, not global. High-impact removals (like a directory rename) can stay active longer than low-impact ones (like a gitignore-marker rename). The sunset value is chosen when the entry is written.

## Audit Family

Family 10 extends [spec 026](../026-framework-self-audit/spec.md)'s `/audit` as `scripts/audit/migration-coverage.sh`. It runs these checks:

- **10a**, no orphan procedure file: every `framework/migrations/{id}.md` has a registry entry.
- **10b**, no stale target path: every `framework/`-prefixed `target_paths` entry of the active registry, and of each archived entry in `CHANGELOG.md` (its `**Target paths:**` line), names something absent from current `framework/`. The others are adopter-relative and unverifiable here.
- **10c**, no broken procedure reference: every `procedure_file` exists.
- **10d**, since task 15: no entry added after another sorts before it, which reads the registry's git history; the two decided pairs `scripts/audit/migration-coverage.sh`'s `EXEMPT` set holds (`criterion-label-backfill` before `ductus-rename` and before `runtime-store-path`) are exempt.
- **10e**: no active entry is past its `sunset_after` against the release `version` names (owner, 2026-11-17). Whether a removal shipped with its migration is a property of a diff, not of the current state, so it is not this family's to check — see the detection-mechanism resolution below. A `version` or `sunset_after` that is no release is a finding too, with a remedy for that value rather than an archival.
- **10f**, since task 19: no `id` names two entries, which Pre-run Migrations' duplicate-id guard aborts on and the other checks pass (AC25).

## Acceptance Criteria

### Registry shape

- [x] AC1: `framework/migrations.toml` exists with one `[[migrations]]` array-of-tables entry per active convention removal. Each entry carries the fields: `id` (slug), `introduced_in` (SemVer string), `sunset_after` (SemVer string or omitted), `summary` (one-line string), `target_paths` (array of strings), `procedure_file` (path string).
- [x] AC2: Six back-filled entries cover every convention removal currently encoded in `framework/bootstrap/ductus.md` Pre-run Migrations: `.governance.toml` rename, `# Governance` gitignore marker, `spec-and-plan.md` sunset, rule-file relocation (subsuming `configuration.md` → `configuration-cross.md`), legacy `skills/` directory, post-005 workflow filename rename. — **superseded by AC14's archival**: all six are now in CHANGELOG.md's Archived migrations, `skills-to-workflows` and `workflow-filename-rename` by 2fcb5f93 and the other four by c061c1f0, so none is still registered
- [x] AC3: Each back-filled entry's `introduced_in` matches the ductus version (or framework commit) where the removal actually shipped, derived from `git log` at registry-introduction time.
- [x] AC4: Each back-filled entry's `sunset_after` is set to `registry_introduction_version + 2 minor versions` (uniform). — **Never true as written:** the registry first shipped in `gvrn-v0.7.3`, so the six entries' uniform `sunset_after` of 0.10.0 is three minors on, not two (`71d72ee1` assumed it would ship at 0.8.0); all six are now archived in `CHANGELOG.md`.
- [x] AC5: Each back-filled entry has a corresponding `framework/migrations/{id}.md` procedure file containing the migration's prose logic (skip conditions, prompts, summary reporting). No declarative-step DSL is introduced. — **superseded by AC14's archival** for all six: archiving an entry moves its procedure into CHANGELOG.md and deletes the file

### Adopter state

- [x] AC6: `.ductus/config.toml` schema documents a `[migrations]` section with a `last_applied` field (string, slug-valued). Absent field means "no migrations applied" — bootstrap runs every active entry. **Since spec 071's task 20, a greenfield run (no config file at any tier and no spec root) is an exception: it seeds the field with the newest active entry's id and runs nothing. Since this spec's task 16 a first adoption is one too (scenario `a-first-adoption-runs-no-migration`).**
- [x] AC7: On `/ductus` re-run against an adopter whose `last_applied` equals the newest active entry's `id`, the migration loop performs zero filesystem reads beyond loading the registry and reports zero migrations applied. — **superseded in part**: the loop also reads the pre-run `git status --porcelain -- specs/`, the active config's `last_applied`, and each `procedure_file`'s presence for the duplicate-id and reference-integrity guard (`framework/bootstrap/ductus-procedure.md` §Pre-run Migrations); it still applies nothing and emits nothing
- [x] AC8: On `/ductus` re-run against an adopter whose `last_applied` is older than the newest entry, only entries newer than `last_applied` (per SemVer comparison on `introduced_in`, lexicographic tie-break on `id`) execute.
- [x] AC9: After each migration completes successfully, `last_applied` is updated to that entry's `id` before the next entry begins. An aborted batch resumes from the next-pending entry on the following `/ductus` run.

### Bootstrap loop

- [x] AC10: The Pre-run Migrations procedure in `framework/bootstrap/ductus-procedure.md` iterates the registry, and no per-migration prose block remains in it or in `framework/bootstrap/ductus.md`.
- [x] AC11: When pending migrations exist, bootstrap emits a single batch prompt listing each pending entry by `id` and `introduced_in`, then asks `Apply now? (Y/n)`. Decline emits a warning summary and makes no filesystem changes.
- [x] AC12: The post-scaffolding summary lists each applied migration by its `summary` field — **narrowed**: it carries the lines each applied entry's procedure emits, its summary line among them, verbatim and in order (`framework/bootstrap/ductus-procedure.md` §Post-Scaffolding Output), not the registry's `summary` field. Entries with no work to do (target artifact absent) emit nothing — the procedure file's per-entry idempotency check is invariant.

### Sunset and archive

- [x] AC13: An entry whose `sunset_after` is older than or equal to the current ductus release is excluded from `framework/migrations.toml` (removed at sunset commit) and its procedure-file content is appended to `CHANGELOG.md` at the repo root under a heading naming the entry's `id`, `introduced_in`, and `sunset_after` version.
- [x] AC14: When a migration is sunsetted: the registry entry, the `framework/migrations/{id}.md` file, and the `CHANGELOG.md` append all land in the same commit. `framework/migrations/{id}.md` no longer exists after the sunset commit.
- [x] AC15: The bootstrap loop never reads `CHANGELOG.md` (verified by tracing reads in the `/ductus` procedure).

### Audit (Family 10)

- [x] AC16: A new `scripts/audit/migration-coverage.sh` (Family 10 of [026](../026-framework-self-audit/spec.md)) runs these checks against the registry, its archive and its history:
  - [x] AC17: **No orphan procedure files**: every `framework/migrations/{id}.md` has a matching TOML entry with `procedure_file` pointing at it. Failures: orphan files exist.
  - [x] AC18: **No stale target paths**: every `target_paths` entry across `framework/migrations.toml` *and* archived entries in `CHANGELOG.md` (each entry's `**Target paths:**` line, without which the entry is itself a failure) refers to a path that does *not* exist in current `framework/`. Failures: a registry entry says "removed" but the path still exists.
  - [x] AC19: **No broken procedure references**: every TOML entry's `procedure_file` field points at an existing `framework/migrations/{id}.md`. Failures: dangling references.
  - [x] AC31: **No late entry sorting early** (10d): every entry sorts, by (`introduced_in`, then `id`), after every entry first present in an earlier commit of `framework/migrations.toml`, entries added in one commit exempt from each other, and the two decided pairs `scripts/audit/migration-coverage.sh`'s `EXEMPT` set holds (`criterion-label-backfill` before `ductus-rename` and before `runtime-store-path`) exempt; a shallow checkout, whose history it cannot read, is a failure. Failures: an entry an adopter's `last_applied` would skip forever.
  - [x] AC32: **No overdue sunset** (10e): no active entry's `sunset_after` is at or below the release the repository's `version` file names, and a missing or malformed `version` is a failure. Failures: an entry Pre-run Migrations already skips, still owed its archival (AC13, AC14).
  - [x] AC33: **No duplicate ids** (10f): no `id` names two `[[migrations]]` entries. Failures: an `id` Pre-run Migrations' duplicate-id guard aborts on (AC25).
- [x] AC20: Family 10 exits `0` when every check passes, `1` otherwise. Output format matches the other families (header + findings rows on stdout). The orchestrator `scripts/audit/run-all.sh` invokes Family 10.

### Idempotency

- [x] AC21: Every registry entry's procedure file is idempotent: re-running it against a state where the target artifact is already migrated or absent produces no filesystem changes, no error, and emits nothing to the post-scaffolding summary. — **narrowed**: no re-run changes a file or errors, and an absent target emits nothing, but a present target already converged reports one line saying so in `claude-md-import-syntax`, `constitution-import`, `inbox-guidance-refresh`, `memory-routing-leaves-claude-md`, `remove-legacy-constitutions-region` and `strip-inbox-guidance`, each from its procedure's idempotency check (`inbox-guidance-refresh`'s `guidance already current`, the others' `… — left as is`)

### Edge cases and failure modes

- [x] AC22: Empty registry (`framework/migrations.toml` exists but contains zero `[[migrations]]` entries): the bootstrap loop is a no-op, no batch prompt is emitted, no `last_applied` write occurs.
- [x] AC23: `.ductus/config.toml` exists but has no `[migrations]` section: treated as "no migrations applied" — bootstrap runs every active entry. Subsequent run writes the `[migrations]` section.
- [x] AC24: `.ductus/config.toml` `[migrations].last_applied` references an `id` that no longer exists in the active registry (sunsetted since the adopter's last run): bootstrap treats the field as "before the oldest active entry" and runs every active entry. A warning is emitted: `last_applied was "{id}" which has been retired; see CHANGELOG.md for its recipe`.
- [x] AC25: Two TOML entries share the same `id`: bootstrap aborts at registry-parse time, before applying anything, with `migration {id}: duplicate id in framework/migrations.toml`. Family 10's duplicate-id check, 10f, fails on a subsequent `/audit`. — **completed by task 19**: this said the no-orphan check would fail, but it, 10c and 10d each passed a duplicated id, and the guard named no message
- [x] AC26: Malformed `framework/migrations.toml` (TOML parse error): bootstrap aborts before running any migration, matching existing 022 TOML-parse-error semantics.
- [x] AC27: `framework/migrations/{id}.md` referenced by a TOML entry does not exist in the fetched archive: bootstrap aborts the batch at that entry, reports `migration {id}: procedure file missing from archive`, and `last_applied` retains its prior value. Family 10's no-broken-references check would have caught this at maintainer time. — **superseded in part by `367c8817`**: the guard runs at step 1, before any entry applies, so the batch aborts before its first entry rather than at that one; the message and the retained `last_applied` hold (`framework/bootstrap/ductus-procedure.md` §Duplicate-id and reference-integrity guard). The guard also aborts, before any entry applies, with `migration {id}: unsafe id or procedure_file` for an entry whose `id` is not a slug (`^[a-z0-9]+(-[a-z0-9]+)*$`) or whose `procedure_file` is not exactly `framework/migrations/{id}.md`, so no `id` reads outside `framework/migrations/` (`BE-INPUT-004`)
- [x] AC28: User confirms the outer batch prompt but a per-entry inner prompt within a procedure file is declined: the procedure handles the decline (typically emits a warning and skips that file), the migration still "completes" from the loop's perspective, and `last_applied` advances. Behavior matches the existing prose migrations.
- [x] AC29: Adopter's installed ductus version is older than an entry's `introduced_in`: assumed impossible (the adopter's bootstrap is taken to be current-ductus or newer than the registry it just fetched), but if observed (e.g., manual archive override), the entry runs anyway — `introduced_in` gates registry order, not adopter eligibility.

### Out of scope

- A "removal must ship with a migration entry" diff-check is out of scope, with no follow-on (owner decision, 2026-11-19): nothing catches, at commit time, a framework removal shipped without a `[[migrations]]` entry. Reason: it's a diff operation against HEAD, not a static-state check, and would couple the check to git history shape.
- `apply-migrations` as a runtime primitive is deferred until Family 9 (`primitive-promotion-candidates.sh`) detects pattern duplication across procedure files.

### Migration scope

- [x] AC30: §Adopter State and `framework/migrations.toml`'s header comment both state that the registry applies an entry **once per repository**, not once per contributor — `[migrations].last_applied` lives in the committed config file, so the first contributor to run `/ductus` applies the pending entries and everyone who pulls afterwards skips them. The rule that follows is stated as one about remedy rather than paths: a migration must not be the sole remedy for a defect in per-contributor state. `session-file-consolidate` and `ductus-rename` are named as the self-healing precedent that keeps the rule from being over-read as "never touch per-contributor state" — a skipped contributor's stale session file is superseded by their next `/{project}:target` under the newest-tier rule, and the procedure exits silently when the legacy file is absent. A cleanup that must reach every contributor is directed to the command owning that file, with `/ductus:configure`'s permission retirement (spec 023, `configure-retires-formerly-canonical-entries`) as the worked example, noting that `/ductus` never writes the full permission set in any case. `target_paths` is recorded as needing no change: `gitignore-marker-rename`, until `c061c1f0` archived it, targeted a file it edited rather than removed, and Family 10's no-stale-target-paths check constrains only `framework/`-prefixed entries, so an edited-not-removed adopter path was always inside the contract. `{config_dir}/commands/` is noted as repo-shared despite living under `.claude/` (`framework/templates/project/gitignore` excepts it with `!.claude/commands/`), so `workflows-sunset` is not a counter-example. See [`scenarios/migrations-apply-once-per-repo.md`](scenarios/migrations-apply-once-per-repo.md).

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Registry format** — TOML. The registry's primary content is structured metadata per entry (`id`, `introduced_in`, `sunset_after`, target paths, procedure type), which is settings, not instructions. Markdown in `framework/` is reserved for instructions (commands, constitution, bootstrap procedure, templates). Entries whose procedure cannot be expressed as a declarative step reference a separate `framework/migrations/{id}.md` file by id, preserving the split: settings in TOML, instructions in markdown.
- **Entry id format** — slug-only (e.g., `spec-and-plan-sunset`), with ordering derived from a separate `introduced_in` field and lexicographic tie-break on id. Rules out content-hash (rewording orphans adopter state), monotonic numbers (concurrent-PR conflicts), and version-prefixed ids (entries get retargeted to later releases). The slug is what the entry *is*; release/date metadata lives in dedicated fields. Idempotency invariant makes a rare slug rename safe — adopter re-runs the entry, no-op against already-migrated target.
- **Registry order** — by `introduced_in` (SemVer comparison, not lexicographic — `0.4.0 < 0.10.0`), with lexicographic tie-break on `id` for entries shipped in the same release. File order in the TOML is not authoritative: the order comes from `introduced_in` and `id`, so where an entry sits in the file carries no meaning. The `introduced_in` comparison matches the mental model: "shipped in 0.4.0" applies to adopters who installed at <0.4.0.
- **Procedure declaration** — every entry references a markdown procedure file at `framework/migrations/{id}.md`; no declarative step DSL. The TOML entry stays pure metadata (`id`, `introduced_in`, `sunset_after`, `summary`, `target_paths`, `procedure_file`). All migration logic — skip conditions, prompts, pinned-file exclusions, collision handling, summary reporting — lives in the markdown procedure file. Of the six migrations then existing, one (gitignore marker rename) was judged one-step-declarative at the decision; the rest had twists that would have required inventing a small DSL for one consumer. `/audit`'s migration-coverage check uses the `target_paths` metadata field, not the procedure body, so opaque procedure files are fine.
- **Sunset format** — SemVer version string, same format as `introduced_in`. An entry expires when current ductus ≥ `sunset_after`. Omitted/null means "active indefinitely." Mirrors `introduced_in` for self-consistency; matches the mental model "active across `[introduced_in, sunset_after)`." Calendar dates decouple from release cadence; count-of-releases is just an indirect encoding of the same field. Default sunset policy is a maintainer-authoring convention documented separately, not part of this field's type.
- **Sunset archive location** — `CHANGELOG.md` at the repo root. Paths disambiguate; the runtime crate's `runtime/CHANGELOG.md` stays where Cargo expects it. Assumption: adopters who hit a removed convention with no active migration browse the repo root first; that's where the recipe-of-record belongs. Category separation: `framework/` holds in-force conventions, the repo-root `CHANGELOG.md` holds the historical record. Append-only, one file, easy to grep — no directory traversal needed. When a migration sunsets: its procedure file is appended to `CHANGELOG.md` under a heading, the procedure file is deleted, and the TOML entry is removed in the same commit. The bootstrap loop never reads `CHANGELOG.md` — it's for humans, not tooling.
- **Back-filled migration values** — hybrid: accurate `introduced_in` per git history (one-time `git log` for six entries), uniform `sunset_after = registry_introduction_version + 2 minor versions` for the back-fill. Accurate `introduced_in` makes the eventual CHANGELOG entries read correctly. Uniform sunset because we lack per-entry signal; +2 minor versions gives adopters two release cycles of grace. Spec doesn't pin the actual `registry_introduction_version` — acceptance criterion phrases it relative to whatever release ships the registry. As built, the uniform value was 0.10.0, three minors after `gvrn-v0.7.3`, the release that first shipped the registry, rather than two (AC4).
- **Batch behavior** — single prompt at start listing all pending migrations with `introduced_in` annotation, then run sequentially. `last_applied` updates after each migration completes (not at the very end), so an aborted batch resumes from the next run. Inner per-migration interactions (e.g., spec-and-plan's per-file Y/n) are unaffected — the outer prompt only gates "attempt the batch." Declining the outer prompt emits a warning summary and makes no filesystem changes.
- **`/audit` detection mechanism** — Family 10 runs static checks: no-orphan-procedure-files (every `framework/migrations/{id}.md` has a TOML entry), no-stale-target-paths (every registry/archive `target_paths` refers to something absent from current `framework/`), no-broken-procedure-references (every TOML `procedure_file` points at an existing markdown file), since 2026-11-17, no-overdue-sunset (10e, owner decision), and, since task 19, no-duplicate-ids (10f, owner decision, 2026-11-18). The "removal must ship with a migration" enforcement is out of scope, with no follow-on (owner decision, 2026-11-19), because it's fundamentally a diff operation against HEAD, not a static-state check. Family 10 reads history in one place only: task 15's 10d compares the registry's order with the order its entries were added, which no current-state check can see (048's review, 2026-10-29), so it needs a full-depth checkout.
- **Removal target type taxonomy** — no type field; per-entry free-form is sufficient. Motivation evaporated when Q4 dropped the declarative DSL (no defaults to drive). No consumer benefits from a type enum: the bootstrap loop and Family 10 both read `target_paths` + `procedure_file`, and humans reading `CHANGELOG.md` get the human-readable framing from the `summary` string. Adding a taxonomy creates maintenance cost (categorizing each new entry, evolving the enum) without a corresponding consumer.
- **Interaction with 022 (runtime primitive vs host-side)** — host-side procedure at v1; no new primitive. Migration loop contains user prompts and per-file branching, which is host-side territory by 022's boundary. The mechanical TOML-read is the only new operation and doesn't warrant a primitive by itself. Defer promotion until Family 9 (`primitive-promotion-candidates.sh`) detects the same pattern across multiple procedure files — then promote whatever small operation duplicates (atomic-rename, semver-compare, registry-read).
