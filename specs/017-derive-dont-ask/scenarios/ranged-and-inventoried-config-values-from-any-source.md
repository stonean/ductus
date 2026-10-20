---
section: "Follow-on scenarios"
---

# Ranged-and-inventoried-config-values-from-any-source

## Context

On 2026-10-20 the owner widened `CFG-ENV-001`'s required-or-optional declaration and `CFG-ENV-003`'s startup validation from environment variables to configuration values from any source — environment variable, configuration-file key, or command-line flag ([required-config-values-from-any-source](required-config-values-from-any-source.md)). 017's review of that change found two sibling rules still scoped to environment variables. `CFG-ENV-005` makes a value outside its documented range fail fast, naming the setting, the value and the violated bound, but only for an environment variable; since `quality-cross.md` `QUAL-GROUND-001` cedes all configuration to the CFG rules, an out-of-range configuration-file key or flag is covered by no rule. `CFG-ENV-002` keeps one canonical inventory of every environment variable the application reads, with its purpose, whether it is required, and a safe placeholder or default; `CFG-ENV-001` now asks configuration-file keys and flags to be declared required or optional too, and the inventory operators read to discover required configuration does not list them.

## Behavior

- Each open question below is answered by the owner, the rule changes (or stays) accordingly, and the answer moves to Resolved Questions with its date.

## Edge Cases

- 063 holds its file-based `[artifacts] read-size-bytes` to `CFG-ENV-005` by analogy, in four places: its plan's threshold section, its spec's invalid-threshold edge case, its data model's invalid-threshold error, and its `an-invalid-threshold-fails-fast` scenario. Widening `CFG-ENV-005` turns each into a direct citation; the data model and the scenario are durable contracts, so 063 is re-reviewed. 063 is in progress, so no done spec reopens.
- Widening `CFG-ENV-002` gives every plan that introduces a configuration-file key or a command-line flag an inventory obligation. In this repository the configuration keys already have one (the installer's §Project Configuration, held to `runtime/src/schema/config_keys.rs`), and the runtime's command-line flags are documented by their `--help`; whether that satisfies a widened rule depends on the forms it accepts.

## Open Questions

- `CFG-ENV-005`, out-of-range values: (a, recommended) widen it to a value with a documented valid range from any source — environment variable, configuration-file key, or command-line flag — matching `CFG-ENV-003` and `CFG-ENV-008`, so 063's by-analogy citations become direct ones (measured 2026-10-20: four 063 artifacts change, two of them durable contracts, and no done spec reopens); or (b) leave it covering environment variables only, so an out-of-range file key or flag stays covered by no rule?
- `CFG-ENV-002`, the inventory: widen it to every configuration value the application reads, each with its purpose, its `CFG-ENV-001` declaration and a safe placeholder or default — accepting as the inventory a commented example configuration file and a CLI's help output when they state those facts — or keep it covering environment variables only, leaving files and flags to their own schemas and help? No recommendation yet: before asking, measure which plans in this repository introduce a configuration-file key or a command-line flag, whether each names an inventory the widened rule would accept, and therefore which specs a widening would hold out of `done`.

## Resolved Questions

*None yet.*
