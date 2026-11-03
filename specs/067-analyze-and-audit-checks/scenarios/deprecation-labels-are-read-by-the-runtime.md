---
section: "Audit checks"
---

# Deprecation-labels-are-read-by-the-runtime

## Context

026's Family 40 (`scripts/audit/deprecation-removal.sh`, scenario `family-40-deprecated-rules-are-removed-on-time`) holds each deprecated rule to the release its `**DEPRECATED in {version}:**` label names. It finds labels with its own parse, a copy of `check-rule-ids`' `section_is_deprecated` (`runtime/src/primitives/check_rule_ids.rs`), and reads each label's `removed in X.Y.Z` with a grammar only it applies, the form 008's `data-model.md` states (Deprecated field). Constitution §runtime-boundary makes a deterministic capability a primitive, which audits call instead of reparsing markdown, since each copy of a tested parse drifts from it. The two copies already had to be brought into line once: before 077's task 8, Family 40 missed a blockquoted or indented label the runtime counted (owner, 2026-11-02, recorded on 026's scenario).

## Behavior

A project-scoped primitive takes rule files and returns, for each rule section `section_is_deprecated` marks deprecated, its file, line, rule ID, deprecating version and removal target, or the reason its label is malformed, by the rules Family 40's scenario states. `check-rule-ids` and the primitive share one label reader, so `check-rule-ids`' `deprecated` result and the primitive's list cannot disagree.

Family 40 becomes the primitive's entry point, the Family 30 shape: it keeps the comparison with `version` and the list of files citing a due rule, and drops its own parse. Its test passes unchanged against the entry point.

## Edge Cases

- **A malformed label is a result, not an error**, so the family still reports each one, and one malformed label never hides the others.
- **A rule file that cannot be read is reported and not counted as examined**, never skipped, as Family 40 reports it today.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
