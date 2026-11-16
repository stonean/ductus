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

The families and their severity tiers are `check-artifacts`' compiled description (`ductus check-artifacts --help`), which is canonical. What each family's specifying scenario or reference adds, and the design behind it:

- `scenario-consistency` — scenario [`scenario-consistency-reads-tasks-history`](scenarios/scenario-consistency-reads-tasks-history.md), from 000's `scenario-without-task-visibility`.
- `scenario-open-questions` — at `done`, `/{project}:analyze --fix` reverts the spec `done → in-progress`. There is no grandfather rule, and the list is `read-spec`'s, so this finding, `check-review-gate`'s block and the count the user sees cannot disagree ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).
- `link-adjacent-drift` — scenario [`link-adjacent-drift-family`](scenarios/link-adjacent-drift-family.md).
- `criterion-path-existence` — a candidate whose path leaves the project lexically (a `..`) or runs through a symbolic link that resolves outside it (`BE-INPUT-004`) is never probed, and is recorded `outside-project`; the shared link check of the sibling-link family judges the link ([`sibling-symlink-trust-boundary`](scenarios/sibling-symlink-trust-boundary.md)). `body-path-existence` reports the same. Scenarios [`criterion-path-existence-family`](scenarios/criterion-path-existence-family.md) and [`status-gated-families-are-not-silent`](scenarios/status-gated-families-are-not-silent.md).
- `body-path-existence` — scenario [`body-path-existence-family`](scenarios/body-path-existence-family.md); the transition markers are below.
- `criterion-labels` — the enforcement half of `label-criteria`, since a criterion typed in an editor never touches a primitive.
- `disposition-drift` — scenario `disposition-drift-is-advisory`.
- `spec-integrity` — the deterministic items of `framework/reference/analyze.md` §Spec integrity; whether the spec holds an implementation code block is the host's judgment (analyze step 15, recorded `integrity-not-checked` under exec); scenario [`spec-integrity-and-plan-consistency-are-run`](scenarios/spec-integrity-and-plan-consistency-are-run.md).
- `plan-consistency` — a decision's rationale and a contradiction of `system.md` are the host's judgment (step 15); a `plan.md` that exists and cannot be read is recorded `artifact-unreadable`, a skip below `done` and a blocking finding at it.
- `cross-spec-impact-drift` — advisory because the gate reads the declaration only on the way to `done`; an affected spec read through a link that leaves the project is named as such, never read (066's gate classification).

**No family judges `analysis.md`**: every family runs inside `/ductus:analyze`, whose own run is about to replace that record, so `/ductus:analyze` judges it from the record it writes. `--all` iteration stays with the caller, and the command-frontmatter-completeness family stays with the host, since it reads the host's command directory.

`scenario-consistency`'s **referencing-task rule is canonical for every surface that asks "does a task reference this scenario?"**: a task references a scenario when the scenario's **slug** appears in the task's heading, in any subtask's text, or in its `Done when` clause. The match is on the slug, not the `scenarios/{slug}.md` path `append-task`'s default body emits, so a hand-written task naming the scenario counts too. A surface restating the rule states *this* rule, or `/{project}:amend`'s reconcile pass offers a task for a scenario this family already considers mapped. `mapped_scenario_produces_no_finding` and `bare_slug_reference_satisfies_the_mapping` cover both forms.

`skipped` records each target a family could not examine, as `{family, reason, path}` over the closed reason set `target-missing` / `target-unparseable` / `no-readable-state` / `outside-project` / `root-absent` / `ships-to-adopter` / `artifact-unreadable` / `not-a-live-claim` / `never-existed` / `gitignored` / `status-gated`, because a family that reads *targets* must not escalate an unreadable one into a finding ([045 — Decision-state drift detection](../045-decision-state-drift-detection/spec.md)). The one exception is `artifact-unreadable` at `done`, a blocking finding, since the subject is the spec's own artifact (scenario `unreadable-scenario-is-reported`). `clean` means `findings.is_empty()`, so the assurance lives in the pair: `clean: true` with an empty `skipped` is verified-clean, and with a non-empty one partially examined. Hosts render the entries in the Informational tier (scenario [`check-artifacts-skipped-targets`](scenarios/check-artifacts-skipped-targets.md)). A sibling whose path traverses a symlink at or below the feature directory is reported `target-unparseable` rather than followed, and so is each fixed-name file or directory in the feature directory that links outside the project, recorded under the family `outward-link` and never opened (scenario [`sibling-symlink-trust-boundary`](scenarios/sibling-symlink-trust-boundary.md)). A linked `spec.md` ends the run with that skip alone and the status `unreadable`. Which families skip at which status is in `check-artifacts`' compiled description (`ductus check-artifacts --help`).

Both lists are bounded (`BE-PAYLOAD-001`): past `[artifacts] read-size-bytes` `form` is `file`, `findings` and `skipped` are absent, each with its `-count` inline, and `lists-file` names the file holding them whole.

### Body transition markers

The canonical list. A line of an `in-progress` or `done` spec's body, plan, data model or scenario is not a live claim when it carries any criterion marker (045's data model), a negated-creation clause, or any of these seven phrases:

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
  "findings-count": 1,
  "clean": false,
  "form": "inline"
}
```

A `spec.md` run adds a finding per defective scenario, so `findings` grows with the scenarios under a spec and is bounded (`BE-PAYLOAD-001`): past `[artifacts] read-size-bytes` `form` is `file`, `findings` is absent, `findings-count` and `clean` stay inline, and `lists-file` names the file holding it whole. The directory the run lists and the files it opens beside the spec, `scenarios/`, each scenario, `review.md` and `analysis.md`, are names the code turns into paths, so each is contained before it is listed or read (the *Paths drawn from committed text* bullet of 022's plan §Path arguments): one that is or runs through a link leaving the project is a hard-fail finding keyed by its path, never listed or read. A `scenarios/` that exists and cannot be listed is a hard-fail finding saying no scenario was checked, so `clean` never reads over scenarios nobody read (`QUAL-CLAIM-001`).

`validate-frontmatter`'s compiled description (`ductus validate-frontmatter --help`) states which path takes which row and what a `spec.md` run also covers (scenario [`a-scenario-file-is-checked-against-the-scenario-schema`](scenarios/a-scenario-file-is-checked-against-the-scenario-schema.md)).

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
  "dependencies-count": 1,
  "compatible": true,
  "cycles": [],
  "cycles-count": 0,
  "form": "inline"
}
```

`cycles` lists each non-trivial strongly connected component of the reachable subgraph, a cycle of two or more specs or a self-loop, as its slugs in traversal order; it is empty when the subgraph is acyclic (scenario [`traverse-deps-cycle-check`](scenarios/traverse-deps-cycle-check.md)). `dependencies-count` and `cycles-count` count the two lists in either form. Past `[artifacts] read-size-bytes` `form` is `file`, `dependencies` and `cycles` are absent, and `lists-file` names the JSON file under `.ductus/results/` holding both whole (spec 080, AC7; `e66f79c2`); `TRAVERSE_DEPS`' description states it.

A `dependencies:` name is frontmatter text the code turns into a path, so it is contained before it is probed or opened (the *Paths drawn from committed text* bullet of 022's plan §Path arguments): a name the lexical screen refuses, one that is absolute or holds a `..` segment wherever it points, or one running through a link leaving the project is an edge with `exists` false and an empty status, which the missing-dependency finding names, and the cycle walk treats it as a sink.

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
  "references-count": 4,
  "unresolved": [],
  "unresolved-count": 0,
  "qualified": 136,
  "intra-document": 6,
  "cross-document": 29,
  "anchor-target-unjudgeable": 4,
  "anchor-target-unreadable": 0,
  "form": "inline"
}
```

The result is bounded (`BE-PAYLOAD-001`, 067 task 35): `references` and `unresolved` are inline while the result fits `[artifacts] read-size-bytes`, and past it `form` is `file`, both are absent, `references-count` and `unresolved-count` stay inline in both forms, and `lists-file` names the JSON file under `.ductus/results/resolve-anchor/{path's file stem}/` holding the two lists whole. Every caller reads them from `lists-file` in the file form: `/{project}:audit` Family 39, `/{project}:analyze` step 4 and the `ductus exec` walker (`restore_lists`).

`ductus resolve-anchor --help` is canonical for the scan: the four kinds and the order they are decided in, how a document and a heading are named, what a reference that cannot be judged carries, and what an empty `unresolved` means. `/ductus:analyze` passes the constitution as `markers-path`, since a spec carries no markers of its own. The kinds are specified by scenarios [`anchor-reference-kinds`](scenarios/anchor-reference-kinds.md), [`pointers-into-shipped-files-are-judged`](scenarios/pointers-into-shipped-files-are-judged.md) and [`shared-constitution-references-are-judged`](scenarios/shared-constitution-references-are-judged.md); a registered shared constitution's markers are read through `resolve-constitutions`' loader.

## `read-deprecation-labels` — the deprecation labels of rule files

Args: `rule-files`, the rule files to read, as `/{project}:audit` Family 40 passes every tracked `framework/rules/*.md`. The result's `labels` holds one entry per line containing `**DEPRECATED`, `unreadable` the rule files that could not be read, and `examined` the files read (scenario [`deprecation-labels-are-read-by-the-runtime`](scenarios/deprecation-labels-are-read-by-the-runtime.md); `ductus read-deprecation-labels --help` is canonical for how a label is read and graded). The result is bounded (`BE-PAYLOAD-001`, 067 task 35): `labels` and `unreadable` are inline while the result fits `[artifacts] read-size-bytes`, and past it `form` is `file`, both are absent, `labels-count` and `unreadable-count` stay inline in both forms, and `lists-file` names the JSON file under `.ductus/results/read-deprecation-labels/paths/` holding the two lists whole. Family 40 reads them from `lists-file` in the file form.

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
  "citations-count": 1,
  "missing": [],
  "missing-count": 0,
  "deprecated": [],
  "deprecated-count": 0,
  "examined": 1,
  "form": "inline"
}
```

The five lists, `citations`, `missing`, `deprecated`, `retired` and `not-installed`, grow with the citations in a file and are bounded (`BE-PAYLOAD-001`): past `[artifacts] read-size-bytes` `form` is `file`, the lists are absent, each carries its `-count` inline (`retired-count` and `not-installed-count` absent at zero), `examined` stays inline, and `lists-file` names the file holding all five whole, each present, empty or not.

`retired` and `not-installed` are each a citation's boolean and a result list, omitted when false or empty; `check-rule-ids`' compiled description states what each state means (scenario [`a-record-cites-a-removed-rule`](scenarios/a-record-cites-a-removed-rule.md) for `retired`).

The shipped-ID index behind `not-installed` is `runtime/src/primitives/shipped_rule_ids.txt`, held to `framework/rules/*.md` by a test (spec 033, scenario `an-out-of-surface-citation-resolves`); `/{project}:analyze` records each such citation unexamined under `rule-not-installed`.

`examined` is the count of rule files read — the denominator `missing` is asserted against. An empty `rule-files` makes every citation `missing`, and `/{project}:analyze` treats `missing` as blocking, so `examined: 0` is how *I was given nothing to check against* stays distinct from *these IDs do not exist*.

## `derive-dependencies` / `derive-references` — the frontmatter index derivations

The two derived frontmatter indexes (scenario [`adopter-generator-promotion`](scenarios/adopter-generator-promotion.md)). Both harvest links from a spec's body — its `spec.md` alone, never a scenario — through the shared scanner, which drops frontmatter, fenced code, blockquote-prefixed lines and `## See also` sections (`## References` is deliberately not an opt-out), and they differ in what they match and write. The `write` and `staged` flags, each result field's meaning and the CLI exit statuses are in the primitives' compiled descriptions (`ductus derive-dependencies --help`, `ductus derive-references --help`). **`write` is off by default**, because the subprocess interpreter has no per-step argument binding: a writing default would make every safety-net step in `/target`, `/clarify`, `/plan` and `/implement` rewrite the corpus.

```json
{
  "drift": true,
  "updated": ["specs/017-derive-dont-ask/spec.md"],
  "updated-count": 1,
  "unwritten": [],
  "unwritten-count": 0,
  "examined": 51,
  "untracked-skipped": [],
  "untracked-skipped-count": 0,
  "absent": [],
  "absent-count": 0,
  "cycles": [],
  "cycles-count": 0,
  "unparseable": [],
  "unparseable-count": 0,
  "specs-root": "specs",
  "wrote": false,
  "form": "inline"
}
```

- Each result field is as the compiled descriptions state: `unwritten` (scenario [`derive-references-unstaged-drift-is-reported`](scenarios/derive-references-unstaged-drift-is-reported.md): a `references:` entry derives from the body *and* the `[services]` registry, so a service rename drifts specs nobody staged), `absent`, `untracked-skipped`, `cycles`, `registered-services`, `unparseable` (scenario [`derive-unparseable-frontmatter-is-reported`](scenarios/derive-unparseable-frontmatter-is-reported.md)) and `unwritable`, the last with the shipped hook's `--staged` stop and the CLI's exit (tasks 14 and 25; owner, 2026-11-10 and 2026-11-11).
- Both results are bounded (`BE-PAYLOAD-001`). Past `[artifacts] read-size-bytes` each moves its lists to the file `lists-file` names, each count staying inline and `unwritable-count` absent at zero, and the CLI gates and reports on the whole derivation, so an unwritable spec moved to the file still fails the run and is still named on stderr. `derive-dependencies` takes the bound from 080 (`specs/080-deterministic-runtime-efficiency/data-model.md` §Bounded results), `derive-references` from task 32 (scenario [`derive-references-results-are-bounded`](scenarios/derive-references-results-are-bounded.md)).

- **A spec another writer changed between its read and its write is `refused`, not written** (`BE-TXN-002`; task 36; owner, 2026-11-16). Under `write`, each primitive reads a spec through `ReadSnapshot` and writes its rewrite through `write_if_unchanged`: the file is read again under the project-wide lock just before the rename, and a spec that differs is left as the other writer made it. It is named in `refused` (`refused-count` beside it, both absent at zero; absent from the inline result in the file form, in the lists file whole), is never in `updated`, `unwritten`, examined-and-clean or in sync, and its derived index is still stale. The walk goes on, so the other drifted specs are written. The CLI exits non-zero on `refused-count`, report-only never reaching it, and names each refused spec on stderr from the whole derivation; the pre-commit hooks run these under `set -e`, so the commit stops and the next run reads the spec as it now is (`BE-TXN-004`). The lock, its order and bounds, and the residual for a writer that does not go through the guard are `specs/022-deterministic-runtime/plan.md` §Compare before persist.
- A tracked spec is a path the index names, so it is contained before it is read or rewritten (the *Paths drawn from committed text* bullet of 022's plan §Path arguments): one that is or runs through a link leaving the project is never opened, and `--write` never replaces it with a regular file holding the outside content. Each primitive names it in `unparseable`, so it does not read as examined or in sync. A registered service's checkout, which `derive-references` reads for its `specs-root`, lies outside the project by contract (specs 030 and 040), so its path is the config-sourced boundary of `validate_no_traversal` and is not contained; only the fixed name `.ductus/config.toml` is read from it.
- A spec that links siblings and carries no `dependencies:` key is `unwritable` in `derive-dependencies`: the splice leaves where the key belongs to the author, so the index cannot be written here, and the spec does not read as in sync (`QUAL-CLAIM-001`).

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

`scope` is `spec-corpus` (every `.md` under the configured spec root, what the shipped pre-commit hook checks) or `repository` (every tracked `.md`, `/{project}:audit` Family 26's subject, enumerated from the git index). `check-corpus-links`' compiled description (`ductus check-corpus-links --help`) states how links resolve, what is excluded by construction and counted, and when an empty `broken` is clean; the generated-copy exclusions are traced to scenarios [`every-agents-generated-copies-are-excluded`](scenarios/every-agents-generated-copies-are-excluded.md) and [`generated-reference-copies-are-excluded`](scenarios/generated-reference-copies-are-excluded.md). Read-only, and it reports rather than repairs (scenario [`adopter-corpus-link-integrity`](scenarios/adopter-corpus-link-integrity.md)). Links are contained to the repository's work tree when the project sits in a git repository (a project in a subdirectory links up into its repository, spec 059), else to the project root (`BE-INPUT-004`): a target that leaves that base lexically, or whose path runs through a symbolic link resolving outside it (the shared link check, as `check-artifacts`' path probes), is reported in `broken` with guidance saying so and is never probed. A tracked markdown file, or a directory the spec-corpus walk would list, that is or lies under such a link is never read or listed and is recorded in `skipped` with reason `outward-link`. A spec root that is, or lies under, such a link is not listed either, and one `skipped` entry names it. A target inside the base resolves lexically as before.

The three lists are bounded (`BE-PAYLOAD-001`): past `[artifacts] read-size-bytes` `form` is `file`, `broken`, `examined` and `skipped` are absent, each with its `-count` inline, and `lists-file` names the file holding them whole.

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
  "findings-count": 1,
  "examined": ["CLAUDE.md", ".githooks/pre-commit"],
  "examined-count": 2,
  "skipped": [],
  "skipped-count": 0,
  "matched-prefixes": [
    ".ductus/",
    ".githooks/",
    ".govern/",
    "scripts/gen-",
    "scripts/lib/",
    "specs/"
  ],
  "attribution": "watermark",
  "last-applied": "ductus-rename",
  "form": "inline"
}
```

The runtime half of [027](../027-bootstrap-migration-registry/spec.md)'s `migration-chain-reference-integrity`, invoked from `/{project}:analyze` §Project-level consistency and from `/{project}`'s Pre-run Migrations at batch end (scenario [`orphaned-reference-check`](scenarios/orphaned-reference-check.md)). Read-only, and it reports rather than repairs.

- **Referrers** are the adopter-owned files `check-orphaned-references`' compiled description names, enumerated in code because the **Shared Files** manifest is not installed in an adopter checkout. `system.md` is examined but never repaired (scenario [`orphan-check-adopter-authored-references`](scenarios/orphan-check-adopter-authored-references.md)).
- **Managed roots** are `.ductus/`, `.githooks/`, the configured spec root, and the **historical** roots `.govern/`, `scripts/gen-` and `scripts/lib/`, since the orphan a migration leaves carries the root before the move (scenario [`orphaned-reference-historical-roots`](scenarios/orphaned-reference-historical-roots.md)). `matched-prefixes` bounds a clean result by scope, as `examined` bounds it by subject.
- **`attribution`** and `last-applied` are as `ductus check-orphaned-references --help` states: `registry` or `watermark`, and what an empty `migration` means under each.
- **What counts as a reference** is `ductus check-orphaned-references --help`'s: the token-start rule, the `@` import form, the prefixes that name the project root and the ones that do not, and the patterns, parent-directory paths, git object names and shipped paths that are never reported (scenarios [`a-revision-qualified-path-is-not-a-working-tree-reference`](scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md) and [`an-at-path-import-is-a-reference`](scenarios/an-at-path-import-is-a-reference.md)). A leading `/` is read from the project root even in a project below its repository's root (spec 059), as `check-corpus-links` reads a root-absolute link. The span still starts at the managed root, so `rewrite-spec-links` keeps the prefix as written (`051-branch-scoped-spec-numbering` task 30).
- `skipped` is as `ductus check-orphaned-references --help` states: empty `findings` is clean only when it is empty. It also names a referrer, or a managed path a referrer names, that is or runs through a link leaving the project, which is never read or probed (the *Paths drawn from committed text* bullet of 022's plan §Path arguments).
- The three lists are bounded (`BE-PAYLOAD-001`): past `[artifacts] read-size-bytes` `form` is `file`, `findings`, `examined` and `skipped` are absent, each with its `-count` inline, and `lists-file` names the file holding them whole.

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
  "findings-count": 1,
  "examined-count": 2,
  "with-flags-table-count": 1,
  "with-argument-hint-count": 1,
  "skipped-count": 0,
  "commands-dir": "framework/commands",
  "guidance": "",
  "form": "inline"
}
```

The runtime half of [020](../020-code-review/spec.md)'s `review-flag-parsing-is-specified`, invoked from `/audit` Family 30 (`scripts/audit/command-flag-hint-parity.sh`). It checks two directions of one contract, over `framework/commands/*.md` sources rather than a host's generated copies:

- **Direction 1, `flag-unsurfaced`** — a flag the `argument-hint` omits, as `check-command-flags`' compiled description states, with `with-flags-table` as its denominator.
- **Direction 3, `hint-unreachable`** — a declared `argument-hint` the body never substitutes (scenario [`argument-hint-needs-a-token`](scenarios/argument-hint-needs-a-token.md)); `with-argument-hint` is its denominator.
- **Not covered: direction 2**, a body documenting an argument the hint omits, which needs a semantic reading of the prose.
- The five lists are bounded (`BE-PAYLOAD-001`): past `[artifacts] read-size-bytes` `form` is `file`, they are absent, each with its `-count` inline, and `lists-file` names the file holding them whole; Family 30's script reads them there. A command directory, or a source in it, that is or runs through a link leaving the project is never listed or read and is a `skipped` entry (the *Paths drawn from committed text* bullet of 022's plan §Path arguments).
- `guidance` is as `ductus check-command-flags --help` states. Section membership comes from the shared fence- and comment-aware scanner, so Family 30's script stays an entry point over this primitive ([§runtime-boundary](../../framework/constitution.md#runtime-boundary) principle 3).

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
  "unclassified-entries-count": 1,
  "unmatched-keys-count": 0,
  "missing-sections": [],
  "missing-sections-count": 0,
  "table-state": "read",
  "form": "inline"
}
```

Computes `unclassified = rule-bearing − (table-keyed ∪ pointer-citing)` over committed markdown, with no authored input, and reports **all four terms**, because disagreements over this figure turn on its terms rather than on decay. `sections` is a required set with no default, and an empty one is refused with an error, since its counts would be zero by construction, not a measurement; a named section that does not occur is reported in `missing-sections`; `unmatched-keys` is how a reworded entry surfaces, a key matching an entry's bolded lead phrase by prefix in either direction after markup is normalized and one trailing parenthetical stripped; `table-state` separates `read` from `no-table`, and a supplied `table-file` that cannot be read is an error. Only top-level bullets outside fences and comments count, through the shared comment- and fence-aware bullet grammar with the top-level constraint kept local (`QUAL-DELEG-001`). `/{project}:audit` Family 38 renders the line to stderr, where it never affects the exit code (scenario [`the-promotion-coverage-line`](scenarios/the-promotion-coverage-line.md)). The three lists, `unclassified-entries`, `unmatched-keys` and `missing-sections`, are bounded (`BE-PAYLOAD-001`): past `[artifacts] read-size-bytes` `form` is `file`, they are absent, each with its `-count` inline, the four terms stay inline, and `lists-file` names the file holding them whole, which the Family 38 script reads there.
