# 045 — Decision-State Drift Detection Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Tasks 3–10 are authored as scenarios under [022 — Deterministic Runtime](../022-deterministic-runtime/spec.md) per the plan's Implementation ownership split; each carries a back-link to this spec. Task 2 opens that back-edge and must land before any runtime work.

## 1. Amend constitution §drift-prevention with the decision trigger

- [ ] Add a `### Decision resolution` subsection to §drift-prevention in `framework/constitution.md`, immediately after `### Cross-document references`
- [ ] State that resolving a decision carries the same audit obligation as editing a document, and name the recognizable events: closing an open question, shipping a scenario, advancing a status, adopting a previously-rejected option (AC1)
- [ ] State that a resolution is incomplete while a sibling artifact still describes the prior state (AC2)
- [ ] Add a Canonical sources row pointing the open-state tell list at `specs/045-decision-state-drift-detection/data-model.md`
- [ ] Confirm no `§drift-prevention` anchor consumers break — the marker and section name are unchanged, only content is added

- **Done when**: `framework/constitution.md` §drift-prevention carries the decision trigger and the completion rule, the Canonical sources table names the tell list's owner, and `/gov:analyze`'s anchor resolution still resolves every `§drift-prevention` reference.

## 2. Open the 022 back-edge and author the four runtime scenarios

- [ ] Revert `022-deterministic-runtime` from `done` to `in-progress` via `set-status`, matching the edge 046 took for the same reason
- [ ] Author `scenarios/block-element-scanner.md`, `scenarios/check-artifacts-skipped-targets.md`, `scenarios/link-adjacent-drift-family.md`, and `scenarios/criterion-path-existence-family.md` under 022, each back-linking to this spec per §cross-spec-impact
- [ ] Add a task to 022's `tasks.md` for each scenario so the scenario→task mapping family stays clean
- [ ] Update 022's `data-model.md` `check-artifacts` section: the family paragraph goes from five to seven, and the result shape gains `skipped`
- [ ] Confirm `traverse-deps` still reports 022 compatible for this spec at `in-progress`

- **Done when**: 022 is `in-progress` with four back-linking scenarios and matching tasks, its data-model names seven families and the `skipped` field, and `check-artifacts` on 022 reports no scenario-mapping finding.

## 3. Add the block-level splitter and expose the code-span helper

- [ ] Promote `inline_code_spans` in `runtime/src/primitives/mod.rs` from private to `pub(crate)` with no behavior change
- [ ] Add a `pub(crate)` block splitter yielding `(line_number, text)` for each block-level element: table row (line starting `|`), list item (bullet line plus indented continuations), paragraph (maximal run of other non-blank, non-heading lines)
- [ ] Feed every line through `SkipScanner` so fenced code blocks and HTML comments are dropped; do **not** modify `SkipScanner` itself
- [ ] Drop blockquote lines (trimmed form starting `>`) in the splitter
- [ ] Unit-test each block kind, the four exempt contexts, and a mixed document where a tell appears once in each exempt context and once in live prose

- **Done when**: the splitter's unit tests pass, `cargo test` is green across the existing `tasks.md` parsers (proving `SkipScanner`'s contract is untouched), and a tell inside a fence, an HTML comment, a blockquote, or a code span is provably not returned as live block text (AC13).

## 4. Add `SkippedTarget` and the `skipped` field

- [ ] Add the `SkippedTarget` type to `runtime/src/schema/primitives.rs` per [data-model.md](data-model.md), with the closed `reason` set
- [ ] Add `skipped: Vec<SkippedTarget>` to `CheckArtifactsResult`, leaving `clean` defined as `findings.is_empty()`
- [ ] Document on the field why an unexaminable target is recorded rather than silently dropped, citing `QUAL-CLAIM-001`
- [ ] Confirm the five existing families still return an empty `skipped` and that no existing test's expectations change

- **Done when**: `CheckArtifactsResult` carries `skipped`, the existing `check_artifacts` test suite passes unmodified, and a result cannot report zero findings for an unexamined target without recording why.

## 5. Resolve sibling links and read target state

- [ ] Add inline-link extraction over a block's text, skipping matches inside inline code spans
- [ ] Resolve each target lexically against the containing file's directory; keep only targets landing inside the feature directory; strip a trailing `#fragment`
- [ ] Reject URL-scheme targets (`http:`, `https:`, `mailto:`) and bare-fragment targets before resolution
- [ ] Read target state per [data-model.md](data-model.md): `status` plus open-question count for `spec.md`, open-question count for a scenario, existence for the rest — reusing `read-spec` rather than a new parser
- [ ] Unit-test that `../spec.md` from a scenario is a sibling and `../022-deterministic-runtime/spec.md` from `spec.md` is not

- **Done when**: sibling classification and target-state reads are unit-tested for all three target kinds, cross-feature and external links are provably excluded, and no `std::fs::canonicalize` call is on the path.

## 6. Implement the `link-adjacent-drift` family

- [ ] Scan `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md`, enumerating scenarios via `list_scenario_files` (AC6)
- [ ] Match the six closed tells from [data-model.md](data-model.md) at offsets outside every code span (AC11)
- [ ] Evaluate per link and emit at most one advisory finding per (block, link) pair, naming the citing file and line, the link target, the tells that fired in list order, and the target's contradicting state (ACs 3, 4, 12)
- [ ] Apply the contradiction mapping so a scenario target evaluates question-state and existence tells only, and implementation-state tells against it emit nothing (AC14)
- [ ] Record `no-readable-state`, `target-missing`, and `target-unparseable` skips rather than emitting findings (AC9)
- [ ] Test a feature directory whose prose matches its link targets' state and assert zero findings (AC5)

- **Done when**: the family emits the expected advisory findings for the observed case's first three rows, emits nothing for a consistent feature directory, and every unexaminable target appears in `skipped` instead of `findings`.

## 7. Prove determinism and pin the recorded limitation

- [ ] Add a test running the family twice over an unchanged fixture and asserting byte-identical findings and skips (AC8)
- [ ] Add a test pinning the recorded non-goal: a link whose target exists but whose cited section changed produces no finding
- [ ] Assert every emitted finding carries `severity: advisory` and that `check-artifacts` reports no blocking finding from either new family (AC7)

- **Done when**: repeat runs are byte-identical, the documented semantic limitation is pinned by a test rather than only by prose, and no new family can block a gate.

## 8. Implement the `criterion-path-existence` family

- [ ] Gate the family to specs at `status: done`, reading acceptance criteria through `read-spec`'s parsed `acceptance-criteria` (AC17)
- [ ] Extract candidate paths from inline code spans only, applying the grammar in [data-model.md](data-model.md) (AC16)
- [ ] Resolve each candidate repo-root-relative, accepting a file or a directory and stripping a trailing `/`
- [ ] Emit one advisory finding per unresolved path, naming the criterion text and the path
- [ ] Test that body prose naming a deleted path produces no finding, so 026's own Behavior §3 retirement note stays clean (AC17)
- [ ] Test the grammar's rejections: a slash-command reference, a `path:line` citation, a glob, a URL, a flag

- **Done when**: the family flags unresolved acceptance-criterion paths on `done` specs only, never flags body prose, and every grammar rejection is covered by a test.

## 9. Reproduce the originating case as a fixture test

- [ ] Add a fixture reproducing 026's AC5 — a `done` spec whose acceptance criterion names `framework/workflows/registry.json` and `scripts/audit/registry-equivalence.sh`, neither present
- [ ] Assert both paths produce findings (AC18)
- [ ] Assert the same fixture at a non-`done` status produces none

- **Done when**: the fixture test fails against the pre-change runtime and passes after, demonstrating the check catches the case that motivated it.

## 10. Sweep the family lists that are already stale

- [ ] Update `check_artifacts.rs` module docs from five families to seven, describing both additions
- [ ] Update the `ArtifactFinding::family` doc comment, which currently names four families and predates 046's fifth
- [ ] Update the `CheckArtifactsResult::findings` doc comment's family-order note
- [ ] Update the `check-artifacts` `#[tool(description = …)]` in `runtime/src/mcp/server.rs`, which also names four
- [ ] Grep `framework/`, `specs/`, `docs/`, and `README.md` for other family-count claims about `check-artifacts` and correct each

- **Done when**: no live artifact states a `check-artifacts` family count or list that disagrees with the seven implemented families.

## 11. Document both checks in `analyze.md`

- [ ] Update step 8's prose from five families to seven, naming both additions and their advisory severity
- [ ] Add a `### Link-adjacent decision drift (advisory)` section to the markdown-only reference: the tell list, the block unit, the four exempt contexts, per-link evaluation, and the target-state mapping
- [ ] Add a `### Acceptance-criterion path existence (advisory)` section: `done`-spec scope, code-span-only extraction, and why body prose is excluded
- [ ] Document the shared promotion criterion once — 5+ findings across a repo on two consecutive `--all` runs **and** every finding in those runs confirmed a true positive — and state why the precision half is required here but not for the LLM-judged checks
- [ ] Document that unexamined targets surface in the Informational tier, cross-referencing the cross-service unknowns that already sit there
- [ ] Confirm the markdown-only prose is walkable without the runtime and states the same rules the primitive implements (AC15)
- [ ] Keep placeholders unsubstituted in the source and let the pre-commit hook regenerate the deployed command file

- **Done when**: `analyze.md` documents both checks and the shared promotion criterion alongside the existing advisory checks (AC10), the placeholder-roundtrip audit family passes, and the regenerated command file appears in the same commit.

## 12. Dogfood both families across the repo

- [ ] Run `check-artifacts` across all 47 specs and collect every finding from the two new families
- [ ] Triage each finding as a true positive or a false positive, and record the tally as the promotion criterion's first precision data point
- [ ] Fix any true positive found in `govern`'s own artifacts, or log it to the inbox when it belongs to a `done` spec and needs the back-edge
- [ ] Confirm no false positive traces to a defect in the grammar or the exempt-context handling; if one does, fix the check rather than the artifact

- **Done when**: the full-repo run is triaged, the tally is recorded in the spec or the inbox, and every false positive is either eliminated or documented as an accepted limitation.

## 13. Verify the feature end to end

- [ ] Walk all eighteen acceptance criteria against shipped behavior and mark each
- [ ] Confirm the five pre-existing families' behavior is unchanged
- [ ] Run the full runtime test suite, `scripts/audit/run-all.sh`, and the feature directory's markdown lint
- [ ] Confirm the four 022 scenarios each back-link here and that 022's data-model is current
- [ ] Return 022 to `done` through its own completion gate

- **Done when**: every acceptance criterion is verified against shipped behavior and both specs' artifacts are consistent.

## 14. Release the runtime

- [ ] Run `cargo fmt` and `cargo clippy` under `runtime/`, then the full `cargo test` suite
- [ ] Run `scripts/audit/run-all.sh` locally — it is a hard release gate and any finding aborts the publish
- [ ] Bump `runtime/Cargo.toml` to `0.26.0` and add the matching `runtime/CHANGELOG.md` section
- [ ] Commit to `main` and push, then tag `gvrn-v0.26.0` at that commit and push the tag
- [ ] Confirm the `runtime-release` workflow completes green so the binaries and the crates.io publish actually land
- [ ] Do not `BLESS=1` any golden — the parity goldens carry the `{{runtime-version}}` placeholder and no golden should change

- **Done when**: `gvrn-v0.26.0` is tagged, pushed, and published green, and the goldens are byte-identical to their pre-bump state.
