---
spec: 042-consolidate-govern-per-project-files-under-govern-directory
reviewed-at: 2026-07-23T00:50:37Z
reviewed-against: 50cc0702cf621c3766668c66275a83c47c0c6455
diff-base: 2b32d415558b5483d523711dd16669ee0566fc10
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 042-consolidate-govern-per-project-files-under-govern-directory

## Summary

Post-fix run: the QUAL-REUSE SHOULD from the 64b926c pass is resolved — `config_path` now derives from `config_display_name` (50cc070), so the new-wins choice lives once and the read path and provenance tag cannot disagree on the resolution rule; behavior unit-proven identical across all four presence cases. The tasks 14–15 delta plus release prep otherwise stands as reviewed: no new attack surface (display literals, doc comments, fixed-constant path helper), no new input handling, network calls, or secrets. 0 MUST, 0 SHOULD, 0 low-confidence — the probe-to-use race this run recorded as a low-confidence note was resolved 2026-08-02 under 022's `config-resolution-single-probe` scenario, and is kept under Low-confidence findings with that resolution recorded. No issues captured to the inbox in the window. Not blocking.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None remaining.* The finding below is **resolved**; it is kept in place with
its resolution recorded because it had been carried unresolved across two
review runs.

### LOW-CONFIDENCE: BE-RACE-001 — resolver existence-probe → use window races a concurrent migration — **RESOLVED**

- **File**: `runtime/src/schema/paths.rs:58-120`
- **Rule**: Shared mutable state reachable from more than one concurrent execution context MUST be protected by a synchronization mechanism — a lock, an atomic primitive, single-owner/actor confinement, or serialized access; unsynchronized concurrent read-write is a data race.
- **Finding**: Carried forward from the 8a770e8 run: the resolvers probe existence and return a path the caller later opens, and `discover-rule-files` / `dashboard` resolve the config once for reading and again for the provenance tag — two temporal probes, so a config file created or removed between read and render could tag a file other than the one read (the choice *logic* is now single-sourced in `config_display_name` after 50cc070, but the probes still run at separate moments). Mitigated by design: the pipeline is serial per constitution §concurrent-features, the migration runs only inside /ductus, writes are atomic tempfile+rename, and the notice renders only when a config was successfully read. Recorded low-confidence for visibility, not as a confirmed defect.
- **Auto-fixable**: no
- **Status**: **resolved 2026-08-02** under 022's [config-resolution-single-probe](../022-deterministic-runtime/scenarios/config-resolution-single-probe.md) scenario (022 task 79). `schema/paths.rs` gained `resolve_config(repo) -> (PathBuf, &'static str)` — one existence probe yielding both the read path and the provenance name — and `discover-rule-files` / `dashboard` now carry the resolved name forward with the parsed content instead of re-probing at render time. The serial-pipeline mitigation still holds, but it was an argument about who else is running rather than about the primitive being self-consistent; the two-probe window is now closed on the primitive's own terms. `DashboardConfig`'s serialized shape is unchanged, so no golden re-bless.

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
