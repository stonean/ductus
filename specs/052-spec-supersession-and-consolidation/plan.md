# 052 — Spec consolidation Plan

Implements [052 — Spec consolidation](spec.md).

## Overview

One command over two existing primitives, plus one gated relaxation. Nothing here invents machinery: `rewrite-spec-links` and `retire-feature` already perform supervised removal with pointer integrity for fold-back. The work is reaching them for a sequential spec rather than a branch-scoped staging one, and doing it behind a confirmation that names what is lost.

## Technical Decisions

### `retire-feature`'s sequential refusal is gated, never removed

The refusal's rationale is recorded at `runtime/src/schema/primitives.rs`: the sequential form is permanent, and "a primitive that could delete one would make an irreversible operation reachable from a typo." Deleting the refusal would discard that protection for every caller.

Instead the primitive gains an explicit opt-in argument that only `/{project}:consolidate` passes. `/{project}:fold` never passes it, so a mistyped feature name during a fold still meets the refusal unchanged. The anti-stranding guard — the target must hold a `spec.md` — is untouched and applies to both callers.

### `rewrite-spec-links` is reused unchanged

Its own documentation already describes it as re-pointing inbound pointers to "a retiring **or renamed**" directory, so consolidation needs no behavioral change there. Matching is by whole path segment, so a directory whose name merely shares a prefix is left alone, and cross-service URLs are never rewritten.

`dependencies:` and `references:` are deliberately not touched: both are derived from body links, and their generators regenerate them from the rewritten bodies on the next commit.

### The confirmation names content loss, not directory removal

Consolidation migrates nothing, so everything in the source directory is destroyed — its scenarios among it. The anti-stranding guard proves the target *exists*, never that anything landed there, so a prompt naming only the directory would understate what the operator is approving. The scenarios are named individually rather than folded into a general claim about content.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/schema/primitives.rs` | Modify | `retire-feature` opt-in argument |
| `runtime/src/primitives/retire_feature.rs` | Modify | Gate the sequential refusal behind the explicit argument; leave the anti-stranding guard untouched |
| `framework/commands/consolidate.md` | Create | Cleanup-family removal command |
| `framework/commands/fold.md` | Modify | Note that the opt-in argument is never passed here |
| `framework/bootstrap/ductus.md` | Modify | Install the command (Family 16 manifest parity) |
| `README.md` | Modify | Command tables; the one-spec/two-spec distinction |
| `specs/051-branch-scoped-spec-numbering/spec.md` | Modify | Record that the sequential refusal is gated here |

## Trade-offs

**Gating the refusal rather than splitting `retire-feature` in two.** A separate `remove-feature` primitive would leave each with one unqualified rule. Rejected as duplication of an irreversible operation — two deletion paths is worse than one with an explicit gate, and the anti-stranding guard would need copying.

**A command rather than a flag on `/{project}:fold`.** Fold's purpose, its enumeration step, its post-merge instruction, and its single-source rule for the fold target are all specific to the branch-scoped staging form; carrying a sequential path through them would qualify seven load-bearing statements.

**Known limitation: the command verifies nothing about the target's coverage.** The guard proves the target directory holds a `spec.md`, never that it says what the source said. That comparison is semantic judgment an operator can ask for in the same conversation, before confirming; building it in would put a slow pairwise read in front of every consolidation.
