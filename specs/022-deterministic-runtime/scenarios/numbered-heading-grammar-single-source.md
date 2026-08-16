---
section: "Follow-on scenarios"
---

# Numbered-heading-grammar-single-source

## Context

Three modules parsed the same `## N. Title` task-heading grammar with their own
private copies. `read_tasks.rs` had `heading_starts_with_number` plus a strict
`split_numbered_heading` returning `Option<(String, String)>`; `prune_tasks.rs`
had `heading_is_numeric` plus a lenient `split_numbered_heading` returning
`(String, String)` that tolerated a missing dot; `primitives/mod.rs`'s phase
scanner had a third copy of the predicate.

The copies agreed today, but nothing held them together. `read_tasks.rs`
documented its duplicate as deliberate — "kept module-local to avoid widening
the crate-internal surface" — which is a real cost, but the three modules all
read *the same tasks file*, so a divergence would let one primitive see a task
another does not. That is the failure the convention was not weighing.

Surfaced as a `QUAL-REUSE` SHOULD violation in
[041-task-pruning's review](../../041-task-pruning/review.md).

## Behavior

`primitives/mod.rs` owns the grammar as a single `pub(crate)` pair:

- `split_numbered_heading(&str) -> Option<(&str, &str)>` — borrowed, not
  allocated, so classifying a heading costs nothing. `None` unless the heading
  opens with decimal digits followed by a literal dot.
- `heading_is_numeric(&str) -> bool` — defined as
  `split_numbered_heading(h).is_some()`, so the predicate cannot drift from the
  splitter.

All three call sites use them. `prune_tasks.rs`'s task branch collapses from a
predicate call followed by a re-parse into a single parse whose `Some` arm *is*
the task test.

The strict contract wins: `prune_tasks`'s lenient copy accepted bare `12` and
returned an empty title, but its only caller was already guarded by the
`N.` predicate, so no reachable behavior depended on the leniency.

## Edge Cases

- **`3 quick wins`** — digits not followed by a dot. Not a task; a prose
  heading must never parse as one. Unchanged from all three copies.
- **`12`** — bare digits, no dot. The `read_tasks` and `mod.rs` copies rejected
  it; the `prune_tasks` copy accepted it with an empty title. The shared
  grammar rejects it. Unreachable from `prune_tasks`'s only call site, which
  tests the same grammar before splitting.
- **`12.`** — dot, no title. A task with an empty title, as before.
- **Non-ASCII titles** — the digit scan is byte-wise over ASCII digits and
  splits at a byte index that is always an ASCII boundary, so a multi-byte
  title is safe.
- **Borrow vs. move** — the borrowed return would extend a borrow of the
  heading across the `if let` in `prune_tasks`, whose else-arm moves that
  heading into the phase-name slot. The call site maps to owned `String`s
  immediately, keeping the borrow dead before the move.
- **Allocation is confined to the branch that keeps the strings.** The first
  cut mapped to owned `String`s before testing the level, so a numbered
  heading *above* the task level — a flat-task remnant in a phased file —
  allocated two `String`s it discarded. `Option<(&str, &str)>` is `Copy`, so
  classification (`is_none()` for the phase test) is allocation-free and only
  the task branch pays. Recorded as a low-confidence efficiency note by
  `/ductus:review` and closed rather than waived: the fix costs one `if` and
  removes the regression against the code this scenario replaced.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
