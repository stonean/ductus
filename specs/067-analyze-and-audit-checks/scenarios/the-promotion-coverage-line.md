---
section: "Audit checks"
---

# The-promotion-coverage-line

## Context

[§drift-prevention](../../../framework/constitution.md#drift-prevention)'s *Shared knowledge stays in git* routes a project learning by **population** — every project running this pipeline, one organization's projects, or this project alone — and the first of those destinations is the constitution. Nothing reports how much of the rules file has actually been routed.

The gap is not the routing rule, which is stated and current. It is that *not having promoted* is invisible. A rule-bearing entry that is universal by the reword test and still sitting in the project's own rules file looks exactly like one that was judged project-only and correctly stayed — both are bullets in the same section, and neither carries a marker, because a per-entry marker is the authored input [§design-principles](../../../framework/constitution.md#design-principles) rejects.

The coverage is derivable without any authored input. Given a rules file, the sections in it that carry rules, a classification table recording one verdict per entry, and the link form that marks an entry as pointing at the constitution rather than restating it, the unrouted set is exactly:

```text
unclassified = rule-bearing − (table-keyed ∪ pointer-citing)
```

Every term is a count over committed markdown, so the whole measurement is deterministic and needs nothing an author has to remember.

What makes it worth computing rather than re-deriving by hand is that a hand derivation is *not reproducible in practice*. The figure has been re-derived repeatedly and disagreed with itself every time, and the disagreements were never decay — they were differences of method. Two are on record: counting the citing side as **anchor-bearing lines in the whole file** rather than as **rule-bearing bullets in the named sections** gives two different honest answers to one question; and a matcher that does not strip a trailing parenthetical from a table key silently fails to match every key that carries one, reporting a coverage gap that is an artifact of the instrument. Both were caught only because someone re-derived a second time and noticed. A recorded figure decays and a hand method drifts; a primitive does neither.

## Behavior

**New primitive: `check-promotion-coverage`.** Project-scoped — it takes no feature argument, like `check-orphaned-references` and `check-unfolded-specs`. Arguments, all repo-relative:

- `rules-file` — the file whose entries are counted (required).
- `sections` — repeatable (one `--section` per heading on the CLI); the level-2 heading names inside `rules-file` whose top-level bullets are rule-bearing. Required, and there is no default: *which* sections carry rules is a property of the project's own file, and guessing it would make the denominator a claim the primitive is not entitled to.
- `table-file` — the file carrying the classification table (optional).
- `table-section` — the level-2 heading inside `table-file` under which the tables live (optional; every markdown table in the file is read when omitted).
- `pointer-link` — the substring that marks an entry as a pointer rather than a restatement (required), e.g. the constitution's repo-relative path with its anchor separator.

It returns the four terms and their difference, plus `unclassified-entries` (the lead phrase of each entry in neither set, so the result is a worklist rather than a number), `table-rows` and `unmatched-keys`.

**`unmatched-keys` is the load-bearing field, not a diagnostic.** A table key that matches no entry is how a *reworded* entry surfaces: the key degrades loudly to not matching at all, where a positional identifier would have degraded silently to matching the wrong entry. A key is matched against an entry's **lead phrase** — the bolded span opening the bullet — by prefix in either direction, after normalizing away inline markup and stripping one trailing parenthetical, since a table may qualify a key with a parenthetical that the entry itself does not carry.

**Family 38 renders it as a coverage line that never affects the exit code.** This is Family 19's shape, not a new one: that family already closes every run with a coverage line stating what it examined, above any findings, and that line is exit-code-neutral so the aggregator is unchanged. Promotion coverage is a **notice, never a gate** for the reason [§brownfield-inbox](../../../framework/constitution.md#brownfield-inbox) gives for the inbox: promotion has to stay free, and gating a release on how much is unrouted would make the honest choice between a growing backlog and a silent one push toward silence.

It renders in `/{project}:audit` rather than in `/{project}:review` or `/{project}:implement` because the subject is maintainer-owned. The classification table is an artifact of the spec that owns constitution content; a project that keeps no such table has nothing to report, and the audit is the surface adopters never invoke.

**Absence is a state, never a zero.** When `table-file` is omitted, the result's `table-state` is `no-table` rather than `read`, and the family prints a line saying coverage was not computed rather than a figure. A project that *has* a table and is fully routed prints the line with zero unclassified, because examined-and-empty and not-computed must not render alike.

## Edge Cases

- **No `table-file` supplied, or the path does not resolve.** Distinct states, distinctly reported: `no-table` for the first, and an error for the second. Silently treating an unreadable table as an absent one would report full coverage over a file that could not be read — `QUAL-CLAIM-001` on the measurement's own surface.
- **A named section is absent from `rules-file`.** Reported in `missing-sections` rather than contributing zero bullets. A denominator that quietly shrinks when a section is renamed is the failure the whole line exists to prevent, and it moves coverage in the flattering direction.
- **A section is named twice.** Counted once. The sections are a set.
- **An entry is both table-keyed and pointer-citing.** Counted once — the formula is a set union, and every promoted entry is deliberately in both: its row records the verdict and its rewritten line carries the pointer.
- **A table key matches more than one entry.** The first is taken and the key is not reported unmatched; ambiguity here is a property of two entries sharing a lead-phrase prefix, which the rules file's own convention already forbids.
- **The rules file has no bullets in any named section.** `rule-bearing` is zero, and the line still renders with its denominator visible. Zero over zero is reported as zero unclassified, which is true, and the stated denominator is what keeps it from reading as assurance.
- **Bullets inside a fenced code block or an HTML comment.** Not counted. The file's guidance blocks legitimately contain list-marker lines, and counting them once inflated an unrelated count by roughly thirty — the shared bullet grammar the inbox count already uses is reused here rather than a second parser being written.
- **A nested list item.** Not counted: only top-level bullets are entries, matching how the rules file's own sections are organized and how every hand derivation of this figure has counted.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Should the line gate?** Resolved: **no, and this is not a preference.** §brownfield-inbox's design rests on capture being free; the same argument holds one tier up for promotion. A gate would make the honest choice between an unrouted backlog and a silent one push toward silence, and the audit is a hard release gate, so a finding here would block a tag on a number that is supposed to inform rather than block. Family 19's coverage line is the precedent: it renders on every run and never touches the exit code.
- **Should it render to adopters, on the inbox-row surface?** Resolved: **no — `/{project}:audit` only.** The inbox row renders in `/{project}:status`, from `dashboard`, on every run because every project is scaffolded with an inbox, so silence there would be wrong. A classification table is opt-in and maintainer-owned, so an adopter who has none would need a configuration section invented for them in order to see anything but silence. `resolve-constitutions` sets the precedent for the opt-in case: report nothing, and a project without the feature reads exactly as before.
- **Why not a script?** Resolved: the check parses markdown structure, which [§runtime-boundary](../../../framework/constitution.md#runtime-boundary)'s third eligibility criterion puts in the runtime rather than in a shell entry point. The family resolves the binary and calls the primitive; it does not reimplement the count.
