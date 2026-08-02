---
spec: 036-quality-cross-rules
reviewed-at: 2026-08-02T23:39:49Z
reviewed-against: afc811bfd091eb138a0b9c785836c9521c3f0d6a
diff-base: e1db1856eca9688dcf96c287d11cf71c6ed86cea
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 036-quality-cross-rules

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes, against task 7 — the addition of `QUAL-CLAIM-001` and its `CLAIM` category. Not blocking.

One finding surfaced during the run and was fixed before this record was written, and it is worth stating plainly because of what it was: the new rule's own Source paragraph asserted more than had been verified. It claimed "four generators print `No changes (all specs in sync)` … asserting a property of files they never enumerated". In fact only `gen-spec-deps.sh` prints that exact string, and only it and `gen-cross-service-refs.sh` enumerate through `list_specs()` and are therefore subject to the untracked-spec exclusion; `gen-help-tables.sh` and `gen-configure-mcp.sh` share the message shape but regenerate from fixed sources, and whether their zero-count can ever mean "did not examine" was never assessed. A rule forbidding unsubstantiated claims is the worst possible place to make one, so the Source now names one confirmed instance, one confirmed sibling, and two explicitly unassessed. The inbox entry that routes the follow-on fix carried the same overstatement and was corrected identically.

Rule integrity: `QUAL-CLAIM-001` is a level-3 heading containing only the ID, under a `## QUAL-CLAIM` category heading, carrying the three required fields (block-quoted Statement, `**Rationale:**`, `**Verification:**`) per the schema in `specs/008-security-rules/data-model.md`. `scripts/lint-rule-ids.sh` accepts it and the runtime's `check-rule-ids` harvester resolves it. RFC 2119 usage is internally consistent — SHOULD in both the Statement and the Verification, matching the advisory severity and the documented promotion criterion.

Non-overlap was checked rather than assumed, since a rule that duplicates a neighbour is worse than no rule: `QUAL-STUB-001` governs unimplemented paths returning success, `QUAL-GROUND-001` governs unverified assumptions inside logic, and `QUAL-CLAIM-001` governs a fully-implemented path whose output overstates what it verified. The distinction is stated in the Rationale so a reviewer choosing between them has the discriminator in hand. Reuse: the rule cites constitution §grounding rather than restating it, and the `data-model.md` entry is a summary in the same shape the two existing namespaces use — Family 6 (SSOT invariants) confirms no canonical text is duplicated.

Security, efficiency, and concurrency passes are not applicable to a prose rule artifact and returned nothing. Simplicity: the Verification paragraph is long but carries a three-part discriminator plus an exemption list, matching `QUAL-GROUND-001`'s established shape rather than exceeding it.

Deterministic corroboration at this HEAD: `scripts/lint-rule-ids.sh` exits 0, `bash scripts/audit/run-all.sh` exits 0, markdownlint clean over 365 files.

Scope note worth recording: `compute-review-scope` returned an **empty** `modified-since`, because the status flip and the rule addition landed in the same commit that became the diff-base — the window starts at the change. The "whichever set is larger" rule is what produced a usable scope here, falling through to the plan's four Affected Files. Had `plan-affected` also been empty, the empty scope would have rendered the "nothing to review yet, blocking: false" report — which is itself precisely the `QUAL-CLAIM-001` shape, in the review tooling. Recorded rather than acted on; the fix belongs to the scope primitive, not to this spec.

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

## Skipped passes

*None.*
