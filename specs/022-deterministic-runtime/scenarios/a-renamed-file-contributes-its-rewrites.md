---
section: "Follow-on scenarios"
---

# A-renamed-file-contributes-its-rewrites

## Context

Review staleness is judged by two implementations of one rule: Family 19
(`scripts/audit/review-freshness.sh`) and the transition gate's
`SweepIndex::build` (`runtime/src/primitives/mechanical_sweep.rs`).
[review-staleness-on-done-specs](review-staleness-on-done-specs.md) records why
there are two, and why `mechanical_sweep_parity` exists to keep them from
drifting apart unnoticed.

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

**Both halves detect renames.** A file present at the base under one path and
at the head under another, at or above the similarity threshold, is diffed as
one file, and its line substitutions count toward the repo-wide pairs exactly
as any other file's do. A renamed file is the same artifact, and in a renumber
its own links are where the sweep's rewrite lands.

**Neither half inherits the setting.** The similarity threshold (50%) and the
rename limit (10,000) are stated in each half — `RENAME_THRESHOLD` and
`RENAME_LIMIT` in `mechanical_sweep.rs` and in Family 19's script — rather than
taken from git or libgit2, whose defaults are each overridable from a
contributor's config: `diff.renames=false` turned Family 19's detection off.
A parity test reads the script and fails when its values drift from the Rust
constants.

**A window over the limit is unreadable in both, by the same count.** The
libraries apply a rename limit differently — git skips inexact detection once
sources × destinations exceeds the limit squared, libgit2 caps the sources it
tries per file — so matching values alone would still let the halves degrade
apart. Instead each half counts the window's changed markdown paths before
renames pair them (`SweepIndex` from the diff's deltas, Family 19 from its
`diff --git` headers plus its `rename from` lines) and, above the limit, grants
no exemption. Below it neither library's limit can bind, and the value sits far
above this corpus's size.

**`mechanical_sweep_parity` pins it with a fixture.** A renamed-directory
fixture — a sweep that renumbers a spec directory and rewrites the references
inside it — asserts both halves exempt the renamed contracts, and still read a
structural edit among them as stale. It fails if either half falls back to a
delete plus an add, and its repository is configured against both halves (rename
detection off, a rename limit of one, no diff prefixes, colour forced on), so a
half that inherited any of it fails too.

## Edge Cases

- **Similarity is scored by two algorithms.** libgit2 and git do not compute
  the same similarity index, so a file near the threshold can be a rename in
  one half and a delete plus an add in the other. The renamed files a sweep
  produces sit far above it — a renumber rewrites a few lines of a file — and
  where the halves disagree one gate is the stricter, so nothing passes both
  unexamined. This residue is stated in `mechanical_sweep.rs`, not closed.
- **A rename with its content unchanged** has no hunk, so it is absent from
  both indexes and reads as changed in both. The Rust half used to open a
  file's entry at its header, which made such a file an empty — exempt — entry
  there while Family 19, which opens one on the `+++ b/` line only a file with
  hunks carries, called it changed. Entries now open on the first hunk in both.
- **Copy detection stays off in both.** A copied file is new content, not a
  rewritten one, and turning copies on in one half only would reopen the same
  disagreement. An explicit `--find-renames` also overrides
  `diff.renames=copies`.
- **A contributor's config reshaped more than detection.** Colour forced on, an
  external diff driver, textconv, and `diff.noprefix` each change the output
  Family 19 parses, so its diff pins all four (`--no-color`, `--no-ext-diff`,
  `--no-textconv`, explicit `a/` and `b/` prefixes). It is the same horizon
  problem one level down: a verdict that depends on who ran it.
- **Family 19's proxy arm lists both sides of a move.** A review record with no
  `reviewed-digest` is judged by a second diff, `--name-only`, whose paths
  are the candidates rather than the source of pairs. It passes
  `--no-renames`, so a contract renamed within the spec is listed under its
  old path too, and that path — absent from the index — reads as changed: the
  digest arm's verdict on the same move, whose old key has vanished. Left to
  `diff.renames`, a contributor with renames on saw only the new path, which
  the sweep exemption could clear. `--name-only` output is never coloured, so
  colour needs no pin there.
- **The rule is 022's, though one half runs in 026's script.** The shared
  rewrite rule is stated here for both halves, as
  [a-multi-number-renumber-is-one-rewrite](a-multi-number-renumber-is-one-rewrite.md)
  states its own, and `mechanical_sweep_parity` is what binds them. 026 owes no
  change:
  [family-19-mechanical-sweep-exemption](../../026-framework-self-audit/scenarios/family-19-mechanical-sweep-exemption.md)
  describes the read shape — one `git diff --unified=0` per distinct base sha —
  which an added flag leaves true, and says nothing about rename detection.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
