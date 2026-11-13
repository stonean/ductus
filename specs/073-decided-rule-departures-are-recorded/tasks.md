# 073 — Decided Rule Departures Are Recorded Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Shared reader and history attribution

- [x] Create `runtime/src/primitives/departures.rs`: the entry parser of `data-model.md` §Departure entry (HTML comments and fenced code skipped; each structural problem reported with its line) and the attribution of §Attribution (the first commit of the unbroken run ending at `HEAD`; `unattributed` when shallow, no repository, absent from `HEAD`, or a revision unreadable)
- [x] Unit tests over repositories built in a temp directory: introduced, edited after the commit (keeps the introducing commit), removed and re-added (the re-adding commit), uncommitted, a spec directory renamed, a project in a repository subdirectory (`git_fixture::subdirectory_project`), and a shallow repository
- [x] A mutant per attribution state and per structural problem in `scripts/mutation/departures.py`, each killed (`QUAL-TEST-001`)

- **Done when**: every attribution state and structural problem in the data model has a passing test that its mutant turns red.

## 2. The `check-departures` primitive (AC1)

Blocked on task 1.

- [x] Create `runtime/src/primitives/check_departures.rs` with the result of `data-model.md` §check-departures result, judging each `cited:` through `resolve_anchor::run` against `paths::constitution_path(repo)`
- [x] Register it at the sites `docs/developing-ductus.md` §Adding or removing a runtime primitive lists, add its name to `framework/runtime-tools.txt`, and run `scripts/gen-configure-mcp.sh`
- [x] Fixtures: each Blocking and Advisory finding with a clean counterpart, a citation that cannot be judged recorded unexamined, a not-installed rule, and the template's commented example yielding no entry

- **Done when**: each AC1 condition has a fixture whose finding appears and whose clean counterpart yields none, `every_registry_name_has_a_case` passes, and the generated allow entry is in `framework/bootstrap/configure/claude.md`.

## 3. The `waived` verdict through the runtime (AC2, AC4)

Blocked on task 2.

- [x] Add `RuleVerdict::Waived` in `runtime/src/schema/primitives.rs`, with an arm at each exhaustive match the plan names
- [x] `AnalyzeTally::record_assessment` records `Waived` for a rule with an entry on any fired answer and counts it into no tier; a pass records nothing
- [x] `write-analysis` refuses `waived` for a rule with no entry and `finding` or `addressed` for one with an entry, fills the fields from the entry, and recomputes a carried `waived` verdict's fields on every write
- [x] Fixtures: a MUST rule that fires is `waived` with `blocking-findings` 0; a rule that does not fire has no verdict; a waiver carries forward while its rule fires and is gone when it does not (AC4); a removed entry makes the rule a blocking finding again; a fired, addressed rule is still `waived`; a commit after an `unattributed` run attributes the next write

- **Done when**: the AC2 and AC4 fixtures pass, a mutant for each refusal and for the recompute is killed, and `write-analysis` on a spec with no entry writes the record it wrote before.

## 4. The record and the result list waived rules (AC3)

Blocked on task 3.

- [x] `write-analysis` renders `## Waived rules` in `analysis.md` (MUST first, apart from the passed rules) and returns `waived` and `waived-count`, bounded as `contradictions` is
- [x] Fixtures: a run with a waived MUST and a run that satisfied it differ in the record and the result; the list moves to a lists file past the read size

- **Done when**: the two fixture records are distinguishable by `rule-verdicts`, the body section and the result, and the bounded form is proven in both directions.

## 5. Review honors the entries (AC5, AC8)

Blocked on task 1; independent of tasks 2 to 4.

- [x] `write-review`: each well-formed entry becomes a new waiver for its rule at the files it fires at that no waiver holds, with `data-model.md` §Review waiver marker's fields, and the run names each malformed entry
- [x] `process-waivers`: expire a marked waiver whose rule has no entry, with the `waiver expired:` notice
- [x] Fixtures: a MUST departure at two files (both anchors written), a file added later where the rule fires (covered with no second decision), a removed entry (the waivers expire and the MUST blocks again), a SHOULD departure (AC8, `should-violations` 0), and a hand `--waive` on the same rule and file staying the first claim

- **Done when**: the five fixtures pass, mutants for the marker's expiry and for the SHOULD path are killed, and the fixture's `review.md` lists each file's anchor under `waivers:`.

## 6. The history test in the job that has history (AC7)

Blocked on task 2.

- [x] Create `scripts/tests/test-departure-attribution.sh`: commit to a repository it builds under a temp directory, run the release `ductus check-departures` for the attributed, edited, uncommitted and re-added cases and against a `git clone --depth 1` for the shallow case, and fail loudly when its own checkout is shallow
- [x] Add its step to `framework-checks` in `.github/workflows/framework-checks.yml`, commented with the history it reads; `actionlint` clean
- [x] Prove the script fails with each case's expectation inverted

- **Done when**: the script passes from a full clone, fails with a message from a depth-1 clone of this repository, and runs in the job that sets `fetch-depth: 0`.

## 7. Command prose, references, template and constitution (AC6)

Blocked on tasks 2 to 5, so the prose lands with the behavior. Run `scripts/gen-claude-commands.sh` before invoking an edited command. The verifiers of tasks 4 and 5 named three more places the prose must reach. First, the review step reports `write-review`'s `malformed-departures`. Second, analyze step 22 reports `waived` and `waived-count`, and reads `waived` from the `lists-file` past the read size, as it reads `contradictions`. Third, the body skeleton in `docs/analyze.md` gains `## Waived rules`, and gains `## Contradictions`, which 082 never added.

- [ ] `framework/commands/analyze.md`: a step invoking `check-departures`; steps 11 and 12 answer `{verdict: waived}` for a listed rule whose trigger fires; step 22 reports `waived` before the passed rules
- [ ] `framework/commands/review.md` and `framework/reference/review.md` §Waivers: a departure needs no `--waive`
- [ ] `framework/reference/analyze.md`: a `Departure entries (blocking)` section and the `rule-verdicts` bullet of §Record fields; `docs/analyze.md`'s row
- [ ] `framework/templates/spec/spec.md`: the grammar inside the Applicable Rules comment; `scripts/audit/template-alignment.sh` judges the new heading
- [ ] `framework/constitution.md`: the plan's bullet added and its two sentences cut; `scripts/lint-governing-size.sh` exits 0
- [ ] Prose-claim sweep: grep `framework/`, `docs/` and `specs/` for claims that a rule verdict is only `addressed` or `finding` and that analyze has no waiver, and correct each
- [ ] Re-bless the `analyze-basic` and `review-basic` goldens if their transcripts move, reading each diff

- **Done when**: `scripts/audit/run-all.sh` and `scripts/tests/test-gen-claude-commands.sh` pass, the constitution is within its ceiling, and the sweep's greps return only intended text.

## 8. Discharge the declared impacts

Blocked on task 7. 082 is `done`: its data-model edit reopens it by `classify-spec-edit`, so record the reopen with `set-status` (`done` to `in-progress`) with the owner's approval, and leave its return to `done` to its own analysis.

- [ ] `specs/082-rule-assessments-converge-across-runs/data-model.md`: the third verdict shape and its handling, back-linking 073
- [ ] `specs/066-audit-records-and-the-completion-gate/data-model.md`: the `rule-verdicts` description, back-linking 073
- [ ] `specs/020-code-review/scenarios/waiver-expiry.md`: the exception sentence, back-linking 073
- [ ] `specs/022-deterministic-runtime/data-model.md`: the `assessSpecQuality` answer for a listed rule; signposts in `specs/050-constitution/spec.md`, `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` and `specs/067-analyze-and-audit-checks/spec.md`, each back-linking 073
- [ ] `runtime/CHANGELOG.md` `[Unreleased]` entry for the feature

- **Done when**: every spec in 073's `cross-spec-impact:` back-links to 073 and `check-artifacts` reports no cross-spec impact drift for 073.

## 9. Pin the landing verifier's remaining test gaps

- [x] `check-departures`: a wrapped `cited:` whose `§` reference is on its continuation line, a resolvable one yielding no finding and an unresolvable one a finding at that line (SHOULD, `QUAL-TEST-001`: `judge_citations`' `first..=last` narrowed to `first..=first` survived every test; landing verifier of `51775a3f`, 2026-11-13)
- [x] `departures.rs` `opens_block`: the one-to-nine-digit bound pinned on both sides, ten digits and zero digits opening no block (`(1..=12)` and `(0..=9)` survived)
- [x] `write-review`'s `malformed-departures` entry asserted exactly, its `line N` included (replacing `problem.line` with 0 survived)
- [x] `opens_block` and a thematic break or setext underline (`---`, `***`) directly after a reason: decide whether it ends the child as CommonMark renders it, and pin the choice

- **Done when**: each item has a test that a named mutant turns red, and the whole `cargo test` passes
