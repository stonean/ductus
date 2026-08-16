---
section: "Follow-on scenarios"
---

# Migration-chain-reference-integrity

## Context

This spec's Motivation names the failure it exists to prevent: *"Nothing prevents
a maintainer from deleting a template, command, or filename convention without
writing an adopter migration. Adopters silently break on the next pipeline
command."* The registry closed that. It did not close the subtler version, where
the migration **does** exist and a *later* migration invalidates what an
*earlier* one wrote.

Each entry is authored against the layout as it stood at its own
`introduced_in`, and is correct there. Nothing validates the **composition** —
and an adopter far enough behind runs several in one batch, so the composition
is what they actually execute.

Two instances surfaced on 2026-08-16, both during the real adopter bootstrap
[048](../../048-govern-acquired-runtime/spec.md) AC10 called for, and neither
visible to any check this repository runs against itself:

- **`constitution-relocate` (0.24.0) step 4** rewrites `CLAUDE.md`, `AGENTS.md`
  and `README.md` to point at `.govern/constitution.md`. **`ductus-rename`
  (0.28.0) step 2** then moves that file to `.ductus/constitution.md` and, as
  authored, said nothing about those three files. Following both literally left
  a dangling `@import`. Found by the maintainer running the batch by hand.
- **`govern-dir-consolidate` (0.22.0)** moved the shipped generators from
  `scripts/` to `.govern/scripts/` and `ductus-rename` moved them again, while
  the adopter-owned `.githooks/pre-commit` that invokes them was re-pointed by
  neither — leaving a hook calling a script whose targets no longer existed.

Both share one shape. The referrer is an **adopter-owned** file — strategy
`create`, so the manifest never overwrites it, and unpinned, so the
pinned-invoker warnings never fire. A migration is the only mechanism that can
follow a move into one, and each migration knows only its own move.

The failure is silent in the way this project pays most for: nothing errors. A
dangling `@import` yields a constitution that is simply not loaded, and a hook
calling a missing script fails at commit time, far from the run that broke it.

## Behavior

**A migration that moves a path re-points every adopter-owned referrer of it,
including references a previous migration wrote.** This is stated as a rule the
registry's procedure files are authored against, so a new entry is written with
the composition in view rather than only its own hop.

**The batch verifies itself when it finishes.** After the pending migrations
have run, `/{project}` checks that no adopter-owned file left behind by the
batch references a framework-owned path that does not exist. The check is
**derived** — it reads the files and the filesystem — rather than declared by
the migration author, because a per-entry "referrers" list would be exactly the
input `AGENTS.md`'s second Design Principle rules out: correctness would depend
on remembering to fill it in, and the cases where it is forgotten are the cases
where it mattered.

**A dangling reference is reported, not repaired.** The batch names the file,
the missing path, and the migration whose move most likely orphaned it. It does
not guess a replacement: the adopter may have hand-edited the reference, and a
rewrite that guessed wrong would be worse than a report that is precise. The run
does not halt — the migrations that did apply are correct and re-running is
safe.

**The check runs on every batch, not only on chains.** A single migration can
orphan a reference just as a chain can; scoping this to multi-entry batches
would make it a check that runs least often in the case it was written for.

## Edge Cases

- **A converged adopter**: the batch applies nothing, so there is nothing to
  verify and the check emits nothing — not a clean bill of health for files it
  never examined.
- **A reference the adopter hand-edited to something valid**: resolves, so it is
  not reported. The check tests reachability, not conformance to a shipped form.
- **A reference into a path that is legitimately absent** — an optional artifact
  a project never scaffolded: reported, because the check cannot tell it from a
  break. The report names the file and lets the operator judge; a suppression
  list would be the declared input this scenario avoids.
- **A pinned referrer**: still reported. Pinning opts out of framework *writes*,
  not out of being told the file is broken.
- **The check cannot read a file it was going to examine**: reported as
  unexamined rather than passed, per `QUAL-CLAIM-001` — the same distinction the
  rest of this framework's checks draw.

## Open Questions

- Should the check live in the bootstrap procedure, where it sees the batch that
  just ran and can name the responsible migration, or as an `/{project}:audit`
  family, where it runs against this repository's own corpus on every push and
  would catch a *newly authored* composition hazard before it ever reaches an
  adopter? The two catch different things at different times and are not
  alternatives so much as two placements of one rule; the question is whether
  the second is worth its cost given the first.

## Resolved Questions

*None yet.*
