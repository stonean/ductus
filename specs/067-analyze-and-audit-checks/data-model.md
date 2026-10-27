# 067 — Analyze and Audit Checks Data Model

The argument and result shapes of the primitives that check a spec corpus. They live in `runtime/src/schema/primitives.rs` as Rust types with kebab-case `serde` derives, and their JSON is the stable host contract, under the conventions [022's data model](../022-deterministic-runtime/data-model.md) states for every primitive: the protocol envelope, the severity tiers each finding's `severity` is drawn from, and its closing notes on paths, timestamps and shas. Which condition earns which analyze tier is the constitution's (§text-first-artifacts, Validation Severity).

## `check-artifacts` — deterministic artifact-check families for one feature

Args:

```json
{ "feature": "022-deterministic-runtime" }
```

Result:

```json
{
  "feature": "022-deterministic-runtime",
  "status": "planned",
  "findings": [
    {
      "family": "artifact-completeness",
      "severity": "blocking",
      "message": "plan.md is required at status 'planned' but does not exist",
      "path": "specs/022-deterministic-runtime/plan.md"
    }
  ],
  "clean": false,
  "skipped": [
    {
      "family": "link-adjacent-drift",
      "reason": "target-missing",
      "path": "specs/022-deterministic-runtime/scenarios/renamed.md"
    }
  ],
  "path": "specs/022-deterministic-runtime/spec.md"
}
```

The families mirror `/ductus:analyze`'s markdown-only reference, severity tiers included:

- `artifact-completeness` (blocking) — `plan.md` and `tasks.md` are required at `planned`, `in-progress` and `done`; `data-model.md` never is.
- `task-consistency` (blocking, when `tasks.md` exists) — strictly increasing task numbers and a `Done when` clause on every task.
- `scenario-consistency` (advisory) — every `scenarios/*.md` has a referencing task. At every status an unmapped scenario is a finding only when no revision of `tasks.md` ever named it, so a task pruned per §tasks-phase satisfies it, and a history that cannot be read flags nothing and records each unmapped scenario in `skipped` as `no-readable-state` (000's `scenario-without-task-visibility`; scenario [`scenario-consistency-reads-tasks-history`](scenarios/scenario-consistency-reads-tasks-history.md)).
- `review-state-drift` (blocking) — a `done` spec whose `review.md` record has `last-run` unset, `blocking: true`, or a non-zero `should-violations`; a `done` spec with no `review.md` at all is grandfathered.
- `scenario-open-questions` (blocking at `done`, advisory otherwise) — a scenario carries an unresolved question; at `done`, `/{project}:analyze --fix` reverts the spec `done → in-progress`. There is no grandfather rule, and the list is `read-spec`'s, so this finding, `check-review-gate`'s block and the count the user sees cannot disagree ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).
- `link-adjacent-drift` (advisory) — prose asserting an open state that its own sibling link's target contradicts (scenario [`link-adjacent-drift-family`](scenarios/link-adjacent-drift-family.md)).
- `criterion-path-existence` (advisory) — a filesystem path named in a `done` spec's acceptance criterion that no longer resolves (scenario [`criterion-path-existence-family`](scenarios/criterion-path-existence-family.md)).
- `body-path-existence` (advisory) — a path named in a `done` spec's body, plan, data model or scenarios that existed in this repository and no longer resolves. A path no revision held is recorded `never-existed`; a line carrying a criterion marker, a negated-creation clause or a body transition marker below is `not-a-live-claim` (scenario [`body-path-existence-family`](scenarios/body-path-existence-family.md)).
- `criterion-labels` (advisory) — a duplicate `AC{n}` within one spec, a `next-criterion` that no longer exceeds the body, and, once the spec carries `next-criterion`, an unlabelled criterion. It runs at every status and contributes nothing to `skipped`; it is the enforcement half of `label-criteria`, since a criterion typed in an editor never touches a primitive.
- `disposition-drift` (blocking) — a `done` spec whose `review.md` records undispositioned findings; silent for a record with no `dispositions:` map.

**No family judges `analysis.md`**: every family runs inside `/ductus:analyze`, whose own run is about to replace that record, so `/ductus:analyze` judges it from the record it writes. `--all` iteration stays with the caller, and the command-frontmatter-completeness family stays in the markdown-only reference, since it reads the host's command directory.

`scenario-consistency`'s **referencing-task rule is canonical for every surface that asks "does a task reference this scenario?"**: a task references a scenario when the scenario's **slug** appears in the task's heading, in any subtask's text, or in its `Done when` clause. The match is on the slug, not the `scenarios/{slug}.md` path `append-task`'s default body emits, so a hand-written task naming the scenario counts too. A surface restating the rule — as the markdown-only path must, per §runtime-host-integration — states *this* rule, or `/{project}:amend`'s reconcile pass offers a task for a scenario this family already considers mapped. `mapped_scenario_produces_no_finding` and `bare_slug_reference_satisfies_the_mapping` cover both forms.

`skipped` records each target a family could not examine, as `{family, reason, path}` over the closed reason set `target-missing` / `target-unparseable` / `no-readable-state` / `root-absent` / `ships-to-adopter` / `artifact-unreadable` / `not-a-live-claim` / `never-existed`, because a family that reads *targets* must not escalate an unreadable one into a finding ([045 — Decision-state drift detection](../045-decision-state-drift-detection/spec.md)). The one exception is `artifact-unreadable` at `done`, a blocking finding, since the subject is the spec's own artifact (scenario `unreadable-scenario-is-reported`). `clean` means `findings.is_empty()`, so the assurance lives in the pair: `clean: true` with an empty `skipped` is verified-clean, and with a non-empty one partially examined. Hosts render the entries in the Informational tier (scenario [`check-artifacts-skipped-targets`](scenarios/check-artifacts-skipped-targets.md)). A sibling whose path traverses a symlink at or below the feature directory is reported `target-unparseable` rather than followed (scenario [`sibling-symlink-trust-boundary`](scenarios/sibling-symlink-trust-boundary.md)).

### Body transition markers

The canonical list. A line of a `done` spec's body, plan, data model or scenario is not a live claim when it carries any criterion marker (045's data model), a negated-creation clause, or any of these seven phrases:

| Group | Phrases |
| --- | --- |
| Deletion | `delete`, `remove` |
| Retirement | `retire` |
| Replacement | `replaced`, `superseded` |
| Rename | `rename to`, `` ` → ` `` |

Each was measured against the done specs on its own and kept only when every line it exempted was a transition statement. Bare `→` and the past-tense auxiliaries (`was`, `were`) were not: the first exempts any line carrying a pipeline diagram, the second the history narratives a living document is meant to rewrite.

## `validate-frontmatter` — full frontmatter schema check

Args:

```json
{ "path": "specs/022-deterministic-runtime/spec.md" }
```

Result:

```json
{
  "findings": [
    { "severity": "hard-fail", "field": "status", "message": "..." }
  ],
  "clean": false
}
```

`severity` is the tier the constitution assigns to that finding's **condition**, not a fixed value for the family, and consumers render the tier the finding names; `framework/commands/analyze.md` step 2 states the host side (scenario [`frontmatter-severity-tiers-on-both-sides`](scenarios/frontmatter-severity-tiers-on-both-sides.md)).

## `traverse-deps` — verify spec dependencies and status compatibility

Args:

```json
{ "feature": "022-deterministic-runtime" }
```

Result:

```json
{
  "dependencies": [
    {
      "feature": "021-runtime-boundary",
      "exists": true,
      "status": "done",
      "compatible": true
    }
  ],
  "compatible": true,
  "cycles": []
}
```

`cycles` lists each non-trivial strongly connected component of the reachable subgraph, a cycle of two or more specs or a self-loop, as its slugs in traversal order; it is always present, and empty when the subgraph is acyclic (scenario [`traverse-deps-cycle-check`](scenarios/traverse-deps-cycle-check.md)).

## `resolve-anchor` — verify every `§<anchor>` reference resolves

Args:

```json
{ "path": "framework/constitution.md", "markers-path": null }
```

Result:

```json
{
  "references": [
    { "anchor": "runtime-boundary", "line": 459, "resolved": true, "kind": "markers" },
    { "anchor": "Workflow", "line": 51, "resolved": true, "kind": "qualified" },
    { "anchor": "Design", "line": 106, "resolved": true, "kind": "intra-document" },
    { "anchor": "Grounding", "line": 63, "resolved": true, "kind": "cross-document" }
  ],
  "unresolved": [],
  "qualified": 136,
  "intra-document": 6,
  "cross-document": 29
}
```

Scans `path` for `§<anchor>` references and resolves each against `<!-- §anchor -->` markers — `path`'s own when `markers-path` is omitted (the constitution's self-consistency check), or another file's: `/ductus:analyze` passes the constitution as `markers-path`, since a spec carries no markers of its own. `kind` records which of four a reference is, decided in the order listed (scenarios [`anchor-reference-kinds`](scenarios/anchor-reference-kinds.md) and [`pointers-into-shipped-files-are-judged`](scenarios/pointers-into-shipped-files-are-judged.md)):

- **`cross-document`** — the reference's line names exactly one document other than the markers file, and the `## Shared Files` section of `framework/bootstrap/ductus.md` maps that document to a readable source. Judged: resolved when the text after `§` begins with a heading of that source or of the citing file, or names a marker; unresolved otherwise. Also, and first, any `§` written as an explicit pair, `**§X** in` a shipped document in backticks, on any line, judged against that document's headings alone: resolved only when `X` equals one of them. Never assigned where `framework/bootstrap/ductus.md` is absent.
- **`qualified`** — any other line naming a markdown document **other than the markers file** (a backticked path, a markdown link target, or a bare `*.md` token). Excluded by construction and **counted**; the match is line-scoped.
- **`intra-document`** — the anchor names a heading in the citing file: the text after `§` begins with one of them.
- **`markers`** — a claim about the markers file.

A heading is an ATX heading line outside a ` ``` ` fenced block and outside an HTML comment, in the citing file and in a mapped source alike. Outside a pair a reference names a heading its text begins with, the heading ending where a word ends (`§Groundingx` names no `Grounding`). Only `markers` and `cross-document` references can come back unresolved. An empty `unresolved` therefore means every claim about the markers file and about a shipped document's sections resolved, never that every `§` in the file is fine. Prose naming another document without naming its file (`spec 022 §Versioning`) can be neither verified nor excluded, and stays reported.

## `check-rule-ids` — verify cited rule IDs exist and aren't deprecated

Args:

```json
{ "path": "specs/022-deterministic-runtime/spec.md", "rule-files": ["framework/rules/security-backend.md"] }
```

Result:

```json
{
  "citations": [
    { "rule-id": "SEC-AUTH-001", "found": true, "deprecated": false }
  ],
  "missing": [],
  "deprecated": [],
  "examined": 1
}
```

`examined` is the count of rule files read — the denominator `missing` is asserted against. An empty `rule-files` makes every citation `missing`, and `/{project}:analyze` treats `missing` as blocking, so `examined: 0` is how *I was given nothing to check against* stays distinct from *these IDs do not exist*.

## `derive-dependencies` / `derive-references` — the frontmatter index derivations

The two derived frontmatter indexes (scenario [`adopter-generator-promotion`](scenarios/adopter-generator-promotion.md)). Both harvest links from a spec's body — its `spec.md` alone, never a scenario — through the shared scanner, which drops frontmatter, fenced code, blockquote-prefixed lines and `## See also` sections (`## References` is deliberately not an opt-out), and they differ in what they match and write. Both accept `write` (perform the rewrite) and `staged` (limit it to specs in the pending commit). **`write` is off by default**, because the subprocess interpreter has no per-step argument binding: a writing default would make every safety-net step in `/target`, `/clarify`, `/plan` and `/implement` rewrite the corpus.

```json
{
  "drift": true,
  "updated": ["specs/017-derive-dont-ask/spec.md"],
  "unwritten": [],
  "absent": [],
  "examined": 51,
  "untracked-skipped": [],
  "cycles": [],
  "specs-root": "specs",
  "unparseable": [],
  "wrote": false
}
```

- `unwritten` — specs examined and found drifted but deliberately not written, the `staged` case. On both primitives, since a `references:` entry derives from the body *and* the `[services]` registry, so a service rename drifts specs nobody staged (scenario [`derive-references-unstaged-drift-is-reported`](scenarios/derive-references-unstaged-drift-is-reported.md)).
- `absent` — specs tracked in the index but missing from the worktree; named, and excluded from `examined`, which counts only what the run read.
- `untracked-skipped` — untracked specs are never enumerated or rewritten (spec 017, `tracked-specs-not-worktree`), and are reported so an empty `updated` cannot be read as "everything is in sync".
- `cycles` (`derive-dependencies` only) — each cycle's members sorted, the cycles sorted by least member; a single-member entry is a self-link. A domain outcome on the MCP surface; the CLI maps it to a non-zero exit for the pre-commit hook and CI.
- `registered-services` (`derive-references` only) — registry size, so "no references found" stays distinct from "no readable config".
- `unparseable` — specs whose frontmatter opens a block it never closes, so nothing was derived, and, since 2026-10-26, specs whose rewritten frontmatter would not parse as YAML, which are left byte-identical rather than written (a frontmatter already invalid before the rewrite lands here too). Reported, never repaired, and not drift; **an empty `updated` means examined-and-clean only when `unparseable` is also empty** (scenario [`derive-unparseable-frontmatter-is-reported`](scenarios/derive-unparseable-frontmatter-is-reported.md)). A file with no frontmatter at all is not unparseable.

`derive-dependencies` writes `dependencies: []` when a spec has no sibling links; `derive-references` removes the `references:` key entirely. Unifying the two would rewrite frontmatter across every spec in a corpus.

## `check-corpus-links` — relative links that resolve to nothing

Args:

```json
{ "scope": "spec-corpus" }
```

Result:

```json
{
  "broken": [
    { "path": "specs/042-widget/scenarios/retry.md", "line": 12, "target": "../041-cache/spec.md",
      "guidance": "the target resolves one directory up — write `../../041-cache/spec.md`" }
  ],
  "examined": ["specs/042-widget/spec.md", "specs/042-widget/scenarios/retry.md"],
  "skipped": [],
  "excluded-by-construction": 3,
  "shapes-skipped": 1,
  "specs-root": "specs",
  "scope": "spec-corpus"
}
```

`scope` is `spec-corpus` (every `.md` under the configured spec root, what the shipped pre-commit hook checks) or `repository` (every tracked `.md`, `/{project}:audit` Family 26's subject, enumerated from the git index). Resolution is lexical against the citing file's directory. Inline code spans are stripped before matching, fenced blocks and frontmatter are skipped, and blockquoted lines are not — that exemption is `derive-dependencies`'. `excluded-by-construction` counts the spec root's `templates/`, whose links resolve only in a scaffolded feature; `shapes-skipped` counts targets carrying `NNN`, a `{placeholder}`, a `*` or a bare `...`; on the repository scope, `excluded-by-construction` also counts every agent's generated command copies (scenario [`every-agents-generated-copies-are-excluded`](scenarios/every-agents-generated-copies-are-excluded.md)) and, in a framework checkout only — a tree carrying `framework/reference/` — the generated `.ductus/reference/` copies, whose sources are checked instead (scenario [`generated-reference-copies-are-excluded`](scenarios/generated-reference-copies-are-excluded.md)), and in every repository the project templates under `framework/templates/project/`, whose links resolve only in a scaffolded adopter, and `runtime/tests/`, whose fixtures and goldens mirror adopter content. Empty `broken` is clean only when `skipped` is empty and `examined` is not; `guidance` appears only when no subject could be established at all. Read-only, and it reports rather than repairs (scenario [`adopter-corpus-link-integrity`](scenarios/adopter-corpus-link-integrity.md)).

## `check-orphaned-references` — adopter-owned files pointing at paths that are gone

Args: none.

Result:

```json
{
  "findings": [
    {
      "referrer": "CLAUDE.md",
      "target": ".ductus/constitution.md",
      "line": 3,
      "migration": ""
    }
  ],
  "examined": ["CLAUDE.md", ".githooks/pre-commit"],
  "skipped": [],
  "matched-prefixes": [
    ".ductus/",
    ".githooks/",
    ".govern/",
    "scripts/gen-",
    "scripts/lib/",
    "specs/"
  ],
  "attribution": "watermark",
  "last-applied": "ductus-rename"
}
```

The runtime half of [027](../027-bootstrap-migration-registry/spec.md)'s `migration-chain-reference-integrity`, invoked from `/{project}:analyze` §Project-level consistency and from `/{project}`'s Pre-run Migrations at batch end (scenario [`orphaned-reference-check`](scenarios/orphaned-reference-check.md)). Read-only, and it reports rather than repairs.

- **Referrers** are the `create`-strategy files the manifest never overwrites and no migration re-points: `CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit`, and the spec root's `system.md`, enumerated in code because the **Shared Files** manifest is not installed in an adopter checkout. `system.md` is examined but never repaired (scenario [`orphan-check-adopter-authored-references`](scenarios/orphan-check-adopter-authored-references.md)).
- **Managed roots** are `.ductus/`, `.githooks/`, the configured spec root, and the **historical** roots `.govern/`, `scripts/gen-` and `scripts/lib/`, since the orphan a migration leaves carries the root before the move (scenario [`orphaned-reference-historical-roots`](scenarios/orphaned-reference-historical-roots.md)). `matched-prefixes` bounds a clean result by scope, as `examined` bounds it by subject.
- **`attribution`** says what an empty `migration` means: under `registry` (`framework/migrations.toml` readable) it names the entry whose `target_paths` covers the missing path, latest hop winning, and empty means no entry claims it; under `watermark` no registry was read — absent, or present but unparseable, which this result does not distinguish — and `last-applied` is the only migration context.
- **A pattern is not a reference** — a candidate containing `*` or `NNN` names a shape — and a path the project declares it *ships* to adopters is not local breakage.
- **A git object name is not a reference** — a path glued to a preceding `:` whose token is empty or revision-shaped (`git show abc1234:specs/x/spec.md`) names a blob; a variable, a path or a link label before the colon still marks a working-tree reference (scenario [`a-revision-qualified-path-is-not-a-working-tree-reference`](scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md)).
- **An `@` that starts a token begins a reference** — at line start or after whitespace, `@.ductus/x` is Claude Code's import form and is checked; `x@.ductus/y` is not (scenario [`an-at-path-import-is-a-reference`](scenarios/an-at-path-import-is-a-reference.md)).
- `skipped` names the referrers it could not read, so empty `findings` is clean only when `skipped` is empty.

## `check-command-flags` — a declared interface vs. the body a host substitutes

Args: none.

Result:

```json
{
  "findings": [
    {
      "command": "framework/commands/review.md",
      "direction": "flag-unsurfaced",
      "flag": "--since",
      "reason": "Flags table documents --since but argument-hint omits it, so it is never surfaced"
    }
  ],
  "examined": ["framework/commands/amend.md", "framework/commands/review.md"],
  "with-flags-table": ["framework/commands/review.md"],
  "with-argument-hint": ["framework/commands/review.md"],
  "skipped": [],
  "commands-dir": "framework/commands",
  "guidance": ""
}
```

The runtime half of [020](../020-code-review/spec.md)'s `review-flag-parsing-is-specified`, invoked from `/audit` Family 30 (`scripts/audit/command-flag-hint-parity.sh`). It checks two directions of one contract, over `framework/commands/*.md` sources rather than a host's generated copies:

- **Direction 1, `flag-unsurfaced`** — a flag a `Flags` table documents that the `argument-hint` omits, so the operator is never shown it. Only a table row's first cell counts, scanned rather than matched once since a cell may name two flags; `with-flags-table` is its denominator, so an empty `findings` means *every tabled flag is surfaced*, never *every documented flag*.
- **Direction 3, `hint-unreachable`** — a declared `argument-hint` the body never substitutes (scenario [`argument-hint-needs-a-token`](scenarios/argument-hint-needs-a-token.md)); `with-argument-hint` is its denominator.
- **Not covered: direction 2**, a body documenting an argument the hint omits, which needs a semantic reading of the prose.
- `guidance` is set when the run examined command files and found no Flags table at all. Section membership comes from the shared fence- and comment-aware scanner, so Family 30's script stays an entry point over this primitive ([§runtime-boundary](../../framework/constitution.md#runtime-boundary) principle 3).

## `check-promotion-coverage` — how much of a rules file is routed to its canonical source

Project-scoped: no feature argument.

Args:

```json
{
  "rules-file": "AGENTS.md",
  "sections": ["Workflow", "Gotchas", "Boundaries", "Design Principles"],
  "table-file": "specs/050-constitution/plan.md",
  "table-section": "Classification",
  "pointer-link": "framework/constitution.md#"
}
```

Result:

```json
{
  "rule-bearing": 121,
  "table-keyed": 98,
  "pointer-citing": 74,
  "classified": 119,
  "unclassified": 2,
  "unclassified-entries": ["append-task has two shapes that both return exit 0 while writing nothing, and neither is an error."],
  "table-rows": 98,
  "unmatched-keys": [],
  "missing-sections": [],
  "table-state": "read"
}
```

Computes `unclassified = rule-bearing − (table-keyed ∪ pointer-citing)` over committed markdown, with no authored input, and reports **all four terms**, because disagreements over this figure turn on its terms rather than on decay. `sections` is a required set with no default, and an empty one is refused with an error, since its counts would be zero by construction, not a measurement; a named section that does not occur is reported in `missing-sections`; `unmatched-keys` is how a reworded entry surfaces, a key matching an entry's bolded lead phrase by prefix in either direction after markup is normalized and one trailing parenthetical stripped; `table-state` separates `read` from `no-table`, and a supplied `table-file` that cannot be read is an error. Only top-level bullets outside fences and comments count, through the shared comment- and fence-aware bullet grammar with the top-level constraint kept local (`QUAL-DELEG-001`). `/{project}:audit` Family 38 renders the line to stderr, where it never affects the exit code (scenario [`the-promotion-coverage-line`](scenarios/the-promotion-coverage-line.md)).
