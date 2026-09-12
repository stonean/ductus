//! Canonical primitive-name registry.
//!
//! Single source of truth for the runtime's primitive names. The parser's
//! `PRIMITIVE_NAMES` and the MCP server's `TOOL_NAMES` are both defined
//! from [`PRIMITIVE_REGISTRY`], so neither can drift; `framework/runtime-tools.txt`
//! (the shipped manifest) is asserted set-equal in `runtime/tests/mcp.rs`, and
//! the interpreter's `dispatch_primitive` is asserted to handle every registry
//! name in `interpreter::tests`.
//!
//! Two hand-written surfaces cannot consume a const slice — the rmcp
//! `#[tool]` methods and the clap subcommand enum in `main.rs` — and each is
//! held to this slice by a **set-equality** test instead: `tests/mcp.rs` lists
//! the served tools against `TOOL_NAMES`, and
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

/// Every primitive name exposed by the runtime, in manifest order. Names
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
    "create-scenario",
    "append-task",
    "label-criteria",
    "prune-tasks",
    "dashboard",
    "write-session",
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
    "derive-routing-candidates",
    "check-corpus-links",
    "check-orphaned-references",
    "check-command-flags",
    "check-review-agreement",
    "derive-dependencies",
    "derive-references",
    "check-unfolded-specs",
    "rewrite-spec-links",
    "retire-feature",
    "invalidate-review",
];
