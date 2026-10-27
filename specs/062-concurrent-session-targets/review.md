---
spec: 062-concurrent-session-targets
last-run: 2026-10-27T15:08:51Z
reviewed-against: 751c0cf3915dbf485910fc9cf23715fd00e8762f
diff-base: 2fadb5a9c1d292e3688b0b6e80e9b58acb67fe84
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 41
scope: 83
skipped-passes: []
reviewed-digest:
  data-model.md: 9ce5458a99a6ba857f8778df877a8b2b0a2708037b23b05319588b4c06aaa10f
  scenarios/first-writers-create-the-ignore-file-once.md: 02149b44fd9c6d76d8e9d97af6235df746bf028cad34e693bb57302dea1b1f15
blocking: false
dispositions:
  fixed: 18
  routed: 0
  discarded: 7
  undispositioned: 0
decisions:
  - key: "contract: `/target X` and `--clear` overwrite a pending fold or consolidation notice undelivered, since target resolves only without an argument (AC21) — task 17 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: no command displays write-session's `peers`, so the writing session is never told who shares its feature (AC19) — task 17 — `framework/commands/target.md:44`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: fold's and consolidate's report steps name neither the other sessions re-pointed or cleared nor unreadable files (AC21, AC22) — task 17 — `framework/commands/consolidate.md:80`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: `ductus exec` consumes resolve-session's notices without emitting them (AC19, AC21) — task 18 — `runtime/src/interpreter/mod.rs:237`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "test-gap: no exec test runs with an identity set, so AC23 is untested where run_exec reads it — task 18 — `runtime/src/main.rs:586`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "test-gap: no test exercises the advisory lock itself; AC4's test passes without it — task 13 — `runtime/tests/concurrent_sessions.rs:178`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "simplicity: two enums represent the removal cause, and the recorded cause is never read back — `runtime/src/primitives/retarget_sessions.rs`"
    outcome: discarded
    reason: one match translates between them, and retarget-sessions, the only caller, validates the cause against the target, so the two cannot disagree
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "efficiency: every resolution parses every session file, and peek takes the exclusive lock — `runtime/src/session.rs:580`"
    outcome: discarded
    reason: the files are sub-kilobyte and bounded by seven days of sessions; the cost is negligible beside a command's own process spawn
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "input: session names have no length limit — `runtime/src/session.rs:71`"
    outcome: discarded
    reason: a name past the filesystem's name limit is not a realistic operator input, and its failure names the path rather than passing silently
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a used-at stamped in the future is never reported — `runtime/src/session.rs:429`"
    outcome: discarded
    reason: a future stamp still expires seven days after it, so nothing accumulates; a clock set ahead is outside the design
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "reuse: inbox_standing keeps its own civil-from-days copy — `runtime/src/primitives/inbox_standing.rs:208`"
    outcome: discarded
    reason: it predates 062 in spec 058's file, which 062 did not touch; consolidating date helpers is a refactor outside this spec
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "reuse: consolidate.md restates §concurrent-features' reasoning in its step and its markdown-only reference — `framework/commands/consolidate.md:77`"
    outcome: discarded
    reason: the restatement predates 062, and a step plus its markdown-only reference both describing the procedure is the command files' two-path convention
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "terminology: 'agent identity' (cli-config-dir) now sits beside 'session identity' — `framework/constitution.md:775`"
    outcome: discarded
    reason: "'agent identity' names cli-config-dir in ten passages that predate 062; renaming it is a terminology sweep outside this spec"
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "exec-path: fold's and consolidate's session step dispatches nothing under exec, and §concurrent-features states no exec carve-out — `framework/commands/fold.md:76`"
    outcome: discarded
    reason: the reduction is documented in each step, the convention the two-paths guarantee sets for exec reductions, as clarify.md's scope note does
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a write overwrites the own record before storing the default and sweeping, so an error part-way loses the notice it replaced — task 20 — `runtime/src/session.rs:1256`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "regression: /target X now fails at step 1 on a malformed own session file it used to repair by overwriting — task 20 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "security: with a proxy set, fetch-archive's connection is tunnelled and its SSRF address pin no longer holds (pre-existing) — `runtime/src/primitives/fetch_archive.rs:130`"
    outcome: routed
    target: specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "test: two lock tests rely on sleeps to reach the contended state — `runtime/src/session.rs:1452`"
    outcome: discarded
    reason: neither can fail falsely, and the deterministic timeout and fresh-repo lock tests carry the behavior
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "concurrency: the legacy layout still read-modify-writes the default without the lock — `runtime/src/session.rs:393`"
    outcome: discarded
    reason: "deliberate and recorded in the plan: creating .ductus/sessions/ beside a legacy file breaks 042's cutover rule, and it lasts only until /ductus migrates the repo"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "ux: /target X announces adopting the feature it is about to leave — `framework/commands/target.md:29`"
    outcome: discarded
    reason: "truthful: the session did adopt that target before the write replaced it"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "reuse: the same 'Display what the write reports' sentence appears in four command files — `framework/commands/target.md:44`"
    outcome: discarded
    reason: each command stating its own procedure in full is the command files' self-contained convention
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "regression: with a malformed shared default, /target X can no longer repair it and every exec walk halts at the seed — task 24 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
  - key: "exec-path: target's continue-past-a-malformed-own-file path has no exec equivalent, and the step does not say so — task 24 — `runtime/src/main.rs:505`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
  - key: "accuracy: 048's new scenario says REQUEST_METHOD makes the matcher ignore HTTP_PROXY; it ignores every proxy variable — task 25 — `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md:21`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
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

# Review — 062-concurrent-session-targets

## Summary

One in-depth review of the shared window 2fadb5a9..751c0cf3, recorded in each spec the change reopened (002, 017, 021, 050, 051, 062, 064, 066, 072): 7ce72592 cut framework/constitution.md to one read and imported it at startup, ad595280 recorded 051's split from 022, d5de121c, da52bc4c and 1503ca58 are three fix rounds, and a750d8a8 routed a bootstrap finding to 013. The five passes read every file the window modified and applied the eleven rule files discover-rule-files selects. The nine code and service rule files (security-backend, security-frontend, api-backend, concurrency-backend, observability-backend, performance-backend, performance-frontend, reliability-backend, accessibility-frontend) have no subject in a window of markdown, shell and TOML; configuration-cross's CFG-CONST-001 holds, since check H in scripts/tests/test-lint-governing-size.sh ties both ceilings to DEFAULT_READ_SIZE_BYTES; quality-cross applies, and QUAL-TEST-001 holds, check H failing when either ceiling is set to 53,813 or the constant to 51,200. No rule violation. An independent five-pass review found 2 MUST and 7 SHOULD defects outside the rules, fixed in round 1; its re-read found 1 SHOULD, fixed in round 2; the re-read of round 2 found no MUST or SHOULD, and round 3, prose only, was not re-read. The 25 observations below record them: 18 fixed, 7 discarded with their reasons. Examined counts the 41 window files the passes read in full. Not counted: .ductus/reference/groom.md and .ductus/reference/review.md, regenerated by scripts/gen-claude-commands.sh from sources read in full; the audit records this run wrote; and this spec's plan Affected Files the window did not touch, which were not reread. runtime/src/primitives/fetch_archive.rs was read for CFG-ENV-001 alone and is not counted: the proxy variables and the system roots are each read once into a OnceLock (PROXY_ENV, SYSTEM_ROOTS), so the rule no longer fires there and its waiver expires. Before the change was committed, three read-only sweeps checked the cut and the import by meaning against the live-artifact set, and their findings were corrected in 7ce72592. Verification: claude -p with every tool disallowed answered NOT IN CONTEXT before the import (2fadb5a9) and quoted the constitution's last sentence verbatim after it (da52bc4c), with no tool call; a real adopter bootstrap on main ran both migrations on a subject adopted at ductus-v0.28.0 and constitution-import alone on one whose last_applied was claude-md-import-syntax.

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
