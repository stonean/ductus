---
status: in-progress
dependencies: [057-analyze-artifact-and-record-relocation, 084-reopen-for-owed-code-review-every-code-change]
cross-spec-impact: [057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate]
next-criterion: 6
---

# 085 — The audit-record gate reads records through the runtime

The adopter CI template's audit-record gate reads each `done` spec's review and analyze records through the runtime, never through a frontmatter parse of its own.

## Motivation

- `framework/templates/ci/adopter-generators.yml`'s step "Audit-record gate on done specs", which [057](../057-analyze-artifact-and-record-relocation/spec.md)'s AC17 states, read every spec's `spec.md` frontmatter and each `done` spec's `review.md` and `analysis.md` frontmatter through an inline python `frontmatter` function: top-level scalars, split at the first colon (read at `ea691221`). §runtime-boundary principle 3 rules out such a parse as a substitute for the runtime, and the template already acquired the runtime for its derive step.
- That parse was a second reader of the records `check-review-gate` reads on the way to `done`, so the two could disagree with nothing to report it.
- [084](../084-reopen-for-owed-code-review-every-code-change/spec.md) rejected the same parse for the CI steps it added to the template (its plan's Trade-offs), and left this step as it stood.

## Behavior

- The step calls one runtime primitive once over the corpus and parses no frontmatter or markdown itself. The primitive reads each `done` spec's two records by the reader `check-review-gate` uses.
- It keeps every property the step enforced: each record in one of three states, absent (never run), present, or present and unparseable (undeterminable, a finding); a present record with a null `last-run`, with no `blocking` field (absent is not `false`), or with `blocking: true` is a finding; a spec whose `spec.md` frontmatter cannot be read is a finding; the never-run set is bounded by the committed high-water marks (`audit-record-baseline.txt` in the project's `.ductus/`), zero when the file is absent and a finding when it is malformed; and the examined, `done` and exempt counts are reported on every run, an empty enumeration failing.
- It honours the job's commented `working-directory` and the configured spec root, and annotates each finding on its file from the repository root, as the step does (spec 059).
- The template's pinned runtime serves the primitive, so the pin moves in the release that ships it.

## Edge Cases

- A `spec.md` whose frontmatter parses but carries no `status` is a finding, as one that does not parse is: whether it is `done` is unknown, so its records were never examined. The step's own parse skipped it as not `done`.
- A spec directory that is or holds a symbolic link leaving the project is a finding naming the link, never read; an untracked spec is counted and skipped, as `check-owed-work`'s corpus form does.
- A baseline key the file does not name holds a mark of zero; a key the primitive does not know is ignored; a value that is not a whole number, or a file that cannot be read, is the malformed-file finding, and the exempt sets are still counted and reported.
- A `done` spec with one record present and the other absent counts only in the absent record's exempt set.
- A spec root that cannot be listed is an error, never zero specs examined, and the step fails on it.

## Cost

> **Signpost:** [089 — The pipeline's own cost is stated and bounded](../089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md) asks a spec that adds a primitive to state what a run of it costs, in a unit it can measure, and what it retires.

Measured at `ed3375eb` with the release build, over this repository's corpus of 83 specs, 78 at `done`:

- **A run.** One `check-audit-records` call took 0.06 s of wall time in four of five runs and 0.07 s in the fifth, by `/usr/bin/time -p` with a warm file cache, on a Darwin arm64 machine.
- **What it reads.** 10,922,907 bytes of file contents, by `wc -c` of the files it opens:
  - the 83 `spec.md` files, 1,913,074 bytes;
  - each `done` spec's `review.md` and `analysis.md`, once, 156 files and 9,006,762 bytes;
  - `.ductus/config.toml`, 955 bytes, read three times, by the spec-root, command-namespace and read-size readers;
  - the checkout's session file, 206 bytes, which CI does not hold.

  The tracked files' sizes are those of the commit's blobs. The count leaves out the git index the untracked listing reads and the directory walks.
- **What it adds.** One MCP tool.
  - Claude Code lists only its name in each session's deferred tools. Its description, 3,147 bytes, and input schema, 74, load when the tool is fetched: 3,221 characters by `scripts/measure-result-sizes.sh`, all ASCII.
  - Pi's bridge (`framework/bootstrap/pi/ductus-bridge.ts`) registers the tool with both at load.
  - Its result over that script's large fixture corpus is 21,197 characters.
- **What it retires.**
  - The step's inline python, 159 lines and 8,327 bytes between `python3 - <<'PY'` and `PY` at `ff1f2157`, so the step went from 195 lines and 10,350 bytes to 64 lines and 3,460 bytes, each counted from its `- name:` line through the next step's.
  - The derive step's spec-root extraction and output, which took that step from 26 lines and 1,487 bytes to 13 and 677.
  - No primitive, check or record field, since the step held its own reader, not one the runtime had.

## Acceptance Criteria

- [ ] AC1: `framework/templates/ci/adopter-generators.yml`'s audit-record gate calls one runtime primitive over the corpus and parses no frontmatter or markdown of its own.
- [ ] AC2: The primitive reports each `done` spec's review and analyze record as absent, present or unparseable, and reports as a finding naming its file a present record with a null `last-run`, with no `blocking` field or with `blocking: true`, and a spec whose `spec.md` frontmatter cannot be read or names no `status`, or whose directory holds a link leaving the project.
- [ ] AC3: The primitive holds each never-run set against its committed high-water mark, read from `.ductus/audit-record-baseline.txt`: a mark is zero when the file is absent or omits its key, a malformed or unreadable file is a finding naming it, and the step fails when either exempt set exceeds its mark.
- [ ] AC4: The step reports the specs it examined, those at `done` and each exempt count with its mark on every run, and fails when it examined none.
- [ ] AC5: A test proves each finding fails the step with its condition present and passes without it, from a project at the repository root and from one in a subdirectory.

## Applicable Rules

- `QUAL-CLAIM-001` — an unparseable record, an absent `blocking` and an empty enumeration are findings, never a clean pass.
- `QUAL-DELEG-001` — replacing the step's parse with the runtime's reader keeps every property the step enforced.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **A new spec, built for the next release** (the agent's recommendation, adopted by the operator, 2026-11-24; built without waiting for 084's bar, the owner's decision, 2026-11-25). The step predates 084, which the owner chose not to widen to cover it. A task on 057 would reopen it, and one on 066 would hold its close, one analysis away at `ea691221`, behind the build. Its review and analysis count in 084's bar (b), which gates nothing (084's Resolved Questions). Rejected: an inbox item, which §design-principles names no disposition; a waiver, since no requirement puts the step out of scope; and building it only once that bar is read, which holds the next release two weeks.
- **The primitive is new** (the agent's recommendation, adopted by the operator, 2026-11-25). It reads each `done` spec's two records by the read and parse `load_review_record` and `load_analyze_record` run, `read_record` and `parse_record` in `runtime/src/primitives/mod.rs`, whose `RecordLoad` holds the step's three states (absent, unreadable, present), each record file read once and its `blocking` probe parsed from the same text (the agent's recommendation, adopted by the operator, 2026-11-26), and walks the corpus as `check-owed-work`'s corpus form does, with its skip reasons (`runtime/src/primitives/check_owed_work.rs`, read at `4cfcf172`). Rejected: a corpus form of `check-review-gate`, whose first check answers `already-done` for exactly the `done` specs this one examines; and `check-owed-work`'s corpus form, whose `owes` is 084's one owed-work test, which a missing or blocking record must never feed.
- **The runtime reads the baseline file** (the agent's recommendation, adopted by the operator, 2026-11-25). The primitive reads `.ductus/audit-record-baseline.txt`, compares each exempt count with its mark, and reports a malformed or unreadable file as a finding, so the step parses nothing and one reader holds both the counts and the marks. The step was the file's only reader (`git grep audit-record-baseline` at `4cfcf172`); this repository's own audit reads a different file, `scripts/audit/analyze-record-baseline.txt`. Rejected: the step keeping its `baselines()` parse and mark comparison, about 45 lines no runtime test covers.
