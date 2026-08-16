---
section: "Behavior"
---

# Criterion-identifiers

## Context

An acceptance criterion has no identifier. It is a bare `- [ ]` bullet, and every surface that needs to name one falls back to its position in the list:

- `mark-criterion` addresses criteria by 0-based index, so `/{project}:implement`'s completion gate ticks a box by counting.
- A reviewer, a commit message, or a contributor in conversation has the same problem: "criterion 33" means counting, and means counting *again* to check.

Position is not just unreadable, it is unstable. Inserting a criterion shifts the address of every criterion below it — silently. Prose references that were accurate go stale with no diff marking them wrong, and an index computed before an insert targets a different criterion after it. The failure is quiet in both directions: nothing errors, and the wrong box is ticked.

The framework already solved this shape once for a different artifact. Rule IDs are permanent: `specs/008-security-rules/data-model.md` states an ID is never renumbered once assigned, even if the rule moves within its file or is deprecated. Criteria have the same need — stable reference across edits — and none of the machinery.

Two specs solved it by hand, and then the practice lapsed. `017-derive-dont-ask` labels all 24 of its criteria `AC1:`–`AC24:`, and `018-adopter-owned-pre-commit` labels all 13 of its own; `specs/018-adopter-owned-pre-commit/spec.md:108` makes a cross-spec reference to "017 AC24" that resolves correctly because the target is labelled. The convention works, is readable in the raw markdown, and survives a copy-paste. What it lacks is any mechanism: nothing assigns, validates, or resolves those labels, so specs 019 onward simply stopped writing them — 660 of the repository's 697 criteria are unlabelled.

## Behavior

- An acceptance criterion carries a stable identifier, visible in the artifact, that a reader can cite without counting and a tool can resolve without position.
- The identifier is permanent for the life of the criterion: assigned once, never reused for a different criterion, never renumbered when criteria are inserted, reordered, or removed. This is the property position cannot provide and the reason rule IDs already work this way.
- Removing a criterion retires its identifier rather than freeing it, so an old reference resolves to "removed" rather than silently to a different requirement.
- The identifier is written in the criterion text, not in frontmatter — it must be readable in the same place the criterion is read, and survive a copy-paste into a review comment or commit message.
- The runtime assigns it, never the agent: an idempotent labelling pass fills in any unlabelled criterion, leaving labelled ones untouched, so no actor ever counts a list to work out the next number.
- The spec's frontmatter records `next-criterion`, the label the next assignment will use. It is monotonically non-decreasing and maintained by the labelling pass, which is what makes a retired label unreissuable without consulting history.
- Criteria without an identifier remain valid to read and to tick: the format stays backward-compatible, so no spec becomes unparseable on the day this lands, and the backfill is what removes the unlabelled state from the corpus rather than a parser requirement.
- The identifier is what tooling addresses when present — a positional fallback exists only for unlabelled criteria, and the two addressing modes never disagree about which criterion is meant.

## Edge Cases

- **An unlabeled criterion** — addressed positionally, exactly as today. The fallback is what keeps existing specs working.
- **A duplicate label within one spec** — ambiguous, so it is a defect the artifact audit reports rather than a state any tool resolves by picking the first.
- **The highest-numbered criterion is deleted** — the only case where reuse was ever possible, since the body maximum drops. `next-criterion` does not, so the retired label is never reissued.
- **A spec with no `next-criterion` field** — it has never been labelled. The absence means "no labels assigned yet", not a defect; the field appears when the backfill or the first labelling pass runs.
- **`next-criterion` below the highest label in the body** — a hand-edited or corrupted counter. The audit reports it, since the invariant is checkable in the artifact alone.
- **Every criterion deleted from a labelled spec** — the body maximum falls to zero while `next-criterion` stands, so the next criterion continues the sequence instead of restarting at `AC1:` and colliding with every historical reference.
- **Criteria reordered after labelling** — labels stop tracking position, by design. `AC5` can sit above `AC3`, so "the third criterion" and `AC3` are different things once a list has been reordered. This is what a permanent identifier means and rules already behave this way; the labelling pass must therefore assign from `max(existing) + 1`, never from position, or a reorder would renumber the list and break every existing reference.
- **A criterion whose text is rewritten in place** — the identifier stays with it. A rewrite that changes the requirement's meaning is a new criterion with a new identifier, but nothing mechanical distinguishes the two; this is an authoring judgment, as it is for rules.
- **Cross-spec references** (`018`'s "017 AC24") — resolvable once the backfill lands, since every spec is labelled from that point. Until then the pair above already resolves because both specs were labelled by hand.
- **A criterion added by a command route rather than by hand** — the route calls the labelling pass and reports the label it was given. This is the dependency `000-slash-commands`'s `criterion-route-after-draft` scenario was blocked on, and the assignment rule resolved here is what unblocks it.

## Open Questions

*None — all resolved.*

## Resolved Questions

**What is the identifier's form and who assigns it?**

Form: `- [ ] AC{n}: <criterion text>` — an in-band label, visible where the criterion is read and intact when the line is copied into a review comment or commit message. This is the form `specs/018-adopter-owned-pre-commit/spec.md:126-135` already uses, so the convention has a working precedent rather than being invented here.

Assignment: **the runtime**, through one idempotent labelling pass rather than at append time. A primitive scans a spec's Acceptance Criteria, leaves every already-labelled entry untouched, and assigns `max(existing) + 1` in body order to any entry that lacks a label — counting retired labels so a number is never reused — returning the assignments it made. The derivation is the one `append-task` already performs for an in-band `## N.` heading; only the invocation shape differs.

It has to be a pass, not append-time assignment, because almost no criterion is written by a per-criterion primitive. `/{project}:specify` fills the whole spec body through the `writeSpecBody` LLM extension as one markdown payload, and `/{project}:clarify` edits criteria as prose (no primitive writes prose). An append-time-only rule would label the rare criterion added by a command route and leave every criterion born at specify or refined at clarify unlabelled forever.

The pass runs at five points, all the same code:

- `/{project}:specify`, after `writeSpecBody` — the initial batch is labelled in the run that created it.
- `/{project}:clarify`, after the criteria pass — criteria added or rewritten during clarification are labelled before the spec advances.
- the criterion-adding command route — the appended criterion is labelled and its label returned to the caller.
- the pre-commit hook — the backstop for criteria typed by hand in an editor, in the same slot as `gen-spec-deps.sh`.
- the one-shot backfill migration — the same primitive, run once across the corpus.

So a label is initialized at the first ductus write that touches the criteria list, and at latest at commit time. The agent authors the criterion's text and never its number; citing a criterion never requires counting, including in the session that created it.

The hook invocation edits body text, which no existing generator does — `gen-spec-deps.sh` and `gen-cross-service-refs.sh` maintain frontmatter only. That widening is accepted deliberately: inserting an `AC7:` prefix after a checkbox is mechanical rather than semantic, and the alternative — an audit that reports unlabelled criteria for a human to fix — puts a person back to counting, which is the work this scenario exists to remove.

The question framed this as author-written versus generator-maintained, and both halves of that pair were wrong for this project:

- "Author-assigned, like rule IDs" papers over which actor counts. In practice the author is the agent, and picking `max + 1` means tallying a 33-item list — the same operation that produced the uncited "criterion 33" this scenario exists to eliminate, and the operation an LLM is least reliable at. Rule IDs survive author assignment because a rule file is small, hand-curated, and rarely appended mid-pipeline; acceptance criteria are none of those.
- "Generated by the pre-commit hook" is the wrong mechanism, not the wrong instinct. The existing generators (`gen-spec-deps.sh`, `gen-cross-service-refs.sh`) maintain *frontmatter*; none rewrites body prose. Worse, a hook-assigned label does not exist until the hook runs, so a criterion written and discussed in one session has no identifier during the conversation that created it — the moment citation matters most.

Deriving it in the primitive avoids both: assignment is deterministic, in-band, and immediate.

This follows the project's direction of putting deterministic work in the runtime rather than in the agent — the same direction [048 — Ductus-Acquired Runtime](../../048-govern-acquired-runtime/spec.md) takes by making `ductus` an artifact the pipeline acquires and version-pins instead of an optional binary on `PATH`. A label is pure arithmetic over a file the runtime already parses; there is no judgment in it to leave with the LLM.

Validation is separate from assignment and remains necessary, because a criterion typed by hand in an editor never touches a primitive. The artifact audit gains a check in the shape `check-rule-ids` already has for rule citations: a duplicate label within a spec is a defect, and a label at or below the current maximum is a reuse defect. Assignment is the runtime's, enforcement is the audit's, authoring is the agent's.

On the markdown-only path the host performs the same derivation by reading the file and taking `max + 1` — one contract, two paths (§runtime-host-integration). The rule is arithmetic, so both paths agree by construction.

**Do unlabelled criteria get backfilled, or do labels apply only going forward?**

Backfilled — every criterion in the corpus carries a label. At decision time that is 660 of 697 criteria across 47 specs; `017-derive-dont-ask` (24/24) and `018-adopter-owned-pre-commit` (13/13) are already labelled and are left exactly as written.

Going-forward-only was rejected because the assignment rule makes it actively misleading rather than merely incomplete. `max(existing) + 1` on a spec with no labels yields `AC1:` for its *newest* criterion, sitting below thirty-odd unlabelled ones written years earlier. The label would be unique, stable, and read as "the first criterion" — worse than no label at all.

It also breaks the audit. A missing-label check cannot distinguish "unlabelled because it predates the convention" from "unlabelled because someone hand-edited and the hook did not run", so the check would need a per-spec grandfather date — the kind of exemption state [046 — Scenario open-question visibility](../../046-scenario-open-question-visibility/spec.md) refused for scenario questions, on the grounds that a sanctioned hiding place is worse than the gap it papers over.

The sweep itself is arithmetic — number in body order from `AC1:`, inserting the label after the checkbox without altering the criterion's own text — so it qualifies as a uniform mechanical edit under §spec-lifecycle and the `done` specs it touches stay `done`. It is performed by the labelling pass above, not by hand.

Two properties the sweep must preserve:

- **Existing labels are never renumbered.** `017`'s `AC1`–`AC24` and `018`'s thirteen stay as authored. Renumbering them would break `specs/018-adopter-owned-pre-commit/spec.md:108`'s working cross-spec reference to "017 AC24" — the one thing the hand-rolled convention has already bought.
- **Body-order numbering makes existing count-derived references correct by construction**, wherever the list has not changed since the reference was written. Anyone who wrote "criterion 12" derived it by counting body order, which is exactly what the backfill assigns. Where a criterion was inserted or removed since, that reference was already wrong and the backfill neither fixes nor worsens it.

Delivery is the migration registry ([027 — Bootstrap migration registry](../../027-bootstrap-migration-registry/spec.md), `framework/migrations.toml`), which already exists to push one-shot artifact migrations to adopter projects on their next `/ductus` run. Without it this repo would be labelled while every adopting project stayed unlabelled — a two-tier corpus at the ecosystem level instead of the spec level, which is the same defect the going-forward-only option was rejected for.

**Is identifier retirement enforced, or left to authoring discipline?**

Enforced structurally, by recording the next label in the spec's frontmatter. No git history is consulted.

The exposure is narrower than "a removed criterion could have its label reused" suggests. Under `max(existing) + 1`, reuse is possible only when the maximum *decreases*, and that happens in exactly one case: deleting the highest-numbered criterion. Removing `AC5` from a spec labelled `AC1`–`AC33` leaves the maximum at 33 and nothing is reused. Removing `AC33` drops it to 32, and the next assignment hands out `AC33` a second time — to a different requirement, silently.

The fix is to stop deriving the maximum from a set that can shrink:

- The frontmatter carries `next-criterion` — the label the next assignment will use, monotonically non-decreasing.
- Assignment takes `max(highest label in body, next-criterion)`, then writes the incremented value back. The labelling pass maintains the field, so it is never hand-updated.
- Deleting the top criterion lowers the body maximum but not `next-criterion`, so a retired label is never reissued.
- The audit check is one comparison: `next-criterion` must exceed every label present in the body. A hand-edited value that was lowered is a defect, detectable in the artifact itself.

A spec that has never been labelled has no `next-criterion`; the field appears when the backfill or the first labelling pass runs, and the schema treats its absence as "no labels assigned yet" rather than as a defect. Defining it belongs here because this spec owns the frontmatter schema (§Frontmatter Schema).

The rule-ID precedent does not transfer, as the question noted, and the reason is instructive: a rule is never deleted — a deprecated rule stays in its file carrying its ID — so the rule file *is* the high-water record. Criteria legitimately disappear when scope changes, so the record has to be explicit.

The broader principle is worth stating, because two other open scenarios are circling it: when a fact will later be needed about something no longer present in an artifact, record it at the moment it is known rather than reconstructing it from history afterward. `000-slash-commands`'s `scenario-without-task-visibility` is unresolved precisely because its fact — was this scenario ever implemented — was never written down, and history-based reconstruction is fragile (a shallow CI checkout already broke one such check; see `AGENTS.md`, "A CI check that reads git history needs `fetch-depth: 0`"). Here the fact is knowable at assignment time, so it is stored, and the question dissolves.
