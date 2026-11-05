---
section: "Decisions persist across runs"
---

# Analyze-findings-match-decisions-by-host-judgment

## Context

An analyze decision is keyed `{family} — {message}`, and that key reproduces across runs only when the message does. The check-artifacts families and step 14's citation template word their messages deterministically. Most other findings are worded by the host: frontmatter, dependency, anchor, and rule-ID results rendered from primitive output (steps 2–7, 9–13), the rule assessments at the `assessSpecQuality` seam (steps 11–12), and grounding (step 15). A stored decision on one of those findings is asked about again whenever the wording drifts. The next run then prunes the old decision as expired.

`/{project}:review` met the same problem for observations and solved it with host judgment rather than byte equality. The host matches each new observation to the stored decision that describes the same issue, and passes that decision's key as `decision-key`. The spec's reason for dropping `{category}` from the analyze key, that it was host-assigned and did not reproduce, applies equally to a host-worded message.

Found by the 058 review (2026-09-25).

## Behavior

`/{project}:analyze` matches findings to stored decisions the way `/{project}:review` does.

- In step 17 the host reads `analysis.md`'s `decisions:` list and matches each live finding to the stored decision describing the same issue. It passes that decision's key in `fired` instead of the finding's own `{family} — {message}`.
- `AnalysisFinding` gains an optional `decision-key`. When present, `write-analysis` keys the finding by it, so a re-matched decision keeps its original stamp. When absent, the key is `{family} — {message}`, as now.
- A finding that matches no stored decision is keyed as now and goes to the disposition rule.

## Edge Cases

- **A deterministic message** matches its stored key exactly, so the host's judgment and byte equality agree.
- **A missed match** costs one repeated question and prunes the old decision. It never waives a finding silently, because an unmatched decision is pruned rather than applied to something else. The review side makes the same trade.
- **Under `ductus exec`** nothing is matched, as now.
- **A `decision-key` naming no stored decision** is used as the finding's key. It becomes a new decision if the finding is deferred or waived.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
