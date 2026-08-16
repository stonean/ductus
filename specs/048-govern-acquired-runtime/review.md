---
spec: 048-govern-acquired-runtime
reviewed-at: 2026-08-16T02:51:23Z
reviewed-against: 948598d28fc2b634a699328201c0d83afecb856f
diff-base: 9832d40
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

Two findings were raised and **both were fixed before this report was written**, so the counts state what is outstanding. They are recorded below with a **Status** line naming the commit that closed each.

This spec's deliverable is mostly *specification* — the acquisition procedure in `framework/bootstrap/ductus.md`, two migration bodies, and a constitution amendment — plus two workflows and the four installer seeds. The passes concentrated there. The one genuinely executable artifact, `runtime-acquisition.yml`'s acquisition script, was extracted and checked standalone: `bash -n` and `shellcheck` both clean, and its assumption that the binary sits at the archive root was verified against how `runtime-release.yml` actually packages it (`tar -czf` from inside the release directory).

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None outstanding.* Both findings below were fixed in-window.

### SHOULD: QUAL-GROUND-001 — the commands contradicted the constitution they follow

- **File**: `framework/commands/*.md` (13 files), `framework/constitution.md:90`
- **Rule**: A claim about existing behavior stated without grounding it in the source is an assumption, and must be labeled one rather than asserted.
- **Finding**: The amendment made the runtime required, but every command source still opened with "the MCP tools of the **optional** ductus runtime" and instructed hosts to walk the prose when no server is registered. This spec's own plan named the risk — "a window in which the canonical source and the commands disagree" — and the window was left open. A reader following a command would have been told the opposite of what the constitution says.
- **Auto-fixable**: no
- **Status**: fixed in `27b2b30`. The no-server sentence stays rather than being deleted: there is a real window between acquisition and the restart that registers the server, so the fallback is the transitional state, reframed as such instead of presented as a first-class mode.

### SHOULD: QUAL-CLAIM-001 — a scheduling failure would read as a broken workflow

- **File**: `.github/workflows/runtime-acquisition.yml:38-56`
- **Rule**: When a code path cannot examine part of its subject, its output should say so rather than emitting a result a caller reads as positive assurance.
- **Finding**: The acquisition matrix must run each target **natively**, because the job executes the binary it installs — deliberately unlike `runtime-release.yml`, which cross-compiles `aarch64-unknown-linux-gnu` and `x86_64-apple-darwin` and never runs what it builds. Two of the required labels are availability-sensitive (`ubuntu-24.04-arm` is a newer offering; `macos-13` is the last Intel macOS image and on a deprecation path), and GitHub fails an unrecognized label at *scheduling* — no step output, no error text. Someone reading that failure sees a broken workflow, not a missing runner, and the divergence from the release matrix looks like a mistake rather than the point.
- **Auto-fixable**: no
- **Status**: fixed in `27b2b30` by recording why the matrix differs and which labels to suspect first. The availability itself cannot be verified from here — it is confirmed by the first run after a release publishes.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
