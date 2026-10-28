---
section: "Follow-on scenarios"
---

# Installed-files-are-not-group-writable

## Context

`extract-archive` took each extracted file's permissions from its archive entry, masked to `rwxrwxrwx` so an untrusted archive could never apply setuid, setgid or sticky. GitHub's tarballs record `0664` for a file and `0775` for an executable, and `apply-manifest` mirrored the extracted mode onto every file it created or updated. Every file `/ductus` installed therefore landed writable by its group. That includes `.githooks/ductus-pre-commit`, which runs on every commit, and Pi's `.pi/extensions/ductus.ts`, which Pi loads as code. On macOS a home directory is readable by the `staff` group, every local account belongs to it, and a file created under it takes that group. So on a shared machine another local user could rewrite an adopter's hook.

Found by the real adopter bootstrap of [050-constitution](../../050-constitution/spec.md) task 40 (2026-10-27): 42 installed files in a subject adopted at `ductus-v0.28.0` were group-writable after a `--ref=main` run, and the files the agent wrote by hand were not. The owner chose to fix it under this spec in the same round, and then to extend the fix to files already installed (2026-10-27).

## Behavior

- **An extracted file never carries group or other write.** `extract-archive` masks each entry's mode to `rwxr-xr-x` (`0o755`): an archive's `0664` lands as `0644`, its `0775` as `0755`, on the tar and the zip path alike.
- **The executable bit survives.** The hook and scripts `apply-manifest` installs keep it, as before.
- **`apply-manifest` mirrors the masked mode onto every file it writes**, on create and update, and onto an `update`-strategy file whose bytes already match when only its mode differs: the mode is re-applied, the bytes and modification time are left alone, and the action stays `unchanged`.

## Edge Cases

- **A file an earlier run installed.** It drops to `0644` or `0755` on the next run once the release carrying this fix is the adopter's runtime, whether or not its content changed. Git records only the executable bit, so the change shows in no diff. A file `apply-manifest` does not manage (a `create` or `skip` row that already exists, or a pinned file) keeps its mode.
- **An archive entry with no group or other write.** It is unchanged by the mask: a `0600` entry lands as `0600`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
