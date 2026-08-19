---
spec: 040-configurable-specs-dir
scenario: spec-root-rule-stated-once
reviewed-at: 2026-08-19T15:25:17Z
reviewed-against: 830e42a0d06396e62f2346c694d5ebd0c075742d
diff-base: c8ae24d8fd9c91686443e3c367541a94e1ba70a4
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 040-configurable-specs-dir

## Summary

Clean at `830e42a` — 0 MUST, 0 SHOULD, 0 low-confidence, across all five passes.

Scope is task 12's de-duplication: `framework/constitution.md`, the six command sources (`specify`, `log`, `groom`, `review`, `amend`, `implement`), their six generated copies under `.claude/commands/ductus/`, and `tasks.md`. Prose and generated artifacts only — no runtime, script, or workflow change, so no version bump and no `ductus-v*` tag.

**The change is a net deletion**: 15 insertions, 36 deletions across 14 files. Seven statements of the spec-root substitution rule become one. `QUAL-CLAIM-001` is the rule this scenario is really about at the artifact level — the sibling sweep's fix worked but left a claim the corpus could not keep true, since nothing made the seven copies converge and the next command file added would silently lack one.

**The canonical statement now carries the instruction.** The prior text described runtime behavior ("wherever a command or the runtime constructs a path under it, it resolves `[paths] specs-root`"), which is a fact, not a directive to the host — precisely the gap the six blockquotes grew to fill. It now states the substitution imperatively, scopes it to the markdown-only path, says explicitly that it applies to commands added later, and records that commands reference rather than restate it. That last clause is load-bearing in the new direction: with the notes gone, the failure mode inverts from "a new command forgets to paste the note" to "a new command's author does not know the rule reaches them," and one sentence closes both.

**Verified by sweep, not assumed.** Zero occurrences of the restatement phrasing survive anywhere under `framework/` or `.claude/`; the only remaining match for "Spec-root resolution" is the new canonical-sources table row, which is the pointer rather than a copy. The two exemptions the scenario names are intact and were checked individually: `framework/bootstrap/ductus.md`'s note (load-bearing — `/ductus` scaffolds the constitution, so it runs where none exists to reference), and the sites the sibling scenario rewrote to name the resolved root as a literal *argument* rather than guidance (`log.md`'s `lint-markdown` target, `groom.md`'s inbox read), which are untouched. Deleting duplicated guidance is not deleting a resolved path, and the diff bears that out.

**Two shapes of removal, handled separately.** In `specify`, `review`, and `implement` the note was a standalone blockquote. In `log`, `groom`, and `amend` it was a second paragraph *inside* the agent-runtimes blockquote, so the bare `>` continuation line had to go with it or the surviving blockquote would have carried a trailing empty quote line. Both cases render clean under `markdownlint-cli2`.

AC11's substantive holding is preserved exactly: `specs/` stays literal and no `{specs-root}` placeholders enter human-read documents. What drops from seven to one is the number of places the *caveat* is repeated, not the number of places the default appears — which is what the scenario committed to and what the diff does.

Checks: `markdownlint-cli2` (440 files, 0 issues), `lint-procedure-parseability`, `lint-tool-coverage`, `lint-frontmatter`, `lint-rule-ids`, `derive-dependencies` / `derive-references` (no drift, 51 examined), `gen-help-tables --dry-run`, and `scripts/audit/run-all.sh` — all clean, the audit re-run against the committed tree.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
