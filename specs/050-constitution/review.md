---
spec: 050-constitution
last-run: 2026-10-27T15:08:51Z
reviewed-against: 751c0cf3915dbf485910fc9cf23715fd00e8762f
diff-base: 2fadb5a9c1d292e3688b0b6e80e9b58acb67fe84
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 41
scope: 60
skipped-passes: []
reviewed-digest:
  scenarios/a-canonical-source-is-pointed-at-not-copied.md: c1458fa54c4d7c5d25161f0d858079055be89e82389830938fef4e36281eb456
  scenarios/a-declared-cross-spec-impact-gates-done.md: b4a96bd919b383288cc871adf160804acfc5bc127b1798963810ff391a3ac078
  scenarios/a-measurement-states-its-method-and-units.md: 4c250a480c11e9788058f1012b647daa14aba2036d4a3baa9d0c1ba3c5965b2a
  scenarios/a-partial-read-is-not-a-read.md: 078952ab7a67edeb949dd30d5f1bc0e0ef91904c467980f1e5547aab145e82f0
  scenarios/a-project-command-is-run-not-reproduced.md: aedb9dd7e77284422f8650a7a43c0a3f660b1eafc13f9f78408444532592ecfc
  scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
  scenarios/a-retired-filename-leaves-a-decision-record.md: f84a3177744eb03a859d82a4017495c24bcf24466448b7c49ad1a253147ab581
  scenarios/a-review-may-precede-its-commit.md: add57e1653b813a3832e3cae908f48b7a37d78f3cc8c1703a911ec6ad90af420
  scenarios/a-status-commit-holds-only-the-transition.md: 2f164ce212508e7f3b82508543157b868e8533fd15f977497d2ab569ec89f79e
  scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
  scenarios/decisions-go-to-the-owner-one-at-a-time.md: 91dd288f71c59a081c23f8b380218cc89924859727e52885a88cf491c5465060
  scenarios/findings-route-by-scope.md: fd7a9d731637cf4530de35b37f09c0720250c5d962516133f9f33a05c4e36bb0
  scenarios/governance-is-multi-source.md: bccd202e87fadb1423ee07cb81233bb28de2b4e7465946411a90bcf3cd53e907
  scenarios/knowledge-routes-by-population-not-by-kind.md: 0e934ea65cbaf2bf31acc4f45c461bfb868be45deb5aa9fa6601c59b8c003c5c
  scenarios/report-outcomes-not-edits.md: caf13342d7fe0a5af4108024cf424ab921030c896f0743307cf5111be2e163ab
  scenarios/reports-end-with-the-remaining-work.md: b642a2d08b2227e856aefba9fa3ec8cd1efae0eb2a14cc674dfcd455982dbec3
  scenarios/the-constitution-fits-one-read.md: bf9ef08a810e983be198ade81efa95e9d4bc266e53bf6ab7f3be659a8643e75a
  scenarios/the-constitution-states-rules-not-stories.md: 691e5ea78b7012fd9f9a8114f20632d7852d5c319f54e3951b2ebaed65c013bc
blocking: false
dispositions:
  fixed: 18
  routed: 0
  discarded: 7
  undispositioned: 0
decisions:
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: routed
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: routed
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: routed
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: routed
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: routed
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: discarded
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: discarded
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "convention: a-status-commit-holds-only-the-transition's Behavior reads as requiring the bullet to say a transition staged alone is lost — `specs/050-constitution/scenarios/a-status-commit-holds-only-the-transition.md`"
    outcome: discarded
    reason: the Behavior requires the bullet to state its condition, that the spec file holds nothing but the transition, and its reason, that the pre-commit hook restages the file whole; the bullet states both, and the loss of a transition staged alone is the scenario's own account of the incident
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md`"
    outcome: discarded
    reason: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 017's plan and 072's Affected Files still name the old import form — `specs/017-derive-dont-ask/plan.md`"
    outcome: discarded
    reason: each records that spec's own edit at the time, a decision record, not a claim about the tree today
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md`"
    outcome: discarded
    reason: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 022's constitution-excerpts-as-skill-resources says every CLAUDE.md carries the constitution import — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`"
    outcome: discarded
    reason: this change makes the premise true again, the template's third line being @.ductus/constitution.md; 022 is another session's and owes no edit
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: discarded
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md`"
    outcome: discarded
    reason: "contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand"
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
---

# Review — 050-constitution

## Summary

One in-depth review of the shared window 2fadb5a9..751c0cf3, recorded in each spec the change reopened (002, 017, 021, 050, 051, 062, 064, 066, 072): 7ce72592 cut framework/constitution.md to one read and imported it at startup, ad595280 recorded 051's split from 022, d5de121c, da52bc4c and 1503ca58 are three fix rounds, and a750d8a8 routed a bootstrap finding to 013. The five passes read every file the window modified and applied the eleven rule files discover-rule-files selects. The nine code and service rule files (security-backend, security-frontend, api-backend, concurrency-backend, observability-backend, performance-backend, performance-frontend, reliability-backend, accessibility-frontend) have no subject in a window of markdown, shell and TOML; configuration-cross's CFG-CONST-001 holds, since check H in scripts/tests/test-lint-governing-size.sh ties both ceilings to DEFAULT_READ_SIZE_BYTES; quality-cross applies, and QUAL-TEST-001 holds, check H failing when either ceiling is set to 53,813 or the constant to 51,200. No rule violation. An independent five-pass review found 2 MUST and 7 SHOULD defects outside the rules, fixed in round 1; its re-read found 1 SHOULD, fixed in round 2; the re-read of round 2 found no MUST or SHOULD, and round 3, prose only, was not re-read. The 25 observations below record them: 18 fixed, 7 discarded with their reasons. Examined counts the 41 window files the passes read in full. Not counted: .ductus/reference/groom.md and .ductus/reference/review.md, regenerated by scripts/gen-claude-commands.sh from sources read in full; the audit records this run wrote; and this spec's plan Affected Files the window did not touch, which were not reread. Before the change was committed, three read-only sweeps checked the cut and the import by meaning against the live-artifact set, and their findings were corrected in 7ce72592. Verification: claude -p with every tool disallowed answered NOT IN CONTEXT before the import (2fadb5a9) and quoted the constitution's last sentence verbatim after it (da52bc4c), with no tool call; a real adopter bootstrap on main ran both migrations on a subject adopted at ductus-v0.28.0 and constitution-import alone on one whose last_applied was claude-md-import-syntax.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: the cut dropped a requirement, that a CI job's full-depth checkout is commented — `framework/constitution.md` — **fixed**
- bug: constitution-import step 5 gave the inserted line the anchor line's ending, so a final @AGENTS.md with no newline became one line that loads neither file — `framework/migrations/constitution-import.md` — **fixed**
- convention: 072's plan stated the constitution's size as 49,921 bytes — `specs/072-governing-documents-stay-within-one-read/plan.md` — **fixed**
- bug: on a pinned 0.28-0.59 CLAUDE.md, constitution-import's warning told the operator to add a second constitution import beside the one claude-md-import-syntax told them to convert — `framework/migrations/constitution-import.md` — **fixed**
- convention: constitution-import's Background named two of the three kinds of CLAUDE.md that lack the import, omitting one the adopter wrote before adopting — `framework/migrations/constitution-import.md` — **fixed**
- convention: 050's plan said spent done-spec tasks keep their wording while the change rewrote two of them — `specs/050-constitution/plan.md` — **fixed**
- convention: the partial-read rule dropped 'a match-only search', which a-partial-read-is-not-a-read lists — `framework/constitution.md` — **fixed**
- convention: a-status-commit-holds-only-the-transition's Behavior reads as requiring the bullet to say a transition staged alone is lost — `specs/050-constitution/scenarios/a-status-commit-holds-only-the-transition.md` — **discarded**: the Behavior requires the bullet to state its condition, that the spec file holds nothing but the transition, and its reason, that the pre-commit hook restages the file whole; the bullet states both, and the loss of a transition staged alone is the scenario's own account of the incident
- convention: cost levers named no platform tooling, which 010 AC11 says the paragraph points at — `framework/constitution.md` — **fixed**
- convention: 'iterate narrowly but report the project's command' and 'It authorizes nothing destructive' read ambiguously — `framework/constitution.md` — **fixed**
- bug: constitution-import step 5 matched @AGENTS.md without the whitespace tolerance step 2 applies — `framework/migrations/constitution-import.md` — **fixed**
- convention: '50,000 bytes' in constitution-import is true of the shipped file only, since an adopter's copy grows with {project} substituted — `framework/migrations/constitution-import.md` — **fixed**
- convention: constitution-import step 2 both pointed at claude-md-import-syntax's fence rule and restated it — `framework/migrations/constitution-import.md` — **fixed**
- convention: the cut dropped 'so match the prefix', which 064's spec credits §runtime-host-integration with — `framework/constitution.md` — **fixed**
- other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md` — **discarded**: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
- other: 017's plan and 072's Affected Files still name the old import form — `specs/017-derive-dont-ask/plan.md` — **discarded**: each records that spec's own edit at the time, a decision record, not a claim about the tree today
- other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md` — **discarded**: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
- other: 022's constitution-excerpts-as-skill-resources says every CLAUDE.md carries the constitution import — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md` — **discarded**: this change makes the premise true again, the template's third line being @.ductus/constitution.md; 022 is another session's and owes no edit
- bug: constitution-import reported 'already imports the constitution' for a file whose only constitution line is the @import form, which loads nothing — `framework/migrations/constitution-import.md` — **fixed**
- convention: constitution-import's @import-form match was looser than claude-md-import-syntax's import-line definition — `framework/migrations/constitution-import.md` — **fixed**
- convention: constitution-import's Verification said line endings are unchanged beside a bullet allowing one to be added — `framework/migrations/constitution-import.md` — **fixed**
- other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md` — **discarded**: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
- convention: constitution-import's @import-form report line read as firing only when the file has a single import line — `framework/migrations/constitution-import.md` — **fixed**
- convention: constitution-import's Verification said every second run reports 'already imports', which a pinned @import-form file does not — `framework/migrations/constitution-import.md` — **fixed**
- other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md` — **discarded**: contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand

## Skipped passes

*None.*

## Unexamined governance

*None.*
