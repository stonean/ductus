# 045 — Decision-State Drift Detection Plan

Implements [045 — Decision-State Drift Detection](spec.md).

## Overview

Three deliverables, in dependency order:

1. **A constitution amendment.** §drift-prevention gains a *Decision resolution* subsection stating that resolving a decision carries the same audit obligation as editing a document (ACs 1–2). No rule file and no rule ID — the §grounding precedent settled that during clarification.
2. **Two new `check-artifacts` families**, both advisory. `link-adjacent-drift` scans an artifact's block-level elements for open-state tells that its own sibling links contradict (ACs 3–9, 11–14). `criterion-path-existence` flags a filesystem path named in a `done` spec's acceptance criterion that no longer resolves (ACs 16–18). They ship as separate families because their parsing rules are inverted: the tell scan ignores inline code spans, the path scan reads only inside them.
3. **Documentation and release.** `analyze.md` documents both families and their shared promotion criterion (ACs 10, 15); the runtime ships as `gvrn-v0.26.0`.

Everything lands inside the existing `check-artifacts` primitive. No new primitive, no new MCP tool, no new slash command.

## Technical Decisions

### Two families inside `check-artifacts`, not a new primitive

`check-artifacts` already carries five families in one `run` that appends to a single `Vec<ArtifactFinding>` (`runtime/src/primitives/check_artifacts.rs:92-128`); the fifth, `scenario-open-questions`, was added by 046 through exactly this path (`check_artifacts.rs:370-397`). Two more are appended the same way, named `link-adjacent-drift` and `criterion-path-existence` to match the existing kebab-case family names.

Consequence worth stating because it inverts the usual expectation: the six-site primitive-wiring checklist in AGENTS.md §Gotchas does **not** apply. `check-artifacts` is already in `TOOL_NAMES`, `PRIMITIVE_NAMES`, `framework/runtime-tools.txt`, the interpreter dispatch, and `main.rs`. Nothing new is registered anywhere.

### Implementation ownership: the runtime work lands as scenarios under 022

This spec owns the **requirement** — what drift means, that the constitution obliges the sweep, and the eighteen acceptance criteria. It does not own the primitive that carries it.

The precedent is exact and recent. 046 added the fifth `check-artifacts` family and routed every primitive-level change into **scenarios under 022-deterministic-runtime**, keeping only its constitution amendments and its acceptance criteria (`specs/046-scenario-open-question-visibility/spec.md:87-101`; the scenario is `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`). It is not merely a convention either: `specs/022-deterministic-runtime/data-model.md:621-648` is the canonical registry of `check-artifacts` families — it names all five and their severity tiers in one paragraph — so a new family has to be recorded there whatever else happens. Leaving that paragraph at five while the primitive ships seven would be the same defect class this spec exists to detect.

Four scenarios land under 022, each back-linking here per §cross-spec-impact:

| Scenario | Covers |
| --- | --- |
| `block-element-scanner` | the block splitter, the blockquote exemption, `inline_code_spans` visibility |
| `check-artifacts-skipped-targets` | `SkippedTarget` and the `skipped` field |
| `link-adjacent-drift-family` | tells, sibling resolution, target-state mapping, per-link evaluation |
| `criterion-path-existence-family` | `done`-spec scope, the code-span path grammar |

Recording them on a `done` 022 takes it through the scenario back-edge to `in-progress`, exactly as 046's did. What stays with 045: the constitution amendment, `analyze.md`'s documentation of the two checks, this plan, and the acceptance criteria — which are verified against shipped behavior regardless of which spec's tasks produced it, so 045 cannot reach `done` before the 022 scenarios do.

This routing was not settled during clarification; it is a planning decision taken on the 046 precedent and on 022's ownership of the family registry.

### A sibling link is one that lexically resolves inside the same feature directory

For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope).

Resolution is **lexical**, not `std::fs::canonicalize`: the target may legitimately not exist (a broken link is a different check's business), and `canonicalize` errors on a missing path. Lexical resolution is also symlink-independent, which AC8's repeat-run determinism wants.

Excluded before resolution: any target carrying a URL scheme (`http:`, `https:`, `mailto:`), and any target that is a bare fragment (`#anchor`). A fragment on a sibling target (`plan.md#tasks`) is stripped and the file part is used — the fragment itself is the semantic case §Scope puts out of scope.

### A new block splitter, and `SkipScanner` is left alone

The scanned unit is the enclosing block-level element (AC12). A new `pub(crate)` helper in `runtime/src/primitives/mod.rs` yields `(start_line, text)` blocks:

- a line whose trimmed form starts with `|` is a **table row**, and is its own block;
- a line matching the bullet grammar (`-`, `*`, or `N.`) opens a **list item**, which extends through following indented continuation lines;
- any other maximal run of non-blank, non-heading lines is a **paragraph**;
- a line whose trimmed form starts with `>` is a **blockquote** line and is dropped entirely.

`SkipScanner` (`runtime/src/primitives/mod.rs:841-894`) already tracks `in_fence` and `in_comment` and skips their delimiter lines, so the first two exempt contexts are free — the splitter feeds every line through it and drops what it reports. The blockquote exemption is added **in the splitter, not in `SkipScanner`**. That is deliberate: `SkipScanner` is shared by the `tasks.md` parsers (`read_tasks`, `mark_task`, `prune_tasks`, `iter_task_numbers_at_levels` at `mod.rs:963-983`), and teaching it to skip blockquotes would silently change how every one of them reads a quoted task line. The spec's phrasing — the first two "fall out of the existing `SkipScanner`", the last two "are added" — is satisfied by adding them at the check, and the narrower change is the safe one.

### The inline-code exemption reuses the existing CommonMark span helper

`inline_code_spans` (`runtime/src/primitives/mod.rs:901-932`) already implements the CommonMark equal-length-backtick-run rule, and `find_outside_code` (`:938-954`) uses it to make comment delimiters inert inside code font. Both are private to `mod.rs`; this change promotes `inline_code_spans` to `pub(crate)` with no behavior change.

The two families then sit on opposite sides of the same computation, which is the cleanest statement of why they are two families:

- `link-adjacent-drift` counts a tell only at a byte offset **outside** every span;
- `criterion-path-existence` reads **only** span contents.

The inline-code exemption is self-demonstrating: this plan, the spec, and `analyze.md` all write the tell list in backticks, so without it every document describing the check would trip it.

### The tell table, and one finding per (block, link) pair

The six tells and the target state each contradicts are tabulated in [data-model.md](data-model.md). Evaluation is per link, so a block with three links is scanned three times and fires only for the target whose state actually contradicts (AC12).

A block emits **one finding per link**, not one per tell. `does not exist yet` matches two tells at once (`does not exist` and `not yet`); emitting per-tell would report one authorial claim twice and inflate the 5-finding promotion threshold with duplicates. The message names every tell that fired, in tell-list order — a fixed order over a fixed list, so repeat runs are byte-identical (AC8).

### An unreadable target produces no finding, but is not silent

AC9 forbids escalating an unknown to a defect: a link whose target state cannot be read produces no finding. Left there, the family would emit zero findings both when it examined every link and found nothing and when it could not read a single target — the exact shape `QUAL-CLAIM-001` forbids (`framework/rules/quality-cross.md:39`).

`CheckArtifactsResult` therefore gains a `skipped: Vec<SkippedTarget>` field recording `{family, reason, path}` for every target a family could not examine. The rule's own Verification names "a `skipped` list" among the explicitly compliant shapes (`quality-cross.md:43`), so this is the sanctioned form rather than an invention.

`clean` keeps its existing meaning — no findings — so no existing consumer changes. The host renders `skipped` in the report's **Informational** tier, which is where `analyze.md:169-174` already puts cross-service unknowns (`unregistered`, `not-checked-out`, `status-unreadable`): informational, not a finding. Same contract, second surface.

### Path extraction reads code spans only

AC16 requires reading inside inline code spans. This narrows further: extraction considers **only** span contents, and a span qualifies as a path when its whole trimmed content

- contains at least one `/`;
- contains no whitespace;
- contains none of `{ } * ? [ ] < > $ | :`;
- does not begin with `-` or `/`.

The exclusion set is what makes the check usable in this repo rather than a noise generator. `:` alone rejects `https://…`, `path:line` citations, and every `/{project}:analyze` slash-command reference; `{`/`}` rejects placeholders; `*`/`?`/`[`/`]` reject globs; a leading `-` rejects flags. A trailing `/` marks a directory. Resolution is repo-root-relative and satisfied by a file **or** a directory, so `framework/workflows/` resolves correctly.

The Behavior section reads "including paths inside inline code spans"; this plan implements "code spans only", the narrower reading AC16 states literally. Rationale in Trade-offs.

### Which artifacts the link-adjacent check scans

Exactly AC6's set: `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md`. Scenario enumeration reuses `list_scenario_files` (already used at `check_artifacts.rs:253`) so the `.md` match stays case-insensitive and the scanned set matches the one `dashboard` counts.

`review.md` is deliberately excluded. A review record is pinned to its `reviewed-against` sha and describes the state at that commit, so prose there is correct-as-written and would generate systematic false positives — the one artifact where staleness is the format, not a defect. `data-model.md` is excluded as structure rather than narrative; it is the cheapest additive extension if evidence ever appears.

### The path-existence family reads through `read-spec`

Acceptance criteria come from `read_spec`'s parsed `acceptance_criteria` (each entry carrying `checked` and `text`), not from a second hand-rolled section walker. `check-artifacts` already delegates task parsing to `read_tasks` (`check_artifacts.rs:96-105`) and scenario questions to `read_spec::collect_scenario_open_questions` (`:377`) for exactly this reason — the module docs state the no-hand-rolled-parsers constraint at `check_artifacts.rs:45-48`. Scope stays `done` specs, `## Acceptance Criteria` only.

### Constitution amendment shape

§drift-prevention gains `### Decision resolution` immediately after its `### Cross-document references` subsection, which is the obligation it extends. It states the trigger list (closing an open question, shipping a scenario, advancing a status, adopting a previously-rejected option) and the completion rule (a resolution is incomplete while a sibling artifact still describes the prior state).

§drift-prevention's **Canonical sources** table gains a row pointing the open-state tell list at this spec's `data-model.md`, matching the existing rows that point rule-ID conventions and the service-registry schema at a spec data-model. Without it the tell list would be a fact described in the constitution, `analyze.md`, and the runtime with no named owner — the drift this section exists to prevent.

The subsection defers to `analyze.md` for the checks themselves rather than naming them, because §drift-prevention's own Canonical sources rule puts command behavior in the command's source. That also keeps the constitution from carrying a forward reference that stays false until task 11 lands.

§drift-prevention's **Template-rule alignment** subsection is not triggered: it binds *blocking* checks to a template element, and both families are advisory at introduction (AC7).

### Release path

The change touches `runtime/`, so it is not landed until a tag ships it (AGENTS.md §Workflow). `runtime/Cargo.toml` goes `0.25.0` → `0.26.0` — minor, because the families are new behavior and `skipped` is an additive schema field — with a matching `runtime/CHANGELOG.md` section, then `git tag gvrn-v0.26.0` and push.

No golden re-bless. `runtime/tests/golden/analyze-basic.jsonl` records 13 lines of `progress` / `llm-request` / `complete` envelopes only — no primitive payload — so new families change nothing in it, and the fixture spec `003-analyze` is at `status: clarified`, below the path-existence family's `done` scope. `BLESS=1` stays untouched either way, per the standing prohibition.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | §drift-prevention *Decision resolution* subsection; Canonical sources row for the tell list |
| `framework/commands/analyze.md` | Modify | Step 8 family count and description; two new markdown-only reference sections; shared promotion criterion |
| `runtime/src/primitives/mod.rs` | Modify | Block splitter helper; `inline_code_spans` promoted to `pub(crate)` |
| `runtime/src/primitives/check_artifacts.rs` | Modify | Both families, their skip records, and unit tests; module docs updated to seven families |
| `runtime/src/schema/primitives.rs` | Modify | `SkippedTarget` type; `skipped` field on `CheckArtifactsResult`; family lists in `ArtifactFinding` / `CheckArtifactsResult` doc comments |
| `runtime/src/mcp/server.rs` | Modify | `check-artifacts` tool description — currently names four families, needs seven |
| `runtime/Cargo.toml` | Modify | Version `0.25.0` → `0.26.0` |
| `runtime/CHANGELOG.md` | Modify | `[0.26.0]` section |
| `specs/045-decision-state-drift-detection/data-model.md` | Create | Tell table, contradiction mapping, path grammar, `SkippedTarget` shape |
| `specs/022-deterministic-runtime/scenarios/block-element-scanner.md` | Create | Block splitter and the blockquote exemption |
| `specs/022-deterministic-runtime/scenarios/check-artifacts-skipped-targets.md` | Create | `SkippedTarget` and the `skipped` field |
| `specs/022-deterministic-runtime/scenarios/link-adjacent-drift-family.md` | Create | The first family's behavior |
| `specs/022-deterministic-runtime/scenarios/criterion-path-existence-family.md` | Create | The second family's behavior |
| `specs/022-deterministic-runtime/data-model.md` | Modify | Family registry five → seven; `skipped` on the `check-artifacts` result shape |
| `specs/022-deterministic-runtime/spec.md` | Modify | Scenario back-edge `done` → `in-progress` |
| `specs/022-deterministic-runtime/tasks.md` | Modify | Tasks for the four scenarios |
| `.claude/commands/ductus/analyze.md` | Generated | Regenerated by the pre-commit hook — never hand-edited |

## Data Model

See [data-model.md](data-model.md). It is the canonical home for the closed tell list (AC11), the tell → contradicted-state mapping, the path grammar, and the `SkippedTarget` shape.

## Trade-offs

**Code-span-only path extraction.** The Behavior section says "including paths inside inline code spans"; this implements code spans only, which is what AC16 states literally. Outside a code span, a `/`-bearing token in this repo's acceptance criteria is far more often a slash command (`/{project}:review`), a placeholder, or an `and/or` than a path — and the exclusion set that would be needed to tell them apart is the same one already applied inside spans, minus the backtick signal that makes it reliable. A bare unbackticked path in an AC will be missed. Accepted: the originating case and every path in this repo's acceptance criteria is backticked, and widening later is additive.

**A block is coarser than a sentence.** Recorded during clarification and unchanged here: a long paragraph can pair a tell with an unrelated link. Bounded by per-link evaluation and by advisory severity. Rejected alternative — a sentence splitter — remains rejected; the repo has none, and Family 8 (`introducing-drift.sh`) is the existing check that splits sentences and concedes false positives in its own done-when.

**`does not exist` fires against artifact existence, not the subject of the sentence.** In the observed case, "the worker it would notify does not exist yet" is a claim about *code*, while the link resolves to a *scenario*. The check reads the link target's existence, so it fires on the artifact rather than on the worker. That is the intended coarse behavior — the finding still lands on the right block — but the finding's message must describe what was actually checked rather than implying the subject was verified.

**`review.md` and `data-model.md` are out of the scanned set.** Excluding `review.md` is a genuine coverage hole: a review record can and does link to siblings. It is excluded because its prose is pinned to a past sha by design, so the check would be wrong there more often than right.

**The tell list is fixed with no per-project extension.** Acknowledged during clarification as the same shape as Family 6's hardcoded SSOT list, which 026's own plan concedes is "a derive-don't-ask principle (`017-derive-dont-ask`) violation in miniature". Accepted because the check is advisory: a false positive costs a glance, not a blocked gate. The revisit trigger is a project reporting domain prose that trips a tell repeatedly.

**The precision half of the promotion criterion needs a human.** "Every finding confirmed a true positive" is a maintainer judgment, not something the tool computes. That is deliberate — a mechanical word match produces 5+ findings on two consecutive runs just as reliably when it is noisy as when it is accurate, so volume alone cannot carry the promotion decision for this check the way it can for the LLM-judged ones.

**`clean` still reports `true` alongside a non-empty `skipped`.** Preserving `clean == findings.is_empty()` keeps every existing consumer working, but it means the assurance a caller gets from `clean` alone is unchanged — the honesty lives in `skipped`, and only a host that renders it delivers the benefit. Changing `clean` to account for skips was rejected as a silent behavior change to four shipped families.
