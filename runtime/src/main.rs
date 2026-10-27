//! `ductus` deterministic runtime CLI entrypoint.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use std::io;

use ductus::mcp::server::GovRuntimeServer;
use ductus::primitives;
use ductus::schema::primitives::{
    AppendInboxArgs, AppendQuestionArgs, AppendTaskArgs, ApplyManifestArgs, CheckArtifactSizeArgs,
    CheckArtifactsArgs, CheckCommandFlagsArgs, CheckCorpusLinksArgs, CheckOrphanedReferencesArgs,
    CheckPromotionCoverageArgs, CheckReviewGateArgs, CheckRuleIdsArgs, CheckStepReferencesArgs,
    CheckStuckArgs, CheckUnfoldedSpecsArgs, ComputeReviewScopeArgs, CreateFeatureArgs,
    CreatePlanArtifactsArgs, CreateScenarioArgs, DashboardArgs, DeriveBoundaryArgs,
    DeriveDependenciesArgs, DeriveReferencesArgs, DeriveRoutingCandidatesArgs, DiffCrossSpecArgs,
    DiscoverRuleFilesArgs, EnforceManifestArgs, ExtractArchiveArgs, FetchArchiveArgs,
    GateConfirmArgs, InvalidateReviewArgs, LabelCriteriaArgs, LintMarkdownArgs, MarkCriterionArgs,
    MarkTaskArgs, MergeManagedBlockArgs, MergePermissionsArgs, MigrateSessionFileArgs,
    ProcessDecisionsArgs, ProcessWaiversArgs, PrunePlanArgs, PruneTasksArgs, ReadSpecArgs,
    ReadTasksArgs, RelocateAuditRecordsArgs, RemoveInboxItemArgs, ResolveAnchorArgs,
    ResolveConstitutionsArgs, ResolveFeatureArgs, ResolveReferencesArgs, ResolveSessionArgs,
    RetargetSessionsArgs, RetireFeatureArgs, RewriteSpecLinksArgs, RunGeneratorArgs, SetStatusArgs,
    TraverseDepsArgs, ValidateFrontmatterArgs, WriteAnalysisArgs, WriteReviewArgs,
    WriteSessionArgs,
};

#[derive(Parser, Debug)]
#[command(
    name = "ductus",
    version,
    about = "Deterministic runtime for the ductus pipeline."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the MCP server, exposing every primitive but `run-generator` as a tool.
    Mcp,

    /// Execute a slash command end-to-end via the subprocess interpreter.
    Exec {
        /// Slash command name (e.g., "status", "validate").
        command: String,
        /// Arguments forwarded to the command.
        args: Vec<String>,
    },

    /// Parse a slash command file under the procedure conventions.
    Parse {
        /// Path to the markdown file. Required unless `--emit-schema` is set.
        file: Option<PathBuf>,
        /// Check parseability without printing the AST. Exit 0 when the
        /// file parses as a Procedure, 2 when it is legacy prose (no
        /// parseable Instructions section — allowlist-gated in CI), and
        /// 1 when it is Invalid (malformed structure — never allowed).
        #[arg(long, conflicts_with = "emit_schema")]
        check: bool,
        /// Print the JSON Schema for the protocol envelope and exit. Debug
        /// surface used to inspect the wire contract.
        #[arg(long, conflicts_with_all = ["file", "check"])]
        emit_schema: bool,
    },

    /// Parse spec frontmatter and body sections.
    ReadSpec(ReadSpecArgs),
    /// Parse `tasks.md` into a structured task list.
    ReadTasks(ReadTasksArgs),
    /// Validate frontmatter shape against the pipeline schema.
    ValidateFrontmatter(ValidateFrontmatterArgs),
    /// Verify `§anchor` references resolve to `<!-- §anchor -->` markers.
    ResolveAnchor(ResolveAnchorArgs),
    /// Resolve an identifier (name, number, or partial slug) to a feature directory.
    ResolveFeature(ResolveFeatureArgs),
    /// Resolve a consumer feature's `references:` index against the `[services]` registry.
    ResolveReferences(ResolveReferencesArgs),
    /// Resolve registered shared constitutions to documents on local disk.
    ResolveConstitutions(ResolveConstitutionsArgs),
    /// Traverse spec dependencies and check status compatibility.
    TraverseDeps(TraverseDepsArgs),
    /// Verify cited rule IDs exist in rule files and aren't deprecated.
    CheckRuleIds(CheckRuleIdsArgs),
    CheckPromotionCoverage(CheckPromotionCoverageArgs),
    /// Count tasks.md commits since the spec entered `in-progress`.
    CheckStuck(CheckStuckArgs),
    /// Derive the runtime write boundary from git history.
    DeriveBoundary(DeriveBoundaryArgs),
    /// Diff the feature's first spec-dir commit against the working tree, filtered to sibling-spec paths.
    DiffCrossSpec(DiffCrossSpecArgs),
    /// Select rule files for /ductus:review (suffix, [rules] surfaces, disabled-rule-files).
    DiscoverRuleFiles(DiscoverRuleFilesArgs),
    /// Classify a spec's recorded waivers in review.md against currently-firing findings.
    ProcessWaivers(ProcessWaiversArgs),
    /// Classify a spec's recorded routed/discarded decisions against this run's finding keys.
    ProcessDecisions(ProcessDecisionsArgs),
    /// Resolve /ductus:review's diff-base and file scope.
    ComputeReviewScope(ComputeReviewScopeArgs),
    /// Render specs/NNN/review.md — the report and its record, in one file.
    WriteReview(WriteReviewArgs),
    /// Record that analyze ran, in specs/NNN/analysis.md.
    WriteAnalysis(WriteAnalysisArgs),
    /// Flip a single subtask checkbox in `tasks.md` (atomic rewrite).
    MarkTask(MarkTaskArgs),
    /// Flip a single acceptance-criterion checkbox in `spec.md`.
    MarkCriterion(MarkCriterionArgs),
    /// Update the `status:` field in spec frontmatter, guarded by `from`.
    SetStatus(SetStatusArgs),
    /// Invoke a bash generator with `--dry-run`; non-zero exit is drift.
    RunGenerator(RunGeneratorArgs),
    /// Wrap `npx markdownlint-cli2` and surface violations.
    LintMarkdown(LintMarkdownArgs),
    /// Download an archive plus its sha256 sidecar and verify the hash.
    FetchArchive(FetchArchiveArgs),
    /// Extract a local `.tar.gz` / `.zip` archive into a destination directory.
    ExtractArchive(ExtractArchiveArgs),
    /// Strategy-aware bulk substitute + write driven by a manifest.
    ApplyManifest(ApplyManifestArgs),
    /// Remove files in a directory that are not in the expected manifest.
    EnforceManifest(EnforceManifestArgs),
    /// Idempotently merge a framework-managed block with configurable marker shape.
    MergeManagedBlock(MergeManagedBlockArgs),
    /// Idempotently merge a canonical permission allow/deny set into a JSON file with dedup.
    MergePermissions(MergePermissionsArgs),
    /// Translate a pre-0.10.0 legacy session JSON into `.ductus/session.toml` and delete the legacy file.
    MigrateSessionFile(MigrateSessionFileArgs),
    /// Move a spec's review/analyze blocks into the artifacts that own them.
    RelocateAuditRecords(RelocateAuditRecordsArgs),
    /// Write a new scenarios/{slug}.md file under a feature with frontmatter and body.
    CreateScenario(CreateScenarioArgs),
    /// Assign stable AC{n} labels to a spec's acceptance criteria (idempotent).
    LabelCriteria(LabelCriteriaArgs),
    /// Scaffold the next {specs-root}/{NNN-slug}/ directory with a spec-template copy.
    CreateFeature(CreateFeatureArgs),
    /// Copy the plan/tasks (and optional data-model) templates into a feature directory.
    CreatePlanArtifacts(CreatePlanArtifactsArgs),
    /// Evaluate /ductus:implement's pre-done review gate (markdown lint, scenario questions, fold and cross-spec obligations, then the review and analyze records in review.md and analysis.md).
    CheckReviewGate(CheckReviewGateArgs),
    /// Append a question bullet to a spec or scenario's ## Open Questions (atomic, with back-edge).
    AppendQuestion(AppendQuestionArgs),
    /// Append a numbered task block to a feature's tasks.md (atomic rewrite).
    AppendTask(AppendTaskArgs),
    /// Append one bullet to {specs-root}/inbox.md (atomic) — the surface behind /log.
    AppendInbox(AppendInboxArgs),
    /// Remove the first bullet matching `item` from {specs-root}/inbox.md (atomic).
    RemoveInboxItem(RemoveInboxItemArgs),
    /// Derive the existing homes — specs, rule surfaces — proposed work could belong to.
    DeriveRoutingCandidates(DeriveRoutingCandidatesArgs),
    /// Report adopter-owned files whose references to ductus-managed paths no longer resolve.
    CheckCorpusLinks(CheckCorpusLinksArgs),
    CheckOrphanedReferences(CheckOrphanedReferencesArgs),
    /// Report flags a command's Flags table documents but its `argument-hint` omits.
    CheckCommandFlags(CheckCommandFlagsArgs),
    CheckStepReferences(CheckStepReferencesArgs),
    /// Report branch-scoped specs still in the tree, with the upstream spec each folds into.
    CheckUnfoldedSpecs(CheckUnfoldedSpecsArgs),
    /// Re-point inbound body links, folds-into fields and root-file references from a feature directory at its fold target.
    RewriteSpecLinks(RewriteSpecLinksArgs),
    /// Remove a folded branch-scoped feature directory, guarded on its fold target existing.
    RetireFeature(RetireFeatureArgs),
    /// Reset a spec's review block to the un-reviewed state, so the pre-done gate demands a fresh review.
    InvalidateReview(InvalidateReviewArgs),
    /// Regenerate every spec's frontmatter `dependencies:` from its body links; report cycles.
    DeriveDependencies(DeriveDependenciesArgs),
    /// Regenerate every spec's frontmatter `references:` from its cross-service body links.
    DeriveReferences(DeriveReferencesArgs),
    /// Run /ductus:analyze's residual deterministic artifact-check families for a feature.
    CheckArtifacts(CheckArtifactsArgs),
    /// Measure a feature's spec artifacts against the configured read size; report each over it.
    CheckArtifactSize(CheckArtifactSizeArgs),
    /// Reduce a feature's tasks.md — drop spent task sections or reset to template state.
    PruneTasks(PruneTasksArgs),
    /// Report a feature's plan.md sections outside the design record; remove the listed ones.
    PrunePlan(PrunePlanArgs),
    /// Emit a `gate-confirm` envelope on stdout and block for a response.
    GateConfirm(GateConfirmArgs),
    /// Single-call pipeline-state surface for `/{project}:status`.
    Dashboard(DashboardArgs),
    /// Atomically rewrite the active session file (`.ductus/session.toml`, falling back to `.govern/session.toml` then the legacy root pre-migration) with the session-target record.
    WriteSession(WriteSessionArgs),
    /// Resolve this process's session target: its own, an adopted default, or the shared default when unidentified.
    ResolveSession(ResolveSessionArgs),
    /// Re-point (fold) or clear (consolidate) every session in the working tree that names a removed feature.
    RetargetSessions(RetargetSessionsArgs),
}

fn emit_protocol_schema() -> ExitCode {
    let schema = schemars::schema_for!(ductus::schema::protocol::ProtocolMessage);
    match serde_json::to_string_pretty(&schema) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("failed to serialize protocol schema: {err}");
            ExitCode::from(1)
        }
    }
}

/// Read a `--<flag> PATH` payload into the field the MCP and interpreter paths
/// receive through the JSON context.
///
/// `apply-manifest`'s `entries` / `pinned` / `substitutions` and
/// `enforce-manifest`'s `expected` / `pinned` are arrays and maps of objects,
/// which clap cannot express as flags. A State-B `/{project}` run drives the
/// whole bootstrap through the CLI (spec 048 `state-b-continues-in-session`),
/// so without this the two primitives would receive **empty** collections —
/// and an empty manifest is a *legal* manifest, so `apply-manifest` would copy
/// nothing and report success. Every failure here is therefore an error, never
/// a default: the silent-empty path is the whole thing this guards against.
fn load_json_arg<T: serde::de::DeserializeOwned>(flag: &str, path: &str) -> Result<T, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|err| format!("--{flag}: cannot read `{path}`: {err}"))?;
    serde_json::from_str(&text)
        .map_err(|err| format!("--{flag}: `{path}` is not valid JSON for this field: {err}"))
}

/// Fill `apply-manifest`'s context-supplied fields from their `--*-json` paths.
fn hydrate_apply_manifest(args: &mut ApplyManifestArgs) -> Result<(), String> {
    if let Some(path) = args.entries_json.clone() {
        args.entries = load_json_arg("entries-json", &path)?;
    }
    if let Some(path) = args.pinned_json.clone() {
        args.pinned = load_json_arg("pinned-json", &path)?;
    }
    if let Some(path) = args.substitutions_json.clone() {
        args.substitutions = load_json_arg("substitutions-json", &path)?;
    }
    Ok(())
}

/// Fill `enforce-manifest`'s context-supplied fields from their `--*-json` paths.
fn hydrate_enforce_manifest(args: &mut EnforceManifestArgs) -> Result<(), String> {
    if let Some(path) = args.expected_json.clone() {
        args.expected = load_json_arg("expected-json", &path)?;
    }
    if let Some(path) = args.pinned_json.clone() {
        args.pinned = load_json_arg("pinned-json", &path)?;
    }
    Ok(())
}

fn emit_result<T: serde::Serialize, E: std::fmt::Display>(
    result: std::result::Result<T, E>,
) -> ExitCode {
    match result {
        Ok(value) => match serde_json::to_string(&value) {
            Ok(text) => {
                println!("{text}");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("failed to serialize result: {err}");
                ExitCode::from(1)
            }
        },
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

/// Like [`emit_result`], but maps a successful *domain* outcome to a non-zero
/// exit status.
///
/// The primitives report findings as data: a dependency cycle, or drift found
/// under `--dry-run`, is a domain outcome rather than an operational error, so
/// the MCP surface returns it in the payload and lets the host decide what it
/// means. The CLI is a different caller with a different contract — it is
/// invoked from pre-commit hooks and CI, where "this blocks" is expressed as
/// an exit code and nothing is around to read JSON. That policy belongs here,
/// at the surface that needs it, not in the primitive.
fn emit_result_gated<T, E, F>(result: std::result::Result<T, E>, blocks: F) -> ExitCode
where
    T: serde::Serialize,
    E: std::fmt::Display,
    F: FnOnce(&T) -> bool,
{
    match result {
        Ok(value) => {
            let blocked = blocks(&value);
            match serde_json::to_string(&value) {
                Ok(text) => {
                    println!("{text}");
                    if blocked {
                        ExitCode::from(1)
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(err) => {
                    eprintln!("failed to serialize result: {err}");
                    ExitCode::from(1)
                }
            }
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

/// Render dependency cycles to stderr in the shape the shell generator used,
/// so the message an author sees on a blocked commit is unchanged.
fn report_cycles(cycles: &[Vec<String>]) {
    if cycles.is_empty() {
        return;
    }
    for cycle in cycles {
        if let Some(first) = cycle.first() {
            eprintln!("cycle: {} -> {first}", cycle.join(" -> "));
        }
    }
    eprintln!();
    eprintln!("derive-dependencies: dependency graph contains cycles (see above).");
    eprintln!("The body inline links above induced cycles in the derived dep graph.");
    eprintln!("Remove or move the offending links under '## See also' before committing.");
}

/// Render broken corpus links to stderr, so an author whose commit is blocked
/// sees the citation and the fix rather than a JSON blob.
///
/// Each line is `path:line` first, which is what an editor and a terminal both
/// make clickable.
fn report_broken_links(result: &ductus::schema::primitives::CheckCorpusLinksResult) {
    for link in &result.broken {
        eprintln!(
            "broken link: {}:{} -> `{}` — {}",
            link.path, link.line, link.target, link.guidance
        );
    }
    for skip in &result.skipped {
        eprintln!(
            "unreadable: {} ({}) — its links were never checked",
            skip.path, skip.reason
        );
    }
    if !result.guidance.is_empty() {
        eprintln!("check-corpus-links: {}", result.guidance);
    }
    if !result.broken.is_empty() {
        eprintln!();
        eprintln!(
            "check-corpus-links: {} relative link(s) in {} resolve to nothing.",
            result.broken.len(),
            result.specs_root
        );
        eprintln!("Re-point them, or — when a later spec removed the target — name it in prose");
        eprintln!("instead of linking it. `git commit --no-verify` bypasses this deliberately.");
    }
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn run_parse(path: &std::path::Path, check_only: bool) -> ExitCode {
    use ductus::parser;

    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("failed to read {}: {err}", path.display());
            return ExitCode::from(1);
        }
    };
    let command_name = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    match parser::parse(&source, &command_name) {
        Ok(procedure) => {
            if check_only {
                ExitCode::SUCCESS
            } else {
                match serde_json::to_string_pretty(&procedure) {
                    Ok(text) => {
                        println!("{text}");
                        ExitCode::SUCCESS
                    }
                    Err(err) => {
                        eprintln!("failed to serialize AST: {err}");
                        ExitCode::from(1)
                    }
                }
            }
        }
        Err(parser::ParseError::LegacyProse) => {
            // Exit 2 distinguishes legacy prose from Invalid (exit 1) so
            // scripts/lint-procedure-parseability.sh can gate legacy on
            // the allowlist while rejecting Invalid unconditionally.
            eprintln!(
                "{}: legacy prose — no parseable Instructions section",
                path.display()
            );
            ExitCode::from(2)
        }
        Err(err) => {
            eprintln!("{}: {err}", path.display());
            ExitCode::from(1)
        }
    }
}

/// Terminal `error` envelope for a command-file parse failure under
/// `ductus exec`. Protocol contract (spec 022 + the versioning-enforcement
/// resolution): every non-zero exit in the 1–127 clean band is preceded
/// by a terminal `error` message on stdout carrying the runtime version,
/// so a host can suspect a framework/runtime version mismatch instead of
/// facing a message-less failure.
fn emit_exec_parse_error(path: &std::path::Path, err: &ductus::parser::ParseError) -> ExitCode {
    use ductus::io::write_envelope;
    use ductus::parser::ParseError;
    use ductus::schema::protocol::{ErrorLocation, ProtocolMessage};

    let location = match err {
        ParseError::Invalid {
            location: Some(loc),
            ..
        } => Some(ErrorLocation {
            file: path.display().to_string(),
            line: loc.start_line,
            col: loc.start_col,
        }),
        _ => None,
    };
    let message = format!(
        "failed to parse command file {}: {err} — a framework/runtime \
         version mismatch is a possible cause (this runtime is v{}; \
         re-run /ductus to realign the installed framework files)",
        path.display(),
        env!("CARGO_PKG_VERSION"),
    );
    let envelope = ProtocolMessage::Error {
        code: "parse-error".into(),
        message: message.clone(),
        runtime_version: env!("CARGO_PKG_VERSION").into(),
        location,
    };
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    if let Err(io_err) = write_envelope(&mut writer, &envelope) {
        eprintln!("ductus exec: failed to emit parse-error envelope: {io_err}");
    }
    eprintln!("{message}");
    ExitCode::from(2)
}

/// Emit a terminal `error` protocol message on stdout for an operational
/// error, honoring the 1–127 clean-band contract: a non-crash exit is always
/// preceded by a terminal `error` carrying the runtime version, so a host can
/// distinguish a clean operational error from a signal-killed crash (128+,
/// no terminal message). Used by the pre-walk and walker-I/O exit paths;
/// parse errors use [`emit_exec_parse_error`], which also carries a location.
fn emit_exec_error(code: &str, message: &str) {
    use ductus::io::write_envelope;
    use ductus::schema::protocol::ProtocolMessage;

    let envelope = ProtocolMessage::Error {
        code: code.into(),
        message: message.into(),
        runtime_version: env!("CARGO_PKG_VERSION").into(),
        location: None,
    };
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    if let Err(io_err) = write_envelope(&mut writer, &envelope) {
        eprintln!("ductus exec: failed to emit error envelope: {io_err}");
    }
}

/// The walker's seed context: the shared session file's keys, with the four
/// target keys (`feature`, `path`, `scenario`, `scenario-path`) replaced by
/// the resolution of the process acting as `identity` (spec 062).
///
/// The default file resolves through `paths::session_path` — the newest
/// existing of `.ductus/session.toml` (spec 049), `.govern/session.toml`
/// (spec 042), or the legacy repo-root `.govern.session.toml`. Its other keys
/// (a seeded `write-boundary`, nested `entries` / `substitutions` tables) are
/// still seeded whole: TOML values are bridged into `serde_json::Value` via
/// serde so nested structures survive, since the walker's context map and
/// every primitive's args struct are JSON-shaped.
///
/// The target is a **peek**, not a resolution: pinning an adoption or
/// consuming a notice here would take them from the walked command's own
/// `resolve-session` step, and the operator would never see them. So
/// `ductus exec` run from an agent's shell tool carries that agent's target,
/// and run with no identity carries the shared default's (AC23).
fn seed_context(
    repo: &std::path::Path,
    identity: Option<&ductus::session::Identity>,
) -> ductus::primitives::Result<serde_json::Map<String, serde_json::Value>> {
    use serde_json::{Map, Value};

    let mut context = Map::new();
    let session_path = ductus::schema::paths::session_path(repo);
    if let Ok(text) = std::fs::read_to_string(&session_path)
        && let Ok(Value::Object(map)) = toml::from_str::<Value>(&text)
    {
        context.extend(map);
    }
    // Only a process with a record of its own overrides the default's keys.
    // Otherwise the default, read leniently above, seeds exactly as it did
    // before spec 062 — so a malformed default halts no walk that does not ask
    // for the session, and the seed is also a general-purpose one, whose `path`
    // may be a primitive argument with no `feature` beside it (a bootstrap
    // walk's `merge-managed-block`). Only the process's own file is read
    // strictly (AC22).
    if !has_own_record(repo, identity) {
        return Ok(context);
    }
    let resolution = ductus::session::peek(repo, identity, std::time::SystemTime::now())?;
    // `set-at` goes with the target keys: the default's stamp belongs to
    // another session's write, not to this process's own target.
    for key in ["feature", "path", "scenario", "scenario-path", "set-at"] {
        context.remove(key);
    }
    if let Some(target) = resolution.target {
        context.insert("feature".into(), Value::String(target.feature));
        context.insert("path".into(), Value::String(target.path));
        if let (Some(scenario), Some(scenario_path)) = (target.scenario, target.scenario_path) {
            context.insert("scenario".into(), Value::String(scenario));
            context.insert("scenario-path".into(), Value::String(scenario_path));
        }
    }
    Ok(context)
}

/// Whether the process acting as `identity` has a per-process record of its own
/// in `repo` — the one session file the exec seed holds strict.
fn has_own_record(repo: &std::path::Path, identity: Option<&ductus::session::Identity>) -> bool {
    ductus::session::effective(repo, identity)
        .is_some_and(|identity| ductus::session::own_path(repo, identity).exists())
}

fn run_exec(command: &str, args: &[String], repo: &std::path::Path) -> ExitCode {
    use ductus::host::Host;
    use ductus::interpreter::{WalkOutcome, Walker};
    use ductus::parser;
    use serde_json::Value;

    let host = Host::load(repo);
    let mut candidates = vec![
        repo.join("framework/commands")
            .join(format!("{command}.md")),
    ];
    // Installed command file under the adopter's config dir — `commands/`
    // (claude-style) or singular `command/` (opencode); see
    // `Host::command_file_candidates`.
    candidates.extend(
        host.command_file_candidates(command)
            .into_iter()
            .map(|rel| repo.join(rel)),
    );
    // Bootstrap procedures (`/ductus` and its successors) live outside
    // the project-installable command namespace because they're invoked
    // before any framework files exist in the adopter's project. See
    // spec 022 scenario `ductus-bootstrap`.
    candidates.push(
        repo.join("framework/bootstrap")
            .join(format!("{command}.md")),
    );
    let Some(path) = candidates.iter().find(|p| p.exists()) else {
        let tried = candidates
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let message = format!("command file not found (tried {tried})");
        emit_exec_error("command-not-found", &message);
        eprintln!("ductus exec: {message}");
        return ExitCode::from(1);
    };

    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            let message = format!("failed to read {}: {err}", path.display());
            emit_exec_error("file-unreadable", &message);
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };
    let procedure = match parser::parse(&source, command) {
        Ok(p) => p,
        Err(err) => return emit_exec_parse_error(path, &err),
    };

    // Seed the walker context from the session (see `seed_context`), then
    // overlay CLI `key=value` arg overrides.
    let identity = match ductus::session::process_identity() {
        Ok(identity) => identity,
        Err(err) => {
            emit_exec_error("session-identity-invalid", &err.to_string());
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    let mut context = match seed_context(repo, identity.as_ref()) {
        Ok(context) => context,
        Err(err) => {
            emit_exec_error("session-unreadable", &err.to_string());
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    for arg in args {
        if let Some((key, value)) = arg.split_once('=') {
            context.insert(key.to_string(), Value::String(value.to_string()));
        }
    }

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    let mut walker = Walker::new(
        &procedure,
        repo.to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    match walker.run() {
        Ok(WalkOutcome::Complete) => ExitCode::SUCCESS,
        Ok(WalkOutcome::Errored { .. }) => ExitCode::from(1),
        Err(err) => {
            let message = format!("I/O error: {err}");
            emit_exec_error("io-error", &message);
            eprintln!("ductus exec: {message}");
            ExitCode::from(74) // EX_IOERR
        }
    }
}

fn run_mcp_server(repo: PathBuf) -> ExitCode {
    use rmcp::ServiceExt;
    use rmcp::transport::stdio;

    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(err) => {
            eprintln!("failed to start tokio runtime: {err}");
            return ExitCode::from(1);
        }
    };

    runtime.block_on(async move {
        let server = GovRuntimeServer::new(repo);
        let service = match server.serve(stdio()).await {
            Ok(svc) => svc,
            Err(err) => {
                eprintln!("failed to start mcp server: {err}");
                return ExitCode::from(1);
            }
        };
        if let Err(err) = service.waiting().await {
            eprintln!("mcp server terminated with error: {err}");
            return ExitCode::from(1);
        }
        ExitCode::SUCCESS
    })
}

// A flat CLI dispatch match with one arm per primitive; it grows by one
// line with each primitive and is mechanical, so the line-count lint is not
// meaningful here.
#[allow(clippy::too_many_lines)]
fn main() -> ExitCode {
    let cli = Cli::parse();
    // The runtime's environment variables — the session identity, the fetch
    // allowlist and the proxy variables — are read here, once, for every
    // subcommand (`CFG-ENV-001`), and primitives answer from this reading.
    // The inventory is docs/runtime.md's. The certificate variables are the
    // one exception it names: on Linux and other non-Apple Unix,
    // `fetch-archive` reads them once per
    // process, at its first fetch (spec 048, `fetch-archive-reads-its-proxy-once`).
    ductus::session::init_process_identity();
    ductus::primitives::fetch_archive::init_insecure_hosts();
    ductus::primitives::fetch_archive::init_proxy_env();
    let repo = cwd();
    match cli.command {
        Command::Mcp => run_mcp_server(repo),
        Command::Exec { command, args } => run_exec(&command, &args, &repo),
        Command::Parse {
            file,
            check,
            emit_schema,
        } => {
            if emit_schema {
                return emit_protocol_schema();
            }
            let Some(path) = file else {
                eprintln!(
                    "runtime parse: missing FILE argument (use --emit-schema for the debug surface)"
                );
                return ExitCode::from(1);
            };
            run_parse(&path, check)
        }
        Command::ReadSpec(args) => emit_result(primitives::read_spec::run(&args, &repo)),
        Command::ReadTasks(args) => emit_result(primitives::read_tasks::run(&args, &repo)),
        Command::ValidateFrontmatter(args) => {
            emit_result(primitives::validate_frontmatter::run(&args, &repo))
        }
        Command::ResolveAnchor(args) => emit_result(primitives::resolve_anchor::run(&args, &repo)),
        Command::ResolveFeature(args) => {
            emit_result(primitives::resolve_feature::run(&args, &repo))
        }
        Command::ResolveReferences(args) => {
            emit_result(primitives::resolve_references::run(&args, &repo))
        }
        Command::ResolveConstitutions(args) => {
            emit_result(primitives::resolve_constitutions::run(&args, &repo))
        }
        Command::TraverseDeps(args) => emit_result(primitives::traverse_deps::run(&args, &repo)),
        Command::CheckRuleIds(args) => emit_result(primitives::check_rule_ids::run(&args, &repo)),
        Command::CheckPromotionCoverage(args) => {
            emit_result(primitives::check_promotion_coverage::run(&args, &repo))
        }
        Command::CheckStuck(args) => emit_result(primitives::check_stuck::run(&args, &repo)),
        Command::DeriveBoundary(args) => {
            emit_result(primitives::derive_boundary::run(&args, &repo))
        }
        Command::DiffCrossSpec(args) => emit_result(primitives::diff_cross_spec::run(&args, &repo)),
        Command::DiscoverRuleFiles(args) => {
            emit_result(primitives::discover_rule_files::run(&args, &repo))
        }
        Command::ProcessWaivers(args) => {
            emit_result(primitives::process_waivers::run(&args, &repo))
        }
        Command::ProcessDecisions(args) => {
            emit_result(primitives::process_decisions::run(&args, &repo))
        }
        Command::ComputeReviewScope(args) => {
            emit_result(primitives::compute_review_scope::run(&args, &repo))
        }
        Command::WriteReview(args) => emit_result(primitives::write_review::run(&args, &repo)),
        Command::WriteAnalysis(args) => emit_result(primitives::write_analysis::run(&args, &repo)),
        Command::MarkTask(args) => emit_result(primitives::mark_task::run(&args, &repo)),
        Command::MarkCriterion(args) => emit_result(primitives::mark_criterion::run(&args, &repo)),
        Command::SetStatus(args) => emit_result(primitives::set_status::run(&args, &repo)),
        Command::RunGenerator(args) => emit_result(primitives::run_generator::run(&args, &repo)),
        Command::LintMarkdown(args) => emit_result(primitives::lint_markdown::run(&args, &repo)),
        Command::FetchArchive(args) => emit_result(primitives::fetch_archive::run(&args, &repo)),
        Command::ExtractArchive(args) => {
            emit_result(primitives::extract_archive::run(&args, &repo))
        }
        Command::ApplyManifest(mut args) => match hydrate_apply_manifest(&mut args) {
            Ok(()) => emit_result(primitives::apply_manifest::run(&args, &repo)),
            Err(err) => emit_result::<(), String>(Err(err)),
        },
        Command::EnforceManifest(mut args) => match hydrate_enforce_manifest(&mut args) {
            Ok(()) => emit_result(primitives::enforce_manifest::run(&args, &repo)),
            Err(err) => emit_result::<(), String>(Err(err)),
        },
        Command::MergeManagedBlock(args) => {
            emit_result(primitives::merge_managed_block::run(&args, &repo))
        }
        Command::MergePermissions(args) => {
            emit_result(primitives::merge_permissions::run(&args, &repo))
        }
        Command::MigrateSessionFile(args) => {
            emit_result(primitives::migrate_session_file::run(&args, &repo))
        }
        Command::RelocateAuditRecords(args) => {
            emit_result(primitives::relocate_audit_records::run(&args, &repo))
        }
        Command::CreateScenario(args) => {
            emit_result(primitives::create_scenario::run(&args, &repo))
        }
        Command::LabelCriteria(args) => emit_result(primitives::label_criteria::run(&args, &repo)),
        Command::CreateFeature(args) => emit_result(primitives::create_feature::run(&args, &repo)),
        Command::CreatePlanArtifacts(args) => {
            emit_result(primitives::create_plan_artifacts::run(&args, &repo))
        }
        Command::CheckReviewGate(args) => {
            emit_result(primitives::check_review_gate::run(&args, &repo))
        }
        Command::AppendQuestion(args) => {
            emit_result(primitives::append_question::run(&args, &repo))
        }
        Command::AppendTask(args) => emit_result(primitives::append_task::run(&args, &repo)),
        Command::AppendInbox(args) => emit_result(primitives::append_inbox::run(&args, &repo)),
        Command::RemoveInboxItem(args) => {
            emit_result(primitives::remove_inbox_item::run(&args, &repo))
        }
        Command::DeriveRoutingCandidates(args) => {
            emit_result(primitives::derive_routing_candidates::run(&args, &repo))
        }
        Command::CheckCorpusLinks(args) => {
            let outcome = primitives::check_corpus_links::run(&args, &repo);
            if let Ok(result) = &outcome {
                report_broken_links(result);
            }
            // A broken link blocks the commit — that is the whole point: a
            // deletion should fail at the commit that makes it rather than at
            // a reader's next traversal. A scan that could not establish a
            // subject, or could not read a file, blocks for the *other*
            // reason: a check that could not run must never exit like one
            // that passed.
            emit_result_gated(outcome, |r| {
                !r.broken.is_empty() || !r.guidance.is_empty() || !r.skipped.is_empty()
            })
        }
        Command::CheckOrphanedReferences(args) => {
            emit_result(primitives::check_orphaned_references::run(&args, &repo))
        }
        Command::CheckCommandFlags(args) => {
            emit_result(primitives::check_command_flags::run(&args, &repo))
        }
        Command::CheckStepReferences(args) => {
            emit_result(primitives::check_step_references::run(&args, &repo))
        }
        Command::CheckUnfoldedSpecs(args) => {
            emit_result(primitives::check_unfolded_specs::run(&args, &repo))
        }
        Command::RewriteSpecLinks(args) => {
            emit_result(primitives::rewrite_spec_links::run(&args, &repo))
        }
        Command::RetireFeature(args) => emit_result(primitives::retire_feature::run(&args, &repo)),
        Command::InvalidateReview(args) => {
            emit_result(primitives::invalidate_review::run(&args, &repo))
        }
        Command::DeriveDependencies(args) => {
            let outcome = primitives::derive_dependencies::run(&args, &repo);
            if let Ok(result) = &outcome {
                report_cycles(&result.cycles);
            }
            // A cycle always blocks. Drift blocks only on a *report-only*
            // run, which is the CI check ("the committed indexes are stale");
            // on a writing run the drift was just resolved, so it is not a
            // failure.
            emit_result_gated(outcome, |r| !r.cycles.is_empty() || (!r.wrote && r.drift))
        }
        Command::DeriveReferences(args) => {
            // No graph here, so stale-on-a-report-only-run is the only blocker.
            emit_result_gated(primitives::derive_references::run(&args, &repo), |r| {
                !r.wrote && r.drift
            })
        }
        Command::CheckArtifacts(args) => {
            emit_result(primitives::check_artifacts::run(&args, &repo))
        }
        Command::CheckArtifactSize(args) => {
            emit_result(primitives::check_artifact_size::run(&args, &repo))
        }
        Command::PruneTasks(args) => emit_result(primitives::prune_tasks::run(&args, &repo)),
        Command::PrunePlan(args) => emit_result(primitives::prune_plan::run(&args, &repo)),
        Command::Dashboard(args) => emit_result(primitives::dashboard::run(&args, &repo)),
        Command::WriteSession(args) => emit_result(primitives::write_session::run(&args, &repo)),
        Command::ResolveSession(args) => {
            emit_result(primitives::resolve_session::run(&args, &repo))
        }
        Command::RetargetSessions(args) => {
            emit_result(primitives::retarget_sessions::run(&args, &repo))
        }
        Command::GateConfirm(args) => {
            // The CLI binding is the subprocess-interpreter surface: emit the
            // gate-confirm envelope on stdout, then read one gate-response
            // line from stdin. The MCP surface routes prompts via the host
            // instead and is wired up in task 6.
            let request_id = primitives::gate_confirm::fresh_request_id();
            let stdin = io::stdin();
            let mut reader = stdin.lock();
            let result = {
                let stdout = io::stdout();
                let mut writer = stdout.lock();
                primitives::gate_confirm::run_blocking(&args, &request_id, &mut reader, &mut writer)
            };
            emit_result(result)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::collections::BTreeSet;

    use clap::CommandFactory;
    use ductus::schema::registry::PRIMITIVE_REGISTRY;
    use ductus::session::Identity;

    use super::{Cli, seed_context};

    fn repo_with_default(body: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(tmp.path().join(".ductus/session.toml"), body).unwrap();
        tmp
    }

    /// AC23, second case: run with no identity, the seed is the shared
    /// default's target — and its other keys still ride along.
    #[test]
    fn an_unidentified_exec_seeds_the_shared_default() {
        let tmp = repo_with_default(
            "feature = \"055-a\"\npath = \"specs/055-a\"\nwrite-boundary = [\"src/**\"]\n",
        );
        let context = seed_context(tmp.path(), None).unwrap();
        assert_eq!(context["feature"], "055-a");
        assert_eq!(context["write-boundary"][0], "src/**");
    }

    /// AC23, first case: run as an agent whose own target differs from the
    /// default, the seed is the agent's — and nothing is written by seeding.
    #[test]
    fn an_identified_exec_seeds_its_own_target_without_writing() {
        let tmp = repo_with_default(
            "feature = \"055-a\"\npath = \"specs/055-a\"\nset-at = \"2026-09-29T12:00:00Z\"\n",
        );
        std::fs::create_dir_all(tmp.path().join(".ductus/sessions")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/sessions/review.toml"),
            "feature = \"056-b\"\npath = \"specs/056-b\"\nscenario = \"s\"\n\
             scenario-path = \"specs/056-b/scenarios/s.md\"\n",
        )
        .unwrap();
        let before =
            std::fs::read_to_string(tmp.path().join(".ductus/sessions/review.toml")).unwrap();
        let review = Identity::named("review").unwrap();
        let context = seed_context(tmp.path(), Some(&review)).unwrap();
        assert_eq!(context["feature"], "056-b");
        assert_eq!(context["path"], "specs/056-b");
        assert_eq!(context["scenario"], "s");
        assert!(
            context.get("set-at").is_none(),
            "the default's stamp is another session's"
        );
        assert_eq!(
            std::fs::read_to_string(tmp.path().join(".ductus/sessions/review.toml")).unwrap(),
            before
        );
    }

    /// The session file is also a general-purpose seed: a `path` with no
    /// `feature` is a primitive argument, and a process with no target of its
    /// own must seed it untouched.
    #[test]
    fn a_process_with_no_own_target_seeds_the_default_untouched() {
        let tmp = repo_with_default("path = \"CLAUDE.md\"\nblock = \"x\"\n");
        let review = Identity::named("review").unwrap();
        let context = seed_context(tmp.path(), Some(&review)).unwrap();
        assert_eq!(context["path"], "CLAUDE.md");
        assert_eq!(context["block"], "x");
    }

    /// A malformed default halts no walk for a process with no record of its
    /// own — it seeds leniently, as before spec 062 — while a malformed own
    /// record is still reported (AC22).
    #[test]
    fn a_malformed_default_seeds_leniently_but_a_malformed_own_record_does_not() {
        let tmp = repo_with_default("feature = [\n");
        assert!(
            seed_context(tmp.path(), None)
                .unwrap()
                .get("feature")
                .is_none()
        );
        let review = Identity::named("review").unwrap();
        let context = seed_context(tmp.path(), Some(&review)).unwrap();
        assert!(context.get("feature").is_none());

        std::fs::create_dir_all(tmp.path().join(".ductus/sessions")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/sessions/review.toml"),
            "feature = [",
        )
        .unwrap();
        assert!(seed_context(tmp.path(), Some(&review)).is_err());
    }

    #[test]
    fn a_cleared_exec_seeds_no_target() {
        let tmp = repo_with_default("feature = \"055-a\"\npath = \"specs/055-a\"\n");
        std::fs::create_dir_all(tmp.path().join(".ductus/sessions")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/sessions/review.toml"),
            "cleared = true\n",
        )
        .unwrap();
        let review = Identity::named("review").unwrap();
        let context = seed_context(tmp.path(), Some(&review)).unwrap();
        assert!(context.get("feature").is_none() && context.get("path").is_none());
    }

    /// Subcommands that are deliberately not registry primitives, each
    /// excluded by name with its reason rather than by loosening the
    /// assertion below to containment.
    ///
    /// `mcp` and `exec` are the runtime's two *surfaces*
    /// (§runtime-boundary), not capabilities it exposes; `parse` is the
    /// parseability check `scripts/lint-procedure-parseability.sh` and CI
    /// drive, plus the `--emit-schema` debug surface. None of the three is
    /// a primitive, so none appears in [`PRIMITIVE_REGISTRY`]. `help` is
    /// synthesised by clap rather than declared here at all.
    ///
    /// A future primitive that genuinely should have no CLI surface is added
    /// here with its own reason. Nothing may be excluded silently: the point
    /// of the set-equality is that *not* being a subcommand has to be a
    /// stated decision.
    const NON_PRIMITIVE_SUBCOMMANDS: &[&str] = &["mcp", "exec", "parse", "help"];

    /// The clap subcommand enum is the fifth primitive-registration surface,
    /// and it was the one nothing pinned: with a `Command` variant and its
    /// dispatch arm deleted the entire suite still passed, so a primitive
    /// could be missing from `ductus <name>` with no test reporting it. That
    /// absence bites hardest on the markdown-only path, which has no MCP
    /// server to fall back to — [§design-principles]' first rule turned on
    /// the runtime's own registration, where four surfaces prove themselves
    /// on every run and the fifth was indistinguishable from them.
    ///
    /// The assertion is set-**equality**, matching what `tests/mcp.rs`
    /// already does for the shipped manifest: a subcommand with no registry
    /// entry is as much a defect as a registry entry with no subcommand,
    /// because it means the CLI offers a verb the canonical set does not
    /// define.
    ///
    /// The comparison reads the names clap itself exposes rather than
    /// transforming the variant identifiers — a second transformation of
    /// `ReadSpec` into `read-spec` would be a second thing to drift.
    ///
    /// [§design-principles]: ../../framework/constitution.md#design-principles
    #[test]
    fn every_registry_primitive_has_a_clap_subcommand() {
        let command = Cli::command();
        let exposed: BTreeSet<&str> = command
            .get_subcommands()
            .map(clap::Command::get_name)
            .filter(|name| !NON_PRIMITIVE_SUBCOMMANDS.contains(name))
            .collect();
        let registry: BTreeSet<&str> = PRIMITIVE_REGISTRY.iter().copied().collect();

        let missing: Vec<&&str> = registry.difference(&exposed).collect();
        let phantom: Vec<&&str> = exposed.difference(&registry).collect();
        assert!(
            missing.is_empty() && phantom.is_empty(),
            "the clap subcommand enum in main.rs diverged from PRIMITIVE_REGISTRY.\n  \
             in the registry, missing a `Command` variant + dispatch arm: {missing:?}\n  \
             on the CLI, missing a registry entry: {phantom:?}"
        );
    }
}
