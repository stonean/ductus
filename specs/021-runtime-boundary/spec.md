---
status: done
dependencies: [020-code-review]
next-criterion: 12
---

# 021 — Runtime Boundary

Establish the constitutional boundary for the deterministic runtime: what it may and may not do, which capabilities are eligible to move into it, and the CI invariant that holds it to its guarantee. This spec ships the constitutional amendment and the CI workflow that enforces it. No binary, no MCP tools, no slash-command rewiring — those are scoped to a follow-up spec (022).

## Motivation

`ductus`'s constitution declared text-first artifacts load-bearing: "adopting `ductus` requires no bootstrap tooling beyond the AI agent itself" (§text-first-artifacts). That principle was correct for the markdown framework, but it foreclosed an emerging need: deterministic execution of mechanical checks and fixes that were LLM-executed at meaningful token cost and with probabilistic reliability.

The motivating evidence was concrete. [020-code-review](../020-code-review/plan.md)'s plan acknowledges directly: "we cannot bolt on a deterministic linter without changing the framework's shape." The CI gate it shipped used `awk`-based YAML parsing because no binary existed; the same blocking check was implemented three times (in `/ductus:implement`, in `/ductus:analyze`, and in CI bash) because there was no single deterministic enforcer; and cross-pass dedupe and idempotency hashing were punted to LLM judgment despite being purely mechanical.

This spec changed the constitution's shape — deliberately, with bounded scope — to permit a runtime that absorbs deterministic work while preserving the load-bearing properties: markdown is source of truth, the agent's write path stays simple, and PRs review glanceably. As shipped it preserved one more, that adopters who install nothing still complete every pipeline cycle; 048 gave that one up when it made the runtime required.

## Constitutional Amendment

> [048-govern-acquired-runtime](../048-govern-acquired-runtime/spec.md) amended three elements this section describes — principle 3, eligibility criterion 3, and the CI invariant — when it made the runtime **required and pipeline-acquired** rather than optional. Each is stated below as it now stands, with what this spec shipped beside it.

### §text-first-artifacts opening paragraph (edit)

The opening paragraph acknowledges the runtime without weakening the markdown source-of-truth claim. Its load-bearing properties (Edit-driven write path, glanceable PRs, rare merge conflicts, markdown-as-source-of-truth) are preserved verbatim. The clause "adopting `ductus` requires no bootstrap tooling beyond the AI agent itself" is replaced by language that separates the artifacts from the pipeline: the artifacts are usable standalone with no tooling beyond the AI agent, while the pipeline that reads them requires the runtime (§runtime-boundary). As this spec shipped it, the second half called the runtime optional and opt-in.

### §runtime-boundary (new subsection)

A new subsection containing five elements:

**Five principles** establishing what the runtime can and cannot do:

1. **Markdown is source of truth** — the runtime MUST NOT own state the markdown cannot reconstruct. Runtime-owned data (caches, indexes, parsed graphs) is derived and gitignored, per the existing rule on structured derived views.
2. **Determinism only** — the runtime MUST NOT call an LLM. Work requiring semantic judgment (content quality, `/clarify` resolution, `/specify` sketching, per-rule Verification reads, `/groom` routing) stays in slash commands.
3. **Required, and acquired by the pipeline** — `/{project}` acquires the pinned runtime during adoption, so a bootstrapped project has the binary and pipeline commands may assume determinism. Acquisition failure halts the run rather than degrading. A project may supply its own binary instead, and shell pipelines that parse frontmatter or markdown structure are not a sanctioned substitute for the runtime's primitives. This spec shipped principle 3 as *Opt-in for adopters*: the runtime MUST NOT be a prerequisite for any pipeline gate, and a markdown-only adopter — agent + `Edit`, no binary on `PATH` — must complete every cycle and reach `done` on every spec.
4. **Schema follows the constitution** — the runtime MUST read frontmatter and artifact structure according to the schemas the constitution declares. Schema changes ship through the constitution, and the runtime MUST update to match. The constitution MUST NOT import runtime types.
5. **MCP is the seam** — the runtime MUST expose its capabilities as MCP tools, so slash commands can call them when they want determinism. This keeps the runtime accessible to any agent host and prevents `ductus`-specific coupling.

**Three eligibility criteria** for moving a capability into the runtime — a capability is runtime-eligible only when **all three** hold:

1. **Deterministic** — no semantic judgment required; the same inputs always produce the same outputs.
2. **Currently mechanical** — already either (a) executed by an LLM following procedural instructions in a slash command body, or (b) implemented as a bash script the framework invokes (pre-commit hooks, generators, CI).
3. **Specifiable as prose** — the capability can be stated completely enough that the markdown-only reference documents it, and a primitive mirrors that reference rather than introducing policy of its own. This spec shipped criterion 3 as *Degradation, not failure, when removed*: without the runtime, the work still completes correctly via the markdown-only path, and only speed, cost, or reliability degrades. With no markdown-only execution path left to degrade to, 048 replaced it; the criterion still keeps a capability out of the runtime unless its policy can be written down.

**Acquisition invariant** — the repository's CI MUST include a job that exercises acquisition end-to-end on every supported platform: fetch the published asset for the target, verify its sidecar digest, install it into a temporary store, and execute the installed binary. A release whose assets an adopter cannot actually acquire is a constitution violation, not a feature. This spec shipped the **opt-in invariant** in its place — a CI job exercising a representative pipeline cycle with the runtime binary absent from `PATH`, so that a slash command silently requiring the runtime failed CI. The old job tested the guarantee principle 3 used to make; the new one tests the guarantee that replaced it.

**Versioning** — the runtime ships in lockstep with the framework. A `ductus` release includes the binary built against the schemas in that release; an adopter's `ductus` version pins their compatible runtime version, eliminating schema/runtime drift as a failure mode.

**Explicit non-scope** — the runtime MUST NOT be a spec authoring tool, MUST NOT be a workflow orchestrator, MUST NOT be a long-running service, and MUST NOT be a storage layer. Lifting any of these exclusions requires a constitutional amendment.

### §drift-prevention canonical sources (edit)

A row is added to the canonical sources table in §drift-prevention naming §runtime-boundary as the source of truth for runtime contract questions, so future work that touches the runtime knows where the binding statements live.

## Acceptance Criteria

- [x] AC1: `framework/constitution.md` §text-first-artifacts opening paragraph is amended to reference an optional runtime; the load-bearing properties (Edit, glanceable PRs, rare conflicts, markdown source-of-truth) remain in the paragraph verbatim. Delivered as written. **Superseded by 048:** the paragraph now says the pipeline that reads the artifacts requires the runtime. The load-bearing properties are still in it verbatim.
- [x] AC2: `framework/constitution.md` contains a new `§runtime-boundary` subsection with the five principles, the three eligibility criteria, the opt-in invariant, the versioning rule, the non-scope list, and a one-line forward pointer to spec 022. Delivered as written. **The invariant is superseded:** 048 replaced the opt-in invariant with the acquisition invariant. Every other element named here is still in the subsection.
- [x] AC3: The five principles use RFC 2119 keywords (MUST / MUST NOT) consistent with the rule format declared in §rules. Delivered as written. **No longer holds for principle 3:** as 048 reworded it, *Required, and acquired by the pipeline* states its requirement in prose and grants a permission (MAY). The other four still use MUST or MUST NOT.
- [x] AC4: The non-scope list uses MUST NOT for each excluded item, in the same RFC 2119 register as the five principles.
- [x] AC5: An anchor marker `<!-- §runtime-boundary -->` exists at the new subsection so `/ductus:analyze`'s anchor-resolution check resolves references to it.
- [x] AC6: `framework/constitution.md` §drift-prevention canonical sources table contains a row whose Fact column names "Runtime contract / boundary," pointing at `framework/constitution.md` §runtime-boundary.
- [x] AC7: A CI workflow exists in this repo that asserts: (a) no runtime binary on `PATH`, (b) all bash generator scripts (`gen-spec-deps.sh`, `gen-readme-table.sh`, `gen-help-tables.sh`) run clean in `--dry-run`, (c) `npx markdownlint-cli2` passes, (d) a tool-coverage lint scans `framework/commands/*.md` and verifies every reference to a runtime tool is wrapped in a graceful-fallback pattern, and (e) frontmatter integrity (status enum and dependencies-list shape) holds across all spec and scenario files. Delivered as written, as `markdown-only-pipeline.yml`. **Superseded in part:** 048 removed check (a) with the opt-in invariant and renamed the workflow `.github/workflows/framework-checks.yml`. Of (b)'s generators, `gen-help-tables.sh` still runs in `--dry-run`; `gen-spec-deps.sh` and `gen-readme-table.sh` were removed, and their derivations are the `derive-dependencies` and `derive-references` primitives, which the workflow runs instead. Checks (c), (d) and (e) still run there.
- [x] AC8: The CI workflow above runs on every PR that modifies `framework/`, `specs/`, or `.claude/commands/`.
- [x] AC9: The existing principle bullets in §text-first-artifacts (markdown by default, frontmatter for metadata, relative links not wiki-links, derived views gitignored, exceptions require amendment) are unchanged — only the opening paragraph is edited.
- [x] AC10: `/ductus:analyze` against this spec passes (no hard-fail or blocking findings) once acceptance criteria are met.
- [x] AC11: `npx markdownlint-cli2` against `framework/constitution.md` and the new spec files passes after the edits.

## Non-Goals

- Building the runtime binary — deferred to spec 022.
- Moving any existing `/ductus:analyze` deterministic checks into a runtime — deferred to spec 022.
- Defining MCP tool schemas — deferred to spec 022.
- Choosing the runtime's implementation language (Rust, Go, etc.) — project-level decision, not constitutional.
- Release engineering, distribution, or version-coordination tooling for the binary — project-level.
- Editing slash command files to call runtime tools — deferred to spec 022.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **§runtime-boundary placement** — subsection within §text-first-artifacts (not a sibling top-level section). The runtime boundary is a clause of the text-first principle ("text is source of truth; runtime is a derived view"), not a standalone load-bearing concept. Subsection placement mirrors the existing structure (Principles, Frontmatter Schema, Validation Severity, then Runtime Boundary) and makes the dependency on text-first explicit. Future amendments can lift the subsection out if scope demands it; that is a small mechanical edit when the need materializes.
- **Opt-in CI invariant — composition of the "representative pipeline cycle"** — the invariant runs as a deterministic check job, not an LLM-driven cycle. Concretely the CI job asserts: (1) the runtime binary is absent from `PATH`, (2) all bash generator scripts (`gen-spec-deps.sh`, `gen-readme-table.sh`, `gen-help-tables.sh`) run clean in `--dry-run`, (3) `npx markdownlint-cli2` passes, (4) a tool-coverage lint scans `framework/commands/*.md` for references to runtime tools and verifies each is wrapped in a graceful-fallback pattern, and (5) frontmatter integrity (status enum + dependencies list shape) is checked programmatically. LLM-driven commands (`/ductus:specify`, `/ductus:clarify`, `/ductus:plan`, etc.) are deliberately excluded — they're LLM-driven by design and the runtime cannot subsume them; their fallback safety is enforced structurally by check (4) at the source-file level. The invariant is a tripwire, not a smoke test: vacuously satisfied today, it fails the moment spec 022 wires a slash command to a runtime tool without a fallback. Precedent: spec 020's plan rejected running `/ductus:review` in CI for the same reason — LLM-in-CI introduces API budget and non-determinism. **Retired by 048** along with the opt-in invariant: check (1) is gone, the acquisition invariant took the invariant's place, and checks (2)–(5) still run in `framework-checks.yml` as ordinary framework checks, (2) through the derivation primitives.
- **"What the runtime is not" wording** — normative, using MUST NOT, to match the RFC 2119 register of the five principles in the same subsection. Final phrasing: *"to prevent scope creep, the runtime MUST NOT be a spec authoring tool, MUST NOT be a workflow orchestrator, MUST NOT be a long-running service, and MUST NOT be a storage layer. Lifting any of these exclusions requires a constitutional amendment."* Rationale: descriptive prose would create an inconsistency within the subsection (the five principles are MUST/MUST NOT) and would make the existing "Lifting requires amendment" sentence incoherent — "lifting" is meaningless against descriptive language.
- **Opt-in CI invariant — when it runs** — every PR that modifies `framework/`, `specs/`, or `.claude/commands/`, starting now (not deferred until the binary exists). The invariant is a tripwire that must already be running before the first runtime-touching PR so that PR is the one that fails. It is vacuously satisfied today (no command references a runtime tool yet) at the cost of trivial workflow-runner minutes. The path filter ensures docs-only or release-only PRs skip the workflow. **Retired by 048** with the invariant; the path filter survives on `framework-checks.yml`, which scopes the remaining framework checks.
- **Capability listing in §runtime-boundary** — capability-agnostic, with a single forward pointer. §runtime-boundary defines the boundary; actual capabilities live in their introducing specs (beginning with spec 022). Mirrors how §rules works: the constitution defines the rules tier and schema; actual rules live in `specs/rules/{rule-set}.md` and are not enumerated in the constitution. A one-line pointer at the end of the subsection — *"Specific capabilities are introduced through their own feature specs, beginning with spec 022 (deterministic runtime)."* — anchors the reader without pinning content. Reader-side concreteness already lives in this spec's Motivation section (awk YAML parsing, three-place gate, cross-pass dedupe); the constitution does not need to repeat it.
- **Runtime-eligibility note requirement on slash commands** — no requirement. A hand-authored "deterministic vs. semantic" note on each slash command source is exactly the anti-pattern the Design Principles section in `AGENTS.md` forbids: it depends on author diligence and fails silently when an author forgets. The substantive invariant — that any runtime-tool reference in a slash command has a graceful fallback — is already enforced by the CI tool-coverage lint (check 4 from the opt-in invariant resolution above, which still runs), which is derived from the command source rather than from an author-supplied marker. If richer signals are wanted later, they are extracted by static analysis, not by hand.
