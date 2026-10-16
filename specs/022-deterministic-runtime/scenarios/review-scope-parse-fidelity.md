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

The captured-issues half of this scenario was removed by 058: neither `compute-review-scope` nor `diff-cross-spec` computes an inbox window any longer.

**A table ends at a non-table line.** `parse_affected_files` resets its header state when it leaves a table, so a section holding several tables parses each one correctly and no header row is ever emitted as a path.

**A qualified cell yields its backticked path.** When a first cell contains a backticked span, that span is the path; the surrounding prose is dropped. A cell with no backticks falls back to its trimmed text, unchanged.

## Edge Cases

- A section with exactly one table is unaffected — the reset only fires where a table has already ended.
- A cell holding several backticked spans yields the first, which is the path; later spans are qualifiers by position.
- The scope is the union of the plan-affected set and the modified-since set, so an accurate small set is never dropped for a larger one (`review-scope-union`).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
