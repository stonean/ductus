---
section: "Follow-on scenarios"
---

# The-cli-surface-nothing-pins

## Context

Registering a primitive touches several surfaces, and most of them are pinned by a test that fails loudly when one is missed. Verified on 2026-09-11 by deleting each wiring in turn and running the suite:

- `framework/runtime-tools.txt` is asserted **set-equal** to `TOOL_NAMES` in `runtime/tests/mcp.rs`, so both a missing entry and a phantom entry fail.
- The rmcp `#[tool]` methods are pinned by the same test, which lists served tools against `TOOL_NAMES`.
- `dispatch_primitive` is pinned by `interpreter::tests::dispatch_handles_every_registry_primitive`.
- `parser/mod.rs` cannot drift at all — `PRIMITIVE_NAMES` is a direct alias of `PRIMITIVE_REGISTRY` rather than a second list.

The clap subcommand enum in `runtime/src/main.rs` is the exception. With a `Command` variant and its dispatch arm deleted, the **entire suite still passes**. A primitive can therefore be absent from the `ductus <name>` CLI with nothing reporting it, and the markdown-only path — which the two-paths guarantee says must reach the same result as the MCP path — is exactly where that absence bites, since it is the path with no MCP server to fall back on.

This is [§design-principles](../../../framework/constitution.md#design-principles)' first rule turned on the runtime's own registration: the surfaces that *are* pinned prove themselves on every run, and the one that is not is indistinguishable from them until someone checks by hand. `schema/registry.rs` now records which is which, and `AGENTS.md` tells a contributor to run `cargo run -- <name> --help` — but a documented manual step is the diligence dependency the same section rejects.

Surfaced while closing [055](../../055-shared-constitution/spec.md), whose `resolve-constitutions` had to be confirmed reachable from the CLI by hand at the completion gate for this reason.

## Behavior

A test iterates `PRIMITIVE_REGISTRY` and asserts every name resolves to a clap subcommand, so the CLI surface is pinned the way the other four already are. A primitive present in the registry and absent from the enum fails the suite, naming the missing subcommand.

The assertion is **set-equality**, not containment, matching what `tests/mcp.rs` already does for the shipped manifest: a subcommand with no registry entry is as much a defect as a registry entry with no subcommand, because it means the CLI offers a verb the canonical set does not define. Containment in one direction would leave the phantom case unreported, which is the half that made the manifest assertion worth tightening.

Once the test exists, `AGENTS.md`'s entry naming `main.rs` as "the one unpinned surface" and prescribing a manual `cargo run -- <name> --help` is corrected in the same change — a pinned surface described as unpinned sends a contributor to do work a test already does, and the stale claim is the kind [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s prose-claim sweep exists to catch. `schema/registry.rs`'s module doc carries the same correction.

## Edge Cases

- **A subcommand that is deliberately not a primitive.** `exec` and `mcp` are runtime entry points rather than registry members, so the test's subject is the registry-backed subset and those are excluded by name, with the exclusion stated where a reader meets it rather than left as an unexplained filter.
- **Clap's own generated subcommands.** `help` is synthesised by clap and is not a registry primitive; it is excluded on the same basis.
- **Name mapping.** Registry names are kebab-case `<verb>-<noun>` and clap derives its subcommand names from the enum variants, so the test compares the names clap actually exposes rather than a hand-written transformation of the variant identifiers — a second transformation would be a second place to drift.
- **A primitive added with no CLI surface on purpose.** There is no such case today and the set-equality assertion forbids one silently appearing. If a future primitive genuinely should not be a subcommand, it is excluded explicitly with its reason, the same way `exec` and `mcp` are — never by loosening the assertion to containment.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
