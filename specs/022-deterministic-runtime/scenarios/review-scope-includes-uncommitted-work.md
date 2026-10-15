---
section: "Follow-on scenarios"
---

# Review-scope-includes-uncommitted-work

## Context

`/{project}:review` reviews the **working tree**. Its own instructions say so,
the constitution's `reviewed-against` row calls a commit the same subject "only
on a clean tree", and
[`the-committed-tree-horizon`](the-committed-tree-horizon.md) moved review
staleness to a digest of what the review read for exactly that reason.
`compute-review-scope` decides what the review reads, and it answers from
committed trees. `modified-since` is a `diff-base..HEAD` tree diff, and the
default base comes from the spec's status history. Neither sees anything that
has not been committed.

This is the third instance of the committed-tree horizon. That scenario named
two, `stale_review_block` and `compute-review-scope`'s inbox window, and fixed
both. `modified-since` is the one it did not name, and it is the one that
decides the review's subject.

Reported by an adopter on 0.58.0. They reopened a `done` spec by flipping it to
`in-progress`, which is what `/{project}:amend` writes. Then they edited a
source file, added an untracked test file, and committed nothing:

| Run | `diff-base` | `modified-since` | Edited file | New file |
| --- | --- | --- | --- | --- |
| Default, uncommitted | parent of the spec's first transition, two months earlier | 475 | absent | absent |
| `--since=HEAD`, uncommitted | `HEAD` | 0 | absent | absent |
| Default, after committing the same change | parent of the new commit | 3 | present | present |

The pipeline's natural order is `/{project}:amend` → `/{project}:implement` →
`/{project}:review` → `/{project}:analyze`, then one commit. In that order the
review runs before anything is committed, so its scope leaves out every file
the change touched. The record still reads "examined N of M", a complete
examination of a subject that never contained the change. The workaround was to
list the changed files under the plan's Affected Files, which the scope always
includes.

Two defects, independent of each other:

- **The changed-file set is committed-only.** Staged edits, unstaged edits and
  new files are all missing from it.
- **The base ignores an uncommitted transition.** With the reopen still
  uncommitted, the status history's newest transition into `in-progress` is the
  spec's first, months back. The window then spans hundreds of unrelated files.

It went unnoticed in this repository because its contributors commit the work
before reviewing it. With a clean tree, `HEAD` and the working tree are the
same subject.

## Behavior

**`modified-since` is the diff from the diff base to the working tree.** One
diff, from the base's tree through the index to the working tree, with
untracked files included and ignored files excluded. That covers committed,
staged, unstaged and new files alike. It has the same shape as the diff
`diff-cross-spec` already takes. Paths are still named from the project root,
a change outside the project directory is still left out, and a deleted file
is still reported by its old path.

**An uncommitted transition puts the base at `HEAD`.** When `spec.md` on disk
reads `in-progress` and the spec at `HEAD` does not, the transition that opened
this window has not been committed yet, so the window starts at `HEAD` and
`diff-base` is `HEAD`'s sha. "Does not" covers `done`, `planned`, any other
status, and a spec that is absent at `HEAD`. This one rule covers both ways a
spec enters `in-progress`: `/{project}:amend` reopening a `done` spec, and
`/{project}:implement` starting a `planned` one. In every other case the base
resolves as it does today: the parent of the newest committed transition, or
empty when there is none. `find_in_progress_commit` is unchanged, because
`check-stuck` shares it.

**The two halves do different jobs.** The first is the correctness fix. Any
base at or before `HEAD` now puts the uncommitted change in scope, so the
change is reviewed even under a wrong base. The second is a precision fix. It
keeps the window to the change, rather than to every file touched since the
spec first went `in-progress`.

**`--since` is still used verbatim**, and it now diffs to the working tree
too, so `--since=HEAD` yields exactly the uncommitted work.

**The recorded denominator follows.** `write-review` recomputes the scope from
the same function with the recorded `diff-base`, so the record's `scope` counts
the files the review was given. A base recorded as a sha keeps the window the
same across a commit. A diff from that sha to the working tree returns the same
files before and after the work is committed.

## Edge Cases

- **Untracked files that are not ignored enter the scope**, such as scratch
  files and unignored build output. This over-includes, in the direction
  [`review-scope-union`](review-scope-union.md) and
  [`review-base-includes-the-transition-commit`](review-base-includes-the-transition-commit.md)
  already chose. A review that reads extra files is slower; one that misses the
  change is wrong. Ignored files stay out.
- **An uncommitted `in-progress → done` flip leaves the base alone.** The spec
  on disk reads `done`, so the uncommitted-transition rule does not fire, and
  the base is the committed transition's parent, as before. A rule keyed on
  "the status differs from `HEAD`" would move that base to `HEAD` and drop the
  committed work from the window.
- **On-disk and `HEAD` both read `in-progress`:** the ordinary case, and
  unchanged.
- **The spec's own uncommitted files enter `modified-since`** (`spec.md`,
  `tasks.md`, the `review.md` being written), as they already do once
  committed. Nothing new in kind.
- **A spec whose on-disk status cannot be read** gives no evidence of an
  uncommitted transition, so the base resolves from history as before. Under
  the first half the change is still in scope, so the cost is a wider window,
  never a missed change.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
