---
section: "Loading tools"
---

# One-search-covers-referenced-sections

## Context

Each Claude Code copy carried one `ToolSearch` line, generated from the `ductus` tools its `## Instructions` steps name (AC29). A step can point at a reference section that calls a tool the steps do not name. In the cost round, plan searched a second time for `derive-boundary`, which its reference's §Validation gate calls: a model step, 1.68 s (`research.md`, R3). A static diff finds primitive names in 11 commands' references that their lines lack, most of them prose mentions. Which ones a run calls depends on its path, and the round ran one path per command.

## Behavior

- A command's one search loads every `ductus` tool its run calls, including one a reference section calls where a step points at that section, so no run searches a second time for a tool of its own command.
- The line stays derived from the command's own files, never hand-kept, so it follows their edits.

## Edge Cases

- A tool that loads with the session is not repeated in the line.
- A tool a reference names without calling it is the plan's to tell apart from one it calls. Loading one that is never called costs its schema in context.
- `ductus exec` dispatches no primitive because of the line, as AC29 requires.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
