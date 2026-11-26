---
spec: 085-the-audit-record-gate-reads-records-through-the-runtime
last-run: 2026-11-26T16:07:35Z
reviewed-against: 8990a700fdb2b6bed9fba70b75ba5c6aa173b279
diff-base: 0d7425e528e516152da2a683bb8278f25705c5e9
must-violations: 0
should-violations: 0
low-confidence: 2
examined: 15
scope: 23
unread:
  - runtime/CHANGELOG.md
  - runtime/src/interpreter/mod.rs
  - runtime/src/main.rs
  - runtime/src/mcp/server.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - .claude/commands/ductus/configure.md
excluded: 29
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 3b98edd3a200e50faf9108076bbcbd1f85328eada87d1013ed5fe763b304c3d1
criteria-judged: 5
criteria-met: 5
criteria-verdicts:
  - label: AC1
    digest: 27bc06381f32a25238aff053e8958c1f93ffa0516b8186e0910405d13d75084b
  - label: AC2
    digest: abbe9d977f039ffb99b08c4d6f72af23a269f657d4bf9bc257f1bc142deb3d90
  - label: AC3
    digest: 64f48c5414ccd645f9645c6be1b00bccc0e0a12989c849a712b9d164ba11402c
  - label: AC4
    digest: 5c344ee50526dcb41e6252876c906e30c84b037062e2f28fc1272790350b2548
  - label: AC5
    digest: c7e18cdc187ee49b0bc4387f523525db8768263770cd8f76d98891f355244b68
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "perf: each present record's text is parsed twice, into the record type and into `BlockingProbe`, though the file is read once — `runtime/src/primitives/check_audit_records.rs`"
    outcome: waived
    reason: the plan's Trade-offs keep `blocking` out of the shared record types, which 066 owns and `check-review-gate` reads, and parse it from the text already read; the Cost section times the whole run at 0.06 s over 156 records, so the second parse costs no reader anything
    decided-at: 2026-11-26T16:06:43Z
    decided-by: rule
  - key: "other: `compute_reading` takes the record read as a `&mut dyn FnMut` only so a test can count reads — `runtime/src/primitives/check_audit_records.rs`"
    outcome: waived
    reason: the seam is what lets `each_record_file_is_read_once` and mutant M49 show a second read failing, as its doc comment says, and it adds one parameter, not a type or a trait
    decided-at: 2026-11-26T16:06:43Z
    decided-by: rule
---

# Review — 085-the-audit-record-gate-reads-records-through-the-runtime

## Summary

Window 0d7425e5 to 8990a700 (diff-base the commit before 085 went in-progress; base-source reopen): 7 commits examined, none claimed by no spec; 29 paths excluded, listed under Excluded from scope. 23 scope entries, 15 examined, each read whole by both pass parts, `runtime/src/primitives/check_audit_records.rs` and the three other added files among them. Unread, for the next review: the seven large shared files, read as 085's own commits' hunks and the context around them (the agent's recommendation, adopted by the operator, 2026-11-26): `runtime/CHANGELOG.md`, `runtime/src/interpreter/mod.rs`, `runtime/src/main.rs`, `runtime/src/mcp/server.rs`, `runtime/src/primitives/mod.rs`, `runtime/src/schema/descriptions.rs` and `runtime/src/schema/primitives.rs`; and `.claude/commands/ductus/configure.md`, generated from `framework/bootstrap/configure/claude.md`, which both parts read whole, and diffed whole against it with the generator's two substitutions, the only difference the generator's two-line `ToolSearch` header. Loaded rule files: `configuration-cross.md`, `quality-cross.md`, `security-backend.md`, `security-cross.md` and `security-frontend.md`, each read whole; no pass was skipped, and seven rule files are judged by analysis alone, by their headers. Two low-confidence SHOULD findings, which block nothing: `QUAL-DELEG-001` at `fields` and `CFG-ENV-012` at `parse_marks`. All five criteria judged met, each against the code its claim names. Observations: four fixed in the run (the Cost section re-measured at `8990a700`, the template's two stale counts, the equal pair `record_names` returned, and the mutation list's docstring) and two waived by the rule; the passes, re-run over the fix diff with each rule file read whole, found nothing new.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

0 of 7 commits in the window no spec claimed.

5 of 5 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-DELEG-001 — the step's lenient record parse was replaced by the gate's typed parse without naming every property that moved

- **File**: `runtime/src/primitives/check_audit_records.rs`
- **Site**: fields
- **Rule**: Replacing a hand-rolled predicate, parser, validator, or comparator with a shared one SHOULD be preceded by enumerating what the local version enforced by construction
- **Finding**: The old step read `blocking` as a raw scalar (`blocking.strip().lower() == "true"`), so `blocking: "false"`, `yes` or `0` passed; `fields` now runs the gate's typed `parse_record::<ReviewBlock/AnalyzeBlock>`, so each is a `record-unreadable` finding. Plan Trade-offs enumerate four narrower and three stricter differences, and the `RECORD_CASES` doc names only `blocking: null`; neither the call site, the plan nor a test names the non-boolean `blocking` spellings the old parse accepted.
- **Auto-fixable**: no
- **Suggested fix**: Add to the plan's Trade-offs 'Where it is stricter' entry, and to `RECORD_CASES`, the spellings the typed parse now rejects (`blocking: "false"`, `blocking: yes`), each case expecting `RecordUnreadable`; or waive citing that the gate's own loader rejects them too.

### LOW-CONFIDENCE: CFG-ENV-012 — an unrecognized key in the committed baseline file is silently ignored

- **File**: `runtime/src/primitives/check_audit_records.rs`
- **Site**: parse_marks
- **Rule**: An unrecognized key SHOULD NOT be silently ignored.
- **Finding**: `parse_marks` skips a line with no `:` and any key other than `review-exempt` and `analyze-exempt` (`_ => continue`), so a typo such as `review-exempt-count: 3` or `review-exampt: 3` is dropped with no finding and the mark stays zero. `.ductus/audit-record-baseline.txt` is a file the runtime owns and reads for a threshold, which `CFG-ENV-012` reads as a configuration file. The spec's Edge Cases and the data model decide this behavior (the format is the old step's), so the disposition is the operator's.
- **Auto-fixable**: no
- **Suggested fix**: Make an unknown key a `baseline-malformed` finding naming the key and file (a data-model, description and test change), or waive with the reason that the format is the step's ratchet format, which the spec keeps and which fails safe: a dropped key leaves the mark at zero, so an exempt set over zero is still reported.

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: `framework/templates/ci/adopter-generators.yml` step "Audit-record gate on done specs" runs only `ductus check-audit-records` and maps its stderr lines with shell `case` and parameter expansion; the python `frontmatter`/`baselines` parse (159 lines at `0d7425e5`) and the derive step's spec-root extraction are gone; proven by `scripts/tests/test-audit-record-gate.sh` and `runtime/tests/check_audit_records_cli.rs`.
- AC2 — met: `runtime/src/primitives/check_audit_records.rs` `Walk::spec`, `fields`, `judge` give absent (exempt set), unparseable (`record-unreadable`), null/blank/absent `last-run`, absent `blocking` (`BlockingProbe`), `blocking: true`, `spec-unreadable` (no/unreadable status via `frontmatter_status`) and `outside-project` (via `validate_feature_argument`), each on its file; proven by tests `each_record_finding_fails_with_its_condition_and_passes_without_it`, `a_spec_whose_status_cannot_be_read_is_a_finding_and_not_examined`, `a_spec_linked_out_of_the_project_is_a_finding`, `one_absent_record_counts_only_in_its_own_exempt_set`, which would fail on mutants M1, M4, M8-M12, M15, M16 in `scripts/mutation/audit-records-085.py`; 20 unit tests passed in a clone
- AC3 — met: `runtime/src/primitives/check_audit_records.rs` `read_baseline`, `parse_marks`, `exempt_set`, `over_mark` (absent file or omitted key is mark 0, malformed/unreadable/outward-linked file is a finding, set over mark is a finding per spec) and `audit_records_blocks` in `runtime/src/main.rs` (exit 1 on a finding); proven by `the_baseline_is_read_in_each_of_its_states`, `a_baseline_that_cannot_be_read_still_holds_the_sets`, `an_exempt_set_past_its_mark_is_a_finding_per_spec`, `a_baseline_linked_out_of_the_project_is_a_finding_and_never_read` and `a_finding_exits_one_naming_its_file` in `runtime/tests/check_audit_records_cli.rs`, which fail on mutants M17-M27 and M43/M48
- AC4 — met: `audit_records_lines` and `audit_records_blocks` in `runtime/src/main.rs` print the examined, done and skipped counts and each exempt count with its baseline on every run and exit 1 when `examined_count` is 0; proven by `a_clean_corpus_exits_zero_with_its_counts_reported` and `a_run_that_examined_no_spec_exits_one_saying_so` in `runtime/tests/check_audit_records_cli.rs`, and cases A and C in `scripts/tests/test-audit-record-gate.sh`, both layouts.
- AC5 — met: `scripts/tests/test-audit-record-gate.sh` (step (ae) in `.github/workflows/framework-checks.yml`) fails the step on each of the data model's ten finding kinds, annotated on its file, and passes the clean corpus, from a project at the repository root and from one in a subdirectory: 58 PASS and exit 0, re-run in the review's worktree.

## Observations

- other: 085's Cost section cites its measurement at `ed3375eb`, a commit on no branch since the rebase, and states the step at 3,460 bytes where `8990a700` holds 3,450 — `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md` — **fixed** (decided by the rule)
- convention: the template's header says two classes of check run above a list of three, and its `on:` comment says a hardcoded paths filter disables both gates — `framework/templates/ci/adopter-generators.yml` — **fixed** (decided by the rule)
- other: `record_names` returns a pair whose two elements are always equal, so each caller binds one value twice — `runtime/src/primitives/check_audit_records.rs` — **fixed** (decided by the rule)
- other: the mutation list's docstring says it is spec 085 task 1's, but it also holds task 4's mutants M16 and M49 — `scripts/mutation/audit-records-085.py` — **fixed** (decided by the rule)
- perf: each present record's text is parsed twice, into the record type and into `BlockingProbe`, though the file is read once — `runtime/src/primitives/check_audit_records.rs` — **waived**: the plan's Trade-offs keep `blocking` out of the shared record types, which 066 owns and `check-review-gate` reads, and parse it from the text already read; the Cost section times the whole run at 0.06 s over 156 records, so the second parse costs no reader anything (decided by the rule)
- other: `compute_reading` takes the record read as a `&mut dyn FnMut` only so a test can count reads — `runtime/src/primitives/check_audit_records.rs` — **waived**: the seam is what lets `each_record_file_is_read_once` and mutant M49 show a second read failing, as its doc comment says, and it adds one parameter, not a type or a trait (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/prune.md`
- `.pi/prompts/ductus-prune.md`
- `README.md`
- `docs/slash-commands.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/commands/prune.md`
- `framework/constitution.md`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_project/moves.rs`
- `runtime/src/session.rs`
- `scripts/mutation/compare-and-set-041.py`
- `scripts/mutation/project-parts-087.py`
- `scripts/mutation/prune-project-containment.py`
- `scripts/mutation/prune-project.py`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/050-constitution/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/075-agents-md-holds-what-every-session-needs/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`

## Unexamined governance

*None.*
