---
spec: 051-branch-scoped-spec-numbering
reviewed-at: 2026-08-29T18:45:34Z
reviewed-against: 6efa5022957594373c5074796c19814acde627c2
diff-base: c3ed65fec51210b39f8212ab26117d6f738eaec8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 051-branch-scoped-spec-numbering

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking. One captured issue outstanding, one observation recorded.

**One MUST was found and fixed inside this window rather than left to block.** The security pass — run first against this code as part of 022's re-review, whose scope covers the same runtime files — found that `build_route_fold_request` read the fold target with a raw `fs::read_to_string` on a path built by joining the `folds-into` frontmatter value onto the spec root, bypassing the `read_repo_file` containment check it used for the source spec two lines earlier. `folds-into` is hand-authored, nothing guarantees `validate-frontmatter` ran before a fold, and the content read is shipped to the host, so a target carrying `../` lifted a `spec.md` from outside the repo into the payload. BE-INPUT-004 is a MUST; fixed in `9be5b00` with a regression test that plants a file outside the repo root and points a traversing target at it. The counts are zero because they state what is outstanding.

**The one irreversible primitive is the one most carefully guarded.** `retire-feature` validates both arguments for traversal, refuses the sequential form *before* touching the filesystem, and requires the target to hold a `spec.md` — so no ordering of bad input reaches `remove_dir_all` against a spec that should survive, and a target directory that exists but holds nothing is correctly not a home. Already-absent is a domain outcome rather than an error, which is what lets an interrupted fold converge on re-run instead of halting.

**`rewrite-spec-links` matches by whole path segment**, so `1234.1-widget` never re-points a link inside `1234.1-widget-cache`. That is the right trade: a wrong rewrite is silent and permanent, while a link left dangling is exactly what `check-orphaned-references` exists to report. Frontmatter indexes are left to the generators rather than hand-edited, so there is no second writer for `dependencies:` / `references:`.

**Claim honesty (QUAL-CLAIM-001) holds across the new surfaces.** `check-unfolded-specs` counts `examined` before the form check and halts on a branch-scoped directory whose `spec.md` cannot be read rather than skipping it, so an empty result reads as "walked the corpus, none staged" and never as "walked nothing". `rewrite-spec-links` reports `examined` alongside `rewritten`. `dashboard` reports an unresolvable fold target as *not in this tree* rather than *missing*, which is the honest claim for a check that can only see one branch.

**The sequential form's own counter was repaired in-window.** `create-feature` formatted `{number:03}` — a minimum width — while `parse_feature_dir` demanded exactly three digits, so the 1000th spec in a corpus would have been created successfully and then been invisible to every corpus reader. Fixed with the mapping kept injective (padding beyond the minimum is rejected) and covered at the parse, at creation, and at resolution.

Reuse, efficiency, and simplicity found nothing against the loaded rules. One thing that maps to no rule is recorded as an observation: `/ductus:fold`'s corpus-wide link rewrite runs before the step that enforces the fold target's existence.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] convention: rewrite-spec-links rewrites a file line-wise with `lines()` + a bare `\n`, so a rewrite on a CRLF checkout converts the whole file to LF — a one-link change lands as a whole-file diff. Two siblings preserve the ending deliberately (create_feature::stamp_fold_target detects `\r\n`; derive_references picks its line_ending the same way) and check_stuck carries CRLF regression tests, so the convention is established and this is the one writer that departs from it. A shared line-ending-preserving rewrite helper would settle it in one place rather than three. — `runtime/src/primitives/rewrite_spec_links.rs` (captured during review of 022-deterministic-runtime)

## Observations

- other: /ductus:fold rewrites corpus-wide links (step 10) before retire-feature enforces that the fold target exists (step 11), and step 11's own prose names that refusal as the answer to an unresolved folds-into — so the refusal is documented as reachable from a state where inbound links have already been re-pointed at a spec that does not exist, leaving the corpus edited and the staging directory still present. The window looks narrow (the body-edit write at step 6 and create-scenario at step 7 both need the target to exist, so most routes fail earlier), but narrow-by-accident is not the same as closed, and AC29 promises fully-folded-or-untouched per spec. Either check the target before the rewrite, or record why step 11 cannot fire once step 10 has run. — `framework/commands/fold.md`

## Skipped passes

*None.*
