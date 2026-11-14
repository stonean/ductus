---
next-task: 23
---

# 059 — Project in a repository subdirectory Tasks

Tasks derived from the [plan](plan.md). Complete in order. Tasks 1–9 and 16 are runtime work, verified through the built binary rather than the MCP tools, which answer with the binary the session started on. Every new test is shown to fail with its conversion removed before it is trusted.

## 22. Analyze the corrections spec 079 made

Spec 079 task 10 corrected AC3 and AC11, superseded when `diff-cross-spec` moved its start and the markdown-only check went; no code, scenario or data model of this spec changed in it, but `compute-review-scope` reports `review-owed: true`, with earlier commits naming this spec and an unclaimed commit in its window, so the gate holds a review for it. It owes `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079).

- **Done when**: an analysis is recorded on this spec, and `/ductus:implement` has returned it to `done`.
