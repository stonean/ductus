---
section: "Check families"
---

# Family-23-sweep-target-manifest-parity

## Context

[§drift-prevention](../../../framework/constitution.md#drift-prevention) tells a
sweep to **keep its own target list current**, because a list naming a
relocated directory sends the grep somewhere clean and the sweep silently
misses the files that moved. That rule is the residue of a real failure: 042
moved the generators from `scripts/` to `.ductus/scripts/`, 049's rename sweep
grepped the stale list, and `config_path_of` in the shipped
`.ductus/scripts/lib/specs-root.sh` kept resolving `.govern/config.toml` then
the legacy root with no `.ductus/` tier. A converged adopter therefore resolved
to a path that does not exist, fell through to the default spec root, and
exited 0.

**The list the rule protects no longer exists.** [050](../../050-constitution/spec.md)
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
- [§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle) case (a)
  defines a mechanical edit as a substitution applied uniformly "per the
  `AGENTS.md` rename rule's scope" — which is now a pointer back to the
  constitution's categorical list. The mechanical-edit rule's scope is
  circular, and that rule is what decides whether a `done` spec reopens.
- The only surviving copies of the enumeration sit in
  [023](../../023-govern-refinement/spec.md)'s `living-specs` scenario and
  [043](../../043-workflows-sunset/spec.md)'s plan, both citing "AGENTS.md line
  42" — itself stale.

So the gap is not that the list drifts unchecked; it is that there is nothing
left to check. A rule whose subject has been deleted reads as satisfied.

## Behavior

Two halves, landing together. Shipping the family without its input, or the
input without the family, reproduces the failure mode in a new place.

**Restore the list.** `AGENTS.md` carries the literal enumeration again — this
is contributor-side, repo-specific detail, which is what that file is for —
delimited by `<!-- audit:sweep-targets:begin -->` / `<!-- audit:sweep-targets:end -->`
so the family has something stable to extract. The constitution stays
categorical: literal repo paths would be meaningless in a document adopters
install, and 050's decision there was right.

§spec-lifecycle case (a) is repaired the other way — by removing the deferral
rather than making it resolve. It pointed at "the `AGENTS.md` rename rule's
scope", and `framework/templates/project/agents.md` ships adopters an
`AGENTS.md` with no rename rule in it, so the pointer dangled for every adopter
however this repository's own copy read. It now names the constitution's own
§drift-prevention enumeration, which is already there, already categorical, and
already the right level for a reader whose layout is not ours. The two
`AGENTS.md`-internal pointers ("the live-artifact paths above", "the artifact
set above") resolve once the list is back in the entry that carries them.

**Then check it.** Family 23 asserts that every **source** path in the
`framework/bootstrap/ductus.md` **Shared Files** manifest is covered by some
entry in that list. Source paths, not destinations: the list governs where a
sweep greps in *this* repository, while destinations describe an adopter's
tree.

The check was measured before being proposed
([§recommendations](../../../framework/constitution.md#recommendations)):

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
  [§design-principles](../../../framework/constitution.md#design-principles)'s
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
- **The list stays in prose, delimited rather than relocated.** It lives in
  `AGENTS.md` because that is where a contributor reads it; moving it to a
  machine-owned file would make extraction trivial and the rule invisible to
  the person who has to follow it. HTML comment markers around the inline list
  buy reliable extraction without either cost, and they are already house idiom
  (`<!-- §anchor -->`, `<!-- audit:ignore-promotion -->`). The markers are load
  bearing, so the entry says so.
- **A marked region on a single line breaks the obvious extraction.** The whole
  entry is one long bullet, so both markers sit on one line — and a `sed` range
  whose start and end regexes match the *same* line does not close there, it
  resumes hunting on the next line and runs to EOF. The failure is silent and
  *generous*: it yields a superset of the list, so every manifest path looks
  covered and the family reports clean for the wrong reason. It surfaced during
  implementation only because the entry count was printed — 280 where a dozen
  was expected — which is the reported-counts requirement above paying for
  itself before the family had even landed. A lone begin marker reaches the
  same over-collection by a different route, so the family checks that both
  markers are present before extracting anything.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
