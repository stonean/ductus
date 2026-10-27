---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-10-27T15:08:51Z
reviewed-against: 751c0cf3915dbf485910fc9cf23715fd00e8762f
diff-base: 2fadb5a9c1d292e3688b0b6e80e9b58acb67fe84
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 41
scope: 74
skipped-passes: []
reviewed-digest:
  data-model.md: 416e488447e1fb195e85babd92882b3d5eefa1cb5e3e2876809a59f487420818
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 6ae5b1d58174acda9aadcc1d96024e65f28eea998dd2fc857a84616991c9bc5b
  scenarios/a-pending-fold-never-holds-done.md: 602c9f86e24d0a2cfa9cdebf78bf3d32e09255b00b189cd882d89d99e433e4b2
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
blocking: false
dispositions:
  fixed: 18
  routed: 0
  discarded: 7
  undispositioned: 0
decisions:
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

# Review — 051-branch-scoped-spec-numbering

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
