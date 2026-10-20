---
section: "Follow-on scenarios"
---

# Required-config-values-from-any-source

## Context

On 2026-10-19 the owner decided that `quality-cross.md` `QUAL-GROUND-001` cedes configuration keys and values to the CFG rules, so one configuration defect is never reported under two IDs. An independent review then found the handover leaves a gap: `CFG-ENV-003` covers only a missing required *environment variable*, `CFG-ENV-008` covers present-but-malformed values from any source, and `CFG-ENV-012` (SHOULD) covers unrecognized keys in a configuration file. No rule now covers a required key absent from a configuration file, or a missing required command-line flag — the exact case of the Shore "Fail Fast" article `QUAL-GROUND-001` cites (a property missing from `config.properties`, silently replaced by a default).

## Behavior

- A required configuration value that is absent fails startup, naming the setting, whichever source it was expected from — environment variable, configuration file, or command-line flag — once the open question below is decided.

## Edge Cases

- A value with a declared default is optional and does not fail when absent. `CFG-ENV-001` declares a value required or optional for environment variables only, so no rule yet says which configuration-file keys or command-line flags are required.

## Open Questions

- Which way closes the gap: (a, recommended) widen CFG-ENV-003 to required values from any source — environment variable, configuration-file key, or command-line flag — together with CFG-ENV-001's required-or-optional declaration, which CFG-ENV-003 relies on to say what is required, matching the ENV category's name "Environment and configuration values" and CFG-ENV-008's scope; or (b) narrow the handover so QUAL-GROUND-001 keeps configuration-file keys and flags and cedes only environment variables and malformed values?

## Resolved Questions

*None yet.*
