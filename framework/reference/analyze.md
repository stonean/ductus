# analyze — markdown-only reference

The full set of checks (frontmatter schema, spec integrity, artifact completeness, plan consistency, plan record, task consistency, grounding, scenario consistency, scenario open questions, link-adjacent decision drift, acceptance-criterion path existence, body path existence, acceptance-criterion labels, cross-spec references, cross-service references, review state drift, analyze state drift, disposition drift, rules, project-level consistency, and severity tiers) and the finding dispositions that follow them are documented below for the markdown-only path. The numbered steps of `analyze.md`'s Instructions invoke the mechanical primitives that automate the deterministic checks; the host applies the same checks against the markdown-only path when the runtime is unavailable.

**Where a named section lives.** This file is read alongside its command, `analyze.md`, which is already loaded whenever this one is. A section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. A bare `§name` is a heading in this file or a marker in the constitution, `.ductus/constitution.md`; a `§name` with another file's path on the same line is a heading in that file, unless it is a constitution marker.

## Frontmatter schema (hard fail)

For each spec file (`spec.md`):

- A YAML frontmatter block exists at the top of the file (delimited by `---` lines).
- The frontmatter parses as valid YAML.
- The `status` field is present and one of: `draft`, `clarified`, `planned`, `in-progress`, `done`.
- The `dependencies` field is present and is a list (empty list permitted).

For each scenario file (`scenarios/{slug}.md`):

- A YAML frontmatter block exists at the top of the file.
- The frontmatter parses as valid YAML.
- Either the `section` field (new schema) or the legacy `spec-ref` field is present and non-empty. New scenarios written by `/{project}:amend` use `section`; pre-017 scenarios written before `section` existed may still carry `spec-ref`. Either field satisfies the check.

For each audit record that exists (`review.md`, `analysis.md`):

- Its frontmatter parses as a record. One that does not is undeterminable, never read as the never-run state.
- Its `decisions:` list, when present, parses. A list that does not parse is a hard fail, never an empty list: reading it as empty would ask about every settled finding again and prune nothing it should keep.

Reference: the schema is canonically declared in `framework/constitution.md` §text-first-artifacts.

## Spec integrity (blocking)

- Acceptance criteria section exists at every status (§spec-requirements). It carries **at least one checkbox item at `clarified` or later** — the minimum belongs to the clarify gate, not to `draft`. A brownfield sketch spec legitimately has none: §brownfield-process says sparse acceptance criteria are expected and valid, `/{project}:specify` tells authors to leave the section with a placeholder comment when no criteria are known yet, and `framework/templates/spec/spec.md` scopes the requirement the same way — *at least one concrete, testable criterion is required before `/{project}:clarify` will advance the spec*. Flagging a zero-criterion `draft` would gate the brownfield entry path on day one, and it is the one place the two execution paths disagreed: `check-artifacts` implements no minimum-criterion family among its nine, so the runtime already answered this way while this reference did not.
- No placeholder or empty acceptance criteria
- Open questions consistent with status — the **spec body's** `## Open Questions` (`clarified` or later must have none). Scenario open questions are a separate signal with their own check (see Scenario open questions below) and never count here. When this check fails — a spec at `clarified` / `planned` / `in-progress` with one or more open questions in the body — the spec is in the recovery state defined by spec 014. Suggested fix: run `/{project}:clarify` (its recovery path will revert status to `draft` and walk the questions), or `/{project}:amend` on a fresh question (which performs the back-edge automatically).
- No implementation code blocks (function signatures, package paths, language-specific snippets) in the spec — those belong in plan.md. Format examples, directory structures, and user-facing commands are acceptable when they define behavioral contracts.

## Artifact completeness (blocking)

- If status is `planned` or later: plan.md exists
- If status is `planned` or later and feature introduces or modifies domain entities or data structures: data-model.md exists
- If status is `planned` or later: tasks.md exists

## Plan consistency (blocking if plan exists)

- Plan references the spec
- Technical decisions section has at least one decision with rationale
- Affected files section lists specific file paths
- Plan does not contradict `specs/system.md`

## Plan record (advisory)

Enforces `.ductus/constitution.md` §plan-phase's *a plan records the design as it stands* on `plan.md` at `planned` or later. The design record is the `##` headings of the project's plan template, `specs/templates/plan.md` (§drift-prevention, Canonical sources), compared case-insensitively and with nothing else forgiven, so *Tradeoffs* does not match *Trade-offs*. Segment the plan into sections: a section is a `##` heading and every line up to the next heading at level 2 or above (a `#` heading ends one too), skipping any heading inside a fenced block or an HTML comment; a `###` stays inside the section that holds it, and the text before the first `##` is the preamble, never a finding. Report every other `##` section as `plan.md §{heading} is outside the design record` in family `plan-record`, so its stored decision is keyed `plan-record — plan.md §{heading} is outside the design record` — and report it **even when** `analysis.md` already stores a discard for that key, so the stored-decision step matches the discard rather than expiring it.

The runtime compiles the heading set from `framework/templates/spec/plan.md` into the `prune-plan` primitive, so an adopter who customizes their own plan template sees their own set only on this markdown-only path — the same bounded divergence `/{project}:prune`'s reset body has.

**Not covered:** a journal nested under a design heading — a `### Task 7 notes` inside Technical Decisions — is part of that section, and no heading test reaches it. The rule applies there all the same.

Suggested fix per finding: run `/{project}:prune`, which moves the section's durable pieces into the design record or their own homes and then removes it; or, when the section is deliberately part of this plan's design — a *Known limitations*, a *Classification* — discard the finding with that reason, which stores the decision so neither the next analyze nor any prune proposes it again.

**Severity:** advisory, and it stays advisory. The heading set is a detector, not a verdict: a legitimate design section under another heading fires too, so the answer to a false positive is one stored discard, never a gate.

## Artifact size (advisory)

Warns when a spec artifact is larger than an agent may read in one call (spec 063). An agent's file reader returns only part of a file past a host-specific cap, and a command that reads an artifact once, at setup, then works from part of it — the failure `.ductus/constitution.md` §grounding names in *A partial read is not a read*. The **subjects** are `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, and each `*.md` directly under `scenarios/`. `research.md`, `review.md` and `analysis.md` are never subjects: no command reads the first, and the two records are regenerated whole on every run, so no fix applies to them.

Measure each subject's length on disk in bytes, line endings and multi-byte characters included, against the **threshold**: `.ductus/config.toml` `[artifacts] read-size-bytes`, 50,000 when unset. A value that is not a positive whole number of bytes — a string, a fraction, zero, a negative — or an `[artifacts]` that is not a table halts the command with an error naming the setting, the value, the file and the accepted form: the default never stands in for a value the project set. There is no value that switches the check off. A subject's **read-page count** is its size divided by the threshold, rounded up, and a subject is reported when that count is two or more, so a file exactly at the threshold is not. Record each as `` `{path}` is {bytes} bytes, {pages} read pages at the {threshold}-byte read size `` in family `artifact-size`. A missing subject is not reported — at `draft`, a missing `plan.md` is a state — and one that exists but cannot be read is an unexamined target under `artifact-unreadable`, never under the threshold. The size check never escalates an unreadable subject: on a `done` spec, **Scenario open questions** and **Link-adjacent decision drift** already block on an unreadable `spec.md`, `plan.md`, `tasks.md` or scenario.

Bytes are a proxy. The runtime has no tokenizer, and the cap differs per host and per host setting, so the threshold is a heuristic the project sets and a warning says the file *may* not be read in one call, never that it *will* be truncated.

Suggested fix per finding, by the subject's kind, naming on a `done` spec which reopen it (§spec-lifecycle):

- **`tasks.md`** — `/{project}:prune` its spent task sections, which does not reopen a `done` spec. Still over the threshold after pruning means the spec itself is too big, and the `spec.md` fixes apply.
- **`spec.md`, `plan.md`** — split the spec, when a slice can reach `done` on its own, which reopens it; or trim — for a plan, its code sketches and the Affected Files rows for finished work. Trimming `spec.md` reopens it unless the trim changes no claim; trimming `plan.md` never does, since a plan edit takes no back-edge (§spec-lifecycle). No command splits a spec, so a split is done by hand: create the new spec with `/{project}:specify`, move the content, and link it from the original.
- **`data-model.md`** — split or trim, reopening it as a split or a `spec.md` trim does, but never into sub-files: review staleness tracks `data-model.md` by its exact name, so content moved to a sibling file drops out of the review's subjects.
- **a scenario** — promote it to its own spec (§scenario-promotion), which reopens the parent.
- **any subject** — discard the finding with its reason, which reopens nothing.

**A discard holds until the file grows into another read page.** A stored discard covers a finding on the same path whose read-page count is no higher than the one the discard recorded; once the file grows past it the finding fires again, because the reason given for one page does not cover two. Shrinking, growth within the page, and a threshold change that leaves the count no higher all stay covered, and a file that drops under the threshold stops firing, so its discard expires. The stored-decision step cannot hold that numeric rule by judgment, so on the runtime path `check-artifact-size` decides it and returns the covering discard's key; on the markdown-only path, compare the page count recorded in each stored `artifact-size` key for the same path.

**Severity:** advisory, and it stays advisory. A size is a heuristic about a host the check cannot see, so a false positive is answered with one stored discard, never a gate.

## Task consistency (blocking if tasks exist)

- Tasks reference the plan
- Each task has a "done when" condition
- Tasks are numbered and ordered

## Grounding (advisory)

Enforces `.ductus/constitution.md` §grounding against the spec and plan bodies: a factual claim about the **existing system** must be grounded — either cited to a primary source or marked as an assumption — rather than asserted from conjecture. `/{project}:analyze` checks the *form* of grounding (is the claim sourced or hedged), not its truth: confirming a claim against the code would require reading source, which is out of this command's scope (see Scope Boundaries). The truth check is the agent's job at authoring time (§grounding) and `/{project}:review`'s job against code.

Applies to the spec body at status `clarified` or later, and to `plan.md` at `planned` or later (when it exists). `draft` specs are exempt — claims are still forming, the same way open questions are tolerated only at `draft`.

Flag a passage when **all** hold:

- It is a **descriptive claim about existing reality** — how current code behaves, what a schema/table/column/interface contains, what an external service returns, what a config value is. Prescriptive requirements about the feature under design ("the endpoint MUST reject unsigned requests") are contracts, not claims, and are never flagged. The descriptive-vs-prescriptive call is the semantic judgment this check turns on.
- It is **asserted as fact** — not already framed as an assumption, an open question, or a proposal.
- It carries **no grounding** — no citation to a primary source (a `path:line` reference, a named query, a command and its output, or a link to an artifact that substantiates it) and no explicit assumption / Open Question marker.

Suggested fix per finding: ground the claim (read the code, query the dev database, run the command — then cite the source), or, when no reachable source can settle it, restate it as an assumption (in a plan) or an Open Question (in a spec, which reverts the spec to `draft` per §spec-lifecycle).

**Severity:** advisory in v1 — grounding is a semantic judgment with false-positive risk, and forcing it blocking before the signal is proven would erode trust the way any noisy gate does. **Promotion criterion:** promote to blocking when a single `/{project}:analyze --all` run reports 5 or more ungrounded claims across the repo on two consecutive runs (the second-run requirement guards against transient mid-authoring states where a claim lands before its source is wired in). This mirrors the **Applicable Rules citation consistency** promotion path below.

## Scenario consistency (advisory)

- Every scenario file has Context and Behavior sections (frontmatter `spec-ref` is checked under Frontmatter schema above)
- Every scenario file in `scenarios/` has a task in `tasks.md` that names its slug, or had one in some revision of `tasks.md`. `tasks.md` is an ephemeral tracking artifact (§tasks-phase) that `/{project}:prune` reduces once work is complete, and after a reset the next task is numbered 1 again, so neither the file's content nor its numbering shows whether a scenario's task was pruned; its history does. Flag a scenario, at any status, only when no revision of the feature's `tasks.md` ever named it (`git log -S'{slug}'` on that file finds no commit), and flag nothing when the history cannot be read in full — no repository, or a shallow clone — but note each unmapped scenario as unexamined, reason `no-readable-state`. Under a `done` spec the finding says it cannot tell unimplemented work from shipped behavior documented after the fact, and the operator decides. The durable record of an implemented scenario is the scenario file, the code, and git history — not a retained checkbox.
- A scenario task that is *still present* in `tasks.md` is marked complete when the spec status is `done`; an absent (pruned) scenario task is not treated as incomplete.

## Scenario open questions (blocking at `done`, advisory otherwise)

A scenario exists to organize information — to keep `spec.md` from becoming one huge document — so its unresolved questions are the spec's questions for the purpose of completeness. Report a finding when any `scenarios/*.md` under the feature carries entries in its `## Open Questions` section, naming the count and the scenarios carrying them.

- **Blocking on a `done` spec.** That state directly contradicts the completion rule in §spec-lifecycle: a spec is not complete while its scenarios carry questions.
- **Advisory otherwise.** The questions are real remaining work, but a spec still in flight is allowed to carry them.
- **No grandfather rule**, unlike Review state drift below. An absent `review.md` genuinely marks a spec as predating `/{project}:review`; an unresolved scenario question is a present-tense defect whenever it arrived, and exempting it would preserve exactly the state this check exists to surface.
- Only `## Open Questions` counts. Per §spec-requirements an open question is an **undecided blocker**; a question deferred pending a condition ("not now; revisit when X lands") is resolved *with a condition* and belongs in that scenario's `## Resolved Questions` with its trigger recorded. This is a convention rather than a skip marker on purpose — an exemptible section would let anything blocking be relabelled to ship past the check.

Suggested fix per finding, offering both exits: resolve each question via `/{project}:target {feature}/<scenario>` then `/{project}:clarify` (scenario-targeted), **or** — when the question is deferred rather than undecided — move it to the scenario's `## Resolved Questions` with the condition that will settle it.

When `--fix` is set, this check additionally reverts affected specs from `done` to `in-progress` — via `set-status` (`from: done`, `to: in-progress`) on the runtime path, a direct frontmatter edit on the markdown-only path — and emits a one-line notice for each (`reverted: specs/{feature}/{file} from done to in-progress — unresolved scenario questions in {scenarios}`). The revert is never silent. Specs below `done` are not reverted: their finding is advisory, so there is no drifted state to correct. See [046 — Scenario open-question visibility](https://github.com/stonean/ductus/blob/main/specs/046-scenario-open-question-visibility/spec.md).

## Link-adjacent decision drift (advisory)

Enforces `.ductus/constitution.md` §drift-prevention's *Decision resolution* obligation where it can be checked deterministically: an artifact's own prose asserting an open state that its own sibling link's target contradicts. The grounding check above cannot catch this — it verifies a claim is *cited*, not that it is *true*, so a stale claim that correctly cites its source passes.

Applies to every artifact in the feature directory that carries inline sibling links: `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md`. `review.md` and `data-model.md` are deliberately outside the set — a review record is pinned to its `reviewed-against` sha and describes the state at that commit, so its prose is correct as written and would flag systematically.

For each **block-level element** — the list item, table row, or paragraph containing the link — scan for open-state tells, then evaluate each sibling link in that block independently:

1. **A sibling link** is one whose target, resolved against the citing file's own directory, lands inside the feature directory. `../spec.md` from a scenario is a sibling; `../NNN-other/spec.md` from a spec is not. Resolution is lexical, never canonicalized: a target may legitimately not exist, and canonicalization would make the result depend on symlinks. Scheme-bearing targets (`https:`, `mailto:`) and bare fragments are excluded; a fragment on a sibling target is stripped and the file part used.
2. **The tell list is closed at six**, framework-fixed, with no per-project configuration surface: `open question` (and `open questions`), `unresolved`, `still open`, `not yet`, `does not exist`, `left unimplemented`. Making it configurable would let the promotion criterion below measure configuration rather than drift, and a list needing adopter curation degrades silently for exactly the projects least likely to notice.
3. **Four contexts are exempt**: a fenced code block, an HTML comment, a blockquote, and an inline code span. The code-span exemption is what lets a document *describe* this check without tripping it — this section writes every tell in backticks. The blockquote exemption is the same idea one level up: a spec documenting drift necessarily quotes the stale claim it documents, and a blockquote is markdown's own marker for "quoted material, not my assertion".
4. **A finding needs an actual contradiction.** A question-state tell (`open question`, `unresolved`, `still open`) is contradicted by a target reporting zero open questions; an implementation-state tell (`not yet`, `does not exist`, `left unimplemented`) by a spec target at `in-progress` or `done`. Every other pairing produces nothing. `does not exist` is judged against lifecycle status rather than file presence on purpose: a link that resolves always points at a present file, so a presence test could only ever fire and never filter, and a test that cannot fail is not a test.
5. **A scenario target is evaluated on its open-question count and its file existence only.** A scenario carries no lifecycle status, so an implementation-state tell against one produces no finding rather than a guess. Deriving that state from the scenario's task checkbox is explicitly rejected: a spent task pruned per §tasks-phase leaves the same absence as an unimplemented one, so the derived signal would be wrong in exactly the mature-spec case where it would matter.
6. **One finding per (block, link) pair**, naming the citing file and line, the link target, every tell that fired, and the target's contradicting state. Not one per tell — `does not exist yet` matches two tells at once, and reporting one authorial claim twice would inflate the promotion threshold with duplicates.

**Not covered, and recorded so it is not mistaken for a gap:** a link whose target *exists* but whose cited section no longer says what the citing prose claims. Verifying that needs a fragment anchor or semantic reading. Prose that merely reads as historical without a structural marker is likewise undetectable deterministically; that is an authoring convention, not a check.

Suggested fix per finding: correct the prose to describe the target's current state, or — when the prose is right and the target is wrong — fix the target.

## Acceptance-criterion path existence (advisory)

For each spec at `status: done`, for each entry under `## Acceptance Criteria`, report every filesystem path the criterion names that no longer resolves.

An acceptance criterion is a **contract**: naming a path asserts that path is part of the delivered system. Nothing re-verifies that assertion after a later spec deletes the subject, so the completion gate keeps reading a checked criterion as satisfied. That is a sharper failure than stale prose — stale prose misleads a reader who can still discount it, while a stale contract is treated as satisfied by the tooling.

- **Scope is `## Acceptance Criteria` on `done` specs, and nothing else.** Body prose may name a dead path perfectly correctly while describing history, so widening the scope would flag true statements. Body prose is **Body path existence**'s, below, which adds the two discriminators that make a body-wide check safe. Criteria below `done` describe work in flight, where a path that does not exist yet is expected.
- **Paths are read inside inline code spans** — the inverse of the tell scan's rule above, because paths are backticked by convention. That inversion is why these are two families rather than one check with a flag: one family cannot hold both parsing rules coherently.
- **The grammar**: a code span's whole trimmed content — with surrounding quotes and a leading `./` stripped — is a candidate when it contains a `/` that is **not** its final character, contains no whitespace, contains none of `{ } * ? [ ] < > $ |` or `:`, does not begin with `-` or `/`, does not contain `NNN`, and is entirely ASCII. Each exclusion is load-bearing — `:` rejects URLs, `path:line` citations, and every `/{project}:…` slash-command reference; the braces reject placeholders and `NNN` the unbraced spec-number one; the bracket and star forms reject globs; a leading `-` rejects flags; the ASCII test rejects the `…` of an elided path; and requiring an *internal* separator rejects a bare directory name used conceptually ("the feature's `scenarios/` directory"). A candidate resolves when the repo-root-relative path exists as a file **or** a directory, with a trailing `/` stripped first.
- **The criterion must be a live claim.** A path is checked only when its criterion actually claims the path is *present*. A criterion carrying any of fifteen closed phrases is exempted whole and its paths recorded as `not-a-live-claim`: deletion and retirement (`deleted`, `does not exist`, `no longer exist`, `is removed`, `are removed`, `since retired`), rename (`is renamed to`, `are renamed to`, `renamed from`, `(was` + space), migration subject (`target paths`, `target_paths`), adopter scope (`in the project`), and hedges (`if it exists`, `e.g.`). This is the tell list's co-occurrence design **inverted**: there a phrase asserting an open state is *contradicted* by a closed target, here a phrase asserting absence is *confirmed* by a path that does not resolve, so the finding would be exactly backwards — `X is deleted` is satisfied precisely because `X` is gone. Matched as phrases, not words: bare `adopter` would exempt a criterion naming a genuinely stale path. The whole criterion is exempted rather than the matched path, because a criterion about a transition names its endpoints together.
- **A criterion asserting a path was never created is exempted the same way.** One construction cannot be written as a phrase: the negator is separated from its verb by the noun it negates — ``no nested `server/.git/` repository was created`` — so no fixed phrase spans them, and the phrase that would, `was created`, exempts positive delivery claims too. Blinding the family is worse than the false positive it fixes, so this group is a clause-scoped predicate instead: a criterion is exempted whole when one of its clauses carries a negator (`no`, `not`, `never`, `without`) **before** a creation verb (`created`, `added`, `introduced`). Both lists are closed and framework-fixed, like the phrases above. Clauses split on `;`, `,` and a period *followed by a space* — never the bare character, which would cut `server/.git/` between the negator and the verb. The ordering is load-bearing in both directions: ``no `X` was created`` is a negated creation, while ``X was created and not modified since`` is a delivery claim carrying a trailing qualifier and still flags.
- **An unresolved candidate is a finding only when its own top-level segment exists.** When that segment is absent, nothing is provable — a framework project's criteria legitimately name paths that live in an adopter's checkout — so the candidate is recorded as `root-absent` under Unexamined targets below rather than flagged. In a project where those roots do exist, drift beneath them is provable again, so the rule self-corrects.
- **A path this project ships into an adopter's checkout is recorded, not flagged.** The rule above cannot catch these — the top-level segments of `specs/rules/security-backend.md`, `.ductus/constitution.md`, `.githooks/ductus-pre-commit` all exist in a framework project, yet the criteria naming them are about the *adopter's* repo, where they do resolve. Derive the destination set from `framework/bootstrap/ductus.md` — the canonical registry of what lands where — in two arms read independently. The **Shared Files** manifest tables contribute every cell that is exactly one backticked span. Each **registered agent** contributes its scaffold paths: every §Agent Registry row's `config_dir` substituted into the §Derived values `ductus` install path and Settings file formulas for its `layout`, plus its §MCP registration target when that target is project-scoped. Every registered agent counts, not only those dogfooded in this repo; a registry cell is read for its first backticked span, so a note beside the path does not drop it, and a result still carrying a placeholder, or starting `~`, is left out. A candidate that equals a destination, or is a directory containing one, is recorded as `ships-to-adopter` under Unexamined targets below. An absent or unparseable manifest yields an empty set, and a missing table empties only its own arm, so nothing is suppressed that was not derived and findings are still emitted: the failure direction is *reporting*, never silent swallowing. In an adopter project the file is absent and the suppression simply never engages — correct rather than limited, since those paths resolve there anyway.
- **An unbackticked path is missed.** Accepted: outside code font a `/`-bearing token is more often a slash command, a placeholder, or an `and/or` than a path.
- **The criteria-against-the-tree count cannot be measured while the spec is `in-progress`, and the gate's own ordering is what hides that.** The artifact family that checks criterion paths examines `done` specs only, so against an in-progress spec it has no subject, and an analysis recorded there reports zero truthfully about a run that never asked. Flip the spec to `done`, run the artifact check, flip it back, then write the record with the measured counts and their reasons. Verify and tick the acceptance criteria first: ticking edits `spec.md`, which the analysis digests, so a record written before reads stale at the gate.

Suggested fix per finding: correct the criterion to name the current path, or — when the deletion means the criterion is no longer satisfied — uncheck it and route the work through `/{project}:amend`.

## Unexamined targets (informational)

Both checks above read *targets* that may be unreadable: a link resolving to a file that no longer exists, a target whose frontmatter will not parse, a target carrying no state the tell's class can be evaluated against. None of these produces a finding — an unknown is never escalated into a defect, matching the `status-unreadable` precedent in **Cross-service references** below — **with one exception**: `artifact-unreadable` on a spec at `status: done` is a **blocking** finding rather than a skipped target.

The exception holds where the rule's reasoning does not. That rule is about *other* files — another spec's frontmatter, an upstream service's state — where the defect belongs to someone else and blocking this spec would punish the wrong artifact. `artifact-unreadable` names the spec's **own** artifact, in its own directory, that its own analysis could not read: not an unknown about someone else, but the analysis unable to examine its own subject. The concrete case is **Scenario open questions**, which is blocking at `done` — an unreadable scenario contributes no questions and, as a skip, no finding, so a scenario carrying unresolved questions that will not parse passed the gate built to catch exactly that. Below `done` it stays a skipped target: the questions check is advisory there, the spec is in flight, and an unreadable artifact mid-work is a state to report rather than a gate to fail.

They are not silent either. On the runtime path the `check-artifacts` result carries a `skipped` list of `{family, reason, path}` records over the closed reason set `target-missing` / `target-unparseable` / `no-readable-state` / `root-absent` / `ships-to-adopter` / `artifact-unreadable` / `not-a-live-claim` / `never-existed`; on the markdown-only path, note the same targets as you walk them. Render them in the **Informational** tier. The reason is `QUAL-CLAIM-001`: without the list, a family that examined every target and found nothing returns exactly what a family that could examine nothing returns, and a reader acts on the reassuring one. `clean` keeps its narrow meaning — no findings — so the assurance lives in the pair: clean with nothing skipped is verified-clean, clean with something skipped is partially examined.

**The reasons fall in two classes, and they call for opposite responses.** *Excluded by construction* — `not-a-live-claim`, `ships-to-adopter`, `root-absent`, `never-existed` — are targets a check decided not to examine; they recur on every run and the exclusion is correct. *Could not be read* — `target-missing`, `target-unparseable`, `no-readable-state`, `artifact-unreadable`, `referrer-unreadable`, `rule-citations-not-checked`, `rule-assessments-not-checked`, `rule-file-unreadable`, `references-not-checked`, `applicable-rules-not-checked`, `grounding-not-checked` — are real gaps in what the run saw. Step 18 reads the split: only the second class sets `restricted`, so a stored decision is retained when its finding's source may have gone unseen, and pruned when the finding genuinely stopped firing (scenario `only-unreadable-targets-retain-decisions`). A reason added later is classified here, once.

Every reason from `referrer-unreadable` on comes from outside `check-artifacts`, and none is an exclusion by construction: nothing decided those targets need not be examined, the run simply did not see them. `artifact-unreadable` has a second source too: step 17 records it for a subject the size check could not measure. `referrer-unreadable` counts the referrers step 9 could not read. `rule-citations-not-checked` counts the spec's rule citations when step 5 read no rule file (`examined: 0`): each is unexamined rather than missing, and a spec citing no rule records nothing. `rule-assessments-not-checked` counts the loaded rules steps 11 and 12 did not ask about — a rule with no Verification, or whose Statement carries no MUST or SHOULD keyword and so has no tier to be asked in. Under `ductus exec` it also counts a step once when the walk loaded no rule at all, or when the step's prose names no MUST or SHOULD tier: that step examined nothing, and the record says so rather than reading as clean. `rule-file-unreadable` counts the rule files steps 11 and 12 could not read, whose rules could not even be counted. A host that cannot read a rule file, or finds a rule it cannot place in either tier, records it under the same reason the walker would. `references-not-checked`, `applicable-rules-not-checked` and `grounding-not-checked` are steps 13–15, which `ductus exec` no-ops as host prose, so an exec run records each of them once whether or not the step would have applied — the walk never looked. A host that runs those steps never records them.

## Unresolved shared constitutions (informational)

A `[constitutions.*]` entry (spec 055) the run could not read is recorded in the analyze record's `unexamined-by-reason` breakdown under `constitution-unresolved`, and counts toward `unexamined`. It is an unexamined **input** rather than an unexamined target — the analysis is not missing a file it meant to inspect, it is running under fewer rules than the project declares — but it lands in the same breakdown because it falsifies the same claim: that everything bearing on the verdict was looked at.

`write-analysis` derives this from the registry itself rather than accepting it as an argument, for the reason it derives `blocking` and sums `unexamined` from the breakdown: a caller that had to supply it could omit it, and the record is exactly where a later gate will trust it. A project with no entries registered records nothing and reads exactly as it did before.

## Promotion criterion for both decision-drift checks (shared)

**Severity:** advisory at introduction, matching the grounding and Applicable-Rules checks. **Promotion criterion:** promote to blocking when a single `/{project}:analyze --all` run reports 5 or more findings across the repo on two consecutive runs, **and** every finding in those runs is confirmed a true positive.

The first half matches the existing criteria verbatim, preserving comparability and the second-run guard against transient mid-authoring states. The second half is required *here* and not for the LLM-judged checks, and the difference is the point: those criteria measure volume, which is adequate when findings are already filtered for plausibility by a semantic judgment. These two are mechanical matches — a word list and a path grammar — so a noisy implementation produces 5+ findings on two consecutive runs exactly as reliably as an accurate one. Volume alone cannot distinguish real drift from false positives at the moment that distinction matters most. The precision half needs maintainer confirmation, which is a judgment step; promoting a check to blocking is already a deliberate human decision rather than something the tool does to itself.

## Body path existence (advisory)

For each spec at `status: done`, report every filesystem path named in its body, `plan.md`, `data-model.md` or scenarios that existed in this project's history and no longer resolves.

- **Scope is everything the criterion family does not read.** `spec.md` outside `## Acceptance Criteria`, `plan.md`, `data-model.md` and every scenario of a `done` spec, with fenced blocks and HTML comments skipped. Specs below `done` are in flight and may name files they are about to create.
- **The same grammar** as acceptance-criterion path existence, read inside inline code spans, with a `#fragment` stripped first. A candidate starting `../` resolves from the citing file's directory and is dropped when it climbs out of the project; any other resolves from the project root. The `root-absent` and adopter-destination rules apply as they do there.
- **The line must be a live claim.** Body prose states transitions in forms criteria do not use, so a line is exempted — its paths recorded as `not-a-live-claim` — when it carries any of the fifteen criterion phrases above, a negated-creation clause, or any of seven body phrases (`delete`, `remove`, `retire`, `replaced`, `superseded`, `rename to`, `` ` → ` ``). Matched over the whole logical line — a paragraph, a list item, a blockquote paragraph, a table row or a heading, its hard wraps joined — so an Affected Files row whose action is `Delete` exempts itself and a wrapped sentence keeps its marker. Bare `→` is not a phrase — a pipeline diagram would hide a stale path beside it — only the arrow between two code spans.
- **History decides.** A path no commit reachable from HEAD ever held is an example, a hypothetical or a never-built alternative — a name, not a file — and is recorded as `never-existed`. A path a commit did hold is a finding. On the markdown-only path, `git log -- <path>` answers the same question. When the history cannot be read in full — no repository, a shallow clone — record each such path as `no-readable-state` and flag nothing.

Suggested fix per finding: edit the line to name the file that carries the content now, or to say what is true now; a spec body is a living document (§spec-lifecycle).

## Acceptance-criterion labels (advisory)

Every acceptance criterion carries a stable `AC{n}:` label after its checkbox (§spec-requirements), assigned by the runtime's labelling pass and permanent for the life of the criterion. Assignment belongs to the pass; **this check is the enforcement half**, and it is needed because a criterion typed by hand in an editor never touches a primitive. Three invariants, each checkable from the artifact alone with no git-history read:

- **A duplicate `AC{n}` within one spec.** A label addresses exactly one criterion, so an ambiguous one is a defect to report rather than a state a tool resolves by picking the first match.
- **`next-criterion` at or below the highest label present in the body.** The frontmatter counter is what makes a retired label unreissuable, so one that has fallen to or below the body maximum means the next assignment hands a *live* label to a second requirement. A value that is not a positive integer is reported the same way; the pass refuses such a spec rather than repairing it, since a corrupted counter may mean a label was already reissued and a silent repair would hide that.
- **An unlabelled criterion, once the spec carries `next-criterion` at all.** The gate is the field's presence, not a date: a spec with no counter has never been through the pass, and §text-first-artifacts defines that absence as "no labels assigned yet" rather than a defect. The corpus backfill is what makes the check universal — deliberately, rather than a per-spec grandfather date, which is the exemption state [046](https://github.com/stonean/ductus/blob/main/specs/046-scenario-open-question-visibility/spec.md) refused for scenario questions on the grounds that a sanctioned hiding place is worse than the gap it papers over.

**Applies at every status.** A label is an identifier, not a contract about the delivered system, so — unlike Acceptance-criterion path existence above — it is as wrong to duplicate one in a `draft` as in a `done` spec.

**Nothing here is an unexamined target.** The whole subject is the spec's own frontmatter and criteria list, both of which any path has already read, so this check never contributes to Unexamined targets above.

Suggested fix per finding: for an unlabelled criterion or a lowered counter, run the labelling pass (`label-criteria` on the runtime path; on the markdown-only path, the same `max(highest label in body, next-criterion)` derivation applied in body order by hand) — the rule is arithmetic, so both paths agree by construction. A duplicate label and a corrupted counter are author decisions the pass deliberately does not make: renumber the later duplicate to a fresh label above the counter, and reconstruct the counter from the highest label ever issued rather than the highest still present.

**Severity:** advisory. The pre-commit hook runs the pass on every commit, so the two assignable states — an unlabelled criterion, a counter that has fallen behind — are resolved by the next commit rather than owed to a gate; the two that are not assignable are rare hand-edit defects. See [013 — Text-First Artifacts](https://github.com/stonean/ductus/blob/main/specs/013-text-first-artifacts/scenarios/criterion-identifiers.md).

## Cross-spec references (advisory)

- Event types mentioned in spec or plan align with `specs/events.md`
- Error codes follow the convention from `specs/errors.md`
- Data model definitions do not conflict with other specs' data-model.md files

## Cross-service references (advisory)

A spec's derived `references:` frontmatter index records each cross-service reference as a `{service, spec}` pair, harvested from body links to a registered service's canonical repo URL (see [030 — Cross-Service References](https://github.com/stonean/ductus/blob/main/specs/030-cross-service-references/spec.md)). On the runtime path the `resolve-references` primitive classifies each entry; when the runtime is unavailable, classify each entry with the host's file tools — read `.ductus/config.toml` and the linked spec directly, with **no shell-pipeline substitution**. The repo URL is identity and navigation only and is **never fetched**; status is read from the local checkout.

For each `{service, spec}` entry, in index order, decide the outcome by what can be proven, then map it to a severity:

- **`broken`** (Advisory finding) — the service is registered in `.ductus/config.toml` `[services]` and its checkout `path` is reachable, but the target `spec.md` does not resolve (renamed, moved, deleted, or mistyped upstream, or the URL is malformed). A provable defect in *this* spec — the cross-repo analog of a broken sibling link — surfaced on every run as an **Advisory** finding (non-blocking, because references are informative and never load-bearing). Suggested fix: correct or remove the reference link in the spec body, then re-run the harvest generator.
- **`unregistered`** (informational, not a finding) — the reference's repo matches no `[services]` entry. A plain navigational link; status was not attempted, so nothing is broken. Surface it with a pointer to `/{project}:link` to register the service.
- **`not-checked-out`** (informational, not a finding) — registered, but the local `path` is missing or not a usable checkout. Nothing can be proven without a checkout, so this is **never** reported as broken.
- **`status-unreadable`** (informational, not a finding) — the target file exists but its `status` cannot be read (no or malformed frontmatter, missing or out-of-set `status`, or the link targets a scenario, which has no status). The defect is upstream's, not this spec's.
- **`ok`** (no finding) — the reference resolves and the linked lifecycle `status` is readable. A clean reference.

The load-bearing line is **provably broken** (a finding) versus **can't check** (an informational unknown): a broken link never hides behind a benign unknown, and an unknown is never escalated to a defect. This classification matches the `resolve-references` primitive and the `/{project}:status` readout exactly — the three surfaces share one contract and none wraps another.

## Review state drift (blocking)

For each spec at `status: done`, read the review record from its `review.md` frontmatter:

- `last-run` is set to a non-null timestamp. If `review.md` is **present** but `last-run` is missing or `null`, report `Review drift: done spec missing review — run /{project}:review` (**blocking**). A `review.md` that exists but carries no parseable record is **undeterminable**, which is a hard fail rather than a never-run — the two must never render alike
- `blocking` is `false`. If `true`, report `Review drift: done spec has unresolved MUST violations — see review.md` (**blocking**)
- `should-violations` is `0`. If greater, report `review drift: done spec has {n} outstanding SHOULD violation(s) — fix each, or move it under review.md's Waived findings with its rationale, then re-run the review so the count states what is outstanding` (**blocking**). §implement-phase gives a SHOULD exactly two dispositions — fixed, or moved under `review.md`'s Waived findings with its rationale — and both drop the count, so a non-zero count at `done` means neither happened. Nothing else catches it: the reconciliation family that once held the spec's `review:` block against `review.md` was retired with that second home (spec 057), and it could not have caught this anyway — one `/{project}:review` run wrote both sides, so a fix applied *during* a pass that never re-runs left them consistently stale and that check clean.

**Grandfather rule.** A `done` spec with **no `review.md` at all** is treated as pre-`/{project}:review` and exempt from this check. The absent artifact is the never-run state (§text-first-artifacts): `/{project}:review` writes the file on its first run, so its absence on a done spec means the spec reached done before `/{project}:review` existed. Adopters who want retroactive review run `/{project}:review` against the spec, after which it is subject to the drift check on every subsequent analyze. Note what the exemption is keyed on — the artifact, never a block in `spec.md`. Keyed on the block it would exempt every spec in a migrated corpus, which is the vacuity spec 057 removed from the shipped CI template.

Specs not at `status: done` are silently exempt — `review.md` is written lazily on first `/{project}:review` run, so its absence on `draft` / `clarified` / `planned` / `in-progress` specs is normal.

**`--fix` reverts; it never edits the count.** The count is the review's to write, and rewriting it here would erase the finding rather than resolve it. When `--fix` is set, this check additionally reverts affected specs from `done` to `in-progress` — via `set-status` (`from: done`, `to: in-progress`) on the runtime path, a direct frontmatter edit on the markdown-only path — and emits a one-line notice for each (`reverted: specs/{feature}/{file} from done to in-progress — re-run /{project}:review`). The revert is never silent; the notice is the point of the action. Re-running `/{project}:review` on each reverted spec is left to the operator — auto-running it during `--fix` is out of scope. The grandfather rule applies under `--fix` too: pre-feature `done` specs with no `review.md` are never reverted.

## Analyze state drift (blocking)

For each spec at `status: done`, judge the `analysis.md` this run just wrote, after step 21 — never the record it is replacing. If that record is `blocking: true`, report `analyze drift: done spec's analysis.md records {hard-fail} hard-fail and {blocking-findings} blocking finding(s) — resolve them and re-run /{project}:analyze` (**blocking**).

- **Why the new record.** Judged during detection, this check read the record the run was about to replace, and its finding could never clear: it is blocking, so it cannot be discarded, and no fix removes it before the record is rewritten. It entered the new record's own tier counts, so every run recorded `blocking: true` again — even one that fixed the finding that made the prior record blocking (scenario `analyze-state-drift-judges-the-record-it-writes`). Judged from the new record, a run that fixes every hard-fail and blocking finding leaves no drift.
- **The report names a state, not a finding.** The hard-fail and blocking findings that made the record blocking are already in its tiers; the drift report is not counted again, because a record cannot count its own verdict.
- **Advisory findings are not checked**, unlike Review state drift's outstanding SHOULD: analyze's advisory tier is advisory by design — some of its checks carry their own published promotion criteria and the rest stay advisory for good — and gating on them here would promote every one of them at once. An undispositioned advisory finding surfaces as Disposition drift instead, which is advisory too.
- **No record, no drift.** The record this run writes always carries `last-run`, so an unset one is never reported here. A `done` spec that has **no** `analysis.md` has never been analyzed; this run writes its record, and the population still without one is bounded by the CI template's record check and counted by `/{project}:audit` Family 37.
- **`--fix` does not revert on this check.** A done spec whose new record is blocking still holds the blocking findings themselves, and fixing them clears it; a route onto this spec reopens it (step 19). Since Disposition drift stopped reverting, no `--fix` trigger covers this state: it is reported on every run until a fix clears it or a route onto the spec reopens it.

## Disposition drift (advisory)

For each spec at `status: done`, judge the `dispositions:` map of each record against the record that will stand after this run:

- **`review.md`**, during detection. If its `undispositioned` is above zero, report `disposition drift: done spec's review.md records {n} undispositioned finding(s) — re-run the review command to decide each; they do not hold done` (**advisory**). This command does not rewrite that record, so only `/{project}:review` can clear it.
- **`analysis.md`**, after step 21, from the record this run just wrote — never during detection. Detection would read the record this run is about to replace, and a finding read from it could never clear: no fix removes it before the record is rewritten, so every run would record it undispositioned and report it again. Judged from the new record instead, a run that dispositions every live finding leaves no drift, and one that leaves a finding undecided reports `disposition drift: done spec's analysis.md records {n} undispositioned finding(s)` (scenario `analysis-drift-judges-the-record-it-writes`); the report is not counted in the record it judges.

A `done` spec whose record still counts an undecided finding is not a contradiction: only a gating finding holds `done` (§brownfield-inbox, Finding dispositions), and a gating one already surfaces as Review state drift or Analyze state drift. The report counts every finding nobody decided; for the observations and advisory findings among them it is the only report, so a later run can decide them.

- **A record with no `dispositions:` map is not drift.** It predates dispositions, and a backfilled map would assert dispositions nobody made. The pre-done gate blocks the same absence on an `in-progress` spec, where the record is writable at the moment the spec is being completed; on a `done` spec it waits for the command's next run.

A newly introduced advisory check that fires across `done` specs therefore surfaces as advisory disposition drift on each spec where a run left its findings undecided, and reopens none of them; one stored decision per finding quiets it until the finding's message changes.

**`--fix` does not revert on this check**, and never edits the map: the count is the recording command's to write, and rewriting it here would erase the finding rather than decide it.

## Rules (blocking and advisory)

Rules are the cross-cutting tier of the framework's three-tier requirement model (see §rules in `.ductus/constitution.md`). Discover rule files by directory walk: list every `*.md` file in the project's rule-file directory and classify each by basename suffix per the closed-suffix policy declared in `.ductus/constitution.md` §rules — `*-backend.md`, `*-frontend.md`, `*-cross.md`, or unrecognized. `/{project}:analyze` loads **every** discovered file regardless of detected stack — and regardless of the project's `[rules] surfaces` setting (`ductus.md`, **Project Configuration**) — because citation verification spans surfaces: a backend project that cites `FE-XSS-001` in a scenario covering HTML output still needs that citation verified. `[rules] surfaces` scopes `/{project}:review` enforcement only (which surface's rules are checked against code); it never prunes the rule-file set `/{project}:analyze` loads for citation resolution.

For each file with an unrecognized suffix, emit one stdout line:

```text
rule file <name> has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md
```

Then emit a single stdout line naming what was selected:

```text
loading rule files: <comma-separated basenames>
```

**When the walk selects no file at all, that line is not enough — emit an advisory finding as well:**

```text
No rule files found, skipping rule checks
```

An empty `loading rule files:` line renders identically whether the rule-file directory held nothing, does not exist, or could not be read, so on its own it makes *examined and found nothing* indistinguishable from *could not examine* — the failure §design-principles in `.ductus/constitution.md` names first and `QUAL-CLAIM-001` names in code. The finding is **advisory, never blocking**: a project with no rule files is a legitimate state, and the rest of the pass still runs. This check has been the contract since 008 (as `No security rule files found, skipping security checks`, generalized to every rule surface by 016); it was dropped from the command source by 022's parseable-procedure rewrite without a decision to drop it, and is restored here.

Rule files reach the directory from two origins: those shipped by `ductus` (introduced via their own feature spec) and those a project authors for itself (no introducing spec — see §rules Lifecycle in `.ductus/constitution.md`). `/{project}:analyze` treats both identically. The suffix governs which stacks see a file at `/{project}:review` time, but `/{project}:analyze` loads them all unconditionally.

For each loaded rule file:

- Every rule heading is level-3 and contains only the rule ID (no surrounding text)
- Every rule has the three required fields: a block-quoted Statement, `**Rationale:**` paragraph, and `**Verification:**` paragraph
- Every rule's ID matches the ID format that governs that file, resolved in the order below
- No two rules in the same file share an ID

**Which ID format governs a file.** Take the first source that applies:

1. **The file's introducing-spec data-model**, when the file has one — `{BE|FE}-{CATEGORY}-{NNN}` for the security files (`specs/008-security-rules/data-model.md`), `CFG-{CONST|ENV|FLAG}-{NNN}` for configuration (`specs/017-derive-dont-ask/data-model.md`).
2. **The format the file declares in its own header**, which is where a rule file's ID prefix and category abbreviations are canonically declared (`.ductus/constitution.md` §rules) — e.g. `quality-cross.md` declares `QUAL-{CATEGORY}-{NNN}` and lists its category abbreviations in that header.
3. **The generic rule-ID grammar** — an uppercase prefix, an `[A-Z][A-Z0-9]*` category abbreviation, and a zero-padded sequence number, as declared in `specs/008-security-rules/data-model.md` §Rule ID format. This is the shape `check-rule-ids` resolves citations against, so an ID that fails it cannot be cited from a spec regardless of what any header says.

If any check above fails, the affected rule file is treated as unloadable for the remainder of this analyze pass — no rule from it is applied to the per-rule assessments (steps 11 and 12) — and the failure is reported as a **Blocking** finding, one of:

- `Malformed rule file {path} at {location}: {reason}` — a missing required field, an ID-format violation, a malformed heading, or a file that does not parse
- `Duplicate rule ID {ID} in {file}; refusing to load` — two rules in the same file share an ID

A file whose rules drop out without a finding makes a malformed rule set indistinguishable from a clean one. These two messages have been the contract since 008, generalized from security files to every rule surface by 016; 022's parseable-procedure rewrite dropped them without a decision to drop them, and they are restored here.

A citation step 5 resolves is reported in the same way, naming the citing artifact:

- `Spec at {path} references unknown rule {ID}` — **Blocking**: no loaded rule file defines the cited ID
- `Spec at {path} references deprecated rule {ID}; targeted for removal in {version}` — **Advisory**: the cited rule is marked `DEPRECATED`, and the citation still satisfies it for the deprecation window

These are 008's messages too, dropped by the same rewrite and restored with the two above.

**Under `ductus exec` neither message is produced.** The walker checks no rule-file integrity: a rule with no Verification, or whose Statement carries no MUST or SHOULD keyword, is recorded unexamined under `rule-assessments-not-checked`, a file it cannot read is recorded under `rule-file-unreadable`, and a missing Rationale or a duplicate ID goes unnoticed — each such rule is assessed as though whole. The reduction is stated rather than silent, per the two-paths guarantee in §runtime-host-integration.

A rule file is **never** unloadable merely for having no introducing spec, or for declaring no format in its header — those conditions select which tier governs, they are not themselves failures. A project-authored rule file has no introducing spec by construction; it is validated on exactly the same terms as a shipped one, at tier 2 when its header declares a format and tier 3 when it does not. Treating the absence of an introducing spec as a validation failure would make a supported adopter extension unloadable, silently dropping its rules from citation resolution and from the fired-rule set that steps 11 and 12 assess.

### Applicable Rules citation consistency (advisory)

The rule-citation audit runs in both directions:

- **Rule fires; not cited (existing).** For every loaded rule whose Verification trigger fires against the target spec, the per-rule semantic assessment (steps 11 and 12) emits a finding when the spec does not address the rule. This direction has been live since 008.
- **Cited; rule does not fire (new in 016).** For every rule ID listed under the spec's optional `## Applicable Rules` section that did NOT appear in the fired set from the existing direction, emit an advisory finding. The author either removes a decorative citation or extends the spec to bring the cited surface into scope; either resolution keeps the section honest.

The check assumes every citation resolves to a real rule — citations to unknown rule IDs are caught earlier by the rule-integrity check (step 5) and are not reprocessed here. Specs without an `## Applicable Rules` section are silently exempt (no citations to police).

**A citation of a code-pattern rule is exempt from the second direction.** A rule file may declare that its rules verify code patterns rather than design-time commitments, as `quality-cross.md` does. Each such rule's Verification is `/{project}:review`'s quality pass flagging code in scope, so it has no trigger against a spec's artifacts and would fire this finding on every citation, however load-bearing. The exemption opens no gap. `/{project}:review` enforces the rule against the code in scope whatever the spec cites, and a `-cross` code-pattern rule applies to every code path by construction, so citing one records which rule a design applies and is never decorative. The first direction is unaffected, because such a rule's trigger never fires against a spec either.

**Severity:** advisory in v1. **Promotion criterion:** promote to blocking when a single `/{project}:analyze --all` run reports 5 or more stale citations across the repo, with the threshold met on two consecutive runs (the second-run requirement guards against transient mid-implement states where citations land before the AC that exercises them). Until that threshold is sustained, the check stays advisory so forward-looking citations remain a usable planning signal rather than a friction point.

## Project-level consistency (advisory)

These checks span the project's installed command set and constitution rather than the target feature. They catch drift in the framework files `ductus` ships, surfaced per the Drift Prevention principles in `.ductus/constitution.md` §drift-prevention. Run once per `/{project}:analyze` invocation regardless of which feature is targeted; with `--all`, run once before per-feature output.

Read inputs:

- `.ductus/constitution.md` (already loaded by `/{project}:target`)
- The installed command set (frontmatter only — do not read bodies for these checks): the files the agent layout's **Command/skill path** names, with membership drawn by that layout's **Slash-command cleanup glob** — the project's commands, never every file in the directory, since a layout's directory can be shared with the adopter's own files. The help command is that set's `help` entry.
- The bootstrap installer at the layout's **`ductus` install path**, if it exists (frontmatter only — it lives outside the project namespace)

These paths are layout-derived and are not spelled here: the bootstrap's [§Derived values](https://github.com/stonean/ductus/blob/main/framework/bootstrap/ductus.md#derived-values) table carries each of them per layout, and the layout is the one declared by the §Agent Registry row whose `config_dir` is the `cli-config-dir` that `/ductus` records for this contributor in `.ductus/session.toml` — the agent that last ran `/ductus` here, normally the one running this command; a contributor who has since switched agents gets the recorded agent's layout checked until `/ductus` runs again. When the layout cannot be determined — no `cli-config-dir` is recorded, or no registry row declares the one that is — each check that reads these paths is reported as unexamined, naming why, and is never run against a guessed layout. A derived path that does not exist is reported as unexamined, naming the path looked for — `Unexamined: {path} does not exist` — and never as a check that passed: a check that read nothing cannot tell a clean command set from one it never looked at.

Checks:

- **Generator drift** — run `scripts/gen-help-tables.sh --dry-run` (through the runtime's CLI on the runtime path, `.ductus/bin/ductus run-generator --script scripts/gen-help-tables.sh`, since `run-generator` is not an MCP tool; when the script exists in the project). Non-empty diff means the help.md command tables are out of sync with their sources. Report it as `Generator out of sync: {script}; the next commit will resolve.`
- **Anchor resolution** — every `§<name>` reference in any installed command file (typically in `Reference: §<first>, §<second>` Scope-Boundaries lines) resolves to a corresponding marker in `.ductus/constitution.md`.
<!-- audit:ignore-placeholders -->
- **Command frontmatter completeness** — every file in the installed command set has a `description:` frontmatter field; the same check applies to the bootstrap installer when that file exists. Where the layout's scaffolding copies the source frontmatter verbatim (the bootstrap, `ductus.md`, says which in §Per-Agent Scaffolding), files whose body documents an `$ARGUMENTS` parameter additionally have `argument-hint:`. A layout that writes its own frontmatter is held to the fields it writes — Antigravity's skills carry `name:` and `description:` alone, so `argument-hint:` is not checked there. Report missing fields; do not check value content.
- **Orphaned references** — every reference an adopter-owned file makes into a ductus-managed path resolves (the `check-orphaned-references` primitive on the runtime path; step 9 of `analyze.md`'s Instructions). The referrers are the `create`-strategy files the manifest never overwrites — `CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit`, and the spec root's `system.md` — and the managed roots are `.ductus/`, `.githooks/`, and the configured spec root, named bare or behind a prefix that names the project root from the referrer (`./` or a leading `/` in a root markdown file, `./` in `.githooks/pre-commit`, `../` or a leading `/` in `system.md`). `system.md` is resolved against the configured spec root rather than spelled literally, and the check reports a reference but never repairs it: a migration re-points only the references the framework itself wrote, so an adopter-authored one is surfaced and left to the adopter. A fold or a consolidation, which knows both endpoints, re-points the ones naming the directory it retires (`rewrite-spec-links`). A migration can re-point the files it owns but not these, so a chain where a later entry moves a path an earlier entry wrote into one leaves a reference pointing at nothing, silently: a dangling import loads nothing, so the file it named is simply not in context, and a hook calling a missing script fails at commit time far from the run that broke it. Report `Orphaned reference: {referrer}:{line} names {target}, which does not exist` — plus `; most likely orphaned by migration {id}` **only** when the result's `attribution` is `registry`. Under `watermark` attribution say so instead, naming `last-applied` as the only migration context available; the two must not render alike. Patterns are not references: a candidate containing `*` or `NNN` is documentation naming a shape, and testing it against the filesystem would manufacture findings out of prose. Nor is a git object name: a path glued to a preceding `:` whose token is empty or shaped like a revision (`git show abc1234:specs/x/spec.md`) names a blob no migration can move, while a variable, a path or a link label before the colon (`PATH="$PATH:.ductus/bin"`) still marks a working-tree reference. A path the project declares it *ships* to adopters (a **Shared Files** manifest destination or a registered agent's scaffold path — the destination set the criterion-path check derives — when that manifest is present, i.e. in ductus's own repo) is likewise not local breakage. Report, never repair; the adopter may have hand-edited the reference. On the markdown-only path, read those five files and test each managed-root path they name with the host's file tools. See [027 — Bootstrap Migration Registry](https://github.com/stonean/ductus/blob/main/specs/027-bootstrap-migration-registry/spec.md)'s `migration-chain-reference-integrity` for the rule.

- **Un-folded branch-scoped specs** — every branch-scoped directory (`{identifier}.{n}-{slug}`) surviving under the spec root is reported with its declared `folds-into` target and its status (the `check-unfolded-specs` primitive on the runtime path; step 10 of `analyze.md`'s Instructions). The form is a staging one: it exists so two branches can each create a spec without claiming the same sequential number, and it is discharged by folding its content into the upstream spec its `folds-into` names. Nothing runs on the merge itself — fold-back is a reviewed step, deliberately — so the merge that ends a branch's life says nothing about the directories it leaves behind. Report `Un-folded branch-scoped spec: {feature} folds into {target}, status {status}`, appending `; {target} does not resolve in this tree` when it names no directory here. That suffix is a **report**, not a defect: a branch-scoped spec exists because upstream moved, so before the merge its target normally lives on the upstream branch, and this check cannot tell which tree it is looking at. Surfacing it is what lets an operator recognize a typo they would otherwise meet as a fold-back refusal weeks later. Advisory in every case, since the check equally cannot tell an open branch from a merged one. On the markdown-only path, list the directories under the spec root whose names carry a `.` before the first `-`, and read each one's `folds-into` and `status` from its `spec.md` frontmatter. See [051 — Branch-scoped spec numbering](https://github.com/stonean/ductus/blob/main/specs/051-branch-scoped-spec-numbering/spec.md) for the grammar and the fold target's semantics.

These are advisory, not blocking — they signal framework drift that the project should resolve at its convenience. They do not prevent pipeline advancement on the target feature.

## Severity tiers

- **Hard fail (blocking)** — required-field violations and malformed frontmatter. The spec is not valid until these are fixed; pipeline advancement is blocked.
- **Blocking** — structural or content issues that must be fixed before the next pipeline gate fires.
- **Advisory** — issues that should be fixed but do not block advancement.
- **Informational** — observations that may warrant attention but are neither errors nor warnings.

## Finding dispositions

Every finding still live when detection ends — hard fail, blocking, and advisory alike — is proposed exactly one disposition before the run is recorded, and gets at most one (§brownfield-inbox, Finding dispositions). Nothing is written to `{specs-root}/inbox.md`: the inbox is the one destination no gate reads, so a finding parked there let a spec reach `done` looking finished while the finding waited somewhere else. The **Informational** tier is not findings — the unexamined-target list and cross-service reference unknowns state what could not be examined, not a defect to decide.

- **The order is detect → decide → re-check → record → render.** Detection (steps 1–17) writes nothing. Stored decisions apply first (step 18), so nothing already decided is asked again. Each remaining finding's disposition is confirmed before it writes (step 19). When anything was written, detection runs again (step 20), and `analysis.md` is written from the re-check (step 21), so the record describes the state the dispositions left and a run never makes its own record stale. The report renders last (step 22); a run interrupted between recording and reporting leaves the record.
- **Fixed** — a chore: a mechanical edit to an artifact the run audited, adding no durable requirement. It is gone from the re-check, and is recorded as `live: false` under `analysis.md`'s `## Fixed in this run`. A confirmed route whose own write resolves the finding, such as a body edit on the spec in hand, is recorded the same way. The record reads the re-check, not the kind of write, and `write-analysis` refuses any outcome but `fixed` for a finding the re-check no longer produces. A fix that fails, or turns out not to be mechanical, is reverted and is not a chore; propose a route or a discard instead. A chore is never counted fixed while its fix is not on disk.
- **Routed** — written to the home **§Groom decision tree** in `.ductus/reference/groom.md` chooses; that is its single canonical statement, and this reference does not restate it. The homes are a task, a scenario with its task, or a body edit on this spec; a scenario or body edit on another existing spec; a new spec, created through `/{project}:specify`'s procedure in the same run; or an amendment to a rule file the project owns. A route to a `done` spec names the `done → in-progress` reopen in its confirmation and performs it with `from: done`, so a status that changed since the proposal surfaces rather than being overwritten; a route to a spec that is not `done` follows `/{project}:amend`'s rules for that status. A rule file `ductus` manages and the project has not pinned is overwritten on the next update, so a finding against it is discarded with that reason. A declined spec creation leaves the finding discarded or undispositioned — never routed to a spec that does not exist. A confirmed route whose write fails (for example, a scenario slug that already exists) is undispositioned, with the failure named beside the finding. A finding is recorded `routed` while it still fires because its routed work has not landed, which is the state a stored decision exists for. Once a route's write removes the finding from the re-check, it is recorded `fixed` (above).
- **Discarded** — the finding and the operator's reason, recorded in `analysis.md`. **Never offered for a `hard-fail` or `blocking` finding**: those already gate `done`, and a discard would add a bypass the gate does not have. Such a finding is fixed or routed, and it keeps `blocking: true` in the record until the re-check no longer produces it; `write-analysis` refuses the discard before any write.
- **Undispositioned** — no decision was made: the operator declined without a reason, the route's write failed, or nobody could be asked. The record counts it, and a later run can decide it. It holds `done` only when it is hard-fail or blocking, which it does through the record's `blocking: true` whatever its disposition. Under `ductus exec` that is every live finding: steps 18–20 are host responsibility, which the subprocess walker no-ops, so an exec run matches nothing against the stored decisions (and prunes none), writes no fix and no route, and still records honestly.
- **Decisions persist.** Detection is stateless, so a decided finding fires again on the next run — a routed one until its routed work lands. Each routed or discarded decision is stored in `analysis.md`'s `decisions:` list, keyed `{family} — {message}`, with its outcome, its target or reason, `decided-at`, and `decided-by`. A later run matches each finding to the stored decision describing the same issue — by the host's judgment, because a host-worded message drifts — and passes that decision's key as the finding's `decision-key`; a matched finding counts under the stored outcome without asking. A missed match costs one repeated question and prunes the old decision; it never waives a finding, because an unmatched decision is pruned rather than applied to something else. A stored decision whose finding no longer fires is pruned — for a routed finding, that is when the routed work landed — unless the run could not read a target or a registered constitution, in which case it is retained. A target excluded by construction does not count (see **Unexamined targets** above).
- **The message is part of the key** because a finding's `path` is the **citing** artifact — the `spec.md` whose criterion or prose raised it — not the subject the finding is about. That subject appears only inside the message, so a key without it would merge every finding one artifact produces. Re-wording a message no longer makes its finding new: the host matches it to its stored decision by judgment (above), so the key's byte form matters only when no match is made. The key omits a category, because a category is a host's judgment and does not reproduce across runs.
- **A `decisions:` list that does not parse is reported, never read as empty.** `validate-frontmatter` names it as a hard fail, and the process-decisions primitive and `write-analysis` refuse it rather than read it as empty. Until it is repaired, no stored decision applies — each finding it held is asked about again — and the run cannot record over it. A malformed list can cost a repeated question, never a silently waived finding.
- **`--all` groups proposals by spec.** Each prompt offers to leave the rest of that spec's findings undispositioned, so a corpus-wide run is not dozens of single-finding confirmations.

## Record fields

The analyze record's fields and types are declared in the constitution's §text-first-artifacts; what each means:

- `analyzed-against` — HEAD sha at the time of the run, provenance only: the working tree and a commit are the same subject only on a clean tree, so it is never the staleness basis.
- `analyzed-digest` — per-path digest of the analyze subjects as the run read them from disk, excluding `analysis.md`'s own record, since a record written after its subjects are read can never digest itself. Absent is undeterminable.
- `analyzed-unreadable` — subjects that exist but could not be read, recorded rather than digested as empty, so a record cannot claim to have covered a file it could not open.
- `hard-fail`, `blocking-findings`, `advisory` — the tier counts; `blocking-findings` is named for the tier to keep it distinct from the derived `blocking` flag, and `advisory` is never gated on: some of the tier's checks carry their own published promotion criteria, and the rest stay advisory by design.
- `unexamined`, `unexamined-by-reason` — targets the run could not examine, the field that makes a clean run honest: clean with nothing skipped and clean with something skipped are two different results.
- `dispositions` — what the run did with its findings, in every tier. `undispositioned` is the live tier total less the live findings routed or discarded, so a finding the run did not itemize counts as undispositioned; a run that produced five findings and decided none is never byte-identical to one that decided all five. Absent is the pre-disposition state.
- `decisions` — stored routed and discarded finding decisions, keyed `{family} — {message}`, in the review record's shape (see **Finding dispositions** above).
