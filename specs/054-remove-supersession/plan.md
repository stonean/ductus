# 054 — Remove supersession Plan

Implements [054 — Remove supersession](spec.md).

## Overview

A removal in five ordered movements: settle the corpus first (053 and 052), then take out the framework text, then the runtime, then sweep the names, then bump and release. The ordering is not cosmetic — the corpus disposition uses `/{project}:consolidate`, which reads command text this change is about to rewrite, and the runtime removal has a test that fails loudly on a half-finished registration but only if it is run first.

Nothing new is built. There is no migration, no replacement command, and no new check. The one thing that gets *written* rather than deleted is prose: `consolidate.md`'s ship/no-ship guidance and `docs/slash-commands.md`'s flag-versus-command paragraph both lose an arm and need restating rather than excising.

## Technical Decisions

### Corpus first, framework second

The 053 → 052 consolidation runs before any command source is edited. `/{project}:consolidate` reads `consolidate.md`'s own procedure to know what to do, and step 3 of that procedure enumerates `supersedes:` edges across the corpus — a step this change deletes. Running the consolidation while the command is still coherent avoids driving a half-rewritten procedure, and the step is a no-op in practice because no spec carries a `supersedes:` key (verified: the only occurrence in the tree is an illustrative example inside `specs/053-supersession-reconciliation/data-model.md:8`).

**`/{project}:consolidate` migrates no content**, which is what makes this cheap. It re-points every inbound body link and removes the directory; it does not copy 053's prose into 052. So "053 is consolidated into 052" means 053 stops existing and its pointers land on 052 — not that 052 absorbs text this change would immediately delete. `consolidate.md`'s own scope note is explicit that the command does not verify the target covers what the source said, and deliberately does not try.

One consequence worth naming: this spec's body links to `../053-supersession-reconciliation/spec.md`, so `rewrite-spec-links` re-points it to 052 as part of the consolidation. That is the intended outcome and the reason 052 was chosen as the target over 054 — consolidating into 054 would have rewritten that link into a self-link, and `derive-dependencies` records self-links rather than stripping them, specifically so the cycle check surfaces them.

### 052 is edited down, not annotated

052 reopens `done → in-progress` (meaningful body edit, §spec-lifecycle), loses its supersession sections and the acceptance criteria whose deliverables no longer exist, keeps its consolidation content, and returns to `done` when this spec lands.

Deleting ticked criteria is safe rather than destructive because `next-criterion` is monotonically non-decreasing and never reissues a retired label — the counter exists for exactly this. That mechanism survives this change and is what replaces the criterion-level annotation rule being removed.

The directory keeps its name. Directory names are stable numbered identifiers that inbound links and git history resolve through; renaming would re-point every pointer across the corpus for cosmetic gain, and would make this change carry two directory rewrites instead of one. Only the H1 is retitled.

### `cargo test --test mcp` runs first, and the generators run in order

Registering a runtime primitive is five sites and **de-registering it is the same five**: the CLI command enum plus its dispatch arm in `runtime/src/main.rs`, the exec-path match arm in `runtime/src/interpreter/mod.rs`, the `#[tool]` in `runtime/src/mcp/server.rs`, `PRIMITIVE_REGISTRY` in `runtime/src/schema/registry.rs`, and `framework/runtime-tools.txt`. `runtime/tests/mcp.rs`'s `lists_every_manifest_tool_and_canonical_set` asserts the manifest is set-**equal** to the registry, so it names the exact divergence with both sets printed — which makes it the fastest signal for a half-removal and the reason it runs before anything else.

Then `scripts/gen-configure-mcp.sh` **followed by** `scripts/gen-claude-commands.sh`, in that order: the second renders command copies from what the first writes, and reversing them leaves a stale `.claude/commands/ductus/configure.md` that fails `/{project}:audit`'s check-zero precondition. This is the sequence `AGENTS.md` records from spec 052's own implementation, applied in reverse.

### `two_spec_commands.rs` is trimmed, not deleted

The file holds five tests. Three are supersede's — `supersede_confirms_before_it_annotates_the_other_spec`, `supersede_reports_all_three_reconciliation_outcomes`, and `nothing_between_classification_and_the_report_can_resolve_a_conflict` — and go with the command. Two survive and must keep passing: `consolidate_confirms_before_it_rewrites_or_removes_anything` and `fold_never_reaches_the_sequential_opt_in`. Its `workspace_root`/`procedure`/`steps`/`position` helpers stay; deleting the file would take the two surviving assertions with it.

### The golden needs re-blessing, and only for the right reason

`runtime/tests/golden/specify-basic.jsonl` embeds `specify.md`'s command text, which this change rewrites, so the golden legitimately changes. Re-bless it for that and read the diff to confirm it is the flag and the two declaration steps leaving — never for a version line. 043 established the same rule for its own bump.

### No migration

The revised AC16 is a negative requirement and the plan honors it as one: no `framework/migrations.toml` entry, no procedure file. Dropping the manifest row in `framework/bootstrap/ductus.md` and `framework/bootstrap/govern.md` is the whole adopter-side change, because `enforce-manifest` prunes anything in the per-layout slash-command cleanup glob that is neither in the manifest nor pinned. The spec's **Adopter cleanup** section carries the per-layout table and the reasoning; it is not restated here.

### The sweep is one uniform commit

§drift-prevention makes the name sweep a mechanical edit — `done` specs stay `done` — but only while the diff stays uniform. The `` `/specify --supersedes` `` worked example appears in three places (`scripts/audit/readme-command-parity.sh:46`, `scripts/audit/README.md:73`, and `specs/026-framework-self-audit/scenarios/link-check-consolidation.md:13`) and is replaced by a surviving flag in a single substitution across all three, landing on its own rather than bundled into the broader rewrite. `/specify --fold-into` is the replacement: it is a flag on the same command, inside a wider code span, so it exercises the exact matching property the example exists to illustrate.

Ordinary-English uses of the verb are left alone — the pre-commit hooks, `framework/rules/security-backend.md`'s CSP note, and `review.md`'s report-regeneration line name no capability, and rewriting them would be the non-uniform diff the rule warns about.

### Constitution surgery

Three excisions, one of which is not a deletion. `§supersession-annotations` goes whole, including its `<!-- §supersession-annotations -->` anchor comment. The `supersedes` row leaves the spec frontmatter schema table. The criterion-level bullet in `§spec-requirements` goes — and it is the one that carries a cross-reference (`This is the criterion-level case of the rule below (§supersession-annotations)`), so removing the section without removing the bullet strands the anchor that `check-corpus-links` will then report.

`runtime/src/schema/primitives.rs:1707` also cites the anchor in a doc comment, but on a type this change removes, so it leaves with its type.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `specs/053-supersession-reconciliation/` | Remove | Consolidated into 052; pointers re-pointed |
| `specs/052-spec-supersession-and-consolidation/spec.md` | Modify | Drop supersession sections and their criteria; retitle H1; keep consolidation |
| `framework/commands/supersede.md` | Remove | The command |
| `framework/commands/specify.md` | Modify | `argument-hint` frontmatter, flag table, scope boundary, steps 3/5/7/8, markdown-only declaration section |
| `framework/commands/consolidate.md` | Modify | Decision table, ship/no-ship guidance, step 3 and its reference section, `supersedes:` mentions in steps 1/4/5/8 |
| `framework/commands/analyze.md` | Modify | Family enumeration nine → eight; drop reciprocity reference section |
| `framework/commands/help.md` | Modify | Drop the command row |
| `framework/constitution.md` | Modify | Remove `§supersession-annotations`, the `§spec-requirements` bullet, the schema row |
| `framework/bootstrap/ductus.md` | Modify | Manifest row; "seventeen" → "sixteen" |
| `framework/bootstrap/govern.md` | Modify | Same two edits |
| `framework/bootstrap/configure/claude.md` | Modify | Drop two tool permissions (generated) |
| `framework/bootstrap/configure/auggie.md` | Modify | Drop two tool permissions (generated) |
| `framework/runtime-tools.txt` | Modify | Drop two tool names |
| `runtime/src/primitives/write_supersession_annotation.rs` | Remove | Primitive |
| `runtime/src/primitives/read_supersession_pair.rs` | Remove | Primitive |
| `runtime/src/primitives/mod.rs` | Modify | Two `pub mod` lines; `blockquote_cites` |
| `runtime/src/main.rs` | Modify | CLI enum + two dispatch arms |
| `runtime/src/interpreter/mod.rs` | Modify | Two exec-path match arms |
| `runtime/src/mcp/server.rs` | Modify | Two `#[tool]` definitions |
| `runtime/src/schema/registry.rs` | Modify | Two `PRIMITIVE_REGISTRY` entries |
| `runtime/src/schema/primitives.rs` | Modify | Args/result types for both primitives |
| `runtime/src/schema/extensions.rs` | Modify | `classifyClaims` types |
| `runtime/src/interpreter/payload.rs` | Modify | `build_classify_claims_request` |
| `runtime/src/primitives/validate_frontmatter.rs` | Modify | `validate_supersedes`, call site, tests |
| `runtime/src/primitives/check_artifacts.rs` | Modify | `check_supersession_reciprocity`, call site, family tests |
| `runtime/src/primitives/read_spec.rs` | Modify | Stale "shared with `read-supersession-pair`" comments |
| `runtime/tests/two_spec_commands.rs` | Modify | Drop three supersede tests; keep two |
| `runtime/tests/golden/specify-basic.jsonl` | Modify | Re-bless for the `specify.md` rewrite |
| `runtime/CHANGELOG.md` | Modify | Breaking entry for 0.42.0 |
| `runtime/Cargo.toml`, `version` | Modify | 0.41.0 → 0.42.0 |
| `docs/slash-commands.md` | Modify | Drop the section; restate the flag-versus-command paragraph; two `/consolidate` callouts |
| `README.md` | Modify | Drop the bullet and its deep link |
| `scripts/gen-help-tables.sh` | Modify | Drop the command entry |
| `scripts/audit/readme-command-parity.sh` | Modify | Worked example |
| `scripts/audit/README.md` | Modify | Worked example |
| `specs/026-framework-self-audit/scenarios/link-check-consolidation.md` | Modify | Worked example (mechanical; 026 stays `done`) |
| `AGENTS.md` | Modify | Five-sites gotcha cites a surviving primitive |

## Trade-offs

**Rejected: keeping `supersedes:` as a documented-but-inert key.** It would leave the schema advertising a relation nothing records, reads, or checks — the exact false-document failure this spec exists to remove, relocated into the frontmatter table.

**Rejected: adding a migration.** Slash-command cleanup already removes the installed command in all three layouts, honors `[pinned] files`, and is silent when nothing is stale. An entry would duplicate that and hand `/{project}:audit` Family 10 a `target_paths` list to police. The two existing migrations are not a precedent: both clean artifacts outside the enforced set.

**Rejected: consolidating 053 into 054.** It would put the whole removal story in one spec, but `rewrite-spec-links` would turn this spec's own body link to 053 into a self-link, which `derive-dependencies` records rather than strips.

**Rejected: renaming 052's directory to match its narrowed subject.** Cosmetic gain against re-pointing every inbound pointer and a second directory rewrite in one change.

**Rejected: deleting `runtime/tests/two_spec_commands.rs`.** Two of its five tests cover `/consolidate` and `/fold` and must keep passing.

**Limitation: an adopter who never re-runs `/{project}` keeps a stale command file.** True of any removal, and a migration would not have helped — migrations also only run during a bootstrap pass.

**Limitation: the sweep's uniformity is an authoring discipline, not a check.** Nothing verifies that the worked-example substitution landed as its own commit rather than folded into a broader rewrite. `check-corpus-links` and `check-orphaned-references` prove the references resolve; they say nothing about whether 026 should have taken a back-edge.
