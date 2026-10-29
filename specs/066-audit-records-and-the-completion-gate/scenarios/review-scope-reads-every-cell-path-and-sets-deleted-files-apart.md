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
- A path deleted since the diff base does not count toward the denominator `examined` is measured against, and the result still names it, so a deletion is visible to the review without reading as an unread file.

## Edge Cases

- A cell carrying a backticked glob beside a path: each is an entry.
- A cell whose second backticked token is not a path (a command, a flag): not a scope entry.
- A file deleted and recreated since the base: present, so in scope as an ordinary file.
- A plan Affected Files row naming a file the window deleted: deleted, not a missing plan path.

## Open Questions

- Does a path deleted since the diff base leave `scope` and `modified-since` entirely, or stay in `modified-since` and move to a separate `deleted` list that `scope` excludes — amending, either way, `review-scope-includes-uncommitted-work`'s "a deleted file is still reported by its old path" in the same change?

## Resolved Questions

*None yet.*
