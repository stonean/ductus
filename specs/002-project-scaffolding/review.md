---
spec: 002-project-scaffolding
last-run: 2026-10-27T01:38:52Z
reviewed-against: e48df859ac3778879d93557ea4844ff29e6dd444
diff-base: a717c3e420872b7962aeff48ee459c514cef6906
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 31
scope: 36
skipped-passes: []
reviewed-digest:
  scenarios/claude-md-template-imports-load.md: b2057e079fb40dec6c6b0b55a0e2c0b2fd19dcd0107f74562c1ba31e02c0f6e6
blocking: false
dispositions:
  fixed: 22
  routed: 1
  discarded: 8
  undispositioned: 0
decisions:
  - key: "other: the module doc keeps 'a dangling @import yields a constitution that is simply not loaded' — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: it recounts the 048 AC10 incident, which was an @import, and 027's migration-chain-reference-integrity scenario keeps the same sentence as its record
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a BOM, or punctuation such as ( or \" before the `@`, keeps an @path import from being read as a reference — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: "contrived: the template and the bootstrap write each import at the start of a line, and 067's an-at-path-import-is-a-reference records both cases as edge cases"
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: --ref=main adopters get the migration before the @path orphan-check fix, which ships only in the runtime — `framework/migrations/claude-md-import-syntax.md`"
    outcome: discarded
    reason: a runtime change reaches adopters only through a ductus-v tag, and the next release is the owner's to cut; the gap closes there
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a legacy root `@import constitution.md` becomes `@constitution.md`, an orphan no check reports — `framework/migrations/claude-md-import-syntax.md`"
    outcome: discarded
    reason: the root path is not a managed root, the line was equally silent before, and `@import constitution.md` never loaded anything
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: 040's spec-root-rule-stated-once rests a rejected option on the CLAUDE.md premise 017 AC16 states — `specs/040-configurable-specs-dir/scenarios/spec-root-rule-stated-once.md`"
    outcome: routed
    target: specs/050-constitution/tasks.md
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "security: the migration turns @import lines that loaded nothing into live imports — `framework/migrations/claude-md-import-syntax.md`"
    outcome: discarded
    reason: step 6 names every rewritten path for the operator, Claude Code asks before loading an import from outside the working directory, and no loaded security rule fires on a local text rewrite
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: the migration's code-span sentence repeats what 'begins with @import' already implies — `framework/migrations/claude-md-import-syntax.md`"
    outcome: discarded
    reason: "harmless: it names the edge case the scenario lists"
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: the adopter runs met pre-existing bootstrap behavior: v0.28.0 cannot be walked at its tag, the Frontmatter Migration's clean-tree precheck refuses after a migration writes under specs/, AGENTS.md placeholders an old release left literal are never repaired, and v0.59.11's dead old-form constitutions region survives — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: observations about the bootstrap's own machinery made while reviewing this change, none caused by it and none blocking it; the dead region is already an inbox item
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
---

# Review — 002-project-scaffolding

## Summary

Five passes over the window a717c3e4..e48df859, reviewed with `--since a717c3e4`: 002's own diff base (112ae023, the parent of the 2026-10-26 amend) also holds 072's, 050's and 058's later work, which their own records review, while every line of this scenario's implementation is in 96934872. The window is shared with 044 and 055, which 96934872 reopened, and was reviewed once in depth here. An independent reviewer ran security, reuse, quality, efficiency and simplicity over 96934872 against api-, concurrency-, observability-, performance-, reliability- and security-backend plus configuration- and quality-cross (the runtime is a local CLI, so the backend surface; the service-only rules AGENTS.md names never fire): 0 MUST, 0 SHOULD against a loaded rule, and 31 observations across the three rounds, none mapping to a loaded rule. Round 1 (d1e3ec96) was re-read independently and raised three more, fixed in round 2 (e48df859); round 2's re-read found no MUST or SHOULD, and its one note — a present tense left in task 5's handoff prose — is fixed on disk after it and was not itself re-read. A real adopter bootstrap of 96934872 on two subjects (adopted at ductus-v0.28.0 and at ductus-v0.59.11 with a registered sibling checkout, each then run at --ref=main) applied the migration in its chain, converted every import, and converged on a second run; a claude -p probe of the first subject answered from AGENTS.md only after the migration. Examined: the 31 window files, the 23 of 96934872 read by the reviewer (the bootstrap twins, ductus-procedure.md and reference/analyze.md in their changed regions with govern.md proven byte-identical by cmp) and the 8 later ones by the re-readers. `scope` counts 36 entries, each one path: those 31, two plan Affected Files rows untouched since the diff base and not read (`framework/templates/project/gitignore`, `framework/templates/project/project-readme.md`), and the three review records this window writes for 002, 044 and 055, which are the passes' output rather than their subject.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 044 line 36 and AC10 said this repository's CLAUDE.md imports framework/constitution.md, false since 072 — `specs/044-relocate-constitution-under-govern-directory/spec.md` — **fixed**
- other: 067's revision-qualified scenario said a prefix after `@` never starts a token, and 067 had no owner artifact for the `@` rule — `specs/067-analyze-and-audit-checks/scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md` — **fixed**
- other: README, the CLAUDE.md template's routing line and 050's spec said /target is what loads a shared constitution — `README.md` — **fixed**
- other: the module doc keeps 'a dangling @import yields a constitution that is simply not loaded' — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: it recounts the 048 AC10 incident, which was an @import, and 027's migration-chain-reference-integrity scenario keeps the same sentence as its record
- other: the migration's wording on quotes, spaces, fences, line endings, the report count and the converged line could be executed two ways — `framework/migrations/claude-md-import-syntax.md` — **fixed**
- other: the migration's pin check ran before its idempotency check, and its advice re-imported the constitution — `framework/migrations/claude-md-import-syntax.md` — **fixed**
- other: a BOM, or punctuation such as ( or " before the `@`, keeps an @path import from being read as a reference — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: contrived: the template and the bootstrap write each import at the start of a line, and 067's an-at-path-import-is-a-reference records both cases as edge cases
- other: 002's plan and scenario said every dangling @path import is reported, not only one into a ductus-managed path — `specs/002-project-scaffolding/plan.md` — **fixed**
- other: 055's plan Overview still called the managed block navigation — `specs/055-shared-constitution/plan.md` — **fixed**
- other: the shared-constitution doc said AGENTS.md has no import directive, though Claude Code expands @path inside it — `docs/shared-constitution.md` — **fixed**
- perf: whether a shared constitution imported both by hand and in the managed block loads twice was unmeasured — `specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md` — **fixed**
- other: --ref=main adopters get the migration before the @path orphan-check fix, which ships only in the runtime — `framework/migrations/claude-md-import-syntax.md` — **discarded**: a runtime change reaches adopters only through a ductus-v tag, and the next release is the owner's to cut; the gap closes there
- other: a legacy root `@import constitution.md` becomes `@constitution.md`, an orphan no check reports — `framework/migrations/claude-md-import-syntax.md` — **discarded**: the root path is not a managed root, the line was equally silent before, and `@import constitution.md` never loaded anything
- other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md` — **discarded**: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
- other: 040's spec-root-rule-stated-once rests a rejected option on the CLAUDE.md premise 017 AC16 states — `specs/040-configurable-specs-dir/scenarios/spec-root-rule-stated-once.md` — **routed** to `specs/050-constitution/tasks.md`
- security: the migration turns @import lines that loaded nothing into live imports — `framework/migrations/claude-md-import-syntax.md` — **discarded**: step 6 names every rewritten path for the operator, Claude Code asks before loading an import from outside the working directory, and no loaded security rule fires on a local text rewrite
- other: 002 task 5 was unticked pending the adopter bootstrap, and 044 and 055 needed a review and an analyze to close — `specs/002-project-scaffolding/tasks.md` — **fixed**
- other: the migration's code-span sentence repeats what 'begins with @import' already implies — `framework/migrations/claude-md-import-syntax.md` — **discarded**: harmless: it names the edge case the scenario lists
- other: /target step 4 ordered every shared constitution after the framework one, though an import loads first — `framework/commands/target.md` — **fixed**
- other: 044's scope still listed the CLAUDE.md seed's `@import constitution.md` line as a shipped reference — `specs/044-relocate-constitution-under-govern-directory/spec.md` — **fixed**
- other: 050's governance-is-multi-source scenario said /target loads each shared document — `specs/050-constitution/scenarios/governance-is-multi-source.md` — **fixed**
- other: 050 task 35 named three passages but said two, and did not price 040's reopen — `specs/050-constitution/tasks.md` — **fixed**
- other: the README said the block loads at startup for every Claude-style agent, unapproved external imports included — `README.md` — **fixed**
- other: 067's punctuation edge case claimed how Claude Code parses (@path) — `specs/067-analyze-and-audit-checks/scenarios/an-at-path-import-is-a-reference.md` — **fixed**
- other: the migration's closing-fence rule was narrower than CommonMark and left an unclosed fence undefined — `framework/migrations/claude-md-import-syntax.md` — **fixed**
- other: the migration's pin warning told the operator to delete a constitution line that may not exist — `framework/migrations/claude-md-import-syntax.md` — **fixed**
- other: the migration reported 'rewrote 0 import line(s)' when it only removed the constitution line — `framework/migrations/claude-md-import-syntax.md` — **fixed**
- other: the migration said the next /ductus run re-renders the block, which the same run does — `framework/migrations/claude-md-import-syntax.md` — **fixed**
- other: 044's plan kept the CLAUDE.md-imports-the-constitution premise in a dated note — `specs/044-relocate-constitution-under-govern-directory/plan.md` — **fixed**
- other: task 5 was ticked on an adopter run of the migration text the next rounds edited, and its prose said the check skips any @ — `specs/002-project-scaffolding/tasks.md` — **fixed**
- other: the adopter runs met pre-existing bootstrap behavior: v0.28.0 cannot be walked at its tag, the Frontmatter Migration's clean-tree precheck refuses after a migration writes under specs/, AGENTS.md placeholders an old release left literal are never repaired, and v0.59.11's dead old-form constitutions region survives — `framework/bootstrap/ductus-procedure.md` — **discarded**: observations about the bootstrap's own machinery made while reviewing this change, none caused by it and none blocking it; the dead region is already an inbox item

## Skipped passes

*None.*

## Unexamined governance

*None.*
