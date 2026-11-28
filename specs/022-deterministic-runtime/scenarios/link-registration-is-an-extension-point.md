---
section: "LLM extension points"
---

# Link-registration-is-an-extension-point

## Context

`/ductus:link`'s registration step (`framework/commands/link.md` step 2, spec 030) carries the marker `<!-- llm:registerService -->`. `specs/022-deterministic-runtime/spec.md` §Parseable slash command procedures requires a marker's identifier to match an entry in its §LLM extension points, and that section says the data model carries every extension point's schema. No `registerService` entry exists, so `ductus exec link` halts at the step with `unknown-extension` (022's review of 2026-11-27, deferred to this spec's own task by the operator).

## Behavior

- `registerService` is an entry in `specs/022-deterministic-runtime/spec.md` §LLM extension points, added by this scenario as each follow-on extension point was. Its request and response schemas are in the data model and typed in `runtime/src/schema/extensions.rs`; `build_extension_request` in `runtime/src/interpreter/payload.rs` builds its request, since an identifier it has no builder for halts there with `unknown-extension` before any response is read; and `validate_response` checks a response against the schemas, so a response that fails halts with `schema-mismatch`, as any extension point's does.
- The request carries what the registration flow prompts for and checks against: the fields `link.md` step 2 names (alias, repo URL, local path, optional description) that the walk's context left unset, and the aliases and repos `[services]` already registers. Under `ductus exec` a pre-supplied field is a `key=value` argument (`alias=`, `repo=`, `path=`, `description=`), since the walker refuses a positional argument with `invalid-argument`; `--list` has no exec form, and a host runs it as `link.md` step 1 says.
- The response carries the alias, repo URL, local path and optional description the operator gave, and the `[services.<alias>]` block is written additively, every other table preserved, as `link.md`'s **Additive write** states. Who performs that write under `ductus exec` is this scenario's open question.
- `ductus exec link` walks `/ductus:link` to its end on a fixture, its registration step answered by a canned response, and a test holds the walk.

## Edge Cases

- A response naming an alias `[services]` already registers is refused before any write, as `link.md` step 2 refuses a duplicate; a duplicate repo is warned and allowed.
- The local path is recorded as the operator gave it, which keeps `[services]` on the config-sourced side of the boundary `validate_no_traversal` states (AC29): a path the runtime or the model originates moves the table into the validated tier in the same change.

## Open Questions

- Under `ductus exec`, does the walk write the `[services.<alias>]` block itself, through a primitive that performs `link.md`'s **Additive write**, or does it end `complete` with the write left to the host, an exec-path scope reduction documented in the command and the data model as `coverage-residue-cleanup` records for `clarify`'s steps 7–8? No primitive writes `[services]` today, so a walk that reaches its end with nothing said writes no registry.

## Resolved Questions

*None yet.*
