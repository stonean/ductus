//! `ductus` deterministic runtime CLI entrypoint.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use std::io::{self, Write as _};

use ductus::mcp::server::GovRuntimeServer;
use ductus::primitives;
use ductus::schema::descriptions;
use ductus::schema::primitives::{
    AppendInboxArgs, AppendQuestionArgs, AppendTaskArgs, ApplyManifestArgs, BriefReReadArgs,
    CheckArtifactSizeArgs, CheckArtifactSizeResult, CheckArtifactsArgs, CheckCommandFlagsArgs,
    CheckCorpusLinksArgs, CheckDeparturesArgs, CheckOrphanedReferencesArgs, CheckPrescriptionsArgs,
    CheckPromotionCoverageArgs, CheckReviewGateArgs, CheckRuleIdsArgs, CheckStepReferencesArgs,
    CheckStuckArgs, CheckUnfoldedSpecsArgs, ClassifySpecEditArgs, ComputeReviewScopeArgs,
    CreateFeatureArgs, CreatePlanArtifactsArgs, CreateScenarioArgs, DashboardArgs,
    DeriveBoundaryArgs, DeriveDependenciesArgs, DeriveDependenciesResult, DeriveReferencesArgs,
    DeriveReferencesResult, DeriveRoutingCandidatesArgs, DiffCrossSpecArgs, DiscoverRuleFilesArgs,
    EnforceManifestArgs, ExtractArchiveArgs, FetchArchiveArgs, GateConfirmArgs,
    InvalidateReviewArgs, LabelCriteriaArgs, LintMarkdownArgs, MarkCriterionArgs, MarkTaskArgs,
    MeasureReopeningsArgs, MergeManagedBlockArgs, MergePermissionsArgs, MergeStatusHookArgs,
    MigrateSessionFileArgs, ProcessDecisionsArgs, ProcessWaiversArgs, ProjectSizeState,
    PrunePlanArgs, PruneProjectArgs, PruneTasksArgs, ReadDeprecationLabelsArgs, ReadReReadArgs,
    ReadSectionArgs, ReadSpecArgs, ReadTasksArgs, RelocateAuditRecordsArgs, RemoveInboxItemArgs,
    RenderHelpArgs, RenderStatusArgs, ResolveAnchorArgs, ResolveConstitutionsArgs,
    ResolveFeatureArgs, ResolveReferencesArgs, ResolveSessionArgs, RetargetSessionsArgs,
    RetireFeatureArgs, RewriteSpecLinksArgs, RunGeneratorArgs, SetStatusArgs, StartFixRoundsArgs,
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

/// The hooks `ductus hook` answers, one per slash command it answers, both
/// on Claude Code's `UserPromptExpansion` event.
#[derive(Subcommand, Debug)]
enum HookCommand {
    /// Claude Code's `UserPromptExpansion` hook for `/{project}:status`.
    ///
    /// Reads the hook input from stdin, and prints a block response whose
    /// reason is the pipeline view as fixed-width text. The project root is
    /// `CLAUDE_PROJECT_DIR`, else the working directory. Exits 1, with the
    /// error on stderr, when the view cannot be rendered.
    Status,
    /// Claude Code's `UserPromptExpansion` hook for `/{project}:help`.
    ///
    /// Reads the hook input from stdin, and prints a block response whose
    /// reason is the guide the installed help copy holds. The project root is
    /// `CLAUDE_PROJECT_DIR`, else the working directory. Exits 1, with the
    /// error on stderr, when the guide cannot be read.
    Help,
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

    /// Answer an agent host's hook (spec 080).
    // clap's own `help` subcommand is off, since `hook help` is a member;
    // `ductus hook --help` still prints this group's help.
    #[command(disable_help_subcommand = true)]
    Hook {
        #[command(subcommand)]
        hook: HookCommand,
    },

    #[command(about = descriptions::summary(descriptions::READ_SPEC), long_about = descriptions::READ_SPEC)]
    ReadSpec(ReadSpecArgs),
    #[command(about = descriptions::summary(descriptions::READ_TASKS), long_about = descriptions::READ_TASKS)]
    ReadTasks(ReadTasksArgs),
    #[command(about = descriptions::summary(descriptions::VALIDATE_FRONTMATTER), long_about = descriptions::VALIDATE_FRONTMATTER)]
    ValidateFrontmatter(ValidateFrontmatterArgs),
    #[command(about = descriptions::summary(descriptions::RESOLVE_ANCHOR), long_about = descriptions::RESOLVE_ANCHOR)]
    ResolveAnchor(ResolveAnchorArgs),
    #[command(about = descriptions::summary(descriptions::RESOLVE_FEATURE), long_about = descriptions::RESOLVE_FEATURE)]
    ResolveFeature(ResolveFeatureArgs),
    #[command(about = descriptions::summary(descriptions::RESOLVE_REFERENCES), long_about = descriptions::RESOLVE_REFERENCES)]
    ResolveReferences(ResolveReferencesArgs),
    #[command(about = descriptions::summary(descriptions::RESOLVE_CONSTITUTIONS), long_about = descriptions::RESOLVE_CONSTITUTIONS)]
    ResolveConstitutions(ResolveConstitutionsArgs),
    #[command(about = descriptions::summary(descriptions::TRAVERSE_DEPS), long_about = descriptions::TRAVERSE_DEPS)]
    TraverseDeps(TraverseDepsArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_DEPARTURES), long_about = descriptions::CHECK_DEPARTURES)]
    CheckDepartures(CheckDeparturesArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_RULE_IDS), long_about = descriptions::CHECK_RULE_IDS)]
    CheckRuleIds(CheckRuleIdsArgs),
    #[command(about = descriptions::summary(descriptions::READ_DEPRECATION_LABELS), long_about = descriptions::READ_DEPRECATION_LABELS)]
    ReadDeprecationLabels(ReadDeprecationLabelsArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_PROMOTION_COVERAGE), long_about = descriptions::CHECK_PROMOTION_COVERAGE)]
    CheckPromotionCoverage(CheckPromotionCoverageArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_STUCK), long_about = descriptions::CHECK_STUCK)]
    CheckStuck(CheckStuckArgs),
    #[command(about = descriptions::summary(descriptions::DERIVE_BOUNDARY), long_about = descriptions::DERIVE_BOUNDARY)]
    DeriveBoundary(DeriveBoundaryArgs),
    #[command(about = descriptions::summary(descriptions::DIFF_CROSS_SPEC), long_about = descriptions::DIFF_CROSS_SPEC)]
    DiffCrossSpec(DiffCrossSpecArgs),
    #[command(about = descriptions::summary(descriptions::DISCOVER_RULE_FILES), long_about = descriptions::DISCOVER_RULE_FILES)]
    DiscoverRuleFiles(DiscoverRuleFilesArgs),
    #[command(about = descriptions::summary(descriptions::PROCESS_WAIVERS), long_about = descriptions::PROCESS_WAIVERS)]
    ProcessWaivers(ProcessWaiversArgs),
    #[command(about = descriptions::summary(descriptions::PROCESS_DECISIONS), long_about = descriptions::PROCESS_DECISIONS)]
    ProcessDecisions(ProcessDecisionsArgs),
    #[command(about = descriptions::summary(descriptions::COMPUTE_REVIEW_SCOPE), long_about = descriptions::COMPUTE_REVIEW_SCOPE)]
    ComputeReviewScope(ComputeReviewScopeArgs),
    #[command(about = descriptions::summary(descriptions::WRITE_REVIEW), long_about = descriptions::WRITE_REVIEW)]
    WriteReview(Box<WriteReviewArgs>),
    #[command(about = descriptions::summary(descriptions::WRITE_ANALYSIS), long_about = descriptions::WRITE_ANALYSIS)]
    WriteAnalysis(WriteAnalysisArgs),
    #[command(about = descriptions::summary(descriptions::MARK_TASK), long_about = descriptions::MARK_TASK)]
    MarkTask(MarkTaskArgs),
    #[command(about = descriptions::summary(descriptions::MARK_CRITERION), long_about = descriptions::MARK_CRITERION)]
    MarkCriterion(MarkCriterionArgs),
    #[command(about = descriptions::summary(descriptions::SET_STATUS), long_about = descriptions::SET_STATUS)]
    SetStatus(SetStatusArgs),
    #[command(about = descriptions::summary(descriptions::RUN_GENERATOR), long_about = descriptions::RUN_GENERATOR)]
    RunGenerator(RunGeneratorArgs),
    #[command(about = descriptions::summary(descriptions::LINT_MARKDOWN), long_about = descriptions::LINT_MARKDOWN)]
    LintMarkdown(LintMarkdownArgs),
    #[command(about = descriptions::summary(descriptions::FETCH_ARCHIVE), long_about = descriptions::FETCH_ARCHIVE)]
    FetchArchive(FetchArchiveArgs),
    #[command(about = descriptions::summary(descriptions::EXTRACT_ARCHIVE), long_about = descriptions::EXTRACT_ARCHIVE)]
    ExtractArchive(ExtractArchiveArgs),
    #[command(about = descriptions::summary(descriptions::APPLY_MANIFEST), long_about = descriptions::APPLY_MANIFEST)]
    ApplyManifest(ApplyManifestArgs),
    #[command(about = descriptions::summary(descriptions::ENFORCE_MANIFEST), long_about = descriptions::ENFORCE_MANIFEST)]
    EnforceManifest(EnforceManifestArgs),
    #[command(about = descriptions::summary(descriptions::MERGE_MANAGED_BLOCK), long_about = descriptions::MERGE_MANAGED_BLOCK)]
    MergeManagedBlock(MergeManagedBlockArgs),
    #[command(about = descriptions::summary(descriptions::MERGE_PERMISSIONS), long_about = descriptions::MERGE_PERMISSIONS)]
    MergePermissions(MergePermissionsArgs),
    #[command(about = descriptions::summary(descriptions::MERGE_STATUS_HOOK), long_about = descriptions::MERGE_STATUS_HOOK)]
    MergeStatusHook(MergeStatusHookArgs),
    #[command(about = descriptions::summary(descriptions::MIGRATE_SESSION_FILE), long_about = descriptions::MIGRATE_SESSION_FILE)]
    MigrateSessionFile(MigrateSessionFileArgs),
    #[command(about = descriptions::summary(descriptions::RELOCATE_AUDIT_RECORDS), long_about = descriptions::RELOCATE_AUDIT_RECORDS)]
    RelocateAuditRecords(RelocateAuditRecordsArgs),
    #[command(about = descriptions::summary(descriptions::CREATE_SCENARIO), long_about = descriptions::CREATE_SCENARIO)]
    CreateScenario(CreateScenarioArgs),
    #[command(about = descriptions::summary(descriptions::LABEL_CRITERIA), long_about = descriptions::LABEL_CRITERIA)]
    LabelCriteria(LabelCriteriaArgs),
    #[command(about = descriptions::summary(descriptions::CREATE_FEATURE), long_about = descriptions::CREATE_FEATURE)]
    CreateFeature(CreateFeatureArgs),
    #[command(about = descriptions::summary(descriptions::CREATE_PLAN_ARTIFACTS), long_about = descriptions::CREATE_PLAN_ARTIFACTS)]
    CreatePlanArtifacts(CreatePlanArtifactsArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_REVIEW_GATE), long_about = descriptions::CHECK_REVIEW_GATE)]
    CheckReviewGate(CheckReviewGateArgs),
    #[command(about = descriptions::summary(descriptions::CLASSIFY_SPEC_EDIT), long_about = descriptions::CLASSIFY_SPEC_EDIT)]
    ClassifySpecEdit(ClassifySpecEditArgs),
    #[command(about = descriptions::summary(descriptions::MEASURE_REOPENINGS), long_about = descriptions::MEASURE_REOPENINGS)]
    MeasureReopenings(MeasureReopeningsArgs),
    #[command(about = descriptions::summary(descriptions::APPEND_QUESTION), long_about = descriptions::APPEND_QUESTION)]
    AppendQuestion(AppendQuestionArgs),
    #[command(about = descriptions::summary(descriptions::APPEND_TASK), long_about = descriptions::APPEND_TASK)]
    AppendTask(AppendTaskArgs),
    #[command(about = descriptions::summary(descriptions::APPEND_INBOX), long_about = descriptions::APPEND_INBOX)]
    AppendInbox(AppendInboxArgs),
    #[command(about = descriptions::summary(descriptions::REMOVE_INBOX_ITEM), long_about = descriptions::REMOVE_INBOX_ITEM)]
    RemoveInboxItem(RemoveInboxItemArgs),
    #[command(about = descriptions::summary(descriptions::DERIVE_ROUTING_CANDIDATES), long_about = descriptions::DERIVE_ROUTING_CANDIDATES)]
    DeriveRoutingCandidates(DeriveRoutingCandidatesArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_CORPUS_LINKS), long_about = descriptions::CHECK_CORPUS_LINKS)]
    CheckCorpusLinks(CheckCorpusLinksArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_ORPHANED_REFERENCES), long_about = descriptions::CHECK_ORPHANED_REFERENCES)]
    CheckOrphanedReferences(CheckOrphanedReferencesArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_COMMAND_FLAGS), long_about = descriptions::CHECK_COMMAND_FLAGS)]
    CheckCommandFlags(CheckCommandFlagsArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_STEP_REFERENCES), long_about = descriptions::CHECK_STEP_REFERENCES)]
    CheckStepReferences(CheckStepReferencesArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_UNFOLDED_SPECS), long_about = descriptions::CHECK_UNFOLDED_SPECS)]
    CheckUnfoldedSpecs(CheckUnfoldedSpecsArgs),
    #[command(about = descriptions::summary(descriptions::REWRITE_SPEC_LINKS), long_about = descriptions::REWRITE_SPEC_LINKS)]
    RewriteSpecLinks(RewriteSpecLinksArgs),
    #[command(about = descriptions::summary(descriptions::RETIRE_FEATURE), long_about = descriptions::RETIRE_FEATURE)]
    RetireFeature(RetireFeatureArgs),
    #[command(about = descriptions::summary(descriptions::INVALIDATE_REVIEW), long_about = descriptions::INVALIDATE_REVIEW)]
    InvalidateReview(InvalidateReviewArgs),
    #[command(about = descriptions::summary(descriptions::START_FIX_ROUNDS), long_about = descriptions::START_FIX_ROUNDS)]
    StartFixRounds(StartFixRoundsArgs),
    #[command(about = descriptions::summary(descriptions::BRIEF_RE_READ), long_about = descriptions::BRIEF_RE_READ)]
    BriefReRead(BriefReReadArgs),
    #[command(about = descriptions::summary(descriptions::READ_RE_READ), long_about = descriptions::READ_RE_READ)]
    ReadReRead(ReadReReadArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_PRESCRIPTIONS), long_about = descriptions::CHECK_PRESCRIPTIONS)]
    CheckPrescriptions(CheckPrescriptionsArgs),
    #[command(about = descriptions::summary(descriptions::DERIVE_DEPENDENCIES), long_about = descriptions::DERIVE_DEPENDENCIES)]
    DeriveDependencies(DeriveDependenciesArgs),
    #[command(about = descriptions::summary(descriptions::DERIVE_REFERENCES), long_about = descriptions::DERIVE_REFERENCES)]
    DeriveReferences(DeriveReferencesArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_ARTIFACTS), long_about = descriptions::CHECK_ARTIFACTS)]
    CheckArtifacts(CheckArtifactsArgs),
    #[command(about = descriptions::summary(descriptions::CHECK_ARTIFACT_SIZE), long_about = descriptions::CHECK_ARTIFACT_SIZE)]
    CheckArtifactSize(CheckArtifactSizeArgs),
    #[command(about = descriptions::summary(descriptions::PRUNE_TASKS), long_about = descriptions::PRUNE_TASKS)]
    PruneTasks(PruneTasksArgs),
    #[command(about = descriptions::summary(descriptions::PRUNE_PLAN), long_about = descriptions::PRUNE_PLAN)]
    PrunePlan(PrunePlanArgs),
    #[command(about = descriptions::summary(descriptions::PRUNE_PROJECT), long_about = descriptions::PRUNE_PROJECT)]
    PruneProject(PruneProjectArgs),
    #[command(about = descriptions::summary(descriptions::GATE_CONFIRM), long_about = descriptions::GATE_CONFIRM)]
    GateConfirm(GateConfirmArgs),
    #[command(about = descriptions::summary(descriptions::DASHBOARD), long_about = descriptions::DASHBOARD)]
    Dashboard(DashboardArgs),
    #[command(about = descriptions::summary(descriptions::RENDER_STATUS), long_about = descriptions::RENDER_STATUS)]
    RenderStatus(RenderStatusArgs),
    #[command(about = descriptions::summary(descriptions::RENDER_HELP), long_about = descriptions::RENDER_HELP)]
    RenderHelp(RenderHelpArgs),
    #[command(about = descriptions::summary(descriptions::READ_SECTION), long_about = descriptions::READ_SECTION)]
    ReadSection(ReadSectionArgs),
    #[command(about = descriptions::summary(descriptions::WRITE_SESSION), long_about = descriptions::WRITE_SESSION)]
    WriteSession(WriteSessionArgs),
    #[command(about = descriptions::summary(descriptions::RESOLVE_SESSION), long_about = descriptions::RESOLVE_SESSION)]
    ResolveSession(ResolveSessionArgs),
    #[command(about = descriptions::summary(descriptions::RETARGET_SESSIONS), long_about = descriptions::RETARGET_SESSIONS)]
    RetargetSessions(RetargetSessionsArgs),
}

/// Exit status of a run whose stdout reader went away before the result was written: the
/// shell's `128 + SIGPIPE`, distinct from a failure (1) and from a panic (101). Only the
/// report is lost; whatever the primitive wrote stands.
const STDOUT_CLOSED_EXIT: u8 = 141;

/// Write `text` and a newline to stdout. A broken pipe is a quiet exit
/// ([`STDOUT_CLOSED_EXIT`]) and any other write error is reported and exits 1: `println!`
/// panics on either, and `SIGPIPE` is ignored by the Rust runtime, so the write is the only
/// place the closed reader is seen. The `mcp` server's transport is not written here.
fn write_stdout_line(text: &str) -> std::result::Result<(), ExitCode> {
    let mut stdout = io::stdout().lock();
    match writeln!(stdout, "{text}").and_then(|()| stdout.flush()) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => {
            Err(ExitCode::from(STDOUT_CLOSED_EXIT))
        }
        Err(err) => {
            let _ = writeln!(io::stderr(), "failed to write result: {err}");
            Err(ExitCode::from(1))
        }
    }
}

fn emit_protocol_schema() -> ExitCode {
    let schema = schemars::schema_for!(ductus::schema::protocol::ProtocolMessage);
    match serde_json::to_string_pretty(&schema) {
        Ok(text) => match write_stdout_line(&text) {
            Ok(()) => ExitCode::SUCCESS,
            Err(code) => code,
        },
        Err(err) => {
            eprintln!("failed to serialize protocol schema: {err}");
            ExitCode::from(1)
        }
    }
}

/// Fill `apply-manifest`'s context-supplied fields from their `--*-json` paths.
fn hydrate_apply_manifest(args: &mut ApplyManifestArgs) -> Result<(), String> {
    hydrate_field(
        "entries-json",
        args.entries_json.as_ref(),
        &mut args.entries,
    )?;
    hydrate_field("pinned-json", args.pinned_json.as_ref(), &mut args.pinned)?;
    hydrate_field(
        "substitutions-json",
        args.substitutions_json.as_ref(),
        &mut args.substitutions,
    )
}

/// Fill `enforce-manifest`'s context-supplied fields from their `--*-json` paths.
fn hydrate_enforce_manifest(args: &mut EnforceManifestArgs) -> Result<(), String> {
    hydrate_field(
        "expected-json",
        args.expected_json.as_ref(),
        &mut args.expected,
    )?;
    hydrate_field("pinned-json", args.pinned_json.as_ref(), &mut args.pinned)
}

/// Fill `field` from the file `flag` names, when one was given. The field keeps
/// its value otherwise, so an absent flag is the absent MCP argument.
fn hydrate_field<T: serde::de::DeserializeOwned>(
    flag: &str,
    path: Option<&String>,
    field: &mut T,
) -> Result<(), String> {
    if let Some(path) = path {
        *field = ductus::primitives::load_cli_json(flag, path)?;
    }
    Ok(())
}

/// Fill `process-waivers`' structured argument from its `--*-json` path.
fn hydrate_process_waivers(args: &mut ProcessWaiversArgs) -> Result<(), String> {
    hydrate_field("fired-json", args.fired_json.as_ref(), &mut args.fired)
}

/// Fill `process-decisions`' structured argument from its `--*-json` path.
fn hydrate_process_decisions(args: &mut ProcessDecisionsArgs) -> Result<(), String> {
    hydrate_field("fired-json", args.fired_json.as_ref(), &mut args.fired)
}

/// Fill `write-review`'s structured arguments from their `--*-json` paths.
fn hydrate_write_review(args: &mut WriteReviewArgs) -> Result<(), String> {
    hydrate_field(
        "findings-json",
        args.findings_json.as_ref(),
        &mut args.findings,
    )?;
    hydrate_field(
        "applied-waivers-json",
        args.applied_waivers_json.as_ref(),
        &mut args.applied_waivers,
    )?;
    hydrate_field(
        "expired-waivers-json",
        args.expired_waivers_json.as_ref(),
        &mut args.expired_waivers,
    )?;
    hydrate_field(
        "new-waivers-json",
        args.new_waivers_json.as_ref(),
        &mut args.new_waivers,
    )?;
    hydrate_field(
        "observations-json",
        args.observations_json.as_ref(),
        &mut args.observations,
    )?;
    hydrate_field(
        "expired-decisions-json",
        args.expired_decisions_json.as_ref(),
        &mut args.expired_decisions,
    )?;
    hydrate_field(
        "decided-by-json",
        args.decided_by_json.as_ref(),
        &mut args.decided_by,
    )?;
    hydrate_field(
        "fix-rounds-json",
        args.fix_rounds_json.as_ref(),
        &mut args.fix_rounds,
    )?;
    hydrate_field("unread-json", args.unread_json.as_ref(), &mut args.unread)
}

/// Fill `write-analysis`' structured arguments from their `--*-json` paths.
fn hydrate_write_analysis(args: &mut WriteAnalysisArgs) -> Result<(), String> {
    hydrate_field(
        "findings-json",
        args.findings_json.as_ref(),
        &mut args.findings,
    )?;
    hydrate_field(
        "expired-decisions-json",
        args.expired_decisions_json.as_ref(),
        &mut args.expired_decisions,
    )?;
    hydrate_field(
        "decided-by-json",
        args.decided_by_json.as_ref(),
        &mut args.decided_by,
    )?;
    hydrate_field(
        "rule-verdicts-json",
        args.rule_verdicts_json.as_ref(),
        &mut args.rule_verdicts,
    )
}

/// Fill `prune-project`'s structured argument from its `--*-json` path.
fn hydrate_prune_project(args: &mut PruneProjectArgs) -> Result<(), String> {
    hydrate_field(
        "replace-json",
        args.replace_json.as_ref(),
        &mut args.replace,
    )
}

fn emit_result<T: serde::Serialize, E: std::fmt::Display>(
    result: std::result::Result<T, E>,
) -> ExitCode {
    emit_result_gated(result, |_| false)
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
                Ok(text) => match write_stdout_line(&text) {
                    Ok(()) if blocked => ExitCode::from(1),
                    Ok(()) => ExitCode::SUCCESS,
                    Err(code) => code,
                },
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

/// Whether `derive-dependencies` exits non-zero: a cycle, a rewrite that could
/// not be written (a stale index nothing here can fix, on a report-only run
/// and a writing one alike), or drift on a report-only run, which is the CI
/// check ("the committed indexes are stale"). On a writing run the drift was
/// just resolved, so it is not a failure.
fn derive_dependencies_blocks(r: &DeriveDependenciesResult) -> bool {
    r.cycles_count > 0 || r.unwritable_count > 0 || (!r.wrote && r.drift)
}

/// Whether `derive-references` exits non-zero: a rewrite that could not be
/// written, or drift on a report-only run.
fn derive_references_blocks(r: &DeriveReferencesResult) -> bool {
    !r.unwritable.is_empty() || (!r.wrote && r.drift)
}

/// The stderr line for each spec whose `key` rewrite the re-parse guard
/// refused, so an author whose commit the hook stops knows which spec and what
/// to do.
fn unwritable_lines(primitive: &str, key: &str, specs: &[String]) -> Vec<String> {
    specs
        .iter()
        .map(|spec| {
            format!(
                "{primitive}: {spec}: the derived `{key}:` could not be written without breaking its frontmatter; edit that key by hand."
            )
        })
        .collect()
}

/// Write [`unwritable_lines`] to stderr. A write that fails is dropped, as
/// `warn_unknown_keys` drops one: `eprintln!` panics when stderr is a pipe
/// whose reader has gone, and a report must not fail the run it reports on.
fn report_unwritable(primitive: &str, key: &str, specs: &[String]) {
    let mut stderr = io::stderr().lock();
    for line in unwritable_lines(primitive, key, specs) {
        let _ = writeln!(stderr, "{line}");
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
fn report_broken_links(result: &primitives::check_corpus_links::CorpusLinks) {
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

/// Whether a staged `check-artifact-size` exits non-zero: a project file
/// refused, or one that could not be measured. It reads `skipped-count`,
/// which is inline in both forms, so a result whose lists moved to its list
/// file past the read size never exits like one that measured everything.
fn size_check_blocks(result: &CheckArtifactSizeResult) -> bool {
    result.skipped_count > 0
        || result.project.as_ref().is_some_and(|half| {
            half.project_files
                .iter()
                .any(|file| file.refused == Some(true))
        })
}

/// Name each refused project file's path, size and limit on stderr, with its
/// one fix, so an author whose commit is stopped knows what to run. Each
/// unmeasurable one is named too: the commit is blocked for it as well. Both
/// are read from `project-files`, which stays inline in both forms: a staged
/// call names no feature, so its `skipped` holds exactly the project files
/// whose state is `unreadable`, each under `artifact-unreadable`.
fn report_refused_project_files(result: &CheckArtifactSizeResult) {
    let Some(half) = &result.project else {
        return;
    };
    let mut stderr = io::stderr().lock();
    for file in &half.project_files {
        if file.refused == Some(true) {
            let head = file.head_bytes.map_or_else(
                || "no HEAD copy".to_string(),
                |head| format!("HEAD copy {head} bytes"),
            );
            let _ = writeln!(
                stderr,
                "check-artifact-size: {} is {} bytes staged ({head}), over the {}-byte read size \
                 ([artifacts] read-size-bytes). Run /prune --project to move its entries to their homes. \
                 `git commit --no-verify` bypasses this deliberately.",
                file.path,
                file.bytes.unwrap_or_default(),
                result.threshold.bytes
            );
        }
    }
    for file in &half.project_files {
        if file.state == ProjectSizeState::Unreadable {
            let _ = writeln!(
                stderr,
                "check-artifact-size: {} could not be measured ({}), so its size was never checked.",
                file.path,
                primitives::check_artifact_size::UNREADABLE
            );
        }
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
                    Ok(text) => match write_stdout_line(&text) {
                        Ok(()) => ExitCode::SUCCESS,
                        Err(code) => code,
                    },
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
    if let Some(text) = ductus::primitives::read_text_if_present(&session_path)? {
        let parsed = toml::from_str::<Value>(&text).map_err(|source| {
            ductus::primitives::PrimitiveError::Toml {
                path: session_path.clone(),
                source,
            }
        })?;
        if let Value::Object(map) = parsed {
            context.extend(map);
        }
    }
    // Only a process with a record of its own overrides the default's keys.
    // Otherwise the default, read strictly above, seeds as it did before spec
    // 062 — and the seed is also a general-purpose one, whose `path` may be a
    // primitive argument with no `feature` beside it (a bootstrap walk's
    // `merge-managed-block`). A malformed default or own file halts the walk,
    // naming the file (AC22, `CFG-ENV-008`).
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
/// in `repo`, whose target then overrides the shared default's target keys in
/// the exec seed.
fn has_own_record(repo: &std::path::Path, identity: Option<&ductus::session::Identity>) -> bool {
    ductus::session::effective(repo, identity)
        .is_some_and(|identity| ductus::session::own_path(repo, identity).exists())
}

/// The files `ductus exec` tries for `command`, in order. The name is already screened.
fn exec_candidates(
    command: &str,
    host: &ductus::host::Host,
    repo: &std::path::Path,
) -> Vec<PathBuf> {
    let mut candidates = vec![
        repo.join("framework/commands")
            .join(format!("{command}.md")),
    ];
    // Installed command file under the adopter's config dir — `commands/`
    // (claude-style) or flat `prompts/` (pi); see
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
    candidates
}

fn run_exec(command: &str, args: &[String], repo: &std::path::Path) -> ExitCode {
    use ductus::host::Host;
    use ductus::interpreter::{WalkOutcome, Walker};
    use ductus::parser;
    use serde_json::Value;

    if let Err(message) = ductus::primitives::screen_command_name(command) {
        emit_exec_error("invalid-command-name", &message);
        eprintln!("ductus exec: {message}");
        return ExitCode::from(1);
    }
    let host = match Host::load(repo) {
        Ok(host) => host,
        Err(err) => {
            emit_exec_error("config-unreadable", &err.to_string());
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    let candidates = exec_candidates(command, &host, repo);
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
    let mut walker = match Walker::new(
        &procedure,
        repo.to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    ) {
        Ok(walker) => walker,
        Err(err) => {
            emit_exec_error("config-unreadable", &err.to_string());
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
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
    // `ductus hook` reads `CLAUDE_PROJECT_DIR` once at its own start, since
    // nothing else reads it.
    // The inventory is docs/runtime.md's. `init_proxy_env` captures the
    // certificate variables here too, where the verifier would read them (spec
    // 048, `fetch-archive-reads-its-proxy-once`); with neither set, the first
    // fetch reads them again through the platform store.
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
        Command::Hook { hook } => {
            // `CLAUDE_PROJECT_DIR` is read here, once, at the start of the one
            // subcommand that reads it (`CFG-ENV-001`).
            let root =
                ductus::hook::project_root(std::env::var_os(ductus::hook::PROJECT_DIR_VAR), repo);
            let mut input = io::stdin().lock();
            match hook {
                HookCommand::Status => emit_result(ductus::hook::status(&root, &mut input)),
                HookCommand::Help => emit_result(ductus::hook::help(&root, &mut input)),
            }
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
        Command::ResolveConstitutions(args) => emit_result(
            primitives::resolve_constitutions::run_measured(&args, &repo),
        ),
        Command::TraverseDeps(args) => emit_result(primitives::traverse_deps::run(&args, &repo)),
        Command::CheckDepartures(args) => {
            emit_result(primitives::check_departures::run(&args, &repo))
        }
        Command::CheckRuleIds(args) => emit_result(primitives::check_rule_ids::run(&args, &repo)),
        Command::ReadDeprecationLabels(args) => {
            emit_result(primitives::read_deprecation_labels::run(&args, &repo))
        }
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
        Command::ProcessWaivers(mut args) => match hydrate_process_waivers(&mut args) {
            Ok(()) => emit_result(primitives::process_waivers::run(&args, &repo)),
            Err(err) => emit_result::<(), String>(Err(err)),
        },
        Command::ProcessDecisions(mut args) => match hydrate_process_decisions(&mut args) {
            Ok(()) => emit_result(primitives::process_decisions::run(&args, &repo)),
            Err(err) => emit_result::<(), String>(Err(err)),
        },
        Command::ComputeReviewScope(args) => {
            emit_result(primitives::compute_review_scope::run(&args, &repo))
        }
        Command::WriteReview(mut args) => match hydrate_write_review(&mut args) {
            Ok(()) => emit_result(primitives::write_review::run(&args, &repo)),
            Err(err) => emit_result::<(), String>(Err(err)),
        },
        Command::WriteAnalysis(mut args) => match hydrate_write_analysis(&mut args) {
            Ok(()) => emit_result(primitives::write_analysis::run(&args, &repo)),
            Err(err) => emit_result::<(), String>(Err(err)),
        },
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
        Command::MergeStatusHook(args) => {
            emit_result(primitives::merge_status_hook::run(&args, &repo))
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
        Command::ClassifySpecEdit(args) => {
            emit_result(primitives::classify_spec_edit::run(&args, &repo))
        }
        Command::MeasureReopenings(args) => {
            emit_result(primitives::measure_reopenings::run(&args, &repo))
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
            // Every broken link is reported from the whole scan, before the
            // result is bounded, so a blocked commit names each one even when
            // the result moved its lists to a file (spec 080).
            let scan = primitives::check_corpus_links::compute(&args, &repo);
            if let Ok(scan) = &scan {
                report_broken_links(scan);
            }
            let outcome = scan.and_then(|scan| primitives::check_corpus_links::bound(scan, &repo));
            // A broken link blocks the commit — that is the whole point: a
            // deletion should fail at the commit that makes it rather than at
            // a reader's next traversal. A scan that could not establish a
            // subject, or could not read a file, blocks for the *other*
            // reason: a check that could not run must never exit like one
            // that passed.
            emit_result_gated(outcome, |r| {
                r.broken_count > 0 || !r.guidance.is_empty() || r.skipped_count > 0
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
        Command::StartFixRounds(args) => {
            emit_result(primitives::start_fix_rounds::run(&args, &repo))
        }
        Command::BriefReRead(args) => emit_result(primitives::brief_re_read::run(&args, &repo)),
        Command::ReadReRead(args) => emit_result(primitives::read_re_read::run(&args, &repo)),
        Command::CheckPrescriptions(args) => {
            emit_result(primitives::check_prescriptions::run(&args, &repo))
        }
        Command::DeriveDependencies(args) => {
            // The whole derivation is reported and gated on before the result
            // is bounded, so a cycle the bound moves to a file still blocks and
            // is still named on stderr (spec 080).
            let derived = primitives::derive_dependencies::compute(&args, &repo);
            if let Ok(derived) = &derived {
                report_cycles(&derived.cycles);
                report_unwritable("derive-dependencies", "dependencies", &derived.unwritable);
            }
            let outcome =
                derived.and_then(|derived| primitives::derive_dependencies::bound(&repo, derived));
            // A cycle always blocks. Drift blocks only on a *report-only*
            // run, which is the CI check ("the committed indexes are stale");
            // on a writing run the drift was just resolved, so it is not a
            // failure.
            emit_result_gated(outcome, derive_dependencies_blocks)
        }
        Command::DeriveReferences(args) => {
            // No graph here, so a stale index on a report-only run and a
            // rewrite that could not be written are the blockers.
            let outcome = primitives::derive_references::run(&args, &repo);
            if let Ok(result) = &outcome {
                report_unwritable("derive-references", "references", &result.unwritable);
            }
            emit_result_gated(outcome, derive_references_blocks)
        }
        Command::CheckArtifacts(args) => {
            emit_result(primitives::check_artifacts::run(&args, &repo))
        }
        Command::CheckArtifactSize(args) => {
            let outcome = primitives::check_artifact_size::run(&args, &repo);
            if args.staged {
                // The hook is the one caller that reads an exit status: a
                // commit that grows a project file past the limit is refused,
                // and so is one the check could not measure, since a check
                // that could not run must never exit like one that passed.
                if let Ok(result) = &outcome {
                    report_refused_project_files(result);
                }
                emit_result_gated(outcome, size_check_blocks)
            } else {
                emit_result(outcome)
            }
        }
        Command::PruneTasks(args) => emit_result(primitives::prune_tasks::run(&args, &repo)),
        Command::PrunePlan(args) => emit_result(primitives::prune_plan::run(&args, &repo)),
        Command::PruneProject(mut args) => match hydrate_prune_project(&mut args) {
            Ok(()) => emit_result(primitives::prune_project::run(&args, &repo)),
            Err(err) => emit_result::<(), String>(Err(err)),
        },
        Command::Dashboard(args) => emit_result(primitives::dashboard::run(&args, &repo)),
        Command::RenderStatus(args) => emit_result(primitives::render_status::run(&args, &repo)),
        Command::RenderHelp(args) => emit_result(primitives::render_help::run(&args, &repo)),
        Command::ReadSection(args) => emit_result(primitives::read_section::run(&args, &repo)),
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

    use super::{
        Cli, derive_dependencies_blocks, derive_references_blocks, seed_context, unwritable_lines,
    };
    use ductus::schema::primitives::{DeriveDependenciesResult, DeriveReferencesResult, ListsForm};

    fn dependencies_result() -> DeriveDependenciesResult {
        DeriveDependenciesResult {
            drift: false,
            updated: Some(vec![]),
            updated_count: 0,
            unwritten: Some(vec![]),
            unwritten_count: 0,
            examined: 1,
            untracked_skipped: Some(vec![]),
            untracked_skipped_count: 0,
            absent: Some(vec![]),
            absent_count: 0,
            cycles: Some(vec![]),
            cycles_count: 0,
            unparseable: Some(vec![]),
            unparseable_count: 0,
            unwritable: None,
            unwritable_count: 0,
            specs_root: "specs".into(),
            wrote: false,
            form: ListsForm::Inline,
            lists_file: None,
        }
    }

    fn references_result() -> DeriveReferencesResult {
        DeriveReferencesResult {
            drift: false,
            updated: vec![],
            unwritten: vec![],
            examined: 1,
            untracked_skipped: vec![],
            registered_services: 0,
            unparseable: vec![],
            unwritable: vec![],
            absent: vec![],
            specs_root: "specs".into(),
            wrote: false,
        }
    }

    /// An unwritable rewrite fails both derivations, report-only and writing,
    /// where an unparseable spec alone does not.
    #[test]
    fn an_unwritable_rewrite_exits_non_zero_and_an_unparseable_spec_does_not() {
        for wrote in [false, true] {
            let mut deps = dependencies_result();
            deps.wrote = wrote;
            assert!(!derive_dependencies_blocks(&deps), "clean, wrote {wrote}");
            deps.unparseable = Some(vec!["specs/001-a/spec.md".into()]);
            deps.unparseable_count = 1;
            assert!(
                !derive_dependencies_blocks(&deps),
                "unparseable, wrote {wrote}"
            );
            deps.unwritable = Some(vec!["specs/001-a/spec.md".into()]);
            deps.unwritable_count = 1;
            assert!(
                derive_dependencies_blocks(&deps),
                "unwritable, wrote {wrote}"
            );

            let mut refs = references_result();
            refs.wrote = wrote;
            assert!(!derive_references_blocks(&refs), "clean, wrote {wrote}");
            refs.unparseable = vec!["specs/001-a/spec.md".into()];
            assert!(
                !derive_references_blocks(&refs),
                "unparseable, wrote {wrote}"
            );
            refs.unwritable = vec!["specs/001-a/spec.md".into()];
            assert!(derive_references_blocks(&refs), "unwritable, wrote {wrote}");
        }
    }

    /// One stderr line per unwritable spec, naming the spec and the key and
    /// saying to edit it by hand.
    #[test]
    fn each_unwritable_spec_gets_one_line_naming_it_and_saying_to_edit_by_hand() {
        let lines = unwritable_lines(
            "derive-references",
            "references",
            &[
                "specs/001-a/spec.md".to_string(),
                "specs/002-b/spec.md".to_string(),
            ],
        );
        assert_eq!(lines.len(), 2);
        for (line, spec) in lines
            .iter()
            .zip(["specs/001-a/spec.md", "specs/002-b/spec.md"])
        {
            assert!(line.contains(spec), "{line}");
            assert!(line.contains("`references:`"), "{line}");
            assert!(line.contains("edit that key by hand"), "{line}");
        }
        assert_eq!(
            unwritable_lines("derive-references", "references", &[]),
            [] as [String; 0]
        );
    }

    /// Each primitive's `about` is the start of its `long_about`, so `-h` and the
    /// `ductus --help` list summarise the text `--help` prints whole (spec 079, AC9),
    /// and a variant that dropped either attribute fails here.
    #[test]
    fn each_primitives_about_summarises_its_long_about() {
        let command = Cli::command();
        for name in PRIMITIVE_REGISTRY {
            let sub = command
                .find_subcommand(name)
                .unwrap_or_else(|| panic!("no subcommand for {name}"));
            let about = sub.get_about().map(ToString::to_string).unwrap_or_default();
            let long_about = sub
                .get_long_about()
                .map(ToString::to_string)
                .unwrap_or_default();
            assert!(!about.is_empty(), "{name} has no about");
            assert!(
                long_about.starts_with(&about),
                "{name}: {about:?} does not start {long_about:?}"
            );
        }
    }

    /// Every subcommand's summary and every argument's short help prints on
    /// one line. clap's help renderer replaces a brace-wrapped `n` with a
    /// newline (`StyledStr::replace_newline_var`), so a doc comment naming a
    /// criterion label that way once split `label-criteria`'s summary in two
    /// in both `ductus --help` and `ductus label-criteria --help`.
    #[test]
    fn every_short_help_prints_on_one_line() {
        fn splits(text: &str) -> bool {
            text.contains('\n') || text.contains("{n}")
        }
        fn walk(cmd: &clap::Command, path: &str, split: &mut Vec<String>) {
            if let Some(about) = cmd.get_about().map(ToString::to_string)
                && splits(&about)
            {
                split.push(format!("{path}: {about:?}"));
            }
            for arg in cmd.get_arguments() {
                if let Some(help) = arg.get_help().map(ToString::to_string)
                    && splits(&help)
                {
                    split.push(format!("{path} {}: {help:?}", arg.get_id()));
                }
            }
            for sub in cmd.get_subcommands() {
                walk(sub, &format!("{path} {}", sub.get_name()), split);
            }
        }
        let mut split = Vec::new();
        walk(&Cli::command(), "ductus", &mut split);
        assert!(split.is_empty(), "short help spanning lines: {split:#?}");
    }

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

    /// A malformed shared default halts the seed for a process with no record
    /// of its own, naming the file, as a malformed own record does (AC22,
    /// `CFG-ENV-008`).
    #[test]
    fn a_malformed_default_halts_the_seed_naming_the_file_as_a_malformed_own_record_does() {
        let tmp = repo_with_default("feature = [\n");
        let err = seed_context(tmp.path(), None).unwrap_err();
        assert!(err.to_string().contains("session.toml"), "{err}");
        let review = Identity::named("review").unwrap();
        let err = seed_context(tmp.path(), Some(&review)).unwrap_err();
        assert!(err.to_string().contains("session.toml"), "{err}");

        std::fs::write(tmp.path().join(".ductus/session.toml"), "").unwrap();
        std::fs::create_dir_all(tmp.path().join(".ductus/sessions")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/sessions/review.toml"),
            "feature = [",
        )
        .unwrap();
        let err = seed_context(tmp.path(), Some(&review)).unwrap_err();
        assert!(err.to_string().contains("review.toml"), "{err}");
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
    /// drive, plus the `--emit-schema` debug surface. `hook` adapts an agent
    /// host's hook protocol, and the capabilities it adapts, the pipeline view
    /// `hook status` shows and the guide `hook help` shows, are served on MCP
    /// by `render-status` and `render-help` (§runtime-boundary principle 5;
    /// spec 080) — a cli-only primitive would need one of that principle's
    /// three exemptions, and rendering a view or a guide meets none. None of
    /// the four is a primitive, so none appears in [`PRIMITIVE_REGISTRY`].
    /// `help` is synthesised by clap rather than declared here at all.
    ///
    /// A future primitive that genuinely should have no CLI surface is added
    /// here with its own reason. Nothing may be excluded silently: the point
    /// of the set-equality is that *not* being a subcommand has to be a
    /// stated decision.
    const NON_PRIMITIVE_SUBCOMMANDS: &[&str] = &["mcp", "exec", "parse", "hook", "help"];

    /// The clap subcommand enum is the fifth primitive-registration surface,
    /// and it was the one nothing pinned: with a `Command` variant and its
    /// dispatch arm deleted the entire suite still passed, so a primitive
    /// could be missing from `ductus <name>` with no test reporting it. That
    /// absence bites hardest where no server runs, since hooks, CI and the
    /// bootstrap have only this surface — [§design-principles]' first rule turned on
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

    /// Every argument a served primitive's MCP tool accepts has a CLI form: a
    /// flag of the same name, or its own `--{name}-json FILE` for a structured
    /// one (spec 022, scenario `every-mcp-argument-has-a-cli-form`). The MCP
    /// side is read from the tool's input schema, which is the allowlist the
    /// server enforces, so an argument added to a primitive's `Args` struct
    /// without a clap field makes this fail naming the primitive and argument.
    /// Arguments are matched by clap id, which is the field name, so a flag
    /// renamed with `long = ...` still counts.
    #[test]
    fn every_mcp_argument_has_a_cli_form() {
        use ductus::mcp::server::GovRuntimeServer;
        use rmcp::ServiceExt;

        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let tools = runtime.block_on(async {
            let (server_side, client_side) = tokio::io::duplex(64 * 1024);
            let server = GovRuntimeServer::new(std::env::temp_dir());
            tokio::spawn(async move {
                if let Ok(service) = server.serve(server_side).await {
                    let _ = service.waiting().await;
                }
            });
            let client = ().serve(client_side).await.unwrap();
            client.list_tools(Option::default()).await.unwrap().tools
        });
        assert!(!tools.is_empty(), "the MCP server listed no tools");

        let command = Cli::command();
        let mut uncovered = Vec::new();
        let mut examined = 0_usize;
        for tool in &tools {
            let subcommand = command
                .find_subcommand(tool.name.as_ref())
                .unwrap_or_else(|| panic!("no CLI subcommand for tool `{}`", tool.name));
            let ids: BTreeSet<&str> = subcommand
                .get_arguments()
                .map(|arg| arg.get_id().as_str())
                .collect();
            let properties = tool
                .input_schema
                .get("properties")
                .and_then(serde_json::Value::as_object);
            for name in properties.into_iter().flat_map(serde_json::Map::keys) {
                examined += 1;
                let field = name.replace('-', "_");
                if !ids.contains(field.as_str()) && !ids.contains(format!("{field}_json").as_str())
                {
                    uncovered.push(format!("{}: {name}", tool.name));
                }
            }
        }
        assert!(examined > 0, "no MCP argument was examined");
        assert!(
            uncovered.is_empty(),
            "MCP arguments with no CLI flag and no `--<name>-json` flag: {uncovered:?}"
        );
    }

    /// The `--{name}-json` flags read their file as the MCP argument: a
    /// structured argument supplied through one reaches the primitive, and a
    /// file outside a staging directory is refused naming the path.
    #[test]
    fn a_json_flag_fills_its_argument_and_is_staging_bound() {
        use ductus::schema::primitives::{ProcessWaiversArgs, WriteReviewArgs};

        let dir = tempfile::tempdir().unwrap();
        let staged = dir.path().join("fired.json");
        std::fs::write(&staged, r#"[{"rule":"R-1","file":"a.rs"}]"#).unwrap();
        let mut args = ProcessWaiversArgs {
            fired_json: Some(staged.to_string_lossy().into_owned()),
            ..Default::default()
        };
        super::hydrate_process_waivers(&mut args).unwrap();
        assert_eq!(args.fired.len(), 1);
        assert_eq!(args.fired[0].rule, "R-1");

        // Malformed JSON for the field is an error naming the flag, never an empty default.
        std::fs::write(&staged, r#"{"rule":"R-1"}"#).unwrap();
        let err = super::hydrate_process_waivers(&mut args).unwrap_err();
        assert!(err.starts_with("--fired-json: "), "{err}");

        // A file outside a staging directory is refused before it is read.
        let outside = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let mut refused = ProcessWaiversArgs {
            fired_json: Some(outside.to_string_lossy().into_owned()),
            ..Default::default()
        };
        let err = super::hydrate_process_waivers(&mut refused).unwrap_err();
        assert!(err.starts_with("--fired-json: "), "{err}");

        // An explicit empty list is distinct from the flag being absent.
        let empty = dir.path().join("rounds.json");
        std::fs::write(&empty, "[]").unwrap();
        let mut review = WriteReviewArgs {
            fix_rounds_json: Some(empty.to_string_lossy().into_owned()),
            ..Default::default()
        };
        super::hydrate_write_review(&mut review).unwrap();
        assert_eq!(review.fix_rounds, Some(vec![]));
        let mut absent = WriteReviewArgs::default();
        super::hydrate_write_review(&mut absent).unwrap();
        assert_eq!(absent.fix_rounds, None);
    }
}
