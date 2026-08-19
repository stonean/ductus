---
spec: 022-deterministic-runtime
scenario: adopter-generator-promotion
reviewed-at: 2026-08-19T13:51:00Z
reviewed-against: f4c3bbd3058b7babf0c757c815341802cfe21c8e
diff-base: 9c06b2dfd5f16618c50fd3a0186caf534a517778
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Re-run against f4c3bbd, the commit that contains the work. The prior run recorded reviewed-against d35bbc2d — the pre-commit HEAD — because it reviewed the change while it was still uncommitted. That satisfied the pre-done gate but left `reviewed-against` naming a commit predating the work, so /ductus:audit Family 19 correctly reported the review stale once the work landed: data-model.md and the adopter-generator-promotion scenario are durable contracts that moved after the recorded sha. No finding changed; the reviewed tree is byte-identical to the one the prior run examined. Findings from that run stand: no MUST or SHOULD violations, one low-confidence QUAL-CLAIM-001 recorded below. The quality pass had found and the change fixed a real regression — the frontmatter-fence test written three different ways across the scanner and the two splices, none matching the shell's column-anchored form, letting an indented `---` inside a YAML block scalar end the frontmatter early so a `dependencies:` key below it was silently never rewritten; fixed by extracting one shared `is_frontmatter_fence` predicate with regression tests in both directions. The reuse pass consolidated two byte-identical golden helpers into tests/common, and the efficiency pass bounded the untracked-spec scan to the spec root and made the inline-code strip borrow on the common path.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-CLAIM-001 — an unterminated frontmatter block yields a clean result for a spec that was not processed

- **File**: `runtime/src/primitives/derive_references.rs:345-395`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both.
- **Finding**: splice_references only places the block at a `dependencies:` line or at the closing `---`. A spec whose frontmatter is unterminated AND carries no `dependencies:` key never reaches either insertion point, so the rewrite is skipped and the spec is silently absent from `updated` — indistinguishable in the result from a spec that genuinely needed no change. Narrow (it requires both conditions plus a cross-service link), and `validate-frontmatter` owns reporting malformed frontmatter, which is why this is recorded rather than fixed here: the honest fix is an `unparseable` field on both results, a deliberate schema addition rather than a review-time patch.
- **Auto-fixable**: no
- **Suggested fix**: Add an `unparseable: Vec<String>` field to both derive results, populated when the frontmatter fence is never closed, so an empty `updated` means examined-and-clean only when `unparseable` is also empty. Mirrors `check-artifacts`' `skipped` and `derive-boundary`'s `guidance`.

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
