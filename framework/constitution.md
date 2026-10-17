# Constitution

The governing rules for spec-driven software development. This document defines the principles, workflow, and quality gates that apply to every project regardless of tech stack.

<!-- §principles -->

## Guiding Principles

These are evaluation criteria, not implementation instructions. Use them to identify gaps or violations, not to drive design decisions.

### Technology

- **Secure:** protect sensitive data through industry standards and best practices. See `specs/rules/security-backend.md` and `specs/rules/security-frontend.md` for enforceable rules.
- **Scalable:** design and implement to be dynamically scaled
- **Learnable:** fast onboarding through clear patterns, documentation, and accessible codebase design
- **Reliable:** graceful degradation and automatic recovery when components fail
- **Recordable:** accurate, durable data capture for business metrics, audit trails, and event tracing
- **Supportable:** simple and quick to detect, identify, and resolve issues
- **Automated:** humans only do what computers can't
- **Testable:** design for security, unit, functional, and load testing
- **Consumable:** simple and intuitive interfaces into our systems
- **Verified:** nothing reaches production without validation

### Business

- **Fast:** responsive systems, short time to market, rapid updates and fixes
- **Serviceable:** solutions exist to serve identified needs, not to justify themselves
- **Evolvable:** the business can adapt, grow, and create products and services as needs change
- **Flexible:** customers are served by products and services that fit their varied needs
- **Observable:** clear, real-time visibility into product and service performance
- **Compliant:** meet regulatory, legal, and industry requirements
- **Cost-conscious:** optimize cost across building, operating, and scaling products and services

<!-- §cost-levers -->

### Cost levers

Per-task token tracking and budget ceilings require instrumenting the model call, which [§runtime-boundary](#runtime-boundary) principle 2 forbids the `ductus` runtime to do, so that work belongs to the AI platform. `ductus` offers cost-aware patterns instead: the stuck-detection step in `/{project}:implement` catches runaway loops before they compound spend, and default-off autonomy keeps the human in the loop unless `--auto` is passed. For runtime cost controls, use the platform's tooling — Claude Code's `/cost`, the Anthropic usage dashboard, Cursor's request limits, and equivalents.

<!-- §design-principles -->

### Design principles

Constraints on anything the pipeline asks an author or a check to do. Each is a hard filter, not a tiebreaker: a design that fails one is redesigned or deferred, not shipped with a note. The list carries no count, because a count is the first thing to go stale.

- **A check that cannot run MUST never be indistinguishable from a check that passed.** When a script, gate, or artifact check skips part of its subject, cannot reach it, or has no basis to inspect it, it MUST say so — a distinct result, a `guidance` field, a count of what *was* examined — rather than the zero, empty list, or success string a clean run produces. The failure is silent, and the states where a check cannot run are disproportionately the states where something is wrong. When adding a gate, prove it **fails** — break what it guards and watch it go red — before trusting that it passes. This is `QUAL-CLAIM-001` (see [§rules](#rules)) applied to the pipeline's own machinery.
- **A check that reads git history must be given git history.** CI checkouts default to a single-commit shallow clone, so a check that resolves a recorded sha, diffs against an earlier commit, or walks the log sees nothing in CI while passing on every developer machine. Configure a full-depth checkout in **every** job that runs such a check, with a comment saying why. A local pass is not evidence: the environments differ in exactly the dimension the check depends on.
- **A test that reads history or shells out to a project script is a change to the job that runs it, not only to the suite.** Its job gains an environment requirement (full history) and a new input (whatever it shells out to, which belongs in that workflow's trigger paths). Miss the first and the test cannot run; miss the second and it silently stops running when its input changes. Prefer a test that fails loudly when its inputs are missing over one that skips.
- **Work that is not complete MUST never be indistinguishable from work that is.** When something is known to be outstanding — a defect found at the completion gate, a claim that no longer holds, a quietly narrowed scope, a deferred question — there are exactly three dispositions. **Fix it.** **Record it where the pipeline will surface it again** — a task, a scenario, an acceptance criterion, a review finding waived with its rationale — and let the status follow that record. Or **decide it is out of scope and record the decision with its reason.** The inbox is none of the three, because no gate reads it ([Finding dispositions](#finding-dispositions)). Nor is a caveat beside a completion claim: a status meaning *"done except for what was mentioned at the time"* carries no information once the summary is gone. Where the residue is knowable only by measuring, **measure it** — an unmeasured gap is a task, not a caveat.
- **Never design a pipeline feature that depends on human diligence.** An artifact section, frontmatter field, command behavior, or workflow step that requires an author to *remember* will be skipped, precisely when it mattered most. Ask what happens when an author forgets; if the feature degrades silently, derive the input from existing artifacts, frontmatter, or history, or do not ship it. Deferring is correct when no derivable design exists; shipping the disciplined version "for now" is not.
- **A check whose subject is drawn from repository state can go vacuous, and a naive vacuity guard will then demand that an unhealthy repository exist.** A check over "every artifact in state X" has a subject only while something is in state X, so a guard that fails when nothing was compared fires once the condition stops occurring, and a healthy repository turns it red. **Build the subject** instead: a fixture carrying the condition under test, asserted against a known verdict in both directions, with the vacuity guard placed there. Let the pass over real state keep running, reporting *nothing to compare* as a distinct, non-failing state. The tell is a check whose subject is a filter over repository state and that has been green for months with nobody knowing which item satisfied it.

<!-- §grounding -->

## Grounding

Work from what can be observed, not from what can be guessed. When a question can be answered from a reachable source — the code, the project's own artifacts, a connected dev database, runtime output — the agent MUST consult that source before answering. Reasoning from the conversation alone when a primary source went unread is a defect, even when the guess was right.

Grounding is the working counterpart to the **Verified** principle: that one governs the product, this one governs how the agent reaches every claim along the way, in every pipeline command.

### Sources, in order of authority

1. **Code and artifacts** — source files, `spec.md`, `plan.md`, scenarios, rules, `system.md`, tests, migrations, config, and git history are the ground truth for what the system *is* and what was *decided*. Read the file; do not recall it.
2. **Live, reachable state** — a connected dev or read-only database, a running dev server, logs, a REPL, `--help` output, an actual test run. When such a source is on hand, query it rather than infer schema, data shape, or behavior.
3. **Inference** — permitted only for what no reachable source can answer, and then stated as an assumption, never asserted as fact.

<!-- §governance-precedence -->

### Governance sources, and which governs

The ranking above orders **evidence**, not **governance documents**: one asks what is true, the other what binds. A project may load more than one governing document — the constitution `ductus` ships, and any shared constitution its `.ductus/config.toml` registers under `[constitutions.*]` (spec 055) — and where they disagree, this is the order:

**The framework constitution is a floor.** A shared or project source may add rules and tighten existing ones; it may not loosen them. Where the framework is silent, the more specific source wins: **project > shared > framework**. Every command reads the framework constitution to know how to behave, so a source able to disable one of its gates would make the pipeline's own invariants negotiable per repository.

**Nothing enforces this.** No check compares governing documents for contradiction, and prose contradiction is not mechanically detectable. This is the rule for whoever resolves a disagreement.

### A partial read is not a read

Grounding governs the **completeness** of a consultation as well as whether it happened. A tool that elides content — truncation to a preview, a saved-output pointer, a page or line cap, a `head`/`tail` window, a match-only search — returns a fragment, and the elided remainder is unread: reasoning as though it said nothing is reasoning from a source that went unread.

There are two dispositions, and prose is not a third. **Read the remainder** — follow the pointer, page through the ranges — or **state what was not examined**, in the artifact where a later reader meets the claim. Prefer a reader that reports its subject's size and takes an offset over a command whose output is capped downstream, since a downstream cap says nothing about what it dropped.

This is `QUAL-CLAIM-001` turned on the agent's own reading. A truncated read looks like success, and the subjects most likely to be truncated — a governing document, a long procedure, a wide diff — are the ones where the missed content matters most.

**Nothing enforces this.** No gate intercepts a tool choice; `/{project}:review` and `/{project}:analyze` cite the rule when a claim rests on a partial read.

<!-- §recommendations -->

### Recommendations

A recommendation is a claim, so grounding governs it. When the agent offers options and recommends one, it MUST work the recommendation out to its result **before** presenting it, not after the user accepts.

Where the options differ by a quantity the user cares about — steps removed, restarts saved, files touched, time or cost — that quantity MUST be computed for **every** option and stated with the recommendation. Reasoning from an option's *shape* to its *effect* ("this is the biggest change available") is inference presented as analysis, whether or not the ranking turns out right.

When the deciding quantity cannot be computed yet, the agent MUST say so and say what would settle it, rather than ranking the options anyway; otherwise the user's approval rests on the framing rather than on the facts. Advice about the system is held to the standard the system is held to.

### Rules

- **Prefer the source to the recollection.** Before asserting how code behaves, what a schema holds, what an artifact says, or what a command does, open it — a read, a `grep` or a query costs less than a wrong answer propagated downstream.
- **A connected database is a primary source, not a hazard.** When the project exposes a dev or read-only database or other live state, use it to confirm schema, constraints and representative data rather than theorizing them. Discover what is reachable from the project's configuration and from any live-source pointer its `AGENTS.md` declares; the absence of a pointer is not evidence that no source exists. Read-only access needs no special authorization; writes need the task to authorize them.
- **Run the project's own command — never a hand-rolled equivalent of one.** When the project declares how to build, test, lint, migrate or package — `package.json` scripts, Makefile or justfile targets, compose files, CI steps, a `Commands` section in its `AGENTS.md` — that declaration is the command, because a substitute that succeeds proves the substitute, not the command every contributor and CI job runs. A narrower invocation while iterating is fine; the result reported at the end comes from the project's command. A project command that fails is a finding to fix, record, or decide out of scope ([§design-principles](#design-principles)), never one to route around. When the declared command cannot do what the task needs, deviate and say which command was bypassed and why; when a recurring operation has no declared command, propose adding one, as a task. A declared command authorizes nothing: a deploy, publish or destructive migration still runs only when asked. Nothing enforces this — the substitute runs in a shell and never reaches a diff.
- **Name the assumption when you must infer.** When no reachable source can settle a load-bearing question, mark the claim as an assumption: an Open Question during a spec ([Spec requirements](#spec-requirements)), a labeled assumption in a plan, and a task or open question during implementation, where the `done` gate reads it ([Finding dispositions](#finding-dispositions)).
- **Cite what you consulted.** When a conclusion turned on a specific source, reference it (`path:line`, the query, the command) so the next reader can re-derive it rather than re-guess.
- **A recorded measurement states its method and its units, or it is not re-derivable.** State the denominator, the population and the boundary a count was taken over, since two honest counts of one subject can legitimately differ. When a fresh derivation disagrees with a recorded one, re-derive at the recorded figure's own commit first, which separates decay from a difference in method; when that does not explain the gap, diff the field across its own history, because a correct value can be overwritten with a worse one. A measurement whose method is unstated is an unmeasured gap ([§design-principles](#design-principles)).
- **Before reporting that a declarative entry misbehaved, read the entry.** A registry row, manifest line or config table carries its own preconditions — a sunset date, a strategy, a pinned flag, a surface filter — so when the observation is *"X did not happen"*, first ask whether X was supposed to. State the entry's own gating fields in the finding, or do not file it.
- **Substitute a recorded timestamp or commit into the command that writes it — never transcribe either.** The primitives record what they are handed, so a guessed time or a short hash completed by hand is well-formed, passes every gate, and surfaces only when a later freshness check fails. Substitute both in the invocation that writes the record, then confirm the recorded commit resolves.
- **An enumeration is a claim of its own, and it goes stale while the requirement around it stays true.** A list inside a criterion, or a set spelled out in prose — the routes a command offers, the back-edges the lifecycle has — carries none of the tokens an identifier sweep greps for. Verify the enumeration, not only the requirement; when a set changes, grep for the count word and the list shape, and prefer stating a set without a count.
- **Pick the checks a change must pass from its blast radius, not from its file extension.** The checks that look unrelated are the ones that go red, so when the blast radius is unclear, run everything. Commit before running any check that reads git history, and re-run those checks after committing.
- **After publishing a change, read every check that ran against it — not the one whose name matches what you changed.** Read every result recorded for that commit, and treat a check with no verdict yet as unknown, never as a pass. A release's job set is not automatically a superset of the ordinary ones, so read it on its own terms.
- **Verify what a commit contains before reporting it landed, and pair that with a check for what it left out.** Checks that read the working tree pass when a change is in the tree but not in the commit, so read the commit's file list and the working tree's remaining changes before recording any claim that a commit holds a change. A review or analysis record is judged by a digest of what it read, so it may be recorded before its work is committed.
- **Never pipe a command whose exit status you intend to read.** A pipeline reports its last element's status, so a filtered failing gate reads as success. Redirect the output, read the status on the line that ran the command, then search the file; and count result lines against the number of targets rather than reading the tail.

<!-- §pipeline -->

## Development Pipeline

Every feature follows the pipeline: **spec → plan → tasks → implement**. No code is written without a spec. No implementation begins without a plan.

<!-- §spec-phase -->

### Spec Phase

Define *what* the feature does and *why*. A spec captures requirements, contracts, and constraints without prescribing implementation details.

Each feature lives in a numbered directory under `specs/`:

```text
specs/
  system.md              # Architecture, shared conventions
  events.md              # Global event catalog
  errors.md              # Error handling conventions
  {NNN-feature}/
    spec.md              # Requirements, contracts, acceptance criteria
    research.md          # (optional) Background research, prior art
    plan.md              # Implementation approach, technical decisions
    data-model.md        # (optional) Domain entities and data structures, generated during plan phase
    tasks.md             # Discrete work items derived from the plan
    scenarios/           # (optional) Scenario files elaborating spec sections
      {slug}.md          # One file per scenario
```

The top-level directory name (`specs` above) is the documented default; a project may rename it via `.ductus/config.toml` `[paths] specs-root` (e.g. to avoid colliding with RSpec's `spec/`). When the key is unset, every command and the runtime default to `specs`, and the literal `specs/` throughout this constitution and the command sources is that default.

**This is an instruction, not only a fact.** Wherever a command acts on a path under the spec root, substitute the configured name for the literal `specs/`. The primitives resolve `[paths] specs-root` themselves, so the substitution is the host's on the markdown-only path, for every command, including ones added later. Commands reference this rule rather than restating it ([§drift-prevention](#drift-prevention)).

<!-- §spec-requirements -->

#### Spec requirements

- Every spec includes a **Status** indicator: `draft`, `clarified`, `planned`, `in-progress`, or `done`
- Every spec includes **Acceptance Criteria** — concrete, testable conditions that define "done". Each carries a stable `AC{n}:` label after its checkbox (`- [ ] AC7: …`), assigned by the runtime's labelling pass and permanent: never renumbered, and never reissued after a criterion is deleted. The label, not the criterion's position, is how a criterion is cited in prose, across specs, and by tooling (spec 013)
- Every spec includes **Open Questions** — undecided blockers. A decision deferred pending a condition ("not now; revisit when X lands") is resolved *with* a condition and belongs in Resolved Questions with its trigger recorded
- Every spec lists **Dependencies** — other specs this feature depends on
- Open questions must be resolved before moving to the plan phase
- Specs describe behavior and contracts, not implementation
- **Never hand-write an `AC{n}:` label.** Add the criterion unlabelled and let the labelling pass assign it: the label is `max(highest label in the body, next-criterion)`, so a hand-written label is a guess that can collide with a retired one. If labels were already written by hand, leave them — stripping them renumbers from the advanced counter.
- **A spec body must not carry a point-in-time status section.** A *State at hand-off*, *Current state*, or *Why this spec is still {status}* section narrates state the frontmatter, `tasks.md`, the pipeline view and git history already answer, and nothing updates it when the state moves, so it soon contradicts the spec's own frontmatter. Record an operator decision in **Resolved Questions** with its trigger, and an obligation in `cross-spec-impact:` or a task. Never add one; when you meet one, delete it and re-home whatever it still asserts truly.
- **Write a criterion's path claim so a check can see it: backticked, with an interior slash, and repo-relative.** The family that verifies criterion paths reads only inline code spans, requires an interior slash, and resolves from the repository root, so a path in plain prose, a bare filename, or a spec-relative path is never checked — a deeper silence than a skip. Read a `done` spec's criteria for paths yourself; a short skip list is a fact about backticks, not about claims. State a claim about an **absence** in non-path form, or the criterion manufactures the finding it documents.

<!-- §spec-lifecycle -->

#### Spec lifecycle

| Status | Meaning |
| --- | --- |
| `draft` | Initial spec written, may have unresolved open questions |
| `clarified` | All open questions resolved, acceptance criteria are concrete and testable |
| `planned` | Plan and tasks exist, readiness check passed |
| `in-progress` | Implementation has started |
| `done` | All acceptance criteria verified, code merged, and no scenario under the spec carries unresolved open questions |

```text
draft ──/clarify──▶ clarified ──/plan──▶ planned ──/implement──▶ in-progress ──[/review gate]──▶ done
```

Forward edges only: `/clarify` raises status to `clarified`, `/plan` to `planned`, `/implement` to `in-progress` and then to `done`. The `in-progress → done` transition is gated by `/review`: `/implement` MUST NOT write `status: done` while the review record's `last-run` is unset or its `blocking` is `true`. `/review` writes its findings and that record to `review.md` and changes no status. The gate composes with `/analyze` (which flags drifted `done` specs) and the shipped CI template (which fails PRs that bypass the local checks), per [§design-principles](#design-principles). Three back-edges exist:

- **Backward via new questions** — `clarified` / `planned` / `in-progress` → `draft` when `/amend` records a new open question, in the same write; the next `/clarify` resolves it and the spec advances again. `draft` is the only status that tolerates open questions in the spec body.
- **Backward via new scenario** — `done` → `in-progress` when `/amend` records a scenario; the scenario's task is implemented and the spec returns to `done`. A scenario that *carries open questions* takes this edge, **not** the question edge: its questions leave the spec body untouched, and `draft` would route to feature-level `/clarify`, which does not read scenarios. The questions still bind — a spec does not reach `done` while any remain.
- **Backward via meaningful body edit** — `done` → `in-progress` when any artifact under `specs/{feature}/` is edited *meaningfully*. An edit is **mechanical** (no back-edge) in three diff-determinable cases: **(a)** every change is the same find-and-replace token substitution, applied uniformly across the live artifacts [§drift-prevention](#drift-prevention) enumerates, mapping a deprecated label (slug, capability, command, identifier, parenthetical descriptor) to its current one; **(b)** every change adds, removes, or rewrites a **cross-service reference** — an inline body link to a registered `.ductus/config.toml` `[services]` entry, with the regenerated `references:` that harvests it — since such references are navigation, never dependencies, criteria, or behavior (spec 030); or **(c)** every change assigns a **runtime-maintained identifier** — an `AC{n}:` label between a criterion's checkbox and its text, with the `next-criterion:` counter — leaving every criterion's own text byte-identical (spec 013). Anything else — new scope, changed semantics, factual corrections, restructuring, edits scoped to one spec — is **meaningful** and takes the back-edge through the same `/amend` flow.

The three cases share one test, and the test decides cases the list does not name: an edit is mechanical when the diff is **determinable without author judgment** *and* **changes no claim the spec makes**. A typo fix, or sweep residue where a substitution landed in a sentence it did not fit, restores what the text already meant and is mechanical. A **factual correction** changes what the spec asserts and takes the back-edge.

This avoids spec proliferation: scenarios evolve the existing spec rather than spawning a new one. Spec bodies are living documents that represent current state; git history is the record of what was written when.

**A branch-scoped spec is a staging form, not a fourth state.** A spec numbered `{identifier}.{n}-{slug}` ([§numbering](#numbering-convention)) is created on a branch that cannot coordinate a sequential number, and declares in `folds-into:` the upstream spec whose statement it is really making. It moves through the lifecycle like any other spec, and **a pending fold never holds it short of `done`** — the pre-`done` gate does not read `folds-into`. Done comes first, then fold-back: the pipeline view reports the pending fold at every status and offers it as the next action once the spec is `done`. Fold-back folds the content into the upstream spec as a body edit or a scenario, re-points every inbound pointer, and removes the staging directory; it needs both specs in one tree, so it normally runs on the upstream branch after the merge.

Fold-back adds **no new back-edge**: reopening a `done` upstream spec takes the scenario edge or the meaningful-body-edit edge, by where the content lands, and an upstream spec that is not `done` is left where it is. What survives the merge is one spec, edited; a branch-scoped directory that outlives its branch is drift, and a detection check reports it.

Operational rules that follow from the lifecycle, on every project (the list carries no count, per [§design-principles](#design-principles)):

- **A retired feature's spec is deleted, not left at `done`.** `done` means *delivered, and still true*; a spec describing something the project no longer has is neither, and one left at `done` puts every other spec's status in question, because a reader cannot tell a live spec from a retired one. Git history records what *was*, as it does for an obsolete scenario ([§scenarios](#scenarios)). When the content belongs with another spec, `/{project}:consolidate` re-points every inbound pointer **before** removing the directory; when nothing survives, the spec simply goes. Retiring only *part* of a feature is an ordinary body edit — the spec stays and describes what remains.
- **Re-open a `done` spec with the status primitive when the only intent is to reflect edits already on disk.** The refinement command expects an input to classify; route through it only when there is genuinely new input to capture.
- **Syncing a canonical record that lives on another spec is a mechanical edit.** When behavior changes and the canonical-sources map points at a table inside another spec's artifacts, update that table in the same change and leave the spec `done` (case (a)); a stale canonical record is worse than the reopen it avoids. When a table needs syncing more than once, move it to the spec that owns the behavior and leave a pointer.
- **Restoring a spec directory from git silently reverts uncommitted pipeline state.** A status flip and ticked checkboxes are writes to tracked files, so a content restore takes them too, unreported. Note the status and checkbox state before restoring and re-apply them, or restore individual files by path; the same holds for any stash, restore, or hard reset. Commit a status transition as its own step — but only when the working copy of the spec file holds the transition and nothing else, because the pre-commit hook restages each staged spec file whole from the working tree: a transition staged alone commits whatever else the file holds, and is lost if the index was its only copy. To separate a transition from edits already on disk, set the edits aside, commit the transition, then restore them and re-apply the transition and any ticked checkbox — or make the transition before the edits.
- **A status transition states both ends — the status you believe the spec is at, and the one you intend.** The status primitive is guarded on the current value, so a transition computed from a stale read fails loudly instead of overwriting what is on disk. When a flip fails on the expected status, read the frontmatter before retrying: the mismatch is the finding.

#### The three cycles

Every spec moves through one of three cycles depending on where it starts and whether new behavior surfaces:

1. **Greenfield** — `/specify` → `/clarify` → `/plan` → `/implement` → `done`. A new feature designed from scratch.
2. **Brownfield** — `/specify` (sketch spec — sparse acceptance criteria are valid) → real work touches the area → `/amend` to add a scenario, or `/clarify` to resolve open questions, or both → `/implement` → `done`. Existing reality being absorbed into specs incrementally.
3. **Reopen** — a `done` spec is revisited because a bug, edge case, or change request surfaces. `/amend` records a scenario, the spec moves back to `in-progress`, and the next pipeline command resumes from there.

All three converge on the same pipeline; what differs is where the spec enters and how precision accumulates.

<!-- §plan-phase -->

### Plan Phase

Define *how* the feature will be implemented. A plan makes technical decisions, identifies affected files, and considers trade-offs.

#### Plan requirements

- References the spec it implements
- Lists technical decisions and their rationale
- Identifies affected files and packages
- Addresses all open questions from the spec
- Produces a data model if the feature introduces or modifies domain entities or data structures

#### A plan records the design as it stands

`plan.md` records the design **as it currently stands**, never the history of how it got there. Its **design record** is the plan template's own `##` sections, compared case-insensitively ([Canonical sources](#canonical-sources)); the template marks which are optional, and only the design record carries a plan's claims. When implementation changes a decision, that entry is edited in place to say what is now true; the plan never gains an entry recording the change, since git history holds what it said when ([§spec-lifecycle](#spec-lifecycle)).

Everything else implementation produces has another home:

- **Verification evidence** — commands run, pass counts, a test proven to fail — belongs to the commit or pull request that lands the task; the lasting form of a test proven to fail is the test itself.
- **Contributor knowledge** routes by population under [Shared knowledge stays in git](#shared-knowledge-stays-in-git).
- **A finding** is dispositioned in the run that surfaced it, and its decision is stored where [Finding dispositions](#finding-dispositions) stores it.
- **A review triage** is not a record at all ([§implement-phase](#implement-phase)).
- **Handoff state** — what is owed, in what order, with what mechanics — belongs in `tasks.md`, in the pending task it concerns ([§tasks-phase](#tasks-phase)).

The rule is detected at the `##` level: `/{project}:analyze` reports each plan section outside the design record, on a spec at `planned` or later, as an advisory finding — discarded with its reason when the section is deliberately part of that plan — and `/{project}:prune` moves such a section's durable pieces home before removing it. A journal nested under a design-record heading escapes the heading test, and the rule holds there all the same.

<!-- §tasks-phase -->

### Tasks Phase

Break the plan into discrete, ordered work items. Each task is small enough to implement and verify independently.

#### Task requirements

- Tasks are derived from the plan, not invented independently
- Each task has a clear definition of done
- Tasks are ordered to respect dependencies
- A task can be completed in a single working session

`tasks.md` is an **ephemeral work-tracking artifact** — a view of what is left to do, derived from the plan. A task's value is spent once its checkbox is checked, because the durable record of what was built lives in the spec, its scenarios, the rules, and git history — the durability test [§bug-handling](#bug-handling) applies to chores. Completed task sections may be pruned, or the file reset to its template, with `/{project}:prune`, and no consumer of `tasks.md` may treat it as a durable index (including `/{project}:analyze`'s scenario-consistency check, which does not require an implemented scenario's task to persist). The durable artifacts are `spec.md`, scenarios and rules, with `data-model.md` and `plan.md`'s design record ([§plan-phase](#plan-phase)) as the record of the design. `/{project}:prune` removes none of what they assert: a decision it moves home from a plan section is edited into the design-record entry it amends, which reopens a `done` spec ([§spec-lifecycle](#spec-lifecycle)).

Because `tasks.md` is ephemeral, **handoff state** belongs there — what is owed, in what order, with what mechanics — as prose in the body of the pending task it concerns, so it lasts exactly as long as its work. An ordering constraint goes on the task that must wait, never in the preamble or under a free-standing heading, which outlive every task; and a note is prose, never a checkbox line, which would count toward its task's completion.

<!-- §readiness-check -->

### Readiness Check

Before implementation begins, verify the feature is ready to build. This is a quick pass/fail gate, not a ceremony.

- [ ] Spec status is `planned`
- [ ] Acceptance criteria are concrete and testable — no empty placeholders
- [ ] All open questions are resolved — the spec body's **and** those carried by any scenario under it
- [ ] Data model exists if the feature introduces or modifies domain entities or data structures
- [ ] Plan does not conflict with `system.md` or other feature specs
- [ ] Tasks are ordered and each has a clear definition of done

If any item fails, fix the gap before writing code.

<!-- §implement-phase -->

### Implement Phase

Write code, tests, and migrations. Implementation follows the tasks list.

#### Implementation requirements

- Code matches the contracts defined in the spec
- Tests verify the acceptance criteria
- **A spec's ticked acceptance criteria are verified against the tree before it closes.** A ticked criterion is a claim, and closing the spec is when to re-earn it. The artifact check that compares criteria to the tree examines `done` specs only, so a spec left in progress is never examined, and it proves only that a path *resolves*, not that a claim about its contents holds. Read the criteria against the tree; a criterion whose claim no longer holds means the spec is edited — reopened and corrected, or consolidated away — not left ticked.
- **A spec does not reach `done` with an outstanding SHOULD.** This is [§design-principles](#design-principles)' completion filter at the review gate. The gate blocks on MUST violations alone and records SHOULD findings as advisory, but advisory is not ignorable at the gate: a SHOULD is addressed when it is **fixed**, or moved under the review's waived section with its rationale. Read the SHOULD count at the completion gate the same way as the MUST count.
- **`examined` counts the in-scope files the review actually read — a file you are confident about by other means is named, never counted.** Believing a file correct — a generated mirror of one you read, say — is not having read it. Count what you opened, and name the rest in the Summary with why you did not open it and what you relied on instead: *generated from a source read in full, generator re-run and reported in sync* is a complete answer, *it is generated* is not. Likewise say which parts of a too-large file you read, and name an in-scope path that no longer exists as absent.
- **A spec's review window is a property of its last reopen, not of the spec.** The diff base is the parent of the commit that last moved the spec into `in-progress`, or `HEAD` while that move is uncommitted, so every back-edge moves it forward, and every change the pass makes afterwards — its own review record included — joins the window. Measure the window rather than quoting an earlier count. Do not narrow a wide window by overriding the base unless it is genuinely wider than the change under review, and record the base used and why in the Summary. The opposite gap does call for an override: a fix committed before the reopen that records it lies outside the window, so record the reopen before making the fix, or, when the fix has already landed, set the base to the commit before it.
- **A review record written before the record gained its coverage and digest fields is unexamined, not clean.** Without `examined` and `scope`, a review that read nothing is byte-identical to one that read everything; without the durable-contract digest, the pre-`done` gate reports freshness as *undeterminable*, which does not block. Nothing backfills these fields: treat such a record as unread and re-run the review, which writes them truthfully only if the passes read the scope. The gap comes in several shapes — all fields absent, a numerator without a digest, a digest without a numerator — so read the record before pricing the work.
- **A spec with no recorded `in-progress` transition derives an empty diff base, and the review's denominator collapses to zero.** Scope resolution still returns the plan's affected files, so the record reads as files examined over a scope of zero. Pass the current commit as the base: the window is then empty by construction, and numerator and denominator agree. The same answer fits a recorded transition whose base is simply old; measure both before choosing, and say in the Summary which base was passed and why.
- **Only a spec's durable contracts stale its review — its scenarios and its data model, nothing else — and that is what prices a corpus-wide sweep.** A sweep confined to spec bodies, plans and tasks costs no re-review however many specs it touches, while one changed line in a scenario or data model obliges a full re-review of that spec. Measure a sweep's durable-contract hits first, then decide each: sweep it and accept the re-review, leave it, or defer it into a pass that re-reviews the spec anyway.
- **The criteria-against-the-tree count cannot be measured while the spec is `in-progress`, and the gate's own ordering is what hides that.** The artifact family that checks criterion paths examines `done` specs only, so against an in-progress spec it has no subject, and an analysis recorded there reports zero truthfully about a run that never asked. Flip the spec to `done`, run the artifact check, flip it back, then write the record with the measured counts and their reasons. Verify and tick the acceptance criteria first: ticking edits `spec.md`, which the analysis digests, so a record written before reads stale at the gate.
- **The review's denominator is derived at write time, so a count measured earlier and typed into the Summary can contradict the record it is written into.** The primitive computes the scope while the reviewer supplies the Summary, and no check compares the two. Read the returned counts against every number the Summary states, and re-run with corrected prose when they differ; the call is idempotent.
- **Nothing durable goes in a review record's body.** `/{project}:review` regenerates `review.md` whole on every run; only the frontmatter's waiver and decision lists carry forward. Whatever must survive belongs on the artifact it describes, in the spec body: the record of a superseded criterion, the reason for a decision, a claim about the tree. **A triage of a review's findings is not such a record.** A MUST or SHOULD violation keeps its fix-or-waive model; every other finding is fixed, routed or discarded in the run that surfaced it ([Finding dispositions](#finding-dispositions)), one awaiting a decision is recorded undispositioned and detected again by the next run, and the fixes a triage plans are tasks. None of it belongs in the plan ([§plan-phase](#plan-phase)).
- No work happens outside the tasks list — if new work is discovered, add it as a task first
- Refactoring that preserves existing behavior and contracts does not require a spec or scenario update. If a refactor reveals a missing requirement or changes documented behavior, update the spec or add a scenario to capture the new expectation before proceeding.
- Before the spec advances to `done`, `/{project}:review` runs against the implementation and `review.md`'s frontmatter records the result. The transition is gated: `/{project}:implement` halts when that record's `last-run` is unset or its `blocking` is `true`. See §spec-lifecycle.

<!-- §constants -->

#### Constants and configuration

See `framework/rules/configuration-cross.md` (`CFG-CONST-NNN` rules) for the enforceable rules covering centralized shared constants, module-local constants, and the no-bare-literals requirement for operator-tunable values. `/{project}:analyze` enforces these rules.

<!-- §env-vars -->

#### Environment variables

See `framework/rules/configuration-cross.md` (`CFG-ENV-NNN` rules) for the enforceable rules covering env-var defaults backed by named constants, `.env.example` completeness, fail-fast startup validation, and unit suffixes for time-valued variables. `/{project}:analyze` enforces these rules.

<!-- §bug-handling -->

## Bug Handling

Bugs are unwritten or violated requirements. Every bug is evidence that one of the framework's three artifact tiers — rules (cross-cutting), specs (feature-wide), or scenarios (situational) — has a gap, and fixing it means making the requirement at the right tier more precise rather than tracking the defect in a separate system. See [§rules](#rules) for the rule tier and [§scenarios](#scenarios) for the scenario tier.

Not every finding or logged item is a requirement gap. A **chore** is a discrete piece of project maintenance — lint or formatting cleanup, dependency cleanup, repo hygiene, a standalone refactor — that adds no missing or violated requirement and belongs to no single feature. A chore spawns no rule, spec, or scenario, and no spec's `tasks.md` is its home. **A chore a run finds is fixed in that run**, confirmed before it writes ([Finding dispositions](#finding-dispositions)); `/{project}:implement` records one found outside its spec as a disposition task, so the task in hand is not derailed, and fixes it when that task is worked. **A chore a person logs by hand** stays a checkbox in `specs/inbox.md` until it is done, then is removed. The test is **durability**: rules, specs, and scenarios hold information that must stay accurate as the project evolves — feature context, acceptance criteria, resolved questions that serve as the decision record, cross-cutting rules — while a chore is transient work whose value is spent once complete. Route requirement gaps through the decision tree below; do chores directly.

### Bug Decision Tree

When a bug is reported, follow this decision tree in order. The first matching condition determines the route:

1. **No rule covers this cross-cutting concern** — the bug surfaces a class of behavior the framework should govern at the rules tier (perf budget, observability commitment, security control, accessibility minimum, etc.). Promote to a rule (new or amended), then fix the code.
2. **No spec exists for the behavior** — the bug is a feature-level gap. Write the spec first, then fix the code.
3. **Spec exists but is ambiguous or incomplete** — the bug is a spec deficiency. Correct or enhance the spec, then fix the implementation.
4. **Spec is clear but implementation is wrong** — add a scenario capturing the correct behavior, then fix the code.

In all four cases, the rule, spec, or scenario becomes more precise. The artifact update is the primary outcome, not a bug report.

<!-- §scenarios -->

### Scenarios

A scenario is a spec at a lower level of abstraction — same format, same discipline, narrower scope. Scenarios live in a `scenarios/` subdirectory alongside the spec they elaborate.

Each scenario file contains:

- **section** (frontmatter) — the parent spec section the scenario elaborates; the parent feature is implicit in the scenario's file path
- **Context** — the specific situation or precondition
- **Behavior** — what the system does in that situation
- **Edge Cases** — boundary conditions and exceptions (optional)

Scenarios use plain language. Given/When/Then syntax is not required.

The scenario-creation primitive writes the frontmatter, the heading, *and* the Open / Resolved Questions scaffolding, so **do not author those question headings in the body passed to it** — a body already carrying them produces duplicate headings the markdown linter rejects. Pass Context, Behavior and Edge Cases, then edit the scaffolded questions section; write the whole file directly when it needs questions at creation time.

The task-append primitive can succeed while writing nothing, and **that is an outcome to read, not an error to retry**. It deduplicates: when a task already references the scenario being recorded, or — when asked to deduplicate on the title — a pending task carries the same title and items, it returns that task's number and leaves the file untouched. Read the fields it returns rather than its exit status, and when nothing was appended, extend or work the task it names.

#### Scenario lifecycle

Scenarios do not have their own status field. A scenario is either written (merged) or not. When a scenario is created, a task is appended to the parent spec's `tasks.md` referencing the scenario. The task carries the completion status — the scenario itself is a permanent requirement document.

- The parent spec's status remains `in-progress` while scenario tasks are being worked
- When the task is complete, the scenario stays as documentation of the expected behavior
- If a scenario becomes obsolete, it is deleted — not marked with a status

#### When to create a scenario

- A bug surfaces that the spec covers at a high level but does not describe in sufficient detail
- An edge case is discovered during implementation or review
- A spec section is growing too large and needs to be decomposed

#### When a scenario is not needed

- The spec itself was missing or ambiguous — fix the spec directly
- The behavior is already captured by an existing scenario — update the existing file

<!-- §scenario-promotion -->

#### Scenario promotion

In brownfield projects, scenarios serve a dual purpose: they elaborate edge cases (as in greenfield) and they decompose broad features into distinct workflows. When a scenario grows complex enough, it signals that the behavior warrants its own feature spec.

Indicators that a scenario should be promoted:

- The scenario has more than three edge cases
- The scenario's behavior section is longer than the parent spec's
- The scenario has open questions unrelated to the parent spec's domain
- Multiple scenarios in the same feature share overlapping concerns that would be better unified in their own spec

To promote: the user runs `/specify` to create the new spec (whether the behavior is new or an existing feature being decomposed — `/specify` accepts both greenfield and brownfield input), then replaces the original scenario with a dependency reference in the parent spec.

Promotion is a user decision, not automated. The framework provides the pattern; the user recognizes when decomposition is needed.

<!-- §rules -->

### Rules

A rule is an enforceable, citable requirement that applies across multiple features. Rules are the third artifact tier — alongside specs (feature-wide) and scenarios (situational), rules cover **cross-cutting** concerns the framework has opinions about regardless of which feature is being built (security, performance, concurrency, observability, accessibility, audit/compliance, data handling).

Rule files ship under `specs/rules/{rule-set}.md` and are referenced from feature specs by ID. The canonical example is `specs/rules/security-backend.md`, whose rules (e.g., `BE-AUTHN-001`) any spec touching authentication can cite. `/{project}:analyze` enforces rules — it loads each rule file, runs each rule's Verification step against feature artifacts, and reports gaps.

#### Rule format (summary)

Every rule has four required fields:

- **ID** — a permanent identifier (e.g., `BE-AUTHN-001`) cited from feature specs.
- **Statement** — a block quote stating the obligation with RFC 2119 keywords of one tier: MUST and MUST NOT rules are blocking, SHOULD and SHOULD NOT rules are advisory. MAY may appear beside either to state a permission, and carries no tier.
- **Rationale** — the threat or risk the rule mitigates.
- **Verification** — instruction to `/{project}:analyze` on how to check compliance against feature artifacts.

The full schema, ID-stability invariants, the ID grammar (including the `[A-Z][A-Z0-9]*` category-abbreviation format), and Verification phrasing rules are canonically declared in `specs/008-security-rules/data-model.md` — and, for configuration rules, in `specs/017-derive-dont-ask/data-model.md`. The specific category abbreviations a given rule file uses are declared in that file's own header (e.g., `api-backend.md` declares `SCHEMA`/`APIVER`/…). New rule files follow the same schema.

#### When to write a rule

**A new rule belonging to an existing rule surface is added to that surface's owning spec through the back-edge — it does not get a spec of its own.** Register the category, add the rule and a task, and let the owning spec return to `done`; a spec per rule fragments one concern's record across many specs ([§spec-lifecycle](#spec-lifecycle)). Keep the cross-cutting principle in one canonical home and have each enforcement point reference it. Reserve a new spec for a genuinely new rule *file* or surface.

A new (or amended) rule is justified when **all four** of these hold:

1. **Cross-cutting** — the concern applies to multiple existing or anticipated features, not a single feature's domain.
2. **Citable** — the concern's verification can be expressed as a step a reviewer or `/{project}:analyze` can check (a code-pattern check, a documentation-commitment check, or both).
3. **Governance-recognized category** — the concern belongs to a class the framework treats as foundational (security, performance, concurrency, observability, accessibility, audit/compliance, data handling, etc.) rather than feature-specific behavior.
4. **Generalizable wording** — the rule statement would make sense in any spec that touches the area, not only the spec that motivated it.

Indicators are evaluative, not mechanical. The same judgment discipline applies to rule promotion as to scenario promotion ([§scenario-promotion](#scenario-promotion)) — the framework provides the pattern; the user recognizes when promotion is warranted.

#### When a rule is not needed

- The concern is **situational** (specific condition, concrete behavior) → write a scenario under the affected spec.
- The concern is **feature-wide** (one feature, broad property) → add an acceptance criterion or section to that spec.
- An existing rule already covers the concern → cite the existing rule from the spec rather than creating a new one.

#### Filename suffix

Rule filenames signal the surface a rule applies to via a closed-suffix convention. Every `framework/rules/*.md` file MUST end in exactly one of:

- `-backend.md` — loaded for backend stacks
- `-frontend.md` — loaded for frontend stacks
- `-cross.md` — loaded for all stacks (cross-cutting)

The suffix is the surface signal `/{project}:review` and `/{project}:analyze` use to derive rule-file selection without a hardcoded allowlist. `/{project}:review` filters discovered files by the project's detected stack; `/{project}:analyze` loads every discovered file regardless of stack (citation verification spans surfaces).

Enforcement is two-layered. In `ductus`'s own repository, `scripts/lint-rule-filenames.sh` fails CI on any file outside the closed-suffix policy. In adopter repositories, where the lint does not run, a rule file with an unrecognized suffix loads for every stack and emits a one-line warning (`rule file <name> has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md`) — load and warn, never silently skip.

#### Project-level opt-out

A project may exclude a stack-selected rule file from `/{project}:review` by listing it in `.ductus/config.toml` `[[review.disabled-rule-files]]` with a mandatory `reason` — the reason is the audit trail for the override, surfaced on stdout at the start of every run. The opt-out is project-wide and applies to whole files; per-`(rule, file)` exceptions remain the job of `/{project}:review --waive`. Schema and behavior are documented in `/{project}:review`'s command source.

#### Lifecycle

- Rule IDs are permanent. Once assigned, an ID is never renumbered, even if the rule moves within the file or is edited.
- Rules are deprecated with a `**DEPRECATED in {version}:**` label and a removal target version, then removed only after the deprecation window has passed.
- New rule files **that ship with `ductus`** are introduced via their own feature spec (as 008 introduced `security-backend.md` and `security-frontend.md`). A rule file a project authors for itself needs no introducing spec: placing it in the rule-file directory registers it, and its own header declares its ID prefix and category abbreviations. Every consumer treats the two origins identically; nothing may condition loading, citation resolution, or validation on a rule file having an introducing spec. **Recorded exception (backfill):** `api-backend.md`, `accessibility-frontend.md`, and `performance-frontend.md` were introduced in commit `9ccbd0b`, bundled into specs 024/025 rather than their own introducing specs; they are in active use, their ID grammar is reconciled with this section, and they are retained as-is.

See `specs/008-security-rules/data-model.md` for the full ID-stability invariants and deprecation rules.

#### Three tiers, selected by scope

| Tier | Scope | Artifact |
| --- | --- | --- |
| **Rule** | Cross-cutting (applies across many features) | A rule file under `specs/rules/{rule-set}.md`, cited by ID from the specs that depend on it |
| **Spec / acceptance criterion** | Feature-wide (one feature, broad property) | A section or AC in the feature's `spec.md` |
| **Scenario** | Situational (a specific condition with concrete behavior) | A file in the feature's `scenarios/` directory |

Bugs route to the tier that matches the *scope* of the missing or violated requirement (see [Bug Decision Tree](#bug-decision-tree) above). A perf bug that affects every API endpoint promotes to a rule; a perf bug specific to one feature becomes an acceptance criterion; a perf bug that only manifests under a specific concurrency condition becomes a scenario.

<!-- §brownfield-inbox -->

### Brownfield Inbox

A `specs/inbox.md` file is the project's capture queue for todos a person has not yet assigned to a feature spec. Items are recorded with `/log` — the inbox's only producer — and groomed into their proper home with `/groom`. No command appends to it on its own: a finding a pipeline run produces is dispositioned in that run ([Finding dispositions](#finding-dispositions) below), because the inbox is the one destination no gate reads. Its standing use is **brownfield migration**: a project adopting `ductus` incrementally parks known issues here until a spec exists to absorb them.

An item's proper home is usually a feature spec, scenario, or rule; a **chore** ([§bug-handling](#bug-handling)) has no spec home, so `/groom` does it in the same pass and removes it.

Inbox rules:

- Do not frontfill bugs that are not being actively worked on
- Write specs for areas being actively touched — let adoption spread naturally
- As specs are written, `/groom` migrates items from the inbox into their proper home
- **When an item routes to a chore, fix it — do not park it.** A parked chore is re-read and re-routed on every grooming pass, so carrying it costs more than the fix, and the inbox stops reflecting real backlog. Leave one in place only when the fix is genuinely blocked or proves not to be mechanical — and say which.
- **An item blocked on an external event is discarded, not held — the inbox has no hold state.** An item waiting on something outside the project — a protocol stabilizing, an upstream tool shipping support, a dependency's next major — matches none of `/groom`'s five routes, and an *on hold* marker is re-read and re-deferred on every pass while its conditions go stale. Discard it, and keep the durable half where it is read when it matters: the maintenance burden in the contributor guide, the constraint in the spec body that must respect it. A change worth acting on resurfaces when someone next touches the surface it affects. An item that is merely *undecided* is not blocked: its premise can be evaluated today, so it routes to a scenario under its covering spec, with the decision in that scenario's open questions.
- The brownfield-migration backlog drains toward empty as adoption completes; the file persists as long as people keep logging todos
- **The standing count is a notice, never a gate.** `/{project}:status` renders how many items are outstanding and how old the oldest is on every run, zero included, so examined-and-empty never renders as not-computed. Nothing blocks on it, so capture stays free — which is safe only because nothing a gate needs can land here.
- **Remove an inbox item by matching its own text, never by position.** The removal primitive matches the bullet's text by equality, because line numbers computed up front shift the moment the first removal lands, and a positional batch deletes unrelated items while every call succeeds. Extract each item's line and strip its checkbox rather than retyping the text, assert the match is unique, and read the diff of removed bullets against what you intended. Never drive a by-position edit over any list that shrinks as you walk it.

#### Finding dispositions

Every finding a pipeline run produces MUST get exactly one disposition before its spec can reach `done`: a `/{project}:review` observation, a `/{project}:analyze` finding still live when detection ends (in any tier), and an issue `/{project}:implement` surfaces outside the task in hand. A command that drops its findings, or parks them where no gate reads, lets the spec reach `done` looking finished.

- **Three dispositions.** **Fixed**: a chore — mechanical, adding no durable requirement by [§bug-handling](#bug-handling)'s test — is fixed in the run; a fix that fails or proves not to be mechanical is reverted and is not a chore, and a chore is never counted fixed while its fix is not on disk. **Routed**: the finding is written to an artifact the pre-`done` gate reads, chosen by the groom decision tree (`groom.md` is its single canonical statement, which each command references rather than restating) — a task, scenario, or body edit on the spec in hand or on another existing spec, reopened `done → in-progress` when it is `done`, with the reopen named in the confirmation; a new spec, created through `/{project}:specify`'s procedure in the same run; or an amendment to a rule file the project owns. A finding against a managed rule file the project has not pinned is discarded with that reason, since the next update overwrites the file, and a declined spec creation leaves the finding discarded or undispositioned, never routed to a spec that does not exist. **Discarded**: the finding and the reason it is out of scope are recorded in the run's own record. The inbox is not a disposition.
- **Detect, decide, re-check, record.** Detection writes nothing. Each fix or route is confirmed with the operator before it writes, and a confirmed route whose write fails is undispositioned. When any disposition wrote, detection runs again over what changed, and only then is the run's record written, so the record describes the state the dispositions left and a run never makes its own record stale. A finding the re-check no longer produces is recorded as fixed, whether a chore or a route removed it; a finding is recorded as routed only while it still fires, until its routed work lands. With no operator to confirm, nothing is written, and each finding without a stored decision is recorded undispositioned — under `ductus exec`, every finding.
- **Scope decides the destination.** Inside the current **task** → fix it in the task; inside the current **spec** but outside the task → a new unchecked task on that spec; outside the spec → one of the three dispositions. An in-progress spec *is* the home its own defects need. This does **not** make `tasks.md` a capture queue: a missing requirement still becomes a scenario or spec edit via `/{project}:amend` ([§bug-handling](#bug-handling)), with the task as the work item that implements it.
- **Record, do not derail — the disposition task.** `/{project}:implement` does not stop the task in hand for a finding outside the spec. It appends `Disposition out-of-spec finding: {summary}` to the targeted spec's `tasks.md`, deduplicated on that title, and keeps working; the unchecked task holds the spec out of `done` and survives an interruption. Working it fixes, routes, or discards the finding, and a discard's reason is written onto the task as it is checked off.
- **A finding that already gates `done` cannot be discarded.** An analyze hard-fail or blocking finding is fixed or routed and keeps blocking until the re-check no longer produces it. Review MUST and SHOULD violations keep their fix-or-waive model. The discard route serves advisory findings, review observations, and disposition tasks.
- **Decisions persist.** Detection is stateless, so a decided finding fires again on the next run. Each routed or discarded decision is stored in the record of the command that made it (`decisions:` in `review.md` or `analysis.md`) with its outcome, its target or reason, when it was made, and by whom, and a later run counts a matching finding under the stored outcome without asking again. A stored decision whose finding no longer fires is pruned; a run that did not evaluate the finding's source retains it. Matching is by judgment, not byte equality, because a finding's wording drifts; a reworded finding left unmatched is new and is asked about again.
- **The gate asks for a decision, not a fix.** Each record counts its findings in a `dispositions:` map, and the pre-`done` gate blocks `in-progress → done` while either record's `undispositioned` is above zero — checked after every other review and analyze check. A record with no map predates dispositions, and absence is not zero: on an `in-progress` spec the gate blocks and names the command to re-run; on a `done` spec it is not drift. Discarding a false positive with its reason clears the block.
- **Severity raises salience, not the routing.** Security issues and memory or resource leaks are surfaced first and flagged; every finding gets the same three dispositions.

This keeps the agent's attention on the task while every finding ends somewhere a gate reads, or in a recorded decision that it belongs nowhere.

<!-- §brownfield-process -->

### Brownfield Process

Brownfield projects adopt `ductus` incrementally. The `/specify` command initializes a skeleton spec from freeform user input — sparse acceptance criteria are expected and valid for brownfield use; no pressure to be comprehensive. Start broad; decompose through scenarios over time.

#### Capture → incremental growth → promotion

1. **Capture** — the user runs `/specify` with whatever description they have. Sparse acceptance criteria are expected and valid — the spec gains precision through subsequent bug fixes, scenarios, and clarifications.
2. **Incremental growth** — every subsequent touch on the feature adds precision:
   - A **bug fix** reveals missing behavior → adds an acceptance criterion or scenario
   - An **enhancement** adds new behavior → follows the normal pipeline (spec change before implementation)
   - A **clarification** resolves an open question → narrows ambiguity
3. **Promotion** — when a scenario outgrows its parent spec, the user promotes it to its own feature spec (see [Scenario promotion](#scenario-promotion))

Over time the spec converges on a complete description of the feature — not from a documentation effort, but as a side effect of doing work.

#### Inbox integration

When a `/groom` pass encounters an item that does not map to any existing spec, `/groom` directs the user to run `/specify` to initialize a spec first, then return to process the item. The commands stay decoupled — `/log` records, `/groom` routes, `/specify` creates specs.

<!-- §text-first-artifacts -->

## Text-First Artifacts

`ductus` treats every artifact — constitution, specs, plans, tasks, scenarios, rules — as plain markdown the agent can edit with `Edit`. This is load-bearing: the agent's write path stays simple, PRs review glanceably, and merge conflicts stay rare and human-resolvable. The **artifacts** are usable standalone with no tooling beyond the AI agent — every one is plain markdown a contributor reads, edits, and reviews by hand, with no build step and no export. That is what text-first governs. The *pipeline* that reads them requires the runtime ([§runtime-boundary](#runtime-boundary)), which parses and writes the same markdown a contributor edits in an editor.

### Principles

- All `ductus` artifacts are markdown by default. The agent reads and writes them with the same `Edit` flow used for code.
- Structured metadata lives in YAML frontmatter at the top of each markdown file; the document body remains markdown prose.
- Cross-artifact references use standard relative markdown links (`[label](../path.md)`), not wiki-links — this keeps PRs reviewable on GitHub and viewers like Quartz/Obsidian still resolve them.
- Source-of-truth artifacts are markdown. Structured derived views are regenerated from canonical sources and never become the canonical record.
- Structured derived views (SQLite caches, JSON indexes, generated graph data, binary artifacts) MUST be gitignored and regenerated on demand by their consumers.
- Exceptions to text-first source-of-truth require an explicit constitutional amendment with stated rationale.
- **A markdown link in a spec body creates a `dependencies:` edge — cite in prose when you mean a citation.** The dependency derivation harvests every inline link to a sibling spec into the frontmatter, and the pre-commit hook applies it before anyone looks. To cite without an edge, name the spec in prose or backticks, or place the link under the section the derivation's opt-out exempts; read the `dependencies:` line the hook reports before committing.
- **Write pipeline state files with the file-writing tool, not shell redirects.** Permission entries scoped to a pipeline-owned path — the session file above all — grant the editing and writing tools, not shell redirection, and widening a shell allowlist to compensate grants write-anywhere and defeats the per-path scoping.
- **Never stage the whole worktree in a project running this pipeline.** A blanket `add` sweeps untracked in-progress spec drafts into a commit, which the tracked-specs rule forbids: the generators and the pipeline scope to the git index, so an unadded draft is deliberately invisible to them. Stage explicit paths.
- **A scenario's link to a sibling spec needs one level more than a spec's.** A scenario sits one directory below its spec, and the corpus link check reports a link one level short with the corrected path. The dependency derivation reads spec bodies only, so a scenario's link induces no edge while a spec-body link does — and when its target already depends on the citing spec, that edge closes a **cycle**. After a spec-body edit, read the derivation's cycle report with its drift report.
- **When an edit's purpose is to remove a section, diff the structure, not only the prose.** An edit anchored on a string that also occurs inside a code span silently consumes everything between the two, and a heading absorbed into an **unterminated** code span stops being a heading: no markdown lint trips, and the spec parser omits the swallowed section *entirely*, so a gate that reads it can be satisfied **by** the damage. Count the headings before and after the edit; it is the only signal this failure gives.

### Frontmatter Schema

The frontmatter schema applies to **spec files** (`spec.md`), **scenario files** (`scenarios/{slug}.md`), and the two **audit records** below (`review.md`, `analysis.md`), whose frontmatter is required because a pipeline gate reads it. Other `ductus` artifacts (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan files, tasks files, rule files, README files) MAY include frontmatter when a specific consumer benefits, but are not required to.

#### Spec files

| Field | Required | Type | Allowed values | Description |
| --- | --- | --- | --- | --- |
| `status` | yes | string | `draft`, `clarified`, `planned`, `in-progress`, `done` | Spec lifecycle state |
| `dependencies` | yes | list of strings | spec slugs (e.g., `002-events`); empty list permitted | **Generated** by the `derive-dependencies` runtime primitive from inline markdown links to sibling specs in the body. Not hand-authored. Author opt-out: links under a `## See also` heading are treated as navigational and do not produce edges (`## References` remains a dep-producing section). |
| `references` | no | list of `{service, spec}` entries | registered service alias + target `NNN-slug`; empty or absent permitted | **Generated** by the `derive-references` runtime primitive from inline body links to a registered service's canonical repo URL. Not hand-authored, and **strictly distinct from `dependencies`** — informative cross-service navigation that never enters the blocking dependency graph (spec 030). |
| `next-criterion` | no | integer | ≥ 1; absent means no criterion has been labelled yet | **Maintained by the runtime's labelling pass.** The `AC{n}` label the next acceptance criterion receives. Monotonically non-decreasing — deleting a criterion never lowers it — so a retired label is never reissued to a different requirement. Not hand-authored; the audit requires it to exceed every `AC{n}` label present in the body (spec 013). |
| `cross-spec-impact` | no | list of strings | spec slugs (e.g., `050-constitution`); empty or absent permitted | **Hand-authored.** The specs this one owes a change to under [§cross-spec-impact](#cross-spec-impact). Each entry is an obligation that blocks `in-progress → done` until the named spec's body or a scenario under it links back to this one; absent and empty are the same state and report nothing. Unlike `dependencies`, it is not derived — whether work here implies a change there is judgment no generator can make. |

#### Scenario files

| Field | Required | Type | Allowed values | Description |
| --- | --- | --- | --- | --- |
| `section` | yes | string | parent spec section name (e.g., `"Authentication flow"`) | The section of the parent spec the scenario elaborates. The parent feature is implicit in the file path. |

#### Audit records

Each audit command records its run in the frontmatter of the artifact it writes: `/{project}:review` in `review.md`, `/{project}:analyze` in `analysis.md`. **`spec.md` carries neither record**; a `review:` or `analyze:` block in a spec's frontmatter is a residual of the earlier schema and is reported as a violation, since one fact has one home ([§drift-prevention](#drift-prevention)).

**The absent file is the never-run state.** A feature with no `review.md` has not been reviewed; one with no `analysis.md` has not been analyzed. Neither is ever written empty to signal a clean run. An artifact that exists but does not parse is **undeterminable**, a third state that MUST NOT be collapsed into never-run.

##### Review record — `review.md`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `spec` | yes | string | Feature slug the record belongs to. |
| `last-run` | yes | ISO-8601 UTC, nullable | When the review ran. Null is permitted only on a record that has never run. |
| `reviewed-against` | no | string | HEAD sha at the time of the run. **Provenance, never the staleness basis** — the working tree and a commit are the same subject only on a clean tree. |
| `diff-base` | no | string | The sha the review diffed from. |
| `must-violations` | yes | integer | Blocking findings. |
| `should-violations` | yes | integer | Advisory findings. |
| `low-confidence` | yes | integer | Findings the reviewer flagged as uncertain. |
| `dispositions` | no | map: `fixed`, `routed`, `discarded`, `undispositioned` → integer | What the run did with its **observations**; MUST and SHOULD violations keep their own counts. Derived, not authored. Absent means the record predates dispositions, which blocks an `in-progress` spec — absence is not zero. |
| `examined` | no | integer | Files the review actually read. |
| `scope` | no | integer | Files the review's scope contained. Distinct from `examined`: equal means fully examined, and less means partially. |
| `skipped-passes` | no | list of strings | Review dimensions that did not run, by name. |
| `reviewed-digest` | no | map of path → sha256 | Per-path digest of the review's **durable contracts** as the run read them from disk. Absent means a pre-digest record, which is undeterminable rather than stale. An empty map is distinct: the digest was taken and there were no contracts to digest, which reads as current. |
| `blocking` | yes | boolean | Derived, not authored. |
| `waivers` | no | list | Waived findings. Each entry names one rule and one path or a list of paths, and each `(rule, file)` pair is its own anchor. |
| `decisions` | no | list | Stored routed and discarded observation decisions, each keyed on the observation's rendered line and carrying `outcome`, `target` (routed) or `reason` (discarded), `decided-at`, and `decided-by`. A list that does not parse is a defect, never an empty list. |

##### Analyze record — `analysis.md`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `spec` | yes | string | Feature slug the record belongs to. |
| `last-run` | yes | ISO-8601 UTC, nullable | When the analysis ran. |
| `analyzed-against` | no | string | HEAD sha at the time of the run. Provenance only, on the same reasoning as `reviewed-against`. |
| `analyzed-digest` | no | map of path → sha256 | Per-path digest of the **analyze subjects** as the run read them from disk, excluding `analysis.md`'s own record — a record written after its subjects are read can never digest itself. Absent is undeterminable. |
| `analyzed-unreadable` | no | list of strings | Subjects that exist but could not be read. Recorded rather than digested as empty, so a record cannot claim to have covered a file it could not open. |
| `hard-fail` | yes | integer | Malformed-artifact findings. |
| `blocking-findings` | yes | integer | Findings in the blocking tier. Named for the tier, to keep it distinct from the derived `blocking` flag. |
| `advisory` | yes | integer | Recorded and never gated on — some of the tier's checks carry their own published promotion criteria, and the rest stay advisory by design. |
| `unexamined` | yes | integer | Targets the run could not examine. The field that makes a clean run honest: clean with nothing skipped and clean with something skipped are two different results. |
| `unexamined-by-reason` | no | map of reason → integer | |
| `dispositions` | no | map: `fixed`, `routed`, `discarded`, `undispositioned` → integer | What the run did with its findings, in every tier. `undispositioned` is the live tier total less the live findings routed or discarded, so a finding the run did not itemize counts as undispositioned. Recorded beside the tier counts because a run that produced five findings and decided none must not be byte-identical to one that decided all five. Absent is the pre-disposition state, as in the review record. |
| `decisions` | no | list | Stored routed and discarded finding decisions, keyed `{family} — {message}`, in the review record's shape. |
| `blocking` | yes | boolean | Derived, not authored. |

#### Open-schema rule

Fields beyond those listed above are permitted and ignored by uninterested consumers, with the one exception **Audit records** states: a `review:` or `analyze:` block in a spec's frontmatter is reported, because the rule admits fields nothing has claimed, not a field this schema has moved. Examples adopters might add: `owner`, `target_release`, `created_at`, `description`, `aliases`. Consumers MUST NOT error on unknown fields; `/{project}:analyze` reports them as informational findings. Stale fields in done specs (e.g., `title`, `tags`, `spec-ref`, `track`) remain valid and produce no findings.

### Validation Severity

`/{project}:analyze` checks frontmatter against this schema with the following severity:

- **Hard fail** — frontmatter block missing on a spec or scenario file; frontmatter YAML malformed; `status` missing or not in the allowed set; `dependencies` missing or not a list; both `section` and the legacy `spec-ref` missing on a scenario; frontmatter missing or malformed on a `review.md` or `analysis.md` that exists (its absence is a state, its presence without a parseable record a defect); a `decisions:` list on either record that does not parse (read as empty, it would drop every stored decision).
- **Blocking** — a `review:` or `analyze:` block present in a spec's frontmatter. The spec file parses, so it is not a hard fail, and a second copy of a gate-read record is drift, not an unknown field, so it is not informational. The remedy is the relocation migration, which the finding names.
- **Advisory** — cross-reference checks; body inline links to sibling specs not yet in the generator-managed `dependencies` (the next commit's `derive-dependencies` pass resolves them).
- **Informational** — unknown fields present.

Hard fail and Blocking both prevent pipeline advancement; Advisory and Informational findings are reported but do not block. Hard fail says the spec file itself is malformed; Blocking says the artifact set is incomplete or inconsistent with itself, which is why a record-block residual is Blocking. Non-frontmatter checks (spec integrity, artifact completeness, plan/task consistency, dependencies, security rules) use the same tiers, and their Blocking findings must be fixed before the next pipeline gate fires (e.g., a missing `plan.md` on a `planned` spec, an unknown rule ID cited in a spec). See `framework/commands/analyze.md` for the full per-check severity assignment.

<!-- §runtime-boundary -->

### Runtime Boundary

`ductus` ships a runtime binary alongside the markdown framework, acquired by `/{project}` during adoption, to execute the deterministic portions of pipeline commands without an LLM. This subsection defines what the runtime can and cannot do; deviations require their own constitutional amendment.

#### Five principles

1. **Markdown is source of truth** — the runtime MUST NOT own state the markdown cannot reconstruct. Runtime-owned data (caches, indexes, parsed graphs) is derived and gitignored, per the existing rule on structured derived views.
2. **Determinism only** — the runtime MUST NOT call an LLM. Work requiring semantic judgment (content quality, `/clarify` resolution, `/specify` sketching, per-rule Verification reads, `/groom` routing) stays in slash commands.
3. **Required, and acquired by the pipeline** — `/{project}` acquires the pinned runtime as part of adoption, so a bootstrapped project has the binary and pipeline commands MAY assume determinism rather than specifying two executable paths to one result. The runtime is *ductus-owned*: acquired and version-managed by the pipeline into a store it writes, never whatever `PATH` resolves. A project that supplies its own binary declares it (`[runtime] path`) and is equally supported. Acquisition failure halts the run rather than degrading, because a requirement that quietly is not one leaves both paths alive. Shell pipelines that parse frontmatter or markdown structure (`awk`, `sed`, `grep` pipelines, `for` loops over files) are **not** a sanctioned substitute for the runtime primitives or the host's file tools.
4. **Schema follows the constitution** — the runtime MUST read frontmatter and artifact structure according to the schemas declared in this document. Schema changes ship through the constitution; the runtime MUST update to match. The constitution MUST NOT import runtime types.
5. **MCP is the seam** — the runtime MUST expose its capabilities as MCP tools so slash commands can call them when they want determinism. This keeps the runtime accessible to any agent host and prevents `ductus`-specific coupling.

<!-- §runtime-host-integration -->

#### Host integration (for agent runtimes)

A command's backticked primitive names map to the MCP tools the runtime exposes under bare `<verb>-<noun>` names. A host prefixes them from its MCP registration — Claude Code: `mcp__ductus__<verb>-<noun>`; Auggie / Antigravity: `mcp:ductus:<verb>-<noun>`; OpenCode: a `<server>_<tool>` name under the `ductus` prefix; Pi: a `ductus__<verb>-<noun>` tool from the project's `.pi/extensions/ductus.ts` bridge, which wraps the runtime's own MCP server over stdio because Pi has no built-in MCP (spec 064). Match the prefix rather than a spelling: the separator is a host detail, and a new host is one more spelling of the same namespace. When the `ductus` server is registered for the session, the agent **calls the corresponding tool** for each step — the deterministic path. A host that loads tool schemas lazily (Claude Code lists deferred tool names before exposing their schemas) still has the runtime registered: fetch the schema through the host's mechanism (`ToolSearch` on Claude Code) and call the tool rather than falling back. When no `ductus` server is configured, the agent walks the same prose with the host's file tools (`Read`, `Edit`, `Write`); the shell pipelines principle 3 names are **not** a stand-in for either path. The two paths share one contract, and neither wraps the other. A rewritten command opens its Instructions with a one-line pointer to this subsection (§runtime-host-integration) rather than restating it.

#### Eligibility criteria

A capability is runtime-eligible only when **all three** hold:

1. **Deterministic** — no semantic judgment required; the same inputs always produce the same outputs.
2. **Currently mechanical** — already either (a) executed by an LLM following procedural instructions in a slash command body, or (b) implemented as a bash script the framework invokes (pre-commit hooks, generators, CI).
3. **Specifiable as prose** — the capability can be stated completely enough that the Markdown-only reference documents it, and a primitive mirrors that reference rather than introducing policy of its own. This is what keeps the specification and the implementation one thing: the reference is where the policy lives, the primitive is how it runs.

A capability that fails any criterion stays out of the runtime. Anything that requires reading prose for intent is permanently LLM-owned regardless of how mechanical its surface looks.

**Eligibility is a default, not a permission.** A capability meeting all three criteria is implemented as a runtime primitive; a shell script is the fallback only when a criterion genuinely fails. Shell entry points — a pre-commit hook, a CI step, an `/audit` family — keep their scripts, but resolve the runtime and call the primitive rather than reimplementing the check. A script that parses frontmatter or markdown structure has failed principle 3 whatever language it reaches for, and each such parser is a fresh copy of a tested parse that drifts from the others. A script runs immediately and a primitive is a build away, but the build is a one-time cost paid by the author, while the parser is a permanent cost paid by every reader.

#### Acquisition invariant

The repository's CI MUST include a job that exercises acquisition end-to-end on every supported platform: fetch the published asset for the target, verify its sidecar digest, install it into a temporary store, and execute the installed binary. A change that causes this job to fail — i.e. a release whose assets an adopter cannot actually acquire — is a constitution violation, not a feature.

It replaced the **opt-in invariant**, a full pipeline cycle with the binary absent from `PATH`, when principle 3 made the runtime required ([048](https://github.com/stonean/ductus/blob/main/specs/048-govern-acquired-runtime/spec.md)): the guarantee is now that the binary is *obtainable*, and the job that proves it is the one that fetches it.

#### Versioning

The runtime ships in lockstep with the framework. A `ductus` release includes the binary built against the schemas in that release; an adopter's `ductus` version pins their compatible runtime version, eliminating schema/runtime drift as a failure mode.

#### What the runtime is not

To prevent scope creep, the runtime MUST NOT be a spec authoring tool, MUST NOT be a workflow orchestrator, MUST NOT be a long-running service, and MUST NOT be a storage layer. Lifting any of these exclusions requires a constitutional amendment.

Specific capabilities are introduced through their own feature specs, beginning with spec 022 (deterministic runtime).

<!-- §drift-prevention -->

## Drift Prevention

These principles keep facts consistent as the framework evolves. They apply both to `ductus` itself and to projects that adopt it. Drift is a class of bug; preventing it is part of the framework's design, not an afterthought.

### Canonical sources

For every kind of fact described in multiple places, one location is authoritative. Other documents that describe the fact MUST reference the canonical source rather than restate it.

**Referencing means a pointer, never a copy.** A link, an anchor, or a named section is a reference; a reproduction of the source's content is not, however faithful on the day it was pasted. A copy is worse than ordinary staleness twice over. **Nothing can detect it** — no link check, anchor resolver, or structural audit reads inside a fenced block, so the copy rots silently. **And every reader pays for it** — in full, in every session, while a pointer costs one line. A historical record is git's job, because a document body describes current state ([§spec-lifecycle](#spec-lifecycle)). Replacing an embedded copy with a pointer changes what the document asserts, so it is a factual correction that takes the back-edge, not a mechanical sweep.

| Fact | Canonical source |
| --- | --- |
| Spec lifecycle states and back-edges | `framework/constitution.md` §spec-lifecycle |
| Pipeline command behavior | each command's source under `framework/commands/*.md` (or `framework/bootstrap/configure/{key}.md`) |
| Frontmatter schema for specs and scenarios | `framework/constitution.md` §text-first-artifacts |
| Validation severity tiers | `framework/constitution.md` §text-first-artifacts (Validation Severity subsection) |
| Per-agent permission set | `framework/bootstrap/configure/{key}.md` |
| Constitution section anchors | `<!-- §<anchor> -->` markers in `framework/constitution.md` |
| Command frontmatter (description, argument-hint) | each command's own frontmatter block |
| Rules artifact tier definition | `framework/constitution.md` §rules |
| Agent grounding / evidence discipline | `framework/constitution.md` §grounding |
| Runtime contract / boundary | `framework/constitution.md` §runtime-boundary |
| Security rule file format and ID conventions (`BE-`/`FE-`) | `specs/008-security-rules/data-model.md` |
| Configuration rule file format and ID conventions (`CFG-`) | `specs/017-derive-dont-ask/data-model.md` |
| Code-quality rule file format and ID conventions (`QUAL-`) | `specs/036-quality-cross-rules/data-model.md` |
| Service registry schema (`.ductus/config.toml` `[services]`) | `specs/030-cross-service-references/data-model.md` |
| Where contributor knowledge is recorded (git vs. per-user agent memory) | `framework/constitution.md` §drift-prevention (Shared knowledge stays in git) |
| A plan's design record — which `plan.md` sections carry its claims | the `##` headings of `framework/templates/spec/plan.md`, installed as `specs/templates/plan.md` |
| Open-state tell list and decision-drift check grammars | `specs/045-decision-state-drift-detection/data-model.md` |
| Scenario→task referencing rule (what counts as a task referencing a scenario) | `specs/067-analyze-and-audit-checks/data-model.md` (`scenario-consistency`) |
| Spec-root resolution (substituting `[paths] specs-root` for the literal `specs/`) | `framework/constitution.md` §spec-phase |
| Constitution content — what belongs in it, how it is organized, which rules adopters receive | `specs/050-constitution/spec.md` (a spec that changes *behavior* still amends the principle its change contradicts, in the same change) |

When adding a new kind of fact that may be referenced from multiple documents, name its canonical source explicitly here.

### Cross-document references

When document B describes content authored in document A, B includes a back-link to A — relative markdown link, anchor reference (`§anchor`), or section name. Consequences follow:

- Changing A includes auditing every back-link to A — structured wherever it can be machine-checked (anchor resolution, help-table descriptions, registry-frontmatter equivalence), and a manual sweep otherwise.
- Adding a fact that conceptually belongs in A but landing it in B is drift. Either move the fact to A and back-link, or extend A's scope explicitly.
- **No dead references in live artifacts.** When renaming or removing a name — a spec slug, a capability, a command, an identifier, even a parenthetical descriptor — update every reference across the project's live artifacts in the same change: specs (including `done` spec bodies), rules, command sources, scripts the pipeline runs, CI configuration, docs, and the README. The sweep is uniform find-and-replace, a mechanical edit under [§spec-lifecycle](#spec-lifecycle), so `done` specs stay `done`; do not bundle it with unrelated edits, which would make the diff meaningful. **Keep the sweep's own target list current**: a list still naming a relocated directory sends the grep somewhere clean, and the sweep silently misses the files that moved.
- **A retired filename is not a retired name — sweep it by meaning, not by token.** When the renamed thing is a *file* the project still reads under its old name, **some occurrences are load-bearing and must survive**: a resolution ladder or fallback tier that lets an unmigrated adopter resolve, a migration procedure that must name both sides to stay auditable, and the prose, shell and configuration that spell either. A blanket substitution breaks all three **silently**. **Everywhere else the old name is residue and goes**: the current name wherever an artifact states current behavior, the retired name only in the few references that record the decision to change it. **A ticked acceptance criterion is swept like any other reference**, since only the file's name moved; annotation is for a criterion whose **behavior** was superseded. Do not annotate a rename, which preserves the retired name permanently. A pure substitution is case (a) and reopens nothing; the runtime computes the matching review-staleness exemption from the diff, so a reworded line costs its spec the exemption.
- **A behavior change needs a prose-claim sweep, not just an identifier sweep.** Stale *claims* about the old behavior contain none of the changed tokens, so an identifier grep passes over them. Enumerate the claims the change falsifies and grep for them **by meaning** across the live artifacts, the README especially. A stale claim in docs is a docs fix; one inside a `done` spec body is a meaningful edit and takes the back-edge.
- **Never edit an installed command file directly.** The installer overwrites it on its next run. Change the source the installer copies from, or pin the file in the project's configuration, which is the supported way to keep a local modification.
- **Renaming a repository orphans contributor-local state that no migration can reach.** State keyed to the project *path* lives outside the repository, so nothing errors — it is simply never found again. After renaming, on **each** machine: rename the local checkout, repoint the remote, move any per-project agent state stored under a path-derived slug (and correct the absolute paths recorded inside it), and fix anything else keyed to the old path — shell aliases, editor workspaces, worktrees.
- **An anchor reference resolves by line, and a line wrap alone breaks it.** A `§name` reference is qualified only when the *same line* names the document the anchor lives in, so wrapping that path onto the previous line re-reads it as a claim about the default document. Within one document, matching runs longest-first against the text after the `§`, so the reference must spell the target heading exactly. A **subsection** carries no anchor marker and is not addressable; cite the marked section that contains it. Read the resolver's per-reference list, not its deduplicated unresolved *set*. And a link that *resolves* may still not point where its text says: when a link's text is itself a path, compare the two.
- **A sweep can make a quoted instruction point at itself, and the identical sentence may be correct one document over — classify by who emits it, never by the string.** A substitution inside a quoted user-facing message still parses, and both sides of the sentence differ, so the uniformity check cannot see it. The tell is the **emitter**: a message redirecting the reader to the command they are already inside is damage, while the same sentence issued by a different command is correct. Find every occurrence and classify each by emitter before correcting one; where the behavior behind the quote also moved, annotate rather than restate.
- **Replacing an embedded copy with a pointer is a corpus sweep, not a deletion.** Other artifacts describe the copy: a sibling names the section, the artifact's own plan may declare it canonical, and another spec may cite it, in the **present tense**, as the motivating instance for the rule against copies. Before deleting, search for the section's name *and* for prose describing it, and classify each hit by tense: a present-tense description goes false, while a past-tense account is the decision record and stays. Also look for acceptance criteria asserting the section exists, and measure the section's extent fence-aware, since a scan for the next heading finds headings inside the copy.
- **A document declaring itself the authoritative shape of something the tooling writes drifts silently — check it against a record the tooling actually wrote, not against the code.** Nothing compares a schema table to a real instance, so fields added by later work can be live for months while the canonical record omits them. When a pass opens a spec whose data model declares a shape something else writes, open a record written minutes ago and compare the field sets; the implementation says only what the code *can* emit.

### Decision resolution

Resolving a decision carries the same audit obligation as editing a document, because the artifacts that describe a decision's state are not always the ones that link to it. The events that trigger it:

- an open question is closed;
- a scenario is implemented and ships;
- a spec, scenario, or task advances its status;
- a previously-rejected option is adopted.

When one fires, every artifact that described the prior state is corrected in the same change. **A resolution is not complete while a sibling artifact still describes the prior state** — the question as open, the option as rejected, the work as unbuilt. Such an artifact does not read as stale: a settled decision reads as work owed, and a criterion naming a deleted path reads as a contract satisfied. `/{project}:analyze` machine-checks the deterministic part; the rest is a manual sweep.

### Template-rule alignment

Every blocking check in `/{project}:analyze` has a corresponding scaffolding element in the template that produces a passing artifact by default. The contract runs in both directions:

- Adding a new blocking check requires a template update so a freshly-copied artifact passes the check without manual editing.
- Adding template structure requires a corresponding rule (an `/{project}:analyze` check, a constitution rule, or both). Sections that don't trace back to a rule are dead weight.

Templates and `/{project}:analyze` evolve together. A diff that touches one without the other is incomplete.

### Manifest discipline

When multiple commands distribute or reference the same set of files (e.g., `/{project}:configure` and the bootstrap install both apply permission sets), the file list lives in one place:

- Either as a shared section the commands include by reference, or
- As a registry both commands read.

Two commands that copy-paste the same manifest into their own bodies are guaranteed to drift over time. Consolidate or accept that drift is the rule, not the exception.

### Shared knowledge stays in git

Knowledge that would help any other contributor belongs in a git-tracked artifact, never in an AI agent's per-user memory. Per-user stores (Claude Code's auto-memory, Cursor's memories, and equivalents) are invisible to every other contributor and absent from a fresh clone, so a fact parked there drifts the moment anyone else works the area: it is knowledge that was never committed at all.

Two questions decide where a learning lands, asked in this order. **Would this help a teammate?** decides whether it is committed at all. **Who is it true for?** decides which committed artifact — a question about the rule's *population*, never about its *kind*:

- A **project learning** — a convention, a gotcha, a workflow rule, a boundary — is routed by population. True for **every project running this pipeline**, its canonical text belongs in this constitution, however it was learned. True across **one organization's projects and no further**, it belongs in the shared constitution that organization registers under `.ductus/config.toml` `[constitutions.*]`, which `/{project}:target` loads alongside this document, within the framework-as-floor rule (*Governance sources, and which governs* above). True of **this project alone**, it stays in the project's own `AGENTS.md` (or the matching rule file under `specs/rules/`).
- A **durable requirement** goes in its canonical artifact: a spec, a scenario, a rule, or this constitution (see [Canonical sources](#canonical-sources)).
- **Per-user agent memory** is correct only for facts that carry no value to other contributors — who the user is (role, persistent personal preferences) and external reference pointers (issue-tracker, chat, or dashboard bookmarks).

**Kind is not population, and routing by kind is the failure this ordering exists to prevent.** A universal rule filed under a gotchas heading is filed correctly by kind and wrongly by population, and the mistake hides itself: the learning *is* committed, so nothing re-asks the second question, and every other project pays for the lesson again. A host's own rules file (`CLAUDE.md`, `AGENTS.md`) supplies the agent-specific routing that applies both questions. Neither is a new authored input — an author already chooses a destination — so [§design-principles](#design-principles)' bar on designs that depend on an author remembering is met rather than waived.

<!-- §pipeline-boundaries -->

## Pipeline Boundaries

- Never implement without a spec
- Never plan without resolving open questions
- Never skip phases — each phase produces artifacts the next phase consumes
- Never transition a spec to the next status without explicit user approval — present the work done and wait for the user to confirm before updating the status field
- Specs and plans are living documents — when a decision changes, edit them in place to say what is now true rather than appending an account of the change ([§plan-phase](#plan-phase)), but don't backtrack silently — a change that moves a spec backward names the back-edge it takes ([§spec-lifecycle](#spec-lifecycle)) before it is made
- Never say a finding will be "recorded" without naming the artifact and the section. The word spans destinations with different gate semantics — an inbox item, an open question, a scenario, an acceptance criterion, a task, the records `/{project}:review` and `/{project}:analyze` write — each produced by a different command: a task gates on its checkbox, a criterion on verification against the tree when the spec closes, and an inbox item on nothing until `/{project}:groom` routes it, which is why `/{project}:log` is its only producer. Name the file and the section, and say what the write does *not* touch.
- **Report outcomes, not edits.** While working, a chat reply does not reproduce the code or artifact changes the agent makes: the operator reads those on disk, in the diff, and in the commit, and a reply that restates them grows with every task and buries the outcome it exists to report. Report which files changed, what a check or test returned, what was committed, and what the operator has to decide, and show a proposed change only when the operator asks. A command's own approval gate is that request: show what approval requires, concisely, and offer the full text rather than pasting it. Failing output that explains a failure is an outcome and is quoted; naming a file or a `path:line` locates an outcome. The rule governs chat replies only — commit messages, `review.md`, `analysis.md`, and spec and plan prose describe changes fully, because those are the records a later reader consults.
- **End a report with the remaining work, or say there is none.** A reply that reports on finished work is short, and its last line is one of exactly two things. One is *No further work identified*. The other is a list of the work still outstanding, each item tagged with a tier the pipeline already uses. **Must** means the work is wrong or incomplete until the item is done: a blocking finding, a failing check, a broken contract. **Should** means the item is expected before the work counts as finished, as an outstanding SHOULD is at the review gate, unless a recorded reason waives it. **Advisory** means the item is worth knowing and nothing holds on it. The reason is the reader: remaining work described in narrative leaves the operator to infer whether anything is left, and a long summary is where such work hides. The list reports work and does not dispose of it. Each item still takes one of the dispositions [§design-principles](#design-principles) names, and the list says where each is recorded, or that the decision is the operator's. *No further work identified* means no item exists in any of the three tiers, not that nothing could be improved.

<!-- §concurrent-features -->

### Concurrent Features

**Each agent process holds its own session target**, so several agents can work in one working tree at once — one implementing a feature, another reviewing it or specifying the next — without one's target write moving the feature another acts on. A process is told apart by an identity from the environment it was launched with, resolved in order: an operator-set `DUCTUS_SESSION`; otherwise the agent's own session identifier, where the agent passes one to the processes it spawns (Claude Code's `CLAUDE_CODE_SESSION_ID`); otherwise none. The environment is set once at launch and inherited by everything the agent spawns, so no command depends on the model remembering to pass a name ([§design-principles](#design-principles)). Within a process nothing changes: one target, and a pipeline serial within a feature (spec 062).

**The shared default.** `.ductus/session.toml` holds the working tree's most recently set target, because every target write, in any process, also sets it. A process with no identity reads and writes only the default. A lone agent whose platform passes a session identifier (Claude Code) is identified without configuration and keeps the same behavior by adopting the default: it resumes the most recently set target, and differs only in its session label in `/{project}:status`, the announced adoption, and — after a restart, until the earlier session's target expires — a notice naming that earlier session as sharing the feature, since an identifier cannot tell a finished session from an idle one. An identified process with no target of its own **adopts** the default the first time it resolves one, says so, and from then on only its own writes — or a fold or consolidation that removes the spec it targets — change it; without that pin, any other process's next write would move it. Per-process targets live in `.ductus/sessions/`, gitignored, and one neither resolved nor written for seven days is removed by the next target write. A cleared target means no target, never "adopt the default". Resolving the target is a runtime primitive (`resolve-session`), because only the runtime can read the process environment; the markdown-only path cannot ([§runtime-host-integration](#host-integration-for-agent-runtimes)), so it reads and writes the shared default only.

**Sessions on one feature.** Two sessions may target the same feature; implementing in one while reviewing in another is the ordinary case. The session making the target write is told which other sessions share the feature and when each last used its target, and each of those is told once, at its next command. Nothing is blocked: status transitions are guarded on the expected current value, so a transition computed from a stale read fails loudly.

**What stays shared, and when a worktree is still the answer.** Per-process targets isolate the *target* and nothing else: two agents in one working tree still share its files, its git index and its commits. When their *edits* need isolating too, `git worktree` and platform isolation (Claude Code's `isolation: "worktree"` agent parameter, Cursor's worktree integration, etc.) remain the answer and compose with this, since each worktree has its own session files — but by default `git worktree add` refuses a branch already checked out by another worktree (git-worktree(1), under `--force`), so under trunk-based work two worktrees are not both on `main`. A push carries every local commit on its branch, another session's included, and the checks that run on it report that work under your push: list what a push carries before making it, confirm that any other session's commits in the range are finished and meant to be published, and name them when you report the push. Two bounds stay open. Processes with **no identity** still share one target and collide undetected — telling live unnamed processes apart needs a liveness check the framework cannot make portably, so `DUCTUS_SESSION` is the remedy. And the per-contributor `cli-config-dir` stays **shared**, so with two different agent CLIs in one working tree, the runtime reads command files from the agent `/ductus` last recorded.

**A command that removes a feature's directory must not leave any session pointing at it.** A target naming a removed directory makes every follow-on command fail, one step removed from the command that broke it. The rule covers every session in the working tree — the shared default and every per-process target naming the removed feature (primitive: `retarget-sessions`) — with one of two outcomes, chosen by what the removal did:

- **Re-target**, when the removal moved the content to a spec that continues the work. `/{project}:fold` is this case: the staging spec's content lands in its upstream home, so every session that named it follows it there.
- **Clear**, when it did not. `/{project}:consolidate` is this case: its target is a spec that already existed and nobody chose to work on, so re-targeting would assert an intent nobody stated. Clearing preserves the per-contributor `cli-config-dir`, so the agent identity survives.

Do either **only for sessions that actually named the removed feature**. Every affected per-process session other than the acting one is told at its next command what happened to its target and which session's command did it; the shared default changes without a notice, since it has no single reader, and the acting command's report names it among the sessions it changed. A teammate's session in another clone is gitignored and unreachable, so it may still name the removed directory; what the rule guarantees is that no session in *this* working tree is stranded.

<!-- §cross-spec-impact -->

### Cross-Spec Impact

Specs are self-contained. When work on one spec identifies changes that affect another spec, those changes are recorded in the affected spec — not left as a note in the originating spec. The affected spec is the source of truth for its own behavior.

This applies when:

- A feature renames or supersedes an artifact from a prior spec
- Work on spec A reveals that spec B needs a new acceptance criterion or scenario
- A scenario in spec A exposes an edge case that belongs to spec B
- An implementation decision in spec A's plan creates a constraint for spec B

In each case:

- The change is recorded in the affected spec as a new acceptance criterion, scenario, or signpost note
- The signpost references the originating spec so the reader understands why the change was made
- If the affected spec is `done`, adding the change reopens it to `in-progress` per the normal lifecycle

The project configuration file is an exception: **it is a shared project-side database, not a schema owned by whichever spec first wrote to it.** A new spec that adds its own section or key documents it, with no signpost on the earlier specs that write to the same file; a configuration change is cross-spec impact only when it modifies an *existing* key another spec already documented.

#### What the framework enforces, and what it does not

**A declared impact is an obligation that gates `done`.** When work on spec A identifies a change that belongs in spec B, A records it in its `cross-spec-impact:` frontmatter — a list of the affected specs' slugs — not only in prose or an acceptance criterion, and A cannot reach `done` while any declaration is undischarged, whatever the state of its review. An impact naming a spec this tree lacks blocks too, since nothing else holds the declaration to its discharge. That is what separates it from a pending fold, which never holds `done` ([§spec-lifecycle](#spec-lifecycle)): fold-back carries an owed fold past `done`, while an undischarged impact would vanish once its spec closed. The mechanism is a check in the pre-`done` gate.

**Discharge is provable, not asserted.** An impact is discharged when spec B carries the change with a back-link to A — the signpost required above. Keying discharge on the reciprocal link, not on the declaration's removal, is what makes it a gate: a key an author deletes to unblock themselves enforces nothing.

**Nothing detects an impact that was never declared.** Whether work on A implies a change to B is semantic judgment no check can make, so the undeclared case stays the author's and the reviewer's, as [§design-principles](#design-principles) requires of a feature with no derivable design. What the framework closes is the commoner failure: an obligation that *was* recognized and then lost because prose and a backlog were the only places to record it.

An acceptance criterion naming the affected spec remains good practice, putting the obligation where a reader of the spec meets it, but it ensures nothing: a criterion is author-written prose that no gate requires. The declaration is the mechanism; the criterion documents it.

<!-- §numbering -->

## Numbering Convention

Feature directories take one of two forms.

**Sequential** — three-digit zero-padded numbers: `000-skeleton`, `001-observability`, `002-events`. A number is an identifier minted in creation order: it records when a spec was conceived, not when it should be built — dependencies determine the build order, and a lower-numbered spec depending on a higher-numbered one is ordinary. A gap in the sequence is expected and never repaired: the counter is `max + 1`, and no directory is renumbered, because `NNN-slug` is the pointer form every `dependencies:` entry and sibling link is written in. **Three digits is a minimum width, not a fixed one**: after 999 comes `1000-`, a feature directory like any other, but padding beyond the minimum (`0500-`) is never produced, since it would give one number two spellings. This is the default and the destination form: spec creation with no branch identifier numbers sequentially, and no persisted setting changes that.

**Branch-scoped** — `{identifier}.{n}-{slug}`, where `{identifier}` is an operator-supplied token sanitized to `^[a-z0-9]+(?:-[a-z0-9]+)*$` and `{n}` counts from 1 within that identifier: `1234.1-retry-budget`, `1234.2-backoff`. The identifier namespaces the counter, so branches numbering under different identifiers cannot collide at merge. The two counters are independent in both directions — a spec root holding `050-a` and `1234.1-b` still yields `051-` next.

**The branch-scoped form is temporary.** It lets work on a branch be specified without claiming a sequential number the branch cannot coordinate. Every branch-scoped spec declares in its `folds-into:` frontmatter key the sequential spec it stands in for, and fold-back discharges it into that spec, whatever its status ([§spec-lifecycle](#spec-lifecycle)).

The membership rule that recognizes a feature directory accepts both forms and is defined in exactly one place; a surface that reads the spec corpus calls it rather than restating the digit convention, since a second copy is how the two forms drift apart.

<!-- §markdown-standards -->

## Markdown Standards

All `.md` files must pass `npx markdownlint-cli2` using the project config in `.markdownlint-cli2.jsonc`.

Key rules:

- Every fenced code block must specify a language — **MD040**
- Files must start with a top-level heading — **MD041**
- No trailing spaces or missing blank lines around headings, lists, and fenced code blocks
- ATX-style headings only (`#`, `##`, etc.)
- Heading levels increment by one — **MD001**
- No duplicate headings at the same level within the same parent — **MD024** (siblings\_only)
- Link fragments must reference valid heading anchors — **MD051**
- Ordered lists use sequential numbering — **MD029**
- Tables use compact style: `| text |` — **MD060**
- Line length is not enforced (MD013 disabled)
- Inline HTML is allowed (MD033 disabled)
