---
section: "Artifact checks"
---

# Criterion-negated-creation-phrasing

## Context

`criterion-path-existence` exempts a criterion that is not a live claim about
this repo, because a path that fails to resolve *confirms* such a criterion
rather than contradicting it. The mechanism has been `NON_ASSERTION_MARKERS` —
a closed phrase list, widened to fourteen by
[criterion-non-assertion-phrasings](criterion-non-assertion-phrasings.md) and to
fifteen by [body-path-existence-family](body-path-existence-family.md).

An adopter running `ductus-v0.52.0` reported a false positive of a shape the
list cannot reach. A `done` spec's criterion read:

```markdown
- [x] AC13: `server/.gitignore` exists (Rails-generated) and `server/config/master.key` is
  git-ignored and not committed; no nested `server/.git/` repository was created.
```

and the family reported `` acceptance criterion names `server/.git/`, which no
longer resolves ``. The nested repository is absent, which is precisely what the
criterion asserts — the finding is exactly backwards, the failure mode
[045's data-model](../../045-decision-state-drift-detection/data-model.md)
records as the whole reason the exemption exists.

Every one of the fifteen phrases describes a path that **existed and was
removed**. A path asserted to have **never been created** is a different
construction, and no member of the list matches it.

It cannot be fixed by appending to the list, and that is the load-bearing part.
Matching is a flat `contains` over the whole criterion, so `was created` —
the only phrase that spans every criterion of this shape — would exempt
positive delivery claims too (``the migration file `db/migrate/…` was
created``), blinding the family wholesale. Blinding a check is strictly worse
than a false positive in it. The narrower `was never created` / `was not
created` do not over-exempt but miss the reported criterion, whose word order
is `no <noun> … was created`: the negator is separated from its verb by the
noun it negates.

## Behavior

A sixth exemption group, carried by a **predicate** rather than a phrase. A
criterion is exempted whole — the same whole-criterion exemption, recorded
under the same `not-a-live-claim` reason — when one of its clauses carries a
negator (`no`, `not`, `never`, `without`) **before** a creation verb
(`created`, `added`, `introduced`). Both lists are closed and framework-fixed,
for the reason the phrase list is: a per-project vocabulary would make the
promotion threshold measure configuration rather than drift.

Three rules keep the predicate from collapsing into the over-exempting phrase:

- **Clause-scoped.** `no` and `not` are common in criteria prose, so a
  criterion-wide match would exempt a delivery claim in another clause.
  ``…`scripts/gen-spec-deps.sh` was added by this spec; no further generators
  are needed`` is a live claim about a path that must still resolve.
- **Word-matched, not substring-matched.** `not` hides inside `cannot` and
  `note`, `no` inside `nano` — the `adopter` trap of the phrase list, in the
  other direction. An identifier is one word: `_` joins, so the field name
  `introduced_in` is not the verb `introduced`.
- **Ordered.** The negator precedes the verb. English negates a verb from in
  front of it, so a negator *after* the verb belongs to something else:
  ``X was created and not modified since`` is a delivery claim with a trailing
  qualifier, and still flags.

Clauses split on `;`, `,`, and a period **followed by a space**.

Three restatements, bound by `/{project}:audit` Family 18's new `18e` arm:
045's data-model carries the canonical table, `check_artifacts.rs` the
`CREATION_NEGATORS` / `CREATION_VERBS` constants, and `analyze.md` the
adopter-facing copy that cannot become a pointer.

Proven by probe in both directions against a fixture reproducing the reported
criterion: `ductus-v0.52.0` emits the reported finding verbatim, and the
patched binary emits nothing while recording all three of the criterion's paths
— `server/.gitignore`, `server/config/master.key`, `server/.git/` — as
`not-a-live-claim`. `18e` was proven the same way, against three injected drift
modes (a word dropped from the runtime constants, a word added to the adopter
restatement only, the canonical section renamed) plus the restored baseline.

## Edge Cases

- **A bare `.` cannot be a clause boundary in this family.** It is the trap
  this scenario exists to not fall into twice: splitting on the character cuts
  `server/.git/` and `master.key` in half, stranding the negator in one
  fragment and its verb in the next. A first draft of this predicate did
  exactly that and therefore did not exempt the criterion it was written for,
  while returning a confident `false`. A criterion-final period needs no split,
  and `e.g.` never reaches the predicate — it is a marker, so the criterion is
  already exempt.
- **Over-splitting errs toward flagging, not silence.** A negated-creation
  clause interrupted by a comma (`no repository, nor its index, was created`)
  stays a finding. This is the one place the family deliberately inverts its
  own error direction, because the failure mode guarded against here is a
  blinded check rather than a noisy one.
- **The phrase list is unchanged by this group.** It adds no phrase, so
  `NON_ASSERTION_MARKERS` and all three spelled-out counts — 045's data-model,
  `analyze.md`, and
  [criterion-path-existence-family](criterion-path-existence-family.md) — are
  unchanged, and Family 18's `18a`–`18d` are untouched.
- **The predicate's word lists state no count, deliberately.** The counts
  above exist because that prose states one; prose that states none cannot
  drift from it, so `18e` has no count arm and the sets alone are bound.
- **The group names in `18e` are not checked.** The two lists are compared as
  one union, matching `18a`–`18c`'s stance that the contract is the set and the
  grouping is editorial. A negator swapped into the verb row is not a drift
  mode anyone has; a word dropped from either list is, and the union catches it.
- **Measured across this repo's corpus, both binaries: 0 findings before, 0
  findings after.** No true positive is lost and none is gained; the fix is
  precision-only here. What moves is the skipped set — `not-a-live-claim`
  109 → 118, `ships-to-adopter` 12 → 9, `root-absent` unchanged at 19 — because
  an exempted criterion records *every* path it names, including ones that
  resolve and were previously recorded nowhere.
- **The over-exemption rate is real and is stated rather than implied: 14
  criteria across 55 specs newly match, and 4 of them match for a reason other
  than a path's creation.** 012's AC11 (`is **not** the canonical set — later
  specs added the rule files`), 022's own AC27 (`never held against a violation
  introduced after it closed`), 025's AC9 (`NOT retro-added to spec 019`) and
  051's AC14 (`No branch-scoped state is persisted … a branch that created
  branch-scoped specs`) each put a negator ahead of a creation verb inside one
  clause while negating something that is not a path coming into existence.
  None costs a finding today — their paths are `ships-to-adopter`, resolve, or
  are not path candidates at all — but each is a criterion whose live paths now
  go unchecked if they later disappear. This is the same whole-criterion
  silence the phrase groups already accept, at a measured rate rather than an
  assumed one.
- **Splitting on the em dash as well was considered and rejected.** It would
  move 2 of those 4 (012's AC11 and 051's AC14) and leaves the other 2, whose
  negator and verb genuinely share a clause — telling those apart needs
  semantic judgment, which
  [§runtime-boundary](../../../framework/constitution.md#runtime-boundary)
  places at an extension point. For a change worth zero findings it also moves
  the splitter toward `;`-plus-em-dash, which is the clause rule 045 measured
  and rejected below. The residual rate is accepted and recorded instead.
- **This is not the clause-scoped design 045 measured and rejected.** 045
  §Resolved Questions rejected checking a removal-phrased criterion by
  *inverting* the assertion, and the implementation it built was also
  clause-scoped — split on `;` and the em dash, 20 findings of which roughly 8
  were false, and it broke the appended-annotation remedy by stopping an
  end-of-criterion qualifier from covering an earlier clause. What differs is
  what the clause scoping decides. There it decided **attribution** — which
  path a phrase is about — which is semantic judgment
  [§runtime-boundary](../../../framework/constitution.md#runtime-boundary)
  places at an extension point rather than in a deterministic family. Here it
  decides only whether one further exemption applies, and the exemption stays
  whole-criterion. The phrases are still matched across the whole
  criterion, so an appended annotation suppresses exactly what it suppressed
  before. That decision is unchanged by this scenario.
- **This is a precision fix, not a recall one.** No criterion that was silently
  exempted becomes a finding. The family's promotion criterion (045 §Resolved
  Questions) requires 5+ findings on two consecutive runs *and* every finding
  confirmed a true positive. The reporting repo met neither half — its single
  finding across seven specs was this false positive — which is that repo's
  measurement as reported, not one taken here.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
