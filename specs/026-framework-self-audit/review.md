---
spec: 026-framework-self-audit
reviewed-at: 2026-08-28T02:34:11Z
reviewed-against: e21c45679b608ed40cb8925ce7ea7109a5dc028d
diff-base: c0eb574f77d702bb047983cfcac88289362aefbf
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Family 31 (review block agreement) reviewed across all five passes over the 22-file scope resolved from `c0eb574..HEAD`. 0 MUST, 1 SHOULD, 0 low-confidence. The family works and is proven red in six modes, green at HEAD, and found a real divergence on its first run (042 carried `low-confidence: 1` against its report's `0`). The SHOULD is about where it lives, not whether it works: the check reimplements frontmatter parsing the runtime already owns, when it meets every runtime-eligibility criterion and Family 30 already ships the correct shape. That default has now been codified in §runtime-boundary, `AGENTS.md`, and this directory's README, so the finding is against a rule that exists rather than a preference. Not blocking; the conversion is follow-on work.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: REUSE-001 — the check reimplements frontmatter parsing the runtime already owns

- **File**: `scripts/audit/review-block-agreement.sh:121-200`
- **Rule**: Identify logic that duplicates existing utilities or that should be extracted into shared code. Severity is SHOULD unless the duplication contradicts an explicit MUST in AGENTS.md Boundaries.
- **Finding**: Family 31 is implemented as a bash script with an embedded python3 heredoc that hand-rolls `frontmatter()`, `scalar()`, `review_block()`, and two waiver extractors. The runtime already parses both sides of exactly this data — `read-spec` returns the `review:` block as structured data and `write-review` writes both files — so this is a fourth hand-rolled frontmatter parser in a repo whose runtime parses frontmatter for a living. The capability meets all three of §runtime-boundary's eligibility criteria (deterministic; already mechanical; specifiable as prose — the scenario is that specification), and Family 30's `command-flag-hint-parity.sh` already demonstrates the correct shape: the check is the `check-command-flags` primitive and the script is a thin entry point that resolves the binary and calls it. The cost is not theoretical: the hand-rolled `scalar()` shipped a first-draft bug using `\s*` after the key, and because `\s` matches a newline it walked past an empty value onto the next line and returned that line's content, producing a confidently-wrong finding against 031 whose waiver is recorded correctly. The runtime's tested scanners do not have that bug.
- **Auto-fixable**: no
- **Suggested fix**: Extract the comparison into a `check-review-agreement` primitive under `runtime/src/primitives/`, registered at the six existing sites (`primitives/mod.rs`, `main.rs`, `interpreter/mod.rs`, `mcp/server.rs`, `schema/registry.rs`, `schema/primitives.rs`), reusing the runtime's frontmatter scanners rather than a fifth regex copy. Reduce `scripts/audit/review-block-agreement.sh` to the Family 30 entry-point shape: resolve `.ductus/bin/ductus` (falling back to `runtime/target/release/ductus`), call the primitive, render its records through `emit`, and treat an unreachable runtime as a finding rather than a silent pass. The shell entry point stays either way — `run-all.sh` registers scripts and Family 28 asserts that registry — so this moves the logic, not the registration.
- **Status**: **resolved 2026-08-28** in `ec0526b` (026 task 35). The check is now the `check-review-agreement` primitive, which deserializes both frontmatter blocks with the runtime's own YAML reader; `scripts/audit/review-block-agreement.sh` is a thin entry point carrying no parsing of its own, matching Family 30's shape. The motivating defect is now impossible rather than fixed — serde cannot express the `\s*` newline-swallow, and the `an_empty_value_never_reads_the_next_line` unit test pins it. The same bug was fixed in `review-freshness.sh`, which carried the helper by copy. The default this finding argued from was codified in `2b4e77e` (§runtime-boundary, `AGENTS.md`, `scripts/audit/README.md`), so the next family author meets the rule before writing the script rather than after.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] convention: `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md` contradicts itself in prose — its Summary says "1 low-confidence note retained (probe-to-use race …)" while its own frontmatter records `low-confidence: 0` and its `## Low-confidence findings` section reads "*None remaining.* The finding below is **resolved**" (resolved 2026-08-02 under 022's config-resolution-single-probe). The Summary is the pre-resolution narrative left unswept when the finding closed. Family 31 (review block agreement) does not catch it by design — its subject is frontmatter only, since a report's narrative legitimately discusses counts and a prose check would report every review that describes its own findings. Candidate: no new family; the sweep obligation belongs to whoever resolves a finding in a report. Surfaced 2026-08-28 while implementing Family 31, which found the frontmatter half of the same drift (`spec.md` carried `low-confidence: 1`, now reconciled to the report's `0`).

## Observations

- bug: `scalar()` in `scripts/audit/review-freshness.sh` (Family 19) uses `\s*` after the key name, and `\s` matches a newline — so on a bare empty value (`reviewed-against:` with nothing after the colon) the greedy run walks onto the next line and returns *that* line's content instead of None. Verified directly: the helper returns `'must-violations: 7'` for a frontmatter block whose `reviewed-against:` is empty. The effect is a wrong finding rather than a missed one — Family 19 would report an unresolvable-sha finding instead of deferring the null to `check-review-gate` as its scenario specifies — so it fails in the safe direction but with a misleading message. Latent today: no spec currently carries a bare empty `reviewed-against` (the template writes `null`, which parses correctly). Family 31 carried the same helper by copy and fixed it there with `[ \t]*`; the two copies have now diverged in correctness, which is the duplication cost REUSE-001 names in this review. Fix with `[ \t]*` in Family 19, or fold both into the runtime primitive that review proposes. — `scripts/audit/review-freshness.sh` **Resolved 2026-08-28 in `ec0526b`** (026 task 35): Family 19's `scalar()` now uses `[ \t]*`, verified to return `None` for a bare empty value, and Family 31's copy is gone entirely — its check is the `check-review-agreement` primitive, which deserializes with a YAML reader that cannot express the bug. Removed from the inbox, since it was captured and closed inside the same window.

## Skipped passes

*None.*
