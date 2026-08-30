---
spec: 051-branch-scoped-spec-numbering
reviewed-at: 2026-08-30T01:19:43Z
reviewed-against: d9c87b0b248abbb70d711a095704a23bb8b5d2ea
diff-base: eb52e24a8bf9ce362430abe583d2ef63dd9bafc9
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 051-branch-scoped-spec-numbering

## Summary

0 MUST, 0 SHOULD, 0 low-confidence; not blocking. One captured issue outstanding.

**The quality pass found a real defect, in the work this review was called to check.** Task 22's done-when — no primitive that rewrites an existing text file changes its line endings — was not met when the task was marked complete. Two writers were missed: `prune-tasks --reset`, which lifts the `#` heading from the file it is resetting and hardcodes `\n` for the rest, and `append-inbox`, which kept the existing content's endings and appended its bullet with `\n`, producing the mixed file the scenario names as the sharper failure. Both fixed in `d9c87b0` before this report was written, so the counts state what is outstanding rather than what was found.

**Why they were missed is the more useful finding.** The first sweep covered seven writers because seven is what reading the code turned up, and its test enumerated those seven. A test that enumerates what someone thought of verifies the thinking, not the property — the same shape as the original defect one level up. `prune-tasks`'s parameter was literally named `_content` because it had been deliberately ignored, which is what let a careful read walk past it.

**So the sweep now enumerates the code.** `tests/line_ending_discipline.rs` requires every primitive calling the text writer to show evidence it accounts for endings — `split_inclusive('\n')`, so terminators never leave the data (the stronger pattern, already used by `mark-task`, `mark-criterion`, `set-status` and `label-criteria`), or detect-and-restore through the shared helpers — or carry an exemption with a stated reason. A companion test rejects any exemption naming a file that no longer writes text, and it earned its keep on the first run by rejecting an entry added for a primitive that only ever wrote bytes.

**The lint's limit is recorded rather than implied.** Its evidence check is per file, not per function, so a file handling endings in one writer and not another passes — which is precisely how both misses survived, each file already carrying an evidence token elsewhere. Reverting both fixes leaves the lint green. Per-path coverage is `crlf_preservation.rs`'s job; the two tests are complements and neither is sufficient. Closing the gap properly means per-function analysis, which is a parser rather than a lint. Saying so is QUAL-CLAIM-001 applied to the test itself: a green run must not imply a check that did not run.

**Task 23's work holds.** `rewrite-spec-links` refuses an absent destination before writing anything, with both arguments through `validate_no_traversal` (BE-INPUT-004). The check sits in the primitive rather than the command deliberately: the fold happens to establish the same fact one step earlier through `invalidate-review`, but that is a side effect of a step added for an unrelated reason, and a property held by a neighbour's side effect is one nobody knows they are maintaining. Three existing tests failed on the new check and were corrected by seeding real targets rather than exempted — their fixtures had been re-pointing links at a spec that did not exist.

Security, efficiency and simplicity found nothing against the loaded rules. `line_ending_of` cannot underflow, since every `\r\n` contains an `\n` and the bare-LF count is their difference; `with_line_ending` is idempotent and leaves a mid-line `\r` alone. Every behavioral case added in this window was confirmed to fail against the pre-fix tree — a CRLF test written on an all-LF repository otherwise proves nothing.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: the three-digit spec-number rule survives in five shell copies the runtime's single predicate does not reach, so a spec numbered past 999 is silently skipped by them — `.githooks/pre-commit:86` and the shipped `framework/bootstrap/hooks/ductus-pre-commit:116` (label-criteria never runs on such a spec), plus `scripts/lint-frontmatter.sh`, `scripts/audit/sibling-coupling.sh` and `scripts/audit/introducing-drift.sh`, which glob the same shape and exit 0 while never having seen the directory. Still outstanding; routing already decided (a scenario, not a new spec).

## Observations

*None.*

## Skipped passes

*None.*
