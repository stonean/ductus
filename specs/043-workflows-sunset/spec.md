---
status: done
dependencies: [010-agent-autonomy, 018-adopter-owned-pre-commit, 019-config-decisions, 027-bootstrap-migration-registry]
next-criterion: 12
---

# 043 — Workflows sunset

Remove the workflows feature (`005-workflows`) from the framework: the `framework/workflows/` directory (tech-stack workflow templates plus `registry.json`), the `/ductus` **Workflow recommendation** flow that scaffolds them as slash commands, and the `[workflows] declined_categories` config surface. A new entry in the [bootstrap migration registry](../027-bootstrap-migration-registry/spec.md) cleans up what earlier `/ductus` runs scaffolded into adopter projects.

## Motivation

The workflows feature scaffolds tech-stack-specific lint/test/format/migrate slash commands (`/ductus:workflows:eslint`, `/ductus:workflows:pytest`, …) into adopter projects. In practice these are never run as slash commands: lint, format, and test invocation is the territory of pre-commit hooks, editor configs, and CI — automation surfaces the adopter already owns ([018-adopter-owned-pre-commit](../018-adopter-owned-pre-commit/spec.md) established that ductus does not own the adopter's pre-commit rules; the same reasoning applies to the rest of this tooling class). Wrapping `eslint` or `pytest` in a slash command adds an LLM round-trip to commands that are cheaper, faster, and more reliable invoked directly by deterministic tooling.

Carrying the feature has real cost: a 13-template directory plus registry to maintain, an ~80-line recommendation flow in the bootstrap procedure with per-category prompts and decline persistence, a `[workflows]` config section with its own validation and write-policy rules, per-agent layout branches, and two historical migrations dedicated to renaming its files. Removing it narrows ductus to its actual responsibility — the spec pipeline — and shortens every `/ductus` run.

## Removal surface (framework side)

- **`framework/workflows/`** — the entire directory is deleted: `registry.json` and all workflow templates.
- **`framework/bootstrap/ductus.md`** — the **Workflow recommendation** section; the `framework/workflows/registry.json` → `workflows/registry.json` Shared Files manifest row; the `[workflows]` section of the config schema and its `declined_categories` documentation; the workflow mentions in the procedural-fidelity preamble, the write-policy paragraph, the managed-block preserve list, the enforce-manifest step note, and the per-layout skip instructions. The **Tech Stack table parsing** spec text (canonical keys `backend_language` etc.) lives inside the recommendation flow and goes with it unless clarify surfaces a residual consumer.
- **`framework/constitution.md`** — the Workflow registry row in the canonical-source map.
- **Command sources** — the `[workflows]` mention in `link.md`'s illustrative preserve list; the "lint/format/test workflows" phrasing in `groom.md` (reword to name adopter-owned tooling rather than the removed feature).
- **Templates** — the `framework/workflows/` disambiguation note in `framework/templates/project/agents.md` is removed.
- **Docs** — README (config-section docs, example TOML, repo-layout listing, any narrative claims that `/ductus` offers workflows) and AGENTS.md (project-structure bullet, procedural-fidelity mirror, the ships-as-is gotcha).
- **Runtime** — comment-level references only (`[workflows]` writes in the config-path doc comment); no workflow-specific runtime behavior existed. Since `e7044738` the unknown-key report registers `[workflows]` as a retired section, so a leftover one is reported key by key until the migration removes it (`runtime/src/schema/config_keys.rs`).
- **Generated copies** — `.claude/commands/ductus/*.md` regenerate from source, so no hand sweep remains.
- **Migration registry** — the `skills-to-workflows` and `workflow-filename-rename` entries and their `framework/migrations/*.md` procedure files leave the tree; their procedure text is archived under root `CHANGELOG.md` § Archived migrations (see Resolved Questions).
- **Sibling spec bodies** — done specs whose bodies document the removed surface as live behavior ([019-config-decisions](../019-config-decisions/spec.md)'s `[workflows]` persistence, [010-agent-autonomy](../010-agent-autonomy/spec.md)'s rename delivery) carry no workflow-feature criteria: a sunset note is not kept, since the change that supersedes a requirement removes it from every artifact that states it (spec 050's AC27), so they state only what holds now.

Prose-claim sweep discipline applies (a behavior change, not just a rename): claims that `/ductus` "recommends" or "scaffolds" workflows must be found by meaning, not token grep. Out of scope: `.github/workflows/` (GitHub Actions CI paths) and generic uses of the word "workflow" (e.g. the constitution's "principles, workflow, and quality gates") are unrelated and untouched.

## Out of scope

- **The AGENTS.md Tech Stack table survives this sunset.** It keeps standalone documentation value and `/ductus:review`'s `[review] tech-stack-verified` flow ([019-config-decisions](../019-config-decisions/spec.md)) still consumes it. Only the workflow-registry trigger consumption is removed.
- **GitHub Actions** — nothing under `.github/workflows/` or the CI template (`framework/templates/ci/adopter-generators.yml` references the Actions path, not the feature) changes beyond incidental prose.
- **Replacement tooling** — ductus does not ship pre-commit rules or editor configs in workflows' place; that surface is adopter-owned by design.

## Acceptance Criteria

- [x] AC1: `framework/workflows/` does not exist, and no live artifact (framework/, scripts/, runtime/, .github/, docs/, README.md, AGENTS.md, spec bodies) references it or the feature's surfaces (`workflows/registry.json`, `[workflows] declined_categories`, the Workflow recommendation flow) as a surface that still works, excluding GitHub Actions paths, generic uses of the word, and the records of the removal, which name those surfaces as a removal must: the `workflows-sunset` migration names `workflows/registry.json` and the `[workflows]` section it deletes, and the unknown-key report names a leftover `[workflows]` section (`framework/bootstrap/ductus.md`, Unknown keys; `runtime/src/schema/config_keys.rs`).
- [x] AC2: The `/ductus` procedure contains no workflow scaffolding: no recommendation flow, no registry manifest row, no `[workflows]` config schema, no workflow prompts in the procedural-fidelity preamble.
- [x] AC7: `skills-to-workflows` and `workflow-filename-rename` have no entry of their own in `framework/migrations.toml`; their procedure text appears under root `CHANGELOG.md` § Archived migrations and their `framework/migrations/*.md` files are gone. The `workflows-sunset` entry's summary names both ids as the archived migrations it subsumes.
- [x] AC8: `005-workflows` is consolidated into this spec and its directory removed, per §spec-lifecycle's rule that a retired feature's spec is deleted, not left at `done`.
- [x] AC9: The constitution's canonical-source map has no Workflow registry row.
- [x] AC10: `scripts/audit/run-all.sh` passes clean after the removal.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Customized scaffolded workflow files: exact set + pinned.** The migration follows `workflow-filename-rename`'s shape, not `skills-to-workflows`' recursive delete: delete by exact filename match against the 13 registry template names in `{config_dir}/commands/{project}/workflows/`, honoring `.ductus/config.toml` `[pinned] files` per file (`pinned (kept): …` summary line); remove the `workflows/` directory only when it is empty afterward, so adopter-authored files (e.g. `pytest-fast.md`) survive and keep the directory alive; remove the synced `workflows/registry.json` with the same pinned check. No inner prompt — the registry's outer "apply N pending migrations" prompt is the only gate (procedural-fidelity rule: pinning is the documented opt-out), and each removal reports a post-scaffolding summary line.
- **`[workflows]` config section: the migration deletes it.** The byte-for-byte preservation discipline governs writers touching a *sibling* section; a registered migration is the framework's designated mechanism for schema moves (042's migration relocated the whole config file). After this spec no shipped artifact documents `[workflows]`, and an undocumented section surviving in adopter configs is §drift-prevention's target case. The migration removes the section (header, keys, attached comment lines, normalizing the surrounding blank line), preserves every other table byte-for-byte, reports the removal in the post-scaffolding summary, and no-ops when the section is absent.
- **005-era migrations: subsumed and archived.** `workflows-sunset`'s deletion set covers the 13 current template filenames **and** the 9 legacy `{category}-{language}-{tool}.md` names, and it also removes a legacy `{config_dir}/commands/{project}/skills/` directory (recursive, pinned-checked — necessarily stale per `skills-to-workflows`' own reasoning), so an adopter stranded pre-rename is still fully cleaned. With their targets subsumed, `skills-to-workflows` and `workflow-filename-rename` (both past `sunset_after = "0.10.0"`) leave `framework/migrations.toml` and their procedure text is archived under root `CHANGELOG.md` § Archived migrations per [027](../027-bootstrap-migration-registry/spec.md)'s sunset flow — its first real exercise. The four other expired entries (`gitignore-marker-rename`, `governance-config-rename`, `spec-and-plan-sunset`, `rule-files-relocate`) are out of scope: unrelated maintenance, not bundled with this feature.
- **Spec 005 is consolidated into this spec and its directory removed.** [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle) says *a retired feature's spec is deleted, not left at `done`*: `done` means *delivered, and still true*, and nothing this spec removed is still true of the project. The first disposition, `status: done` with a sunset blockquote at the top of 005's body, was chosen before that rule existed and is not operative.
- **Tech Stack table parsing leaves `framework/bootstrap/ductus.md` entirely.** Grounded by inspection: the canonical-key parsing (`Layer` → `backend_language` / `frontend_language` / …, inside the Workflow recommendation flow) feeds only the registry trigger matching, and `registry.json` is the only other file in the repo naming those keys. `/ductus:review`'s tech-stack alignment check reads the AGENTS.md Tech Stack section as freeform prose under host judgment and never consumes the parsed keys. No residual consumer — the parsing text is removed with the flow; the AGENTS.md Tech Stack table keeps its documentation role and review-side consumer.
- **Did the `workflows-sunset` migration ship, and where is it now?** Yes: it shipped (introduced 0.23.0) and was archived under [027](../027-bootstrap-migration-registry/spec.md)'s sunset rule after 0.25.0, so `framework/migrations.toml` no longer carries it. Root `CHANGELOG.md` § Archived migrations is the record of its recipe (owner decision, 2026-11-27, adopting the agent's recommendation).
