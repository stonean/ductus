# ductus

Deterministic runtime for the [ductus](https://github.com/stonean/ductus) spec-driven development pipeline.

`ductus` interprets the parseable procedures of the pipeline's slash commands — reading specs, walking tasks, checking dependencies, performing atomic checkbox updates, and handshaking pipeline gates in Rust instead of LLM tokens. The model is invoked only at named extension points, where semantic judgment actually matters. The pipeline requires the runtime; [§runtime-boundary](https://github.com/stonean/ductus/blob/main/framework/constitution.md#runtime-boundary) in the constitution states the constraints this binary commits to.

## Install

A project adopting ductus does not install the runtime by hand: `/ductus` acquires the pinned release into a ductus-owned store during adoption. See [The runtime](https://github.com/stonean/ductus#the-runtime) in the ductus README, and [docs/runtime.md](https://github.com/stonean/ductus/blob/main/docs/runtime.md) for supplying your own binary.

Every `ductus-v*` tag publishes `ductus-<target>.tar.gz` with a `.sha256` sidecar to GitHub releases, for `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc`. The crate also builds from source with `cargo install ductus`.

## Usage

```bash
ductus mcp                # start an MCP server exposing every primitive but run-generator as a tool
ductus exec <command>     # interpret a slash command procedure end-to-end via JSON-over-stdio
ductus parse <file>       # parse a procedure file and print its AST (debug)
ductus parse --check <f>  # exit 0 when it parses, 2 when it is legacy prose, 1 when it is invalid
ductus <primitive> [...]  # invoke a single primitive standalone (e.g., ductus read-spec --feature 001)
```

The runtime expects to be invoked from a ductus-adopting repository — one with `framework/commands/*.md` or the project's installed command files, and feature directories under the configured spec root (`specs` by default).

## JSON-over-stdio protocol

`ductus exec` (and the interactive `ductus gate-confirm` subcommand) speaks the envelope protocol: one JSON envelope per line on stdout. Standalone `ductus <primitive>` invocations instead print the primitive's bare result object as a single line of JSON, with no envelope wrapper. The closed set of envelope types:

| Type | Direction | Payload |
| --- | --- | --- |
| `progress` | runtime → host | Step number and human-readable status text |
| `llm-request` | runtime → host | Extension point, request-id, payload to send to the LLM |
| `llm-response` | host → runtime | request-id correlating an open `llm-request` |
| `gate-confirm` | runtime → host | Named gate plus prompt to surface to the user |
| `gate-response` | host → runtime | request-id and the user's `confirmed: bool` decision |
| `complete` | runtime → host | Procedure finished cleanly; carries the runtime version |
| `error` | runtime → host | Procedure halted on operational failure; halts the walk |

See [022's data model](https://github.com/stonean/ductus/blob/main/specs/022-deterministic-runtime/data-model.md) for the envelope shapes and the core primitive schemas; the audit-record and completion-gate primitives are in [066's](https://github.com/stonean/ductus/blob/main/specs/066-audit-records-and-the-completion-gate/data-model.md), and the analyze and audit checks in [067's](https://github.com/stonean/ductus/blob/main/specs/067-analyze-and-audit-checks/data-model.md).

## Stability

The CLI surface and the JSON-over-stdio protocol are the supported public surface. The Rust library exposed by this crate (`use ductus::…`) ships only so the runtime's own integration tests can link against its internals; the module surface is **not stable** in 0.x and may change without a semver bump. Use the binary, not the library.

## Repository

Full source, specs, and the framework itself live at <https://github.com/stonean/ductus>.

## License

MIT — see [LICENSE](LICENSE).
