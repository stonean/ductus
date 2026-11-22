---
spec: 008-security-rules
last-run: 2026-11-22T12:50:35Z
analyzed-against: be0edef9aaf784e5255cb586e50c92da057698f1
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 10
analyzed-digest:
  data-model.md: b3c50297287cbc973c443e03f8f40f62dd8921d274b613aabf22bc87cdbc7acc
  plan.md: 920cdae131a185e8a3519191f168d4009276b011bd66dfa47ff7c072371ec7fe
  research.md: ed21f392dba2461ab97f92269f73dcaa38aaf924e16ce309427a316a797da9d4
  review.md: 6097a6939e1a99702cc42c525412af7d9d649416167fe09a62d180e2b6b4544a
  scenarios/a-custom-credential-header-does-not-follow-a-redirect.md: 2c25f447f5731eb05192956205e08b600cc355376fd35492fc47573d2ebe0b2f
  scenarios/a-statement-carries-one-obligation-keyword.md: 25130f7eb66ca03216927ef7293bbb0a1e55d9e26697be629c143b176d687e11
  scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md: db3d868a816c78dd41f62dd5bb08b44da41e0d694ea28097e8de53dcba753011
  scenarios/independent-review-findings-await-the-owner.md: 356a0ea3225449ddeff7e75966ade95c35af36f83e3187baf9ca05db8eed5c30
  scenarios/rules-are-as-short-as-their-required-content.md: f6afe0da29359d2c2ae5ef8f81930ddf7b82cb8fc02d53c7dcaaf237d1455705
  scenarios/security-rules-flag-what-their-sources-support.md: 4e0fd44b925d8c60d60af3237679ae4d8027dd96188b79ff1ea75cc8d19faebe
  scenarios/single-host-appliance-rule-fit.md: 8f352bb3397cb2ccd81c841a72c74a19f6f125f8171485948969d337b1441a14
  scenarios/x-frame-options-carries-one-strength.md: 3f003bce779297c8d2af9d604d36f9d197534e28cfe7e835a2c4d5463ce5ee06
  spec.md: a00d7fe397ee7c2784e64fc6a53435b3f06375679b9b67c667a5b81429d20a46
  tasks.md: 0d1112f459467211f1c8766662a6dc4a71fb74ba30f3bc4e8a82242781afbc70
unexamined-by-reason:
  not-a-live-claim: 4
  root-absent: 1
  ships-to-adopter: 5
rule-digest:
  accessibility-frontend.md: 4d7f5ac09d457c99774f5fc341222a464d2b461cb0467cd58a13dc31c928359b
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: f892282383d7377ded2e5fd1ee3e30421d0d6ad3f9bb1fd354ff0d9d114e807a
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "data-model.md": b3ff6cd14897e6b8
  "data-model.md#008 — Security Rules Data Model": 9387fd5f02931e02
  "data-model.md#Category abbreviations": b741c6bb41cd1f81
  "data-model.md#Examples": 6b71e56b17450425
  "data-model.md#ID stability invariants": 5f1278ed184b8f7c
  "data-model.md#Rule ID format": c8f20ab8793b088e
  "data-model.md#Rule entry fields": d0b5c1c566a67dc7
  "data-model.md#Rule file integrity": 3e9118d2020b9d92
  "data-model.md#Rule file structure": 62536748a40b8f69
  "data-model.md#Severity classification": 85993b0583eeee26
  "data-model.md#Verification phrasing": 8dbba8ffc7ef85cf
  "plan.md": a3f480b7582171f5
  "plan.md#008 — Security Rules Plan": b742d983058c26dd
  "plan.md#Affected Files": 0c11bc9787927abd
  "plan.md#Block on malformed/duplicate/unknown vs. warn-and-skip": 32fc2797c2efebbd
  "plan.md#Brownfield audit lives in ductus, not `/{project}:analyze`": 7a4fddb12bec8168
  "plan.md#Constitution gets a one-line append": f4fe361c12b8f0a4
  "plan.md#Coverage rationale for the additions": 166b80e4d8a28d5e
  "plan.md#Data model formalizes rule entry schema": 00aa940ef63333c7
  "plan.md#Edge case behaviors are encoded as a dedicated check section in `/{project}:analyze`": 94f316758cf411b2
  "plan.md#One-line constitution edit vs. expanded principle": 4e233e622c2f84b5
  "plan.md#Open Questions Resolved": 80688dc67a66c34a
  "plan.md#Overview": 45d4b4e0ded83715
  "plan.md#Project destination is the spec root's `rules/` directory": 3823ed1d9b3933fd
  "plan.md#Rule entry format is heading-anchored markdown": a0627ebeb804f5d3
  "plan.md#Rule files live at `framework/rules/`": a9c89c876cd2bf81
  "plan.md#Rule files written in markdown vs. structured format (YAML/JSON)": 577829b1adb901a2
  "plan.md#Starter set vs. comprehensive coverage": b9026bbc016e1fb3
  "plan.md#Starter set: real, threat-grounded rules covering OWASP Top 10 + API Top 10 essentials": 3e5275efdc7e1dce
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Trade-offs": 6d48e49804e54ced
  "plan.md#Verification-by-prompt vs. Verification-by-pattern": 0cf91d5ab581a804
  "plan.md#`/{project}:analyze` uses each rule's `Verification` field as a mini-prompt": f91e8a6eb4c1b595
  "scenarios/a-custom-credential-header-does-not-follow-a-redirect.md": 243aa1ae24ab1ed2
  "scenarios/a-custom-credential-header-does-not-follow-a-redirect.md#A-custom-credential-header-does-not-follow-a-redirect": b3329ae71ce8aa41
  "scenarios/a-custom-credential-header-does-not-follow-a-redirect.md#Behavior": db05f64cfda91d52
  "scenarios/a-custom-credential-header-does-not-follow-a-redirect.md#Context": f78801f37f70c716
  "scenarios/a-custom-credential-header-does-not-follow-a-redirect.md#Edge Cases": 2d1c45a50553ae6f
  "scenarios/a-custom-credential-header-does-not-follow-a-redirect.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-custom-credential-header-does-not-follow-a-redirect.md#Resolved Questions": 4307b06d1cdf7084
  "scenarios/a-statement-carries-one-obligation-keyword.md": f25619bd7430f2d4
  "scenarios/a-statement-carries-one-obligation-keyword.md#A-statement-carries-one-obligation-keyword": d36e45ba9a5e3c7e
  "scenarios/a-statement-carries-one-obligation-keyword.md#Behavior": 93d13abb205dd00b
  "scenarios/a-statement-carries-one-obligation-keyword.md#Context": 7d18655734cf83d8
  "scenarios/a-statement-carries-one-obligation-keyword.md#Edge Cases": 542467e6593b1a98
  "scenarios/a-statement-carries-one-obligation-keyword.md#Open Questions": 20edda8881516e4e
  "scenarios/a-statement-carries-one-obligation-keyword.md#Resolved Questions": 40541b38ed9c219c
  "scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md": 243aa1ae24ab1ed2
  "scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md#An-operator-proxy-is-the-fetch-trust-boundary": 424cec977f0ca861
  "scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md#Behavior": 29ddc16bf661dbd2
  "scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md#Context": c43d96a3d67e5d65
  "scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md#Edge Cases": 7da90c9681ad717e
  "scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md#Open Questions": 9fc5979d590cdb97
  "scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md#Resolved Questions": 5c26ec44d6f82e5e
  "scenarios/independent-review-findings-await-the-owner.md": f4036d4aa3763d17
  "scenarios/independent-review-findings-await-the-owner.md#Behavior": e8999e59207720bf
  "scenarios/independent-review-findings-await-the-owner.md#Context": d908edbd7ea8370f
  "scenarios/independent-review-findings-await-the-owner.md#Edge Cases": ce17c73c7143b8a7
  "scenarios/independent-review-findings-await-the-owner.md#Independent-review-findings-await-the-owner": c8dce226c5444c61
  "scenarios/independent-review-findings-await-the-owner.md#Open Questions": 20edda8881516e4e
  "scenarios/independent-review-findings-await-the-owner.md#Resolved Questions": a56c347231de944c
  "scenarios/rules-are-as-short-as-their-required-content.md": f25619bd7430f2d4
  "scenarios/rules-are-as-short-as-their-required-content.md#Behavior": 6c80cd314b8607c3
  "scenarios/rules-are-as-short-as-their-required-content.md#Context": 3a9b4986775d1f93
  "scenarios/rules-are-as-short-as-their-required-content.md#Edge Cases": 85e7243968b685d1
  "scenarios/rules-are-as-short-as-their-required-content.md#Open Questions": 9fc5979d590cdb97
  "scenarios/rules-are-as-short-as-their-required-content.md#Resolved Questions": 5f55717fa55741df
  "scenarios/rules-are-as-short-as-their-required-content.md#Rules-are-as-short-as-their-required-content": cb08d83147e93b9c
  "scenarios/security-rules-flag-what-their-sources-support.md": 4cdb53960f38cccc
  "scenarios/security-rules-flag-what-their-sources-support.md#Behavior": c2db40c74cbcf466
  "scenarios/security-rules-flag-what-their-sources-support.md#Context": 4bf7d895fc03f535
  "scenarios/security-rules-flag-what-their-sources-support.md#Edge Cases": ff562ffd81851deb
  "scenarios/security-rules-flag-what-their-sources-support.md#Open Questions": 9fc5979d590cdb97
  "scenarios/security-rules-flag-what-their-sources-support.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/security-rules-flag-what-their-sources-support.md#Security-rules-flag-what-their-sources-support": ae659dff00012d38
  "scenarios/single-host-appliance-rule-fit.md": 243aa1ae24ab1ed2
  "scenarios/single-host-appliance-rule-fit.md#Behavior": 6170d3afe278157b
  "scenarios/single-host-appliance-rule-fit.md#Context": a2c8d3e05df7e229
  "scenarios/single-host-appliance-rule-fit.md#Edge Cases": c0ad9a3a62089ffd
  "scenarios/single-host-appliance-rule-fit.md#Open Questions": 20edda8881516e4e
  "scenarios/single-host-appliance-rule-fit.md#Resolved Questions": d7678a93d1f79e4a
  "scenarios/single-host-appliance-rule-fit.md#Single-host-appliance-rule-fit": cd15e6b0cb4fe86b
  "scenarios/x-frame-options-carries-one-strength.md": 4cdb53960f38cccc
  "scenarios/x-frame-options-carries-one-strength.md#Behavior": cefe7d2cab01fe63
  "scenarios/x-frame-options-carries-one-strength.md#Context": b4a4c74c6bc25489
  "scenarios/x-frame-options-carries-one-strength.md#Edge Cases": d7c339ff6e779fe9
  "scenarios/x-frame-options-carries-one-strength.md#Open Questions": 20edda8881516e4e
  "scenarios/x-frame-options-carries-one-strength.md#Resolved Questions": 397845cefbd54705
  "scenarios/x-frame-options-carries-one-strength.md#X-frame-options-carries-one-strength": 988e9ca14e902fd8
  "spec.md": 9199fced20d71b25
  "spec.md#008 — Security Rules": de50df2df01620af
  "spec.md#Acceptance Criteria": 22162e6761ba81f1
  "spec.md#Analyze Integration": 4f17bfd8baed7d63
  "spec.md#Audit logic": 734de12f37075f45
  "spec.md#Backend (`security-backend.md`)": 36ea0cd01f546e85
  "spec.md#Brownfield Adoption": 8ea733739d5c4e81
  "spec.md#Constitution Reference": b9ffa7a78fe46d0e
  "spec.md#Ductus Integration": edf0fe321af2833b
  "spec.md#Edge Cases": 5422b85065109d29
  "spec.md#Edge-case behavior": 1d3a3aa42bee1912
  "spec.md#Finding format": 116d25fc17bff7e1
  "spec.md#Frontend (`security-frontend.md`)": 612e9f1a1f798a07
  "spec.md#ID stability": 2331ab0c3fcb9b5b
  "spec.md#Idempotency": 8c063486e6be4f8a
  "spec.md#Motivation": 551d2061b840d56e
  "spec.md#Open Questions": 3e91c5044330286f
  "spec.md#References": cd842b648699eaf4
  "spec.md#Reporting": dd1ec5081cd5cc30
  "spec.md#Resolved Questions": ae0ed2e5d7387d7c
  "spec.md#Rule Files": 07a2201de274c636
  "spec.md#Rule Format": 1316b45b37f24b93
  "spec.md#Trigger": f8509c54c7558fc7
  "spec.md#Two Rule Files": b5f95279b6106de4
  "spec.md#Verification phrasing": 88336fbad5d271ca
  "spec.md#Versioning and Evolution": ec9c1c76b83c46a3
  "spec.md#Why not block the analyze gate instead?": 71ea7d8d154ba046
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "grounding — Technical Decisions, Coverage rationale: the claim that OWASP's 2025 edition maps SSRF (CWE-918) into A01:2025 is stated as fact with no link, fetch date or citation to a fetched source."
    outcome: waived
    reason: the claim names its source, the OWASP Top 10's 2025 edition and its A01:2025 category, as constitution §grounding's citation rule asks of a citation; the coverage rationale records why the rule was written
    decided-at: 2026-11-22T12:50:35Z
    decided-by: rule
  - key: "grounding — Technical Decisions, Coverage rationale: native deserialization (Java ObjectInputStream, Python Pickle, .NET BinaryFormatter) and type-instantiating parsers 'enable RCE on many platforms' is asserted about external systems with no source."
    outcome: waived
    reason: a coverage rationale's statement of the threat a rule addresses, not a claim about how the existing system behaves, what a schema holds or what an external system returns, which is what step 15's grounding check covers
    decided-at: 2026-11-22T12:50:35Z
    decided-by: rule
  - key: "grounding — Technical Decisions, Coverage rationale: 'default parser configurations frequently allow external entity expansion' (XXE) is asserted about external parsers with no source."
    outcome: waived
    reason: a coverage rationale's statement of the threat a rule addresses, not a claim about how the existing system behaves, what a schema holds or what an external system returns, which is what step 15's grounding check covers
    decided-at: 2026-11-22T12:50:35Z
    decided-by: rule
  - key: "grounding — Resolved Questions 8: the claim that PEP 740 makes no recommendation that installing clients verify attestations, and that PyPI documents verification only through pypi-attestations, is stated as fact with no link or fetch date (unlike Q11, which records fetch dates)."
    outcome: waived
    reason: a Resolved Question dated 2026-10-19 that records the owner's decision and names the source it consulted, PEP 740, as constitution §grounding's citation rule asks of a citation
    decided-at: 2026-11-22T12:50:35Z
    decided-by: rule
  - key: "grounding — Resolved Questions 5: the claim about the current OWASP CSRF Prevention Cheat Sheet's server-side policy offering both treatments, with fail-safe blocking recommended, names the source but gives no link or fetch date."
    outcome: waived
    reason: a Resolved Question dated 2026-10-19 that records the owner's decision and names its source, the OWASP Cross-Site Request Forgery Prevention Cheat Sheet, as constitution §grounding's citation rule asks of a citation
    decided-at: 2026-11-22T12:50:35Z
    decided-by: rule
---

# Analysis — 008-security-rules

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 10 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Technical Decisions, Coverage rationale: the claim that OWASP's 2025 edition maps SSRF (CWE-918) into A01:2025 is stated as fact with no link, fetch date or citation to a fetched source. — `specs/008-security-rules/plan.md` — **waived**: the claim names its source, the OWASP Top 10's 2025 edition and its A01:2025 category, as constitution §grounding's citation rule asks of a citation; the coverage rationale records why the rule was written (decided by the rule)
- grounding — Technical Decisions, Coverage rationale: native deserialization (Java ObjectInputStream, Python Pickle, .NET BinaryFormatter) and type-instantiating parsers 'enable RCE on many platforms' is asserted about external systems with no source. — `specs/008-security-rules/plan.md` — **waived**: a coverage rationale's statement of the threat a rule addresses, not a claim about how the existing system behaves, what a schema holds or what an external system returns, which is what step 15's grounding check covers (decided by the rule)
- grounding — Technical Decisions, Coverage rationale: 'default parser configurations frequently allow external entity expansion' (XXE) is asserted about external parsers with no source. — `specs/008-security-rules/plan.md` — **waived**: a coverage rationale's statement of the threat a rule addresses, not a claim about how the existing system behaves, what a schema holds or what an external system returns, which is what step 15's grounding check covers (decided by the rule)
- grounding — Resolved Questions 8: the claim that PEP 740 makes no recommendation that installing clients verify attestations, and that PyPI documents verification only through pypi-attestations, is stated as fact with no link or fetch date (unlike Q11, which records fetch dates). — `specs/008-security-rules/spec.md` — **waived**: a Resolved Question dated 2026-10-19 that records the owner's decision and names the source it consulted, PEP 740, as constitution §grounding's citation rule asks of a citation (decided by the rule)
- grounding — Resolved Questions 5: the claim about the current OWASP CSRF Prevention Cheat Sheet's server-side policy offering both treatments, with fail-safe blocking recommended, names the source but gives no link or fetch date. — `specs/008-security-rules/spec.md` — **waived**: a Resolved Question dated 2026-10-19 that records the owner's decision and names its source, the OWASP Cross-Site Request Forgery Prevention Cheat Sheet, as constitution §grounding's citation rule asks of a citation (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 4
- root-absent: 1
- ships-to-adopter: 5

## Fixed in this run

- grounding — Technical Decisions, Project destination: 'the rules/ subdirectory arrived in ductus 0.6.0' is stated as fact with no changelog or commit citation (only the migration is named). — `specs/008-security-rules/plan.md` — **fixed** (decided by the rule)
- plan-consistency — Technical Decisions, Brownfield audit: steps 4-5 and the trade-off still have the audit append findings to specs/inbox.md, which the spec's Brownfield Adoption and AC22/AC25-AC27 (as superseded by 058) reverse to report-only, writing nothing; a 058 signpost marks it as first-built design, but the body still states the reversed sink. — `specs/008-security-rules/plan.md` — **fixed** (decided by the rule)
