---
section: "Derived frontmatter indexes"
---

# Sibling-link-grammar-is-the-shared-one

## Context

Spec `051-branch-scoped-spec-numbering` established `parse_feature_dir` as the single rule that recognizes a feature directory, in both the sequential (`NNN-slug`, three digits as a *minimum*) and branch-scoped (`{identifier}.{n}-slug`) forms. Its AC15 asks that no consumer able to call it carry a copy.

Two consumers carried one anyway. `derive_dependencies::leading_slug` tested for *exactly* three ASCII digits followed by a hyphen, and `derive_references::is_spec_slug` was an independent re-derivation of the same rule. Both therefore disagreed with the shared grammar in both directions at once: a body link to a branch-scoped spec (`1234.1-slug`) and a body link to a four-digit spec (`1000-slug`) each derived **no** edge.

The failure is silent, which is what makes it worth a scenario rather than a patch. An absent dependency looks exactly like a spec that cites nobody — nothing errors, `examined` counts the spec as read, and the frontmatter the pre-commit hook writes is simply missing a row. Three consequences follow: `traverse-deps` inherits the gap through the frontmatter it reads; `scan_line` is `pub(crate)` for the pre-`done` gate's cross-spec-impact back-link matcher, so a declared impact on such a spec could never be discharged; and neither is reachable in this corpus, which holds no branch-scoped and no four-digit directory — so no existing test could have caught it.

## Behavior

**Both generators call the shared grammar.** `leading_slug` takes the first path segment and asks `is_feature_slug`; `is_spec_slug` delegates outright. Neither restates the digit rule, so a future change to the corpus's membership rule reaches both without being copied into them.

**The guard is widened, not removed — and the distinction is the reusable part.** `harvest` writes its result straight into `dependencies:` with no downstream validation against the enumerated corpus, so the predicate is the only thing standing between an ordinary relative link (`../docs/guide.md`) and a false edge. That is what separates these two call sites from the third copy of the same grammar, in `scripts/audit/sibling-coupling.sh`, which was fixed by **deleting** its predicate: that caller matched every candidate against a slug list the runtime had already enumerated, so extracting the segment was strictly safer than re-testing it. Ask which case you are in before reaching for either fix.

**Proven by probe in both directions**, in a scratch repo holding `001-a` linking to `1234.1-staged` and `1000-thousandth`. Before: `dependencies: []` with `updated: []`. After: `dependencies: [1000-thousandth, 1234.1-staged]`. `examined: 3` on both runs, which is what localizes the defect to the target grammar rather than the enumeration.

## Edge Cases

- A link to a directory that is not a feature directory — `../docs/guide.md`, `../notes/a.md`, `../abc-def/spec.md` — still yields no edge. Widening the accepted shape must not turn every relative link into a dependency.
- A four-digit name carrying a leading zero (`0500-slug`) is rejected by the shared grammar, so it is rejected here too, and for the shared grammar's reason: `{number:03}` never emits it, and accepting it would give one number two spellings.
- `scan_line` is shared with the pre-`done` cross-spec-impact back-link matcher, so widening it widens discharge detection in the same change — a declared impact on a branch-scoped spec can now be discharged by the reciprocal link.
- The corpus this ships from holds neither form, so the regression tests carry the whole guarantee; there is no fixture in the live tree that would notice a regression.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
