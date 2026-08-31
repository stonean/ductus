# 054 — Remove supersession Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Consolidate 053 into 052

- [x] Run `/{project}:consolidate` with 053 as the source and 052 as the target, while `consolidate.md` is still intact
- [x] Confirm the prompt names 053's content loss and that no `supersedes:` edge is reported (none exists in the corpus)
- [x] Verify `rewrite-spec-links` re-pointed this spec's own body link from 053 to 052, and that `specs/053-supersession-reconciliation/` is gone
- [x] Commit before anything else, so the pre-commit hook regenerates `dependencies:` from the rewritten links

- **Done when**: `specs/053-supersession-reconciliation/` does not exist, no file in the tree links to it, and the corpus has no dangling pointer.

## 2. Edit 052 down and return it to `done`

- [x] `set-status` 052 from `done` to `in-progress` (meaningful body edit, §spec-lifecycle)
- [x] Delete its supersession sections and every acceptance criterion whose deliverable this change removes; leave `next-criterion` alone so retired labels are never reissued
- [x] Retitle the H1 to name consolidation alone; leave the directory name unchanged
- [x] Confirm what remains describes `/{project}:consolidate` truthfully and nothing else
- [x] `set-status` 052 back to `done`

- **Done when**: 052 is `done`, mentions supersession nowhere, and its surviving criteria all describe things that still exist.

## 3. Replace the `/specify --supersedes` worked example — its own commit

- [x] Substitute `/specify --fold-into` for `` `/specify --supersedes` `` in `scripts/audit/readme-command-parity.sh`, `scripts/audit/README.md`, and `specs/026-framework-self-audit/scenarios/link-check-consolidation.md`
- [x] Confirm the replacement is a flag on the same command inside a wider code span, so it still illustrates Family 33's matching property
- [x] Commit this substitution alone, bundled with nothing else

- **Done when**: the three files carry one uniform substitution in a single commit, and 026 is still `done`.

## 4. Remove the command and rewrite what pointed at it

- [x] Delete `framework/commands/supersede.md`
- [x] `specify.md`: drop `[--supersedes <feature>]` from the `argument-hint:` frontmatter, the flag-table row, the second-spec scope boundary, step 5, step 8, the supersession answer in step 3, the body-link warning in step 7, the markdown-only **Declare a supersession** section, and both `ductus exec` reduction notes
- [x] `consolidate.md`: drop the decision-table row, step 3 and its **Settling `supersedes:` edges** reference, the `supersedes:` mentions in steps 1/4/5/8, and the exec note that step 3 does not run; rewrite the ship/no-ship guidance to ask whether the earlier spec still describes something true
- [x] `analyze.md`: nine residual families → eight; drop the reciprocity entry, its reference section, and the paragraph bounding what its silence means
- [x] `help.md`: drop the command row
- [x] Confirm no command source defers to `supersede.md` for Declaration semantics

- **Done when**: `grep -ri supersede framework/commands/` returns nothing and every surviving command file reads coherently without the deleted statements.

## 5. Constitution surgery

- [x] Remove `§supersession-annotations` whole, including its `<!-- §supersession-annotations -->` anchor comment
- [x] Remove the criterion-level supersession bullet from `§spec-requirements` — it carries the cross-reference that would otherwise strand the anchor
- [x] Remove the `supersedes` row from the spec frontmatter schema table
- [x] Confirm no remaining text references the removed anchor

- **Done when**: `grep -n "supersession-annotations" framework/constitution.md` returns nothing and the surrounding sections still read continuously.

## 6. Bootstrap manifest and tool manifest

- [x] Drop the `framework/commands/supersede.md` row from the slash-command manifest in `framework/bootstrap/ductus.md` and `framework/bootstrap/govern.md`
- [x] Correct the hardcoded "seventeen `framework/commands/*.md` rows" to sixteen in both files, in both the skills and OpenCode sections
- [x] Remove `read-supersession-pair` and `write-supersession-annotation` from `framework/runtime-tools.txt`
- [x] Add no `framework/migrations.toml` entry and no procedure file

- **Done when**: the manifest lists sixteen command rows, every count in both bootstrap files agrees, and the migration registry is untouched.

## 7. Remove the two primitives and their five registration sites each

- [x] Delete `runtime/src/primitives/write_supersession_annotation.rs` and `read_supersession_pair.rs`, and their `pub mod` lines
- [x] Remove both from the CLI command enum and dispatch arms in `runtime/src/main.rs`, the exec-path match arms in `runtime/src/interpreter/mod.rs`, the `#[tool]` definitions in `runtime/src/mcp/server.rs`, and `PRIMITIVE_REGISTRY` in `runtime/src/schema/registry.rs`
- [x] Remove their args and result types from `runtime/src/schema/primitives.rs`, including the `criterion` granularity argument
- [x] Remove the `classifyClaims` extension point: `SupersededClaim`, `ClassifyClaimsRequest` and its response type in `runtime/src/schema/extensions.rs`, and `build_classify_claims_request` in `runtime/src/interpreter/payload.rs`
- [x] Run `cargo test --test mcp` before anything else — it prints the exact manifest/registry divergence on a half-removal

- **Done when**: `cargo test --test mcp` passes with the manifest set-equal to the registry, and no extension point remains without a caller.

## 8. Remove the validator, the check family, and the shared predicate

- [x] Remove `validate_supersedes`, its call site, and its five tests from `runtime/src/primitives/validate_frontmatter.rs`
- [x] Remove `check_supersession_reciprocity`, its call site, and the `supersession-reciprocity` family tests from `runtime/src/primitives/check_artifacts.rs`
- [x] Remove `blockquote_cites` from `runtime/src/primitives/mod.rs` — both its callers are now gone
- [x] Correct the two `read_spec.rs` comments that name `read-supersession-pair` as a second caller
- [x] Leave the blockquote exclusion in `runtime/src/primitives/spec_links.rs` untouched — it serves four other primitives and predates supersession

- **Done when**: `validate-frontmatter` reports `clean` on a spec carrying a `supersedes:` key, `check-artifacts` returns eight residual families, and `spec_links.rs` is unchanged.

## 9. Trim the tests and re-bless the golden

- [x] Delete the three supersede tests from `runtime/tests/two_spec_commands.rs`; keep `consolidate_confirms_before_it_rewrites_or_removes_anything`, `fold_never_reaches_the_sequential_opt_in`, and the shared helpers
- [x] Re-bless `runtime/tests/golden/specify-basic.jsonl` and read the diff to confirm it is the flag and the two declaration steps leaving, nothing else
- [x] `cargo build`, `cargo clippy`, and the full `cargo test` under `runtime/`

- **Done when**: the suite passes with no dead-code or unused-import warnings, and the golden diff is attributable only to `specify.md`'s rewrite.

## 10. Sweep docs, README, scripts, and AGENTS.md

- [x] `docs/slash-commands.md`: delete the `/supersede` section, both "reach for `/supersede` instead" callouts under `/consolidate`, and the `--supersedes` clause in `/specify`'s flag line; restate the flag-versus-command paragraph as the rule without its exception, naming `/fold` and `/consolidate`
- [x] `README.md`: delete the `/supersede` bullet and its deep link into `docs/slash-commands.md`
- [x] `scripts/gen-help-tables.sh`: drop the `/{project}:supersede` entry
- [x] `AGENTS.md`: retarget the five-registration-sites gotcha at a primitive that still exists, keeping the lesson
- [x] Leave ordinary-English uses of the verb alone — the pre-commit hooks, `framework/rules/security-backend.md`, `review.md`'s report-regeneration line

- **Done when**: sixteen commands are documented with no `/supersede` among them, and every ordinary-English use of the verb is unchanged.

## 11. Version bump and changelog

- [x] Bump `runtime/Cargo.toml` and `version` from `0.41.0` to `0.42.0`
- [x] Add the `runtime/CHANGELOG.md` 0.42.0 section recording the removal as breaking: both MCP tools, both CLI subcommands, the `classifyClaims` extension point, and the frontmatter key that stops being validated
- [x] `cargo build --release` to refresh the parity binary

- **Done when**: the changelog names every removed surface and no golden diff is attributable to the version line.

## 12. Regenerate, verify, and audit

- [x] Run `scripts/gen-configure-mcp.sh` **then** `scripts/gen-claude-commands.sh`, in that order
- [x] Run `scripts/gen-help-tables.sh` and confirm the command tables regenerate to sixteen rows
- [x] Confirm `framework/bootstrap/configure/claude.md` and `configure/auggie.md` carry no supersession tool permissions and reproduce byte-for-byte
- [x] Run `check-corpus-links` and `check-orphaned-references` across the corpus
- [x] Run `/{project}:audit` and confirm zero findings, including Family 16 and Family 33
- [x] Grep the whole tree for `/{project}:supersede`, `--supersedes`, `supersedes:`, `§supersession-annotations`, `write-supersession-annotation`, and `read-supersession-pair`, and confirm every remaining hit is ordinary English

- **Done when**: `/{project}:audit` reports zero findings, both link checks are clean, and no live artifact names a removed capability.
