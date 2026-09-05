---
section: "Follow-on scenarios"
---

# Write-analysis and the second gate

## Context

The runtime half of [047's `analyze-run-durability`](../../047-analyze-findings-durability/scenarios/analyze-run-durability.md), recorded here because 022 owns the runtime and is where runtime rules stay findable.

The requirement is 047's: `/{project}:analyze` had no durable record, so `check-review-gate` could enforce the review half of the pipeline and nothing at all of the analyze half. What 022 owns is the shape of the primitive, the block's field set, the two new gate reasons, and the ninth `check-artifacts` family.

## Behavior

**`write-analysis`** — a new primitive, registered at all five sites (the CLI enum and its dispatch arm, the exec-path match arm, the `#[tool]`, `PRIMITIVE_REGISTRY`, and `framework/runtime-tools.txt`).

It writes the spec's `analyze:` frontmatter block and nothing else, splicing it in without disturbing sibling keys. The splice reuses `write_review`'s region logic rather than copying it: `splice_review_block` was generalized to `splice_top_level_block(fm_text, key, block)` and both callers go through it. Sharing is the point rather than a tidy-up — two copies of "find the top-level key, find where it ends, swap the region" would agree until one met a frontmatter shape the other had not, and the failure mode there is a corrupted `spec.md`, not a wrong answer.

Field set, and the two entries that are not in `review:`:

| field | gates? | why |
| --- | --- | --- |
| `last-run` | yes (absent/null → `not-analyzed`) | the record's presence *is* the claim |
| `analyzed-against` | no | recorded now so a future staleness check has data; asserting on it today would invent a gate to match a symmetry |
| `hard-fail`, `blocking-findings` | yes, via derived `blocking` | the two tiers that hold a spec out of `done` |
| `advisory` | **no** | analyze's advisory tier is checks introduced advisory *with published promotion criteria*; gating would promote all of them at once |
| `unexamined` | no | the `QUAL-CLAIM-001` field — see below |
| `blocking` | yes | derived by the primitive, never supplied by the caller |

`unexamined` is why this is not a copy of `ReviewBlock`. A clean analyze is two states — everything examined and clean, or something unexaminable and the rest clean — and `/{project}:analyze`'s own Unexamined-targets section already draws that line. A record carrying only finding counts would collapse it into the reassuring reading inside the artifact a later gate reads, which is the worst possible siting for that failure. `blocking` being **derived** rather than passed is the same discipline: a caller cannot record a clean gate over a dirty run.

A spec whose frontmatter does not deserialize gets **no** record. The value is parsed and discarded purely for that guard — the analysis would have hard-failed on such a spec, and writing a clean record into it inverts the mechanism.

**`check-review-gate`** gains checks 7 and 8, extracted into `analyze_gate_block` (the function was already at clippy's 100-line ceiling; the existing `pending_fold_block` / `stale_review_block` seams are the pattern). Two new `ReviewGateBlock` variants, `NotAnalyzed` and `AnalyzeFindings`, ordered after every `review:` check.

The primitive keeps its name though it now gates on both commands. Renaming is five registration sites and a breaking MCP change to buy a more accurate noun; the name is documented as historical in `/{project}:implement` instead.

**`check-artifacts`** gains `analyze-state-drift` — eight residual deterministic families become nine. It mirrors `review-state-drift` on the two states that gate and deliberately not on the advisory count, where its sibling *does* check `should-violations`. That asymmetry is stated in both the family and the block's own documentation, because a reader who notices it will otherwise read it as an omission.

## Edge Cases

- **`skip_serializing_if` on `Frontmatter.analyze`.** Absent stays absent: `read-spec` on a pre-record spec emits no `analyze` key rather than a defaulted block, so a consumer can distinguish "no record" from "an empty one". The grandfather rule depends on that distinction.
- **A newline in `analyzed-at` or `analyzed-against`.** Flattened to a space, not rejected. `write-review` rejects the equivalent because its fields carry operator prose with intent to preserve; these two are a timestamp and a sha, where a newline is a caller defect with no legitimate reading and only a frontmatter injection to defuse.
- **CRLF specs.** The whole file is normalized to its own existing line ending after the splice, the same guard `update_spec_review_block` carries and for the same reason: a partially-converted file is the outcome no later reader can tell from a hand-edit.
- **The gate's fixtures.** Every existing `check-review-gate` test fixture carried a `review:` block and no `analyze:` one, so all of them began failing the moment the check landed — which is the check working. They were given a clean analyze block; the new reasons got their own fixtures, including one asserting that a spec failing *both* gates is told about the review.

## Resolved Questions

- **Why not fold the analyze record into `write-review`, so one primitive writes both?** Because the two are written by different commands at different times, and a primitive that writes a block its caller did not compute is how one of them ends up stale. The generalized splice is the right amount of sharing: the mechanism is common, the claims are not.
- **Why is `blocking` derived rather than an argument?** So there is no call that records `blocking: false` alongside a non-zero `hard-fail`. `write-review` derives its own `blocking` from `must > 0` for the same reason. A field a caller can contradict is a field that will eventually be contradicted.
