---
section: "Follow-on scenarios"
---

# Review-scope-parse-fidelity

## Context

`compute-review-scope` resolves what `/{project}:review` actually looks at. Two of its three outputs were wrong in ways that made a review examine the wrong files while reporting normally — the `QUAL-CLAIM-001` shape at the command level rather than the primitive level.

Found by running the command against `017-derive-dont-ask` while preparing that spec's own completion gate.

**The plan-affected set carried junk.** 017's `## Affected Files` section holds eleven tables, one per area. `parse_affected_files` cleared its `saw_header` flag only at an H2 boundary, so after the first separator row the flag stayed set and every later table's `| File | Action |` header was read as a data row — the literal word `File`, eight times. Cells carrying a qualifier after the path (`` `constitution.md` (root) ``) fared no better: trimming stray backticks off the whole cell kept the trailing prose, yielding `constitution.md` (root)` as a "path".

That would be cosmetic if the scope rule ignored it, but the rule is *whichever set is larger*, and the inflated set won — so the review scoped ~50 files that mostly do not exist and missed the ones the work actually changed.

**Captured issues counted comment lines.** The list was every line the inbox diff added. Restoring the shipped `<!-- Rules: … -->` guidance block reported roughly thirty captured issues, one per comment line, including the bare `-` lines inside it.

## Behavior

**A table ends at a non-table line.** `parse_affected_files` resets its header state when it leaves a table, so a section holding several tables parses each one correctly and no header row is ever emitted as a path.

**A qualified cell yields its backticked path.** When a first cell contains a backticked span, that span is the path; the surrounding prose is dropped. A cell with no backticks falls back to its trimmed text, unchanged.

**Captured issues are real inbox bullets — in both primitives.** The added lines are intersected with the bullets the shared comment- and fence-aware grammar finds in the post-image inbox — the same `iter_bullets` / `bullet_text` pair `append-inbox` and `remove-inbox-item` already agree on. The authority for what counts as an item is the file, not the diff.

`diff-cross-spec` carries the same correction. It already filtered with `bullet_text`, which drops a heading or a blank line — but that test is line-local and accepts a bullet line sitting inside an HTML comment, so its `inbox-additions` (what `/{project}:implement` reports per task) had the weaker form of the same defect. Both primitives now intersect against the post-image file's real bullets.

## Edge Cases

- A section with exactly one table is unaffected — the reset only fires where a table has already ended.
- A cell holding several backticked spans yields the first, which is the path; later spans are qualifiers by position.
- An inbox absent from the post-image tree, or not valid UTF-8 there, yields no captured issues rather than an error — nothing can be proven about a file that cannot be read.
- A bullet added and then removed within the same window does not appear: it is absent from the post-image file, which is what the intersection tests against.
- An item whose text was reworded in the window appears once, under its new text — the added line and the post-image bullet agree, which is the intended pairing.
- The "larger set wins" scope rule is left as-is. It was the *inflated* set that made it harmful, and with the parse corrected the rule selects on real content again; whether an accurate small set should beat a large one is a separate question this scenario does not settle.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
