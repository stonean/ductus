---
spec: 051-branch-scoped-spec-numbering
reviewed-at: 2026-08-29T19:03:18Z
reviewed-against: 4b9b887bfa73309f64c935391e0f5adbd806b53b
diff-base: c3ed65fec51210b39f8212ab26117d6f738eaec8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 051-branch-scoped-spec-numbering

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking. Two captured issues outstanding, both awaiting `/ductus:groom`.

**Re-run against the finished tree.** The first pass of this review recorded its verdict at `6efa502`; tasks 20 and 21 then added a primitive and a result field, and documenting them in `data-model.md` — a durable contract — made that verdict stale by the same rule this spec's own gate applies. This run covers the same window plus that work.

**One MUST was found across the two runs and fixed rather than left to block.** `build_route_fold_request` read the fold target with a raw `fs::read_to_string` on a path built by joining the `folds-into` frontmatter value onto the spec root, bypassing the `read_repo_file` containment check it used for the source spec two lines earlier. `folds-into` is hand-authored, nothing guarantees `validate-frontmatter` ran before a fold, and the content read is shipped to the host, so a target carrying `../` lifted a `spec.md` from outside the repo into the payload. BE-INPUT-004 is a MUST; fixed in `9be5b00` with a regression test that plants a file outside the repo root and points a traversing target at it. The counts are zero because they state what is outstanding.

**The two newest primitives were examined on the same terms as the rest.** `invalidate-review` writes through the same splice `write-review` uses rather than a second frontmatter writer, preserves waivers verbatim — extras included, so an adopter's policy field is not dropped — and treats "no current review" as `invalidated: false` rather than an error, so it converges with the other six per-spec writes. `append-task`'s dedup keys on the whole rendered `scenarios/{slug}.md` pointer through the shared fence-aware scanner, so a prefix slug is still its own task and the template's commented `## 1.` examples stay invisible to it; the guard is scoped to the slug case, leaving a caller-supplied body appending as before.

**The one irreversible primitive remains the most carefully guarded.** `retire-feature` validates both arguments for traversal, refuses the sequential form before touching the filesystem, and requires the target to hold a `spec.md` — so no ordering of bad input reaches `remove_dir_all` against a spec that should survive. `rewrite-spec-links` matches by whole path segment, so a wrong rewrite — silent and permanent — is traded for a dangling link that `check-orphaned-references` can still report.

**Claim honesty (QUAL-CLAIM-001) holds across every new surface.** `check-unfolded-specs` counts `examined` before the form check and halts on an unreadable branch-scoped `spec.md` rather than skipping it; `rewrite-spec-links` reports `examined` beside `rewritten`; `dashboard` reports an unresolvable fold target as *not in this tree* rather than *missing*, which is the honest claim for a check that can see only one branch.

Reuse, efficiency, and simplicity found nothing against the loaded rules. The waiver renderer was extracted from `write-review` for the new primitive rather than copied, which is the reuse pass's own preference applied in the window it was reviewing. Both outstanding observations were recorded in earlier runs and remain in the inbox; neither maps to a loaded rule and neither is a finding.

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
- [ ] other: /ductus:fold rewrites corpus-wide links (step 11) before retire-feature enforces that the fold target exists (step 12), and that step's own prose names its refusal as the answer to an unresolved folds-into — so the refusal is documented as reachable from a state where inbound links have already been re-pointed at a spec that does not exist. The window is narrow (the body-edit write and create-scenario both need the target too), but narrow by accident is not closed. Either check the target before the rewrite, or record why the refusal cannot fire once the rewrite has run. — `framework/commands/fold.md` (captured during review of 051-branch-scoped-spec-numbering)

## Observations

*None.*

## Skipped passes

*None.*
