---
spec: 071-command-reference-split
last-run: 2026-10-26T15:06:31Z
reviewed-against: b540581ca6f76ea36a0d9083fa0ee2508744474d
diff-base: 59dc7aa40e99b69294368d5762ae62f3a095ae17
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 98
scope: 141
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 40
  routed: 0
  discarded: 13
  undispositioned: 0
decisions:
  - key: "other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: "contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: gen-claude-commands.sh writes and prunes through a symlink at .ductus/reference/ — `scripts/gen-claude-commands.sh`"
    outcome: discarded
    reason: the maintainer's own checkout, the same shape as the generator's two existing passes, and no trust boundary crossed (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: check-corpus-links reads framework/reference/'s presence as a framework checkout, which an adopter's own directory of that name would match — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: discarded
    reason: "contrived: it costs only the repository-scope check of shipped copies, which an adopter's spec-corpus hook never runs (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: resolve-anchor and check-artifacts each parse the Shared Files table, and the bootstrap path literal appears three times — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: discarded
    reason: their scopes differ on purpose (the plan records why), and merging them would narrow check-artifacts' suppression set (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: gen-claude-commands.sh's reference pass is a third inline copy of the generate, compare and prune blocks — `scripts/gen-claude-commands.sh`"
    outcome: discarded
    reason: the file's existing style; bash 3.2 has no namerefs, and refactoring it is its own change (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: Family 16's reference arm is not section-scoped and duplicates Family 35's row parser — `scripts/audit/installer-command-parity.sh`"
    outcome: discarded
    reason: "contrived: no framework/reference row exists outside Shared Files (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: Family 18 restates its file label nine times — `scripts/audit/marker-list-parity.sh`"
    outcome: discarded
    reason: the file's existing style (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: a **§X** pair naming the markers file is judged against its headings, not its markers — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: discarded
    reason: no such pair exists anywhere, and it fails loudly as unresolved, never silently (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: create-strategy rows map specs/inbox.md and its siblings to seed templates — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: discarded
    reason: "contrived: this repository's inbox headings equal the template's (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: the word boundary rejects a heading ending in punctuation followed by a word character, a non-ASCII letter after an ASCII anchor, and underscore emphasis around a § — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: discarded
    reason: "contrived: none occurs in the corpus, and each fails loudly as unresolved"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: a greenfield --ref run whose installed ductus.md is stale writes the config before aborting, so its re-run is not greenfield and prompts — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: pre-existing and not made worse; the documented install path places a current bootstrap
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: the real bootstrap is walked by the host, as step 8's per-directory loop already is
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: the greenfield rule's newest-entry id is undefined when the active registry is empty — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: "contrived: the registry is never empty while migrations exist, and 027's AC22 already writes nothing then"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
---

# Review — 071-command-reference-split

## Summary

Reviewed in depth across all five dimensions, by seven parallel passes over the window 59dc7aa4..e6b43796, each reading the commit through git and loading the eleven rule files its slice named (security-backend, quality-cross, configuration-cross and performance-backend in full; the frontend files and the service-shape backend files by each rule's statement and Verification trigger, none firing on this code). The passes covered runtime correctness, security, scripts and CI, the shipped prose of all thirteen split commands and the installer, reuse, efficiency and simplicity, and the spec artifacts' claims. Moved text was read as normalized diffs (each original reference range against its reference file, heading markers normalized), since the move itself was proven byte-for-byte at implementation; every pointer was checked against its target heading. Findings: no MUST; one SHOULD, QUAL-GROUND-001 (Family 4 hardcoded the agent config dirs), fixed in the run by a registry-derived lib.sh helper shared with Family 17, so nothing fires now. 53 observations: 40 fixed in the run, 13 discarded with the owner's reasons (owner decisions 2026-10-26: fix all, discard nine; AC15 reworded; 027 reopened for the greenfield migration change). Fix rounds: b0b1f9d0 and 36b3ddb6 (round 1), 083c4b08 (round 2), b540581c (round 3). Rounds 1 and 2 were each re-read by an independent agent; re-read 1 found three SHOULD-level sentences and a missing test, fixed in round 2; re-read 2 found no MUST and no SHOULD. Round 3 (notes only: one doc comment, three wording fixes, a plan row, two test assertions and mutation M6) was not independently re-read. The three runtime contract changes (exact pairs, headings outside fences and HTML comments, a word boundary outside pairs) were each measured over all 823 tracked .md files: no reference changed kind or resolution. scripts/mutation/command-reference-split.py holds six mutations, each killed by the test that names it. Two --ref=main scratch adoptions of all five agents ran: the first proved the reference files install with {project} substituted, land in no agent's command directory and are pruned when stale, and found eleven pre-071 installer defects (task 20, fixed); the second, on a greenfield project, proved those fixes and ran the installed hook. examined is 98 of the 141 scope entries. Not read, and not counted: the 42 generated mirrors under .claude/commands/ductus/, .pi/prompts/, .pi/extensions/ and .ductus/reference/ (the generator's --check reports them in sync and run-all's check-zero passes; the Pi bridge mirror was compared with its source); framework/bootstrap/govern.md, compared byte for byte with ductus.md rather than read; and, under the two scenario pattern entries, the 026 and 067 scenarios 071 did not change.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: installer step 8 never said enforce-manifest's expected and pinned lists are paths relative to the enforced directory, so a literal host deleted every reference file or a pinned one — `framework/bootstrap/ductus.md` — **fixed**
- convention: review's Scope Boundaries named the Groom decision tree bare, which the location line sent to review's own reference — `framework/commands/review.md` — **fixed**
- convention: review and groom stated their reference location twice — `framework/commands/review.md` — **fixed**
- claim: analyze's reference took the layout from a cli-config-dir in session.toml that may be absent and names the recorded agent, not necessarily the running one — `framework/reference/analyze.md` — **fixed**
- claim: audit's reference named .ductus/constitution.md, which exists nowhere audit's reference is installed — `framework/reference/audit.md` — **fixed**
- claim: audit's reference described Families 16 and 39 without 071's reference arm and cross-document judgment — `framework/reference/audit.md` — **fixed**
- claim: status's reference cited the installer's §Project Configuration without naming ductus.md — `framework/reference/status.md` — **fixed**
- claim: groom's and review's Where paragraphs said the command is always loaded, false for readers arriving from other files — `framework/reference/groom.md` — **fixed**
- claim: every reference's Where paragraph read a § beside another file's path as that file's heading, though constitution markers sit on such lines — `framework/reference/analyze.md` — **fixed**
- claim: the constitution's §runtime-host-integration sentence said a runtime-backed run never loads the reference — `framework/constitution.md` — **fixed**
- claim: three one-restart sentences were false for Pi once its bridge waits for the archive — `framework/bootstrap/ductus.md` — **fixed**
- claim: Pre-flight Checks did not tell the host to note config and spec-root absence, which the greenfield migration rule reads — `framework/bootstrap/ductus.md` — **fixed**
- claim: step 8's default *.md glob did not defer to each layout's cleanup glob, so Pi's flat prompts/ could be pruned by*.md — `framework/bootstrap/ductus.md` — **fixed**
- dead: docs/analyze.md linked two moved anchors and placed the promotion criteria in the command file — `docs/analyze.md` — **fixed**
- claim: AGENTS.md's pre-commit gotcha cited stale hook line numbers and omitted .ductus/reference/ — `AGENTS.md` — **fixed**
- claim: AGENTS.md's new-command entry pointed at installer-command-parity.sh's excl block, retired for scripts/maintainer-only-commands.txt — `AGENTS.md` — **fixed**
- convention: AGENTS.md's Project Structure and README's layout omitted framework/reference/ — `AGENTS.md` — **fixed**
- claim: Family 4 never implemented AC4's $ARGUMENTS check for reference files — `scripts/audit/placeholder-roundtrip.sh` — **fixed**
- claim: Family 39's unresolved-cross message was false for an explicit pair and named only the first word of a multi-word anchor — `scripts/audit/shipped-anchor-resolution.sh` — **fixed**
- claim: Family 39's header said a line naming one document qualifies every § on it — `scripts/audit/shipped-anchor-resolution.sh` — **fixed**
- other: Family 16 gave an orphan reference two contradictory findings and named a retired excl list — `scripts/audit/installer-command-parity.sh` — **fixed**
- other: lint-tool-coverage --help stopped mid-sentence — `scripts/lint-tool-coverage.sh` — **fixed**
- bug: resolve-anchor counted headings inside fenced blocks and HTML comments, so a pointer could resolve to an example record's heading — `runtime/src/primitives/resolve_anchor.rs` — **fixed**
- bug: a **§X** pair resolved by prefix, so a pointer longer than its heading passed — `runtime/src/primitives/resolve_anchor.rs` — **fixed**
- bug: outside a pair a heading matched inside a word (§Groundingx against Grounding) — `runtime/src/primitives/resolve_anchor.rs` — **fixed**
- bug: check-step-references resolved step 5 of **§X** against the command's own step 5, so implement.md's three Instructions pointers passed silently — `runtime/src/primitives/check_step_references.rs` — **fixed**
- claim: resolve-anchor's module doc, ordering comment and heading-collector doc described three kinds and a longest-first order that never decided a match — `runtime/src/primitives/resolve_anchor.rs` — **fixed**
- test: a pair on a line naming only its document, and a heading in an HTML comment, were untested — `runtime/src/primitives/resolve_anchor.rs` — **fixed**
- claim: the schema's resolve-anchor and check-corpus-links docs omitted the pair form and the .ductus/reference/ exclusion — `runtime/src/schema/primitives.rs` — **fixed**
- claim: the enforce-manifest, apply-manifest, parity and orphaned-references docs and comments described callers and modes that had changed — `runtime/src/primitives/enforce_manifest.rs` — **fixed**
- claim: 071's plan said the implement-basic golden moved by shas only and that ductus-basic was re-blessed — `specs/071-command-reference-split/plan.md` — **fixed**
- claim: 071's plan cited line numbers 071 itself moved, and its Affected Files omitted files the change touched — `specs/071-command-reference-split/plan.md` — **fixed**
- claim: 071's AC15 said the fixture prunes a stale reference, which the test does itself (reworded, owner decision 2026-10-26) — `specs/071-command-reference-split/spec.md` — **fixed**
- claim: 071's spec gave a citing-file-fallback example that does not use it, and listed check-step-references as unaffected — `specs/071-command-reference-split/spec.md` — **fixed**
- claim: 071's tasks 13, 14 and 18 recorded verifications that did not run as stated (dated corrections added) — `specs/071-command-reference-split/tasks.md` — **fixed**
- convention: two 067 scenarios carried § references the checker reads as unresolved markers — `specs/067-analyze-and-audit-checks/scenarios/criterion-negated-creation-phrasing.md` — **fixed**
- claim: 067's anchor-reference-kinds, AC2 and data model still described three kinds and longest-first matching — `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md` — **fixed**
- claim: 067's data model and pointers scenario omitted the .ductus/reference/ exclusion, the comment and word-boundary rules, and dated or scoped none of their measurements — `specs/067-analyze-and-audit-checks/data-model.md` — **fixed**
- claim: 026's AC6, AC8 and Behavior §4 placed Families 4 and 6's subjects in the command file — `specs/026-framework-self-audit/spec.md` — **fixed**
- claim: 026's scenarios credited Family 34's 83-to-80 to references never in that count, said Family 39 names the shipped document, and said log.md has no procedure — `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md` — **fixed**
- other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md` — **discarded**: contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)
- other: gen-claude-commands.sh writes and prunes through a symlink at .ductus/reference/ — `scripts/gen-claude-commands.sh` — **discarded**: the maintainer's own checkout, the same shape as the generator's two existing passes, and no trust boundary crossed (owner decision, 2026-10-26)
- other: check-corpus-links reads framework/reference/'s presence as a framework checkout, which an adopter's own directory of that name would match — `runtime/src/primitives/check_corpus_links.rs` — **discarded**: contrived: it costs only the repository-scope check of shipped copies, which an adopter's spec-corpus hook never runs (owner decision, 2026-10-26)
- convention: resolve-anchor and check-artifacts each parse the Shared Files table, and the bootstrap path literal appears three times — `runtime/src/primitives/resolve_anchor.rs` — **discarded**: their scopes differ on purpose (the plan records why), and merging them would narrow check-artifacts' suppression set (owner decision, 2026-10-26)
- convention: gen-claude-commands.sh's reference pass is a third inline copy of the generate, compare and prune blocks — `scripts/gen-claude-commands.sh` — **discarded**: the file's existing style; bash 3.2 has no namerefs, and refactoring it is its own change (owner decision, 2026-10-26)
- convention: Family 16's reference arm is not section-scoped and duplicates Family 35's row parser — `scripts/audit/installer-command-parity.sh` — **discarded**: contrived: no framework/reference row exists outside Shared Files (owner decision, 2026-10-26)
- convention: Family 18 restates its file label nine times — `scripts/audit/marker-list-parity.sh` — **discarded**: the file's existing style (owner decision, 2026-10-26)
- other: a **§X** pair naming the markers file is judged against its headings, not its markers — `runtime/src/primitives/resolve_anchor.rs` — **discarded**: no such pair exists anywhere, and it fails loudly as unresolved, never silently (owner decision, 2026-10-26)
- other: create-strategy rows map specs/inbox.md and its siblings to seed templates — `runtime/src/primitives/resolve_anchor.rs` — **discarded**: contrived: this repository's inbox headings equal the template's (owner decision, 2026-10-26)
- other: the word boundary rejects a heading ending in punctuation followed by a word character, a non-ASCII letter after an ASCII anchor, and underscore emphasis around a § — `runtime/src/primitives/resolve_anchor.rs` — **discarded**: contrived: none occurs in the corpus, and each fails loudly as unresolved
- other: a greenfield --ref run whose installed ductus.md is stale writes the config before aborting, so its re-run is not greenfield and prompts — `framework/bootstrap/ductus.md` — **discarded**: pre-existing and not made worse; the documented install path places a current bootstrap
- other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md` — **discarded**: the real bootstrap is walked by the host, as step 8's per-directory loop already is
- other: the greenfield rule's newest-entry id is undefined when the active registry is empty — `framework/bootstrap/ductus-procedure.md` — **discarded**: contrived: the registry is never empty while migrations exist, and 027's AC22 already writes nothing then

## Skipped passes

*None.*

## Unexamined governance

*None.*
