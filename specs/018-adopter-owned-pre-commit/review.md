---
spec: 018-adopter-owned-pre-commit
last-run: 2026-11-19T13:43:35Z
reviewed-against: b2bd79ef1cbeb54ae8d84cd3e2d7e0d40f4f718e
diff-base: 27588f3052fa359b114945518991248f3f74b579
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 5
scope: 6
unread:
  - framework/bootstrap/govern.md
excluded: 158
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
criteria-judged: 14
criteria-met: 14
criteria-verdicts:
  - label: AC1
    digest: f7f289b7b43eae383bf32f4323af4694c53744a83f0b52356df65a5db8f7b857
  - label: AC2
    digest: bf2a7f67f33561263c5ca97cb1e708a868346376ec9f0e85c266aa75ba92053b
  - label: AC3
    digest: 1ec7a1ef5d61a807adcd7f240218399e104a9ed19612455441f7b727d4515d74
  - label: AC4
    digest: 24c1461439dc668ed02a3ab4424101d774e5b161a65a424892258bff07525496
  - label: AC5
    digest: 0e771cc394dcc8ea6691c07631960019f1a341b88650081abcbf1d5f93fd47a0
  - label: AC6
    digest: 063f10e861e6b6e10d93a35354be741e854bc09c423efbe3792f5b398e9dcd07
  - label: AC7
    digest: e2e8f4a00b583c35e65e67357a282062636202cf297e854c8687c27437eceba8
  - label: AC8
    digest: 0b6d938e212ca16a0bb76ea4af607b78723e6264805c7e965bacc421f31b2b90
  - label: AC9
    digest: 4972c0c6bf408665053c21ed80383ae2d4826b4dce458f638545325183b3652a
  - label: AC10
    digest: b1016af9a6e12f601793dd782f6c845049284e5ebfac3daa8b72857ca2325b69
  - label: AC11
    digest: 97441b132ebd9863a28f03dba7f6630c80b6049160b0562fd17608950e416a90
  - label: AC12
    digest: ee9d2140d1b0806d3b4f371d1d8bcf4f24f3a92cd4d10e8c59cdbea97715f81f
  - label: AC13
    digest: f4fedd6e839d9fc14a7f53df781561f3ce8bc2b832095f29a163e877f7ea7165
  - label: AC14
    digest: c63051702658ee0d0e98aa30457fb4d4417c3c4bf193c9f497c6a24d4f84b95c
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: quality — Frontmatter Migration walks the literal `specs/` and tests `specs/` existence rather than the configured `[paths] specs-root`, so a project with a renamed spec root and bold-prefix specs is skipped silently — `framework/bootstrap/ductus-procedure.md`
    outcome: waived
    reason: The Frontmatter Migration serves pre-017 projects, which predate `[paths] specs-root` (040), so the literal `specs/` is their spec root; 029's review waived the same by this reason.
    decided-at: 2026-11-19T13:43:34Z
    decided-by: rule
  - key: convention — the new Frontmatter Migration containment screen has the agent run `realpath` by hand, a shell step where a runtime primitive would be the sanctioned form, though `/bin/realpath` exists on macOS and no primitive covers it — `framework/bootstrap/ductus-procedure.md`
    outcome: waived
    reason: No runtime primitive resolves a canonical path today, and the step is one comparison the agent makes with `realpath`; a primitive would be a new feature, not a fix to this change.
    decided-at: 2026-11-19T13:43:34Z
    decided-by: rule
---

# Review — 018-adopter-owned-pre-commit

## Summary

Window `27588f30` to `9b034410`, base-source `reopen`: `0c9f4341` reopened 018 with 017 (owner decision: the spec-017 hook migration is retired), and `0e33dc54` retired it in the four bootstrap files. A review is owed because the last record predates the criteria step (066 task 27), so all 14 criteria were judged. One shared run, from a clean worktree of `origin/main`, reviewed 017's and 018's window; 017's record (`b2bd79ef`) gives the same passes. Scope: 6 entries, examined 5. `framework/bootstrap/govern.md` is unread: it is byte-identical to `framework/bootstrap/ductus.md` (`cmp`), which the security pass read whole and the other passes read at its `## Instructions` section, the one `0e33dc54` touched. The security pass read the three security rule files, the three distinct bootstrap files and both specs' artifacts whole. The other passes read the nine other rule files, `framework/bootstrap/ductus-procedure.md`, the hook and the artifacts whole. Security found one MUST, BE-INPUT-004 at high confidence: the Frontmatter Migration in `framework/bootstrap/ductus-procedure.md` rewrote globbed spec files in place with no containment check. It was fixed in `892c1b07` (owner decision): §Walk resolves the canonical project root, `specs/` and each match, and goes on only when `specs/` lies under the root and the match under `specs/`, by whole path components. The re-run found no gap. One low-confidence BE-INPUT-004 stands on `[runtime] path` in `framework/bootstrap/ductus.md`: the key names a binary anywhere, so no base directory exists to contain it. Observations: four fixed by the rule (the spec's pointer to the retired §Migration, the plan's risk surface, the plan's ladder count and CI wording, and §Hook Installation's two read-only checks), and two waived by the rule (the migration's literal `specs/`, and its hand-run `realpath`). The one stored decision no longer fires. Criteria step: 14 of 14 met. AC1, AC7, AC8 and AC13 were judged on their Superseded annotations. The shipped stub was driven through the inner hook on a fixture. AC11 and AC14 were judged from the procedure text, since a review cannot drive a real `/ductus` run. Fix rounds: one, the shared fixes (committed in `892c1b07`), re-read clean by an independent agent; it did not judge 017's regenerated `review.md`, which the round listed and is 017's record. The next round was empty.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

14 of 14 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: BE-INPUT-004 — The committed `[runtime] path` value is resolved, executed with `--version` and linked or copied to the pointer with no canonicalization or containment check

- **File**: `framework/bootstrap/ductus.md`
- **Rule**: User-supplied values MUST NOT be used directly in filesystem paths. Filesystem operations MUST resolve the canonical path and verify it falls within the expected base directory before opening the file.
- **Finding**: Unchanged: the key deliberately names a binary anywhere, so no expected base directory exists; 048's plan records the trust boundary for BE-INPUT-003 only.
- **Auto-fixable**: no
- **Suggested fix**: Record the trust boundary and waive with 048's reasoning, or canonicalize and require a regular executable file.

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: Judged on its Superseded annotation: `framework/bootstrap/hooks/ductus-pre-commit` carries `# managed-by: ductus` on line 2 (checked with `sed -n 2p`), runs `derive-dependencies`/`derive-references --write --staged` then `git add`s staged specs; fixture run of the hook staged a labelled spec, and `scripts/audit/adopter-shell-behavior.sh` exits 0.
- AC2 — met: `framework/bootstrap/hooks/pre-commit` invokes `"$hooks_dir/ductus-pre-commit"` with `hooks_dir` from `dirname "${BASH_SOURCE[0]}"` and has no `managed-by` sentinel (`grep -c` gives 0); a fixture run of the stub reached the inner hook and exited 0.
- AC3 — met: `framework/bootstrap/ductus.md` §Shared Files, subsection `ductus`-owned shared files (strategy: update), lists `framework/bootstrap/hooks/ductus-pre-commit` to `.githooks/ductus-pre-commit`.
- AC4 — met: `framework/bootstrap/ductus.md` §Shared Files, subsection Project-specific shared files (strategy: create), lists `framework/bootstrap/hooks/pre-commit` to `.githooks/pre-commit`.
- AC5 — met: `framework/bootstrap/ductus-procedure.md` §Hook Installation has the ladder with the four 018 cases (already wired, custom hooksPath, third-party system, no conflicts) plus 059's rewire case; no sentinel-detected branch remains and the section states the outer file is not treated as ductus-managed.
- AC6 — met: `framework/bootstrap/ductus-procedure.md` §Hook Installation, Manual integration snippet, names `./{P}.githooks/ductus-pre-commit`; no `./.githooks/pre-commit` snippet remains.
- AC7 — met: Judged on its Superseded annotation: the migration was retired by `0e33dc54`; `framework/bootstrap/ductus-procedure.md` §Hook Installation holds no migration subsection, and `spec.md` §Migration records the retirement. Class if read as unmet: superseded, `0e33dc54`.
- AC8 — met: Judged on its Superseded annotation: the migration is retired (`0e33dc54`), and the surviving half holds: the manifest row for `.githooks/pre-commit` is `create` strategy, so an adopter's added lines are untouched.
- AC9 — met: `framework/bootstrap/ductus-procedure.md` §Hook Installation case 5 runs `git config core.hooksPath {H}` and `chmod +x` on both hooks; manifest passes write both files; only `framework/bootstrap/hooks/ductus-pre-commit` carries the sentinel (`grep -c managed-by` gives 2 inner, 0 outer).
- AC10 — met: `specs/017-derive-dont-ask/spec.md` has a `> **Signpost (post-018):**` block right after the H1 naming the outer stub, `ductus-pre-commit`, `install.sh` deletion and AC21-AC23; its `dependencies: []` stays empty.
- AC11 — met: Judged on its narrowed annotation (fresh-install case only): the layout it describes is what the Hook Installation case 5 and manifest rows produce, and a fixture of the two shipped hook files ran end to end; `/ductus` itself cannot be run from a review, so the sandbox run is the recorded gate of `5842d6d6`, not re-executed here.
- AC12 — met: `framework/bootstrap/hooks/install.sh` is absent (`ls` fails), its two actions are inlined in `framework/bootstrap/ductus-procedure.md` §Hook Installation cases 1 and 5, and the remaining `install.sh` mentions in `framework/`, `scripts/` and `docs/` name the repository-root installer, a different file.
- AC13 — met: Judged on its Superseded annotation: `.ductus/scripts/` no longer exists, derivation is the `derive-dependencies` primitive, and `specs/017-derive-dont-ask/spec.md` and `specs/000-slash-commands/spec.md` both carry `dependencies: []` despite signpost blockquotes.
- AC14 — met: `framework/bootstrap/ductus-procedure.md` §Hook Installation, Precondition for a wired outcome (cases 1, 2 and 5), requires a non-comment line of `.githooks/pre-commit` naming `ductus-pre-commit`, else leaves `core.hooksPath` as found and reports the skip with the snippet; read, not run, since the ladder is an agent procedure.

## Observations

- convention — 018's Hook Installation logic paragraph says migration of pre-existing ductus-installed hooks "is handled separately (see §Migration below)", but §Migration records that migration as retired and the wired-outcome precondition is what handles such a file — `specs/018-adopter-owned-pre-commit/spec.md` — **fixed** (decided by the rule)
- convention — the plan's Overview names the migration code path as the risk surface with recovery "documented in §Edge Cases", but the migration is retired and the plan has no Edge Cases section — `specs/018-adopter-owned-pre-commit/plan.md` — **fixed** (decided by the rule)
- convention — the plan records the ladder as collapsing from 7 items to 4 and CI as running generators in dry-run, but the ladder in `framework/bootstrap/ductus-procedure.md` has 5 cases since 059 and CI runs the generators for real (017 AC24) — `specs/018-adopter-owned-pre-commit/plan.md` — **fixed** (decided by the rule)
- convention — Hook Installation describes the inner hook as holding the derivation orchestration plus `label-criteria`, omitting the two blocking read-only checks (`check-corpus-links`, `check-artifact-size --project-files --staged`) the shipped hook runs after the writes, so an adopter is not told a commit can be refused by them — `framework/bootstrap/ductus-procedure.md` — **fixed** (decided by the rule)
- quality — Frontmatter Migration walks the literal `specs/` and tests `specs/` existence rather than the configured `[paths] specs-root`, so a project with a renamed spec root and bold-prefix specs is skipped silently — `framework/bootstrap/ductus-procedure.md` — **waived**: The Frontmatter Migration serves pre-017 projects, which predate `[paths] specs-root` (040), so the literal `specs/` is their spec root; 029's review waived the same by this reason. (decided by the rule)
- convention — the new Frontmatter Migration containment screen has the agent run `realpath` by hand, a shell step where a runtime primitive would be the sanctioned form, though `/bin/realpath` exists on macOS and no primitive covers it — `framework/bootstrap/ductus-procedure.md` — **waived**: No runtime primitive resolves a canonical path today, and the step is one comparison the agent makes with `realpath`; a primitive would be a new feature, not a fix to this change. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/implement.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `.github/workflows/runtime-acquisition.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime-release.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime.yml` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `.pi/prompts/ductus-amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-implement.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/developing-ductus.md` — 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate
- `docs/runtime.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/slash-commands.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/commands/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `framework/commands/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/constitution.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/migrations.toml` — 027-bootstrap-migration-registry
- `framework/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/CHANGELOG.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 073-decided-rule-departures-are-recorded
- `runtime/README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `runtime/src/interpreter/mod.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/interpreter/payload.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/main.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/classify_spec_edit.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/compute_review_scope.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/criterion_verdicts.rs` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/derive_routing_candidates.rs` — 079-review-and-analysis-work-converges
- `runtime/src/primitives/invalidate_review.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/line_citations.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/mod.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/descriptions.rs` — 000-slash-commands, 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/schema/extensions.rs` — 022-deterministic-runtime, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/primitives.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/fixtures/implement-basic/.govern.session.toml` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/implement-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/review-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/implement-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/review-basic.jsonl` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/mcp.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/review_command.rs` — 066-audit-records-and-the-completion-gate
- `scripts/audit/README.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/done-spec-criteria.sh` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/audit/promotion-coverage.sh` — 067-analyze-and-audit-checks
- `scripts/mutation/criteria-in-review-066.py` — 000-slash-commands, 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `scripts/mutation/fix-round-record.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/line-citation-strip-066.py` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/mutation/reopen-measure.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/reviews-follow-what-changed.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/scope-attribution.py` — 066-audit-records-and-the-completion-gate
- `scripts/tests/test-migration-coverage.sh` — 027-bootstrap-migration-registry
- `specs/000-slash-commands/analysis.md` — 000-slash-commands
- `specs/000-slash-commands/plan.md` — 000-slash-commands
- `specs/000-slash-commands/review.md` — 000-slash-commands
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/scenarios/validation-gates.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/003-bootstrap-automation/analysis.md` — 003-bootstrap-automation
- `specs/003-bootstrap-automation/plan.md` — 003-bootstrap-automation, 012-multi-agent-govern
- `specs/003-bootstrap-automation/review.md` — 003-bootstrap-automation
- `specs/003-bootstrap-automation/spec.md` — 003-bootstrap-automation, 012-multi-agent-govern
- `specs/003-bootstrap-automation/tasks.md` — 003-bootstrap-automation, 007-govern-workflow, 012-multi-agent-govern, 022-deterministic-runtime, 027-bootstrap-migration-registry, 056-bootstrap-archive-boundary-split
- `specs/007-govern-workflow/analysis.md` — 007-govern-workflow
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/007-govern-workflow/review.md` — 007-govern-workflow
- `specs/007-govern-workflow/scenarios/ductus-self-update-precheck.md` — 007-govern-workflow
- `specs/007-govern-workflow/spec.md` — 003-bootstrap-automation, 007-govern-workflow
- `specs/007-govern-workflow/tasks.md` — 003-bootstrap-automation, 007-govern-workflow, 012-multi-agent-govern, 027-bootstrap-migration-registry, 056-bootstrap-archive-boundary-split
- `specs/012-multi-agent-govern/analysis.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/plan.md` — 003-bootstrap-automation, 012-multi-agent-govern
- `specs/012-multi-agent-govern/review.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/spec.md` — 003-bootstrap-automation, 012-multi-agent-govern
- `specs/012-multi-agent-govern/tasks.md` — 003-bootstrap-automation, 007-govern-workflow, 012-multi-agent-govern, 027-bootstrap-migration-registry, 056-bootstrap-archive-boundary-split
- `specs/013-text-first-artifacts/analysis.md` — 013-text-first-artifacts
- `specs/013-text-first-artifacts/plan.md` — 013-text-first-artifacts, 026-framework-self-audit
- `specs/013-text-first-artifacts/spec.md` — 013-text-first-artifacts, 026-framework-self-audit
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/017-derive-dont-ask/data-model.md` — 017-derive-dont-ask, 056-bootstrap-archive-boundary-split
- `specs/017-derive-dont-ask/review.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/spec.md` — 017-derive-dont-ask
- `specs/020-code-review/analysis.md` — 020-code-review
- `specs/020-code-review/data-model.md` — 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `specs/020-code-review/plan.md` — 020-code-review
- `specs/020-code-review/review.md` — 020-code-review, 022-deterministic-runtime
- `specs/020-code-review/spec.md` — 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/every-mcp-argument-has-a-cli-form.md` — 003-bootstrap-automation, 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` — 020-code-review, 022-deterministic-runtime
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/tasks.md` — 003-bootstrap-automation, 022-deterministic-runtime
- `specs/026-framework-self-audit/analysis.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-10-migration-coverage.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 013-text-first-artifacts, 026-framework-self-audit
- `specs/027-bootstrap-migration-registry/analysis.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/plan.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/review.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/a-first-adoption-runs-no-migration.md` — 007-govern-workflow, 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/a-projects-own-constitution-is-not-relocated.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/seed-files-keep-no-literal-placeholder.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/spec.md` — 027-bootstrap-migration-registry, 056-bootstrap-archive-boundary-split
- `specs/027-bootstrap-migration-registry/tasks.md` — 003-bootstrap-automation, 007-govern-workflow, 012-multi-agent-govern, 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire, 056-bootstrap-archive-boundary-split
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/048-govern-acquired-runtime/data-model.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 022-deterministic-runtime, 048-govern-acquired-runtime, 062-concurrent-session-targets, 067-analyze-and-audit-checks
- `specs/056-bootstrap-archive-boundary-split/analysis.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/plan.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/review.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/scenarios/the-walker-places-every-archive-half-section.md` — 017-derive-dont-ask, 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/spec.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/tasks.md` — 003-bootstrap-automation, 007-govern-workflow, 012-multi-agent-govern, 017-derive-dont-ask, 027-bootstrap-migration-registry, 056-bootstrap-archive-boundary-split
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/062-concurrent-session-targets/data-model.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/review.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/scenarios/first-writers-create-the-ignore-file-once.md` — 048-govern-acquired-runtime, 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/spec.md` — 048-govern-acquired-runtime, 062-concurrent-session-targets
- `specs/066-audit-records-and-the-completion-gate/analysis.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/067-analyze-and-audit-checks/review.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md` — 048-govern-acquired-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/spec.md` — 048-govern-acquired-runtime, 067-analyze-and-audit-checks
- `specs/073-decided-rule-departures-are-recorded/plan.md` — 073-decided-rule-departures-are-recorded
- `specs/079-review-and-analysis-work-converges/analysis.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/plan.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/review.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
