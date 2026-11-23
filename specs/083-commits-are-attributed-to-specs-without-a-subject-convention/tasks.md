# 083 — Commits are attributed to specs without a subject convention Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Count the window's examined and unclaimed commits

- [ ] Count in `attribute_window` per the plan's §Counting in the window walk, carried on `WindowClaims` into `ReviewScope` as an `Option`, `None` when `diff_base` is empty
- [ ] Add `commits-examined` and `commits-unclaimed` to `ComputeReviewScopeResult`, inline in both forms and omitted when `None`
- [ ] State both, and their absence without a window, in `COMPUTE_REVIEW_SCOPE`
- [ ] Fixtures in `runtime/src/primitives/compute_review_scope.rs` for each AC13 direction: unclaimed commits reported, a credited one still reported, zero beside the count examined, a clean merge examined and not unclaimed, and an empty diff base reporting neither
- [ ] Prove each fixture fails with its counting removed

- **Done when**: the AC13 fixtures pass, each fails when its behavior is removed, and `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` are clean.

## 2. Prove a ticked commit is attributed whatever its subject

- [ ] A fixture in `runtime/src/primitives/compute_review_scope.rs` with specs 022 and 034: a commit with subject `123 - 022: x` holding a code change and a tick in 022's `tasks.md` puts the code in 022's scope and not in 034's (AC11)
- [ ] The same code change committed with no tick under the same subject stays in both scopes (AC12)

- **Done when**: both fixtures pass and each fails when the tick, or its absence, is swapped.

## 3. Render the counts in the review's Summary

Blocked on task 1: the line reads the counts `compute` returns.

- [ ] `resolve_scope` returns the counts, `Record` carries them, and `report_summary` appends the line on both its branches, as the plan's §The Summary line says
- [ ] Tests for both forms of the line, including a run with an empty diff base; update the tests that assert the Summary's exact text
- [ ] State the line in `WRITE_REVIEW`
- [ ] Add the unreleased entry for tasks 1 and 3 to `runtime/CHANGELOG.md`

- **Done when**: the tests pass, each new one fails with its line removed, the record's frontmatter gains no field, and the whole `cargo test` passes.

## 4. Commit each task's code with its tick

- [x] Rewrite step 8 of `framework/reference/implement.md` §Walk through tasks (per task, in order) as the plan's §The commit step says, covering confirmation, decline, refusal, staging and no push
- [x] Point `framework/commands/implement.md`'s `--auto` Flags bullet and step 7's commit reminder at that step
- [x] Confirm that no line of `framework/` asks for a commit-subject form (AC14)

- **Done when**: AC10, AC14 and AC15 hold on reading both files, and `framework-checks.yml`'s and `generators.yml`'s steps pass (`scripts/run-workflow-steps.py`).

## 5. Sync 066's data model and link back

Blocked on tasks 1 and 3: the fields must exist, and the Size figures are measured on the release binary that has them.

- [ ] Document `commits-examined` and `commits-unclaimed` in `specs/066-audit-records-and-the-completion-gate/data-model.md` §`compute-review-scope`
- [ ] Re-measure the Size paragraph's file-form bytes with the release binary, by the method it states, and update them
- [ ] Add a blockquoted signpost to `specs/066-audit-records-and-the-completion-gate/spec.md` linking back to 083
- [ ] Reopen 066 with `set-status` (`done` → `in-progress`) in the same commit, since its data model changed

- **Done when**: `check-review-gate` on 083 reports its cross-spec impact on 066 discharged, and 066 is `in-progress` with the edits committed. 066 then owes an analysis and no review (081), on its own walk.

## 6. Sweep prose claims

- [ ] Run `scripts/stale-literals.sh`, then grep the live-artifact set by meaning for claims about `compute-review-scope`'s result fields, the Summary's lines, and `/{project}:implement`'s commit prompt; fix each stale one at its source

- **Done when**: no live artifact describes the commit prompt or the result as they were before this spec. 083 then owes its review and analysis, and the completion gate, once every box here is ticked.

## 7. Disposition out-of-spec finding: a path-list commit leaves the shared index holding the generated outputs' old blobs

- [ ] `.githooks/pre-commit` — under `git commit -- <paths>` the hook writes each generated output it carries to `GIT_INDEX_FILE`, git's temporary index, so the commit holds the new outputs while the shared index keeps their pre-commit blobs, and a later whole-index commit would revert them. Seen on 7eb29d1f's three generated `implement` copies, reset by hand on 2026-11-23. Owner of the hook's staging rule: 017's scenario `a-commit-stages-only-its-own-generated-files`; the shipped `framework/bootstrap/hooks/ductus-pre-commit` is unchecked for the same.

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task
