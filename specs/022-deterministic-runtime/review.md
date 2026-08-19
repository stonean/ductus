---
spec: 022-deterministic-runtime
scenario: adopter-generator-promotion
reviewed-at: 2026-08-19T12:47:42Z
reviewed-against: d35bbc2d0a91b367be87cae68378cae8065bec67
diff-base: 9c06b2dfd5f16618c50fd3a0186caf534a517778
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviewed the adopter-generator-promotion cutover: the two new derivation primitives, the shared body scanner and corpus-enumeration helpers, the CLI blocking contract, both pre-commit hooks, Family 22, the CI workflows, and the migration. No MUST violations; the spec is not blocked. The quality pass found and the change fixed one real regression: the frontmatter-fence test had been written three different ways across the scanner and the two splices (a trimmed compare, a trim-start compare, a trim-end compare), none matching the shell's column-anchored `^---[[:space:]]*$`. The trimmed form treated an indented `---` inside a YAML block scalar as the closing fence, so a `dependencies:` key below it was silently never rewritten — the exact silent-staleness class this promotion exists to remove, reintroduced in new code. Fixed by extracting one `is_frontmatter_fence` predicate the three sites share, with regression tests for both the indented-fence and trailing-whitespace directions. The reuse pass consolidated two byte-identical golden helpers into `tests/common`, and the efficiency pass bounded the untracked-spec scan to the spec root (it was a full worktree status on every commit) and made the inline-code strip borrow on the common path. One SHOULD remains, recorded below at low confidence.

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
