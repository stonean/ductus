---
section: "Follow-on scenarios"
---

# Scenario-question-parser-fix

## Context

`parse_open_questions` walked the plain section helper while the sibling acceptance-criteria parser in the same file already walked the comment- and fence-aware one. The asymmetry was invisible in normal use because `/{project}:specify` overwrites the template's guidance comment moments after scaffolding, but it was real: reading a spec straight from the shipped template returned the three commented-out example questions as genuine entries while acceptance criteria correctly returned empty.

Two consequences followed. A question shown inside a fenced block as an example counted as asked, and — because the walker folded any non-blank line into the preceding question — a comment following a question silently became part of its text.

Surfaced by [046 — Scenario open-question visibility](../../046-scenario-open-question-visibility/spec.md), which gates a spec's `done` on its scenarios' question count. Under that gate a phantom question stops being a cosmetic miscount and becomes a hard block on completion, so the fix is a prerequisite for the feature rather than a cleanup alongside it.

## Behavior

`parse_open_questions` walks the comment- and fence-aware section helper, the same one the acceptance-criteria parser uses. Questions inside an HTML comment block or a fenced code block are not entries, so a spec scaffolded from the shipped template reports zero open questions. Blank lines are still yielded by that helper, so the blank-line terminator and continuation folding are unchanged.

The placeholder guard widens from an exact match on the spec template's placeholder to a set that also covers the two `create-scenario` compiles into every new scenario. Neither is authored as a list bullet today, so the guard is belt-and-braces — the set means the behavior no longer depends on that remaining true.

`append-question`'s dedup calls this same parser, so reader and dedup agree by construction rather than by coincidence.

## Edge Cases

- A comment that opens and closes on one line stays **inline** per the skip scanner's documented exemption — that exemption is what keeps `- [ ] criterion <!-- note -->` intact. A standalone one-line comment after a question therefore still folds into that question's text as a lazy list continuation. It adds no entry, so the count every consumer reads is correct either way.
- A spec whose Open Questions section contains no commented-out bullets reports exactly what it did before; the fix changes no existing count.
- A placeholder authored as a list bullet is skipped, which the previous exact-match guard would have missed for the scenario placeholders.

## Open Questions

*None — resolved in 046's clarify before this scenario was written.*

## Resolved Questions

*None yet.*
