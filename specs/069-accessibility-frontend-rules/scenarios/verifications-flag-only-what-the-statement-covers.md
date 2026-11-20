---
section: "Enforcement"
---

# Verifications-flag-only-what-the-statement-covers

## Context

A pre-tag audit read five Verifications as flagging a case their Statement does not cover. Re-read whole against that reading, each holds: the Statement bars less than the `/{project}:analyze` flag catches. Each Verification gains one `**Not flagged:**` clause naming the harmless case; no Statement changes, so no rule requires more of a spec than before.

## Behavior

- `FE-KBD-004` does not flag a region the user opened (a modal dialog and its backdrop), which the Statement lets cover the focused element when the user can reveal it, nor a fixed or sticky element the spec states overlaps no focusable content.
- `FE-A11YFORM-003` does not flag a field collecting information about someone other than the user (a recipient's name or address): the Statement covers inputs that collect information about the user.
- `FE-A11YFORM-006` does not flag a `change` or `input` handler that only updates the current page's content or its URL query (a filter, sort or search refresh) and moves no focus, opens no window, submits nothing and loads no different page. WCAG 2.2's glossary entry "changes of context" says "A change of content is not always a change of context", and its examples are opening a new window, moving focus, going to a new page, or significantly re-arranging the content of a page (fetched from w3.org on 2026-11-19). A navigation to a different page stays flagged.
- `FE-LAYOUT-002` does not flag a fixed-height `overflow: hidden` container whose full text the spec or snippet makes revealable (an expand control, or a view of the full text): the Statement bars truncation "with no way to reveal it".
- `FE-MOTION-002` does not flag a sub-333 ms cycle the spec or snippet bounds below the general flash and red flash thresholds, the exception the Statement keeps.
- `FE-A11YTEST-001` is unchanged and stays MUST.

## Edge Cases

- A container truncated with no stated way to reveal its text stays flagged under `FE-LAYOUT-002`.
- A field for the user's own address stays flagged under `FE-A11YFORM-003` when it carries no token or `autocomplete="off"`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
