---
section: "Consolidation"
---

# Stranded-session-after-removal

## Context

Two commands remove a feature directory, and they answer the stranded-session question differently.

`/{project}:fold` re-targets the session at the upstream spec as its last step, and says why: "the targeted directory no longer exists after step 12, so leaving the session pointing at it would strand every follow-on command on a path that is gone."

`/{project}:consolidate` deliberately does not. Its report step names the stranded target and directs the operator to `/{project}:target`, on the reasoning that consolidating a spec asserts its *content* belongs with the target, not that the operator's next *work* does.

Each is locally defensible. Together they are two answers to one problem, and nothing records which is the framework's position — so the next command that removes a directory has no precedent to follow, only a coin to flip. That is the shape §drift-prevention exists to catch: a decision made twice, differently, with neither instance knowing about the other.

The asymmetry may also be real rather than accidental. Fold's target is *where the content went*, so re-targeting follows the operator's attention. Consolidation's target is a spec that already existed and that the operator may have no interest in — re-targeting there is a guess about intent. If that distinction is the answer, it is worth writing down; if it is a rationalization, the two should converge.

## Behavior

The framework takes one position on what happens to a session target whose feature directory has just been removed, and both commands implement it.

- The position is stated once, where a reader meets it — not restated per command.
- Whichever way it resolves, the *reason* is recorded: either re-targeting follows the content and consolidation is the exception with a stated justification, or leaving the session stranded-but-named is the rule and fold is the exception.
- A stranded session is never silent. Whatever the position, an operator whose target no longer resolves is told so by the command that removed it, rather than discovering it from the next command's failure.

## Edge Cases

- **Clearing rather than re-targeting is a third option** neither command currently takes. `write-session`'s clear mode removes the target block while preserving `cli-config-dir`, which leaves the operator explicitly untargeted rather than pointed at a directory that is gone or at a spec they did not choose. It may be the honest answer for consolidation specifically.
- **The session file is per-contributor and gitignored.** A teammate's session may point at the removed directory and nothing in this repository can reach it — so whatever the position, the first command that teammate runs must fail legibly rather than confusingly. This is a bound on what the decision can achieve, and it should be stated rather than left to be discovered.
- **`/{project}:fold`'s behavior is settled** by spec 051 and is not reopened by this scenario; if the position lands the other way, changing fold is a cross-spec impact on 051 to record rather than a silent edit.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
