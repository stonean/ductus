---
section: "Check Families"
---

# Family-34-step-reference-integrity

## Context

A command file under `framework/commands/` numbers its Instructions steps and then refers to them in prose — "settled in step 4", "the confirmation in step 3 carries", "refused by the primitive in step 5". The numbers are the only binding between the reference and its target, and nothing checks that a referenced number names a step that exists.

They drift the moment a step is inserted or removed, because removing one renumbers every step after it while the prose keeps the old numbers. Spec 054 removed two steps from `specify.md` and one from `consolidate.md` and left four stale references behind:

- `specify.md` named `create-feature` at step 6 in two places after it moved to step 5 — once in the branch-scoped sanitization note, once in the `ductus exec` reduction note.
- `consolidate.md`'s step 2 said "the confirmation in step 4 carries" after the confirmation moved to step 3.
- `consolidate.md`'s step 5 said "even though step 5 established the same fact", which had been step 6 pointing at step 5 and silently became a **self-reference** — the degenerate case, and the one least likely to be noticed by reading, since the sentence still parses.

**Three of those four are outside any exact check, and the family says so rather than implying otherwise.** Renumbering shifts a reference onto a *different existing* step: after the removal `specify.md` still had a step 6 (`writeSpecBody`) and `consolidate.md` still had a step 4 (`rewrite-spec-links`), so all three of those references resolve — they simply resolve to the wrong step. Only the self-reference is detectable without knowing what each step *does*. Deciding that "step 6" should now read "step 5" means matching prose against step content, which is a heuristic, and a family that fires falsely on a correct reference is worse than the silence it replaces — the standard [045](../../045-decision-state-drift-detection/spec.md) applied when it rejected the criterion-supersession check at 455 pairs.

All four were caught by a human re-reading the files during an unrelated review. Nothing in the test suite, the pre-commit hook, or the 33 existing `/audit` families would have reported any of them, and the runtime's own procedure parser is no help: it reads the numbered steps to build a `Procedure` and never looks at the prose between them.

This is the shape `/audit` already exists for — two things that must agree with nothing making them agree — and it is a **diligence dependency**, which [§design-principles](../../../framework/constitution.md#design-principles) rejects outright. The existing step-ordering tests (`runtime/tests/two_spec_commands.rs`, `runtime/tests/specify_command.rs`) assert what each step *dispatches*, which is why they caught the renumbering itself and said nothing about the prose describing it.

## Behavior

A new `/{project}:audit` family asserts that every `step N` reference in a command file resolves to a numbered step that file actually has. That is a narrower guarantee than "the references are correct", and the difference is stated wherever the family is described rather than left to be discovered: one of the four defects that motivated it is in reach, and three are not.

- **Subject:** each `framework/commands/*.md`, plus `framework/bootstrap/ductus.md` and `framework/bootstrap/govern.md`, which number their procedures the same way. The generated copies under the host command directory are **not** a second subject — auditing them would report the generator rather than a defect, exactly as Family 28 states for `audit.md`.
- **Step set:** derived from the file's own top-level numbered Instructions lines, never hardcoded — a hardcoded expectation would be a second copy of the fact under test. A file counts as carrying a *procedure* only when those numbers form a single ascending run starting at 1, with at least three of them; anything else is several lists sharing a heading and is named rather than examined (see Edge Cases).
- **Reference set:** derived from prose mentions of the form `step N` and `steps N–M` / `steps N and M`, case-insensitive.
- **Finding:** a reference whose number is outside the file's step set. Reported per file with the referencing line, the number named, and the range that does exist.

Two additional findings, because both were real in the 054 case and neither is caught by existence alone:

- **Self-reference.** A step whose own prose refers to its own number. It resolves, so an existence check passes it, and it is almost always the residue of a renumbering — the 054 case is exactly this.
- **Discontinuous numbering.** A file whose steps do not run `1..n` without gaps. This is what a partial removal leaves, and it makes every reference after the gap ambiguous rather than wrong.

The family reports; it never repairs. Which number a stale reference *should* name is a judgment — the renumbering may have been the error — and a rewrite that guessed wrong would be worse than a precise report, the same stance Family 26 takes on broken relative links.

## Edge Cases

- **An empty step set is a finding, not a pass.** A command file whose steps failed to parse yields zero steps, and every reference in it is then trivially out of range — or, if the reference extraction failed too, two empty sets agree and the check reports clean. Both are the false green `/audit` exists to prevent, so the family fails closed on an empty extraction from a file that has an Instructions section, matching Families 17, 18, 23, and 28.
- **A file with no numbered steps at all is out of subject, not a failure.** `log.md` and `help.md` carry no numbered procedure; they contribute no steps and no references, and are counted as examined rather than skipped.
- **`MD029` is disabled for these files, so numbering is not self-correcting.** `.markdownlint-cli2.jsonc` turns off the ordered-list continuity check because `<!-- audit:ignore-promotion -->` markers between steps structurally break it. That is precisely why the discontinuity check belongs here — the one tool that would have noticed was deliberately switched off, for an unrelated reason.
- **Prose naming a step in another file.** A command that refers to another command's step ("groom's step 3") must not be resolved against the citing file's own step set. The extraction is anchored to a bare `step N` with no intervening file or command name; a qualified reference is not a subject.
- **The markdown-only reference sections renumber independently, and are counted rather than resolved.** Several command files carry a `## Markdown-only reference` whose sub-procedures restart at 1. Those are separate lists, so the subject is the Instructions section alone and mentions elsewhere land in `references-out-of-subject` — 96 of them across the corpus — so a clean result is never read as *every step reference in the file resolves*.
- **A file whose Instructions holds several numbered lists is not a subject.** `amend.md` restarts at 1 under each `###` subsection and `status.md` uses three separate one-item lists; both are legitimate authoring, and MD029 is disabled here so nothing pushes them to be otherwise. Merging those into one step set and resolving against it would invent findings — the one failure this family must not have — so they are named in `not-a-procedure` instead. Measured: 15 of 19 examined files carry one procedure; `amend.md`, `audit.md`, `help.md`, and `status.md` do not.
- **Retired numbers are not a case here, unlike Family 28.** Audit family numbers are permanent identifiers with deliberate gaps; command steps are positional and must be contiguous, so a gap is a defect rather than a stable state.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
