---
section: "LLM extension points"
---

# Link-registration-is-an-extension-point

## Context

`/ductus:link`'s registration step (`framework/commands/link.md` step 2, spec 030) carries the marker `<!-- llm:registerService -->`. `specs/022-deterministic-runtime/spec.md` §Parseable slash command procedures requires a marker's identifier to match an entry in its §LLM extension points, and that section says the data model carries every extension point's schema. No `registerService` entry exists, so `ductus exec link` halts at the step with `unknown-extension` (022's review of 2026-11-27, deferred to this spec's own task by the operator).

## Behavior

- `registerService` is an entry in `specs/022-deterministic-runtime/spec.md` §LLM extension points, added by this scenario as each follow-on extension point was. Its request and response schemas are in the data model and typed in `runtime/src/schema/extensions.rs`; `build_extension_request` in `runtime/src/interpreter/payload.rs` builds its request, since an identifier it has no builder for halts there with `unknown-extension` before any response is read; and `validate_response` checks a response against the schemas, so a response that fails halts with `schema-mismatch`, as any extension point's does.
- The request carries what the registration flow prompts for and checks against: the fields `link.md` step 2 names (alias, repo URL, local path, optional description) that the walk's context left unset, and the aliases and repos `[services]` already registers. Under `ductus exec` a pre-supplied field is a `key=value` argument (`alias=`, `repo=`, `path=`, `description=`), since the walker refuses a positional argument with `invalid-argument`; `--list` has no exec form, and a host runs it as `link.md` step 1 says.
- The response carries the alias, repo URL, local path and optional description the operator gave, and the walk passes them to the primitive spec 030 adds for `link.md`'s **Additive write**, the primitive a host walking `/ductus:link` calls too, so both paths make one write. Passing the fields is this spec's walker work; the primitive, what it writes and `link.md` step 2's call to it are 030's scenario `registration-writes-through-a-primitive`.
- `ductus exec link` walks `/ductus:link` to its end on a fixture, its registration step answered by a canned response, and a test holds the walk and the block it wrote, the fixture's other tables unchanged.

## Edge Cases

- A response naming an alias `[services]` already registers is refused before any write, and the walk halts at the step; a duplicate repo is warned and allowed.
- The walk passes the response's local path verbatim, which keeps `[services]` on the config-sourced side of the boundary `validate_no_traversal` states (AC29): a path the runtime or the model originates moves the table into the validated tier in the same change.
- The write's own edge cases, field validation, a missing or legacy config file, escaping, an unparseable config and a concurrent writer, are 030's scenario `registration-writes-through-a-primitive`'s.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Under `ductus exec`, who writes the `[services.<alias>]` block?** A primitive, which spec 030 adds for `link.md`'s **Additive write** and which a host walking `/ductus:link` calls too. The write is deterministic, performed mechanically today by a host following that procedure, and specifiable with no policy of its own, so §runtime-boundary's eligibility criteria make it a primitive by default; leaving it to the host after a walk that ends `complete` would make a complete walk indistinguishable from a registered service, with no later gate to catch the omission (§design-principles). A feature-specific primitive lives with its feature, so it lands in 030, which reopens for its implementation, review and analysis, and 022 declares 030 in `cross-spec-impact:` until 030 links back. The agent's recommendation, adopted by the operator, 2026-11-28.
