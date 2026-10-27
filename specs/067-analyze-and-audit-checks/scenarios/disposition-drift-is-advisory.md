---
section: "Artifact checks"
---

# Disposition-drift-is-advisory

## Context

[058 — Findings route at discovery](../../058-findings-route-at-discovery/spec.md)'s `findings-cost-in-proportion-to-risk` stopped undecided advisory findings and review observations holding `done`. The `disposition-drift` family reported a `done` spec whose `review.md` counts undispositioned findings as `Blocking`, on the ground that the status said finished while the record said otherwise. Once those findings no longer hold `done`, the two do not disagree, and a blocking report would hold every such spec until a reopen cleared it.

## Behavior

- **`disposition-drift` is `Advisory`.** `check_artifacts` still emits one finding when a `done` spec's `review.md` records `undispositioned` above zero, with `path` = `spec.md`, and its message ends `they do not hold done`.
- **A gating finding on a `done` spec is still reported as blocking**, by `review-state-drift` for the review record and by `/{project}:analyze`'s analyze-state drift for its own record; this family adds nothing there.
- **`/{project}:analyze --fix` does not revert on it.** Its revert triggers are review-state drift and unresolved scenario open questions.

## Edge Cases

- **A record with no `dispositions:` map** still produces nothing.
- **The finding is itself advisory**, so a run may discard it with its reason, and a stored decision quiets it until its message changes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
