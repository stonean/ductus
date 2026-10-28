---
spec: 050-constitution
last-run: 2026-10-28T18:52:10Z
analyzed-against: d1853444f01e2623b1b4bc7b92629fed3dde2825
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 5
analyzed-digest:
  plan.md: b479ada101fb4ab4eb0e644b4004cde6516a37ce4c9e3be824caa362c9ac557f
  review.md: 637e854aef9d2a3a5e1427eabe427152f9756b7d1ab84aaeef884263eca72b43
  scenarios/a-canonical-source-is-pointed-at-not-copied.md: c1458fa54c4d7c5d25161f0d858079055be89e82389830938fef4e36281eb456
  scenarios/a-declared-cross-spec-impact-gates-done.md: b4a96bd919b383288cc871adf160804acfc5bc127b1798963810ff391a3ac078
  scenarios/a-measurement-states-its-method-and-units.md: 4c250a480c11e9788058f1012b647daa14aba2036d4a3baa9d0c1ba3c5965b2a
  scenarios/a-partial-read-is-not-a-read.md: 078952ab7a67edeb949dd30d5f1bc0e0ef91904c467980f1e5547aab145e82f0
  scenarios/a-project-command-is-run-not-reproduced.md: 82eb4ccfa37346e02b05e32e802f5ec528a633801407a32e43ff678921329ce6
  scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
  scenarios/a-retired-filename-leaves-a-decision-record.md: f84a3177744eb03a859d82a4017495c24bcf24466448b7c49ad1a253147ab581
  scenarios/a-review-may-precede-its-commit.md: add57e1653b813a3832e3cae908f48b7a37d78f3cc8c1703a911ec6ad90af420
  scenarios/a-status-commit-holds-only-the-transition.md: 2f164ce212508e7f3b82508543157b868e8533fd15f977497d2ab569ec89f79e
  scenarios/an-answer-is-applied-as-chosen.md: ddccff2f13211a718842fd0954bc8e2e446eb99ad8fe6c60ebc9f87f2704f5fc
  scenarios/artifacts-are-as-short-as-their-required-content.md: 36325215c271aa3d621586e98236815e8966ef9dcf315035c2248351ef6fb31a
  scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
  scenarios/decisions-go-to-the-owner-one-at-a-time.md: 91dd288f71c59a081c23f8b380218cc89924859727e52885a88cf491c5465060
  scenarios/findings-route-by-scope.md: fd7a9d731637cf4530de35b37f09c0720250c5d962516133f9f33a05c4e36bb0
  scenarios/governance-is-multi-source.md: bccd202e87fadb1423ee07cb81233bb28de2b4e7465946411a90bcf3cd53e907
  scenarios/knowledge-routes-by-population-not-by-kind.md: f6b0e95b914cb35047a97cbc569d911538dfafdb8fb35f1b0baec6434ad9445a
  scenarios/memory-routing-lives-in-agents-md.md: d631351034c765f258bfcdd2bbe335614d9c274b3cc0f761c3be568a66808c21
  scenarios/report-outcomes-not-edits.md: caf13342d7fe0a5af4108024cf424ab921030c896f0743307cf5111be2e163ab
  scenarios/reports-end-with-the-remaining-work.md: b642a2d08b2227e856aefba9fa3ec8cd1efae0eb2a14cc674dfcd455982dbec3
  scenarios/the-constitution-fits-one-read.md: bf9ef08a810e983be198ade81efa95e9d4bc266e53bf6ab7f3be659a8643e75a
  scenarios/the-constitution-states-rules-not-stories.md: 691e5ea78b7012fd9f9a8114f20632d7852d5c319f54e3951b2ebaed65c013bc
  spec.md: a9c4fdfd9d48937e67329dc4f7bfb8f432f971c3879bde23dba20d2c830946ce
  tasks.md: 9523f31bab3b25deb61923c57cb7f3c954055771fa28ad1ed4b3b1f2da82e4eb
unexamined-by-reason:
  root-absent: 3
  ships-to-adopter: 2
blocking: false
dispositions:
  fixed: 3
  routed: 0
  discarded: 2
  undispositioned: 0
decisions:
  - key: plan-record — plan.md §Classification is outside the design record
    outcome: discarded
    reason: the section is 050's AC1 verdict table, one promotion verdict per AGENTS.md rule-bearing entry, which the promotion-coverage audit reads; it is deliberately part of this plan's design, the case analyze.md names a Classification for
    decided-at: 2026-10-01T21:11:05Z
    decided-by: andy@stone.dev
  - key: artifact-size — `specs/050-constitution/plan.md` is 60525 bytes, 2 read pages at the 50000-byte read size
    outcome: discarded
    reason: the excess is the §Classification table, AC1's verdict record that the promotion-coverage audit reads, and the per-section byte measurement the-constitution-fits-one-read requires
    decided-at: 2026-10-28T18:52:10Z
    decided-by: andy@stone.dev
---

# Analysis — 050-constitution

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 5 unexamined target(s). Dispositions: 3 fixed, 0 routed, 2 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- plan-record — plan.md §Classification is outside the design record — `specs/050-constitution/plan.md` — **discarded**: the section is 050's AC1 verdict table, one promotion verdict per AGENTS.md rule-bearing entry, which the promotion-coverage audit reads; it is deliberately part of this plan's design, the case analyze.md names a Classification for
- artifact-size — `specs/050-constitution/plan.md` is 60525 bytes, 2 read pages at the 50000-byte read size — `specs/050-constitution/plan.md` — **discarded**: the excess is the §Classification table, AC1's verdict record that the promotion-coverage audit reads, and the per-section byte measurement the-constitution-fits-one-read requires

## Unexamined targets

- root-absent: 3
- ships-to-adopter: 2

## Fixed in this run

- grounding — spec.md gives 56 entries and a split summing to 46 with no method, beside the measured 54 — `specs/050-constitution/spec.md` — **fixed**
- grounding — plan.md says the constitution carries 40-odd anchors, uncited, beside the plan's own 31 — `specs/050-constitution/plan.md` — **fixed**
- grounding — plan.md says nothing quoted the removed phrases, naming no search — `specs/050-constitution/plan.md` — **fixed**
