---
status: done
dependencies: []
next-criterion: 29
retired-rules: [BE-DEPS-006]
---

# 017 — Derive, Don't Ask

> **Signpost (post-018):** The adopter pre-commit hook design is
> [018-adopter-owned-pre-commit](../018-adopter-owned-pre-commit/spec.md)'s.
> The shipped file at `framework/bootstrap/hooks/pre-commit` is an
> adopter-owned outer stub written via `create` strategy. The ductus-owned
> generator orchestration is `framework/bootstrap/hooks/ductus-pre-commit`,
> written via `update` strategy. No install script ships; the install actions
> are in `framework/bootstrap/ductus-procedure.md` §Hook Installation.

Apply the **Design Principles** rule added to `AGENTS.md` on 2026-05-06 ("Never design framework features that depend on human diligence or discipline") to every existing framework input that violates it. For each violation, land on a derivable design or an explicit deletion. No item is deferred to inbox — the principle's own escape hatch is exercised in this spec.

The principle, restated: any framework input that requires an author to *remember* to fill it in, set a flag, update a doc alongside code, or otherwise be careful will fail in practice — silently, and exactly in the cases where it mattered most. The remedy is to derive the input from existing artifacts, frontmatter, git history, or code analysis; or, if no derivation is viable, to delete the input.

This spec is a discipline-cleanup pass. It does not introduce new framework capabilities — every change either removes an input or replaces a manual input with a generator/hook/derivation.

## Violation Inventory

The 17 violations identified during the audit, classified by disposition. Acceptance criteria below trace back to this inventory by item number.

### Delete entirely (no viable derivation, not load-bearing)

| # | Violation | Where it lives |
| --- | --- | --- |
| 1 | `title:` PKM frontmatter | `framework/templates/spec/{spec,plan,tasks,data-model,research,scenario}.md`; written by `/specify`, `/plan`, `/amend`; checked + auto-fixed by `/analyze` |
| 2 | `track: lightweight` field | the single-file spec-and-plan template only; no template carries it |
| 3 | `tags` frontmatter | Constitution §text-first-artifacts schema; prompted by `/specify`; checked advisory by `/clarify` and `/analyze`; "Starter Tag Vocabulary" table in constitution |
| 4 | `[simple]` task tier marker | Constitution §cost-levers; `framework/templates/spec/tasks.md` template; proposed by `/plan`; surfaced by `/implement` |
| 5 | `[promote-to-rule]` inbox prefix | `framework/commands/groom.md` |
| 6 | Plan template "Open Questions Resolved" section | `framework/templates/spec/plan.md`; it is an optional design-record section now (AC7) |
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
> no tier. 008 records the obligation in its
> `cross-spec-impact:` frontmatter; this signpost is its discharge.
>
> **Signpost ([077 — Security cross rules](../077-security-cross-rules/spec.md)):**
> `configuration-cross.md` `CFG-ENV-006` names a sensitive value that is not
> configuration, in a log line or in propagated trace context, as `security-backend.md`
> `BE-LOG-002`'s or `BE-LOG-008`'s where that file is also loaded and `security-cross.md`
> `SEC-LOG-001`'s otherwise; a sensitive configuration value in trace context is
> `BE-LOG-008`'s where `security-backend.md` is also loaded and `CFG-ENV-006`'s otherwise, by 008's task 43. This spec's `plan.md` and scenario `the-adopter-ci-template-can-run` cite
> `SEC-DEPS-001`, the rule `security-cross.md` holds for build-input pinning. 077 records the obligation
> in its `cross-spec-impact:` frontmatter; this signpost is its discharge.

## Schema Changes

Frontmatter schema (constitution §text-first-artifacts) after this spec:

| File | Required | Removed |
| --- | --- | --- |
| `spec.md` | `status`, `dependencies` | `tags` (deleted), `title` (deleted) |
| `scenarios/{slug}.md` | `section` | `spec-ref` (replaced by `section`), `tags` (deleted), `title` (deleted) |
| `plan.md`, `tasks.md`, `data-model.md`, `research.md` | (none — frontmatter is optional) | `title` (deleted) |

`/{project}:analyze` severity changes:

- Hard fail: unchanged for `status`, `dependencies`, frontmatter parse.
- Hard fail (new): `section` on scenario files (replaces `spec-ref`).
- Advisory: PKM `title:` check **removed**; `tags` advisory **removed**.
- Fixable (`--fix`): PKM title fix **removed**; checkbox fixes **removed** (per item #15).
- New blocking checks: project-level `help.md` equivalence becomes structural (the table is generated, so the check verifies the file is up-to-date with the generator output, not that descriptions match by hand).

## Migration

Existing `ductus` repo dogfood specs (000–016) and any adopter projects already on the current schema will have stale `title:`, `tags:`, `spec-ref:`, `track:` fields and `[simple]` task markers. Done specs were not rewritten retroactively: when this landed, a meaningful body edit reopened a spec through the back-edge, so a bulk retroactive migration would have reopened every one of them.

- Existing `done` specs: no migration. Stale fields remain; the open-schema rule (constitution §text-first-artifacts) ignores unknown fields. `/{project}:analyze` stops checking them, so they cause no findings.
- New specs created after this lands: no longer have the deleted fields.
- Adopter projects: receive the new templates on next `/ductus`. Existing specs in adopter projects were left alone for the same reason.
- In-flight specs that still carry the deleted fields: an author strips them opportunistically; it is not enforced.

This spec's frontmatter carried `title:` and `tags:` at creation, because the templates then required them, and its final implementation task stripped them.

## Generators and Hooks

Per Q2 (twin constitutions collapsed) and Q7 (spec-deps derivation), two derivations land alongside the existing `gen-claude-commands.sh`:

1. `scripts/gen-help-tables.sh` — rebuilds the command tables in `framework/commands/help.md` from each command file's frontmatter `description:`, run by the hook.
2. The `derive-dependencies` runtime primitive, joined by `derive-references` — scans every spec body for inline markdown links to sibling specs (excluding code fences, blockquote-prefixed lines, and `## See also` opt-out sections) and rewrites the frontmatter `dependencies` list to match. `## References` is *not* an opt-out (it is the canonical body-authored dep section); only `## See also` suppresses edges. It then runs an SCC-based cycle check across the full derived graph, and a cycle makes the command exit non-zero and block the commit (AC23).

(The fourth proposed generator, `gen-root-constitution.sh`, is *not* needed — Q2 collapsed the twin constitutions to a single canonical file at `framework/constitution.md`, removing the divergence the generator would have managed.)

### ductus repo's pre-commit hook

`.githooks/pre-commit` orchestrates the generators. Installed via `git config core.hooksPath .githooks`, run by `scripts/install-hooks.sh`. Runs them unconditionally on every commit, over a scratch copy of the commit's index rather than the working tree, and stages only what that commit may carry: a file the commit already stages, or a generated output when the commit stages a generator source. Another session's unstaged sources, outputs and hunks are neither staged nor overwritten — trades a fraction of a second per commit (0.77 s for the three generators in their check modes and both derivations report-only, `/usr/bin/time -p` at `e9cd5434`, 2026-11-19) for generation that has no gate logic to get wrong. It runs three generators (`gen-configure-mcp`, `gen-claude-commands`, `gen-help-tables`) plus the two frontmatter derivations as runtime primitives, and halts when the runtime is unreachable rather than skipping them. It normalizes the staged markdown there with `markdownlint-cli2 --fix`, and a violation `--fix` cannot repair stops the commit, printing the violations, when this repository's own install ran and exited 1; the `npx` fallback's failure is swallowed, since its exit 1 can be a failed fetch.

### Adopter projects' pre-commit hook

`framework/bootstrap/hooks/pre-commit` ships with the framework as an adopter-owned outer stub that calls the ductus-owned inner hook — see the post-018 signpost at the top of this spec — which runs the `derive-dependencies` and `derive-references` primitives from the acquired runtime, then `label-criteria` and the read-only `check-corpus-links` and `check-artifact-size --project-files --staged` checks. The two derivations replaced the `.ductus/scripts/gen-spec-deps.sh` generator this spec shipped (AC23), so nothing for them is scaffolded, refreshed, or pinned.

`/ductus` manages the adopter hook (see Q7 resolution for the install/update/skip behavior). Pinning is `.ductus/config.toml` `[pinned] files`.

### CI safety net

Both repos run the generators in CI and fail the build on a non-empty diff — this repository in `.github/workflows/generators.yml`, adopters through the template `framework/templates/ci/adopter-generators.yml`, which they copy into their own CI. An adopter's checkout carries no runtime, so the template downloads the release binary at a version and sha256 it names, which needs no Rust or cargo ([the-adopter-ci-template-can-run](scenarios/the-adopter-ci-template-can-run.md)). Catches contributors who never installed the hook locally and adopters whose hook was skipped due to existing-hook detection. The CI step runs them for real and compares the tree afterwards, which catches a generator that creates a *new* output rather than modifying a tracked one.

## Acceptance Criteria

- [x] AC1: All six template files under `framework/templates/spec/` (spec, plan, tasks, data-model, research, scenario) have no `title:` field in frontmatter
- [x] AC2: No template under `framework/templates/spec/` carries a `track:` field or a comment about one
- [x] AC3: Scenario template uses `section:` (not `spec-ref:`) and parent feature is no longer encoded in the field
- [x] AC4: `tags` is no longer a frontmatter field an author supplies: no template carries it, no command writes, prompts for, or checks it, and the constitution names it only as a stale field the open-schema rule ignores; the "Starter Tag Vocabulary" table is removed from the constitution. The runtime still parses the stale field from the specs that carry it: `read-spec` returns it, `/{project}:status` renders a read-only tags-in-use line from it (the pipeline view `render-status` renders, `runtime/src/primitives/pipeline_view.rs`), and the `dashboard` primitive carries that read deliberately (`022-deterministic-runtime`'s `dashboard-primitive` scenario)
- [x] AC5: No template, command, or constitution section references the `[simple]` task marker; the §cost-levers reference is updated to remove the bullet
- [x] AC6: `framework/commands/groom.md` no longer instructs the agent to prefix items with `[promote-to-rule]`; groom always re-walks every item it didn't migrate on the next pass
- [x] AC7: `framework/templates/spec/plan.md` carries `## Open Questions Resolved` only as an optional section of the plan's design record, for a resolved question that needs a plan-level answer; a plan records the design as it stands
- [x] AC8: `AGENTS.md` carries no "mirror constitutions" instruction and no commit-time instruction to run `gen-claude-commands.sh`, because the pre-commit hook regenerates the installed commands; it carries one instruction to run the generator before invoking a command edited in the same session, since the host loads the installed copy at invocation and the hook regenerates it only at commit
- [x] AC9: `/specify`, `/plan`, and `/amend` each write the canonical filename-derived metadata (formerly `title:`) at scaffold time without prompting the author
- [x] AC11: `framework/commands/help.md` command tables are bounded by marker comments and produced by `scripts/gen-help-tables.sh`; running the script on a fresh checkout produces no diff
- [x] AC12: A pre-commit hook at `.githooks/pre-commit` runs `gen-configure-mcp.sh`, `gen-claude-commands.sh` and `gen-help-tables.sh` plus the `derive-dependencies` and `derive-references` primitives (AC23) and stages outputs, and halts when the runtime is unreachable rather than skipping them; `scripts/install-hooks.sh` configures `core.hooksPath`. No `root-constitution` generator exists — Q2 collapsed the twin constitutions to a single canonical file, removing the need
- [x] AC13: `dependencies:` is fully derived from the spec body's sibling-spec links (Q7), so `/clarify` (its **Recompute dependencies (safety net)** step) and `/plan` (its sub-step 1.1) run `derive-dependencies` report-only on entry, surfacing drift from uncommitted body edits; the author maintains links in the body and confirms nothing
- [x] AC14: `/implement` derives the write-boundary check from `git diff` against the spec dir's first commit, not from a manually maintained Affected Files list; the plan's Affected Files section is documented as a planning aid only
- [x] AC15: `/analyze` no longer offers `--fix` for checkbox state; the Fix Mode section is removed and the checkbox-related entries in the Fixable list are gone
- [x] AC16: Root `constitution.md` is deleted; `framework/constitution.md` is the sole canonical file; root `CLAUDE.md` imports it as `@framework/constitution.md`; no root-constitution generator exists
- [x] AC17: `/clarify`, `/plan`, and `/implement` each include a cross-spec scan step that surfaces sibling slugs cited in the body but not in `dependencies`; `/implement` completion additionally surfaces files outside the target spec dir touched during the implementation window. The scan in `/plan` and `/implement` is each command's sub-step 1.1
- [x] AC18: A new rule file `framework/rules/configuration-cross.md` exists with rules covering shared constants, module-local constants, configurable values backed by env vars, env-var defaults, env-var validation, and unit-suffixed time variables; each rule has a Verification step `/analyze` runs; constitution §constants and §env-vars shrink to one-line pointers
- [x] AC19: `/analyze` runs cleanly on every existing spec (000–016) plus this spec after migration — no new findings introduced by schema changes; stale fields in done specs are silently ignored per the open-schema rule
- [x] AC20: This spec's own frontmatter has `title:` and `tags:` removed by the final task
- [x] AC21: `framework/bootstrap/hooks/pre-commit` ships with the framework as an adopter-owned outer stub that calls the ductus-owned inner hook `framework/bootstrap/hooks/ductus-pre-commit`, which calls only adopter-relevant primitives (`derive-dependencies`, `derive-references`, `label-criteria`, `check-corpus-links`, `check-artifact-size`); no install script ships beside them, its install actions being in `framework/bootstrap/ductus-procedure.md` §Hook Installation
- [x] AC22: `/ductus` installs the adopter hook on first run when no existing hook system is detected, and warns and skips with a manual integration snippet when one is (a `core.hooksPath` pointing elsewhere, husky, lefthook, `.pre-commit-config.yaml`, or a `.githooks/pre-commit` that does not invoke `ductus-pre-commit`); it writes `.githooks/pre-commit`, an adopter-owned outer stub, once (`create`) and never updates it, and updates the ductus-owned inner `.githooks/ductus-pre-commit` each run unless pinned in `.ductus/config.toml`; an existing `.githooks/pre-commit` is told apart by whether a non-comment line of it names `ductus-pre-commit`, not by a sentinel (`framework/bootstrap/ductus-procedure.md` §Hook Installation)
- [x] AC23: The frontmatter derivations reach adopter projects as the `derive-dependencies` and `derive-references` runtime primitives, which arrive with the runtime `/ductus` acquires and which the shipped inner hook calls; no shell generator is scaffolded, refreshed, or pinned for them
- [x] AC24: A CI workflow runs all generators for real and fails the build when the tree is not clean afterwards, untracked paths included, which catches a generator that creates a new output rather than modifying a tracked one — in both this repo (`.github/workflows/generators.yml`) and, as a shipped example, adopter projects (`framework/templates/ci/adopter-generators.yml`); protects against contributors or adopters whose hook was skipped or never installed
- [x] AC25: Capturing a review finding that maps to no loaded rule does not depend on the reviewer remembering a separate step: recording an observation in the report puts it in front of the review's fix-and-route step, and the record counts it undispositioned until it is fixed, deferred, or waived, so an uncaptured observation is not a reachable state; an undecided observation does not hold the spec out of `done`, a broken contract apart
- [x] AC26: The routing rules that decide whether work becomes a new spec, a scenario on an existing spec, or a rule-file amendment bind wherever work enters — not only when it arrives through the inbox, where `/{project}:groom`'s decision tree happens to run
- [x] AC27: `framework/rules/configuration-cross.md` declares a third category, `FLAG` (feature flags), in its header, and `data-model.md` registers it in the ID-prefix and Category-name bullets and the category-abbreviation table; `CFG-FLAG-001` (MUST) and `CFG-FLAG-002` (SHOULD) are present, as are `CFG-ENV-008` (MUST), `CFG-ENV-009` (SHOULD), `CFG-ENV-010` (MUST), `CFG-ENV-011` (MUST) and `CFG-ENV-012` (SHOULD), each with a Statement, a Verification and a Source; and `data-model.md` § Rule set enumerates every rule in the file with its tier
- [x] AC28: `framework/rules/configuration-cross.md` carries `CFG-ENV-013` (MUST), with a Statement, a Verification and a Source, owning a secret written in source or a container-build manifest, committed to version control, passed to a container build, or placed into a container image or other build artifact; `CFG-ENV-009`, `CFG-ENV-010` and `framework/rules/security-backend.md` `BE-DATA-003`, `BE-DATA-008` and `BE-DATA-012` leave that case to it; and `data-model.md` § Rule set enumerates it with its tier

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Q1 (item #15, checkbox flipping):** Remove `--fix` entirely. Checkboxes are flipped only by `/implement` (per its existing per-criterion verification step at `framework/commands/implement.md`, "Walk through each acceptance criterion from the spec and verify it is met", at `7e390d3c`) and by `/clarify` (which moves resolved questions section-to-section). Hand-implementation that bypasses `/implement` is treated as out-of-framework — the framework does not ship a fix-up tool for it. Combined with the title-field removal in AC1, this leaves `--fix` mode with nothing to do, so the entire flag goes away from `/analyze`. Friction with parent-checkbox-not-flipped surfaces only outside the framework path; `/implement` already marks parents and sub-items together (`implement.md`, "including nested sub-item checkboxes", at `7e390d3c`).
- **Q2 (item #16, twin constitutions):** Collapse to a single canonical file at `framework/constitution.md`. Delete root `constitution.md`. Update root `CLAUDE.md` to `@import framework/constitution.md` and any root README link to point at `framework/constitution.md`. No generator, no section markers, no install-time stripper. Verified the two files were byte-identical at decision time (`git diff --quiet 7e390d3c:constitution.md 7e390d3c:framework/constitution.md` exits 0) — the documented "may diverge" pattern was theoretical and the discipline trap was guarding nothing. AGENTS.md already exists as the ductus-internal home for project-specific rules; anything that ever needs to be ductus-only belongs there, not in the constitution. If ductus-internal content ever needs to live in the constitution, a section-marker mechanism is introduced then, not preemptively. In adopter installs the constitution lands at `.ductus/constitution.md` (`AGENTS.md` §Project Structure). The AGENTS.md "mirror constitutions" instruction is removed by AC8.
- **Q3 (item #14, Affected Files diff window):** Diff against the spec dir's first commit (`git log specs/{feature}/ | tail -1`), filtered to files outside `specs/{feature}/`. Cumulative across multi-session implements — each new out-of-plan file surfaces once on first appearance, then is added to the plan and not re-surfaced. Equivalent to "since planned→in-progress transition" for clean pipelines (pre-implement work touches only the spec dir, which the filter excludes), but simpler to compute. No new state in `session.json`.
- **Q4 (item #17, cross-spec scan precision):** Inline markdown links only — match `[label](../NNN-feature/...)` or paths to `specs/NNN-feature/`. Bare slug mentions in prose are treated as commentary, not cross-references, on two grounds: (1) prose references to other specs are common and would flood the proposal with false positives, and (2) "always write proper links" is a markdown convention authors already follow, not a ductus-specific discipline. Rule-ID citations (e.g., `BE-AUTHN-001`) are out of scope here — those go through the existing rule-reference scan in `/analyze` (`analyze.md`'s "Rule references" scan, as `validate.md` at `7e390d3c`).
- **Q5 (item #18, configuration rule file naming):** Single file at `framework/rules/configuration-cross.md`. Both rule sets cover the same domain (operator-tunable values) and apply equally to backend and frontend code, so the security-files split-by-surface precedent does not apply. Splitting by topic would create artificial boundaries — env-var defaults *are* constants and `.env.example` is the manifest of env vars; they belong together.
- **Q6 (item #18, configuration rule ID prefix):** `CFG-` with three categories — `CONST` (shared / module-local constants), `ENV` (environment and configuration values) and `FLAG` (feature flags) — in the format `CFG-{CONST|ENV|FLAG}-{NNN}`. Examples: `CFG-CONST-001` (shared constants in one centralized location), `CFG-ENV-001` (every configuration value declared required, optional with a default constant, or optional with no default where its absence disables a named feature), `CFG-ENV-002` (a canonical inventory, in any accepted form, of every environment variable and every configuration-file key), `CFG-ENV-003` (required values from any source validated at startup), `CFG-ENV-004` (time-valued env vars state their unit). `data-model.md` § Rule set enumerates the current rules. Flags joined this file rather than a new one on Q5's reasoning: they are configuration, the same problem on both surfaces. The `ENV` category is named *Environment and configuration values* (owner decision, 2026-10-19), because `CFG-ENV-007`, `CFG-ENV-008` and `CFG-ENV-012` govern configuration files and flags as well as environment variables. A rule file's ID format is taken from its introducing spec's data model before the file's own header (`framework/reference/analyze.md` §Rules (blocking and advisory), *Which ID format governs a file*); this spec sets the configuration file's pattern.
- **Q7 (item #13, dependencies derivation + sync mechanism):** Body is authoritative; frontmatter `dependencies` is fully derived. The derivation runs in two places: (a) the pre-commit hook runs the `derive-dependencies` runtime primitive, which scans every spec body for inline markdown links to sibling specs (excluding code fences), recomputes the union, and rewrites the frontmatter list; (b) `/clarify`, `/plan`, `/implement`, `/amend` and `/target` run it report-only on entry, so drift from uncommitted body edits is surfaced rather than silently carried. The author maintains links in the body — there is no place to author the frontmatter list. To remove a dep, remove the inline link (or move it inside a code fence). To mention a spec without depending on it, use a bare slug per Q4.

  **Hooks are symmetric across ductus and adopters; content is asymmetric.** ductus's hook runs `gen-configure-mcp`, `gen-claude-commands` and `gen-help-tables` plus the `derive-dependencies` and `derive-references` primitives, `label-criteria` and `check-corpus-links`. Adopter projects' hook runs the two derivations, `label-criteria`, `check-corpus-links` and `check-artifact-size --project-files --staged`, and no generator.

  **`/ductus` manages the adopter hook on every install/update run.** The detection ladder, the write strategies and the skip-with-snippet path for an existing hook system live in `framework/bootstrap/ductus-procedure.md` §Hook Installation, which spec `018-adopter-owned-pre-commit` reshaped into an adopter-owned outer stub and a ductus-owned inner hook. The derivations arrive with the acquired runtime, so nothing for them is scaffolded, refreshed, or pinned.

  **CI safety net for both surfaces.** The same generators and derivations run in CI and the tree is compared afterwards; a diff fails the build. Catches contributors who never installed the hook locally and adopters whose hook was skipped due to existing-hook detection.

  **Migration of existing specs.** The 16 specs that existed when this landed had hand-authored frontmatter deps that might not match their body inline links, and the first derivation could *remove* deps that weren't body-linked. A one-time migration step added inline links to spec bodies for any frontmatter dep not already linked, so the first run removed nothing by surprise.

  **Recategorization.** Item #13 sits in the same conceptual bucket as items #11–#12 — generated artifacts maintained by the pre-commit hook.
- **Q8 (constitutional promotion of the discipline principle):** The principle lives in the constitution's §design-principles as *Never design a pipeline feature that depends on human diligence*, and `AGENTS.md` keeps a pointer rather than the rule. The maturation path is that principles prove themselves in `AGENTS.md`, then promote to the constitution if they generalize — the §rules tier in spec 016 followed that path after specs 008 and 015 established the pattern, and `050-constitution` promoted this principle the same way once this spec and the ones after it had applied it. Constitutional changes ship via `/ductus` to every adopter and are stickier to roll back, which is why promotion waits for the proving step.
- **Q9 (`CFG-ENV-013`, throwaway credentials):** A credential for a throwaway local or test resource that grants no access to a production or shared system is not a secret under the rule, stated in its Statement and Verification (a `docker-compose.yml` `POSTGRES_PASSWORD: postgres`, a committed `.env.test` key for a local emulator). Committing one exposes nothing, so flagging it taught authors to ignore the rule; the encrypted-copy permission stays as strict as it is, and `CFG-ENV-011` stays a MUST. Scenario `a-throwaway-credential-is-not-a-secret` (owner, 2026-11-19).
- **Q10 (the `BE-DEPS-006` signpost):** The signpost no longer says `SEC-DEPS-001` takes over `BE-DEPS-006`. 008 removes that rule, which no tagged release carried (`runtime/CHANGELOG.md` `[Unreleased]` Removed: "no tagged release shipped them"), so the sentence would name a removed ID; the signpost names `SEC-DEPS-001` alone, and this entry, task 47 and the scenario `the-adopter-ci-template-can-run` still name the removed ID, which is why `retired-rules:` lists it. No criterion in another spec asserts the signpost: a search of `specs/*/spec.md` for "takes over" on a line naming `SEC-DEPS-001` or `BE-DEPS-006` matched only this entry (owner, 2026-11-19; search run 2026-11-20).
- **Q11 (the reach of Q9's carve-out):** `CFG-ENV-001` and `CFG-ENV-010` keep "secret" unqualified. An in-application default runs wherever the application runs, production included, so a throwaway value supplied as one is the "local value reused in production" the scenario `a-throwaway-credential-is-not-a-secret` calls no throwaway; and an artifact delivered to end users is no throwaway local or test context. In `security-backend.md` the carve-out reaches `BE-DATA-012` alone, by 008's scenario `a-throwaway-credential-owes-no-revocation-path`. This spec's review of 2026-11-20 raised the question and waived it as the owner's (the agent's recommendation, adopted by the operator, 2026-11-23).
