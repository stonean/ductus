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

> [048-govern-acquired-runtime](../048-govern-acquired-runtime/spec.md) made the runtime **required and pipeline-acquired**: principle 3, eligibility criterion 3 and the CI invariant below are stated as they now stand.

### §text-first-artifacts opening paragraph (edit)

The opening paragraph acknowledges the runtime without weakening the markdown source-of-truth claim. Its load-bearing properties (Edit-driven write path, glanceable PRs, rare merge conflicts, markdown-as-source-of-truth) are preserved verbatim. The clause "adopting `ductus` requires no bootstrap tooling beyond the AI agent itself" is replaced by language that separates the artifacts from the pipeline: the artifacts are usable standalone with no tooling beyond the AI agent, while the pipeline that reads them requires the runtime (§runtime-boundary).

### §runtime-boundary (new subsection)

A new subsection containing five elements:

**Five principles** establishing what the runtime can and cannot do:

1. **Markdown is source of truth** — the runtime MUST NOT own state the markdown cannot reconstruct. Runtime-owned data (caches, indexes, parsed graphs) is derived and gitignored, per the existing rule on structured derived views.
2. **Determinism only** — the runtime MUST NOT call an LLM. Work requiring semantic judgment (content quality, `/clarify` resolution, `/specify` sketching, per-rule Verification reads, `/groom` routing) stays in slash commands.
3. **Required, and acquired by the pipeline** — `/{project}` acquires the pinned runtime during adoption, so a bootstrapped project has the binary and pipeline commands may assume determinism. Acquisition failure halts the run rather than degrading. A project may supply its own binary instead, and shell pipelines that parse frontmatter or markdown structure are not a sanctioned substitute for the runtime's primitives.
4. **Schema follows the constitution** — the runtime MUST read frontmatter and artifact structure according to the schemas the constitution declares. Schema changes ship through the constitution, and the runtime MUST update to match. The constitution MUST NOT import runtime types.
5. **MCP is the seam** — the runtime MUST expose its capabilities as MCP tools, so slash commands can call them when they want determinism. This keeps the runtime accessible to any agent host and prevents `ductus`-specific coupling. A primitive that, called without a prompt, would run a command the project supplies, write outside the project and its staging directory, or write a host's settings or command files MAY be left off the MCP surface; it stays a `ductus <name>` subcommand and a `ductus exec` step.

> **Signpost ([022-deterministic-runtime](../022-deterministic-runtime/spec.md)):** principle 5's
> exception for a primitive left off the MCP surface came from 022's tasks 14 and 19, which took
> `run-generator` off it because it runs any project file with `bash`. 022 records the obligation
> in its `cross-spec-impact:` frontmatter; this signpost is its discharge.

**Three eligibility criteria** for moving a capability into the runtime — a capability is runtime-eligible only when **all three** hold:

1. **Deterministic** — no semantic judgment required; the same inputs always produce the same outputs.
2. **Currently mechanical** — already either (a) executed by an LLM following procedural instructions in a slash command body, or (b) implemented as a bash script the framework invokes (pre-commit hooks, generators, CI).
3. **Specifiable** — the primitive's compiled description can state its rules completely, and it introduces no policy of its own. The binary is the source of a primitive's rules (spec 079), so a capability whose policy cannot be written down stays out of the runtime.

**Acquisition invariant** — the repository's CI MUST include a job that exercises acquisition end-to-end on every supported platform: fetch the published asset for the target, verify its sidecar digest, install it into a temporary store, and execute the installed binary. A release whose assets an adopter cannot actually acquire is a constitution violation, not a feature.

**Versioning** — the runtime ships in lockstep with the framework. A `ductus` release includes the binary built against the schemas in that release; an adopter's `ductus` version pins their compatible runtime version, eliminating schema/runtime drift as a failure mode.

**Explicit non-scope** — the runtime MUST NOT be a spec authoring tool, MUST NOT be a workflow orchestrator, MUST NOT be a long-running service, and MUST NOT be a storage layer. Lifting any of these exclusions requires a constitutional amendment.

### §drift-prevention canonical sources (edit)

A row is added to the canonical sources table in §drift-prevention naming §runtime-boundary as the source of truth for runtime contract questions, so future work that touches the runtime knows where the binding statements live.

## Acceptance Criteria

- [x] AC1: `framework/constitution.md` §text-first-artifacts opening paragraph acknowledges the runtime: the artifacts are usable standalone with no tooling beyond the AI agent, while the pipeline that reads them requires the runtime (§runtime-boundary). The load-bearing properties (Edit, glanceable PRs, rare conflicts, markdown source-of-truth) remain in the paragraph verbatim.
- [x] AC2: `framework/constitution.md` contains a new `§runtime-boundary` subsection with the five principles, the three eligibility criteria, the acquisition invariant, the versioning rule, and the non-scope list.
- [x] AC3: Principles 1, 2, 4 and 5 use RFC 2119 keywords (MUST / MUST NOT) consistent with the rule format declared in §rules; principle 3, *Required, and acquired by the pipeline*, states its requirement in prose and grants a permission (MAY).
- [x] AC4: The non-scope list uses MUST NOT for each excluded item, in the same RFC 2119 register as the five principles.
- [x] AC5: An anchor marker `<!-- §runtime-boundary -->` exists at the new subsection so `/ductus:analyze`'s anchor-resolution check resolves references to it.
- [x] AC6: `framework/constitution.md` §drift-prevention canonical sources table contains a row whose Fact column names "Runtime contract / boundary," pointing at `framework/constitution.md` §runtime-boundary.
- [x] AC7: A CI workflow, `.github/workflows/framework-checks.yml`, exists in this repo that asserts, among its checks: (b) the derivations and `gen-help-tables.sh` report the committed indexes in sync, the generator in `--dry-run` and `derive-dependencies` and `derive-references` as report-only primitives that exit non-zero when stale, (c) `npx markdownlint-cli2` passes, (d) a tool-coverage lint scans `framework/commands/*.md` and verifies every command naming a runtime tool carries the host-integration pointer, and (e) frontmatter integrity (status enum and dependencies-list shape) holds across all spec and scenario files.
- [x] AC8: The CI workflow above runs on every PR that modifies `framework/`, `specs/`, or `.claude/commands/`.
- [x] AC9: The principle bullets in §text-first-artifacts (frontmatter for metadata, relative links not wiki-links, derived views gitignored, exceptions require amendment) state the rules this spec's edit left in place; its edit touched only the opening paragraph, which states markdown by default.
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
- **CI checks — composition of the "representative pipeline cycle"** — the checks run as a deterministic check job, not an LLM-driven cycle. Concretely `framework-checks.yml` asserts: (1) the generators and derivations are in sync (`gen-help-tables.sh --dry-run`, `derive-dependencies`, `derive-references`), (2) `npx markdownlint-cli2` passes, (3) a tool-coverage lint scans `framework/commands/*.md` for references to runtime tools and verifies each command naming one carries the host-integration pointer, and (4) frontmatter integrity (status enum + dependencies list shape) is checked programmatically. LLM-driven commands (`/ductus:specify`, `/ductus:clarify`, `/ductus:plan`, etc.) are deliberately excluded — they're LLM-driven by design and the runtime cannot subsume them. Precedent: spec 020's plan rejected running `/ductus:review` in CI for the same reason — LLM-in-CI introduces API budget and non-determinism.
- **"What the runtime is not" wording** — normative, using MUST NOT, to match the RFC 2119 register of the five principles in the same subsection. Final phrasing: *"to prevent scope creep, the runtime MUST NOT be a spec authoring tool, MUST NOT be a workflow orchestrator, MUST NOT be a long-running service, and MUST NOT be a storage layer. Lifting any of these exclusions requires a constitutional amendment."* Rationale: descriptive prose would create an inconsistency within the subsection (the five principles are MUST/MUST NOT) and would make the existing "Lifting requires amendment" sentence incoherent — "lifting" is meaningless against descriptive language.
- **CI checks — when they run** — on every pull request and every push to `main`, with no path filter, because the self-audit's families (spec 026) read far past `framework/` and `specs/`.
- **Capability listing in §runtime-boundary** — capability-agnostic. §runtime-boundary defines the boundary; actual capabilities live in their introducing specs, and a primitive's behavior in its compiled description, which §drift-prevention's canonical-sources table names. Mirrors how §rules works: the constitution defines the rules tier and schema; actual rules live in `specs/rules/{rule-set}.md` and are not enumerated in the constitution. Reader-side concreteness already lives in this spec's Motivation section (awk YAML parsing, three-place gate, cross-pass dedupe); the constitution does not need to repeat it.
- **Runtime-eligibility note requirement on slash commands** — no requirement. A hand-authored "deterministic vs. semantic" note on each slash command source is exactly the anti-pattern the Design Principles section in `AGENTS.md` forbids: it depends on author diligence and fails silently when an author forgets. The substantive invariant — that a command naming a runtime tool carries the host-integration pointer — is enforced by the CI tool-coverage lint (check 3 of the CI checks above), which is derived from the command source rather than from an author-supplied marker. If richer signals are wanted later, they are extracted by static analysis, not by hand.
