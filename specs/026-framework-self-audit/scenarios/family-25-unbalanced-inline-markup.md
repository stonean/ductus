---
section: "Follow-on scenarios"
---

# Family-25-unbalanced-inline-markup

## Context

Spec 050 rewrote 21 `AGENTS.md` entries to point at the constitution rather than restate it. Two of the 21 did not survive the rewrite: one had its bold title deleted rather than converted, leaving an orphan backtick and no title at all (`` - ` The rule is … ``); the other kept its old body and had `** The rule is …` appended, where the `**` is followed by a space and so never opens emphasis — it renders literally.

Both are invisible to `markdownlint`. An unclosed backtick never becomes a code span, so `MD038` does not apply, and a literal `**` is not a heading, list, or link. 050's review recorded clean and the defects stood until a reader noticed the rendered text.

## Behavior

The family reports a line in `AGENTS.md` — and in the `AGENTS.md` template that ships to adopters — carrying an odd number of backticks or an odd number of `**` markers, outside fenced code blocks.

The check is per-line, which is exact for these two files and only these two: every rule entry is a single unwrapped bullet (69 bullets, 0 continuation lines), so a span that opens on a line must close on that line. It is deliberately **not** extended to the rest of the corpus, where prose wraps freely and a bold span legitimately crosses a line break — measured at 283 such lines, every one of them correct.

Measured: 2 findings at the commit before the repair — exactly the two malformed entries — and 0 after.

## Edge Cases

- **Fenced code blocks** are skipped, and a fence marker is itself three backticks.
- **`***bold italic***`** yields an even `**` count and is not reported.
- **A wrapped bullet introduced later.** The single-line convention is what makes a per-line check exact. If a continuation line appears the family reports it rather than silently narrowing its scope — a check whose subject has drifted out from under it is the failure mode §design-principles names.
- **Scope reporting.** The family names the files it examined on stderr, so a clean exit is never read as *"all markdown is balanced"* — it covers two files by design, not the corpus.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
