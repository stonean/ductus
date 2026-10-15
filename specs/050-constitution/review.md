---
spec: 050-constitution
scenario: a-project-command-is-run-not-reproduced
last-run: 2026-10-15T00:21:57Z
reviewed-against: 8ad033b87a3f2c862c9615a01d6b00e8189d6d8c
diff-base: 9f7c5a792bd0c27660a66296244b9375997ef68a
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
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
  scenarios/a-status-commit-holds-only-the-transition.md: 0d3fb1a701bc4b3112cfc2e54bb488df060c75fceeb0dc6c6b39205eb01d4438
  scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
  scenarios/findings-route-by-scope.md: a29ba00826b6109390c701ebd3eabdd8a1bd687f972f77339e57b629c6122226
  scenarios/governance-is-multi-source.md: ae59aca7a049317806297839a73cf335eeb3764db97999a7ddc0b12103e6ffeb
  scenarios/knowledge-routes-by-population-not-by-kind.md: 0e934ea65cbaf2bf31acc4f45c461bfb868be45deb5aa9fa6601c59b8c003c5c
  scenarios/report-outcomes-not-edits.md: caf13342d7fe0a5af4108024cf424ab921030c896f0743307cf5111be2e163ab
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 050-constitution

## Summary

050's reopen for scenario a-project-command-is-run-not-reproduced, over the window since 050 re-entered in-progress (diff-base 9f7c5a79, one commit, 8ad033b8): 0 MUST, 0 SHOULD, 0 low-confidence, no observations; not blocking. Examined 4 of 8 in scope: the four files the window changes, framework/constitution.md's one added §grounding bullet with its neighbours, the new scenario in full, and spec.md's and tasks.md's hunks (the status reopen and task 29). The other four are plan-affected files the window does not touch: AGENTS.md and plan.md were searched for an entry or table row that would now duplicate the rule (none; the nearest, AGENTS.md's local-gate entry, names the same npx markdownlint-cli2 invocation framework-checks.yml runs, so it is a declared command rather than a substitute), and 045's spec.md and specs/inbox.md were not read. The selected rule files govern code, and the window carries none, so the five passes applied no rule; what they checked was the prose. The bullet sits after the connected-database bullet as a bullet, so AC10's anchor set is unchanged, and its disposition clause matches §design-principles' three dispositions. Each factual claim in the scenario resolves to its source: §grounding source 2's "an actual test run" (constitution.md:67), the constitution's update strategy and .ductus/ destination (ductus.md:771), config.toml's managed host block (ductus.md:673), the gitignored bin/ pointer (ductus.md:426), and 055's never-fetch, not-checked-out degradation (055 spec.md:36, :83, :88).

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
