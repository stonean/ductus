---
section: "Follow-on scenarios"
---

# Family-19-review-freshness

## Context

The release-time half of the rule 022's `review-staleness-gate` enforces at
completion time. `check-review-gate` fires when someone moves *one* spec to
`done`; nothing asked the same question of every `done` spec at once, which is
the question a release needs answered.

`gvrn-v0.26.2` is the case: tagged at `334907f` with spec 022's review reading
`reviewed-against: 1f7ee722`. The tag pipeline runs the self-audit as a hard
gate and the audit had no opinion, because no family compared a recorded review
against history.

## Behavior

`scripts/audit/review-freshness.sh`. For each spec at `status: done` carrying a
`review:` block with a resolvable `reviewed-against`, emit a finding when any of
the spec's **durable contracts** — `scenarios/*.md` or `data-model.md` — changed
between that commit and `HEAD`.

Grandfathering matches the rule `/{project}:analyze` and the shipped CI template
already apply: a `done` spec with no `review:` block predates `/{project}:review`
and is exempt. A null `reviewed-against` is `check-review-gate`'s
`NotReviewed` finding, not this one. A `reviewed-against` that does not resolve
to a commit here — a shallow clone, a rewritten history — is reported as its own
finding rather than silently passing, since the check cannot prove staleness it
cannot resolve.

## Scoping, measured rather than guessed

The scope was wrong twice before it was right, and both wrong versions were
caught by measuring against this repo before wiring anything:

| Rule | Fires on | Why it fails |
| --- | --- | --- |
| Plan's **Affected Files** | 42 of 48 | Old specs list shared surfaces (`AGENTS.md`, `README.md`, `framework/bootstrap/ductus.md`) that every later spec also touches, so spec 004 reads stale because spec 042 edited `AGENTS.md` |
| The whole spec directory | 31 of 48 | `tasks.md` churns on every ticked checkbox and is ephemeral by construction ([§tasks-phase](../../framework/constitution.md#tasks-phase)); `plan.md` churns as Affected Files are revised |
| **Durable contracts** | 10 of 48 | Ships |

(The three counts are snapshots taken on 2026-08-03. The shipped rule's count drains as reviews are refreshed — it was 9 within the hour, once reviewing 026 cleared its own entry.)

A gate firing on 87% of specs is one people disable in a week, which is worse
than no gate — it trains the reader that findings are noise. The durable-contract
rule was verified to catch both real failures: `gvrn-v0.26.1` and `gvrn-v0.26.2`
each added 022 scenarios after the review they shipped under.

This is the second time this family set has needed a scope narrower than the
obvious one — `criterion-path-existence` needed the same treatment — and the
lesson generalizes: measure a proposed check against the corpus *before* wiring
it, because a check's value is its precision, not its coverage.

## Edge Cases

- **Deliberately not wired into `run-all.sh`.** It reported 10 pre-existing
  stale reviews when it landed, so wiring it would block the next `ductus-v*` tag
  until every one is re-reviewed. That is real debt and the findings are real,
  but imposing a release freeze is the maintainer's call, not a side effect of
  landing the check. Wiring is one `run_check` line once the debt is cleared.
- **Different scope from the runtime gate, on purpose.** `check-review-gate`
  reads Affected Files because it judges one spec at the moment someone
  completes it, where a broad scope costs one re-review. This family judges all
  of them at release, where a broad scope costs a freeze.
- **`plan.md` and `tasks.md` are never counted** — the first churns, the second
  is ephemeral by construction.
- **A spec with no scenarios and no `data-model.md`** has no durable contract to
  compare and is skipped rather than flagged.
- **Requires `git` and `python3`**, matching Families 17 and 18; a missing tool
  is a precondition finding, not a silent pass.
- **Requires full git history, which CI does not give by default.** Resolving
  `reviewed-against` needs the commits it names, and `actions/checkout` clones
  shallow. The first release after this family was wired (`gvrn-v0.27.1`)
  failed its gate with "not a commit in this repo" for all 48 specs and
  skipped the publish — while passing locally on every run, because a
  developer checkout has full history. Both workflows that run `run-all.sh`
  now set `fetch-depth: 0` with a comment saying why. The unresolvable-sha
  case stays a **finding** rather than a skip: with history present it means
  a rewritten or bogus sha, which is worth reporting.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
