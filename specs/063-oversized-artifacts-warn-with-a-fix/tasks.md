# 063 — Oversized artifacts warn with a fix Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Schema types and threshold resolution

- [x] Add `CheckArtifactSizeArgs`, `CheckArtifactSizeResult`, `ReadSizeThreshold`, `ThresholdSource`, `OversizedArtifact`, `ArtifactKind`, `ArtifactFix`, `FixKind`, `OnDone` and `DecisionsState` to `runtime/src/schema/primitives.rs` per [data-model.md](data-model.md), with a serde round-trip test.
- [x] Create `runtime/src/primitives/check_artifact_size.rs` with the threshold resolution: `[artifacts] read-size-bytes` through `paths::resolve_config`, held as a raw value; `default` when unset or the file is absent, `config` for a positive integer, `invalid` with `rejected` and a notice for anything else; a config file that does not parse is `PrimitiveError::Toml`.
- [x] Unit tests for each source: unset, absent file, valid value, and each invalid shape (string, float, zero, negative, array), plus the unparseable-file error.
- **Done when**: the threshold tests pass and an invalid value yields the 50,000-byte default with a notice naming the rejected value (AC4, AC5).

## 2. Measurement, fixes, and the warning

- [x] Enumerate the subjects in the data model's order; skip a missing subject with no entry; list one that exists but cannot be opened under `skipped` as `artifact-unreadable`; measure the rest by metadata length; compute `pages` and report those with two or more.
- [x] Compute `fixes` per kind and, on a `done` spec only, each fix's `on-done`; render `warning` and `message` in the data model's formats, with command names as `/{project}:…`.
- [x] Unit tests: `research.md`, `review.md`, `analysis.md` and an unknown file are never examined, whatever their size (AC3); a missing `plan.md` produces no entry (AC15); an unreadable subject is skipped, never clean (AC11); a CRLF file is measured as its bytes on disk, a file at the threshold is not reported and one byte over is (AC16); each kind's fixes match the spec's table (AC6); `on-done` is absent below `done` and `never`/`always`/`if-claim-changes` at `done` (AC8); no warning names a split command (AC9); every warning says *may* and none says *will* (AC14).
- **Done when**: the measurement and rendering tests pass.

## 3. The decided rule

- [x] Read `analysis.md`'s `decisions:` through `decisions::read_decisions`; set `decisions` to `absent`, `read` or `unreadable`, never failing on an unreadable list, and add the notice when it is unreadable.
- [x] Parse each `artifact-size` key under the message format; mark a subject decided by a stored discard for the same path at a page count greater than or equal to its own, and carry that key as `decision-key`.
- [x] Unit tests: a discard at the same page count decides; at a higher recorded count decides; at a lower recorded count does not (AC10); a routed decision, another path's discard, and an unparseable key never decide; an unreadable list leaves every subject undecided with the notice and no error.
- **Done when**: the decided-rule tests pass, including the growth-into-another-page case.

## 4. Register the primitive

- [x] Add `check-artifact-size` to `PRIMITIVE_REGISTRY`, the clap subcommand in `runtime/src/main.rs`, the MCP tool in `runtime/src/mcp/server.rs`, and the exec dispatch arm in `runtime/src/interpreter/mod.rs`.
- [x] Add it to `framework/runtime-tools.txt` and to the permission entries in `framework/bootstrap/configure/claude.md` and `framework/bootstrap/configure/auggie.md`.
- **Done when**: the registry set-equality test, `scripts/lint-tool-coverage.sh`, and the manifest-parity audit family pass.

## 5. `/analyze`: the `artifact-size` family

- [x] Add step 17 to `framework/commands/analyze.md` invoking `check-artifact-size` and recording each oversized subject as an advisory `artifact-size` finding, a skipped subject as unexamined, and a decided subject under its `decision-key`; say that the discard is the further fix offered (AC1, AC7). Include the markdown-only text (AC12).
- [x] Renumber steps 17–21 to 18–22 and every reference to them in the file; state at the stored-decisions step that an `artifact-size` finding fires its `decision-key` when decided.
- [x] Add an **Artifact size** section to the markdown-only reference and the family to its per-check severity assignment as advisory for good.
- [x] Count the step in `runtime/src/interpreter/analyze_tally.rs` (undecided and decided oversized subjects advisory, skipped ones unexamined under `artifact-unreadable`), update its step-number comments, add walker coverage, and re-bless `runtime/tests/golden/analyze-basic.jsonl`.
- [x] Sync the moved step numbers in `specs/058-findings-route-at-discovery/scenarios/only-unreadable-targets-retain-decisions.md` (step 17 → 18, twice) and `specs/058-findings-route-at-discovery/plan.md` (step 20 → 21), leaving 058 `done`.
- **Done when**: `scripts/lint-procedure-parseability.sh`, the step-reference audit family, the walker tests and the analyze golden pass, and an oversized fixture subject appears as an advisory `artifact-size` finding.

## 6. `/clarify` and `/plan`: the warning, and code out of plans

- [x] Add step 10 to `framework/commands/clarify.md` after `label-criteria`, printing each warning and the threshold notice and stating that it never blocks the transition; renumber steps 10–13 to 11–14 and every reference to them (the feature-walk range and the scenario-targeted step list included).
- [x] Add step 8 to `framework/commands/plan.md` after the task breakdown, likewise; renumber steps 8–10 to 9–11 and every reference to them; list the size check as advisory in the **Validation gate** reference beside markdownlint.
- [x] Give each new step the markdown-only text: measure with the host's file tools, and list a subject whose size cannot be reported as not examined (AC12).
- [x] Replace the Technical Decisions guidance in `framework/commands/plan.md` that code snippets, function signatures, and package paths belong in the plan with guidance to state the decision and its rationale and name the code by `path:line` (AC13).
- [x] Add walker coverage for both steps and re-bless `runtime/tests/golden/plan-basic.jsonl`.
- **Done when**: both commands parse, the step-reference audit family passes, the walker tests and the plan golden pass, and an oversized fixture subject produces a warning in both commands without blocking either transition (AC2).

## 7. Documentation

- [x] Add a `[artifacts]` bullet to `README.md`'s Configuration list and a commented `[artifacts]` block to `framework/bootstrap/ductus.md` §Project Configuration and its twin `framework/bootstrap/govern.md`, each stating the 50,000-byte default, that the value is a whole number of bytes, and that it cannot switch the check off (AC17).
- [x] Add the `artifact-size` family to `docs/analyze.md`.
- [x] Add the change to `runtime/CHANGELOG.md`'s `[Unreleased]` section.
- **Done when**: the transitional-bootstrap parity audit family passes and the three documentation sites state the default, the unit, and the absence of an off switch.

## 8. Re-review 058 so its record covers the synced scenario

Task 5's step-number sync rewrote 058's scenario `only-unreadable-targets-retain-decisions` (step 17 → 18). That is a content change to a durable contract, and not a sweep Family 19 excuses — its exemption counts a token pair only across files whose whole diff is a pure substitution, and `analyze.md`'s is not — so 058's review reads stale and `scripts/audit/run-all.sh` fails until 058 is reviewed again. 058 stays `done`; `/{project}:review` runs against a `done` spec.

- [x] Run `/ductus:review` against `058-findings-route-at-discovery`, dispositioning whatever it finds.
- **Done when**: `scripts/audit/review-freshness.sh` reports no stale review for 058.

## 9. Full local gate

- [x] Run the whole surface AGENTS.md's local-gate entry lists: markdownlint, the six lint scripts, the script tests, shellcheck, the generators and both derive commands with no drift, `scripts/audit/run-all.sh`, and `cargo fmt --check`, `clippy -D warnings`, `cargo test --release --locked` and `cargo audit` under `runtime/`.
- **Done when**: every check in the surface passes against the committed tree.

## 10. A scenario listing that drops an entry is not clean

`/ductus:review` found a `QUAL-CLAIM-001` gap in `check-artifact-size`'s scenario enumeration: an entry the `scenarios/` listing returned as an error, and a `*.md` whose name is not valid UTF-8, were both dropped without a trace, so the result read as fully examined while a scenario went unmeasured (AC11).

- [x] Record the `scenarios/` directory as skipped (`artifact-unreadable`) when any entry of its listing cannot be read, as an unlistable directory already is.
- [x] Measure a `*.md` whose name is not valid UTF-8 like any other scenario, reporting its path lossily, rather than dropping it.
- [x] Unit tests for both, built so each fails when its handling is removed.
- **Done when**: the tests pass and fail under the mutation that removes each behavior.

## 11. Name the threshold's bounds

`/ductus:analyze`'s rule assessment found `CFG-CONST-004` unmet: the threshold's valid range has a minimum, a whole number of bytes of at least 1 ([data-model.md](data-model.md)), which `check-artifact-size` checked as a bare `bytes > 0` and the plan committed to no named constant for. `CFG-CONST-003` asks the same of the default, which the plan did not name either. The review of c38053d7 missed the code half.

- [x] Add `MIN_READ_SIZE_BYTES` beside `DEFAULT_READ_SIZE_BYTES` in `runtime/src/primitives/check_artifact_size.rs` and check the configured value against it.
- [x] Name both constants in the plan's threshold decision.
- **Done when**: the threshold tests pass and fail when the minimum moves above 1.

## 12. An invalid read-size-bytes fails fast (CFG-ENV-008)

- [x] Implement the behavior described in `scenarios/an-invalid-threshold-fails-fast.md`
- [x] The runtime reader halts on a present read-size-bytes that is not a positive whole number, naming the setting, the value, and the accepted form; unset still means 50,000
- [x] The markdown-only path halts the same way
- [x] spec.md's invalid-threshold edge case and AC5, plan.md, data-model.md, README.md, framework/commands/analyze.md (and every other command source describing the threshold), the check-artifact-size tool description in runtime/src/mcp/server.rs, and the read-size-bytes comment in framework/bootstrap/ductus.md (and govern.md) state the fail-fast behavior
- [x] A test fails if the reader falls back to the default for a malformed value

- **Done when**: A malformed read-size-bytes halts every command that reads it, no artifact describes a reported fallback, and 063 is re-reviewed back to done.

## 13. An unknown [artifacts] key is named (CFG-ENV-012, for 019)

- [x] Implement the behavior described in `scenarios/an-unknown-artifacts-key-is-named.md`
- [x] check-artifact-size adds a notice naming each key in [artifacts] the shared registry does not recognize, and still runs at the threshold the recognized keys give
- [x] data-model.md's notices row names the notice
- [x] A test fails if a mistyped [artifacts] key is dropped without a notice

- **Done when**: A mistyped [artifacts] key is named among check-artifact-size notices on every command that reads it, and 063 is re-reviewed back to done.

## 14. Implement scenario: a-plan-trim-does-not-reopen-a-done-spec

- [x] Implement the behavior described in `scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- [x] `check_artifact_size::on_done` takes the artifact kind and returns `OnDone::Never` for a `plan.md` trim; `the_effect_on_done_is_named_only_at_done` asserts it and the plan warning's text, and mutation M4 in `scripts/mutation/plan-edit-takes-no-back-edge.py` kills it (for `041-task-pruning`'s `a-plan-edit-does-not-reopen-a-done-spec`)
- [x] §The warning names a fix, AC8 and `data-model.md`'s `on-done` row state the plan-trim effect

- **Done when**: The scenario's behavior holds in `runtime/src/primitives/check_artifact_size.rs`, its test fails with the behavior removed, and 063's spec and data model describe the advice as it runs.

## 15. Measure the project files

Blocked on `041-task-pruning`'s `/prune --project` task, because every message this task adds names that command.

- [ ] Implement the behavior described in `scenarios/project-files-are-measured.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.
