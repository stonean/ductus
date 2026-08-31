---
spec: 054-remove-supersession
reviewed-at: 2026-08-31T00:30:00Z
reviewed-against: 018f8be23ebc2ccd339263d0075a77b7bab534c2
diff-base: ec078af27a5ed2a30ab7c2967785c62f8113c771
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 054-remove-supersession

## Summary

Removal-only change: no new logic, no new abstraction, no new configuration. Zero MUST violations, zero SHOULD violations, zero low-confidence findings, not blocking.

**Security.** No new surface. The two removed primitives were the only writers of a hand-authored frontmatter key, and nothing that survives gained a capability. `retire-feature`'s gated sequential refusal and its ungated anti-stranding guard are untouched by this change and were re-examined during 052's review in the same session.

**Reuse.** The removal is subtractive throughout: `blockquote_cites` and `names_feature` went with their only callers rather than being left as orphans (clippy's dead-code warning is what surfaced the second), and `SpecRead`/`ScenarioRead` went with the only result type that referenced them. The blockquote exclusion in `spec_links.rs` was deliberately *not* removed — it serves four other primitives and its stated reason predates supersession — and is confirmed byte-identical against `main`.

**Quality.** The adopter-facing behavior of the removed `supersedes:` key was measured rather than assumed, against the built 0.42.0 binary: `validate-frontmatter` reports `clean: true` with zero findings, `read-spec` omits the key from its result without erroring, and `set-status` and `label-criteria` leave it byte-for-byte intact on disk because the frontmatter writers splice rather than re-serialize. The key is inert in both directions — the pipeline neither reads it nor destroys it — and `runtime/CHANGELOG.md` now states that measured behavior instead of asserting inertness generally.

`check-artifacts` emits exactly eight family names after the removal, matching `analyze.md`'s corrected count; eight extension points remain and all eight are dispatched, so none is left without a caller.

**Efficiency.** Nothing added; the runtime carries two fewer primitives, one fewer extension point, and one fewer check family.

**Simplicity.** The clearest win is `docs/slash-commands.md`'s flag-versus-command paragraph. `--supersedes` was the sole exception to the rule that a two-spec operation cannot be a flag, so removing it let the rule be stated without a carve-out rather than needing a replacement example.

**Two corrections made during this review, both recorded rather than left standing.** First, four `step N` cross-references in `specify.md` and `consolidate.md` still named pre-renumbering positions; all four now resolve, and the underlying gap is recorded as an observation because nothing checks them. Second — and more consequential — an acceptance criterion had been marked verified on the strength of an MCP primitive result, and the `ductus` MCP server runs the binary it was started with rather than the working tree. `read-spec` returned a `supersedes` key the new code cannot emit, which first read as a passthrough bug in `Frontmatter` and was in fact the pre-change server answering. Every affected claim was re-verified through `./runtime/target/release/ductus`, and `AGENTS.md` now carries the gotcha.

**Bounds on this review.** The scope is the whole 054 diff and was examined as such. `runtime/CHANGELOG.md`'s historical entries below 0.42.0 still name the removed primitives; that is deliberate — a changelog records what shipped, and rewriting past entries would falsify it. The corpus-wide sweep behind AC19 was verified by grep plus `check-corpus-links` at repository scope (400+ files, zero broken, zero skipped) and `check-orphaned-references` (four referrers, zero findings, zero skipped); in both, the empty `skipped` is what makes the clean reading valid.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

- convention: nothing verifies that a `step N` cross-reference in a command file resolves to a step that exists. Removing steps 5 and 8 from specify.md and step 3 from consolidate.md left four stale references — one of which had become a self-reference — and they were caught by reading, not by any check. The same class of error is reachable by any future edit that inserts or removes a numbered step. — `framework/commands/specify.md`

## Skipped passes

*None.*
