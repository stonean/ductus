---
section: "Artifact checks"
---

# Status-gated-families-are-not-silent

## Context

`criterion-path-existence` and `body-path-existence` examine a spec only at `done`. Below `done` each reports no finding and no skipped target, so the analysis that satisfies the pre-`done` gate — taken while the spec is `in-progress` — never runs them, and its `unexamined: 0` reads as complete coverage. Measured 2026-10-25 on 0.59.11 and HEAD f24c55c6: a `done` spec with 4 body-path findings gave 0 findings and an empty `skipped` once its status was set to `in-progress`. The scoping to `done` is deliberate (scenarios `criterion-path-existence-family` and `body-path-existence-family`); what is missing is any trace of it in the run's result or record.

## Behavior

- A `check-artifacts` run on a spec below `done` makes visible, in its result and in the `analysis.md` record it feeds, that the status-gated families did not examine the spec, so the result is distinguishable from one in which they ran and found nothing.
- A run at `done` is unchanged.

## Edge Cases

- A spec whose criteria and body carry no backticked path: at `done` the families examine it and report nothing; below `done` the result still says they did not run.
- The pre-`done` gate's own ordering: the analysis it reads is necessarily taken below `done`, so whatever this scenario decides is what the gate's analysis says about these two families.

## Open Questions

- How is the status gate made visible: a new `status-gated` reason in `check-artifacts`' closed skip-reason set (a corpus sweep — every list and count naming the set changes with it), running both families at `in-progress` too, or keeping the bound stated once in the doc comment and `analyze.md` as today — and which of these satisfies the constitution's "a check that cannot run MUST never be indistinguishable from a check that passed"?

## Resolved Questions

*None yet.*
