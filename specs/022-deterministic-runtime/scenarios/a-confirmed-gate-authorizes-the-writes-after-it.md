---
section: "The interpreter"
---

# A-confirmed-gate-authorizes-the-writes-after-it

## Context

`/{project}:prune` previews its reduction, confirms it at a `gate-confirm` step, and then calls `prune-tasks` again "with `apply: true`" to write it (`041-task-pruning`, `framework/commands/prune.md` steps 2, 6 and 7). The exec walker binds a primitive's arguments from its context by field name, and nothing put `apply` into that context. So under `ductus exec prune` the step after a confirmed gate dispatched a second preview, and the walk ended with `complete` and exit 0 having written nothing. Reproduced against the 0.56.0 binary on a fixture with one spent and one pending section: `tasks.md` was byte-identical after the operator confirmed. A path documented to write that reports success without writing is the silent pass-through `QUAL-STUB-001` forbids. The walker test covered only the denied gate, whose "wrote nothing" assertion could not fail because the confirmed gate wrote nothing either.

## Behavior

**A confirmed gate binds `apply: true` into the arguments of every primitive dispatched after it.** A gate exists to authorize the writes that follow it, so the operator's confirmation is what the walk needs to apply them. It is an argument, not a context key: an extension payload the walker builds from its context — `/{project}:specify`'s `writeSpecBody` request after its routing gate — carries no `apply`. A primitive that takes an `apply` argument previews before the gate and writes after it. prune.md's step 7 therefore performs the reduction step 6 confirmed.

**A denied gate is unchanged.** It ends the walk with a clean `complete` carrying `confirmed: false`, so no later step runs and nothing is written.

**Nothing else moves.** A primitive whose description names an `apply` argument takes it, and one that declares none ignores it. prune.md dispatches `prune-plan` only before its gate, at step 3, so the plan half stays a preview under exec: removing a plan section is the host's judgment, and step 8 is host work the walker does not perform.

## Edge Cases

- **A gate before any primitive that takes `apply`.** The binding is unused, as any argument a primitive does not declare is.
- **A `--reset` blocked by the status gate.** The applying call returns the `blocked-needs-force` domain outcome and writes nothing, as the preview reported.
- **Nothing to prune.** The applying call returns `nothing-to-prune` and writes nothing.
- **`--all`.** The one confirmation covers every `tasks.md` reduction, which the applying walk writes spec by spec, as prune.md step 6 says.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
