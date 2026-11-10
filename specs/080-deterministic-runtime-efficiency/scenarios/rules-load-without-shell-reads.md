---
section: "Behavior"
---

# Rules-load-without-shell-reads

## Context

review's §Load rules loaded every selected rule file inline, and analyze asked about each loaded rule. In the cost round, review read the rule files in four shell calls: their sizes, heading counts, headings, and the Statements of five rules. analyze read them in three, and its `grep` of every Verification line came back as a 44,805-character saved output, read back whole (`research.md`, R5). review ran 27 turns over 1,502,864 input tokens, analyze 27 over 1,266,279.

## Behavior

- The rules a review or an analysis loads reach the model as tool results, with no shell read of a rule file.
- review gets each loaded rule's ID, tier and Statement. analyze also gets its Verification, and each heading that opens no rule with the lines under it, so it judges a malformed rule heading without reading the file.
- Whether a rule applies to the code or artifact in hand stays the model's judgment.

## Edge Cases

- A rule file outside the rule-file directory that review loads reaches the model the same way.
- A result over the read size moves to a file, as every bounded result does (AC7).
- A rule file that does not parse is named in the result, never dropped from it silently.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
