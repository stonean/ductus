---
spec: 085-the-audit-record-gate-reads-records-through-the-runtime
last-run: 2026-11-26T16:46:44Z
analyzed-against: a19e6ff2a7171041f75757195af7040859b98bb3
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 3
analyzed-digest:
  data-model.md: 3b98edd3a200e50faf9108076bbcbd1f85328eada87d1013ed5fe763b304c3d1
  plan.md: 0ab50f1fd406885dc11c668c696c9e6aa06120743f35da48a7934648148a6b41
  review.md: 37e16ae3dcbf108bd370e872f1157461b139cf50dbd40ccf1f58560f0a6efd7a
  spec.md: d7a23e0379d76bf5810052707b228473c00d9a4f4672cccd0598ff44feab6303
  tasks.md: b0406e5825d651381da08a5da938d30712eb940de4842805b9082f9ed1a0ea2c
unexamined-by-reason:
  never-existed: 1
  root-absent: 1
  ships-to-adopter: 1
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
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#085 — The audit-record gate reads records through the runtime Data Model": 2aa976ce4e426e8d
  "data-model.md#Arguments": 8a36000d9fe8ebef
  "data-model.md#Baseline file": e1b76c6f7d3d6e93
  "data-model.md#CLI report and exit status": 0d30193e1985b5e2
  "data-model.md#Finding kinds": c69bbe049eca78d2
  "data-model.md#Result": 95aba80c27e08530
  "plan.md": e3b0c44298fc1c14
  "plan.md#085 — The audit-record gate reads records through the runtime Plan": 965a548016121e3e
  "plan.md#Affected Files": eb2124399c7dd135
  "plan.md#Cross-spec impact": 83e86f4254dc2cff
  "plan.md#Data Model": 504e96002b5143c2
  "plan.md#One corpus-wide primitive with no arguments": f45879f54487a3b0
  "plan.md#Overview": be35805b146ea2c4
  "plan.md#Records through the gate's loaders, with a probe for `blocking`": 6d0fe3921eb25306
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Tests: the primitive and the step, each finding both ways": 4ce69ba5e6d495e0
  "plan.md#The pin moves in the release that ships the primitive": f6f542a170328774
  "plan.md#The report and the exit status": c5e6bdf94b538618
  "plan.md#The runtime reads the baseline file": 76017aa9b65f46a0
  "plan.md#The template's step runs the primitive": 39d25883e847ad37
  "plan.md#The walk is `check-owed-work`'s corpus form": 281bdae91326d5cf
  "plan.md#Trade-offs": 44e97aa82d451ecc
  "plan.md#What a run costs, and what it retires (089, AC4)": 2c80a18872a1f21b
  "spec.md": 0b4c2952348ea9d6
  "spec.md#085 — The audit-record gate reads records through the runtime": 578550cac7539a6e
  "spec.md#Acceptance Criteria": 55faaa3634d853aa
  "spec.md#Applicable Rules": 082b62cdc15c500c
  "spec.md#Behavior": ef8724f7de28a420
  "spec.md#Cost": c7567db88a3c767e
  "spec.md#Edge Cases": 75a11ed5d786f130
  "spec.md#Motivation": 408c465414c1e8cc
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 31d75776917326b0
rule-verdicts:
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Records through the gate's loaders, with a probe for `blocking`"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#The runtime reads the baseline file"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#The template's step runs the primitive"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#The walk is `check-owed-work`'s corpus form"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#Records through the gate's loaders, with a probe for `blocking`"}
  BE-INPUT-011: {verdict: addressed, cite: "data-model.md#CLI report and exit status"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#Result"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#The report and the exit status"}
  QUAL-DELEG-001: {verdict: addressed, cite: "plan.md#Trade-offs"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Tests: the primitive and the step, each finding both ways"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#The pin moves in the release that ships the primitive"}
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "decision-reasons — Resolved Questions, 'A new spec, built for the next release': the reason against a task on 057 or 066 is the reopen and held close it would cause, not the result that option would deliver; unchanged."
    outcome: waived
    reason: stating the result a task on 057 or 066 would have delivered re-decides the operator's adoption of 2026-11-24, which the rule does not make; the build it chose is done, and 085's own criteria hold the reader's properties either home would have held
    decided-at: 2026-11-26T16:46:44Z
    decided-by: rule
  - key: "orphaned-references — Orphaned reference: README.md:187 names .ductus/docs/, which does not exist"
    outcome: waived
    reason: README.md names `.ductus/docs/` as where spec 087 moves a project file's parts, a directory a project holds only once it moves one; the text, from 087's own commit, and the check's reading of it are spec 087's, in progress in another lane
    decided-at: 2026-11-26T16:46:44Z
    decided-by: rule
---

# Analysis — 085-the-audit-record-gate-reads-records-through-the-runtime

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 3 unexamined target(s). Dispositions: 4 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- decision-reasons — Resolved Questions, 'A new spec, built for the next release': the reason against a task on 057 or 066 is the reopen and held close it would cause, not the result that option would deliver; unchanged. — `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md` — **waived**: stating the result a task on 057 or 066 would have delivered re-decides the operator's adoption of 2026-11-24, which the rule does not make; the build it chose is done, and 085's own criteria hold the reader's properties either home would have held (decided by the rule)
- orphaned-references — Orphaned reference: README.md:187 names .ductus/docs/, which does not exist — `README.md` — **waived**: README.md names `.ductus/docs/` as where spec 087 moves a project file's parts, a directory a project holds only once it moves one; the text, from 087's own commit, and the check's reading of it are spec 087's, in progress in another lane (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- root-absent: 1
- ships-to-adopter: 1

## Fixed in this run

- grounding — Technical Decisions, 'Records through the gate's loaders': 'YAML's null spellings (null, Null, NULL, ~, an empty value) deserialize to None' states what the YAML library does as fact with no citation, test or assumption marker. — `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, 'The walk is check-owed-work's corpus form': 'list_feature_dirs returns an empty list for a root read_dir cannot open' states current code behavior as fact; it names the symbol but cites no file or test. — `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md` — **fixed** (decided by the rule)
- grounding — Cost, 'What it adds': 'Claude Code lists only its name in each session's deferred tools' is a claim about an external host stated as fact, with no source cited. — `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, 'Where the reader is narrower': 'In an adopter that is specs/templates/, whose spec.md is a template' states what an adopter's spec root holds as fact with no cited source (the shipped template or a command naming that directory). — `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md` — **fixed** (decided by the rule)
