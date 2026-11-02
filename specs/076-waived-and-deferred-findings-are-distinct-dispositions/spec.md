---
status: in-progress
dependencies: [020-code-review, 058-findings-route-at-discovery]
cross-spec-impact:
  - 020-code-review
  - 058-findings-route-at-discovery
  - 066-audit-records-and-the-completion-gate
  - 073-decided-rule-departures-are-recorded
next-criterion: 19
---

# 076 — Waived and deferred findings are distinct dispositions

A **waived** finding is one the pipeline will not address, recorded with the reason it is accepted as it stands. A **deferred** finding is one that will be addressed later, recorded with the work that will address it: pipeline work such as a task, an external work-item ID (a Jira issue key, a GitLab work item), or both. The two terms name different states, so a record never leaves a reader to guess which one it holds.

## Motivation

The pipeline used *waived* for two different things. The constitution's completion filter (§design-principles) listed a "waived review finding" among the records the pipeline will surface again, beside a task, scenario or criterion. But review's waiver ([020](../020-code-review/spec.md)) recorded a decision **not** to fix a MUST or SHOULD violation, with its reason, and took the finding out of the gate's counts until the file or rule it was anchored to changed. The review reference also invited adopters to add a `ticket` field to a waiver (`framework/reference/review.md`, §Waivers). That let one entry mean either "will not fix" or "will fix later, tracked elsewhere", and nothing told the two apart.

No disposition meant *will be addressed later*. A finding was fixed, routed to an artifact the pre-`done` gate reads, or discarded with its reason ([058](../058-findings-route-at-discovery/spec.md)), and a gating review finding was fixed or waived. Work tracked in an external tracker had no recorded form.

*Deferred* already had another meaning: a question deferred until a trigger is resolved *with* that trigger, in Resolved Questions (§spec-requirements).

## Terms

- **Waived.** A decision not to address a finding, at any tier. It records the reason and its author and time, and names no future work. It is terminal: nothing about the finding is outstanding while the waiver holds. *Waived* replaces *discarded* as the disposition's name; the inbox's `discard` route keeps its name, since an inbox item is not a finding.
- **Deferred.** Put off, with its point of return recorded. A question is deferred when it is resolved with the trigger that reopens it, in Resolved Questions (§spec-requirements); one put off with no trigger is still open. A finding is deferred by a commitment to address it later, which names at least one work reference — the pipeline work the routing tree chose (a task, a scenario, a spec body edit, a new spec, or an amendment to a rule file the project owns), an external work-item ID, or both — and may carry a reason. A deferred finding is outstanding work. *Deferred* replaces *routed* as the disposition's name; *routing* stays the name of the act of choosing where the work goes.

## Behavior

- The constitution defines both terms once. Every other artifact that uses either term for a finding points at that definition.
- §design-principles stops counting a waived finding as a record the pipeline surfaces again: a waiver is the *decided out of scope, with its reason* disposition.
- A finding takes one of three dispositions: fixed, deferred or waived. The `dispositions:` map in `review.md` and `analysis.md` counts `deferred` where it counted `routed`, and `waived` where it counted `discarded`. A stored decision whose outcome is `routed` is read as `deferred`, and one whose outcome is `discarded` as `waived`, so a record written before this change still parses and its decisions still apply. The `waivers:` and `decisions:` lists keep their shape and their homes.
- A gating finding — a review MUST or SHOULD violation, or an analyze hard-fail or blocking finding — can be deferred as well as fixed; a review violation can also be waived, and whether a gating analyze finding can be waived stays with 073. A gating finding's deferral releases the spec's pre-`done` gate only when it names pipeline work on **another** spec, whose own gate then holds the work. Deferred to a task on its own spec, it holds `done` until that task is checked. Deferred only to an external work item, it releases nothing: that deferral is a valid record, and it settles a non-gating finding, but a gating one still blocks.
- A deferral retires as resolved when its finding stops firing. When the pipeline work it names is complete — its task checked, or the spec it names at `done` — while the finding still fires, the deferral **lapses**: the finding needs a new disposition, and a gating one blocks again. A deferral naming only an external work item retires only when its finding stops firing, since the pipeline never reads the tracker. A run that could not evaluate a finding's source retains its deferral.
- An external work-item ID is free text: non-empty, one line, and stored exactly as given. The pipeline checks no format and never reads the tracker.
- A record that holds a deferral holds each work reference it was given exactly as supplied. A record that holds a waiver holds a reason and no work reference.
- `/{project}:review` and `/{project}:analyze` report deferred findings apart from waived ones, so a run with a deferral cannot be mistaken for a run with a waiver (§design-principles: work that is not complete is never indistinguishable from work that is).

## Edge Cases

- **Both references.** A deferral naming pipeline work and an external work item follows the pipeline work: it lapses when that work is complete while the finding still fires. The external ID is carried, never read.
- **A reference that does not exist.** A deferral naming a task or spec that does not exist when it is recorded is refused, naming the reference. One whose named task or spec has since gone — a task pruned or reset out of `tasks.md`, a spec retired or folded — counts as complete, so a still-firing finding lapses rather than pointing at nothing.
- **A rule amendment.** A deferral to an amendment of a rule file the project owns is complete once the amendment is written.
- **A `done` destination.** Deferring to pipeline work on a `done` spec reopens it `done → in-progress`, named in the confirmation before it happens, as routing to one does (058).
- **One disposition at a time.** Recording a waiver for a deferred finding, or a deferral for a waived one, replaces the earlier disposition; a record never holds both for one finding.
- **An empty or multi-line ID.** An empty external ID counts as no reference. One spanning more than one line is refused, naming it.
- **No operator.** Under `ductus exec`, a finding whose deferral lapsed is recorded undispositioned, and a gating one blocks, as any finding no stored decision matches is.

## Acceptance Criteria

- [ ] AC1: The constitution defines *waived* as a decision not to address a finding, recorded with its reason, and *deferred* as a finding to be addressed later, identified by the pipeline work the routing tree chose, an external work-item ID, or both.
- [ ] AC9: The constitution's definition of *deferred* covers a question resolved with its trigger as well as a finding, and §spec-requirements' rule for a deferred question points at that definition.
- [ ] AC2: §design-principles no longer lists a waived finding among the records the pipeline surfaces again.
- [ ] AC3: A deferral that names neither pipeline work nor an external work-item ID is refused, and the refusal names the missing reference.
- [ ] AC4: A recorded deferral carries each work reference it was given, the external ID byte-identical to its input, and a recorded waiver carries a reason and no work reference.
- [ ] AC10: An external work-item ID is accepted as any non-empty one-line text, with no format check. An empty ID counts as no reference, and one spanning more than one line is refused, naming it.
- [ ] AC11: A deferral naming a task or spec that does not exist when it is recorded is refused, naming the reference. A deferral whose named task or spec has since been removed counts as complete.
- [ ] AC12: `/{project}:review`, `/{project}:analyze` and `/{project}:implement` each accept an external work-item ID on a deferral, alone or beside pipeline work.
- [ ] AC13: A finding's record holds one disposition at a time: recording a waiver for a deferred finding, or a deferral for a waived one, replaces the earlier disposition.
- [ ] AC5: `/{project}:review`'s and `/{project}:analyze`'s reports and records list deferred findings apart from waived ones.
- [ ] AC6: `framework/reference/review.md` no longer suggests a work-reference field such as `ticket` on a waiver.
- [ ] AC7: A search of the live artifacts for *waive* and *defer* applied to a finding returns only uses in the defined sense, outside Resolved Questions, signposts and git history.
- [ ] AC14: `review.md`'s and `analysis.md`'s `dispositions:` map carries `deferred` in place of `routed` and `waived` in place of `discarded`; a stored decision with `outcome: routed` is counted as deferred and one with `outcome: discarded` as waived; and a record written before this change parses without error.
- [ ] AC15: `/{project}:review` offers to defer a MUST or SHOULD violation, beside fixing and waiving it.
- [ ] AC16: `check-review-gate` does not block `in-progress → done` on a gating finding deferred to pipeline work on another spec. It blocks on a gating finding deferred to an unchecked task on the same spec, and on one deferred only to an external work item, naming the finding and its work reference.
- [ ] AC17: A deferral whose finding no longer fires is pruned from the record. One whose named task is checked, or whose named spec is `done`, while its finding still fires lapses: the run reports the finding as needing a new disposition, and as blocking when it is gating. One naming only an external work item is pruned only when its finding stops firing.
- [ ] AC18: No live artifact names a finding's disposition *routed* or *discarded* outside Resolved Questions, signposts, git history and the code that reads a record written before this change.
- [ ] AC8: Each spec in `cross-spec-impact:` carries a blockquote signpost linking back to this spec with the contract change it owes, and `check-review-gate` reports the impact discharged.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Is *deferred* a fourth disposition, or a form of *routed*?** *Deferred* replaces *routed*: one disposition for work to be addressed later, whose record names the pipeline work the routing tree chose (any destination *routed* had), an external work-item ID, or both. *Routing* stays the name of the act of choosing the destination. A fourth disposition was rejected because a deferral naming a task and a route to a task would be one act under two names; keeping *routed* with an added ID field was rejected because it leaves *deferred* undefined. A record written before the change keeps parsing, its `outcome: routed` read as `deferred`.
- **Can a spec reach `done` with a deferred gating finding?** Only when the deferral names pipeline work on another spec, whose gate then holds it. Deferred to a task on its own spec, the finding holds `done` until the task is checked; deferred only to an external work item, it releases nothing, because §design-principles requires outstanding work to be recorded where the pipeline surfaces it again and no gate reads an external tracker. Review MUST and SHOULD violations gain deferral beside fix and waive. Never releasing the gate was rejected because, once a waiver means *will not fix*, it left no truthful *fix later* path; releasing it on any deferral was rejected because it would close a spec over work only an external tracker holds.
- **Are *waived* and *discarded* one term?** Yes: *waived* names every decision not to address a finding, at any tier, and replaces *discarded*. Both already recorded a reason, an author and a time, and both lapsed when their finding stopped firing, so a split by tier added a word without a distinction. A record written before the change keeps parsing, its `outcome: discarded` read as `waived`. Whether a gating analyze finding can be waived stays with 073.
- **What retires a deferral?** Its finding no longer firing retires it as resolved, as a stored decision is pruned today. Its named pipeline work being complete — the task checked, or the named spec `done` — while the finding still fires makes it lapse, so the finding needs a new disposition and a gating one blocks again; the runtime reads completeness from the checkbox or the status. A deferral naming only an external work item retires only when its finding stops firing. Retiring on the finding alone was rejected because a deferral would then outlive the work it names and, for a gating finding deferred to its own spec's task, release the gate over a standing violation.
- **Does *deferred* keep its meaning for a question resolved with a trigger?** Yes, under one definition: *deferred* means put off with its point of return recorded — a finding's work reference, a question's trigger. Every existing use for a question already records its trigger (046 AC25), so only the constitution's §spec-requirements line changes, to point at the definition. Two separate definitions of one word were rejected, and renaming the question meaning was rejected because it rewrites framework, runtime and a `done` spec's text for no distinction the shared definition does not already draw.
- **Is an external work-item ID free text, or checked against a configured pattern?** Free text: non-empty, one line, stored byte-identical; an empty ID counts as no reference. The pipeline never reads the tracker, so a pattern could check only an ID's shape, never that the item exists — a check that would look like validation it cannot perform. This follows 051's branch identifier, an opaque token because trackers disagree on form. A team wanting a format check adds one in its own CI over the open-schema record. A configurable pattern and a required one were rejected for that reason; a required one would also make a project with no tracker invent a pattern to defer at all.
