---
spec: 026-framework-self-audit
reviewed-at: 2026-08-03T14:47:58Z
reviewed-against: 2f226b5805d32ec2c2db23b94438519af7255dee
diff-base: d99df57ecd05936029a1d29d08706ff48904ae01
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Covers the two audit families added since the prior pass — Family 18 (`marker-list-parity`) and Family 19 (`review-freshness`) — plus their registry rows and the corrected family count. **0 MUST, 0 SHOULD — not blocking.** This review was demanded by 022's new staleness gate, which flagged 026 as stale on `framework/commands/audit.md`; both families had landed under 026 with its review still pointing at `d99df57e`. Rule files loaded: the backend + cross set. Nothing in the diff is application code — two bash/python check scripts, a registry row each, and markdown — so security, api, concurrency, and performance rules find no surface; the scripts spawn `git` and `python3` with fixed argument vectors (no shell interpolation of derived values), read only, and write nothing. `QUAL-GROUND-001` is the pass with real content, and both families are on the right side of it: Family 18 exists *because* of it, binding the non-assertion marker list to its canonical source in 045's data-model instead of letting four restatements drift; Family 19 derives its spec root from the same config resolution the runtime uses rather than assuming `specs/`. Failure direction is correct and, importantly, **different between them** — Family 18 treats an empty derivation as a finding (an empty marker set would mean checking nothing), while Family 19's runtime counterpart fails open (an empty scope means blocking nothing). That asymmetry is documented in both rather than left for a reader to infer. `QUAL-CLAIM-001` holds: Family 19 reports an unresolvable `reviewed-against` as its own finding instead of skipping quietly, so it cannot report clean on a spec it could not examine. Family 19's scoping is evidence-led rather than asserted — two wider rules measured at 42/48 and 31/48 against this corpus and rejected before the 10/48 rule shipped, with the rejected numbers recorded in the script header so the next maintainer does not re-derive them. The decision to leave Family 19 out of `run-all.sh` is recorded in the scenario, the script header, and `scripts/audit/README.md`, with the one-line wiring step and its precondition named. Verified: shellcheck clean on both new scripts and on `run-all.sh`, the 18 wired families exit 0, markdownlint clean across 390 files, `check-artifacts` clean on 026, and Family 18 exercised against five injected drift modes plus a restored baseline.

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
