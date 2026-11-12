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

`/ductus` wires the runtime in the same run that acquires the binary, automatically for both agents:

- **Claude** — `/ductus` writes `.mcp.json` naming the repo-relative pointer; just start a fresh session.
- **Pi** — `/ductus` writes the project-local `.pi/extensions/ductus.ts` bridge, which starts the runtime from the same pointer and registers its tools; accept the project-trust prompt once, then restart pi.

From that session on, the pipeline takes the deterministic path. File writes are additive — an existing MCP config keeps its other servers, and a `ductus` entry that's already present is left untouched.

## Environment variables

This is the one inventory of the environment variables that configure ductus — the runtime, the installer and `/ductus` that acquire it, and the maintainer script that regenerates the command copies. None is required, and with none set ductus is adopted from `stonean/ductus`, and the runtime behaves as a single session on the shared default and fetches over a direct connection.

| Variable | Required | Default when unset | Purpose |
| --- | --- | --- | --- |
| `DUCTUS_REPO` | no | `stonean/ductus` | The GitHub `owner/repo` that `install.sh` and `/ductus` fetch from — the bootstrap, the version pin, the framework archive and the runtime binary — so a fork can be adopted or tested (spec 065). Any other value is announced before the first fetch. Empty counts as unset. Not read by the runtime. |
| `DUCTUS_BIN` | no | `runtime/target/release/ductus` in the ductus checkout | The path of the runtime binary that `scripts/gen-claude-commands.sh` and its test run, a relative path taken from where the script started. The pre-commit hook passes the one it resolved. A maintainer of this repository sets it; adopters never do. Not read by the runtime. |
| `DUCTUS_GEN_TIMEOUT_SECONDS` | no | `120` | The seconds `scripts/gen-claude-commands.sh` waits on each runtime call before stopping it and failing, naming the runtime. A positive whole number; any other value fails the script, naming the variable. Its test sets it; adopters never do. Not read by the runtime. |
| `DUCTUS_SESSION` | no | the agent's own session id, below | Names this process's session, so it keeps a target of its own (spec 062). Processes launched with the same value share one target. The value is sanitized by the slug rule; empty counts as unset, and a value that sanitizes to nothing is refused with an error naming the variable. |
| `CLAUDE_CODE_SESSION_ID` | no | no identity: the shared default `.ductus/session.toml` | Set by Claude Code in every MCP server and shell it spawns — not by you. Read as that agent's session identity when `DUCTUS_SESSION` is unset. |
| `CLAUDE_PROJECT_DIR` | no | the working directory | Set by Claude Code for each hook it runs — not by you — to the directory the session started in. Read only by `ductus hook status` and `ductus hook help`, as the project root, since sessions start at the project root (spec 080). Empty counts as unset. |
| `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` | no | no host exempted | Comma-separated hosts that `fetch-archive` exempts from its `https`-only and internal-address screens, for a trusted internal mirror or local testing. Each entry is a bare host — a name, an IPv4 literal, or an IPv6 literal in brackets — with no scheme, port or path; empty entries are ignored, and any other entry fails every fetch with an error naming the variable and the entry. It only ever loosens the guard, so leaving it unset is the secure posture. |
| `HTTPS_PROXY`, `HTTP_PROXY`, `ALL_PROXY` (or lowercase) | no | a direct connection | The proxy `fetch-archive` sends its downloads through, by the standard convention its HTTP client follows. A value that does not parse, or names a scheme other than `http` or `https`, refuses every fetch, naming the variable. |
| `NO_PROXY` (or `no_proxy`) | no | nothing bypasses a set proxy | Hosts `fetch-archive` reaches directly even when a proxy is set. |
| `REQUEST_METHOD` | no | unset | Set only in a CGI environment. When it is set, the HTTP client ignores every proxy variable above and fetches go direct, as the convention requires. |
| `SSL_CERT_FILE`, `SSL_CERT_DIR` (Linux and other non-Apple Unix) | no | the system certificate store | The certificates `fetch-archive` trusts: when either is set, only those it names — the certificate authority of a TLS-inspecting proxy, for one. Read when the process starts; what they name, or the system store, is loaded once per process, at the first fetch. With neither set, the system store's loader reads both again at that fetch and finds them unset. |

Every runtime variable above is read once, when the process starts. The certificates the two certificate variables name are loaded later, once per process, at its first fetch (with neither set, the system store's loader reads both again there and finds them unset), because loading the store at startup would cost every invocation that never fetches. Either way an agent's MCP server keeps the configuration it was launched with for its whole life. Through a proxy, the proxy is the trust boundary. `fetch-archive` still refuses a URL or redirect hop that resolves to an internal address, and a direct connection — no proxy, or a `NO_PROXY` host — connects only to a screened address, the first host's pinned to the addresses its screen resolved and a redirect host's screened again at connect; a proxied connection is resolved by the proxy, so what it reaches is the policy of the operator who configured it (spec 048, `fetch-archive-reads-its-proxy-once`). `DUCTUS_REPO` is the exception to that reading: `install.sh` reads it once, at its start, and `/ductus` reads it in Source resolution and names it at every fetch, so it is fixed for the length of a run. Not listed are the variables the Rust standard library, the async runtime and git read the same way for every program built on them — `HOME`, `TMPDIR`, `RUST_BACKTRACE`, `TOKIO_WORKER_THREADS` — which tune the platform rather than configure this runtime.
