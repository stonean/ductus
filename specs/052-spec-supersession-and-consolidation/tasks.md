# 052 — Spec supersession and consolidation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Runtime first, then the command surfaces that call it, then documentation, then the release the runtime change obliges. Reconciliation is not here — it is [053](../053-supersession-reconciliation/spec.md).

## 1. Add `supersedes:` to the frontmatter schema

- [x] Add the `supersedes` field to the spec-file schema table in `framework/constitution.md`, marked hand-authored and contrasted with the generated `dependencies:` / `references:` indexes
- [x] Add the field to the spec frontmatter struct in `runtime/src/schema/primitives.rs`
- [x] Extend `validate-frontmatter` with shape-only validation: each entry parses as a feature slug; a self-reference is rejected
- [x] Confirm an absent key and a populated key both round-trip through `read-spec` unchanged

- **Done when**: `validate-frontmatter` accepts a well-shaped `supersedes:` list, rejects a malformed entry and a self-reference, and reports an unresolvable-but-well-shaped entry as no finding.

## 2. Add the `write-supersession-annotation` primitive

- [x] Create `runtime/src/primitives/write_supersession_annotation.rs` taking the superseded feature, the superseding feature, and the authored substance as one payload
- [x] Compile the frame: blockquote prefix, `**Sunset ([link]):**` citation, record-of-what-shipped closer
- [x] Insert after the H1 and lead paragraph, ahead of any annotation already present
- [x] Leave the superseded spec's `status` untouched at **any** lifecycle state — the write is a mechanical edit, not a `done`-only one
- [x] Return an already-applied outcome when an annotation naming this superseding spec is already present, rather than stacking a duplicate
- [x] Add tests covering a first annotation, a second from a *different* spec accumulating above it, a repeat from the *same* spec that writes nothing, and an unreadable target

- **Done when**: the primitive writes a blockquoted annotation naming the superseding spec, an annotation from a different spec stacks above it without replacing it, a repeat declaration writes nothing and reports it, and the superseded spec's status is byte-identical before and after at every lifecycle state.

## 3. Prove the blockquote keeps the link out of `dependencies:`

- [x] Write a test that annotates a spec, runs `derive-dependencies`, and asserts no edge appears from the superseded spec to its successor
- [x] Assert the same body with the annotation un-blockquoted *does* produce the edge, so the test fails if the exemption changes

- **Done when**: both assertions hold, pinning the blockquote as structural rather than stylistic.

## 4. Gate `retire-feature`'s sequential refusal

- [x] Add an explicit opt-in argument to `RetireFeatureArgs` in `runtime/src/schema/primitives.rs`
- [x] In `retire_feature.rs`, apply Refusal 1 unless the argument is set; leave Refusal 2 (target holds a `spec.md`) untouched for both callers
- [x] Record in the doc comment why the refusal is gated rather than removed — it keeps an irreversible operation out of reach of a typo
- [x] Add tests: sequential without the flag refuses; sequential with it and a valid target succeeds; sequential with it and a target holding no `spec.md` still refuses

- **Done when**: `/{project}:fold`'s call path behaves identically to today, and a sequential directory is removable only when the opt-in and a valid target are both present.

## 5. Add the `supersession-reciprocity` check family

- [x] Add the family to `runtime/src/primitives/check_artifacts.rs` at the advisory tier
- [x] For each declared `supersedes:` entry, report when the named spec's body does not name the superseding spec back
- [x] Report coverage as bounded to declared edges, so an undeclared corpus is never presented as clean
- [x] Add the family to `/{project}:analyze`'s enumerated check list in `framework/commands/analyze.md`

- **Done when**: the family fires on a declared edge whose target does not name it back, stays silent on a satisfied edge, never fires on an undeclared pair, and its result states the coverage bound.

## 6. Register the new primitive

- [x] Add the tool to `runtime/src/mcp/server.rs`
- [x] Add it to `framework/runtime-tools.txt`
- [x] Regenerate the MCP allow blocks via `scripts/gen-configure-mcp.sh`

- **Done when**: the primitive is callable over MCP and `/{project}:audit`'s tool-coverage family reports no drift.

## 7. Add `--supersedes` to `/{project}:specify`

- [x] Add the flag to the Flags table and `argument-hint` in `framework/commands/specify.md` (Family 30 binds these)
- [x] Write `supersedes:` at creation and invoke the annotation primitive against each named spec
- [x] Write no markdown link to the superseded spec into the new spec's body — the pointer is frontmatter, so `derive-dependencies` derives no edge from the superseding spec to the one it supersedes
- [x] Offer the declaration as a selectable classification when `derive-routing-candidates` surfaces a candidate
- [x] Accept a named spec that is not `done`, naming consolidation as the likelier outcome and the reason, without refusing

- **Done when**: creating a spec with the flag writes the key and the reciprocal annotation, omitting it leaves creation behaving exactly as today, and a non-`done` target produces guidance rather than a refusal.

## 8. Create `/{project}:supersede`

- [x] Write `framework/commands/supersede.md` for declaration over two existing specs
- [x] State the declaration semantics once and reference them from `specify.md` rather than restating
- [x] Confirm through `gate-confirm` before any write
- [x] Make a re-declaration converge: no duplicate `supersedes:` entry, no second annotation, each step reporting already-applied as a domain outcome rather than a failure
- [x] Depend on nothing from 053 — the declaration is complete when the key and annotation are written

- **Done when**: a supersession can be declared over two existing specs, producing the same key and annotation as a creation-time declaration, and re-running it changes nothing.

## 9. Create `/{project}:consolidate`

- [x] Write `framework/commands/consolidate.md` in the cleanup family, calling `rewrite-spec-links` then `retire-feature` with the opt-in argument
- [x] Perform none of fold's content migration — no body edit, scenario, task, status change, or review invalidation
- [x] Name every spec whose `supersedes:` points at the source and offer re-point, drop, or cancel, defaulting to none
- [x] Name the specs the source itself superseded, whose annotations cite a spec about to disappear
- [x] Have `gate-confirm` name **content loss**, not only directory removal — and name the source's scenarios specifically, since they are destroyed with the directory and migrated nowhere
- [x] Make an interrupted consolidation converge on re-run, per `rewrite-spec-links`' idempotence and `retire-feature`'s already-absent outcome

- **Done when**: consolidation re-points every inbound pointer then removes the source, writes nothing to the target's own artifacts, refuses when the target holds no `spec.md`, and never re-points a `supersedes:` edge the operator has not settled.

## 10. Note the gate in `/{project}:fold`

- [ ] Record in `framework/commands/fold.md` step 12 that fold never passes the opt-in argument, so the sequential refusal still guards this path

- **Done when**: fold's account of `retire-feature` matches the primitive's behavior after task 4.

## 11. Codify the annotation in the constitution

- [ ] Document the annotation at whole-spec, section, and criterion granularity
- [ ] State that the citation links from a blockquote and cites by name from a criterion, with the harvesting reason
- [ ] State that the annotation is a mechanical edit taking no back-edge
- [ ] State the non-claim phrasing requirement

- **Done when**: an adopter can apply the convention from the constitution alone, without reading a spec that already carries one.

## 12. Install the new commands

- [ ] Add `supersede.md` and `consolidate.md` to the Slash commands manifest in `framework/bootstrap/ductus.md`
- [ ] Run `/{project}:audit` Family 16 and confirm installer-command parity

- **Done when**: both commands reach adopter projects through `/{project}` and Family 16 reports no drift.

## 13. Update the README

- [ ] Add both commands to the command tables
- [ ] Add the one-spec/two-spec distinction, placing `fold`, `consolidate`, and `supersede` in the two-spec group
- [ ] Mark `/{project}:consolidate` as the only command that removes a durable artifact

- **Done when**: the README documents both new commands and states the distinction that explains why they are commands rather than flags.

## 14. Record the cross-spec impact on 051

- [ ] Note in `specs/051-branch-scoped-spec-numbering/spec.md` that `retire-feature`'s sequential refusal is gated by this spec, with a back-link
- [ ] Apply it as a mechanical annotation so 051 stays `done`

- **Done when**: 051's account of the refusal matches shipped behavior and its status is unchanged.

## 15. Release the runtime change

Preconditions matter more than the steps here. As of planning, `version`, `runtime/Cargo.toml`, and the newest CHANGELOG heading all read `0.37.0` while **no `ductus-v0.37.0` tag exists** — the release workflow fires only on a pushed tag, so that version is committed but unpublished. This spec's release must not be built on top of an unfinished one: bumping past `0.37.0` while its assets do not exist would leave a version an adopter's `/{project}` pin can name and never acquire.

- [ ] Confirm the preceding release is complete before starting: a pushed `ductus-v0.37.0` tag whose workflow published assets. If it is not, finish or unwind it first — do not bump past it
- [ ] Add a fresh `## [Unreleased]` section to `runtime/CHANGELOG.md` for this spec's primitives (the annotation writer, the `retire-feature` gate, the reciprocity family). The `merge-permissions` `revoke` entry is **not** pending — it shipped under `[0.37.0]`
- [ ] Keep the heading non-numeric while work is in flight, with the comment recording why: `/{project}:audit` Family 20 binds the `version` pin, `runtime/Cargo.toml`, and the newest `## [X.Y.Z]` heading, and matches only numeric headings — so an `[Unreleased]` section is invisible to it and the previous release stays newest
- [ ] At release, rename that heading to its version and bump the repo-root `version`, `runtime/Cargo.toml`, and `Cargo.lock` in one commit. Do not re-bless the parity goldens — they store the version as a `{{runtime-version}}` placeholder, and blessing would hardcode a literal and destroy it
- [ ] Run `scripts/audit/run-all.sh` locally before tagging. It is a hard release gate, and a local pass is weaker evidence than it looks: `ductus-v0.28.0` was green locally and failed in CI on a BSD-awk incompatibility that exited 0 rather than reporting it could not run
- [ ] Commit, push `main`, then tag at that commit and push the tag — in that order, with a short window between. The commit alone reaches nobody, and a pushed `version` bump without its tag sends every adopter after assets that do not exist

- **Done when**: the preceding release is published, this spec's tag is pushed, the release workflow's self-audit gate passes, and acquisition is verified against the published assets on all five targets.
