# 017 — Derive, Don't Ask Plan

Implements [017 — Derive, Don't Ask](spec.md).

## Overview

Discipline-cleanup pass across templates, commands, constitution, validate, and the bootstrap installer. Adds a help-table generator (`gen-help-tables.sh`), frontmatter `dependencies` derived from body links (today the runtime's `derive-dependencies` primitive), one new pre-commit hook (`.githooks/pre-commit`), one new install script (`scripts/install-hooks.sh`), one new rule file (`framework/rules/configuration-cross.md`), and one new adopter hook surface under `framework/bootstrap/hooks/`. Migrates existing dogfood specs only insofar as adding inline links to spec bodies for any frontmatter dependency not already linked, so the derivation does not strip them. Frontmatter field deletions are not retroactively migrated; the open-schema rule ignores the stale fields. Collapses the twin constitutions to a single canonical file at `framework/constitution.md` and deletes root `constitution.md`.

The work is intentionally additive in mechanism (add generators and hooks) and subtractive in surface (remove fields, sections, and `--fix` mode). No new agent-facing concepts are introduced.

## Technical Decisions

### Single canonical constitution at `framework/constitution.md`

Per spec Q2: collapse the twin constitutions. Root `constitution.md` is deleted. Root `CLAUDE.md` updates `@import constitution.md` → `@import framework/constitution.md`. Root `README.md` link to `constitution.md` updates likewise. `/ductus`'s install-time remap (`framework/constitution.md` → `constitution.md` at the adopter root) was unchanged by this spec; spec 044 later moved the adopter's copy to `.ductus/constitution.md` (`AGENTS.md` §Project Structure). The AGENTS.md "mirror constitutions" instruction is removed.

Alternative considered: generator with section markers — rejected because the historical divergence between the two files was zero at decision time, so the discipline trap was guarding nothing. Pre-emptive scaffolding for hypothetical divergence violates the "don't design for hypothetical future requirements" guidance in the system prompt.

### `--fix` mode removed entirely from `/validate`

Per spec Q1: checkboxes are flipped only by `/implement` (which marks parents and sub-items together at verification, `framework/commands/implement.md:91, 99`) and by `/clarify` (which moves resolved questions section-to-section). Hand-implementation that bypasses `/implement` is treated as out-of-framework — no fix-up tool ships for it. Combined with the title-field removal, `--fix` mode has nothing to do, so the entire flag is deleted from `/validate`.

### Body inline links are authoritative for `dependencies`

Per spec Q7: frontmatter `dependencies` is fully derived from inline markdown links to sibling specs in the spec body, excluding code fences. The runtime's `derive-dependencies` primitive walks every `specs/*/spec.md`, computes the union, and rewrites the frontmatter list. The author maintains links in the body — no place to author the frontmatter list directly.

Sync runs in two places:

1. **Pre-commit hook** — primary sync. Runs `ductus derive-dependencies --write` on every commit and stages the result.
2. **Command-entry recompute** — `/clarify`, `/plan`, `/implement`, `/amend`, and `/target` recompute on entry as an idempotent safety net for uncommitted body edits.

Removal mechanism: remove the inline link, or move it inside a code fence (the scanner ignores fenced-code links). To mention a spec without depending on it, use a bare slug per Q4.

### Pre-commit hook architecture

ductus repo's `.githooks/pre-commit` orchestrates the generators and derivations, among them:

1. `scripts/gen-claude-commands.sh`
2. `scripts/gen-help-tables.sh` (new, this spec)
3. `ductus derive-dependencies` (the derivation this spec introduced)

Hook installation via `scripts/install-hooks.sh` (sets `git config core.hooksPath .githooks`). Idempotent — safe to run repeatedly.

Generators run unconditionally on every commit (no file-change gating). Trades a fraction of a second per commit for a one-line implementation that can't get the gate logic wrong.

### Adopter hook surface and `/ductus` integration

Per spec Q7 expansion: `framework/bootstrap/hooks/pre-commit`, an adopter-owned stub, and `framework/bootstrap/hooks/ductus-pre-commit`, which the stub calls, ship with the framework. The shipped hook runs only adopter-relevant derivations — `ductus derive-dependencies` and `derive-references`. Slot is extensible.

`/ductus` adds a new "Hook installation" section between **Per-Agent Scaffolding** and **Post-Scaffolding Output**. On every run it detects state and acts as below. **Superseded in part by `018-adopter-owned-pre-commit`**: `.githooks/pre-commit` is now an adopter-owned `create`-strategy stub with no sentinel, and the sentinel and the `update` strategy belong to `.githooks/ductus-pre-commit` (this spec's `data-model.md`, *Post-018*).

| Detected state | Action |
| --- | --- |
| `core.hooksPath` unset and `.githooks/pre-commit` absent | Install both, set `core.hooksPath .githooks`, report installed |
| `.githooks/pre-commit` exists from a prior `/ductus` run (detected by a sentinel comment in the file) | Overwrite (`update` strategy, pinnable via `.ductus/config.toml`) |
| `core.hooksPath` points elsewhere, OR `.githooks/pre-commit` exists without the sentinel comment, OR husky/lefthook/`.pre-commit-config.yaml` detected | Skip install; report a warning with a manual integration snippet; continue |

The sentinel comment is a single line near the top of the shipped hook (e.g., `# managed-by: ductus`) that the detection logic looks for to distinguish a ductus-installed hook from a hand-rolled one. The same sentinel survives `/ductus` updates because it's part of the shipped file.

### Generated artifacts use marker comments

The generated content blocks — `framework/commands/help.md`'s command tables — are bounded by HTML marker comments:

```markdown
<!-- generated:commands-pipeline:start -->
| Command | Pipeline Gate | Description |
| --- | --- | --- |
| ... |
<!-- generated:commands-pipeline:end -->
```

The generator finds the markers, splices the regenerated content between them, and leaves everything outside the markers untouched. This lets `help.md` retain hand-authored prose around the generated tables.

For `framework/commands/help.md`, each of the five command tables (Pipeline, Elaborate, Brownfield, Orient, Bootstrap) gets its own marker pair. The generator emits one block per table from the appropriate command-source frontmatter.

### `/validate` schema and check changes

The frontmatter schema in `framework/constitution.md` §text-first-artifacts is updated to:

- **Spec files** (`spec.md`): required = `status`, `dependencies`. Removed = `tags` (deleted).
- **Scenario files** (`scenarios/{slug}.md`): required = `section`. Replaced from `spec-ref`. Removed = `tags`.
- **Other artifacts** (`plan.md`, `tasks.md`, `data-model.md`, `research.md`): no required frontmatter. The `title` field is deleted from all templates.

Validate check changes:

- PKM `title` advisory check: **removed**.
- `tags` advisory check: **removed**.
- `spec-ref` hard fail check: **renamed** to `section`.
- `--fix` mode: **removed entirely** (Fix Mode section deleted).
- Help-equivalence check: **changed from per-row description match to dry-run-of-generator check** — `/validate` runs `gen-help-tables.sh --dry-run` and reports if it would produce a diff.
- New advisory: when a body inline link to a sibling spec is found that is not yet in the generator-managed `dependencies` list, surface as `Body link to {slug} not in dependencies; the next commit will add it.` (Generator runs at commit, so this is informational — a hint that the frontmatter is currently stale.)

### Configuration rule file

Per spec Q5/Q6: a single file at `framework/rules/configuration-cross.md` with `CFG-` prefix and `CONST`/`ENV`/`FLAG` categories — `FLAG` (feature flags) added 2026-10-19 by the rule-file review. Format `CFG-{CONST|ENV|FLAG}-{NNN}`. The first version shipped seven rules (`CFG-CONST-001`–`003`, `CFG-ENV-001`–`004`); the current rule set, with each rule's tier and obligation, is enumerated in `data-model.md` § Rule set rather than repeated here.

Rule format details (Statement, Rationale, Verification, Source, ID stability) declared in `data-model.md` alongside this plan, mirroring the precedent set by `specs/008-security-rules/data-model.md`.

Adopter shipping: added to `framework/bootstrap/ductus.md`'s Shared Files manifest under "ductus-owned shared files (strategy: update)" — alongside the security rule files.

### Migration of existing dogfood specs

Done specs were not migrated for the deleted fields — stale `title:`, `tags:`, `spec-ref:`, and `track:` fields remain. `/validate` stops checking them, so they cause no findings.

The one exception that needs active migration: **inline links for dependencies**. Each existing spec's frontmatter `dependencies` list must be reflected in the spec body via inline markdown links — otherwise the first derivation strips them. Migration task: for each existing spec, scan the body for inline links to declared deps; add a "References" list at the bottom of the spec body for any declared dep not already inline-linked.

### CI safety net

A GitHub Actions workflow (`.github/workflows/generators.yml`) runs the generators and derivations on PR and fails on any diff. Catches contributors who never ran `scripts/install-hooks.sh`.

A second workflow file (`framework/templates/ci/adopter-generators.yml`) is a template kept in this repository, never installed: no Shared Files manifest row writes it into an adopter's tree, and nothing in `README.md` or `docs/` points at it (measured 2026-10-20, scenario `the-adopter-ci-template-can-run`). Adopters who want CI enforcement copy it into their own `.github/workflows/`. Auto-shipping CI files via `/ductus` would require platform detection (GitHub Actions vs. GitLab CI vs. Buildkite) that's out of scope for this spec.

Both workflows reference their actions by full commit SHA with the release tag in a same-line comment (`BE-DEPS-006`), the template included, so an upstream that moves a tag changes nothing either one runs. `generators.yml` also installs the Rust toolchain through `dtolnay/rust-toolchain`, whose `toolchain:` input names the release; rustup checks each component against the sha256 that release's channel manifest records, and does not verify the manifest's signature (022's plan, **Pinned inputs**, cites the sources). The template's first step after the checkout installs the runtime its other steps call, because an adopter's checkout carries none: `/ductus` keeps it in a per-machine store and links it from the gitignored `.ductus/bin/` (`framework/bootstrap/ductus.md` §Pointer materialization). The step downloads the x86_64 Linux release archive at the version its `DUCTUS_VERSION` names, checks it against the `DUCTUS_SHA256` beside it before anything runs, and puts the binary on `PATH` from outside the work tree, so the sync check cannot see it. The download is bounded by `DOWNLOAD_CONNECT_TIMEOUT_SECONDS`, `DOWNLOAD_MAX_TIME_SECONDS` and the step's `timeout-minutes` (`BE-TIMEOUT-001`), and is not retried, so `BE-RETRY-001` has nothing to govern: a failed download fails the job, which can be re-run. It is fetched over HTTPS only, at TLS 1.2 or later (`--proto '=https' --tlsv1.2`, `BE-DATA-001`), with curl's certificate-chain and hostname verification left on, never `-k` or `--insecure` (`BE-DATA-011`). That needs no Rust or cargo, which is what the release binaries are for (owner decision, 2026-10-20, scenario `the-adopter-ci-template-can-run`). A release cannot carry its own archive's hash, so the template always names a release already published. Their update process is this repository's weekly currency check: the `pins` check of `scripts/check-runtime-currency.sh`, run every Monday by `runtime-currency.yml`, reads the template along with the repository's own workflows and fails when a pin is behind its action's latest release or no longer names the commit its tag does, when `DUCTUS_VERSION` is behind this repository's latest release, and when `DUCTUS_SHA256` is not the sha256 that release published. A bump to the template changes only this repository's copy; an adopter who copied it re-copies it.

## Affected Files

### Templates

| File | Action | Purpose |
| --- | --- | --- |
| `framework/templates/spec/spec.md` | Modify | Remove `title:` from frontmatter |
| `framework/templates/spec/plan.md` | Modify | Remove `title:` from frontmatter; remove "Open Questions Resolved" section |
| `framework/templates/spec/tasks.md` | Modify | Remove `title:` from frontmatter; remove `[simple]` marker documentation |
| `framework/templates/spec/data-model.md` | Modify | Remove `title:` from frontmatter |
| `framework/templates/spec/research.md` | Modify | Remove `title:` from frontmatter |
| `framework/templates/spec/scenario.md` | Modify | Replace `spec-ref:` with `section:`; remove `title:` from frontmatter |

### Commands

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/specify.md` | Modify | Remove tag prompt step; remove title fill-in; remove tags-related instructions |
| `framework/commands/clarify.md` | Modify | Remove title check; remove tags advisory; add cross-spec scan step; add deps recompute on entry |
| `framework/commands/plan.md` | Modify | Remove title fill-in; remove "Open Questions Resolved" reference; remove `[simple]` marker proposal step; reframe Affected Files as planning aid; add cross-spec scan; add deps recompute on entry |
| `framework/commands/implement.md` | Modify | Remove `[simple]` marker reading; replace Affected Files boundary with `git diff` derivation; add cross-spec scan; add deps recompute on entry |
| `framework/commands/groom.md` | Modify | Remove `[promote-to-rule]` prefix instruction; specify "always re-walk every item not migrated on next pass" |
| `framework/commands/analyze.md` | Modify | Remove Fix Mode section; remove PKM title check; remove tags advisory; rename `spec-ref` check to `section`; change help-equivalence to dry-run-of-generator check; add body-link-vs-deps advisory; add configuration rule file to rule-file list |
| `framework/commands/amend.md` | Modify | Add deps recompute on entry |
| `framework/commands/target.md` | Modify | Remove `tags` from frontmatter parse; add deps recompute on entry |
| `framework/commands/help.md` | Modify | Add HTML marker comments around the five command tables; tables become generator output |
| `framework/commands/log.md` | No change | — |
| `framework/commands/status.md` | No change | — |

### Constitution

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | Remove `tags` from frontmatter schema; remove "Starter Tag Vocabulary" table; remove `[simple]` marker bullet from §cost-levers; replace §constants and §env-vars with one-line pointer to `framework/rules/configuration-cross.md` |
| `constitution.md` (root) | Delete | Collapse to single canonical at `framework/constitution.md` |

### Project root files

| File | Action | Purpose |
| --- | --- | --- |
| `CLAUDE.md` (root) | Modify | Update `@import constitution.md` → `@import framework/constitution.md` |
| `README.md` (root) | Modify | Update `constitution.md` link to `framework/constitution.md` |
| `AGENTS.md` (root) | Modify | Remove "mirror constitutions" instruction; remove "run gen-claude-commands.sh" instruction |

### Generators (new)

| File | Action | Purpose |
| --- | --- | --- |
| `scripts/gen-help-tables.sh` | Create | Rebuild the help.md command tables from each command's frontmatter |
| `scripts/install-hooks.sh` | Create | One-time `git config core.hooksPath .githooks`; idempotent |

### Hooks (new)

| File | Action | Purpose |
| --- | --- | --- |
| `.githooks/pre-commit` | Create | ductus repo's hook — runs the generators and derivations, stages outputs |
| `framework/bootstrap/hooks/pre-commit` | Create | Adopter-shipped hook stub; calls `framework/bootstrap/hooks/ductus-pre-commit`, which runs the derivations and carries the `# managed-by: ductus` sentinel |

### New rule file

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/configuration-cross.md` | Create | `CFG-CONST-NNN`, `CFG-ENV-NNN` and `CFG-FLAG-NNN` rules with Statement / Rationale / Verification / Source per data-model |

### Bootstrap installer

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | Add Hook Installation section; add `framework/rules/configuration-cross.md` to Shared Files (update strategy); add `framework/bootstrap/hooks/pre-commit` to Shared Files (create strategy) |
| `framework/bootstrap/configure/claude.md` | Modify | Add Bash permissions for hook install/run paths (`Bash(git config *)`, `Bash(.githooks/*)`, `Bash(scripts/gen-*)`) |
| `framework/bootstrap/configure/auggie.md` | Modify | Same permission additions in Auggie's format |

### CI

| File | Action | Purpose |
| --- | --- | --- |
| `.github/workflows/generators.yml` | Create | ductus repo CI: run the generators and derivations; fail on diff |
| `framework/templates/ci/adopter-generators.yml` | Create | Shipped template adopters can copy into their own `.github/workflows/` |

### Existing dogfood spec migration

| File | Action | Purpose |
| --- | --- | --- |
| `specs/000-016/spec.md` | Modify | Add inline links in body for any declared frontmatter dependency not already linked (so the first derivation does not strip them) |

(No frontmatter migration on done specs — see the trade-off below.)

### This spec's own artifacts

| File | Action | Purpose |
| --- | --- | --- |
| `specs/017-derive-dont-ask/spec.md` | Modify (final task) | Strip `title:` and `tags:` from frontmatter |
| `specs/017-derive-dont-ask/plan.md` | Modify (final task) | Strip `title:` from frontmatter |
| `specs/017-derive-dont-ask/tasks.md` | Modify (final task) | Strip `title:` from frontmatter |
| `specs/017-derive-dont-ask/data-model.md` | Modify (final task) | Strip `title:` from frontmatter |
| `runtime/tests/golden/plan-basic.jsonl`, `runtime/tests/golden/implement-basic.jsonl` | Modify | Re-blessed for the dependency safety net each command walks again as sub-step 1.1 (task 43) |

## Data Model

See `data-model.md` for the configuration rule file structure, ID format, category abbreviations, current rule set, and the new spec/scenario frontmatter schema after deletions.

## Trade-offs

- **Symmetric hooks across ductus and adopters.** Considered: command-entry recompute only, no adopter hook. Rejected because drift between commits is a real failure mode (a teammate pulls an out-of-date branch and reads stale deps without running a ductus command). The user explicitly pushed for symmetry — if hooks are right for ductus, they're right for adopters.
- **Adopter hook installs only when no existing hook system is detected.** Considered: always install (clobbering existing hooks). Rejected because adopter projects already use husky/lefthook/pre-commit-py and clobbering their setup is a pipeline-violating action. Skip-and-warn-with-snippet keeps the adopter in control.
- **No `gen-root-constitution.sh` generator.** Q2 collapsed the twin constitutions; the divergence the generator would have managed is gone.
- **Existing dogfood specs not migrated for deleted fields.** Stale `title:`, `tags:`, `spec-ref:`, `track:` fields remain in done specs; the open-schema rule ignores them. The cost is visual noise in old specs; the benefit is no rewrite of merged history.
- **CI templates ship but aren't auto-installed for adopters.** Auto-installing requires CI-platform detection (GHA vs. GitLab vs. Buildkite) that's out of scope. Adopters with CI enforcement opt in by copying the template.
- **The pre-commit hook runs all generators unconditionally** rather than gating on which files changed. Trades sub-second hook execution time for a hook script that can't have wrong gate logic.
- **`--fix` mode deletion is irreversible.** A user who relied on `/validate --fix` for hand-implementation flows loses that workflow. The framework's position becomes "use `/implement`," and the trade-off is documented in the spec's Q1 resolution. Rolling back requires another spec.
