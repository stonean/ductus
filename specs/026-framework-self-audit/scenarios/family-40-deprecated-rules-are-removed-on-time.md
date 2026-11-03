---
section: "Check families"
---

# Family-40-deprecated-rules-are-removed-on-time

## Context

A rule that `ductus` ships is retired in two steps, per constitution §rules: a `**DEPRECATED in {version}:**` label names the release that deprecates it and its removal target, and the rule is removed in that target release. 077 wrote the first such labels in shipped rule files: `security-backend.md`'s `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` read "**DEPRECATED in 0.60.0:** … removed in 0.62.0", and the changelog, `docs/rules.md` and 008's `data-model.md` repeat the promise.

Nothing held a release to it. No task, script or workflow scheduled the removal, and nothing compared a label's removal version with `version`, so the removal depended on whoever cut that release remembering it, which §design-principles rules out. A release that ships past the target leaves every adopter an Advisory naming a removal that did not happen; a removal without a sweep turns each live citation of the old ID into a Blocking "unknown rule" finding at `/{project}:analyze`. 077's re-read found the gap (077 task 8, owner, 2026-11-02).

## Behavior

Family 40, `scripts/audit/deprecation-removal.sh`, reads every tracked `framework/rules/*.md` and finds each deprecation label as the runtime's `check-rule-ids` finds it (`section_is_deprecated`): a rule's section runs from its `##`–`####` ID heading to the next heading at its level or above, and a line in it containing `**DEPRECATED` is its label. The label must open its line as `**DEPRECATED in X.Y.Z:**`, and its paragraph must name the removal target once, as `removed in X.Y.Z` in any case, the form 008's `data-model.md` states (Deprecated field). The family compares that target with the first line of `version`.

- **A target at or below `version` is a finding** at `file:line`: the rule is still in the file in or after the release that was to remove it. The suggested fix names the rule's file and every tracked file citing its ID, since removing the rule turns each live citation into an unknown-rule finding; a decision record, a changelog entry or an audit record keeps the old ID (§drift-prevention), so the remover classifies each.
- **A malformed label is a finding**, since it can never be held to a release: one outside any rule section, a second in one section, one that does not open its line (a blockquote, an indent, a mid-line mention), a deprecating version or target that is not `MAJOR.MINOR.PATCH`, and no target or more than one.
- It runs in the release audit: AGENTS.md runs `scripts/audit/run-all.sh` after the version-bump commit and before tagging, `runtime-release.yml`'s `audit` job runs it before publishing, and `framework-checks.yml` runs it on every push. So the release that reaches a target fails its audit while the rule is present.
- **It re-reads what the runtime already parses**, so it mirrors that parse rather than the schema's wording, fenced blocks included. A primitive returning each label will replace the parse, and this family will become its entry point (067's scenario `deprecation-labels-are-read-by-the-runtime`).

## Edge Cases

- **No deprecation label is not a finding.** Nothing is deprecated, so there is nothing to compare, and the coverage line on stderr says so, distinct from a clean comparison.
- **An unreadable or malformed `version`, no tracked rule file, a tracked rule file that cannot be read, and a parse that cannot run** are findings: each leaves a comparison unmade, and none may exit like a clean run. An unreadable file is not counted as examined.
- **Every run ends with a coverage line on stderr**: the rule files examined of those tracked, the labels found and compared, and the `version` they were compared with, or what left nothing compared.
- **A target above `version` is silent.** The rule binds until its release, so 0.60.0 and 0.61.0 report nothing for the four 077 labels, and 0.62.0's bump commit fails the audit until they are removed.
- **`scripts/tests/test-deprecation-removal.sh` proves it both ways** on fixture trees, each case with the exit and finding it must produce: targets above version at the minor and the patch number, at it, and below it at the major, minor and patch numbers, compared numerically; a wrapped label; the target in another case; no target, two targets and a malformed target; a malformed deprecating version; a label before any rule heading and one under a `##` heading after a rule; a blockquoted label; two labels in two files, both counted; no label at all; a missing and a malformed `version`; no tracked rule file; and a tracked file that cannot be read. Its control case does not require the tree to carry a label, so the commit that removes the last deprecated rule does not fail it. `framework-checks.yml` step (p) runs it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Which spec owns this family?** → 026, as Family 40, since 026 owns the shell audit families and 067 the runtime checks some of them call (owner, 2026-11-02, option a; recorded on 077's task 8).
- **Where is a label parsed?** → In this family for now, mirroring the runtime's `section_is_deprecated`, with a primitive returning each label to follow as 067's scenario `deprecation-labels-are-read-by-the-runtime`, once the 076 work in the runtime's registration files has landed. Waiting for the primitive was rejected because no check would run until then, and a permanent shell parse because two parses can drift (owner, 2026-11-02, option a).
- **What form does a label's removal target take?** → `removed in X.Y.Z`, named once and matched in any case, stated in 008's `data-model.md` (Deprecated field). A list of phrasings accepted by the family alone was rejected because the grammar would live only in the script, and a target written inside the label itself because it rewrites 077's four labels (owner, 2026-11-02, option a).
