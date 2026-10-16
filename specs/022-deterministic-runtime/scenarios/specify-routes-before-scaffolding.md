---
section: "Follow-on scenarios"
---

# Specify-routes-before-scaffolding

## Context

`/{project}:groom` owns a five-route decision tree: it matches an inbox item to an existing spec, adds a scenario, takes the back-edge, and only recommends `/{project}:specify` when nothing covers the area. `/{project}:specify` has no equivalent. It resolves the feature number and goes straight to the create-feature gate, with no check for whether the work belongs to an existing rule surface or to 022.

The consequence is that `AGENTS.md`'s two routing rules — *"add a rule to its surface's home spec via the back-edge, do not spawn a new spec for it"* and *"route runtime work to 022 via the back-edge"* — are enforced only for work that happens to arrive **through the inbox**. Work arriving through conversation bypasses the tree entirely, and creating a spec is the one action those rules exist to prevent.

The rules also live in `AGENTS.md` §Workflow, which no command loads as normative criteria (`/{project}:review` loads Code Style, Testing, Gotchas and Boundaries). So at the single moment routing can be got wrong, nothing in the agent's context states the rule.

Observed 2026-08-16: a `check-artifacts` family change was proposed as a new spec, which the runtime-work rule routes to 022 as a scenario; the operator caught it. Nothing mechanical would have.

Requiring spec: [017 — Derive, Don't Ask](../../017-derive-dont-ask/spec.md). This is its principle exactly — a rule that holds only when the author remembers it, failing silently and asymmetrically, since the cases where it is skipped are the cases where someone was already confident enough to skip it.

## Behavior

**`/{project}:specify` runs the routing decision before scaffolding.** After the feature name is known and before `create-feature` writes anything, the command derives candidate homes and presents them. It reuses groom's tree rather than growing a second one — two routing rules that could disagree is the drift this repo has been bitten by before.

**Candidates are derived, not recalled.** From the rule-file directory (a rule surface whose category matches), from the spec corpus (a spec whose subject covers the area), and from the runtime-work signal: work naming a primitive or a path under `runtime/` is offered, as a scenario, to the lowest-numbered spec whose plan claims a `runtime/` path. That candidate is a lexical hint; which spec owns the primitive is the routing decision. An empty derivation is reported as *no candidate found* rather than silently yielding "new spec".

**"New spec" becomes a confirmed choice, not the default.** When candidates exist, the gate names them and the operator picks — a scenario on `NNN`, a rule-file amendment, or a new spec anyway. When no candidate is found, the gate says so and the run proceeds as it does today. Either way the spec is created only after the routing is confirmed, so the rules bind wherever work enters.

**It reports, it does not veto.** A new spec remains creatable over any candidate; the operator's answer decides. The framework's job is to make the alternative visible at the moment of the decision, not to litigate it.

## Edge Cases

- No rule files and a single-spec corpus: no candidates, the gate reports that, and specify runs unchanged — a fresh adopter sees no new friction.
- The work genuinely is a new surface (a new rule *file*, a new capability): candidates may still be offered; "new spec anyway" is one keystroke and is the correct answer.
- `/{project}:groom` routing an item to `/{project}:specify`: the tree already ran, so the routing gate is skipped rather than asked twice.
- A candidate spec at `done`: naming it must also name the back-edge it implies, exactly as groom's confirmation does, so the operator consents to the reopen before it happens.
- Derivation failing (unreadable rule dir, unparseable spec): reported as *could not derive candidates* and distinguished from *no candidates found* — the two are not the same answer (`QUAL-CLAIM-001`).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Should the runtime-work candidate be the spec that owns the named primitive,
  rather than the lowest-numbered spec whose plan claims `runtime/`?** No.
  Resolved 2026-10-16 by the operator.

  Neither source the corpus offers records ownership. Measured over the 67
  modules under `runtime/src/primitives/`, with `compute-review-scope`'s
  `plan-affected` for every spec and a scan of every `data-model.md` heading:
  a plan's Affected Files records which specs *touched* a file, so
  `check_artifacts.rs` is claimed by seven plans and fifteen modules by none,
  since 022's plan claims the whole directory; and headings name 44 modules in
  exactly one data model, 6 in two (058 beside the owner) and 17 in none,
  `fetch-archive` among them. A derivation from either would offer history as
  ownership, or fall back to this candidate anyway — `resolve-constitutions`,
  `resolve-references` and `enforce-manifest` would still be offered 022.

  So the candidate stays a lexical hint, and which spec owns the primitive
  stays with the operator at the `routeInboxItem` judgment, where `AGENTS.md`'s
  routing entry names the runtime's three homes: 022, 066 and 067.
