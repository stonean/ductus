# 071 — Command reference split Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Amend the constitution

- [x] Name `framework/reference/<command>.md` in §drift-prevention's canonical-sources row for pipeline command behavior.
- [x] Add the sentence on where a command's markdown-only prose lives to §runtime-host-integration.

- **Done when**: both edits are in `framework/constitution.md` and `resolve-anchor` over it reports no unresolved marker.

## 2. Cut each reference into its own file

- [x] Script the cut over the thirteen ranges the plan names, at the commit the task starts from: write `framework/reference/<command>.md` (H1, the moved body with headings promoted one level outside fences, numbered headings unnumbered) and truncate the command file at the range's start, keeping audit's two trailing sections.
- [x] Prove the move by diff: each reference body, with the promotion undone, equals its source range byte for byte.

- **Done when**: no `framework/commands/*.md` contains `## Markdown-only reference`, thirteen `framework/reference/*.md` exist, and the byte-equality check passes for all thirteen.

## 3. Repair the prose the move falsifies

Blocked on task 2: the repair reads the files the cut wrote.

- [x] In each command's kept part, find every pointer into the moved text in any wording and rewrite it to name the full heading and the file, as **§Heading** in `.ductus/reference/<command>.md`.
- [x] Add the location line after the *For agent runtimes* blockquote in each of the thirteen commands.
- [x] Open each reference file with its lead paragraph and a **Where a named section lives** paragraph.
- [x] Reword specify's `$ARGUMENTS` step and analyze's `{cli-config-dir}` line; turn intra-file links whose target stayed in the command into bolded section names; fix "above"/"below" whose target changed file.
- [x] Run `resolve-anchor` over every command and reference file and fix each unresolved anchor.
- [x] Run `check-corpus-links --scope repository` and fix each broken link under `framework/reference/`.

- **Done when**: every pointer names a heading that exists in the named reference file (checked by reading each against the file's headings), no `framework/reference/*.md` holds `{cli-config-dir}` or a `$ARGUMENTS` used as the argument token, and markdownlint is clean over both directories.

## 4. Sweep pointers from other live artifacts into moved sections

- [x] Re-point `framework/bootstrap/ductus.md`'s four `review.md` §Behavior step 5 citations at the section that holds rule-surface selection after the move.
- [x] Grep `framework/`, `docs/`, `README.md`, `AGENTS.md`, `scripts/` and `runtime/src/` for citations of a moved section by its command file, and re-point each at the reference file.
- [x] Correct the comments and descriptions that place a reference in its command file: `sibling-coupling.sh`, `template-alignment.sh`, `scripts/audit/README.md`, `check_artifacts.rs:9`, `schema/primitives.rs:4417`, and the `check-step-references` description in `mcp/server.rs` and `schema/primitives.rs`.

- **Done when**: no live artifact outside `specs/` cites a moved section as being in its command file, and none says a command's markdown-only reference is in the command file.

## 5. Ship the reference files through the installer

- [x] Add the twelve Shared Files rows, `audit` withheld.
- [x] Add `.ductus/reference/` to the enforce-directories list in `ductus.md`'s Instructions steps 1 and 8, and write the **Reference cleanup** paragraph for the markdown-only path.
- [x] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md`.

- **Done when**: `ductus.md` carries twelve reference rows and the cleanup on both paths, and Family 21 passes.

## 6. Dogfood the reference files here

- [x] Extend `scripts/gen-claude-commands.sh` to write, prune and `--check` `.ductus/reference/*.md`.
- [x] Stage `.ductus/reference/` in `.githooks/pre-commit`.
- [x] Run the generator and confirm `--check` passes, then fails on a hand-edited copy and on an orphan copy.

- **Done when**: thirteen `.ductus/reference/*.md` exist with `{project}` substituted, and `--check` has been seen to fail on drift in both directions.

## 7. Keep the audit families on their subjects

- [x] Point Family 18 at `framework/reference/analyze.md` and Family 28 at `framework/reference/audit.md`, labels and fixes included.
- [x] Extend Family 4 to `framework/reference/*.md` with the shared-file rule.
- [x] Add Family 16's reference arm.
- [x] Remove `lint-tool-coverage.sh`'s dead skip.
- [x] Prove each changed family fails on its defect: delete the Family 18 sentence, empty the Family 28 list, plant `{cli-config-dir}` in a reference file, drop a reference row, add a row with no source.

- **Done when**: each family passes on the tree and has been seen to report each seeded defect.

## 8. Trigger CI on reference edits

- [x] Add `framework/reference/*.md` beside both `framework/commands/*.md` path filters in `.github/workflows/runtime.yml`.

- **Done when**: both filters list it and `actionlint` passes.

## 9. Record the runtime half under 067

- [x] Reopen 067 through the back-edge and add its two scenarios, each back-linking to 071: the `.ductus/reference/` exclusion and the bounded `resolve-anchor` judgment.
- [x] Correct the resolved question in 067's `anchor-reference-kinds` to point at the new scenario.
- [x] Update 067's `data-model.md` `resolve-anchor` section for the `cross-document` kind and count, and add one task per scenario to 067's `tasks.md`.

- **Done when**: 067 is `in-progress` with both scenarios linking to 071, its data model matches the planned result shape, and each scenario has a task.

## 10. Exclude the generated reference copies from the link check

Blocked on task 9: the change is 067's scenario.

- [x] Add `.ductus/reference/` to `is_excluded`'s repository arm, with a test that fails when the exclusion is removed.

- **Done when**: `check-corpus-links --scope repository` is clean over this repository with the generated copies present, and the test has been seen to fail with the exclusion removed.

## 11. Judge pointers into shipped files

Blocked on task 9: the change is 067's scenario.

- [x] Read the destination → source map from the `## Shared Files` section of `framework/bootstrap/ductus.md`.
- [x] Add the `cross-document` kind and count to `resolve-anchor`, resolved by a heading in the mapped source, a heading in the citing file, or a marker.
- [x] Test each case: a heading resolves, a marker resolves, neither is unresolved, two documents stay qualified, an unmapped document stays qualified, an unreadable source stays qualified, and no map engages nothing. Mutate each path to see its test fail.
- [x] Report the count in Family 39's stderr line.
- [x] Run `resolve-anchor` over every tracked markdown file under `framework/` and `specs/` before and after, and confirm the change adds no unresolved reference beyond broken pointers into reference files.

- **Done when**: the tests pass and have been seen to fail under mutation, Family 39 passes with every pointer into a reference file judged, and the before/after corpus run shows no new unresolved reference.

## 12. Record the audit-family changes under 026

- [x] Reopen 026 through the back-edge and add one scenario, back-linking to 071, recording the changed subjects of Families 4, 16, 18, 28 and 39.
- [x] Correct the lines in 026's Family 18 and Family 28 scenarios and its AC21 that name the old file, and add a task for the scenario to 026's `tasks.md`.

- **Done when**: 026 is `in-progress`, its scenario links to 071, and no 026 artifact says Family 18 or 28 reads the command file.

## 13. Prove install and pruning in the parity fixture

- [x] Add a reference file to the `ductus-basic` staging archive, a manifest entry and an enforce-directory for `.ductus/reference/`, and a stale reference file in the fixture project.
- [x] Assert the reference file lands with `{project}` substituted and the stale one is removed, then re-bless `ductus-basic` filtered to its test and read the golden's diff.
- [x] Mutate: drop the manifest entry, then the enforce-directory, and see each assertion fail.
- **Correction (2026-10-26):** no enforce-directory entry was added to the fixture; the test calls `enforce-manifest` itself against `project/.ductus/reference`, with the reference rows as the expected list (AC15 reworded 2026-10-26 to match). The `ductus-basic` golden did not move: it records progress envelopes only.

- **Done when**: both assertions pass and have been seen to fail under mutation, and the golden diff holds only the new entries.

## 14. Re-bless the goldens the command edits moved

- [x] Run `cargo test --release --locked --test parity`; re-bless `implement-basic` filtered to its test and confirm its diff is the `first-commit` and `current-head` shas only.
- **Correction (2026-10-26):** the `implement-basic` diff also carries step 15's `gate-confirm` prompt prose, which quotes implement's step 15 and changed with the pointer the rewrite gave it.

- **Done when**: the parity suite passes and the only golden change from the command edits is those two shas.

## 15. Update the contributor docs

- [x] Extend `AGENTS.md`'s new-slash-command entry with the reference file, its manifest row, and the generator pass.
- [x] Update `docs/slash-commands.md` if it describes a command file's structure.

- **Done when**: the entry names all three, and `docs/` makes no claim the split falsified.

## 16. Measure after the split

- [x] Measure each command file and its reference after the split and record the before/after bytes per invocation in the plan's design record.

- **Done when**: the plan carries a table measured after the split, against the spec's before figures.

## 17. Run the whole local gate, after committing

- [x] Commit, then `scripts/gen-claude-commands.sh` → `scripts/audit/run-all.sh`.
- [x] `npx markdownlint-cli2`; the six `lint-*.sh`; `scripts/tests/*.sh`; `shellcheck -S warning` over the tracked shell set; `actionlint`; the generators plus `derive-dependencies` and `derive-references` reporting no drift.
- [x] Under `runtime/`: `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, `cargo test --release --locked`, `cargo audit`.

- **Done when**: every command has been run with its exit status read directly, and all pass.

## 18. Verify the adopter shape and both paths

- [x] In a scratch project, drive `apply-manifest` and `enforce-manifest` with the manifest `ductus.md` now describes against this checkout as the source root, and confirm `.ductus/reference/*.md` lands with `{project}` substituted, nothing lands in an agent's command directory, and a stale reference file is pruned.
- [x] Run `/ductus:analyze` on 071 with the MCP server and walk a command's reference-backed step on the markdown-only path, confirming the agent reads the named sections from `.ductus/reference/`.
- **Correction (2026-10-26):** the markdown-only walk verified was `/ductus:status` (commit 85b2e1a5), not `/ductus:analyze`.
- **Addendum (2026-10-26):** a `--ref=main` scratch adoption of all five agents on 2026-10-26 confirmed that the 12 reference files install with `{project}` substituted, none lands in any agent's command directory, every pointer in each layout's analyze resolves, and a stale reference is pruned on a re-run. Its commits were not made (permission denial), so the installed hook was not exercised.

- **Done when**: both verifications have run and their results are recorded in the commit that closes the task.

## 19. Disposition out-of-spec finding: status's markdown-only reference leaves several derivations unstated

- [x] `framework/reference/status.md` — a Read-only walk of `/ductus:status` on 2026-10-25 followed every pointer 071 wrote to its exact heading, but could not derive the Scenarios cell, the scenario-question callout, blocked-by, or the unknown-keys line: the reference does not say where scenarios live or how a scenario question is counted, how blocked-by is derived, which file "the installer" is, or the inbox bullet grammar. The text moved verbatim from `status.md`; 071 changed none of it. — discarded: the gaps predate 071 and lie in text it moved without change; the runtime path, required by §runtime-boundary principle 3, derives every one of those fields through `dashboard`, and the markdown-only path only bridges the window before the restart that loads the server. Completing status's markdown-only reference is its own change, not one to bundle into this split.

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task.

## 20. Disposition out-of-spec findings: installer defects from the --ref=main scratch adoption

- [x] `framework/templates/project/{claude-md,agents}.md` mark the constitutions region `<!-- ductus:constitutions -->`/`<!-- /ductus:constitutions -->` while `ductus.md` §Shared Files "Shared constitution imports" step 3 calls `merge-managed-block` html-comment style, which reads and writes `<!-- BEGIN ductus:constitutions -->`/`<!-- END … -->`, so a fresh adoption appends a second block and never maintains the template's region (eb043505, 055) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] AGENTS.md and CLAUDE.md map to `skip-if-conflict` on the runtime path (`ductus.md` §Instructions step 4; 022 scenarios `apply-manifest`, `govern-bootstrap`), which applies no substitution, so they ship with `{project-name}`, `{One-line project description.}` and `{project}` literal although §Shared Files says AGENTS.md is substituted (a3b02b9f, 022) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] the Pi bridge is a verbatim copy "from the staging archive" in the State B / MCP wiring step (`ductus.md` §MCP wiring and the registration table), which runs in pre-flight before the archive is fetched; §Pi layout's Bridge step writes it later (2c55c608, 064) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] §Instructions step 4's "single call" passes one `cli-config-dir` to every agent's Per-Agent Scaffolding entries, so a claude+auggie run gives each configure file the other's settings path; step 9's "single entry" is one per agent and omits pi's `{config_dir}/prompts/ductus.md` (a3b02b9f, 022; 2c55c608, 064) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] the Antigravity skill transform, the `name: ductus` wrap and the `{project}-*/` skill-directory cleanup are covered by no primitive, yet steps 4, 8 and 9 say the primitive calls replace those loops; the text never assigns them to the host (a3b02b9f, 022; 032) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] a greenfield project is prompted to apply nine no-op migrations (`ductus-procedure.md` §Pre-run Migrations: null `last_applied` qualifies every entry, no first-run exemption); `inbox-guidance-refresh` prints a line where the procedure says it exits silently; `generator-primitives.md` has `## Steps`, not `## Procedure` (55523e94, 027; 02b4e56f, 058) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] `ductus-procedure.md` step 8 says `check-orphaned-references`' `attribution` reads `registry`, but the primitive reads `framework/migrations.toml` from the project tree, so in an adopter it always reads `watermark` (1a5bf19f, 027/067) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] the first run's `.gitignore` is not a fixed point: language patterns appended after the managed-block merge repeat canonical lines (`.env`), and the next run's merge removes them from the language section `ductus.md` §Shared Files calls adopter territory (e71bd410, b1c5b4df, 022) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] the closing restart notice says "Installed `{version}` to `{store-path}`" and "wired in this run" when the store was already current or the wiring already present (a4a33584, 3103ef68, 048) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] the post-write integrity check says the body after the frontmatter "must start with `# ductus`" though every correct copy starts with a blank line (396952eb) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project
- [x] wording: step 5 says the `.gitignore` merge ends at the next blank line; steps 1 and 5 cite `specs/.cache/` where the template has `.ductus-cache/`; Shared constitution imports omit pi; `migrations/ductus-rename.md` registers Auggie with a bare `--command ductus` (a3b02b9f, 022; eb043505, 055; 9da4a7ae, 049) — fixed (owner decision 2026-10-26) in `b0b1f9d0`, with `083c4b08`'s follow-ups, and proven by a second `--ref=main` scratch adoption of all five agents on a greenfield project

- **Done when**: every item is fixed, routed, or discarded with the outcome written on it, and a second scratch adoption on the fixed tree proves the fixes

## 21. Disposition out-of-spec findings: ambiguities from the second scratch adoption

- [x] `ductus.md` §Instructions step 7 records `cli-config-dir` as "the agent's resolved config-dir" without saying which agent on a multi-agent run (the run used the running host's) — fixed (owner decision 2026-10-26) in `framework/bootstrap/ductus.md` and `govern.md`: (name the agent running the bootstrap)
- [x] §Instructions step 1 gives the per-agent ductus self-install entry `keep-literals: ["project", "cli-config-dir"]` while step 9 passes an empty substitution map — fixed (owner decision 2026-10-26) in `framework/bootstrap/ductus.md` and `govern.md`: (step 1 names step 9's empty map)
- [x] step 4 says an agent with no other Per-Agent entry needs no call of its own, implying Antigravity's rules mirror is an `apply-manifest` entry, while §Antigravity layout makes its source a walk of the project's own `specs/rules/`, which one archive-rooted call cannot read — fixed (owner decision 2026-10-26) in `framework/bootstrap/ductus.md` and `govern.md`: (the mirror is its own call rooted at the project)
- [x] the Pi bridge rides Pi's substituted per-agent call though §Pi layout says no substitution; it holds no placeholders, so the copy is byte-identical — discarded (owner decision 2026-10-26): (no effect)
- [x] Antigravity skills drop `argument-hint` (only `name:` and `description:` are carried) — discarded (owner decision 2026-10-26): (the Antigravity skill format the layout specifies)
- [x] Antigravity's MCP JSON must name the absolute store path, which `{store-path}`'s `~` form is not — discarded (owner decision 2026-10-26): (§MCP wiring already says absolute, and the run wrote it so)
- [x] Auggie's `mcp:ductus:*` wildcard is written "if Auggie's matcher honors the wildcard", which a run cannot verify — discarded (owner decision 2026-10-26): (host behavior this repository cannot test)
- [x] step 4 does not say whether the Shared Files call's map carries `cli-config-dir` — discarded (owner decision 2026-10-26): (no shared source holds that placeholder)
- [x] the Frontmatter Migration's `specs/**/spec.md` walk matches the template `specs/templates/spec.md`, skipped as already frontmatter — discarded (owner decision 2026-10-26): (harmless)
- [x] a re-run re-links an already-valid pointer, counted neither created nor repaired — discarded (owner decision 2026-10-26): (the restart notice correctly reads no files written)
- [x] `write-session` creates `.ductus/sessions/` with `.gitignore` and `.lock` and rewrites an unchanged `session.toml` — discarded (owner decision 2026-10-26): (062's per-session files, gitignored)
- [x] the run's host saw this repository's `ductus` tools as deferred and would have read State A — discarded (owner decision 2026-10-26): (an artifact of a subagent sharing the parent's MCP server, not an adopter shape)

- **Done when**: every item is fixed, routed, or discarded with the owner's decision written on it

## 22. Disposition out-of-spec finding: Antigravity's rules-mirror cleanup has no owner on the runtime path

- [ ] `framework/bootstrap/ductus.md` §Antigravity layout **Rules mirror cleanup** removes `{config_dir}/rules/*.md` files whose basename left `specs/rules/`, but step 8's enforce list excepts Antigravity and the **Host-side work** paragraph names only the skill transform, the `name: ductus` wrap and the `{project}-*/` directory cleanup, so on the runtime path no step owns it (pre-existing; found by re-read 1 of 44c0babe; 117413d6's step 4 made the mirror an apply-manifest call). Options, owner decision pending: (a) add Antigravity's `{config_dir}/rules/` to step 8's enforce list with the mirrored basenames as expected and pinned honored (enforce-manifest removes files, which is all this cleanup does), or (b) add it to the Host-side work list; copy ductus.md to govern.md either way

Handoff (2026-10-29). Ask the owner as one decision; (a) is recommended. Then 071's review record (`b540581c`) predates task 21 (`117413d6`), task 23 (reviewed in the `c1cf51ff` records), `d9ac40a3`'s Pi-bridge sentence and this task: re-record it by the reuse rule, carrying its 13 decisions. Re-analyze (tasks.md is stale; last `unexamined`: not-a-live-claim 2, ships-to-adopter 1, root-absent 2). Then the gate: re-measure AC20's table in `plan.md` with `git show HEAD:<path> | wc -c` for each `framework/commands/<cmd>.md` and `framework/reference/<cmd>.md`, stamping the sha; re-run AC18 (`scripts/audit/run-all.sh` after committing) and AC19 (every step of `framework-checks.yml`, `generators.yml` and `runtime.yml`, plus actionlint); verify all 24 criteria individually and tick each with `mark-criterion`. 071's runtime changes ship at 0.60.0 and are breaking, per `plan.md` Runtime.

- **Done when**: the item is fixed, routed, or discarded with the owner's decision written on it

## 23. Existing adopters' unmaintained constitutions region (task 20's first item, for files already installed)

- [x] Found by `050-constitution` task 40's real adopter bootstrap (2026-10-27): a subject adopted at `ductus-v0.59.11` keeps the template's `<!-- ductus:constitutions -->` region in `CLAUDE.md` and `AGENTS.md`, whose comment says `/ductus` maintains it, beside the `BEGIN`/`END` block the run appended. Task 20 fixed the templates in `b0b1f9d0`, which reaches new adoptions only; every adoption from `ductus-v0.47.0` through `v0.59.11` carries the region. The owner chose to fix it under this spec in that session (2026-10-27).
- [x] The migration `remove-legacy-constitutions-region` (0.60.0, `framework/migrations/remove-legacy-constitutions-region.md`; re-id'd in task 24) removes the region from each file when it is a shipped version, unedited, and leaves an edited, unclosed or pinned one with a report
- [x] A fixture walk of the procedure and a real adopter bootstrap of a `ductus-v0.59.11` subject show both regions removed, the `BEGIN`/`END` blocks intact, and a re-run writing nothing. Walked over 12 cases (2026-10-27), and over 15 after step 7 decided a region followed only by blank lines (`47e68233`), the real subjects' results byte-identical; bootstrapped on `f056b209`: both regions matched their shipped version and were removed with one blank line each, `CLAUDE.md`'s `BEGIN`/`END` block was unchanged, and the re-run left both files byte-identical

- **Done when**: the migration is registered, a fixture walk passes every case, and a real adopter bootstrap removes both regions and converges on a re-run

## 24. Re-id the constitutions-region migration so it sorts after the entry it shipped after

- [x] framework/migrations.toml — `legacy-constitutions-region` (task 23) and `memory-routing-leaves-claude-md` are both `introduced_in = "0.60.0"`, and the Pre-run Migrations filter breaks the tie on `id`, so the earlier-pushed `memory-routing-leaves-claude-md` sorted after it: a --ref=main adopter who ran /ductus between the two pushes on 2026-10-27 (14:35 and 17:34) recorded that id as `last_applied` and would never run this one (048's review, 2026-10-29; owner chose the re-id). Now `remove-legacy-constitutions-region`, ordered after it in the registry with a comment saying why, its procedure file and report strings renamed; no adopter's `last_applied` can name the old id, which never sorted newest, and the procedure no-ops where it already ran

- **Done when**: the entry's id sorts after every 0.60.0 entry pushed before it, the registry parses with no duplicate id, the procedure differs from task 23's only in the id, and no live artifact names the old id
