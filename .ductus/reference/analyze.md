# analyze — reference

The checks, then the finding dispositions that follow them. The numbered steps of `analyze.md`'s Instructions invoke the primitives that run the deterministic checks; a primitive's own rules are its `--help`.

**Where a named section lives.** A section named below with no heading here is in the command file, `analyze.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker.

## Frontmatter schema (hard fail)

The schema and its hard fails are `.ductus/constitution.md` §text-first-artifacts. `validate-frontmatter` (step 2) judges the spec file and each audit record and scenario beside it: an unparseable record is undeterminable, never the never-run state, and an unparseable `decisions:` list is a hard fail, never an empty list (**Finding dispositions** below).

The path chooses the row: a `spec.md` takes the spec row, a file under `scenarios/` the scenario row, `review.md` and `analysis.md` their record checks, and any other path is refused. A `spec.md` run also checks each `scenarios/{slug}.md` beside it, each finding keyed by that path: a YAML block exists at the top of the file and parses, and `section`, or the legacy `spec-ref` of a pre-017 scenario, is present and non-empty; a missing `status` or `dependencies` is no finding there. A scenario the run cannot read is a hard fail, as an unreadable record is.

## Spec integrity (blocking)

- Acceptance criteria section exists at every status (§spec-requirements), with **at least one checkbox item at `clarified` or later**. A zero-criterion `draft` is not flagged: the minimum belongs to the clarify gate, and §brownfield-process makes sparse criteria valid.
- No placeholder or empty acceptance criteria
- Open questions consistent with status — the **spec body's** `## Open Questions` (`clarified` or later must have none); a scenario's have their own check below and never count here. Suggested fix on failure: run `/ductus:clarify`, whose recovery path (spec 014) reverts the status to `draft` and walks the questions, or `/ductus:amend` on a fresh question, which takes the back-edge itself.
- No implementation code blocks (function signatures, package paths, language-specific snippets) in the spec — those belong in plan.md. Format examples, directory structures, and user-facing commands are acceptable when they define behavioral contracts.

**How each runs.** `check-artifacts` (step 8) runs the family `spec-integrity` over the first three bullets: a spec with no `## Acceptance Criteria` section, one at `clarified` or later whose section holds no checkbox, a criterion that is empty or only a placeholder (`TBD`, `TODO`, `TBC`, `FIXME`, `placeholder`, `n/a`, an ellipsis, or text wholly in `<>` or `{}`), and a spec at `clarified` or later with an open question in its own body are each one Blocking finding. The fourth bullet is the host's judgment in step 15: an implementation code block is a Blocking finding. Under `ductus exec` step 15 does not run, and the run records that judgment unexamined under `integrity-not-checked`.

## Artifact completeness (blocking)

- If status is `planned` or later: plan.md exists
- If status is `planned` or later and feature introduces or modifies domain entities or data structures: data-model.md exists
- If status is `planned` or later: tasks.md exists

## Plan consistency (blocking if plan exists)

- Plan references the spec
- Technical decisions section has at least one decision with rationale
- Affected files section lists specific file paths
- Plan does not contradict `specs/system.md`

**How each runs.** `check-artifacts` (step 8) runs the family `plan-consistency` when `plan.md` exists, and a spec with no plan skips it: the plan carries no link to its spec's `spec.md` and names neither its feature directory nor its number (a plan written before the template carried the link names its feature in its title), its Technical Decisions section holds nothing outside comments, or its Affected Files section lists no file outside comments (no list item, table body row, code span or word holding a `/`, since the section is a planning aid the template calls non-exhaustive), each one Blocking finding. Whether a decision carries a rationale and whether the plan contradicts `system.md` are the host's judgments in step 15, each failure a Blocking finding, recorded unexamined under `integrity-not-checked` under `ductus exec`.

## Plan record (advisory)

Enforces `.ductus/constitution.md` §plan-phase's *a plan records the design as it stands* on `plan.md` at `planned` or later, from `prune-plan`'s preview (step 16), which segments the plan by `##` heading and compares each case-insensitively against the design record. The runtime compiles that heading set from `framework/templates/spec/plan.md`, so an adopter's customized plan template does not apply — the same bounded divergence `/ductus:prune`'s reset body has.

**Not covered:** a journal nested under a design heading — a `### Task 7 notes` inside Technical Decisions — is part of that section, and no heading test reaches it. The rule applies there all the same.

Suggested fix per finding: run `/ductus:prune`, which moves the section's durable pieces into the design record or their own homes and then removes it; or, when the section is deliberately part of this plan's design — a *Known limitations*, a *Classification* — fold it under a design-record heading, which ends the finding. A waiver only keeps the next analyze from deciding it again, so a prune still proposes the section.

**Severity:** advisory, and it stays advisory: the heading set is a detector, not a verdict, so a false positive is answered by folding the section, never a gate.

## Artifact size (advisory)

Warns when a spec artifact is larger than an agent may read in one call (§grounding, *A partial read is not a read*), from `check-artifact-size`'s result (step 17). Its warning and per-kind fixes are what the report renders; here a waiver with its reason is the further fix, and it reopens nothing.

The check also measures the four project files — `AGENTS.md`, and `system.md`, `events.md` and `errors.md` under the spec root — against the same threshold (spec 063, scenario `project-files-are-measured`). Each one over it is an advisory finding like a spec artifact, whose one fix is `/ductus:prune --project`, and a stored waiver in the targeted feature's `analysis.md` covers it by the same rule. An absent project file is reported in the result's `project-files` and is no finding; one that cannot be read is unexamined under `artifact-unreadable`.

Bytes are a proxy: the runtime has no tokenizer and the cap differs per host and per host setting, so the threshold is a heuristic the project sets, and no value switches the check off.

**A waiver holds until the file grows into another read page**, because the reason given for one page does not cover two. The primitive decides that numeric rule and returns the covering waiver's key.

The size check never escalates an unreadable subject: on a `done` spec, **Scenario open questions** and **Link-adjacent decision drift** already block on an unreadable `spec.md`, `plan.md`, `tasks.md` or scenario.

**Severity:** advisory, and it stays advisory: a false positive is answered with one stored waiver, never a gate.

## Task consistency (blocking if tasks exist)

- Tasks reference the plan
- Each task has a "done when" condition
- Tasks are numbered in strictly increasing order: digits and at most one lowercase letter before the dot (`11`, `11a`), ordered by digits then suffix (`11` < `11a` < `12`)

## Grounding (advisory)

Enforces `.ductus/constitution.md` §grounding against the spec and plan bodies; step 15 defines the passage to flag and where the check applies, and a passage framed as a proposal is not asserted as fact either. It checks the *form* of grounding, not its truth: the truth check is the agent's at authoring time and `/ductus:review`'s against code. A `draft` spec is exempt because its claims are still forming.

Suggested fix per finding: ground the claim (read the code, query the dev database, run the command — then cite the source), or, when no reachable source can settle it, restate it as an assumption (in a plan) or an Open Question (in a spec, which reverts the spec to `draft` per §spec-lifecycle).

**Severity:** advisory in v1 — grounding is a semantic judgment with false-positive risk, and a noisy gate erodes trust. **Promotion criterion:** promote to blocking when a single `/ductus:analyze --all` run reports 5 or more ungrounded claims across the repo on two consecutive runs.

## Scenario consistency (advisory)

- Every scenario file has Context and Behavior sections, which the host judges (its frontmatter is under Frontmatter schema above)
- Every scenario file in `scenarios/` has a task in `tasks.md` that names its slug, or had one in some revision of it, since `tasks.md` is ephemeral (§tasks-phase) and neither its content nor its numbering shows whether a task was pruned. `check-artifacts` flags a scenario, at any status, only when no revision ever named it, and its message says what the operator decides on a `done` spec. When the history cannot be read in full — no repository, or a shallow clone — it flags nothing and notes each unmapped scenario as unexamined, reason `no-readable-state`.
- A scenario task that is *still present* in `tasks.md` is marked complete when the spec status is `done`; an absent (pruned) scenario task is not treated as incomplete.

## Scenario open questions (blocking at `done`, advisory otherwise)

A scenario's unresolved questions are the spec's for the purpose of completeness; on a `done` spec that contradicts §spec-lifecycle.

- **No grandfather rule**, unlike Review state drift below: an unresolved scenario question is a present-tense defect whenever it arrived.
- Only `## Open Questions` counts. A question deferred pending a condition is resolved *with* that condition (§spec-requirements) and belongs in that scenario's `## Resolved Questions`. There is no skip marker on purpose: an exemptible section would let anything blocking be relabelled to ship past the check.

Suggested fix per finding, offering both exits: resolve each question via `/ductus:target {feature}/<scenario>` then `/ductus:clarify` (scenario-targeted), **or** — when the question is deferred rather than undecided — move it to the scenario's `## Resolved Questions` with the condition that will settle it.

When `--fix` is set, this check additionally reverts affected specs from `done` to `in-progress` — via `set-status` (`from: done`, `to: in-progress`) — and emits a one-line notice for each (`reverted: specs/{feature}/{file} from done to in-progress — unresolved scenario questions in {scenarios}`). The revert is never silent. Specs below `done` are not reverted: their finding is advisory, so there is no drifted state to correct.

## Link-adjacent decision drift (advisory)

Enforces `.ductus/constitution.md` §drift-prevention's *Decision resolution* obligation where it can be checked deterministically: prose in `spec.md`, `plan.md`, `tasks.md` or `scenarios/*.md` asserting an open state that its own sibling link's target contradicts. The grounding check cannot catch this: it verifies a claim is *cited*, not *true*. The finding's message names the line, every fired tell the target contradicts, the target and its contradicting state, with one finding per block and link. The tell list is closed and framework-fixed, with no per-project configuration, so the promotion criterion below measures drift rather than configuration.

**Not covered:** `review.md`, pinned to its `reviewed-against` sha so its prose is correct as written, and `data-model.md`; a link whose target *exists* but whose cited section no longer says what the citing prose claims; prose that merely reads as historical; and an implementation-state tell against a scenario target, since a scenario has no lifecycle status and a spent task pruned per §tasks-phase looks like an unimplemented one.

Suggested fix per finding: correct the prose to describe the target's current state, or — when the prose is right and the target is wrong — fix the target. Prose that only quotes a stale claim goes in a blockquote or an inline code span, which the scan exempts, as it does fenced code blocks and HTML comments.

## Acceptance-criterion path existence (advisory)

For each spec at `in-progress` or `done`, `check-artifacts` reports every filesystem path an entry under `## Acceptance Criteria` names that no longer resolves. A criterion is a **contract**, and nothing re-verifies a named path after a later spec deletes it, so the completion gate keeps reading the checked criterion as satisfied.

- **Scope is `## Acceptance Criteria` on `in-progress` and `done` specs, and nothing else.** Body prose may name a dead path correctly while describing history, and is **Body path existence**'s, below. The analysis the completion gate reads is taken at `in-progress`, so it runs this family; a criterion naming a file the spec has yet to create is a finding until the file lands. At any other status the family examines nothing and records one `status-gated` skip, its path the spec's `spec.md`, so the record says it did not run.
- **Paths are read inside inline code spans**, the inverse of the tell scan above: a span is a candidate when its whole content is a repo-relative path with an interior `/`, never a URL, a `path:line` citation, a slash command, a flag, a glob, a placeholder or an elided path. A candidate resolves as a file **or** a directory.
- **The criterion must be a live claim.** A path is checked only when its criterion actually claims the path is *present*. A criterion carrying any of fifteen closed phrases is exempted whole and its paths recorded as `not-a-live-claim`: deletion and retirement (`deleted`, `does not exist`, `no longer exist`, `is removed`, `are removed`, `since retired`), rename (`is renamed to`, `are renamed to`, `renamed from`, `(was` + space), migration subject (`target paths`, `target_paths`), adopter scope (`in the project`), and hedges (`if it exists`, `e.g.`).
- **A criterion asserting a path was never created is exempted the same way**, when one of its clauses carries a negator (`no`, `not`, `never`, `without`) **before** a creation verb (`created`, `added`, `introduced`). Both lists are closed and framework-fixed. `X was created and not modified since` is a delivery claim and still flags.
- **An unresolved candidate is a finding only when its own top-level segment exists.** When that segment is absent nothing is provable — a framework project's criteria legitimately name paths that live in an adopter's checkout — so the candidate is recorded as `root-absent` under Unexamined targets below.
- **A path this project ships into an adopter's checkout is recorded, not flagged**, as `ships-to-adopter`: a Shared Files manifest destination or a registered agent's scaffold path in `framework/bootstrap/ductus.md`, or a directory containing one. An absent or unparseable manifest yields an empty set, so nothing is suppressed that was not derived; in an adopter project the file is absent, and those paths resolve there anyway.
- **A path the repository's own ignore rules ignore is recorded, not flagged**, as `gitignored`: local state such as `.ductus/session.toml`, present in a working checkout and absent from a fresh clone, so the result does not depend on which tree it ran in. The rules are the ones `git check-ignore` reads; outside a repository none apply.
- **An unbackticked path is missed.** Accepted: outside code font a `/`-bearing token is more often not a path.

Suggested fix per finding: correct the criterion to name the current path, or — when the deletion means the criterion is no longer satisfied — uncheck it and route the work through `/ductus:amend`.

## Unexamined targets (informational)

The checks above read *targets* that may be unreadable: a link resolving to a file that no longer exists, a target that cannot be read or is reached through a symbolic link, a target carrying no state the tell's class can be evaluated against. None of these produces a finding — an unknown is never escalated into a defect — **with one exception**: `artifact-unreadable` on a spec at `status: done` is a **blocking** finding rather than a skipped target. `artifact-unreadable` names the spec's **own** artifact, which its own analysis could not read, as when a scenario carrying unresolved questions will not parse under the gate built to catch them. Below `done` it stays a skipped target: the spec is in flight.

They are not silent either. The `check-artifacts` result carries a `skipped` list of `{family, reason, path}` records. Render them in the **Informational** tier. `clean` means only no findings, so the assurance lives in the pair (`QUAL-CLAIM-001`): clean with nothing skipped is verified-clean, clean with something skipped is partially examined.

**The reasons form a closed set in two classes, and they call for opposite responses.** *Excluded by construction* — `not-a-live-claim`, `ships-to-adopter`, `root-absent`, `never-existed`, `gitignored`, `status-gated`, `anchor-target-unjudgeable` — are targets a check decided not to examine; they recur on every run and the exclusion is correct. *Could not be read* — `target-missing`, `target-unparseable`, `no-readable-state`, `artifact-unreadable`, `referrer-unreadable`, `anchor-target-unreadable`, `rule-citations-not-checked`, `rule-not-installed`, `rule-assessments-not-checked`, `rule-file-unreadable`, `references-not-checked`, `applicable-rules-not-checked`, `grounding-not-checked`, `integrity-not-checked` — are real gaps in what the run saw. Step 18 reads the split: only the second class sets `restricted`, so a stored decision is retained when its finding's source may have gone unseen, and pruned when the finding genuinely stopped firing. A reason added later is classified here, once.

Every reason from `referrer-unreadable` on in the second class comes from outside `check-artifacts`, as does `anchor-target-unjudgeable`, the one exclusion by construction that does. `artifact-unreadable` has a second source too: step 17 records it for a subject the size check could not measure.

- `referrer-unreadable` counts the referrers step 9 could not read.
- `anchor-target-unjudgeable` counts the `§` references step 4 could not judge by construction: each names only documents absent from this tree, or matched nothing on a line naming a file that is not markdown, which holds no section to judge against.
- `anchor-target-unreadable` counts the `§` references step 4 could not judge because a document its line names exists but could not be read, or is a registered shared constitution that did not resolve.
- `rule-citations-not-checked` counts the spec's rule citations when step 5 read no rule file (`examined: 0`): each is unexamined rather than missing, and a spec citing no rule records nothing.
- `rule-not-installed` counts the spec's citations of a rule the framework ships whose file the project never installed, as under a `[rules] surfaces` that leaves the surface out (`check-rule-ids`' `not-installed`): informational, never missing, and unexamined because no rule text was loaded to assess the spec against. An ID no shipped file defines stays missing.
- `rule-assessments-not-checked` counts the loaded rules steps 11 and 12 did not ask about — a rule with no Verification, or whose Statement carries no MUST or SHOULD keyword and so has no tier to be asked in. Under `ductus exec` it also counts a step once when the walk loaded no rule at all, or when the step's prose names no MUST or SHOULD tier: that step examined nothing, and the record says so.
- `rule-file-unreadable` counts the rule files steps 11 and 12 could not read, whose rules could not even be counted. A host that cannot read a rule file, or finds a rule it cannot place in either tier, records it under the same reason the walker would.
- `references-not-checked`, `applicable-rules-not-checked`, `grounding-not-checked` and `integrity-not-checked` are steps 13–15, which `ductus exec` no-ops as host prose, so an exec run records each once whether or not the step would have applied; step 15 is the reading of the spec and plan for both grounding and the three judged items of the Spec integrity and Plan consistency sections above, and so owns two reasons. A host that runs those steps never records them.

## Unresolved shared constitutions (informational)

A `[constitutions.*]` entry the run could not read is recorded in the record's `unexamined-by-reason` under `constitution-unresolved`, and counts toward `unexamined`: the analysis is running under fewer rules than the project declares. `write-analysis` derives this from the registry itself, so a caller cannot omit it, and a project with no entries registered records nothing.

## Promotion criterion for both decision-drift checks (shared)

**Severity:** advisory at introduction, matching the grounding and Applicable-Rules checks. **Promotion criterion:** promote to blocking when a single `/ductus:analyze --all` run reports 5 or more findings across the repo on two consecutive runs, **and** every finding in those runs is confirmed a true positive.

The second half is required *here* and not for the LLM-judged checks: these two are mechanical matches — a word list and a path grammar — so a noisy implementation reaches the volume as reliably as an accurate one.

## Body path existence (advisory)

For each spec at `in-progress` or `done`, `check-artifacts` reports every filesystem path named in its body, `plan.md`, `data-model.md` or scenarios that existed in this project's history and no longer resolves.

- **Scope is everything the criterion family does not read**, with fenced blocks and HTML comments skipped. A file a spec in flight has yet to create never existed, so it is recorded `never-existed`, not flagged. At any other status the family records one `status-gated` skip, as the criterion family does.
- **Paths are read as in acceptance-criterion path existence**, with a `#fragment` stripped first. A candidate starting `../` resolves from the citing file's directory; any other resolves from the project root. The `root-absent` and adopter-destination rules apply as they do there.
- **The line must be a live claim.** Body prose states transitions in forms criteria do not use, so a line is exempted — its paths recorded as `not-a-live-claim` — when it carries any of the fifteen criterion phrases above, a negated-creation clause, or any of seven body phrases (`delete`, `remove`, `retire`, `replaced`, `superseded`, `rename to`, `` ` → ` ``). Matched over the whole logical line, so an Affected Files row whose action is `Delete` exempts itself. Bare `→` is not a phrase, only the arrow between two code spans.
- **History decides.** A path no commit reachable from HEAD ever held is a name, not a file, and is recorded as `never-existed`. When the history cannot be read in full — no repository, a shallow clone — record each such path as `no-readable-state` and flag nothing.

Suggested fix per finding: edit the line to name the file that carries the content now, or to say what is true now; a spec body is a living document (§spec-lifecycle).

## Acceptance-criterion labels (advisory)

The enforcement half of the labelling pass (§spec-requirements), because a criterion typed by hand in an editor never touches a primitive. A spec with no `next-criterion` has never been through the pass, which §text-first-artifacts defines as no labels assigned yet, so its unlabelled criteria are not findings. Applies at every status: a label is an identifier, not a contract about the delivered system. Its subject is the spec's own frontmatter and criteria list, so it never contributes to Unexamined targets above.

Suggested fix per finding: for an unlabelled criterion or a lowered counter, run the labelling pass (`label-criteria`). A duplicate label and a corrupted counter are author decisions the pass deliberately does not make: renumber the later duplicate to a fresh label above the counter, and reconstruct the counter from the highest label ever issued rather than the highest still present.

**Severity:** advisory. The pre-commit hook runs the pass on every commit, so the two assignable states are resolved by the next commit rather than owed to a gate; the two that are not assignable are rare hand-edit defects.

## Cross-spec references (advisory)

- Event types mentioned in spec or plan align with `specs/events.md`
- Error codes follow the convention from `specs/errors.md`
- Data model definitions do not conflict with other specs' data-model.md files

## Cross-service references (advisory)

`resolve-references` classifies each `{service, spec}` entry of a spec's derived `references:` index (step 13). The repo URL is identity and navigation only and is **never fetched**; status is read from the local checkout. Map each outcome, and each repo in the result's `duplicate-repos`, to a severity:

- **`broken`** (Advisory finding) — the service is registered and its checkout reachable, but the target `spec.md` does not resolve. A provable defect in *this* spec, non-blocking because references are informative and never load-bearing. Suggested fix: correct or remove the reference link in the spec body, then re-run `derive-references`.
- **`unregistered`** (informational, not a finding) — the repo matches no `[services]` entry, so it is a plain navigational link. Surface it with a pointer to `/ductus:link` to register the service.
- **`not-checked-out`** (informational, not a finding) — registered, but the local `path` is missing or not a usable checkout, so nothing can be proven.
- **`status-unreadable`** (informational, not a finding) — the target file exists but its `status` cannot be read, or the link targets a scenario, which has no status. The defect is upstream's.
- **`ok`** (no finding) — the reference resolves and the linked `status` is readable.
- **`duplicate-repos`** (Advisory finding, one per repo) — the registry holds the repo under two or more aliases, compared with a trailing `/` and `.git` removed, so a link to it resolves through one of them silently. Suggested fix: keep one `[services.<alias>]` table for the repo.

The load-bearing line is **provably broken** (a finding) versus **can't check** (an informational unknown): a broken link never hides behind a benign unknown, and an unknown is never escalated to a defect.

## Review state drift (blocking)

For each spec at `status: done`, `check-artifacts` reads the review record from its `review.md` frontmatter and reports an unset `last-run`, `blocking: true`, a non-zero `should-violations`, a broken contract left in `open-findings`, or the `fix-rounds-open` mark of a review cut off, each message saying what to do. A `review.md` carrying no parseable record is the hard fail under Frontmatter schema above, never a never-run; the two must never render alike. §implement-phase gives a SHOULD three exits — fixed, waived with its rationale, or deferred where the deferral releases it (**§Deferrals** in `.ductus/reference/review.md`) — and each drops the count, so a non-zero count at `done` means none happened.

A `done` spec holding an unticked criterion, or an unchecked box in `tasks.md` under any heading, has drifted too, whatever its record says, since `done` says each is verified and the tasks complete (§spec-lifecycle).

**Grandfather rule.** A `done` spec with **no `review.md` at all** predates `/ductus:review`, which writes the file on its first run (§text-first-artifacts), and is exempt from the record's conditions. Adopters who want retroactive review run `/ductus:review` against the spec, after which those conditions apply. The exemption is keyed on the artifact, never a block in `spec.md`, which would exempt every spec in a migrated corpus.

Specs not at `status: done` are silently exempt, since `review.md` is written lazily.

**`--fix` reverts; it never edits the count.** The count is the review's to write, and rewriting it here would erase the finding rather than resolve it. When `--fix` is set, this check additionally reverts affected specs from `done` to `in-progress` — via `set-status` (`from: done`, `to: in-progress`) — and emits a one-line notice for each (`reverted: specs/{feature}/{file} from done to in-progress — re-run /ductus:review`). The revert is never silent. Re-running `/ductus:review` on each is left to the operator. The grandfather rule applies under `--fix` too: a `done` spec with no `review.md` is not reverted over its record, though an unticked criterion or an unchecked box, which have no grandfather rule, still reverts it.

## Analyze state drift (blocking)

For each spec at `status: done`, judge the `analysis.md` this run just wrote, after step 21 — never the record it is replacing. If that record is `blocking: true`, report `analyze drift: done spec's analysis.md records {hard-fail} hard-fail and {blocking-findings} blocking finding(s) — resolve them and re-run /ductus:analyze` (**blocking**).

- **Why the new record.** A finding read from the record about to be replaced could never clear: it is blocking, so it cannot be waived, and no fix removes it before the rewrite. Judged from the new record, a run that fixes every hard-fail and blocking finding leaves no drift.
- **The report names a state, not a finding**, and is not counted in the record, which cannot count its own verdict.
- **Advisory findings are not checked**, unlike Review state drift's outstanding SHOULD: some of analyze's advisory checks carry published promotion criteria and the rest stay advisory for good, so gating on them here would promote every one at once. An undispositioned advisory finding surfaces as Disposition drift instead.
- **No record, no drift.** The record this run writes always carries `last-run`. The `done` specs still without an `analysis.md` are bounded by the CI template's record check and counted by `/ductus:audit` Family 37.
- **`--fix` does not revert on this check.** It is reported on every run until a fix clears it or a deferral onto the spec reopens it (step 19).

## Disposition drift (advisory)

For each spec at `status: done`, judge the `dispositions:` map of each record against the record that will stand after this run:

- **`review.md`**, during detection, by `check-artifacts`. This command does not rewrite that record, so only `/ductus:review` can clear it.
- **`analysis.md`**, after step 21, from the record this run just wrote — never during detection, for the reason Analyze state drift gives. A run that leaves a finding undecided reports `disposition drift: done spec's analysis.md records {n} undispositioned finding(s)`; the report is not counted in the record it judges.

A `done` spec whose record still counts an undecided finding is not a contradiction: only a gating finding holds `done` (§brownfield-inbox, Finding dispositions), and a gating one already surfaces as Review state drift or Analyze state drift. For any other undecided observation or advisory finding this is the only report, so a later run can decide it.

- **A record with no `dispositions:` map is not drift.** It predates dispositions, and a backfilled map would assert dispositions nobody made. The pre-done gate blocks the same absence on an `in-progress` spec; on a `done` spec it waits for the command's next run.

**`--fix` does not revert on this check**, and never edits the map: the count is the recording command's to write.

## Rules (blocking and advisory)

Rules are the cross-cutting tier of the framework's three-tier requirement model (see §rules in `.ductus/constitution.md`). Invoke `discover-rule-files` with `all`, `rules` and `verification`: it lists every `*.md` file in the project's rule-file directory, classifies each by basename suffix per the closed-suffix policy there, and returns each file's rules, with each rule's ID, heading line, tier, Statement and Verification, and each heading that opens no rule, with its line and its `body`, the lines under it up to the next heading, under `other-headings`, in its `rule-files`, or past the read size in the file `lists-file` names. Read that file with the host's file reader, in parts by line offset when it is long. Neither it nor a rule file is read through the shell. `/ductus:analyze` loads **every** discovered file, shipped or project-authored, regardless of detected stack and of `[rules] surfaces` (`ductus.md`, **Project Configuration**), because citation verification spans surfaces: a backend project that cites `FE-XSS-001` in a scenario covering HTML output still needs that citation verified. `[rules] surfaces` scopes `/ductus:review` enforcement only. `/ductus` installs only the surfaces' files, so a citation of a rule whose file is absent resolves from the runtime's index of the shipped rule IDs: `check-rule-ids` reports it `not-installed`, known and not missing, and step 5 records it unexamined under `rule-not-installed`, since the rule's text was not available to judge it against.

Emit the notice lines it returns verbatim. For each file with an unrecognized suffix, that is one stdout line:

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

An empty `loading rule files:` line renders identically whether the rule-file directory held nothing, does not exist, or could not be read (§design-principles in `.ductus/constitution.md`, `QUAL-CLAIM-001`). The finding is **advisory, never blocking**: a project with no rule files is a legitimate state, and the rest of the pass still runs.

For each loaded rule file, checked from the rules the result lists and the headings it lists under `other-headings`:

- Every rule heading is level-3 and contains only the rule ID (no surrounding text)
- No heading in `other-headings` was meant as a rule's. The parser opens a rule only at a heading the rule-ID grammar matches, so a malformed rule heading, such as `### be-authn-001`, `### BE_AUTHN_001`, a rule at level 5, or a rule's Statement under a prose heading, is listed there and nowhere else. Judge each heading by its `body`, and report one that heads a rule as a malformed heading; no rule file is read, through the shell or git, to judge one
- Every rule has the two required fields: a block-quoted Statement and a `**Verification:**` paragraph. A `**Rationale:**` paragraph is ignored, never malformed
- Every rule's ID matches the ID format that governs that file, resolved in the order below
- No two rules in the same file share an ID

**Which ID format governs a file.** Take the first source that applies:

1. **The file's introducing-spec data-model**, when the file has one — `{BE|FE}-{CATEGORY}-{NNN}` for the backend and frontend security files (`specs/008-security-rules/data-model.md`), `CFG-{CONST|ENV|FLAG}-{NNN}` for configuration (`specs/017-derive-dont-ask/data-model.md`), `SEC-{LOG|DEPS|AUTHN|DATA}-{NNN}` for `security-cross.md` (`specs/077-security-cross-rules/data-model.md`), `QUAL-{STUB|GROUND|CLAIM|DELEG|TEST}-{NNN}` for `quality-cross.md` (`specs/036-quality-cross-rules/data-model.md`).
2. **The format the file declares in its own header**, where a rule file's ID prefix and category abbreviations are canonically declared (`.ductus/constitution.md` §rules) — e.g. `api-backend.md`, which has no introducing-spec data-model, declares `BE-{CATEGORY}-{NNN}` and lists its category abbreviations in that header.
3. **The generic rule-ID grammar** — an uppercase prefix, an `[A-Z][A-Z0-9]*` category abbreviation, and a zero-padded sequence number, as declared in `specs/008-security-rules/data-model.md` §Rule ID format. This is the shape `check-rule-ids` resolves citations against, so an ID that fails it cannot be cited from a spec regardless of what any header says.

If any check above fails, the affected rule file is treated as unloadable for the remainder of this analyze pass — no rule from it is applied to the per-rule assessments (steps 11 and 12) — and the failure is reported as a **Blocking** finding, one of the following, since a file whose rules drop out without a finding makes a malformed rule set indistinguishable from a clean one:

- `Malformed rule file {path} at {location}: {reason}` — a missing required field, an ID-format violation, a malformed heading, or a file that does not parse, among them each file the result names under `unreadable`
- `Duplicate rule ID {ID} in {file}; refusing to load` — two rules in the same file share an ID

A citation step 5 resolves is reported in the same way, naming the citing artifact:

- `Spec at {path} references unknown rule {ID}` — **Blocking**: no loaded rule file defines the cited ID
- `Spec at {path} references deprecated rule {ID}; targeted for removal in {version}` — **Advisory**: the cited rule is marked `DEPRECATED`, and the citation still satisfies it for the deprecation window

**Under `ductus exec` neither rule-file message is produced.** The walker checks no rule-file integrity: a rule it cannot assess and a file it cannot read are recorded unexamined (**Unexamined targets** above), and a duplicate ID goes unnoticed, each such rule assessed as though whole. The reduction is stated rather than silent.

A rule file is **never** unloadable merely for having no introducing spec, or for declaring no format in its header — those conditions select which source governs. A project-authored rule file has no introducing spec by construction and is validated on the same terms as a shipped one.

### Applicable Rules citation consistency (advisory)

The rule-citation audit runs in both directions:

- **Rule fires; not cited.** For every loaded rule whose Verification trigger fires against the target spec, the per-rule assessment (steps 11 and 12) emits a finding when the spec does not address the rule.
- **Cited; rule does not fire.** Step 14 emits an advisory finding for a rule ID under the spec's optional `## Applicable Rules` section that is not in the fired set. The author either removes a decorative citation or extends the spec to bring the cited surface into scope.

**A citation of a code-pattern rule is exempt from the second direction.** A rule file may declare that its rules verify code patterns rather than design-time commitments, as `quality-cross.md` does. Each such rule's Verification is `/ductus:review`'s quality pass over code in scope, so it has no trigger against a spec's artifacts and would fire this finding on every citation, however load-bearing. The exemption opens no gap: `/ductus:review` enforces the rule whatever the spec cites, so citing one records which rule a design applies and is never decorative.

**Severity:** advisory in v1. **Promotion criterion:** promote to blocking when a single `/ductus:analyze --all` run reports 5 or more stale citations across the repo on two consecutive runs; the second run guards against citations landing before the AC that exercises them. Until then, forward-looking citations remain a usable planning signal.

## Project-level consistency (advisory)

These checks span the project's installed command set and constitution rather than the target feature (`.ductus/constitution.md` §drift-prevention). Run once per `/ductus:analyze` invocation regardless of which feature is targeted; with `--all`, run once before per-feature output. They do not prevent pipeline advancement on the target feature.

Read inputs:

- `.ductus/constitution.md` (already loaded by `/ductus:target`)
- The installed command set (frontmatter only — do not read bodies for these checks): the files the agent layout's **Command/skill path** names, with membership drawn by that layout's **Slash-command cleanup glob** — the project's commands, never every file in the directory, since a layout's directory can be shared with the adopter's own files. The help command is that set's `help` entry.
- The bootstrap installer at the layout's **`ductus` install path**, if it exists (frontmatter only — it lives outside the project namespace)

These paths are layout-derived and are not spelled here: the bootstrap's [§Derived values](https://github.com/stonean/ductus/blob/main/framework/bootstrap/ductus.md#derived-values) table carries each of them per layout, and the layout is the one declared by the §Agent Registry row whose `config_dir` is the `cli-config-dir` that `/ductus` records for this contributor in `.ductus/session.toml`. When the layout cannot be determined — no `cli-config-dir` is recorded, or no registry row declares the one that is — each check that reads these paths is reported as unexamined, naming why, and is never run against a guessed layout. A derived path that does not exist is reported as unexamined, naming the path looked for — `Unexamined: {path} does not exist` — and never as a check that passed.

Checks:

- **Generator drift** — run `scripts/gen-help-tables.sh --dry-run` (through the runtime's CLI, `.ductus/bin/ductus run-generator --script scripts/gen-help-tables.sh`, since `run-generator` is not an MCP tool; when the script exists in the project). Non-empty diff means the help.md command tables are out of sync with their sources. Report it as `Generator out of sync: {script}; the next commit will resolve.`
- **Anchor resolution** — every `§<name>` reference in any installed command file (typically in `Reference: §<first>, §<second>` Scope-Boundaries lines) resolves, by `resolve-anchor`'s rule, to a corresponding marker in `.ductus/constitution.md` or a section of the document its line names.
<!-- audit:ignore-placeholders -->
- **Command frontmatter completeness** — every file in the installed command set has a `description:` frontmatter field; the same check applies to the bootstrap installer when that file exists. Files whose body documents an `$ARGUMENTS` parameter additionally have `argument-hint:`, since every layout's scaffolding copies the source frontmatter verbatim (the bootstrap, `ductus.md`, in §Per-Agent Scaffolding). Report missing fields; do not check value content.
- **Orphaned references** — every reference an adopter-owned file makes into a ductus-managed path resolves (`check-orphaned-references`; step 9). The managed roots are `.ductus/`, `.githooks/`, the configured spec root, and the historical `.govern/`, `scripts/gen-` and `scripts/lib/`. Report `Orphaned reference: {referrer}:{line} names {target}, which does not exist` — plus `; most likely orphaned by migration {id}` **only** when the result's `attribution` is `registry`. Under `watermark` attribution say so instead, naming `last-applied` as the only migration context available; the two must not render alike. A migration re-points only the references the framework wrote, and a fold or a consolidation re-points those naming the directory it retires (`rewrite-spec-links`); an adopter-authored one is reported and left to the adopter. Not references, so never reported: a candidate containing `*` or `NNN`, a git object name (`git show abc1234:specs/x/spec.md`), and a path the project ships to adopters (`ships-to-adopter` above).

- **Un-folded branch-scoped specs** — every branch-scoped directory (`{identifier}.{n}-{slug}`) surviving under the spec root is reported (`check-unfolded-specs`; step 10). Report `Un-folded branch-scoped spec: {feature} folds into {target}, status {status}`, appending `; {target} does not resolve in this tree` when it names no directory here. That suffix is a **report**, not a defect: before the merge the target normally lives on the upstream branch, and this check cannot tell which tree it is looking at. Advisory in every case, since the check equally cannot tell an open branch from a merged one.

## Cross-spec impact drift (advisory)

A `done` spec declaring a `cross-spec-impact:` entry no back-link discharges: the gate reads the declaration only on the way to `done`, and an entry added afterwards is a correction, so nothing else would (`check-artifacts`; §cross-spec-impact).

## Severity tiers

- **Hard fail (blocking)** — required-field violations and malformed frontmatter. The spec is not valid until these are fixed; pipeline advancement is blocked.
- **Blocking** — structural or content issues that must be fixed before the next pipeline gate fires.
- **Advisory** — issues that should be fixed but do not block advancement.
- **Informational** — observations that may warrant attention but are neither errors nor warnings.

## Finding dispositions

Every finding still live when detection ends — hard fail, blocking, and advisory alike — takes at most one disposition before the run is recorded (§brownfield-inbox, Finding dispositions), by the rule step 19 applies. A person's stored decision wins, and the rule does not re-decide it. Nothing is written to `{specs-root}/inbox.md`, the one destination no gate reads. The **Informational** tier is not findings: it states what could not be examined, not a defect to decide.

- **The order is detect → decide → re-check → record → render.** Detection (steps 1–17) writes nothing but the lists files under `.ductus/results/` a primitive's result names past the read size. Stored decisions apply first (step 18), so nothing already decided is decided again. The rule disposes of the rest, and each gating write is confirmed first (step 19). When anything was written, detection runs again (step 20), and `analysis.md` is written from the re-check (step 21), so a run never makes its own record stale. The report renders last (step 22); a run interrupted between recording and reporting leaves the record.
- **Fixed** — a chore: a mechanical edit to an artifact the run audited, adding no durable requirement. It is recorded as `live: false` under `analysis.md`'s `## Fixed in this run`, as is any other write that removes the finding from the re-check, such as a body edit on the spec in hand: the record reads the re-check, not the kind of write, and `write-analysis` refuses any outcome but `fixed` for a finding the re-check no longer produces. A chore is never counted fixed while its fix is not on disk.
- **Deferred** — to the home **§Groom decision tree** in `.ductus/reference/groom.md` chooses, its single canonical statement, or a task on this spec where the finding needs work and no new requirement, which releases nothing, so the finding still holds `done`; an external work-item ID may stand beside that home, or in its place, as the operator gives it. A new spec is created through `/ductus:specify`'s procedure in the same run, and a declined creation leaves the finding waived or undispositioned — never deferred to a spec that does not exist. A deferral to a `done` spec reopens it with `from: done`, so a status that changed since the proposal surfaces rather than being overwritten; a deferral to a spec that is not `done` follows `/ductus:amend`'s rules for that status. A rule file `ductus` manages and the project has not pinned is overwritten on the next update, so an advisory finding against it is waived with that reason, and a gating one, which `write-analysis` refuses to waive, is deferred to a task on this spec. A finding is recorded `deferred` while it still fires because its deferred work has not landed, which is the state a stored decision exists for; a confirmed write that removes it from the re-check is `fixed`. A `hard-fail` or `blocking` finding its deferral releases (**§Deferrals** in `.ductus/reference/review.md`) leaves the record's gating counts and renders under Deferred to other specs; any other deferred one keeps `blocking: true`, and its stored decision carries the derived `gating: true` that `check-review-gate` names. A stored deferral whose named work is complete while the finding still fires is **lapsed**: `process-decisions` reports it, it is dropped, and the finding takes a new disposition.
- **Waived** — an advisory finding, recorded in `analysis.md` with its reason and stored with `decided-by: rule`, or with the person's name when one made the call. **Never a `hard-fail` or `blocking` finding**: those already gate `done`, and a waiver would add a bypass the gate does not have. Such a finding is fixed or deferred, and only a deferral to another spec's work releases it.
- **Undispositioned** — no decision was made: the operator declined a deferral, the deferral's write failed, its stored deferral lapsed with nobody to ask, or a gating finding had nobody to ask. The record counts it, and a later run can decide it. It holds `done` only when it is hard-fail or blocking, through the record's `blocking: true`. Under `ductus exec` that is every live finding: steps 18–20 are host responsibility, which the subprocess walker no-ops, so an exec run matches nothing against the stored decisions (and prunes none), writes no fix and no deferral, and still records honestly.
- **Decisions persist.** Detection is stateless, so a decided finding fires again on the next run — a deferred one until its deferred work lands. Each deferred or waived decision is stored in `analysis.md`'s `decisions:` list, keyed `{family} — {message}`. A later run matches each finding to the stored decision describing the same issue — by the host's judgment, because a host-worded message drifts — and passes that decision's key as the finding's `decision-key`; a matched finding counts under the stored outcome without asking. A missed match costs one repeated decision and prunes the old one; it never waives a finding, because an unmatched decision is pruned rather than applied to something else. A stored decision whose finding no longer fires is pruned, unless the run could not read a target or a registered constitution, in which case it is retained; a target excluded by construction does not count (**Unexamined targets** above).
- **The message is part of the key** because a finding's `path` is the **citing** artifact, not its subject, which appears only inside the message; a key without it would merge every finding one artifact produces. The key omits a category, a host's judgment that does not reproduce across runs.
- **A `decisions:` list that does not parse is reported, never read as empty.** `validate-frontmatter` names it as a hard fail, and `process-decisions` and `write-analysis` refuse it. Until it is repaired no stored decision applies, each finding it held takes a new disposition, and the run cannot record over it. A malformed list can cost a repeated question, never a silently waived finding.

## Record fields

The analyze record's fields and types are declared in the constitution's §text-first-artifacts; what each means:

- `analyzed-against` — HEAD sha at the time of the run, provenance only: the working tree and a commit are the same subject only on a clean tree, so it is never the staleness basis.
- `analyzed-digest` — per-path digest of the analyze subjects as the run read them from disk, excluding `analysis.md`'s own record, since a record written after its subjects are read can never digest itself. Absent is undeterminable.
- `analyzed-unreadable` — subjects that exist but could not be read, recorded rather than digested as empty.
- `hard-fail`, `blocking-findings`, `advisory` — the tier counts, the first two outstanding ones: the re-check's count less the live findings their deferral releases, listed under Deferred to other specs. `blocking-findings` is named for the tier to keep it distinct from the derived `blocking` flag, and `advisory` is never gated on.
- `unexamined`, `unexamined-by-reason` — targets the run could not examine, the field that makes a clean run honest (**Unexamined targets** above).
- `dispositions` — what the run did with its findings, in every tier. `undispositioned` is the live tier total less the live findings deferred or waived, so a finding the run did not itemize counts as undispositioned. Absent is the pre-disposition state.
- `decisions` — stored deferred and waived finding decisions, each with its key, its outcome, a deferral's `target`, `task` and `work-item` or a waiver's reason, `decided-at`, and `decided-by`, in the review record's shape (**Finding dispositions** above).
