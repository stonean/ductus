---
spec: 050-constitution
scenario: a-review-may-precede-its-commit
last-run: 2026-10-15T19:26:21Z
reviewed-against: 37fe8e117aaf3c3b0859788dcf011a1545d15e6b
diff-base: f0c26020b7cd87af3adce037880dd11fbba24d9c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 8
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
  fixed: 2
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 050-constitution

## Summary

Recorded before the work was committed, in the order this change makes legitimate. reviewed-against 37fe8e11 is the amend commit, and the reviewed change is in the working tree over it. The window is f0c26020 (the parent of the reopen, 37fe8e11) to the working tree. compute-review-scope, served by a runtime that includes 0.59.4's fix, listed all six changed files in modified-since, including the three uncommitted ones (framework/constitution.md, AGENTS.md, plan.md). The change retires §implement-phase's commit-first review bullet (scenario a-review-may-precede-its-commit, task 30). It also narrows two sentences that assumed that order: §grounding's commit-verification rule, and §implement-phase's review-window bullet, which predated 0.59.4's HEAD base for an uncommitted reopen. AGENTS.md's mirror is rewritten as a pointer to §grounding's history-reading rule, and the W7 classification row is re-keyed to the new lead with the retirement noted. Rule files loaded: all eleven selected by discover-rule-files. The change is prose in the constitution, AGENTS.md and 050's own artifacts, with no code, configuration or endpoint, so no rule in any of them applies; quality-cross.md's rules are code patterns by its own declaration. No MUST or SHOULD finding. Two observations, both fixed in the run after confirmation, with the passes re-run over the fixed text. The new sentences said a review's digest holds what it read, while reviewed-digest covers only the durable contracts; the five sites now say a digest of the spec artifacts it read. The scenario's Context stated the retired bullet in the present tense. Checked: no remaining constitution text requires a review to follow its commit; the promotion-coverage family keys all 107 table rows, with the re-keyed W7 row matched and 0 unmatched; shipped anchors resolve; markdownlint is clean; scripts/audit/run-all.sh exited 0 before the commit. That audit run is recorded as pre-commit evidence only, because the history-reading families must be re-run after the commit. **Examined 6 of 8.** The 6 are the changed files: the three constitution bullets read in full before and after, the AGENTS.md entry and the W7 row in full, the scenario in full, and spec.md's status line and tasks.md's task 30 as their diff. The other two scope entries, specs/inbox.md and 045's spec.md, are plan-affected paths this change does not touch, and were not read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: the new §grounding sentence, and the scenario, AGENTS.md and plan.md sentences that echo it, said a review's digest holds what the review read, where reviewed-digest covers only the spec's durable contracts — `framework/constitution.md` — **fixed**
- convention: the new scenario's Context stated the retired bullet's presence in the present tense, which is false once the change lands — `specs/050-constitution/scenarios/a-review-may-precede-its-commit.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
