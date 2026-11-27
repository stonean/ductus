# 027 — Bootstrap Migration Registry Plan

Implements [027 — Bootstrap Migration Registry](spec.md).

## Overview

The implementation drives adopter cleanup from one registry-driven loop, the `## Pre-run Migrations` section of `framework/bootstrap/ductus-procedure.md`: each convention removal is a TOML entry plus a small markdown procedure file, and the loop is the only bootstrap step that applies one. Sunset is delivered via a `CHANGELOG.md` at the repo root and a Family 10 audit that enforces registry/state consistency.

No new runtime primitive at v1 (Q11). Per-entry idempotency is an invariant each procedure file owns: a re-run changes no file and an absent target emits nothing, while six procedures report a converged target in one line (AC21).

## Technical Decisions

### Registry schema (`framework/migrations.toml`)

TOML array-of-tables, one entry per active migration:

```toml
[[migrations]]
id = "spec-and-plan-sunset"
introduced_in = "0.4.0"
sunset_after  = "0.X.0"  # chosen per entry when it is written
summary       = "Rename specs/*/spec-and-plan.md → specs/*/spec.md (lightweight-track sunset)"
target_paths  = ["specs/*/spec-and-plan.md"]
procedure_file = "framework/migrations/spec-and-plan-sunset.md"
```

Field semantics:

- `id` — slug, stable, lowercase-hyphenated. The `.ductus/config.toml` `[migrations].last_applied` references this string.
- `introduced_in` — SemVer string, the release the removal shipped in.
- `sunset_after` — SemVer string or omitted. Entry expires when current ductus ≥ `sunset_after`. Omitted means "active indefinitely."
- `summary` — one-line human-readable description of the entry, never printed; the post-scaffolding summary carries the lines each applied procedure emits.
- `target_paths` — array of paths or globs (relative to adopter project root). Family 10's 10b holds each `framework/`-prefixed one absent from current `framework/`; an adopter-relative one cannot be checked from this repository (`scripts/audit/migration-coverage.sh`'s header).
- `procedure_file` — path (relative to repo root) to the markdown procedure file. Always present; there is no declarative-step DSL (Q4).

Ordering rule (Q3): by `introduced_in` SemVer ascending, lexicographic tie-break on `id`. File order in the TOML is not authoritative: the order comes from `introduced_in` and `id`, so where an entry sits in the file carries no meaning.

### `.ductus/config.toml` `[migrations]` section

```toml
[migrations]
# Slug of the newest migration applied. Bootstrap runs only entries newer
# than this. Absent field means "no migrations applied" — runs every active entry.
last_applied = "substitute-seed-placeholders"
```

An absent section reads as "no migrations applied". A greenfield run — no config file at any tier and no spec root — skips the batch and its prompt and seeds `last_applied` directly, because the prompt asked a brand-new adopter to approve every active entry, and declining it left a warning that was false and a prompt on every later run. A first adoption does the same, which Pre-flight Checks decides from finding no signal an earlier run leaves (scenario `a-first-adoption-runs-no-migration`).

Stale-reference behavior (edge case from spec): if `last_applied` references an id that no longer exists in the active registry (sunsetted since the adopter's last run), bootstrap treats the field as "before the oldest active entry," runs every active entry, and emits a warning naming the retired id with a pointer to `CHANGELOG.md` (`framework/bootstrap/ductus-procedure.md` §Stale-reference behavior).

### Procedure file shape (`framework/migrations/{id}.md`)

Each file is a small markdown procedure with a fixed top-level shape. The bootstrap loop reads the file and executes its prose as the active step's body. Conventions:

```markdown
# {id}

**Introduced in:** {ductus version}
**Summary:** {one-line summary}

## Procedure

1. **Idempotency check.** {How to detect "already migrated" or "target absent" — exit silently with no summary line.}
2. {Migration step 1 — typically a prompt + action.}
3. {Migration step N.}
4. **Summary line.** Report `{summary text}` in the post-scaffolding summary.
```

The procedure file owns idempotency; the bootstrap loop does not enforce it. The shape above puts the check first, and six of the fifteen procedure files name their step 1 so; the other nine open by resolving or locating their target or, in `generator-primitives`, by removing retired scripts (each file's first numbered step, read 2026-11-19), and AC21 states what a re-run of each emits. Family 10 does not verify idempotency, which static state cannot show.

### Bootstrap loop placement and shape

The loop is the `## Pre-run Migrations` section of `framework/bootstrap/ductus-procedure.md`, which `framework/bootstrap/ductus.md` reaches at its step 3a, after the archive is extracted and before the first manifest pass. It sits there because it reads the registry and each procedure file from the extracted archive, and it converts what an earlier release left behind before this release's files land (`framework/bootstrap/ductus.md` §Instructions, its **Pre-run Migrations** bullet). Its shape:

```markdown
## Pre-run Migrations

Take the pre-run `git status --porcelain -- specs/` first, and again when the section ends.

1. Read `framework/migrations.toml` from the fetched archive. Abort with
   `Failed to read framework/migrations.toml; cannot run pre-run migrations.` when it is
   missing or malformed. Then apply the Duplicate-id and reference-integrity guard: before
   any later step, abort the loop with one line for each failure —
     - `migration {id}: duplicate id in framework/migrations.toml`
     - `migration {id}: unsafe id or procedure_file` (`id` not a slug, or `procedure_file`
       not exactly `framework/migrations/{id}.md`)
     - `migration {id}: procedure file missing from archive` (probed only for an entry
       that passes the rule above)
   `last_applied` keeps its prior value.
2. Resolve the active config file once; read its `[migrations].last_applied`
   (absence is null).
3. Filter the registry to entries where `introduced_in` is newer than `last_applied`'s
   (SemVer compare, lex tie-break on id) AND the current ductus version is less than
   `sunset_after` (or it is absent).
4. A greenfield run or a first adoption seeds `last_applied` with the newest passing
   entry's id and applies nothing. Otherwise an empty filtered list emits nothing and
   proceeds.
5. Prompt once: `N framework migrations are pending ...  Apply now? (Y/n)`.
6. On decline: emit the skipped-migrations warning; no filesystem changes.
7. On confirm, for each entry in filter order: read `framework/migrations/{id}.md`,
   execute its `## Procedure`, then update `[migrations].last_applied = "{id}"`
   atomically; a procedure that aborts halts the loop, and the next run resumes.
8. After the loop, invoke `check-orphaned-references` once and report each finding,
   attributing it against the staging registry; the batch does not halt.
```

`ductus.md` holds the step 3a pointer and the `[migrations]` schema in §Project Configuration.

### SemVer comparison (no new primitive)

SemVer compare is a small shell helper, not a runtime primitive. Two options the host can use:

- `sort -V` on a two-line input. Run on macOS `sort` (2.3-Apple) on 2026-11-19, `printf '0.10.0\n0.4.0\n' | sort -V` orders `0.4.0` first, as SemVer does; GNU `sort -V` support is an assumption, unrun here.
- A 10-line Python one-liner via `python3 -c 'from packaging.version ...'` when `sort -V` is insufficient for pre-release tags: the same `sort -V` run orders `0.4.0` before `0.4.0-rc1`, the reverse of SemVer, which puts the pre-release first.

In the bootstrap, the loop's filter is SemVer compare's only consumer, in the consolidated `## Pre-run Migrations` prose rather than a script: no procedure file compares versions, and pre-flight's migration-floor check looks an id up in a registry rather than comparing versions (`framework/bootstrap/ductus.md`, **Source resolution**'s *Check the migration floor*). Family 10's 10d and 10e compare versions in their own Python (`scripts/audit/migration-coverage.sh`).

If, after implementation, a SemVer helper proves common across multiple procedure files or scripts, Family 9 of `/audit` (`primitive-promotion-candidates.sh`) will surface it and we promote then.

### Sunset commit shape

When the release reaches an entry's `sunset_after` (current ductus ≥ `sunset_after`), Family 10's check 10e fails until the entry is archived, in one commit:

1. Delete the `[[migrations]]` entry from `framework/migrations.toml`.
2. Append it to `CHANGELOG.md`'s `## Archived migrations` at the repo root:

   ```markdown
   ### {id} (introduced {introduced_in}, sunset after {sunset_after})

   **Summary:** {the procedure file's summary}

   **Target paths:** `{each target_paths entry}`, …

   {the procedure file's steps}
   ```

3. Delete `framework/migrations/{id}.md`.
4. `/audit`'s Family 10 verifies steps 1 and 3 (10a, 10c, 10e) and the archived entry's target paths (10b, which reads the `**Target paths:**` line and fails an archived entry without one). It does not verify that the recipe text was appended.

`CHANGELOG.md` holds seven archived entries (seven `###` headings under `## Archived migrations`, counted 2026-11-19): `skills-to-workflows` and `workflow-filename-rename` archived by `2fcb5f93` (043, 2026-07-22), and five archived by `c061c1f0` on 2026-11-17 when 10e was added (owner decision).

### Family 10 design (`scripts/audit/migration-coverage.sh`)

Shell script under the existing `scripts/audit/` directory. Follows the contract `scripts/audit/README.md` §Contract gives every family: stdout findings, exit 0/1, read-only. The sketch below shows the first three checks; the script's header states every one. 10d holds each entry to sort after every entry added before it, reading the registry's git history; 10e fails an active entry past its `sunset_after` against the repository's `version` (owner, 2026-11-17); 10f fails an `id` two entries share:

```bash
# 10a no-orphan-procedure-files
# Parse each entry's procedure_file with tomllib (a path in a comment names no entry).
# Every framework/migrations/*.md must be one of them.

# 10b no-stale-target-paths
# Parse target_paths from migrations.toml AND from CHANGELOG.md archived entries.
# For each path/glob, verify it does NOT exist in current framework/.
# (Glob expansion via find; literal paths via test -e.)

# 10c no-broken-procedure-references
# Parse procedure_file from each migrations.toml entry.
# Verify each path exists.
```

CHANGELOG.md parsing for 10b: each archived entry under `## Archived migrations` opens with a `### {id} (…)` heading and keeps its target paths on a `**Target paths:**` line of backticked paths, which 10b checks as it does an active entry's. An archived entry with no such line is a finding, so a format change fails loudly rather than leaving entries unchecked.

Wiring: append one line to `scripts/audit/run-all.sh` after the Family 9 invocation:

```bash
run_check "Family 10 — migration coverage" "scripts/audit/migration-coverage.sh"
```

And append item 10 to `framework/reference/audit.md`, matching the existing pattern. No new MCP primitive — `run-all.sh` is the orchestrator; `/audit` itself invokes it via `run-generator` (`framework/commands/audit.md` step 1).

### CHANGELOG.md shape

`CHANGELOG.md` at the repository root opens by saying it is the adopter-facing record of removed conventions, each entry a recipe for an adopter who skipped past an entry's sunset, links this spec, and points at `runtime/CHANGELOG.md` for the runtime crate's own release notes. Under `## Archived migrations`, a paragraph says why entries leave the registry and that 10b holds their target paths absent, and each archived entry follows in the shape **Sunset commit shape** gives.

It is a separate file because no step of the bootstrap reads it (AC15): an adopter past an entry's sunset applies that recipe by hand, while the runtime crate's release notes describe the binary, not an adopter's layout. Each archive commit appends its entries below the last, so the oldest archived sit at the top: `2fcb5f93`'s two, then `c061c1f0`'s five.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/migrations.toml` | Create | Registry; one entry per active removal |
| `framework/bootstrap/ductus-procedure.md` | Modify | The `## Pre-run Migrations` section: the registry-driven loop, its guard and its step 8 check |
| `framework/bootstrap/ductus.md` | Modify | The step 3a pointer to Pre-run Migrations; the `.ductus/config.toml` `[migrations]` section in the §Project Configuration schema |
| `scripts/audit/migration-coverage.sh` | Create | Family 10 — its checks, 10a to 10f |
| `scripts/tests/test-migration-coverage.sh` | Create | Cases for 10a to 10f, each failing when its check is removed |
| `scripts/audit/run-all.sh` | Modify | Append Family 10 invocation |
| `framework/reference/audit.md` | Modify | Append item 10 for Family 10 |
| `CHANGELOG.md` | Create | Adopter-facing archive: each archived entry's recipe and its target paths |
| `specs/026-framework-self-audit/scenarios/family-10-migration-coverage.md` | Create | Cross-spec impact: 026 is `done`; Family 10 extension lands as a scenario with back-link to 027 |
| `specs/027-bootstrap-migration-registry/spec.md` | Modify | Status `clarified → planned` at the end of this phase, then `→ in-progress` and `→ done` during implementation |

The runtime write boundary that `/ductus:implement` derives from git history will include the above plus incidental files, such as `CHANGELOG.md` entries on this spec's commits.

## Trade-offs

### Considered: declarative-step DSL for simple migrations (rejected)

Q4. A small DSL of `rename` / `delete` / `regex-replace` step types could express the gitignore-marker migration in two TOML lines. Rejected because five of the six back-filled migrations had twists that would require extending the DSL until it became a programming language: skip-if-both-exist (`governance-config-rename`), per-file Y/n (`spec-and-plan-sunset`), pinned-files exclusion (`skills-to-workflows`, `workflow-filename-rename`), and a batch prompt with collision handling (`rule-files-relocate`), each in its recipe in `CHANGELOG.md`'s Archived migrations; only `gitignore-marker-rename` had none. Markdown procedure files are honest about being prose; the TOML stays pure metadata.

### Considered: `apply-migrations` runtime primitive (deferred)

Q11. A primitive that reads the registry, computes the filter, and dispatches procedure files could replace the bootstrap loop's prose. Deferred to a follow-on once Family 9 (`primitive-promotion-candidates.sh`) detects pattern duplication that justifies the maintenance overhead. The loop asks the operator, once for the batch and again wherever a procedure prompts per file, and executes each procedure's prose (`framework/bootstrap/ductus-procedure.md` §Pre-run Migrations, steps 5 and 7.2), which only the host can do: a primitive reaches the operator only by returning a gate for the host to route (`ductus gate-confirm --help`), so one replacing the loop would still hand every prompt and every procedure back to the host.

### Considered: git archaeology inside Family 10 (rejected)

Q9. A check that walks `git log -- framework/templates/ framework/commands/` to detect deletions without a corresponding registry entry would catch "removal without migration" mistakes automatically. Rejected because (a) it couples the check to commit history shape (rebases, squashes lose signal), and (b) it requires distinguishing "convention removal" from "internal refactor that renamed a file" — open-ended classification. 10d does read history, but only `framework/migrations.toml`'s own, to order its entries, which needs no such classification. The cleaner alternative, a separate pre-commit hook that compares a commit's diff against registry changes, is out of scope with no follow-on (spec.md's Out of scope list; owner decision, 2026-11-19).

### Considered: unifying `enforce-manifest`'s legacy-cleanup work with the registry (accepted)

Leaving the legacy `skills/` directory and legacy workflow filenames to `enforce-manifest` would have been coherent but left two places to look for "what does bootstrap clean up on every run?". Decision: register both, and keep `enforce-manifest` generic. As built, `runtime/src/primitives/enforce_manifest.rs`'s module doc states that adopter cleanup of historical conventions is owned by the registry-driven Pre-run Migrations loop, and the primitive removes whatever files the caller's `expected` list omits.

## Known Limitations

### Sunset values are chosen per entry, with no signal behind them

The six back-filled entries, which shared one uniform `sunset_after`, are all archived in `CHANGELOG.md`; each entry's `sunset_after` is chosen when it is written, and 10e fails one past it. Nothing derives how long a migration needs to stay active, so the manual recipe in `CHANGELOG.md` covers an adopter who skips past one.

### "Removal must ship with migration" is discipline-only at v1

Family 10 verifies registry/state consistency, not registry/diff coverage. A maintainer who deletes `framework/templates/foo.md` without adding a `[[migrations]]` entry breaks adopters silently — Family 10 only catches the divergence after the registry diverges from filesystem state, not at commit time. No check covers it and none is planned: a pre-commit diff check is out of scope, with no follow-on (spec.md's Out of scope list; owner decision, 2026-11-19).

### CHANGELOG.md append at sunset is maintainer-discipline

When a migration sunsets, three actions land in one commit: delete TOML entry, append CHANGELOG entry, delete procedure file. 10e makes the sunset itself not depend on anyone remembering it: before it existed, five entries were still registered at version 0.59.11 when `c061c1f0` archived them, four with `sunset_after` 0.10.0, 49 minor releases past it (0.59 less 0.10), and `workflows-sunset` with 0.25.0, 34 past it (each entry's `CHANGELOG.md` heading, and `git show c061c1f0:version`). Family 10 still cannot verify that the CHANGELOG received the recipe: a maintainer who deletes the TOML entry and procedure file without appending breaks the "adopters skipping past sunset have a recipe" promise. Code review is the gate for that step.

### Procedure file content format is loosely specified

The procedure file shape above is a convention, not a parsed schema. A maintainer who writes a procedure file without a "Summary line" step would silently break the post-scaffolding summary contract. Acceptable for v1 because (a) the existing migration prose is the back-fill source, (b) procedure files are small enough that code review catches deviations, (c) `grep -rn 'framework/migrations/' runtime/src` finds three lines, each a `//!` module doc comment (run 2026-11-19), so the runtime never parses procedure files (the bootstrap loop reads and dispatches; the LLM/host executes them). If procedure files proliferate, a parseable shape (matching 022's parseable command convention) is a natural promotion path.
