---
section: "Record freshness"
---

# A-renamed-file-contributes-its-rewrites

## Context

Review staleness was judged by two implementations of one rule: Family 19
(`scripts/audit/review-freshness.sh`) and the transition gate's
`SweepIndex::build` (`runtime/src/primitives/mechanical_sweep.rs`).
[review-staleness-on-done-specs](review-staleness-on-done-specs.md) records why
there were two, and why `mechanical_sweep_parity` held them together until
Family 19 came to call the runtime (066 plan, *Family 19 calls the runtime*).

They still built different repo-wide rewrite sets. Family 19 ran `git diff`,
which detected renames under git's default config, so a renamed file was diffed
against its old path and its substitutions counted toward the repo-wide pairs.
`SweepIndex::build` used git2's tree diff with no rename detection, so the same
file was a delete plus an add and contributed nothing.

This surfaced on 2026-10-05 on PR #5's `063` → `064` / `064` → `065` renumber,
where it was why the halves disagreed. The chaining that turned that
disagreement into a wrong verdict had been fixed
([a-multi-number-renumber-is-one-rewrite](a-multi-number-renumber-is-one-rewrite.md));
the disagreement itself had not. The parity test did not catch it because it
compared the real corpus, which carried no renamed-file case.

## Behavior

**The exemption detects renames.** A file present at the base under one path
and at the head under another, at or above the similarity threshold, is diffed
as one file, and its line substitutions count toward the repo-wide pairs
exactly as any other file's do. A renamed file is the same artifact, and in a
renumber its own links are where the sweep's rewrite lands.

**It inherits no setting.** The similarity threshold (50%) and the rename
limit (10,000) are `RENAME_THRESHOLD` and `RENAME_LIMIT` in
`mechanical_sweep.rs`, not libgit2's defaults or a contributor's git config,
so the verdict does not depend on who asks. Family 19 asks this same code
(066 plan, *Family 19 calls the runtime*); while it carried its own copy,
`diff.renames=false` turned that copy's detection off, and a parity test held
its values to the constants.

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
- **A record with no `reviewed-digest`** asks no rename question: it is
  undeterminable, since Family 19's commit-diff proxy, which listed both sides
  of a move, went with its copy of the rule.
- **The rule is 066's.** It is stated here, as
  [a-multi-number-renumber-is-one-rewrite](a-multi-number-renumber-is-one-rewrite.md)
  states its own, and Family 19 reaches it through `check-review-freshness`.
  [family-19-mechanical-sweep-exemption](../../026-framework-self-audit/scenarios/family-19-mechanical-sweep-exemption.md)
  states the exemption's conditions, not a second implementation of them.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
