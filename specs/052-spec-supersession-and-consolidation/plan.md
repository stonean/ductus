# 052 — Spec supersession and consolidation Plan

Implements [052 — Spec supersession and consolidation](spec.md).

## Overview

Three surfaces over two existing primitives plus two new ones. Nothing here invents machinery: `rewrite-spec-links` and `retire-feature` already perform supervised removal with pointer integrity for fold-back, `create-scenario` already establishes how authored prose crosses the runtime boundary, and `check-artifacts` already hosts advisory families of exactly the shape the reciprocity check needs. The work is a frontmatter field, an annotation writer, one gated relaxation, one new check family, and the command wiring that reaches them.

Reconciliation is deliberately absent — it is [053 — Supersession reconciliation](../053-supersession-reconciliation/spec.md), which depends on this spec. This one ships without it.

## Technical Decisions

### `supersedes:` is a hand-authored frontmatter list

It joins the schema as a list of feature slugs, explicitly unlike `dependencies:` and `references:`, which the frontmatter schema marks **Generated** and not hand-authored. A list rather than a scalar because one spec routinely supersedes several at once — `043-workflows-sunset`'s removal surface names four sibling specs receiving the annotation — where `folds-into:` is correctly scalar because a staging spec has exactly one home.

Validation splits the way `folds-into:` already does: `validate-frontmatter` checks **shape** only, and the named spec's existence is enforced where the consequence lands. The precedent is stated in `runtime/src/primitives/retire_feature.rs:9-17` — `create-feature` checks `fold-into` for shape, `validate-frontmatter` reports an unresolvable target as no finding, and existence is enforced at fold-back, the first moment the question has an answer worth refusing on. Here the equivalent moment is the declaration itself, since both specs exist by then.

### The annotation crosses the boundary as one payload

A new `write-supersession-annotation` primitive frames; the host supplies the substance. This is the content-ingestion convention already used by `create-scenario`, whose `body` argument is documented as "the markdown the LLM authored, crossing the runtime boundary as one payload" with the primitive contributing the frontmatter, heading, and scaffolding. No primitive writes prose, per §runtime-boundary principle 2.

The frame contributes placement (top of body, newest first), the `> **Sunset ({link}):**` citation, and the closing record-of-what-shipped sentence. The **blockquote wrapper is load-bearing, not stylistic**: `derive-dependencies` exempts blockquote-prefixed lines from harvesting, which is how `specs/005-workflows/spec.md:20` links `043-workflows-sunset` while its `dependencies:` names only `004` and `010`. Without the wrapper every annotation would silently create a dependency from the superseded spec onto its successor.

### `retire-feature`'s sequential refusal is gated, never removed

The refusal's rationale is recorded at `runtime/src/schema/primitives.rs:3166`: the sequential form is permanent, and "a primitive that could delete one would make an irreversible operation reachable from a typo." Deleting the refusal would discard that protection for every caller.

Instead the primitive gains an explicit opt-in argument that only `/{project}:consolidate` passes. `/{project}:fold` never passes it, so a mistyped feature name during a fold still meets Refusal 1 unchanged. The anti-stranding guard (Refusal 2 — the target must hold a `spec.md`) is untouched and applies to both callers.

### The reciprocity check is a new `check-artifacts` family

Existing families are named strings dispatched inside one primitive — `criterion-path-existence` (`runtime/src/primitives/check_artifacts.rs:1047`), `link-adjacent-drift` (`:754`). A `supersession-reciprocity` family joins them at the **advisory** tier, surfaced through `/{project}:analyze`.

It reads a declared edge and asks only whether the named spec's body names the superseding spec back. It never infers an undeclared supersession — that is the check measured at 455 pairs with 215 firing and every sample a false positive. Its coverage is therefore bounded to declared edges, and it reports that bound rather than presenting an undeclared corpus as clean.

### Three commands, one shared statement

`/{project}:specify --supersedes` declares at creation; `/{project}:supersede` declares over two existing specs; `/{project}:consolidate` removes. The declaration semantics are stated once and referenced, following the `routeInboxItem` precedent where `specify.md` walks "the **same** decision tree" as `groom.md` and is instructed not to restate it. A second copy is how the two drift apart.

`/{project}:consolidate` reuses `rewrite-spec-links` unchanged — its own documentation already describes it as re-pointing inbound pointers to "a retiring **or renamed**" directory, so no behavioral change is needed there.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/schema/primitives.rs` | Modify | `supersedes` field; annotation-primitive args; `retire-feature` opt-in argument |
| `runtime/src/primitives/retire_feature.rs` | Modify | Gate Refusal 1 behind the explicit argument; leave Refusal 2 untouched |
| `runtime/src/primitives/write_supersession_annotation.rs` | Create | Frame the annotation; ingest authored substance |
| `runtime/src/primitives/check_artifacts.rs` | Modify | Add the `supersession-reciprocity` family with its coverage bound |
| `runtime/src/primitives/validate_frontmatter.rs` | Modify | Shape validation for `supersedes:` |
| `runtime/src/mcp/server.rs` | Modify | Register the new primitive |
| `framework/commands/specify.md` | Modify | `--supersedes` flag; offer it as a routing-candidate classification |
| `framework/commands/supersede.md` | Create | Retroactive declaration over two existing specs |
| `framework/commands/consolidate.md` | Create | Cleanup-family removal command |
| `framework/commands/fold.md` | Modify | Note that the opt-in argument is never passed here |
| `framework/constitution.md` | Modify | Annotation rule at three granularities; `supersedes:` schema row |
| `framework/bootstrap/ductus.md` | Modify | Install the two new commands (Family 16 manifest parity) |
| `framework/runtime-tools.txt` | Modify | Register the new primitive |
| `README.md` | Modify | Command tables; the one-spec/two-spec distinction |
| `specs/051-branch-scoped-spec-numbering/spec.md` | Modify | Record that the sequential refusal is gated here |

## Trade-offs

**A second command rather than a flag on `/{project}:amend`.** Amend already owns the lifecycle back-edges, which is a real pull. Rejected because a supersession writes two specs and amend declares a single-spec scope; widening it is the erosion already rejected for `/{project}:fold`. Cost accepted: two commands added to a sixteen-command surface.

**Gating the refusal rather than splitting `retire-feature` in two.** A separate `remove-feature` primitive would leave each with one unqualified rule. Rejected as duplication of an irreversible operation — two deletion paths is worse than one with an explicit gate, and the anti-stranding guard would need copying.

**Annotation frame compiled into the primitive rather than read from a template.** Matches `create-scenario`, which compiles its framing rather than reading `specs/templates/scenario.md`. The known limitation is the same one that primitive carries: an adopter customizing a template sees the compiled framing on the runtime path and the on-disk template only on the markdown-only path.

**No automatic backfill for the twelve specs already annotated.** Deriving those edges is the rejected inference; a migration guessing them would write false relations into every adopter's frontmatter unattended. The cost is a two-tier corpus — the objection that justified `criterion-label-backfill` — accepted here because that backfill is deterministic and this one would not be.

**Known limitation:** the reciprocity check sees only declared edges, so a corpus of hand-written annotations is invisible to it. It reports that bound rather than implying coverage it does not have.
