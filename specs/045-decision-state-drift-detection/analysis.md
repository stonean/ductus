---
spec: 045-decision-state-drift-detection
last-run: 2026-11-14T17:37:37Z
analyzed-against: 509948e848c3a71e3b9e24b98c259a2081c1c7aa
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 17
analyzed-digest:
  data-model.md: a5623b241c91e487a621f633851dd80ca8bf5545791f3bdc2b823c9b647b3fbe
  plan.md: 620b33f10fa8d055c2e3b007a6de59f3dc7140979a75c7829c976686a5426d08
  review.md: 81930205c4a39cb4d673aeeb283d6b60b62e42ca1adf83f88010240f8ecba2a6
  spec.md: 82cad93cbea7f84885222be8beb9a2f8ac03a59ced2de7ff4757f99428b8f854
  tasks.md: c3ad2f67a9faa7e23625aaf5c10197f857cc0b3a026c2439c7b4219441a8336f
unexamined-by-reason:
  not-a-live-claim: 8
  root-absent: 4
  ships-to-adopter: 5
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Release path"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#What the families read, and how much they return"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Acceptance-criterion path grammar"}
  BE-INPUT-004: {verdict: finding, tier: must, at: "plan.md#What the families read, and how much they return", message: "The path-existence probes screen a criterion path only lexically, so exists() still follows a symbolic link out of the project; canonical containment is deferred to 067 task 33 (owner, 2026-11-14)."}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#What the families read, and how much they return"}
  BE-REPR-002: {verdict: addressed, cite: "data-model.md#`SkippedTarget`"}
  BE-SCHEMA-002: {verdict: addressed, cite: "data-model.md#`SkippedTarget`"}
blocking: false
dispositions:
  fixed: 1
  deferred: 1
  waived: 4
  undispositioned: 0
decisions:
  - key: "body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: \"For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope).\""
    outcome: waived
    reason: "correct as written: `../spec.md` is the worked example of a link resolved from the hypothetical `scenarios/foo.md`, not a path the plan claims exists; the family resolves it from plan.md's directory instead"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: "body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: \"| `framework/workflows/registry.json` | yes | no → finding |\""
    outcome: waived
    reason: "correct as written: the row is the worked example's expected outcome, the path deleted by 531e3ea and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: "body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: \"| `scripts/audit/registry-equivalence.sh` | yes | no → finding |\""
    outcome: waived
    reason: "correct as written: the row is the worked example's expected outcome, the script deleted by 3ff65445 and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: "rule-assessment — BE-INPUT-004: The path-existence probes screen a criterion path only lexically, so exists() still follows a symbolic link out of the project; canonical containment is deferred to 067 task 33 (owner, 2026-11-14)."
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/tasks.md
    task: "33"
    reason: the owner deferred it (2026-11-14) to 067, which owns the analyze checks; the sibling-link family already refuses an outward link, and task 33 makes both path probes refuse one before probing
    decided-at: 2026-11-14T17:37:37Z
    decided-by: andy@stone.dev
  - key: "grounding — plan.md Trade-offs, Code-span-only path extraction: \"the originating case and every path in this repo's acceptance criteria is backticked\" and \"a `/`-bearing token in this repo's acceptance criteria is far more often a slash command ... than a path\" describe the corpus with no command, count or citation"
    outcome: waived
    reason: the trade-off records the corpus the choice was weighed against when it was made, names its cost — a bare unbackticked path is missed — and states that widening is additive; it governs no behavior
    decided-at: 2026-11-14T17:37:37Z
    decided-by: rule
---

# Analysis — 045-decision-state-drift-detection

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 17 unexamined target(s). Dispositions: 1 fixed, 1 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — plan.md Trade-offs, Code-span-only path extraction: "the originating case and every path in this repo's acceptance criteria is backticked" and "a `/`-bearing token in this repo's acceptance criteria is far more often a slash command ... than a path" describe the corpus with no command, count or citation — `specs/045-decision-state-drift-detection/plan.md` — **waived**: the trade-off records the corpus the choice was weighed against when it was made, names its cost — a bare unbackticked path is missed — and states that widening is additive; it governs no behavior (decided by the rule)
- body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: "For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope)." — `specs/045-decision-state-drift-detection/plan.md` — **waived**: correct as written: `../spec.md` is the worked example of a link resolved from the hypothetical `scenarios/foo.md`, not a path the plan claims exists; the family resolves it from plan.md's directory instead
- body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: "| `framework/workflows/registry.json` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the path deleted by 531e3ea and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach
- body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: "| `scripts/audit/registry-equivalence.sh` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the script deleted by 3ff65445 and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach

## Deferred to other specs

- rule-assessment — BE-INPUT-004: The path-existence probes screen a criterion path only lexically, so exists() still follows a symbolic link out of the project; canonical containment is deferred to 067 task 33 (owner, 2026-11-14). — `specs/045-decision-state-drift-detection/plan.md` — **deferred** to `specs/067-analyze-and-audit-checks/tasks.md` task 33

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 8
- root-absent: 4
- ships-to-adopter: 5

## Fixed in this run

- task-consistency — tasks.md line 9 says tasks 3-10 are authored as scenarios under 022-deterministic-runtime 'per the plan's Implementation ownership split', but the plan's Implementation ownership decision places the four scenarios under 067-analyze-and-audit-checks — `specs/045-decision-state-drift-detection/tasks.md` — **fixed** (decided by the rule)
