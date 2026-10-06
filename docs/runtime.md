# The runtime

The deep reference for the deterministic execution layer. The README's [The runtime](../README.md#the-runtime) section covers what it is, that you do not install it, and how the store and pointer relate; this is where the remaining operational detail lives — supplying your own binary, what happens when acquisition fails, how the MCP server is registered for each agent, and the environment variables the runtime reads.

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

## Environment variables

This is the one inventory of the environment variables that configure ductus — the runtime, and the installer and `/ductus` that acquire it. None is required, and with none set ductus is adopted from `stonean/ductus`, and the runtime behaves as a single session on the shared default and fetches over a direct connection.

| Variable | Required | Default when unset | Purpose |
| --- | --- | --- | --- |
| `DUCTUS_REPO` | no | `stonean/ductus` | The GitHub `owner/repo` that `install.sh` and `/ductus` fetch from — the bootstrap, the version pin, the framework archive and the runtime binary — so a fork can be adopted or tested (spec 065). Any other value is announced before the first fetch. Empty counts as unset. Not read by the runtime. |
| `DUCTUS_SESSION` | no | the agent's own session id, below | Names this process's session, so it keeps a target of its own (spec 062). Processes launched with the same value share one target. The value is sanitized by the slug rule; empty counts as unset, and a value that sanitizes to nothing is refused with an error naming the variable. |
| `CLAUDE_CODE_SESSION_ID` | no | no identity: the shared default `.ductus/session.toml` | Set by Claude Code in every MCP server and shell it spawns — not by you. Read as that agent's session identity when `DUCTUS_SESSION` is unset. |
| `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` | no | no host exempted | Comma-separated hosts that `fetch-archive` exempts from its `https`-only and internal-address screens, for a trusted internal mirror or local testing. It only ever loosens the guard, so leaving it unset is the secure posture. |
| `HTTPS_PROXY`, `HTTP_PROXY`, `ALL_PROXY` (or lowercase) | no | a direct connection | The proxy `fetch-archive` sends its downloads through, by the standard convention its HTTP client follows. |
| `NO_PROXY` (or `no_proxy`) | no | nothing bypasses a set proxy | Hosts `fetch-archive` reaches directly even when a proxy is set. |
| `REQUEST_METHOD` | no | unset | Set only in a CGI environment. When it is set, the HTTP client ignores every proxy variable above and fetches go direct, as the convention requires. |
| `SSL_CERT_FILE`, `SSL_CERT_DIR` (Linux and other non-Apple Unix) | no | the system certificate store | The certificates `fetch-archive` trusts: when either is set, only those it names — the certificate authority of a TLS-inspecting proxy, for one. Read with the system store once per process, at the first fetch. |

Every runtime variable above is read once, when the process starts, except the two certificate variables: they are read once per process, at its first fetch, because loading the system store at startup would cost every invocation that never fetches. Either way an agent's MCP server keeps the configuration it was launched with for its whole life. Through a proxy, the proxy is the trust boundary. `fetch-archive` still refuses a URL or redirect hop that resolves to an internal address, and a direct connection — no proxy, or a `NO_PROXY` host — stays pinned to the addresses it screened; a proxied connection is resolved by the proxy, so what it reaches is the policy of the operator who configured it (spec 048, `fetch-archive-reads-its-proxy-once`). `DUCTUS_REPO` is the exception to that reading: `install.sh` reads it once, at its start, and `/ductus` reads it in Source resolution and names it at every fetch, so it is fixed for the length of a run. Not listed are the variables the Rust standard library, the async runtime and git read the same way for every program built on them — `HOME`, `TMPDIR`, `RUST_BACKTRACE`, `TOKIO_WORKER_THREADS` — which tune the platform rather than configure this runtime.
