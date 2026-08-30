# 053 — Supersession reconciliation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Runtime first — the bounded read, the annotation form, the extension point — then the command surfaces that drive them, then the documentation and the release the runtime change obliges. The classification itself is authored by the host at the extension point and has no implementation task; what is buildable is everything around it.

## 1. Add the bounded read

- [ ] Add `ReadSupersessionPairArgs` / `ReadSupersessionPairResult` to `runtime/src/schema/primitives.rs`, with no argument that could name a plan, data model, tasks file, source path, or third spec
- [ ] Create `runtime/src/primitives/read_supersession_pair.rs` returning both specs' bodies and criteria plus the superseded spec's scenarios
- [ ] Reuse `list_scenario_files` so scenario order matches every other surface, and report unreadable files the way `collect_scenario_open_questions` does — named, and excluded from `examined`
- [ ] Record the read bound in the doc comment as the reason the primitive exists, not as a rule it follows
- [ ] Add tests: a pair with scenarios; a pair with none; an unreadable scenario named and excluded; an absent superseding spec as a domain outcome

- **Done when**: the primitive returns exactly the declared pair plus the superseded spec's scenarios, there is no argument by which a caller could request anything else, and an unreadable file is named in `unreadable` and absent from `examined`.

## 2. Add the criterion granularity to the annotation writer

- [ ] Add an optional `criterion` label argument to `WriteSupersessionAnnotationArgs`
- [ ] With it absent, write today's whole-spec banner unchanged — every existing test must pass untouched
- [ ] With it present, append the annotation to that criterion's line, citing the superseding spec **by name** and never by link
- [ ] Leave the criterion's checkbox and its own text byte-identical, so a superseded criterion stays ticked
- [ ] Reuse the slug-boundary matching from `blockquote_cites` for the already-present check, so `043-workflows` is not satisfied by `043-workflows-sunset`
- [ ] Add tests: a first criterion annotation; a repeat writing nothing; a criterion whose label does not exist reported as a domain outcome; the checkbox and text unchanged; `derive-dependencies` deriving no edge from the annotated spec

- **Done when**: one primitive writes both granularities, a criterion annotation cites by name and induces no `dependencies:` edge, the criterion stays ticked with its text unchanged, and every pre-existing whole-spec test passes without modification.

## 3. Register the classification extension point

- [ ] Add `classifyClaims` to `build_extension_request` in `runtime/src/interpreter/payload.rs`, beside `routeFold` and `performReview`
- [ ] Build its request from the `read-supersession-pair` result: both specs, the scenarios, and the four-outcome vocabulary
- [ ] Give it its own vocabulary rather than reusing `routeFold`'s or `routeInboxItem`'s — those answer *where does this belong*, this answers *what did the later spec do to this claim*
- [ ] Add a test that an unknown identifier still errors, so the registry stays closed

- **Done when**: a walker reaching a `classifyClaims` step emits a request carrying the bounded pair, and the extension registry rejects an unknown identifier as it did before.

## 4. Register the new primitive

- [ ] Add the CLI command enum plus dispatch arm in `runtime/src/main.rs`
- [ ] Add the exec-path match arm in `runtime/src/interpreter/mod.rs`
- [ ] Add the `#[tool]` in `runtime/src/mcp/server.rs`
- [ ] Add it to `framework/runtime-tools.txt` and to `PRIMITIVE_REGISTRY` in `runtime/src/schema/registry.rs`
- [ ] Run `cargo test --test mcp` first, then `scripts/gen-configure-mcp.sh` followed by `scripts/gen-claude-commands.sh`, in that order

- **Done when**: `cargo test --test mcp` reports the manifest and the canonical registry set-equal, and `/{project}:audit`'s tool-coverage family reports no drift.

## 5. Reconcile at declaration time

- [ ] Add the reconciliation steps to the **Declaration semantics** reference in `framework/commands/supersede.md` — the canonical statement both declaration routes already share
- [ ] Order them: bounded read, `classifyClaims`, annotate each superseded claim, surface conflicts, report
- [ ] Confirm `framework/commands/specify.md` inherits it through the pointer it already carries, and add nothing that restates it
- [ ] Record why this runs at declaration rather than at the completion gate: the information is cheap while the superseding spec's claims are being authored and unrecoverable afterward

- **Done when**: both declaration routes reconcile through one statement of the procedure, `specify.md` restates none of it, and nothing in `check-review-gate` learns about reconciliation.

## 6. Surface conflicts without resolving them

- [ ] Report every `Conflicting` claim with its rationale and both sides, and offer no resolution
- [ ] Make the report distinguish the three outcomes by construction: examined-with-conflicts, examined-with-nothing-to-reconcile, and could-not-examine — naming the unexamined files rather than folding them into a total
- [ ] Name a pass incomplete whenever `unreadable` is non-empty or `guidance` is set

- **Done when**: a conflict is never resolved by the command, and a reader of the report can tell a clean reconciliation from an empty one from an incomplete one without consulting anything else.

## 7. Gate the one edit that reopens a spec

- [ ] Offer a body-prose edit only behind `gate-confirm`, with a prompt naming the `done → in-progress` back-edge **before** the edit
- [ ] Perform the status change through `set-status` with a `from` guard, like every other back-edge
- [ ] Never offer an edit for an acceptance criterion — annotation is the only outcome available there

- **Done when**: no body edit happens without a confirmation that named the reopen first, no criterion is ever offered for editing, and the reopen goes through the same guarded transition the rest of the pipeline uses.

## 8. Document the runtime-written criterion annotation

- [ ] Record in `framework/constitution.md` `§supersession-annotations` that the criterion granularity may now be written by the runtime, and that the section granularity remains hand-authored
- [ ] Add the reconciliation outcomes to `/{project}:analyze`'s enumerated list in `framework/commands/analyze.md`, including the coverage bound

- **Done when**: an adopter can tell from the constitution alone which granularities are mechanical and which are theirs to write.

## 9. Release the runtime change

- [ ] Confirm the preceding release is complete: a pushed `ductus-v0.38.0` tag whose workflow published assets. If it is not, finish or unwind it first — do not bump past it
- [ ] Add a fresh `## [Unreleased]` section to `runtime/CHANGELOG.md`, keeping the heading non-numeric while work is in flight so Family 20 still sees the previous release as newest
- [ ] At release, rename that heading to its version and bump the repo-root `version`, `runtime/Cargo.toml`, and `Cargo.lock` in one commit. Do not re-bless the parity goldens — they hold the version as a `{{runtime-version}}` placeholder
- [ ] Confirm every affected spec is `done` before the tag is cut
- [ ] Run `scripts/audit/run-all.sh` locally before tagging, and again against the tag's own tree if it is not `HEAD`
- [ ] Commit, push `main`, then tag at that commit and push the tag — in that order, with a short window between

- **Done when**: the tag is pushed, the release workflow's self-audit gate passes, and acquisition is verified against the published assets on all five targets.
