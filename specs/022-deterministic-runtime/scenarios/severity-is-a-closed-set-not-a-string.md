---
section: "Follow-on scenarios"
---

# Severity-is-a-closed-set-not-a-string

## Context

Every finding the runtime produces carries `severity` as a bare `String`: `ReviewFinding` (`schema/primitives.rs:417`), `FrontmatterFinding` (`:1313`) and `ArtifactFinding` (`:3669`). The tiers themselves are spelled as inline literals — 16 occurrences in `check_artifacts.rs`, 15 in `validate_frontmatter.rs`, 3 in `schema/extensions.rs`, 2 in `schema/primitives.rs` — with no constant, no enum, and no validation on deserialize. The doc comments name the legal values; nothing enforces them.

**The field holds three different closed sets under one type**, not the two this scenario was first written against — a count corrected while implementing it, by reading every `severity` field rather than the two the grooming pass had measured. `ReviewFinding` uses `must` / `should`. `FrontmatterFinding` and `ArtifactFinding` use `hard-fail` / `blocking` / `advisory` / `informational`. And `AssessSpecQualityRule` / `AssessSpecQualityFinding` carry a *rule* tier — `must` / `should` / `info`, plus the empty string that `severity_from_step_prose` returns when a step's prose names no tier. All five share a name and a `String`, so nothing prevents a value from one vocabulary being emitted into another.

**On the review side the failure direction is permissive, which is what makes this more than hygiene.** `write_review.rs:118-128` buckets findings with the severity test *last* in an if/else chain whose `else` is the catch-all:

- a finding is `waived` if a waiver matches, else `low` if confidence is low, else `must` if `severity` equals `"must"` case-insensitively, **else `should`**.

`blocking` is then `must_n > 0` (`:130`). So any severity string that is not exactly `"must"` — a typo, a trailing space, a value borrowed from the analyze vocabulary — is silently filed as a SHOULD, `blocking` is written `false`, and the spec passes `check-review-gate`'s MUST check. `finding_rank` (`:402`) derives its ordering from the identical test, so a misspelled MUST also *loses* cross-pass dedup to a correctly-spelled SHOULD, and the surviving copy is the weaker one.

The reachable path is not a source typo. `findings` is `write-review`'s argument, supplied by the host from the `performReview` extension point — the severity string is written freehand by an LLM into JSON on every review run. An unvalidated permissive default sits directly on the path that gates `done`.

**The analyze side is the same root cause with a milder consequence**, and the asymmetry is worth recording rather than flattening. Nothing in `runtime/src` branches on `FrontmatterFinding.severity` or `ArtifactFinding.severity` — both results are serialized straight out to the host, which renders per-finding tiers by the prose in `framework/commands/analyze.md` step 2. An unrecognized tier there is mis-rendered, not silently downgraded past a gate. That absence of a branch is exactly why 067's `frontmatter-severity-tiers-on-both-sides` could correct eight wrong tiers with a string substitution — and equally why those eight wrong tiers survived months undetected.

## Behavior

**A severity tier is a closed set, bound so an unrecognized value cannot be constructed, deserialized, or emitted.** The three vocabularies stay distinct rather than being merged into one enum — they are different tier systems belonging to different commands, and collapsing them would make the cross-vocabulary error representable in the type that exists to forbid it.

- The review tiers (`must`, `should`), the analyze tiers (`hard-fail`, `blocking`, `advisory`, `informational`) and the rule tiers (`must`, `should`, `info`, `""`) each become their own type, serialized to exactly the strings they carry today, so `review.md` and `analysis.md` are byte-identical across the change and no consumer's parsing moves. The rule set's empty string becomes a named `Unspecified` variant rather than an absent value, so it cannot be confused with a tier that failed to parse.
- Deserialization of an out-of-set value is an **error**, not a default. This is the load-bearing half: the current behavior's defect is not that a bad value exists but that it resolves to the permissive tier. A rejected payload names the offending value and the field; it never silently degrades to `should` or to `advisory`.
- `write_review`'s bucketing and `finding_rank` branch on the bound value rather than on a string comparison, so the `else`-is-`should` catch-all disappears by construction rather than by a guard someone has to remember.
- The inline literals across `check_artifacts.rs`, `validate_frontmatter.rs` and `schema/extensions.rs` are replaced by the bound values at every site.
- 022's `data-model.md` result-shape registry records the three closed sets and points at the constitution's [§text-first-artifacts](../../../framework/constitution.md#text-first-artifacts) Validation Severity subsection for the analyze tiers' meaning rather than restating the assignment.

The extension-point contract is where the guarantee has to hold, because that is the reachable path: a `performReview` response carrying an unrecognized severity is rejected with the value named, and the run halts rather than recording a review whose counts silently understate what the passes found.

## Edge Cases

- **A rejected extension response is worse than a demoted finding only if it is silent.** Halting on an unrecognized severity turns a silently-wrong review into a loudly-failed one, which is the intended trade — but the error must let an operator tell a model typo from a contract change. **Delivered as:** `schema-mismatch in`performReview`: unrecognized review severity "mandatory" — expected one of: must, should`. A `Deserialize` impl has no access to the key it was reached through, so the message names the *vocabulary* rather than the field; the interpreter supplies the extension point, which is what locates it. Between them the operator gets where, which set, what was sent, and what was legal.
- **Case-insensitivity is preserved — decided, not inherited.** Both replaced comparisons used `eq_ignore_ascii_case`, so `"MUST"` was legal before and remains legal; deserialization accepts case variants and serialization always emits the canonical lower-case form. Narrowing to an exact match was rejected because it would turn a working review run into a halted one over a case difference carrying no ambiguity — and `MUST` is the spelling the rule files themselves use, so it is the likely thing for an LLM to send. Strictness targets *unrecognized* values, not capitalization.
- **Waived and low-confidence bucketing sit ahead of the severity test and are unaffected.** A waiver match and a low-confidence tier both short-circuit before severity is read, so this change cannot move a finding out of those buckets.
- **The analyze tiers already have a fifth informal member.** `check_artifacts` emits `blocking` and `advisory` only; `hard-fail` and `informational` arrive from other producers and from the host's rendering. The closed set is the constitution's four, so binding it will surface any producer that was relying on a value outside them.
- **No shipped record changes shape, and the corpus sweep came back empty for a structural reason.** Serialized strings are unchanged, so records stay byte-identical. The sweep found **no** `review.md` or `analysis.md` carrying a `severity` field at all: findings are rendered into the report *body* as prose, while the frontmatter records only counts. So there was no record to migrate and none that could fail the new deserialization — the exposure is entirely on the live extension-point path, not in the stored corpus.
- **This is a `runtime/` change and therefore a release.** Per AGENTS.md §Workflow the tag is cut from completed work: implement, review, analyze, 022 back to `done`, and only then bump the three version sites and push `ductus-v<version>`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
