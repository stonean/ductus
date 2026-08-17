---
section: "Follow-on scenarios"
---

# Applied-set-not-watermark

## Context

`[migrations].last_applied` held a single slug, and the bootstrap selected entries whose `introduced_in` exceeded that entry's (SemVer, lex tie-break on `id`). One string, interpreted as a high-water mark.

A watermark cannot represent *"entry N was passed over"*. It can only say how far the adopter has got, and it says that by naming one entry — so the moment a later entry completes, every earlier entry becomes permanently ineligible, whether or not it ever ran. Nothing detects the hole, because the pointer that would reveal it is the same pointer that hid it.

Measured on the real adopter bootstrap for [048](../../048-govern-acquired-runtime/spec.md)'s AC10, 2026-08-17. Six entries were pending. The run applied five and never processed `workflows-sunset`; its own completion summary lists five and omits it. That entry's step 4 deletes `workflows/registry.json` when it exists unpinned, and step 1 exits silently only when *none* of its four targets exist — but the adopter's root `workflows/` held exactly that file, the active config carried no `[pinned]` section, and the other three targets were absent. The file survives byte-unchanged with its pre-run mtime.

The unrecoverable part is the gate, not the miss. `last_applied` is now `runtime-store-path` (`0.28.0`); `workflows-sunset` is `0.23.0`. No future `/ductus` run can ever select it again. The adopter keeps a file the framework intended to delete, with no pipeline path to remove it — recovery means deleting it by hand or editing `last_applied` backwards.

This is `QUAL-CLAIM-001` in the registry's own state: an advancing pointer asserts that every earlier entry converged, and nothing verifies that.

## Behavior

**`[migrations]` gains `applied`, a list of the entry ids this project has run, and it is what decides eligibility.** An active entry is pending when its `id` is absent from `applied`. `last_applied` stays, holding the most recently applied id, because readers report it as migration context — `check-orphaned-references` surfaces it as `last-applied` — but it no longer gates anything.

**Recording is per entry and only on success.** The id is appended to `applied` after its procedure completes, in the same atomic write that sets `last_applied`. An entry that did not complete stays absent, because absence is exactly what makes it eligible again. An aborted batch therefore resumes at the first entry missing from the list — including one *earlier* in registry order than the last success, which the watermark could not express.

**`applied` is backfilled once from `last_applied` when absent.** A project bootstrapped before the list existed carries only the watermark, so the bootstrap derives the list — every entry whose `introduced_in` is at most `last_applied`'s — writes it, and proceeds. The backfill inherits exactly what the watermark asserted and nothing more.

**A hole predating the backfill stays a hole.** The derivation cannot distinguish an entry that ran from one the watermark skipped, so it trusts the watermark's claim. This is stated rather than papered over: the fix prevents new holes, it does not repair old ones, and an adopter carrying one clears it by hand. The AC10 subject is exactly this case.

**Ordering is unchanged.** Entries still apply in registry order — `introduced_in`, lex tie-break on `id`. Only the eligibility test changes, from comparison to membership.

**A retired id needs no special case.** An id in `applied` (or `last_applied`) that no longer exists in the active registry matches no active entry, so it neither selects nor suppresses one. The watermark needed a rule here — a retired `last_applied` had no position to compare against, so it was treated as *before the oldest active entry* and re-ran everything — and membership removes it.

## Edge Cases

- **Absent `[migrations]` section**: nothing applied, every active entry pending. Unchanged.
- **`applied` present and `last_applied` absent**: `applied` governs; no backfill runs, and `last_applied` is written on the next successful entry.
- **Both absent but the project is plainly migrated** (an adopter who hand-cleared the section): indistinguishable from a fresh project, so every active entry runs. Every procedure owns its idempotency check, so a re-run over converged state is a silent no-op — which is the property that makes membership safe to be wrong in this direction.
- **An entry appears twice in `applied`**: harmless, since only membership is tested; the write appends without deduplicating rather than growing a uniqueness rule for a case that costs nothing.
- **An adopter who never applied a since-retired entry**: undetectable under either model, because the entry is no longer in the registry to test against. They apply it from `CHANGELOG.md`, as before.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
