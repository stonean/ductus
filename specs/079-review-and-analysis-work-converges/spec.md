---
status: draft
dependencies: []
next-criterion: 4
---

# 079 — Review and analysis work converges

A spec's `/{project}:review` and `/{project}:analyze` work ends in a bounded number of runs, and one spec's run does not create work for another spec. This is the efficiency work `078-review-fix-rounds-are-re-read-within-a-bound` was meant to be.

## Motivation

`078-review-fix-rounds-are-re-read-within-a-bound` bounded one loop: the fix rounds inside a single `/{project}:review` run, re-read by an independent agent within a budget of two re-reads. That loop worked; 078's second review ran two rounds and one re-read. The cost the owner was losing time to was the outer loop: a review deferred part of what it found into the spec it reviewed, or into a sibling, so another implement → review → analyze cycle followed, and that review found more. No criterion of 078 measured an outcome, so its reaching `done` could not have shown whether reviews converged.

The corpus showed it. Each figure was measured on 2026-11-04 at `d62e72f3`, before this spec existed:

- **Status.** 37 specs were `in-progress`, 34 `done` and 5 `draft`, read from each `specs/NNN-*/spec.md`'s `status:`. 10 of the 37 had no unchecked `- [ ]` line in their `tasks.md`, so they waited only on review, analysis or the completion gate: 015, 016, 029, 034, 037, 039, 045, 055, 061 and 078. Eight more had exactly one.
- **Runs per spec.** Counted from commit subjects that start `review(NNN)` or `analyze(NNN)`, over all history; a run recorded under another subject, or two runs in one commit, are not counted.
  - `done` specs: review runs median 2, maximum 7, and 11 of 34 took more than two (006, 013, 019, 023, 025, 040, 042, 043, 046, 051, 062). Analyze runs median 1, maximum 5.
  - `in-progress` specs: review runs median 2, maximum 52 (022), and 15 of 37 had already taken more than two (008, 017, 020, 022, 026, 029, 036, 041, 045, 047, 048, 050, 052, 058, 063). Analyze runs median 2, maximum 21.
- **Share of the history.** Since 2026-10-01, 112 of 847 commits were `review(` or `analyze(` commits, by subject prefix.
- **078 itself.** It was drafted on 2026-11-03 with nine tasks; its first review (`f47d2b03`) added five more (tasks 10–14) and recorded 40 observations; its second (`d62e72f3`) found 4 violations and 41 observations in its passes. Its artifacts and code came to 3,820 lines (`wc -l` over `git ls-files` of its spec directory, its five loop primitives and `runtime/tests/fix_rounds.rs`).

078's second review showed where the outer loop came from:

- **The window.** The default diff base, the parent of 078's `in-progress` commit, resolved 127 scope entries; a base at the first review's commit resolved 62 (`compute-review-scope`). Of those 62, 27 were touched by 078's own commits since the first review and 28 only by 066's commits (`git diff-tree` per commit), and 17 of the 45 items the passes found (2 violations, 15 observations) concerned 066's or 026's work, as 078's `review.md` records them.
- **Copies drifting.** 16 of the 21 observations fixed as a group had prose-only fixes: doc comments, MCP descriptions, the changelog, the command, the reference, and 078's spec, plan and data model disagreeing with the code or with each other (classified by whether the fix edited only prose). All 7 findings of its one re-read were prose too.
- **Deferral into the reviewed spec.** Every deferral to 078's own tasks guaranteed another cycle. The owner then chose to waive, under the AGENTS.md rule that waives an observation about the pipeline's own machinery outside the code under review, 14 observations that the recommendations had proposed deferring.
- **The tie-breaker.** AGENTS.md's rule that process cost never weighs in a recommendation (owner, 2026-10-27) made every recommendation ignore the cycles it started.

The session working 066 reported four more costs while 078's review ran:

- A fix round diffed the whole working tree against its base, so another session's landing in the shared checkout joined the round as though it were the review's fix; 066 held every landing for 078's whole loop.
- The shared-checkout commit procedure ran the expensive checks twice: the pre-commit hook in a fresh clone, then a gate mirroring CI with two release builds, release tests, release clippy and the audit. A `cargo clippy --all-targets -- -D warnings` before committing would have saved one hook cycle on 066's task 14.
- `diff-cross-spec`, which `/{project}:implement` renders after every task, diffed from the feature's first spec-directory commit, and reported 507 cross-spec paths for a 066 task that touched no sibling.
- macOS arm64 runners went unacquired on three consecutive attempts, each holding about 15 minutes before it was cancelled. That is the CI provider's capacity, outside this framework.

## Behavior

- **A spec's review and analysis converge.** After a spec's last planned task, its `/{project}:review` and `/{project}:analyze` runs end within a bound this spec states and measures from history, so a spec waiting only on them does not stay `in-progress` across cycles.
- **A run creates work only where it is owed.** A review or analysis run adds a task, scenario or body edit to the spec it runs on only to dispose of a gating finding; everything else it finds is fixed in the run or waived with a reason.
- **A run reads its own spec's work.** What a review or analysis reads, and what its fix rounds re-read, is the spec's own change, so another spec's commits in the same tree neither enter it nor wait on it.
- **A behavior is stated once.** Each primitive's or command's behavior has one canonical description, and every other place points at it, so a change edits one sentence rather than seven.
- **The bound is measured, not asserted.** The outcome this spec targets is computed from the repository's history by a stated method, and the measurement reports what it could not count.

Each bullet is a direction the Open Questions below settle; none is a design yet.

## Acceptance Criteria

Candidates, which the Open Questions below settle:

- [ ] AC1: A spec reaches `done` within the number of `/{project}:review` and `/{project}:analyze` runs after its last planned task that this spec states, measured over the specs that reach `done` after it ships, by a method this spec records and that reports each run it could not attribute.
- [ ] AC2: A review or analysis run adds a task, scenario or body edit to the spec it runs on only to dispose of a gating finding.
- [ ] AC3: A review's scope and its fix rounds' paths exclude a change another spec's commit made, and the record names any it excluded.

## Open Questions

- What outcome does this spec target, and how is it measured? For example, `done` within N review runs and M analyze runs after the last planned task, counted from commit history by a stated method; against the baseline in Motivation, where 11 of 34 `done` specs exceeded two review runs.
- Where does a re-review's window start by default: at the last reopen, as `compute-review-scope` derives it now (`020-code-review`, `066-audit-records-and-the-completion-gate`), or at the spec's last review commit?
- What is a review's scope: every path the repository changed since the base, as 066's union defines it, or the spec's own change (its plan's files and the commits made for it), and how is a commit attributed to a spec? The same question covers a fix round's paths (078's `brief-re-read`, which diffs the whole working tree) and `diff-cross-spec`'s window.
- How are a run's findings dispositioned so that it creates no new cycle: no deferral into the spec under review except for a gating finding; an observation fixed in the run when mechanical, otherwise waived; an observation about another spec's code waived rather than deferred, as AGENTS.md already says for the pipeline's machinery? This touches the disposition rules of `058-findings-route-at-discovery` and `076-waived-and-deferred-findings-are-distinct-dispositions` and the review and analyze references.
- Which surface is canonical for a primitive's or command's behavior (the data model, the doc comment, the MCP description, the command, the reference), and are the others reduced to pointers, generated from it, or checked against it?
- Does AGENTS.md's rule that process cost never weighs in a recommendation (owner, 2026-10-27) stay, change to weigh the cycles an option starts, or go?
- Does the shared-checkout commit procedure (AGENTS.md's gotcha on `017` task 45) keep its two gate runs, and does a pre-commit lint step come first?
- Where does each change land: in this spec, or as a scenario on the spec that owns the surface (`020-code-review`, `066-audit-records-and-the-completion-gate`, `022-deterministic-runtime`, `058-findings-route-at-discovery`, `076-waived-and-deferred-findings-are-distinct-dispositions`, `017-derive-dont-ask` for the pre-commit hook, the constitution), back-linking this spec under AGENTS.md's routing rule?
- Does 078's own loop change? Two gaps stand: a fix round's paths include other sessions' changes in a shared tree (above), and a review cut off between its first `write-review` and its last leaves a record without the loop fields, which gate check 12 passes as predating the loop (078's `review.md`, waived on 2026-11-04 under option B).
