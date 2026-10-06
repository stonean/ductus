---
section: "Primitive request/response schemas"
---

# Argument-hint-needs-a-token

## Context

A command's `argument-hint:` frontmatter is what a host renders when it offers the command, and a host injects the invocation's arguments into the command body through **substitution only**. Pi's `substituteArgs` replaces `$ARGUMENTS` / `$1` / `$@` / `${N:-…}`, and there is no fallback that appends an unreferenced argument. A command that declares an `argument-hint` but carries no substitution token in its body therefore has an unreachable interface: the operator supplies an argument, the host discards it, and the command reads its no-argument branch.

This is the defect behind `/ductus:target 015` (renders the resolved target and stops), `/ductus:link <alias> <repo> <path>`, and `/ductus:prune --apply` — the argument is silently dropped in all three.

## Behavior

`check-command-flags` reports the **third** direction of the argument-hint contract: a command that declares an `argument-hint` but whose body contains no substitution token is a finding. The accepted token set is `$ARGUMENTS`, `$@`, `$1`, `${N:-…}`, `${@:-…}`, `${@:N}`, `${@:N:L}` — the tokens the supported hosts expand. The subject is every examined command that declares a hint, whether or not it carries a `Flags` table (which the first direction governs); the count of those commands is the direction's denominator, reported alongside `examined` and `with-flags-table` so a clean result can say what it examined.

`framework/commands/{target,link,prune}.md` are the three instances the check catches; each gains a one-line `$ARGUMENTS` reference under its H1 so its declared interface is reachable.

## Edge Cases

- A command that declares **no** `argument-hint` is not a subject: the four tokenless commands (`audit`, `groom`, `help`, `status`) take no argument and are correct as they stand.
- A command whose body uses any accepted token — not only `$ARGUMENTS` — passes. The check accepts the full set, so a command legitimately written with `$1` is not a false finding.
- The check reads the **sources** (`framework/commands/*.md`), never the generated copies: a copy carries whatever its source carries, and an adopter cannot repair one.
- Direction 1 (a `Flags` table flag absent from `argument-hint`) and direction 3 are independent: a command can fail either, both, or neither.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
