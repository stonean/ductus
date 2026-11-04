---
section: "The loop"
---

# Fix-round-results-have-a-bounded-size

## Context

`brief-re-read` returns a round's `paths`, `read-re-read` the re-read's `findings`, and `check-prescriptions` its `differing`, `unprescribed` and `failed` lists. Each grows with the round: a fix round that sweeps a rename across the corpus changes as many files as the sweep does, and a re-read's result file is written by an agent with no size limit. No artifact of this spec commits to a maximum size or to pagination, which `BE-PAYLOAD-001` (`framework/rules/performance-backend.md`) requires of any result that returns a collection; this spec's first review (2026-11-04) found it as a MUST violation. 066's scenario `scope-results-have-a-bounded-size` bounds `compute-review-scope` and `diff-cross-spec` the same way, and its `bounded_lists::place` (`runtime/src/primitives/bounded_lists.rs`) is the shared helper.

The routing was the owner's decision (2026-11-04): this spec owns these result shapes (`data-model.md`).

## Behavior

- **Each result of `brief-re-read`, `read-re-read` and `check-prescriptions` has a stated maximum size: the `[artifacts] read-size-bytes` setting, 50,000 bytes by default, of serialized result**, stated in `data-model.md` beside each shape.
- **Up to the bound, the lists are inline, as now. Past it, the result carries each list's count, `form: file` and `lists-file`, the path of a file holding the lists whole**, written through `bounded_lists::place` as 066's two results are. A counts-only result never reads as an empty round, a clean re-read or a matched check: `kind`, `state`, `verdict` and every count are inline in both forms.
- **Nothing a host needs is dropped.** Every list stays reachable whole, and `brief-re-read`'s `round.toml` and brief keep every path whatever form the result takes.

## Edge Cases

- **A result exactly at the bound** fits and is inline.
- **A lists file that cannot be written** fails the call, and its round counts as needing a re-read, as any failed call does (AC15).
- **A malformed `[artifacts] read-size-bytes`** halts the call naming the setting, the value and the file, as every reader of the setting does.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
