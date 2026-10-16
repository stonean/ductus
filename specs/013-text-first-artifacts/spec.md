---
title: "013-text-first-artifacts — spec"
status: done
dependencies: [000-slash-commands, 007-govern-workflow, 012-multi-agent-govern]
tags: [format, migration, pipeline]
next-criterion: 19
---

# 013 — Text-First Artifacts

Declare `ductus`'s implicit "all artifacts are markdown" principle in the constitution, formalize spec metadata as YAML frontmatter, and migrate adopted projects to the new format on the next `/ductus` run.

## Problem

`ductus` has always treated every artifact — constitution, specs, plans, tasks, scenarios, rules — as plain markdown the agent edits with `Edit`. That stance is load-bearing in non-obvious ways: the agent's write path stays simple, PRs review glanceably, merge conflicts stay rare and human-resolvable, and adopting governance requires no bootstrap tooling beyond the AI agent itself. But the principle is implicit. As candidates for new artifacts surface (dependency graphs, audit trails), there is no declared rule for when structured storage is permitted and what constraints it must meet, so each one risks relitigating the question from scratch.

Spec metadata today (`**Status:** in-progress`, `**Dependencies:** 002, 005`) is bold-prefix text parsed by every consuming slash command via custom regex. The cost compounds: adding a metadata field means touching every parser; type semantics (lists, enums, dates) must be string-decoded ad hoc; cross-spec schema validation is impossible without per-command logic; and any external tool that wants to read `ductus` artifacts has to reimplement the parser. A move to YAML frontmatter — the de facto standard for markdown-with-metadata — preserves the text-first stance while unlocking structured types, schema-driven validation, and ecosystem compatibility (Quartz, Obsidian, Logseq, MkDocs, GitHub Actions, `yq`).

## Behavior

> **Signpost (post-017):** [017 — Derive, don't ask](../017-derive-dont-ask/spec.md) reversed three of
> this spec's decisions and left no record here until 2026-09-14. **(1) The `tags` programme is gone.**
> `fc737946` dropped the starter vocabulary from the constitution, `d355db91` stripped `tags: []` from the
> spec templates, and `1c50e2cb` removed the `/{project}:specify` prompt and the `/{project}:clarify`
> advisory — on the principle that a field an author must remember to fill is exactly the diligence
> dependency [§design-principles](../../framework/constitution.md#design-principles) rejects. The
> constitution now names `tags` a **stale field** that produces no findings. **(2) A scenario's required
> field is `section`, not `spec-ref`** (`d355db91`); `spec-ref` survives only as the legacy fallback the
> constitution names for pre-017 scenarios, and the hard fail fires only when _both_ are missing.
> **(3) The Quartz recommendation was dropped as broken** (`f1ada44d`), not merely reworded. The
> frontmatter block this spec introduced, its open-schema rule, and the migration all still stand.

### Principle Declaration

The constitution gains a new section declaring text-first artifacts as a guiding rule:

- All `ductus` artifacts are markdown by default. The agent reads and writes them with the same `Edit` flow used for code.
- Structured metadata lives in YAML frontmatter at the top of each markdown file; the document body remains markdown prose.
- Cross-artifact references use standard relative markdown links (`[label](../path.md)`), not wiki-links — this keeps PRs reviewable on GitHub and viewers like Quartz/Obsidian still resolve them.
- Source-of-truth artifacts are markdown. Structured derived views (SQLite caches, generated graph data, JSON indexes) are permitted only as gitignored build artifacts that consumers regenerate on demand. They never become the canonical record.
- Exceptions to text-first source-of-truth require an explicit constitutional amendment with stated rationale.

### Frontmatter Schema

The schema applies to **spec files** (`spec.md`) and **scenario files** (`scenarios/{slug}.md`). (As written it also named `spec-and-plan.md`, the lightweight track's combined document, retired by `023-govern-refinement`.) Other `ductus` artifacts (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan files, tasks files, rule files, README files) MAY include frontmatter when a specific consumer benefits, but are not required to. The schema is declared as a markdown table in `framework/constitution.md`, next to the text-first principle, and is the authoritative source for `/ductus:analyze` and any tooling.

**Required fields for spec files:**

- `status` — one of `draft`, `clarified`, `planned`, `in-progress`, `done`
- `dependencies` — list of spec slugs this feature depends on; empty list permitted

**Required fields for scenario files:**

- `spec-ref` — string identifying the parent spec and section the scenario elaborates (replaces the bold-prefix `**spec-ref:**` line). **Superseded by 017**, which renamed the field to `section` and narrowed it to the parent _section_ alone, the parent feature being implicit in the scenario's path. `spec-ref` is now the legacy fallback the constitution names for pre-017 scenarios, and is still read.

**Runtime-maintained field (spec files):**

- `next-criterion` — integer; the `AC{n}` label the next acceptance criterion will receive. Monotonically non-decreasing: deleting a criterion never lowers it, which is what keeps a retired label from being reissued to a different requirement without consulting git history. Maintained by the labelling pass, never hand-edited; absent on a spec whose criteria have never been labelled, which means "no labels assigned yet" rather than a defect. See [`scenarios/criterion-identifiers.md`](scenarios/criterion-identifiers.md) for the assignment rule, the backfill, and the audit invariant (`next-criterion` exceeds every label present in the body).

**Standard optional fields (specs and scenarios):**

- `tags` — **retired by 017; the three reinforcement points below were all removed.** As specified: a list of free-form strings used by graph-view consumers (Quartz, Obsidian, etc.) for cross-cutting groupings, treated as first-class for spec files through three reinforcement points: (1) bundled spec templates include `tags: []` so authors see the field at every new spec, (2) `/ductus:specify` prompts for at least one tag at creation and surfaces existing sibling specs' tags as suggestions, (3) `/ductus:clarify` flags missing or empty tags as an advisory finding at the `draft → clarified` transition (not a hard gate). The constitution publishes a starter vocabulary as guidance, not enforcement; new tags can be introduced as needed. Scenarios may also carry tags for graph-view consistency.

The schema is open: additional fields beyond those listed are permitted and ignored by uninterested consumers, leaving room for future metadata (`owner`, `target_release`, etc.) without coordinated parser changes.

### Slash Command Updates

Every slash command source in `framework/commands/` (and the regenerated `.claude/commands/ductus/` instances) that reads or writes spec metadata is updated to use frontmatter parsing instead of bold-prefix regex. At minimum this covers: `/ductus:status`, `/ductus:target`, `/ductus:clarify`, `/ductus:plan`, `/ductus:implement`, `/ductus:analyze`, `/ductus:groom`, `/ductus:specify`. Commands that don't read metadata are unaffected.

Templates (`framework/templates/spec/spec.md`, and `spec-and-plan.md` until `023-govern-refinement` deleted it) ship with the new frontmatter format so newly created specs use it from day one.

### Migration via `/ductus`

The next `/ductus` run in any adopted project performs the migration:

1. Precheck `git status --porcelain` scoped to `specs/` and `framework/templates/spec/` (or the project's equivalent). If dirty, refuse with a clear message instructing the user to commit or stash their in-flight changes, then exit. Unrelated in-flight work elsewhere in the tree does not block migration.
2. Walk the project's `specs/` directory and detect spec files (`spec.md`, `spec-and-plan.md`) and scenario files (`scenarios/{slug}.md`) using bold-prefix metadata (no frontmatter block present at the top of the file).
3. Convert bold-prefix lines (`**Status:**`, `**Dependencies:**` for specs; `**spec-ref:**` for scenarios; any other recognized fields) to YAML frontmatter at the top of the file.
4. Strip the now-redundant bold-prefix lines from the document body.
5. Leave non-spec artifacts (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan files, tasks files, rule files) untouched — frontmatter is not required for these.
6. Apply `ductus`'s standard `update`/`create`/`skip` strategy to the project's bundled spec and scenario templates and slash commands so they pick up the new format.
7. Pinned files (via `.ductus/config.toml`) are skipped — the adopter is responsible for their own migration of pinned files.
8. Print a summary of converted files. The user reviews the result via `git diff`, commits, or aborts via `git restore`. No backup directory is created — git is the recovery mechanism.

Migration is idempotent: re-running `/ductus` on an already-migrated project produces no further metadata changes.

### Rendering Convention

This repo's `README.md` documents `npx quartz` as the recommended viewer for browsing `ductus` artifacts as a graph. Quartz is recommended, not enforced; the artifacts work unchanged in Obsidian, Logseq, Foam, MkDocs, or no viewer at all. The point of the recommendation is to give adopters a default answer to "how do I see this as a graph?" without prescribing tooling.

### `ductus` Self-Migration

This repo dogfoods the principle: every existing spec under `specs/` in this repo is migrated to frontmatter as part of implementation. This repo has no `/ductus` to run on itself — the migration is manual work captured in `tasks.md`.

## Edge Cases

- **Spec with malformed bold-prefix metadata** (missing `**Status:**` line, typo in field name): migration logs a warning and skips the file; the user repairs manually before re-running.
- **Spec already partially migrated** (frontmatter present but body still has bold-prefix lines): migration completes the conversion idempotently — frontmatter wins, redundant body lines are removed.
- **Pinned spec files via `.ductus/config.toml`**: skipped during migration. The adopter receives a summary listing pinned files so they know which need manual conversion.
- **Project on an older `ductus` version**: `/ductus` always migrates to the current schema. There is no version negotiation; older projects pull current.
- **Spec with custom non-schema fields in bold-prefix form** (e.g., a project added their own `**Owner:**` line): migration preserves these as additional frontmatter fields. The schema permits unknown fields by design.
- **Spec created manually outside `/ductus:specify`** (e.g., direct file creation): the bundled template's `tags: []` placeholder is visible as a reminder, but no creation-time prompt fires. The advisory finding at `/ductus:clarify` catches missing tags before the spec advances to `clarified`.
- **Migrated specs with no tag signal**: migration adds frontmatter without populating `tags`. Backfill is organic — every subsequent `/ductus:clarify` pass on the spec is a chance to add tags.
- **Non-spec artifacts** (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan files, tasks files, rule files): migration leaves them untouched. They have no required schema and are not part of the lifecycle the schema models.
- **Scenario files using bold-prefix `spec-ref`**: migration converts the bold-prefix line to a `spec-ref` frontmatter field and removes the redundant body line. Scenarios remain status-less per the constitution; only `spec-ref` is required.

## Acceptance Criteria

- [x] AC1: `framework/constitution.md` declares the text-first artifacts principle in a new section, including the frontmatter requirement, relative-link rule, and structured-derived-view caveat.
- [x] AC2: The frontmatter schema is declared as a markdown table in `framework/constitution.md`, listing required fields per artifact kind (specs: `status`, `dependencies`; scenarios: `spec-ref`), standard optional fields (`tags`), and the open-schema rule for additional fields. **Superseded in part.** The table exists and is still the canonical declaration, and the spec-file required set is unchanged. Two of its enumerations moved: a scenario's required field is `section`, and `tags` is no longer a standard optional field — the constitution names it among the **stale fields** the open-schema rule tolerates without findings. The open-schema clause holds exactly. Retired by `017-derive-dont-ask` — see the signpost under **Behavior**. A field an author must remember to fill is the diligence dependency [§design-principles](../../framework/constitution.md#design-principles) rejects, which is 017's whole thesis.
- [x] AC3: The constitution declares the schema applies to spec files and scenario files only; other artifacts (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan files, tasks files, rule files) MAY include frontmatter when a consumer benefits but are not required to.
- [x] AC4: The constitution publishes a starter `tags` vocabulary as guidance (not enforcement). **Superseded.** Delivered as written; `fc737946` removed the vocabulary. The constitution's only surviving mention of `tags` is the open-schema rule naming it a stale field. Retired by `017-derive-dont-ask` — see the signpost under **Behavior**. A field an author must remember to fill is the diligence dependency [§design-principles](../../framework/constitution.md#design-principles) rejects, which is 017's whole thesis.
- [x] AC5: `framework/templates/spec/spec.md` uses YAML frontmatter and includes `tags: []` as a visible placeholder so authors see the field at every new spec. The same held for the spec-and-plan template until spec `023-govern-refinement` retired it. **Superseded in part.** The template uses YAML frontmatter, which is the load-bearing half and still holds. `d355db91` removed the `tags: []` placeholder as discipline-required frontmatter. Retired by `017-derive-dont-ask` — see the signpost under **Behavior**. A field an author must remember to fill is the diligence dependency [§design-principles](../../framework/constitution.md#design-principles) rejects, which is 017's whole thesis.
- [x] AC6: `framework/templates/spec/scenario.md` uses YAML frontmatter for `spec-ref` instead of bold-prefix. **Superseded in part.** The template uses YAML frontmatter rather than bold-prefix, which is what this criterion exists to assert and what still holds; `d355db91` renamed the field, so the template ships `section: "{Section name}"`.
- [x] AC7: `/ductus:specify` prompts for at least one tag at spec creation time, surfacing existing sibling specs' tags as suggestions; the author can decline (leaving the list empty) without blocking creation. **Superseded.** Delivered as written; `1c50e2cb` removed the prompt. `framework/commands/specify.md` carries no tag prompt today. Retired by `017-derive-dont-ask` — see the signpost under **Behavior**. A field an author must remember to fill is the diligence dependency [§design-principles](../../framework/constitution.md#design-principles) rejects, which is 017's whole thesis.
- [x] AC8: `/ductus:clarify` flags missing or empty `tags` as an advisory finding at the `draft → clarified` transition; the finding does not block the transition. **Superseded.** Delivered as written; `1c50e2cb` removed the check. `framework/commands/clarify.md` does not mention tags. Retired by `017-derive-dont-ask` — see the signpost under **Behavior**. A field an author must remember to fill is the diligence dependency [§design-principles](../../framework/constitution.md#design-principles) rejects, which is 017's whole thesis.
- [x] AC9: Every existing spec in this repo's `specs/` directory uses frontmatter; no spec file contains both formats.
- [x] AC10: Every slash command source under `framework/commands/` that reads or writes spec metadata parses frontmatter, not bold-prefix lines.
- [x] AC11: `/ductus:analyze` hard-fails on required-field violations (missing or invalid `status`, missing or invalid `dependencies`, malformed YAML, or no frontmatter block) and reports missing `tags`, unknown fields, and other discrepancies as advisory findings. **Superseded in part.** The hard-fail set holds exactly, and is restated in the constitution's Validation Severity subsection with one addition this spec could not have made — the scenario hard fail now fires when _both_ `section` and the legacy `spec-ref` are missing. Unknown fields are still reported, as informational. Missing `tags` is no longer reported at any severity: the constitution states that stale fields including `tags` _produce no findings_. Retired by `017-derive-dont-ask` — see the signpost under **Behavior**. A field an author must remember to fill is the diligence dependency [§design-principles](../../framework/constitution.md#design-principles) rejects, which is 017's whole thesis.
- [x] AC12: `/ductus` (the unified bootstrap from 012) detects pre-frontmatter spec files in adopted projects and migrates them on its next run.
- [x] AC13: `/ductus` migration is idempotent — running it twice on the same project produces no second-run changes.
- [x] AC14: `/ductus` migration prechecks `git status --porcelain` scoped to `specs/` and refuses to run on a dirty tree, instructing the user to commit or stash. No automatic backup directory is created; git is the recovery mechanism.
- [x] AC15: `/ductus` migration respects `.ductus/config.toml` pinning — pinned files are skipped and surfaced in the post-run summary.
- [x] AC16: The root `README.md` includes a "Viewing artifacts" section that documents `npx quartz` as the recommended viewer and notes that other PKM tools work unchanged. The recommendation lives in this repo only — the project-readme template is unchanged. **Superseded in part.** The section exists and the scoping decision holds — it is in this repo's README only and the project-readme template is still unchanged. The _recommendation_ is gone: `f1ada44d` dropped `npx quartz specs/` as **broken**, and the section now lists GitHub, Obsidian/Logseq/Foam, Quartz-or-MkDocs and plain `cat` as equal options under "no viewer required". Naming one recommended viewer is the part that did not survive.
- [x] AC17: `framework/bootstrap/ductus.md` post-run output mentions `npx quartz specs/` as a one-line tip so adopters discover the viewer at bootstrap time without the recommendation being baked into their own README. **Superseded in part.** A one-line viewer tip is still in the post-run output and still serves the discoverability purpose this criterion exists for. It no longer names `npx quartz specs/` — `f1ada44d` dropped that invocation as broken — and reads tool-neutrally instead: plain markdown that works in any PKM tool or as a published site, "or none".
- [x] AC18: All updated and migrated `.md` files pass `npx markdownlint-cli2`.

## Open Questions

_All open questions resolved. See Resolved Questions below._

## Resolved Questions

- **Schema location and format** — markdown table in the constitution, next to the text-first principle declaration. Defer JSON Schema until a concrete tooling consumer asks for it. This aligns with the principle the spec is declaring (markdown canonical, structured forms are derived views regenerated on demand), keeps a single source of truth that cannot drift, and is sufficient because the agent is the primary validator and reads the table natively. The table has columns `Field | Required | Type | Allowed values | Description` and is open — additional fields beyond the required set are permitted and ignored by uninterested consumers, preserving the cheap-to-extend property. JSON Schema can be added later as a derived artifact if a tool that needs it (e.g., editor autocomplete, CI hook) emerges.
- **Required vs. optional fields at launch** — **the `tags` half was reversed by 017; see the signpost under Behavior.** The required set resolved here is unchanged and still correct. As resolved: required fields are `status` and `dependencies` only. `tags` is a standard optional field treated as first-class through three reinforcement points: bundled spec templates include `tags: []` as a visible placeholder, `/ductus:specify` prompts for at least one tag at creation (with autocomplete from existing sibling specs), and `/ductus:clarify` flags missing tags as an advisory finding at the `draft → clarified` transition (not a hard gate). Tag values are free-form strings; the constitution publishes a starter vocabulary as guidance, not enforcement, so adopters converge on a small consistent set without rigid ceremony. This avoids forced placeholders that pollute graph views and keeps migration friction at zero — graph-view value accrues organically as specs are touched. Other speculative fields (`description`, `created_at`) remain optional or omitted: `description` duplicates the prose body, and `created_at` is authoritative in git. The schema is open — additional fields beyond required and standard-optional are permitted and ignored by uninterested consumers.
- **Field naming for dependencies** — flat list of slugs (`dependencies: [002-events, 005-auth]`). No current consumer distinguishes hard from soft dependencies — `/ductus:status` and `/ductus:clarify`'s gate check both treat the relationship as binary. Object form (`[{slug: 002-events, kind: hard}]`) preemptively encodes a distinction nothing reads, and migration from bold-prefix to flat list is mechanical. The forward path is open: YAML accepts mixed flat-and-object lists in the same document, so a future per-dep metadata need (kind, via, since) can be introduced without re-flattening — and starting simple carries lower regret risk than starting structured. Empty list permitted (replaces the bold-prefix `none` convention).
- **Validation strictness** — split by field criticality. `/ductus:analyze` hard-fails on required-field violations: missing or invalid `status`, missing or invalid `dependencies`, malformed YAML in the frontmatter block, or no frontmatter block at all. Everything else stays advisory: missing or empty `tags`, unknown fields (legal under the open-schema rule, surfaced as informational), and existing advisory checks (checkbox mismatches, cross-spec reference issues). Required fields are load-bearing — `/ductus:status` and the pipeline gates cannot function if `status` is unparseable, so advisory treatment would make the validator's report less reliable than runtime behavior. Pipeline gates already do runtime enforcement on `status`; validate's job is batch surfacing for the rest, not duplicate gating. The hard-fail surface is small enough that adopters running `/ductus` to migrate will not see a spec graveyard.
- **Migration scope for non-spec artifacts** — frontmatter-free for now. The schema applies to spec files (`spec.md`, `spec-and-plan.md`) and scenario files (`scenarios/{slug}.md`); other markdown artifacts (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan files, tasks files, rule files, README files) MAY include frontmatter when a specific consumer benefits but are not required to. None of the non-spec artifacts have a status lifecycle, none have dependencies the pipeline reads, and no current consumer needs metadata on them — so a stub would be performative and migration ceremony for nothing. Scenarios are pulled in scope because they have a meaningful required field (`spec-ref`) currently in bold-prefix form, and uniform parsing across pipeline-relevant artifacts keeps the slash-command parser story simple. Forward path is open: any artifact kind can declare its own schema later if a need emerges. Concrete deferred candidate: optional frontmatter on `tasks.md` (and possibly `plan.md`), surfaced during 010 clarify, with potential consumers being a `/ductus:analyze` rule, a graph view, or cross-cutting task queries; deferred until one of those consumers actually drives the requirement.
- **Quartz recommendation scope** — **the scoping decision holds; the recommendation it scoped does not.** Everything below about _where_ a recommendation belongs is still the framework's position, and the project-readme template is still unmodified for exactly these reasons. But `f1ada44d` dropped `npx quartz specs/` as broken, so what this repo's README and the bootstrap tip carry today is a tool-neutral list rather than a recommendation. As resolved: this repo's README only. The project-readme template is not modified. Adopters' READMEs serve their _product's_ users, not their `ductus` maintainers — telling product users to browse specs with `npx quartz` assumes specs are a primary artifact for that audience, which usually they aren't. Quartz is a maintainer concern, and the maintainer is the same person who runs `/ductus` and reads `ductus`'s README, so they'll find the recommendation where it lives. Templates calcify — embedding the recommendation in the project-readme template means every adopter ships a README mentioning Quartz, and if Quartz is later deprecated, every adopter's README is wrong. The principle being declared (text-first, portable artifacts) means any adopter who wants Quartz adopts it in three lines without `ductus` prescribing it. Discoverability is preserved by adding a one-line tip to `framework/bootstrap/ductus.md`'s post-run output ("`npx quartz specs/` for a graph view"), so adopters encounter the viewer at bootstrap time without committing the recommendation to their own repo permanently.
- **Migration safety net** — rely on git, no backup directory. `/ductus`'s migration step prechecks `git status --porcelain` scoped to `specs/` and `framework/templates/spec/`; if dirty, refuse with a message instructing the user to commit or stash. If clean, migrate in place and print a summary; the user reviews via `git diff`, commits, or aborts via `git restore`. A `.governance-migration-backup/` directory would be a parallel undo system that disagrees with git the moment anyone runs `git restore`, would rot once the migration is complete (orphaned files no one trusts), and would contradict `/ductus`'s existing `update`/`create`/`skip` strategy that overwrites files in place. The clean-tree precheck is the actual safety net — it ensures the migration diff is reviewable, atomic, and revertable. The check is scoped (not whole-repo) so unrelated in-flight work elsewhere does not block migration.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [000-slash-commands](../000-slash-commands/spec.md)
- [007-govern-workflow](../007-govern-workflow/spec.md)
- [012-multi-agent-govern](../012-multi-agent-govern/spec.md)
