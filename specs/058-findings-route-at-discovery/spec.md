---
status: in-progress
dependencies: [008-security-rules, 020-code-review, 022-deterministic-runtime, 047-analyze-findings-durability, 050-constitution, 057-analyze-artifact-and-record-relocation]
cross-spec-impact:
  - 008-security-rules
  - 020-code-review
  - 022-deterministic-runtime
  - 047-analyze-findings-durability
  - 050-constitution
  - 054-remove-supersession
  - 057-analyze-artifact-and-record-relocation
next-criterion: 34
---

# 058 — Findings route at discovery

Every finding `/{project}:review`, `/{project}:analyze`, or
`/{project}:implement` produces gets exactly one disposition before its spec can
reach `done`: fixed, routed to an artifact the `done` gate reads, or discarded
with its reason recorded. `specs/inbox.md` stops being a place findings are
sent. It holds only the todos a person captures with `/{project}:log`.

## Motivation

Four automated writers appended findings to `specs/inbox.md`:

- `/{project}:review` wrote every **observation** straight through to the
  inbox. `write-review` performed the append itself, so no review could record
  an observation without also capturing it.
- `/{project}:analyze` captured every finding still live at the end of a run
  ([047 — analyze findings durability](../047-analyze-findings-durability/spec.md)).
  That included hard-fail and blocking findings already recorded in
  `analysis.md`, and advisory findings recorded nowhere else.
- `/{project}:implement` captured every incidental issue that fell outside the
  spec being implemented.
- `/ductus`'s brownfield security audit wrote one item per rule gap it found in
  legacy specs at adoption
  ([008 — security rules](../008-security-rules/spec.md)).

The inbox was the one destination the pre-`done` gate never read.
`check-review-gate` blocked on unchecked tasks, unresolved scenario questions,
an undischarged fold or cross-spec impact, and a missing, blocking, or stale
review or analysis record. The inbox reached the operator only as a standing
count at completion, and that count was a notice by design, never a gate. So a
spec could reach `done` while findings that belonged to it sat in the inbox,
indistinguishable on disk from a spec with nothing outstanding. The operator
kept finding them after the transition. The costliest case was `055`: an
obligation owed to `050` was moved to the inbox at 055's completion gate, and
`ductus-v0.47.0` was published before anyone noticed it, as the
[`AGENTS.md`](../../AGENTS.md) entry *`done` is not the same as discharged*
records.

The volume came from the machines. On 2026-09-13, 20 items stood open, and 14
of them had been captured by `/{project}:review` or `/{project}:analyze` passes,
as the [`AGENTS.md`](../../AGENTS.md) entry *The inbox is a queue to drain*
records. A person's `/{project}:log` entries were a minority of their own backlog.

The constitution sanctioned the gap. §design-principles listed "an inbox item"
among the ways to *record [outstanding work] where the pipeline will surface it
again*. The inbox surfaced its items only as a count that never blocked, so
recording work there satisfied the rule's wording while defeating its purpose:
the work was not complete, and nothing kept it from looking complete.

## Behavior

### Three dispositions

Each finding below gets exactly one disposition:

- a `/{project}:review` observation;
- a `/{project}:analyze` finding still live when detection ends, in any tier;
- a `/{project}:implement` incidental issue outside the task in hand.

The three dispositions:

1. **Fixed.** A chore (mechanical, and adding no durable requirement, per the
   §bug-handling durability test) is fixed in the run. For `/{project}:analyze`
   this includes a mechanical edit to an artifact it audited.
2. **Routed.** The finding is written to an artifact the pre-`done` gate reads,
   chosen by the groom decision tree. `groom.md` stays its single canonical
   statement, and each command references it rather than restating it:
   - **The spec in hand:** a task on its `tasks.md`, a scenario, or a body edit.
     If it is `done` — a review or analyze run can examine a finished spec — it
     is reopened `done → in-progress` as below, so the routed work does not sit
     unchecked on a spec no gate reads.
   - **Another existing spec:** a scenario or body edit on that spec. If it is
     `done`, it is reopened `done → in-progress`, and the confirmation names the
     reopen before it happens, as `/{project}:groom`'s does.
   - **No covering spec:** the run offers to create one through
     `/{project}:specify`'s procedure, with its routing and creation
     confirmations. A created spec is the route. If the operator declines, the
     finding is discarded with a reason or left undispositioned; it is never
     counted as routed to a spec that does not exist.
   - **A rule the project owns:** the covering rule file is amended. The
     project owns a rule file it authored, or one it has pinned in its config.
     A rule file `/ductus` manages and the project has not pinned is
     overwritten on the next update, so the finding is discarded with that
     reason.
3. **Discarded with its reason.** The finding and the reason it is out of scope
   are recorded in the run's own record: `review.md` for a review,
   `analysis.md` for an analysis, and the checked-off disposition task for an
   implementation run.

An `/{project}:implement` finding outside the spec is recorded in the run that
found it, as a disposition task, and dispositioned when that task is worked
(see §`/{project}:implement`). Review and analyze findings are dispositioned
in the run that found them.

The inbox is not a disposition. No command appends to it automatically.

Findings whose record already gates `done` keep that gate: a review MUST or
SHOULD violation, and an analyze hard-fail or blocking finding. What changes is
that none of them is also copied to the inbox, and that `/{project}:analyze`
may now fix or route them in the same run.

### Order within a run

Detection runs first and writes nothing. Dispositions follow; each fix or route
is confirmed with the operator before it writes. If any disposition wrote,
detection runs again over what changed. Only then is the run's record written,
and only then is the report rendered.

The record therefore describes the state after dispositions, and a run does not
make its own record stale. A routed scenario on the spec in hand still marks
the review stale, because the review digest covers `scenarios/*.md`. That is
intended: the scenario is new, unimplemented work, and its task holds the spec
out of `done` until it is implemented and reviewed.

`analysis.md` records what the re-check found, not which kind of write
produced it. A finding the re-check no longer produces is recorded as fixed
(`live: false`), whether a chore removed it or a confirmed route did. A body
edit on the spec in hand that resolves the finding is the common route of
that kind. A finding is recorded as routed only while it still fires, because
its routed work has not landed yet: an open task, a scenario, a spec created
for it. That is also the only state a stored decision exists for, since a
finding that no longer fires has nothing to match against. `write-analysis`
refuses any outcome but fixed for a finding gone from the re-check.

### `/{project}:analyze`

Detection stays read-only. The read-only promise is narrowed to detection:
after detection, a **fix-and-route step** proposes a disposition for each live
finding and writes each fix or route only after `gate-confirm`. When the
operator declines a proposal, the finding is discarded with the operator's
reason or left undispositioned. The re-check follows the step, so `analysis.md`
records the tier counts after dispositions.

- **It runs on every invocation, with no flag.** Every write it makes is
  confirmed first, so running by default adds no unasked-for write. A flag
  would make "record everything undispositioned" the default, and every
  unflagged run would then leave `done` blocked.
- **`--fix` keeps its meaning and gains a third trigger.** It still reverts a
  drifted `done` spec to `in-progress` and never edits content. A `done` spec
  whose `analysis.md` records undispositioned findings is now drift, beside the
  existing review-state and scenario-question triggers. It is reported on every
  run and reverted only with `--fix`, matching how an outstanding SHOULD on a
  `done` spec is already handled.
- **With no operator, it writes nothing.** Under `ductus exec`, or on any host
  that cannot confirm, the step proposes nothing. Each live finding that
  matches no stored decision is recorded as undispositioned, so the record stays
  honest and the gate blocks `done` until an interactive run dispositions it.
  Under `ductus exec` that is every live finding: the walker itemizes no
  finding, so none is matched against the stored decisions either, and every
  stored decision is retained for the next interactive run.
- **`--all` groups proposals by spec.** Each prompt offers to leave the rest of
  that spec's findings undispositioned, so a corpus-wide run is not dozens of
  single-finding confirmations.

A newly introduced advisory check that fires across `done` specs shows up as
drift on each of them until its findings are dispositioned. That is intended:
either the specs are not finished, or the check is noise, and one stored
decision per finding quiets it until the finding's message changes.

### Undispositioned findings block `done`

A finding left with no disposition is counted in its run's record, and
`check-review-gate` blocks `in-progress → done` while that count is above zero
in either `review.md` or `analysis.md`. The block asks for a decision, not a
fix: discarding a false positive with that reason clears it. So the block does
not promote analyze's advisory checks to blocking in the sense their promotion
criteria guard against. An advisory finding still never has to be *fixed* to
reach `done`; it has to be dispositioned. The gate reports undispositioned
findings after every existing review and analyze check, because each of those
names a more upstream defect.

### Decisions persist across runs

Detection is stateless, so a finding decided in one run fires again in the
next. That holds for a routed finding as much as a discarded one: a finding
routed to a scenario or task keeps firing until the routed work lands. Without
a persisted decision, every re-run would ask again for decisions already made,
and a routed finding would come back undispositioned and block `done` again.
Each **routed** or **discarded** decision is therefore stored in the record of
the command that made it. A fixed finding needs no entry, because it stops
firing.

- **`analysis.md`** stores analyze decisions, keyed on `{family} — {message}`.
  That is the deterministic part of the key the inbox capture used. The
  capture key's leading `{category}` was assigned by the host when it wrote the
  bullet, so it does not reproduce across runs and is left out. Many messages
  are worded by the host too, so the host matches each new finding to the
  stored decision that describes the same issue, as it does for review
  observations
  ([analyze-findings-match-decisions-by-host-judgment](scenarios/analyze-findings-match-decisions-by-host-judgment.md)).
- **`review.md`** stores observation decisions beside its existing waivers,
  keyed on the observation's text and path. Observation text is the reviewer's
  own wording, so the host matches a new observation against the stored
  decisions and supplies the matching key, rather than relying on the text
  reproducing byte for byte.

Each stored decision carries its outcome, its target (for a route) or reason
(for a discard), when it was made, and who made it, matching the review waiver
shape. A later run that produces a finding matching a stored decision counts
it under the stored outcome without asking. A stored decision whose finding no
longer fires is pruned on the next run, as an expired review waiver is. For a
routed finding, that is the moment the routed work lands. A run that did not
evaluate a finding's source retains the decision rather than pruning it, as a
dimension-restricted review retains a waiver. A reworded finding that the
host does not match to its stored decision is a new finding and is asked about
again.

### `/{project}:review`

Observations are no longer written through to the inbox. Each observation is
fixed, routed, or discarded with its reason, and the Observations section of
`review.md` records the disposition beside the observation. A chore fix is
confirmed, and the affected passes re-run before the record is written, as
`--fix` does today. The **Captured issues** section, and the window diff over
the inbox that fed it, are retired.

An observation about the pipeline's own machinery, rather than the code under
review, is the case most likely to feed a loop of reopens. Routing it to a
`done` spec reopens that spec, whose next review produces the next
observation. The discard route, with its reason, is the disposition that ends
the loop.

### `/{project}:implement`

An incidental issue inside the task in hand is still fixed as part of the task.
Anything else becomes an unchecked task on the targeted spec's `tasks.md` the
moment it surfaces, through `append-task`:

- **Inside the targeted spec:** the task implements the fix, as today.
- **Outside the spec:** a **disposition task**, titled
  `Disposition out-of-spec finding: {summary}`, with the path and detail as
  its body. It is no longer captured to the inbox.

`/{project}:implement` works a disposition task like any other. It fixes the
finding if it is a chore, routes it with confirmation, or discards it, writing
the reason onto the task as it is checked off. The completion summary lists
each disposition task the run appended and each one it worked, with its
outcome.

This keeps each part of the old capture contract:

- **Nothing is lost.** The task is on disk as soon as the finding surfaces, so
  an interrupted run loses nothing.
- **Nothing is ungated.** An unchecked task already holds the spec out of
  `done`.
- **Nothing derails the task in hand.** Recording is one append, and the
  disposition happens when the task is worked.

A disposition task is a work item, not the finding's record. Where a finding is
routed, its lasting record is the scenario or spec edit it lands in. So
`tasks.md` still does not become a second capture queue. The cost is that a
finding unrelated to the spec holds that spec out of `done` until it is
dispositioned. That is intended: the spec's work surfaced it.

### `/ductus` adoption security audit

The audit reports its findings and writes no inbox item. Each finding names its
spec. `/{project}:analyze` applies the same rule Verification triggers to a
spec whenever it runs, so the gap resurfaces when that spec is next touched.
The accepted cost: a legacy spec nobody touches is never re-checked. That is
the brownfield stance of letting adoption spread through the areas being
worked on. An operator who wants a specific finding tracked sooner can `/log`
it.

### The inbox

`specs/inbox.md` holds the todos a person captures with `/{project}:log`.
`/{project}:groom` walks them through its five routes, unchanged, and remains
the only command that removes an item. `/{project}:status` renders the
standing inbox row — the outstanding count and the oldest item's date — on
every run, including when the inbox is clean. Its four states are unchanged
from the row `/{project}:review` and `/{project}:implement` rendered before,
and it moves because a count of personal todos has no bearing on the spec a
review or implementation run is working on. The inbox template documents the manual
entry form only.

The brownfield inbox described in `006-bug-workflow` and
`011-brownfield-process` was always manual capture, and it survives unchanged.
Items captured automatically before this change stay in an adopter's inbox
until `/{project}:groom` walks them like any other item. No migration rewrites
them.

### Run records

`review.md` and `analysis.md` each drop `captured-issues` and carry the same
two new fields:

- **`dispositions:`** is a map of four counts: `fixed`, `routed`, `discarded`,
  and `undispositioned`. In `analysis.md` it counts the run's findings; in
  `review.md` it counts observations, since MUST and SHOULD violations keep
  their own counts. The map's name says what is being counted, so a `fixed`
  count beside `must-violations` cannot be read as violations fixed. The gate
  reads `dispositions.undispositioned`.
- **`decisions:`** is the list of stored routed and discarded decisions
  (§Decisions persist across runs), beside `review.md`'s existing `waivers:`.

A run that found five findings and dispositioned fewer is visible in the
record. `captured-issues` existed for that same reason, and it measured the
wrong thing once the inbox stopped being the destination.

No migration rewrites existing records. Neither record rejects an unknown key,
so a record written before this change still parses with its `captured-issues`
field, which is dropped the next time the record is rewritten. A record
**without** a `dispositions:` map predates dispositions, and absence is not
zero:

- **On an `in-progress` spec,** `check-review-gate` blocks and names the
  command whose record predates dispositions, so that command is re-run. There
  is no exemption, matching the gate's standing refusal of a grandfather
  clause: the record is always writable at the moment a spec is being
  completed.
- **On a `done` spec,** it is not drift. This follows the bounded exemption
  the CI template's record check gives a `done` spec that predates a record.

Reading absence as zero would pass the case this spec exists for: an old run
whose findings went to the inbox would reach `done` with none of them seen.

Each report body lists
every finding with its disposition and target: the fixed file, the artifact it
was routed to, or the discard reason.
[057 — analyze artifact and record relocation](../057-analyze-artifact-and-record-relocation/spec.md)
defines the `analysis.md` skeleton, and
[020 — code review](../020-code-review/spec.md) defines `review.md`'s.

### Runtime

The runtime work lands in
[022 — deterministic runtime](../022-deterministic-runtime/spec.md)'s
primitives:

- `write-review` stops appending observations to the inbox and records their
  dispositions instead.
- `compute-review-scope` and `diff-cross-spec` stop computing an inbox-additions
  window.
- The standing inbox row leaves the `/{project}:review` and
  `/{project}:implement` output and moves to `dashboard`, which renders it in
  `/{project}:status` (see §The inbox).
- `write-analysis` renders the disposition sections in place of captured issues.
- `append-inbox` and `remove-inbox-item` remain, serving `/{project}:log` and
  `/{project}:groom`.

### Constitution

- §design-principles no longer lists an inbox item among the dispositions for
  known-outstanding work.
- §grounding routes an assumption made during implementation to a task or an
  open question on the spec in hand.
- §bug-handling's chore paragraph says a chore found by a run is fixed in that
  run, and a chore logged by hand lives in the inbox until it is done.
- §brownfield-inbox describes the inbox as manual capture, and its Automatic
  issue capture subsection is replaced by the three-disposition rule.
- The Frontmatter Schema declares the disposition counts in place of
  `captured-issues`.

[050 — constitution](../050-constitution/spec.md)'s `findings-route-by-scope`
scenario carries two edge cases this reverses: a chore found during spec work
went to the inbox, and so did a finding made while no spec was in progress.

### Cross-spec impact

Each spec named in `cross-spec-impact:` gets a signpost linking back here,
written as a blockquote. The dependency generator skips blockquote lines, so
the back-link adds no `dependencies:` edge that would close a cycle with this
spec's links to it. Each also gets the contract change itself:

- **047:** its premise that analyze findings persist to the inbox, and the
  analyze-state drift family it added to `check-artifacts`, which `/{project}:analyze`
  now judges from the record it writes.
- **022:** the `review-observations-write-through`, `the-inbox-row`, and
  `the-analyze-record-states-what-it-captured` scenarios, plus any other 022
  scenario that names the inbox-additions window; and the `check-artifacts`
  registry, which gains `disposition-drift` and loses `analyze-state-drift`.
- **020:** `review.md`'s `captured-issues` field and its Captured issues section.
- **057:** `analysis.md`'s captured-issues section and field.
- **050:** the `findings-route-by-scope` edge cases above.
- **054:** AC10's `check-artifacts` family count, nine again with a different
  membership.
- **008:** the adoption audit's inbox write.

## Edge Cases

- **A finding that already gates `done` cannot be discarded.** An analyze
  hard-fail or blocking finding is fixed or routed. It keeps blocking until the
  re-check no longer produces it, because discarding it would add a bypass the
  gate does not have today. The discard route applies to advisory findings,
  review observations, and implementation disposition tasks. Review MUST and
  SHOULD violations keep their existing fix-or-waive model.
- **A finding that spans the spec in hand and another spec is split.** The part
  the spec in hand owns becomes a task or scenario on it, and the rest is
  routed on its own.
- **A chore fix that fails, or turns out not to be mechanical, is not a
  chore.** The run reverts the attempt and proposes a route or a discard
  instead. A chore is never counted as fixed while its fix is not on disk.
- **A confirmed route whose write fails is undispositioned.** For example,
  `create-scenario` refuses a slug that already exists. The failure is reported
  with the finding, and the record never counts a route that did not land.
- **Two findings with the same key in one run get one disposition.** A
  finding's key is the stored `decision-key` the host matched it to, or else
  its own: `{family} — {message}` for an analyze finding, and the text and
  path for a review observation.
- **A route to a spec that is not `done` does not reopen it.** It follows
  `/{project}:amend`'s rules for that status, including amend's own back-edge
  when a body edit lands in a `clarified` or `planned` spec.
- **The target spec changed status between proposal and write.** The reopen's
  `from: done` guard surfaces the discrepancy instead of overwriting, as
  `/{project}:groom`'s reopen does.
- **An observation on a review whose scope is empty is still dispositioned.**
  The reviewer's judgment is the input, not the diff, as it was when
  observations were captured.
- **A `decisions:` list that does not parse is reported, never treated as
  empty.** `validate-frontmatter` names it. Until it is repaired, the run asks
  about each finding again, so a malformed list can cost a repeat question but
  never silently waive a finding.
- **`/{project}:prune`'s default mode never drops an unchecked disposition
  task.** It removes only spent sections, so an undispositioned finding keeps
  holding its spec out of `done`. `--reset` does drop unchecked tasks, but it
  is refused below `done` without `--force`, and a `done` spec cannot hold an
  unchecked task. A forced reset is the operator discarding the work
  deliberately.
- **An adopter inbox keeps its original guidance comment.** The inbox is
  installed once and never updated, so the header in an existing adopter's
  inbox still describes automatic capture. A registry migration replaces that
  comment block with the new template's, and the introduction too when it is
  the old template's text unchanged, and leaves every item untouched.

## Acceptance Criteria

- [x] AC1: `/{project}:log` is the only framework command that invokes `append-inbox`, and no runtime primitive other than `append-inbox` writes an item to `{specs-root}/inbox.md`. A search of `framework/commands/`, `framework/bootstrap/`, and `runtime/src/` finds no other writer.
- [x] AC2: A `/{project}:review` run with one or more observations appends nothing to the inbox, and `review.md`'s Observations section records each observation with its disposition (fixed, routed with its target artifact, or discarded with its reason).
- [x] AC3: `/{project}:analyze`'s detection steps write no file. A fix-and-route step after detection proposes one disposition per live finding and performs each fix or route only after `gate-confirm` returns a confirmed decision.
- [x] AC4: After the fix-and-route step writes anything, `/{project}:analyze` re-runs detection and writes `analysis.md` from the re-check. `check-review-gate` run immediately afterwards does not report that analysis stale.
- [x] AC5: A `/{project}:implement` run that surfaces an issue outside the targeted spec captures nothing to the inbox. It appends an unchecked `Disposition out-of-spec finding:` task to the targeted spec's `tasks.md` before continuing the task in hand, and its completion summary lists the appended task.
- [x] AC6: `review.md`, `analyze.md`, and `implement.md` route findings by referencing the Groom decision tree in `groom.md` rather than restating it, as `specify.md` already does.
- [x] AC7: A finding routed to an existing `done` spec names the `done → in-progress` reopen in its confirmation prompt, and performs the reopen through `set-status` with `from: done`.
- [x] AC8: A finding with no covering spec is routed only when the operator creates a spec through `/{project}:specify`'s procedure in the same run. A declined creation leaves the finding discarded with a reason or undispositioned, and the record never counts it as routed.
- [x] AC9: `review.md` and `analysis.md` frontmatter carry a `dispositions:` map (`fixed`, `routed`, `discarded`, `undispositioned`) and a `decisions:` list in place of `captured-issues`. The constitution's Frontmatter Schema declares each field and its type, and `validate-frontmatter` validates them.
- [x] AC10: The Captured issues section is removed from `review.md`'s and `analysis.md`'s skeletons. `compute-review-scope` no longer returns its `captured-issues` window, and `diff-cross-spec` no longer returns `inbox-additions` or `inbox-standing`.
- [x] AC11: The standing inbox row no longer appears in `/{project}:review` or `/{project}:implement` output. `/{project}:status` renders it from `dashboard` on every run, in the four existing states: outstanding count with the oldest item's date, outstanding with age undeterminable, clean, and no inbox file.
- [x] AC12: The `/ductus` brownfield security audit writes no inbox item. It reports each finding with its spec and points to `/{project}:analyze`.
- [x] AC13: The constitution's §design-principles no longer lists an inbox item as a disposition for known-outstanding work. §grounding routes an implementation-time assumption to a task or open question. §bug-handling's chore paragraph states that a chore found by a run is fixed in that run. §brownfield-inbox describes the inbox as manual capture, with Automatic issue capture replaced by the three-disposition rule.
- [x] AC14: `/{project}:amend`'s chore branch fixes the chore rather than directing the user to `/{project}:log`.
- [x] AC15: The inbox template's header, and this repository's `specs/inbox.md` header, document the manual entry form only; the auto-capture and audit-finding forms are gone.
- [x] AC16: `/{project}:log` and `/{project}:groom` behave as before. An item captured automatically before this change is walked by `/{project}:groom` through the same five routes as a logged one.
- [x] AC17: `/{project}:help` describes the inbox as the place for manually captured todos.
- [x] AC18: Each spec in `cross-spec-impact:` carries a blockquote signpost linking back to this spec, together with the contract change it owes, and `check-review-gate` reports the impact discharged.
- [x] AC19: `docs/analyze.md`, `docs/slash-commands.md`, and `README.md` describe the new behavior, and none of them states that review, analyze, or implement writes to the inbox.
- [x] AC20: `AGENTS.md` entries that describe automatic inbox capture as current behavior are updated or retired in the same change.
- [x] AC21: The generated per-agent command mirrors match their updated sources, and the command-parity audit passes.
- [x] AC22: The change adds one `framework/migrations.toml` entry, with its `framework/migrations/{id}.md` procedure. The procedure replaces the guidance comment in an adopter's existing inbox with the new template's, leaves every item byte-identical, and is idempotent. The migration-coverage audit (Family 10) passes.
- [x] AC23: A repo-wide search for the automatic-capture shape — `captured during` bullets, `inbox-additions`, a `Captured issues` section, or an `append-inbox` call outside `/{project}:log` — returns no hit outside this spec, Resolved Questions sections, signposts, and git history.
- [x] AC24: Full markdown lint passes across every file the change touches.
- [x] AC25: `check-review-gate` blocks `in-progress → done` while `review.md` or `analysis.md` records one or more undispositioned findings, names the count and the command that dispositions them, and checks this only after every existing review and analyze check has passed.
- [x] AC26: `/{project}:analyze` runs its fix-and-route step on every invocation without a flag. Under `ductus exec` it writes no fix or route, and records each live finding that matches no stored decision as undispositioned.
- [x] AC27: `/{project}:analyze` reports a `done` spec whose `analysis.md` or `review.md` records undispositioned findings as drift, and with `--fix` reverts it `done → in-progress` through `set-status` with `from: done`.
- [x] AC28: `/{project}:analyze --all` groups fix-and-route proposals by spec, and each prompt offers to leave that spec's remaining findings undispositioned.
- [x] AC29: `check-review-gate` blocks an `in-progress` spec whose `review.md` or `analysis.md` has no `dispositions:` map, and names the command to re-run. A record written before this change that carries `captured-issues` still parses, and `/{project}:analyze` does not report a `done` spec's map-less record as drift.
- [x] AC30: `/{project}:analyze` offers no discard for a hard-fail or blocking finding. Such a finding is fixed or routed, and it keeps `blocking: true` in the record until the re-check no longer produces it.
- [x] AC31: A confirmed disposition whose write fails (a refused scenario slug, a chore fix that fails or proves not mechanical) is counted as undispositioned, never as fixed or routed, and the report names the failure beside the finding.
- [x] AC32: A routed or discarded decision made in one run is stored with its outcome, its target or reason, its time, and its author in the record of the command that made it. A re-run over an unchanged tree counts the finding under its stored outcome without prompting. A re-run in which the finding no longer fires prunes the stored decision, and a run that did not evaluate the finding's source retains it.
- [x] AC33: `/{project}:implement` working a disposition task fixes, routes with confirmation, or discards the finding, and checks the task off only once one of the three has happened. A discard writes its reason onto the checked task.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does an undispositioned finding block `done`?** Yes. `check-review-gate`
  blocks while either record carries an undispositioned finding, and
  decisions persist across runs (see §Undispositioned findings block `done` and
  §Decisions persist across runs). Clarify persisted discards only. Planning
  generalized that to routed decisions too, because a routed finding keeps
  firing until its routed work lands, and a discard-only list would bring it
  back undispositioned on the next run. Without the block, a declined proposal would
  leave the same residue this spec removes. Without persistence, the block
  would ask again on every re-run, because detection is stateless. Grounded in
  the existing asymmetry: analyze advisories were kept out of the gate because
  gating them "would promote all of them at once, past the criteria they each
  declare" (`runtime/src/schema/primitives.rs`, the `advisory` field's doc).
  That concern is about forcing fixes. A block that a one-line discard clears
  asks for a decision instead, and the review waiver
  (`framework/commands/review.md`, §Waivers) is the persisted-decision shape
  it reuses.
- **How durable is an `/{project}:implement` finding before its
  disposition?** Fully durable: the finding becomes an unchecked disposition
  task on the targeted spec the moment it surfaces, and `/{project}:implement`
  works it like any other task (see §`/{project}:implement`). The alternatives
  lost something. A disposition held until the task boundary is lost when a run
  is interrupted mid-task. Dispositioning the finding the moment it surfaces
  derails the task in hand, which `framework/commands/implement.md`'s
  walk-through step 5 forbids ("record and keep working"). The task route needs
  no new gate, because an unchecked task already blocks `done`. It also
  collapses the scope tiers into one rule: anything outside the task in hand
  becomes a task on the spec in hand.
- **Where does the standing inbox count go?** To `/{project}:status`, rendered
  by `dashboard` in its four existing states, and out of the
  `/{project}:review` and `/{project}:implement` output. The row was added
  because items older than the feature in hand were invisible, and one of them
  was an obligation a release was cut over (`AGENTS.md`, the "`done` is not the
  same as *discharged*" entry). With no finding able to reach the inbox, that
  motivation is gone, and what remains is a count of logged todos. Those belong
  in the view of what is outstanding across the project, not in a single spec's
  run. Dropping the row entirely would have left a forgotten todo visible only
  to someone who opens the file.
- **How does analyze's fix-and-route step relate to `--fix`?** They stay
  separate. The step runs on every invocation, and `--fix` keeps its existing
  meaning: it reverts a drifted `done` spec and never edits content. It gains
  a third trigger, a `done` spec whose `analysis.md` records undispositioned
  findings. With no operator, the step writes nothing and records findings as
  undispositioned. `--all` groups proposals by spec (see §`/{project}:analyze`).
  Grounded in `framework/commands/analyze.md`: `--fix` already reverts on
  review-state drift, including an outstanding SHOULD on a `done` spec, and on
  unresolved scenario questions. The new trigger follows the SHOULD precedent,
  where an unresolved decision on a finished spec is drift rather than a notice.
- **What are the disposition counts called, and where do they live?** In a
  `dispositions:` map with `fixed`, `routed`, `discarded`, and
  `undispositioned`, identical in both records, with stored decisions in a
  `decisions:` list (see §Run records). No migration rewrites existing records.
  A map-less record blocks an `in-progress` spec until its command re-runs, and
  is not drift on a `done` spec. Grounded in `runtime/src/schema/primitives.rs`,
  where neither record struct denies unknown fields, so an old record still
  parses. The no-exemption stance comes from
  `runtime/src/primitives/check_review_gate.rs`, whose analyze-gate doc refuses
  a grandfather clause because "an exemption here would be a permanent hole".
  The upgrade cost in this repository was measured at zero: all 55 specs were
  `done` and none `in-progress`.
