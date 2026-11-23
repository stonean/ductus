---
section: "Two Rule Files"
---

# Be-deps-005-flags-what-its-statement-requires

## Context

With `security-backend.md` loaded, `FE-DEPS-008` and `FE-DEPS-009` yield every dependency, the frontend's included, to `BE-DEPS-004` and `BE-DEPS-005`, so the owner's 2026-11-19 changes to the two frontend rules (this spec's Resolved Questions 18 and 19) reached only a project without it, as this spec's review of 2026-11-20 reported. Resolved Question 19's two reasons hold for `BE-DEPS-005` word for word: its Verification flags an omission only "when the project uses private packages or has a high dependency-churn rate", a condition its Statement does not carry, and its (c) names no artifact an analysis could see. Resolved Question 18's do not hold for `BE-DEPS-004`: it already names its moment, before the dependencies are built into or shipped with an artifact, and gives an installer with no verification a compliant design, the documented gap with hash-checked installs (the agent's recommendation, adopted by the operator, 2026-11-23).

## Behavior

`framework/rules/security-backend.md` states, and `/{project}:analyze` flags accordingly:

- **`BE-DEPS-005`.** The Verification flags a spec that omits (a) where private packages exist, omits (b), or names no place for (c)'s check, with no dependency-churn condition, and still flags a pip configuration that reaches private packages through `--extra-index-url`. (c) requires the plan to name where a new dependency's name is checked (an allowlist, a lockfile-diff review step, or a tool), so an analysis can see it. A backend-only project is held to both, as a full-stack one is.
- **`BE-DEPS-004` is unchanged**, and so is the hand-off: with both files loaded, `FE-DEPS-008` and `FE-DEPS-009` still yield every dependency to the two backend rules.

## Edge Cases

- **(b) still binds only where the package manager supports it,** as its Statement says, so dropping the churn condition does not reach a package manager that commits no hashes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
