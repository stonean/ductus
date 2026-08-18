---
section: "Check families"
---

# Family-23-sweep-target-manifest-parity

## Context

[§drift-prevention](../../framework/constitution.md#drift-prevention) tells a
sweep to **keep its own target list current**, because a list naming a
relocated directory sends the grep somewhere clean and the sweep silently
misses the files that moved. That rule is the residue of a real failure: 042
moved the generators from `scripts/` to `.ductus/scripts/`, 049's rename sweep
grepped the stale list, and `config_path_of` in the shipped
`.ductus/scripts/lib/specs-root.sh` kept resolving `.govern/config.toml` then
the legacy root with no `.ductus/` tier. A converged adopter therefore resolved
to a path that does not exist, fell through to the default spec root, and
exited 0.

**The list the rule protects no longer exists.** [050](../050-constitution/spec.md)
promoted the rule into the constitution in categorical form — "specs, rules,
command sources, scripts the pipeline runs, CI configuration, docs, and the
README" — which is correct for a document that ships to adopters, whose repo
layouts differ. The same promotion rewrote the `AGENTS.md` entry into a pointer
mirror and dropped the literal enumeration it had carried: `framework/`,
`scripts/`, `.ductus/scripts/`, `runtime/` (including `tests/fixtures/`,
`tests/golden/`, `tests/parity/`), `.github/`, `docs/`, `README.md`,
`AGENTS.md`, and `specs/NNN-*/`.

Three live pointers now dangle:

- The `AGENTS.md` entry still says to grep "the live-artifact paths above" and
  describes a substitution "applied uniformly across the artifact set above".
  Nothing above it lists paths.
- [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle) case (a)
  defines a mechanical edit as a substitution applied uniformly "per the
  `AGENTS.md` rename rule's scope" — which is now a pointer back to the
  constitution's categorical list. The mechanical-edit rule's scope is
  circular, and that rule is what decides whether a `done` spec reopens.
- The only surviving copies of the enumeration sit in
  [023](../023-govern-refinement/spec.md)'s `living-specs` scenario and
  [043](../043-workflows-sunset/spec.md)'s plan, both citing "AGENTS.md line
  42" — itself stale.

So the gap is not that the list drifts unchecked; it is that there is nothing
left to check. A rule whose subject has been deleted reads as satisfied.

## Behavior

Two halves, landing together. Shipping the family without its input, or the
input without the family, reproduces the failure mode in a new place.

**Restore the list.** `AGENTS.md` carries the literal enumeration again — this
is contributor-side, repo-specific detail, which is what that file is for — and
the three dangling pointers are repaired to name it. The constitution stays
categorical: literal repo paths would be meaningless in a document adopters
install, and 050's decision there was right.

**Then check it.** Family 23 asserts that every **source** path in the
`framework/bootstrap/ductus.md` **Shared Files** manifest is covered by some
entry in that list. Source paths, not destinations: the list governs where a
sweep greps in *this* repository, while destinations describe an adopter's
tree.

The check was measured before being proposed
([§recommendations](../../framework/constitution.md#recommendations)):

- Against the restored list today it produces exactly one finding —
  `.markdownlint-cli2.jsonc`, shipped to adopters and uncovered by any entry.
- Replaying the originating failure, it fires: the manifest ships
  `.ductus/scripts/gen-spec-deps.sh` while the list says `scripts/`, so the
  path is uncovered and the family goes red on the exact defect 049's sweep
  walked past.

That is the opposite result from the acceptance-criterion-supersession check,
which was measured at 215 firings with every sample a false positive and
rejected on the strength of it.

## Edge Cases

- **The check runs in one direction only.** Manifest ⊆ list proves no shipped
  file is unswept; it cannot prove the list is complete, because the list
  legitimately covers paths the manifest never mentions (`runtime/`,
  `.github/`, `docs/`). The family must say which direction it verified rather
  than let a clean run imply both — a check that examined half its subject and
  reports the same zero as one that examined all of it is
  [§design-principles](../../framework/constitution.md#design-principles)'s
  first failure.
- **A list that cannot be parsed is a finding, never a pass.** If the
  enumeration's markers are renamed or the section is restructured, extraction
  yields an empty set and every manifest path trivially fails — or, worse,
  trivially passes, depending on how the comparison is written. Emit the count
  of entries extracted and of paths examined, and treat a zero-entry list as an
  error.
- **Coverage is prefix-based, not exact.** `specs/rules/security-backend.md` is
  covered by `specs/NNN-*/`'s sibling entry for `specs/`, and
  `framework/commands/plan.md` by `framework/`. Requiring an exact row per file
  would fire on every manifest row and make the family noise.
- **The list is prose, not config.** It lives in `AGENTS.md` because that is
  where a contributor reads it; extracting it means parsing a known-shaped
  sentence. Moving it to a machine-owned file would make extraction trivial and
  the rule invisible to the person who has to follow it — which is the trade
  this family accepts, and the reason the parse failure above is treated as an
  error rather than a skip.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
