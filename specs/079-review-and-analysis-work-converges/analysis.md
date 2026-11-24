---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-24T01:09:43Z
analyzed-against: f14e0cbd246c3e93557923feff53ece38c420d9a
hard-fail: 0
blocking-findings: 0
advisory: 7
unexamined: 5
analyzed-digest:
  data-model.md: 2c8d2c8174acb033015d094acf4db88c5c05efa068077ef11d03a1405433d1c2
  plan.md: a4c4020f984ed90bbb57e03e43c36d3b9f1eef2605302e879975a230bca1ea9c
  research.md: 0b2684cba2edfbdf8778e2367ed7811a5aa7be89839395f897f42f28d6dfb993
  review.md: fb6453b63a0dca65f1dfda56365e23a93a57961d095bf0e1ff24b68caa123d5e
  scenarios/the-excluded-section-lists-paths-only.md: 1763a8c3a9dd478a6f39d2813a73d4804c21dc8e9581c401af3fdaf35e6ea8b1
  spec.md: 4d89b951b10389343a95844f5ea08a1146e45c400395c4320e2d9f31fcd6804f
  tasks.md: bce783b0e2bbdaa06e290639767f21f41b4b01bea3e7e0324087749d15a3e6de
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  never-existed: 2
  ships-to-adopter: 2
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
  "data-model.md#079 — Review and analysis work converges Data Model": df3373784178481d
  "data-model.md#Constants": f37c400acb2b92f5
  "data-model.md#`brief-re-read` result and `round.toml`": ea609bb20766adaf
  "data-model.md#`check-review-gate` result": 7f6a708ee2a6c60c
  "data-model.md#`classify-spec-edit` (new primitive)": 50af62ec3d4cfdc1
  "data-model.md#`compute-review-scope` result": 7161ab11fb57407b
  "data-model.md#`diff-cross-spec` result": f5a011cdf0f1d685
  "data-model.md#`measure-reopenings` (new primitive)": af14012fb60274c9
  "data-model.md#`review.md` frontmatter": 63895d2f8520a6bb
  "data-model.md#`write-review` arguments": 2a258012bcea9512
  "plan.md": e3b0c44298fc1c14
  "plan.md#079 — Review and analysis work converges Plan": 3e80813376f513f8
  "plan.md#A run waits for the work": 0c3dc74561220aea
  "plan.md#Affected Files": 8aa851eecef098fa
  "plan.md#Cross-spec impact": e42e9f392be561f0
  "plan.md#Data Model": 40ce1846aee13dcc
  "plan.md#Dispositions": c6a2975ce785e126
  "plan.md#Fix rounds": c11e610c9207075c
  "plan.md#One commit-attribution helper": 2f5dd511c27018dc
  "plan.md#One description per primitive": 53540b37939dfdb2
  "plan.md#Open Questions Resolved": 943a9d6191d629c0
  "plan.md#Overview": f00c3e7fdb316af7
  "plan.md#Prose cuts come last": 0a3ba0aeb3df320f
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The change is breaking": bc0d8b2b3ff1587c
  "plan.md#The constitution": 69aae4b6425c2723
  "plan.md#The gate reads what a review owes": a1ef714684072d43
  "plan.md#The measurement": 5f11de2d3b504396
  "plan.md#The open mark": 749fcf6569d30083
  "plan.md#The scope": 636ce8fcaa935c01
  "plan.md#The window": 0676fba69ec69e19
  "plan.md#The workflow-step runner": c7345624328d0309
  "plan.md#The writers this spec changes are guarded where they are owned": 4ad9db3c2e1d81b4
  "plan.md#Trade-offs": 611fcc8441fe0424
  "plan.md#What reopens": ff858a10c62b69af
  "plan.md#`AGENTS.md`": 0953ae4fedcce8ff
  "plan.md#`diff-cross-spec`": ba5029839bda78a5
  "scenarios/the-excluded-section-lists-paths-only.md": 50d3794a575f214e
  "scenarios/the-excluded-section-lists-paths-only.md#Behavior": 49fdc5f0964a61c6
  "scenarios/the-excluded-section-lists-paths-only.md#Context": 0237c1f49780b845
  "scenarios/the-excluded-section-lists-paths-only.md#Edge Cases": 8206bcffe2cfcc32
  "scenarios/the-excluded-section-lists-paths-only.md#Open Questions": 9fc5979d590cdb97
  "scenarios/the-excluded-section-lists-paths-only.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/the-excluded-section-lists-paths-only.md#The-excluded-section-lists-paths-only": e9dfd6102a162346
  "spec.md": 83b2ff8037b4008f
  "spec.md#079 — Review and analysis work converges": 968644a8e412d2a5
  "spec.md#Acceptance Criteria": 80beeb001d8a9c6f
  "spec.md#Applicable Rules": edc80fdad54b842a
  "spec.md#Behavior": 55145a9018734d1c
  "spec.md#Edge Cases": 2f5971c6c1453633
  "spec.md#Motivation": 81069fbaa0db88c6
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 3cf35ef33cdec278
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#The change is breaking"}
  BE-INPUT-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`compute-review-scope` result"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#The writers this spec changes are guarded where they are owned"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#The writers this spec changes are guarded where they are owned"}
  CFG-CONST-001: {verdict: addressed, cite: "data-model.md#Constants"}
  CFG-CONST-002: {verdict: addressed, cite: "data-model.md#Constants"}
  CFG-CONST-003: {verdict: addressed, cite: "data-model.md#Constants"}
  QUAL-DELEG-001: {verdict: addressed, cite: "plan.md#One commit-attribution helper"}
  QUAL-TEST-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
  - key: artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 102154 bytes, 3 read pages at the 50000-byte read size
    outcome: waived
    reason: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing.
    decided-at: 2026-11-24T01:09:43Z
    decided-by: rule
  - key: "grounding — Resolved Questions, 'Does the fix-round loop earn its cost?': 'Commit messages report at least three more re-read MUSTs that could not be paired, on 076, 077 and 022' names no commit, so the claim cannot be re-derived."
    outcome: waived
    reason: "advisory and no chore: the sentence before it states the count's method, commit messages since 2026-10-01 at `1268b9a9`, and these three are the re-reads of that population whose counts could not be paired with a round, so the same count re-derives them"
    decided-at: 2026-11-24T01:09:43Z
    decided-by: rule
  - key: "grounding — Resolved Questions, 'Where does a re-review's window start by default?': 'an adopter on 0.64.0 reported two reviews of one reopening that each read the whole window, 59 scope entries over 68 commits, ... about 2.1M subagent tokens' is an external report stated as fact with no link to an artifact holding it."
    outcome: waived
    reason: "advisory and no chore: the source is an adopter's report, which no artifact of this repository can hold, since an adopter's tree is out of reach and its name is never recorded here (AGENTS.md's Workflow rules); the entry attributes the figures to that report"
    decided-at: 2026-11-24T01:09:43Z
    decided-by: rule
  - key: grounding — Motivation, the four costs the 066 session reported (shared-checkout commit procedure, clippy too_many_lines appearing only after a full hook run on task 14) are descriptive claims about a session's experience with no citation to a record of it.
    outcome: waived
    reason: "advisory and no chore: the Motivation attributes each cost to the session working 066 that reported it, a report no committed artifact holds, and cites the commits and CI run IDs where one exists (`d62e72f3`, `69a42912`, `08f0b199`, runs 41856373009, 41862325548 and 41897995093)"
    decided-at: 2026-11-24T01:09:43Z
    decided-by: rule
  - key: "decision-reasons — Resolved Questions, 'Does a record's commit leave code committed after the run unread?': 'adding what lies between to the next window' is rejected because a review preceding its commit would owe a second review for code it read, 1 of 143 and over the bound; the reason is the review run it would start, and the result given up (code in that gap read) is not stated as the cost of keeping the chosen rule beyond the accepted gap."
    outcome: waived
    reason: "advisory and no chore: the entry states what the choice gives up, an adopter committing code in that gap reaching `done` with it unread and nothing reporting it, beside the measured case the rejected option would serve, which none of the 136 measurable runs since 2026-10-01 showed"
    decided-at: 2026-11-24T01:09:43Z
    decided-by: rule
  - key: "decision-reasons — Resolved Questions, 'Is this work a new spec, or 078 renamed and broadened?': renaming 078 is rejected for 'a sweep of 13 files and 46 lines, 078 back to draft and re-planned, and its done waiting on this whole design', which is the reopen and re-planning it would start; the result it would have delivered is not stated."
    outcome: waived
    reason: "advisory and no chore: three correctness reasons decided it, the constitution sending a concern shared across siblings to its own spec, AGENTS.md's routing rule needing each owner to back-link to a requiring spec, and 078's name and criteria describing the loop it delivered; the rename's cost is listed beside them, which framework/reference/analyze.md §Decision reasons does not flag"
    decided-at: 2026-11-24T01:09:43Z
    decided-by: rule
  - key: "decision-reasons — Resolved Questions, 'Does an entry a review left unread come back into its spec's window?': 'an entry owing a review, as open-findings do' is rejected because 'each of those 34 specs would then owe a review whatever changed'; the review runs it would start is the reason, with no result of that option named as given up."
    outcome: waived
    reason: "advisory and no chore: the reason is AC1's bound as well as its cost, since the entry records that most records leave a generated mirror unread (82 of the 269 entries), so an entry owing a review would have a review owe another past the one AC1 allows; what the choice gives up is stated in AC30 itself, an unread entry joining only the scope of a review that runs"
    decided-at: 2026-11-24T01:09:43Z
    decided-by: rule
---

# Analysis — 079-review-and-analysis-work-converges

## Summary

0 hard-fail, 0 blocking, 7 advisory; not blocking. 5 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 7 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 102154 bytes, 3 read pages at the 50000-byte read size — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing. (decided by the rule)
- grounding — Resolved Questions, 'Does the fix-round loop earn its cost?': 'Commit messages report at least three more re-read MUSTs that could not be paired, on 076, 077 and 022' names no commit, so the claim cannot be re-derived. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: advisory and no chore: the sentence before it states the count's method, commit messages since 2026-10-01 at `1268b9a9`, and these three are the re-reads of that population whose counts could not be paired with a round, so the same count re-derives them (decided by the rule)
- grounding — Resolved Questions, 'Where does a re-review's window start by default?': 'an adopter on 0.64.0 reported two reviews of one reopening that each read the whole window, 59 scope entries over 68 commits, ... about 2.1M subagent tokens' is an external report stated as fact with no link to an artifact holding it. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: advisory and no chore: the source is an adopter's report, which no artifact of this repository can hold, since an adopter's tree is out of reach and its name is never recorded here (AGENTS.md's Workflow rules); the entry attributes the figures to that report (decided by the rule)
- grounding — Motivation, the four costs the 066 session reported (shared-checkout commit procedure, clippy too_many_lines appearing only after a full hook run on task 14) are descriptive claims about a session's experience with no citation to a record of it. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: advisory and no chore: the Motivation attributes each cost to the session working 066 that reported it, a report no committed artifact holds, and cites the commits and CI run IDs where one exists (`d62e72f3`, `69a42912`, `08f0b199`, runs 41856373009, 41862325548 and 41897995093) (decided by the rule)
- decision-reasons — Resolved Questions, 'Does a record's commit leave code committed after the run unread?': 'adding what lies between to the next window' is rejected because a review preceding its commit would owe a second review for code it read, 1 of 143 and over the bound; the reason is the review run it would start, and the result given up (code in that gap read) is not stated as the cost of keeping the chosen rule beyond the accepted gap. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: advisory and no chore: the entry states what the choice gives up, an adopter committing code in that gap reaching `done` with it unread and nothing reporting it, beside the measured case the rejected option would serve, which none of the 136 measurable runs since 2026-10-01 showed (decided by the rule)
- decision-reasons — Resolved Questions, 'Is this work a new spec, or 078 renamed and broadened?': renaming 078 is rejected for 'a sweep of 13 files and 46 lines, 078 back to draft and re-planned, and its done waiting on this whole design', which is the reopen and re-planning it would start; the result it would have delivered is not stated. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: advisory and no chore: three correctness reasons decided it, the constitution sending a concern shared across siblings to its own spec, AGENTS.md's routing rule needing each owner to back-link to a requiring spec, and 078's name and criteria describing the loop it delivered; the rename's cost is listed beside them, which framework/reference/analyze.md §Decision reasons does not flag (decided by the rule)
- decision-reasons — Resolved Questions, 'Does an entry a review left unread come back into its spec's window?': 'an entry owing a review, as open-findings do' is rejected because 'each of those 34 specs would then owe a review whatever changed'; the review runs it would start is the reason, with no result of that option named as given up. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: advisory and no chore: the reason is AC1's bound as well as its cost, since the entry records that most records leave a generated mirror unread (82 of the 269 entries), so an entry owing a review would have a review owe another past the one AC1 allows; what the choice gives up is stated in AC30 itself, an unread entry joining only the scope of a review that runs (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- never-existed: 2
- ships-to-adopter: 2

## Fixed in this run

- grounding — Resolved Questions, 'What may one review run cost?': 'The draft that would have carried it, 074-delegated-gates-leave-proposals-for-review, was retired unbuilt and its need waived' is stated as fact about repository history with no commit, path or command cited. — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- grounding — Overview: '`specs/events.md` and `specs/errors.md` do not exist in this repository' is an absence claim about the tree stated without a command or listing. — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
