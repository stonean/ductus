---
section: "Follow-on scenarios"
---

# Criterion-adopter-scope-destinations

## Context

`criterion-path-existence` reported 19 findings across specs 000, 003, 008, 012,
018, and 044 for paths `ductus` creates in an *adopter's* checkout —
`specs/templates/`, `specs/rules/*`, `specs/system.md`, `specs/errors.md`,
`specs/events.md`, `.githooks/ductus-pre-commit`, `.ductus/constitution.md`.
Every one of those criteria is correct and satisfied in the repo it describes.
They fail here for a single reason: this repo is the framework *source*, not an
adopter.

The existing `root-absent` arm cannot catch them. It exempts a candidate whose
top-level segment is missing, and these segments — `specs`, `.ductus`,
`.githooks` — all exist here. The `in the project` non-assertion marker was
meant for the same class but only matches criteria that happen to use that
phrasing; the other five specs use "scaffolded", "shipped", "adopted project",
or no adopter phrase at all.

[045's data-model](../../045-decision-state-drift-detection/data-model.md)
measured this class and named it "a dogfooding artifact, not a check defect",
deferring it on the reasoning that it does not generalize to the projects the
check ships to. That reasoning was right about generality and wrong about cost:
19 of 21 findings were noise, so the two true positives (005's
`framework/workflows/`, 025's `scripts/lint-ductus-toml.sh`) sat invisible
inside a wall of false ones. A check whose signal is 10% is not consulted, and
one that is not consulted catches nothing.

## Behavior

`adopter_destinations(repo)` derives the set of paths this repo declares it
scaffolds elsewhere from the **Shared Files** manifest tables in
`framework/bootstrap/ductus.md` — the canonical registry of what lands where,
per the constitution's canonical-sources map. It takes the destination column
of every row whose cell is exactly one backticked span, which skips header
rows, separator rows, and prose cells without needing to know how many tables
the section holds.

A candidate is suppressed when it equals a destination, or when it is a
*directory containing* one — the second case is what lets a criterion naming
`specs/templates/` match the six per-file template rows beneath it.

The check runs **after** the resolve and `root-absent` arms, so it only ever
fires on a path that failed to resolve here. It records
`skipped { reason: "ships-to-adopter" }` rather than dropping the candidate:
the report still states which paths went unexamined and why, so `clean: true`
with a non-empty `skipped` keeps its meaning under `QUAL-CLAIM-001`.

Measured against this repo: **21 findings → 2**, with 9 distinct
`ships-to-adopter` skips. The two survivors are exactly the true positives.

## Edge Cases

- **No manifest** — an adopter checkout has no `framework/bootstrap/`, so
  derivation returns an empty set and nothing is suppressed. That is the
  correct shape rather than a limitation: in an adopter these destinations *do*
  resolve, so there is no finding to suppress. Pinned by
  `without_a_manifest_nothing_is_suppressed`.
- **Derivation failure fails toward reporting.** A missing or unparseable
  manifest yields an empty set, so findings are emitted rather than silently
  swallowed. The dangerous direction here is the opposite of Family 17's: there
  an empty derivation would have meant checking nothing, so it had to be a
  finding; here it means checking everything, which is safe.
- **A path the manifest does not ship still flags.** The suppression is scoped
  to declared destinations, not a blanket adopter-layout exemption — 025's
  `scripts/lint-ductus-toml.sh` sits under a real `scripts/` directory and is
  still reported. Pinned by
  `a_genuinely_stale_path_still_flags_alongside_the_manifest`.
- **Placeholder destinations are inert.** The manifest also carries rows like
  `{config_dir}/commands/{project}/plan.md` and `~/.augment/settings.json`.
  These enter the set but can never match a candidate: the criterion path
  grammar already rejects `{`, `}`, and `:`, and a destination with no `/`
  cannot match a candidate that requires one.
- **A destination that also exists here** (`.markdownlint-cli2.jsonc`,
  `.ductus/scripts/*`) never reaches this arm — it resolves, so the check
  returns before the manifest is consulted.
- **Manifest read cost** is one file per feature, not per candidate.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
