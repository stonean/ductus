---
section: "Install location"
---

# Pointer-follows-the-store

## Context

The pointer did not follow the store on Windows. In CI on `windows-latest`, the bootstrap's `ln -sf ~/.ductus/bin/ductus.exe .ductus/bin/ductus.exe` exits 0 but writes a regular file holding the store's bytes, not a symlink. The run was runtime run 43336398418, job 131593547621, under Git Bash (MSYS runtime 3.6.10, `MSYS` unset) on a runner that is elevated with Developer Mode on. After the store is replaced by tempfile + rename, that file still holds the old binary. On Linux and macOS the same commands make a symlink, which holds the new binary afterwards. `scripts/measure-replace-under-server.sh` measures each case.

State A version-checks the store, not the pointer, so on Windows a pin bump leaves the project on the old binary three ways:

- the rest of that run's `.ductus/bin/ductus <primitive>` calls;
- every later session's MCP server;
- the pre-commit hook.

No later run notices, because the store reports the pin. AC13's "resolves to the store on every supported platform" was never true on Windows.

## Behavior

Every `/ductus` run keeps the pointer holding the resolved binary's bytes. The resolved binary is the store, or the `[runtime] path` when the project configures one.

- **Compare, then repair.** After **Runtime acquisition** resolves the binary, compute the SHA-256 of `{pointer-path}` and of the resolved binary. Use the platform checksum tool the **Permission Setup** seed already grants: `shasum -a 256`, `sha256sum`, or `certutil -hashfile … SHA256`. A pointer that is missing, dangling, or whose digest differs is re-made by **Pointer materialization**. An equal digest leaves it alone. A symlink to the resolved binary always digests equal, so on Linux and macOS the check repairs only a missing, dangling or differing link. After a re-make, compare once more: a pointer that still differs halts the run naming the pointer, the resolved binary and both digests, or saying which digest could not be computed.
- **State A, routine path.** When the store reports `{pin}` but the pointer differs, re-make the pointer. The live server was spawned from the old pointer, so run the rest of the session through `{pointer-path} <primitive>` and add the repair to the **deferred-restart set**, as the live-but-stale path does.
- **State A, live but stale.** Re-make the pointer after acquiring and before the first `{pointer-path} <primitive>` call.
- **State B.** Step 2's **Pointer materialization** runs the same comparison.
- **A `[runtime] path` is linked and copied as its resolved absolute path.** A relative value resolves against the directory the bootstrap resolves it against, `~` is expanded, and on Windows it keeps the Git Bash form. A relative value in `ln -sf` would dangle, since a link resolves relative to `.ductus/bin/`.
- **The `cp` fallback removes a differing symlink first.** `cp` onto a symlink writes into the link's target, which could be the store or a `[runtime] path` binary, so a pointer that is a symlink is removed with `rm -f` (the link, not its target) before the copy. The check is `ls -ld`, which the seed covers.
- **The `cp` fallback meets a busy pointer.** When `cp` onto the pointer fails, move the pointer aside to `{pointer-path}.old`, copy again, then remove `{pointer-path}.old`. A server holding the pointer gives "Text file busy" on Linux and "Device or resource busy" on Windows. The sequence was measured to succeed on Linux, macOS and Windows with the server still running. If the second copy fails too, halt naming the pointer.
- **The Windows `ln -sf` keeps its spelling**, with no `MSYS` prefix. Under Git Bash's defaults it writes a copy, and the comparison keeps that copy equal to the store.

## Edge Cases

- **A regular-file pointer whose digest equals the store's** is current and is left alone: a copy is not a defect, only a stale one is.
- **A digest that cannot be computed** (the checksum tool is missing, or the file is unreadable) is treated as differing, so the pointer is re-made. It is never treated as equal: a check that cannot run must not read as one that passed.
- **A re-make that exits 0 but leaves the pointer differing** is caught by the re-compare and halts, rather than going unnoticed until the next run. A persistent misread of a digest, `certutil`'s on Windows, is then diagnosable from the two digests the halt names.
- **A leftover `{pointer-path}.old`** from an interrupted run is removed before the pointer is moved aside again.
- **Unmeasured: Windows unelevated, without Developer Mode.** The CI runner is both elevated and has Developer Mode on. Git Bash's default copy does not depend on either, so the comparison covers that machine too, but no run has exercised it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
