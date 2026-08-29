# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Phase 1 (tasks 1–5) makes branch-scoped directories creatable and visible. Phase 2 (6–8) adds the fold target and its detection. Phase 3 (9–13) adds the fold-back command. Each phase leaves the corpus in a working state.

## 1. Introduce the directory-form parse

- [ ] Add `FeatureForm` and `parse_feature_dir` to `runtime/src/primitives/mod.rs` per the [data model](data-model.md) grammar
- [ ] Redefine `is_feature_slug` in terms of it and remove `feature_number`, updating its two callers (`resolve_feature.rs`, `create_feature.rs`) to match on `Sequential`
- [ ] Unit-test both forms plus the rejections: no `.`-free branch form, leading-zero `n`, empty identifier, empty slug, uppercase input, `1234.1-slug` yielding no sequential number

- **Done when**: `parse_feature_dir` is the only place either directory form is recognized, `feature_number` no longer exists, and the existing `is_feature_slug` acceptance and rejection tests still pass unchanged.

## 2. Make both forms visible to every corpus reader

- [ ] Confirm `list_feature_dirs` and `is_spec_path` accept both forms through the new parse
- [ ] Order branch-scoped directories by identifier then `n`, sequential ones by number, with a defined total order across the mixed corpus
- [ ] Add tests over a fixture spec root holding both forms, asserting `dashboard`, `derive-dependencies`, and `derive-references` each see the branch-scoped directory

- **Done when**: a spec root containing `050-a` and `1234.1-b` yields both directories from every corpus-reading surface, in a stable documented order (AC5, AC15).

## 3. Add branch-scoped creation to `create-feature`

- [ ] Add the `branch-id` and `fold-into` arguments and the `identifier` result field to the schema
- [ ] Sanitize `branch-id` through `derive_slug`; refuse an identifier that sanitizes to empty through the existing `InvalidArgument` path
- [ ] Compute `.{n}` as `max + 1` over `BranchScoped` forms matching the sanitized identifier
- [ ] Test: first and second spec under one identifier; identifiers differing only in case landing in one namespace; `PROJ-1111` and `1111-PROJ` sanitizing as specified; an identifier containing `.`; an existing directory returning `created: false`

- **Done when**: creation with no `branch-id` produces byte-identical behavior to today, and creation with one produces `{identifier}.{n}-{slug}` with the sanitized identifier returned (AC1, AC8, AC9, AC10, AC11, AC12, AC26, AC27).

## 4. Keep the sequential counter independent

- [ ] Restrict `next_feature_number` to `Sequential` forms
- [ ] Test: a spec root of `050-a` plus `1234.1-b` yields `051-…` next; a root holding only branch-scoped directories yields `001-…`

- **Done when**: no branch-scoped directory can influence the sequential counter (AC2, AC3, AC4, AC14).

## 5. Teach feature resolution both forms

- [ ] Match sequential identifiers against `Sequential` numbers and branch identifiers against `BranchScoped` identifiers, exactly per form
- [ ] Return every match so a string naming both forms produces the existing `Ambiguous` outcome
- [ ] Test: `123` against `123-a` and `1234.1-b`; `1234` against the branch set; `051` against both `051-a` and `051.1-b`

- **Done when**: resolving `123` never matches `1234.1-…`, and an identifier naming both forms is reported ambiguous rather than silently resolved (AC16).

## 6. Add the `folds-into` frontmatter field

- [ ] Write `folds-into:` from `create-feature`'s `fold-into` argument; omit the key when the argument is absent
- [ ] Add the `validate-frontmatter` shape check: present ⇒ names an existing sequential feature
- [ ] Document the key in `framework/templates/spec/spec.md`
- [ ] Test that `set-status`, `derive-dependencies`, and `label-criteria` each leave the key byte-identical

- **Done when**: the key round-trips through every existing frontmatter writer untouched, and a `folds-into` naming a missing feature is a reported finding (AC19, AC20, AC28).

## 7. Add `check-unfolded-specs`

- [ ] Implement the primitive per the data model, reporting each branch-scoped directory with its `folds-into` and status, plus an `examined` count
- [ ] Register it in `runtime/src/schema/primitives.rs`, the MCP server, and `framework/runtime-tools.txt`
- [ ] Test: a corpus with no branch-scoped directories reports empty with a non-zero `examined`

- **Done when**: every surviving branch-scoped directory is reported with its declared fold target, and an empty result is distinguishable from an unexamined corpus (AC21).

## 8. Surface un-folded specs in `/ductus:analyze`

- [ ] Add the check to `framework/commands/analyze.md` as a reported finding
- [ ] Verify the command still parses under `runtime/src/parser/mod.rs`'s step-numbering assertions

- **Done when**: `/ductus:analyze` reports surviving branch-scoped specs, and the command-parse test passes (AC21).

## 9. Add `rewrite-spec-links`

- [ ] Implement the primitive: re-point inbound body links from a retiring feature directory at the fold target, reporting `rewritten` and `examined`
- [ ] Leave frontmatter alone — `dependencies:` and `references:` regenerate from body links via the pre-commit hook
- [ ] Test: sibling `../{feature}/spec.md` links, scenario-targeted links, and a corpus with no inbound links

- **Done when**: every inbound body link to the retiring directory points at the fold target, and no frontmatter was hand-edited to achieve it (AC22, AC23).

## 10. Add `retire-feature`

- [ ] Implement the primitive: remove a `BranchScoped` directory, refusing when the fold target does not exist, and refusing a `Sequential` feature outright
- [ ] Return `retired: false` as the domain outcome for an already-absent directory
- [ ] Test both refusals and the already-gone case

- **Done when**: a branch-scoped directory can be retired only when its fold target exists, and a sequential feature can never be retired by this primitive (AC28).

## 11. Add the fold-routing extension point

- [ ] Define the request/response types in `runtime/src/schema/extensions.rs` and the payload builder in `runtime/src/interpreter/payload.rs`
- [ ] Keep the vocabulary separate from `routeInboxItem`'s closed five-route set
- [ ] Test the payload shape, including a branch-scoped spec that carries its own scenarios

- **Done when**: the extension point returns a body-edit-or-scenario decision per branch-scoped spec, with the target section or scenario slug named.

## 12. Add the `/ductus:fold` command

- [ ] Write `framework/commands/fold.md`: enumerate branch-scoped specs, route each at the extension point, confirm with the operator, then per spec — apply the content, rewrite links, reopen a `done` upstream spec with a guarded `set-status`, and retire the directory
- [ ] Handle the empty-`folds-into` case as the renumbering path
- [ ] Add `fold` to the parser's command list in `runtime/src/parser/mod.rs`
- [ ] Regenerate the help and installer surfaces (`scripts/gen-help-tables.sh`, `scripts/gen-claude-commands.sh`, `framework/bootstrap/*`)

- **Done when**: folding a branch-scoped spec into a `done` upstream spec leaves that spec `in-progress` with the content applied, the directory gone, no dangling links, and every installer-parity audit family passing (AC6, AC7, AC18, AC24, AC25, AC29).

## 13. Update the constitution

- [ ] Amend §numbering to define both directory forms, naming the branch-scoped one as temporary
- [ ] Amend §spec-lifecycle to record that a branch-scoped spec is a staging form discharged by fold-back, reconciling it with the anti-proliferation stance
- [ ] Run the audit families that check cross-doc agreement

- **Done when**: both sections describe the shipped behavior, and `scripts/audit/run-all.sh` reports no new findings (AC17).
