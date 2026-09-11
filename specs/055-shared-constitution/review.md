---
spec: 055-shared-constitution
reviewed-at: 2026-09-11T16:00:31Z
reviewed-against: 51c2b2c444593cb62b4f43726558921eac9f7187
diff-base: dd2fc0c502466bb6db80612b4f10db124fb674f0
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 055-shared-constitution

## Summary

No outstanding violations across all five passes. The run found one real regression and it was fixed before this report was written, so it is not carried as a finding: the registry read added to `write-review` and `write-analysis` was `?`-propagated, so an unrelated parse error in `.ductus/config.toml` made both primitives fail — costing the operator the whole `review.md` in one case and suppressing the `analyze:` record in the other, which the pre-done gate reads as "never analyzed". An unreadable registry is now rendered as the strongest form of unexamined governance instead of raised, with a regression test on each primitive. BE-INPUT-004 was evaluated against `resolve_constitutions::classify` and does not fire: the `path` value is committed project config, not request-time user input, and `primitives/mod.rs` documents that boundary — primitives taking host- or LLM-supplied paths call `validate_no_traversal`, config-sourced ones do not, exactly as `resolve-references` treats `[services]`. Three observations map to no loaded rule and are captured to the inbox rather than filed as findings.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: the clap subcommand enum in `runtime/src/main.rs` is the one primitive-wiring surface nothing pins — deleting a `Command` variant and its dispatch arm leaves the full suite green, so a primitive can be missing from the `ductus <name>` CLI undetected. Every other site is pinned (registry set-equality in tests/mcp.rs, the `#[tool]` listing, `dispatch_handles_every_registry_primitive`, and `PRIMITIVE_NAMES` being a direct alias). A test iterating `PRIMITIVE_REGISTRY` against the clap command list would close it. (found 2026-09-11 during 055)

## Observations

- security: the decision not to call `validate_no_traversal` on a `[constitutions.*]` path rests on the claim that the value is hand-edited committed config — true today, but `/ductus:link` already sets the precedent for a command that writes a registry table programmatically. If anything ever writes `[constitutions]`, that trust claim silently becomes false and nothing checks it. Either keep hand-edit-only as a stated invariant or add the traversal check. — `runtime/src/primitives/resolve_constitutions.rs`
- convention: `schema/constitutions.rs` and `schema/services.rs` are near-identical registry modules, and `duplicate_paths` / `duplicate_repos` differ only in which field they group by. Deliberate at introduction (modelling on services was the point), but a third registry would make the case for one generic group-by-field helper. — `runtime/src/schema/constitutions.rs`
- other: `ConstitutionEntry.description` is parsed, documented in the config schema, and never surfaced — it reaches no primitive result and no report, unlike `ServiceEntry.description`, which the dashboard renders. Either surface it where a source is named or drop it from the schema. — `runtime/src/schema/constitutions.rs`

## Skipped passes

*None.*
