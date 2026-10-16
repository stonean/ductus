---
section: "Follow-on scenarios"
---

# Merge-managed-block-multi-subsection-end

## Context

The `merge-managed-block` primitive (line-prefix style) does not reliably detect the end of an existing canonical block when that block contains interior blank lines between subsections. The `.gitignore` template shipped by `/ductus` ([framework/templates/project/gitignore](../../../framework/templates/project/gitignore)) is exactly this shape: five subsections (`# Environment and secrets`, `# Claude Code local settings`, `# ductus derived views`, `# IDE`, `# OS`), each separated by a blank line. Every `/ductus` invocation against an adopter project re-walks the same canonical block; each run leaves a fresh trail of orphan subsection headers below the real block. Over time the file accumulates a tail like:

```text
# Claude Code local settings (keep commands tracked for project-wide access)

# ductus derived views — non-markdown caches and indexes generated from specs.
# Markdown artifacts (specs, plans, scenarios) stay in git as the source of truth;
# binary or machine-only views regenerated from them do not. See constitution
# §text-first-artifacts.

# IDE

# OS
```

— canonical-block subsection headers, comment-shaped, with their list bodies dedup'd away. The accumulation reproduces on every adopter that ran `/ductus` against a template with multi-subsection blocks.

Root cause is local to [`runtime/src/primitives/merge_managed_block.rs`](../../../runtime/src/primitives/merge_managed_block.rs):

1. `find_line_prefix_block` locates the `# {marker}` line and returns `body_end` as "the next blank line." When the canonical block contains an interior blank, `body_end` is the *first interior blank within the canonical block*, not the block's actual terminator.
2. `merge_line_prefix`'s `Some(...)` arm compares the returned `body` (truncated to the first subsection) against the supplied `block` (the full multi-subsection canonical). The comparison can never succeed for multi-subsection canonicals, so the `unchanged` branch is unreachable — the file is rewritten on every run.
3. The `updated` branch computes `after = &text[body_end..]` using the same wrong `body_end`. `after` therefore contains the rest of the on-disk canonical block (subsections 2..N), which gets concatenated below the freshly written full block.
4. The dedup pass strips non-blank, non-comment adopter-area duplicates, but explicitly preserves comment lines (`# foo`). The subsection headers in the duplicated tail are comments, so they survive — producing the orphan-empty-header trail.

The companion `unchanged`-arm comment already calls out that `find_line_prefix_block`'s "next blank" heuristic only matches when the canonical block has no interior blanks; the `updated` arm inherited the same assumption without the safeguard.

The accompanying regression test `line_prefix_preserves_multi_subsection_block_with_interior_blank_lines` covers the *insert* path (first run, no marker present). It does not cover the *update* path (subsequent runs with the marker present), which is where the bug lives.

## Behavior

- `merge-managed-block` (line-prefix style) MUST bound the existing on-disk canonical block at its true end — past every interior blank line that separates its subsections — using the supplied `block`, not by finding the next blank line. The bound comes from group alignment (`merge-managed-block-subsection-insertion`, refined by `merge-managed-block-trailing-append` and `merge-managed-block-renamed-subsection`); the line-shape walk first specified here is retired.
- When the on-disk body span byte-equals the supplied `block`, the primitive emits `unchanged` and does not rewrite — preserving mtime and idempotency. Multi-subsection canonicals reach `unchanged` on stable reruns, the same way single-subsection canonicals already do.
- When the on-disk body differs from the supplied `block`, the primitive replaces exactly that span and computes `after` from the byte offset immediately following the last consumed body line (including its terminating newline). The post-merge file contains the new managed region followed by the adopter content that originally lived past the block, with no duplicated subsections. A canonical whose structure changed between runs is bounded the same way, within the limits `merge-managed-block-subsection-insertion` and its refinements state.
- The `find_line_prefix_block` helper takes the supplied block as an additional parameter and bounds the body through the helper-internal `walk_body_extent`. The previously-used `find_blank_line` helper is removed.
- The cross-boundary dedup pass continues to operate on the corrected block bounds (`block_start..block_end` derived from `managed_region_len` against the new write), so adopter-area duplicates above and below the canonical region are still removed. Canonical-block-wins semantics are unchanged.
- A new unit test under `merge_managed_block::tests` (`line_prefix_multi_subsection_rerun_is_unchanged_and_preserves_mtime`) reproduces the bug: it (a) writes a file containing the shipped multi-subsection canonical under the marker, (b) reruns `merge-managed-block` with the same canonical, (c) asserts `action == "unchanged"`, `dedup_removed == Some(0)`, mtime preserved, and file contents byte-equal to the pre-run state.
- A second unit test (`line_prefix_multi_subsection_update_replaces_cleanly_without_duplicated_tail`) exercises the same-structure update path — a multi-subsection canonical whose comment wording changes between runs — and asserts the on-disk block is replaced cleanly with each subsection header appearing exactly once (the orphan-tail symptom would surface as duplicated `# IDE` / `# OS` headers).

## Edge Cases

- **Stable canonical, repeated runs (the bug case)** — the on-disk body matches the supplied block byte-for-byte after a `created` or `inserted` run. The block is bounded at its true end, the body slice byte-equals `block`, and the primitive emits `unchanged` without rewriting. Mtime is preserved. This is the case that accumulates orphan headers under the broken next-blank heuristic; the fix must make it perfectly idempotent.
- **Same-structure content change** — supplied `block` and on-disk block carry the same subsections in the same order (the realistic update path when the framework template tweaks comment wording or adds an ignore pattern within an existing subsection). The on-disk block is bounded at its true end, the body slice differs from `block`, and the *updated* arm replaces exactly the on-disk span. Adopter content past the block — and its single separating blank line — moves to `after` and is preserved.
- **Canonical block with no interior blank lines** — a single subsection, bounded at the blank line (or EOF) that terminates it; behavior is unchanged for single-subsection canonicals and all existing tests pass.
- **Canonical block grew between runs** (more lines than on-disk wrote) — the block is bounded at its true end and replaced cleanly, with no orphan tail (`merge-managed-block-subsection-insertion` for an inserted subsection, `merge-managed-block-trailing-append` for an appended one).
- **Canonical block shrank or structurally diverged between runs** — subsections are matched on their pattern lines rather than line shape (`merge-managed-block-subsection-insertion`); a subsection removed or renamed mid-block is consumed and replaced rather than left as orphan headers, within the bounded lookahead `merge-managed-block-renamed-subsection` states.
- **Adopter edited inside the canonical region** — out of scope. The primitive's contract has always been "the managed region is framework-owned; don't edit inside it." If an adopter inserted a line inside the block, the bounded body slice may include their edit, the body comparison fails, and the next run overwrites. This matches the existing `html-comment` style's behavior.
- **Canonical block where the *last* line is blank** — the supplied block is trimmed of leading and trailing newlines before the merge, and the on-disk body is trimmed the same way before the byte-equality check, so a trailing blank line changes neither the bounds nor the comparison.
- **Marker appears in adopter content as a literal `# {marker}` line outside the managed region** — already handled by the marker-line scan returning the first match. The behavior is unchanged: the first `# {marker}` line wins; the rest of the file (including any second occurrence) is adopter territory.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
