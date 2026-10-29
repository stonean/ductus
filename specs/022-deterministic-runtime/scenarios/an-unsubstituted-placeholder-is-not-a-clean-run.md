---
section: "The primitive library"
---

# An-unsubstituted-placeholder-is-not-a-clean-run

## Context

AC20 makes `apply-manifest` reject a substitution key that is placeholder-shaped or empty before any write, so a malformed map cannot produce a tree in which every placeholder survived. Its complement is unguarded: a map that is well formed but missing a key the sources use. Read in HEAD f24c55c6's source on 2026-10-25, not run: nothing checks the written text for a placeholder left unsubstituted, so such a map writes `{key}` literally while `substitutions-applied` and `entries-substituted` read as a clean run. An adopter receives command files carrying a raw placeholder, and the run that wrote them says nothing.

## Behavior

- After substitution, an entry whose written text still carries one of the source's known placeholders is not reported as a clean substitution: the result names the entry and the placeholder.
- A placeholder the call asks to keep literal (keep-literals) is not reported.
- A run whose map covers every placeholder the sources use reports as today.

## Edge Cases

- A brace-delimited token in a source that is not a known placeholder (a JSON example, a format string in a code block): not reported.
- An entry that was copied without substitution: nothing to check, and its per-entry count stays absent rather than zero (AC21).
- The claim is read from source: implementation reproduces it with a fixture before changing behavior.

## Open Questions

- Does a surviving known placeholder make `apply-manifest` refuse before any write, as AC20 does for a malformed key, or write and report the entry and placeholder — and where does the set of known placeholders come from (the manifest's declared substitutions, or a scan of the source for the placeholder grammar)?

## Resolved Questions

*None yet.*
