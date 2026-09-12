---
section: "Follow-on scenarios"
---

# Config-sourced-paths-and-the-traversal-boundary

## Context

`validate_no_traversal` is the `BE-INPUT-004` defence-in-depth check, and its own doc comment scopes it: *"Primitives that accept paths from the host or LLM call this before any filesystem operation."* Primitives whose paths come from committed project config do not call it, because `..` is the normal case there — `resolve-references` reads `[services]` paths that way, and `resolve-constitutions` ([055](../../055-shared-constitution/spec.md)) reads `[constitutions.*]` paths the same way, with a sibling checkout (`path = "../governance"`) being the documented shape.

That boundary is sound, and `/{project}:review` evaluated it against `resolve_constitutions::classify` and correctly declined to file a finding. What makes it worth recording is that its soundness rests on a claim about **how the value got there**, not on anything the primitive can see: the path is trusted because a human hand-edited committed config. `resolve-constitutions` has no way to distinguish that from a value some future command wrote.

`/{project}:link` is the precedent that makes this more than hypothetical — it is a command that writes a registry table programmatically. `[constitutions.*]` has no such command today, and `framework/bootstrap/ductus.md` states that registration is a hand-edit. But the invariant lives in prose in the installer's config schema, while the code that depends on it is in a different file, and nothing connects them. If a `/{project}:link`-shaped command is ever added for constitutions, the trust claim becomes false silently: no test fails, no review fires, and the primitive keeps resolving a path it now has no grounds to trust.

## Behavior

The trust boundary is stated where the code that relies on it can be read against it, rather than inferred from two files that do not reference each other.

`primitives/mod.rs` is the canonical statement — `validate_no_traversal`'s doc comment already scopes the check to host- and LLM-supplied paths, and it additionally names the converse obligation: a primitive that reads a path from committed config may skip the check **only while nothing in the framework writes that config programmatically**, and adding such a writer is what moves the path into the validated tier. `resolve_constitutions::classify` and `resolve_references::classify` cite that statement rather than each restating the reasoning, so there is one rule and two pointers.

The obligation is recorded as a rule for whoever adds the writer, not as a check, and that is stated plainly rather than implied. Nothing detects a new command writing `[constitutions]`, no test fails when one appears, and pretending otherwise would be the defect [§grounding](../../../framework/constitution.md#grounding) names — a rule implying enforcement it does not have.

## Edge Cases

- **A command is added that writes `[constitutions]`.** The path is then framework-written rather than hand-edited, so `resolve-constitutions` calls `validate_no_traversal` on it — and that change ships with the writing command, not after it.
- **An adopter hand-edits a traversing path today.** Unchanged and intended: `..` is the documented normal case, the config is committed and reviewable, and rejecting it would break the sibling-checkout layout the manual prescribes.
- **An absolute path.** Also accepted today, and also a committed-config decision rather than an oversight — though `docs/shared-constitution.md` already tells adopters a relative sibling path is required, because `.ductus/config.toml` is team-shared and an absolute path resolves on one machine only.
- **`resolve-references` and `[services]`.** Governed by the same statement, and `/{project}:link` already writes that table — so it is the case where the converse obligation is live rather than latent, and the boundary statement has to be true of it as written.
- **A future registry that is neither.** A third config table read by a primitive inherits the same question, and the boundary statement is what it is judged against — which is the whole reason it is stated once rather than per primitive.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
