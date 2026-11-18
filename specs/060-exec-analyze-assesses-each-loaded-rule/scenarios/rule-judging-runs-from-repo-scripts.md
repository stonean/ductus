---
section: "Acceptance Criteria"
---

# Rule-judging-runs-from-repo-scripts

## Context

AC10 has the host judge steps 11 and 12 in subagents, and `framework/reference/analyze.md` §Rule judging in a subagent says how each is briefed and what the host checks on the hand-back. Around that judging, analysing a spec is mechanical: running the deterministic steps, splitting the rules into shares, writing each brief, checking every hand-back against its share and every cite against a heading, combining cells when a spec's subjects exceed one judge's read, and building `process-decisions`' and `write-analysis`' arguments. On 2026-11-18 one session analysed 13 specs and rebuilt all of it in the system temp directory, where no later session finds it and nothing refines it (owner, 2026-11-18).

## Behavior

- The orchestration lives in this repository's `scripts/`, one script per step it automates, runnable on any feature: the detection steps through the release binary with a compact summary; the share split, from `discover-rule-files`' result, never a fixed list; the judge and step-15 briefs, from a template in the repository; the hand-back check, each judge's answers against its share and each cite against the design subject's headings; the cell combine, for any number of rule shares and scenario groups; and the record, `process-decisions` and `write-analysis` from a findings file, `decided-by` from `git config user.email`.
- The scripts write only staging files under the system temp directory, which the CLI requires; `analysis.md` is written through `write-analysis` alone.
- Each script has a test that fails when its behavior is removed (`QUAL-TEST-001`).
- `AGENTS.md` names the scripts, so a session analysing here uses them and fixes them where they fall short rather than writing its own.
- Rule judgment stays in subagents; the host still checks each hand-back before using it.

## Edge Cases

- **A read account that does not cover a subject** is reported, and the judge finishes before its answers count.
- **A cite naming a heading by a shortened form** does not resolve; the combine prefers a cell whose cite does.
- **Cells that disagree on a rule** are reported, never settled silently: settling is the host's, against the rule's Verification and the subjects.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
