---
spec: 026-framework-self-audit
reviewed-at: 2026-09-07T22:39:12Z
reviewed-against: 9b855180eee416407f576f2b25e9869aa70f59df
diff-base: d79a701ca837b53f7fd9f862bccfd5dc4b21f513
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Clean across all five passes: 0 MUST, 0 SHOULD, 0 low-confidence, nothing blocking. No `--since` override was needed or used — 026's diff base resolves to `d79a701`, the commit that reopened it on the scenario back-edge during this session's grooming, giving a 19-file scope whose modified-since half is exactly the work under review: `scripts/audit/review-freshness.sh`, the new scenario, `spec.md`, `tasks.md`, and the inbox. That is the difference between this spec and 020/022, whose bases predate the `0.28.0` cycle; the AGENTS.md override entry does not apply here.

The subject is task 41: Family 19 gaining a digest arm alongside its commit-diff arm, and a coverage line that reports the split. Reviewed against `quality-cross.md`, `configuration-cross.md`, the security and performance files, and AGENTS.md §Gotchas / §Boundaries / §Design Principles.

**Quality — two defects, both found here, both fixed in `9b85518` before this record was written, neither outstanding.** They are recorded rather than filed as findings because they map to no loaded rule, and as fixed because they are.

The first is the sharper one. `subject_digest` walks with `follow_links(false)` and tests the entry's own file type, so the runtime skips a symlinked scenario outright. The new digest arm globbed `scenarios/*.md` and called `is_file()`, which follows the link — so it digested a file `check-review-gate` never saw. AC25's whole claim is that the two enforcement points agree by construction on a record carrying a digest, and this broke that claim on the one input class where it is checkable. Proven rather than reasoned: a symlinked scenario under 022 produced a stale finding from this family and nothing from the gate; after the fix it produces neither, matching the runtime exactly.

The second is `QUAL-CLAIM-001` inside the line written to satisfy `QUAL-CLAIM-001`. `examined_proxy` was incremented before the `git diff` that is the proxy arm's entire comparison, so a git failure would have counted a spec as examined that was not examined at all — inflating the coverage claim in the field whose only purpose is to make that claim honest. It now counts as `unresolvable`.

**Security** — nothing. The family reads files under the configured spec root and shells out to `git` with argument vectors rather than a shell string; the sha256 addition introduces no new input path. The symlink fix above narrows rather than widens what is read.

**Reuse** — nothing, and the direction is right: `is_durable_contract` is shared between both arms rather than restated, and `changed_beyond_spelling` — the mechanical-sweep exemption — is applied on the digest arm too, so the rename rule has one implementation serving both. The digest itself is sha256 over file bytes, the same function the runtime uses, which is what makes the two comparable at all rather than merely similar.

**Efficiency** — nothing. The digest arm hashes a spec's durable contracts only for records that carry a digest, which is 1 of 54 today; the proxy arm's one-`git diff`-per-base caching that task 25 introduced is untouched. Family 19 remains well inside the release gate's budget.

**Simplicity** — the two arms are the one place this could be argued, and the scenario's Resolved Questions carry the argument: digest-only was rejected on measurement (it would examine 1 spec of 54), commit-diff-only on correctness (the false positive is reachable and guarded only by a human convention). The proxy arm ships with a documented exit condition — when the coverage line's proxy count reaches zero it is dead code with its own evidence for deletion — which is what keeps the second arm from becoming permanent.

Verified before recording: `shellcheck -S warning` clean, `scripts/audit/run-all.sh` exits 0, every lint script and `scripts/tests/*.sh` green, `npx markdownlint-cli2` clean. The family is proven to fail as well as to pass — a modified digest-covered contract, a deleted one, and a seeded symlink each produce the expected verdict.

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
