---
spec: 050-constitution
last-run: 2026-10-17T13:52:52Z
reviewed-against: 164f2209bdf3dfc456e9b0b48cb66c1a377440a0
diff-base: ce4e73ea8c9d4e0b9eb8d0a5b94f363b86184b9c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 9
skipped-passes: []
reviewed-digest:
  scenarios/a-canonical-source-is-pointed-at-not-copied.md: 6936b866607a842ece8ebfd749223d4f7d98637200e8db87d85772702589e98a
  scenarios/a-declared-cross-spec-impact-gates-done.md: b84dfb3cd3652dc116fbdd7dee60199185ae661a3f0aefb7ad14bc64a2c1d4cc
  scenarios/a-measurement-states-its-method-and-units.md: 4c250a480c11e9788058f1012b647daa14aba2036d4a3baa9d0c1ba3c5965b2a
  scenarios/a-partial-read-is-not-a-read.md: 8d6851e20f8a2e9f083434a37cbf39ab3bca533ec819a8a258197e4afb797b07
  scenarios/a-project-command-is-run-not-reproduced.md: aedb9dd7e77284422f8650a7a43c0a3f660b1eafc13f9f78408444532592ecfc
  scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
  scenarios/a-retired-filename-leaves-a-decision-record.md: f84a3177744eb03a859d82a4017495c24bcf24466448b7c49ad1a253147ab581
  scenarios/a-review-may-precede-its-commit.md: 922278626cb1df8c001117f0567ad266d5efaf70b45dfba900e7e4ff0df26958
  scenarios/a-status-commit-holds-only-the-transition.md: 0d3fb1a701bc4b3112cfc2e54bb488df060c75fceeb0dc6c6b39205eb01d4438
  scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
  scenarios/findings-route-by-scope.md: a29ba00826b6109390c701ebd3eabdd8a1bd687f972f77339e57b629c6122226
  scenarios/governance-is-multi-source.md: ae59aca7a049317806297839a73cf335eeb3764db97999a7ddc0b12103e6ffeb
  scenarios/knowledge-routes-by-population-not-by-kind.md: 0e934ea65cbaf2bf31acc4f45c461bfb868be45deb5aa9fa6601c59b8c003c5c
  scenarios/report-outcomes-not-edits.md: caf13342d7fe0a5af4108024cf424ab921030c896f0743307cf5111be2e163ab
  scenarios/reports-end-with-the-remaining-work.md: b642a2d08b2227e856aefba9fa3ec8cd1efae0eb2a14cc674dfcd455982dbec3
  scenarios/the-constitution-states-rules-not-stories.md: 2461f7371619e2dbd236a602b2dc76fb77ab844accb7d22b495d38e54b0a0d05
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 050-constitution

## Summary

Re-review after 050's reopen added the reporting rule (1cd22539) and tightened the whole constitution (979d77a2, 164f2209). Two new scenarios staled the review. Diff base ce4e73ea, the parent of the reopen commit; scope 9 entries. Rule files: all 11 under framework/rules/ loaded — quality-cross.md and configuration-cross.md read in full, the nine surface files at rule-ID and Statement level, since the scope is markdown prose and no surface rule's trigger fires on it. Examined 6. Read in full: framework/constitution.md, the entire tightened document, checked rule by rule against the committed base for a dropped or changed meaning; and the scenarios reports-end-with-the-remaining-work and the-constitution-states-rules-not-stories. Read in part: 050's tasks.md (tasks 32 and 33), plan.md (§Classification, edited last round), and AGENTS.md (Workflow and Gotchas). A structure comparison held all 31 anchor markers, 77 headings, 76 table lines, both code fences and all 130 bold rule leads byte-identical, with RFC 2119 keyword counts unchanged except one noun use of SHOULD. Not examined, 3: 050's spec.md, whose only change in the window is its status line; specs/045-decision-state-drift-detection/spec.md and specs/inbox.md, plan-affected entries unchanged since the diff base. The full read found one unclear sentence (the opt-in invariant described as a cycle rather than a job asserting one), fixed before this record (164f2209). 0 MUST, 0 SHOULD, 0 low-confidence; no open observations.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
