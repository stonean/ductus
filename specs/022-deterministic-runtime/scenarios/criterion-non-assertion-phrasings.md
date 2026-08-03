---
section: "Follow-on scenarios"
---

# Criterion-non-assertion-phrasings

## Context

`criterion-path-existence` exempts a criterion that is not a live claim about
this repo — a deletion, a rename, an adopter-scoped path, a hedge — because a
path that fails to resolve *confirms* such a criterion rather than contradicting
it. The mechanism is `NON_ASSERTION_MARKERS`, a closed list of phrases.

Re-measuring across all 47 specs found 25 findings. Reading each criterion
rather than classifying by path prefix — the discipline
[045's data-model](../045-decision-state-drift-detection/data-model.md) records
as the correction to an earlier mis-triage — splits them three ways:

- **19 adopter-scope.** Paths `govern` creates in an adopter's checkout
  (`.govern/constitution.md`, `specs/rules/*`, `.githooks/govern-pre-commit`,
  `specs/templates/`, `specs/system.md`). 045 already triaged these and ruled
  them "a dogfooding artifact, not a check defect", noting the class "does not
  generalize to the projects this check ships to". **Deliberately unaddressed
  here** — suppressing them would mean shipping govern-repo-only machinery, and
  it would key off a manifest an adopter does not have, so it would never
  engage where the check actually runs.
- **2 true positives.** 005's `framework/workflows/` (sunset by 043) and 025's
  `scripts/lint-govern-toml.sh` (never built). These are what the family exists
  to surface; they are spec-level drift, not check defects.
- **4 residual false positives.** Three phrasings the marker list missed. Those
  are this scenario.

## Behavior

Three additions, each earned against a real criterion in this repo and each
generalizing beyond it:

- **`deleted`** replaces the narrower `is deleted` / `are deleted` pair. The old
  forms missed the past-tense-agent phrasing a criterion reaches for when it
  names the commit that did the deleting — 045's own AC18, ``… after
  `531e3ea` deleted both``. A criterion carrying the word at all is describing a
  removal, which is the group's premise.
- **`(was` + space** — the parenthetical history form of a rename: ``… for session
  state (was `.claude/gov-session.json` pre-0.10.0)``. The old path is named to
  date the change, never to claim it is still present. The opening paren is
  load-bearing: bare `was` would exempt any past-tense criterion.
- **`target paths`** — a path named as the *subject of a migration record*
  (``whose target paths cover … `framework/workflows/```) is data inside a
  manifest describing what to remove, not a claim that the path is delivered.
  This is a fifth group alongside the four already documented.

Measured effect: 25 findings → 21, suppressing exactly the four residual cases
and nothing else. The 19 adopter-scope and 2 true positives are untouched,
verified by diffing findings on `(spec, path)` keys before and after.

## Edge Cases

- **`deleted` is wider than the pair it replaces** — a criterion that asserts a
  live path *and* mentions a deletion elsewhere is now exempted whole. That is
  the documented behavior of every marker ("the whole criterion is exempted, not
  just the matched path"), because a criterion about a transition names its
  endpoints together. Erring toward silence is how the rest of this family
  already errs.
- **Bare `was` is deliberately not a marker** — it would exempt most past-tense
  criteria. Only the parenthetical `(was` + space form qualifies.
- **`target paths` is phrase-shaped, not word-shaped** — bare `target` would
  exempt unrelated criteria, the same trap `adopter` was rejected for.
- **The 19 adopter-scope findings persist by design** — a re-run of this repo's
  sweep still reports them. They are the honest output of running a
  framework-authoring check against the framework's own source; the promotion
  verdict 045 recorded (do not promote; re-measure in an adopter repo) is
  unchanged by this scenario.
- **Marker matching remains case-insensitive and code-span-inclusive** — a
  marker is prose, and a criterion carrying one anywhere is describing a
  transition throughout. Unchanged.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
