---
section: "Fold-back on merge"
---

# A-link-inside-the-target-stays-inside-it

## Context

`rewrite-spec-links` re-points a link naming the retiring directory by replacing that one segment with the target's and keeping the rest. From any other spec that is right. From a file inside the target's directory it is not: in `{target}/spec.md`, `../{from}/scenarios/x.md` became `../{target}/scenarios/x.md`, a link from the target to itself. `derive-dependencies` records a self-link rather than stripping it, so its cycle check reports a 1-cycle and the pre-commit hook refuses the commit.

A target that absorbs a spec usually links it. So every `/{project}:consolidate` into such a target ended in a commit the hook refused, in every flow ([052 — Spec consolidation](../../052-spec-supersession-and-consolidation/spec.md), scenario `the-content-move-and-the-consolidation-commit-together`). A fold whose upstream links its staging spec does the same. Three of this primitive's own tests asserted the self-link form, because their fixtures used the target as the referrer.

The same report found a second gap: the rewrite moves pointers and nothing else, which is its contract, but its result gave the caller nothing to report about what is left. That residue is a re-pointed link's label naming the removed spec, prose and code spans naming it by directory name or by number, and a hand-authored `cross-spec-impact:` entry naming it.

## Behavior

- **A link inside the target is written relative to its own file.** This applies to a link in a file inside the fold target's directory that climbs to the spec root (one `..` per directory between the file and the root) and then descends into the retiring directory:
  - From `{target}/spec.md`, `../{from}/scenarios/x.md` becomes `scenarios/x.md`, and `../{from}/spec.md` becomes `spec.md`.
  - From `{target}/scenarios/y.md`, `../../{from}/spec.md` becomes `../spec.md`.
  - A link naming the directory itself becomes `./` from the target's top level and `..` from a scenario.
  - On a scenario target, the collapsed link is made local the same way.

  The tail, its fragment, and whether the reference is listed in `kept-tails` are all unchanged; only the prefix changes.
- **Any other link form is re-pointed as written**, such as a root-absolute `/{specs-root}/{from}/…`, which `derive-dependencies` does not read as a sibling link.
- **The result gains `mentions`**: each line of an examined file that still names the retiring directory once the rewrite has run, as a whole token, by:
  - its directory name, or
  - the number it is cited by: `NNN` for a sequential directory, `{identifier}.{n}` for a branch-scoped one.

  A longer name (`050-alpha-cache`), a rule ID (`BE-ERR-050`) and a version (`0.050`) are not matches. The frontmatter's `dependencies:` and `references:` entries are not listed, since the pre-commit hook regenerates them. Only the examined files are searched, so docs, code and every other file are not, and the field's description says so.
- **`/{project}:fold` step 14 and `/{project}:consolidate` step 7 report `mentions`** as the sweep the operator owes.

## Edge Cases

- **A link that climbs too far or not far enough** for its depth, such as `../../{from}/spec.md` from `{target}/spec.md`, is not the climbing form, so it is re-pointed as written; it was broken before the run.
- **An unrelated line shares the number**, such as "100 files" when the retiring directory is `100-…`: it is listed. `mentions` is a token match, and the operator judges each line.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
