---
section: "LLM extension points"
---

# Exec-clarify-asks-each-open-question

## Context

`/{project}:clarify` step 6 promises one `askClarifyQuestion` round trip per entry under the spec's `## Open Questions`, in sequence (`framework/commands/clarify.md`), and the [clarify-command-acceleration](clarify-command-acceleration.md) scenario states the same contract: one `llm-request` / `llm-response` round trip per question. The request builder was written for that loop — `resolve_clarify_question` takes an explicit `question` the walker seeds "one per round trip" (`runtime/src/interpreter/payload.rs`) — but the walker never seeded one. It sent one request per extension step, so the builder fell back to the first entry of `read-spec`'s `open-questions`, and a `ductus exec clarify` run over a spec with three entries under `## Open Questions` asked the host about the first and never the other two. No fixture or golden exercises exec clarify, so nothing noticed.

Found while planning `060-exec-analyze-assesses-each-loaded-rule`, whose exec analyze walk had the same one-request-per-step shape for `assessSpecQuality` and now fans out one request per loaded rule.

## Behavior

An `askClarifyQuestion` step sends one request per entry of the walker context's `open-questions` list — the result of `read-spec` at clarify step 2 — in the order `read-spec` returns them. The walker seeds each entry as the request's `question`, sends it, awaits and validates the response, and removes the seed before the next, exactly as the `assessSpecQuality` fan-out seeds each rule. Every response is received and reported in the stream; the last is kept under `llm:askClarifyQuestion`, as before.

## Edge Cases

- **No open question.** When `open-questions` is present and empty, the step sends no request and says so in the stream. This is clarify step 2's zero-questions short-circuit: the question loop has nothing to walk, and steps 7–12 still run.
- **No `open-questions` in the context.** When no step has produced the list, the walker cannot know the questions, so it sends the single request the builder has always built — an explicitly seeded `question`, else an empty one — rather than inventing an empty list and asking nothing.
- **A malformed response partway through.** It halts the walk with `error: schema-mismatch`, as any extension response does, before the status-advance gate; no question after it is asked, and no status is written.
- **Order and ids.** Requests go out in `read-spec`'s order, numbered by the walker's request counter, so a run's stream is reproducible.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
