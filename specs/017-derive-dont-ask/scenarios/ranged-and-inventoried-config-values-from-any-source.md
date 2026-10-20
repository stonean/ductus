---
section: "Follow-on scenarios"
---

# Ranged-and-inventoried-config-values-from-any-source

## Context

On 2026-10-20 the owner widened `CFG-ENV-001`'s required-or-optional declaration and `CFG-ENV-003`'s startup validation from environment variables to configuration values from any source — environment variable, configuration-file key, or command-line flag ([required-config-values-from-any-source](required-config-values-from-any-source.md)). 017's review of that change found two sibling rules still scoped to environment variables. `CFG-ENV-005` made a value outside its documented range fail fast, naming the setting, the value and the violated bound, but only for an environment variable; since `quality-cross.md` `QUAL-GROUND-001` cedes all configuration to the CFG rules, an out-of-range configuration-file key or flag was covered by no rule. `CFG-ENV-002` kept one canonical inventory of every environment variable the application reads, with its purpose, whether it is required, and a safe placeholder or default; `CFG-ENV-001` now asked configuration-file keys and flags to be declared required or optional too, and the inventory operators read to discover required configuration did not list them.

## Behavior

- Each open question below is answered by the owner, the rule changes (or stays) accordingly, and the answer moves to Resolved Questions with its date.

## Edge Cases

- 063 held its file-based `[artifacts] read-size-bytes` to `CFG-ENV-005` by analogy, in four places: its plan's threshold section, its spec's invalid-threshold edge case, its data model's invalid-threshold error, and its `an-invalid-threshold-fails-fast` scenario. Widening `CFG-ENV-005` turned each into a direct citation, and 063's Applicable Rules cites the rule again; the data model and the scenario are durable contracts, so 063 is re-reviewed. 063 is in progress, so no done spec reopens.
- Widening `CFG-ENV-002` gave every plan that introduces a configuration-file key an inventory obligation. In this repository the configuration keys already had one (the installer's §Project Configuration, held to `runtime/src/schema/config_keys.rs`), and the runtime's command-line flags are documented by their `--help`, which the widened rule leaves them to.

## Open Questions

*None — all resolved.*

## Resolved Questions

- `CFG-ENV-005`, out-of-range values: (a, recommended) widen it to a value with a documented valid range from any source — environment variable, configuration-file key, or command-line flag — matching `CFG-ENV-003` and `CFG-ENV-008`, so 063's by-analogy citations become direct ones (measured 2026-10-20: four 063 artifacts change, two of them durable contracts, and no done spec reopens); or (b) leave it covering environment variables only, so an out-of-range file key or flag stays covered by no rule? Resolved 2026-10-20 by the owner, option (a): `CFG-ENV-005` now covers a configuration value with a documented valid range from any source — environment variable, configuration-file key, or command-line flag — and its error names the setting, and the file for a file key, the value, and the violated bound, as `CFG-ENV-003`'s does. `CFG-ENV-003`'s hand-off and `data-model.md` §Rule set say so, and 063's four citations of it are direct. Measured beforehand: of the configuration keys any spec in this repository introduces, only 063's `[artifacts] read-size-bytes` has a documented range, and 063 already fails fast on it, so no spec fails its next analyze on the widening.
- `CFG-ENV-002`, the inventory: widen it to every configuration value the application reads, each with its purpose, its `CFG-ENV-001` declaration and a safe placeholder or default — accepting as the inventory a commented example configuration file and a CLI's help output when they state those facts — or keep it covering environment variables only, leaving files and flags to their own schemas and help? No recommendation yet: before asking, measure which plans in this repository introduce a configuration-file key or a command-line flag, whether each names an inventory the widened rule would accept, and therefore which specs a widening would hold out of `done`. Resolved 2026-10-20 by the owner, option (a): `CFG-ENV-002` now requires every environment variable and every configuration-file key in a canonical inventory — one per kind of source or one for both, never two for one source — accepts a commented example of the configuration file as that inventory, and leaves command-line flags to the interface's help output. Measured beforehand over the 65 specs with a plan: of the in-progress specs introducing a configuration-file key, 033 and 063 name an inventory and 020 did not, so 020's plan now names the installer's §Project Configuration; that inventory gained the defaults it left implicit, for `[project]`, `[pinned] files` and `[review] tech-stack-verified`; and four done specs that introduced a key without naming an inventory (029, 030, 040, 048) are flagged when next reopened, each needing one Affected Files row. Counting flags would have added defaults for about 70 runtime arguments that `--help` does not state and up to 14 more done specs, mostly over per-invocation arguments.
