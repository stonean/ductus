---
section: "The primitive library"
---

# Append-task-honours-slug-with-explicit-body

## Context

`append-task` documents `slug` as *"Ignored when `body` is supplied — the
caller has provided the full body, so no slug is needed."* An adopter passed
both, got a task with no link to its scenario, and repaired it by hand.

The contract is honoured exactly as written, so this is not a defect in the
implementation. It is a sharp edge in the contract: `/ductus:groom` and
`/ductus:amend` both promise a task that references its scenario, and a caller
who supplies a custom body silently loses that linkage with nothing to catch
it. The `scenario-consistency` check family reads the linkage from the rendered
`scenarios/{slug}.md` line, so a task that lost it reads as a task with no
scenario rather than as a task whose link was dropped.

The framework's own callers never trip it. Both scenario-route invocations pass
`slug` and omit `body` explicitly — `/ductus:amend`'s scenario route and its
reconcile pass both say so in the command body, and `/ductus:groom`'s step 5
inherits the same shape. The exposure is entirely to a host that assembles its
own body, which is exactly what an adopting project's model does when the
default single-checkbox body is not what it wants.

"Always prepend the scenario line" — the adopter's suggestion — is not
available: `slug` is optional, and when a caller supplies `body` it usually
omits `slug` entirely, so there is no slug to render. The primitive already
refuses the reverse case (`body` omitted **and** `slug` omitted) with a
`MissingArgument` error rather than guessing.

## Behavior

When **both** `body` and `slug` are supplied, `append-task` prepends the
default scenario line — ``- [ ] Implement the behavior described in
`scenarios/{slug}.md` `` — above the caller's body items, then renders those
items unchanged beneath it.

When `body` is supplied and `slug` is omitted, behavior is unchanged: the
caller named no scenario, so there is none to link, and nothing is prepended.

When `body` is omitted, behavior is unchanged: `slug` is required and renders
the default body alone, as today.

The rule this replaces the "ignored" clause with: **a supplied `slug` is never
silently discarded.** Either it is rendered, or its absence is the caller's own
choice. Ignoring an argument the caller deliberately passed is the failure mode
here — the adopter passed it, reasonably expected it to mean something, and got
no signal that it had not.

`slug` keeps its existing validation against the slug grammar (`BE-INPUT-002`)
on this path, since it is now interpolated into a rendered line in the
`body`-supplied case too. That validation already runs unconditionally on any
supplied `slug`, so no new screening is introduced — it simply stops being
screening for a value that was then thrown away.

The schema documentation is corrected in the same change: the `slug` field's
"Ignored when `body` is supplied" sentence is the contract this scenario
overturns, and leaving it in place would document the opposite of the
behavior.

## Edge Cases

- **A caller who supplies both and does *not* want the link** now gets one.
  This is the deliberate trade: no shipped caller passes both today, so the
  change is non-breaking in the corpus, and a caller who wants a bare body can
  express that precisely by omitting `slug`.
- **Marker normalization is unaffected.** The prepended line is rendered by the
  primitive, not caller-supplied, so it never passes through
  `strip_bullet_marker` and cannot double a marker.
- **Phased and flat `tasks.md` render identically.** The prepend happens inside
  body assembly, above the structure choice, so both the `## N.` and `### N.`
  paths get it without a second code path.
- **An empty `body` array with a `slug` supplied** renders exactly the default
  body — the same output as omitting `body` entirely, which is the consistent
  reading rather than a special case.
- **Idempotency is unchanged.** `append-task` appends a new numbered block per
  invocation; it does not deduplicate, and this scenario does not ask it to.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
