# 026 — Framework self-audit (`/audit`) Plan

Implements [026 — Framework self-audit](spec.md).

## Overview

`/audit` ships as a parseable slash command at `framework/commands/audit.md` (NOT scaffolded into adopters). The command orchestrates a check-zero precondition pass over the project's generators and lints, then walks the family checks — the eight defined in the spec at v1, and every family `scripts/audit/run-all.sh` registers since. No count is stated: Family 28 holds the registry, the command's enumeration, and `scripts/audit/README.md` §Scripts in agreement, and a number in prose is the copy nothing updates. Each family check is implemented as a focused shell script under `scripts/audit/{family}.sh`, invoked from the command procedure via the runtime's existing `run-generator` primitive.

Per [Q2's resolution](spec.md#resolved-questions), v1 uses shell-first orchestration with the runtime as the invocation mechanism — no new primitives are added in 026. The pattern is intentional: when a check pattern proves common enough across multiple commands, it graduates to a runtime primitive in a follow-on. Inventing primitives for `/audit` alone would overengineer ahead of data.

Output is stdout-only ([Q3](spec.md#resolved-questions) — no `audit.md` artifact). Exit code is binary (`0` / `1`). CI integration: `.github/workflows/framework-checks.yml` runs `/audit` on every PR; `.github/workflows/runtime-release.yml` runs `/audit` as a pre-tag gate.

## Technical Decisions

### Shell-first orchestration via `run-generator`

The `/audit` command procedure is parseable per spec 022's conventions — numbered steps, backtick-quoted primitive names. Each family check appears as a numbered step that invokes `run-generator` with the corresponding shell script path. The runtime's `run-generator` primitive already handles spawn-with-`--dry-run`, capture stdout/stderr/exit, and reports `drift: true` on non-zero exit (extended for `/audit` to also report drift on stdout content even when exit is 0, so a script that prints findings but returns 0 still surfaces).

Why not add new `/audit`-specific primitives:

- Each family check is a one-off (registry equivalence, manifest parity, etc.) with no obvious reuse across other commands. New primitives would carry the maintenance burden of testing, schema definition, and MCP wiring without earning their keep.
- `run-generator` already exists and already understands the contract `/audit` needs.
- Shell scripts under `scripts/audit/` are easy to read, easy to modify when checks evolve, and easy to test by invoking directly. Future maintainers do not have to recompile the runtime to fix a check.

Why not a single monolithic `scripts/audit.sh`:

- Each family check has distinct logic (text diff vs JSON walk vs git log scan). A single script would be hundreds of lines with branching by check name.
- Per-family scripts keep `/audit` as a thin orchestrator over composable pieces. CI can invoke an individual family check directly when triaging a specific failure.

### Check zero: generator/lint precondition pass

Before any family check fires, `/audit` invokes nine scripts in order via `run-generator`. The order matters because some checks read generator outputs:

1. `ductus derive-dependencies` (first, so other checks see fresh `dependencies:` fields)
2. `ductus derive-references`
3. `scripts/gen-help-tables.sh --dry-run`
4. `scripts/gen-configure-mcp.sh --dry-run`
5. `scripts/gen-claude-commands.sh --check`
6. `scripts/lint-rule-filenames.sh`
7. `scripts/lint-frontmatter.sh`
8. `scripts/lint-procedure-parseability.sh`
9. `scripts/lint-tool-coverage.sh`

The first two are runtime primitives, so `scripts/audit/check-zero.sh` builds the runtime before running them.

Any non-zero exit produces a `check-zero` finding pointing at the failing script. When check-zero fails, the family checks are skipped — running them against known-stale generator output produces misleading findings. Exit code is `1`; stdout makes the cause unambiguous.

Future-proofing: when a future spec adds a new generator or lint, the author updates both `scripts/audit/check-zero.sh` (orchestrator script) AND `framework-checks.yml`'s generator-orchestration step. The audit's Family 5 (template-validate alignment) doesn't catch this — Family 1c (cross-doc claim consistency) could be extended to verify the two lists agree, but that's a follow-on enhancement, not v1 scope.

### Per-family script designs

Each family check is one shell script. The contract:

- Print findings (one per line) to stdout in the maintainer-friendly format: `FAMILY | LOCATION | MESSAGE | SUGGESTED-FIX`.
- Exit `0` when no findings, `1` when findings present.
- Read-only — no file modifications.
- Idempotent — same inputs produce same output.

The eight scripts, with implementation notes:

**`scripts/audit/cross-doc-consistency.sh`** (Family 1)
Two sub-checks composed in one script:

- 1b pipeline diagrams: extract the `draft → clarified → planned → in-progress → done` diagram block from `framework/constitution.md` §spec-lifecycle, `README.md`, and `framework/templates/project/project-readme.md`. Verify all three visit the same five states in the same order; finding per divergence.
- 1c back-edge wording: extract the back-edge sentences from §spec-lifecycle, then compare against the references in `framework/commands/amend.md` and `framework/commands/target.md`'s Status→next-action table.

**`scripts/audit/manifest-parity.sh`** (Family 2)
One sub-check:

- 2b permission set: extract Claude `permissions.allow` array from `framework/bootstrap/configure/claude.md`, extract Auggie `toolPermissions` array from `framework/bootstrap/configure/auggie.md`. Normalize each entry to `(tool, command-pattern)` per Q6's resolution (strip `^` anchor and trailing space from Auggie regexes; case-normalize). Sort both, diff; finding per asymmetric entry.

**`scripts/audit/placeholder-roundtrip.sh`** (Family 4)
`grep -rn` over `framework/commands/*.md` for hardcoded tokens that should be placeholders: `.claude/` (should be `{cli-config-dir}/`), `gov:` (should be `{project}:`), `/ductus:` (should be `/{project}:`). A few documented exceptions (e.g., `framework/commands/audit.md` itself NOT scaffolded so its self-references stay literal) are allowlisted via a comment-prefix `<!-- audit:ignore-placeholders -->` on the affected line.

**`scripts/audit/template-alignment.sh`** (Family 5)
Parse `framework/commands/analyze.md` for blocking-check sections (`### Spec integrity (blocking)`, `### Artifact completeness (blocking)`, etc.). Each check names fields it requires (`status`, `dependencies`, `Acceptance criteria`, etc.). Verify each named field appears in the corresponding template under `framework/templates/spec/`. Finding per missing template element.

**`scripts/audit/ssot-invariants.sh`** (Family 6)
Maintain a curated list of normative rules whose text should appear in one canonical location. v1 list:

- Open-question counting rule (counts top-level list items or `**Bold-prefix**` headings in `## Open Questions`).
- Status state machine: `draft → clarified → planned → in-progress → done`.
- Back-edge ownership: `/amend` owns the back-edges.

For each rule, the script grep's all framework artifacts for the canonical text plus known paraphrases. If the canonical text appears in more than one file (or paraphrases appear that should reference the canonical), surface as a finding suggesting the duplicate location reference the canonical one.

The curated list grows as drift surfaces. New entries land here, not in framework files themselves — the script is the source of truth for which rules are SSOT-tracked.

**`scripts/audit/sibling-coupling.sh`** (Family 7)
Walk every `specs/NNN-*/spec.md`. For each non-`done` spec, parse:

- Frontmatter `dependencies:` and inline markdown links to other `specs/NNN-*` paths in the body (the union).
- `## Affected Files` table rows (file paths from the first column).

For each pair `(A, B)` where A and B inline-link each other AND share at least one Affected Files row: surface as a bundling candidate. Check the suppression contract per Q5: grep the second-drafted spec's `## Resolved Questions` for the literal phrase `Why split from {A-slug}:`. If found, skip the pair. Otherwise, emit a finding with both resolution paths in the suggested-fix column (fold-one-into-other vs record-split-rationale).

Identifying "second-drafted spec": the spec with the higher NNN prefix. (Pre-026 specs all use `NNN-slug` shape; this rule is mechanically derivable from the directory name.)

**`scripts/audit/introducing-drift.sh`** (Family 8)
Use `git log --all --pretty=format:"%H %s" -- framework/commands/ scripts/` to build a rename-history catalog. Parse commit messages for rename indicators (`renamed from X to Y`, `X → Y`, etc.). Build the set of old-name tokens (`/capture`, `/elaborate`, `/validate`, etc.).

For each `done` spec's body, grep for each old-name token in code-span form (`` `/capture` ``). For each match, check the surrounding sentence for current-tense or imperative verbs (`is`, `provides`, `exposes`, `creates`, `runs`). If found, surface as a finding with the affected sentence and a past-tense rewrite suggestion (`was`, `provided`, `exposed`, `created`, `ran`).

Heuristic by design — false positives expected on first runs. The maintainer accepts or dismisses each per-spec via a small `/ductus:amend` cycle on the affected spec.

### `/audit`'s command procedure shape

`framework/commands/audit.md` follows the parseable conventions established by spec 022:

```markdown
## Instructions

1. Invoke `run-generator` against `scripts/audit/check-zero.sh`. If it
   reports drift, emit a check-zero finding to stdout and halt with exit 1.
2. Invoke `run-generator` against `scripts/audit/cross-doc-consistency.sh`;
   stream its stdout to /audit's stdout under the family header.
3. Invoke `run-generator` against `scripts/audit/manifest-parity.sh`; …
… (one numbered step per family check) …
9. Invoke `run-generator` against `scripts/audit/introducing-drift.sh`; …
10. Exit 0 if no family check produced findings; otherwise exit 1.
```

The `run-generator` primitive's existing contract (drift detection on non-zero exit) extends cleanly to `/audit`: a family script with findings exits non-zero, the primitive surfaces it as drift, `/audit` aggregates across all families.

### CI integration

**`.github/workflows/framework-checks.yml`** — a step after the lint steps:

```yaml
- name: (h) Framework self-audit
  run: bash scripts/audit/run-all.sh
```

The binary is already built earlier in the workflow; the audit reuses that build. Failure of this step fails the PR.

**`.github/workflows/runtime-release.yml`** — append a pre-tag gate after the build/test steps, before the matrix release fires:

```yaml
- name: Framework self-audit (pre-tag gate)
  run: ./runtime/target/release/ductus exec audit
```

This is belt-and-suspenders per Q4 — the PR gate above already blocks merge of any drift-bearing PR, so a tagged commit shouldn't have drift in practice; but the release-time check guards against tags pushed directly to main without PR.

The shipped `framework/templates/ci/adopter-generators.yml` is NOT modified.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/audit.md` | Create | New parseable slash command — orchestrator for the eight family checks plus the check-zero precondition pass |
| `.claude/commands/ductus/audit.md` | Generate | Regenerated by `scripts/gen-claude-commands.sh` (the pre-commit hook handles this) |
| `scripts/audit/check-zero.sh` | Create | Orchestrates the 9 generator/lint scripts as the precondition pass |
| `scripts/audit/cross-doc-consistency.sh` | Create | Family 1 — README spec-status table, pipeline diagrams, back-edge wording |
| `scripts/audit/manifest-parity.sh` | Create | Family 2 — installer file list parity, permission-set parity with normalization |
| `scripts/audit/adopter-shell-behavior.sh` | Create | Family 22 — the shipped adopter hook and generators run against a generated adopter-shaped fixture |
| `scripts/audit/placeholder-roundtrip.sh` | Create | Family 4 — hardcoded token detection in `framework/commands/*.md` |
| `scripts/audit/template-alignment.sh` | Create | Family 5 — `analyze.md` blocking checks ↔ template scaffolding |
| `scripts/audit/ssot-invariants.sh` | Create | Family 6 — duplicate normative rule text detection |
| `scripts/audit/sibling-coupling.sh` | Create | Family 7 — bundling-candidate detection with Q5 suppression-phrase support |
| `scripts/audit/introducing-drift.sh` | Create | Family 8 — current-tense old-name references in done spec bodies |
| `.github/workflows/framework-checks.yml` | Modify | Add `/audit` step after existing lint steps |
| `.github/workflows/runtime-release.yml` | Modify | Add `/audit` pre-tag gate before release matrix fires |
| `runtime/legacy-prose-commands.txt` | Modify | Remove `framework/commands/audit.md` from the legacy allowlist — the command ships parseable from day one |
| `specs/026-framework-self-audit/spec.md` | Modify | Status `clarified → planned` at the end of this phase, then `planned → in-progress` and `→ done` during implementation |

The runtime write boundary that `/ductus:implement` derives from git history will include the above plus any incidental files (CHANGELOG entries, README spec-status table regeneration via the pre-commit hook).

## Trade-offs

### Considered: add `/audit`-specific runtime primitives

Each family check could become a primitive (`audit-cross-doc`, `audit-manifest-parity`, …). The runtime would gain richer typed interfaces and the procedure would shrink. Rejected because: (a) primitives carry maintenance overhead (schema, MCP wiring, tests) that earns its keep only when reused across commands — these are one-offs; (b) shell scripts can be modified by future maintainers without recompiling the runtime; (c) primitive promotion is a documented path for when reuse surfaces. v1 ships the simpler design and lets data drive promotion.

### Considered: single bash entrypoint `scripts/audit.sh`

One script with subcommands (`scripts/audit.sh cross-doc`, etc.). Smaller filesystem footprint. Rejected because: (a) the script would be hundreds of lines with branching by family name; (b) per-family scripts let CI invoke an individual check when triaging; (c) each family's logic is distinct enough that separation is genuine, not arbitrary.

### Considered: write findings to `audit.md` artifact like `/ductus:review`'s `review.md`

A persistent on-disk report would let CI consumers parse findings programmatically. Rejected for v1 per Q3 — `/audit` runs interactively by maintainers, not as a CI parsing target. CI gates on the exit code, which is already binary. A follow-on scenario can add file output if a real consumer surfaces.

### Considered: MUST/SHOULD severity tier

Mirroring `/ductus:review`'s convention would unify the framework's audit shape. Rejected per Q3 — the heterogeneity that justifies MUST/SHOULD in `/ductus:review` (adopter risk profiles) doesn't exist for `/audit` (ductus's own internal framework, uniform risk profile). Binary severity is simpler and CI-trivial.

### Considered and then adopted: primitive-promotion check as Family 9

Scan slash command prose for deterministic steps without primitive calls, surface as primitive-candidate findings. Rejected at plan time — different shape (opportunity vs drift), requires LLM judgment, best deferred until `/audit` is live and there is data on which patterns recur — and **adopted the same day**, in `b160cfbe`, as Family 9. The objection that carried the rejection is the one the shipped design answers: `scripts/audit/primitive-promotion-candidates.sh` decides nothing about whether a step *should* be a primitive, it reports every numbered Instructions step carrying neither a backticked primitive name nor an `<!-- llm:* -->` marker, and `<!-- audit:ignore-promotion -->` is how host-responsibility prose opts out — deterministic throughout, so the binary-severity model is untouched. The spec's own §9 and AC11 are the live statement of it. This entry is kept rather than deleted because the reversal is the decision record; what is corrected is that it read as a road not taken while the family it describes was a hard release gate.

The pointer that rejection carried — *"captured in spec's Future Considerations section"* — went dead in the same commit: `0c26aa41` gave `spec.md` a `## Future Considerations` section holding exactly this item, and adopting it removed the section. Nothing in the spec or in `specs/README.md` §Future Considerations has carried it since.

### Known limitation: SSOT curated list grows by author discipline

Family 6's curated list of normative rules to track lives in `scripts/audit/ssot-invariants.sh`. New entries are added by hand when drift surfaces. This is a §design-principles violation in miniature (the framework checks something whose check-list itself depends on author discipline). Acceptable for v1 because: (a) the list is small and grows slowly; (b) the rules being tracked are the most-canonical ones (status state machine, open-question counting), which rarely change; (c) the alternative (deriving the SSOT list from artifact analysis) is itself a research problem. A future scenario can promote SSOT-detection from a hardcoded list to a derived signal.

### Known limitation: Family 8 heuristic emits false positives

Detecting current-tense vs past-tense prose is heuristic. Sentences like *"`/capture` provided…"* (already past tense) might be flagged if the verb is far from the code-span token. Sentences like *"this is how `/capture` worked"* should NOT be flagged (verb already past) but might be on first runs. The maintainer dismisses false positives per-spec via `/ductus:amend` resolution comments — over time, the heuristic's false-positive rate is itself a signal (high rate ⇒ rewrite the heuristic).

### Known limitation: new generators require manual update in two places

When a future spec adds a new generator or lint, the author updates `scripts/audit/check-zero.sh` AND the PR workflow's generator-orchestration step — `.github/workflows/framework-checks.yml`, which `048-govern-acquired-runtime` renamed from `markdown-only-pipeline.yml`. Family 5 (template-validate alignment) does not catch this — both files enumerate the same list textually. A follow-on enhancement could add a Family 1d sub-check comparing the two lists; it has not shipped, and this entry is the only place the requirement is recorded. It formerly pointed at the spec's `## Future Considerations` section, which was removed when Family 9 was adopted (see the trade-off above), so the pointer named nothing.

### Known limitation: suppression contract is literal-phrase-fragile

Family 7 suppresses bundling-candidate findings by greping for the literal phrase `Why split from {first-slug}:` in the second spec's Resolved Questions. A typo in the literal phrase fails to suppress. Mitigation: the suggested-fix in the unsuppressed finding includes the literal phrase verbatim — copy-paste from the audit output produces a working suppression entry.

## Open Questions Resolved

All 7 questions resolved at clarify. No new questions surfaced during planning.

- Adopter-facing or maintainer-only? → Maintainer-only in v1.
- Consume runtime primitives? → Defer per-check to plan; this plan opts for shell-first orchestration via `run-generator`, with new primitives deferred to follow-ons.
- Severity levels? → Binary in v1.
- CI integration? → framework-checks.yml on every PR + runtime-release.yml as pre-tag gate.
- Coupling-check ergonomics? → Both resolution paths in the suggested-fix; `Why split from {slug}:` literal-phrase suppression.
- Manifest parity normalization? → Inline in `scripts/audit/manifest-parity.sh`, not a shared script.
- Bootstrap order of checks? → Check-zero precondition pass before family checks; halt on precondition failure.
