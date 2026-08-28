---
section: "Follow-on scenarios"
---

# Review-base-includes-the-transition-commit

## Context

`compute-review-scope` resolves the default diff base to the commit at which
the spec advanced to `in-progress`, then diffs `base..HEAD`. A git diff from a
commit excludes that commit's own changes. So whatever the transition commit
itself carried is outside every review window that starts at it.

That is harmless in the flow the design assumed — `/ductus:implement` flips
`planned → in-progress` in a commit containing only the status line, and the
work follows in later commits. It is not harmless in the reopen flow.
`/ductus:amend` takes the `done → in-progress` back-edge, and the natural
commit is the one carrying both the flip and the work it authorises. The review
window then starts *after* the work and contains none of it.

Observed twice in one round on 2026-08-27, on 017 and 020. Both had their
deliverables — a corrected scenario in one case, a new primitive, an audit
family, and a rewritten command body in the other — landed in the same commit
as the back-edge flip. `compute-review-scope` returned that commit as the base,
`modified-since` came back holding none of the files under review, and the
scope fell through to the plan's `Affected Files` alone, which on both specs
predates the work. Each would have recorded **0 MUST, 0 SHOULD** having
examined nothing. Both were caught only because the operator read the scope
list and overrode it with `--since=<base>~1`.

This is `QUAL-CLAIM-001` one level up from where the rule usually bites. The
result is not a wrong value; it is a *review* that reports a clean posture over
an empty subject, and nothing in the report distinguishes that from a genuinely
clean one. The gate then passes, `review.blocking` stays false, and the spec
advances — a check that could not run wearing the costume of one that passed.

Its sibling [review-scope-union](review-scope-union.md) fixed the adjacent
defect: the scope was *whichever set was larger*, which dropped the files the
work touched. This is the same failure a layer earlier — there the union was
computed over a good window, here the window itself excludes the work.

## Behavior

**The default diff base is the parent of the status-transition commit, not the
transition commit itself.** The commit that moved the spec into `in-progress`
is the boundary of the work window, and a boundary belongs inside the window it
bounds. `find_in_progress_commit` still identifies the transition — the lookup
is shared with `check-stuck` and is unchanged — and `compute-review-scope`
peels to its first parent before diffing.

In the flow the old behavior served, this widens the window by one commit
containing a status line, which changes no finding. In the reopen flow it is
the difference between reviewing the change and reviewing nothing.

**An explicit `--since` is used verbatim.** No parent walk is applied to a
caller-supplied base: an operator naming a commit means that commit, and
silently reviewing one more than they asked for would be its own scope
surprise.

**A transition commit with no parent falls back to itself.** On a root commit
there is nothing earlier to diff from; the base stays as it resolves today
rather than erroring, since an unusual history is not a defect to halt on.

**No transition commit found is unchanged** — the base is empty and the scope
comes from the plan's `Affected Files` alone, exactly as now.

## Edge Cases

- **The transition commit carries unrelated work.** Someone flips the status
  inside a larger commit touching another spec's files. Those files enter
  `modified-since` and are reviewed. This is deliberate over-inclusion: a
  review that looks at slightly too much produces noise the reviewer can
  discard, while one that looks at too little produces a clean verdict nobody
  can distinguish from a real one. The asymmetry is the whole argument, and it
  is the same one `review-scope-union` settled in the same direction.
- **A spec reopened several times** resolves to its most recent transition, as
  today; the parent of that one is the base. Earlier windows are the earlier
  reviews' business and are already recorded in their `review.md`.
- **`check-stuck` is unaffected.** It consumes `find_in_progress_commit` to
  count commits *on* `tasks.md` since the transition, where the transition
  commit itself is correctly the origin. Only `compute-review-scope` peels to
  the parent, and it does so at its own call site rather than by changing the
  shared lookup — a shared helper that returned different commits to its two
  callers would be the drift this keeps out.
- **The report records the base it used.** `diff-base` in `review.md`'s
  frontmatter is the resolved base, so a reader can always tell which window a
  verdict covers; this scenario changes which commit that is, not whether it is
  stated.
- **`--since=<sha>~1` remains available** and is what the operator used before
  this shipped. It stays the escape hatch for widening a window deliberately.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
