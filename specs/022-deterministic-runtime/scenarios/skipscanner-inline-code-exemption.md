---
section: "Follow-on scenarios"
---

# Skipscanner-inline-code-exemption

## Context

`SkipScanner` (`runtime/src/primitives/mod.rs`) is the shared line scanner every comment/fence-aware parser uses to skip content inside HTML comments and fenced code blocks — `read-tasks` / `mark-task` (task headings and checkboxes), `dashboard` (open-question counts), and the section walkers. It matches the HTML-comment delimiters (`<!--` and `-->`) textually, with no awareness of markdown inline-code (backtick) spans, and reads a code fence by the rule in Behavior.

So prose that merely *mentions* a comment-open delimiter inside a backtick span — with no matching close delimiter later on the same line — is read as a real comment opener: `SkipScanner` enters comment mode and skips every following line to the next close delimiter or EOF, and each comment-aware parser silently drops all structure after that line.

Hit concretely during 022 tasks 66–68: task 67's `done-when` in `tasks.md` embedded a backticked comment-open delimiter, opening a comment region that hid task 68 from `read-tasks` and `mark-task` (`task '68' not found`) until the prose was reworded. It was worked around at the prose level in two artifacts (`tasks.md` and the `append-inbox-comment-aware-write` scenario) but not fixed at the parser. Because ductus's own tasks, specs, and scenarios routinely write *about* comments and fences, the hazard recurs.

## Behavior

- `SkipScanner` does not treat an HTML-comment or code-fence delimiter as active when it falls inside a markdown inline-code span (a backtick-delimited run) on the line: such a delimiter neither opens nor closes a skip region. A region is entered only by a delimiter in ordinary, non-code-span text.
- Every parser built on `SkipScanner` — `read-tasks`, `mark-task`, `dashboard`'s open-question count, and the section / heading / bullet walkers — therefore sees all structure in a document whose prose mentions backticked delimiters. A task whose `done-when` carries a backticked comment-open delimiter no longer hides the following task.
- A genuine comment or fence — delimiters in ordinary text — is skipped exactly as before; the change narrows only the inline-code case.
- A code fence opens at a line whose first non-blank characters are a run of three or more backticks or tildes (a backtick run followed by no backtick), and closes at a line holding a run of the same character at least as long as the opener, with nothing after it. A shorter run, a run of the other character and a run followed by an info string are the fence's content. A fence indented to any depth still opens, because tasks and criteria nest fences in lists.
- `check-corpus-links` skips a fenced block by the same rule and still reads links inside HTML comments.

## Edge Cases

- A line that already carries both an open and a close delimiter (the inline form `SkipScanner` treats as inert today) stays inert; the fix does not change already-balanced lines.
- A lone backticked comment-open delimiter on an otherwise-ordinary prose line opens no region.
- A real fenced code block, whose fence lines begin with the fence delimiter in ordinary text, still skips its contents — a mid-prose backtick run is distinct from a fence line.
- A four-backtick block holding a triple-backtick example is one fenced block.
- A line that begins with an inline code span (a backtick run, text, another backtick run) opens no fence.
- A fence left open runs to the end of the file.
- Inline-code tracking is per line (markdown inline spans do not cross line breaks), so an unbalanced backtick run on one line never leaks a skip region into following lines.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
