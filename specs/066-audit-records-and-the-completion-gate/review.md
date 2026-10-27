---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-10-27T15:08:51Z
reviewed-against: 751c0cf3915dbf485910fc9cf23715fd00e8762f
diff-base: 2fadb5a9c1d292e3688b0b6e80e9b58acb67fe84
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 41
scope: 67
skipped-passes: []
reviewed-digest:
  data-model.md: 8851119617356540be9a3fd4434cdf7eecc04bcb9ae5471322a0c5a93ae1ed77
  scenarios/a-done-spec-has-no-transition-to-gate.md: d4a9f47277ddcd5e8735be7d13ee7f57606827a5f36db608da3fd70a05aaa39a
  scenarios/a-done-specs-plan-edit-leaves-its-analysis-current.md: 3d63a66653413cb90127966cf092b984c71d44dad6d9dda77eba08131f6737e6
  scenarios/a-multi-number-renumber-is-one-rewrite.md: 0b0c25fc2e8446f403bff82cff5b0d64c86cfbc45d98bc492fd5fa824c9fdd5a
  scenarios/a-renamed-file-contributes-its-rewrites.md: 52907e92b668e9d100ab339e0d36953cd6d025104b3650174bf7b806b3631490
  scenarios/a-review-states-what-it-read.md: 333ffc663a0d0f471970dbcdad9b010583dd891f651e626741962cad8265da14
  scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md: 4273f4da9ffa9a0b4b02b06dba0b8d05a2c8ceb4780937dc360c1e72153d57fc
  scenarios/only-gating-findings-hold-done.md: 43012eb89501db9f4abc468448aadf7fc1867f72cd0bb1502bad16f26ea107d0
  scenarios/process-waivers-file-lists.md: 3fb3b9e54f414691d28d984ba76cd48c7e1de4fe2ee67259a44ea8559da447dc
  scenarios/review-base-includes-the-transition-commit.md: 62f1434d3a2d59d62ebbfa37d5c6561d82c4cb343907d7cbe081e0df63801b3f
  scenarios/review-scope-includes-uncommitted-work.md: 72e5ad46790ca087bd9ee0be134f875ebad189eed87002409f4ede60524d3097
  scenarios/review-scope-plan-affected-table-format.md: 25d9a4b8b89fea9460ea2b134e4184bcf117df678ed501d1ba16efa9f84abe02
  scenarios/review-scope-union.md: e5fa76ef2b48ea246649338d71d8c0a6988d7e48a9202e677ad593657316c0ae
  scenarios/review-staleness-gate.md: 61427d7c8f1393cf72edeae32fd54502c99641499c429cf33b7368d93e0e7131
  scenarios/review-staleness-on-done-specs.md: 18484a327c18ea44c3cfbe24fc22d1b727c5033cf619aa3760c3652fdc1b66f5
  scenarios/the-committed-tree-horizon.md: cac65fb842defe0ce0326a76274794d1402d96c10aa02a2f977fd6d326ab3772
  scenarios/the-cross-spec-impact-gate.md: b85c2d557733a29beb8084823a9156e13b283090a283203aa14dcaa85afed3c1
  scenarios/write-analysis-and-the-second-gate.md: afaaa2f6957547d1f9dce559426353e58a00da287b1066f7eb0af05d51330397
  scenarios/write-review-known-field-quoting.md: 914954823ee3da52dccbb25749851013d5dd3be442dcc85b09661c27c7bea2a9
blocking: false
dispositions:
  fixed: 18
  routed: 0
  discarded: 7
  undispositioned: 0
decisions:
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: discarded
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: discarded
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: discarded
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: discarded
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: discarded
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: discarded
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-10-27T06:47:44Z
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

# Review — 066-audit-records-and-the-completion-gate

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
