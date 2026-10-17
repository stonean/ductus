---
spec: 050-constitution
last-run: 2026-10-17T12:36:40Z
analyzed-against: 10e93d223a05039a05265ac50c41cbceed3a4c9f
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 4
analyzed-digest:
  plan.md: 7918f73197dd0c6d9ed94ca3f0262870d81f1ab08334719a67cd078e7819f4c9
  review.md: 9ff53e0a84a5cbf0ebab650b871e43596ea658d73afe49229249802ec86b2046
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
  spec.md: 79c6b9b0c83748023101d6246dd941fe2bffa6a191fbbcdb645a77fae3b0de68
  tasks.md: d238786bcb795bfdabbde58b13e22a574c64d476e6f489150abff506b204d59c
unexamined-by-reason:
  root-absent: 1
  ships-to-adopter: 3
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: plan-record — plan.md §Classification is outside the design record
    outcome: discarded
    reason: the section is 050's AC1 verdict table, one promotion verdict per AGENTS.md rule-bearing entry, which the promotion-coverage audit reads; it is deliberately part of this plan's design, the case analyze.md names a Classification for
    decided-at: 2026-10-01T21:11:05Z
    decided-by: andy@stone.dev
---

# Analysis — 050-constitution

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 4 unexamined target(s). Dispositions: 0 fixed, 0 routed, 1 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- plan-record — plan.md §Classification is outside the design record — `specs/050-constitution/plan.md` — **discarded**: the section is 050's AC1 verdict table, one promotion verdict per AGENTS.md rule-bearing entry, which the promotion-coverage audit reads; it is deliberately part of this plan's design, the case analyze.md names a Classification for

## Unexamined targets

- root-absent: 1
- ships-to-adopter: 3

## Fixed in this run

*None.*
