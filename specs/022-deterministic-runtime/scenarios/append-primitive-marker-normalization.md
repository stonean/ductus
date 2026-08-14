---
section: "Follow-on scenarios"
---

# Append-primitive-marker-normalization

## Context

Every `append-*` primitive renders its own list marker onto caller-supplied text: `append-inbox` and `append-task` render a `- [ ]` checkbox, `append-question` a plain `-` bullet. When the caller includes a marker in the text it passes, the marker is doubled — `- [ ] - [ ] text`, or `- - text`.

The failure is silent at every stage that could catch it. The caller cannot see the rendering, so nothing at the call site looks wrong; a doubled marker is valid markdown, so `lint-markdown` passes; and the result is an atomic write to a committed artifact. It surfaces only when a human reads the file.

Observed 2026-08-14: `append-task` invoked with checkbox-prefixed body items wrote six doubled checkboxes into `specs/022-deterministic-runtime/tasks.md`, caught by eye after the write.

The same shape has a second form in `append-inbox`'s `dedup-prefix`. `has_bullet_with_prefix` compares the prefix against `bullet_text`-extracted content, which has the marker already stripped — so a marker-bearing prefix matches nothing, the dedup guard silently no-ops, and re-running an audit appends duplicates. That is the guard `/{project}:analyze`'s finding capture depends on for idempotence.

## Behavior

Each append primitive strips **one** leading list marker from caller-supplied bullet content before rendering its own.

**One shared grammar.** Stripping reuses `bullet_text`, the helper `append-inbox`'s dedup and `remove-inbox-item`'s removal already share, which resolves both the plain `- text` and checkbox `- [ ] text` / `- [x] text` forms through `checkbox::parse_checkbox_line`. Reuse rather than a second matcher is the point: the write side and the dedup read side must agree on bullet identity, and two grammars would eventually disagree about which inputs carry a marker.

**One marker, never more.** The observed failure is a single doubling. Looping would eat legitimate content from text that genuinely begins with a dash, trading a visible defect for a silent one.

**Applied before dedup, not after.** In `append-question` and `append-inbox` the normalized form is what gets compared *and* what gets written, so an entry can never be stored in one form and matched in another. `append-inbox`'s `dedup-prefix` is normalized the same way, which is what makes a marker-bearing prefix match the bullets it names.

**The contract is documented at the argument.** Each affected parameter's schema description states that the primitive renders the marker and that a caller-supplied one is stripped — the argument description is where a caller looks, and an undocumented normalization is its own surprise.

## Edge Cases

- Text with no leading marker is unchanged apart from trimming — the overwhelmingly common case pays nothing.
- The checked forms `- [x]` and `- [X]` are stripped like `- [ ]`. A caller passing a checked box to `append-task` gets an unchecked one, because a newly appended task is unchecked by definition.
- Text whose content legitimately begins with a dash but is not a list marker — `--fix reverts a drifted done spec` — is untouched: the grammar requires a dash followed by a space, and `--fix` does not match.
- Text that is only a marker reduces to empty and is rejected by each primitive's existing empty-argument guard rather than writing a bare bullet.
- A doubled marker supplied deliberately (`- [ ] - [ ] text`) loses one level, not both. There is no case for supporting it, and stripping to exhaustion is the looping behavior rejected above.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
