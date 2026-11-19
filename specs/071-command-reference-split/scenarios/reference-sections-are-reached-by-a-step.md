---
section: "How a step points"
---

# Reference-sections-are-reached-by-a-step

## Context

An agent reads a reference section only when a step names it, so a section no step names is never read, whatever it holds. Before the split the same text sat in the command file the agent had already loaded. Measured at `b7b07500` over the 12 shipped references that have a command (every `framework/reference/*.md` but `audit.md`), counting each `##` section outside a fenced block: 23 of 93 were named by no command text, no section already reached and no anchor link. Among them were the readiness gate of `/plan`, the task walk of `/implement` and the next-step line of `/specify`.

## Behavior

- A step whose work a reference section governs names that section where the work happens, not only in a later or sibling step.
- A reference section that no step needs and that only restates its command or a primitive's contract is deleted. A rule it held that nothing else states moves into the step or section that needs it, and the heading goes.
- A reference whose sections are all deleted stays as its opening paragraph, since the criteria name every command that has a reference.
- A section is reached when the command text, or a section already reached, names it by `§` and its heading, by a bold span equal to the heading, or by a markdown anchor link to it.
- A check fails when a `##` section of a shipped reference is reached by no step, and states the section; a section kept for a reader outside the command carries a stated reason beneath its heading and is counted as exempt, never silent. Spec 026 implements the check.
- Two sections are kept for such a reader: `Severity tiers` and `Record fields` in `framework/reference/analyze.md`. A done scenario of spec 067 names `Severity tiers` as the list of the four tiers, and the constitution names each command's reference as the home of its record's field semantics, which is `Record fields` for the analyze record.

## Edge Cases

- A section reached only by a markdown anchor link inside a reached section is reached.
- A heading inside a fenced block is not a section.
- A reference with no `##` section (`log`, `prune`) has nothing to reach: the check reports it as having no sections, a distinct state, not as a pass.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
