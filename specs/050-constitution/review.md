---
spec: 050-constitution
last-run: 2026-10-15T21:19:49Z
reviewed-against: 9f3ffc8402b0909086a9d5c40a7e76a8c17a7053
diff-base: 6ebe6c3978f8d6b1dcc4683de04180b1c5cd1b7c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 12
skipped-passes: []
reviewed-digest:
  scenarios/a-canonical-source-is-pointed-at-not-copied.md: 6936b866607a842ece8ebfd749223d4f7d98637200e8db87d85772702589e98a
  scenarios/a-declared-cross-spec-impact-gates-done.md: b84dfb3cd3652dc116fbdd7dee60199185ae661a3f0aefb7ad14bc64a2c1d4cc
  scenarios/a-measurement-states-its-method-and-units.md: 4c250a480c11e9788058f1012b647daa14aba2036d4a3baa9d0c1ba3c5965b2a
  scenarios/a-partial-read-is-not-a-read.md: 8d6851e20f8a2e9f083434a37cbf39ab3bca533ec819a8a258197e4afb797b07
  scenarios/a-project-command-is-run-not-reproduced.md: aedb9dd7e77284422f8650a7a43c0a3f660b1eafc13f9f78408444532592ecfc
  scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
  scenarios/a-retired-filename-leaves-a-decision-record.md: f84a3177744eb03a859d82a4017495c24bcf24466448b7c49ad1a253147ab581
  scenarios/a-review-may-precede-its-commit.md: e72adeae4243fa510cf762fc3e569f7764e9fb50653cddb076c541ee0f62cb2b
  scenarios/a-status-commit-holds-only-the-transition.md: 0d3fb1a701bc4b3112cfc2e54bb488df060c75fceeb0dc6c6b39205eb01d4438
  scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
  scenarios/findings-route-by-scope.md: a29ba00826b6109390c701ebd3eabdd8a1bd687f972f77339e57b629c6122226
  scenarios/governance-is-multi-source.md: ae59aca7a049317806297839a73cf335eeb3764db97999a7ddc0b12103e6ffeb
  scenarios/knowledge-routes-by-population-not-by-kind.md: 0e934ea65cbaf2bf31acc4f45c461bfb868be45deb5aa9fa6601c59b8c003c5c
  scenarios/report-outcomes-not-edits.md: caf13342d7fe0a5af4108024cf424ab921030c896f0743307cf5111be2e163ab
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 050-constitution

## Summary

Review of task 31, the fourth classification round, run at 9f3ffc84 with one fix in the working tree. The window is 6ebe6c39 (the parent of the reopen) to the working tree. Its 050 work: plan.md §Classification gains 11 rows (9 project-only, 2 promote); framework/constitution.md extends §implement-phase's review-window bullet with the converse case (a fix committed before the reopen that records it lies outside the window) and §concurrent-features' What-stays-shared paragraph with what a push carries; AGENTS.md turns both promoted entries into pointers with byte-identical leads; tasks.md and spec.md carry task 31 and the reopen. Examined 5 of the 12 in scope: those five files, read in full where they changed and around each edit. Not examined here: the five 022 files in the window (fetch_archive.rs, 022's plan, spec, tasks and new scenario), which landed after this reopen under 022's own reopen and are reviewed in 022's record; and specs/inbox.md and 045's spec.md, plan-affected but unchanged since the base. AGENTS.md's own commit 6ebe6c39 (the rule-scope entry and the run-all.sh trim) is the base itself, so it sits outside the window; its lines were read as part of AGENTS.md, which is in scope, and the new entry is one of the 11 rows classified here. Rule files: all eleven framework/rules/*.md; the change is prose in the constitution and this repository's notes, so quality-cross.md (QUAL-CLAIM-001 in particular) was applied, and the code-surface rules have nothing to fire on. Verified: promotion-coverage reports 140 rule-bearing, 140 classified, 0 unclassified, 0 unmatched keys; each promoted rule's distinctive phrasing appears once as a normative statement, with pointers only elsewhere (AC3); the anchor set is unchanged. One observation, fixed in the run: the --since pointer's how-to now reads as a way to name a base the fix comes after, not a second rule.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention — AGENTS.md's --since pointer called the previous review's reviewed-against "usually the right base" while §implement-phase now names the commit before the fix, so the two read as different rules — `AGENTS.md:50` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
