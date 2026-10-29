---
section: "The primitive library"
---

# A-closed-stdout-is-a-quiet-exit

## Context

Every `ductus` subcommand panics with exit 101 when the reader of its stdout exits before draining it: `failed printing to stdout: Broken pipe (os error 32)`. Reproduced 2026-10-26 with the 0.59.11 binary: `ductus dashboard | true`, and `read-spec --include-body` piped to `true` or to `dd bs=1 count=1`. Reported against `extract-archive … | head`, it is general: the result emitters in `main.rs` write with `println!`, and nothing handles a broken pipe. Plain `| head` on single-line JSON does not trigger it, because `head` reads to the end of the one line; a reader that exits early does. The stderr side was fixed in b482e029 (063's review) with `let _ = writeln!`. Exit 101 is also what a panicking or failing run returns, so a shell script cannot tell an early-exiting reader from a crash.

## Behavior

- A subcommand whose stdout reader has gone exits without a panic and without a panic message, at the first write that meets the closed pipe.
- Its exit status is distinguishable from a primitive failure and from a panic.
- The `mcp` stdio server's handling of a closed transport is unchanged: the fix is at the CLI result-write sites, not a process-wide SIGPIPE reset.

## Edge Cases

- A write error other than a broken pipe (a full disk on a redirected stdout): still reported as an error.
- A broken pipe after a primitive has already written files: the files stand; only the report is lost.
- The proof uses a pipe whose reader is dropped (`std::io::pipe()`, as `runtime/tests/config_unknown_keys.rs` does for stderr), not a closed descriptor.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
