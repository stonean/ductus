---
status: in-progress
dependencies: []
next-criterion: 28
---

# 017 — Derive, Don't Ask

> **Signpost (post-018):** The adopter pre-commit hook design described in
> this spec was superseded by [018-adopter-owned-pre-commit](../018-adopter-owned-pre-commit/spec.md).
> The shipped file at `framework/bootstrap/hooks/pre-commit` is now an
> adopter-owned outer stub written via `create` strategy, not a ductus-owned
> hook updated via `update` strategy. The ductus-owned generator orchestration
> moved to `framework/bootstrap/hooks/ductus-pre-commit`. `install.sh` is
> deleted; install actions inline in `ductus-procedure.md` §Hook Installation. AC21–AC23
> reflect the pre-018 design.

Apply the **Design Principles** rule added to `AGENTS.md` on 2026-05-06 ("Never design framework features that depend on human diligence or discipline") to every existing framework input that violates it. For each violation, land on a derivable design or an explicit deletion. No item is deferred to inbox — the principle's own escape hatch is exercised in this spec.

The principle, restated: any framework input that requires an author to *remember* to fill it in, set a flag, update a doc alongside code, or otherwise be careful will fail in practice — silently, and exactly in the cases where it mattered most. The remedy is to derive the input from existing artifacts, frontmatter, git history, or code analysis; or, if no derivation is viable, to delete the input.

This spec is a discipline-cleanup pass. It does not introduce new framework capabilities — every change either removes an input or replaces a manual input with a generator/hook/derivation.

## Violation Inventory

The 17 violations identified during the audit, classified by disposition. Acceptance criteria below trace back to this inventory by item number.

### Delete entirely (no viable derivation, not load-bearing)

| # | Violation | Where it lives |
| --- | --- | --- |
| 1 | `title:` PKM frontmatter | `framework/templates/spec/{spec,plan,tasks,data-model,research,scenario,spec-and-plan}.md`; written by `/specify`, `/plan`, `/amend`; checked + auto-fixed by `/analyze` |
| 2 | `track: lightweight` field | `framework/templates/spec/spec-and-plan.md` only, a template 023 has since deleted |
| 3 | `tags` frontmatter | Constitution §text-first-artifacts schema; prompted by `/specify`; checked advisory by `/clarify` and `/analyze`; "Starter Tag Vocabulary" table in constitution |
| 4 | `[simple]` task tier marker | Constitution §cost-levers; `framework/templates/spec/tasks.md` template; proposed by `/plan`; surfaced by `/implement` |
| 5 | `[promote-to-rule]` inbox prefix | `framework/commands/groom.md` |
| 6 | Plan template "Open Questions Resolved" section | `framework/templates/spec/plan.md` |
| 7 | AGENTS.md "remember to mirror" instruction (constitutions) | `AGENTS.md` Workflow section |
| 8 | AGENTS.md "remember to run gen-claude-commands.sh" instruction | `AGENTS.md` Workflow section |

### Derive (replace the input with a scaffold/generator/hook)

| # | Violation | Derivation path |
| --- | --- | --- |
| 9 | `spec-ref` parent half in scenario frontmatter | Parent feature is the directory the scenario lives in; scenario frontmatter shrinks to the section name only (`section:` field), and `/amend` (scenario branch) writes it. |
| 10 | README feature table | Generator script reads `specs/*/spec*.md` frontmatter (`status`, `dependencies`, body's first paragraph) and rewrites the table between marker comments. Wired into a pre-commit hook. |
| 11 | `.claude/commands/ductus/*.md` generator drift | Pre-commit hook auto-runs `./scripts/gen-claude-commands.sh` whenever `framework/commands/**` or `framework/bootstrap/configure/claude.md` changes, and stages the result. |
| 12 | `help.md` ↔ command `description:` equivalence | `framework/commands/help.md` "Pipeline / Elaborate / Brownfield / Orient / Bootstrap" command tables generated from each command's frontmatter `description:` between marker comments. Same pre-commit hook. |
| 13 | `dependencies` frontmatter | `/clarify` and `/plan` scan the spec body for relative markdown links to sibling spec files (`../NNN-feature/...` or `[NNN-feature](...)`) and propose the union as `dependencies`. The author confirms or removes; the field is no longer hand-authored from scratch. |

### Replace with structural enforcement (Bucket 3 — real designs)

| # | Violation | Design |
| --- | --- | --- |
| 14 | Plan "Affected Files" used as the implement write boundary | `/implement` derives the boundary at runtime from `git diff --name-only` between the spec dir's first commit and `HEAD`, plus uncommitted changes. The plan's Affected Files section becomes a *planning aid* (proposal during `/plan`), not the authoritative boundary. The implement command stops asking the author to backfill files into the section after-the-fact. |
| 15 | Acceptance-criteria + task checkbox flipping | `/validate --fix` for checkboxes is removed. The framework's position becomes: checkboxes are flipped only by `/implement` (per its existing per-criterion verification step) and `/clarify` (for resolved questions). Hand-implementation flows that bypass `/implement` are out-of-framework — the discipline patch is not shipped. |
| 16 | Twin constitutions (root ↔ framework) mirror | Per Q2: collapse to a single canonical file at `framework/constitution.md`. Delete root `constitution.md` entirely. Update root `CLAUDE.md` to `@import framework/constitution.md` and any root README link to point at `framework/constitution.md`. No generator, no section markers, no install-time stripper. The AGENTS.md mirror instruction is deleted. |
| 17 | §cross-spec-impact rule | `/clarify`, `/plan`, and `/implement` each gain a "cross-spec scan" step: enumerate sibling spec slugs cited (by inline link or bare slug) in the target's body that are not in `dependencies`, and surface them. At `/implement` completion, additionally diff `git diff --stat specs/` over the implementation window and prompt the user when files outside the target spec dir were modified, asking whether those changes need to be recorded as new acceptance criteria/scenarios in the affected spec. The constitution rule stays as the human-readable principle; enforcement moves into the commands. |

### Promote to rule file (the framework's own enforceable-rule mechanism)

| # | Violation | Design |
| --- | --- | --- |
| 18 | §constants + §env-vars rules | Move both constitution sections into a new rule file at `framework/rules/configuration-cross.md` with rules carrying RFC 2119 statements + Verification steps that `/analyze` runs against feature artifacts. The constitution sections shrink to a one-line pointer ("See `framework/rules/configuration-cross.md`"). Adoption: shipped to adopters via `/ductus` like other rule files. |

(Original audit numbered violations 1–17 across templates, commands, constitution, and AGENTS.md. The inventory above splits violation #18 — promoting constants + env-vars to a rule file — out as its own item because it requires creating a new rule file rather than editing existing artifacts.)

> **Signpost ([008-security-rules](../008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md)):**
> the configuration rule format's Statement field and severity invariant in this spec's
> `data-model.md` are stated in tier terms: a Statement is a block quote whose obligation
> keywords come from one tier, and MAY may appear beside either as a permission that carries
> no tier. They previously read "one sentence" and "exactly one of the four keywords", which
> most shipped Statements contradicted. 008 records the obligation in its
> `cross-spec-impact:` frontmatter; this signpost is its discharge.

## Schema Changes

Frontmatter schema (constitution §text-first-artifacts) after this spec:

| File | Required | Removed |
| --- | --- | --- |
| `spec.md` / `spec-and-plan.md` | `status`, `dependencies` | `tags` (deleted), `title` (deleted), `track` (deleted from `spec-and-plan.md`) |
| `scenarios/{slug}.md` | `section` | `spec-ref` (replaced by `section`), `tags` (deleted), `title` (deleted) |
| `plan.md`, `tasks.md`, `data-model.md`, `research.md` | (none — frontmatter is optional) | `title` (deleted) |

`/{project}:analyze` severity changes:

- Hard fail: unchanged for `status`, `dependencies`, frontmatter parse.
- Hard fail (new): `section` on scenario files (replaces `spec-ref`).
- Advisory: PKM `title:` check **removed**; `tags` advisory **removed**.
- Fixable (`--fix`): PKM title fix **removed**; checkbox fixes **removed** (per item #15).
- New blocking checks: project-level `help.md` equivalence becomes structural (the table is generated, so the check verifies the file is up-to-date with the generator output, not that descriptions match by hand).

## Migration

Existing `ductus` repo dogfood specs (000–016) and any adopter projects already on the current schema will have stale `title:`, `tags:`, `spec-ref:`, `track:` fields and `[simple]` task markers. Per constitution §spec-lifecycle, done specs are not rewritten retroactively: a meaningful body edit reopens a spec through the back-edge, so a bulk retroactive migration would reopen every one of them.

- Existing `done` specs: no migration. Stale fields remain; the open-schema rule (constitution §text-first-artifacts) ignores unknown fields. `/{project}:analyze` stops checking them, so they cause no findings.
- New specs created after this lands: no longer have the deleted fields.
- Adopter projects: receive the new templates on next `/ductus`. Existing specs in adopter projects are left alone for the same reason — a bulk rewrite would reopen each one through the back-edge.
- Active in-flight specs in this repo (none at the moment beyond this spec): the author can strip the deleted fields opportunistically; it is not enforced.

This spec's own frontmatter contains `title:` and `tags:` because they are still required by the current templates at spec-creation time. They will be stripped from this spec's frontmatter as part of the implementation tasks (the spec edits its own frontmatter as a final task).

## Generators and Hooks

Per Q2 (twin constitutions collapsed) and Q7 (spec-deps derivation), three new generators land alongside the existing `gen-claude-commands.sh`:

1. `scripts/gen-readme-table.sh` — rebuilt the Feature Specs table in the root `README.md` between marker comments from `specs/*/spec*.md` frontmatter. **Since retired** along with the generated table (see AC10).
2. `scripts/gen-help-tables.sh` — rebuilds the command tables in `framework/commands/help.md` from each command file's frontmatter `description:`. Still shipped and still run by the hook.
3. `.ductus/scripts/gen-spec-deps.sh` — scanned every `specs/*/spec*.md` body for inline markdown links to sibling specs (excluding code fences, blockquote-prefixed lines, and `## See also` opt-out sections) and rewrote the frontmatter `dependencies` list to match. `## References` is *not* an opt-out (it is the canonical body-authored dep section per task 29's migration); only `## See also` suppresses edges. After the rewrite it ran an SCC-based cycle check across the full derived graph, reporting any cycle on stderr and exiting non-zero to block the commit. **Since promoted to a runtime primitive** — `derive-dependencies`, joined by `derive-references` — by `067-analyze-and-audit-checks`'s `adopter-generator-promotion`, so `.ductus/scripts/` no longer exists (see AC23). The derivation rules and the cycle check carried across unchanged.

(The fourth proposed generator, `gen-root-constitution.sh`, is *not* needed — Q2 collapsed the twin constitutions to a single canonical file at `framework/constitution.md`, removing the divergence the generator would have managed.)

### ductus repo's pre-commit hook

`.githooks/pre-commit` orchestrates the generators. Installed via `git config core.hooksPath .githooks`, run by `scripts/install-hooks.sh`. Runs them unconditionally on every commit and stages any changes — trades a fraction of a second per commit for a one-line implementation that can't get the gate logic wrong. As delivered here it ran four shell generators; it now runs three (`gen-configure-mcp`, `gen-claude-commands`, `gen-help-tables`) plus the two frontmatter derivations as runtime primitives, and halts when the runtime is unreachable rather than skipping them.

### Adopter projects' pre-commit hook

`framework/bootstrap/hooks/pre-commit` ships with the framework as an adopter-owned outer stub that calls the ductus-owned inner hook — see the post-018 signpost at the top of this spec — which runs the `derive-dependencies` and `derive-references` primitives from the acquired runtime. They replaced the `.ductus/scripts/gen-spec-deps.sh` generator this spec shipped (AC23), so nothing for them is scaffolded, refreshed, or pinned.

`/ductus` manages the adopter hook (see Q7 resolution for the install/update/skip behavior). Pinning is `.ductus/config.toml` `[pinned] files`.

### CI safety net

Both repos run the generators in CI and fail the build on a non-empty diff — this repository in `.github/workflows/generators.yml`, adopters through the shipped template `framework/templates/ci/adopter-generators.yml`. Catches contributors who never installed the hook locally and adopters whose hook was skipped due to existing-hook detection. As delivered here the CI step ran them in dry-run mode; it now runs them for real and compares the tree afterwards, which additionally catches a generator that creates a *new* output rather than modifying a tracked one.

## Acceptance Criteria

- [x] AC1: All six template files (spec, spec-and-plan, plan, tasks, data-model, research, scenario) have no `title:` field in frontmatter
- [x] AC2: the spec-and-plan template had no `track:` field and no comment about it. Met at v1 and **since retired** — spec `023-govern-refinement` deleted `framework/templates/spec/spec-and-plan.md`. Retained as a record of what shipped, not as a live requirement.
- [x] AC3: Scenario template uses `section:` (not `spec-ref:`) and parent feature is no longer encoded in the field
- [x] AC4: `tags` is no longer a frontmatter field an author supplies: no template carries it, no command writes, prompts for, or checks it, and the constitution names it only as a stale field the open-schema rule ignores; the "Starter Tag Vocabulary" table is removed from the constitution. Corrected from "no template, command, or constitution section references `tags`", which never held. The runtime still parses the stale field from the specs that carry it: `read-spec` returns it, and `/{project}:status` renders a read-only tags-in-use line from it (`framework/commands/status.md`). The status command did so when this spec first closed, and `022-deterministic-runtime`'s `dashboard-primitive` scenario carries that read into the `dashboard` primitive deliberately
- [x] AC5: No template, command, or constitution section references the `[simple]` task marker; the §cost-levers reference is updated to remove the bullet
- [x] AC6: `framework/commands/groom.md` no longer instructs the agent to prefix items with `[promote-to-rule]`; groom always re-walks every item it didn't migrate on the next pass
- [x] AC7: `framework/templates/spec/plan.md` does not contain an "Open Questions Resolved" section
- [x] AC8: `AGENTS.md` does not contain the "mirror constitutions" or "run gen-claude-commands.sh" instructions
- [x] AC9: `/specify`, `/plan`, and `/amend` each write the canonical filename-derived metadata (formerly `title:`) at scaffold time without prompting the author
- [x] AC10: The README's "Feature Specs" table is bounded by marker comments and produced by a generator; running it on a fresh checkout produces no diff. Met at v1 with `scripts/gen-readme-table.sh`, **since retired** along with the generated table.
- [x] AC11: `framework/commands/help.md` command tables are bounded by marker comments and produced by `scripts/gen-help-tables.sh`; running the script on a fresh checkout produces no diff
- [x] AC12: A pre-commit hook at `.githooks/pre-commit` runs all four generators (claude-commands, readme-table, help-tables, spec-deps) and stages outputs; `scripts/install-hooks.sh` configures `core.hooksPath`. (No `root-constitution` generator — Q2 collapsed the twin constitutions to a single canonical file, removing the need.) Delivered as written; **two of the four no longer exist** — `gen-readme-table.sh` was retired with the generated table (AC10) and `gen-spec-deps.sh` was promoted to the `derive-dependencies` / `derive-references` primitives (AC23). The hook today runs `gen-configure-mcp.sh`, `gen-claude-commands.sh` and `gen-help-tables.sh` plus those two primitives, and halts when the runtime is unreachable rather than skipping them — see `.githooks/pre-commit`.
- [x] AC13: `/clarify` and `/plan` scan the target spec body for sibling-spec links and propose missing `dependencies` entries; the author confirms or removes
- [x] AC14: `/implement` derives the write-boundary check from `git diff` against the spec dir's first commit, not from a manually maintained Affected Files list; the plan's Affected Files section is documented as a planning aid only
- [x] AC15: `/analyze` no longer offers `--fix` for checkbox state; the Fix Mode section is removed and the checkbox-related entries in the Fixable list are gone
- [x] AC16: Root `constitution.md` is deleted; `framework/constitution.md` is the sole canonical file; root `CLAUDE.md` imports it via `@import framework/constitution.md`; no root-constitution generator exists
- [x] AC17: `/clarify`, `/plan`, and `/implement` each include a cross-spec scan step that surfaces sibling slugs cited in the body but not in `dependencies`; `/implement` completion additionally surfaces files outside the target spec dir touched during the implementation window
- [x] AC18: A new rule file `framework/rules/configuration-cross.md` exists with rules covering shared constants, module-local constants, configurable values backed by env vars, env-var defaults, env-var validation, and unit-suffixed time variables; each rule has a Verification step `/analyze` runs; constitution §constants and §env-vars shrink to one-line pointers
- [x] AC19: `/analyze` runs cleanly on every existing spec (000–016) plus this spec after migration — no new findings introduced by schema changes; stale fields in done specs are silently ignored per the open-schema rule
- [x] AC20: This spec's own frontmatter has `title:` and `tags:` removed by the final task
- [x] AC21: `framework/bootstrap/hooks/pre-commit` ships with the framework, as did `framework/bootstrap/hooks/install.sh`, which no longer exists — spec `018-adopter-owned-pre-commit` inlined its install actions into `framework/bootstrap/ductus-procedure.md` §Hook Installation; the shipped hook calls only adopter-relevant generators (initially `gen-spec-deps.sh`)
- [x] AC22: `/ductus` installs the adopter hook on first run when no existing hook system is detected; updates on subsequent runs; warns and skips with a manual integration snippet when an existing hook system is detected (`.githooks/pre-commit` not from `/ductus`, husky, lefthook, pre-commit-py, or `core.hooksPath` pointing elsewhere); respects `.ductus/config.toml` pinning
- [x] AC23: `.ductus/scripts/gen-spec-deps.sh` ships to adopter projects with `update` strategy on every `/ductus` run (pinnable via `.ductus/config.toml`); the shipped pre-commit hook references it via the project-relative path — superseded by 022-deterministic-runtime: the shell generators it names were promoted to runtime primitives, so `.ductus/scripts/` no longer exists — the derivation this criterion delivered now runs as `derive-dependencies` and `derive-references`
- [x] AC24: A CI workflow runs all generators in dry-run mode and fails the build on non-empty diff, in both this repo and (as a shipped example) adopter projects; protects against contributors or adopters whose hook was skipped or never installed. Delivered as written; the CI step (`.github/workflows/generators.yml`, and `framework/templates/ci/adopter-generators.yml` for adopters) now runs the generators **for real** and compares the tree afterwards, which additionally catches a generator that creates a new output rather than modifying a tracked one — so the dry-run half is dated while the guarantee it names is unchanged.
- [x] AC25: Capturing a review finding that maps to no loaded rule does not depend on the reviewer remembering a separate step: recording it in the report is what writes it to the inbox, so the two cannot diverge and an uncaptured observation is not a reachable state. Delivered as written; **the inbox half is superseded** — `058-findings-route-at-discovery` stopped writing observations to the inbox, the one destination no gate reads. Recording an observation in the report now puts it in front of the review's fix-and-route step, and the record counts it undispositioned, holding the spec out of `done`, until it is fixed, routed, or discarded. The guarantee this criterion names — no separate step to remember, no uncaptured observation — holds unchanged
- [x] AC26: The routing rules that decide whether work becomes a new spec, a scenario on an existing spec, or a rule-file amendment bind wherever work enters — not only when it arrives through the inbox, where `/{project}:groom`'s decision tree happens to run
- [x] AC27: `framework/rules/configuration-cross.md` declares a third category, `FLAG` (feature flags), in its header, and `data-model.md` registers it in the ID-prefix and Category-name bullets and the category-abbreviation table; `CFG-FLAG-001` (MUST) and `CFG-FLAG-002` (SHOULD) are present, as are `CFG-ENV-008` (MUST), `CFG-ENV-009` (SHOULD), `CFG-ENV-010` (MUST), `CFG-ENV-011` (MUST) and `CFG-ENV-012` (SHOULD), each with a Statement, a Rationale, a Verification and a Source; and `data-model.md` § Rule set enumerates every rule in the file with its tier

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Q1 (item #15, checkbox flipping):** Remove `--fix` entirely. Checkboxes are flipped only by `/implement` (per its existing per-criterion verification step at `framework/commands/implement.md:99`) and by `/clarify` (which moves resolved questions section-to-section). Hand-implementation that bypasses `/implement` is treated as out-of-framework — the framework does not ship a fix-up tool for it. Combined with the title-field removal in AC1, this leaves `--fix` mode with nothing to do, so the entire flag goes away from `/analyze`. Friction with parent-checkbox-not-flipped surfaces only outside the framework path; `/implement` already marks parents and sub-items together (`implement.md:91`).
- **Q2 (item #16, twin constitutions):** Collapse to a single canonical file at `framework/constitution.md`. Delete root `constitution.md`. Update root `CLAUDE.md` to `@import framework/constitution.md` and any root README link to point at `framework/constitution.md`. No generator, no section markers, no install-time stripper. Verified the two files were byte-identical at decision time — the documented "may diverge" pattern was theoretical and the discipline trap was guarding nothing. AGENTS.md already exists as the ductus-internal home for project-specific rules; anything that ever needs to be ductus-only belongs there, not in the constitution. If ductus-internal content ever needs to live in the constitution, a section-marker mechanism is introduced then, not preemptively. Adopter installs are unchanged: `/ductus` still remaps `framework/constitution.md` → `constitution.md` at the adopter root. The AGENTS.md "mirror constitutions" instruction is removed by AC8.
- **Q3 (item #14, Affected Files diff window):** Diff against the spec dir's first commit (`git log specs/{feature}/ | tail -1`), filtered to files outside `specs/{feature}/`. Cumulative across multi-session implements — each new out-of-plan file surfaces once on first appearance, then is added to the plan and not re-surfaced. Equivalent to "since planned→in-progress transition" for clean pipelines (pre-implement work touches only the spec dir, which the filter excludes), but simpler to compute. No new state in `session.json`.
- **Q4 (item #17, cross-spec scan precision):** Inline markdown links only — match `[label](../NNN-feature/...)` or paths to `specs/NNN-feature/`. Bare slug mentions in prose are treated as commentary, not cross-references, on two grounds: (1) prose references to other specs are common and would flood the proposal with false positives, and (2) "always write proper links" is a markdown convention authors already follow, not a ductus-specific discipline. Rule-ID citations (e.g., `BE-AUTHN-001`) are out of scope here — those go through the existing rule-reference scan in `/analyze` (`analyze.md:163-166`).
- **Q5 (item #18, configuration rule file naming):** Single file at `framework/rules/configuration-cross.md`. Both rule sets cover the same domain (operator-tunable values) and apply equally to backend and frontend code, so the security-files split-by-surface precedent does not apply. Splitting by topic would create artificial boundaries — env-var defaults *are* constants and `.env.example` is the manifest of env vars; they belong together.
- **Q6 (item #18, configuration rule ID prefix):** `CFG-` with two categories: `CONST` (shared / module-local constants) and `ENV` (env-var rules). Format: `CFG-{CONST|ENV}-{NNN}`. Examples: `CFG-CONST-001` (shared constants in centralized location), `CFG-ENV-001` (every env var has a default constant), `CFG-ENV-002` (`.env.example` contains every introduced var), `CFG-ENV-003` (time variables include unit suffix in name and constant). Rule files declare their own ID format per `analyze.md:145`; this spec sets the configuration file's pattern.

  **Updated 2026-10-19 — a third category.** The rule-file review of 2026-10-19 added `FLAG` (feature flags: `CFG-FLAG-001` and `CFG-FLAG-002`) through this spec's back-edge, so the current decision is `CFG-` with three categories — `CONST`, `ENV` and `FLAG` — in the format `CFG-{CONST|ENV|FLAG}-{NNN}`. Flags joined this file rather than a new one on Q5's reasoning: they are configuration, the same problem on both surfaces. The examples above describe the first version, and two no longer hold: the time-unit rule is `CFG-ENV-004` (`CFG-ENV-003` has been required-value startup validation since the file first shipped, for environment variables until 2026-10-20 and for every source since), and `CFG-ENV-001` no longer gives every variable a default: a value may be required with no default, or optional with no default when an absent value disables a feature the declaration names — every configuration value, environment variable, file key or flag, since 2026-10-20. `data-model.md` § Rule set enumerates the current rules. The `ENV` category's name widened from *Environment variables* to *Environment and configuration values* (owner decision, 2026-10-19), because `CFG-ENV-007`, `CFG-ENV-008` and `CFG-ENV-012` govern configuration files and flags as well as environment variables; the abbreviation and every `CFG-ENV-*` ID are unchanged. The `analyze.md:145` pointer is now `analyze.md` §Rules (*Which ID format governs a file*), which takes a file's ID format from its introducing spec's data model before the file's own header.
- **Q7 (item #13, dependencies derivation + sync mechanism):** Body is authoritative; frontmatter `dependencies` is fully derived. The derivation runs in two places: (a) the pre-commit hook runs the `derive-dependencies` runtime primitive, which scans every spec body for inline markdown links to sibling specs (excluding code fences), recomputes the union, and rewrites the frontmatter list; (b) `/clarify`, `/plan`, `/implement`, `/amend` and `/target` run it report-only on entry, so drift from uncommitted body edits is surfaced rather than silently carried. The author maintains links in the body — there is no place to author the frontmatter list. To remove a dep, remove the inline link (or move it inside a code fence). To mention a spec without depending on it, use a bare slug per Q4.

  **Hooks are symmetric across ductus and adopters; content is asymmetric.** ductus's hook runs `gen-configure-mcp`, `gen-claude-commands` and `gen-help-tables` plus the `derive-dependencies` and `derive-references` primitives. Adopter projects' hook runs only the two derivations.

  **`/ductus` manages the adopter hook on every install/update run.** The detection ladder, the write strategies and the skip-with-snippet path for an existing hook system live in `framework/bootstrap/ductus-procedure.md` §Hook Installation, which spec `018-adopter-owned-pre-commit` reshaped into an adopter-owned outer stub and a ductus-owned inner hook. The derivations arrive with the acquired runtime, so nothing for them is scaffolded, refreshed, or pinned.

  **CI safety net for both surfaces.** The same generators and derivations run in CI and the tree is compared afterwards; a diff fails the build. Catches contributors who never installed the hook locally and adopters whose hook was skipped due to existing-hook detection.

  **Migration of existing specs.** The 16 specs that existed when this landed had hand-authored frontmatter deps that might not match their body inline links, and the first derivation could *remove* deps that weren't body-linked. A one-time migration step added inline links to spec bodies for any frontmatter dep not already linked, so the first run removed nothing by surprise.

  **Recategorization.** Item #13 moves from "Derive (proposal + author confirms)" to the same conceptual bucket as items #10–#12 — generated artifacts maintained by the pre-commit hook.
- **Q8 (constitutional promotion of the discipline principle):** Leave the principle in `AGENTS.md` for now. It is one day old (added 2026-05-06) and has been applied to two decisions (the upgrade-impact deferral and this spec). The framework's maturation path is: principles prove themselves in `AGENTS.md`, then promote to the constitution if they generalize — the §rules tier in spec 016 followed exactly this path after specs 008 and 015 established the pattern. Constitutional changes ship via `/ductus` to every adopter and are stickier to roll back; preemptive promotion skips the proving step. Re-evaluate in a follow-up spec once a third or fourth application accumulates.

  **Resolved by 050 — the deferral has been discharged.** The re-evaluation this resolution defers has happened: `050-constitution` promoted the principle to the constitution's §design-principles as *Never design a pipeline feature that depends on human diligence*, classified universal in that spec's plan, with `AGENTS.md` keeping a pointer rather than the rule. The maturation path described here is exactly what ran — the principle proved itself in this spec and the ones after it, then moved — so this is annotated rather than reversed. Recorded because a resolution that *rejects* an option goes stale silently when the option is later adopted: nothing greps for it, the option's name never changed, and the text still reads as a live decision to leave the principle where it no longer solely lives.
