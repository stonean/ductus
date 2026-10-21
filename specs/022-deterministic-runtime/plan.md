# 022 — Deterministic Runtime Plan

Implements [022 — Deterministic Runtime](spec.md).

## Overview

This spec ships a substantial new artifact: a Rust binary, `ductus`, built from `runtime/` in this repo, plus six rewritten slash command files and two new CI workflows. The spec body already settles the largest design questions in its Resolved Questions section (language, surfaces, ABI, distribution, install policy, MCP naming convention, partial-failure semantics, state management). This plan's job is to make the remaining implementation-concrete choices: project layout, dependency selection, parser strategy, test-fixture shape, ordering, and release engineering. Roughly:

1. **A new Rust crate at `runtime/`** with `clap`-based CLI exposing two top-level subcommands (`ductus mcp`, `ductus exec`), plus per-primitive subcommands so each primitive is invokable standalone.
2. **The primitive operations** implemented in-process, each with a deterministic core and a thin CLI/MCP/JSON-protocol wrapper. State-modifying primitives use tempfile-plus-rename atomicity.
3. **A markdown procedure parser** for slash command Instructions sections, hand-walked over `pulldown-cmark`'s event stream. The parser produces a typed AST the interpreter walks.
4. **An interpreter** that walks the AST, calls primitives in-process, and round-trips JSON messages over stdio for LLM extension points and gates.
5. **Three extension points** (`assessSpecQuality`, `writeCode`, `writeSpecBody`) with versioned request/response schemas defined in `data-model.md` and serialized via `serde`.
6. **Six rewritten slash command files** under `framework/commands/`, adopting the parseable conventions.
7. **Two new CI workflows**: `runtime.yml` (per-PR build + test on changes to runtime source paths) and a release workflow triggered by tags that cross-compiles and uploads artifacts.
8. **Parseability check** in `framework-checks.yml` — `scripts/lint-procedure-parseability.sh` runs `ductus parse --check` against every `framework/commands/*.md` and `framework/bootstrap/*.md` file, tolerating legacy prose only for a file on the `runtime/legacy-prose-commands.txt` allowlist.
9. **Distribution + discovery**: GitHub release artifacts, which `/ductus` acquires during adoption since `048-govern-acquired-runtime`; the README's runtime section; a one-line pointer in the `/ductus` bootstrap completion message. No per-invocation install nags.

Implementation proceeds bottom-up: primitives → MCP surface → parser → interpreter → slash command rewrites → CI/distribution. Each layer is testable in isolation before the one above it depends on it.

## Technical Decisions

### Project layout: single Rust crate at `runtime/`

The runtime is a single binary crate at `runtime/Cargo.toml` (sibling to `framework/`, `specs/`, `scripts/`). No workspace, no library/binary split — the binary is the surface, and the primitives and parser are internal modules `lib`-shaped only inasmuch as integration tests need to call them. The crate's targets:

- `runtime/Cargo.toml` — package manifest pinned to a stable Rust edition (2024) and an MSRV declared in the manifest's `rust-version` field.
- `runtime/src/main.rs` — `clap`-derive CLI entrypoint, dispatching to subcommand handlers.
- `runtime/src/lib.rs` — re-exports the modules below so integration tests under `runtime/tests/` can call into them.
- `runtime/src/parser/` — procedure parser.
- `runtime/src/interpreter/` — procedure walker + JSON protocol I/O.
- `runtime/src/primitives/` — one module per primitive, each exposing a pure-Rust function plus `clap`-derive args for the CLI surface.
- `runtime/src/mcp/` — MCP server wiring (uses `rmcp`).
- `runtime/src/schema/` — request/response schemas for primitives, JSON protocol messages, and extension points. `schemars` derives each JSON Schema from these types at runtime — the MCP tool schemas when the server registers its tools, the protocol envelope's on demand through `parse --emit-schema` — and no generated schema file is committed, so there is nothing to regenerate and diff.
- `runtime/tests/` — integration tests against fixture repos under `runtime/tests/fixtures/`.
- `runtime/CHANGELOG.md` — runtime release notes, maintained in lockstep with framework releases per §runtime-boundary.

`runtime/` is shipped as part of the repo, and `runtime/.gitignore` excludes its `target/`.

### Dependency selection

Each pick has a one-line rationale; cargo-version pins go into `Cargo.toml`.

- `clap` (derive feature) — CLI parsing. Industry standard, good UX, derive-based ergonomics fit the per-primitive subcommand pattern.
- `serde`, `serde_json` — JSON protocol messages and primitive payloads.
- `tokio` (rt-multi-thread, io-util, signal features) — async runtime required by `rmcp`. The non-MCP subprocess interpreter path stays mostly synchronous but shares the runtime to avoid mode-switching.
- `rmcp` — Anthropic's reference MCP SDK. Mandated by the spec's resolution on implementation language.
- `pulldown-cmark` — markdown event parser. The procedure parser walks its event stream rather than re-tokenizing markdown by hand. Chosen for correctness on edge cases (escapes, nested lists, fenced blocks) and to avoid maintaining a markdown lexer.
- `tempfile` — atomic-write tempfile creation in the target file's parent directory (so the subsequent rename stays on the same filesystem).
- `thiserror` — typed error definitions per primitive and interpreter surface.
- `regex` — limited use in lints (e.g., anchor reference matching). The bulk of "matching" is structural via `pulldown-cmark` events.
- `serde_norway` — frontmatter parsing in `read-spec` and `validate-frontmatter`. Chosen over hand-rolled because frontmatter is real YAML; it replaced `serde_yaml` in 0.11.1 (`5e82f339`). It is also the safe-loading mode `BE-INPUT-008` asks a spec to name. YAML is deserialized into a typed Rust structure the caller names (`frontmatter_status` in `runtime/src/primitives/mod.rs` reads a `Frontmatter`), or, where a reader must accept what it does not name, into `serde_norway::Value`, a tree of plain data. `validate-frontmatter` walks the whole block as one to report each field; the waiver and decision entries keep fields they do not interpret in one, so a rewrite preserves them; `write-review` parses a scalar as one to test whether it reads back unquoted; and the visitor that reads one keyed list from a frontmatter mapping reads each key as one. Neither path lets a document choose what is constructed. The set of types is fixed when the runtime is compiled, so a YAML tag such as `!!python/object` can at most select a variant of an enum the caller named, or survive as a `Value::Tagged` node holding the tag as text, and parsing runs no code. TOML configuration goes through `toml` and protocol messages through `serde_json` on the same terms.
- `walkdir` — `derive-boundary` and lint primitives need recursive directory walks.
- `git2` — `derive-boundary` and `check-stuck` need access to git history. Rust bindings to the C library libgit2, which the `vendored-libgit2` feature (`runtime/Cargo.toml`) compiles from bundled source and links statically, so no system libgit2 is needed; no shell-out to `git`, which would couple the runtime to system git on PATH.

No `tracing`, no `log`, no async logging crates. Human-readable stderr output uses `eprintln!`; structured `progress` JSON messages over stdout are emitted by the interpreter directly. Observability is a single function in `src/io.rs` that writes one JSON object per line. Keeping the dependency surface tight matters because every new crate is supply-chain surface for a tool that runs in every adopter's environment.

Nothing secret reaches stderr (`BE-LOG-002`). Its lines are diagnostics: paths, parse errors, and guidance. The one value the runtime holds that can carry a credential is a proxy URL, and `ProxyEnv` in `runtime/src/primitives/fetch_archive.rs` deliberately implements no `Debug`, so it is never formatted. The one raw echo, an unparseable stdin line under `ductus exec` (`runtime/src/io.rs`), returns the host's own input to the host's own stderr.

### Dependency pinning, provenance, and upgrades

- **Pinned** (`BE-DEPS-002`). `runtime/Cargo.lock` is committed, and every CI build and test and every release build runs `--locked` (`.github/workflows/runtime.yml`, `.github/workflows/runtime-release.yml`, and the builds `scripts/lint-procedure-parseability.sh` and `scripts/audit/check-zero.sh` make in `framework-checks.yml` and the release's audit job), so the lockfile alone decides what ships.
- **Upgraded on a schedule** (`BE-DEPS-002`). `.github/workflows/runtime-currency.yml` runs `scripts/check-runtime-currency.sh` every Monday: the lockfile, the direct dependencies' major versions, the pinned toolchain, the advisories, and the CI pins the lockfile does not cover (see the `runtime.yml` CI workflow below). A red run is fixed by a lockfile refresh, a major bump, a toolchain bump, a dependency update, or a pin bump, each committed on its own and shipped through the ordinary gate and release. `cargo audit` also runs on every push that touches the runtime (`runtime.yml`) and as a release-gate job (`runtime-release.yml`).
- **Provenance** (`BE-DEPS-004`), by installer:
  - **Crates: a documented exception.** The runtime's build verifies no publisher signature or provenance attestation for its crates. Its workflows run no such step, and this plan assumes cargo offers none for crates.io packages beyond checking each downloaded crate against the sha256 `checksum` its `runtime/Cargo.lock` entry carries, so the crates cannot meet the rule as written. That checksum and the audits above stand in for it. The same holds for the two build tools the release installs from crates.io, `cargo-zigbuild` and `cargo-cyclonedx`, which `runtime/Cargo.lock` does not list; the exact version each installs (**Release workflow** step 2) stands in for them. Remediation: adopt signature or attestation verification for crates.io packages once cargo supports it.
  - **`ziglang`, from PyPI: an installer gap, documented.** pip installs it for the Linux ARM cross-build and verifies no signature or attestation: its documentation says "By default, pip does not perform any checks to protect against remote tampering" and names no signature or attestation check (Secure installs). So the install runs in pip's hash-checking mode, which "uses local hashes, embedded in a requirements.txt file, to protect against remote tampering and network issues": `pip3 install --require-hashes` against the wheel's sha256, which `runtime-release.yml` commits (`BE-DEPS-005`). Remediation: verify the wheel's attestation once pip does so on install.
  - **`cargo-audit` and the Rust toolchain: an installer gap, documented.** `taiki-e/install-action` checks the `cargo-audit` it downloads against a SHA256 checksum, and rustup checks each toolchain component against the sha256 its channel manifest records but does not validate signatures (both quoted under **Pinned inputs** below). Neither check identifies the publisher. Remediation: verify a signature once either installer does.
  - **`markdownlint-cli2`, from npm: verified.** `framework-checks.yml` installs the lint tooling with `npm ci`, then runs `npm audit signatures`, with which npm's documentation says "you can verify the registry signatures of downloaded packages" and which "will also verify the provenance attestations of downloaded packages" (npm-audit, Audit Signatures). The npm CLI sets a failing exit code when any signature is invalid or missing (`lib/utils/verify-signatures.js`), so an unsigned package fails the step. This is the project's npm dependency rather than the runtime's: `lint-markdown` runs it through the project's own `npx`.

### Parser strategy: pulldown-cmark events + hand-walked structural recognizer

The procedure parser reads a markdown file with `pulldown-cmark`, walks its event stream, and emits a typed AST defined in `data-model.md`. Two reasons for this layering:

1. `pulldown-cmark` handles the surface lexing (code spans, fenced blocks, lists, headings, HTML blocks) correctly. Hand-rolling that is bug-prone.
2. The structural conventions (numbered steps, backtick-quoted primitive names, HTML-comment extension-point markers) are recognized at the event level by a small state machine. This is what the parser owns.

The parser exposes two public entry points: `parse(source: &str) -> Result<Procedure, ParseError>` and `check(source: &str) -> Result<(), ParseError>` — the latter is what the CI parseability check invokes. Parse errors carry line/column ranges (`pulldown-cmark` provides byte offsets; the parser converts to line/col on demand).

**Backward-compatibility provision**: when an Instructions section does not match the parseable conventions, `parse` returns `Err(ParseError::LegacyProse)`. The CI check treats this variant as a non-failure only for a file the allowlist `runtime/legacy-prose-commands.txt` names, and a file leaves the allowlist once its Instructions follow the conventions. The allowlist is also the discoverable surface that lists what hasn't been ported yet.

### Interpreter: synchronous walk + stdio JSON for asynchronous seams

The interpreter is a synchronous walker over the parsed AST. When it reaches a primitive step, it calls the primitive in-process and gets back a result. When it reaches an LLM extension point or a gate-confirm, it serializes a JSON request to stdout, blocks reading stdin one line at a time until it gets a parseable response, then resumes. No tasks, no futures, no concurrency in the walker itself — the surface is request/response.

The async runtime (tokio) is started for the MCP surface; the subprocess interpreter runs on `block_on` of a small main-task that drives the synchronous walker. This keeps both paths in the same binary with a single async-init cost.

### JSON-over-stdio framing: line-delimited JSON, one message per line

Newline-delimited JSON (`\n`-terminated). Each line is one complete JSON object. The protocol uses an envelope with a `type` discriminator (`llm-request`, `llm-response`, `gate-confirm`, `gate-response`, `progress`, `complete`, `error`) per the spec body. The set of message types is closed; adding one requires a versioned protocol bump per §runtime-boundary's lockstep-versioning rule. Stderr is human-readable text (no JSON), reserved for logs the host may surface to its own log streams.

A blank or unparseable line on stdin is reported on stderr and skipped, and the interpreter keeps reading for the response it is waiting on (`runtime/src/io.rs`), so a stray line from the host does not end the walk.

### MCP surface: one tool per primitive, no orchestration tools

`ductus mcp` starts an `rmcp` server exposing one tool per primitive in `spec.md` §The primitive library, under bare `<verb>-<noun>` names each host prefixes with its own server name, per the resolved-questions section. Tool input schemas are derived from each primitive's args struct via `schemars`. No orchestration tools (`run-procedure`, `walk-tasks`, etc.) are exposed via MCP — orchestration is the subprocess interpreter's job, not the MCP server's. This keeps the MCP surface flat and stable.

A tool result is bounded by the project it describes rather than paginated (`BE-PAYLOAD-001`). A list grows with the project's own tree. `compute-review-scope`'s `scope` grows with the plan's Affected Files and the files modified since its base, with ignored files left out (066's `review-scope-includes-uncommitted-work`), and a body is one artifact's, as with `read-spec` and `read-tasks`, whose subjects `check-artifact-size` warns about past the configured read size. Pagination is not provided, because the subject is the project's own working tree and spec corpus. A result over a host's tool-output cap is re-run through the runtime binary with its output redirected to a file, as `framework/commands/review.md` step 1 directs for `compute-review-scope`.

Tool calls run concurrently. rmcp 3.5.1 handles each request in a task of its own (`spawn_service_task` in its `src/service.rs`, which is `tokio::spawn` without its `local` feature), on the multi-threaded runtime `run_mcp_server` builds (`runtime/src/main.rs`), and `dispatch_blocking` (`runtime/src/mcp/server.rs`) moves the slower primitives onto blocking threads. The in-memory state those calls share is synchronized (`BE-RACE-001`). Two values change after they are first set: the record of which unknown-key warnings the config readers have written, so that each is written once per process (`scenarios/config-readers-name-unknown-keys.md`), is `warn_unknown_keys`' `static WARNED: Mutex<BTreeSet<String>>` (`runtime/src/schema/config_keys.rs`), held only for the check and the insert; and `gate-confirm`'s request counter is an `AtomicU64` (`fresh_request_id`). Every value read once per process — the staging roots, `fetch-archive`'s proxy, insecure-host and certificate settings, the startup environment the session identity comes from, and the compiled patterns — sits in a `OnceLock`, which std documents as "a thread-safe `OnceCell`" that "can nominally be written to only once".

### Atomic writes: tempfile in same directory, then `std::fs::rename`

State-modifying primitives (`mark-task`, `mark-criterion`, `set-status`) write to a tempfile created via `tempfile::NamedTempFile::new_in(parent_dir)`. The tempfile is written, flushed, fsynced, then renamed over the target with `tempfile::NamedTempFile::persist`. POSIX rename is atomic within the same filesystem; placing the tempfile in the same directory guarantees that property. Windows rename semantics are weaker but acceptable for this use case — adopters running on Windows accept the small atomicity gap, and a partial write is recovered from git.

### Path arguments: a lexical screen, then canonical containment

A path the host or the LLM supplies is checked before the primitive opens anything under it (`BE-INPUT-004`, [a-path-argument-resolves-inside-its-base](scenarios/a-path-argument-resolves-inside-its-base.md)). Both halves live in `runtime/src/primitives/mod.rs`:

- **The lexical screen.** `validate_no_traversal` refuses an empty, absolute, rooted, drive-prefixed or `..` path. It reads the text alone, so on its own it guards only the primitives that record a path and never open it: `write-session` and `retarget-sessions`.
- **Canonicalization and containment.** `validate_contained` runs the screen, then resolves every symlink on the path and refuses it unless the resolution lies inside the canonicalized base, compared by whole components (`Path::starts_with`), never by string prefix. A path not created yet is judged by its deepest existing ancestor, and a dangling link by where its target would resolve. The resolver follows at most 40 links itself (`MAX_SYMLINK_HOPS`, the value of Linux's `MAXSYMLINKS`) from one budget for the whole resolution, as the kernel keeps one per lookup, so a crafted chain of links that each name the next more than once is refused rather than resolved after exponential work; each `canonicalize` it calls stops at the platform's own limit. The base is the directory the primitive names: the project root, or `apply-manifest`'s `source-root` and `target-root`. A link that stays inside the base is followed and accepted. The check compares canonical forms only, and the primitive keeps using and naming the path it was given.
- **Every site that opens a path.** Of the 32 call sites that take a feature argument, 31 go through `validate_feature_argument` or `validate_feature_path` and `check-artifacts`' through `validate_feature_contained`, which skips the walk below; the 7 that take a file or directory path (`apply-manifest`, `enforce-manifest`, `merge-managed-block`, `merge-permissions`, `migrate-session-file`, `run-generator`) go through `validate_contained`.
- **The paths no `validate_no_traversal` call screened** (022 task 9, the owner's answer (a)). `validate-frontmatter`, `resolve-anchor`, `check-rule-ids` and `check-promotion-coverage` take each path through `contained_path`, which runs `validate_contained` against the project root and refuses an absolute path, checking every path before reading any. `lint-markdown` checks each path through `validate_glob_contained`. It refuses the forms the check does not model: a leading `:`, markdownlint-cli2's literal path; a leading `!` or `#`, its negations; a brace, parenthesis or `|`, micromatch's brace expansion (`foo/{1..5}.md`), extglobs (`+(x|y)`) and regex "or" (`foo/(abc|xyz).js`), each in micromatch 4.0.8's README; and a backslash, which markdownlint-cli2 turns into `/` except before one of `$()*+?[]^` (its `processArgv`), where fast-glob reads it as an escape (its README: "Use backslashes for escaping characters") and Rust on Windows as a separator. It walks a glob's literal prefix and any directory a path names, `.` among them, because markdownlint-cli2 0.23.2 sets neither `followSymbolicLinks` nor `expandDirectories` in its `globbyOptions`, and fast-glob 3.3.3 (`settings.js`) and globby 16 (`normalizeOptions`) default both to true, so a linked directory is followed and a directory expands to everything under it. `resolve-feature` checks the feature it matches with `validate_feature_argument`. `resolve_path`, which accepts an absolute path, is left to the bases: `apply-manifest`'s `source-root` and `target-root`, and the checkouts committed config names.
- **Staging paths.** `fetch-archive`'s destination and `extract-archive`'s archive and destination go through `validate_in_temp_dir`: absolute and strictly inside one of its roots, spelled as the root is or canonically, the root itself refused, and the rest checked against it by `validate_contained`. A candidate is matched to a root by its text before anything resolves it, so one outside every root is refused without being opened (on Windows, std's `canonicalize` opens the path it resolves: `sys/fs/windows.rs`), and the root itself is refused when the candidate spells the root it matched or its canonical form is any root's. The roots are `std::env::temp_dir()`, which follows `TMPDIR` and, on Unix other than macOS, stands for `/tmp` when an empty `TMPDIR` leaves it empty, where GNU `mktemp -t` then puts its directory (a relative one contributes no root, nor does a root that cannot be resolved), and on macOS also the per-user temp directory `getconf DARWIN_USER_TEMP_DIR` names: `/ductus` makes `{tempdir}` with `mktemp -d -t`, which on macOS uses `TMPDIR` only as a fallback (its manual: "based on the prefix and the _CS_DARWIN_USER_TEMP_DIR configuration variable if available"), while GNU `mktemp -t` on Linux uses `TMPDIR`, or `/tmp` when it is unset or empty (coreutils `mktemp.c`: `if (env && *env) dest_dir = env;`). The roots are read once per process, and `getconf` runs under `GETCONF_TIMEOUT` (5 s, `BE-TIMEOUT-001`): one that has not answered by then is treated as unreadable, so the per-user directory is a root only when `temp_dir()` names it too, and a staging path only it would admit is refused rather than waited on. On Windows, Git Bash's `mktemp` prints a path with no drive letter: handed over MCP, the runtime refuses it as relative, so such a run is likely to halt at the archive fetch, naming the path, while passed to the CLI from Git Bash it is converted to a Windows path first. Neither is measured on Windows.
- **Archive entries.** `extract-archive` screens each entry's path as a path argument is screened (no absolute path, no `..`) and checks its destination with `validate_contained` against `dest` before writing it, so a link already under `dest` cannot carry an entry outside it; the archive itself creates no link, since symlink entries are skipped (`safe_join` in `runtime/src/primitives/extract_archive.rs`).
- **A walk of the feature directory.** A feature-argument call also lists the directory recursively and is refused when any link in it resolves outside the project, which covers the files it then opens under fixed names (`spec.md`, `tasks.md`, the `scenarios/` listing). `check-artifacts` checks its argument but is exempt from the walk, because it skips a linked sibling as `target-unparseable` and keeps running (067's `sibling-symlink-trust-boundary`); it reads its spec and tasks through `read_spec::read_checked` and `read_tasks::read_checked`, which skip the walk for that reason. The walk checks only what it can list, so a link inside a directory that is traversable but not readable is not seen.
- **The `writeCode` boundary.** The runtime never opens an edit path, since the host applies the edits, so the boundary is the containment decision the host relies on: `validate_write_code_resolution` matches where each edit path lands (`resolved_relative`) as well as its text, and refuses one that reaches outside the project or through a link that cannot be resolved.

The check runs once per call, so a link created between it and the open is not caught. The tests are `interpreter::tests::every_feature_argument_site_refuses_an_outward_feature_link`, `every_feature_argument_site_but_check_artifacts_walks_the_feature_directory`, `every_file_path_site_refuses_an_outward_link`, `every_staging_site_refuses_a_path_outside_the_temp_directory` and `the_write_code_boundary_matches_where_an_edit_lands`, which hold each call site to the check by name, and the `primitives::tests` cases for the resolution itself. On macOS, `exec_subprocess.rs`'s `a_staging_directory_from_mktemp_t_is_accepted_whatever_tmpdir_says` runs the binary, with `TMPDIR` pointed elsewhere, on a directory `mktemp -d -t` made.

### CLI surface shape

`clap` derive-based, with subcommands flat under the top-level binary:

```text
ductus mcp                           # start MCP server (long-running)
ductus exec <command> [args...]      # subprocess interpreter
ductus parse <file>                  # parser (debug/manual use)
ductus parse --check <file>          # parseability check (CI)
ductus <primitive-name> [args...]    # invoke a primitive standalone
ductus --version                     # build-time version, from CARGO_PKG_VERSION
ductus --help                        # rendered by clap
```

Primitive subcommand names match their bare MCP tool names (e.g., `ductus read-spec --feature 022-deterministic-runtime`). This makes the CLI surface a 1:1 mirror of the MCP surface, which is what `framework/runtime-tools.txt` ultimately enumerates.

### Versioning: built-in from `Cargo.toml`

`ductus --version` reads `env!("CARGO_PKG_VERSION")` baked at compile time. Every `error` JSON message includes the runtime version so adopters reporting issues can attach it without an extra invocation. Per the resolved-questions section, there is no startup framework-version comparison; parse failures and primitive errors carry version metadata so mismatch is diagnosable when it arises.

### Fixture-based integration testing

Integration tests live under `runtime/tests/` and exercise the runtime against fixture repos checked into `runtime/tests/fixtures/<name>/`. Each fixture is a minimal git-tracked directory tree resembling a real `ductus`-adopting project: a `specs/<feature>/` with spec.md, plan.md, tasks.md as appropriate; a `framework/constitution.md`; a `.ductus/session.toml` at the repo root carrying the walker-context seed (session-target keys plus, for fixtures whose procedure needs more args, any non-string TOML the primitives consume). Fixtures are committed real files — no setup scripts — so they can be inspected and diffed by reviewers.

Each fixture exercises one slash command end-to-end. The test asserts:

1. The runtime's output stream matches an expected sequence of JSON messages (golden file under `runtime/tests/golden/<fixture>.jsonl`).
2. The fixture's on-disk state after the run, where a command writes, is asserted by the test itself (`ductus_basic_post_run_filesystem_state_matches_expectations` in `runtime/tests/parity.rs`).

Updating goldens uses a `BLESS=1` env var that rewrites the golden files. CI runs without the env var; failures show a diff in the test output.

The fixture repo is the integration substrate. Per-primitive unit tests under each `src/primitives/<name>.rs` cover narrower behaviors (frontmatter shapes, anchor mismatches, atomic-write crash points via interrupted writes).

### Per-command parity testing: LLM-output equivalence is approximate, not byte-equal

The spec's acceptance criterion "produces output consistent with the LLM-driven path against the same fixture, within the determinism bounds defined for each command" requires per-command "determinism bounds." For each of the six commands, the parity bound is stated in the rewritten command's frontmatter (a new `parity:` field with sub-keys):

- **`/ductus:status`** — strict byte-equality on the dashboard output.
- **`/ductus:target`** — strict byte-equality on the session file after the run. Its `parity:` field names the legacy root path `.govern.session.toml`, which the `target-basic` fixture is laid out with, as `target.md` says beside it; a current project's shared default is `.ductus/session.toml` (042, 049), and before 0.10.0 it was `.claude/gov-session.json`.
- **`/ductus:analyze`** — set-equality on the list of findings (each finding's rule ID, severity, file, line), but not on the per-finding prose (semantic extension point varies wording).
- **`/ductus:implement`** — set-equality on the set of files modified per task; checkbox state strict-equal; code content is the LLM extension point's responsibility, not the runtime's.
- **`/ductus:plan`** — frontmatter status transition strict-equal; plan/tasks body content is semantic (extension point).
- **`/ductus:specify`** — new feature directory created at the right path; spec frontmatter strict-equal; spec body is semantic.

The `parity:` frontmatter field is read by the parity-test harness in `runtime/tests/parity.rs`. The LLM-driven path is recorded once (manually, by a maintainer running the legacy prose-walked command and capturing output) and committed under `runtime/tests/parity/<command>/` as the "expected LLM output." Subsequent test runs compare the runtime's output against that capture under the declared bound.

This approach makes parity testing concrete and reviewable without requiring CI to invoke an LLM (the parity captures are committed, not regenerated).

### Slash command rewrites: order and rewrite shape

Order matches the spec's resolved Initial Scope:

1. `/ductus:status` first — smallest surface, 100% deterministic, validates the architecture end-to-end.
2. `/ductus:target` next — exercises session-file write and constitution loading.
3. `/ductus:analyze` — first command with a real extension point (`assessSpecQuality`).
4. `/ductus:implement` — largest behavioral surface, exercises `writeCode` extension point and atomic write primitives in anger.
5. `/ductus:plan` — exercises `writeSpecBody`.
6. `/ductus:specify` — second `writeSpecBody` user; validates the extension point against two callers.

Each rewrite follows the spec's Per-rewrite checklist verbatim and adds nothing new beyond it. Rewrites do not change command behavior or argument shape; the runtime executes the same procedure the LLM walked before.

### Slash command frontmatter: add a `parity:` field

One new optional frontmatter field:

- `parity: { strict-files?: [...], strict-stdout?: bool, semantic-fields?: [...] }` — per the parity testing section above.

There is no version field: the spec's Versioning-enforcement resolution rejects a startup version comparison. The field is unknown to the markdown-only path (the LLM ignores fields it doesn't recognize). It is unknown to spec 021's parseability convention (which only constrains the Instructions section). No constitution edit needed.

### Parseability check: a step in `framework-checks.yml`

A step in `framework-checks.yml` runs `bash scripts/lint-procedure-parseability.sh`, which:

1. Builds the runtime binary in `--release` mode in the workflow runner and invokes it by its relative path, `runtime/target/release/ductus` — a workflow-private compile, not an install.
2. Invokes `ductus parse --check` on every `framework/commands/*.md` and `framework/bootstrap/*.md` file, allowing legacy prose only for files listed in `runtime/legacy-prose-commands.txt`; an Invalid parse fails for every file, allowlisted or not.
3. Exits non-zero on any other failure.

Alternative considered: ship a tiny standalone parser in bash. Rejected — the parser is the single source of truth for what "parseable" means, and a bash reimplementation would drift.

### `runtime.yml` CI workflow

Triggers on PR and push to `main`, with `paths` filter covering `runtime/**` and any `framework/commands/*.md` (rewrites must keep parseability). Steps:

1. Checkout.
2. Set up the Rust toolchain with `dtolnay/rust-toolchain`, whose `toolchain:` and `components:` inputs match `runtime/rust-toolchain.toml`.
3. `cargo build --release` in `runtime/`.
4. `cargo test --release` in `runtime/`.
5. `cargo clippy -- -D warnings`.
6. `cargo fmt --check`.
7. `cargo audit` — dependency vulnerability scan against the RustSec advisory database (`BE-DEPS-001`, and the per-change scan of `BE-DEPS-007`). Policy on findings: any advisory affecting the resolved `Cargo.lock` graph fails the build; a deliberate exception requires an `[advisories] ignore` entry in `runtime/audit.toml` with a justification comment naming the RUSTSEC id and the revisit condition.

Build and test run on a matrix of `ubuntu-latest`, `macos-latest` and `windows-latest`; `cargo audit` runs once, on Linux.

**Pinned inputs** (`BE-DEPS-006`). Every action this workflow, the release workflow and the repository's other workflows use is referenced by a full commit SHA with its release tag in a same-line comment (`actions/checkout@<sha> # v7.0.1`), so an upstream that moves a tag changes nothing that runs. `dtolnay/rust-toolchain` pins a commit on its master branch and takes the version as its `toolchain:` input, because its README requires a pinned SHA to be "within the history of the master branch", which its per-version branches are not. The toolchain itself is fetched by rustup, which checks each component it downloads against the sha256 its release's channel manifest records and refuses a mismatch as `checksum failed` (rustup's `src/dist/download.rs`); the Rust Forge's channel layout gives each release its own manifest, `channel-rust-x.yy.z.toml`, with a `hash` for every package. The limit is rustup's own: it "does not yet validate signatures of downloads" (the rustup book, Security), so the manifest is trusted over HTTPS. The one local reusable workflow, `./.github/workflows/runtime-acquisition.yml`, is this repository's own file and has no upstream to pin. `cargo-audit` installs through `taiki-e/install-action` as `tool: cargo-audit@<version>`, and that action's README says it "will also verify SHA256 checksums for downloaded files". Runner labels (`ubuntu-latest` and the rest) stay as they are, since a GitHub-hosted runner image has no immutable reference to pin, and a tool a job takes from the image rather than downloading moves with it: Node in `framework-checks.yml` is the image's own (owner decision, 2026-10-20), since `actions/setup-node` downloaded it with no checksum check, from a version manifest read off a branch.

**Their update process** (`BE-DEPS-006`). The fifth check of `scripts/check-runtime-currency.sh`, `pins`, which `runtime-currency.yml` runs every Monday, compares each action pin with the action's latest release and with the commit its tag names now, each tool pin with the latest version its registry reports (crates.io's newest stable version, PyPI's latest), the adopter CI template's `DUCTUS_VERSION` with this repository's latest release and its `DUCTUS_SHA256` with the sha256 that release published, and each `toolchain:` input with `runtime/rust-toolchain.toml`. It also reports an action, `cargo install`, `pip install` or taiki-e tool that is not pinned at all. It reports and never edits: a red run is fixed by a pin bump committed on its own. Each of its lookups, the GitHub API's included, goes through one `curl` bounded by `lookup_connect_seconds` and `lookup_max_seconds` (`BE-TIMEOUT-001`), and `runtime-currency.yml` bounds the whole job with `timeout-minutes`. `scripts/tests/test-check-runtime-currency.sh`, run by `framework-checks.yml`, shows it fails on each case it claims to catch.

### Release workflow

A separate workflow at `.github/workflows/runtime-release.yml`, triggered on tag pushes matching `ductus-v*`. Steps:

1. Matrix across the target triples in the acceptance criteria: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc`.
2. Build via `cargo-zigbuild` for Linux ARM (cross-compilation), native build for macOS targets on macOS runners, and `cargo build` on a Windows runner for the Windows target. `cargo-zigbuild` installs with `cargo install --locked --version <version>`, which the Cargo book says "will install exactly that version", and a crates.io version "can never be overwritten". The zig it drives comes from PyPI's `ziglang` in pip's hash-checking mode (`--require-hashes`, against the wheel's sha256).
3. Package each binary as `ductus-<target>.tar.gz` on every platform, with a `.sha256` checksum file.
4. Upload to a GitHub release created from the tag, using `softprops/action-gh-release`, pinned by commit SHA like every action (see **Pinned inputs** above).
5. Generate a CycloneDX SBOM during the release build (`cargo cyclonedx --format json --override-filename ductus.cdx`, `BE-DEPS-003`), with `cargo-cyclonedx` installed at an exact version the same way as `cargo-zigbuild`, and upload it with its own sha256 checksum to the same release. The SBOM is derived from the resolved `Cargo.lock` graph, which is identical across the target matrix, so one SBOM covers every artifact; attached to the GitHub release, it shares the artifacts' retention lifetime.

Every `curl` download in the release path is HTTPS only at TLS 1.2 or later (`--proto '=https' --tlsv1.2`, `BE-DATA-001`), keeps curl's certificate and host-name verification, which curl's manual (8.7.1, `--insecure`) says it performs on every TLS connection unless `--insecure` is given ("curl verifies the server's TLS certificate before it continues: that the certificate contains the right name which matches the host name used in the URL and that the certificate has been signed by a CA certificate present in the cert store", `BE-DATA-011`), and is bounded (`BE-TIMEOUT-001`) by named environment variables on its step:

- **`runtime-acquisition.yml`'s** fetch of each published archive and its `.sha256`: `DOWNLOAD_CONNECT_TIMEOUT_SECONDS` (30) for the connection and `DOWNLOAD_MAX_TIME_SECONDS` (300) for the transfer, this workflow's own values; the adopter CI template names its own for the same archive and starts at the same numbers, and neither reads the other.
- **`verify-installer`'s** fetch of the published `install.sh`: `FETCH_CONNECT_TIMEOUT_SECONDS` (10) and `FETCH_MAX_TIME_SECONDS` (60) per attempt. A just-uploaded asset can answer 404 from the download host for a brief window, so the fetch is retried (`BE-RETRY-001`, `BE-RETRY-003`). Only a transient failure earns a retry: a transport failure curl reports as exit 6, 7, 28, 35, 52, 55 or 56 (in its manual: "Could not resolve host", "Failed to connect to host", "Operation timeout", "SSL connect error", "The server did not reply anything", "Failed sending network data", "Failure in receiving network data"), or an HTTP 404, 408, 429 or 5xx; any other answer fails the step at once. At most `FETCH_ATTEMPTS` (6) attempts are made. Before attempt n+1 the job waits a random half to all of `FETCH_BACKOFF_BASE_SECONDS` (5) × 2^(n−1) seconds, exponential backoff with equal jitter, so the five waits sum to between 77 and 155 seconds, never less than the 50 the fixed-delay `curl --retry 5 --retry-delay 10` it replaces always waited. A `Retry-After` that names a number of seconds sets the minimum wait, and one above `FETCH_MAX_RETRY_AFTER_SECONDS` (120) fails the step instead; a `Retry-After` given as an HTTP date is not parsed, and the jittered wait applies. A GET of a release asset is idempotent, so retrying it is safe.

The tool installs on the release path (`pip3 install --require-hashes`, the two `cargo install` steps, `taiki-e/install-action`) and `cargo audit`'s advisory fetch keep their tools' own network behavior, which the `curl` bounds above do not reach. What bounds them is the job they run in (`BE-TIMEOUT-001`): every job in `runtime.yml`, `runtime-release.yml`, `runtime-acquisition.yml` and `framework-checks.yml` sets `timeout-minutes`, as `runtime-currency.yml`'s one job already did, where GitHub's default would let a job run 360 minutes (its workflow syntax reference: "The maximum number of minutes to let a job run before GitHub automatically cancels it. Default: 360"). Each is four times the job's longest successful run among its 15 most recent runs, rounded up to a whole five minutes, and never under ten, so a slow runner does not fail a job that would have passed. Measured 2026-10-21, the longest runs were 15.2 minutes for the Windows build and test, 8.4 for a release build, 4.0 for the release's self-audit, 3.6 for the framework checks, 2.0 for the crates.io publish, 1.5 for fmt and clippy, 1.4 for the SBOM, and at most 0.5 for every other job. Two jobs are sized from their own steps' bounds instead, since a job timeout below those would stop the job before they could fire: `runtime-acquisition.yml`'s acquire job makes two downloads of at most `DOWNLOAD_CONNECT_TIMEOUT_SECONDS` plus `DOWNLOAD_MAX_TIME_SECONDS` each, 11 minutes together, and `verify-installer`'s fetch makes at most `FETCH_ATTEMPTS` attempts of `FETCH_CONNECT_TIMEOUT_SECONDS` plus `FETCH_MAX_TIME_SECONDS` each with a wait of at most `FETCH_MAX_RETRY_AFTER_SECONDS` between two, 17 minutes; each job's timeout is the next whole five minutes above that. `verify-published` sets none, because a job that calls a reusable workflow cannot (GitHub's reusing-workflows reference: "When you call a reusable workflow, you can only use the following keywords in the job containing the call", a list without `timeout-minutes`); `runtime-acquisition.yml`'s acquire job, which it runs, carries the bound. `generators.yml` is 017's surface and sets none yet.

The installer's own two requests are bounded in `install.sh`, as 061's plan §Installer states.

Tag scheme is `ductus-v<MAJOR>.<MINOR>.<PATCH>`, one tag per release, carrying the framework and the runtime in lockstep. The release procedure is `AGENTS.md`'s release entries under §Workflow.

### README placement: root `README.md` Runtime section

The runtime section is `## The runtime` in the root `README.md`. It says what the runtime is, that `/ductus` acquires it rather than the adopter installing it, how the per-machine store and the per-project pointer relate, and that it is required, and it points at `docs/runtime.md` for the operational detail.

No edits to `framework/templates/project/project-readme.md`: the template ships to adopters, and the runtime reaches them through `/ductus`, not through their README.

### Bootstrap completion-message pointer

The "Next steps" list in `framework/bootstrap/ductus-procedure.md` §Post-Scaffolding Output ends with a line saying the runtime is already acquired and wired by the run, pointing at the README's runtime section for how it is pinned and upgraded.

Per the spec's install-policy resolution, no slash command may detect-and-warn about a missing binary.

### This repository's generator scripts stay bash

The `gen-*.sh` scripts under `scripts/` that build this repository's own artifacts (`gen-claude-commands.sh`, `gen-help-tables.sh`, `gen-configure-mcp.sh`) are not wrapped or replaced by the runtime; the pre-commit hook calls them directly. The two frontmatter derivations were the exception: `derive-dependencies` and `derive-references` replaced the adopter-facing generator scripts (067's scenario `adopter-generator-promotion`). The runtime's `run-generator` primitive invokes a generator with `--dry-run` for procedure use and surfaces drift as a finding — `/{project}:analyze` runs `scripts/gen-help-tables.sh` through it — but the pre-commit path never goes through it.

### `framework/runtime-tools.txt` is the manifest

`framework/runtime-tools.txt` lists every MCP tool name the binary exposes, one bare `<verb>-<noun>` name per line under the comment header spec 021 gave it. `runtime/tests/mcp.rs` holds it set-equal to the server's `TOOL_NAMES`, so a missing entry and a phantom one both fail the suite.

`scripts/lint-tool-coverage.sh` (from spec 021) verifies that every reference to a name in this manifest appears within 20 lines of a fallback marker, except in a command carrying the §runtime-host-integration pointer, which declares the fallback once for the whole file.

### No data persistence outside session file + markdown

State management is the spec's already-resolved decision: in-memory within a run, markdown + the session file (gitignored, TOML; consolidated in 0.10.0 onto the repo-root `.govern.session.toml` from the pre-0.10.0 host- and project-name-specific `{cli-config-dir}/{project}-session.json` JSON, and at `.ductus/session.toml` since 042 and 049) are the durable journal. The plan reaffirms with concrete implementation: the interpreter holds the parsed procedure and the walk's context in `interpreter::Walker`, a plain `struct` with no `Drop`-time side effects. Process death loses this state without consequence; the user re-invokes the slash command and the runtime re-derives position from the markdown.

### Error semantics and exit codes

Already resolved in the spec; concrete mapping:

- Exit code `0` — `complete` JSON message written; clean run.
- Exit codes `1-127` — `error` JSON message written; primitive- or interpreter-level failure. Specific codes:
  - `1` — generic operational error.
  - `2` — parse error.
  - `3` — gate-confirm denied (clean denial, not an error per spec; mapped to `complete` with a `confirmed: false` payload — exit `0`. This code is reserved but unused for now; documented for forward compatibility).
  - `64-78` — `sysexits.h` ranges for I/O, permission, OS errors.
- Exit codes `128+` — signal-killed (no terminal message). Host fallback applies.

### `fetch-archive` transport

`fetch-archive` is the runtime's one outbound HTTP path (the `reqwest` entry under Trade-offs). Code holds its transport commitments, not configuration:

- **HTTPS by default** (`BE-DATA-001`). `validate_fetch_url` in `runtime/src/primitives/fetch_archive.rs` refuses every scheme but `https` before a connection is made (test `validate_fetch_url_rejects_non_https_scheme`). The one exception is opt-in: a host the operator names in `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS`, which is empty by default, is exempt from the https-only and internal-address screens. It exists for an internal mirror or local testing ([runtime-review-019-hardening](scenarios/runtime-review-019-hardening.md)). The variable is optional with no default — unset or empty exempts nothing — is read once at startup, and is listed in the environment-variable inventory, `docs/runtime.md` §Environment variables (`CFG-ENV-001`, `CFG-ENV-002`). Its type is a comma-separated list of host names, parsed once at startup (`parse_insecure_hosts`): each entry is trimmed, an empty one skipped, and the rest must each be a bare host — a name, an IPv4 literal or a bracketed IPv6 literal — kept lower-cased as a URL's host is. An entry with a scheme, port, path or credentials would never match a host, so it fails every fetch with an error naming the variable and the entry rather than silently exempting nothing (`CFG-ENV-008`); a primitive that never fetches is unaffected.
- **Every connection uses a screened address** (`BE-INPUT-007`, `BE-INPUT-016`). `validate_fetch_url` refuses a URL whose host resolves to a loopback, link-local (the cloud-metadata endpoint among them), private, unique-local or unspecified address, on the first URL and on every redirect target (`redirect_verdict`). The connection itself uses only screened addresses: the first host is pinned to the addresses the screen resolved (`resolve_to_addrs`), and every other connect-time lookup goes through `ScreeningResolver`, a `reqwest::dns::Resolve` applying the same screen, so a host that rebinds between the screen and the connect is refused on every hop ([fetch-archive-dns-rebinding](scenarios/fetch-archive-dns-rebinding.md)). A redirect host is thus looked up twice, both times screened. An allowlisted host is exempt from both screens; a configured proxy's named host is resolved at connect without the internal-address screen, and a fetch URL or redirect target naming it still passes the URL screen first. Through a proxy the proxy resolves the target and is the trust boundary. A refusal is named in the fetch's own error. Test `a_redirect_host_that_rebinds_is_refused_at_connect` stages the rebind through an injected lookup, and `a_proxy_host_is_exempt_only_at_connect` holds the proxy exemption to the connect.
- **TLS 1.2 or later.** `reqwest` is built with its `rustls` backend and default features off (`runtime/Cargo.toml`). rustls 0.23 implements only TLS 1.3 and 1.2 (`ALL_VERSIONS` in its `versions.rs`), so an older protocol cannot be negotiated.
- **Full certificate and hostname validation** (`BE-DATA-011`). rustls verifies the chain and hostname against the system trust store (`runtime/src/primitives/fetch_archive.rs` module docs). On Linux and other non-Apple Unix the roots are loaded once per process, as 048's `fetch-archive-reads-its-proxy-once` scenario requires. Nothing in the runtime disables verification: there is no `danger_accept_invalid_certs` and no custom verifier, and an allowlisted host's `https` URL is verified like any other.
- **A bounded wait** (`BE-TIMEOUT-001`). The client is built with `FETCH_TIMEOUT`, a named constant of 30 seconds in `runtime/src/primitives/fetch_archive.rs`, beside `MAX_FETCH_BYTES` and `MAX_FETCH_REDIRECTS`. `reqwest`'s blocking client applies it to the wait for the response and again to each read of the body (`blocking/client.rs` and `blocking/response.rs` in reqwest 0.13.5), so it bounds any one stall. The value is `reqwest`'s own default, declared in the runtime so that a `reqwest` upgrade cannot move it ([fetch-archive-names-its-timeout](scenarios/fetch-archive-names-its-timeout.md)). Test `a_stalled_fetch_fails_within_the_timeout_it_is_given` drives the client with a short timeout against a proxy that never answers. The whole download is bounded by `FETCH_DEADLINE`, 600 seconds per fetch, which a `Deadline` reader around the response body checks before each read, so the worst case is the deadline plus one `FETCH_TIMEOUT`; its size is bounded by `MAX_FETCH_BYTES` ([fetch-archive-bounds-the-whole-download](scenarios/fetch-archive-bounds-the-whole-download.md)). Test `a_body_that_outlasts_the_deadline_fails_naming_it` drives that reader with a short deadline over a body that outlasts it.

### Trade-offs

Considered and rejected:

- **Hand-rolled markdown lexer instead of `pulldown-cmark`** — rejected. Brings the maintenance burden of a markdown spec edge-case-tracker into this repo for no benefit. `pulldown-cmark` is widely used, well-tested, and the event stream is the right level of abstraction for the structural recognizer.
- **Separate `procedure` parser surface separate from `Instructions`** — rejected at clarify-time (spec resolution). Plan reaffirms: prose IS the procedure.
- **Cargo workspace with `runtime-core` + `runtime-cli`** — rejected. A workspace is premature factoring for a single binary with no other consumer; the boundary between primitive logic and CLI args is already small. If a second consumer ever appears, splitting is a small refactor.
- **`tracing` for structured logs** — rejected. Tracing's value is multi-target output sinks and async-aware spans; this runtime is short-lived and single-threaded for the walker. `eprintln!` on stderr is the floor; structured JSON `progress` messages on stdout are the structured channel. Adding tracing would increase dep surface and binary size for no observability win.
- **`reqwest` for HTTP** — rejected while the runtime made no outbound HTTP call, and since adopted: `reqwest` is a dependency (`runtime/Cargo.toml`) used by `fetch-archive`, which this spec's [ductus-bootstrap](scenarios/govern-bootstrap.md) scenario added to download the framework tarball `/ductus` installs. That is the runtime's one outbound HTTP path; `048-govern-acquired-runtime`'s `fetch-archive-reads-its-proxy-once` scenario has it read its proxy and certificate configuration once per process. No other primitive makes an HTTP request of its own (`lint-markdown`'s `npx` may fetch `markdownlint-cli2`, which is npm's request, not the runtime's).
- **Daemon / persistent mode** — explicit non-goal in the spec.
- **In-process LLM client** — explicit non-goal (`Determinism only` principle).
- **Build the runtime in the framework workflow on every PR** — rejected while only the parseability check needed a binary and the runtime-absent path had to stay provable; adopted since `048-govern-acquired-runtime` made the runtime required, and `framework-checks.yml` builds it for the derivation primitives.
- **Cross-platform parity testing in CI** — first out of scope, since adopted: `runtime.yml` runs the suite, parity tests included, on Linux, macOS and Windows, and the release workflow's acquire jobs run each built binary.
- **Embedding a markdown linter natively** — explicit non-goal in spec; runtime wraps `npx markdownlint-cli2` via the `lint-markdown` primitive. Wrapping a tool means waiting on a child process, so `lint-markdown` and `run-generator` each run theirs under a bounded timeout (`BE-TIMEOUT-001`) — named constants in their modules, `LINT_MARKDOWN_TIMEOUT` and `GENERATOR_TIMEOUT`, five minutes each — and a child that outlives it is killed and the call errors naming the timeout. Both spawn their child with an argument vector, never a shell string (`BE-INPUT-003`). `run-generator` passes the script path and `--dry-run` as arguments to `bash` (`runtime/src/primitives/run_generator.rs`). `lint-markdown` resolves `npx` without a login shell ([lint-markdown-tool-resolution](scenarios/lint-markdown-tool-resolution.md)).

### Known limitations

- **No retry / no resume**: a runtime crash mid-procedure loses position; the user re-invokes the command and the runtime walks the markdown to figure out where to resume. This is acceptable per the spec's state-management resolution but means long procedures with many tasks re-do all the boilerplate (re-reads files, re-parses) on resume. Mitigation: primitives are cheap enough that re-doing them is sub-second.
- **`pulldown-cmark` edge cases**: certain markdown extensions (tables, footnotes, definition lists) are off by default in `pulldown-cmark`. The procedure parser opts into tables; other extensions are off. If a slash command body relies on an extension we don't opt into, the parser will silently misread it. Mitigation: the parseability check on every PR catches this at the earliest opportunity.
- **Windows atomic-write semantics**: weaker than POSIX. A crash on Windows between write and rename can leave a partial tempfile in the parent directory (not the target file, which is unchanged). Cleanup is a manual `rm`.
- **Fixture maintenance cost**: every command rewrite needs a fixture under `runtime/tests/fixtures/`. The first fixture (for `/ductus:status`) costs the most to build; subsequent fixtures fork from it. The cost is bounded but real.
- **Parity captures are manual**: the LLM-output captures under `runtime/tests/parity/` are taken once per command by a maintainer and committed. If the LLM-driven path's output changes (because constitution updates change what `/ductus:analyze` flags, for instance), the captures must be re-taken. There is no automatic regeneration; that would require an LLM in CI, which the spec rules out.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/Cargo.toml` | Create | Crate manifest, dependencies, MSRV |
| `runtime/Cargo.lock` | Create | Lockfile, committed |
| `runtime/src/main.rs` | Create | CLI entrypoint, clap dispatch |
| `runtime/src/lib.rs` | Create | Module re-exports for integration tests |
| `runtime/src/parser/` | Create | Procedure parser over `pulldown-cmark` events |
| `runtime/src/interpreter/` | Create | Procedure walker, JSON protocol I/O |
| `runtime/src/primitives/` | Create | One module per primitive |
| `runtime/src/mcp/` | Create | `rmcp`-based MCP server |
| `runtime/src/schema/` | Create | Typed schemas for primitives, protocol, extension points |
| `runtime/src/io.rs` | Create | Stdio framing for protocol messages and progress |
| `runtime/tests/` | Create | Integration tests + golden output comparisons |
| `runtime/tests/fixtures/` | Create | Per-command fixture repos |
| `runtime/tests/golden/` | Create | Expected JSON message streams |
| `runtime/tests/parity/` | Create | Captured LLM-driven outputs for parity comparison |
| `runtime/legacy-prose-commands.txt` | Create | Allowlist of not-yet-rewritten command files |
| `runtime/CHANGELOG.md` | Create | Runtime release notes |
| `runtime/.gitignore` | Create | Exclude `target/` |
| `framework/runtime-tools.txt` | Edit | Every MCP tool name the binary exposes |
| `framework/commands/status.md` | Edit | Rewrite Instructions to parseable conventions |
| `framework/commands/target.md` | Edit | Rewrite Instructions to parseable conventions |
| `framework/commands/analyze.md` | Edit | Rewrite Instructions to parseable conventions; add `assessSpecQuality` extension point marker |
| `framework/commands/implement.md` | Edit | Rewrite Instructions to parseable conventions; add `writeCode` extension point marker |
| `framework/commands/plan.md` | Edit | Rewrite Instructions to parseable conventions; add `writeSpecBody` extension point marker |
| `framework/commands/specify.md` | Edit | Rewrite Instructions to parseable conventions; add `writeSpecBody` extension point marker |
| `scripts/lint-procedure-parseability.sh` | Create | Bash wrapper that builds runtime and invokes `ductus parse --check` |
| `.github/workflows/framework-checks.yml` | Edit | The parseability check step |
| `.github/workflows/runtime.yml` | Create | Per-PR build + test + clippy + fmt + `cargo audit` dependency scan |
| `.github/workflows/runtime-release.yml` | Create | Tag-triggered cross-compile + release upload + CycloneDX SBOM |
| `.github/workflows/runtime-acquisition.yml` | Edit | The published archive's fetch: TLS floor and named bounds |
| `.github/workflows/runtime-currency.yml` | Edit | The weekly currency check, which runs the pins check |
| `scripts/check-runtime-currency.sh` | Edit | The `pins` check: every CI pin against its upstream |
| `scripts/tests/test-check-runtime-currency.sh` | Create | The cases the currency check claims to catch |
| `runtime/rust-toolchain.toml` | Edit | The pinned toolchain each workflow's `toolchain:` input matches |
| `framework/bootstrap/ductus-procedure.md` | Edit | The runtime line in the §Post-Scaffolding Output next steps |
| `README.md` | Edit | The runtime section: acquired by `/ductus`, store and pointer, required |
| `docs/runtime.md` | Edit | §Environment variables: the `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` row, its type and its fail-fast parse (`CFG-ENV-002`) |
| `specs/022-deterministic-runtime/plan.md` | Create | This file |
| `specs/022-deterministic-runtime/tasks.md` | Create | Task breakdown |
| `specs/022-deterministic-runtime/data-model.md` | Create | Procedure AST, JSON protocol, primitive and extension-point schemas |

## Trade-offs

See the Considered and rejected and Known limitations subsections in Technical Decisions above. The major trade-offs are: dependency surface (kept tight); Windows atomicity (accepted as weaker); fixture maintenance cost (accepted as bounded); parity captures committed-not-regenerated (accepted to keep LLMs out of CI).
