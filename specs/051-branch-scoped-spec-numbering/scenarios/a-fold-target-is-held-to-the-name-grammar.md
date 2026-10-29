---
section: "Fold-back on merge"
---

# A-fold-target-is-held-to-the-name-grammar

## Context

`rewrite-spec-links` writes its `to` argument into every file that names the retiring directory: the feature half replaces the directory segment, and on a fold into a scenario `scenarios/{scenario}.md` replaces the tail. Since task 29 those files include `.githooks/pre-commit`, which git executes, and `CLAUDE.md`'s `@` imports, which every session loads.

`to` was held only to no parent-directory escape, containment inside the project, and two non-empty halves. The scenario half, which the data model calls a scenario slug, accepted any text — a space, `;`, `$(...)`, a further `/` — and the feature half accepted any directory holding a `spec.md`, `templates` included, which every adopter has. `/{project}:fold` validates the slug one step earlier, at `create-scenario`, so only a caller outside that flow reached the gap; but the primitive is callable on its own, which is why its other guards live inside it. A denylist as the sole defense for a constrained input is what `BE-INPUT-002` forbids, and it is the pattern `retarget-sessions` was held to the session validators for (062). Found by [048](../../048-govern-acquired-runtime/spec.md)'s review on 2026-10-29: `--to '002-b/a b;c'` wrote `specs/002-b/scenarios/a b;c.md` into a fixture's hook and README.

## Behavior

- The feature half of `to` is a feature directory name by the shared membership rule, and its scenario half, when present, matches the scenario slug grammar: the two checks `retarget-sessions` applies. Anything else is refused as an invalid `to`, naming the grammar, before any file is read or written.
- `from` is unchanged: it is compared against path segments and never written.

## Edge Cases

- **`templates`, or any other directory that is not a feature**, is refused though it holds a `spec.md`: the existence check answers whether a link can resolve, not whether the name is a feature's.
- **A branch-scoped target** (`{identifier}.{n}-{slug}`) passes, so a rename or a fold into one is unaffected.
- **A refused call leaves the corpus byte-identical**, as the existence check's refusal does.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
