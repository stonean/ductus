---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-28T00:11:31Z
reviewed-against: ca473fd324c763af21dc8ff5ee0d7220b490fb6c
diff-base: 7e98cc48963acaad87b9c2d86071bc8d5eaa5c27
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking.

**The diff base resolved itself this time.** The two previous runs in this round needed a manual `--since=<base>~1` because the base landed on the commit carrying the work. `compute-review-scope` now peels the transition commit to its first parent, and this review's window opens at `7e98cc4` without an override — covering `compute_review_scope.rs` and `check_command_flags.rs`, the two files it exists to examine. The fix is exercised by the run that reports on it.

Window covers four changes. `derive-references` enumerates every tracked spec and filters only the write, reporting drifted-but-unstaged specs in `unwritten` — the gap that let a `[services]` alias rename leave dead references in place for nine commits. Both derive primitives gained `absent` and now count only specs they actually read. `append-task` stopped discarding a supplied `slug` when `body` is given. And `compute-review-scope` bases the window on the transition commit's parent, so work committed alongside an `/ductus:amend` back-edge flip is inside it.

All three items this round's earlier reviews captured to the inbox are addressed rather than carried past the tag: `argument_hint` now delegates to `split_frontmatter` (with tests for the empty-block and CRLF-opener cases the hand-rolled scan got wrong), the extra corpus walk was measured at ~20 ms per commit and the measurement recorded in the scenario that caused it, and the diff-base defect became a scenario and a fix.

Security: `transition_parent` consumes a sha produced internally by `find_in_progress_commit`, not caller input, and a parentless commit falls back rather than erroring. Quality: the shared lookup is unchanged, so `check-stuck` still counts from the transition commit itself — a helper returning different commits to its two callers is the drift the placement avoids. Efficiency and simplicity found nothing; the `argument_hint` refactor removed a second definition of where frontmatter ends rather than adding one.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
