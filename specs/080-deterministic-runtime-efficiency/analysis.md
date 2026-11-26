---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-26T01:21:32Z
analyzed-against: 319bf0778489f42cdc9dbdac35012e2eaa82407a
hard-fail: 0
blocking-findings: 0
advisory: 6
unexamined: 8
analyzed-digest:
  data-model.md: e3acb6f5f486c0ce215e0302facfe5d4b7679860df4f54678c4e304c0a6b8b35
  plan.md: cbb0c5380ffee72aad3d5c0d37ff18e37109f346d0ca02627f937121fe1955b0
  research.md: 8cb129845003758f3a835b5701b0ac0f4beaeb5cb4addec9488fccfe2553e56b
  review.md: 872d5cfbfbaabdb7ef6f9a966bf37e19b29b6803702be4162e7a67c1df705eff
  scenarios/a-step-gates-on-a-bounded-list-by-its-count.md: bdf77659a5e3c0b7dac784691889a9a0160fbe5859a072adb33f7a4134217e83
  scenarios/configure-types-no-permission-entries.md: 6c764c4d44c20f0b7d88582dbd52de2d9c1efd915080813f41eb858e16ca7313
  scenarios/groom-derives-routing-candidates.md: 4eb0d78c838663e44391d9ff28ec9f88f815673edffbfa6e3c2e9d86e865eb09
  scenarios/help-answers-with-no-model-turn.md: 8962bafa8cbf7224dc43d5d73a0507d3bd1088492c5977c287eac52acff33189
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
  spec.md: 657ac727b5e674ed234982ee66ee0ac04e43e882d5205a819e903791a5990455
  tasks.md: 74b690526b9bf38c7a0519ea98553a49527360ebc6032c6ba6185a827a3034d2
unexamined-by-reason:
  anchor-target-unjudgeable: 2
  never-existed: 1
  root-absent: 4
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
  "data-model.md#080 — Deterministic runtime efficiency Data Model": 8dde779474480c0c
  "data-model.md#Bounded results": 20767fae21827e2b
  "data-model.md#The hook entries in `{cli-config-dir}/settings.local.json`": 1399b298d1f614d0
  "data-model.md#The pipeline view": e5399a910b4c6106
  "data-model.md#`ManifestEntry.tool-search-line`": 48fff4f84c72cff0
  "data-model.md#`_meta` on `tools/list`": de505ec03f02fbef
  "data-model.md#`discover-rule-files`' rules": 7aafca770e5ebb75
  "data-model.md#`ductus hook help`": 17046739e94612f6
  "data-model.md#`ductus hook status`": add4ad566fd25876
  "data-model.md#`merge-permissions`' canonical call": ebee29fb76c3dfe3
  "data-model.md#`merge-status-hook`": dd1571cfc8830f42
  "data-model.md#`read-section`": eba5ec5076ff3ed8
  "data-model.md#`render-help`": afad6e4c9a8e6675
  "data-model.md#`render-status`": c44664780041fd3d
  "plan.md": e3b0c44298fc1c14
  "plan.md#080 — Deterministic runtime efficiency Plan": 2bde2027a7b75184
  "plan.md#A `claude -p` harness records each command's cost": 5b9eb7e591ed8c4e
  "plan.md#A generated corpus and a size script measure before anything is bounded": 22909a5e8f246e02
  "plan.md#Affected Files": aebe03db5d3d13bd
  "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive": 828eae7c4407fa04
  "plan.md#Configure's canonical sets come from the runtime": 4aa3d4e60b1cb38a
  "plan.md#Corpus-growing results are bounded through `bounded_lists`": 2ee0e96ef86b5ade
  "plan.md#Cross-spec impact": 151108c84840d634
  "plan.md#Data Model": 2758f2864e4a3ab9
  "plan.md#Help answers through the status surfaces": 35be8db3ead2b1da
  "plan.md#One view model, two renderings, one bound": 3428665bbd12b966
  "plan.md#Open Questions Resolved": 34397479ccfa5a18
  "plan.md#Overview": fa4d590cc53ee1a4
  "plan.md#Pi registers the command in the bridge": 1a6f6faf407702b5
  "plan.md#Reference sections are read through `read-section`": 7bb145a4bd2036e2
  "plan.md#Rules reach the model through `discover-rule-files`": 89d814525276d25a
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Tool loading derives from each command's parsed steps": e691dcb124612229
  "plan.md#Trade-offs": 7cff0c7a0ceaabce
  "plan.md#`merge-status-hook` installs the hook and gates it on the version": ceba65cddc69f819
  "plan.md#`render-status` serves the view, and `dashboard` stops carrying it": b3103066337ea370
  "plan.md#groom derives its routing candidates": f36b5843c3fe57de
  "scenarios/a-step-gates-on-a-bounded-list-by-its-count.md": 50d3794a575f214e
  "scenarios/a-step-gates-on-a-bounded-list-by-its-count.md#A-step-gates-on-a-bounded-list-by-its-count": dba0722d6a7e097a
  "scenarios/a-step-gates-on-a-bounded-list-by-its-count.md#Behavior": e6b5ce6bcacfaa6b
  "scenarios/a-step-gates-on-a-bounded-list-by-its-count.md#Context": eed2571095ade271
  "scenarios/a-step-gates-on-a-bounded-list-by-its-count.md#Edge Cases": 3e6137307d0c93be
  "scenarios/a-step-gates-on-a-bounded-list-by-its-count.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-step-gates-on-a-bounded-list-by-its-count.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/configure-types-no-permission-entries.md": 50d3794a575f214e
  "scenarios/configure-types-no-permission-entries.md#Behavior": 12aeb76ad026899d
  "scenarios/configure-types-no-permission-entries.md#Configure-types-no-permission-entries": 985242536b392c63
  "scenarios/configure-types-no-permission-entries.md#Context": fa500f085a11b15d
  "scenarios/configure-types-no-permission-entries.md#Edge Cases": 94d87d4cddf2ed5a
  "scenarios/configure-types-no-permission-entries.md#Open Questions": 9fc5979d590cdb97
  "scenarios/configure-types-no-permission-entries.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/groom-derives-routing-candidates.md": 50d3794a575f214e
  "scenarios/groom-derives-routing-candidates.md#Behavior": f77924225b21332f
  "scenarios/groom-derives-routing-candidates.md#Context": 4cd2333ae6e6dba5
  "scenarios/groom-derives-routing-candidates.md#Edge Cases": 0d4ee6331561a158
  "scenarios/groom-derives-routing-candidates.md#Groom-derives-routing-candidates": b2a13b8f9fcf781f
  "scenarios/groom-derives-routing-candidates.md#Open Questions": 9fc5979d590cdb97
  "scenarios/groom-derives-routing-candidates.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/help-answers-with-no-model-turn.md": 50d3794a575f214e
  "scenarios/help-answers-with-no-model-turn.md#Behavior": 2d8a0841be5dfc01
  "scenarios/help-answers-with-no-model-turn.md#Context": 03f7df1375a6f65a
  "scenarios/help-answers-with-no-model-turn.md#Edge Cases": 97d08e6949ca3686
  "scenarios/help-answers-with-no-model-turn.md#Help-answers-with-no-model-turn": 0f614ca491bed7c1
  "scenarios/help-answers-with-no-model-turn.md#Open Questions": 9fc5979d590cdb97
  "scenarios/help-answers-with-no-model-turn.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/one-search-covers-referenced-sections.md": cb923bc6c520d885
  "scenarios/one-search-covers-referenced-sections.md#Behavior": eb4cdc89cb3771db
  "scenarios/one-search-covers-referenced-sections.md#Context": 0c52c47a6560da73
  "scenarios/one-search-covers-referenced-sections.md#Edge Cases": bdd0569eb5b64c4e
  "scenarios/one-search-covers-referenced-sections.md#One-search-covers-referenced-sections": 763e79c13b6bf910
  "scenarios/one-search-covers-referenced-sections.md#Open Questions": 9fc5979d590cdb97
  "scenarios/one-search-covers-referenced-sections.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/reference-reads-follow-the-cited-sections.md": 50d3794a575f214e
  "scenarios/reference-reads-follow-the-cited-sections.md#Behavior": 7f7beebd1d93133e
  "scenarios/reference-reads-follow-the-cited-sections.md#Context": 95d01ed3d85da4ab
  "scenarios/reference-reads-follow-the-cited-sections.md#Edge Cases": 2e396e1f0464085b
  "scenarios/reference-reads-follow-the-cited-sections.md#Open Questions": 9fc5979d590cdb97
  "scenarios/reference-reads-follow-the-cited-sections.md#Reference-reads-follow-the-cited-sections": 3f4664e2b21ee299
  "scenarios/reference-reads-follow-the-cited-sections.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/rules-load-without-shell-reads.md": 50d3794a575f214e
  "scenarios/rules-load-without-shell-reads.md#Behavior": c0314a10daca8ffa
  "scenarios/rules-load-without-shell-reads.md#Context": 3ab17e32e196e039
  "scenarios/rules-load-without-shell-reads.md#Edge Cases": 7d4f62c2b91d3876
  "scenarios/rules-load-without-shell-reads.md#Open Questions": 9fc5979d590cdb97
  "scenarios/rules-load-without-shell-reads.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/rules-load-without-shell-reads.md#Rules-load-without-shell-reads": 8d87b33640c35aac
  "scenarios/the-search-goes-out-with-the-first-call.md": cb923bc6c520d885
  "scenarios/the-search-goes-out-with-the-first-call.md#Behavior": 1bda8fc7f0f9d4d6
  "scenarios/the-search-goes-out-with-the-first-call.md#Context": c50f825b9386d65a
  "scenarios/the-search-goes-out-with-the-first-call.md#Edge Cases": e5765fcb05b522af
  "scenarios/the-search-goes-out-with-the-first-call.md#Open Questions": 9fc5979d590cdb97
  "scenarios/the-search-goes-out-with-the-first-call.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/the-search-goes-out-with-the-first-call.md#The-search-goes-out-with-the-first-call": 10eb5658eaecebec
  "spec.md": 68bf40ed3440dfba
  "spec.md#080 — Deterministic runtime efficiency": 67305eed78f0c877
  "spec.md#A view for each path": eae36e7c8ed8fdeb
  "spec.md#Acceptance Criteria": eed9179e151e2ef4
  "spec.md#Applicable Rules": bd97f621053ed391
  "spec.md#Behavior": 9eefe5610fa92954
  "spec.md#Bounded tool results": c11e0a628f58820f
  "spec.md#Edge Cases": 86970d58dae27734
  "spec.md#Loading tools": 149ca89a5b2ae964
  "spec.md#Motivation": df4dac1820c9e059
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 0169ec5fb4385b79
  "spec.md#`/{project}:status` without a model turn": fe1050dd9d692ea9
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#`render-status` serves the view, and `dashboard` stops carrying it"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Bounded results"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#One view model, two renderings, one bound"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Corpus-growing results are bounded through `bounded_lists`"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-TXN-002: {verdict: waived, reason: "concurrent `/{project}:configure` runs read-modify-write the per-contributor `settings.local.json` through `merge-status-hook` with no version check or lock, so one run's hook entry can be lost; both merges are idempotent, so the next run restores it, and a lock would serialize a file two runs rarely share (Edge Cases; owner, 2026-11-14)", decided-by: "andy@stone.dev", decided-at: "2026-11-14T17:31:43Z"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 6
  undispositioned: 0
decisions:
  - key: grounding — plan.md says the parser ends the Instructions section at its first sub-heading, so amend parses as legacy prose, with no file-and-symbol citation
    outcome: waived
    reason: "advisory and no chore: a claim about existing repository, library or host behavior whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore; it predates this reopening and the claim is true"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: grounding — plan.md says a name costs about a third of a row, so 1,701 planned specs name 1,498 where rows would name 515, with no method or corpus cited
    outcome: waived
    reason: "advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: grounding — plan.md states the 279,572-byte and 791,817-byte list-file sizes at the 1,701-spec corpus with no command or research.md citation
    outcome: waived
    reason: "advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: "grounding — Trade-offs, 'Renaming merge-status-hook': 'The name is in 28 live files (115 occurrences, 7 specs)' states a count with no query or command, so it cannot be re-derived."
    outcome: waived
    reason: "advisory and no chore: a measurement recorded where the decision was made, re-derivable by grep over the live-artifact set; it states no false figure"
    decided-at: 2026-11-26T01:21:32Z
    decided-by: andy@stone.dev
  - key: "grounding — Trade-offs, 'Host-run claude --version passed to the primitive': 'It would need a permission entry in two seeds' asserts what the current seeds contain without naming the seed files."
    outcome: waived
    reason: "advisory and no chore: a claim about existing configure seeds whose re-sourcing needs a source read outside analyze's scope; research rather than a chore"
    decided-at: 2026-11-26T01:21:32Z
    decided-by: andy@stone.dev
  - key: artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 65585 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: plan.md is 080's design record, over one read before this reopening (it was 64,683 bytes at 0.59.11's review); its finished Affected Files rows are trimmed with /ductus:prune once 080 is done
    decided-at: 2026-11-26T01:21:32Z
    decided-by: andy@stone.dev
---

# Analysis — 080-deterministic-runtime-efficiency

## Summary

0 hard-fail, 0 blocking, 6 advisory; not blocking. 8 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 6 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Technical Decisions, 'One view model, two renderings, one bound': states 'At the rich 1,701-spec corpus that file is 279,572 bytes ... dashboard's list file ... is 791,817 bytes' as fact, with no command, script or research.md section naming how either was measured. — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure (decided by the rule)
- grounding — Technical Decisions, 'One view model, two renderings, one bound' step 3: 'at 1,701 planned specs this names 1,498 specs where keeping rows would name 515' is a measured-looking figure with no cited source or method. — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure (decided by the rule)
- grounding — Technical Decisions, 'Tool loading derives from each command's parsed steps': 'The parser ends the section at its first sub-heading, so a command ... as amend's do, parses as legacy prose' asserts current parser behavior without citing a file and symbol. — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a claim about existing repository, library or host behavior whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore; it predates this reopening and the claim is true (decided by the rule)
- grounding — Trade-offs, 'Renaming merge-status-hook': 'The name is in 28 live files (115 occurrences, 7 specs)' states a count with no query or command, so it cannot be re-derived. — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a measurement recorded where the decision was made, re-derivable by grep over the live-artifact set; it states no false figure
- grounding — Trade-offs, 'Host-run claude --version passed to the primitive': 'It would need a permission entry in two seeds' asserts what the current seeds contain without naming the seed files. — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a claim about existing configure seeds whose re-sourcing needs a source read outside analyze's scope; research rather than a chore
- artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 65585 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: plan.md is 080's design record, over one read before this reopening (it was 64,683 bytes at 0.59.11's review); its finished Affected Files rows are trimmed with /ductus:prune once 080 is done

## Deferred to other specs

*None.*

## Waived rules

- BE-TXN-002 (MUST) — concurrent `/{project}:configure` runs read-modify-write the per-contributor `settings.local.json` through `merge-status-hook` with no version check or lock, so one run's hook entry can be lost; both merges are idempotent, so the next run restores it, and a lock would serialize a file two runs rarely share (Edge Cases; owner, 2026-11-14) — decided by `andy@stone.dev` at `2026-11-14T17:31:43Z`

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 2
- never-existed: 1
- root-absent: 4
- ships-to-adopter: 1

## Fixed in this run

*None.*
