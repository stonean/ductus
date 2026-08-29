---
section: "Fold-back on merge"
---

# Fold-target-checked-before-the-rewrite

## Context

`/{project}:fold` performs a spec's writes in a fixed order. Step 11 invokes `rewrite-spec-links`, which re-points every inbound pointer across the whole spec root at the fold target. Step 12 invokes `retire-feature`, which is where the fold target's existence is **finally enforced** — the primitive refuses when the target names no directory holding a `spec.md`.

So the enforcement runs *after* the corpus-wide rewrite that assumes it. Step 12's own prose names that refusal as reachable: it says a refusal there "is the answer to a `folds-into` that step 1 reported as unresolved". If it fires, the tree is left with every inbound link re-pointed at a spec that does not exist, and the staging directory still present — the corpus edited on the strength of a destination that was never checked.

The window is narrow rather than closed. The body-edit write (step 6) and `create-scenario` (step 7) both need the upstream spec to exist, so most routes fail earlier and never reach step 11. But "narrow because the earlier steps happen to need the same thing" is a property of the current step list, not a guarantee — a route that skipped those steps, or a target directory that exists without a `spec.md`, reaches the rewrite with the check still ahead of it.

AC29 promises a fold leaves each spec fully folded or untouched, and the resumption contract added alongside it makes every step a no-op where a previous run landed. Neither covers this: re-running after the refusal does not un-rewrite the links, because the rewrite is idempotent in the wrong direction — once re-pointed, nothing names the retiring directory any more, so a second run finds nothing to repair.

## Behavior

The fold target's existence is established before any write that depends on it, not after. A fold whose target does not resolve refuses with the corpus untouched — the same refusal, the same message, reached before the rewrite rather than after it.

`retire-feature` keeps its own check. It is the last line of defense on the one irreversible step and must not be weakened to a caller's promise: the primitive is callable on its own, and a check that exists only in the command is a check an operator can walk past. What changes is that the command establishes the same fact earlier, so the refusal is reached with nothing to undo.

The alternative — recording why the refusal cannot fire once the rewrite has run — is not available, because it can: step 12's prose already describes the case.

## Edge Cases

- **A target directory that exists but holds no `spec.md`** must fail the earlier check exactly as it fails the later one. A directory is not a home content can have landed in, and the two checks must agree on that or the early one is not the same check.
- **A target that appears between the early check and the retirement** (a concurrent write) still meets `retire-feature`'s check, which is why that check stays. The early check is an ordering fix, not a replacement.
- **A fold routed into a scenario** names `{feature}/{scenario}` as the rewrite target but only the feature as the retirement target. The early check is about the feature, matching what `retire-feature` enforces.
- **A re-run after a refusal** finds the staging directory still present and the corpus unedited, so it is an ordinary first run rather than a recovery.
- **An already-retired spec** — the directory gone from a previous successful run — is unaffected: `retire-feature` reports `retired: false` and the early check has nothing to refuse.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
