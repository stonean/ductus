---
spec: 017-derive-dont-ask
reviewed-at: 2026-09-13T00:17:28Z
reviewed-against: 19745b4abfdc123f838523cde2f4b1affb9e68bf
diff-base: 3db3d0e9238f824995b87e3757d97363a76d2029
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

Full five-pass review against the 11 loaded rule files. Scope read in full: the two binding cross-cutting rule files; every executable artifact in scope (.githooks/pre-commit, scripts/install-hooks.sh, scripts/gen-help-tables.sh, framework/bootstrap/hooks/pre-commit, .github/workflows/generators.yml, framework/templates/ci/adopter-generators.yml); both permission sets; framework/bootstrap/ductus.md; framework/constitution.md and AGENTS.md; the command sources log, status, groom, plan, amend, implement, analyze, clarify, specify; all five spec templates; README.md; and this spec's own spec.md, plan.md and data-model.md. Twelve defects were found and all twelve are fixed — none stands. QUAL-CLAIM-001 x3: the shipped adopter CI gate hardcoded `find specs`, so on a configured [paths] specs-root it enumerated nothing and exited 0 (proven red against a renamed-root fixture, then green; it now resolves the root from derive-dependencies, reports what it examined, and fails on a zero count); both CI workflows gated on `git diff --exit-code`, blind to untracked generator output (verified by adding a command source and watching the gate pass); the adopter template derived dependencies: and never references: while its step name claimed generators were in sync. Stale claims in live artifacts x9: this spec's §Generators and Hooks named four generators, two deleted, plus .ductus/scripts/, .govern.toml, and a dry-run CI mode that is not how CI runs; "frozen archaeology" phrasing AGENTS.md forbids; data-model.md named a commands-elaborate marker that has never existed (it is commands-refine), credited gen-spec-deps.sh, and described the pre-018 hook sentinel; AGENTS.md pointed at two files as frozen-archaeology drift candidates that are both clean; README claimed inbox.md eventually disappears where the constitution says it persists; implement.md carried a duplicated audit:ignore-promotion marker; analyze.md cited step 18 twice for a record written at step 17, in check-step-references' documented blind spot; clarify.md still dispatched run-generator at the script 022 deleted while its own markdown-only reference already named the primitive; and configure/auggie.md granted the five file-content parsers claude.md deliberately excludes, which also required giving auggie.md the retirement mechanism it lacked. Fixed in de5ccb4, 1e9fa8c, 3d832ae, 19745b4. 0 MUST, 0 SHOULD, 0 low-confidence outstanding.

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

## Unexamined governance

*None.*
