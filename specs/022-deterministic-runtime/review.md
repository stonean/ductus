---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-29T18:43:37Z
reviewed-against: 9be5b00d9b45651578dadfa0de6410495e50b048
diff-base: 7e98cc48963acaad87b9c2d86071bc8d5eaa5c27
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking. One observation captured to the inbox.

**Window.** The diff base is unchanged from the previous run (`7e98cc4`), which reported clean at `ca473fd`; this run therefore concentrated on the ~7,200 lines added since that review — spec 051's branch-scoped numbering work end to end, plus `check-review-agreement` (audit Family 31) and the fold command surfaces.

**Security found one MUST and it was fixed inside this run rather than recorded as blocking.** `build_route_fold_request` read the *source* spec through `read_repo_file` — canonicalize plus containment against the repo root — and then read the *target* spec two lines later with a raw `fs::read_to_string` on a path built by joining the `folds-into` frontmatter value onto the spec root. That value is hand-authored, nothing guarantees `validate-frontmatter` ran before a fold, and the shape check it performs is not a containment check; a `folds-into` carrying `../` resolved outside the repo and the file it named was placed in `target-content` and shipped to the host. BE-INPUT-004 is a MUST, the helper that satisfies it was already in use one read earlier, and the fix was to stop bypassing it (`9be5b00`, with a regression test that plants a `spec.md` outside the repo root and points a traversing target at it). The finding is recorded here and in that commit rather than in the counts, because the counts state what is outstanding.

**The rest of the new surface holds.** `retire-feature` — the only irreversible primitive added — validates both arguments for traversal, refuses the sequential form before touching the filesystem, and requires the target to hold a `spec.md`, so no ordering of bad input reaches `remove_dir_all` against a spec that should survive; already-absent is a domain outcome, so an interrupted fold converges on re-run. `rewrite-spec-links` matches by whole path segment rather than prefix, which is what stops `1234.1-widget` re-pointing links inside `1234.1-widget-cache` — a wrong rewrite would be worse than the dangling link `check-orphaned-references` can still see. `check-unfolded-specs` halts on a branch-scoped directory whose `spec.md` cannot be read rather than skipping it, and counts `examined` before the form check, so an empty result reads as "walked the corpus and found none staged" rather than "walked nothing".

Reuse, efficiency, and simplicity found nothing against the loaded rules. The one thing worth saying that no rule covers is recorded as an observation: `rewrite-spec-links` is the only file-rewriting primitive that does not preserve CRLF line endings, and the convention it departs from is implemented separately in two siblings — which is itself the argument for a shared helper.

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

- convention: rewrite-spec-links rewrites a file line-wise with `lines()` + a bare `\n`, so a rewrite on a CRLF checkout converts the whole file to LF — a one-link change lands as a whole-file diff. Two siblings preserve the ending deliberately (create_feature::stamp_fold_target detects `\r\n`; derive_references picks its line_ending the same way) and check_stuck carries CRLF regression tests, so the convention is established and this is the one writer that departs from it. A shared line-ending-preserving rewrite helper would settle it in one place rather than three. — `runtime/src/primitives/rewrite_spec_links.rs`

## Skipped passes

*None.*
