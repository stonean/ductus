---
section: "Follow-on scenarios"
---

# Required-config-values-from-any-source

## Context

On 2026-10-19 the owner decided that `quality-cross.md` `QUAL-GROUND-001` cedes configuration keys and values to the CFG rules, so one configuration defect is never reported under two IDs. An independent review then found the handover leaves a gap: `CFG-ENV-003` covers only a missing required *environment variable*, `CFG-ENV-008` covers present-but-malformed values from any source, and `CFG-ENV-012` (SHOULD) covers unrecognized keys in a configuration file. No rule now covers a required key absent from a configuration file, or a missing required command-line flag — the exact case of the Shore "Fail Fast" article `QUAL-GROUND-001` cites (a property missing from `config.properties`, silently replaced by a default).

## Behavior

- A required configuration value that is absent fails startup, naming the setting — and the file, for a file key — whichever source it was expected from: environment variable, configuration file, or command-line flag. `configuration-cross.md` `CFG-ENV-003` says so, and `CFG-ENV-001`'s required-or-optional declaration, which it relies on, covers every source.

## Edge Cases

- A value with a declared default is optional and does not fail when absent; `CFG-ENV-001` asks every value, from any source, to be declared required or optional.
- The read-once requirement in `CFG-ENV-001` stays about the process environment; widening the declaration does not extend it to files or flags.

## Open Questions

*None — all resolved.*

## Resolved Questions

- Which way closes the gap: (a, recommended) widen CFG-ENV-003 to required values from any source — environment variable, configuration-file key, or command-line flag — together with CFG-ENV-001's required-or-optional declaration, which CFG-ENV-003 relies on to say what is required, matching the ENV category's name "Environment and configuration values" and CFG-ENV-008's scope; or (b) narrow the handover so QUAL-GROUND-001 keeps configuration-file keys and flags and cedes only environment variables and malformed values? Resolved 2026-10-20 by the owner, option (a): `CFG-ENV-003` and `CFG-ENV-001`'s declaration now cover environment variables, configuration-file keys and command-line flags, so the 2026-10-19 handover stands and configuration stays in one rule file. Measured beforehand: either option reopened 036 for an analyze, since its `spec.md` described `CFG-ENV-003` as covering environment variables; option (b) also reversed part of the 2026-10-19 decision and split configuration across two rule files. The Fail Fast article's worked example, checked at its source the same day, is a `maxConnections` property missing from a configuration file through a typo: the non-failing reader returns a default of 10, and the failing one throws an error naming the property and the file's path.
