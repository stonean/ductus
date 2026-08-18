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

Per-task token tracking and budget ceilings require a runtime `ductus` does not have — that work belongs to the AI platform. `ductus` contributes by offering cost-aware patterns the user can opt into. The current levers: the stuck-detection step in `/{project}:implement` catches runaway loops before they compound spend; default-off autonomy keeps the human in the loop unless `--auto` is explicitly passed. For runtime cost controls, point the adopter at the platform's tooling — Claude Code's `/cost`, the Anthropic usage dashboard, Cursor's request limits, and equivalents.

<!-- §design-principles -->

### Design principles

Two constraints on anything the pipeline asks an author or a check to do. Both
are hard filters rather than tiebreakers: a design that fails one is redesigned
or deferred, not shipped with a note.

- **A check that cannot run MUST never be indistinguishable from a check that passed.** When a script, gate, or artifact check skips part of its subject, cannot reach it, or has no basis to inspect it, it MUST say so — a distinct result, a `guidance` field, a count of what *was* examined — rather than the same zero, empty list, or success string a genuinely-clean run produces. The failure is asymmetric and silent: nothing errors, a gate passes, and the missing check surfaces later when someone re-derives the result by hand. The states where a check *cannot* run are disproportionately the states where something is wrong. When adding a gate, prove it **fails** before trusting that it passes — break the thing it guards and watch it go red. This is `QUAL-CLAIM-001` (see [§rules](#rules)) applied to the pipeline's own machinery.
- **A check that reads git history must be given git history.** CI checkouts default to a shallow clone carrying a single commit, so any check that resolves a recorded sha, diffs against an earlier commit, or walks the log sees nothing and reports every subject as unresolvable — while passing locally on every developer machine, where the clone is complete. Configure a full-depth checkout in **every** job that runs such a check, and say why in a comment, because the default is invisible until it bites. A local pass is not evidence the check works in CI: the environments differ in exactly the dimension the check depends on.
- **A test that reads history or shells out to a project script is a change to the job that runs it, not only to the suite.** The visible half is the test; the invisible half is that its job now has an environment requirement (full history) and a new input (whatever it shells out to, which belongs in that workflow's trigger paths). Miss the first and the test cannot run; miss the second and it silently stops running when the thing it checks changes. Prefer a test that fails loudly when its inputs are missing over one that skips — a skipped conformance test is the previous principle in its purest form.
- **Never design a pipeline feature that depends on human diligence.** Any artifact section, frontmatter field, command behavior, or workflow step that requires an author to *remember* — to fill something in, set a flag, or update a document alongside a change — will be skipped, and skipped precisely in the cases where it mattered most. When proposing a new input, ask what happens when an author forgets. If the answer is "the feature degrades silently," derive the input instead — from existing artifacts, frontmatter, or history — or do not ship it. Deferring the feature is the correct outcome when no derivable design exists; shipping the disciplined version "for now" is not.

<!-- §grounding -->

## Grounding

Work from what can be observed, not from what can be guessed. When a question can be answered by consulting a source that is actually reachable — the code, the project's own artifacts, a connected dev database, runtime output — the agent MUST consult that source before answering. Reasoning from the conversation alone, when a primary source was available and went unread, is a defect regardless of whether the guess turned out to be right.

Grounding is the working-discipline counterpart to the **Verified** principle: *nothing reaches production without validation* governs the product; grounding governs how the agent reaches every claim along the way — during `/specify`, `/clarify`, `/plan`, `/implement`, `/review`, and `/analyze` alike.

### Sources, in order of authority

1. **Code and artifacts** — source files, `spec.md`, `plan.md`, scenarios, rules, `system.md`, tests, migrations, config, and git history are the ground truth for what the system *is* and what was *decided*. Read the file; do not recall it.
2. **Live, reachable state** — a connected dev or read-only database, a running dev server, logs, a REPL, `--help` output, an actual test run. When such a source is on hand, query it rather than infer schema, data shape, or behavior.
3. **Inference** — permitted only for what no reachable source can answer, and then stated as an assumption, never asserted as fact.

<!-- §recommendations -->

### Recommendations

A recommendation is a claim, so grounding governs it. When the agent offers options and recommends one, it MUST work the recommendation out to its result **before** presenting it — not after the user accepts.

Where the options differ by a quantity the user cares about — steps removed, restarts saved, files touched, time or cost — that quantity MUST be computed for **every** option and stated alongside the recommendation. Reasoning from an option's *shape* to its *effect* ("this is the biggest change available", "this reuses the most existing machinery") is inference presented as analysis, and it is a defect on this principle whether or not the ranking turns out to be right.

When the deciding quantity cannot be computed yet, the agent MUST say so and describe what would settle it, rather than ranking the options anyway. "I don't know which is better until X is measured" is a usable answer; a confident ranking with an unchecked basis is not — it reads as analysis and carries none, so the user's approval rests on the framing rather than on the facts.

This is the same standard the pipeline applies to its own machinery: `/{project}:review` findings quantify their scope, artifact checks report what they examined, and a gate that cannot run must never look like one that passed. Advice about the system is held to the standard the system is held to.

### Rules

- **Prefer the source to the recollection.** Before asserting how code behaves, what a schema holds, what an artifact says, or what a command does, open it — a `Read`, a `grep`, or a query is cheaper than a wrong answer propagated downstream.
- **A connected database is a primary source, not a hazard.** When the project exposes a dev or read-only database (or equivalent live state), use it to confirm schema, constraints, and representative data instead of theorizing them. **Discover what is reachable** from the project's own configuration — environment files, service/compose definitions, framework config — and from any live-source pointer the project declares in its `AGENTS.md`; a declared pointer is authoritative when present, and its absence is not evidence that no source exists. Read-only access to such a source needs no special authorization — it is the default; treat every source read-only unless the task explicitly authorizes writes.
- **Name the assumption when you must infer.** When no reachable source can settle a load-bearing question, mark the claim as an assumption rather than laundering a guess into a fact. During a spec that assumption is an Open Question ([Spec requirements](#spec-requirements)); in a plan it is a labeled assumption in the plan body; during implementation it is captured to the inbox and surfaced at completion ([Automatic issue capture](#automatic-issue-capture)).
- **Cite what you consulted.** When a conclusion turned on a specific source, reference it (`path:line`, the query, the command) so the next reader can re-derive it rather than re-guess.
- **Before reporting that a declarative entry misbehaved, read the entry.** A registry row, a manifest line, or a config table carries its own preconditions — a sunset date, a strategy, a pinned flag, a surface filter — and a step that "should have run" may have been correctly excluded by one of them. When the observation is *"X did not happen"*, the first question is whether X was supposed to, and the answer lives in the declaration rather than in the procedure that consumed it. This is *prefer the source to the recollection* applied where the source is a data row, and the failure mode is worse there: a procedure reads plausibly while quietly honouring a field never looked at. State the entry's own gating fields in the finding, or do not file it.

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

The top-level directory name (`specs` above) is the documented default; a project may rename it via `.ductus/config.toml` `[paths] specs-root` (e.g. to avoid colliding with a sibling framework's `spec/`, like RSpec's). When the key is unset every command and the runtime default to `specs`, so an adopter who never sets it sees unchanged behavior. The literal `specs/` throughout this constitution and the command sources is that default; wherever a command or the runtime constructs a path under it, it resolves `[paths] specs-root` (spec 040).

<!-- §spec-requirements -->

#### Spec requirements

- Every spec includes a **Status** indicator: `draft`, `clarified`, `planned`, `in-progress`, or `done`
- Every spec includes **Acceptance Criteria** — concrete, testable conditions that define "done". Each carries a stable `AC{n}:` label after its checkbox (`- [ ] AC7: …`), assigned by the runtime's labelling pass and permanent for the life of the criterion: never renumbered when criteria are inserted, reordered, or removed, and never reissued after one is deleted. The label — not the criterion's position — is how a criterion is cited in prose, across specs, and by tooling (spec 013)
- Every spec includes **Open Questions** — uncertainties and unresolved decisions. An open question is an **undecided blocker**: a decision deferred pending a condition ("not now; revisit when X lands") is resolved *with* a condition and belongs in Resolved Questions with its trigger recorded, not left open
- Every spec lists **Dependencies** — other specs this feature depends on
- Open questions must be resolved before moving to the plan phase
- Specs describe behavior and contracts, not implementation
- **Never hand-write an `AC{n}:` label.** Add the criterion unlabelled and let the labelling pass assign it. The label is `max(highest label in the body, next-criterion)`, and `next-criterion` is frontmatter an author is not reading while drafting prose — so a hand-written label is a guess that happens to be right until it collides with a retired one. The counter is what stops a deleted criterion's label being reissued to a different requirement. If labels have already been written by hand, leave them: stripping them renumbers from the advanced counter and opens a gap for no gain.
- **Record a superseded acceptance criterion on the criterion itself, never in the review.** When a later spec removes something an earlier spec delivered, the earlier criterion stays ticked — it *was* delivered, and the removal belongs to the later spec — and gains an inline annotation naming the superseding spec. Cite that spec by name rather than by link, since a body link to a sibling spec is harvested into `dependencies:` and citing a remover is not depending on it. Phrase the annotation so it reads as a non-claim ("no longer exists", "is removed") rather than relying on whether its paths happen to resolve. The review artifact is the wrong home: it is regenerated wholesale on the next run, so a supersession recorded there is destroyed by the next review.

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

Forward edges only — `/clarify` raises status to `clarified`, `/plan` to `planned`, `/implement` to `in-progress` and then to `done`. The `in-progress → done` transition is gated by `/review`: `/implement` MUST NOT write `status: done` while the spec's `review.last-run` is unset or `review.blocking` is `true`. `/review` is a gate, not a state transition — it records findings and updates the `review:` frontmatter block, but does not change `status`. The gate composes with `/analyze` (which flags drifted `done` specs) and the shipped CI template (which fails PRs that bypass the local checks) per [§design-principles](#design-principles): never depend on human diligence. Three back-edges exist:

- **Backward via new questions** — `clarified` / `planned` / `in-progress` → `draft` when `/amend` records a new open question; the next `/clarify` resolves the question and the spec advances forward again. `draft` is the only status that tolerates open questions, so it is the destination; `/amend` performs the status mutation in the same write that records the question.
- **Backward via new scenario** — `done` → `in-progress` when `/amend` records a scenario. The scenario's task is implemented and the spec returns to `done`. A scenario that *carries open questions* takes this same edge, **not** the question edge above: that edge exists because `draft` is the only status tolerating open questions **in the spec body**, and a scenario's questions are a separate signal that leaves the body untouched. Reverting to `draft` would assert a body state that is not true and route to feature-targeted `/clarify`, which does not read scenarios. The questions still bind — a spec does not reach `done` while any remain (see the `done` row above) — but the routing pressure comes from that gate, not from the status.
- **Backward via meaningful body edit** — `done` → `in-progress` when any artifact under `specs/{feature}/` is edited *meaningfully*. An edit is **mechanical** (no back-edge) in any of three diff-determinable cases: **(a)** every change in the diff is the same find-and-replace token substitution, applied uniformly across the live artifacts enumerated in [§drift-prevention](#drift-prevention), mapping a deprecated label (slug, capability, command, identifier, parenthetical descriptor) to its current label; **(b)** every change in the diff adds, removes, or rewrites a **cross-service reference** — an inline body link whose target resolves to a registered `.ductus/config.toml` `[services]` entry, together with the regenerated `references:` frontmatter that harvests it — because such references are informative cross-service navigation, never dependencies, acceptance criteria, or behavior (spec 030); or **(c)** every change in the diff assigns a **runtime-maintained identifier** — an `AC{n}:` label written between an acceptance criterion's checkbox and its text, together with the `next-criterion:` counter that backs it — leaving every labelled criterion's own text byte-identical, because an identifier names a requirement without stating one, exactly as a rule ID does (spec 013). Anything else — new scope, changed semantics, factual corrections, restructuring, edits scoped to a single spec — is a **meaningful edit** and triggers the back-edge via the same `/amend` flow used for scenarios. The distinction is determinable from the diff alone, so the rule does not depend on author judgment.

The three cases share one test, and it is the test rather than the list that decides a case the list does not name: an edit is mechanical when the diff is **determinable without author judgment** *and* **changes no claim the spec makes** — no requirement added, removed, or reworded; no behavior described differently; no fact corrected. An edit that changes no claim is therefore mechanical even when it matches none of (a)–(c): repairing a typo, or sweep residue where a substitution landed in a sentence it did not fit, restores the text to what it already meant and asserts nothing new. A **factual correction is not this** — correcting a claim that was wrong changes what the spec asserts, and takes the back-edge. Stating the test rather than extending the list is deliberate: a closed enumeration makes every new case an argument about whether it deserves an exception, when the question is only ever whether a claim moved.

This avoids spec proliferation; scenarios evolve the existing spec rather than spawning a new one. Spec bodies are living documents that represent current state — git history is the historical record of what was written when.

Three operational rules follow from the lifecycle and apply on every project:

- **Re-open a `done` spec with the status primitive when the only intent is to reflect edits already on disk.** When scenario files or body edits are already written and the spec's `status` is the last inconsistency, set the status directly rather than routing through the refinement command — that command expects an input to classify, and manufacturing one either creates a second scenario or is treated as a no-op. Route through refinement only when there is genuinely new input to capture.
- **Syncing a canonical record that lives on another spec is a mechanical edit.** When behavior changes and the canonical-sources map points at a table inside some other spec's artifacts, update that table in the same change and leave the spec `done` — it is case (a), a uniform substitution. Leaving it stale would make a `done` spec's canonical record contradict shipped behavior, which is worse than the reopen it avoids. When the same table needs syncing more than once, move it to the spec that owns the behavior and leave a pointer.
- **Restoring a spec directory from git silently reverts uncommitted pipeline state.** A status flip, a ticked criterion, and a ticked subtask are all writes to tracked files, so a restore aimed at *content* takes the *bookkeeping* with it — and nothing reports the loss, because the pipeline reads status from the file just reverted. Before restoring, note the current status and checkbox state and re-apply them, or restore individual files by path. Better: commit a status transition as its own step, so a content restore cannot reach it. The same hazard applies to any stash, restore, or hard reset over a spec directory mid-pipeline.

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

<!-- §tasks-phase -->

### Tasks Phase

Break the plan into discrete, ordered work items. Each task is small enough to implement and verify independently.

#### Task requirements

- Tasks are derived from the plan, not invented independently
- Each task has a clear definition of done
- Tasks are ordered to respect dependencies
- A task can be completed in a single working session

`tasks.md` is an **ephemeral work-tracking artifact** — a view of what is left to do, derived from the plan. It is not a durable source of truth: a task's value is spent the moment its checkbox is checked, because the durable record of what was built lives in the spec, its scenarios, the rules, and git history — never in a checked-off box. This is the same durability test [§bug-handling](#bug-handling) applies to chores, stated here for `tasks.md` directly. Completed task sections may therefore be pruned — or the file reset to its template state — with `/{project}:prune` without loss, and no consumer of `tasks.md` may treat its content as a durable index (including `/{project}:analyze`'s scenario-consistency check, which does not require an implemented scenario's task to persist). This stands in contrast to the durable artifacts: `spec.md`, scenarios, and rules carry the requirements and decisions that must stay accurate as the project evolves, with `plan.md` and `data-model.md` as the design record — none of which `/{project}:prune` touches.

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
- **A spec's ticked acceptance criteria are verified against the tree before it closes.** A ticked criterion is a completed *claim*, and closing a spec is the moment to re-earn it rather than bank it. Two gaps make this necessary and neither can be closed by a check. The artifact check that compares criteria to the tree examines specs at `done` **only** — correctly, since a criterion on a spec still in progress may name a path not yet created — so a spec that sits in progress indefinitely never has its criteria examined at all. And that check proves only that a path *resolves*: a criterion whose paths all exist while its claim about their contents is false is invisible to it. Read the criteria against the tree, and treat a criterion whose claim no longer holds as a supersession to record (see [§spec-requirements](#spec-requirements)) rather than a checkbox to leave standing.
- **A spec does not reach `done` with an outstanding SHOULD.** The review gate blocks on MUST violations alone and records SHOULD findings as advisory — but advisory is not "ignorable at the gate". A finding is addressed when it is **fixed**, or when it is moved under the review's waived section with its rationale, which is the disposition for a SHOULD whose answer is "keep as-is". What is not acceptable is a spec sitting at `done` with a non-zero SHOULD count and the finding still filed under its original heading: that state is indistinguishable from unfinished work, and nothing ever comes back to it. Read the SHOULD count at the completion gate the same way the MUST count is read.
- **Record a review against a commit that already contains what it reviewed.** The review records the HEAD it ran against, so the natural order — edit, review, commit everything together — records the HEAD from *before* the edits landed, and every durable contract in that commit then reads as changed since the review. Two commits, always: the work, then the review against that new HEAD, then the review itself. Freshness checks read committed state, so they must be *asked* about committed state — a check run before committing passes for the wrong reason.
- No work happens outside the tasks list — if new work is discovered, add it as a task first
- Refactoring that preserves existing behavior and contracts does not require a spec or scenario update. If a refactor reveals a missing requirement or changes documented behavior, update the spec or add a scenario to capture the new expectation before proceeding.
- Before the spec advances to `done`, `/{project}:review` runs against the implementation and the spec's frontmatter `review:` block records the result. The transition is gated: `/{project}:implement` halts when `review.last-run` is unset or `review.blocking` is `true`. See §spec-lifecycle.

<!-- §constants -->

#### Constants and configuration

See `framework/rules/configuration-cross.md` (`CFG-CONST-NNN` rules) for the enforceable rules covering centralized shared constants, module-local constants, and the no-bare-literals requirement for operator-tunable values. `/{project}:analyze` enforces these rules.

<!-- §env-vars -->

#### Environment variables

See `framework/rules/configuration-cross.md` (`CFG-ENV-NNN` rules) for the enforceable rules covering env-var defaults backed by named constants, `.env.example` completeness, fail-fast startup validation, and unit suffixes for time-valued variables. `/{project}:analyze` enforces these rules.

<!-- §bug-handling -->

## Bug Handling

Bugs are unwritten or violated requirements. Every bug is evidence that one of the framework's three artifact tiers — rules (cross-cutting), specs (feature-wide), or scenarios (situational) — has a gap. Rather than tracking defects in a separate system, fixing a bug means making the requirement at the right tier more precise. See [§rules](#rules) for the rule tier and [§scenarios](#scenarios) for the scenario tier.

Not every captured item is a requirement gap. An inbox item may be a **chore** — a discrete piece of project maintenance (lint or formatting cleanup, dependency cleanup, repo hygiene, a standalone refactor) that adds no missing or violated requirement and belongs to no single feature. A chore does **not** spawn a rule, spec, or scenario, and it is **not** a spec task — a spec's `tasks.md` holds work derived from that feature's plan, never standalone chores. It stays tracked as a checkbox in `specs/inbox.md` (the project's non-feature work surface) and is resolved by being *done*, then removed — not migrated to a spec. The test is **durability**: rules, specs, and scenarios hold durable information that must stay accurate as the project evolves — feature description and context, acceptance criteria kept current, resolved open questions that serve as the project's architecture-decision record, and cross-cutting rules. A chore captures none of that; it is transient work whose value is spent once complete. Route requirement gaps through the decision tree below; leave chores in the inbox to be done directly.

### Bug Decision Tree

When a bug is reported, follow this decision tree in order. The first matching condition determines the route:

1. **No rule covers this cross-cutting concern** — the bug surfaces a class of behavior the framework should ductus at the rules tier (perf budget, observability commitment, security control, accessibility minimum, etc.). Promote to a rule (new or amended), then fix the code.
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

The scenario-creation primitive frames the body it is given: it writes the frontmatter, the heading, *and* the Open / Resolved Questions scaffolding. **Do not author those question headings in the body passed to it** — a body already carrying them produces two, which the markdown linter rejects as a duplicate heading. Pass Context, Behavior and Edge Cases only, then edit the scaffolded questions section afterwards; write the whole file directly when it needs questions at creation time.

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
- **Statement** — one sentence using RFC 2119 keywords (MUST, MUST NOT, SHOULD, SHOULD NOT). MUST/MUST NOT rules are blocking; SHOULD/SHOULD NOT are advisory.
- **Rationale** — the threat or risk the rule mitigates.
- **Verification** — instruction to the validate agent on how to check compliance against feature artifacts.

The full schema, ID-stability invariants, the ID grammar (including the `[A-Z][A-Z0-9]*` category-abbreviation format), and Verification phrasing rules are canonically declared in `specs/008-security-rules/data-model.md` — and, for configuration rules, in `specs/017-derive-dont-ask/data-model.md`. The specific category abbreviations a given rule file uses are declared in that file's own header (e.g., `api-backend.md` declares `SCHEMA`/`APIVER`/…). New rule files follow the same schema.

#### When to write a rule

**A new rule belonging to an existing rule surface is added to that surface's owning spec through the back-edge — it does not get a spec of its own.** Register the category, add the rule, add a task, and let the owning spec return to `done`. A spec per rule fragments the durable record of one concern across many specs, which is the anti-proliferation stance [§spec-lifecycle](#spec-lifecycle) exists to hold. Keep the cross-cutting principle in its single canonical home and have each enforcement point reference it. Reserve a new spec for a genuinely new rule *file* or surface.

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

Enforcement is two-layered. In `ductus`'s own repository, `scripts/lint-rule-filenames.sh` fails CI on any file that violates the closed-suffix policy. In adopter repositories — where the lint does not run — a rule file with an unrecognized suffix loads for every stack and emits a one-line stdout warning (`rule file <name> has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md`). The default is "load + warn," never "silent skip."

#### Project-level opt-out

A project may exclude a stack-selected rule file from `/{project}:review` by listing it in `.ductus/config.toml` `[[review.disabled-rule-files]]` with a mandatory `reason` — the reason is the audit trail for the override, surfaced on stdout at the start of every run. The opt-out is project-wide and applies to whole files; per-`(rule, file)` exceptions remain the job of `/{project}:review --waive`. Schema and behavior are documented in [`framework/commands/review.md`](commands/review.md).

#### Lifecycle

- Rule IDs are permanent. Once assigned, an ID is never renumbered, even if the rule moves within the file or is edited.
- Rules are deprecated with a `**DEPRECATED in {version}:**` label and a removal target version, then removed only after the deprecation window has passed.
- New rule files **that ship with `ductus`** are introduced via their own feature spec (the same way 008 introduced `security-backend.md` and `security-frontend.md`). A rule file a project authors for itself has no introducing spec and needs none — placing it in the rule-file directory is the whole registration step, and its own header is where its ID prefix and category abbreviations are declared (as above). Every consumer of the rule set treats the two origins identically; nothing may condition loading, citation resolution, or validation on a rule file having an introducing spec. **Recorded exception (backfill):** `api-backend.md`, `accessibility-frontend.md`, and `performance-frontend.md` were introduced in commit `9ccbd0b` bundled into specs 024/025 rather than through their own introducing specs. They are in active use — discovered by the suffix directory-walk and cited by ID like every other rule file — and their ID grammar is reconciled with this section, so they are retained as-is; no retroactive introducing specs are required.

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

A `specs/inbox.md` file is the project's capture queue for issues not yet assigned to a feature spec. It serves two roles:

- **Brownfield migration** — for projects adopting `ductus` incrementally, known issues are parked here until a spec exists to absorb them.
- **Incidental capture** — issues an agent discovers as a side effect of other work are recorded here automatically (see [Automatic issue capture](#automatic-issue-capture) below).

Items are recorded with `/log` (manual) or captured automatically during work, and groomed into their proper home with `/groom`. An item's "proper home" is usually a feature spec, scenario, or rule; an item that is a **chore** (project maintenance belonging to no feature — lint or dependency cleanup, repo hygiene, see [§bug-handling](#bug-handling)) has no spec home and is resolved by being done directly, then removed — `/groom` recognizes it and leaves it in place rather than forcing it into a spec.

Inbox rules:

- Do not frontfill bugs that are not being actively worked on
- Write specs for areas being actively touched — let adoption spread naturally
- As specs are written, `/groom` migrates items from the inbox into their proper home
- **When an item routes to a chore, fix it — do not park it.** The chore route means the item is done directly, in the same pass, and then removed because it is resolved. A parked chore is re-read and re-routed on every subsequent grooming pass, so the recurring cost of carrying it exceeds the one-time cost of the fix, and the inbox stops reflecting real backlog. Leave one in place only when the fix is genuinely blocked or turns out not to be mechanical after all — and say which.
- The brownfield-migration backlog drains toward empty as adoption completes; the incidental-capture role is ongoing, so the file persists as long as new work keeps surfacing new issues

#### Automatic issue capture

Findings reach the inbox two ways, and both MUST be captured, not dropped. **Incidentally**: while working a task, an agent surfaces issues that fall outside the current task's scope — a security weakness, a resource or memory leak, a violated convention, a latent bug in adjacent code. **As primary output**: a command whose whole purpose is to produce findings — `/{project}:analyze` — records what it found rather than only printing it. A findings-producing command that discards its findings is the failure the **Design Principles** rule names directly, since recovering them then depends on someone remembering to re-run the audit.

- **Capture automatically, without prompting.** When an agent identifies such an issue during any task work, it appends the issue to `specs/inbox.md` itself — the same mechanical append `/log` performs — without pausing to ask the user. Capture is not a pipeline gate; it never interrupts the task in flight.
- **Record, do not derail.** The agent does not stop the current task to fix an out-of-scope issue. It records the finding and continues. An issue *inside* the current task's scope is fixed as part of the task, not logged.
- **Severity raises salience, not the routing.** Security issues and memory or resource leaks are the cases most costly to lose, so they are captured first and flagged; convention violations and lesser findings are captured the same way. Everything routes through `/groom` later — capture itself is uninterpreted.
- **Surface at completion.** Issues captured during a unit of work are presented back to the user when that work completes — as part of the `/{project}:implement` completion summary, the `/{project}:review` report, and the `/{project}:analyze` report. The surfacing step is the backstop that keeps capture from being silent: per the **Design Principles** rule, the framework does not rely on the agent *remembering* a mid-task finding, it makes every capture visible at the next gate.
- **Re-capture is idempotent.** A command that captures on every run guards each append against what the inbox already holds, so re-running an audit against an unchanged repo records nothing new. Without the guard, the honest choice between a growing backlog and a silent one would push toward silence.

This keeps the agent's attention on the task while guaranteeing that discoveries — incidental or audited — reach the inbox and, through `/groom`, the right artifact tier ([Bug Decision Tree](#bug-decision-tree)).

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
- **A markdown link in a spec body creates a `dependencies:` edge — cite in prose when you mean a citation.** The dependency generator harvests every inline link to a sibling spec and rewrites the frontmatter from it, so a link added purely to reference another spec's history or review silently declares a dependency, and the pre-commit hook applies it before anyone looks. To cite without an edge, name the spec in prose or backticks rather than linking it, or place the link under the section the generator's opt-out exempts. After editing a spec body, read the `dependencies:` line the hook reports before committing.
- **Write pipeline state files with the file-writing tool, not shell redirects.** Permission entries that scope writes to a specific pipeline-owned path — the session file above all — grant the editing and writing tools, not shell redirection, which falls under separate command permissions. Reaching for the right tool is cheaper than widening a shell allowlist, and widening it to compensate grants write-anywhere-via-shell and defeats the per-path scoping the entry exists for.
- **Never stage the whole worktree in a project running this pipeline.** A blanket `add` sweeps untracked in-progress spec drafts into a commit, which the tracked-specs rule forbids: the generators and the pipeline both scope to the git index, so a draft that is not yet added is deliberately invisible to them. Stage explicit paths.

### Frontmatter Schema

The frontmatter schema applies to **spec files** (`spec.md`) and **scenario files** (`scenarios/{slug}.md`). Other `ductus` artifacts (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan files, tasks files, rule files, README files) MAY include frontmatter when a specific consumer benefits, but are not required to.

#### Spec files

| Field | Required | Type | Allowed values | Description |
| --- | --- | --- | --- | --- |
| `status` | yes | string | `draft`, `clarified`, `planned`, `in-progress`, `done` | Spec lifecycle state |
| `dependencies` | yes | list of strings | spec slugs (e.g., `002-events`); empty list permitted | **Generated** by `.ductus/scripts/gen-spec-deps.sh` from inline markdown links to sibling specs in the body. Not hand-authored. Author opt-out: links under a `## See also` heading are treated as navigational and do not produce edges (`## References` remains a dep-producing section). |
| `references` | no | list of `{service, spec}` entries | registered service alias + target `NNN-slug`; empty or absent permitted | **Generated** by `.ductus/scripts/gen-cross-service-refs.sh` from inline body links to a registered service's canonical repo URL. Not hand-authored, and **strictly distinct from `dependencies`** — informative cross-service navigation that never enters the blocking dependency graph (spec 030). |
| `next-criterion` | no | integer | ≥ 1; absent means no criterion has been labelled yet | **Maintained by the runtime's labelling pass.** The `AC{n}` label the next acceptance criterion receives. Monotonically non-decreasing — deleting a criterion never lowers it — so a retired label is never reissued to a different requirement. Not hand-authored; the audit requires it to exceed every `AC{n}` label present in the body (spec 013). |

#### Scenario files

| Field | Required | Type | Allowed values | Description |
| --- | --- | --- | --- | --- |
| `section` | yes | string | parent spec section name (e.g., `"Authentication flow"`) | The section of the parent spec the scenario elaborates. The parent feature is implicit in the file path. |

#### Open-schema rule

Additional fields beyond those listed above are permitted and ignored by uninterested consumers. Examples adopters or future `ductus` work might add: `owner`, `target_release`, `created_at`, `description`, `aliases`. Consumers MUST NOT error on the presence of unknown fields. `/ductus:analyze` reports unknown fields as informational findings (not errors). Stale fields in done specs (e.g., `title`, `tags`, `spec-ref`, `track`) remain valid under this rule and produce no findings.

### Validation Severity

`/ductus:analyze` checks frontmatter against this schema with the following severity:

- **Hard fail** — frontmatter block missing on a spec or scenario file; frontmatter YAML malformed; `status` missing or not in the allowed set; `dependencies` missing or not a list; both `section` and the legacy `spec-ref` missing on a scenario.
- **Advisory** — cross-reference checks; body inline links to sibling specs that are not yet in the generator-managed `dependencies` (informational — the next commit's `gen-spec-deps.sh` run will resolve).
- **Informational** — unknown fields present.

Hard fails block the validation pass. Advisory and informational findings are reported but do not block.

For non-frontmatter checks (spec integrity, artifact completeness, plan/task consistency, dependencies, security rules), `/ductus:analyze` adds a fourth tier — **Blocking** — between Hard fail and Advisory. Blocking findings are structural or content issues that must be fixed before the next pipeline gate fires (e.g., missing `plan.md` on a `planned` spec, an unknown rule ID referenced in a spec). Hard fail and Blocking both prevent pipeline advancement; the distinction is that Hard fail says "the spec file itself is malformed," while Blocking says "the artifact set is incomplete or inconsistent." See `framework/commands/analyze.md` for the full per-check severity assignment.

<!-- §runtime-boundary -->

### Runtime Boundary

`ductus` ships a runtime binary alongside the markdown framework, acquired by `/{project}` during adoption. The runtime exists to execute the deterministic portions of pipeline commands without an LLM. This subsection defines what the runtime can and cannot do; deviations require their own constitutional amendment.

#### Five principles

1. **Markdown is source of truth** — the runtime MUST NOT own state the markdown cannot reconstruct. Runtime-owned data (caches, indexes, parsed graphs) is derived and gitignored, per the existing rule on structured derived views.
2. **Determinism only** — the runtime MUST NOT call an LLM. Work requiring semantic judgment (content quality, `/clarify` resolution, `/specify` sketching, per-rule Verification reads, `/groom` routing) stays in slash commands.
3. **Required, and acquired by the pipeline** — `/{project}` acquires the pinned runtime as part of adoption, so a bootstrapped project has the binary and pipeline commands MAY assume determinism rather than specifying two executable paths to one result. The runtime is *ductus-owned*: acquired and version-managed by the pipeline into a store it writes, never whatever `PATH` happens to resolve. A project that supplies its own binary declares it (`[runtime] path`) and is equally supported; acquisition failure halts the run rather than degrading, because a requirement that quietly is not one leaves both paths alive. Shell pipelines that parse frontmatter or markdown structure (`awk`, `sed`, `grep` pipelines, `for` loops over files) remain **not** a sanctioned substitute for the runtime primitives or the host's file tools.
4. **Schema follows the constitution** — the runtime MUST read frontmatter and artifact structure according to the schemas declared in this document. Schema changes ship through the constitution; the runtime MUST update to match. The constitution MUST NOT import runtime types.
5. **MCP is the seam** — the runtime MUST expose its capabilities as MCP tools so slash commands can call them when they want determinism. This keeps the runtime accessible to any agent host and prevents `ductus`-specific coupling.

<!-- §runtime-host-integration -->

#### Host integration (for agent runtimes)

The backticked primitive names in a rewritten command's Instructions section map to the MCP tools the runtime exposes under bare `<verb>-<noun>` names. A host wraps them with a server-name prefix taken from its MCP registration — Claude Code: `mcp__ductus__<verb>-<noun>`; Auggie / Antigravity: `mcp:ductus:<verb>-<noun>`. When the `ductus` server is registered for the session, the agent **calls the corresponding tool** for each step — the deterministic path. If a host loads MCP tool schemas lazily (e.g., Claude Code lists tool names in a deferred-tool reminder before exposing their schemas), the runtime is still registered: the agent fetches the schema through the host's mechanism (`ToolSearch` on Claude Code) and calls the tool rather than bailing to the fallback. When no `ductus` server is configured, the agent walks the same prose with the host's file tools (`Read`, `Edit`, `Write`); the shell-pipeline substitutes named in principle 3 are **not** a sanctioned stand-in for either the runtime primitives or those file tools. The two paths share one contract; neither wraps the other. A rewritten command opens its Instructions with a one-line pointer to this subsection (§runtime-host-integration) rather than restating it — the contract lives here once.

#### Eligibility criteria

A capability is runtime-eligible only when **all three** hold:

1. **Deterministic** — no semantic judgment required; the same inputs always produce the same outputs.
2. **Currently mechanical** — already either (a) executed by an LLM following procedural instructions in a slash command body, or (b) implemented as a bash script the framework invokes (pre-commit hooks, generators, CI).
3. **Specifiable as prose** — the capability can be stated completely enough that the Markdown-only reference documents it, and a primitive mirrors that reference rather than introducing policy of its own. This is what keeps the specification and the implementation one thing: the reference is where the policy lives, the primitive is how it runs.

A capability that fails any criterion stays out of the runtime. Anything that requires reading prose for intent is permanently LLM-owned regardless of how mechanical its surface looks.

#### Acquisition invariant

The repository's CI MUST include a job that exercises acquisition end-to-end on every supported platform: fetch the published asset for the target, verify its sidecar digest, install it into a temporary store, and execute the installed binary. A change that causes this job to fail — i.e. a release whose assets an adopter cannot actually acquire — is a constitution violation, not a feature.

This replaces the **opt-in invariant**, which asserted a full pipeline cycle with the binary absent from `PATH`. That job tested the guarantee principle 3 used to make; the guarantee it now makes is that the binary is *obtainable*, and the job that proves it is the one that fetches it. Amended by [048](../specs/048-govern-acquired-runtime/spec.md).

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
| Service registry schema (`.ductus/config.toml` `[services]`) | `specs/030-cross-service-references/data-model.md` |
| Where contributor knowledge is recorded (git vs. per-user agent memory) | `framework/constitution.md` §drift-prevention (Shared knowledge stays in git) |
| Open-state tell list and decision-drift check grammars | `specs/045-decision-state-drift-detection/data-model.md` |
| Scenario→task referencing rule (what counts as a task referencing a scenario) | `specs/022-deterministic-runtime/data-model.md` (`scenario-consistency`) |
| Constitution content — what belongs in it, how it is organized, which rules adopters receive | `specs/050-constitution/spec.md` (a spec that changes *behavior* still amends the principle its change contradicts, in the same change) |

When adding a new kind of fact that may be referenced from multiple documents, name its canonical source explicitly here.

### Cross-document references

When document B describes content authored in document A, B includes a back-link to A — relative markdown link, anchor reference (`§anchor`), or section name. Two consequences follow:

- Changing A includes auditing every back-link to A. The audit is structured wherever it can be machine-checked (anchor resolution, help-table descriptions, registry-frontmatter equivalence), and a manual sweep otherwise.
- Adding a fact that conceptually belongs in A but landing it in B is drift. Either move the fact to A and back-link, or extend A's scope explicitly.
- **No dead references in live artifacts.** When renaming or removing a name — a spec slug, a capability, a command, an identifier, even a parenthetical descriptor — update every reference across the project's live artifacts in the same change: specs (including `done` spec bodies), rules, command sources, scripts the pipeline runs, CI configuration, docs, and the README. A reader following a pointer must never land on an outdated name. The sweep is uniform find-and-replace, which makes it a mechanical edit under [§spec-lifecycle](#spec-lifecycle) — `done` specs stay `done`. Do not bundle it with unrelated edits: a non-uniform diff is a meaningful edit and reopens what it touches. **Keep the sweep's own target list current** — when an earlier change relocated a directory, a list still naming the old location sends the grep somewhere clean and the sweep silently misses the files that moved.
- **A behavior change needs a prose-claim sweep, not just an identifier sweep.** The rule above catches renamed *names*; a change to what the system *does* additionally needs a sweep for stale *claims* about the old behavior — and those claims contain none of the changed tokens, so a path- or identifier-scoped grep passes straight over them. Enumerate the claims the change falsifies and grep for them **by meaning**, across the live artifacts and the README especially, since it narrates behavior to users. Fixing a stale claim in docs is docs-only; a stale behavioral claim inside a `done` spec body is a meaningful edit and takes the back-edge.
- **Never edit an installed command file directly.** A file the installer places is overwritten on its next run, so an edit made there is lost without warning. Change the source the installer copies from, or pin the file in the project's configuration to opt it out of updates — pinning is the supported way to keep a local modification.
- **Renaming a repository orphans contributor-local state that no migration can reach.** A migration converges state named for the project; it cannot touch state keyed to the project *path*, because that lives outside the repository and differs per contributor. Nothing errors — the state is simply never found again — so this needs a checklist rather than a check. After renaming, on **each** machine: rename the local checkout to match, repoint the remote, move any per-project agent state stored under a path-derived slug (and correct the absolute paths recorded inside it, which a copy alone leaves pointing at the old location), and fix anything else keyed to the old path — shell aliases, editor workspaces, worktrees.

### Decision resolution

Resolving a decision carries the same audit obligation as editing a document.

The rule above triggers on *editing document A*, and audits the back-links to A. A resolution does edit some document, but the event a contributor recognizes is "the question got settled", not "a file changed" — and the artifacts that describe a decision's state are not always the ones that link to it. The recognizable events:

- an open question is closed;
- a scenario is implemented and ships;
- a spec, scenario, or task advances its status;
- a previously-rejected option is adopted.

When one fires, every artifact that described the prior state is corrected in the same change. **A resolution is not complete while a sibling artifact still describes the prior state** — the question as open, the option as rejected, the work as unbuilt. Such an artifact does not read as stale, which is what makes it costly: a settled design decision still described as an open obligation reads as work owed, and an acceptance criterion naming a deleted path reads as a contract satisfied.

The deterministic part of this audit is machine-checked by `/{project}:analyze`; the rest is a manual sweep, as above.

### Template-rule alignment

Every blocking check in `/{project}:analyze` has a corresponding scaffolding element in the template that produces a passing artifact by default. The contract runs in both directions:

- Adding a new blocking check requires a template update so a freshly-copied artifact passes the check without manual editing.
- Adding template structure requires a corresponding rule (validate check, constitution rule, or both). Sections that don't trace back to a rule are dead weight.

Templates and validate evolve together. A diff that touches one without the other is incomplete.

### Manifest discipline

When multiple commands distribute or reference the same set of files (e.g., `/ductus` and `/{project}:init` both scaffold a project; `/{project}:configure` and the bootstrap install both apply permission sets), the file list lives in one place:

- Either as a shared section the commands include by reference, or
- As a registry both commands read.

Two commands that copy-paste the same manifest into their own bodies are guaranteed to drift over time. Consolidate or accept that drift is the rule, not the exception.

### Shared knowledge stays in git

Knowledge that would help any other contributor belongs in a git-tracked artifact, never in an AI agent's per-user memory. Per-user memory stores (Claude Code's auto-memory, Cursor's memories, and equivalents) live outside the repository — invisible to every other contributor and absent from a fresh clone — so a fact parked there is guaranteed to drift from the shared source the moment anyone else works the area. It is the most severe form of the drift this section exists to prevent: not an inconsistency between two committed documents, but knowledge that was never committed at all.

- A **project learning** — a convention, a gotcha, a workflow rule, a boundary — goes in `AGENTS.md` (or the matching rule file under `specs/rules/`), where every contributor and every agent reads it.
- A **durable requirement** goes in its canonical artifact: a spec, a scenario, a rule, or this constitution (see [Canonical sources](#canonical-sources)).
- **Per-user agent memory** is correct only for facts that carry no value to other contributors — who the individual user is (role, persistent personal preferences) and external reference pointers (issue-tracker, chat, or dashboard bookmarks).

The test before saving to per-user memory: *would this help a teammate?* If yes, commit it. A host's own rules file (`CLAUDE.md`, `AGENTS.md`) supplies the agent-specific routing that applies this principle.

<!-- §pipeline-boundaries -->

## Pipeline Boundaries

- Never implement without a spec
- Never plan without resolving open questions
- Never skip phases — each phase produces artifacts the next phase consumes
- Never transition a spec to the next status without explicit user approval — present the work done and wait for the user to confirm before updating the status field
- Specs and plans are living documents — update them when decisions change, but don't backtrack silently

<!-- §concurrent-features -->

### Concurrent Features

The session state file (`.ductus/session.toml`) holds a single target by design. The pipeline is serial within a feature, and concurrent work on independent features uses two independent sessions in two terminals — not multi-target session state. Isolation is provided by the platform layer: `git worktree` keeps the working trees separate, and AI-agent platforms typically expose isolation primitives (Claude Code's `isolation: "worktree"` agent parameter, Cursor's worktree integration, etc.). Reach for those rather than asking `ductus` to track multiple targets at once.

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

The project configuration file is an exception, and it is worth stating because the reflex is to treat it like any other shared artifact: **it is a shared project-side database, not a schema owned by whichever spec first wrote to it.** When a new spec adds a section or key — its own table — that spec documents it, and no signpost is generated on the earlier specs that happen to write to the same file. Treat a configuration change as cross-spec impact only when it modifies an *existing* key another spec already documented.

The originating spec's acceptance criteria include delivering the cross-spec update. This ensures the change is tracked as part of the work that discovered it.

<!-- §numbering -->

## Numbering Convention

Feature directories use three-digit zero-padded numbers: `000-skeleton`, `001-observability`, `002-events`. Numbers establish creation order and suggest a natural implementation sequence, but dependencies between features determine the actual build order.

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
