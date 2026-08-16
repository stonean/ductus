---
section: "Follow-on scenarios"
---

# Merge-managed-block-renamed-subsection

## Context

`merge-managed-block`'s `line-prefix` style has no end marker, so the extent of
the on-disk block is *inferred*: [`walk_body_extent`] aligns the file's
blank-line-delimited subsections against the canonical block's, and a subsection
is identified by its **pattern** lines — comment wording drifts between releases,
patterns do not.

A subsection whose patterns are all *renamed* therefore matches nothing. The walk
treated that as the end of the block and stopped, which its own doc comment
recorded as a deliberate trade-off: *"a framework group removed or fully renamed
mid-block also stops the walk now; its old content survives below the merged
block instead of being replaced — preservation over consumption when identity is
ambiguous."* That choice was made to fix
[merge-managed-block-trailing-append](merge-managed-block-trailing-append.md),
where consuming an unmatched group deleted adopter content outright.

Observed 2026-08-16, on the real adopter bootstrap spec 048's AC10 called for.
049 renamed `.govern.session.toml` to `/.ductus/session.toml`, and that
subsection carries **no other pattern** — so it matched nothing, the walk stopped
at it, and the entire tail of the old block was stranded below the newly written
one. The cross-boundary dedup pass then removed the stranded *pattern* lines
(they duplicate canonical ones) and left their *comment headers* behind. The
adopter's `.gitignore` ended up carrying a dead `.govern.session.toml` plus
headerless `# IDE` and `# OS` comments — every run, growing no worse but never
converging.

The neighbouring subsection survived only by luck: `.govern-cache/` → `.ductus-cache/`
is also a rename, but its group also lists `*.sqlite` and `*.sqlite-journal`,
which did not change, so it still matched.

Requiring spec: this spec. Surfaced by [048 — Ductus-Acquired Runtime](../../048-govern-acquired-runtime/spec.md) AC10.

## Behavior

**A retired subsection is distinguishable from adopter content by what follows
it.** When an on-disk group matches no canonical group, the walk looks ahead: if
a later on-disk group still aligns with a remaining canonical group, the block
has not ended — the unmatched group is a retired framework subsection sitting
*inside* it, and is consumed and replaced. When nothing later aligns, the group
is the first adopter group past the block and the walk stops, exactly as before.

**The lookahead is bounded.** A framework release retires a subsection or two at
a time; an adopter tail is arbitrarily long. An unbounded scan would reach a
pattern an adopter pasted far below, decide the block extended that far, and
consume everything above it — reintroducing the deletion the trailing-append
scenario exists to prevent. The bound is small and its value is stated in code.

**The trailing-append invariant is unchanged.** Adopter content following the
managed block is still never consumed by group alignment: appended canonical
groups have no on-disk match, so no on-disk group after the block aligns, so the
lookahead finds nothing and the walk stops where it always did.

## Edge Cases

- The renamed subsection is the *first* group after the marker: already handled
  by the existing full-rewrite branch (nothing has been consumed yet), and that
  path is unchanged.
- A run of three or more consecutive unmatched groups: past the bound, treated as
  adopter territory and preserved. A framework release that retires that many
  subsections at once should ship a migration rather than rely on inference.
- An adopter group that shares a pattern with a canonical group and sits within
  the bound: consumed. This is the accepted cost of the bound, and it is narrow —
  the dedup pass already deletes exact pattern duplicates found in adopter
  territory, so for the common paste-a-duplicate case the end state is the same.
- A subsection renamed but retaining one unchanged pattern (`.govern-cache/`
  alongside `*.sqlite`): matches on the surviving pattern and never reaches the
  lookahead.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Why not add an explicit end marker to `line-prefix`?** It would make the
  extent exact instead of inferred, and remove this class of defect entirely —
  but every adopter's `.gitignore` already carries the marker-less form, so the
  new terminator would have to be introduced by a migration that first needs to
  infer the extent it is trying to replace. The inference has to be correct
  either way; adding the marker is a separate change that this one does not
  block.
