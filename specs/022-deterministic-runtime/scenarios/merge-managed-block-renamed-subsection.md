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

**The lookahead is bounded.** It steps over at most three consecutive unmatched
groups, the most any template change has retired at once: the agent removal
retired the Auggie, Antigravity and OpenCode subsections together. An adopter
tail is arbitrarily long, and an unbounded scan would reach a pattern an adopter
pasted far below, decide the block extended that far, and consume everything
above it — reintroducing the deletion the trailing-append scenario exists to
prevent. The bound is small and its value is stated in code.

**Every released template merges cleanly into the shipped one.** Each distinct
`.gitignore` template a release has shipped is a test fixture, and merged into
the current template it must leave nothing below the block. The current
template must itself be one of the fixtures, so a change to it adds itself and
is checked against every older one before it can ship.

**The trailing-append invariant is unchanged.** Adopter content following the
managed block is still never consumed by group alignment: appended canonical
groups have no on-disk match, so no on-disk group after the block aligns, so the
lookahead finds nothing and the walk stops where it always did.

## Edge Cases

- The renamed subsection is the *first* group after the marker: already handled
  by the existing full-rewrite branch (nothing has been consumed yet), and that
  path is unchanged.
- A run of four or more consecutive unmatched groups: past the bound, treated as
  adopter territory and preserved. A template change that retires that many at
  once fails the replay of the released templates, so it raises the bound or
  ships a migration before it can release.
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
- **Is a bound of two enough, with a migration for any release that retires
  more?** No. The agent removal retired three subsections at once and shipped no
  migration, so an upgrade stranded the old block's tail for every adopter
  installed since `gvrn-v0.13.0`: replayed on 2026-11-16 into `d296f1e3`'s
  template, 7 of the 11 distinct released templates left 24 to 34 lines below the
  block, and at a bound of three all 11 merged cleanly. Found by 012 task 12's
  scratch-adopter upgrade from `ductus-v0.59.0`. The owner chose a bound of three
  and the replay of every released template over a migration (2026-11-16): the
  replay closes the class, since a later change that would strand fails before
  release, where a migration covers only the retirement it was written for. The
  accepted cost is that an adopter group within three groups of a coincidental
  canonical match, not two, is consumed. A `--ref=main` adopter whose
  `.gitignore` a run since 2026-11-10 already stranded keeps the old block's
  tail below the new one until it is edited by hand: waived by the owner on
  2026-11-16, since `--ref=main` opts into `main`'s exposure between releases
  (`framework/bootstrap/ductus.md` §Runtime acquisition, Branch 2), no released
  version strands, and every pattern still resolves through the new block.
