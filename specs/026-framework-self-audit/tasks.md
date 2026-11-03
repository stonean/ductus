# 026 — Framework self-audit (`/audit`) Tasks

Tasks derived from the [plan](plan.md). Complete in order.

<!-- Each task should be small enough to implement and verify independently.
     Mark subtasks as they are completed. Every task MUST close with a
     `- **Done when**: …` line stating its completion condition — the
     tooling reads this exact form to confirm the task is fully specified.

     A task body may also carry working notes — what the next session needs
     to resume it: the mechanics, the order, what is left on disk — placed
     as §tasks-phase says. Write them as prose on the pending task they
     concern, so they go when the task is pruned. Put an ordering constraint
     on the task that must wait, never above the first task or under a
     heading of its own, which outlive every task; and never write a note as
     a checkbox, which would count toward the task's completion. Anything
     that must outlast the task belongs in its durable home first. Example:

## 1. Create sessions table migration

- [ ] Write SQL migration for `sessions` table
- [ ] Run migration and verify schema

- **Done when**: the migration applies cleanly and `sessions` matches the data model.

## 2. Implement session store

Blocked on task 1: the store tests run against the migrated schema.

- [ ] Create `shared/auth/session.go` with Create, Get, Delete methods
- [ ] Write store integration tests against real PostgreSQL

- **Done when**: all store methods are covered by passing integration tests.

## 3. Update README link to migration guide

- [ ] Edit `README.md` to point at the new path

- **Done when**: the README link resolves to the new path.

-->

## 1. Implement scenario: families-follow-the-command-reference-split

- [x] Implement the behavior described in `scenarios/families-follow-the-command-reference-split.md`
- [x] Families 18 and 28 read `framework/reference/analyze.md` and `framework/reference/audit.md`
- [x] Family 4 scans `framework/reference/*.md` under the shared-file rule; Family 16 gains its reference arm; Family 39 reports and judges `cross-document` references
- [x] Each family seen to fail on its seeded defect

- **Done when**: every changed family passes on the tree and has been seen to report each seeded defect (spec 071 tasks 7 and 11).

## 2. Record the review and analysis of the 071 reopen's fixes

Blocked on 067 task 14 and 071 task 22: both change files in 026's scope.

- [ ] Record 026's review
- [ ] Analyze 026, then run its completion gate, re-verifying its 26 ticked criteria

Handoff (2026-10-29). 026's record (2026-10-16) is review-stale. For the reuse rule: two slices read 026 at `72288af1`; fix rounds `44c0babe` and `d9ac40a3` were re-read and `c16eee54` (prose) was not; the records written at `c1cf51ff` in 002, 017, 050, 051, 052, 062, 064 and 072 cover `af99793c..c1cf51ff`; no record read `70bb4af5`. Carry byte-identically, fired through process-decisions: 10 of the 13 decisions in 071's `review.md` (omit the two on `scripts/gen-claude-commands.sh` and the one on `scripts/audit/marker-list-parity.sh`, outside this scope), the `c1cf51ff` records' in-scope ones, and 026's own `family-19-review-freshness` waiver, which still holds. The Summary states the fixed findings: QUAL-STUB-001 MUST (Family 6 writes a stderr skip line), QUAL-GROUND-001 twice (Families 1b/1c guard the constitution; Family 5 guards analyze's blocking headings), and that Families 28's and 34's findings are recorded in 067's review, whose scope holds those scripts. Observations (owner decisions 2026-10-26):

- Fixed: Family 16 judged no pointer into an uninstalled reference (`installer-command-parity.sh`), and its "two counts" fix text; AC21's §Scripts agreement was unscoped and one-way; the family-34 scenario's 80 where the family reports 90; placeholder-roundtrip's "fires today"; lib.sh and README §Contract on the python finding shape, the one-file edit and "four families"; 1c said it extracts edges and checks wording (`cross-doc-consistency.sh`); "the audit runs before the build" and the workflow-level `contents: write` (`runtime-release.yml`); the PR paths filter and the "eight checks" count (`framework-checks.yml`); no pinned toolchain in the release audit job, and check-zero hid cargo's output; audit.md's removed `docs/introduction.md` and its Boundary table's missing row; AC10's tense claim; the scaffold never checked the spec was tracked (`adopter-shell-behavior.sh`); round 2 and 3's sentences (Q4, AC7, AC8, AC10, Family 28's three-set wording, Family 6's header).
- Waived, with the same text and reason as 067 task 14's where the path is shared: pair_document restating the pair grammar; a maintainer-only reference allowed from any file; exact lowercase names only; step 7 with `--agents=` omitting the host; 1c reducing bullets to distinct edges.
- Deferred: the Antigravity rules-mirror cleanup to 071 task 22; fixed once that lands.

- **Done when**: 026's review and analysis are recorded and not stale, and every criterion is re-verified against the tree

## 3. Implement scenario: family-40-deprecated-rules-are-removed-on-time

- [x] Implement the behavior described in `scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- [x] `scripts/audit/deprecation-removal.sh`, registered in `scripts/audit/run-all.sh` and listed in `framework/reference/audit.md` and `scripts/audit/README.md` §Scripts
- [x] `scripts/tests/test-deprecation-removal.sh` shows each case the scenario's last edge case names, and `framework-checks.yml` step (p) runs it
- [x] The family passes on the tree, its coverage line counting the four labels 077 wrote against `version` 0.59.11

Eight independent re-reads ran on 2026-11-02 and 2026-11-03. The first's findings are fixed in `62814405`, whose message lists them, the second's in the items below, and the third's to eighth's, MUSTs and SHOULDs on test coverage, 067's contract and the label paragraph's end, in the commit that ticks them, whose message lists them and the NOTEs it waived.

- [x] Tests that fail without each behaviour the scenario claims, with any defect they expose fixed: two deprecated rules in one file, both compared; two labels in one section; a `### Notes` heading after a rule ending its section; a rule file that is not UTF-8, giving the parse-failure finding; python3 missing from `PATH`. Until then, `framework/reference/audit.md` item 40's and `scripts/audit/README.md`'s "proves each case" overclaim
- [x] A target counts only when it starts like a version (`removed in` then an optional backtick and `v?` and a digit), so a label reading "removed in favour of `SEC-X-001`; removed in 0.62.0" names one target, and a label paragraph ends at a line opening with `>` or `**Field:**`, so a Statement after the label is not read as part of it
- [x] Messages that say what is wrong: an unreadable `version` reports it unreadable; a label opening its line in another form (`**DEPRECATED:**`) says it is not in the form `**DEPRECATED in X.Y.Z:**`; one target named twice says so
- [x] 067's scenario `deprecation-labels-are-read-by-the-runtime` states the primitive's result per label line, with the rule section it falls in if any, so Family 40's outside-section findings survive the move to the primitive
- [x] The "mirrors the runtime" claim is made true or bounded: a tab after a rule heading's hashes crashes the parse (`level = atx(line) or 4`, as the runtime defaults to 4); a rule heading nested in another rule's section is read differently; the pathspec becomes `':(glob)framework/rules/*.md'`, as the runtime's discovery does not descend into subdirectories
- [x] The family rejects a deprecating version that is not MAJOR.MINOR.PATCH (`**DEPRECATED in v0.60:**`), which 008's `data-model.md` Deprecated row does not require, and the deprecating version is never compared. — owner's call: drop the check; or state MAJOR.MINOR.PATCH for `{version}` in 008's row — stated: 008's `data-model.md` Deprecated row reads "`{version}` is the deprecating release as `MAJOR.MINOR.PATCH`", and the family keeps the check, its fix text citing that row (owner, 2026-11-02, option b)
- [x] Family 40 holds every shipped rule file to the label form 008's `data-model.md` states, but `configuration-cross.md`'s schema, 017's `data-model.md`, restated a Deprecated row that had drifted to "Includes the removal target version", so a CFG label written to it would draw a finding its schema does not explain. Found while fixing the second re-read's items. — owner's call: replace 017's row with a pointer to 008's Deprecated row, as 036 and 077 point at 008's schema; or sync the copy to 008's wording — a pointer: 017's row reads "As the Deprecated row of `specs/008-security-rules/data-model.md`, which states the label's form, its removal target and what holds a release to it." (owner, 2026-11-02, option a)
- [x] Test T pinned a rule file that is not UTF-8 as failing the whole parse, the expectation the second re-read's item set, while 067's scenario `deprecation-labels-are-read-by-the-runtime` has the replacing primitive report a file it cannot read and count it unexamined, and says Family 40's test passes unchanged against it; the runtime's `read_text` fails on invalid UTF-8 as an I/O error. Found by the fourth re-read. — owner's call: Family 40 treats a non-UTF-8 file as one it cannot read; or keep the whole-parse failure and state it in 067's scenario — an unreadable file: the family reports it "not UTF-8, so its deprecation labels were not examined", does not count it as examined and still compares every other file's labels, test T shows it beside a python3 that fails, and the scenario's edge case, `audit.md` item 40 and the README say so (owner, 2026-11-03, option a)
- [x] An independent agent re-reads the fix round, re-running both earlier rounds' probes and mutants, repeated until a round finds no MUST or SHOULD. Rounds 3 to 7 each found a MUST, from round 4 on only test-coverage gaps, while the family matched the runtime throughout; 078's spec bounds such a loop at two finding re-reads. — owner's call: repeat until a round is clean; one more round, then stop; or stop now — one more round: round 8 re-reads round 7's fixes, and then the loop stops, the owner deferring or waiving any finding still open (owner, 2026-11-03, option b). Round 8 found two MUSTs, both test-coverage gaps where the family behaved correctly: case F proved the any-case match with a capital R only, and case I's loop asserted no count for its blockquoted, indented and mid-line labels. — owner's call, each: fix as prescribed, defer, or waive — fixed as prescribed: F holds an all-caps and a mixed-case target, and I's loop asserts "1 deprecation label(s) found, 0 compared", each proven by its surviving mutants dying on both trees (owner, 2026-11-03, option a, both). Its six NOTEs are waived, with reasons, in the message of the commit that ticks this item

- **Done when**: Family 40 passes on the tree, its test passes, and the test is seen to fail when the family's comparison is removed (077 task 8, owner, 2026-11-02).
