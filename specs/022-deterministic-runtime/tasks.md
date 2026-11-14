---
next-task: 32
---

# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 23. Installed files are not group-writable

- [x] Implement the behavior described in `scenarios/installed-files-are-not-group-writable.md`
- [x] `extract-archive` masks each entry's mode to `rwxr-xr-x` (`PERMISSION_MASK` in `runtime/src/primitives/extract_archive.rs`), on the tar and zip paths, with the module doc saying why
- [x] Tests show a `0664` entry landing as `0644` and a `0775` one as `0755` on both paths, and each fails when the mask keeps group write (`scripts/mutation/installed-file-modes.py`)
- [x] `apply-manifest` re-applies the extracted mode to an `update`-strategy file whose bytes already match when only its mode differs, leaving bytes and modification time alone, with a test and a mutation in `scripts/mutation/installed-file-modes.py` (owner decision 2026-10-27: reach files already installed)
- [ ] A destination whose mode the runner may not change keeps its mode and is named in `apply-manifest`'s `modes-kept`, and the walk goes on rather than stopping mid-manifest; `framework/bootstrap/ductus.md` step 4 names it in the completion message (found by the independent re-read of `47e68233`). Written 2026-10-28 and its macOS-only test passes; `python3 scripts/mutation/run.py scripts/mutation/installed-file-modes.py` ran whole on macOS at `77fff147` on 2026-10-28, 8 of 8 killed (M7 and M8 are this item's). Still owed: a real adopter bootstrap that shows the completion message naming a `modes-kept` file, run in a scratch adopter whose `.ductus/config.toml` sets `[runtime] path` to this tree's release build, with one destination made immutable (`chflags uchg`) so the runner may not change its mode; no release carries this runtime, and none is cut before every spec is `done` (owner, 2026-11-11)

- **Done when**: an archive's `0664` and `0775` entries extract as `0644` and `0755`, the executable bit survives, an already-installed file with matching bytes takes the masked mode on the next run, and the mutation run kills every mutation in `scripts/mutation/installed-file-modes.py`
