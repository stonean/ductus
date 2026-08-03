---
status: done
dependencies: [013-text-first-artifacts, 022-deterministic-runtime, 046-scenario-open-question-visibility]
review:
  last-run: 2026-08-03T15:03:53Z
  reviewed-against: 1eda6f6f626eb368473b1dcae957392ba0e210d0
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

### Second observed case — a stale contract

Repo `govern`, spec `026-framework-self-audit`, 2026-08-02, found while running that spec's completion gate. Two acceptance criteria were checked `[x]` and had been true when 026 reached `done` in May:

| 026 criterion | Actual state |
| --- | --- |
| AC5: "Registry equivalence verifies every entry in `framework/workflows/registry.json`…" | `framework/workflows/` and `scripts/audit/registry-equivalence.sh` deleted by `531e3ea` |
| AC2: "The nine check families listed in Behavior are implemented" | Eight; Family 3 was retired with the workflows feature |

Spec 043 sunset the workflows feature without unchecking 026's criteria or updating its Behavior section, which still documented `#### 3. Registry equivalence`.

This is a sharper failure than the first case. There, stale *prose* misled a reader who could still discount it. Here a stale **contract** was treated as satisfied: `/{project}:implement`'s completion gate would have passed 026 at 14/14 had the criteria not been read against the filesystem by hand. Nothing re-verifies a `done` spec's criteria after a later spec deletes their subject.

It also has a cleaner tell. A criterion naming a path that no longer resolves is mechanically checkable with no semantic judgment at all — which is why it justifies its own check rather than an extension of the first.

## Scope

Two checks, each narrow by construction. Neither attempts to prove an arbitrary assertion is currently true — that is unbounded.

**The link-adjacent check** targets the high-signal case where an artifact's *own link* resolves to a state that contradicts the prose around it. Against the observed case, that catches the first three rows. It does not catch the fourth: "See the note in `tasks.md`" is a valid link to a section that no longer says what the citing prose claims, and verifying it needs a fragment anchor or semantic reading.

**The path-existence check** targets a narrower and cleaner case: a `done` spec's acceptance criterion naming a filesystem path that no longer resolves. It was added during clarification after a second observed case (026's AC5, below) that the link-adjacent check provably misses, and it shares none of that check's machinery.

Partial mechanical coverage plus an explicit obligation in the constitution is the deliverable; complete coverage is not achievable deterministically.

Two classes are explicitly **out of scope**, recorded so they are not mistaken for oversights. Prose that reads as historical without a structural marker is not deterministically detectable — the `## Motivation` staleness case is real, is not addressed here, and is tracked separately as an authoring convention. And a link whose target *exists* but whose cited section no longer says what the citing prose claims (the observed case's fourth row) needs semantic reading.

## Behavior

### The decision trigger (constitution)

§drift-prevention states that resolving a decision carries the same audit obligation as editing a document: every artifact that described the prior state is corrected in the same change, and a resolution is not complete while a sibling artifact still describes the question as open. The trigger list names the recognizable events — closing an open question, shipping a scenario, advancing a status, adopting a previously-rejected option.

### The link-adjacent check

For each artifact in a feature directory, for each inline link to a sibling artifact in the same feature:

1. Read the target's current state — its frontmatter `status` where it has one, and its open-question count. A scenario has no status, so it is evaluated on its open-question count and its file existence alone.
2. Scan the **enclosing block-level element** containing the link — the list item, table row, or paragraph — for **open-state tells**.
3. Emit a finding when a tell co-occurs with a target state that contradicts it: prose says "open question" while the target reports zero; prose says "does not exist yet" while the target is `in-progress` or `done`.

The tell list is closed, framework-fixed, and not configurable: `open question` (and `open questions`), `unresolved`, `still open`, `not yet`, `does not exist`, `left unimplemented`.

A tell is **not** counted inside a fenced code block, an HTML comment, a blockquote, or an inline code span. The first two fall out of the existing `SkipScanner`; the last two are added, and both are one-character structural tests that preserve determinism.

Evaluation is **per link**, so a block carrying several links is scanned once for each and fires only for the target whose state actually contradicts.

### The path-existence check

For each `done` spec, for each entry under `## Acceptance Criteria`: extract every filesystem path the criterion names — including paths inside inline code spans, which is where they conventionally appear — and emit a finding for each that no longer resolves.

Scoped to acceptance criteria rather than whole spec bodies because an AC is a **contract**, not narrative: naming a path asserts it is part of the delivered system. Body prose may name a dead path correctly while describing history, so widening this check would flag true statements.

This check reads inside inline code spans, the inverse of the link-adjacent check's rule. That inversion is why they are two families rather than one.

### Common properties

Both checks are **advisory** (non-blocking) at introduction, matching the existing grounding and Applicable-Rules checks, and share one documented promotion criterion.

Both ship as `check-artifacts` families on the runtime path, with the markdown-only path performing the same procedure as prose per §runtime-host-integration. This extends the deterministic check families introduced by [022-deterministic-runtime](../022-deterministic-runtime/spec.md); severity tiers follow [013-text-first-artifacts](../013-text-first-artifacts/spec.md) and introduce no new policy.

## Acceptance Criteria

- [x] §drift-prevention names the decision trigger, not only the document-edit trigger, and lists the recognizable resolution events
- [x] §drift-prevention states that a resolution is incomplete while a sibling artifact still describes the prior state
- [x] `/{project}:analyze` emits an advisory finding when an artifact's link-adjacent prose asserts an open state that the link target's current state contradicts
- [x] Each finding names the citing file and line, the link target, and the target's contradicting state
- [x] A feature directory whose prose matches its link targets' state produces zero findings
- [x] The link-adjacent check covers every artifact in a feature directory that carries inline sibling links — `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md`
- [x] Findings from both checks are advisory (non-blocking) at introduction and do not gate `done`
- [x] Repeat runs over an unchanged feature directory produce identical findings
- [x] A link whose target state cannot be read produces no finding (an unknown is never escalated to a defect, matching the `status-unreadable` precedent in cross-service reference classification)
- [x] `analyze.md` documents both checks and their shared promotion criterion alongside the existing advisory checks
- [x] The tell list is exactly the six closed entries, framework-fixed, with no per-project configuration surface
- [x] The scanned unit is the enclosing block-level element (list item, table row, or paragraph), and evaluation is per link so a multi-link block fires only for the contradicting target
- [x] A tell inside a fenced code block, an HTML comment, a blockquote, or an inline code span produces no finding
- [x] A link targeting a scenario is evaluated on the scenario's open-question count and file existence; a tell requiring a lifecycle status produces no finding
- [x] Both checks ship as `check-artifacts` families, with the markdown-only path performing the same procedure as prose
- [x] `/{project}:analyze` emits an advisory finding for each filesystem path named in a `done` spec's acceptance criterion that no longer resolves, reading inside inline code spans
- [x] The path-existence check is scoped to `## Acceptance Criteria` and does not scan body prose, so a correct historical mention of a deleted path produces no finding
- [x] The path-existence check reproduces the originating case: 026's AC5 naming `framework/workflows/registry.json` and `scripts/audit/registry-equivalence.sh` after `531e3ea` deleted both

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Where does the rule live — a constitution amendment, or a new artifact-tier rule file?** **Resolved: amend §drift-prevention; mint no rule file and no rule ID.** The decisive precedent is §grounding, which is already split this way: the *principle* lives in the constitution, the *artifact-side* enforcement is a deterministic `/{project}:analyze` step, and only the *code-side* enforcement is a rule (`QUAL-GROUND-001` in `quality-cross.md`). 045 is artifact-only, so it needs the first two layers and not the third. Three supporting reasons: §drift-prevention already owns the adjacent obligation ("Changing A includes auditing every back-link to A"), so the decision trigger is an extension of an existing principle rather than a new surface; rule files are RFC 2119 Statement/Rationale/Verification units enforced per-rule, whereas this spec's Behavior is a mechanical scan emitting findings, which needs no citable ID; and a new rule *file* is a new surface warranting its own spec per §rules, which would make it the first rule file `/{project}:review` never reads. `quality-cross.md` was correctly ruled out by the question itself — its header scopes it to code patterns. This resolution makes the spec internally consistent rather than reshaping it: ACs 1–2 already presuppose the constitution amendment while ACs 3–10 describe the deterministic check.

- **What is the closed list of open-state tells, and is it framework-fixed or configurable?** **Resolved: six tells, framework-fixed, not configurable in v1.** The list is `open question` (and `open questions`), `unresolved`, `still open`, `not yet`, `does not exist`, `left unimplemented` — the seed list minus two. `TBD` is dropped because it marks the *citing* document's own incompleteness rather than asserting anything about the link target, so under the co-occurrence design it can never produce a true positive, only noise. `deferred` is dropped because it contradicts a convention 046 established: a deferral is a *resolution with a condition* belonging in `## Resolved Questions`, so flagging it would penalise authors for following the framework's own guidance. Fixed rather than configurable for three reasons: the promotion criterion counts findings across a repo, and a per-project list would make the threshold measure configuration rather than drift; a list requiring adopter curation degrades silently for exactly the projects least likely to notice (the derive-don't-ask principle (`017-derive-dont-ask`)); and adding configurability later with evidence is additive, whereas removing it is breaking. Acknowledged counter-argument: 026's plan already concedes Family 6's hardcoded SSOT list is "a the derive-don't-ask principle (`017-derive-dont-ask`) violation in miniature", which is the same shape. Accepted here because the check is advisory, so a false positive costs a glance rather than a blocked gate. *Trigger to revisit:* a project reports domain prose that trips a tell repeatedly, at which point the additive per-project extension is the first option to reconsider.

- **What is the prose unit scanned for a tell?** **Resolved: the enclosing block-level element** — the list item, table row, or paragraph containing the link. Determinism decides it: markdown block boundaries are structural and already parsed by existing helpers (`SkipScanner`, the section walkers, the shared bullet grammar), whereas the repo has no sentence splitter and writing one that survives version strings, `e.g.`/`i.e.`, and period-bearing code spans is its own project. The unit also matches how these artifacts are written — every claim in the observed case is a table row or bullet, and govern already treats the `-` bullet as one atomic claim in open questions, acceptance criteria, and inbox items. Family 8 (`introducing-drift.sh`) is the one existing check that splits sentences, and its own done-when concedes "False positives expected" — a precedent to design away from rather than follow. Accepted cost: a block is coarser than a sentence, so a long paragraph can pair a tell with an unrelated link. Two things bound it — the check is evaluated **per link**, so a paragraph with three links is scanned three times and fires only for the target whose state actually contradicts, and advisory severity makes the residue a glance rather than a blocked gate.

- **Does the check ship as a `check-artifacts` family, or start markdown-only?** **Resolved: a sixth `check-artifacts` family from the start.** AC8 decides it — "repeat runs over an unchanged feature directory produce identical findings" is a determinism guarantee an LLM-walked prose scan cannot make, so shipping markdown-only-first would ship with AC8 knowingly unmet. The deferral's premise is also gone: "mechanize after the tell list stabilizes" assumed an evolving list, and the resolution above closed it at six, framework-fixed. And the work is exactly 022's category — a fixed word list, a structural prose unit, and a frontmatter read carry no semantic content, which is the definition of work that belongs off the LLM; `check-artifacts` already carries five families, the fifth added by 046 through this same path. This does not narrow who gets the check: the markdown-only path still performs it as prose the host walks, per the two-paths contract in §runtime-host-integration, exactly as every existing family does.

- **What is the promotion criterion from advisory to blocking?** **Resolved: the house threshold plus a precision guard** — 5 or more findings across a repo on two consecutive `/{project}:analyze --all` runs, **and** every finding in those runs confirmed a true positive. The first half matches the three existing criteria verbatim (grounding, Applicable-Rules citation consistency, and `QUAL-CLAIM-001`), preserving comparability and the second-run guard against transient mid-authoring states. The second half exists because copying the house pattern alone would be unsound *for this check*: those criteria measure volume, which is adequate when findings are LLM-judged and already filtered for plausibility, whereas this check is a mechanical word match — a noisy implementation produces 5+ findings on two consecutive runs exactly as reliably as an accurate one, so volume alone cannot distinguish real drift from false positives at the moment that distinction matters most. The precision half requires maintainer confirmation, which is a judgment step, but promoting to blocking is already a deliberate human decision rather than something the tool does to itself.

- **How does a link targeting a scenario behave, given a scenario has no `status` field?** **Resolved: evaluate against the two signals a scenario actually has — its open-question count and its file existence — and emit no finding for tells that would require a lifecycle status.** Grounded by reading `runtime/src/schema/primitives.rs:363-368` and `:388`: after [046-scenario-open-question-visibility](../046-scenario-open-question-visibility/spec.md), `ReadSpecResult` carries `scenario-open-questions` as a `Vec<{scenario, text}>`, so a per-scenario open-question count is readable even though no status is. That signal is what makes this resolution possible, which is why 046 remains a declared dependency. Concretely: `open question` / `unresolved` / `still open` contradict a count of zero; `not yet`, `does not exist`, and `left unimplemented` have no readable target state on a scenario and therefore produce nothing.

  **Amended 2026-08-02 during implementation, on measured evidence.** This resolution originally read "`does not exist` contradicts a present file", classing it as an existence tell. The first full-repo run showed that test can only ever fire and never filter — a link that resolves always points at a present file, so it cannot fail — and its one finding across all 47 specs was a false positive: `017/detect-dependency-cycles.md:28` says an *override mechanism* "does not exist today" while linking to a scenario that does. `does not exist` is therefore judged against lifecycle status, which is the mapping this spec's own §Behavior section already stated ("prose says 'does not exist yet' while the target is `in-progress` or `done`"); the two statements were in tension and the measured one wins. The tell list is still exactly six, so AC11 is unaffected, and AC14 still holds — more cleanly, since every tell needing a lifecycle status now produces nothing against a scenario. That third case is AC9 applied rather than a new rule — an unknown is never escalated to a defect. Rejected alternative: deriving a scenario's implementation state from its task checkbox in `tasks.md`, which appears to rescue the status-shaped tells but is unreliable by design — `check-artifacts`' existing scenario family documents that "a spent task pruned per §tasks-phase never counts against its scenario", so an absent task means "pruned" as often as "unimplemented", and the derived signal would be wrong in exactly the mature-spec case where it would matter most.

- **Does a tell inside a fenced code block, an HTML comment, or a quoted historical passage count?** **Resolved: four exempt contexts — fenced code block, HTML comment, blockquote, and inline code span.** Grounded by reading `runtime/src/primitives/mod.rs:841-844`: `SkipScanner` already tracks `in_fence` and `in_comment`, so the first two are free; it has no blockquote handling, so that one is added. Both additions are one-character structural tests, preserving determinism. The inline-code exemption is self-demonstrating — this spec's own resolved tell list is written in backticks, so without it any document *describing* the check trips it, starting with 045 itself and `analyze.md`. The blockquote exemption has the same shape one level up: a spec documenting drift necessarily quotes the stale claim it documents, as this spec's Motivation does; that table survives the check today only because those rows happen to carry no links, which is an accident rather than a design. A blockquote is markdown's own marker for "this is quoted material, not my assertion", which is exactly the distinction the check needs. Deliberately narrower than a general "historical passage" exemption: prose that merely reads as historical without a structural marker is not detectable deterministically, and that case is handled by the `## Motivation` resolution below.

- **How does the check avoid firing on every completed spec's `## Motivation`?** **Resolved: the existing link-adjacency requirement already handles it — no exemption, and no acceptance-criteria changes.** The question anticipated that this choice would change the ACs; measurement showed it does not. Across all 47 specs, `## Motivation` sections contain **zero** links to same-feature sibling artifacts and 21 links to *other specs* (`../NNN-…/spec.md`), which are outside this check's scope — it evaluates links to siblings within the same feature directory. The rejected heading-exemption (option b) would additionally suppress the one case that *should* fire: a Motivation bullet that links to `scenarios/foo.md` and calls its question open is a genuine stale claim about a same-feature sibling, and firing there is correct.

  **Recorded limitation:** this check would **not** have caught the 046 Motivation staleness that prompted the question, because those bullets carry no same-feature links. The Motivation-tense problem is real and remains **unaddressed by 045** — enforcing it needs tense analysis, which no deterministic check here can carry. The past-tense-Motivation authoring convention (option a) is the answer to that separate problem and was logged to the inbox for routing, with the 046/041 evidence and its two candidate homes (`framework/templates/spec/spec.md`, so every scaffolded spec carries the guidance, versus constitution §spec-requirements, normative for all adopters). It is deliberately not settled here: it is spec-authoring guidance that must reach adopters, and choosing its home is its own scoping decision rather than a sub-decision of this spec.

- **Does the check cover stale acceptance criteria, not just stale prose?** **Resolved in two parts: (a) already yes, by AC6; (b) yes, as a second, separately-specified family in this spec.** On (a), AC6 puts `spec.md` in scope wholesale, so its `## Acceptance Criteria` section is already scanned — no change. But in-scope is not the same as caught: the originating case (026's AC5, 2026-08-02) named `` `framework/workflows/registry.json` `` as a *backticked path*, not a markdown link to a same-feature sibling, so the link-adjacent check has nothing to fire on and would not have caught it.

  On (b), a **path-existence** check would have — both dead paths (`framework/workflows/registry.json`, `scripts/audit/registry-equivalence.sh`) are mechanically testable against the filesystem, with none of the tell-list machinery. It ships as its **own family**, not as an extension of the link-adjacent one, because the two have **inverted parsing rules**: the tell-scan ignores inline code spans (per the exempt-contexts resolution above), whereas path-existence must read inside them, since paths are backticked by convention. One family cannot hold both rules coherently.

  Its scope is **`## Acceptance Criteria` of `done` specs only**. A path-existence check over whole spec bodies would flag correct historical prose — 026's own Behavior §3 now reads "Retired by spec `043-workflows-sunset`, which … deleted `framework/workflows/`", a true statement naming a dead path. Acceptance criteria avoid that class entirely because they are contracts rather than narrative: an AC naming a path asserts that path is part of the delivered system.

  Kept in 045 rather than split into its own spec because two specs each adding a `check-artifacts` family and each editing `analyze.md` is precisely the bundling pair Family 7 (sibling-spec coupling) exists to surface. Accepted cost: this widens 045 beyond the "narrow by construction" framing in §Scope and requires its own acceptance criteria, added below.

## Prior art

Adopter-side interim: `svc-zmc-api` `AGENTS.md` §"Never Knowingly Leave Stale Information" (commit `8b077a77`) carries the rule, the trigger list, the sweep greps, and the worked example. It was placed in `AGENTS.md` rather than a rule file because the concern is artifact discipline enforced by `/analyze`, and because adopter edits to shipped rule files are overwritten by `/govern` unless
pinned. If this spec ships, that section becomes a candidate for replacement by the framework rule.
