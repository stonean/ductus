# The runtime

The deep reference for the deterministic execution layer. The README's [The runtime](../README.md#the-runtime) section covers what it is, that you do not install it, and how the store and pointer relate; this is where the remaining operational detail lives — supplying your own binary, what happens when acquisition fails, and how the MCP server is registered for each agent.

## Supplying your own binary

Set `[runtime] path` in `.ductus/config.toml` and `/ductus` downloads nothing, resolving the pointer to the binary you name:

```toml
[runtime]
path = "runtime/target/release/ductus"
```

This is the supported route for building from source, for an air-gapped or firewalled checkout, and for a platform with no published asset. A version mismatch against the pin warns rather than halts — you have stated deliberately which binary you want. A path that does not exist, or will not execute, halts naming it; `/ductus` never falls back to downloading, which would discard your choice without saying so.

## When acquisition fails

A network failure, an unpublished asset for your platform, or a checksum mismatch halts the run naming the store path and the release URL — so you can place the binary there by hand and re-run, or set `[runtime] path`. There is no silent degradation: the runtime is required, and a requirement that quietly is not one leaves both execution paths alive.

Binaries are published for `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc`. Every target ships a `.tar.gz` plus a `.sha256` sidecar, and a release publishes only when all five are present.

If a runtime process crashes mid-procedure, just re-run the command — state lives in your markdown, and writes are filesystem-atomic, so the runtime resumes from the next incomplete step.

## Registering the runtime

`/ductus` wires the MCP server in the same run that acquires the binary. Where it points depends on where your agent reads MCP config:

- **Claude** — `/ductus` writes `.mcp.json` naming the repo-relative pointer; just start a fresh session. Fully automatic.
- **OpenCode** — `/ductus` writes the `ductus` `mcp` block into your committed root `opencode.json`, also naming the pointer; because OpenCode loads config once at startup, quit and restart it. No manual `mcp add`.
- **Auggie** — Auggie reads MCP servers from your user-level `~/.augment/settings.json`, which `/ductus` does not write. It surfaces a one-line command to run once per machine — `auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp"` — then start a fresh session.
- **Antigravity** — Antigravity reads MCP servers only from your home-level `~/.gemini/config/mcp_config.json` (project-local config is ignored), which `/ductus` does not write. It surfaces an instruction: add a `ductus` block naming that same store path, then reload with the in-prompt `/mcp` overlay.

The two home-level agents name the **absolute store path** rather than the pointer, for the mirror-image reason: their config is per-machine and serves every project, so no project-relative path could be correct in it.

From that session on, the pipeline takes the deterministic path. File writes are additive — an existing MCP config keeps its other servers, and a `ductus` entry that's already present is left untouched.
