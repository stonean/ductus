---
status: draft
dependencies: [013-text-first-artifacts, 022-deterministic-runtime, 046-scenario-open-question-visibility]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
---

# 045 — Decision-state drift detection

Detect artifacts that assert a state a sibling artifact has since resolved. When a decision is made — an open question closed, a scenario shipped, a status advanced — the artifacts that referenced the prior state keep describing it as current, and nothing in the pipeline notices.

A missing document sends the reader looking. A stale one is trusted and acted on.

## Motivation

The framework covers the *document → back-link* case: §drift-prevention's Cross-document references requires that "Changing A includes auditing every back-link to A". The common failure is *decision → dependents*, and three mechanisms each miss it for a different reason.

**The grounding check is a form check, not a truth check.** `/{project}:analyze`'s grounding step verifies that a claim about existing reality carries a citation or an explicit hedge — it explicitly does not read source to confirm the claim is true. A stale assertion that correctly cites its source ("the scenario's open question expects…") passes grounding while being false. Grounding is structurally blind to this class.

**§drift-prevention is scoped to documents, not decisions.** Its audit obligation triggers on *editing document A*. When a scenario's questions are resolved, document A *was* edited — but the trigger a contributor recognizes is "a question got resolved", and the obligation is not stated in those terms anywhere.

**`/{project}:review` cannot see it.** It audits code against rules; artifact-vs-artifact consistency is `/{project}:analyze`'s job by design.

### Observed case

Adopter repo `svc-zmc-api`, spec `020-server-config`, 2026-07-28. A scenario's five open questions were resolved and its worker was implemented, tested, and shipped. The `plan.md` that links to that scenario was not swept, and continued to assert:

| `plan.md` claim | Actual state |
| --- | --- |
| "still open on where the dump executes" | Resolved the same day |
| "the scenario's open question expects the `PUT` path to signal the worker" | Resolved to pull-not-push; the `PUT` owes no emit |
| "the worker it would notify does not exist yet" | Shipped, registered, running |
| "See the note in `tasks.md`" | That note had been replaced by two tasks |

The second is the damaging one: a deliberate design decision read as an unmet obligation. Any reader of that plan would have concluded the feature was unbuilt. A human caught it during `/analyze`'s semantic pass; nothing mechanical flagged it.

## Scope

The check is narrow by construction. It does **not** attempt to prove an arbitrary assertion is currently true — that is unbounded. It targets the high-signal case where an artifact's *own link* resolves to a state that contradicts the prose around it.

Against the observed case, that catches the first three rows. It does not catch the fourth: "See the note in `tasks.md`" is a valid link to a section that no longer says what the citing prose claims, and verifying it needs a fragment anchor or semantic reading. Partial mechanical coverage plus an explicit obligation in the constitution is the deliverable; complete coverage is not achievable
deterministically.

## Behavior

### The decision trigger (constitution)

§drift-prevention states that resolving a decision carries the same audit obligation as editing a document: every artifact that described the prior state is corrected in the same change, and a resolution is not complete while a sibling artifact still describes the question as open. The trigger list names the recognizable events — closing an open question, shipping a scenario, advancing a status, adopting a previously-rejected option.

### The advisory check

For each artifact in a feature directory, for each inline link to a sibling artifact in the same feature:

1. Read the target's current state — its frontmatter `status` where it has one, and its open-question count.
2. Scan the prose unit containing the link for **open-state tells** — phrases asserting the target is unresolved or absent.
3. Emit a finding when a tell co-occurs with a target state that contradicts it: prose says "open question" while the target reports zero; prose says "does not exist yet" while the target is `in-progress` or `done`.

The check is **advisory** (non-blocking) at introduction, matching the existing grounding and Applicable-Rules checks, and carries a documented promotion criterion.

This extends the deterministic check families introduced by [022-deterministic-runtime](../022-deterministic-runtime/spec.md); severity tiers follow [013-text-first-artifacts](../013-text-first-artifacts/spec.md) and introduce no new policy.

## Acceptance Criteria

- [ ] §drift-prevention names the decision trigger, not only the document-edit trigger, and lists the recognizable resolution events
- [ ] §drift-prevention states that a resolution is incomplete while a sibling artifact still describes the prior state
- [ ] `/{project}:analyze` emits an advisory finding when an artifact's link-adjacent prose asserts an open state that the link target's current state contradicts
- [ ] Each finding names the citing file and line, the link target, and the target's contradicting state
- [ ] A feature directory whose prose matches its link targets' state produces zero findings
- [ ] The check covers every artifact in a feature directory that carries inline sibling links — `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md`
- [ ] The finding is advisory (non-blocking) at introduction and does not gate `done`
- [ ] Repeat runs over an unchanged feature directory produce identical findings
- [ ] A link whose target state cannot be read produces no finding (an unknown is never escalated to a defect, matching the `status-unreadable` precedent in cross-service reference classification)
- [ ] `analyze.md` documents the check and its promotion criterion alongside the existing advisory checks

## Open Questions

- Where does the rule live — an amendment to constitution §drift-prevention, or a new artifact-tier rule file carrying a citable rule ID? The existing `quality-cross.md` is explicitly scoped to code patterns enforced by `/{project}:review`, so it is not the home; an artifact-tier rule file would be a new rule category.
- What is the closed list of open-state tells, and is it fixed by the framework or configurable per project? Candidate seed list: `open question`, `unresolved`, `still open`, `not yet`, `does not exist`, `left unimplemented`, `deferred`, `TBD`.
- What is the prose unit scanned for a tell — the sentence containing the link, the list item, or the paragraph? The unit determines both false-positive and false-negative rate and needs to be deterministic.
- Does the check ship as a `check-artifacts` family on the runtime path, or start markdown-only and mechanize after the tell list stabilizes?
- What is the promotion criterion from advisory to blocking? The issue proposed a threshold shaped like the existing ones (e.g. a minimum finding count across a repo on two consecutive `--all` runs).
- How does a link targeting a scenario behave, given a scenario has no `status` field? This intersects the scenario open-question signal specified in [046-scenario-open-question-visibility](../046-scenario-open-question-visibility/spec.md).
- Does a tell inside a fenced code block, an HTML comment, or a quoted historical passage count? The existing markdown-parsing helpers already exempt some of these.
- How does the check avoid firing on every completed spec's `## Motivation`? A Motivation legitimately describes the world *before* the feature — a world that stops existing the moment the spec ships — so its prose naturally carries the candidate tells (`does not exist`, `not yet`, `has no`, `stays silent`). Evidence (2026-07-31, `/gov:analyze` run against 046): three of 046's four Motivation bullets became false on ship, and `041-task-pruning` is `done` with a Motivation still reading "`govern` has no command to reclaim that space" — false since `/prune` shipped, and only *reading* correctly because the next sentence introduces `/prune`, narrative framing a bulleted list lacks. So this is systemic, not a per-spec slip. Candidate resolutions: (a) an **authoring convention** — Motivation is written in past tense, so a present-tense tell there is genuinely suspect (the fix 046 applied); (b) **exempt `## Motivation` by heading** — simple, but blind to a real stale claim parked there; (c) **rely on the existing link-adjacency requirement** — already this spec's design, so the question becomes whether Motivation prose links often enough to trip it. The choice changes this spec's acceptance criteria.
- Does the check cover **stale acceptance criteria**, not just stale prose? Concrete case found 2026-08-02 while running 026's completion gate: 026 AC5 asserted "Registry equivalence verifies every entry in `framework/workflows/registry.json`…" and AC2 asserted "The nine check families are implemented" — both checked `[x]`, both true when 026 reached `done` in May. Spec 043 then sunset the workflows feature, deleting `framework/workflows/` and `scripts/audit/registry-equivalence.sh` in commit `531e3ea`, without unchecking 026's criteria or updating its Behavior section (which still documented `#### 3. Registry equivalence`). Nothing caught it, because **nothing re-verifies a `done` spec's criteria after a later spec deletes their subject** — and `/gov:implement`'s completion gate would have rubber-stamped 14/14 had the criteria not been read against the filesystem. This is a stronger signal than the Motivation-tense cases in the question above: those are stale *prose* a careful reader can discount, whereas this is a stale *contract* that a gate treats as satisfied. It also has a cleaner tell — an acceptance criterion naming a path (`framework/workflows/registry.json`) or an artifact (`scripts/audit/registry-equivalence.sh`) that no longer exists is mechanically checkable without any semantic judgment, unlike the present-tense-prose heuristic. Two implications to settle: (a) whether `## Acceptance Criteria` is in scope for the link-adjacent open-state check at all, and (b) whether a *path-existence* sub-check — every filesystem path named in a `done` spec's criteria still resolves — is worth shipping as its own deterministic family, since it needs none of the tell-list machinery the rest of 045 depends on.

## Prior art

Adopter-side interim: `svc-zmc-api` `AGENTS.md` §"Never Knowingly Leave Stale Information" (commit `8b077a77`) carries the rule, the trigger list, the sweep greps, and the worked example. It was placed in `AGENTS.md` rather than a rule file because the concern is artifact discipline enforced by `/analyze`, and because adopter edits to shipped rule files are overwritten by `/govern` unless
pinned. If this spec ships, that section becomes a candidate for replacement by the framework rule.
