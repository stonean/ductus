//! MCP server exposing every primitive but the CLI-only ones as a tool.
//!
//! Tool names are bare `<verb>-<noun>` strings (e.g. `read-spec`).
//! Server-level namespacing is supplied by the MCP server registration —
//! the adopter registers this server as `ductus` in `.mcp.json`, which
//! makes the Claude Code-side wire identifier `mcp__ductus__<verb>-<noun>`.
//! Tools are async wrappers around the synchronous
//! `primitives::<name>::run` functions; the server holds an `Arc<PathBuf>`
//! to the repo root that every primitive operates on. Blocking-heavy
//! primitives (network downloads, subprocess shell-outs, whole-archive
//! decompression, full-history git revwalks, and the session primitives'
//! lock waits) are dispatched through [`dispatch_blocking`] so they run on
//! tokio's blocking pool instead of pinning an async worker thread.
//!
//! `gate-confirm` is special-cased: the MCP surface returns the prompt
//! payload (gate + prompt + fresh request-id) without blocking. The LLM
//! orchestrator is responsible for routing the prompt to the user. The
//! subprocess-interpreter surface (CLI `ductus gate-confirm`) handles
//! the blocking handshake via [`crate::primitives::gate_confirm::run_blocking`].

use std::collections::HashSet;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use rmcp::ErrorData;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::model::{CallToolResponse, Implementation, JsonObject, ServerCapabilities, ServerConfig};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use serde_json::Value;

use crate::primitives;
use crate::primitives::gate_confirm::GatePromptPayload;
use crate::schema::primitives::{
    AppendInboxArgs, AppendInboxResult, AppendQuestionArgs, AppendQuestionResult, AppendTaskArgs,
    AppendTaskResult, ApplyManifestArgs, ApplyManifestResult, CheckArtifactSizeArgs,
    CheckArtifactSizeResult, CheckArtifactsArgs, CheckArtifactsResult, CheckCommandFlagsArgs,
    CheckCommandFlagsResult, CheckCorpusLinksArgs, CheckCorpusLinksResult,
    CheckOrphanedReferencesArgs, CheckOrphanedReferencesResult, CheckPromotionCoverageArgs,
    CheckPromotionCoverageResult, CheckReviewGateArgs, CheckReviewGateResult, CheckRuleIdsArgs,
    CheckRuleIdsResult, CheckStepReferencesArgs, CheckStepReferencesResult, CheckStuckArgs,
    CheckStuckResult, CheckUnfoldedSpecsArgs, CheckUnfoldedSpecsResult, CheckboxToggleResult,
    ComputeReviewScopeArgs, ComputeReviewScopeResult, CreateFeatureArgs, CreateFeatureResult,
    CreatePlanArtifactsArgs, CreatePlanArtifactsResult, CreateScenarioArgs, CreateScenarioResult,
    DashboardArgs, DashboardResult, DeriveBoundaryArgs, DeriveBoundaryResult,
    DeriveDependenciesArgs, DeriveDependenciesResult, DeriveReferencesArgs, DeriveReferencesResult,
    DeriveRoutingCandidatesArgs, DeriveRoutingCandidatesResult, DiffCrossSpecArgs,
    DiffCrossSpecResult, DiscoverRuleFilesArgs, DiscoverRuleFilesResult, EnforceManifestArgs,
    EnforceManifestResult, ExtractArchiveArgs, ExtractArchiveResult, FetchArchiveArgs,
    FetchArchiveResult, GateConfirmArgs, InvalidateReviewArgs, InvalidateReviewResult,
    LabelCriteriaArgs, LabelCriteriaResult, LintMarkdownArgs, LintMarkdownResult,
    MarkCriterionArgs, MarkTaskArgs, MergeManagedBlockArgs, MergeManagedBlockResult,
    MergePermissionsArgs, MergePermissionsResult, MigrateSessionFileArgs, MigrateSessionFileResult,
    ProcessDecisionsArgs, ProcessDecisionsResult, ProcessWaiversArgs, ProcessWaiversResult,
    PrunePlanArgs, PrunePlanResult, PruneTasksArgs, PruneTasksResult, ReadSpecArgs, ReadSpecResult,
    ReadTasksArgs, ReadTasksResult, RelocateAuditRecordsArgs, RelocateAuditRecordsResult,
    RemoveInboxItemArgs, RemoveInboxItemResult, ResolveAnchorArgs, ResolveAnchorResult,
    ResolveConstitutionsArgs, ResolveConstitutionsResult, ResolveFeatureArgs, ResolveFeatureResult,
    ResolveReferencesArgs, ResolveReferencesResult, ResolveSessionArgs, ResolveSessionResult,
    RetargetSessionsArgs, RetargetSessionsResult, RetireFeatureArgs, RetireFeatureResult,
    RewriteSpecLinksArgs, RewriteSpecLinksResult, SetStatusArgs, SetStatusResult, TraverseDepsArgs,
    TraverseDepsResult, ValidateFrontmatterArgs, ValidateFrontmatterResult, WriteAnalysisArgs,
    WriteAnalysisResult, WriteReviewArgs, WriteReviewResult, WriteSessionArgs, WriteSessionResult,
};

/// The MCP tool names the server exposes, in registry order: the canonical
/// registry (`crate::schema::registry::PRIMITIVE_REGISTRY`) less
/// [`crate::schema::registry::CLI_ONLY_PRIMITIVES`]
/// ([`crate::schema::registry::served_primitives`]), so it follows the parser's
/// [`crate::parser::PRIMITIVE_NAMES`] by construction. The `#[tool]` methods
/// below stay hand-written (the rmcp macro needs literal method items);
/// `runtime/tests/mcp.rs` asserts the served set equals this one.
pub fn tool_names() -> impl Iterator<Item = &'static str> {
    crate::schema::registry::served_primitives()
}

/// MCP server. Cloned per request by `rmcp`, so all state lives behind
/// `Arc` and tool dispatch is `&self`.
#[derive(Clone)]
pub struct GovRuntimeServer {
    repo: Arc<PathBuf>,
    // Dispatch router the `#[tool_handler(router = self.tool_router)]` impl
    // reads for both `list_tools` and `call_tool`. Built once in `new()` and
    // post-processed by `strip_nonstandard_formats` so every served schema is
    // sanitized. The handler must point here rather than at its default
    // `Self::tool_router()`, which would regenerate a fresh, unsanitized
    // router on every call.
    tool_router: ToolRouter<Self>,
}

impl GovRuntimeServer {
    /// Construct a new server rooted at `repo` (the path every primitive
    /// resolves relative paths against).
    #[must_use]
    pub fn new(repo: PathBuf) -> Self {
        let mut tool_router = Self::tool_router();
        strip_nonstandard_formats(&mut tool_router);
        reject_unknown_fields(&mut tool_router);
        Self {
            repo: Arc::new(repo),
            tool_router,
        }
    }

    fn repo(&self) -> &std::path::Path {
        self.repo.as_path()
    }
}

/// Strip non-standard numeric `format` hints from every tool schema.
///
/// `schemars` stamps OpenAPI-style `format` values (`uint32`, `uint64`,
/// `uint8`, …) onto every Rust integer/float field. JSON Schema defines
/// no `format` values for numeric types, so strict MCP clients (opencode
/// and other Ajv-based validators) log `unknown format "uint32" ignored`
/// for each occurrence in a tool's input/output schema. The hints carry no
/// validation meaning for our consumers, so we drop them at construction —
/// leaving string formats (`date-time`, `uri`, `uuid`, …) untouched.
fn strip_nonstandard_formats(router: &mut ToolRouter<GovRuntimeServer>) {
    for route in router.map.values_mut() {
        route.attr.input_schema = sanitized_schema(&route.attr.input_schema);
        if let Some(output) = route.attr.output_schema.as_ref() {
            route.attr.output_schema = Some(sanitized_schema(output));
        }
    }
}

/// Reject MCP tool calls carrying an argument key outside the target
/// primitive's known field set.
///
/// No primitive `Args` struct sets `#[serde(deny_unknown_fields)]`, and it
/// cannot: the subprocess interpreter binds every primitive's args from a
/// clone of the whole walker context (a deliberate superset — see
/// [`crate::interpreter`]'s `dispatch_primitive`), so the shared `Args`
/// structs must stay lenient or every exec-path call would be rejected.
/// Strictness therefore lives at the MCP boundary only. This pass derives
/// each tool's allowlist from the field names already present in its input
/// schema (`route.attr.input_schema["properties"]` — the schema `schemars`
/// produces, so the list can never drift from the `Args` type) and wraps the
/// route's call so an unknown key becomes a clear `invalid_params` rejection
/// naming the field, before the macro's lenient `Parameters` deserialize
/// would silently default it. The exec path never constructs this router, so
/// its superset-context binding is unaffected.
///
/// No `Args` struct uses `#[serde(flatten)]`, so `properties` enumerates
/// every known field; a schema with no `properties` object (none today) is
/// left unwrapped rather than rejecting all keys.
fn reject_unknown_fields(router: &mut ToolRouter<GovRuntimeServer>) {
    for route in router.map.values_mut() {
        let Some(allowed) = known_field_names(&route.attr) else {
            continue;
        };
        let allowed = Arc::new(allowed);
        let inner = Arc::clone(&route.call);
        route.call = Arc::new(
            move |ctx: ToolCallContext<'_, GovRuntimeServer>| -> Pin<
                Box<dyn Future<Output = Result<CallToolResponse, ErrorData>> + Send + '_>,
            > {
                if let Some(arguments) = ctx.arguments.as_ref()
                    && let Some(unknown) =
                        arguments.keys().find(|key| !allowed.contains(key.as_str()))
                {
                    let error = unknown_field_error(ctx.name(), unknown, &allowed);
                    return Box::pin(std::future::ready(Err(error)));
                }
                (inner)(ctx)
            },
        );
    }
}

/// The set of wire field names a primitive accepts, read from its input
/// schema's `properties`. `None` when the schema carries no `properties`
/// object.
fn known_field_names(tool: &rmcp::model::Tool) -> Option<HashSet<String>> {
    let properties = tool.input_schema.get("properties")?.as_object()?;
    Some(properties.keys().cloned().collect())
}

/// Build the `invalid_params` rejection naming the unknown field and listing
/// the known ones (sorted, for a deterministic message).
fn unknown_field_error(tool: &str, field: &str, allowed: &HashSet<String>) -> ErrorData {
    let mut known: Vec<&str> = allowed.iter().map(String::as_str).collect();
    known.sort_unstable();
    ErrorData::invalid_params(
        format!(
            "unknown field `{field}` for tool `{tool}`; known fields: {}",
            known.join(", ")
        ),
        None,
    )
}

/// Clone a tool schema, drop each numeric node's `format`, and re-wrap it.
fn sanitized_schema(schema: &Arc<JsonObject>) -> Arc<JsonObject> {
    let mut value = Value::Object(schema.as_ref().clone());
    strip_numeric_formats(&mut value);
    let Value::Object(map) = value else {
        unreachable!("a JSON Schema root is always an object");
    };
    Arc::new(map)
}

/// Recursively remove `format` from any node typed `integer` or `number`.
fn strip_numeric_formats(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if node_is_numeric(map) {
                map.remove("format");
            }
            for child in map.values_mut() {
                strip_numeric_formats(child);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(strip_numeric_formats),
        _ => {}
    }
}

/// Run a blocking primitive on tokio's dedicated blocking pool and map its
/// outcome into the tool-handler result shape.
///
/// `fetch-archive` drives reqwest's *blocking* client, which panics when
/// invoked from an async context in debug builds (reqwest guards
/// blocking-in-async under `cfg(debug_assertions)`) and would pin a tokio
/// worker thread for the whole download in release. `lint-markdown` shells
/// out via `std::process::Command`, and
/// `extract-archive` decompresses entire archives — the same
/// worker-pinning hazard, so they take the same seam. The git-walk tools
/// (`check-stuck`, `compute-review-scope`, `derive-boundary`,
/// `diff-cross-spec`) run a full-history `libgit2` revwalk that is likewise
/// CPU-bound and unbounded in a large adopter repo, so they route here too,
/// as do `check-review-gate`, which shells out to the markdown linter and
/// reads history for its rename exemption, and `dashboard`, which blames the
/// inbox for its oldest item. `write-session`, `resolve-session` and
/// `retarget-sessions` wait on the sessions lock for up to ten seconds, so
/// they take the seam as well. The CLI surfaces
/// (`main.rs` subcommands and the subprocess interpreter) call the
/// primitives directly with no tokio runtime and are unaffected.
async fn dispatch_blocking<T, F>(work: F) -> Result<Json<T>, String>
where
    F: FnOnce() -> primitives::Result<T> + Send + 'static,
    T: Send + 'static,
{
    match tokio::task::spawn_blocking(work).await {
        Ok(outcome) => outcome.map(Json).map_err(|e| e.to_string()),
        // A join error means the blocking task panicked or the runtime is
        // shutting down; surface it as a tool error instead of unwinding.
        Err(join_error) => Err(format!("blocking primitive task failed: {join_error}")),
    }
}

/// A schema node is numeric when its `type` is (or includes) `integer` or
/// `number` — the two JSON Schema types that admit no standard `format`.
fn node_is_numeric(map: &JsonObject) -> bool {
    fn is_numeric_type(ty: &Value) -> bool {
        matches!(ty.as_str(), Some("integer" | "number"))
    }
    match map.get("type") {
        Some(ty @ Value::String(_)) => is_numeric_type(ty),
        Some(Value::Array(types)) => types.iter().any(is_numeric_type),
        _ => false,
    }
}

#[tool_router]
impl GovRuntimeServer {
    #[tool(
        name = "read-spec",
        description = "Parse spec frontmatter and body sections."
    )]
    async fn read_spec(
        &self,
        params: Parameters<ReadSpecArgs>,
    ) -> Result<Json<ReadSpecResult>, String> {
        primitives::read_spec::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "read-tasks",
        description = "Parse `tasks.md` into a structured task list."
    )]
    async fn read_tasks(
        &self,
        params: Parameters<ReadTasksArgs>,
    ) -> Result<Json<ReadTasksResult>, String> {
        primitives::read_tasks::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "mark-task",
        description = "Flip a single subtask checkbox in `tasks.md` (atomic rewrite)."
    )]
    async fn mark_task(
        &self,
        params: Parameters<MarkTaskArgs>,
    ) -> Result<Json<CheckboxToggleResult>, String> {
        primitives::mark_task::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "mark-criterion",
        description = "Flip a single acceptance-criterion checkbox in `spec.md`."
    )]
    async fn mark_criterion(
        &self,
        params: Parameters<MarkCriterionArgs>,
    ) -> Result<Json<CheckboxToggleResult>, String> {
        primitives::mark_criterion::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "set-status",
        description = "Update the `status:` field in spec frontmatter, guarded by `from`."
    )]
    async fn set_status(
        &self,
        params: Parameters<SetStatusArgs>,
    ) -> Result<Json<SetStatusResult>, String> {
        primitives::set_status::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "derive-boundary",
        description = "Derive the runtime write boundary from git history."
    )]
    async fn derive_boundary(
        &self,
        params: Parameters<DeriveBoundaryArgs>,
    ) -> Result<Json<DeriveBoundaryResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::derive_boundary::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "diff-cross-spec",
        description = "Compute /ductus:implement's cross-spec impact surface: diff the feature's first spec-dir commit against the working tree, scoped to the spec root and filtered to paths outside the feature's own directory. {specs-root}/inbox.md is excluded: it holds what a person logs, which is no spec's impact. Read-only; empty lists are the no-impact outcome."
    )]
    async fn diff_cross_spec(
        &self,
        params: Parameters<DiffCrossSpecArgs>,
    ) -> Result<Json<DiffCrossSpecResult>, String> {
        // Full-history revwalk — blocking pool, like derive-boundary.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::diff_cross_spec::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "check-stuck",
        description = "Count `tasks.md` commits since status entered `in-progress`."
    )]
    async fn check_stuck(
        &self,
        params: Parameters<CheckStuckArgs>,
    ) -> Result<Json<CheckStuckResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::check_stuck::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "validate-frontmatter",
        description = "Validate frontmatter shape against the pipeline schema."
    )]
    async fn validate_frontmatter(
        &self,
        params: Parameters<ValidateFrontmatterArgs>,
    ) -> Result<Json<ValidateFrontmatterResult>, String> {
        primitives::validate_frontmatter::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "resolve-anchor",
        description = "Verify `§anchor` references resolve to `<!-- §anchor -->` markers. Markers come from `path` itself by default (self-consistency); pass `markers-path` to resolve one file's references against another file's markers (e.g. a spec's references against the constitution)."
    )]
    async fn resolve_anchor(
        &self,
        params: Parameters<ResolveAnchorArgs>,
    ) -> Result<Json<ResolveAnchorResult>, String> {
        primitives::resolve_anchor::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "traverse-deps",
        description = "Traverse spec dependencies and check status compatibility."
    )]
    async fn traverse_deps(
        &self,
        params: Parameters<TraverseDepsArgs>,
    ) -> Result<Json<TraverseDepsResult>, String> {
        primitives::traverse_deps::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-rule-ids",
        description = "Verify cited rule IDs exist in rule files and aren't deprecated."
    )]
    async fn check_rule_ids(
        &self,
        params: Parameters<CheckRuleIdsArgs>,
    ) -> Result<Json<CheckRuleIdsResult>, String> {
        primitives::check_rule_ids::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-promotion-coverage",
        description = "How much of a rules file is routed to its canonical source: rule-bearing minus (table-keyed union pointer-citing). A notice, never a gate."
    )]
    async fn check_promotion_coverage(
        &self,
        params: Parameters<CheckPromotionCoverageArgs>,
    ) -> Result<Json<CheckPromotionCoverageResult>, String> {
        primitives::check_promotion_coverage::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "lint-markdown",
        description = "Wrap `npx markdownlint-cli2` and surface violations."
    )]
    async fn lint_markdown(
        &self,
        params: Parameters<LintMarkdownArgs>,
    ) -> Result<Json<LintMarkdownResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::lint_markdown::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "gate-confirm",
        description = "Return the gate prompt payload (non-blocking). The orchestrator routes the prompt to the user and supplies the confirmed decision out-of-band; this tool never blocks waiting for input."
    )]
    async fn gate_confirm(
        &self,
        params: Parameters<GateConfirmArgs>,
    ) -> Result<Json<GatePromptPayload>, String> {
        let request_id = primitives::gate_confirm::fresh_request_id();
        Ok(Json(primitives::gate_confirm::prompt_payload(
            &params.0,
            &request_id,
        )))
    }

    #[tool(
        name = "fetch-archive",
        description = "Download an archive plus its sha256 sidecar and verify the hash."
    )]
    async fn fetch_archive(
        &self,
        params: Parameters<FetchArchiveArgs>,
    ) -> Result<Json<FetchArchiveResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::fetch_archive::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "extract-archive",
        description = "Extract a local `.tar.gz` / `.zip` archive into a destination directory."
    )]
    async fn extract_archive(
        &self,
        params: Parameters<ExtractArchiveArgs>,
    ) -> Result<Json<ExtractArchiveResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::extract_archive::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "apply-manifest",
        description = "Strategy-aware bulk substitute + write driven by a manifest."
    )]
    async fn apply_manifest(
        &self,
        params: Parameters<ApplyManifestArgs>,
    ) -> Result<Json<ApplyManifestResult>, String> {
        primitives::apply_manifest::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "enforce-manifest",
        description = "Remove files in a directory that are not in the expected manifest."
    )]
    async fn enforce_manifest(
        &self,
        params: Parameters<EnforceManifestArgs>,
    ) -> Result<Json<EnforceManifestResult>, String> {
        primitives::enforce_manifest::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "merge-managed-block",
        description = "Idempotently merge a framework-managed block with configurable marker shape."
    )]
    async fn merge_managed_block(
        &self,
        params: Parameters<MergeManagedBlockArgs>,
    ) -> Result<Json<MergeManagedBlockResult>, String> {
        primitives::merge_managed_block::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "merge-permissions",
        description = "Idempotently merge a canonical permission allow/deny set into a JSON file with exact-match dedup."
    )]
    async fn merge_permissions(
        &self,
        params: Parameters<MergePermissionsArgs>,
    ) -> Result<Json<MergePermissionsResult>, String> {
        primitives::merge_permissions::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "relocate-audit-records",
        description = "Move a spec's `review:` and `analyze:` frontmatter blocks into the artifacts that own them (review.md / analysis.md) and remove them from spec.md \u{2014} the sweep behind spec 057's migration. Idempotent: a spec carrying no block is a no-op reporting `changed: false`, so a re-run over a partially migrated corpus completes the remainder. Never invents a record for a spec that has none, never rewrites body prose (the edit is frontmatter-only, so \u{a7}spec-lifecycle's mechanical-edit rule applies and a `done` spec stays `done`), and never overwrites an artifact that already exists \u{2014} that conflict is reported with both copies left intact, since which record is current is a judgement the primitive cannot date."
    )]
    async fn relocate_audit_records(
        &self,
        params: Parameters<RelocateAuditRecordsArgs>,
    ) -> Result<Json<RelocateAuditRecordsResult>, String> {
        primitives::relocate_audit_records::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "migrate-session-file",
        description = "Translate a pre-0.10.0 legacy session JSON (`.claude/{project}-session.json`) into the consolidated `.ductus/session.toml`, applying camelCase→kebab-case key renames (`scenarioPath`→`scenario-path`, `setAt`→`set-at`), and delete the legacy file. Idempotent: no-op when no legacy file is present; preserves any existing `.ductus/session.toml`."
    )]
    async fn migrate_session_file(
        &self,
        params: Parameters<MigrateSessionFileArgs>,
    ) -> Result<Json<MigrateSessionFileResult>, String> {
        primitives::migrate_session_file::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "label-criteria",
        description = "Assign stable AC{n} labels to a spec's acceptance criteria and maintain the next-criterion counter (idempotent; already-labelled criteria are left byte-identical)."
    )]
    async fn label_criteria(
        &self,
        params: Parameters<LabelCriteriaArgs>,
    ) -> Result<Json<LabelCriteriaResult>, String> {
        primitives::label_criteria::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "create-scenario",
        description = "Write a new scenarios/{slug}.md file under a feature with frontmatter and prose body."
    )]
    async fn create_scenario(
        &self,
        params: Parameters<CreateScenarioArgs>,
    ) -> Result<Json<CreateScenarioResult>, String> {
        primitives::create_scenario::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "discover-rule-files",
        description = "Select rule files for /ductus:review by suffix, [rules] surfaces, and disabled-rule-files, with the notice lines to emit, among them one per unknown key in [rules] or [review]."
    )]
    async fn discover_rule_files(
        &self,
        params: Parameters<DiscoverRuleFilesArgs>,
    ) -> Result<Json<DiscoverRuleFilesResult>, String> {
        primitives::discover_rule_files::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "process-waivers",
        description = "Classify the waivers recorded in a spec's review.md against currently-firing findings (apply/expire/retain/dedup). Pass skipped-pass for each dimension not run this pass: on a dimension-restricted run a non-firing waiver is retained (kept in frontmatter), never expired, so a partial run cannot prune a waiver anchored to a skipped dimension."
    )]
    async fn process_waivers(
        &self,
        params: Parameters<ProcessWaiversArgs>,
    ) -> Result<Json<ProcessWaiversResult>, String> {
        primitives::process_waivers::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "process-decisions",
        description = "Classify the routed and discarded decisions recorded in a spec's review.md or analysis.md against this run's finding keys (matched/expired/retained/dedup), before the host proposes dispositions, so a settled finding is counted under its stored outcome and not asked about again. Pass restricted when the run did not evaluate every source: a non-firing decision is then retained, never expired. Read-only; malformed entries are reported and never pruned."
    )]
    async fn process_decisions(
        &self,
        params: Parameters<ProcessDecisionsArgs>,
    ) -> Result<Json<ProcessDecisionsResult>, String> {
        primitives::process_decisions::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "compute-review-scope",
        description = "Resolve /ductus:review's diff-base (the parent of the commit the spec entered in-progress at, or a --since override) and its file scope (the union of the plan's Affected Files and the files modified since the diff base)."
    )]
    async fn compute_review_scope(
        &self,
        params: Parameters<ComputeReviewScopeArgs>,
    ) -> Result<Json<ComputeReviewScopeResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::compute_review_scope::run(&params.0, repo.as_path()))
            .await
    }

    #[tool(
        name = "write-review",
        description = "Render specs/NNN/review.md — the review report and, in that same file's frontmatter, the record of the run. `spec.md` is not written: each audit record has one home (spec 057). Consumes the pass findings as a single array (plus waiver results, scope scalars, and skipped-pass flags), applies the cross-pass dedup (highest-severity-wins on rule-id + file + overlapping range) before counting, buckets survivors into MUST / SHOULD / low-confidence / waived, prunes expired waivers, and emits the 0-findings / blocking:false report for empty scope. Each observation carries its disposition (fixed, routed with its target, discarded with its reason, or undispositioned by default); the record counts them in a derived dispositions: map, a newly routed or discarded observation is stored under decisions: (decided-by required), and expired-decisions from process-decisions are pruned. Nothing is written to the inbox. Atomic."
    )]
    async fn write_review(
        &self,
        params: Parameters<WriteReviewArgs>,
    ) -> Result<Json<WriteReviewResult>, String> {
        primitives::write_review::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "write-analysis",
        description = "Record that /ductus:analyze ran, by writing specs/NNN/analysis.md — the record in its frontmatter, the findings in a fixed body skeleton. The durable counterpart to review.md, and what `check-review-gate` reads to hold a spec out of `done` until the second gate has actually run. Takes the run's hard-fail / blocking / advisory counts (from the re-check after the fix-and-route step) plus `unexamined` (the skipped-target count, so a clean record cannot be read as a fully-examined one), and `findings`, each with its tier and disposition. Sets `blocking` from the two gating tiers and derives a dispositions: map in which a live finding the caller did not itemize counts as undispositioned; refuses a discard on a hard-fail or blocking finding; stores routed and discarded decisions under decisions: (decided-by required) and prunes expired-decisions. Nothing is written to the inbox. Refuses a spec whose frontmatter does not parse rather than recording a clean run into it. Atomic."
    )]
    async fn write_analysis(
        &self,
        params: Parameters<WriteAnalysisArgs>,
    ) -> Result<Json<WriteAnalysisResult>, String> {
        primitives::write_analysis::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "append-task",
        description = "Append a numbered task block to a feature's tasks.md (atomic rewrite)."
    )]
    async fn append_task(
        &self,
        params: Parameters<AppendTaskArgs>,
    ) -> Result<Json<AppendTaskResult>, String> {
        primitives::append_task::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "prune-tasks",
        description = "Reduce a feature's tasks.md — drop spent task sections (keep-pending) or reset to template state; or, with `all`, every spec's, returning one line per spec (gate, status, counts, sizes, path) and the specs skipped for having no tasks.md. `force` overrides the reset gate on one spec and is refused with `all`."
    )]
    async fn prune_tasks(
        &self,
        params: Parameters<PruneTasksArgs>,
    ) -> Result<Json<PruneTasksResult>, String> {
        primitives::prune_tasks::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "prune-plan",
        description = "Report a feature's plan.md sections outside the design record (the plan template's own ## headings, compared case-insensitively), each with its size, a digest, the plan-record advisory /{project}:analyze records for it, and whether analysis.md stores a discard for that advisory. With apply, remove the listed sections by heading and digest, refusing the whole write when any changed since the preview (stale-sections). On a done spec every call reports whether the tree takes the done → in-progress back-edge against HEAD (reopen-required): a preview for the tree as it stands, an apply for the tree it leaves, computed before it writes. With `all`, preview every spec — listing each whose plan holds such a section, or a done spec that already requires a reopen — and refuse `apply`. Never returns a section's text."
    )]
    async fn prune_plan(
        &self,
        params: Parameters<PrunePlanArgs>,
    ) -> Result<Json<PrunePlanResult>, String> {
        primitives::prune_plan::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "dashboard",
        description = "Single-call pipeline-state surface for /{project}:status. Returns the per-spec inventory (status, deps, tags, open-question count, artifact existence, scenarios count, blocked-by), the repo-wide tags-union, the config summary (.ductus/config.toml: disabled rule files, and every key in a ductus-owned section that no reader recognizes), the standing inbox (outstanding count and oldest item's date, rendered as an Inbox: line on every run), and the optional session target resolved for this process (spec 062): its own target when it has a session identity (DUCTUS_SESSION or a platform session id), else the shared default (.ductus/session.toml, falling back to .govern/session.toml then the legacy root pre-migration). An identified process also gets session-identity, session-source and session-notices, rendered under the target line. The read is a peek: it pins no adoption and consumes no notice, so the notices shown are the ones the process's next command delivers."
    )]
    async fn dashboard(
        &self,
        params: Parameters<DashboardArgs>,
    ) -> Result<Json<DashboardResult>, String> {
        // The standing inbox row blames the inbox's history — blocking pool,
        // like the other history walks.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::dashboard::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "write-session",
        description = "Atomically merge-write the active session file (`.ductus/session.toml`, falling back to `.govern/session.toml` then the legacy root `.govern.session.toml` pre-migration; gitignored). A target write (supply `feature`+`path`, optional `scenario`) sets the target and preserves the per-contributor `cli-config-dir`; a host-config write (supply only `cli-config-dir`) sets the agent config-dir and preserves the existing target. Since spec 062 an identified process (DUCTUS_SESSION, or a platform session id such as CLAUDE_CODE_SESSION_ID) also writes its own target under .ductus/sessions/, and the result reports other sessions on the same feature (peers), the per-process targets its expiry sweep removed (expired), session files it could not read (unreadable), and any pending removal notice the write replaced (notices). Pairs with `resolve-session`'s read; allowing this MCP tool once suppresses the per-invocation Write permission prompt the host-write path triggers."
    )]
    async fn write_session(
        &self,
        params: Parameters<WriteSessionArgs>,
    ) -> Result<Json<WriteSessionResult>, String> {
        // The session lock can wait on another process — blocking pool.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::write_session::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "resolve-session",
        description = "Resolve this process's session target (spec 062). The identity comes from the runtime's own environment: DUCTUS_SESSION, then the agent's platform session id (Claude Code: CLAUDE_CODE_SESSION_ID), else none. An unidentified process reads the shared default (.ductus/session.toml). An identified one gets its own target (.ductus/sessions/{key}.toml), or adopts and pins the shared default when it has none, or no target when it was cleared. Returns identity, source (own / adopted / default / cleared / none), target, notices to display once (adoption, other sessions on the same feature, a fold or consolidation that moved the target), and unreadable: session files that did not parse, so were not checked for the same feature. Commands call this instead of reading the session file by hand."
    )]
    async fn resolve_session(
        &self,
        params: Parameters<ResolveSessionArgs>,
    ) -> Result<Json<ResolveSessionResult>, String> {
        // The session lock can wait on another process — blocking pool.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::resolve_session::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "retarget-sessions",
        description = "After /{project}:fold or /{project}:consolidate removes a feature directory, re-point or clear every session in the working tree that names it (spec 062): the shared default (.ductus/session.toml) and every per-process target (.ductus/sessions/*.toml). cause=fold with the upstream feature+path (optional scenario) re-targets; cause=consolidate with clear clears. Sessions naming other features are untouched; every affected session other than the caller's gets a notice its next resolve-session delivers. Returns retargeted and cleared session labels, and unreadable files left in place."
    )]
    async fn retarget_sessions(
        &self,
        params: Parameters<RetargetSessionsArgs>,
    ) -> Result<Json<RetargetSessionsResult>, String> {
        // The session lock can wait on another process — blocking pool.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::retarget_sessions::run(&params.0, repo.as_path()))
            .await
    }

    #[tool(
        name = "resolve-references",
        description = "Resolve a consumer feature's `references:` index against the config's [services] registry (`.ductus/config.toml`, with the `.govern/` and legacy-root fallbacks): for each cross-service reference, read the linked spec's live lifecycle status from its local checkout and classify the outcome (ok / unregistered / not-checked-out / broken / status-unreadable)."
    )]
    async fn resolve_references(
        &self,
        params: Parameters<ResolveReferencesArgs>,
    ) -> Result<Json<ResolveReferencesResult>, String> {
        primitives::resolve_references::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "resolve-constitutions",
        description = "Resolve the project config's [constitutions] registry (spec 055) to shared-constitution documents on local disk. For each registered alias, resolve its local `path` and look for `constitution.md` inside it; the `repo` URL is identity only and is never fetched, so there is no transport and no network failure mode. Returns `loaded` and `skipped` separately — a missing checkout is `not-checked-out` and a checkout holding no document is `no-constitution-document`, two distinct operator mistakes with distinct messages. Empty `loaded` with empty `skipped` means none registered; empty `loaded` with a non-empty `skipped` means registered and unreadable, and the caller must not collapse them. Aliases sharing a path are reported in `duplicate-paths` (warn and allow; the document is read once)."
    )]
    async fn resolve_constitutions(
        &self,
        params: Parameters<ResolveConstitutionsArgs>,
    ) -> Result<Json<ResolveConstitutionsResult>, String> {
        primitives::resolve_constitutions::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "resolve-feature",
        description = "Resolve an identifier (exact directory name, feature number like 7/007, or partial slug substring) to a feature directory under the configured spec root. Returns the directory name, repo-relative path, spec status, and — when a scenario slug is supplied — the scenario file's existence plus its `section` frontmatter. Ambiguity returns the sorted candidate list and no-match returns not-found, both as domain outcomes for the host to mediate."
    )]
    async fn resolve_feature(
        &self,
        params: Parameters<ResolveFeatureArgs>,
    ) -> Result<Json<ResolveFeatureResult>, String> {
        primitives::resolve_feature::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "create-feature",
        description = "Scaffold the next {specs-root}/{NNN-slug}/ feature directory: compute the next feature number (max existing prefix + 1, zero-padded), derive the kebab-case slug from the title, create the directory, and copy the spec template into it (atomic, mode-preserving). Refuses with created: false when the target directory already exists."
    )]
    async fn create_feature(
        &self,
        params: Parameters<CreateFeatureArgs>,
    ) -> Result<Json<CreateFeatureResult>, String> {
        primitives::create_feature::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "create-plan-artifacts",
        description = "Copy the plan/tasks (and, with include-data-model, data-model) templates into an existing feature directory (atomic, mode-preserving) — /ductus:plan's template-copy and existing-artifact-detection step. Pre-existing artifacts are never touched by default: each is reported kept (file, path, action, template — primitive results carry no wall-clock data, so the keep-or-replace prompt stats the file itself for any timestamp); only overwrite: true (the confirmed replace branch) copies fresh templates over them."
    )]
    async fn create_plan_artifacts(
        &self,
        params: Parameters<CreatePlanArtifactsArgs>,
    ) -> Result<Json<CreatePlanArtifactsResult>, String> {
        primitives::create_plan_artifacts::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "append-question",
        description = "Append one `- {question}` bullet to the target artifact's ## Open Questions section — /ductus:amend's question-route write. Targets the feature's spec.md, or scenarios/{slug}.md when `scenario` is passed. Dedup is normalized-whitespace + case-insensitive (a match is a clean appended: false outcome reporting the existing entry); a missing section is created per template order; on a spec target whose status is clarified/planned/in-progress the status reverts to draft in the same atomic write (the back-edge). A `done` spec is excluded (§spec-lifecycle): the question is appended but status stays done, since done reopens via the scenario route, not a question."
    )]
    async fn append_question(
        &self,
        params: Parameters<AppendQuestionArgs>,
    ) -> Result<Json<AppendQuestionResult>, String> {
        primitives::append_question::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-review-gate",
        description = "Evaluate /ductus:implement's pre-done review gate for one feature, in order: whether the spec is already done, the feature directory's markdown lint (via the lint-markdown machinery), unresolved scenario open questions, an undischarged cross-spec impact, the review record in review.md, whether that review is still current, the analyze record in analysis.md, whether that analysis is still current, whether either record predates finding dispositions (no dispositions: map), and whether an outstanding gating finding remains, in practice an unwaived review SHOULD; undispositioned advisory findings and observations never block (spec 058). Neither record lives in spec.md frontmatter (spec 057). A declared folds-into is not checked: a pending fold never holds done, and the fold is run after it (spec 051). Returns the verdict plus, when blocked, the first failing check and the canonical blocked message; a blocked gate is a domain outcome the host halts on, never an error."
    )]
    async fn check_review_gate(
        &self,
        params: Parameters<CheckReviewGateArgs>,
    ) -> Result<Json<CheckReviewGateResult>, String> {
        // Shells out to `npx markdownlint-cli2` — blocking pool, like
        // `lint-markdown` itself.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::check_review_gate::run(&params.0, repo.as_path()))
            .await
    }

    #[tool(
        name = "append-inbox",
        description = "Append one `- [ ] {text}` checkbox bullet to {specs-root}/inbox.md (atomic write), creating the file when missing. The surface behind /ductus:log, the inbox's only producer. Returns item-count, the total inbox bullets (comment/fence-aware) after the call."
    )]
    async fn append_inbox(
        &self,
        params: Parameters<AppendInboxArgs>,
    ) -> Result<Json<AppendInboxResult>, String> {
        primitives::append_inbox::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "remove-inbox-item",
        description = "Remove the first bullet from {specs-root}/inbox.md whose text equals `item` (atomic write). The complement of append-inbox; a no-match or missing inbox is a clean outcome (removed: false). Returns the remaining bullet count."
    )]
    async fn remove_inbox_item(
        &self,
        params: Parameters<RemoveInboxItemArgs>,
    ) -> Result<Json<RemoveInboxItemResult>, String> {
        primitives::remove_inbox_item::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-artifacts",
        description = "Run /ductus:analyze's residual deterministic check families against one feature: artifact completeness per status tier (plan.md/tasks.md at planned+), task consistency (strictly-increasing numbering, Done-when presence), scenario→task mapping (a scenario any revision of tasks.md named is satisfied, so a §tasks-phase pruned task never counts against it), review-state drift on done specs, disposition drift on done specs (advisory: review.md counting undispositioned findings, which do not hold done; a record predating the dispositions map is silent), scenario open questions (blocking at done, advisory otherwise), link-adjacent decision drift (prose asserting an open state its own sibling link's target contradicts), acceptance-criterion path existence on done specs, body path existence on done specs (a path named in the body, plan, data model or scenarios that existed in the repository and no longer resolves), and acceptance-criterion labels (duplicate AC{n}, a next-criterion that no longer exceeds the body, an unlabelled criterion in a labelled spec). Each finding carries family, severity, message, and path; severity tiers mirror the command's markdown-only reference. Targets a family could not examine are reported separately in `skipped`, so a zero-finding result never reads as assurance the primitive cannot give."
    )]
    async fn check_artifacts(
        &self,
        params: Parameters<CheckArtifactsArgs>,
    ) -> Result<Json<CheckArtifactsResult>, String> {
        primitives::check_artifacts::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-artifact-size",
        description = "Measure a feature's spec artifacts (spec.md, plan.md, tasks.md, data-model.md, and each scenario — never research.md, review.md or analysis.md) by their length on disk against the configured read size, `[artifacts] read-size-bytes` in .ductus/config.toml (default 50000 when unset; a value that is not a positive whole number, or a non-table `artifacts`, is an error naming the setting, the value and the file, never the default). Each artifact over it is reported with its size, read-page count, the fixes for its kind and, on a done spec, whether each reopens it, a warning saying it may not be read in one call, the artifact-size finding message /{project}:analyze records, and whether a stored discard in analysis.md still covers it (a discard of the same path at a read-page count at least this one's), with that discard's key. A missing artifact is not reported; one that cannot be opened is listed in `skipped`, never as clean. A key in `[artifacts]` other than read-size-bytes is named in `notices`. Read-only. Used by /{project}:clarify, /{project}:plan and /{project}:analyze (spec 063)."
    )]
    async fn check_artifact_size(
        &self,
        params: Parameters<CheckArtifactSizeArgs>,
    ) -> Result<Json<CheckArtifactSizeResult>, String> {
        primitives::check_artifact_size::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "derive-routing-candidates",
        description = "Derive the existing homes proposed work could belong to, so /ductus:specify runs the routing decision before create-feature writes anything. Three sources: runtime-work (the description names a primitive or a `runtime/` path, routed to the spec whose plan claims `runtime/` — derived from the corpus, never a hardcoded slug), rule-surface (a rule file whose category stem the description shares), and spec-corpus (a spec whose slug it shares vocabulary with). Each candidate names the route it implies (`rule` or `scenario`), the target's status, and whether accepting it implies the done -> in-progress back-edge. Matching is lexical and advisory — the semantic call stays at the routeInboxItem extension point and the decision stays with the operator. Every source lands in `sources-examined` or in `skipped`, so empty candidates with empty skipped means 'examined and matched nothing' while a non-empty `skipped` means 'could not derive' — different answers. `routed-by` marks a tree that already ran and returns gate-required: false."
    )]
    async fn derive_routing_candidates(
        &self,
        params: Parameters<DeriveRoutingCandidatesArgs>,
    ) -> Result<Json<DeriveRoutingCandidatesResult>, String> {
        primitives::derive_routing_candidates::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "retire-feature",
        description = "Remove a branch-scoped feature directory (`<identifier>.<n>-<slug>`) once its content has been folded into the upstream spec that receives it — the last step of a fold and the only irreversible one. This is where the fold target's existence is finally enforced: everything earlier treats an absent target as the normal case — validation checks `folds-into` for shape only, and the pre-done gate blocks on a fold only when its target is in this tree — because a branch-scoped spec exists precisely because upstream diverged, so before the merge its target normally lives on the upstream branch. Fold-back runs after the merge, in the first tree holding both, which is the first moment the question has an answer worth refusing on. Refuses when `fold-target` names no feature directory holding a spec.md (a directory with no spec is not a home content can have landed in), and refuses a sequential `NNN-slug` feature unless `allow-sequential` is set — the sequential form is permanent, and the refusal is gated rather than removed so that an irreversible operation stays out of reach of a typo: fold never sets the flag, so a mistyped feature name on a fold still refuses, while consolidation sets it having already named both specs and confirmed the removal. The spec.md guard above is **not** gated — opting in to removing a sequential directory is not opting in to stranding its content. `retired: false` is the domain outcome for a directory already gone, so re-running an interrupted fold converges rather than halting."
    )]
    async fn retire_feature(
        &self,
        params: Parameters<RetireFeatureArgs>,
    ) -> Result<Json<RetireFeatureResult>, String> {
        primitives::retire_feature::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "invalidate-review",
        description = "Reset a spec's review.md record to the un-reviewed state — `last-run` and `reviewed-against` nulled, counts zeroed, `blocking` cleared — so the pre-`done` gate demands a fresh review before the spec can complete. The gate's staleness check diffs the spec's durable contracts (scenarios/*.md, data-model.md); spec.md is deliberately outside that set, so a fold-back that routes content into the upstream spec's BODY moves no durable contract and leaves a review that never saw the code the fold brought with it. This is how the fold says what the diff cannot see. Waivers survive verbatim, adopter-authored extra fields included: an invalidation says the review is out of date, not that an operator's recorded judgement about a finding was withdrawn. `invalidated: false` is the domain outcome for a spec that records no current review — already in this state — so a re-run of an interrupted fold converges rather than halting."
    )]
    async fn invalidate_review(
        &self,
        params: Parameters<InvalidateReviewArgs>,
    ) -> Result<Json<InvalidateReviewResult>, String> {
        primitives::invalidate_review::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "rewrite-spec-links",
        description = "Re-point every inbound pointer to a retiring or renamed feature directory at its fold target: markdown body links across the spec corpus, the `folds-into` frontmatter field, and the spec-root paths named in the referrers `check-orphaned-references` reads outside it (`CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit`). The frontmatter key is included deliberately — it is the one pointer whose job is to survive until the merge, so a rename that repaired body links and left it behind would break exactly the thing the field exists for. `dependencies:` and `references:` are NOT touched: they are derived from body links, and derive-dependencies / derive-references regenerate them from the rewritten bodies on the next commit. Matching is by whole path segment, so a directory whose name merely shares a prefix is left alone, and cross-service URLs are never rewritten. `to` is `<feature>` (each link's tail survives; one naming a file the fold did not write is listed in `kept-tails`) or `<feature>/<scenario>` (a fold into one scenario — a link to one of the retiring spec's scenarios keeps its tail and fragment, and every other tail becomes the routed scenario). A scenario tail the retiring directory does not hold is listed in `kept-tails` on either target. Every referrer is read before anything is written, and one that cannot be rewritten safely (a symbolic link, a parent resolving outside the project, text that is not UTF-8) is named in `skipped`. Empty `rewritten` means examined-and-nothing-pointed-here only when `examined` is non-zero and `skipped` is empty."
    )]
    async fn rewrite_spec_links(
        &self,
        params: Parameters<RewriteSpecLinksArgs>,
    ) -> Result<Json<RewriteSpecLinksResult>, String> {
        primitives::rewrite_spec_links::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-unfolded-specs",
        description = "Report branch-scoped specs (`<identifier>.<n>-<slug>`) still present in the working tree, each with the upstream spec its `folds-into:` names and its current status. A branch-scoped directory is a staging form discharged by fold-back; nothing runs on the merge itself, so a merged branch leaves its staging directories behind silently — this is what notices. Read-only, and it reports rather than repairs: which upstream section the content belongs in is the operator's judgement at fold-back. A `folds-into` naming a spec absent from this tree is reported as declared, never as broken — that absence is the normal pre-merge state, and existence is enforced at fold-back by retire-feature. Empty `unfolded` means examined-and-clean only when `examined` is non-zero."
    )]
    async fn check_unfolded_specs(
        &self,
        params: Parameters<CheckUnfoldedSpecsArgs>,
    ) -> Result<Json<CheckUnfoldedSpecsResult>, String> {
        primitives::check_unfolded_specs::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-corpus-links",
        description = "Report relative markdown links in the spec corpus that resolve to nothing \u{2014} the failure an adopter creates by deleting or renaming a spec directory, which dangles every inbound pointer with no error and no gate. Fills the gap between three checks that each miss it: check-orphaned-references scopes to adopter-owned bootstrap referrers, /ductus:analyze is bounded to one feature plus its dependencies, and /ductus:audit Family 26 is maintainer-only. Read-only, and reports rather than repairs \u{2014} a depth error is re-pointed while a deliberate removal is named in prose, and a rewrite that guessed wrong is worse than a precise report. Resolution is lexical against the citing file's own directory, never canonicalized, so the answer is a property of the text rather than of the checkout. Inline code spans are stripped before matching (documentation quoting link syntax is not making a link), fenced blocks and frontmatter are skipped, and blockquoted lines are NOT \u{2014} that exemption belongs to derive-dependencies, where it decides whether a link induces an edge. Empty `broken` means clean only when `skipped` is empty and `examined` is non-empty; `guidance` is set when the scan could not establish a subject at all. Targets carrying NNN, a {placeholder}, a `*`, or a bare `...` are counted in `shapes-skipped` rather than tested, and the spec root's templates/ are excluded by construction and counted. With `scope: repository` the subject is every tracked .md file, and every agent's generated command copies, framework/templates/project/, runtime/tests/ and \u{2014} where framework/reference/ holds their sources \u{2014} the generated .ductus/reference/ copies are excluded by construction and counted too."
    )]
    async fn check_corpus_links(
        &self,
        params: Parameters<CheckCorpusLinksArgs>,
    ) -> Result<Json<CheckCorpusLinksResult>, String> {
        primitives::check_corpus_links::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-orphaned-references",
        description = "Report adopter-owned files (CLAUDE.md, AGENTS.md, README.md, .githooks/pre-commit) whose references to ductus-managed paths no longer resolve — the failure a migration chain leaves behind when a later entry moves a path an earlier entry wrote into a create-strategy file the manifest never overwrites. Read-only, and reports rather than repairs: the adopter may have hand-edited the reference. Attribution has two modes and the result names which ran — `registry` when framework/migrations.toml is readable in the project tree (a framework checkout, where it names the entry whose target_paths covers the missing path), `watermark` otherwise (an adopter checkout, where only .ductus/config.toml's [migrations].last_applied is available and `migration` is empty). An unreadable referrer lands in `skipped`, so empty `findings` means examined-and-clean only when `skipped` is empty; `examined` names the files the result actually describes."
    )]
    async fn check_orphaned_references(
        &self,
        params: Parameters<CheckOrphanedReferencesArgs>,
    ) -> Result<Json<CheckOrphanedReferencesResult>, String> {
        primitives::check_orphaned_references::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-step-references",
        description = "Report `step N` prose references in a command file that name a numbered step the file does not have. A command numbers its Instructions steps and refers to them in prose; the numbers are the only binding, and removing a step renumbers everything after it while the prose keeps the old numbers. Three findings, because the repairs differ: `unresolved` (names a number the file lacks), `self-reference` (a step referring to its own number \u{2014} it resolves, so an existence check passes it, and it is almost always renumbering residue), and `discontinuous` (steps not running 1..n, which makes every reference after the gap ambiguous rather than wrong). Subject is the Instructions section alone: a numbered list elsewhere in the file restarts at 1 and is a different list, so `references-out-of-subject` counts those mentions rather than resolving them (a command's markdown-only reference, whose sub-procedures restart the same way, is its own file under framework/reference/ and outside the subject), and an empty `findings` is never read as every reference in the file resolving. A qualified reference (groom\u{2019}s step 3) names another file\u{2019}s procedure and is excluded by construction. A file with an Instructions section from which no steps are extracted is a finding, not a pass \u{2014} two empty sets compare equal."
    )]
    async fn check_step_references(
        &self,
        params: Parameters<CheckStepReferencesArgs>,
    ) -> Result<Json<CheckStepReferencesResult>, String> {
        primitives::check_step_references::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "check-command-flags",
        description = "Report flags a command's Flags table documents but its `argument-hint:` frontmatter omits. `argument-hint` is the surface a host renders when it offers a command, so an omitted flag is one the operator is never shown \u{2014} the defect an adopter hit when review.md's table listed eight flags and its hint named three. Maintainer scope: the subject is framework/commands/, where the divergence originates, not the generated copies an adopter cannot repair. Only a `Flags` section's table rows count (a command documenting flags in prose is examined and contributes nothing), and only each row's first cell, so behavior prose naming another flag is not a finding; fenced example tables are excluded. Empty `findings` means examined-and-clean over `with-flags-table`, not over `examined`; an unreadable file lands in `skipped`, and `guidance` is set when no Flags table was found at all, which is an extraction failure rather than a clean corpus."
    )]
    async fn check_command_flags(
        &self,
        params: Parameters<CheckCommandFlagsArgs>,
    ) -> Result<Json<CheckCommandFlagsResult>, String> {
        primitives::check_command_flags::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "derive-dependencies",
        description = "Regenerate every tracked spec's frontmatter `dependencies:` from the sibling-spec inline links in its body, then report any cycle in the derived graph. Body links are authoritative; the frontmatter is a derived index (spec 017). Links inside frontmatter, fenced code, blockquote-prefixed lines, or a `## See also` section do not induce edges; `## References` does. A spec with no sibling links gets `dependencies: []` — the key present and empty, which is deliberately unlike `derive-references`' absent-when-empty rule. Self-links are recorded rather than stripped so the cycle check surfaces them. `dry-run` reports without writing and pairs with `drift` for a CI check; `staged` limits the rewrite to specs in the pending commit while the cycle check still spans the full graph. Untracked specs are never enumerated and are reported in `untracked-skipped`, so an empty `updated` is not a claim that they are in sync. A spec tracked in the index but missing from the worktree is reported in `absent` and excluded from `examined`, so the count never overstates what was read. A cycle is a domain outcome in `cycles`, not an error — the caller decides whether it blocks."
    )]
    async fn derive_dependencies(
        &self,
        params: Parameters<DeriveDependenciesArgs>,
    ) -> Result<Json<DeriveDependenciesResult>, String> {
        primitives::derive_dependencies::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "derive-references",
        description = "Regenerate every tracked spec's frontmatter `references:` from the cross-service spec links in its body (spec 030). Strictly distinct from `dependencies:` — a reference is informative navigation and never enters the blocking dependency graph; this primitive never reads or writes that field, and relative sibling links are never references. Matches absolute http(s) links whose path holds `/<root>/NNN-slug/spec(-and-plan).md`, resolving each repo against the `[services]` registry: a registered service with a reachable checkout matches only that checkout's own `[paths] specs-root`, while an unreachable or unregistered repo accepts any single segment and records `service: null` rather than dropping the reference. Applies the shared body exclusions plus one of its own — a backticked link is an illustrative example, not a reference. Absent-when-empty: a spec with no cross-service links carries no `references:` key and a stale block is removed, deliberately unlike `derive-dependencies`' `dependencies: []`. `registered-services` reports the registry size, so 'no references' is distinguishable from 'no readable config'. `staged` filters the *write*, never the walk: a reference derives from the `[services]` registry as well as the body, so a service rename drifts specs nobody edited and therefore nobody staged. Every tracked spec is examined, and drifted-but-unstaged specs are reported in `unwritten` — neither 'in sync' nor 'not examined'. Untracked specs are never enumerated and appear in `untracked-skipped`. A spec tracked in the index but missing from the worktree is reported in `absent` and excluded from `examined`, so the count never overstates what was read."
    )]
    async fn derive_references(
        &self,
        params: Parameters<DeriveReferencesArgs>,
    ) -> Result<Json<DeriveReferencesResult>, String> {
        primitives::derive_references::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }
}

// The `async` handlers that `#[tool_handler]` expands to are rmcp's, not
// ours, and some resolve without an `.await`; clippy 1.99's
// `unused_async_trait_impl` fires on that generated code.
#[allow(clippy::unused_async_trait_impl)]
#[tool_handler(router = self.tool_router)]
impl ServerHandler for GovRuntimeServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            // `ServerConfig::new` fills `serverInfo` from rmcp's own build
            // environment, so without this a host is told it is talking to
            // `rmcp` at the SDK's version rather than to this runtime.
            .with_server_info(Implementation::new(
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Deterministic runtime for the ductus pipeline. Exposes per-primitive tools; \
                 see specs/022-deterministic-runtime/ for the protocol contract.",
            )
    }
}
