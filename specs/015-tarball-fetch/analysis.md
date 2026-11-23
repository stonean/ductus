---
spec: 015-tarball-fetch
last-run: 2026-11-23T21:47:13Z
analyzed-against: 1184151dd231c2a552d5a8615b68bb930227afea
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 4
analyzed-digest:
  plan.md: 227b95734ce5bb6a10473edc433deb7e6ed7e114c184a484b58d6a03af4358df
  review.md: f858eb674807424bede7317b5d7311e785d0aaf756dfca8a5dc835ca1e7acdaa
  spec.md: 2b3ef73a0724f3f803782d5e91fa5d12989b773d0af1eeac55d0c65589d6d22e
  tasks.md: a71ba8b6dda7ecd9636e7fe23c33ddae47541b773816195fa9104cc196787a0c
unexamined-by-reason:
  root-absent: 4
rule-digest:
  accessibility-frontend.md: 20fb956e35bb749f061090adf631329bafe3e37662a4e47b0076b697784bbf50
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: c9d90cf6bfa6e4f5492e6d1a49e073fbee467ba08c7788bf6c7224ae8bd5d411
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 9a200916d11a5597a6860d1538840bf7d9aa573bddce953a0bef604379d1451b
section-digest:
  "plan.md": e284325a920dcc5b
  "plan.md#015 — Tarball Fetch Plan": 5fc7c73eadbf7fb9
  "plan.md#Affected Files": cd3661faf1d05482
  "plan.md#Agent registry edits": 102ef74b0b2c106d
  "plan.md#Configure surface stays unchanged": 3fdcc302edd6223f
  "plan.md#Edge Cases reconciliation": cb5417738e98a187
  "plan.md#Fetch and extract sequencing": 495a7d715207e43a
  "plan.md#Manifest source resolution": 8ca23e52adb94f90
  "plan.md#Open Questions Resolved": dd385d5c97193a55
  "plan.md#Overview": fcec8152ae6b2fc1
  "plan.md#Per-language gitignore fetches stay separate": 4dec29d612182c4b
  "plan.md#Post-Write Integrity Check wording": 249d5675cf272e79
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The archive carries only what a run reads": af6c8b5b6fee6eb4
  "plan.md#Trade-offs": b3f7e831d1c6a0f7
  "spec.md": bb76dc9a06906bc7
  "spec.md#015 — Tarball Fetch": e32291e484449df4
  "spec.md#Acceptance Criteria": 346d85707d5cf1ce
  "spec.md#Behavior": 29ab98cdbf8ff7c1
  "spec.md#Cleanup": 33fbab1635ebdd0d
  "spec.md#Extract": 235830242b48f5b6
  "spec.md#One-time cleanup of stale per-run entries": a9ec3564f78bf009
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Per-file resolution": caf910781d00bdb8
  "spec.md#Permission bootstrap": 262732250bb4f426
  "spec.md#Problem": c53d38901122b5cf
  "spec.md#References": 18e51cf7cb66a56f
  "spec.md#Resolved Questions": 1b6448e4ef570cde
  "spec.md#Self-update notice": bd1cc545fc04abfb
  "spec.md#Source": 98fcff3ca2f3ea35
  "spec.md#Tradeoffs": 8d70f63d3e779106
  "spec.md#Why both leading-slash forms": 1d4d4c86ab6b8c3a
  "spec.md#Why pre-allow the temp-path Read globs": 505003187988d1a5
rule-verdicts:
  BE-DATA-001: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Fetch and extract sequencing"}
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: grounding — Problem states per-file counts as fact about the pre-015 flow (~14 shared files, ~4 project files, ~35 fetches for one agent, ~50+ for two) with no citation to the manifest or a command.
    outcome: waived
    reason: historical figures for the per-file installer this spec replaced, stated in the past tense; git history holds that installer, and no criterion or current behavior rests on them
    decided-at: 2026-11-23T21:47:13Z
    decided-by: rule
  - key: grounding — Source asserts that GitHub names the archive's top-level directory after the ref (ductus-main/, ductus-ductus-v0.55.0/) with no cited source or command.
    outcome: waived
    reason: "the run never relies on the naming: Extract step 2 reads the framework root from the extraction, so the names only illustrate it (the scratch-adopter runs of 2026-11-23 extracted main to ductus-main/)"
    decided-at: 2026-11-23T21:47:13Z
    decided-by: rule
  - key: grounding — Permission bootstrap, Why both leading-slash forms, states Claude Code's matcher behavior as empirical fact with no reproducible source; the signpost cites the docs but only to overtake it.
    outcome: waived
    reason: the record of the observation the decision was taken on; the signpost beneath it quotes the documentation that overtakes it and records the seed's resulting change
    decided-at: 2026-11-23T21:47:13Z
    decided-by: rule
  - key: grounding — Fetch and extract sequencing asserts that mktemp -d -t lands under $TMPDIR on macOS and /tmp on Linux, uncited.
    outcome: waived
    reason: the run reads the directory mktemp prints and never predicts it, so where it lands decides nothing in the design
    decided-at: 2026-11-23T21:47:13Z
    decided-by: rule
---

# Analysis — 015-tarball-fetch

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 4 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Problem states per-file counts as fact about the pre-015 flow (~14 shared files, ~4 project files, ~35 fetches for one agent, ~50+ for two) with no citation to the manifest or a command. — `specs/015-tarball-fetch/spec.md` — **waived**: historical figures for the per-file installer this spec replaced, stated in the past tense; git history holds that installer, and no criterion or current behavior rests on them (decided by the rule)
- grounding — Source asserts that GitHub names the archive's top-level directory after the ref (ductus-main/, ductus-ductus-v0.55.0/) with no cited source or command. — `specs/015-tarball-fetch/spec.md` — **waived**: the run never relies on the naming: Extract step 2 reads the framework root from the extraction, so the names only illustrate it (the scratch-adopter runs of 2026-11-23 extracted main to ductus-main/) (decided by the rule)
- grounding — Permission bootstrap, Why both leading-slash forms, states Claude Code's matcher behavior as empirical fact with no reproducible source; the signpost cites the docs but only to overtake it. — `specs/015-tarball-fetch/spec.md` — **waived**: the record of the observation the decision was taken on; the signpost beneath it quotes the documentation that overtakes it and records the seed's resulting change (decided by the rule)
- grounding — Fetch and extract sequencing asserts that mktemp -d -t lands under $TMPDIR on macOS and /tmp on Linux, uncited. — `specs/015-tarball-fetch/plan.md` — **waived**: the run reads the directory mktemp prints and never predicts it, so where it lands decides nothing in the design (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 4

## Fixed in this run

- grounding — Cleanup asserts that macOS and Linux (systemd-tmpfiles) sweep their temp directories automatically, uncited and not marked as an assumption. — `specs/015-tarball-fetch/spec.md` — **fixed** (decided by the rule)
- grounding — The archive carries only what a run reads asserts that GitHub builds the codeload tarball with git archive and honors export-ignore from the archived commit's .gitattributes, with no link or command substantiating it. — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- grounding — Configure surface stays unchanged asserts that tar and mktemp are used by no other slash command, with no search or citation. — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
