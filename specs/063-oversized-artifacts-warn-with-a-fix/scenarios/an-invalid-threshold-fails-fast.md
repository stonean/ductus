---
section: "Edge Cases"
---

# An-invalid-threshold-fails-fast

## Context

`configuration-cross.md` `CFG-ENV-008` (MUST, added 2026-10-19) requires a configuration value that is present but does not parse into its declared type to fail fast, naming the setting and the value, and never to fall back to the default. This spec's edge case for an invalid configured threshold does the opposite: an `[artifacts] read-size-bytes` that is not a positive whole number is reported, and the check runs at the 50,000-byte default. The fallback is announced, not silent, but it does not fail fast. The owner decided on 2026-10-19 that the rule stands and `ductus` changes to comply.

## Behavior

- A `read-size-bytes` value that is not a positive whole number of bytes halts the command that reads it, naming the setting, the offending value, and the accepted form. No default is substituted for a value the project set.
- An unset `read-size-bytes` still means the 50,000-byte default; only a present, malformed value halts.
- The spec's edge case, `data-model.md`, and the `read-size-bytes` comment in `framework/bootstrap/ductus.md` (and its `govern.md` mirror) state the fail-fast behavior instead of the reported fallback.

## Edge Cases

- A fraction or a non-numeric string does not parse as a whole number, so it is malformed under `CFG-ENV-008`. Zero or a negative number parses but falls outside the documented range (at least 1 byte). `CFG-ENV-005` requires an out-of-range configuration value from any source, this file-based key included, to fail fast, naming the setting, the file, the value, and the violated bound. Both halt alike.
- An `artifacts` that is not a table — `artifacts = 50000` — cannot hold the key, so it halts the same way, naming `artifacts` and its value.
- The markdown-only path halts the same way, naming the same setting and value.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
