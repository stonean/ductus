---
section: "Follow-on scenarios"
---

# Sibling-symlink-trust-boundary

## Context

`check-artifacts`'s `link-adjacent-drift` family resolves sibling links
lexically: `..` is consumed by `PathBuf::pop` and the result must
`starts_with(feature_dir)`. Canonicalization was rejected deliberately — it
fails on a legitimately-missing target and makes the answer depend on where a
link points, which would break the repeat-run determinism AC8 requires.

Lexical resolution performs the containment half of `BE-INPUT-004` correctly: a
link *target* like `../../../etc/passwd` is refused. What it cannot see is a
symlink committed **inside** the feature directory — `scenarios/evil.md ->
/etc/shadow` resolves lexically inside the base and is then opened.

Recorded as a low-confidence finding on
[045](../../045-decision-state-drift-detection/review.md), scoped to
applicability rather than mechanism: the hrefs come from repo-committed
markdown carrying the same trust as the source, the primitive runs locally
against the operator's own checkout, and the content is discarded after a
readability test. The finding's own remedy was to either close it or record the
trust boundary as a decision. This closes it, because the guard costs one
`symlink_metadata` call and does not trade away determinism.

## Behavior

`traverses_symlink(target, base)` walks each component of `target` at or below
`base` and returns `true` when any is a symbolic link. It uses
`std::fs::symlink_metadata`, which does not follow links, so the answer depends
only on *whether* a component is a link — never on its destination. Repeat-run
determinism is preserved.

Two gates use it:

1. The up-front scenario-readability pass — the only place an href-reachable
   file is actually opened — tests for a link *before* the read. A linked entry
   is never opened, so its destination is never touched.
2. `read_target_state` tests before `is_file()`, which follows links. A
   symlinked sibling is reported as `target-unparseable` rather than read
   through.

`target-unparseable` is an existing member of the closed skip-reason set, so no
result shape changes. The honesty contract holds: the target lands in `skipped`
rather than being silently dropped, so a feature containing one cannot report
as fully scanned.

## Edge Cases

- **Symlinked intermediate directory** — `scenarios/` itself being a link is
  caught; the walk tests every component below the feature directory, not just
  the leaf.
- **Symlink pointing *inside* the feature directory** — also refused. The test
  is on the link, not its destination; resolving the destination to decide
  would reintroduce exactly the path-dependence determinism forbids. A
  same-directory link is not an authoring form ductus uses.
- **Missing component** — cannot be a link; the walk stops and the existing
  `target-missing` outcome reports it.
- **Target outside `base`** — `strip_prefix` fails and the walk returns
  `false`; containment is `resolve_sibling`'s job and it has already run.
- **Windows** — `symlink_metadata` is cross-platform; the regression test is
  `#[cfg(unix)]` only because creating the fixture link is.
- **A legitimately symlinked artifact** — would now be skipped rather than
  scanned. No ductus artifact is authored this way, and the skip is visible in
  `skipped` rather than silent, so the case is diagnosable if it ever arises.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
