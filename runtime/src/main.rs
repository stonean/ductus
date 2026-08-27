//! `ductus` deterministic runtime CLI entrypoint.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use std::io;

use ductus::mcp::server::GovRuntimeServer;
use ductus::primitives;
use ductus::schema::primitives::{
    AppendInboxArgs, AppendQuestionArgs, AppendTaskArgs, ApplyManifestArgs, CheckArtifactsArgs,
    CheckCommandFlagsArgs, CheckOrphanedReferencesArgs, CheckReviewGateArgs, CheckRuleIdsArgs,
    CheckStuckArgs, ComputeReviewScopeArgs, CreateFeatureArgs, CreatePlanArtifactsArgs,
    CreateScenarioArgs, DashboardArgs, DeriveBoundaryArgs, DeriveDependenciesArgs,
    DeriveReferencesArgs, DeriveRoutingCandidatesArgs, DiffCrossSpecArgs, DiscoverRuleFilesArgs,
    EnforceManifestArgs, ExtractArchiveArgs, FetchArchiveArgs, GateConfirmArgs, LabelCriteriaArgs,
    LintMarkdownArgs, MarkCriterionArgs, MarkTaskArgs, MergeManagedBlockArgs, MergePermissionsArgs,
    MigrateSessionFileArgs, ProcessWaiversArgs, PruneTasksArgs, ReadSpecArgs, ReadTasksArgs,
    RemoveInboxItemArgs, ResolveAnchorArgs, ResolveFeatureArgs, ResolveReferencesArgs,
    RunGeneratorArgs, SetStatusArgs, TraverseDepsArgs, ValidateFrontmatterArgs, WriteReviewArgs,
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
    /// Start the MCP server, exposing every primitive as a tool.
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
    /// Traverse spec dependencies and check status compatibility.
    TraverseDeps(TraverseDepsArgs),
    /// Verify cited rule IDs exist in rule files and aren't deprecated.
    CheckRuleIds(CheckRuleIdsArgs),
    /// Count tasks.md commits since the spec entered `in-progress`.
    CheckStuck(CheckStuckArgs),
    /// Derive the runtime write boundary from git history.
    DeriveBoundary(DeriveBoundaryArgs),
    /// Diff the feature's first spec-dir commit against the working tree, filtered to sibling-spec paths + inbox additions.
    DiffCrossSpec(DiffCrossSpecArgs),
    /// Select rule files for /ductus:review (suffix, [rules] surfaces, disabled-rule-files).
    DiscoverRuleFiles(DiscoverRuleFilesArgs),
    /// Classify a spec's review.waivers against currently-firing findings.
    ProcessWaivers(ProcessWaiversArgs),
    /// Resolve /ductus:review's diff-base, file scope, and captured issues.
    ComputeReviewScope(ComputeReviewScopeArgs),
    /// Render specs/NNN/review.md and update the spec `review:` frontmatter block.
    WriteReview(WriteReviewArgs),
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
    /// Write a new scenarios/{slug}.md file under a feature with frontmatter and body.
    CreateScenario(CreateScenarioArgs),
    /// Assign stable AC{n} labels to a spec's acceptance criteria (idempotent).
    LabelCriteria(LabelCriteriaArgs),
    /// Scaffold the next {specs-root}/{NNN-slug}/ directory with a spec-template copy.
    CreateFeature(CreateFeatureArgs),
    /// Copy the plan/tasks (and optional data-model) templates into a feature directory.
    CreatePlanArtifacts(CreatePlanArtifactsArgs),
    /// Evaluate /ductus:implement's pre-done review gate (markdown lint + spec review: block).
    CheckReviewGate(CheckReviewGateArgs),
    /// Append a question bullet to a spec or scenario's ## Open Questions (atomic, with back-edge).
    AppendQuestion(AppendQuestionArgs),
    /// Append a numbered task block to a feature's tasks.md (atomic rewrite).
    AppendTask(AppendTaskArgs),
    /// Append one bullet to {specs-root}/inbox.md (atomic, optional dedup-by-prefix).
    AppendInbox(AppendInboxArgs),
    /// Remove the first bullet matching `item` from {specs-root}/inbox.md (atomic).
    RemoveInboxItem(RemoveInboxItemArgs),
    /// Derive the existing homes — specs, rule surfaces — proposed work could belong to.
    DeriveRoutingCandidates(DeriveRoutingCandidatesArgs),
    /// Report adopter-owned files whose references to ductus-managed paths no longer resolve.
    CheckOrphanedReferences(CheckOrphanedReferencesArgs),
    /// Report flags a command's Flags table documents but its `argument-hint` omits.
    CheckCommandFlags(CheckCommandFlagsArgs),
    /// Regenerate every spec's frontmatter `dependencies:` from its body links; report cycles.
    DeriveDependencies(DeriveDependenciesArgs),
    /// Regenerate every spec's frontmatter `references:` from its cross-service body links.
    DeriveReferences(DeriveReferencesArgs),
    /// Run /ductus:analyze's residual deterministic artifact-check families for a feature.
    CheckArtifacts(CheckArtifactsArgs),
    /// Reduce a feature's tasks.md — drop spent task sections or reset to template state.
    PruneTasks(PruneTasksArgs),
    /// Emit a `gate-confirm` envelope on stdout and block for a response.
    GateConfirm(GateConfirmArgs),
    /// Single-call pipeline-state surface for `/{project}:status`.
    Dashboard(DashboardArgs),
    /// Atomically rewrite the active session file (`.ductus/session.toml`, falling back to `.govern/session.toml` then the legacy root pre-migration) with the session-target record.
    WriteSession(WriteSessionArgs),
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
        eprintln!("runtime exec: failed to emit parse-error envelope: {io_err}");
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
        eprintln!("runtime exec: failed to emit error envelope: {io_err}");
    }
}

fn run_exec(command: &str, args: &[String], repo: &std::path::Path) -> ExitCode {
    use ductus::host::Host;
    use ductus::interpreter::{WalkOutcome, Walker};
    use ductus::parser;
    use serde_json::{Map, Value};

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
        eprintln!("runtime exec: {message}");
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

    // Seed the walker context: session file (when present) overlaid with
    // CLI `key=value` arg overrides. The session resolves through
    // `paths::session_path` — `.ductus/session.toml` (spec 042) with a
    // fallback to the legacy repo-root `.govern.session.toml`; the path is
    // uniform across every adopter regardless of AI CLI or project name. TOML
    // values are bridged into `serde_json::Value` via serde so nested
    // structures (arrays-of-tables for `entries`, sub-tables for
    // `substitutions`, etc.) survive intact — the walker's context map
    // and every primitive's args struct are JSON-shaped.
    let mut context = Map::new();
    let session_path = ductus::schema::paths::session_path(repo);
    if let Ok(text) = std::fs::read_to_string(&session_path)
        && let Ok(Value::Object(map)) = toml::from_str::<Value>(&text)
    {
        context.extend(map);
    }
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
            eprintln!("runtime exec: {message}");
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
        Command::TraverseDeps(args) => emit_result(primitives::traverse_deps::run(&args, &repo)),
        Command::CheckRuleIds(args) => emit_result(primitives::check_rule_ids::run(&args, &repo)),
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
        Command::ComputeReviewScope(args) => {
            emit_result(primitives::compute_review_scope::run(&args, &repo))
        }
        Command::WriteReview(args) => emit_result(primitives::write_review::run(&args, &repo)),
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
        Command::CheckOrphanedReferences(args) => {
            emit_result(primitives::check_orphaned_references::run(&args, &repo))
        }
        Command::CheckCommandFlags(args) => {
            emit_result(primitives::check_command_flags::run(&args, &repo))
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
        Command::PruneTasks(args) => emit_result(primitives::prune_tasks::run(&args, &repo)),
        Command::Dashboard(args) => emit_result(primitives::dashboard::run(&args, &repo)),
        Command::WriteSession(args) => emit_result(primitives::write_session::run(&args, &repo)),
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
