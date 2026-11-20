---
section: "Artifact checks"
---

# A-plan-heading-is-matched-without-regard-to-case

## Context

`plan-consistency` finds a plan's Technical Decisions and Affected Files sections by exact, case-sensitive heading, and reports a heading it did not match as an empty section. A plan written with `## Technical decisions`, or by hand from an older template, holds its decisions and files and still takes two blocking findings that say they are empty.

## Behavior

- The family matches each of the two headings without regard to case.
- A plan with no such heading is reported as lacking the section, naming the heading, and is told apart from a section that exists and holds nothing, which keeps its present message.
- Every other reader of a section by heading keeps its exact match, `mark-criterion`, `label-criteria`, `read-spec`, `check-command-flags` and `check-step-references` among them.

## Edge Cases

- A differently worded heading (`## Technical Decisions and Rationale`) is not matched: it is reported as absent, and the analyze reference's Plan record section reports it as outside the design record.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
