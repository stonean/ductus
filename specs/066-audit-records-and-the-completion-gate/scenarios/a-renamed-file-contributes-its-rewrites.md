---
section: "Record freshness"
---

# A-renamed-file-contributes-its-rewrites

## Context

The mechanical-sweep exemption, which
[review-staleness-on-done-specs](review-staleness-on-done-specs.md) states,
builds the window's repo-wide rewrite set in `SweepIndex::build`
(`runtime/src/primitives/mechanical_sweep.rs`) from a tree diff. With no rename
detection, a renamed file was a delete plus an add and contributed nothing to
that set, though in a renumber the file's own links are where the sweep's
rewrite lands.

This surfaced on 2026-10-05 on PR #5's `063` → `064` / `064` → `065` renumber.
The chaining that turned a missed rewrite into a wrong verdict had been fixed
([a-multi-number-renumber-is-one-rewrite](a-multi-number-renumber-is-one-rewrite.md));
the missed rewrite itself had not. No test caught it, since the real corpus
carried no renamed-file case.

## Behavior

**The exemption detects renames.** A file present at the base under one path
and at the head under another, at or above the similarity threshold, is diffed
as one file, and its line substitutions count toward the repo-wide pairs
exactly as any other file's do. A renamed file is the same artifact, and in a
renumber its own links are where the sweep's rewrite lands.

**It inherits no setting.** The similarity threshold (50%) and the rename
limit (10,000) are `RENAME_THRESHOLD` and `RENAME_LIMIT` in
`mechanical_sweep.rs`, not libgit2's defaults or a contributor's git config,
so the verdict does not depend on who asks.

**A window over the limit is unreadable.** `SweepIndex` counts the window's
changed markdown paths before renames pair them and, above the limit, grants
no exemption. Below it libgit2's own limit cannot bind, and the value sits far
above this corpus's size.

**`mechanical_sweep_parity` pins it with a fixture.** A renamed-directory
fixture — a sweep that renumbers a spec directory and rewrites the references
inside it — asserts the renamed contracts are exempt and a structural edit
among them still reads as stale. It fails if the index falls back to a delete
plus an add, and its repository is configured against it (rename detection
off, a rename limit of one, no diff prefixes, colour forced on), so an index
that inherited any of it fails too.

## Edge Cases

- **A rename with its content unchanged** has no hunk, so it is absent from
  the index and reads as changed. Opening a file's entry at its header made
  such a file an empty, and so exempt, entry; entries open on the first hunk.
- **Copy detection stays off.** A copied file is new content, not a rewritten
  one.
- **A record with no `reviewed-digest`** asks no rename question: it adds no
  contract to a review's scope.
- **The rule is 066's.** It is stated here, as
  [a-multi-number-renumber-is-one-rewrite](a-multi-number-renumber-is-one-rewrite.md)
  states its own.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
