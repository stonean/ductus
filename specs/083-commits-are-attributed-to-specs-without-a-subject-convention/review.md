---
spec: 083-commits-are-attributed-to-specs-without-a-subject-convention
last-run: 2026-11-24T00:52:01Z
reviewed-against: 93107c5058bcdd00e9afc3ebfda09f83f05d3454
diff-base: cd8f39c42631150fad98d7a22cb7c567d73b6a5d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 18
unread:
  - .claude/commands/ductus/implement.md
  - .ductus/reference/implement.md
  - .pi/prompts/ductus-implement.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/compute_review_scope.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/mcp.rs
  - scripts/mutation/scope-attribution.py
excluded: 57
contract-breaks: 0
skipped-passes: []
fix-rounds: 3
re-reads: 3
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
criteria-judged: 7
criteria-met: 7
criteria-verdicts:
  - label: AC10
    digest: c76d13fcd2bee68c8c2978e3013a620b2870cd0835df03cca9c9e9faa1229619
  - label: AC15
    digest: 4af10be2578c0c270bd2482b17ffbcaf9532adf7e93893802a59f95a2ab1b0fa
  - label: AC11
    digest: 841873f087297f5a45df7e3eff5fd0d4219f1f028142783f974e609458441f52
  - label: AC12
    digest: aaefe6972f7587baea06983b6171078e45506579735eff63f582319d416bbc56
  - label: AC13
    digest: 8719bc763494540e6d66a293f015e6b2075f2f99e93f160531fb99cf2ec6fada
  - label: AC16
    digest: 5263a9607bc366a2601d4faf4461e902b7a3fabdc7b8e5683e8d2b40b9e3acdf
  - label: AC14
    digest: 00a2d67a4fee1435d47438e6395345a90488f107329477fca986d241829cc78b
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "other: AC14 says attribution reads no project setting, but the attribution walk is handed `specs_root`, which `compute` and `write-review` read from the project's `[paths] specs-root` config (via `paths::Paths::load`), so the spec-root location is a configured value attribution depends on; no subject form or attribution setting is read. — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`"
    outcome: waived
    reason: The spec root locates the spec directories attribution reads, the resolution constitution §spec-phase gives every primitive for every spec path; attribution reads no setting of its own, and AC14 answers 083's Resolved Question on a declared commit-subject form, which nothing reads.
    decided-at: 2026-11-24T00:52:01Z
    decided-by: rule
---

# Review — 083-commits-are-attributed-to-specs-without-a-subject-convention

## Summary

Review of 083's reopening from diff-base cd8f39c4 (base-source reopen, no override) against 93107c50. Of the window's 20 commits, the nine whose subjects name 083 are its own (7eb29d1f, 97412348, fbdb88c9, 694a9d72, 385e3fbe, c9944f16, fadcb1dc, e5f23fa9, 296ab36c), each read with git show; the commits of 073, 050, 022, 017, 079 and 081 are excluded, as are c9944f16's and fadcb1dc's paths in 066's and 010's directories. Three readers ran the five passes and the criteria step at 296ab36c in parts split by behavior, each reading the 100,074-byte rule list of configuration-cross.md, quality-cross.md, security-backend.md, security-cross.md and security-frontend.md whole, and each driving a clone of 296ab36c: A the commit counts and attribution (tasks 1 and 2; AC11, AC12, AC13), B the Summary line (task 3; AC16), C the /implement commit step and 083's artifacts (task 4; AC10, AC14, AC15). Three commits landed after 296ab36c while the passes ran, 079's review (60fc2033), 081's docs (e54acd35) and 022's task 63 (93107c50); their changes to scope paths were read by hunk: 60fc2033's join_unread and is_glob, the doc comments of other compute-review-scope fields, the glob clause in the description, its CHANGELOG line and its re-anchored and added mutations, none touching the commit counts, and e54acd35's two paragraphs of .ductus/docs/agents/developing-ductus.md. Read whole, 8 of the 18 scope entries: .ductus/docs/agents/developing-ductus.md (whole at 296ab36c, and e54acd35's hunk), framework/commands/implement.md, framework/reference/implement.md, the implement-basic and review-basic goldens, scripts/mutation/commit-counts.py (fbdb88c9's whole new file, and 385e3fbe's additions), spec.md and tasks.md. Unread, 10: the three generated implement copies, each diffed against its source read whole, with every generator re-run in a clone and leaving no change; and seven files too large to read whole, of which the passes read every 083 hunk and each enclosing symbol whole: compute_review_scope.rs (run, ReviewScope, WindowCommits, compute, WindowClaims, take_unclaimed, attribute_window and the seven 083 tests), write_review.rs (resolve_scope, Record, report_summary, commits_summary and the three tests 385e3fbe added or changed), primitives.rs (ComputeReviewScopeResult), descriptions.rs (COMPUTE_REVIEW_SCOPE whole, and WRITE_REVIEW's changed part), mcp.rs (the file-form test and its fixture), CHANGELOG.md (the unreleased section) and scope-attribution.py (M266). Driven in the clone: the seven 083 lib tests, the Summary tests, the MCP file-form test and the implement-basic and review-basic parity tests passed; commit-counts.py's 15 mutations and scope-attribution.py's M266 were each killed, and M16, added in the fix rounds, was killed by the file-form test, with no invalid anchor in either list; the release binary's compute-review-scope and write-review were driven on fixture repositories, a clean merge, an empty diff base and the file form among them. No finding. Four observations: three chores fixed in the run (report_summary's stale doc comment; assertions that the commit counts stay inline in the file form, with mutation M16, shown to fail with the counts cleared there; and the rewrap of the docstring line that addition overran, which round 2 noted) and one waived by the rule. The fixes passed cargo fmt --check, cargo clippy --all-targets -- -D warnings and the whole cargo test --no-fail-fast in a clone of 93107c50 with this diff applied.

3 fix round(s): 3 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

0 of 20 commits in the window no spec claimed.

7 of 7 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC10 — met: framework/reference/implement.md §Walk through tasks step 8 commits the task's paths and tasks.md together, with --auto on its own and otherwise on confirmation; framework/commands/implement.md --auto bullet and step 7 point at it; generated copies match the generators' output.
- AC15 — met: framework/reference/implement.md step 8 shows staged paths and a proposed subject the operator may replace, stages only the task's paths and tasks.md, never pushes, a declined commit goes on to step 9, and a hook refusal halts the walk before the next task.
- AC11 — met: runtime/src/primitives/compute_review_scope.rs attribute_window + commit_attribution.rs Attributor::claims; a_ticked_commit_is_its_specs_whatever_its_subject passes both directions and M9 kills it; driven on the release binary (`123 - 001: with tick` in 001's scope).
- AC12 — met: an_unticked_commit_under_the_same_subject_stays_in_every_window passes for 022-a and 034-b and M10 kills it; driven on the release binary (the untouched-directory commit stays in 001-x's scope).
- AC13 — met: compute_review_scope.rs run/WindowCommits::of, schema ComputeReviewScopeResult, descriptions.rs COMPUTE_REVIEW_SCOPE names both counts; five fixtures pass, M1-M8 kill each direction, and the release binary reported 1 of 3, 2 of 4 with a clean merge, both counts absent with an empty diff-base, and both inline in the file form.
- AC16 — met: runtime/src/primitives/write_review.rs commits_summary via report_summary and resolve_scope; proved by write_review::tests::the_summary_counts_the_windows_unclaimed_commits, M11-M15 each killed, and the release binary wrote both forms with no new frontmatter field
- AC14 — met: attribute_window and Attributor::claims in runtime/src/primitives/compute_review_scope.rs and commit_attribution.rs read no subject-form setting (only the [paths] specs-root location is passed in), resolve_scope in write_review.rs reads none for attribution, and no framework/ command asks for a commit-subject form (step 8 only proposes a subject by the project's own conventions).

## Observations

- other: report_summary's doc comment still says a supplied Summary is rendered exactly as given and names only the loop's account as appended, but both branches now append the window's commit line (and, since 066, the criteria sentence) — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- other: `compute-review-scope`'s description and `ComputeReviewScopeResult` say both commit counts are inline in the file form too, and no test pins it: `a_scope_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp` only bounds the result's size (460 bytes) and the 083 fixtures call `run` on results small enough to stay inline, so a change that dropped the counts from the file form before `bounded_lists::place` would pass every test — `runtime/tests/mcp.rs` — **fixed** (decided by the rule)
- other: AC14 says attribution reads no project setting, but the attribution walk is handed `specs_root`, which `compute` and `write-review` read from the project's `[paths] specs-root` config (via `paths::Paths::load`), so the spec-root location is a configured value attribution depends on; no subject form or attribution setting is read. — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md` — **waived**: The spec root locates the spec directories attribution reads, the resolution constitution §spec-phase gives every primitive for every spec path; attribution reads no setting of its own, and AC14 answers 083's Resolved Question on a declared commit-subject form, which nothing reads. (decided by the rule)
- other: fix round 2's re-read noted that the commit-counts.py docstring line edited for the file form ran past the file's wrap width — `scripts/mutation/commit-counts.py` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `docs/rules.md`
- `framework/commands/groom.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/reference/groom.md`
- `framework/reference/review.md`
- `framework/templates/spec/spec.md`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/schema/extensions.rs`
- `runtime/tests/golden/specify-basic.jsonl`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/routing-path-claims.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `specs/010-agent-autonomy/plan.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/spec.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/073-decided-rule-departures-are-recorded/data-model.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`

## Unexamined governance

*None.*
