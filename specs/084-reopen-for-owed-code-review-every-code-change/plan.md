# 084 — Reopen for owed code, review every code change Plan

Implements [084 — Reopen for owed code, review every code change](spec.md).

## Overview

The work lands in the order the spec's Resolved Questions fix: the reopen rule first, then the sweep of what states the old one, then the machinery that owes a review to every code change, then the retirement of what nothing calls.

- **The rule.** A shared owed-work helper and a `check-owed-work` primitive answer whether a spec holds an unchecked box or an unticked criterion. `/ductus:amend` and `/ductus:groom` reopen a `done` spec from that answer, and the constitution states the rule in place of the third back-edge.
- **The sweep.** Every live site that states the old rule is corrected. Under the new rule this leaves each `done` spec `done`.
- **Reviews owed wherever code changed.** The review window starts where the last review ended. Code no spec claims leaves open specs' scopes for a project record, `specs/review.md`, which is seeded at the update. A `check-unreviewed-code` primitive reports the code no review is going to read, and runs from a shipped pre-push hook, from the CI template and from `/ductus:status`. `/ductus:review` reviews a `done` spec's code as the whole review, and asks the owner before a ticked criterion changes.
- **Retirement and delivery.** `classify-spec-edit` is removed once its two callers stop calling it; `check-artifact-size` stops naming per-fix reopens. Each surface owner gains its back-link, and this repository's own hook moves from `warn` to `block` once its backlog drains.

## Technical Decisions

### One owed-work test, three readers

`runtime/src/primitives/owed_work.rs` holds one function: given a spec's text and its `tasks.md` text, it returns the unchecked boxes and the unticked criteria. It calls the two parsers every current reader already uses: `read_tasks::unchecked_lines` (any heading, outside fences and HTML comments, 079's AC27 test) and `read_spec::parse_checkboxes` over "Acceptance Criteria". Those readers are `check_artifacts::check_unchecked_boxes`, `check_artifacts::check_unticked_criteria`, `check_review_gate::unticked_criterion_block` and `measure_reopenings::waiting_features`. Because every caller already reads those two functions, no reader loses a property it enforced by construction (`QUAL-DELEG-001`).

`check-owed-work` (feature in; `owes`, `unchecked-boxes`, `unticked-criteria` out) exposes it, reading the working tree. `measure-reopenings` (AC16) calls the same helper on blobs at each commit, and the CI template calls `check-owed-work` per `done` spec (AC15).

### The reopen decision reads state, not a diff

`framework/commands/amend.md` step 5 and `framework/commands/groom.md` step 7 invoke `check-owed-work` after their write in place of `classify-spec-edit`. On a `done` spec that owes work, each confirms through `gate-confirm`, naming the boxes and criteria, then calls `set-status` with `from: done`. The text that changes is fixed by the census in the spec's Resolved Questions:

- amend's step 5 and its intro;
- the criterion route's "meaningful body edit" wording;
- the Scope Boundaries line naming the diff read;
- the two status-table rows keyed on `classify-spec-edit`;
- groom's step 7, its step 3 and Scope line;
- `framework/reference/groom.md` §Reopening a `done` spec's last bullet.

Amend and groom keep their own reopen for a scenario or criterion, which the state rule reproduces.

### The constitution, size-neutral

The current text of §spec-lifecycle's second and third back-edges, the "Any other edit there reopens" paragraph and the operational bullet on syncing a canonical record (1,651 bytes together) is replaced by:

- a back-edge bullet, **Backward via owed code**, carrying `` `done` → `in-progress` `` and stating the state rule, who writes it (the writing command, `/review` on an untick or a deferral to its own spec's work), and that any other edit leaves a spec `done`;
- a shortened sentence on living specs.

The bullet keeps the `- **Backward via` form with a backticked `` `done` → `in-progress` ``, because `scripts/audit/cross-doc-consistency.sh` family 1c (`derive_constitution_back_edges`) derives its edge set from those bullets and holds it equal to `BACK_EDGES`. That script's header and the `sub_1c` comment that name "a meaningful body edit" are rewritten with it.

§implement-phase's "reopened and corrected", §drift-prevention's bullets on a meaningful edit and on the diff computing a sweep's reopens, §cross-spec-impact's `classify-spec-edit` clause and §bug-handling's fourth route (AC10) change in the same commit. `framework/constitution.md` is 49,990 bytes against the 50,000-byte ceiling `scripts/lint-governing-size.sh` reads from the index, so the commit must leave it no larger. The 1,651 bytes removed are the budget. `AGENTS.md` (49,905 bytes) is held the same way.

### The review window starts where the last review ended

In `runtime/src/primitives/compute_review_scope.rs`, `resolve_base` takes the last counted review's end, `last_review_commit` walking `HEAD` without hiding the reopen commit, whatever the reopen.

- `held_base`'s clamp to `transition_parent`, the carried-in-record branch of `last_review_commit` and the `UncommittedTransition` shortcut that skipped the last review are removed (AC7).
- With no counted review, an `in-progress` spec starts at the reopen as today (`Reopen`, `ReopenPastPartialReview`). A `done` spec starts at the commit that set it `done` or added it as `done`, found by a new `check_stuck` lookup beside `walk_reopenings` that reuses `status_of_commit`, where today's base is empty (`BaseSource::None`).
- The tests that assert the old start, among them `a_review_older_than_the_reopen_leaves_the_start_at_the_reopen` and `a_record_in_the_transition_commit_leaves_the_start_at_the_reopen`, are inverted. The module doc, `COMPUTE_REVIEW_SCOPE`, `BaseSource`'s doc comments, `framework/reference/review.md` §Window and coverage and `runtime/tests/golden/review-basic.jsonl` follow.

### Code no spec claims leaves open specs' scopes

`attribute_window`'s branch for a commit with no claimant stops adding its paths to the spec's scope. `WindowClaims::take_unclaimed`, `ReadRecord`, `read_records` and `commits_between`, 081's crediting, have no other caller and are removed. `WindowCommits::unclaimed`, which 083's AC13 count reads, stays. Uncommitted paths still join the scope whole in `split_changes`, since uncommitted work has no commit to attribute (Trade-offs).

### The project record

`write-review` takes `feature` as optional. Without it the record is `{specs-root}/review.md`, written whole through `write_if_unchanged` like a feature's (`BE-TXN-002`), and skipping what has no meaning there:

- `spec.md`, criteria and departures;
- `tasks.md`'s refusal;
- the contracts digest and analysis freshness.

Its scope is the unclaimed half of `check-unreviewed-code` at the run's head. A `seed` flag writes the record with `last-run: null` and `reviewed-against` at `HEAD`, so the window starts at the update (Open Questions Resolved, below), with the commit substituted by the runtime, never transcribed.

`fix_rounds::RoundSubject` takes an optional feature. `round_paths` leaves out `{specs-root}/review.md` for the project subject (AC14). `validate_feature`, `feature_rounds_dir` (a project subject uses `.ductus/review-rounds/project`, a name no feature directory can take) and the callers `start_fix_rounds::run`, `brief_re_read::run` and `check_prescriptions` accept it.

`/ductus:review --project` reviews under the project record, and `--all` includes it. The record's shape is in [data-model.md](data-model.md).

### `check-unreviewed-code`

A new primitive (`runtime/src/primitives/check_unreviewed_code.rs`) reads the committed tree at `at` (default `HEAD`), never the working tree, since a push carries commits. It reports:

- **for each `done` spec**, the code paths changed after its window start (the decision above), by commits that claim it;
- **for code no spec claims**, the code paths changed after the project record's `reviewed-against`.

Code is every path for which `is_project_artifact` is false. A path a commit gives an `in-progress` spec is not reported. The per-path claim rule now inside `attribute_window` moves to `commit_attribution` as one function both read, so the two never disagree (`QUAL-DELEG-001`).

A subject whose history is cut (`PrimitiveError::ShallowHistory`, as `diff_cross_spec` already catches it) is reported under `not-examined` with its reason, never as clean (`QUAL-CLAIM-001`). With `since`, the primitive walks the first-parent commits from `at` back to `since` (`Revwalk::simplify_first_parent`) and counts those at which it reports code, for bar (c) (AC17).

`[review] unreviewed-push` is read by a reader shaped like `fix_rounds::re_reads_off`, returning `block` when absent. Any other value fails with a new `PrimitiveError` variant naming the key and the file (`CFG-ENV-001`, `CFG-ENV-008`). The key joins the `review` section of `runtime/src/schema/config_keys.rs` and `framework/bootstrap/ductus.md` §Project Configuration, the inventory `the_installer_documents_the_registry` holds equal to it (`CFG-ENV-002`). The CLI exits 1 under `block` when anything is reported or not examined, and 0 under `warn`.

### `/ductus:review` on a `done` spec

`framework/commands/review.md` changes in three places, with `framework/reference/review.md` §Criteria step and §Deferrals stating the rules (AC3, AC5):

- **Step 7.3.** On a `done` spec, before `mark-criterion` changes a ticked criterion judged not met, it asks the owner which side is correct through `gate-confirm`, unless the invocation instructed it to decide. Code correct: the criterion's text is corrected, stays ticked, and goes to `write-review` judged met with its new text. Code wrong: it is unticked and `set-status` writes `in-progress`.
- **Step 10.** A deferral to the spec's own work appends its task and writes `in-progress`, as a deferral to another `done` spec's work does today. No gating finding is recorded without a disposition.
- **Scope Boundaries.** It gains the one status the review writes on its own spec.

066's scenario `criteria-are-verified-in-review` gains the `done`-spec exception (Cross-spec impact). `runtime/tests/golden/*-basic.jsonl` moved by a `review.md` edit are re-blessed alone and read.

### The pre-push hook

`framework/bootstrap/hooks/ductus-pre-push` (managed, `# managed-by: ductus` on line 2, `update`) and an outer stub `framework/bootstrap/hooks/pre-push` (`create`) follow `ductus-pre-commit`'s shape:

- ROOT from its own directory (spec 059);
- `.ductus/bin/ductus`, else `ductus` on `PATH`;
- a bounded `--help` capability probe for `check-unreviewed-code`;
- a missing or too-old runtime halts naming it and `--no-verify`.

For each line git gives on stdin, it runs `check-unreviewed-code --at <local sha>`, skipping a ref the push deletes (an all-zero local sha). The CLI's exit status is the hook's. Shipping it adds:

- two manifest rows in `framework/bootstrap/ductus.md` §Shared Files;
- `framework/bootstrap/ductus-procedure.md` §Hook Installation's file list, `chmod` line, report lines, manual-integration snippet and `pinned.files` paragraph;
- the `chmod` seed entries in the `claude` Agent Registry row;
- `.github/workflows/runtime.yml`'s two path triggers that name the pre-commit hook;
- a `scripts/audit/adopter-shell-behavior.sh` fixture for each AC8 case, a shallow clone among them.

No migration is needed to install it: the `update` row and the `create` stub land on the next `/ductus` run.

### The seeded project record

A migration (`framework/migrations/project-review-record.md`, with its `framework/migrations.toml` entry ordered after every existing one) runs `write-review --seed` once per adopter. `framework/bootstrap/ductus-procedure.md` does the same on a fresh install, so the window for code no spec claims starts at the update (the agent's recommendation, adopted by the operator, 2026-11-24). This repository's seed lands with the project-record task.

### The CI template and the audit backstop

`framework/templates/ci/adopter-generators.yml`'s checkout gains `fetch-depth: 0`, with the comment §design-principles requires of a history reader, and two steps:

- one runs `check-unreviewed-code` with its exit status, printing a `::warning` annotation per reported path under `warn` (AC9);
- one runs `check-owed-work` for each `done` spec and fails naming each box and criterion (AC15).

Both honour the job's commented `working-directory`. The pinned `DUCTUS_VERSION` predates both primitives, so the pin and its sha move in the release commit that ships them (AC13).

`scripts/audit/done-spec-criteria.sh` (Family 27) reads each `done` spec's `tasks.md` beside its criteria, with the AC27 test (any heading, outside fences and HTML comments). A new `scripts/tests/test-done-spec-owed-work.sh`, modelled on `test-audit-path-containment.sh`'s `new_tree` and `run_family`, proves each condition fails present and passes absent. It is run from `.github/workflows/framework-checks.yml`.

### The status count

`dashboard` gains an `unreviewed-code` block from `check-unreviewed-code` at `HEAD`, with a state for found, none and not examined, beside `reopenings`. `pipeline_view::summary` renders it as a line before the Reopenings line, never omitted or cut (`whole`), as `reopenings_line` and `inbox_line` are.

### Measuring the bar

`measure_reopenings::OpenPass` gains `held_owed`, ORed at every commit the walk observes for the open pass. It reads `spec.md` and `tasks.md` at that directory version through `file_in` and the owed-work helper, memoized per blob as `statuses` is. `Reopening` carries `held-owed-work`, and `ReopeningCounts` carries `never-owed-count` over closed reopenings, `not-counted` ones outside it (AC16). Bar (c) is `check-unreviewed-code --since` (above).

### Retiring `classify-spec-edit`

Once amend and groom stop calling it, `runtime/src/primitives/classify_spec_edit.rs` is removed from the seven registration sites `docs/developing-ductus.md` §Adding or removing a runtime primitive names:

- `PRIMITIVE_REGISTRY`;
- its description constant;
- its `#[tool]`;
- its `dispatch_primitive` arm;
- its `Command`;
- `framework/runtime-tools.txt`;
- its `runtime/tests/result_sizes.rs` case.

Its schema types go with them. `scripts/gen-configure-mcp.sh` regenerates the configure allow blocks. `merge_permissions::RETIRED_ALLOW` and `framework/bootstrap/configure/claude.md`'s list of tools the server no longer serves gain it, as 081's AC3 did for `check-review-freshness`.

Its mutants in `scripts/mutation/reopen-measure.py`, `review-022-task-46.py` and `line-citation-strip-066.py` go. So do the comment-only mentions in:

- `check_artifact_size.rs`;
- `relocate_audit_records.rs`;
- `read_tasks.rs`;
- `analyze_subjects.rs`;
- `line_citations.rs`;
- `criterion_verdicts.rs`;
- `prune_plan.rs`.

`mechanical_sweep.rs`'s module doc, `runtime/tests/mechanical_sweep_parity.rs` and `framework/reference/review.md`'s §Window and coverage sentence name the analysis record's freshness (`analyze_subjects::exempt_renames`) as the sweep rule's one reader (AC12).

### `check-artifact-size` names no per-fix reopen

`check_artifact_size::on_done` and the `OnDone` enum go: in a `done` spec no size fix leaves an unchecked box or an unticked criterion, so every cell reads the same. A `done` spec's warning states the state rule once, and the tests asserting the per-fix table are rewritten to it (AC18).

### This repository's hook

`.githooks/pre-push` runs the check with this repository's runtime, as `.githooks/pre-commit` locates it, the agent's recommendation adopted by the operator, 2026-11-24. `.ductus/config.toml` sets `[review] unreviewed-push = "warn"` while the measured backlog of `done` specs owing a review drains. The last task removes it, so 084 does not reach `done` at `warn`.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/owed_work.rs` | Create | The owed-work helper |
| `runtime/src/primitives/check_owed_work.rs` | Create | `check-owed-work` |
| `runtime/src/primitives/check_unreviewed_code.rs` | Create | `check-unreviewed-code`, its `[review] unreviewed-push` reader and the `since` count |
| `runtime/src/primitives/classify_spec_edit.rs` | Delete | Retired (AC12) |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Window start (AC7); unclaimed code out of scope (AC6) |
| `runtime/src/primitives/check_stuck.rs` | Modify | The commit that set a spec `done` |
| `runtime/src/primitives/commit_attribution.rs` | Modify | The per-path claim rule, shared |
| `runtime/src/primitives/write_review.rs`, `runtime/src/primitives/write_review/` | Modify | The project record and its seed |
| `runtime/src/primitives/fix_rounds.rs`, `start_fix_rounds.rs`, `brief_re_read.rs`, `check_prescriptions.rs` | Modify | A project subject (AC14) |
| `runtime/src/primitives/measure_reopenings.rs` | Modify | `held-owed-work`, `never-owed-count` (AC16) |
| `runtime/src/primitives/dashboard.rs`, `pipeline_view.rs`, `render_status.rs` | Modify | The status count (AC9) |
| `runtime/src/primitives/check_artifacts.rs`, `check_review_gate.rs`, `read_tasks.rs`, `read_spec.rs` | Modify | Read the shared helper |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | No per-fix reopen (AC18) |
| `runtime/src/primitives/merge_permissions.rs` | Modify | `RETIRED_ALLOW` |
| `runtime/src/primitives/mod.rs`, `mechanical_sweep.rs`, `analyze_subjects.rs`, `line_citations.rs`, `criterion_verdicts.rs`, `prune_plan.rs`, `relocate_audit_records.rs` | Modify | Module list and stale comments |
| `runtime/src/schema/primitives.rs`, `descriptions.rs`, `registry.rs`, `config_keys.rs` | Modify | Args, results, descriptions, registry, the config key |
| `runtime/src/main.rs`, `runtime/src/mcp/server.rs`, `runtime/src/interpreter/mod.rs` | Modify | Registration |
| `runtime/tests/` | Modify | `result_sizes.rs`, `mcp.rs`, `mechanical_sweep_parity.rs`, `golden/*-basic.jsonl`, new fixtures |
| `runtime/CHANGELOG.md` | Modify | `[Unreleased]` entries (AC13) |
| `framework/runtime-tools.txt` | Modify | Two primitives added, one retired |
| `framework/constitution.md` | Modify | §spec-lifecycle, §implement-phase, §drift-prevention, §cross-spec-impact, §bug-handling (AC1, AC10) |
| `framework/commands/amend.md`, `groom.md`, `review.md`, `status.md`, `prune.md`, `consolidate.md`, `help.md`, `fold.md` | Modify | The rule, the review changes, the sweep |
| `framework/reference/groom.md`, `review.md`, `implement.md`, `fold.md`, `analyze.md`, `audit.md` | Modify | The same |
| `framework/bootstrap/hooks/ductus-pre-push`, `framework/bootstrap/hooks/pre-push` | Create | The shipped hook (AC8) |
| `framework/bootstrap/ductus.md`, `framework/bootstrap/ductus-procedure.md`, `framework/bootstrap/configure/claude.md` | Modify | Manifest rows, installation, the config inventory, seeds, retired tools |
| `framework/migrations/project-review-record.md`, `framework/migrations.toml` | Create, Modify | The seed for existing adopters |
| `framework/templates/ci/adopter-generators.yml` | Modify | Full history, two steps (AC9, AC15) |
| `scripts/audit/done-spec-criteria.sh`, `cross-doc-consistency.sh`, `ssot-invariants.sh`, `rename-sweep-residue.sh`, `adopter-shell-behavior.sh`, `run-all.sh`, `README.md` | Modify | Family 27, family 1c, the sweep, the hook fixture |
| `scripts/tests/test-done-spec-owed-work.sh` | Create | AC15's test |
| `scripts/mutation/reopen-measure.py`, `review-022-task-46.py`, `line-citation-strip-066.py` | Modify | Retired mutants |
| `.github/workflows/runtime.yml`, `framework-checks.yml` | Modify | Hook path triggers, the new test |
| `.githooks/pre-push`, `.ductus/config.toml`, `specs/review.md` | Create, Modify, Create | This repository's hook, its `warn`, its seed |
| `AGENTS.md`, `README.md`, `specs/README.md`, `docs/slash-commands.md`, `docs/analyze.md`, `docs/developing-ductus.md` | Modify | The sweep |
| `specs/008-*`, `011-*`, `013-*`, `022-*`, `023-*`, `026-*`, `030-*`, `035-*`, `041-*`, `047-*`, `049-*`, `050-*`, `051-*`, `054-*`, `063-*`, `079-*`, `081-*` | Modify | The sweep and AC11's notes |
| `specs/018-*`, `020-*`, `022-*`, `050-*`, `057-*`, `066-*`, `067-*`, `079-*`, `081-*`, `083-*` | Modify | Back-links discharging `cross-spec-impact:` |

## Data Model

[data-model.md](data-model.md) defines the project record and the fields 084 adds to primitive results; once built, each primitive's compiled description is canonical for its own result (§drift-prevention).

## Trade-offs

- **Rejected: keeping `classify-spec-edit` to decide a reopen.** That kept the diff rule the spec replaces.
- **Rejected: a new parse for owed work.** It would be a second copy of the two parsers four readers share.
- **Rejected: reviewing unclaimed code inside open specs.** 081's crediting survives only as long as that does. The spec moves it to one record.
- **Rejected: a hook that parses `.ductus/config.toml` itself.** §runtime-boundary principle 3 rules out a shell parse, and the runtime is required.
- **Rejected: bar (c) from a hook log or the remote's push history.** See the spec's Resolved Questions.
- **Limitation: uncommitted unclaimed code still joins an open spec's scope.** Uncommitted work has no commit to attribute, as `split_changes` has it today.
- **Limitation: `since` mode is slow.** It runs the whole check once per first-parent commit in its window. It is a measurement, run when the bar is read, not on every push.
- **Limitation: the CI template cannot run either new step until a release ships the primitives.** Its pin moves in that release's commit.
- **Limitation: the estimate's blind spot stays in `held-owed-work`.** A box added and ticked in one commit is not seen.

## Open Questions Resolved

- **The project record's start** is its seed at the update, not the first commit. Seeding leaves 0 of this repository's 208 unclaimed code commits owed at landing, measured by `git log` over non-merge commits naming no spec and touching no spec directory and a path outside the project's artifacts, at `1590941c`. (The agent's recommendation, adopted by the operator, 2026-11-24.)
- **This repository runs the hook**, so it meets the cost it ships (the agent's recommendation, adopted by the operator, 2026-11-24).
- **It lands at `warn` and moves to `block` once its backlog drains**, by the last task (the agent's recommendation, adopted by the operator, 2026-11-24). The backlog at `1590941c`: 9 `done` specs, 000, 012, 021, 025, 049, 060, 062, 069 and 081, with 20 commits. The method: non-merge commits from each spec's current `diff-base` to `HEAD` whose subject names the spec and that change a path outside the project's artifacts. It approximates the new window, which AC7 may shift.
- **A missing runtime halts the hook** whatever the key says, since the runtime is bundled and therefore required (owner, 2026-11-24).

## Cross-spec impact

Each spec in `cross-spec-impact:` gains a back-link to this spec in its `spec.md`, read by `check-artifacts`' cross-spec-impact drift as discharging. For 079 and 081 it sits under `## See also`, where `derive-dependencies` induces no edge, since this spec's body links both and an edge back would close a cycle. The owed back-links:

- **018:** the pre-push hook beside its pre-commit hook.
- **020:** the review command's `--project` flag and its status write.
- **022:** the `dashboard` count and the configuration key.
- **050:** the constitution's new rule.
- **057:** the CI template's new steps.
- **066:** the project record in the review record's schema, and its scenario `criteria-are-verified-in-review`'s `done`-spec exception.
- **067:** `review-state-drift` read through the shared helper.
- **079:** AC5, AC15, AC25 and the retirement.
- **081:** AC10 and AC13.
- **083:** the unclaimed-commit count staying while unclaimed code leaves open specs' scopes.
