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
use crate::schema::descriptions;
use crate::schema::primitives::{
    AppendInboxArgs, AppendInboxResult, AppendQuestionArgs, AppendQuestionResult, AppendTaskArgs,
    AppendTaskResult, ApplyManifestArgs, ApplyManifestResult, BriefReReadArgs, BriefReReadResult,
    CheckArtifactSizeArgs, CheckArtifactSizeResult, CheckArtifactsArgs, CheckArtifactsResult,
    CheckCommandFlagsArgs, CheckCommandFlagsResult, CheckCorpusLinksArgs, CheckCorpusLinksResult,
    CheckOrphanedReferencesArgs, CheckOrphanedReferencesResult, CheckPrescriptionsArgs,
    CheckPrescriptionsResult, CheckPromotionCoverageArgs, CheckPromotionCoverageResult,
    CheckReviewFreshnessArgs, CheckReviewFreshnessResult, CheckReviewGateArgs,
    CheckReviewGateResult, CheckRuleIdsArgs, CheckRuleIdsResult, CheckStepReferencesArgs,
    CheckStepReferencesResult, CheckStuckArgs, CheckStuckResult, CheckUnfoldedSpecsArgs,
    CheckUnfoldedSpecsResult, CheckboxToggleResult, ClassifySpecEditArgs, ClassifySpecEditResult,
    ComputeReviewScopeArgs, ComputeReviewScopeResult, CreateFeatureArgs, CreateFeatureResult,
    CreatePlanArtifactsArgs, CreatePlanArtifactsResult, CreateScenarioArgs, CreateScenarioResult,
    DashboardArgs, DashboardResult, DeriveBoundaryArgs, DeriveBoundaryResult,
    DeriveDependenciesArgs, DeriveDependenciesResult, DeriveReferencesArgs, DeriveReferencesResult,
    DeriveRoutingCandidatesArgs, DeriveRoutingCandidatesResult, DiffCrossSpecArgs,
    DiffCrossSpecResult, DiscoverRuleFilesArgs, DiscoverRuleFilesResult, EnforceManifestArgs,
    EnforceManifestResult, ExtractArchiveArgs, ExtractArchiveResult, FetchArchiveArgs,
    FetchArchiveResult, GateConfirmArgs, InvalidateReviewArgs, InvalidateReviewResult,
    LabelCriteriaArgs, LabelCriteriaResult, LintMarkdownArgs, LintMarkdownResult,
    MarkCriterionArgs, MarkTaskArgs, MeasureReopeningsArgs, MeasureReopeningsResult,
    MergeManagedBlockArgs, MergeManagedBlockResult, MergePermissionsArgs, MergePermissionsResult,
    MigrateSessionFileArgs, MigrateSessionFileResult, ProcessDecisionsArgs, ProcessDecisionsResult,
    ProcessWaiversArgs, ProcessWaiversResult, PrunePlanArgs, PrunePlanResult, PruneProjectArgs,
    PruneProjectResult, PruneTasksArgs, PruneTasksResult, ReadReReadArgs, ReadReReadResult,
    ReadSpecArgs, ReadSpecResult, ReadTasksArgs, ReadTasksResult, RelocateAuditRecordsArgs,
    RelocateAuditRecordsResult, RemoveInboxItemArgs, RemoveInboxItemResult, RenderStatusArgs,
    RenderStatusResult, ResolveAnchorArgs, ResolveAnchorResult, ResolveConstitutionsArgs,
    ResolveConstitutionsResult, ResolveFeatureArgs, ResolveFeatureResult, ResolveReferencesArgs,
    ResolveReferencesResult, ResolveSessionArgs, ResolveSessionResult, RetargetSessionsArgs,
    RetargetSessionsResult, RetireFeatureArgs, RetireFeatureResult, RewriteSpecLinksArgs,
    RewriteSpecLinksResult, SetStatusArgs, SetStatusResult, StartFixRoundsArgs,
    StartFixRoundsResult, TraverseDepsArgs, TraverseDepsResult, ValidateFrontmatterArgs,
    ValidateFrontmatterResult, WriteAnalysisArgs, WriteAnalysisResult, WriteReviewArgs,
    WriteReviewResult, WriteSessionArgs, WriteSessionResult,
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
/// `diff-cross-spec`, `measure-reopenings`) run a full-history `libgit2` revwalk that is likewise
/// CPU-bound and unbounded in a large adopter repo, so they route here too,
/// as do `check-review-gate`, which shells out to the markdown linter and
/// reads history for its rename exemption, `dashboard`, which blames the
/// inbox for its oldest item and measures reopenings, and `classify-spec-edit`,
/// which diffs a tree against the working tree. `write-session`, `resolve-session` and
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
        description = descriptions::READ_SPEC
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
        description = descriptions::READ_TASKS
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
        description = descriptions::MARK_TASK
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
        description = descriptions::MARK_CRITERION
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
        description = descriptions::SET_STATUS
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
        description = descriptions::DERIVE_BOUNDARY
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
        description = descriptions::DIFF_CROSS_SPEC
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
        description = descriptions::CHECK_STUCK
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
        description = descriptions::VALIDATE_FRONTMATTER
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
        description = descriptions::RESOLVE_ANCHOR
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
        description = descriptions::TRAVERSE_DEPS
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
        description = descriptions::CHECK_RULE_IDS
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
        description = descriptions::CHECK_PROMOTION_COVERAGE
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
        description = descriptions::LINT_MARKDOWN
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
        description = descriptions::GATE_CONFIRM
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
        description = descriptions::FETCH_ARCHIVE
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
        description = descriptions::EXTRACT_ARCHIVE
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
        description = descriptions::APPLY_MANIFEST
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
        description = descriptions::ENFORCE_MANIFEST
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
        description = descriptions::MERGE_MANAGED_BLOCK
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
        description = descriptions::MERGE_PERMISSIONS
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
        description = descriptions::RELOCATE_AUDIT_RECORDS
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
        description = descriptions::MIGRATE_SESSION_FILE
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
        description = descriptions::LABEL_CRITERIA
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
        description = descriptions::CREATE_SCENARIO
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
        description = descriptions::DISCOVER_RULE_FILES
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
        description = descriptions::PROCESS_WAIVERS
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
        description = descriptions::PROCESS_DECISIONS
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
        description = descriptions::COMPUTE_REVIEW_SCOPE
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
        name = "start-fix-rounds",
        description = descriptions::START_FIX_ROUNDS
    )]
    async fn start_fix_rounds(
        &self,
        params: Parameters<StartFixRoundsArgs>,
    ) -> Result<Json<StartFixRoundsResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::start_fix_rounds::run(&params.0, repo.as_path()))
            .await
    }

    #[tool(
        name = "brief-re-read",
        description = descriptions::BRIEF_RE_READ
    )]
    async fn brief_re_read(
        &self,
        params: Parameters<BriefReReadArgs>,
    ) -> Result<Json<BriefReReadResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::brief_re_read::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "read-re-read",
        description = descriptions::READ_RE_READ
    )]
    async fn read_re_read(
        &self,
        params: Parameters<ReadReReadArgs>,
    ) -> Result<Json<ReadReReadResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::read_re_read::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "check-prescriptions",
        description = descriptions::CHECK_PRESCRIPTIONS
    )]
    async fn check_prescriptions(
        &self,
        params: Parameters<CheckPrescriptionsArgs>,
    ) -> Result<Json<CheckPrescriptionsResult>, String> {
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::check_prescriptions::run(&params.0, repo.as_path()))
            .await
    }

    #[tool(
        name = "write-review",
        description = descriptions::WRITE_REVIEW
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
        description = descriptions::WRITE_ANALYSIS
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
        description = descriptions::APPEND_TASK
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
        description = descriptions::PRUNE_TASKS
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
        description = descriptions::PRUNE_PLAN
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
        name = "prune-project",
        description = descriptions::PRUNE_PROJECT
    )]
    async fn prune_project(
        &self,
        params: Parameters<PruneProjectArgs>,
    ) -> Result<Json<PruneProjectResult>, String> {
        primitives::prune_project::run(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "dashboard",
        description = descriptions::DASHBOARD
    )]
    async fn dashboard(
        &self,
        params: Parameters<DashboardArgs>,
    ) -> Result<Json<DashboardResult>, String> {
        // The inbox row blames the inbox's history and the reopenings row walks
        // it — blocking pool, like the other history walks.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::dashboard::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "render-status",
        description = descriptions::RENDER_STATUS
    )]
    async fn render_status(
        &self,
        params: Parameters<RenderStatusArgs>,
    ) -> Result<Json<RenderStatusResult>, String> {
        // The same history walks as `dashboard`'s, so the blocking pool too.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::render_status::run(&params.0, repo.as_path())).await
    }

    #[tool(
        name = "write-session",
        description = descriptions::WRITE_SESSION
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
        description = descriptions::RESOLVE_SESSION
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
        description = descriptions::RETARGET_SESSIONS
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
        description = descriptions::RESOLVE_REFERENCES
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
        description = descriptions::RESOLVE_CONSTITUTIONS
    )]
    async fn resolve_constitutions(
        &self,
        params: Parameters<ResolveConstitutionsArgs>,
    ) -> Result<Json<ResolveConstitutionsResult>, String> {
        primitives::resolve_constitutions::run_measured(&params.0, self.repo())
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "resolve-feature",
        description = descriptions::RESOLVE_FEATURE
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
        description = descriptions::CREATE_FEATURE
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
        description = descriptions::CREATE_PLAN_ARTIFACTS
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
        description = descriptions::APPEND_QUESTION
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
        name = "check-review-freshness",
        description = descriptions::CHECK_REVIEW_FRESHNESS
    )]
    async fn check_review_freshness(
        &self,
        params: Parameters<CheckReviewFreshnessArgs>,
    ) -> Result<Json<CheckReviewFreshnessResult>, String> {
        // Hashes the contracts and may build a sweep index from two trees —
        // blocking pool, like check-review-gate.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || {
            primitives::check_review_freshness::run(&params.0, repo.as_path())
        })
        .await
    }

    #[tool(
        name = "classify-spec-edit",
        description = descriptions::CLASSIFY_SPEC_EDIT
    )]
    async fn classify_spec_edit(
        &self,
        params: Parameters<ClassifySpecEditArgs>,
    ) -> Result<Json<ClassifySpecEditResult>, String> {
        // Diffs a tree against the working tree — blocking pool, like the other
        // history readers.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::classify_spec_edit::run(&params.0, repo.as_path()))
            .await
    }

    #[tool(
        name = "measure-reopenings",
        description = descriptions::MEASURE_REOPENINGS
    )]
    async fn measure_reopenings(
        &self,
        params: Parameters<MeasureReopeningsArgs>,
    ) -> Result<Json<MeasureReopeningsResult>, String> {
        // A history walk unbounded in a large adopter repo — blocking pool, like
        // the other history readers.
        let repo = Arc::clone(&self.repo);
        dispatch_blocking(move || primitives::measure_reopenings::run(&params.0, repo.as_path()))
            .await
    }

    #[tool(
        name = "check-review-gate",
        description = descriptions::CHECK_REVIEW_GATE
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
        description = descriptions::APPEND_INBOX
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
        description = descriptions::REMOVE_INBOX_ITEM
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
        description = descriptions::CHECK_ARTIFACTS
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
        description = descriptions::CHECK_ARTIFACT_SIZE
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
        description = descriptions::DERIVE_ROUTING_CANDIDATES
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
        description = descriptions::RETIRE_FEATURE
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
        description = descriptions::INVALIDATE_REVIEW
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
        description = descriptions::REWRITE_SPEC_LINKS
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
        description = descriptions::CHECK_UNFOLDED_SPECS
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
        description = descriptions::CHECK_CORPUS_LINKS
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
        description = descriptions::CHECK_ORPHANED_REFERENCES
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
        description = descriptions::CHECK_STEP_REFERENCES
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
        description = descriptions::CHECK_COMMAND_FLAGS
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
        description = descriptions::DERIVE_DEPENDENCIES
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
        description = descriptions::DERIVE_REFERENCES
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
