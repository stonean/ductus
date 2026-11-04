---
section: "The review record"
---

# Review-scope-reads-every-cell-path-and-sets-deleted-files-apart

## Context

`compute-review-scope` derives two wrong scopes (0.59.11 and HEAD f24c55c6, 2026-10-25).

- An Affected Files cell naming two backticked paths (`` `a.md` / `b.md` ``) contributes only the first to `plan-affected`, so the second is reviewed only when it also changed since the diff base. Scenario `review-scope-plan-affected-table-format` settled table-versus-bullet parsing, not a cell holding several paths.
- A file deleted since the diff base is listed in `scope` and `modified-since`, as scenario `review-scope-includes-uncommitted-work` decided ("a deleted file is still reported by its old path"). It cannot be read, so a review that reads every readable file still records `examined` short of `scope`, and the shortfall reads as files left unread.

## Behavior

- Every backticked path in an Affected Files cell joins `plan-affected`, whatever separates them.
- A path deleted since the diff base does not count toward the denominator `examined` is measured against, and the result still names it, so a deletion is visible to the review without reading as an unread file: it is in `modified-since` and in `deleted`, and not in `scope`.

## Edge Cases

- A cell carrying a backticked glob beside a path: each is an entry.
- A cell whose second backticked token is not a path (a command, a flag): not a scope entry. A later span is a path when it holds no whitespace and starts with none of `-`, `/` and `§`.
- A later span written relative to an earlier one (`dashboard.rs` after a `runtime/src/primitives/` path) is read from the project root as written, as a first span is, so it is a scope entry no file matches.
- A file deleted and recreated since the base: present, so in scope as an ordinary file.
- A plan Affected Files row naming a file the window deleted: deleted, not a missing plan path, so it is in `deleted` and out of `scope` while `plan-affected` still lists it.
- A deletion not yet committed, staged or not: deleted, since the window reaches the working tree.
- A file moved with `git mv`: its old path is deleted and its new path changed, since the diff detects no renames.
- A path deleted in the index but present on disk: present, since the working tree decides.
- A window with no deletion: `deleted` is empty and `deleted-count` is zero.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does a path deleted since the diff base leave `scope` and `modified-since` entirely, or move to a separate list?** It stays in `modified-since`, is named in a new `deleted` list, and leaves `scope`, so the denominator holds only files a review can read while the deletion stays visible (owner, 2026-11-04). `deleted` carries a `deleted-count` and moves to the list file past the read size as the other lists do (scenario `scope-results-have-a-bounded-size`), and `review-scope-includes-uncommitted-work`'s "a deleted file is still reported by its old path" is amended to match in the same change. Rejected: dropping the path from both lists, which leaves an unplanned deletion nowhere in the result. Measured 2026-11-04 over the 37 `in-progress` specs, with the release binary's `compute-review-scope` at default arguments: 12 windows hold one `modified-since` path absent from the working tree.
