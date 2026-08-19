---
section: "Acquisition"
---

# State-a-version-checks-the-pin

## Context

A **live but stale** runtime was never detected. §ductus runtime detection
resolves State A on tool-inventory introspection alone — "any `ductus`-namespaced
MCP tool ⇒ State A" — and State A then declared the runtime live, contributed
nothing to the pending-restart set, and emitted no message. §Runtime acquisition,
which is the only place `{pin}` is compared against anything, runs **only in
State B**. So a project whose runtime was registered but old passed detection
silently, every run, forever.

Found 2026-08-19 while updating a real adopter project. The store held
`0.29.10`; the framework pinned `0.31.0`. The `/ductus` run refreshed the
adopter's pre-commit hook to the current version, which calls
`derive-dependencies` and `derive-references` — primitives `0.29.10` does not
carry — and the shell generators those primitives replaced had already been
deleted from the framework. Every commit in that project then halted on a bare
`unrecognized subcommand` from clap.

The shape is worth naming, because it is the one this framework keeps paying
for: the `/ductus` run **reported success**. A tool *was* in the inventory, so
detection's question was answered truthfully — it was simply the wrong question.
The run refreshed a hook that depended on a runtime it never checked, and the
failure surfaced later, in a different command, as something that looked like a
hook bug.

It is also the mirror image of the greenfield defect fixed alongside it
([pin-is-readable-when-acquisition-needs-it](pin-is-readable-when-acquisition-needs-it.md)):
that one blocked adopters who had **no** runtime, this one silently degraded
adopters who had **the wrong** one. Between them, acquisition was correct only
for the adopter who was already current.

## Behavior

State A version-checks the live runtime against `{pin}` before trusting it.

Probe the resolved binary — the `[runtime] path` when the project configures
one, else `{store-path}` — and read its reported version. This is the same probe
§Runtime acquisition step 2 already performs, and the §Permission Setup seed
already authorizes it, so the check costs a version comparison and no new grant.

Three outcomes:

- **Reports `{pin}`** — proceed exactly as before: nothing added to the
  pending-restart set, no message. This is the routine path and it is unchanged.
- **A project-supplied `[runtime] path` reports something else** — emit Branch
  1's existing warning and continue. A project naming a path has stated
  deliberately which binary it wants, and a development build running ahead of
  the last release is the expected case.
- **Anything else** — the runtime is live but stale. Acquire `{pin}` per
  §Runtime acquisition Branch 2, then run the rest of the session through
  `{pointer-path} <primitive>` rather than the MCP tools, and carry the
  acquisition to the **Closing restart**.

That last clause is the non-obvious one. Re-acquiring does **not** refresh the
running MCP server: it was spawned once at session start and holds the old
binary regardless of what is now in the store. Continuing to call its tools
would run the stale code the check just diagnosed. Switching to the freshly
acquired CLI for the remainder is the same move State B already makes, for the
same reason.

## Edge Cases

- **The adopter already in the broken state.** The State A check prevents the
  state; it does not rescue a project whose commits are already halting, because
  reaching the check requires running `/ductus`. So the shipped pre-commit hook
  gains its own guard: before calling a primitive it probes that the binary
  carries it, and halts naming the installed version and the missing subcommand.
  It asks **capability**, not version — there is no pin for the hook to drift
  from, and a hand-placed binary or a future primitive is covered by the same
  question.
- **A runtime newer than the pin.** Treated as a mismatch and re-acquired down
  to `{pin}`, deliberately: the framework revision states which runtime it was
  tested against, and "newer" is not a synonym for "compatible". A project that
  wants to run ahead has `[runtime] path`, which is exactly the warn-and-continue
  branch above.
- **The probe itself fails** — the binary will not execute, or reports nothing.
  §Runtime acquisition step 2 already settles this: treat it as *no usable
  runtime* rather than *version unknown*, and acquire. A false negative costs a
  version comparison; a false positive costs the failure this scenario exists to
  end.
- **State A with no store and no `[runtime] path`.** Possible when an adopter's
  MCP config names a binary elsewhere by hand. The probe finds nothing to run,
  which lands in the case above and acquires — the correct outcome, since the
  run cannot otherwise establish what version it is talking to.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
