---
section: "Concurrent writes lose nothing"
---

# First-writers-create-the-ignore-file-once

## Context

Any write on the `.ductus/` layout, as well as an identified resolution, creates `.ductus/sessions/` and writes a `.gitignore` of `*` inside it when absent, so per-process files never show as untracked (the plan's *Gitignore: two layers*). That write sat ahead of the session lock. Every writer that found the file absent wrote it, each through the shared atomic write — a tempfile renamed over the path — so sixteen writers starting together on a fresh tree made sixteen renames onto one path. Measured 2026-09-29 on macOS: all 16 writers of `concurrent_writers_each_keep_their_own_target` wrote the file, in each of 20 runs.

Unix lets the last rename win. Windows refuses a rename onto a file another process is replacing, with *Access is denied* (os error 5), so on `windows-latest` that test failed on its first run (runtime run 36653211031): a writer lost its target to an error before its own write began, which is the loss this section rules out.

Found by 041's task 18 (2026-09-29), after 062's runtime commits were first pushed.

## Behavior

The sessions directory's `.gitignore` is written only by a process holding the session lock, and only when it is absent. The directory itself is created before the lock, because the lock file lives in it, and creating a directory that already exists is not an error. Racing first writers therefore create the ignore file exactly once, and each of them goes on to keep the target it wrote, on every operating system the runtime is released for: Linux, macOS and Windows, the three its test matrix runs on.

## Edge Cases

- **A tree whose sessions directory already holds its `.gitignore`** has nothing rewritten.
- **A writer that times out waiting for the lock** has written nothing, the ignore file included, and reports the lock file as before.
- **The shared default and each per-process file** were already written under the lock, one writer at a time, and are unchanged.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
