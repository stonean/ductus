---
section: "Bash script relationships"
---

# Derive-unparseable-frontmatter-is-reported

## Context

`/ductus:review` recorded this against `derive-references` under
`QUAL-CLAIM-001` and it was left as a finding rather than patched at review
time, because the honest fix is a result-shape addition and that is a
deliberate change, not a review-time edit.

`splice_references` places the `references:` block at one of two anchors: the
line after `dependencies:`, or the closing `---`. A spec whose frontmatter is
**unterminated** and which carries **no `dependencies:` key** reaches neither.
The rewrite is skipped, the spec never enters `updated`, and the result is
byte-identical to one for a spec that genuinely needed no change.

That is exactly the shape `QUAL-CLAIM-001` names:

> A result that reports a clean, empty, or in-sync state SHOULD distinguish
> *"examined the subject and found nothing"* from *"could not examine the
> subject"*, rather than emitting the same value for both.

The two primitives already report `examined`, `untracked-skipped`, `unwritten`
and `registered-services` precisely so a caller can tell scope from silence —
this is the one remaining hole in that surface, and it is the one where the
subject was reachable and still went unexamined.

`derive-dependencies` has the same gap for a different reason: its splice only
rewrites a `dependencies:` key found **inside** the frontmatter, so on an
unterminated block the key is never located and the spec is silently left
alone. Both primitives need the same treatment; fixing one would leave the
pair inconsistent, which is the drift the shared scanner exists to prevent.

Narrow in practice — it needs an unterminated block, and for `derive-references`
a missing `dependencies:` key as well — but the failure is silent, and a spec
mid-edit is exactly when a pre-commit hook runs.

## Behavior

Both `DeriveDependenciesResult` and `DeriveReferencesResult` gain
`unparseable: Vec<String>`: the repo-relative paths of specs whose frontmatter
could not be read well enough to derive from, sorted, absent-when-empty in the
same sense as the other list fields.

A spec lands there when its frontmatter fence is never closed — the condition
both splices already detect implicitly by never finding their anchor, made
explicit and reported instead of swallowed.

The contract a caller can then rely on: **an empty `updated` means
examined-and-clean only when `unparseable` is also empty.** That is the same
pairing `check-artifacts` established with `skipped` and `derive-boundary`
with `guidance`, so this introduces no new idea — it applies the existing one
where it was missing.

Reporting is the whole behavior. Neither primitive repairs the frontmatter,
and neither treats it as an error: `validate-frontmatter` owns diagnosing a
malformed block, and a derivation that halted the pre-commit hook over a spec
it was not asked about would be worse than one that names it and moves on.

The CLI's blocking contract is unchanged — an unparseable spec does not fail
the check, because it is not drift. It is an unknown, and the existing rule
that an unknown is never escalated into a defect holds here too.

## Edge Cases

- **A spec with no frontmatter at all** is not unparseable for this purpose:
  there is no block to close, nothing is derived from it, and reporting every
  such file would bury the real signal. Only an *opened but unterminated*
  block counts.
- **`--staged` scoping applies unchanged.** The field describes what this run
  examined, not the corpus.

  This bullet originally added "an unparseable spec outside the staged set is
  not enumerated, so it cannot appear", which was true of `derive-references`
  only while that primitive narrowed its *enumeration* to the staged set.
  [`derive-references-unstaged-drift-is-reported`](derive-references-unstaged-drift-is-reported.md)
  removed that narrowing — a reference derives from the `[services]` registry
  as well as the body, so a service rename drifts specs nobody staged. Both
  primitives now walk every tracked spec and filter only the write, so an
  unparseable **tracked** spec is reported whether or not it is staged. What
  still cannot appear is an unparseable *untracked* spec, which neither
  primitive enumerates (`untracked-skipped` is where that scope is stated).
- **The field is additive.** Existing consumers ignore it, and the MCP goldens
  that assert the ordinary payload stay byte-identical when it is empty —
  which is what keeps this from being a breaking change to the wire contract.
- **`data-model.md` records it**, because that file is the canonical registry
  of primitive result shapes and a field that exists only in code is one a
  markdown-only host cannot know to read.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
