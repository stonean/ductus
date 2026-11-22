---
next-task: 27
---

# 023 — `ductus` Refinement Tasks

Tasks derived from the [plan](plan.md).

## Phase C — Follow-on scenarios

## 26. Hold the /ductus seed to the wildcard-position invariant

- [x] Implement the behavior described in `scenarios/configure-permission-pattern-safety.md`
- [x] Replace the Claude seed's `Bash(certutil -hashfile * SHA256)` with `Bash(certutil -hashfile .ductus/bin/ductus.exe SHA256)` in `framework/bootstrap/ductus.md`, `framework/bootstrap/govern.md` and `install.sh`, and append the old entry to Claude's retired seed list
- [x] Extend Family 29 (`scripts/audit/permission-wildcard-position.sh`) and its `scripts/audit/README.md` entry to the Claude seed's allow entries, and show the family fails with the old entry restored

- **Done when**: No allow entry in the Claude seed or /configure's canonical set places a standalone `*` before a further word, Family 29 reads both and reports each count, and a restored `certutil -hashfile * SHA256` makes it fail
