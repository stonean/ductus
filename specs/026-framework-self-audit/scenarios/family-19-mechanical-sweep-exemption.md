---
section: "Follow-on scenarios"
---

# Family-19-mechanical-sweep-exemption

## Context

Required by [049 — Rename govern to ductus](../../049-rename-govern-to-ductus/spec.md), which surfaced the gap the moment its sweep landed.

Family 19 ([family-19-review-freshness](family-19-review-freshness.md)) compares each `done` spec's `review.reviewed-against` against history and emits a finding when a durable contract changed since. It compares *shas*, so it cannot see what kind of change it is looking at.

[§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle) case (a) already rules on that: a uniform token substitution across live artifacts is a **mechanical** edit, which is precisely why a rename sweep does not reopen a `done` spec. A review reads contracts; a contract that changed only in spelling states exactly what it stated before. The two rules have to agree — if the sweep does not reopen the spec, it does not stale the review either.

Left alone, one repo-wide rename turns half the corpus stale at once. 049's sweep produced **22 findings across 48 specs**, every one of them a diff whose changed lines were the rename and nothing else. That is the failure this family's own header already names: the wider rules it rejected during design flagged 42 of 48 and 31 of 48, and were rejected because "both would have been disabled within a week." A gate that fires on half the corpus for a diff nobody needs to read is a gate people route around.

## Behavior

A changed durable contract does **not** stale a review when its diff is a uniform token substitution that the same sweep applied elsewhere. Three conditions, all derived from the diff:

1. **One-for-one lines.** Each changed run replaces the same number of lines it removes. Adding or dropping a line is structural, never a substitution.
2. **One-for-one tokens.** Each replaced line has the same token count as the line it replaced.
3. **Repo-wide.** Every token rewrite in the file is one the sweep also made in another file, *or* a direct consequence of one — applying the repo-wide rewrites to the old token reproduces the new one.

Condition 3 is what carries the weight, and it comes straight from §spec-lifecycle's wording: uniformity is a property "across live artifacts", not within one file. A rename rewrites the same token in many files. A one-cell edit to a data-model table rewrites one token in one file and reads as perfectly uniform on its own — and it is exactly the contract change this family exists to catch.

**Derived, never declared.** The exemption is computed from the diff. A commit trailer, an opt-out flag, or a maintainer-maintained list of sweep commits would each make correctness depend on someone remembering to set it, which `AGENTS.md`'s design principle rules out: the cases where it gets skipped are exactly the cases where it mattered.

**Collapse is allowed.** Two old tokens may rewrite to one new token — 049 sent both `govern` and `gvrn` to `ductus`. Requiring the rewrite to be invertible was tried first and rejected six spec-diffs in. A collapse that is not a rename (`MUST` and `MAY` both rewritten to `SHOULD`) still fails condition 3.

Implementation reads one `git diff --unified=0` per distinct base sha rather than two blob reads per changed file. The blob-per-file shape measured 61s on this repo's history; the diff shape measures under 4s. This family runs as a hard release gate, so its cost is paid on every tag.

## Edge Cases

- **A rename mixed with a reword in the same file** (`govern MUST halt` → `ductus SHOULD halt`) — stale. The `MUST` → `SHOULD` rewrite is not repo-wide and is not derivable from the rename.
- **A token variant appearing in exactly one file** (`gvrn_` → `ductus_`, which occurred once in 032's data model) — not stale. It is a consequence of the repo-wide `gvrn` → `ductus`.
- **A changed table cell** (`| timeout | 30s |` → `| timeout | 60s |`) — stale. Uniform within the file, derivable from nothing.
- **A contract added or deleted since the review** — stale. There is no before-and-after to compare, so it is a real change by construction.
- **Two unrelated sweeps in the same range.** Both contribute pairs to the repo-wide set; a file explained by either is exempt. Neither can explain a genuine edit, so the additional pairs do not weaken the check.
- **A sweep touching exactly one file in the whole range.** Its pairs are not repo-wide, so the file stays stale. Correct: a one-file "sweep" is indistinguishable from an edit, and §spec-lifecycle's rule is about substitutions *across* artifacts.

## Open Questions

*None — all resolved.*
