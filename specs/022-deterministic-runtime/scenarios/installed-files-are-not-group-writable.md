---
section: "Follow-on scenarios"
---

# Installed-files-are-not-group-writable

## Context

`extract-archive` takes each extracted file's permissions from its archive entry, masked to `rwxrwxrwx` so an untrusted archive can never apply setuid, setgid or sticky. GitHub's tarballs record `0664` for a file and `0775` for an executable, and `apply-manifest` mirrors the extracted mode onto every file it creates or updates. Every file `/ductus` installs therefore landed writable by its group. That includes `.githooks/ductus-pre-commit`, which runs on every commit, and Pi's `.pi/extensions/ductus.ts`, which Pi loads as code. On macOS a home directory is readable by the `staff` group, every local account belongs to it, and a file created under it takes that group. So on a shared machine another local user could rewrite an adopter's hook.

Found by the real adopter bootstrap of [050-constitution](../../050-constitution/spec.md) task 40 (2026-10-27): 42 installed files in a subject adopted at `ductus-v0.28.0` were group-writable after a `--ref=main` run, and the files the agent wrote by hand were not. The owner chose to fix it under this spec in the same round (2026-10-27).

## Behavior

- **An extracted file never carries group or other write.** `extract-archive` masks each entry's mode to `rwxr-xr-x` (`0o755`): an archive's `0664` lands as `0644`, its `0775` as `0755`, on the tar and the zip path alike.
- **The executable bit survives.** The hook and scripts `apply-manifest` installs keep it, as before.
- **`apply-manifest` needs no change.** It mirrors the extracted mode on every create and update, so each file it writes takes the masked mode.

## Edge Cases

- **A file an earlier run installed.** It drops to `0644` or `0755` the next time a run rewrites it. A file whose content is unchanged is not written, so it keeps its mode until the framework next changes it.
- **An archive entry with no group or other write.** It is unchanged by the mask: a `0600` entry lands as `0600`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
