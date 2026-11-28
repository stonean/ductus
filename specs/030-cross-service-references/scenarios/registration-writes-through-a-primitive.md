---
section: "Resolving the linked status"
---

# Registration-writes-through-a-primitive

## Context

`/ductus:link` registers a service by adding a `[services.<alias>]` block to the project config, as `framework/commands/link.md`'s **Additive write** states, and a host performs that write with its file tools: this spec's plan (D6) and `link.md` both say the command has no primitive. Under `ductus exec link` nothing would write the registry, and [022's scenario `link-registration-is-an-extension-point`](../../022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md) resolved that a primitive does (the agent's recommendation, adopted by the operator, 2026-11-28): the write is deterministic, performed mechanically by a host following a procedure, and specifiable with no policy of its own, which §runtime-boundary's eligibility criteria make a primitive by default. A primitive specific to one feature lives with that feature.

## Behavior

- A runtime primitive performs `link.md`'s **Additive write**: given an alias, a repo URL, a local path and an optional description, it adds a `[services.<alias>]` table holding `repo`, `path` and, when one is given, `description` to the active config file (the newest of `.ductus/config.toml`, `.govern/config.toml` and `.govern.toml` that exists, as `link.md`'s Scope Boundaries state), leaves every other byte as it was, and writes the file through tempfile and rename. It is an MCP tool and a CLI subcommand, since the file it writes is inside the project and is no host's settings (§runtime-boundary principle 5).
- Its result names the block it wrote and each warning the write carries: a repo another alias already registers, and a path that resolves to no directory. `link.md` step 3 shows both.
- `link.md` step 2 calls the primitive once the fields are validated, so a host walking the command writes through it, and `link.md`'s statements that the command is host-driven with no runtime primitive and no runtime shortcut, and this spec's plan D6, say so instead. The step names the primitive unbackticked, as `.ductus/docs/agents/developing-ductus.md` §Editing a command's steps says of a primitive whose arguments the exec walker cannot supply: passing the `registerService` response's fields to it under `ductus exec` is 022's task 65, which waits on this scenario's task.
- One registration costs one call, which reads the active config file once and writes it once. It retires no primitive: it replaces the host's file-tool write.

## Edge Cases

- An alias `[services]` already registers is refused before any write, as `link.md`'s **Validation** refuses a duplicate; the primitive reads `[services]` when it writes, so an alias registered after the operator was prompted is refused too.
- A field `link.md`'s **Validation** rejects (an alias that is empty or not a bare TOML key, a repo that is not URL-shaped, an empty path) is refused before any write, naming the field.
- The path is recorded verbatim, a transcription of the operator's answer, which keeps `[services]` on the config-sourced side of the boundary `validate_no_traversal` states (022's AC29).
- With no config file, the primitive creates `.ductus/config.toml` holding only the new block; with only a legacy file present, the block lands there and no `.ductus/config.toml` is created.
- A value holding a quote, a backslash or a newline is written as an escaped TOML string, so the block reads back as the value given.
- A config that does not parse, or one an appended `[services.<alias>]` table would leave unparseable (a `services` key written as an inline table or as dotted keys), is refused with nothing written, naming the file.
- A config another writer changed between the primitive's read and its rename is refused with nothing written, as the guarded read-modify-writes in `specs/022-deterministic-runtime/data-model.md` are.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
