---
section: "The primitive library"
---

# An-unsubstituted-placeholder-is-not-a-clean-run

## Context

AC20 makes `apply-manifest` reject a substitution key that is placeholder-shaped or empty before any write, so a malformed map cannot produce a tree in which every placeholder survived. Its complement is unguarded: a map that is well formed but missing a key the sources use. Read in HEAD f24c55c6's source on 2026-10-25, not run: nothing checks the written text for a placeholder left unsubstituted, so such a map writes `{key}` literally while `substitutions-applied` and `entries-substituted` read as a clean run. An adopter receives command files carrying a raw placeholder, and the run that wrote them says nothing.

## Behavior

- After substitution, an entry whose text would still carry one of the known placeholders makes `apply-manifest` refuse before any write, as AC20 does for a malformed key: the error names the entry and the placeholder, and no destination is written.
- The known placeholders are the four `framework/bootstrap/ductus.md` §Placeholder Substitution declares — `{project}`, `{cli-config-dir}`, `{project-name}` and `{One-line project description.}` — compiled into the runtime and pinned to that list by a test that fails when the two diverge.
- A placeholder the call asks to keep literal (keep-literals) is not refused.
- A run whose map covers every placeholder the sources use reports as today.

## Edge Cases

- A brace-delimited token in a source that is not a known placeholder (a JSON example, a format string in a code block, `{feature}`, `{NNN}`): not refused.
- An empty substitutions map, as the `ductus` self-install call passes on purpose (`framework/bootstrap/ductus.md` §Instructions step 9), substitutes nothing and checks nothing, so it is never refused.
- A known placeholder the map omits that no entry's text carries: not refused.
- An entry that was copied without substitution: nothing to check, and its per-entry count stays absent rather than zero (AC21).
- The claim is read from source: implementation reproduces it with a fixture before changing behavior.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does a surviving known placeholder make `apply-manifest` refuse before any write, or write and report the entry and placeholder, and where does the set of known placeholders come from?** It refuses before any write, against a fixed set: the four placeholders `framework/bootstrap/ductus.md` §Placeholder Substitution declares, pinned to that list by a test, with an empty map exempt (owner, 2026-11-12). `run` in `runtime/src/primitives/apply_manifest.rs` pre-read only the entries with a command or skill destination before writing, so the refusal adds a pre-write pass over every `update` and `create` entry, one extra read each when the map is non-empty (022 task 26). The change narrows what the primitive accepts, as AC20's did, so it is breaking under `BE-COMPAT-001`, and it is verified by a real adopter bootstrap. Rejected: writing and reporting the entry in a new result field, which relies on the host reading the field — the silent state AC20 closed; and scanning sources for the placeholder grammar, which at `ab2ad085` would have flagged about 100 of the 105 brace tokens under `framework/` that are not placeholders, while a set taken from the map's own keys is vacuous.
