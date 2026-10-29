---
spec: 048-govern-acquired-runtime
last-run: 2026-10-29T23:33:22Z
reviewed-against: c8314288f7f381d68b369dbc59c0a474ef3c9834
diff-base: 41f807d6777579ec2ec31ba8f12e84d753d86d4f
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 62
scope: 466
skipped-passes: []
reviewed-digest:
  data-model.md: 29931ec6a3cb9225f2438a42e4459436f23cc93114d80ee0ac28f525b609b0a7
  scenarios/fetch-archive-reads-its-proxy-once.md: 1e1d9506af9ed4186032a1e205ff4eabf4045dacb0b5defab5023abbc95b0133
  scenarios/pin-is-readable-when-acquisition-needs-it.md: fb1368d6a85c6291628b23755e70841e55e1919235e7cbda6393f9e8402ae518
  scenarios/release-halves-publish-together.md: 0448ad2cb2df94e5d6b97ce4660345e26619af68d5f05da29d8b8dd8a011306f
  scenarios/retired-namespace-tools-are-off-limits.md: 0c5984d011f5d35b3685f05bc083e01d9aaa7bac4a31c0949e1d5bcd1f00627b
  scenarios/state-a-version-checks-the-pin.md: 40a4cbaf5dab4b5ee1127c4e808f70bbd08433415e952011ceb7c419a5e24e23
  scenarios/state-b-continues-in-session.md: 3be3a28c32a0f33f136b599afaf528c7ce976dcda9fa118a8619782847edfa75
blocking: false
dispositions:
  fixed: 49
  routed: 12
  discarded: 12
  undispositioned: 0
decisions:
  - key: "contract: prune-plan's apply writes plan.md before its reopen check can fail, against the plan's 'each error writes nothing' — `runtime/src/primitives/prune_plan.rs:264`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "contract: prune-tasks' all + apply walk rewrites tasks.md before its status read can fail — `runtime/src/primitives/prune_tasks.rs:177`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: prune-plan rebuilds the stored-decision key inline beside write_analysis::finding_key — `runtime/src/primitives/prune_plan.rs:305`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "contract: 041's spec, plan, data-model, scenarios, prune.md, analyze.md and the constitution's new text disagree on the reopen's timing, AC19's trigger, what prune touches, contributor knowledge's destinations and the design-record list's home — `specs/041-task-pruning/spec.md`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 052's review.md calls its 7 unread files unchanged since the full review; all 7 changed — `specs/052-spec-supersession-and-consolidation/review.md:32`"
    outcome: routed
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "claim: 052's spec says prune destroys tasks.md; since 041 it also removes plan sections — `specs/052-spec-supersession-and-consolidation/spec.md:65`"
    outcome: routed
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: common/mod.rs's stated reason for concurrent_sessions.rs's own spawn helper is inexact — `runtime/tests/common/mod.rs:22`"
    outcome: discarded
    reason: that file removes the same variables and then sets each child's identity; converging it on ductus_command is a refactor with no behavior at stake
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: a redundant `||` in a crlf assertion — `runtime/tests/crlf_preservation.rs:210`"
    outcome: discarded
    reason: cosmetic, and predates the window
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: the crlf prune-tasks cases do not assert that the rewrite happened — `runtime/tests/crlf_preservation.rs:196`"
    outcome: discarded
    reason: their inputs do trigger the rewrite, so a CRLF regression fails them; predates the window
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: the walker prune test's doc says the preview reports a missing plan without asserting it — `runtime/tests/walker.rs:829`"
    outcome: discarded
    reason: prune_plan's a_missing_plan_is_reported_not_an_error pins that; the walker test's subject is the walk continuing
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: runtime_binary, ensure_binary_built and the envelope loop are copied across test files — `runtime/tests/exec_subprocess.rs:18`"
    outcome: discarded
    reason: predates the window; consolidating test helpers is a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "dead code: dispatch_primitive's gate-confirm arm is unreachable in production — `runtime/src/interpreter/mod.rs:1083`"
    outcome: discarded
    reason: the registry-pin test requires every registry primitive to dispatch; removing the arm means exempting gate-confirm from that pin, a design change outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: the exec session report joins feature and scenario by hand beside SessionTarget::display — `runtime/src/interpreter/mod.rs:875`"
    outcome: discarded
    reason: a SessionPeer is not a SessionTarget, and a shared helper for two three-line joins would be premature
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "robustness: blob_text reads every tree lookup error as an absent file — `runtime/src/primitives/mod.rs:1488`"
    outcome: discarded
    reason: only a corrupt object store reaches it, and the repository's other reads would fail first
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: fetch-archive's sha256_hex hand-rolls the encoding analyze_subjects::hex shares, and hex lives in the analyze-record module — `runtime/src/primitives/fetch_archive.rs:655`"
    outcome: discarded
    reason: moving hex into mod.rs first is the prerequisite, a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "naming: check_stuck mixes project-root and work-tree paths, and its _rel names mean work-tree paths — `runtime/src/primitives/check_stuck.rs:48`"
    outcome: discarded
    reason: both are correct as written and pinned by the subdirectory test; a read_at_commit sibling is a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "errors: InvalidSlug and InvalidPath print a rejected value raw, a newline included — `runtime/src/primitives/mod.rs:332`"
    outcome: discarded
    reason: the value reaches callers inside a JSON error envelope, which escapes it
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "errors: a consolidation given only path or scenario is refused naming feature — `runtime/src/primitives/retarget_sessions.rs:146`"
    outcome: discarded
    reason: the refusal is correct and names an argument the call needs; which one it names first is cosmetic
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 058's review.md cites plan.md line 274 for 275 and says no 022 file changed — `specs/058-findings-route-at-discovery/review.md:83`"
    outcome: discarded
    reason: a review record is pinned to its reviewed-against sha and is regenerated by 058's next review
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 058's plan calls its write-analysis step 'now step 19' — `specs/058-findings-route-at-discovery/plan.md:347`"
    outcome: discarded
    reason: it records 058's own renumbering at the time, correct as history
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "inbox: an item's fix list predates 041's prune — `specs/inbox.md:31`"
    outcome: discarded
    reason: /groom re-reads each item when it routes it; the inbox is not a live artifact
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "readability: three nested em-dashes in §concurrent-features' adoption sentence — `framework/constitution.md:782`"
    outcome: discarded
    reason: readability only; the sentence's meaning is intact
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "claim: 067's Overview says no runtime behavior changes, against its 18 runtime commits — `specs/067-analyze-and-audit-checks/plan.md`"
    outcome: routed
    target: specs/067-analyze-and-audit-checks/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: Family 19's stated reason for its own implementation, no Rust toolchain in the audit job, is contradicted by runtime-release.yml's audit job — `scripts/audit/README.md`"
    outcome: routed
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "migration: an entry added after another but sorting before it at the same introduced_in is never run for an adopter whose last_applied names the later-sorting one — `scripts/audit/migration-coverage.sh`"
    outcome: routed
    target: specs/027-bootstrap-migration-registry/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "ci: runtime.yml does not trigger on the spec corpus that tests/mechanical_sweep_parity.rs reads — `.github/workflows/runtime.yml`"
    outcome: discarded
    reason: "owner's choice, stated in runtime.yml's header: a corpus edit can expose a disagreement between Family 19's two implementations but not create one, so a spec-only change waits for the next runtime change"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "contract: whether the per-agent configure commands carry the host-integration pointer is undecided — `framework/bootstrap/configure/claude.md`"
    outcome: routed
    target: specs/012-multi-agent-govern/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "errors: rewrite-spec-links checks the path escape before the name grammar, so a `to` failing both is refused as InvalidPath — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: contrived input; refused before any write either way, and the error names the more serious defect, the escape
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "concurrency: the read-all-then-write phase widens the window in which another session's edit between read and write is overwritten — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: "contrived timing: reading the 656 spec-root markdown files takes about 0.02 s; BE-TXN-002 concerns a service's persisted state, and §concurrent-features says sessions in one tree are not isolated from each other's edits"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "test: check_artifact_size.rs's two permission tests return silently as root (predates the window, 5b575782) — `runtime/src/primitives/check_artifact_size.rs`"
    outcome: routed
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "ci: lint-release-ordering.sh accepts a tag-check step with `if: false` or `continue-on-error: true` and a publish with `if: always()`, and refuses a check inside publish — `scripts/lint-release-ordering.sh`"
    outcome: discarded
    reason: "contrived: each accepted form is a deliberate disabling edit, not the reasonable-looking edit the lint exists for, and a condition it cannot evaluate would still pass an extended lint"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: check-release-tag.sh strips all whitespace from `version`, so a two-line file collapses — `scripts/check-release-tag.sh`"
    outcome: discarded
    reason: the collapsed value refuses every real tag, and Family 20 names the extra line in the same audit job publish needs, so nothing publishes and the cause is reported
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "concurrency: this repository's pre-commit hook stages generator outputs regenerated from the working tree, sweeping another session's in-flight files into a commit — `.githooks/pre-commit`"
    outcome: routed
    target: specs/017-derive-dont-ask/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "test: the Family 19 parity test returns ok unrun outside a git checkout — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: routed
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: since ced8c7ca a linked spec root is listed in `skipped` as not followed while its files are rewritten (walkdir reports the followed root entry as a link), so examined-and-nothing-pointed-here can never be reported — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: routed
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: 051 AC22 counts two cases the result names and misses the spec-root symbolic link ced8c7ca added to `skipped` — `specs/051-branch-scoped-spec-numbering/spec.md`"
    outcome: routed
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "doc: the reason a spec-root link is not rewritten, that the write would turn it into a copy, holds for a file link only, not a directory link — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: routed
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "doc: the fold reference's stop clause after the new symlink sentence reads as being about a symbolic link — `framework/reference/fold.md`"
    outcome: routed
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: the docs say a symbolic link to a markdown file is listed, while the code tests the link's own name for `.md` — `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md`"
    outcome: routed
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: check-orphaned-references reads a trailing-star glob as a missing path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: predates the window; a loud false finding, not a miss; no instance exists; the fix is not mechanical because bold-wrapped paths rely on the trim
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: check-orphaned-references would block on a FIFO at a referrer path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: contrived input
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: is_path_char is ASCII-only — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the names the check matches are ASCII by grammar; predates the window
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: extract-archive leaves a zip entry with no Unix mode at the umask default — `runtime/src/primitives/extract_archive.rs`"
    outcome: discarded
    reason: "extends 050's stored discard: no archive ductus extracts is a zip (048 AC23)"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "test: installed-file-modes.py's M7 and M8 survive off macOS while EXPECTED_SURVIVORS is empty — `scripts/mutation/installed-file-modes.py`"
    outcome: discarded
    reason: disclosed in the file; the lists are run on macOS, and run.py has no per-platform model
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: 008's single-host-appliance-rule-fit scenario argued from a since-removed Rationale — `specs/008-security-rules/scenarios/single-host-appliance-rule-fit.md`"
    outcome: discarded
    reason: "already corrected at HEAD: 554d0ee3 rewrote the scenario, which now states the rules carry no Rationale and argues from none"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: 021 AC7(d) and spec.md:94 still say a graceful-fallback pattern — `specs/021-runtime-boundary/spec.md`"
    outcome: discarded
    reason: the pointer is the command's fallback declaration, as 021's plan says; nothing became untrue
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
---

# Review — 048-govern-acquired-runtime

## Summary

Review of 048's reopen window, from 41f807d6 (the parent of the reopen fd2ca7ec) to 973a729c: 114 commits and 421 scope entries. Five read-only reviewers ran the five passes; their reports were read and checked, and the owner decided every finding and observation on 2026-10-29. Nothing remains firing, so the counts are zero. Each finding is listed below as an observation with its disposition: fixed in the commits after 973a729c whose messages name the finding and its proof, routed to a task its owning spec's gate reads, or discarded with its reason.

examined counts the 62 in-scope files the passes read in full: reviewer A 2 (resolve_anchor.rs, spec_links.rs), B 6 (Cargo.lock, Cargo.toml, two fixtures, implement-basic.jsonl, plan-basic.jsonl), C 35 (7 workflows and the hook, 11 scripts/audit files, gen-claude-commands.sh, three lints, 11 mutation lists, 2 script tests), D 18 (ductus.md, commands/review.md, constitution.md, 5 migration bodies, reference/review.md, 2 project templates, .ductus/config.toml, .gitignore, .mcp.json, AGENTS.md, CLAUDE.md, install.sh, version), and 048's spec.md. write-review derives scope at the reviewed-against sha, so it also counts files committed after the passes, the fix rounds and other sessions' work among them (2f61d67d, edd08387, 075's commits), which this review did not read. Not read in full, and so not counted: every framework/rules/*.md (read at HEAD as criteria, and the rules session's sweep within the window, not reviewed as subjects); the 209 files under specs/ outside 048 that the window changed (git diff --name-only --diff-filter=d 41f807d6..973a729c -- specs/), read as window diffs only; the large runtime files, read in part; framework/bootstrap/govern.md, checked byte-identical to ductus.md with cmp, not read; and the generated command copies, checked with scripts/gen-claude-commands.sh --check. The window held 66 commits that no recorded in-depth review covered, as counted by the session that ran the passes. 022 task 18's re-read 3 of cd06a227 and 047c6e9b is still owed (specs/022-deterministic-runtime/tasks.md) and was not performed here.

Independent read-only agents re-read each fix round that changed code or a durable contract. Round 1, two agents over 2f79a9ee to 75da4745: its findings were fixed in 2741fb12 and ed079593 or decided by the owner. Round 2, over 2741fb12, ed079593 and f395900f: fixed in ced8c7ca, which also reopened 052 for AC7. Round 3, over ced8c7ca, reported two SHOULDs and three NOTEs and no MUST. The owner routed all five to specs/051-branch-scoped-spec-numbering/tasks.md task 35, because another session's 051 work (edd08387) was rewriting the same files, so no fourth round ran under this review and 051's own review re-reads that fix. Round 2's export showed the Family 19 parity test passing unrun outside a git checkout (routed, 066 task 12). scripts/mutation/rewrite-spec-links.py ran 44 mutations, all killed, at ced8c7ca; fetch-archive-user-agent.py and claude-md-imports.py ran 2 of 2 killed each at f395900f.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- security: rewrite-spec-links wrote `to` into every referrer, the executable hook and CLAUDE.md's imports among them, holding it only to no traversal and containment (BE-INPUT-002) — fixed 2f79a9ee, 051 task 32 — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- claim: lint-tool-coverage.sh paired tool references with the per-step fallback AC17 removed, so it judged 0 references and exited 0 on the real tree (QUAL-CLAIM-001) — fixed 341149e3, 048 task 21 — `scripts/lint-tool-coverage.sh` — **fixed**
- migration: legacy-constitutions-region tied memory-routing-leaves-claude-md on introduced_in and sorted before it by id, so a --ref=main adopter between the two pushes never ran it — fixed 0f31a124 by the re-id, 071 task 24 — `framework/migrations.toml` — **fixed**
- bug: rewrite-spec-links read and wrote the spec root one file at a time, so a file it could not read stopped the walk after earlier files were re-pointed — fixed c1ca45b5, 051 task 33 — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- claim: AC15's tag clause was untrue — Family 20 leaves the tag out and no release step compared it, so a mistyped tag published the crate — fixed bcc26682, 048 task 22 — `specs/048-govern-acquired-runtime/spec.md` — **fixed**
- claim: AC9 ticked while Antigravity's seed has never carried the store probe — fixed 20a6def1 by annotation, 048 task 23 — `specs/048-govern-acquired-runtime/spec.md` — **fixed**
- dead code: ductus-procedure.md's runtime tip rendered only on a failed acquisition, which halts before the file is read — fixed 2ce9e62d, 048 task 24 — `framework/bootstrap/ductus-procedure.md` — **fixed**
- claim: §Permission Setup's lead said each seed entry grants only what a step runs while seeds keep `ls` and `command -v` — fixed 86503cfb, 048 task 25 — `framework/bootstrap/ductus.md` — **fixed**
- claim: State A's version check said the seed pre-authorizes its probe, untrue for a `[runtime] path` — fixed 86503cfb, 048 task 26 — `framework/bootstrap/ductus.md` — **fixed**
- claim: the pointer's prompt was said to occur on OpenCode, which allows an unmatched command until /configure adds its catch-all `ask` — fixed 31d064e6, 048 task 27 — `framework/bootstrap/ductus.md` — **fixed**
- contract: the review reference had reviewers hand-edit a record /review regenerates whole — fixed b82c1c86 — `framework/reference/review.md` — **fixed**
- contract: the review reference's sample output sent three SHOULDs to analyze and done, while the gate holds on an unwaived SHOULD — fixed 8815e460 — `framework/reference/review.md` — **fixed**
- ci: runtime.yml's paths missed seven repository files its tests read — fixed 585e8103 — `.github/workflows/runtime.yml` — **fixed**
- ci: runtime.yml's header sent other edits to the retired markdown-only workflow — fixed f84acb98 — `.github/workflows/runtime.yml` — **fixed**
- ci: runtime-release.yml's job diagram left out the cargo-audit gate — fixed b31c3b5a — `.github/workflows/runtime-release.yml` — **fixed**
- doc: apply-manifest's modes-kept doc left out a linked update destination, neither set nor listed — fixed c399913e — `runtime/src/schema/primitives.rs` — **fixed**
- doc: the Pi configure command called a Pi adopter's checkout pip-installed — fixed 40358729 — `framework/bootstrap/configure/pi.md` — **fixed**
- claim: 022's govern-bootstrap scenario said the release ships .zip on Windows and stated the optional runtime as current — fixed 2c5865f0 — `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — **fixed**
- claim: 038's lock-ordering scenario and task required a Rationale the rule format no longer has — fixed 73f729f4 — `specs/038-concurrency-backend-rules/scenarios/lock-the-parent-before-rewriting-its-children.md` — **fixed**
- contract: check-promotion-coverage's empty-sections refusal was classified breaking only in its commit message (BE-COMPAT-001) — fixed 2b021345 in 067's Technical Decisions — `specs/067-analyze-and-audit-checks/plan.md` — **fixed**
- claim: 067's Overview says no runtime behavior changes, against its 18 runtime commits — `specs/067-analyze-and-audit-checks/plan.md` — **routed** to `specs/067-analyze-and-audit-checks/tasks.md`
- claim: Family 19's stated reason for its own implementation, no Rust toolchain in the audit job, is contradicted by runtime-release.yml's audit job — `scripts/audit/README.md` — **routed** to `specs/066-audit-records-and-the-completion-gate/tasks.md`
- ci: generators.yml's check job was the only job without timeout-minutes — fixed 6ca9bfd9, re-sized in ed079593 — `.github/workflows/generators.yml` — **fixed**
- claim: by-hand advice ran a bare `ductus derive-dependencies`, whatever PATH resolves, at six sites — fixed 75da4745; 022 task 21's item notes it (422e21bc) — `framework/commands/target.md` — **fixed**
- migration: an entry added after another but sorting before it at the same introduced_in is never run for an adopter whose last_applied names the later-sorting one — `scripts/audit/migration-coverage.sh` — **routed** to `specs/027-bootstrap-migration-registry/tasks.md`
- contract: skipping a non-UTF-8 spec-root file let the fold retire the directory with that file's link dangling, leaving 051 AC6 and 052 AC7 untrue — fixed 2741fb12 by stopping the run; the docs, data model, fold reference and MCP description describe the stop — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- claim: the scenario's Context said a non-UTF-8 file can be committed past the hook, which refuses one — fixed 2741fb12 — `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md` — **fixed**
- claim: the kept-tails doc, fold reference and resumption rows did not say a re-run cannot list the kept tails of files a failed run re-pointed — fixed 2741fb12 — `framework/reference/fold.md` — **fixed**
- claim: the target scenario and run's doc did not name the visible-ASCII bound on the feature half — fixed 2741fb12 — `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-target-is-held-to-the-name-grammar.md` — **fixed**
- record: 051's plan did not record the read phase and the grammar check — fixed 2741fb12 — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- record: 051 task 33's done-when did not state what was done — fixed 2741fb12 — `specs/051-branch-scoped-spec-numbering/tasks.md` — **fixed**
- claim: tests/mcp.rs's comment named the retired fallback lint — fixed 2741fb12 — `runtime/tests/mcp.rs` — **fixed**
- record: 048 task 21's done-when unmet — 021's plan still described the proximity scan — fixed ed079593 — `specs/021-runtime-boundary/plan.md` — **fixed**
- record: 048 task 22's done-when unmet — 048's plan and data model said a family checks all four version artifacts — fixed ed079593 — `specs/048-govern-acquired-runtime/plan.md` — **fixed**
- bug: lint-release-ordering.sh counted a commented-out tag-check step as present — fixed ed079593 with test N — `scripts/lint-release-ordering.sh` — **fixed**
- test: test-lint-release-ordering.sh's case M passed when its edit matched nothing — fixed ed079593 — `scripts/tests/test-lint-release-ordering.sh` — **fixed**
- doc: lint-release-ordering.sh's comment counted two properties — fixed ed079593 — `scripts/lint-release-ordering.sh` — **fixed**
- ci: runtime.yml missed framework/constitution.md, read by resolve_anchor's corpus test — fixed ed079593 — `.github/workflows/runtime.yml` — **fixed**
- ci: runtime.yml does not trigger on the spec corpus that tests/mechanical_sweep_parity.rs reads — `.github/workflows/runtime.yml` — **discarded**: owner's choice, stated in runtime.yml's header: a corpus edit can expose a disagreement between Family 19's two implementations but not create one, so a spec-only change waits for the next runtime change
- ci: generators.yml's 20-minute timeout was not sized by 022's own method — fixed ed079593 at 15 minutes — `.github/workflows/generators.yml` — **fixed**
- ci: runtime-release.yml's diagram left out verify-published and verify-installer — fixed ed079593 — `.github/workflows/runtime-release.yml` — **fixed**
- claim: 022's govern-bootstrap signpost did not cover its first edge case — fixed ed079593 — `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — **fixed**
- test: test-lint-tool-coverage.sh's case B let one file's pointer excuse another's — fixed ed079593 — `scripts/tests/test-lint-tool-coverage.sh` — **fixed**
- doc: lint-tool-coverage.sh's header did not state its subject, framework/commands/*.md alone — fixed ed079593 — `scripts/lint-tool-coverage.sh` — **fixed**
- contract: whether the per-agent configure commands carry the host-integration pointer is undecided — `framework/bootstrap/configure/claude.md` — **routed** to `specs/012-multi-agent-govern/tasks.md`
- claim: 051's and 023's plans and 022's coverage-residue-cleanup scenario said the lint checks tool names against commands — fixed ed079593 — `specs/022-deterministic-runtime/scenarios/coverage-residue-cleanup.md` — **fixed**
- record: 071 task 24 did not state exactly when an adopter's last_applied can name the old migration id — fixed ed079593 — `specs/071-command-reference-split/tasks.md` — **fixed**
- errors: rewrite-spec-links checks the path escape before the name grammar, so a `to` failing both is refused as InvalidPath — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: contrived input; refused before any write either way, and the error names the more serious defect, the escape
- concurrency: the read-all-then-write phase widens the window in which another session's edit between read and write is overwritten — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: contrived timing: reading the 656 spec-root markdown files takes about 0.02 s; BE-TXN-002 concerns a service's persisted state, and §concurrent-features says sessions in one tree are not isolated from each other's edits
- test: two permission tests in rewrite_spec_links.rs returned silently as root, so a root run reported them ok unrun — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- test: check_artifact_size.rs's two permission tests return silently as root (predates the window, 5b575782) — `runtime/src/primitives/check_artifact_size.rs` — **routed** to `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`
- ci: lint-release-ordering.sh accepts a tag-check step with `if: false` or `continue-on-error: true` and a publish with `if: always()`, and refuses a check inside publish — `scripts/lint-release-ordering.sh` — **discarded**: contrived: each accepted form is a deliberate disabling edit, not the reasonable-looking edit the lint exists for, and a condition it cannot evaluate would still pass an extended lint
- robustness: check-release-tag.sh strips all whitespace from `version`, so a two-line file collapses — `scripts/check-release-tag.sh` — **discarded**: the collapsed value refuses every real tag, and Family 20 names the extra line in the same audit job publish needs, so nothing publishes and the cause is reported
- concurrency: this repository's pre-commit hook stages generator outputs regenerated from the working tree, sweeping another session's in-flight files into a commit — `.githooks/pre-commit` — **routed** to `specs/017-derive-dont-ask/tasks.md`
- contract: rewrite-spec-links passed over a symbolic link under the spec root, file or directory, unlisted, so a link in it dangled behind `skipped: []`, leaving 051 AC6 and 052 AC7 untrue for it — fixed ced8c7ca by listing it in `skipped`; 052 reopened, task 2 — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- test: no test pinned that an unlistable spec-root directory's error names it — fixed ced8c7ca, M45 — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- doc: the fold reference's 'So is every other markdown file' sentence read as the opposite of its next clause — fixed ced8c7ca — `framework/reference/fold.md` — **fixed**
- claim: the read-phase scenario said nothing would report an unreadable spec-root file, while the hook's check-corpus-links does — fixed ced8c7ca — `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md` — **fixed**
- ci: runtime.yml did not trigger on the three project templates prune_project.rs's compiled_templates_match_the_framework reads (2f61d67d) — fixed ced8c7ca — `.github/workflows/runtime.yml` — **fixed**
- test: the Family 19 parity test returns ok unrun outside a git checkout — `runtime/tests/mechanical_sweep_parity.rs` — **routed** to `specs/066-audit-records-and-the-completion-gate/tasks.md`
- claim: 048's data model said Family 20 treats a newest-tag lag as expected, while it reads no tag — fixed ced8c7ca — `specs/048-govern-acquired-runtime/data-model.md` — **fixed**
- bug: since ced8c7ca a linked spec root is listed in `skipped` as not followed while its files are rewritten (walkdir reports the followed root entry as a link), so examined-and-nothing-pointed-here can never be reported — `runtime/src/primitives/rewrite_spec_links.rs` — **routed** to `specs/051-branch-scoped-spec-numbering/tasks.md`
- claim: 051 AC22 counts two cases the result names and misses the spec-root symbolic link ced8c7ca added to `skipped` — `specs/051-branch-scoped-spec-numbering/spec.md` — **routed** to `specs/051-branch-scoped-spec-numbering/tasks.md`
- doc: the reason a spec-root link is not rewritten, that the write would turn it into a copy, holds for a file link only, not a directory link — `runtime/src/primitives/rewrite_spec_links.rs` — **routed** to `specs/051-branch-scoped-spec-numbering/tasks.md`
- doc: the fold reference's stop clause after the new symlink sentence reads as being about a symbolic link — `framework/reference/fold.md` — **routed** to `specs/051-branch-scoped-spec-numbering/tasks.md`
- claim: the docs say a symbolic link to a markdown file is listed, while the code tests the link's own name for `.md` — `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md` — **routed** to `specs/051-branch-scoped-spec-numbering/tasks.md`
- bug: check-orphaned-references reads a trailing-star glob as a missing path — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: predates the window; a loud false finding, not a miss; no instance exists; the fix is not mechanical because bold-wrapped paths rely on the trim
- robustness: check-orphaned-references would block on a FIFO at a referrer path — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: contrived input
- robustness: is_path_char is ASCII-only — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the names the check matches are ASCII by grammar; predates the window
- robustness: extract-archive leaves a zip entry with no Unix mode at the umask default — `runtime/src/primitives/extract_archive.rs` — **discarded**: extends 050's stored discard: no archive ductus extracts is a zip (048 AC23)
- test: installed-file-modes.py's M7 and M8 survive off macOS while EXPECTED_SURVIVORS is empty — `scripts/mutation/installed-file-modes.py` — **discarded**: disclosed in the file; the lists are run on macOS, and run.py has no per-platform model
- claim: 008's single-host-appliance-rule-fit scenario argued from a since-removed Rationale — `specs/008-security-rules/scenarios/single-host-appliance-rule-fit.md` — **discarded**: already corrected at HEAD: 554d0ee3 rewrote the scenario, which now states the rules carry no Rationale and argues from none
- claim: 021 AC7(d) and spec.md:94 still say a graceful-fallback pattern — `specs/021-runtime-boundary/spec.md` — **discarded**: the pointer is the command's fallback declaration, as 021's plan says; nothing became untrue

## Skipped passes

*None.*

## Unexamined governance

*None.*
