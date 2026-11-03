---
section: "Audit checks"
---

# Deprecation-labels-are-read-by-the-runtime

## Context

026's Family 40 (`scripts/audit/deprecation-removal.sh`, scenario `family-40-deprecated-rules-are-removed-on-time`) holds each deprecated rule to the release its `**DEPRECATED in {version}:**` label names. It finds labels with its own parse, a copy of `check-rule-ids`' `section_is_deprecated` (`runtime/src/primitives/check_rule_ids.rs`), and reads each label's `removed in X.Y.Z` with a grammar only it applies, the form 008's `data-model.md` states (Deprecated field). Constitution §runtime-boundary makes a deterministic capability a primitive, which audits call instead of reparsing markdown, since each copy of a tested parse drifts from it. The two copies already had to be brought into line once: before its first re-read (077 task 8; `62814405`), Family 40 missed a blockquoted or indented label the runtime counted.

## Behavior

A project-scoped primitive takes rule files and returns one result for each line containing `**DEPRECATED`. Each result gives the line's file and line number, and the rules it labels, those whose section `section_is_deprecated` reads it in first. It also gives the rules in whose section it follows another label, and either its deprecating version and removal target or the reason it is malformed, by the rules Family 40's scenario states. A line in no rule section is a result with neither list, so Family 40's finding for a label outside any rule section survives the move. `check-rule-ids` and the primitive share one label reader, so `check-rule-ids`' `deprecated` result and the primitive's list cannot disagree.

Family 40 becomes the primitive's entry point, the Family 30 shape: it keeps the comparison with `version` and the list of files citing a due rule, and drops its own parse. Its test's cases carry over to the entry point unchanged, its fixture builder supplying the runtime binary the entry point calls.

## Edge Cases

- **A malformed label is a result, not an error**, so the family still reports each one, and one malformed label never hides the others.
- **A rule file that cannot be read is reported and not counted as examined**, never skipped, and the other files are still read, as Family 40 reports it today.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
