# Constitution

The governing rules for spec-driven software development. A rule only one command applies lives in that command's markdown-only reference, which this document points at.

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

Token budgets need the model call instrumented, which [§runtime-boundary](#runtime-boundary) principle 2 forbids the runtime, so they are the AI platform's (Claude Code's `/cost`, a usage dashboard, request limits). `ductus` instead catches runaway loops with `/{project}:implement`'s stuck detection, and keeps autonomy off unless `--auto` is passed.

<!-- §design-principles -->

### Design principles

Constraints on anything the pipeline asks an author or a check to do. Each is a hard filter: a design that fails one is redesigned or deferred, not shipped with a note.

- **A check that cannot run MUST never be indistinguishable from a check that passed.** One that skips or cannot reach part of its subject MUST say so — a distinct result, or a count of what *was* examined — as those states are disproportionately the broken ones. Prove a new gate **fails** before trusting that it passes (`QUAL-CLAIM-001`, [§rules](#rules-1)).
- **A check that reads git history must be given git history, and a test that reads history or shells out to a project script is a change to the job that runs it.** CI checkouts are shallow by default, so that job needs a full-depth checkout, commented, and the script in its trigger paths, or the test cannot run or silently stops; a local pass is no evidence. Prefer a test that fails loudly on a missing input.
- **Work that is not complete MUST never be indistinguishable from work that is.** Known outstanding work — a defect found at the completion gate, a claim that no longer holds, a narrowed scope, a deferred question — has exactly three dispositions: **fix it**; **record it where the pipeline will surface it again** (a task, scenario, criterion, or waived review finding) and let the status follow that record; or **decide it is out of scope and record the decision with its reason**. The inbox is none of the three, since no gate reads it, and neither is a caveat beside a completion claim. Residue knowable only by measuring is **measured**: an unmeasured gap is a task, not a caveat.
- **Never design a pipeline feature that depends on human diligence.** What requires an author to *remember* is skipped when it matters most: derive the input from artifacts, frontmatter, or history, or do not ship it.
- **A check whose subject is drawn from repository state can go vacuous, and a naive vacuity guard will then demand that an unhealthy repository exist.** **Build the subject** — a fixture carrying the condition, asserted in both directions — and let the pass over real state report *nothing to compare* as a distinct, non-failing state.

<!-- §grounding -->

## Grounding

Work from what can be observed, not from what can be guessed. When a reachable source can answer a question, the agent MUST consult it first; reasoning from the conversation alone when a primary source went unread is a defect, even when the guess was right.

### Sources, in order of authority

1. **Code and artifacts**, git history included — the ground truth for what the system *is* and what was *decided*. Read the file; do not recall it.
2. **Live, reachable state** — a dev or read-only database, a running server, logs, an actual test run. Query it rather than infer.
3. **Inference** — only for what nothing reachable answers, stated as an assumption.

<!-- §governance-precedence -->

### Governance sources, and which governs

A project may load the constitution `ductus` ships and any shared constitution its `.ductus/config.toml` registers under `[constitutions.*]` (spec 055). **The framework constitution is a floor**: a shared or project source may add and tighten rules but never loosen one, except a rule that names itself a default a project may replace. Where the framework is silent, the more specific source wins: **project > shared > framework**. **Nothing enforces this**; it binds whoever resolves a disagreement.

### A partial read is not a read

A tool that elides content — truncation, a saved-output pointer, a line cap, a `head`/`tail` window, a match-only search — returns a fragment, and the rest is unread. **Read the remainder**, or **state what was not examined** where a later reader meets the claim; prefer a reader that reports its subject's size and takes an offset over output capped downstream. This is `QUAL-CLAIM-001` turned on the agent's own reading; nothing enforces it.

<!-- §recommendations -->

### Recommendations

A recommendation is a claim: the agent MUST work it out to its result **before** presenting it, and MUST compute for **every** option the quantity they differ by — steps, files touched, time, cost — since reasoning from an option's shape to its effect is inference. When that quantity cannot be computed yet, the agent MUST say so and say what would settle it, rather than rank anyway.

**Decisions go to the owner one at a time.** Present one decision per message — the question, its context, the options, a recommendation — and wait for the answer before the next; never a grouped list, and never while work you started is still reporting. This rule is a default: a project MAY replace it in its `AGENTS.md` ([Governance sources, and which governs](#governance-sources-and-which-governs)).

### Rules

- **Prefer the source to the recollection.** Open the code, artifact, or command before asserting what it does.
- **A connected database is a primary source, not a hazard.** Confirm schema and data from dev or read-only state, discovering what is reachable from the project's configuration and any live-source pointer its `AGENTS.md` declares; reads need no authorization, writes need the task's.
- **Run the project's own command — never a hand-rolled equivalent of one.** Its declared scripts, targets and CI steps are the command; a substitute that succeeds proves only itself. A narrower invocation while iterating is fine, but the reported result comes from the project's command. A failing one is a finding to dispose of, not route around; if it cannot do the task, say which command you bypassed and why, and propose a declared command for a recurring operation as a task. A declared command authorizes nothing destructive, and nothing enforces this rule.
- **Name the assumption when you must infer** — an Open Question in a spec, a labeled assumption in a plan, a task or open question during implementation.
- **Cite what you consulted** (`path:line`, the query, the command), so the next reader can re-derive it.
- **A recorded measurement states its method and its units, or it is not re-derivable.** State the denominator, population and boundary a count was taken over: without them a later reader cannot tell a decayed figure from one taken by a different method, and trusting the newer figure is wrong exactly when the older was better informed. On a disagreement, re-derive at the recorded figure's own commit first, then diff the field across its own history. An unstated method is an unmeasured gap wearing a number.
- **Before reporting that a declarative entry misbehaved, read the entry** — its own gating fields may say it was never supposed to fire.
- **Substitute a recorded timestamp or commit into the command that writes it — never transcribe either.**
- **An enumeration is a claim of its own, and it goes stale while the requirement around it stays true.** Verify the list too, and prefer a set stated without a count.
- **Pick the checks a change must pass from its blast radius, not from its file extension.** When unsure, run everything; run history-reading checks after committing.
- **After publishing a change, read every check that ran against it — not the one whose name matches what you changed.** No verdict yet is unknown, never a pass.
- **Verify what a commit contains before reporting it landed, and pair that with a check for what it left out** — its file list and the tree's remaining changes. A review or analysis record is judged by a digest of what it read, so it may precede its commit.
- **Never pipe a command whose exit status you intend to read.** A pipeline reports its last element's status.

<!-- §pipeline -->

## Development Pipeline

Every feature follows **spec → plan → tasks → implement**. No code is written without a spec, and no implementation begins without a plan.

<!-- §spec-phase -->

### Spec Phase

Define *what* the feature does and *why*, not how. Each feature lives in a numbered directory under `specs/`:

```text
specs/
  system.md         # Architecture, shared conventions
  events.md         # Global event catalog
  errors.md         # Error handling conventions
  {NNN-feature}/
    spec.md         # Requirements, contracts, acceptance criteria
    research.md     # (optional) Background research, prior art
    plan.md         # Implementation approach, technical decisions
    data-model.md   # (optional) Domain entities, written while planning
    tasks.md        # Work items derived from the plan
    scenarios/      # (optional) Scenario files elaborating spec sections
      {slug}.md     # One file per scenario
```

`specs` is the default name: a project may rename it with `.ductus/config.toml` `[paths] specs-root`, and every command and the runtime use `specs` when that is unset.

**This is an instruction, not only a fact.** Wherever a command acts on a path under the spec root, substitute the configured name for the literal `specs/`; the primitives resolve the key themselves, so on the markdown-only path the substitution is the host's.

<!-- §spec-requirements -->

#### Spec requirements

- Every spec has a **Status** (`draft`, `clarified`, `planned`, `in-progress`, `done`), **Acceptance Criteria** (testable conditions defining "done"), **Open Questions** (undecided blockers), and **Dependencies**, and describes behavior and contracts, not implementation
- Each criterion carries a permanent `AC{n}:` label after its checkbox, assigned by the runtime's labelling pass, never renumbered or reissued, and used to cite it (spec 013). **Never hand-write an `AC{n}:` label** — a guess can collide with a retired one
- A decision deferred until a condition is resolved *with* that trigger, in Resolved Questions; open questions must be resolved before moving to the plan phase
- **A spec body must not carry a point-in-time status section** — it soon contradicts the frontmatter; record decisions in Resolved Questions and obligations in `cross-spec-impact:` or a task
- **Write a criterion's path claim so a check can see it: backticked, with an interior slash, and repo-relative**; read a `done` spec's criteria for the rest yourself, since a short skip list is a fact about backticks, not about claims, and state a claim about an **absence** in non-path form

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

Forward edges only: `/clarify`, `/plan`, then `/implement` to `in-progress` and `done`. `/implement` MUST NOT write `status: done` while the review record's `last-run` is unset or its `blocking` is `true`; `/review` changes no status. The gate composes with `/analyze`, which flags drifted `done` specs, and the shipped CI template. Three back-edges exist:

- **Backward via new questions** — `clarified` / `planned` / `in-progress` → `draft` when `/amend` records a new open question, in the same write; the next `/clarify` resolves it. `draft` is the only status that tolerates open questions in the spec body.
- **Backward via new scenario** — `done` → `in-progress` when `/amend` records a scenario; its task is implemented and the spec returns to `done`. A scenario carrying open questions takes this edge too, and the spec does not reach `done` while any remain.
- **Backward via meaningful body edit** — `done` → `in-progress` when any artifact under `specs/{feature}/` is edited *meaningfully*. An edit is **mechanical** (no back-edge) when **(a)** every change is the same token substitution, applied uniformly across the live artifacts [§drift-prevention](#drift-prevention) enumerates, mapping a deprecated label to its current one; **(b)** every change adds, removes, or rewrites a **cross-service reference** and its regenerated `references:` (spec 030); or **(c)** every change assigns an `AC{n}:` label and the `next-criterion:` counter, leaving each criterion's text byte-identical (spec 013). Anything else — new scope, changed semantics, factual corrections — is **meaningful** and takes the back-edge through the same `/amend` flow.

The three cases share one test, which decides cases the list does not name: an edit is mechanical when the diff is **determinable without author judgment** *and* **changes no claim the spec makes**. A typo fix or misplaced sweep residue is mechanical; a **factual correction** is not. Spec bodies are living documents that represent current state, and scenarios evolve the existing spec rather than spawning a new one; git history records what was written when.

**A branch-scoped spec is a staging form, not a fourth state.** A spec numbered `{identifier}.{n}-{slug}` ([§numbering](#numbering-convention)) names in `folds-into:` the upstream spec it stands in for. A pending fold never holds it short of `done`; then fold-back moves its content upstream and removes it, adding **no new back-edge** — a `done` upstream spec reopens by one of the two above.

Operational rules:

- **A retired feature's spec is deleted, not left at `done`**, which means *delivered, and still true*. When its content survives elsewhere, `/{project}:consolidate` re-points every inbound pointer **before** removing the directory, so none dangles; when nothing survives, the spec simply goes; retiring *part* of a feature is an ordinary body edit.
- **Re-open a `done` spec with the status primitive when the only intent is to reflect edits already on disk.**
- **Syncing a canonical record that lives on another spec is a mechanical edit** — that spec stays `done`, and a table synced twice moves to the spec owning the behavior.
- **Restoring a spec directory from git silently reverts uncommitted pipeline state**, status flips and ticked checkboxes included; re-apply them. Commit a status transition alone only when the spec file holds nothing else, since the pre-commit hook restages it whole and a transition staged alone is lost if the index was its only copy; to separate one from edits on disk, set the edits aside, commit the transition, then restore them, or make the transition before the edits.
- **A status transition states both ends**, so a stale read fails loudly; read the frontmatter before retrying.

<!-- §plan-phase -->

### Plan Phase

Define *how* the feature is built: a plan references its spec, addresses its open questions, states technical decisions with their rationale, names affected files, and adds a data model when domain entities change.

#### A plan records the design as it stands

`plan.md` records the design **as it currently stands**, never how it got there. Its **design record** is the plan template's own `##` sections ([Canonical sources](#canonical-sources)), and a changed decision is edited in place. Everything else lives elsewhere: verification evidence in the commit, contributor knowledge per [Shared knowledge stays in git](#shared-knowledge-stays-in-git), a finding per [Finding dispositions](#finding-dispositions), handoff state in its pending task ([§tasks-phase](#tasks-phase)), a review triage nowhere ([§implement-phase](#implement-phase)). `/{project}:analyze` reports a section outside the design record; `/{project}:prune` moves it home.

<!-- §tasks-phase -->

### Tasks Phase

Break the plan into ordered work items, each derived from the plan, finishable in one working session, and carrying a clear definition of done.

`tasks.md` is an **ephemeral work-tracking artifact**, distinct from the durable `spec.md`, scenarios and rules: a task's value is spent once checked, since those, the design record and git history record what was built ([§bug-handling](#bug-handling)'s durability test). `/{project}:prune` may drop done sections; nothing may treat `tasks.md` as a durable index. **Handoff state** — what is owed, in what order — is prose in the pending task it concerns, never a checkbox or a free-standing heading; an ordering constraint goes on the task that must wait.

<!-- §readiness-check -->

### Readiness Check

Before implementation begins, a pass/fail gate:

- [ ] Spec status is `planned`
- [ ] Acceptance criteria are concrete and testable — no empty placeholders
- [ ] All open questions are resolved — the spec body's **and** those carried by any scenario under it
- [ ] Data model exists if the feature introduces or modifies domain entities or data structures
- [ ] Plan does not conflict with `system.md` or other feature specs
- [ ] Tasks are ordered and each has a clear definition of done

If any item fails, fix the gap before writing code.

<!-- §implement-phase -->

### Implement Phase

#### Implementation requirements

- Code matches the spec's contracts and tests verify its criteria; if new work is discovered, add it as a task first, and a refactor that reveals a missing requirement updates the spec or adds a scenario first
- **A spec's ticked acceptance criteria are verified against the tree before it closes.** A ticked criterion is a claim to re-earn. The artifact check that compares criteria to the tree examines `done` specs only, and proves only that a path *resolves*, not that a claim about its contents holds. A criterion that no longer holds means the spec is edited — reopened and corrected, or consolidated away — not left ticked.
- **A spec does not reach `done` with an outstanding SHOULD.** This is [§design-principles](#design-principles)' completion filter at the review gate: the gate blocks on MUST alone, but advisory is not ignorable at the gate: a SHOULD is addressed only when **fixed** or waived with its rationale.
- **Nothing durable goes in a review record's body.** `/{project}:review` regenerates `review.md` whole, keeping only its waiver and decision lists, so what must survive goes in the spec body; a triage of findings is not a record, and the fixes it plans are tasks.
- The review's window and coverage counts are `.ductus/reference/review.md` **§Window and coverage**; measuring the criteria-path family before the close is `.ductus/reference/analyze.md` **§Acceptance-criterion path existence (advisory)**.

<!-- §constants -->

#### Constants and configuration

`specs/rules/configuration-cross.md`'s `CFG-CONST-NNN` and `CFG-FLAG-NNN` rules, enforced by `/{project}:analyze`.

<!-- §env-vars -->

#### Environment variables

`specs/rules/configuration-cross.md`'s `CFG-ENV-NNN` rules, enforced by `/{project}:analyze`.

<!-- §bug-handling -->

## Bug Handling

Bugs are unwritten or violated requirements: each shows a gap in a rule (cross-cutting), spec (feature-wide), or scenario (situational), and the fix makes that requirement more precise.

A **chore** — lint, dependency or repo-hygiene cleanup, a standalone refactor — adds no requirement and belongs to no feature, so it spawns no rule, spec, or scenario. **A chore a run finds is fixed in that run** ([Finding dispositions](#finding-dispositions)), via a disposition task when `/{project}:implement` finds it outside its spec; one a person logs stays a checkbox in `specs/inbox.md` until done, then is removed. The test is **durability**: rules, specs and scenarios hold what must stay accurate, while a chore's value is spent once complete. Route requirement gaps through the decision tree below; do chores directly.

### Bug Decision Tree

The first matching condition determines the route:

1. **No rule covers this cross-cutting concern** — promote a rule, then fix the code.
2. **No spec exists for the behavior** — write the spec, then fix the code.
3. **The spec is ambiguous or incomplete** — correct the spec, then the implementation.
4. **The spec is clear but the implementation is wrong** — add a scenario, then fix the code.

<!-- §scenarios -->

### Scenarios

A scenario is a spec at a lower level of abstraction — same format, narrower scope — in a `scenarios/` directory beside its spec: **section** frontmatter (the parent section it elaborates), **Context**, **Behavior**, and optional **Edge Cases**. The scenario-creation primitive writes the frontmatter and question scaffolding, so pass it the body only; the task-append primitive can succeed while writing nothing — deduplicating against a task already referencing the scenario, or a pending task with the same title and items — so read its result, not its exit status.

#### Scenario lifecycle

A scenario has no status: creating one appends a task to the parent's `tasks.md`, which carries completion, and the scenario stays as a requirement until obsolete, then is deleted. Write one for a bug the spec covers only broadly, an edge case found while implementing or reviewing, or a section grown too large; fix the spec itself when it was missing or ambiguous, and update a scenario already covering the behavior.

<!-- §scenario-promotion -->

#### Scenario promotion

A scenario with more than three edge cases, a behavior section longer than its parent's, unrelated open questions, or a concern shared with siblings warrants its own spec: the user runs `/specify` and replaces the scenario with a dependency reference. Promotion is a user decision.

<!-- §rules -->

### Rules

A rule is an enforceable requirement for a **cross-cutting** concern (security, performance, concurrency, observability, accessibility, audit/compliance, data handling, etc.), kept in `specs/rules/{rule-set}.md`, cited from specs by ID (`BE-AUTHN-001`), and verified by `/{project}:analyze` against feature artifacts.

#### Rule format (summary)

Each rule has an **ID** (permanent), a **Statement** (a block quote with RFC 2119 keywords of one tier: MUST/MUST NOT blocking, SHOULD/SHOULD NOT advisory, MAY tierless), a **Rationale**, and a **Verification** for `/{project}:analyze`. The schema, ID grammar and Verification phrasing are canonical in `specs/008-security-rules/data-model.md`, which binds every rule to two disciplines: a MUST never requires CI, and a claim about an external standard is fetched, never recalled. The specific category abbreviations a given rule file uses are declared in that file's own header.

#### When to write a rule

**A new rule on an existing rule surface is added to that surface's owning spec through the back-edge**; only a new rule file or surface gets a new spec. A rule is justified when **all four** hold: **cross-cutting**, **citable**, in a **governance-recognized category** (security, performance, concurrency, observability, accessibility, audit/compliance, data handling), and **generalizably** worded. A situational concern is a scenario, a feature-wide one an acceptance criterion, and a covered one a citation.

#### Filename suffix

Every rule file MUST end in `-backend.md` (backend stacks), `-frontend.md` (frontend stacks), or `-cross.md` (all stacks), with no hardcoded allowlist; `/{project}:review` selects by it, and `/{project}:analyze` loads every discovered file regardless of stack; a file with any other suffix loads for every stack with a warning, never silently skipped. A project may exclude a whole file from review — never from `/{project}:analyze` — under `[[review.disabled-rule-files]]` with a mandatory `reason`.

#### Lifecycle

Rule IDs are never renumbered, and a rule is deprecated with a `**DEPRECATED in {version}:**` label and a removal target version before removal (invariants: `specs/008-security-rules/data-model.md`). A rule file shipped with `ductus` has its own introducing spec; a project's own needs none — placing it in the rule directory registers it, and its header declares its ID prefix — and no consumer may treat the two differently. **Recorded backfill:** `api-backend.md`, `accessibility-frontend.md` and `performance-frontend.md` shipped in `9ccbd0b` and gained owning specs `068-api-backend-rules`, `069-accessibility-frontend-rules` and `070-performance-frontend-rules` on 2026-10-19.

<!-- §brownfield-inbox -->

### Brownfield Inbox

`specs/inbox.md` captures todos a person has not yet assigned to a spec: `/log` is its only producer, and `/groom` routes each item home or, for a chore, does it. No command appends on its own ([Finding dispositions](#finding-dispositions)). Its standing use is **brownfield migration** — parking known issues until a spec absorbs them — so do not frontfill bugs nobody is working on. When an item routes to a chore, fix it, leaving it only when the fix is blocked or proves not to be mechanical. The brownfield backlog drains toward empty as adoption completes; the file persists while people log todos. **The standing count is a notice, never a gate**: `/{project}:status` renders it on every run, so capture stays free. Grooming follows `.ductus/reference/groom.md` **§Inbox upkeep**.

#### Finding dispositions

Every finding a pipeline run produces MUST get exactly one disposition before its spec can reach `done`: a `/{project}:review` observation, a `/{project}:analyze` finding live when detection ends, and an issue `/{project}:implement` surfaces outside the task in hand.

- **Three dispositions.** **Fixed** — a chore, fixed in the run and on disk. **Routed** — to an artifact the pre-`done` gate reads, by the Groom decision tree (`.ductus/reference/groom.md` **§Groom decision tree**): a task, scenario or body edit on an existing spec, reopening a `done` one; a new spec via `/{project}:specify`; or an amendment to a project-owned rule file. **Discarded** — with its reason, in the run's own record. The inbox is not a disposition.
- **Detect, decide, re-check, record.** Detection writes nothing, each write is confirmed first, and the record is written from a re-check; with no operator, nothing is written and undecided findings are recorded undispositioned (`.ductus/reference/analyze.md` **§Finding dispositions**).
- **Scope decides the destination.** Inside the current **task** → fix it there; inside the current **spec** → a new unchecked task on it, since an in-progress spec is the home its own defects need; outside the spec → one of the three dispositions. `tasks.md` does not thereby become a capture queue or a durable record: a missing requirement still becomes a scenario or spec edit.
- **Record, do not derail — the disposition task.** `/{project}:implement` records a finding outside its spec as a `Disposition out-of-spec finding: {summary}` task and keeps working; working that task disposes of the finding.
- **A finding that already gates `done` cannot be discarded.** Analyze hard-fail and blocking findings are fixed or routed; review MUST and SHOULD violations keep their fix-or-waive model.
- **Decisions persist** in the record of the command that made them (`decisions:`), so a later run does not ask again.
- **Severity raises salience, not the routing**: security issues and leaks are surfaced first.
- **The gate asks for a decision, not a fix.** The pre-`done` gate blocks while either record counts an `undispositioned` finding, or has no `dispositions:` map on an `in-progress` spec.

<!-- §brownfield-process -->

### Brownfield Process

Brownfield projects adopt `ductus` incrementally: start broad and decompose through scenarios.

#### Capture → incremental growth → promotion

1. **Capture** — `/specify` with whatever description exists; sparse acceptance criteria are expected and valid for brownfield use.
2. **Incremental growth** — each bug fix, enhancement or clarification adds precision.
3. **Promotion** — a scenario that outgrows its parent becomes a spec ([Scenario promotion](#scenario-promotion)).

When `/groom` meets an item no spec covers, it sends the user to `/specify` first: `/log` records, `/groom` routes, `/specify` creates.

<!-- §text-first-artifacts -->

## Text-First Artifacts

`ductus` treats every artifact — constitution, specs, plans, tasks, scenarios, rules — as plain markdown the agent can edit with `Edit`. This is load-bearing: the agent's write path stays simple, PRs review glanceably, and merge conflicts stay rare and human-resolvable. The **artifacts** are usable standalone with no tooling beyond the AI agent. The *pipeline* that reads them requires the runtime ([§runtime-boundary](#runtime-boundary)), which parses and writes the same markdown a contributor edits in an editor.

### Principles

- All `ductus` artifacts are markdown by default. The agent reads and writes them with the same `Edit` flow used for code.
- Structured metadata lives in YAML frontmatter at the top of each markdown file; the document body remains markdown prose.
- Cross-artifact references use standard relative markdown links (`[label](../path.md)`), not wiki-links — this keeps PRs reviewable on GitHub and viewers like Quartz/Obsidian still resolve them.
- Source-of-truth artifacts are markdown. Structured derived views are regenerated from canonical sources and never become the canonical record.
- Structured derived views (SQLite caches, JSON indexes, generated graph data, binary artifacts) MUST be gitignored and regenerated on demand by their consumers.
- Exceptions to text-first source-of-truth require an explicit constitutional amendment with stated rationale.
- **A markdown link in a spec body creates a `dependencies:` edge — cite in prose when you mean a citation.**
- **Write pipeline state files with the file-writing tool, not shell redirects** — their permissions are scoped to those tools.
- **Never stage the whole worktree in a project running this pipeline** — it sweeps untracked spec drafts into a commit.
- **A scenario's link to a sibling spec needs one level more than a spec's**, and a spec-body link can close a dependency **cycle**.
- **When an edit's purpose is to remove a section, diff the structure, not only the prose** — a heading swallowed by an unterminated code span vanishes silently.

### Frontmatter Schema

Required on **spec files**, **scenario files**, and the two **audit records** (`review.md`, `analysis.md`), which a gate reads. Other artifacts (`system.md`, `errors.md`, `events.md`, `inbox.md`, plan, tasks, rule and README files) MAY carry frontmatter when a consumer benefits.

#### Spec files

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `status` | yes | string | `draft`, `clarified`, `planned`, `in-progress`, or `done` |
| `dependencies` | yes | list of spec slugs | **Generated** by `derive-dependencies` from body links to sibling specs, except under `## See also` |
| `references` | no | list of `{service, spec}` | **Generated** by `derive-references`; navigation only (spec 030) |
| `next-criterion` | no | integer | **Maintained by the labelling pass**; never lowered, and absent means no criterion has been labelled yet (spec 013) |
| `cross-spec-impact` | no | list of spec slugs | **Hand-authored** obligations under [§cross-spec-impact](#cross-spec-impact) |

#### Scenario files

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `section` | yes | string | The parent spec section the scenario elaborates |

#### Audit records

`/{project}:review` records its run in `review.md`'s frontmatter and `/{project}:analyze` in `analysis.md`'s; **`spec.md` carries neither**. **The absent file is the never-run state**; an unparseable record is **undeterminable**, a third state that MUST NOT be collapsed into never-run. Field semantics: each command's reference.

##### Review record — `review.md`

| Fields: type, required in bold |
| --- |
| **`spec`**: string; **`last-run`**: ISO-8601 UTC, nullable; `reviewed-against`: sha; provenance, never the staleness basis, since the working tree and a commit are the same subject only on a clean tree; `diff-base`: sha; **`must-violations`**, **`should-violations`**, **`low-confidence`**: integer; `dispositions`: map of observation counts; `examined`: integer, the in-scope files the review read; `scope`: integer, the entries in the resolved scope — the plan's Affected Files entries as written, a pattern counting once, united with the paths modified since `diff-base` — so the two are equal when fully examined only if no entry is a pattern; `skipped-passes`: list; `reviewed-digest`: map of durable-contract path → sha256; `reviewed-unreadable`: list; **`blocking`**: boolean, derived; `waivers`: list; `decisions`: list |

##### Analyze record — `analysis.md`

| Fields: type, required in bold |
| --- |
| **`spec`**: string; **`last-run`**: ISO-8601 UTC, nullable; `analyzed-against`: sha (provenance only); `analyzed-digest`: map of subject path → sha256; `analyzed-unreadable`: list; **`hard-fail`**, **`blocking-findings`**, **`advisory`**, **`unexamined`**: integer; `unexamined-by-reason`: map of reason → integer; `dispositions`: map of finding counts; `decisions`: list; **`blocking`**: boolean, derived |

#### Open-schema rule

Other fields are permitted and ignored by uninterested consumers, except the residual record block, which this schema moved; consumers MUST NOT error on them, and `/{project}:analyze` reports them as informational. Stale fields in done specs (`title`, `tags`, `spec-ref`, `track`) produce no findings.

### Validation Severity

`/{project}:analyze` reports four tiers: **Hard fail**, a malformed artifact (frontmatter missing or not valid YAML; `status` missing or outside the set; `dependencies` missing or not a list; a scenario with neither `section` nor legacy `spec-ref`; an existing audit record or `decisions:` list that does not parse); **Blocking**, an inconsistent artifact set (a record block in a spec's frontmatter — the spec parses, so not a hard fail — no `plan.md` on a `planned` spec, an unknown rule ID); **Advisory**, never gating; **Informational**, unknown fields. Hard fail and Blocking prevent advancement.

<!-- §runtime-boundary -->

### Runtime Boundary

`ductus` ships a runtime binary alongside the markdown framework, acquired by `/{project}` during adoption, to execute the deterministic portions of pipeline commands without an LLM. This subsection defines what the runtime can and cannot do; deviations require their own constitutional amendment.

#### Five principles

1. **Markdown is source of truth** — the runtime MUST NOT own state the markdown cannot reconstruct. Runtime-owned data (caches, indexes, parsed graphs) is derived and gitignored, per the existing rule on structured derived views.
2. **Determinism only** — the runtime MUST NOT call an LLM. Work requiring semantic judgment (content quality, `/clarify` resolution, `/specify` sketching, per-rule Verification reads, `/groom` routing) stays in slash commands.
3. **Required, and acquired by the pipeline** — `/{project}` acquires the pinned runtime as part of adoption, so a bootstrapped project has the binary and pipeline commands MAY assume determinism rather than specifying two executable paths to one result. The runtime is *ductus-owned*: acquired and version-managed by the pipeline into a store it writes, never whatever `PATH` resolves. A project that supplies its own binary declares it (`[runtime] path`) and is equally supported. Acquisition failure halts the run rather than degrading, because a requirement that quietly is not one leaves both paths alive. Shell pipelines that parse frontmatter or markdown structure (`awk`, `sed`, `grep` pipelines, `for` loops over files) are **not** a sanctioned substitute for the runtime primitives or the host's file tools.
4. **Schema follows the constitution** — the runtime MUST read frontmatter and artifact structure according to the schemas declared in this document. Schema changes ship through the constitution; the runtime MUST update to match. The constitution MUST NOT import runtime types.
5. **MCP is the seam** — the runtime MUST expose its capabilities as MCP tools so slash commands can call them when they want determinism. This keeps the runtime accessible to any agent host and prevents `ductus`-specific coupling. A primitive that, called without a prompt, would run a command the project supplies, write outside the project and its staging directory, or write a host's settings or command files MAY be left off the MCP surface; it stays a `ductus <name>` subcommand and a `ductus exec` step.

<!-- §runtime-host-integration -->

#### Host integration (for agent runtimes)

Backticked primitive names are the runtime's MCP tools, bare `<verb>-<noun>` (one principle 5 leaves off MCP runs through the CLI). Hosts prefix them — Claude Code `mcp__ductus__<verb>-<noun>`; Auggie / Antigravity `mcp:ductus:<verb>-<noun>`; OpenCode a `<server>_<tool>` name under `ductus`; Pi a `ductus__<verb>-<noun>` tool from the `.pi/extensions/ductus.ts` bridge, which wraps the runtime's MCP server over stdio because Pi has no built-in MCP (spec 064) — so match the prefix. With the server registered, **call the tool** for each step, fetching a lazy schema first (`ToolSearch`); without it, walk the same prose with file tools; the shell pipelines principle 3 names stand in for neither. The two paths share one contract, and neither wraps the other. A command's markdown-only reference, `framework/reference/<command>.md` (installed at `.ductus/reference/<command>.md`), is read only where a step points at it; a command's Instructions open with a one-line pointer here.

#### Eligibility criteria

A capability is runtime-eligible only when **all three** hold:

1. **Deterministic** — no semantic judgment required; the same inputs always produce the same outputs.
2. **Currently mechanical** — already either (a) executed by an LLM following procedural instructions in a slash command body, or (b) implemented as a bash script the framework invokes (pre-commit hooks, generators, CI).
3. **Specifiable as prose** — the capability can be stated completely enough that the Markdown-only reference documents it, and a primitive mirrors that reference rather than introducing policy of its own. This is what keeps the specification and the implementation one thing: the reference is where the policy lives, the primitive is how it runs.

A capability that fails any criterion stays out of the runtime. Anything that requires reading prose for intent is permanently LLM-owned regardless of how mechanical its surface looks.

**Eligibility is a default, not a permission**: a capability meeting all three is a primitive, which hooks, CI and audits call instead of reparsing markdown, and a shell script is the fallback only when a criterion genuinely fails, since each such parser is a fresh copy of a tested parse that drifts from the others.

#### Acquisition invariant

The repository's CI MUST include a job that exercises acquisition end-to-end on every supported platform: fetch the published asset for the target, verify its sidecar digest, install it into a temporary store, and execute the installed binary. A change that causes this job to fail — i.e. a release whose assets an adopter cannot actually acquire — is a constitution violation, not a feature.

#### Versioning

The runtime ships in lockstep with the framework. A `ductus` release includes the binary built against the schemas in that release; an adopter's `ductus` version pins their compatible runtime version, eliminating schema/runtime drift as a failure mode.

#### What the runtime is not

To prevent scope creep, the runtime MUST NOT be a spec authoring tool, MUST NOT be a workflow orchestrator, MUST NOT be a long-running service, and MUST NOT be a storage layer. Lifting any of these exclusions requires a constitutional amendment.

Specific capabilities are introduced through their own feature specs, beginning with spec 022 (deterministic runtime).

<!-- §drift-prevention -->

## Drift Prevention

### Canonical sources

For every kind of fact described in multiple places, one location is authoritative. Other documents that describe the fact MUST reference the canonical source rather than restate it. **Referencing means a pointer, never a copy**: nothing detects a copy rotting, every reader pays for it in full, in every session, and a historical record is git's job. Replacing a copy with a pointer changes what a document asserts, so it takes the back-edge.

| Fact | Canonical source |
| --- | --- |
| Pipeline command behavior | `framework/commands/*.md` (or `framework/bootstrap/configure/{key}.md`), with its markdown-only reference under `framework/reference/*.md` |
| Lifecycle; rules tier; grounding; contributor-knowledge routing | this document: §spec-lifecycle, §rules, §grounding, §drift-prevention |
| Frontmatter schema | §text-first-artifacts |
| Validation severity tiers | §text-first-artifacts (Validation Severity subsection) |
| Per-agent permission set | `framework/bootstrap/configure/{key}.md` |
| Constitution section anchors | its `<!-- §<anchor> -->` markers |
| Command frontmatter (description, argument-hint) | each command's own frontmatter |
| Runtime contract / boundary | `framework/constitution.md` §runtime-boundary |
| Rule file format and ID conventions | `specs/008-security-rules/data-model.md`; `CFG-`: `specs/017-derive-dont-ask/data-model.md`; `QUAL-`: `specs/036-quality-cross-rules/data-model.md` |
| Service registry schema (`.ductus/config.toml` `[services]`) | `specs/030-cross-service-references/data-model.md` |
| A plan's design record | the `##` headings of `framework/templates/spec/plan.md`, installed as `specs/templates/plan.md` |
| Decision-drift tell lists and grammars | `specs/045-decision-state-drift-detection/data-model.md` |
| Scenario→task referencing rule | `specs/067-analyze-and-audit-checks/data-model.md` |
| Spec-root resolution (`[paths] specs-root`) | §spec-phase |
| Constitution content — what belongs in it, which rules adopters receive | `specs/050-constitution/spec.md` (a spec that changes *behavior* still amends the principle it contradicts) |

Name the canonical source here for any new kind of fact several documents reference.

### Cross-document references

When document B describes content authored in document A, B back-links to A — a relative link, an anchor reference (`§anchor`), or a section name. Changing A includes auditing every back-link to A.

- **No dead references in live artifacts.** When renaming or removing a name — a spec slug, a capability, a command, an identifier, even a parenthetical descriptor — update every reference across the project's live artifacts in the same change: specs (`done` bodies included), rules, command sources, scripts, CI, docs, README. The uniform sweep is mechanical ([§spec-lifecycle](#spec-lifecycle)), kept apart from unrelated edits; **keep the sweep's own target list current**, or it greps somewhere clean.
- **A retired filename is not a retired name — sweep it by meaning, not by token.** Where a file is still read under its old name, a resolution ladder or fallback tier, a migration procedure, and the prose, shell and configuration spelling either keep it; elsewhere the current name stands wherever an artifact states current behavior, and the retired name only where the decision to change it is recorded. A ticked criterion is swept like any other reference, because only the filename moved; annotation is for a criterion whose behavior was superseded. The runtime computes which specs a sweep reopens from the diff, so a reworded line takes the back-edge.
- **A behavior change needs a prose-claim sweep, not just an identifier sweep** — stale claims carry none of the changed tokens, so grep by meaning; one in a `done` spec body is a meaningful edit.
- **Never edit an installed command file directly** — change its source, or pin it.
- **Renaming a repository orphans contributor-local state that no migration can reach** — move path-keyed agent state on each machine.
- **An anchor reference resolves by line, and a line wrap alone breaks it.** A `§name` is qualified only when its line names its document, and within a document it matches heading text longest-first; a subsection has no marker, so cite its marked section.
- **A sweep can make a quoted instruction point at itself — classify by who emits it, never by the string.**
- **Replacing an embedded copy with a pointer is a corpus sweep, not a deletion** — find what describes the copy, criteria included.
- **A document declaring itself the authoritative shape of something the tooling writes drifts silently — check it against a record the tooling actually wrote, not against the code.**

### Decision resolution

Closing an open question, shipping a scenario, advancing a status, or adopting a rejected option obliges correcting, in the same change, every artifact that described the prior state. **A resolution is not complete while a sibling artifact still describes the prior state**: it reads as work owed, not as stale.

### Template-rule alignment

Every blocking `/{project}:analyze` check has template scaffolding that passes it by default, and every template section traces to a rule; a diff touching one without the other is incomplete.

### Manifest discipline

A file list several commands distribute or reference lives in one place — a shared section or a registry they all read.

### Shared knowledge stays in git

Knowledge that would help another contributor belongs in git, never in an agent's per-user memory. Two questions route a learning, in order: **would this help a teammate?** decides whether it is committed, and **who is it true for?** decides where — by *population*, never *kind*. True for **every project running this pipeline**: this constitution. True across **one organization's projects**: the shared constitution it registers under `[constitutions.*]`, within the framework-as-floor rule. True of **this project alone**: its `AGENTS.md` or a rule file. A durable requirement goes in its canonical artifact ([Canonical sources](#canonical-sources)); per-user memory holds only personal facts and external pointers. **Kind is not population**: a universal rule filed as a gotcha is committed, so nothing re-asks the second question. A host's rules file carries this routing as a pointer here.

<!-- §pipeline-boundaries -->

## Pipeline Boundaries

- Never implement without a spec, plan without resolving open questions, skip a phase, or transition a spec's status without explicit user approval — present the work done and wait for the user to confirm
- Specs and plans are living documents: edit a changed decision in place, but don't backtrack silently — name the back-edge a backward move takes ([§spec-lifecycle](#spec-lifecycle))
- Never say a finding will be "recorded" without naming the artifact and section, since each destination gates differently
- **Report outcomes, not edits.** A chat reply reports which files changed, what checks returned, what was committed, and what the operator must decide — not the diff, unless asked, since restating it buries the outcome the reply exists to report; commits and records describe changes fully.
- **End a report with the remaining work, or say there is none.** A reply on finished work is short, and its last line is *No further work identified* or the outstanding items, tagged **Must** (wrong or incomplete until done), **Should** (expected before finishing unless a recorded reason waives it) or **Advisory** (nothing holds on it), each saying where it is recorded or that the decision is the operator's. The reason is the reader, who should not have to infer what is left. The list reports work and does not dispose of it: each item still takes a disposition §design-principles names.

<!-- §concurrent-features -->

### Concurrent Features

**Each agent process holds its own session target** (spec 062), identified by `DUCTUS_SESSION`, else the agent's session identifier (Claude Code's `CLAUDE_CODE_SESSION_ID`); `resolve-session` reads it; within a process, one target and a pipeline serial within a feature. Every target write also sets the shared default, `.ductus/session.toml`, which an unidentified process uses and an identified one adopts once. Per-process targets live in gitignored `.ductus/sessions/`; the markdown-only path cannot read the process environment, so it reads and writes the shared default only. Sessions on one feature are told of each other, never blocked.

Per-process targets isolate the target only: agents in one tree share files, index and commits, so `git worktree` and platform isolation remain the answer for isolating edits, though by default git refuses one branch in two worktrees. A push carries every local commit: list and name another session's before pushing. Two bounds stay open: processes with no identity share one target, and with two agent CLIs in one tree the runtime reads command files from the agent `/ductus` last recorded. A command that removes a feature's directory leaves no session in this tree pointing at it: `/{project}:fold` re-targets each to the spec that continues the work, since its content moved there, and `/{project}:consolidate` clears each, keeping `cli-config-dir`, since its target is a spec nobody chose to work on. A teammate's session in another clone is unreachable and may still name the removed directory.

<!-- §cross-spec-impact -->

### Cross-Spec Impact

A change one spec's work finds another spec needs is recorded **in the affected spec** — a criterion, scenario, or signpost back-linking the originating spec — reopening it if `done`. The project configuration file is a shared project-side database, not one spec's schema: changing it is cross-spec impact only when it modifies a key another spec documented.

#### What the framework enforces, and what it does not

**A declared impact is an obligation that gates `done`**: spec A lists affected specs in `cross-spec-impact:` and cannot reach `done` while one is undischarged, unlike a pending fold, which never holds `done`. **Discharge is provable, not asserted** — B's back-link to A, not the key's removal. **Nothing detects an impact that was never declared**; that stays the author's and reviewer's judgment. A criterion naming the affected spec remains good practice but ensures nothing.

<!-- §numbering -->

## Numbering Convention

Feature directories take one of two forms, recognized by one membership rule defined in one place, which every surface reading the spec corpus calls rather than restating it.

**Sequential**, the default — `000-skeleton`, `001-observability`: `max + 1` in creation order, never renumbered or gap-filled; dependencies decide build order. Three digits is a minimum width, never padded wider.

**Branch-scoped** — `{identifier}.{n}-{slug}`, with `{identifier}` sanitized to `^[a-z0-9]+(?:-[a-z0-9]+)*$` and `{n}` counting from 1 within it, on a counter independent of the sequential one. The form is temporary: `folds-into:` names the sequential spec it stands in for, and fold-back discharges it, whatever its status ([§spec-lifecycle](#spec-lifecycle)).

<!-- §markdown-standards -->

## Markdown Standards

All `.md` files must pass `npx markdownlint-cli2` with the project's `.markdownlint-cli2.jsonc`, the canonical rule list.
