---
section: "Follow-on scenarios"
---

# Lint-markdown-tool-resolution

## Context

`lint-markdown` spawns `Command::new("npx")` and maps any spawn failure to `PrimitiveError::Io { path: repo }`. Both halves of that are wrong in the same situation, and together they cost most of a working session.

**The error names the wrong thing.** When the spawn fails with `ENOENT`, the path reported is the *repository*, not the executable. The operator sees:

```text
I/O error on /Users/stonean/src/stonean/ductus: No such file or directory (os error 2)
```

which reads as a missing repository or fixture directory — the one thing that is definitely present. Observed 2026-08-27: the same message from `runtime/tests/mcp.rs`'s fixture path sent the reader looking for a missing `sample-repo`, and the actual cause was that `npx` could not be found. The message is not merely unhelpful; it is actively misdirecting, and it is emitted by the path that knows exactly which program it failed to launch.

**`npx` is frequently not on `PATH`.** Under `nvm` — the common way to manage Node — `npx` is a lazy-loading *shell function*, not a binary: `command -v npx` prints `npx` with no path. A spawned process inherits `PATH`, never the parent shell's functions, so `Command::new("npx")` cannot resolve it. This is not a misconfiguration to be worked around but how `nvm` is built, so any contributor using it hits this on a clean checkout.

The consequences ran through the whole pipeline, because `lint-markdown` is gate check 1 of `check-review-gate`: the MCP tool errored, `check-review-gate` could not be reached at all (twice, forcing the documented markdown-only fallback), `runtime/tests/mcp.rs:86` failed on an unmodified tree, and a `git commit` was rejected by the pre-commit hook's `cargo test`. Every one of those presented as an unrelated failure. The same suite passes in CI, where `npx` is a real binary — so this is invisible to the one signal that would otherwise catch it.

What the runtime must **not** do is reach for a login shell to resolve the function. That would be slow on every lint, non-deterministic across contributors' shell configs, and a code-execution surface — sourcing a user's profile to find a linter inverts [§runtime-boundary](../../../framework/constitution.md#runtime-boundary)'s determinism guarantee. The fix is to look where a program can legitimately look, and to say plainly when it cannot find it.

## Behavior

- `lint-markdown` prefers a project-local `node_modules/.bin/markdownlint-cli2` when one exists at the repo root, invoking it directly. This removes the `npx` dependency entirely for projects that vendor the tool, and is deterministic — a path check, not a shell.
- Failing that, it falls back to `npx markdownlint-cli2` as today (`npx.cmd` on Windows).
- A spawn failure is reported as a **distinct, self-describing outcome** naming the executable rather than the repository. The message identifies what could not be launched and, on `ENOENT`, states that it was not found on `PATH` — with guidance noting that a shell-function `npx` (nvm's lazy loader) is invisible to a spawned process, and that putting the real Node `bin` directory on `PATH` or vendoring `markdownlint-cli2` resolves it.
- The `repo` path stops appearing in this error. It is the working directory, never the thing that was missing; naming it is what made the failure read as a missing fixture.
- A non-zero exit from `markdownlint-cli2` itself is unchanged — still not an error, still recorded as `exit_code` with the parsed violations, per the existing contract.

## Edge Cases

- `node_modules/.bin/markdownlint-cli2` present but not executable, or a broken symlink: the spawn fails and reports that path as what could not be launched — the same self-describing shape, naming the local binary rather than falling silently back to `npx`, since a vendored tool that cannot run is a condition worth seeing.
- Neither the local binary nor `npx` resolves: one error naming `npx` as the last thing tried, with the `PATH` guidance. This is the observed case.
- A spawn failure that is not `ENOENT` (a permissions error, for instance) keeps the underlying OS error and names the executable, without asserting it was a `PATH` problem — the guidance is specific to not-found and must not be attached to every failure.
- `check-review-gate` inherits the improvement without change: it already surfaces the primitive's error, so a legible cause replaces a misdirecting one at the gate too.
- CI is unaffected — `npx` resolves there, so the local-binary branch is simply not taken. The scenario's value is entirely in the environment CI cannot represent, which is why a passing CI run was never evidence against this bug.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
