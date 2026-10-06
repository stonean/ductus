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

Per-task token tracking and budget ceilings require instrumenting the model call itself, which [§runtime-boundary](#runtime-boundary) principle 2 forbids the `ductus` runtime from doing — so that work belongs to the AI platform. `ductus` contributes by offering cost-aware patterns the user can opt into. The current levers: the stuck-detection step in `/{project}:implement` catches runaway loops before they compound spend; default-off autonomy keeps the human in the loop unless `--auto` is explicitly passed. For runtime cost controls, point the adopter at the platform's tooling — Claude Code's `/cost`, the Anthropic usage dashboard, Cursor's request limits, and equivalents.

<!-- §design-principles -->

### Design principles

Constraints on anything the pipeline asks an author or a check to do. Each is a
hard filter rather than a tiebreaker: a design that fails one is redesigned or
deferred, not shipped with a note. The list carries no count, because a count is
the first thing to go stale when the list grows.

- **A check that cannot run MUST never be indistinguishable from a check that passed.** When a script, gate, or artifact check skips part of its subject, cannot reach it, or has no basis to inspect it, it MUST say so — a distinct result, a `guidance` field, a count of what *was* examined — rather than the same zero, empty list, or success string a genuinely-clean run produces. The failure is asymmetric and silent: nothing errors, a gate passes, and the missing check surfaces later when someone re-derives the result by hand. The states where a check *cannot* run are disproportionately the states where something is wrong. When adding a gate, prove it **fails** before trusting that it passes — break the thing it guards and watch it go red. This is `QUAL-CLAIM-001` (see [§rules](#rules)) applied to the pipeline's own machinery.
- **A check that reads git history must be given git history.** CI checkouts default to a shallow clone carrying a single commit, so any check that resolves a recorded sha, diffs against an earlier commit, or walks the log sees nothing and reports every subject as unresolvable — while passing locally on every developer machine, where the clone is complete. Configure a full-depth checkout in **every** job that runs such a check, and say why in a comment, because the default is invisible until it bites. A local pass is not evidence the check works in CI: the environments differ in exactly the dimension the check depends on.
- **A test that reads history or shells out to a project script is a change to the job that runs it, not only to the suite.** The visible half is the test; the invisible half is that its job now has an environment requirement (full history) and a new input (whatever it shells out to, which belongs in that workflow's trigger paths). Miss the first and the test cannot run; miss the second and it silently stops running when the thing it checks changes. Prefer a test that fails loudly when its inputs are missing over one that skips — a skipped conformance test is the previous principle in its purest form.
- **Work that is not complete MUST never be indistinguishable from work that is.** This is the first principle turned on the pipeline's own output: a check that cannot run must not look like one that passed, and work left undone must not look finished. When something is known to be outstanding — a defect found at the completion gate, a claim that no longer holds, a scope quietly narrowed, a question deferred — there are exactly three dispositions. **Fix it.** **Record it where the pipeline will surface it again** — a task, a scenario, an acceptance criterion, a review finding moved under the waived section with its rationale — and let the status follow that record rather than run ahead of it. Or **decide it is out of scope and record the decision with its reason.** The inbox is none of the three: no gate reads it, so work parked there is indistinguishable from work that is done ([Finding dispositions](#finding-dispositions)). Disclosing it in prose alongside a completion claim is none of the three, and it is the most tempting failure available because it feels like candour: the status says `done`, the exception is stated once in a summary or a commit message nobody re-reads, and nothing ever comes back to it. A status that means *"done except for what was mentioned at the time"* carries no information, and the next reader has no way to recover what the caveat said. Where the residue is knowable only by measuring — how many instances exist, whether a check is warranted, what a fix would cost — **measure it**: an unmeasured gap is a task, not a caveat.
- **Never design a pipeline feature that depends on human diligence.** Any artifact section, frontmatter field, command behavior, or workflow step that requires an author to *remember* — to fill something in, set a flag, or update a document alongside a change — will be skipped, and skipped precisely in the cases where it mattered most. When proposing a new input, ask what happens when an author forgets. If the answer is "the feature degrades silently," derive the input instead — from existing artifacts, frontmatter, or history — or do not ship it. Deferring the feature is the correct outcome when no derivable design exists; shipping the disciplined version "for now" is not.
- **A check whose subject is drawn from repository state can go vacuous, and a naive vacuity guard will then demand that an unhealthy repository exist.** A check that compares two implementations, or asserts a property, over "every artifact in state X" has a subject only while something is in state X. Guarding it with *fail when nothing was compared* is right in spirit — a green run over an empty set is the first principle above — but it inverts once the condition stops occurring: the guard fires precisely when the thing it watches for has stopped happening, so a healthy repository turns the check red. **Build the subject** instead — a fixture carrying the condition under test, asserted against a known verdict in both directions, with the vacuity guard placed there, where it can never be empty. Let the pass over real state keep running as an additional check, and have it report *nothing to compare* as a distinguishable, non-failing state, since a healthy repository and a broken check must not render alike. The tell that a check is in this shape: its subject is a filter over repository state rather than something the check constructs, and it has been green for months with nobody knowing which item satisfied it.

<!-- §grounding -->

## Grounding

Work from what can be observed, not from what can be guessed. When a question can be answered by consulting a source that is actually reachable — the code, the project's own artifacts, a connected dev database, runtime output — the agent MUST consult that source before answering. Reasoning from the conversation alone, when a primary source was available and went unread, is a defect regardless of whether the guess turned out to be right.

Grounding is the working-discipline counterpart to the **Verified** principle: *nothing reaches production without validation* governs the product; grounding governs how the agent reaches every claim along the way — during `/specify`, `/clarify`, `/plan`, `/implement`, `/review`, and `/analyze` alike.

### Sources, in order of authority

1. **Code and artifacts** — source files, `spec.md`, `plan.md`, scenarios, rules, `system.md`, tests, migrations, config, and git history are the ground truth for what the system *is* and what was *decided*. Read the file; do not recall it.
2. **Live, reachable state** — a connected dev or read-only database, a running dev server, logs, a REPL, `--help` output, an actual test run. When such a source is on hand, query it rather than infer schema, data shape, or behavior.
3. **Inference** — permitted only for what no reachable source can answer, and then stated as an assumption, never asserted as fact.

<!-- §governance-precedence -->

### Governance sources, and which governs

The ranking above orders **evidence**. It does not order **governance documents**, and the two questions are different: one asks what is true, the other asks what binds. A project may load more than one governing document — the constitution `ductus` ships, and any shared constitution its `.ductus/config.toml` registers under `[constitutions.*]` (spec 055) — and where they disagree, this is the order:

**The framework constitution is a floor.** A shared or project source may add rules and tighten existing ones; it may not loosen them. Where the framework is silent, the more specific source wins: **project > shared > framework**. The framework constitution is what every command reads to know how to behave, so a source able to disable one of its gates would make the pipeline's own invariants negotiable per repository.

**Nothing enforces this.** No check compares two governing documents for contradiction, and none is planned — prose contradiction is not mechanically detectable. This is the rule for whoever resolves a disagreement, and stating it as anything more would be the defect [§grounding](#grounding) names one paragraph down: a rule that implied enforcement it does not have.

### A partial read is not a read

Grounding governs *whether* a source was consulted. It governs the **completeness** of that consultation too, because a read that returned part of its subject satisfies "read the file" while delivering something else.

A tool that elides content has not delivered the source. Truncation to a preview, a saved-output pointer, a page or line cap, a `head`/`tail` window, a match-only search — each returns a fragment. The elided remainder is unread, and the prohibition above applies to it unchanged: reasoning as though it said nothing is reasoning from a source that went unread.

There are two dispositions, and prose is not a third. **Read the remainder** — follow the pointer, page through the ranges — or **state what was not examined**, in the artifact where a later reader meets the claim rather than in a summary they will never see. Where both a reader that reports its subject's size and takes an offset and a command whose output is capped downstream are available, the explicit reader is the grounded choice: a downstream cap is applied after the fact and says nothing about what it dropped relative to what was needed.

This is `QUAL-CLAIM-001` turned on the agent instead of on its code. That rule requires a result to distinguish *examined and found nothing* from *could not examine*; this requires the agent's own reading to do the same. The failure is asymmetric in the usual direction — a truncated read reads as success, since the content that arrived is genuine and the missing part is invisible precisely because it is missing — and the subjects most likely to be truncated (a governing document, a long procedure, a wide diff) are the ones where the missed content matters most.

**Nothing enforces this, and that is stated rather than implied.** No gate intercepts a tool choice. It is a governed requirement, cited by `/{project}:review` and `/{project}:analyze` when a claim rests on a partial read, in the same way the rest of this section is cited. A rule that implied enforcement it does not have would be committing the defect it describes.

<!-- §recommendations -->

### Recommendations

A recommendation is a claim, so grounding governs it. When the agent offers options and recommends one, it MUST work the recommendation out to its result **before** presenting it — not after the user accepts.

Where the options differ by a quantity the user cares about — steps removed, restarts saved, files touched, time or cost — that quantity MUST be computed for **every** option and stated alongside the recommendation. Reasoning from an option's *shape* to its *effect* ("this is the biggest change available", "this reuses the most existing machinery") is inference presented as analysis, and it is a defect on this principle whether or not the ranking turns out to be right.

When the deciding quantity cannot be computed yet, the agent MUST say so and describe what would settle it, rather than ranking the options anyway. "I don't know which is better until X is measured" is a usable answer; a confident ranking with an unchecked basis is not — it reads as analysis and carries none, so the user's approval rests on the framing rather than on the facts.

This is the same standard the pipeline applies to its own machinery: `/{project}:review` findings quantify their scope, artifact checks report what they examined, and a gate that cannot run must never look like one that passed. Advice about the system is held to the standard the system is held to.

### Rules

- **Prefer the source to the recollection.** Before asserting how code behaves, what a schema holds, what an artifact says, or what a command does, open it — a `Read`, a `grep`, or a query is cheaper than a wrong answer propagated downstream.
- **A connected database is a primary source, not a hazard.** When the project exposes a dev or read-only database (or equivalent live state), use it to confirm schema, constraints, and representative data instead of theorizing them. **Discover what is reachable** from the project's own configuration — environment files, service/compose definitions, framework config — and from any live-source pointer the project declares in its `AGENTS.md`; a declared pointer is authoritative when present, and its absence is not evidence that no source exists. Read-only access to such a source needs no special authorization — it is the default; treat every source read-only unless the task explicitly authorizes writes.
- **Name the assumption when you must infer.** When no reachable source can settle a load-bearing question, mark the claim as an assumption rather than laundering a guess into a fact. During a spec that assumption is an Open Question ([Spec requirements](#spec-requirements)); in a plan it is a labeled assumption in the plan body; during implementation it becomes a task or an open question on the spec in hand, where the `done` gate reads it ([Finding dispositions](#finding-dispositions)).
- **Cite what you consulted.** When a conclusion turned on a specific source, reference it (`path:line`, the query, the command) so the next reader can re-derive it rather than re-guess.
- **A recorded measurement states its method and its units, or it is not re-derivable.** *Cite what you consulted* covers the source; a number additionally carries **how** it was counted and **what** it counts. Without both, the next reader who measures the same subject and gets a different answer cannot tell a figure that has **decayed** from one taken by a **different method** — and the reflex, to trust the newer figure, is wrong in exactly the cases where the older one was better informed. So state the denominator, the population, and the boundary the count was taken over: *rule-bearing bullets across four named sections* and *anchor-bearing lines in the whole file* are two honest measurements of one document that legitimately disagree, as are *test binaries* and *reported result lines*. When a fresh derivation disagrees with a recorded one, **re-derive at the recorded figure's own commit first** — that separates decay from method, and it is one command — and when that does not explain the gap, diff the field across its own history, because a value can have been correct and then overwritten with a worse one. A measurement whose method is unstated is an unmeasured gap wearing a number, which [§design-principles](#design-principles) already places in the task column rather than the caveat column.
- **Before reporting that a declarative entry misbehaved, read the entry.** A registry row, a manifest line, or a config table carries its own preconditions — a sunset date, a strategy, a pinned flag, a surface filter — and a step that "should have run" may have been correctly excluded by one of them. When the observation is *"X did not happen"*, the first question is whether X was supposed to, and the answer lives in the declaration rather than in the procedure that consumed it. This is *prefer the source to the recollection* applied where the source is a data row, and the failure mode is worse there: a procedure reads plausibly while quietly honouring a field never looked at. State the entry's own gating fields in the finding, or do not file it.
- **Substitute a recorded timestamp or commit into the command that writes it — never transcribe either.** A review or analysis record carries two values that are trivially guessable and silently wrong: the time it ran and the commit it ran against. Nothing at write time inspects them — the primitives record what they are handed — so a fabricated value is well-formed, passes every gate, and surfaces only when a freshness check tries to resolve the commit and reports that it is not one, which fires at a release gate rather than at the write. Substitute both in the same invocation that writes the record, never a value transcribed from earlier output and never a short hash completed by hand, then confirm the recorded commit actually resolves.
- **An enumeration is a claim of its own, and it goes stale while the requirement around it stays true.** A parenthetical list inside an acceptance criterion, or a sentence spelling out a canonical set in prose — the routes a command offers, the back-edges the lifecycle has, the steps of a decision tree — is a separate assertion from the rule it illustrates, and it carries none of the tokens an identifier sweep greps for, so it survives every rename and drifts silently as the set grows. Verify the enumeration, not only the requirement, and then decide which of two it is: an anticipated item that proved unnecessary, which is corrected in place, or a cross-reference genuinely owed, which is a gap in the source. When a canonical set changes, grep for the **count word and the list shape** — *two back-edges*, *three routes*, `(1) … (2) … (3)` — rather than for the names, and prefer stating the set without a count at all.
- **Pick the checks a change must pass from its blast radius, not from its file extension.** A change rarely stays inside the one check whose name resembles it, and the checks that look unrelated are the ones that go red — a prose edit that reaches a generated artifact, a script edit outside a workflow's trigger paths. A green result from a *chosen* subset is evidence about that subset and nothing more, so when the blast radius is unclear, run everything: a full local pass costs minutes and a wrong completion claim costs a round trip. Two orderings are not optional — commit before running any check that reads git history, since such a check must be *asked* about committed state, and re-run those checks after committing rather than before.
- **After publishing a change, read every check that ran against it — not the one whose name matches what you changed.** Independent jobs fail for unrelated reasons at unrelated times, so watching the job that resembles the diff and reporting it green is a claim about that job rather than about the change. Read every result recorded for that commit before reporting anything, and treat a check with no verdict yet as **unknown**, never as a pass. A release needs the same sweep on its own terms: a release job set is not automatically a superset of the ordinary ones, so a gate that runs on every push may be absent from the path that actually publishes.
- **Verify what a commit contains before reporting it landed, and pair that with a check for what it left out.** Checks that read the working tree pass for the wrong reason when a change is in the tree but not in the commit, and nothing errors — the evidence is genuine and is about the wrong subject. Read the commit's own file list against what you meant to commit, and read the working tree's remaining changes alongside it: the first verifies what went *in*, the second is the only one that sees an unstaged leftover. Do both before recording anything that makes a claim about the tree — a review, an analysis, a completion report — because such a record is written against a commit and is false the moment the two disagree.
- **Never pipe a command whose exit status you intend to read.** A pipeline's status is its last element's, so filtering a gate's output through a pager or a tail reports the filter's success over a failing gate — and the output shown is genuine while the status is manufactured, which is the hardest combination to notice. Redirect the output, read the status on the line that ran the command, then search the file. Reading only the tail is the other half of the trap: the end of a multi-target run shows one target's result and hides the rest, so count the result lines against the number of targets rather than eyeballing the end.

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

The top-level directory name (`specs` above) is the documented default; a project may rename it via `.ductus/config.toml` `[paths] specs-root` (e.g. to avoid colliding with a sibling framework's `spec/`, like RSpec's). When the key is unset every command and the runtime default to `specs`, so an adopter who never sets it sees unchanged behavior. The literal `specs/` throughout this constitution and the command sources is that default.

**This is an instruction, not only a fact.** Wherever a command acts on a path under the spec root, substitute the configured name for the literal `specs/`. The primitives resolve `[paths] specs-root` themselves, so the substitution is the host's to perform on the markdown-only path — and it applies to every command, including ones added after this line. Commands reference this rule rather than restating it: seven copies had accumulated before spec 040 collapsed them here, and every copy was one more thing to keep in sync ([§drift-prevention](#drift-prevention)).

<!-- §spec-requirements -->

#### Spec requirements

- Every spec includes a **Status** indicator: `draft`, `clarified`, `planned`, `in-progress`, or `done`
- Every spec includes **Acceptance Criteria** — concrete, testable conditions that define "done". Each carries a stable `AC{n}:` label after its checkbox (`- [ ] AC7: …`), assigned by the runtime's labelling pass and permanent for the life of the criterion: never renumbered when criteria are inserted, reordered, or removed, and never reissued after one is deleted. The label — not the criterion's position — is how a criterion is cited in prose, across specs, and by tooling (spec 013)
- Every spec includes **Open Questions** — uncertainties and unresolved decisions. An open question is an **undecided blocker**: a decision deferred pending a condition ("not now; revisit when X lands") is resolved *with* a condition and belongs in Resolved Questions with its trigger recorded, not left open
- Every spec lists **Dependencies** — other specs this feature depends on
- Open questions must be resolved before moving to the plan phase
- Specs describe behavior and contracts, not implementation
- **Never hand-write an `AC{n}:` label.** Add the criterion unlabelled and let the labelling pass assign it. The label is `max(highest label in the body, next-criterion)`, and `next-criterion` is frontmatter an author is not reading while drafting prose — so a hand-written label is a guess that happens to be right until it collides with a retired one. The counter is what stops a deleted criterion's label being reissued to a different requirement. If labels have already been written by hand, leave them: stripping them renumbers from the advanced counter and opens a gap for no gain.
- **A spec body must not carry a point-in-time status section.** A *State at hand-off*, *Current state*, or *Why this spec is still {status}* section narrates pipeline state that is already derived — the frontmatter, `tasks.md`, the pipeline view, and git history all answer it — so it serves no reader a live source would not serve better, and nothing updates it when the state moves. It then contradicts the spec's own frontmatter, silently and indefinitely, because a snapshot has no expiry. An operator decision worth keeping goes in **Resolved Questions** with its trigger recorded; an obligation goes in `cross-spec-impact:` or a task; anything else is already derivable. Never add one, including at hand-off, however useful it feels in the moment — and when you meet an existing one, delete it and re-home whatever it still asserts truly, which routinely turns out to be nothing.
- **Write a criterion's path claim so a check can see it: backticked, with an interior slash, and repo-relative.** The artifact family that verifies criterion paths reads inline code spans and nothing else, so a path in plain prose is not a finding, not a skip, and not a candidate — the family reports clean while never having had the claim as a subject, which is a deeper silence than a skip, since a skip at least names what went unchecked. Backticks alone are not sufficient: the candidate test requires an **interior** slash, so a bare filename or a bare top-level directory is rejected even when correctly spelled, and the path is resolved from the repository root, so a spec-relative path written inside that spec's own criterion resolves nowhere and is recorded as unreachable rather than checked. Read a `done` spec's criteria for paths yourself and never let a short skip list stand in for that — the list's length is a fact about backticks and slashes, not about claims. State a claim about an **absence** in non-path form, since a criterion asserting that something does not exist otherwise manufactures the finding it documents.

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

Forward edges only — `/clarify` raises status to `clarified`, `/plan` to `planned`, `/implement` to `in-progress` and then to `done`. The `in-progress → done` transition is gated by `/review`: `/implement` MUST NOT write `status: done` while the review record's `last-run` is unset or its `blocking` is `true`. `/review` is a gate, not a state transition — it writes its findings and that record to `review.md`, but does not change `status`. The gate composes with `/analyze` (which flags drifted `done` specs) and the shipped CI template (which fails PRs that bypass the local checks) per [§design-principles](#design-principles): never depend on human diligence. Three back-edges exist:

- **Backward via new questions** — `clarified` / `planned` / `in-progress` → `draft` when `/amend` records a new open question; the next `/clarify` resolves the question and the spec advances forward again. `draft` is the only status that tolerates open questions, so it is the destination; `/amend` performs the status mutation in the same write that records the question.
- **Backward via new scenario** — `done` → `in-progress` when `/amend` records a scenario. The scenario's task is implemented and the spec returns to `done`. A scenario that *carries open questions* takes this same edge, **not** the question edge above: that edge exists because `draft` is the only status tolerating open questions **in the spec body**, and a scenario's questions are a separate signal that leaves the body untouched. Reverting to `draft` would assert a body state that is not true and route to feature-targeted `/clarify`, which does not read scenarios. The questions still bind — a spec does not reach `done` while any remain (see the `done` row above) — but the routing pressure comes from that gate, not from the status.
- **Backward via meaningful body edit** — `done` → `in-progress` when any artifact under `specs/{feature}/` is edited *meaningfully*. An edit is **mechanical** (no back-edge) in any of three diff-determinable cases: **(a)** every change in the diff is the same find-and-replace token substitution, applied uniformly across the live artifacts enumerated in [§drift-prevention](#drift-prevention), mapping a deprecated label (slug, capability, command, identifier, parenthetical descriptor) to its current label; **(b)** every change in the diff adds, removes, or rewrites a **cross-service reference** — an inline body link whose target resolves to a registered `.ductus/config.toml` `[services]` entry, together with the regenerated `references:` frontmatter that harvests it — because such references are informative cross-service navigation, never dependencies, acceptance criteria, or behavior (spec 030); or **(c)** every change in the diff assigns a **runtime-maintained identifier** — an `AC{n}:` label written between an acceptance criterion's checkbox and its text, together with the `next-criterion:` counter that backs it — leaving every labelled criterion's own text byte-identical, because an identifier names a requirement without stating one, exactly as a rule ID does (spec 013). Anything else — new scope, changed semantics, factual corrections, restructuring, edits scoped to a single spec — is a **meaningful edit** and triggers the back-edge via the same `/amend` flow used for scenarios. The distinction is determinable from the diff alone, so the rule does not depend on author judgment.

The three cases share one test, and it is the test rather than the list that decides a case the list does not name: an edit is mechanical when the diff is **determinable without author judgment** *and* **changes no claim the spec makes** — no requirement added, removed, or reworded; no behavior described differently; no fact corrected. An edit that changes no claim is therefore mechanical even when it matches none of (a)–(c): repairing a typo, or sweep residue where a substitution landed in a sentence it did not fit, restores the text to what it already meant and asserts nothing new. A **factual correction is not this** — correcting a claim that was wrong changes what the spec asserts, and takes the back-edge. Stating the test rather than extending the list is deliberate: a closed enumeration makes every new case an argument about whether it deserves an exception, when the question is only ever whether a claim moved.

This avoids spec proliferation; scenarios evolve the existing spec rather than spawning a new one. Spec bodies are living documents that represent current state — git history is the historical record of what was written when.

**A branch-scoped spec is a staging form, not a fourth state.** A spec numbered `{identifier}.{n}-{slug}` ([§numbering](#numbering-convention)) is created on a branch that cannot coordinate a sequential number, and it declares in `folds-into:` the upstream spec whose statement it is really making. It moves through `draft` → `clarified` → `planned` → `in-progress` → `done` like any other spec, and **a pending fold never holds it short of `done`**: the pre-`done` gate does not read `folds-into`. The order is done first, then fold. `done` says the spec's own work is finished — implemented, reviewed, analyzed; fold-back is what follows, the upstream consolidation of its durable content, and the work is complete once it has run. The fold is still owed until then, so the pipeline view reports the spec as carrying a pending fold at every status, and offers the fold as the next action once the spec is `done`. Its end is retirement, never a place in the corpus. Fold-back is the discharge — it folds the content into the upstream spec as a body edit or as a scenario, re-points every inbound pointer, and removes the staging directory. It needs both specs in one tree, so it normally runs on the upstream branch after the merge.

Fold-back adds **no new back-edge**. Reopening a `done` upstream spec is one of the two `done → in-progress` edges already defined above: the scenario edge when the content lands as a scenario, the meaningful-body-edit edge when it lands in the body. An upstream spec that is not `done` is left where it is.

This holds the anti-proliferation stance rather than relaxing it. A branch-scoped spec is a place to write on a branch, not a second durable home for a concern — its lifetime ends at the merge that makes the upstream spec reachable, and what survives is one spec, edited. A branch-scoped directory that outlives its branch is drift, and a detection check reports it.

Operational rules follow from the lifecycle and apply on every project. The list carries no count, for the reason [§design-principles](#design-principles) gives: a count is the first thing to go stale when the list grows.

- **A retired feature's spec is deleted, not left at `done`.** When a feature is removed from the project, its spec directory goes with it. `done` means *delivered, and still true*; a spec describing something the project no longer has is neither, and leaving it in place quietly converts the corpus from a description of the system into an archive of everything the system was ever intended to be. Those are different artifacts, and only the first is worth trusting — a reader cannot tell a live spec from a retired one by looking, so one retired spec left at `done` puts every other spec's status in question. This is the spec-level form of the rule [§scenarios](#scenarios) already states one tier down — an obsolete scenario is deleted, not marked with a status — and it holds for the same reason: git history is the record of what *was*, so a living artifact never has to be. When the content belongs with another spec, `/{project}:consolidate` is the supported path; it re-points every inbound pointer **before** removing the directory, because a deleted spec with live inbound references trades one durability problem for a worse one. When nothing survives to consolidate into, the spec simply goes. Retiring only *part* of a feature is an ordinary body edit, not a deletion — the spec stays and describes what remains.
- **Re-open a `done` spec with the status primitive when the only intent is to reflect edits already on disk.** When scenario files or body edits are already written and the spec's `status` is the last inconsistency, set the status directly rather than routing through the refinement command — that command expects an input to classify, and manufacturing one either creates a second scenario or is treated as a no-op. Route through refinement only when there is genuinely new input to capture.
- **Syncing a canonical record that lives on another spec is a mechanical edit.** When behavior changes and the canonical-sources map points at a table inside some other spec's artifacts, update that table in the same change and leave the spec `done` — it is case (a), a uniform substitution. Leaving it stale would make a `done` spec's canonical record contradict shipped behavior, which is worse than the reopen it avoids. When the same table needs syncing more than once, move it to the spec that owns the behavior and leave a pointer.
- **Restoring a spec directory from git silently reverts uncommitted pipeline state.** A status flip, a ticked criterion, and a ticked subtask are all writes to tracked files, so a restore aimed at *content* takes the *bookkeeping* with it — and nothing reports the loss, because the pipeline reads status from the file just reverted. Before restoring, note the current status and checkbox state and re-apply them, or restore individual files by path. The same hazard applies to any stash, restore, or hard reset over a spec directory mid-pipeline. Better: commit a status transition as its own step, so a content restore cannot reach it. That advice holds only on one condition, and the condition is about the working file, not the index: when the commit is made, the working copy of the spec file holds the transition and nothing else. The pre-commit hook restages each staged spec file whole from the working tree. So a transition staged alone, by an index edit or by hunk staging, commits whatever else the working file holds under the transition's message, and the transition itself is lost when the index was the only place it existed. Nothing errors, and the commit's own diff is the only signal. To separate a transition from body edits already on disk, set the edits aside, commit the transition, then restore them. The parked copy predates the transition, so re-apply the transition and any ticked checkbox when putting it back. Or make the transition before the edits.
- **A status transition states both ends — the status you believe the spec is at, and the one you intend.** The status primitive is guarded on the current value rather than taking the destination alone, and the guard is the useful half rather than ceremony: it makes the caller declare what they think is on disk, so a transition computed from a stale read fails loudly instead of overwriting whatever is actually there. When a flip fails because the expected status did not match, read the frontmatter before retrying — the mismatch is the finding, not an argument to widen the call.

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

`plan.md` records the design **as it currently stands** — never the history of how it got there. Its **design record** is the plan template's own `##` sections, compared case-insensitively ([Canonical sources](#canonical-sources)); the template marks which are optional. Only the design record carries a plan's claims. When implementation changes a decision, that decision's entry is edited in place to say what is now true, and the plan never gains an entry recording that it changed: git history is the record of what the plan said when, exactly as it is for a spec body ([§spec-lifecycle](#spec-lifecycle)).

Everything else implementation produces has a home, and none of them is the plan:

- **Verification evidence** — the commands run, the pass counts, a test proven to fail — belongs to the commit that lands the task, or the pull request carrying it. It is a claim about one commit, and in a plan it is stale by the next task; the lasting form of a test proven to fail is the test itself.
- **Contributor knowledge** routes by population under [Shared knowledge stays in git](#shared-knowledge-stays-in-git).
- **A finding** is dispositioned in the run that surfaced it, and its decision is stored where [Finding dispositions](#finding-dispositions) stores it.
- **A review triage** is not a record at all ([§implement-phase](#implement-phase)).
- **Handoff state** — what is owed, in what order, with what mechanics — belongs to `tasks.md`, in the pending task it concerns ([§tasks-phase](#tasks-phase)).

The rule is detected at the `##` level, not remembered: `/{project}:analyze` reports each plan section outside the design record, on a spec at `planned` or later, as an advisory finding, discarded with its reason when the section is deliberately part of that plan's design, and `/{project}:prune` moves such a section's durable pieces home before removing it. A journal nested under a design-record heading is part of that section, so no heading test reaches it; the rule holds there all the same.

<!-- §tasks-phase -->

### Tasks Phase

Break the plan into discrete, ordered work items. Each task is small enough to implement and verify independently.

#### Task requirements

- Tasks are derived from the plan, not invented independently
- Each task has a clear definition of done
- Tasks are ordered to respect dependencies
- A task can be completed in a single working session

`tasks.md` is an **ephemeral work-tracking artifact** — a view of what is left to do, derived from the plan. It is not a durable source of truth: a task's value is spent the moment its checkbox is checked, because the durable record of what was built lives in the spec, its scenarios, the rules, and git history — never in a checked-off box. This is the same durability test [§bug-handling](#bug-handling) applies to chores, stated here for `tasks.md` directly. Completed task sections may therefore be pruned — or the file reset to its template state — with `/{project}:prune` without loss, and no consumer of `tasks.md` may treat its content as a durable index (including `/{project}:analyze`'s scenario-consistency check, which does not require an implemented scenario's task to persist). This stands in contrast to the durable artifacts: `spec.md`, scenarios, and rules carry the requirements and decisions that must stay accurate as the project evolves, with `data-model.md` and `plan.md`'s design record ([§plan-phase](#plan-phase)) as the record of the design. `/{project}:prune` removes none of what they assert: a decision it moves home from a plan section is edited into the design-record entry it amends, which reopens a `done` spec ([§spec-lifecycle](#spec-lifecycle)), and the command's own source states the rest.

Because `tasks.md` is ephemeral, it is also where **handoff state** belongs — what is owed, in what order, with what mechanics — as prose in the body of the pending task it concerns. Keep-pending keeps a pending section verbatim and drops a spent one whole, so the note lasts exactly as long as its work. An ordering constraint goes on the task that must wait, never in the preamble or under a free-standing heading, which outlive every task; and a note is prose, never a checkbox line, which would count toward its task's completion.

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
- **A spec's ticked acceptance criteria are verified against the tree before it closes.** A ticked criterion is a completed *claim*, and closing a spec is the moment to re-earn it rather than bank it. Two gaps make this necessary and neither can be closed by a check. The artifact check that compares criteria to the tree examines specs at `done` **only** — correctly, since a criterion on a spec still in progress may name a path not yet created — so a spec that sits in progress indefinitely never has its criteria examined at all. And that check proves only that a path *resolves*: a criterion whose paths all exist while its claim about their contents is false is invisible to it. Read the criteria against the tree, and treat a criterion whose claim no longer holds as a spec to edit — reopen it and correct the claim, or consolidate it away — rather than a checkbox to leave standing.
- **A spec does not reach `done` with an outstanding SHOULD.** [§design-principles](#design-principles)' completion filter applied at the review gate, which is where it fires most often. The gate blocks on MUST violations alone and records SHOULD findings as advisory — but advisory is not "ignorable at the gate". A finding is addressed when it is **fixed**, or when it is moved under the review's waived section with its rationale, which is the disposition for a SHOULD whose answer is "keep as-is". What is not acceptable is a spec sitting at `done` with a non-zero SHOULD count and the finding still filed under its original heading. Read the SHOULD count at the completion gate the same way the MUST count is read.
- **Record a review against a commit that already contains what it reviewed.** The review records the HEAD it ran against, so the natural order — edit, review, commit everything together — records the HEAD from *before* the edits landed, and every durable contract in that commit then reads as changed since the review. Two commits, always: the work, then the review against that new HEAD, then the review itself. Freshness checks read committed state, so they must be *asked* about committed state — a check run before committing passes for the wrong reason.
- **`examined` counts the in-scope files the review actually read — a file you are confident about by other means is named, never counted.** The recurring temptation is a scope file that is a *generated mirror* of one you did read, where opening it would show the same bytes with substitutions applied. Believing it correct and having read it are different claims, and the field is only the second; it exists precisely so a review cannot conflate confidence with coverage. Count what you opened, and name the rest individually in the Summary with why you did not open it and what you relied on instead — *generated from a source read in full, generator re-run and reported in sync* is a complete answer, *it is generated* is not. The same holds for a file too large to read fully (say which parts you read) and for an in-scope path that no longer exists (it stays in scope because the plan lists it, and is named as absent).
- **A spec's review window is a property of its last reopen, not of the spec.** The diff base is derived from the most recent commit at which the spec entered `in-progress`, so every back-edge moves it forward and any file count taken earlier describes a window that has already closed. Measure it rather than quoting it, and measure it *after* the reopen's first commit — before that commit there is no new transition to derive from — bearing in mind that any commit the pass makes afterwards joins the window, including the spec's own review record. A wide count reflects real history rather than a drifted spec: do not narrow it by overriding the base unless the resolved window is genuinely wider than the change being re-reviewed, and record in the Summary which base was used and why.
- **A review record written before the record gained its coverage and digest fields is unexamined, not clean.** Without the examined and scope counts, a review that read none of its subject is byte-identical to one that read all of it; without the durable-contract digest, the pre-`done` gate reports freshness as *undeterminable*, which does not block — so a spec sits at `done` carrying a record no gate can evaluate. Nothing backfills these fields. Treat such a record as unread rather than as a pass, and re-run the review to write them, which only tells the truth if the passes actually read the scope. Read the record before pricing that work: the gap comes in several shapes — all the fields absent, a numerator with no digest, or a digest with no numerator — and they cost different things to close, so carrying a sibling spec's shape forward produces a wrong plan.
- **A spec with no recorded `in-progress` transition derives an empty diff base, and the review's denominator collapses to zero.** Scope resolution still returns the plan's affected files, but the denominator is derived separately, so the record comes out incoherent — a non-zero count of files examined over a scope of zero. Pass the current commit as the base for such a spec: the window is then empty by construction, the scope falls back to the plan's affected files, and numerator and denominator agree. The same answer fits the opposite state — a spec that *does* record a transition whose base is simply old, resolving a window far wider than the change under review. Measure both before choosing, and say in the Summary which was passed and why.
- **Only a spec's durable contracts stale its review — its scenarios and its data model, nothing else — and that is what prices a corpus-wide sweep.** The review digest covers exactly those files, so a sweep confined to the spec body, the plan, and the tasks costs no re-review however many specs it touches, while a single line changed in one scenario or data model obliges a full re-review of that spec. Plan a sweep by measuring its durable-contract hits first: that number, not the total hit count, decides whether it is an afternoon or a campaign. Then decide each such hit deliberately — sweep it and accept the re-review, leave it, or defer it into a pass that will re-review the spec anyway.
- **The criteria-against-the-tree count cannot be measured while the spec is `in-progress`, and the gate's own ordering is what hides that.** The artifact family that checks a criterion's paths examines specs at `done` only, so running it against an in-progress spec yields neither a finding nor a skip — the family has no subject at all — and an analysis recorded there records zero truthfully about a run that never asked. Meanwhile the pre-`done` gate blocks on a stale analysis, so the obvious order is exactly the one that never measures, and nothing reports the omission because a computed zero and an unasked family render identically. Flip the spec to `done`, run the artifact check, flip it back, then write the record with the measured counts and their stated reasons. Verify and tick the acceptance criteria before writing that record, not after: ticking one edits `spec.md`, which the analysis digests, so a record written first reads stale at the pre-`done` gate and the run has to be repeated.
- **The review's denominator is derived at write time, so a count measured earlier and typed into the Summary can contradict the record it is written into.** The Summary is prose the reviewer supplies while the scope is computed by the primitive, and because one call writes both, they cannot disagree in any way a check would notice — the agreement check holds the report against the spec's own frontmatter block, which that same call wrote from the same values, so the two agree while both differ from the prose. Read the returned counts against every number the Summary states and re-run with corrected prose when they differ; the call is idempotent, so the fix leaves one record rather than a correction. The denominator moves *during* the pass, which is why any number quoted from before the write describes a window that has already closed.
- **Nothing durable goes in a review record's body.** `/{project}:review` regenerates `review.md` on every run. The report body, Summary included, is rewritten whole, and only the frontmatter's waiver and decision lists carry forward from one run to the next. A note left anywhere else in the file is destroyed by the next review, so whatever must survive belongs on the artifact it describes, in the spec body: the record of a superseded criterion, the reason for a decision, a claim about the tree. **A triage of a review's findings is not such a record, and needs no home that survives the run.** A MUST or SHOULD violation keeps its fix-or-waive model, as above — it is counted as a violation, not dispositioned; every other finding is fixed, routed or discarded in the run that surfaced it ([Finding dispositions](#finding-dispositions)), and one still awaiting a decision is recorded as undispositioned and detected again by the next run; and the fixes a triage plans are tasks. None of it belongs in the plan ([§plan-phase](#plan-phase)).
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

Bugs are unwritten or violated requirements. Every bug is evidence that one of the framework's three artifact tiers — rules (cross-cutting), specs (feature-wide), or scenarios (situational) — has a gap. Rather than tracking defects in a separate system, fixing a bug means making the requirement at the right tier more precise. See [§rules](#rules) for the rule tier and [§scenarios](#scenarios) for the scenario tier.

Not every finding or logged item is a requirement gap. It may be a **chore** — a discrete piece of project maintenance (lint or formatting cleanup, dependency cleanup, repo hygiene, a standalone refactor) that adds no missing or violated requirement and belongs to no single feature. A chore does **not** spawn a rule, spec, or scenario, and it is **not** a spec task — a spec's `tasks.md` holds work derived from that feature's plan, never a chore as its home. **A chore a run finds is fixed in that run**, confirmed before it writes ([Finding dispositions](#finding-dispositions)). The one task a chore touches is transient: `/{project}:implement` records a chore found outside its spec as a disposition task, so the task in hand is not derailed, and fixes it when that task is worked — the task is the work item that gets it decided, not a place the chore lives. **A chore a person logs by hand** stays tracked as a checkbox in `specs/inbox.md` (the project's non-feature work surface) until it is *done*, then is removed — not migrated to a spec. The test is **durability**: rules, specs, and scenarios hold durable information that must stay accurate as the project evolves — feature description and context, acceptance criteria kept current, resolved open questions that serve as the project's architecture-decision record, and cross-cutting rules. A chore captures none of that; it is transient work whose value is spent once complete. Route requirement gaps through the decision tree below; do chores directly.

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

The scenario-creation primitive frames the body it is given: it writes the frontmatter, the heading, *and* the Open / Resolved Questions scaffolding. **Do not author those question headings in the body passed to it** — a body already carrying them produces two, which the markdown linter rejects as a duplicate heading. Pass Context, Behavior and Edge Cases only, then edit the scaffolded questions section afterwards; write the whole file directly when it needs questions at creation time.

The task-append primitive can succeed while writing nothing, and **that is an outcome to read, not an error to retry**. It deduplicates in two ways. When a task already references the scenario being recorded, or, when asked to deduplicate on the title, a still-pending task carries the same title and items, it returns that task's number and leaves the file untouched. The right move is then to extend or work that task rather than open a second one for the same work. Read the fields it returns (whether it appended, whether it created the file) rather than its exit status, and when nothing was appended, open the task it names before trying again.

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

A project may exclude a stack-selected rule file from `/{project}:review` by listing it in `.ductus/config.toml` `[[review.disabled-rule-files]]` with a mandatory `reason` — the reason is the audit trail for the override, surfaced on stdout at the start of every run. The opt-out is project-wide and applies to whole files; per-`(rule, file)` exceptions remain the job of `/{project}:review --waive`. Schema and behavior are documented in `/{project}:review`'s command source.

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

A `specs/inbox.md` file is the project's capture queue for todos a person has not yet assigned to a feature spec. Items are recorded with `/log` — the inbox's only producer — and groomed into their proper home with `/groom`. No command appends to it on its own: a finding a pipeline run produces is dispositioned in that run ([Finding dispositions](#finding-dispositions) below), because the inbox is the one destination no gate reads. Its standing use is **brownfield migration**: for projects adopting `ductus` incrementally, known issues are parked here until a spec exists to absorb them.

An item's "proper home" is usually a feature spec, scenario, or rule; an item that is a **chore** (project maintenance belonging to no feature — lint or dependency cleanup, repo hygiene, see [§bug-handling](#bug-handling)) has no spec home and is resolved by being done directly, then removed — `/groom` recognizes it and does it in the same pass rather than forcing it into a spec.

Inbox rules:

- Do not frontfill bugs that are not being actively worked on
- Write specs for areas being actively touched — let adoption spread naturally
- As specs are written, `/groom` migrates items from the inbox into their proper home
- **When an item routes to a chore, fix it — do not park it.** The chore route means the item is done directly, in the same pass, and then removed because it is resolved. A parked chore is re-read and re-routed on every subsequent grooming pass, so the recurring cost of carrying it exceeds the one-time cost of the fix, and the inbox stops reflecting real backlog. Leave one in place only when the fix is genuinely blocked or turns out not to be mechanical after all — and say which.
- **An item blocked on an external event is discarded, not held — the inbox has no hold state.** When an item cannot be acted on today because it waits on something outside the project — a protocol stabilizing, an upstream tool shipping support, a dependency's next major — it matches none of `/groom`'s five routes: there is no rule to amend, no spec to write against a premise that cannot yet be evaluated, no scenario whose behavior is knowable, and no chore to do. Marking it *on hold* in place is a status note, which this file does not carry: every later pass re-reads and re-defers it, and the marker's own conditions go stale silently, so the item outlives the reason it was kept. Discard it, and keep the durable half where it will be read at the moment it matters — the maintenance burden in the project's contributor guide, the constraint in the spec body that must respect it. Nothing then watches for the external change, and that is the accepted cost: a change worth acting on resurfaces when someone next touches the surface it affects. An item that is merely *undecided* is not blocked — its premise can be evaluated today, so it routes to a scenario under its covering spec and the decision lives in that scenario's open questions.
- The brownfield-migration backlog drains toward empty as adoption completes; the file persists as long as people keep logging todos
- **The standing count is a notice, never a gate.** `/{project}:status` renders how many items are outstanding and how old the oldest is on every run, including when the count is zero, so examined-and-empty never renders as not-computed. Nothing blocks on it: capture has to stay free, because the honest choice between a growing backlog and a silent one would otherwise push toward silence. That is safe only because nothing a gate needs can land here — every finding a run produces is dispositioned where a gate reads it.
- **Remove an inbox item by matching its own text, never by position.** The removal primitive takes the bullet's text and matches on equality, and that shape is the safeguard rather than an inconvenience: line numbers computed up front all shift the moment the first removal lands, so a batch driven by position deletes unrelated items while every individual call remains valid and nothing errors. Extract each item's line and strip its checkbox marker rather than retyping the text, assert the match is unique before removing, and read the diff of removed bullets against what you intended. The hazard is general — never drive a by-position edit over any list that shrinks as you walk it.

#### Finding dispositions

Every finding a pipeline run produces MUST get exactly one disposition before its spec can reach `done`: a `/{project}:review` observation, a `/{project}:analyze` finding still live when detection ends (in any tier), and an issue `/{project}:implement` surfaces outside the task in hand. A findings-producing command that drops its findings is the failure the **Design Principles** rule names directly, and so is one that parks them where no gate reads — the spec reaches `done` looking finished while its findings wait somewhere else.

- **Three dispositions.** **Fixed**: a chore — mechanical, and adding no durable requirement by [§bug-handling](#bug-handling)'s durability test — is fixed in the run. A fix that fails, or proves not to be mechanical, is reverted and is not a chore; a chore is never counted as fixed while its fix is not on disk. **Routed**: the finding is written to an artifact the pre-`done` gate reads, chosen by the groom decision tree (`groom.md` is its single canonical statement, and each command references it rather than restating it) — a task, scenario, or body edit on the spec in hand, or a scenario or body edit on another existing spec, either one reopened `done → in-progress` when it is `done`, with the reopen named in the confirmation, so routed work never sits on a spec no gate reads; a new spec, created through `/{project}:specify`'s procedure in the same run; or an amendment to a rule file the project owns. A managed rule file the project has not pinned is overwritten on the next update, so a finding against it is discarded with that reason, and a declined spec creation leaves the finding discarded or undispositioned — never counted as routed to a spec that does not exist. **Discarded**: the finding and the reason it is out of scope are recorded in the run's own record. The inbox is not a disposition.
- **Detect, decide, re-check, record.** Detection writes nothing. Each fix or route is confirmed with the operator before it writes, and a confirmed route whose write fails is undispositioned. When any disposition wrote, detection runs again over what changed, and only then is the run's record written — so the record describes the state after dispositions, and a run never makes its own record stale. The analyze record reads that re-check, not the kind of write. A finding the re-check no longer produces is recorded as fixed whether a chore or a confirmed route removed it, and a finding is recorded as routed only while it still fires, until its routed work lands. With no operator to confirm, nothing is written, and each finding without a stored decision is recorded as undispositioned; under `ductus exec`, whose walker matches no stored decision either, that is every finding.
- **Scope decides the destination.** Three tiers: inside the current **task** → fix it in the task; inside the current **spec** but outside the task → a new unchecked task on that spec; outside the spec → one of the three dispositions. An in-progress spec *is* the home its own defects need, so routing one anywhere else costs a full loop back to it. This does **not** make `tasks.md` a second capture queue: the durable record still lands where [§bug-handling](#bug-handling) puts it — a missing requirement becomes a scenario or a spec edit via `/{project}:amend`, with the task as the work item that implements it.
- **Record, do not derail — the disposition task.** `/{project}:implement` does not stop the task in hand for a finding outside the spec. It appends a disposition task to the targeted spec's `tasks.md`, titled `Disposition out-of-spec finding: {summary}`, and keeps working. The task is on disk the moment the finding surfaces, so an interrupted run loses nothing, and as an unchecked task it already holds the spec out of `done`. Working it fixes, routes, or discards the finding, and a discard's reason is written onto the task as it is checked off. The append is deduplicated on the task's title, so re-running an interrupted run records nothing new. The cost is deliberate: a finding unrelated to the spec holds it out of `done` until it is decided, because that spec's work surfaced it.
- **A finding that already gates `done` cannot be discarded.** An analyze hard-fail or blocking finding is fixed or routed, and keeps blocking until the re-check no longer produces it — a discard would be a bypass the gate does not have. Review MUST and SHOULD violations keep their fix-or-waive model. The discard route serves advisory findings, review observations, and disposition tasks.
- **Decisions persist.** Detection is stateless, so a decided finding fires again on the next run — a routed one until its routed work lands. Each routed or discarded decision is stored in the record of the command that made it (`decisions:` in `review.md` or `analysis.md`), with its outcome, its target or reason, when it was made, and by whom, and a later run counts a matching finding under the stored outcome without asking again. A stored decision whose finding no longer fires is pruned; a run that did not evaluate the finding's source retains it. The next run matches each finding to the stored decision describing the same issue by judgment rather than byte equality, because a finding's wording drifts; a reworded finding left unmatched is a new finding, and is asked about again.
- **The gate asks for a decision, not a fix.** Each record counts its findings in a `dispositions:` map, and the pre-`done` gate blocks `in-progress → done` while either record's `undispositioned` is above zero — checked after every other review and analyze check, since each of those names a more upstream defect. A record with no map predates dispositions, and absence is not zero: on an `in-progress` spec the gate blocks and names the command to re-run; on a `done` spec it is not drift. Discarding a false positive with its reason clears the block, so an advisory finding still never has to be *fixed* to reach `done` — it has to be decided.
- **Severity raises salience, not the routing.** Security issues and memory or resource leaks are the findings most costly to lose, so they are surfaced first and flagged; every finding gets the same three dispositions.

This keeps the agent's attention on the task while guaranteeing that every finding ends somewhere a gate reads — or in a recorded decision that it belongs nowhere.

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
- **A scenario's link to a sibling spec needs one level more than a spec's, and the depth error is the half a link check catches.** A scenario sits one directory below its spec, so a sibling path written as though from the spec body resolves one level short; the corpus link check reports that and states the corrected path, which is the cheap half. The expensive half is invisible there: the dependency derivation harvests sibling links from scenario files as well as spec bodies, so a *correct* link creates a `dependencies:` edge from the parent spec — and when the target already depends on that parent, the edge closes a **cycle** no link check mentions. Cite a sibling spec from a scenario with a backticked slug rather than a link unless the dependency is real, and after any body edit read the derivation's cycle report alongside its drift report; a clean drift result on its own is not the pair worth checking.
- **When an edit's purpose is to remove a section, diff the structure, not only the prose.** An edit anchored on a string that also occurs inside a code span silently consumes everything between the two occurrences, and a heading absorbed into an **unterminated** code span stops being a heading — which is invisible to every reader the pipeline has, because an unclosed backtick never becomes a code span at all and so trips no markdown lint. The artifact checks report clean, the link and anchor resolvers read past it, and the spec parser omits the swallowed section *entirely* — so a gate that reads that section can be satisfied **by** the damage rather than despite it, and a spec can reach `done` partly because a heading stopped being one. Count the headings before and after the edit: it costs a second, and it is the only signal this failure gives.

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

Each audit command records its run in the frontmatter of the artifact it writes: `/{project}:review` in `review.md`, `/{project}:analyze` in `analysis.md`. **`spec.md` carries neither record.** A `review:` or `analyze:` block in a spec's frontmatter is a residual from before this schema and is reported as a violation, *not* accepted under the open-schema rule below — one fact, one home, per [§drift-prevention](#drift-prevention).

**The absent file is the never-run state.** A feature with no `review.md` has not been reviewed; one with no `analysis.md` has not been analyzed. Neither artifact is ever written empty to signal a clean run — every run writes its own, so absence carries information rather than ambiguity. An artifact that exists but does not parse is **undeterminable**: a third state, which MUST NOT be collapsed into never-run.

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

Additional fields beyond those listed above are permitted and ignored by uninterested consumers — with one exception, stated in **Audit records** above: a `review:` or `analyze:` block in a spec's frontmatter is a residual of the pre-relocation schema and is reported rather than tolerated. The rule admits fields nothing has claimed; it does not re-admit a field this schema has moved. Examples adopters or future `ductus` work might add: `owner`, `target_release`, `created_at`, `description`, `aliases`. Consumers MUST NOT error on the presence of unknown fields. `/{project}:analyze` reports unknown fields as informational findings (not errors). Stale fields in done specs (e.g., `title`, `tags`, `spec-ref`, `track`) remain valid under this rule and produce no findings.

### Validation Severity

`/{project}:analyze` checks frontmatter against this schema with the following severity:

- **Hard fail** — frontmatter block missing on a spec or scenario file; frontmatter YAML malformed; `status` missing or not in the allowed set; `dependencies` missing or not a list; both `section` and the legacy `spec-ref` missing on a scenario; frontmatter block missing or malformed on a `review.md` or `analysis.md` that exists (the artifact's absence is a state, its presence without a parseable record is a defect); a `decisions:` list on either record that does not parse (read as empty, it would silently drop every stored decision).
- **Blocking** — a `review:` or `analyze:` block present in a spec's frontmatter. It is not a hard fail: the spec file itself parses, and under the pre-relocation schema the block was valid. It is not informational either, because the open-schema rule does not cover it and a second copy of a gate-read record is the drift condition, not an unknown field. The remedy is the relocation migration, and the finding names it.
- **Advisory** — cross-reference checks; body inline links to sibling specs that are not yet in the generator-managed `dependencies` (informational — the next commit's `derive-dependencies` pass will resolve).
- **Informational** — unknown fields present.

Hard fails block the validation pass. Advisory and informational findings are reported but do not block.

For non-frontmatter checks (spec integrity, artifact completeness, plan/task consistency, dependencies, security rules) — and for the one frontmatter check named above, a record block residual in a spec — `/{project}:analyze` adds a fourth tier, **Blocking**, between Hard fail and Advisory. The residual is the exception that proves the tier's shape rather than breaking it: the spec file is well-formed, so Hard fail would overstate the defect, while the artifact set is inconsistent with itself, which is precisely what Blocking says. Blocking findings are structural or content issues that must be fixed before the next pipeline gate fires (e.g., missing `plan.md` on a `planned` spec, an unknown rule ID referenced in a spec). Hard fail and Blocking both prevent pipeline advancement; the distinction is that Hard fail says "the spec file itself is malformed," while Blocking says "the artifact set is incomplete or inconsistent." See `framework/commands/analyze.md` for the full per-check severity assignment.

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

The backticked primitive names in a rewritten command's Instructions section map to the MCP tools the runtime exposes under bare `<verb>-<noun>` names. A host wraps them with a server-name prefix taken from its MCP registration — Claude Code: `mcp__ductus__<verb>-<noun>`; Auggie / Antigravity: `mcp:ductus:<verb>-<noun>`; OpenCode: a `<server>_<tool>` name under the `ductus` prefix; Pi: a `ductus__<verb>-<noun>` extension-bridge tool supplied by the project's `.pi/extensions/ductus.ts` — Pi has no built-in MCP, so the bridge wraps the runtime's own MCP server over stdio and the server remains the single source for names and schemas (spec 064). Match the prefix rather than a spelling: the separator is a host detail, and a new host is one more spelling of the same namespace. When the `ductus` server is registered for the session, the agent **calls the corresponding tool** for each step — the deterministic path. If a host loads MCP tool schemas lazily (e.g., Claude Code lists tool names in a deferred-tool reminder before exposing their schemas), the runtime is still registered: the agent fetches the schema through the host's mechanism (`ToolSearch` on Claude Code) and calls the tool rather than bailing to the fallback. When no `ductus` server is configured, the agent walks the same prose with the host's file tools (`Read`, `Edit`, `Write`); the shell-pipeline substitutes named in principle 3 are **not** a sanctioned stand-in for either the runtime primitives or those file tools. The two paths share one contract; neither wraps the other. A rewritten command opens its Instructions with a one-line pointer to this subsection (§runtime-host-integration) rather than restating it — the contract lives here once.

#### Eligibility criteria

A capability is runtime-eligible only when **all three** hold:

1. **Deterministic** — no semantic judgment required; the same inputs always produce the same outputs.
2. **Currently mechanical** — already either (a) executed by an LLM following procedural instructions in a slash command body, or (b) implemented as a bash script the framework invokes (pre-commit hooks, generators, CI).
3. **Specifiable as prose** — the capability can be stated completely enough that the Markdown-only reference documents it, and a primitive mirrors that reference rather than introducing policy of its own. This is what keeps the specification and the implementation one thing: the reference is where the policy lives, the primitive is how it runs.

A capability that fails any criterion stays out of the runtime. Anything that requires reading prose for intent is permanently LLM-owned regardless of how mechanical its surface looks.

**Eligibility is a default, not a permission.** A capability meeting all three criteria is implemented as a runtime primitive; a shell script is the fallback, taken only when a criterion genuinely fails. The framework still needs shell entry points — a pre-commit hook, a CI step, an `/audit` family — and those keep their scripts, but the entry point resolves the runtime and calls the primitive rather than reimplementing the check inside itself. A script that parses frontmatter or markdown structure to do its work has already failed principle 3, whether it reaches for `awk` or for an embedded interpreter: the language is not what the principle turns on. The cost of ignoring this is not hypothetical — each hand-rolled parser is a fresh copy of a parse the runtime already owns and has tested, and the copies drift, so a bug fixed in one survives in the others.

The pull is toward the script, because a script runs immediately and a primitive is a build away. Weigh that honestly: the build is a one-time cost paid by the author, and the parser is a permanent cost paid by every reader afterward.

#### Acquisition invariant

The repository's CI MUST include a job that exercises acquisition end-to-end on every supported platform: fetch the published asset for the target, verify its sidecar digest, install it into a temporary store, and execute the installed binary. A change that causes this job to fail — i.e. a release whose assets an adopter cannot actually acquire — is a constitution violation, not a feature.

This replaces the **opt-in invariant**, which asserted a full pipeline cycle with the binary absent from `PATH`. That job tested the guarantee principle 3 used to make; the guarantee it now makes is that the binary is *obtainable*, and the job that proves it is the one that fetches it. Amended by [048](https://github.com/stonean/ductus/blob/main/specs/048-govern-acquired-runtime/spec.md).

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

**Referencing means a pointer, never a copy.** A link, an anchor, or a named section is a reference; a reproduction of the source's content is not, however faithful it was on the day it was pasted. Embedding a copy is the failure this rule exists to prevent, and it is worse than ordinary staleness for two reasons that compound. **Nothing can detect it** — a snapshot inside a fenced code block is invisible to every link check, anchor resolver, and structural audit, because none of them read inside a fence; the copy rots with no signal, and even a measurement of its own extent misreads it, since a scan for the next heading stops at the first one *inside* the fence. **And every reader pays for it, repeatedly** — a copy is read in full by every contributor and every agent that loads the file, in every session, for as long as it exists, while a pointer costs one line. The second cost is invisible in a diff and unbounded in time, which is why it is stated here rather than left to judgment. Where the copy exists to preserve a historical record, git history already holds it: a document body describes current state ([§spec-lifecycle](#spec-lifecycle)), so it never has to carry a frozen duplicate to remember what something used to say. Use the pointer wherever a canonical source exists. Replacing an embedded copy with one changes what the document asserts, so it is a factual correction that takes the back-edge — not a mechanical sweep.

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
| Scenario→task referencing rule (what counts as a task referencing a scenario) | `specs/022-deterministic-runtime/data-model.md` (`scenario-consistency`) |
| Spec-root resolution (substituting `[paths] specs-root` for the literal `specs/`) | `framework/constitution.md` §spec-phase |
| Constitution content — what belongs in it, how it is organized, which rules adopters receive | `specs/050-constitution/spec.md` (a spec that changes *behavior* still amends the principle its change contradicts, in the same change) |

When adding a new kind of fact that may be referenced from multiple documents, name its canonical source explicitly here.

### Cross-document references

When document B describes content authored in document A, B includes a back-link to A — relative markdown link, anchor reference (`§anchor`), or section name. Two consequences follow:

- Changing A includes auditing every back-link to A. The audit is structured wherever it can be machine-checked (anchor resolution, help-table descriptions, registry-frontmatter equivalence), and a manual sweep otherwise.
- Adding a fact that conceptually belongs in A but landing it in B is drift. Either move the fact to A and back-link, or extend A's scope explicitly.
- **No dead references in live artifacts.** When renaming or removing a name — a spec slug, a capability, a command, an identifier, even a parenthetical descriptor — update every reference across the project's live artifacts in the same change: specs (including `done` spec bodies), rules, command sources, scripts the pipeline runs, CI configuration, docs, and the README. A reader following a pointer must never land on an outdated name. The sweep is uniform find-and-replace, which makes it a mechanical edit under [§spec-lifecycle](#spec-lifecycle) — `done` specs stay `done`. Do not bundle it with unrelated edits: a non-uniform diff is a meaningful edit and reopens what it touches. **Keep the sweep's own target list current** — when an earlier change relocated a directory, a list still naming the old location sends the grep somewhere clean and the sweep silently misses the files that moved. This holds for a retired **name**; a retired **filename** is the narrower case the next bullet covers, because the project may still have to read it under the old name.
- **A retired filename is not a retired name — sweep it by meaning, not by token.** When what was renamed is a *file* the project still reads under its old name, the rule above inverts in two places at once, so applying it unchanged does damage in both directions. **Some occurrences are load-bearing and must survive:** a resolution ladder or fallback tier that reads the old name so an adopter who has not migrated still resolves; a migration procedure, whose subject *is* the rename and which must name both sides to stay auditable; and the prose, shell and configuration that spell either. A blanket substitution breaks all three **silently** — the ladder still compiles, the migration still parses, and only an unmigrated adopter ever finds out. **Everywhere else the old name is residue and goes:** the end state is the current name wherever an artifact states current behavior, with the retired name surviving only in the few references that record the decision to change it. A spec corpus is a durable description of current state plus the decisions that produced it; a retired filename scattered through hundreds of references is neither, and it teaches the wrong name in the same breath a spec claims to describe what is. **A ticked acceptance criterion is swept like any other reference** — the requirement it states is unchanged when only the file's name moved, so naming the file correctly asserts nothing new and leaves the criterion true. Annotation is for a criterion whose **behavior** was superseded, which is a different thing: there the claim itself stopped holding, and rewriting it would record a fiction. Do not annotate a rename — an annotation preserves the retired name in the corpus permanently, which is the residue this rule exists to remove. Because the whole pass is then a substitution, it satisfies [§spec-lifecycle](#spec-lifecycle) case (a) and reopens nothing; the review-staleness exemption mirroring that case is computed by the runtime from the diff rather than claimed by the author, so a reworded line — a stale annotation removed, a fact corrected — costs its spec the exemption and takes the back-edge on its own.
- **A behavior change needs a prose-claim sweep, not just an identifier sweep.** The rule above catches renamed *names*; a change to what the system *does* additionally needs a sweep for stale *claims* about the old behavior — and those claims contain none of the changed tokens, so a path- or identifier-scoped grep passes straight over them. Enumerate the claims the change falsifies and grep for them **by meaning**, across the live artifacts and the README especially, since it narrates behavior to users. Fixing a stale claim in docs is docs-only; a stale behavioral claim inside a `done` spec body is a meaningful edit and takes the back-edge.
- **Never edit an installed command file directly.** A file the installer places is overwritten on its next run, so an edit made there is lost without warning. Change the source the installer copies from, or pin the file in the project's configuration to opt it out of updates — pinning is the supported way to keep a local modification.
- **Renaming a repository orphans contributor-local state that no migration can reach.** A migration converges state named for the project; it cannot touch state keyed to the project *path*, because that lives outside the repository and differs per contributor. Nothing errors — the state is simply never found again — so this needs a checklist rather than a check. After renaming, on **each** machine: rename the local checkout to match, repoint the remote, move any per-project agent state stored under a path-derived slug (and correct the absolute paths recorded inside it, which a copy alone leaves pointing at the old location), and fix anything else keyed to the old path — shell aliases, editor workspaces, worktrees.
- **An anchor reference resolves by line, and a line wrap alone breaks it.** A `§name` reference counts as qualified only when the *same line* also names the document the anchor lives in, so wrapping that document's path onto the previous line silently re-reads the reference as a claim about the default document, where no such marker exists. Within one document, matching runs longest-first against the text following the `§` rather than as a prefix match on the first token, so the reference must spell the target heading exactly. A **subsection** carries no anchor marker and is not addressable at all — cite the marked section that contains it. Read the resolver's per-reference list rather than its unresolved *set*: the set deduplicates by anchor name while the repair is per occurrence, and the two diverge badly whenever one file cites the same anchor more than once. The same gap runs one tier down in ordinary links — a link that *resolves* is not a link that points where its text says, so when a link's text is itself a path, read the two against each other, because no link check compares them.
- **A sweep can make a quoted instruction point at itself, and the identical sentence may be correct one document over — classify by who emits it, never by the string.** When a substitution lands inside a quoted user-facing message, the result still parses and reads as ordinary guidance, so the uniformity check that catches a collapsed substitution cannot see it: both sides of the sentence genuinely differ. The tell is not in the sentence but in the **emitter** — a message that redirects the reader to the command they are already inside is always damage, while the same sentence issued by a different command names a genuinely different destination and is correct. Before correcting one occurrence, find the rest and classify each by emitter; for any message whose job is to redirect *between* commands, one string with two meanings is the normal case rather than the exception. Where the behavior behind the quote also moved, annotate rather than restate.
- **Replacing an embedded copy with a pointer is a corpus sweep, not a deletion.** The copy is not the only thing that describes it: a sibling artifact names the section, the artifact's own plan may declare the copy canonical — which is the inversion that produced the snapshot in the first place — and another spec may record it as the motivating instance for the rule against copies, in the **present tense**, so removing the copy falsifies the document that governs copies. Before deleting, search the corpus for the section's name *and* for prose describing it, then classify each hit by tense: a present-tense description of the thing being deleted goes false, while a past-tense account of why it mattered stays and is the decision record that must survive. Two further checks belong to the same pass — look for acceptance criteria asserting the section exists, and measure the section's own extent fence-aware, because a scan for the next heading finds headings belonging to the *copied* document, which is the same blindness that let the copy rot, applied to removing it.
- **A document declaring itself the authoritative shape of something the tooling writes drifts silently — check it against a record the tooling actually wrote, not against the code.** Nothing compares a schema table to a real instance: the artifact checks read criteria and paths, never a table against the thing it describes, so fields added by later work can be live for months while the canonical record omits them, and the same spec's own acceptance criterion may assert a field its data model does not list. When a pass opens a spec whose data model declares a shape something else writes, open a record written minutes ago and compare the field sets directly. Reading the implementation instead is weaker evidence: it says what the code *can* emit, not what the shipped artifact carries, and the two differ wherever a field is conditional or supplied by the caller.

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
- Adding template structure requires a corresponding rule (an `/{project}:analyze` check, a constitution rule, or both). Sections that don't trace back to a rule are dead weight.

Templates and `/{project}:analyze` evolve together. A diff that touches one without the other is incomplete.

### Manifest discipline

When multiple commands distribute or reference the same set of files (e.g., `/{project}:configure` and the bootstrap install both apply permission sets), the file list lives in one place:

- Either as a shared section the commands include by reference, or
- As a registry both commands read.

Two commands that copy-paste the same manifest into their own bodies are guaranteed to drift over time. Consolidate or accept that drift is the rule, not the exception.

### Shared knowledge stays in git

Knowledge that would help any other contributor belongs in a git-tracked artifact, never in an AI agent's per-user memory. Per-user memory stores (Claude Code's auto-memory, Cursor's memories, and equivalents) live outside the repository — invisible to every other contributor and absent from a fresh clone — so a fact parked there is guaranteed to drift from the shared source the moment anyone else works the area. It is the most severe form of the drift this section exists to prevent: not an inconsistency between two committed documents, but knowledge that was never committed at all.

Two questions decide where a learning lands, and they are asked in that order. **Would this help a teammate?** decides whether it is committed at all. **Who is it true for?** decides which committed artifact — a question about the rule's *population*, never about its *kind*:

- A **project learning** — a convention, a gotcha, a workflow rule, a boundary — is routed by population, not by which of those it happens to be. True for **every project running this pipeline**, its canonical text belongs in this constitution, however it was learned. True across **one organization's projects and no further**, it belongs in the shared constitution that organization registers under `.ductus/config.toml` `[constitutions.*]` and `/{project}:target` loads alongside this document, bounded by the framework-as-floor rule under *Governance sources, and which governs* above. True because of something particular to **this project alone**, it stays in the project's own `AGENTS.md` (or the matching rule file under `specs/rules/`), where every contributor and every agent reads it.
- A **durable requirement** goes in its canonical artifact: a spec, a scenario, a rule, or this constitution (see [Canonical sources](#canonical-sources)).
- **Per-user agent memory** is correct only for facts that carry no value to other contributors — who the individual user is (role, persistent personal preferences) and external reference pointers (issue-tracker, chat, or dashboard bookmarks).

**Kind is not population, and routing by kind is the failure this ordering exists to prevent.** A universal rule filed under a gotchas heading is filed correctly by kind and wrongly by population, and the mistake conceals itself: the learning *is* committed, so the first question is satisfied and nothing afterwards re-asks the second. What results is this section's own drift one tier up from the memory store — knowledge committed where it is not needed and absent everywhere it is, reached by a routing default rather than by a decision anyone made, so every project but the one that learned it pays for the lesson again. The two questions are asked in order for that reason: answering the first is what makes the second easy to skip.

The test before saving to per-user memory is the first question above: *would this help a teammate?* If yes, commit it, then ask the second to decide where. A host's own rules file (`CLAUDE.md`, `AGENTS.md`) supplies the agent-specific routing that applies both. Neither question is a new authored input — an author already chooses a destination, so what changes is which question decides it, not how many questions there are, and [§design-principles](#design-principles)' bar on designs that depend on an author remembering is met rather than waived.

<!-- §pipeline-boundaries -->

## Pipeline Boundaries

- Never implement without a spec
- Never plan without resolving open questions
- Never skip phases — each phase produces artifacts the next phase consumes
- Never transition a spec to the next status without explicit user approval — present the work done and wait for the user to confirm before updating the status field
- Specs and plans are living documents — when a decision changes, edit them in place to say what is now true rather than appending an account of the change ([§plan-phase](#plan-phase)), but don't backtrack silently — a change that moves a spec backward names the back-edge it takes ([§spec-lifecycle](#spec-lifecycle)) before it is made
- Never say a finding will be "recorded" without naming the artifact and the section. The word spans several destinations with different gate semantics — an inbox item, an open question, a scenario, an acceptance criterion, a task, and the frontmatter records `/{project}:review` and `/{project}:analyze` write — each produced by a different command. A task gates on its checkbox, a criterion gates on verification against the tree when the spec closes, and an inbox item gates on nothing until `/{project}:groom` routes it — which is why `/{project}:log` is its only producer, and no finding a run produces is recorded there — so a reader who guesses wrong misjudges what the write commits them to. Name the file and the section, and say what the write does *not* touch.
- **Report outcomes, not edits.** While working, a chat reply does not reproduce the code or artifact additions and modifications the agent makes: the operator reads those on disk, in the diff, and in the commit, and a reply that restates them grows with every task and buries the outcome it exists to report. Report at the level of outcomes — which files changed, what a check or test returned, what was committed, and what the operator has to decide — and show a proposed change only when the operator asks to see it. A command's own approval gate is that request: show what approval requires, concisely, and offer the full text rather than pasting it. Failing output that explains a failure is an outcome and is quoted; naming a file or a `path:line` is locating an outcome, not reproducing a change. The rule governs chat replies only — commit messages, `review.md`, `analysis.md`, and spec and plan prose describe changes fully, because those are the records a later reader consults.

<!-- §concurrent-features -->

### Concurrent Features

**Each agent process holds its own session target**, so several agents can work in one working tree at once — one implementing a feature, another reviewing it or specifying the next — without a target written by one moving the feature another acts on. A process is told apart by an identity carried in the environment it was launched with, resolved in order: an operator-set `DUCTUS_SESSION`; otherwise the agent's own session identifier, where the agent passes one to the processes it spawns (Claude Code's `CLAUDE_CODE_SESSION_ID`); otherwise none. The environment is the carrier because it is set once at launch and inherited by everything the agent spawns — the runtime and the agent's shell see the same value — so no command depends on the model remembering to pass a name ([§design-principles](#design-principles)). Within a process nothing changes: one target, and a pipeline that is serial within a feature, so "which target does this command act on" has exactly one answer (spec 062).

**The shared default.** `.ductus/session.toml` holds the working tree's most recently set target, because every target write, in any process, also sets it. A process with no identity reads and writes only the default, exactly as a single-target session always did. A lone agent on a platform that passes its own session identifier (Claude Code) is identified even though its operator configured nothing, and it keeps the same behavior by adopting the default: it resumes the most recently set target, and what it sees differently is its session's label in `/{project}:status`, the announced adoption, and — after a restart, until the earlier session's target expires — a notice naming that earlier session as sharing the feature, since a session identifier cannot tell a finished session from an idle one. An identified process with no target of its own **adopts** the default the first time it resolves one, says so, and from then on only its own writes — or a fold or consolidation that removes the spec it targets — change it — without that pin, a process that started on the inherited default would be moved by the next write any other process made. Per-process targets live in `.ductus/sessions/`, gitignored, and one neither resolved nor written for seven days is removed by the next target write. A cleared target means no target, never "adopt the default". Resolving the target is a runtime primitive (`resolve-session`), because only the runtime can read the process environment. The markdown-only path has no sanctioned way to read it ([§runtime-host-integration](#host-integration-for-agent-runtimes)), so that path reads and writes the shared default only — a reduction stated rather than hidden.

**Sessions on one feature.** Two sessions may target the same feature; implementing in one while reviewing in another is the ordinary case. The session making the target write is told which other sessions share the feature and when each last used its target, and each of those is told once, at its next command. Nothing is blocked: status transitions are already guarded on the expected current value, so a transition computed from a stale read fails loudly.

**What stays shared, and when a worktree is still the answer.** Per-process targets isolate the *target* and nothing else. Two agents in one working tree still share its files, its git index and its commits; when their *edits* need isolating too, `git worktree` and platform isolation (Claude Code's `isolation: "worktree"` agent parameter, Cursor's worktree integration, etc.) remain the answer, and they compose with this — each worktree has its own session files. Mind the trade-off git imposes: by default `git worktree add` refuses a branch already checked out by another worktree (git-worktree(1), under `--force`), so under trunk-based work two worktrees are not both on `main` without overriding that safeguard. Two bounds stay open and are stated rather than covered. Processes with **no identity** still share one target and collide exactly as before, and nothing detects it — telling a second live unnamed process apart needs a liveness check the framework cannot make portably, so `DUCTUS_SESSION` is the remedy. And the per-contributor `cli-config-dir` stays **shared**, so with two different agent CLIs running in one working tree, the runtime reads command files from the agent `/ductus` last recorded.

**A command that removes a feature's directory must not leave any session pointing at it.** A target naming a directory that is gone is a dangling pointer in a file the framework itself owns — every follow-on command resolves it and fails, one step removed from the command that broke it. Every session in the working tree is reachable, so the rule covers all of them — the shared default and every per-process target that names the removed feature (primitive: `retarget-sessions`). There are two acceptable outcomes and the choice between them is not stylistic:

- **Re-target**, when the removal moved the content to a spec that continues the work. `/{project}:fold` is this case: the staging spec's content lands in its upstream home, so attention follows it there, and the new target is a fact rather than a guess — for every session that named it, not only the acting one.
- **Clear**, when it did not. `/{project}:consolidate` is this case: its target is a spec that already existed, which nobody chose to work on — they were removing something, not adopting it. Re-targeting there would assert an intent nobody stated, while clearing says truthfully that there is now no target. Clearing preserves the per-contributor `cli-config-dir`, so the agent identity survives.

Do either **only for sessions that actually named the removed feature**. A session pointing somewhere else is not the removal's business and is left alone. Every affected per-process session other than the acting one is told at its next command what happened to its target and which session's command did it. The shared default is changed without a notice: every process without an identity reads it, so there is no single reader to tell, and the acting command's report names it among the sessions it changed.

One bound is worth stating because nothing can close it: the session files are per-contributor and gitignored, so a teammate's session — in another clone — may still name the removed directory, and no command in this repository can reach it. What the rule buys is that no session in *this* working tree is stranded, and that a teammate's next command fails on a target that is legibly absent rather than on one the framework could have cleared and did not.

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

#### What the framework enforces, and what it does not

The two halves of this rule are not the same kind of thing, and reading them as one is how an obligation gets lost.

**A declared impact is an obligation that gates `done`.** When work on spec A identifies a change that belongs in spec B, A records the obligation in its `cross-spec-impact:` frontmatter — a list of the affected specs' slugs — rather than only in prose or an acceptance criterion, and A cannot reach `done` while any declaration is undischarged. A spec carrying an obligation nobody has discharged is not a candidate for `done`, so whether its review is fresh does not yet matter. An impact naming a spec this tree lacks blocks too, because nothing but this gate holds the declaration to its discharge. That is what separates it from a pending fold, which never holds `done` ([§spec-lifecycle](#spec-lifecycle)): the pipeline view and fold-back carry an owed fold past `done`, while an undischarged impact would disappear the moment its spec closed. The mechanism is a check in the pre-`done` gate; this section states the requirement.

**Discharge is provable, not asserted.** An impact is discharged when spec B carries the change with a back-link to A — the signpost this section already requires above. Keying discharge on the reciprocal link rather than on the declaration being removed is what makes it a gate: a key an author deletes to unblock themselves enforces nothing.

**Nothing detects an impact that was never declared.** Whether work on A implies a change to B is semantic judgment, and no check can make it — so the undeclared case stays the author's and the reviewer's, and is named as such rather than covered by a mechanism that does not exist. [§design-principles](#design-principles) is explicit that a feature with no derivable design is deferred rather than shipped in a disciplined-for-now form. What the framework closes is the narrower and more common failure: an obligation that *was* recognized and then evaporated because the only places to record it were prose and a backlog.

An acceptance criterion naming the affected spec remains good practice — it puts the obligation where a reader of the spec meets it. It is not what ensures the update happens: a criterion is author-written prose, nothing requires one to exist, and a spec with none passes every gate the framework has. The declaration is the mechanism; the criterion is documentation of it.

<!-- §numbering -->

## Numbering Convention

Feature directories take one of two forms.

**Sequential** — three-digit zero-padded numbers: `000-skeleton`, `001-observability`, `002-events`. A number is an identifier, minted in creation order. It records when a spec was conceived and claims nothing about when it should be built — dependencies between features determine the actual build order, and a lower-numbered spec depending on a higher-numbered one is the ordinary case rather than a defect. A gap in the sequence is expected and is not repaired: the counter is `max + 1`, which never looks below the highest number in use, so nothing backfills a gap and no directory is renumbered to close one — `NNN-slug` is the pointer form every `dependencies:` entry and sibling link is written in, and renumbering would strand all of them. **Three digits is a minimum width, not a fixed one**: a corpus that passes 999 continues at `1000-`, and such a directory is a feature directory like any other. What is not accepted is padding beyond the minimum — `0500-` is not a name this convention produces, and reading it as `500` would give one number two spellings. This is the default and the destination form: spec creation with no branch identifier supplied numbers sequentially, and no persisted setting changes that.

**Branch-scoped** — `{identifier}.{n}-{slug}`, where `{identifier}` is an operator-supplied token sanitized to `^[a-z0-9]+(?:-[a-z0-9]+)*$` and `{n}` counts from 1 within that identifier: `1234.1-retry-budget`, `1234.2-backoff`. The identifier namespaces the counter, so branches numbering under different identifiers cannot collide at merge. The two counters are independent in both directions — a spec root holding `050-a` and `1234.1-b` still yields `051-` next, and a branch-scoped directory never advances the sequential sequence.

**The branch-scoped form is temporary.** It exists so work on a branch can be specified without claiming a sequential number the branch has no way to coordinate. Every branch-scoped spec declares in its `folds-into:` frontmatter key the sequential spec it stands in for, and is discharged into that spec by fold-back rather than kept in place, whatever its status — see [§spec-lifecycle](#spec-lifecycle).

The membership rule that recognizes a feature directory accepts both forms and is defined in exactly one place. A surface that reads the spec corpus calls it rather than restating the digit convention; a second copy of the rule is how the two forms drift apart.

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
