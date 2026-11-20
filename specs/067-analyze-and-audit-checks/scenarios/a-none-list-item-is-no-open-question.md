---
section: "Artifact checks"
---

# A-none-list-item-is-no-open-question

## Context

`read-spec`'s parser counts each list item under `## Open Questions` as a question and skips only the scaffold placeholders, so an author who writes the empty state as a bullet (`- None`) is read as holding one open question. `spec-integrity` then blocks the spec at `clarified` or later, and a scenario written the same way holds `done`.

## Behavior

- A list item under a spec's or a scenario's `## Open Questions` whose whole text is `none`, `n/a` or `none yet`, or begins with `none —` and a space, and does not end in a question mark, is a placeholder, not a question, with emphasis marks (wherever they stand), a trailing period or exclamation mark, and case ignored. Every reader of the parser agrees: `read-spec`, `dashboard`, `check-review-gate`, `append-question`'s dedup, `classify-spec-edit` and `check-artifacts`. The three scaffold placeholders stay skipped (spec 046).
- A list item that merely begins with the word, such as "None of the queues cover retries?", is a question.

## Edge Cases

- A paragraph under the section was never a question; its reading is unchanged.
- `append-question` still removes only a `*None …*` line when it appends; a `- None` bullet beside the new question stays and counts for nothing.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
