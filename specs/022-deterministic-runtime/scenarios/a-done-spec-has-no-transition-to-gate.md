---
section: "Follow-on scenarios"
---

# A-done-spec-has-no-transition-to-gate

## Context

`check-review-gate` is the **pre-done** gate: every check it runs exists to answer whether an `in-progress` spec may become `done`. It never reads the spec's status, which was harmless while its checks were about records and contracts. [047's analyze-freshness check](../../047-analyze-findings-durability/scenarios/analyze-record-freshness.md) made it visible, because that check compares the spec's artifacts against a digest of what the analysis read — and `set-status` rewrites `spec.md`.

So the completing flip stales the record by construction. Verified on 047 seconds after its own transition: the gate had returned `passed: true`, `set-status` moved `in-progress → done`, and the next call returned `blocked: analysis is stale — 1 artifact(s) changed since it ran: specs/047-analyze-findings-durability/spec.md`. The analysis was not wrong and the record was not neglected; the spec simply finished, and finishing edits the spec.

Nothing is broken today. The gate is invoked at the transition, and the family that audits `done` specs — `analyze-state-drift` — deliberately checks the two states that gate rather than freshness. The reachable defect is a confusing one: re-running `/{project}:implement` against a finished spec reports a stale analysis and directs the operator to re-run `/{project}:analyze` on work that is complete. Following that advice writes a fresh record, which the next `set-status`-free run then reports as current — so the advice appears to work, which is worse than advice that plainly fails.

## Behavior

**`check-review-gate` reports that a spec at `status: done` has no transition to gate.** It reads the spec's status — already in hand, since the frontmatter is parsed for the `review:` and `analyze:` blocks — and when the status is `done` returns a distinct outcome rather than running the checks: the transition this gate exists to authorize has already happened, so every check below it is answering a question nobody asked.

The outcome is **not** `passed: true`. A gate that says "passed" for a spec it did not examine is the `QUAL-CLAIM-001` conflation the rest of this gate is built to avoid, and a caller could read it as authorization to transition a spec that is already there. It is its own variant, naming the state.

**Ordered first, ahead of the markdown lint.** Every other check presumes a pending transition; running any of them on a `done` spec spends work to produce an answer that cannot matter. Ordering it first also means the lint cannot block a completed spec on a violation introduced long after it closed.

**`/{project}:implement` halts on it cleanly** — it is not a failure. The walker reports that the spec is already `done` and exits without proposing anything, which is what the operator's re-run actually deserves as an answer.

## Edge Cases

- **A `done` spec being deliberately re-examined.** `/{project}:review` and `/{project}:analyze` both accept `done` specs and are unaffected — neither goes through this gate. Only the transition path short-circuits.
- **A spec at `draft`, `clarified`, or `planned`.** Unchanged: the gate runs its checks as today. It is not this check's job to police whether the *forward* path was followed, only to recognize a transition that has already happened.
- **A `folds-into` spec at `done`.** Not reachable — the pending-fold check blocks the transition, so such a spec never reaches `done` through the gate. If one exists by hand-edit, the done-state outcome reports it as done and the fold check does not run; the un-folded-specs family is what surfaces that spec, and it is not this gate's subject.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Why not simply exclude `status:` from the analyze digest, so the flip stops mattering?** Because the families condition on status — `criterion-path-existence` examines `done` specs only, `scenario-consistency` is skipped on them — so a digest blind to status would miss a change that genuinely alters what an analysis finds. The digest is right; the gate is asking at the wrong time.
- **Why not have `/{project}:implement` re-record the analysis after the flip?** Because that writes a record no run substantiates. 047's AC11 rejected backfilling for exactly this reason: a record asserting an analysis that did not happen inverts the mechanism the record exists to provide.
