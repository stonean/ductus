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

- **Done when**: both assertions pass and have been seen to fail under mutation, and the golden diff holds only the new entries.

## 14. Re-bless the goldens the command edits moved

- [x] Run `cargo test --release --locked --test parity`; re-bless `implement-basic` filtered to its test and confirm its diff is the `first-commit` and `current-head` shas only.

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
- [ ] Run `/ductus:analyze` on 071 with the MCP server and walk a command's reference-backed step on the markdown-only path, confirming the agent reads the named sections from `.ductus/reference/`.

- **Done when**: both verifications have run and their results are recorded in the commit that closes the task.
