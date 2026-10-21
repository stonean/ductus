//! Canonical primitive-name registry.
//!
//! Single source of truth for the runtime's primitive names. The parser's
//! `PRIMITIVE_NAMES` is [`PRIMITIVE_REGISTRY`], and the MCP server's
//! `tool_names` is the registry less [`CLI_ONLY_PRIMITIVES`], so neither can
//! drift; `framework/runtime-tools.txt` (the shipped manifest of MCP tools)
//! is asserted set-equal to the served tools in `runtime/tests/mcp.rs`, and
//! the interpreter's `dispatch_primitive` is asserted to handle every registry
//! name in `interpreter::tests`.
//!
//! Two hand-written surfaces cannot consume a const slice — the rmcp
//! `#[tool]` methods and the clap subcommand enum in `main.rs` — and each is
//! held to this slice by a **set-equality** test instead: `tests/mcp.rs` lists
//! the served tools against `tool_names`, and
//! `main::tests::every_registry_primitive_has_a_clap_subcommand` compares the
//! subcommand names clap exposes against this registry. Both directions fail:
//! a registry name with no surface is unreachable, and a surface with no
//! registry entry offers a verb the canonical set does not define.
//!
//! Every registration surface is therefore covered by a test. Until 2026-09-12
//! the clap enum was the exception — deleting a variant and its dispatch arm
//! left the whole suite green, so a primitive could be absent from the
//! `ductus <name>` CLI with nothing reporting it, on exactly the surface the
//! markdown-only path depends on. That is why [`PRIMITIVE_REGISTRY`] is `pub`
//! rather than `pub(crate)`: `main.rs` is a separate crate from the library
//! and has to be able to name the canonical set to be tested against it.

/// Every primitive name the runtime defines. Its order is its own — the
/// tests compare it, less [`CLI_ONLY_PRIMITIVES`], with
/// `framework/runtime-tools.txt` as a set. Names
/// are bare `<verb>-<noun>` strings; server-level namespacing (`ductus`) is
/// supplied by the host's MCP registration.
pub const PRIMITIVE_REGISTRY: &[&str] = &[
    "read-spec",
    "read-tasks",
    "mark-task",
    "mark-criterion",
    "set-status",
    "derive-boundary",
    "discover-rule-files",
    "process-waivers",
    "process-decisions",
    "compute-review-scope",
    "write-review",
    "write-analysis",
    "check-step-references",
    "check-stuck",
    "validate-frontmatter",
    "resolve-anchor",
    "traverse-deps",
    "check-rule-ids",
    "run-generator",
    "lint-markdown",
    "gate-confirm",
    "fetch-archive",
    "extract-archive",
    "apply-manifest",
    "enforce-manifest",
    "merge-managed-block",
    "merge-permissions",
    "migrate-session-file",
    "relocate-audit-records",
    "create-scenario",
    "append-task",
    "label-criteria",
    "prune-tasks",
    "prune-plan",
    "dashboard",
    "write-session",
    "resolve-session",
    "retarget-sessions",
    "resolve-references",
    "resolve-constitutions",
    "resolve-feature",
    "create-feature",
    "create-plan-artifacts",
    "check-review-gate",
    "append-question",
    "diff-cross-spec",
    "append-inbox",
    "remove-inbox-item",
    "check-artifacts",
    "check-artifact-size",
    "derive-routing-candidates",
    "check-corpus-links",
    "check-orphaned-references",
    "check-command-flags",
    "check-promotion-coverage",
    "derive-dependencies",
    "derive-references",
    "check-unfolded-specs",
    "rewrite-spec-links",
    "retire-feature",
    "invalidate-review",
];

/// Primitives the MCP server does not serve, each reachable only through the
/// `ductus <name>` CLI and the `ductus exec` walker. A host auto-approves the
/// server's tools, so a primitive here is one an agent must not be able to
/// call without the host's prompt (022's plan, Agent reach):
///
/// - `run-generator` hands any project file to `bash`, so any ductus tool
///   that writes a project file would otherwise make it run any command
///   (022 task 14, the owner's answer (b)). Its CLI and `exec` forms are
///   kept out of the shell allow `/ductus` seeds by 022 task 18.
pub const CLI_ONLY_PRIMITIVES: &[&str] = &["run-generator"];
