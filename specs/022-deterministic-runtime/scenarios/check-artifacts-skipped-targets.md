---
section: "Follow-on scenarios"
---

# Check-artifacts-skipped-targets

## Context

`check-artifacts` reports two things: a `findings` list and a `clean` boolean derived from it. Every family it carries examines a subject it can always read — a status tier, a task list, a scenario directory — so the two have been sufficient.

The families added by [045 — Decision-state drift detection](../../045-decision-state-drift-detection/spec.md) break that assumption. Both read *targets* that may be unreadable: a link pointing at a file that no longer exists, a spec whose frontmatter will not parse, a target carrying no state the check can be evaluated against. 045's own acceptance criteria forbid escalating any of those into a finding — an unknown is never a defect. Left there, a family that examined every target and found nothing returns exactly what a family that could examine nothing returns, and a caller reads the reassuring one.

That is `QUAL-CLAIM-001` precisely: a fully-implemented path whose output overstates what it verified. This scenario carries the runtime work for 045's requirement, per that spec's Implementation ownership split.

## Behavior

**A `skipped` list on the result.** `CheckArtifactsResult` gains `skipped`, a list of `{family, reason, path}` records naming each target a family could not examine. The rule's own Verification names a `skipped` list among the compliant shapes, so this is the sanctioned form rather than a local invention.

**A closed reason set**, so repeat runs over unchanged inputs are byte-identical:

| `reason` | Raised when |
| --- | --- |
| `target-missing` | the target path resolves to no file on disk |
| `target-unparseable` | the target exists but its frontmatter will not parse |
| `no-readable-state` | the target carries no state the check can be evaluated against |

**`clean` is unchanged.** It stays `findings.is_empty()`. Redefining it to account for skips would silently change the verdict four shipped families produce, which is a larger behavior change than the honesty problem warrants. The assurance therefore lives in the pair: `clean: true` with an empty `skipped` is a verified-clean result, and `clean: true` with a non-empty `skipped` is a partially-examined one.

**The five pre-existing families always return an empty `skipped`.** Their subjects are fully examinable by construction, so no existing result changes shape in value — only in schema.

**Hosts render skips in the Informational tier.** That is where `/{project}:analyze` already puts the cross-service reference unknowns (`unregistered`, `not-checked-out`, `status-unreadable`), which are the same distinction one surface over: provably broken is a finding, cannot-be-checked is recorded and not counted against the artifact.

## Edge Cases

- An empty `skipped` is the ordinary case and carries no rendering — a report gains an Informational line only when something was actually skipped.
- The same target skipped by two families produces one record per family: the reason is family-specific, and collapsing them would lose which check went unperformed.
- The same target skipped twice by one family — two links in one document pointing at the same missing file — produces one record; the fact is about the target, not the citation.
- `clean: true` alongside a non-empty `skipped` is a legal and meaningful state, not an inconsistency to normalize away.
- A family that reads no targets at all for a given spec (the path-existence family below `done`, for instance) records nothing: it was not applicable, which is distinct from having tried and failed.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
