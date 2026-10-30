//! Deterministic primitive operations.
//!
//! Each primitive has a pure-Rust `run` function (no stdout/stderr I/O — the
//! caller wraps the result into a JSON envelope), a `clap`-derive args struct
//! from [`crate::schema::primitives`], and a unit test against a fixture file
//! under `runtime/tests/fixtures/primitives/`.

#![allow(clippy::module_name_repetitions)]

use std::borrow::Cow;
use std::io::Write;
use std::path::{Path, PathBuf};

/// The analyze record's subject set and freshness comparison, shared by the
/// completion gate, `write-review`'s row, and `write-analysis`.
pub(crate) mod analyze_subjects;
pub mod append_inbox;
pub mod append_question;
pub mod append_task;
pub mod apply_manifest;
pub mod check_artifact_size;
pub mod check_artifacts;
pub mod check_command_flags;
pub mod check_corpus_links;
pub mod check_orphaned_references;
pub mod check_promotion_coverage;
pub mod check_review_gate;
pub mod check_rule_ids;
pub mod check_step_references;
pub mod check_stuck;
pub mod check_unfolded_specs;
pub mod compute_review_scope;
pub mod create_feature;
pub mod create_plan_artifacts;
pub mod create_scenario;
pub mod dashboard;
pub(crate) mod decisions;
pub mod derive_boundary;
pub mod derive_dependencies;
pub mod derive_references;
pub mod derive_routing_candidates;
pub mod diff_cross_spec;
pub mod discover_rule_files;
pub mod enforce_manifest;
pub mod extract_archive;
pub mod fetch_archive;
pub mod gate_confirm;
pub(crate) mod host_config;
pub(crate) mod inbox_standing;
pub mod invalidate_review;
pub mod label_criteria;
pub mod lint_markdown;
pub mod mark_criterion;
pub mod mark_task;
pub mod mechanical_sweep;
pub mod merge_managed_block;
pub mod merge_permissions;
pub mod migrate_session_file;
pub mod process_decisions;
pub mod process_waivers;
pub mod prune_plan;
pub mod prune_project;
pub mod prune_tasks;
pub mod read_spec;
pub mod read_tasks;
pub mod relocate_audit_records;
pub mod remove_inbox_item;
pub mod resolve_anchor;
pub mod resolve_constitutions;
pub mod resolve_feature;
pub mod resolve_references;
pub mod resolve_session;
pub mod retarget_sessions;
pub mod retire_feature;
pub mod rewrite_spec_links;
pub(crate) mod rule_sections;
pub mod run_generator;
pub mod set_status;
pub(crate) mod spec_links;
pub mod traverse_deps;
pub mod validate_frontmatter;
pub mod write_analysis;
pub mod write_review;
pub mod write_session;

/// Operational errors common to every primitive. Domain outcomes (findings,
/// violations, drift) are reported through the result struct; this enum is
/// reserved for operational failures that halt the procedure.
#[derive(Debug, thiserror::Error)]
pub enum PrimitiveError {
    /// I/O failure on a specific path.
    #[error("I/O error on {path}: {source}")]
    Io {
        /// Path involved in the failed operation.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// YAML parse failure for a frontmatter block.
    #[error("YAML parse error in {path}: {source}")]
    Yaml {
        /// Path of the file whose frontmatter failed to parse.
        path: PathBuf,
        /// Underlying YAML error.
        #[source]
        source: serde_norway::Error,
    },
    /// File has no leading `---` frontmatter block.
    #[error("frontmatter missing in {path} (no leading `---` block)")]
    MissingFrontmatter {
        /// Path of the offending file.
        path: PathBuf,
    },
    /// File opens a `---` frontmatter block that no closing `---` line ends.
    #[error("frontmatter in {path} opens with `---` but never closes")]
    UnclosedFrontmatter {
        /// Path of the offending file.
        path: PathBuf,
    },
    /// `next-criterion` frontmatter is present but not a positive integer.
    /// Refused rather than repaired: a corrupted counter may mean a retired
    /// `AC{n}` label was already reissued, and silently rewriting it would
    /// hide that (spec 013, `criterion-identifiers`).
    #[error("next-criterion in {path} is not a positive integer: {value:?}")]
    InvalidNextCriterion {
        /// Path of the spec carrying the invalid counter.
        path: PathBuf,
        /// The offending value, verbatim.
        value: String,
    },
    /// Feature directory does not exist under the configured spec-root.
    #[error("feature directory not found: {root}/{feature}")]
    FeatureNotFound {
        /// Configured spec-root directory name (default `specs`; spec 040).
        root: String,
        /// Requested feature name.
        feature: String,
    },
    /// Git operation failed.
    #[error("git error: {0}")]
    Git(#[from] git2::Error),
    /// Requested task number not found in `tasks.md`.
    #[error("task '{task_number}' not found in {root}/{feature}/tasks.md")]
    TaskNotFound {
        /// Configured spec-root directory name (default `specs`; spec 040).
        root: String,
        /// Feature whose tasks file was scanned.
        feature: String,
        /// Task number that was requested.
        task_number: String,
    },
    /// Subtask index is out of bounds for the located task.
    #[error(
        "subtask index {subtask_index} is out of range for task '{task_number}' (found {total})"
    )]
    SubtaskOutOfRange {
        /// Feature whose tasks file was scanned.
        feature: String,
        /// Task number whose subtasks were counted.
        task_number: String,
        /// Requested subtask index.
        subtask_index: usize,
        /// Number of subtasks present.
        total: usize,
    },
    /// Acceptance-criterion index is out of bounds.
    #[error(
        "criterion index {criterion_index} is out of range for {root}/{feature}/spec.md (found {total})"
    )]
    CriterionOutOfRange {
        /// Configured spec-root directory name (default `specs`; spec 040).
        root: String,
        /// Feature whose spec was scanned.
        feature: String,
        /// Requested criterion index.
        criterion_index: usize,
        /// Number of acceptance criteria present.
        total: usize,
    },
    /// `mark-criterion` was given a label no criterion carries. A stale
    /// reference is surfaced rather than silently ignored — naming a
    /// criterion that does not exist is the condition labels exist to
    /// catch (spec 013).
    #[error("no criterion labelled {label} in {root}/{feature}/spec.md")]
    CriterionLabelNotFound {
        /// Configured spec-root directory name.
        root: String,
        /// Feature whose spec was scanned.
        feature: String,
        /// The label requested.
        label: String,
    },
    /// `mark-criterion` was given neither, or both, of `criterion-index`
    /// and `label`.
    #[error("mark-criterion needs exactly one of criterion-index or label ({detail})")]
    CriterionAddressAmbiguous {
        /// Which of the two degenerate forms was supplied.
        detail: String,
    },
    /// `set-status` was invoked with a `from` value that does not match disk.
    #[error("status mismatch in {root}/{feature}/spec.md: expected '{expected}', found '{actual}'")]
    StatusMismatch {
        /// Configured spec-root directory name (default `specs`; spec 040).
        root: String,
        /// Feature whose spec was scanned.
        feature: String,
        /// Status the caller expected on disk.
        expected: String,
        /// Status actually present on disk.
        actual: String,
    },
    /// Frontmatter does not contain a `status:` field.
    #[error("frontmatter in {root}/{feature}/spec.md has no `status:` field")]
    StatusFieldMissing {
        /// Configured spec-root directory name (default `specs`; spec 040).
        root: String,
        /// Feature whose spec was scanned.
        feature: String,
    },
    /// HTTP fetch returned a non-success status code.
    #[error("HTTP {status} fetching {url}")]
    HttpStatus {
        /// URL that returned the error.
        url: String,
        /// HTTP status code observed.
        status: u16,
    },
    /// Underlying `reqwest` failure (connect refused, TLS error, etc.).
    #[error("HTTP error on {url}: {source}")]
    Http {
        /// URL involved in the failed request.
        url: String,
        /// Underlying reqwest error.
        #[source]
        source: reqwest::Error,
    },
    /// sha256 sidecar did not match the computed hash of the downloaded archive.
    #[error("sha256 mismatch for {path}: sidecar declared {expected}, computed {actual}")]
    ChecksumMismatch {
        /// Local path of the archive whose sha didn't match.
        path: PathBuf,
        /// Hex digest declared in the sidecar.
        expected: String,
        /// Hex digest computed locally.
        actual: String,
    },
    /// sha256 sidecar payload didn't parse as `<hex>  <filename>` format.
    #[error("malformed sha256 sidecar from {url}: {reason}")]
    MalformedSidecar {
        /// URL the sidecar was fetched from.
        url: String,
        /// One-line description of what was malformed.
        reason: String,
    },
    /// Archive format could not be inferred from extension and no override given.
    #[error("unknown archive format for {path} (expected .tar.gz/.tgz/.zip)")]
    UnknownArchiveFormat {
        /// Local archive path whose format couldn't be determined.
        path: PathBuf,
    },
    /// Archive entry path that is not safe to join to the destination: an
    /// absolute path, a `..` component, or a name that is not UTF-8.
    #[error("unsafe archive entry path: {entry}")]
    UnsafeArchivePath {
        /// Entry path as it appeared inside the archive.
        entry: String,
    },
    /// CLAUDE.md merge found a BEGIN marker without a matching END (or
    /// vice versa).
    #[error("malformed managed-block markers in {path}: {reason}")]
    MalformedMarkers {
        /// Path of the file whose markers were malformed.
        path: PathBuf,
        /// One-line description of the structural failure.
        reason: String,
    },
    /// A manifest entry, or a managed-block merge, named a strategy the
    /// primitive does not know. `expected` names the valid set for the
    /// primitive that raised it: `apply-manifest`'s `update`, `create` and
    /// `skip-if-conflict`, or `merge-managed-block`'s marker styles.
    #[error("unknown manifest strategy '{strategy}' (expected {expected})")]
    UnknownManifestStrategy {
        /// Strategy string as it appeared in the manifest entry.
        strategy: String,
        /// The values the raising primitive accepts, quoted and joined.
        expected: String,
    },
    /// A substitution key was given in placeholder form (`{project}`) rather
    /// than bare (`project`), or was empty.
    ///
    /// `apply-manifest` builds each placeholder by wrapping the key in braces,
    /// so a key that already carries them yields `{{project}}` and matches
    /// nothing. That is not a near-miss the caller can be trusted to notice:
    /// every strategy still runs, every file is still written, and the counts
    /// come back looking like a successful run while every placeholder in
    /// every copied file survives literally. An adopter reported exactly this
    /// — 370 unsubstituted placeholders across the constitution, all 17
    /// commands, 5 rule files and 2 templates, with `/{project}:review` in
    /// place of the real command name and no error, no warning.
    ///
    /// A key containing a brace can never match a well-formed placeholder, so
    /// rejecting it is exact rather than heuristic. The check runs before any
    /// filesystem operation, so a bad map halts the walk with zero writes
    /// rather than leaving a half-substituted tree.
    #[error(
        "substitution key '{key}' is malformed — keys are bare (`project`), not placeholder-shaped (`{{project}}`); the primitive adds the braces"
    )]
    InvalidSubstitutionKey {
        /// The offending key, as it appeared in the substitutions map.
        key: String,
    },
    /// `create-scenario` refused to overwrite an existing scenario file.
    #[error("scenario already exists: {path}")]
    ScenarioConflict {
        /// Path of the existing scenario file the primitive refused to overwrite.
        path: PathBuf,
    },
    /// `create-feature` found no spec template at any candidate location,
    /// so there is nothing to copy into the new feature directory. Raised
    /// before the directory is created, so a missing template leaves no
    /// half-scaffolded feature behind.
    #[error("spec template not found (tried {tried})")]
    TemplateNotFound {
        /// Comma-separated repo-relative candidate paths that were tried.
        tried: String,
    },
    /// Feature path supplied to a primitive does not exist.
    #[error("feature path does not exist: {path}")]
    FeaturePathNotFound {
        /// Caller-supplied feature path that did not resolve to a directory.
        path: PathBuf,
    },
    /// Slug supplied by a caller failed the slug-grammar allowlist
    /// (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, BE-INPUT-002) — empty, or holding a
    /// character outside lowercase-alphanumeric-plus-single-hyphen (path
    /// separators, dots, whitespace, control characters, uppercase).
    #[error("invalid slug '{slug}': {reason}")]
    InvalidSlug {
        /// Slug that was rejected.
        slug: String,
        /// One-line reason describing the rejection.
        reason: String,
    },
    /// Caller-supplied path failed traversal-safety validation.
    #[error("invalid path '{path}': {reason}")]
    InvalidPath {
        /// Path that was rejected.
        path: String,
        /// One-line reason describing the rejection.
        reason: String,
    },
    /// A required argument was omitted by the caller. Distinct from
    /// `InvalidSlug` / `InvalidPath` — the value was never supplied, not
    /// supplied-and-rejected.
    #[error("{primitive}: '{argument}' is required ({reason})")]
    MissingArgument {
        /// Primitive name (e.g., `append-task`).
        primitive: String,
        /// Argument name that was omitted.
        argument: String,
        /// One-line reason explaining why the argument is required in
        /// this context.
        reason: String,
    },
    /// A supplied argument value failed validation (e.g., embedded
    /// newlines in a single-line field). Distinct from
    /// [`PrimitiveError::MissingArgument`] — the value was supplied but
    /// rejected.
    #[error("{primitive}: invalid '{argument}': {reason}")]
    InvalidArgument {
        /// Primitive name (e.g., `append-task`).
        primitive: String,
        /// Argument name carrying the rejected value.
        argument: String,
        /// One-line reason describing the rejection.
        reason: String,
    },
    /// An external tool could not be launched. Distinct from
    /// [`PrimitiveError::Io`] because that variant names a *path the
    /// primitive was operating on*, and using it for a spawn failure named
    /// the repository — the one thing that was definitely present — while
    /// the missing executable went unnamed. Observed 2026-08-27: an
    /// unresolvable `npx` reported "I/O error on <repo>: No such file or
    /// directory", which reads as a missing fixture and sent the reader
    /// looking for the wrong thing (spec 022, scenario
    /// `lint-markdown-tool-resolution`).
    #[error("could not launch {program}: {source}{}", .guidance.as_ref().map(|g| format!(" — {g}")).unwrap_or_default())]
    ToolLaunch {
        /// The executable that could not be launched, as invoked.
        program: String,
        /// Underlying spawn error.
        #[source]
        source: std::io::Error,
        /// Actionable guidance, attached only when the failure is
        /// `NotFound` — a permissions or other spawn error must not carry
        /// a `PATH` explanation it has no basis for.
        guidance: Option<String>,
    },
    /// A subprocess a primitive runs outlived its bounded timeout, so it was
    /// killed and its output discarded. A child that never exits would
    /// otherwise hold the primitive — and every caller waiting on it — forever
    /// (`BE-TIMEOUT-001`).
    #[error("{program} did not finish within {seconds}s and was stopped")]
    ToolTimedOut {
        /// The executable that was stopped, as invoked.
        program: String,
        /// The timeout it outlived, in seconds.
        seconds: u64,
    },
    /// `set-status` was invoked with a `from` or `to` value outside the
    /// constitution's lifecycle set. Transition-edge legality stays with
    /// procedures; the primitive guards set membership only.
    #[error("set-status: '{argument}' value '{value}' is not one of {allowed}")]
    InvalidStatus {
        /// Argument name (`from` or `to`) carrying the invalid value.
        argument: String,
        /// The rejected status value.
        value: String,
        /// Pipe-joined allowed lifecycle set.
        allowed: String,
    },
    /// `append-task` was called with a `parent-heading` argument that does
    /// not match any `## …` phase container in the target `tasks.md`.
    #[error(
        "append-task: parent-heading '{heading}' not found in tasks.md (available: {available})"
    )]
    ParentHeadingNotFound {
        /// Caller-supplied heading text that didn't match.
        heading: String,
        /// Comma-separated list of available phase headings (for the
        /// operator to choose from when retrying).
        available: String,
    },
    /// `merge-permissions` was invoked with an entry present in both the
    /// canonical `allow` set and the `revoke` set. The two passes would
    /// fight — revoke removes it, the canonical-presence pass appends it
    /// back — so the merge would never reach a fixed point and the
    /// `unchanged` short-circuit could never fire. Rejected as a caller
    /// error rather than resolved by pass order, because either order
    /// silently discards half of what the caller asked for.
    #[error(
        "merge-permissions: {count} entr{plural} appear in both --allow and --revoke: {entries}"
    )]
    ConflictingRevoke {
        /// Number of conflicting entries.
        count: usize,
        /// `y` / `ies`, so the message reads correctly for one or many.
        plural: &'static str,
        /// Comma-separated conflicting entries, in canonical-set order.
        entries: String,
    },
    /// JSON parse failure (e.g., `merge-permissions` reading a malformed
    /// `.claude/settings.local.json`).
    #[error("JSON parse error in {path}: {source}")]
    Json {
        /// Path of the file whose JSON failed to parse.
        path: PathBuf,
        /// Underlying `serde_json` error.
        #[source]
        source: serde_json::Error,
    },
    /// JSON parsed but its shape doesn't match the primitive's expected
    /// schema (e.g., `permissions.allow` exists but is not an array).
    #[error("JSON schema mismatch in {path}: {reason}")]
    JsonSchema {
        /// Path of the file whose JSON shape was rejected.
        path: PathBuf,
        /// One-line description of the schema mismatch.
        reason: String,
    },
    /// TOML parse failure (e.g., `dashboard` reading a malformed resolved
    /// project config).
    #[error("TOML parse error in {path}: {source}")]
    Toml {
        /// Path of the file whose TOML failed to parse.
        path: PathBuf,
        /// Underlying TOML error.
        #[source]
        source: toml::de::Error,
    },
    /// Spec directory missing its `spec.md` file. `dashboard` raises this
    /// when an `NNN-feature` directory under the configured spec-root lacks
    /// the expected `spec.md` — the directory naming convention promises one.
    #[error("missing spec.md in {root}/{feature}")]
    MissingSpecFile {
        /// Configured spec-root directory name (default `specs`; spec 040).
        root: String,
        /// Feature directory name that lacks a `spec.md`.
        feature: String,
    },
    /// Feature directory exists but has no `tasks.md`. `prune-tasks` raises
    /// this for a named feature, and a walk records the feature as a
    /// `no-tasks-file` skip instead; `/{project}:prune` directs to the plan
    /// phase only when the feature has no `plan.md` either.
    #[error("tasks.md not found: {root}/{feature}/tasks.md")]
    TasksFileMissing {
        /// Configured spec-root directory name (default `specs`; spec 040).
        root: String,
        /// Feature whose tasks file is missing.
        feature: String,
    },
    /// `prune-tasks --reset` found a `tasks.md` with no `# …` heading, so it
    /// cannot preserve the feature identity for the reset. Writes nothing.
    #[error("malformed tasks.md at {path}: {reason}")]
    MalformedTasks {
        /// Path of the offending tasks file.
        path: PathBuf,
        /// One-line description of the structural problem.
        reason: String,
    },
    /// `[rules] surfaces` named a member outside `{backend, frontend}`.
    /// `discover-rule-files` fails fast rather than silently ignoring it.
    #[error(
        "invalid [rules] surfaces member \"{value}\" — accepted members are \"backend\" and \"frontend\" (use [] for cross-only; -cross.md files always apply)"
    )]
    InvalidSurfacesMember {
        /// The offending member string.
        value: String,
    },
    /// `[rules] surfaces` was set to something other than a list of strings.
    #[error("[rules] surfaces must be a list of strings, got {got}")]
    InvalidSurfacesType {
        /// Human-readable description of the actual type found.
        got: String,
    },
    /// A `[constitutions.<alias>]` entry parsed but carries a value the
    /// registry's schema forbids (spec 055).
    ///
    /// Distinct from [`PrimitiveError::Toml`], which covers a document that
    /// will not parse: this entry *did* parse, so nothing but the value check
    /// would ever name it. Raised rather than reported as an outcome because
    /// `framework/bootstrap/ductus.md` §Validating the registry makes it a
    /// halt per `CFG-ENV-008` — a malformed entry is a mistake in the
    /// project's own committed config and is always wrong, unlike an
    /// unresolved checkout, which stays the warning-level `not-checked-out`
    /// state. The asymmetry is the whole point; collapsing the two would tell
    /// an operator to clone a repository when the mistake is in their config.
    ///
    /// The alias is escaped for display, never raw: a TOML quoted key may
    /// carry a newline, and a non-bare key is precisely what this error
    /// reports, so a raw rendering would let committed config forge a second
    /// line in an operator-facing message (`BE-INPUT-011`). The field stays
    /// verbatim for callers.
    #[error("invalid entry in {path}: [constitutions.{}] `{field}` {reason}", .alias.escape_debug())]
    InvalidConstitutionEntry {
        /// Path of the config file carrying the offending entry.
        path: PathBuf,
        /// `[constitutions.<alias>]` — the offending entry, verbatim.
        alias: String,
        /// Which field was rejected — `alias`, `repo`, or `path`.
        field: String,
        /// One-line description of why, phrased to follow the field name.
        reason: String,
    },
    /// `[artifacts] read-size-bytes` is present but is not a positive whole
    /// number of bytes, or `artifacts` is not a table that could hold it
    /// (spec 063).
    ///
    /// Raised rather than reported because a present value that fails to
    /// parse, or parses outside its range, must halt the command instead of
    /// letting the default stand in for what the project set (`CFG-ENV-008`,
    /// and `CFG-ENV-005` applied to a file-based key). An unset key is not an
    /// error: it means the default.
    ///
    /// The value is flattened to one line, never raw: a TOML string may carry
    /// a newline, and a raw rendering would let committed config forge a
    /// second line in an operator-facing message (`BE-INPUT-011`).
    #[error(
        "invalid `{setting}` in {path}: {value} — `[{section}] {key}` takes a whole number of bytes, at least {min}; remove it to use the {default}-byte default",
        section = check_artifact_size::SECTION,
        key = check_artifact_size::KEY,
        min = check_artifact_size::MIN_READ_SIZE_BYTES,
        default = check_artifact_size::DEFAULT_READ_SIZE_BYTES
    )]
    InvalidReadSize {
        /// Path of the config file carrying the value.
        path: PathBuf,
        /// `[artifacts] read-size-bytes`, or `artifacts` when the section
        /// itself is not a table.
        setting: String,
        /// The value as TOML renders it, flattened to one line.
        value: String,
    },
    /// An environment variable the runtime reads at startup is set to a
    /// value that does not parse into its declared type, so the primitive
    /// that reads it halts rather than acting as though it were unset
    /// (`CFG-ENV-008`). `reason` names the offending value, on one line.
    #[error("invalid {variable}: {reason}")]
    InvalidEnvironmentVariable {
        /// The variable's name.
        variable: String,
        /// Why the value was rejected, naming it.
        reason: String,
    },
    /// A `[[review.disabled-rule-files]]` entry is malformed: it is not a
    /// table, its `file` or `reason` is missing or not a string, or its
    /// `reason` is shorter than the minimum once trimmed (spec 025).
    ///
    /// Raised before any rule file is selected, rather than reported and the
    /// entry skipped, because a present value that does not parse must halt
    /// the command instead of being dropped (`CFG-ENV-008`). It is a
    /// configuration error that stops `/{project}:review`, never a review
    /// finding, so `blocking: true` keeps meaning only that a MUST rule was
    /// violated. `problem` names the failing field and carries any offending
    /// value flattened to one line, for the reason [`Self::InvalidReadSize`]
    /// gives (`BE-INPUT-011`).
    #[error(
        "invalid [[review.disabled-rule-files]] entry [{index}] in {path}: {problem} — each entry is a table with a `file` naming a rule file and a `reason` of at least {min} characters",
        min = discover_rule_files::MIN_REASON_LEN
    )]
    InvalidDisabledRuleFile {
        /// Path of the config file carrying the entry.
        path: PathBuf,
        /// The entry's 0-based position in the array, as the duplicate
        /// warning numbers it.
        index: usize,
        /// What failed, naming the field: `` `reason` is missing ``.
        problem: String,
    },
}

/// Convenience alias for primitive return values.
pub type Result<T> = std::result::Result<T, PrimitiveError>;

/// The three states loading an audit record can produce.
///
/// Absent and unreadable are **different answers**, and collapsing the second
/// into the first is the failure spec 057 AC5 names: a `review.md` that exists
/// but will not parse says nothing about whether a review ran, while a
/// `review.md` that is not there says a review did not. Reporting the former as
/// never-run claims a fact the artifact never supplied — `QUAL-CLAIM-001`
/// applied to the gate's own inputs.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RecordLoad<T> {
    /// The artifact does not exist. The never-run state, and information in
    /// its own right: every run writes its artifact, so absence is never
    /// ambiguous between "no run" and "nothing to report".
    Absent,
    /// The artifact exists but its frontmatter could not be read or parsed.
    /// Carries the reason, so a caller reports *why* it is undeterminable
    /// rather than that it is.
    Unreadable(String),
    /// The record, as the artifact carries it.
    Present(T),
}

impl<T> RecordLoad<T> {
    /// The record when one was read, else `None`.
    ///
    /// Reaching for this is an explicit decision to treat absent and unreadable
    /// alike, which is right for a caller whose subject is something else —
    /// and wrong for a gate, which is why the gate matches on the variants.
    pub(crate) fn as_present(&self) -> Option<&T> {
        match self {
            Self::Present(record) => Some(record),
            Self::Absent | Self::Unreadable(_) => None,
        }
    }
}

/// Load an audit record from the frontmatter of the artifact that owns it.
///
/// A missing file is [`RecordLoad::Absent`]; every other failure — unreadable
/// bytes, a missing frontmatter fence, YAML that will not deserialize — is
/// [`RecordLoad::Unreadable`] with the reason.
pub(crate) fn load_record<T: serde::de::DeserializeOwned>(path: &Path) -> RecordLoad<T> {
    if !path.exists() {
        return RecordLoad::Absent;
    }
    let content = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(source) => return RecordLoad::Unreadable(source.to_string()),
    };
    let (fm_text, _body) = match split_frontmatter(&content, path) {
        Ok(split) => split,
        Err(error) => return RecordLoad::Unreadable(error.to_string()),
    };
    match serde_norway::from_str::<T>(fm_text) {
        Ok(record) => RecordLoad::Present(record),
        Err(error) => RecordLoad::Unreadable(error.to_string()),
    }
}

/// The review record, from `{feature_dir}/review.md`.
pub(crate) fn load_review_record(
    feature_dir: &Path,
) -> RecordLoad<crate::schema::primitives::ReviewBlock> {
    load_record(&feature_dir.join(REVIEW_RECORD_FILE))
}

/// The analyze record, from `{feature_dir}/analysis.md`.
pub(crate) fn load_analyze_record(
    feature_dir: &Path,
) -> RecordLoad<crate::schema::primitives::AnalyzeBlock> {
    load_record(&feature_dir.join(ANALYSIS_RECORD_FILE))
}

/// The waivers recorded in `review.md`'s frontmatter.
///
/// Generic over the waiver shape because the callers want different ones —
/// `process-waivers` parses loosely so a malformed entry is a reportable
/// warning rather than a whole-file parse failure, while `write-review` parses
/// with `#[serde(flatten)]` so an organization's custom fields survive the
/// re-render. What they must **not** differ on is where the list lives, which
/// is the one thing this function owns.
///
/// An absent `review.md` yields an empty list rather than an error: no review,
/// no waivers. That is a state, not a failure.
pub(crate) fn read_recorded_waivers<T: serde::de::DeserializeOwned>(
    feature_dir: &Path,
) -> Result<Vec<T>> {
    read_recorded_list(feature_dir, REVIEW_RECORD_FILE, "waivers")
}

/// A recorded waiver's `file`: one repo-relative path, or a list of them.
///
/// Each path is its own `(rule, file)` anchor, so an entry listing N paths is
/// N anchors sharing one `reason`, `waived-at` and `waived-by` (spec 020's
/// `waiver-file-lists`). Both spellings read into the same list; the writer
/// chooses the spelling from the count, never from how the entry was written.
/// A value that is neither — a number, a mapping, a list holding a non-string —
/// is a parse error, as a non-string known field is.
pub(crate) struct WaiverPaths(Vec<String>);

impl WaiverPaths {
    /// The listed paths, in record order.
    pub(crate) fn as_slice(&self) -> &[String] {
        &self.0
    }

    /// Whether the value names no usable anchor: an empty list, or any blank
    /// path. `process-waivers` reports either as `missing 'file'`.
    pub(crate) fn is_blank(&self) -> bool {
        self.0.is_empty() || self.0.iter().any(|path| path.trim().is_empty())
    }

    /// Keep only the paths `keep` accepts.
    pub(crate) fn retain(&mut self, keep: impl FnMut(&String) -> bool) {
        self.0.retain(keep);
    }
}

impl<'de> serde::Deserialize<'de> for WaiverPaths {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        struct PathsVisitor;

        impl<'de> serde::de::Visitor<'de> for PathsVisitor {
            type Value = WaiverPaths;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "a path or a list of paths")
            }

            fn visit_str<E: serde::de::Error>(
                self,
                path: &str,
            ) -> std::result::Result<Self::Value, E> {
                Ok(WaiverPaths(vec![path.to_owned()]))
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut paths = Vec::new();
                while let Some(path) = seq.next_element::<String>()? {
                    paths.push(path);
                }
                Ok(WaiverPaths(paths))
            }
        }

        deserializer.deserialize_any(PathsVisitor)
    }
}

/// A list recorded under `key` in the frontmatter of `feature_dir/file`.
///
/// The one reader for every list an audit record carries beside its counts —
/// `review.md`'s `waivers:`, and both records' `decisions:` (spec 058). Each
/// list is read apart from the record struct, so a list that will not parse
/// leaves the record itself readable. Within a list, each caller's entry
/// shape makes every known field an optional string, so an entry missing a
/// field deserializes and the caller reports it; an entry that is not a
/// mapping, or a known field holding a non-string such as `reason: [a, b]`,
/// does not, and fails the whole list. Owning the location here is what keeps
/// those readers from disagreeing about it.
///
/// An absent file, a file that opens no frontmatter block, or an absent key
/// yields an empty list: no record, or a record with nothing decided, is a
/// state, and `write-review` and `write-analysis` overwrite such a file whole.
/// A block that never closes is [`PrimitiveError::UnclosedFrontmatter`], and
/// a key whose value will not deserialize into the list is
/// [`PrimitiveError::Yaml`] — errors, **never** an empty list: a `decisions:`
/// list that silently read as empty would re-ask every settled question, and
/// a `waivers:` list that did would silently re-block.
pub(crate) fn read_recorded_list<T: serde::de::DeserializeOwned>(
    feature_dir: &Path,
    file: &str,
    key: &str,
) -> Result<Vec<T>> {
    let path = feature_dir.join(file);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let content = read_text(&path)?;
    let fm_text = match split_frontmatter(&content, &path) {
        Ok((fm_text, _body)) => fm_text,
        Err(PrimitiveError::MissingFrontmatter { .. }) => return Ok(Vec::new()),
        Err(other) => return Err(other),
    };
    serde::de::DeserializeSeed::deserialize(
        KeyedList::<T> {
            key,
            item: std::marker::PhantomData,
        },
        serde_norway::Deserializer::from_str(fm_text),
    )
    .map_err(|source| PrimitiveError::Yaml {
        path: path.clone(),
        source,
    })
}

/// Deserializes the one list stored under `key` in a frontmatter mapping and
/// skips every other entry unread.
///
/// Deliberately a visitor rather than a parse into `serde_norway::Value`: a
/// `Value` mapping rejects a duplicated key *anywhere* in the frontmatter,
/// while the derived struct this replaced ignored every key it did not name.
/// A hand-edited `review.md` with a repeated scalar must keep yielding its
/// waivers, so only a repeat of `key` itself is an error here.
struct KeyedList<'k, T> {
    key: &'k str,
    item: std::marker::PhantomData<T>,
}

impl<'de, T: serde::de::DeserializeOwned> serde::de::DeserializeSeed<'de> for KeyedList<'_, T> {
    type Value = Vec<T>;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> std::result::Result<Self::Value, D::Error> {
        deserializer.deserialize_map(self)
    }
}

impl<'de, T: serde::de::DeserializeOwned> serde::de::Visitor<'de> for KeyedList<'_, T> {
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a frontmatter mapping")
    }

    // An empty frontmatter block carries no list at all.
    fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Self::Value, E> {
        Ok(Vec::new())
    }

    fn visit_none<E: serde::de::Error>(self) -> std::result::Result<Self::Value, E> {
        Ok(Vec::new())
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(
        self,
        mut map: A,
    ) -> std::result::Result<Self::Value, A::Error> {
        let mut found: Option<Vec<T>> = None;
        // Keys are read as values rather than strings, so a non-string key
        // elsewhere in the frontmatter is skipped exactly as the derived
        // struct skipped it.
        while let Some(entry_key) = map.next_key::<serde_norway::Value>()? {
            if entry_key.as_str() != Some(self.key) {
                map.next_value::<serde::de::IgnoredAny>()?;
                continue;
            }
            if found.is_some() {
                return Err(serde::de::Error::custom(format!(
                    "duplicate entry with key \"{}\"",
                    self.key
                )));
            }
            found = Some(map.next_value()?);
        }
        Ok(found.unwrap_or_default())
    }
}

/// The artifact owning the review record.
pub(crate) const REVIEW_RECORD_FILE: &str = "review.md";

/// The artifact owning the analyze record.
///
/// `analysis.md`, not `analyze.md`: the runtime already pairs `write-review`
/// with `write-analysis`, and `review.md` takes its command's noun rather than
/// its verb (spec 057).
pub(crate) const ANALYSIS_RECORD_FILE: &str = "analysis.md";

/// Split a markdown file's content into its frontmatter YAML block and the
/// body that follows. Returns [`PrimitiveError::MissingFrontmatter`] when no
/// `---` opening fence is present and [`PrimitiveError::UnclosedFrontmatter`]
/// when no closing `---` line follows it. The closing fence may be the file's
/// last line with no newline after it.
pub(crate) fn split_frontmatter<'a>(content: &'a str, path: &Path) -> Result<(&'a str, &'a str)> {
    let (fm_text, body, _fm_offset) = split_frontmatter_with_offset(content, path)?;
    Ok((fm_text, body))
}

/// Like [`split_frontmatter`], but also returns the byte offset of the
/// frontmatter text within `content` — the length of the opener fence that
/// actually matched (4 for `---\n`, 5 for `---\r\n`). Callers that splice
/// edits back into the full file (e.g. `set-status`) need the real offset;
/// hardcoding the LF opener corrupts CRLF checkouts by one byte.
pub(crate) fn split_frontmatter_with_offset<'a>(
    content: &'a str,
    path: &Path,
) -> Result<(&'a str, &'a str, usize)> {
    let (after_open, fm_offset) = ["---\n", "---\r\n"]
        .iter()
        .find_map(|opener| {
            content
                .strip_prefix(opener)
                .map(|rest| (rest, opener.len()))
        })
        .ok_or_else(|| PrimitiveError::MissingFrontmatter { path: path.into() })?;

    // Empty frontmatter (`---\n---\n`): the closing fence is the very next
    // line, so there is no preceding newline for the `\n---` search below
    // to find. Present-but-empty frontmatter is a validation concern
    // (missing required fields), not a missing-frontmatter halt.
    for fence in ["---\n", "---\r\n"] {
        if let Some(body) = after_open.strip_prefix(fence) {
            return Ok(("", body, fm_offset));
        }
    }

    for fence in ["\n---\n", "\n---\r\n"] {
        if let Some(idx) = after_open.find(fence) {
            return Ok((
                &after_open[..idx],
                &after_open[idx + fence.len()..],
                fm_offset,
            ));
        }
    }
    // No closing line ends in a newline, so the only candidate left is a
    // closing fence that ends the file.
    if after_open == "---" {
        return Ok(("", "", fm_offset));
    }
    if let Some(fm_text) = after_open.strip_suffix("\n---") {
        return Ok((fm_text, "", fm_offset));
    }
    Err(PrimitiveError::UnclosedFrontmatter { path: path.into() })
}

/// Lowercase hex of `bytes` — the form every recorded sha256 digest takes.
pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut acc, byte| {
        use std::fmt::Write as _;
        let _ = write!(acc, "{byte:02x}");
        acc
    })
}

/// The line ending an existing text file uses, for a rewrite that has to
/// give the file back the way it found it.
///
/// A primitive that reassembles a file it did not create — splitting it into
/// lines, changing some, and re-joining — silently rewrites every line's
/// ending unless it asks. `str::lines()` strips a trailing `\r`, so joining
/// with a bare `\n` converts a CRLF checkout's file to LF as a side effect
/// of changing one line: a one-line edit lands as a whole-file diff, and
/// worse, a writer that reassembles only *part* of a file leaves the two
/// halves disagreeing.
///
/// **This is the one place the detection lives.** It replaced three
/// independent copies (`derive-dependencies`, `derive-references`, and
/// `create-feature`'s fold-target stamp), each of which had solved it alone
/// while seven other writers had not solved it at all. A second copy is how
/// they diverge (spec 051, scenario `rewrites-preserve-line-endings`).
///
/// **Mixed endings resolve to the dominant one**, ties to `\n`. A file whose
/// endings already disagree has no state to preserve, so it is normalized
/// rather than having the disagreement encoded permanently; and a file with
/// no line ending at all — empty, or one unterminated line — carries no
/// evidence either way, so it takes the platform-neutral default.
pub(crate) fn line_ending_of(content: &str) -> &'static str {
    let crlf = content.matches("\r\n").count();
    // Every `\r\n` contains an `\n`, so the bare-LF count is the difference.
    let bare_lf = content.matches('\n').count() - crlf;
    if crlf > bare_lf { "\r\n" } else { "\n" }
}

/// Re-join `lines` with `ending`, terminated by exactly one `ending`.
///
/// The shape most reassembling writers want: a file is a sequence of lines
/// and ends with a single newline. Callers that must preserve an *absent*
/// trailing newline join by hand with [`line_ending_of`]'s result instead.
pub(crate) fn join_lines_terminated<S: AsRef<str>>(lines: &[S], ending: &str) -> String {
    let mut out = String::new();
    for line in lines {
        out.push_str(line.as_ref());
        out.push_str(ending);
    }
    out
}

/// Join blocks of lines into a file: each block trimmed of its leading and
/// trailing blank lines, an all-blank block dropped, one blank line between
/// blocks, and one trailing `ending`. The `markdownlint`-clean seams
/// `prune-tasks` and `prune-plan` leave where they removed a section.
///
/// `ending` is the source file's own, so a reduction never converts a CRLF
/// checkout to LF as a side effect.
pub(crate) fn join_blocks(blocks: &[&[String]], ending: &str) -> String {
    let rendered: Vec<String> = blocks
        .iter()
        .map(|lines| {
            let mut start = 0;
            let mut end = lines.len();
            while start < end && lines[start].trim().is_empty() {
                start += 1;
            }
            while end > start && lines[end - 1].trim().is_empty() {
                end -= 1;
            }
            lines[start..end].join(ending)
        })
        .filter(|block| !block.is_empty())
        .collect();
    let mut out = rendered.join(&format!("{ending}{ending}"));
    out.push_str(ending);
    out
}

/// Re-terminate every line of `text` with `ending`.
///
/// The companion to [`line_ending_of`] for writers that assemble their
/// output with `\n` throughout — rendering new blocks with `writeln!`,
/// splicing, stitching — and would need the ending threaded through a dozen
/// pushes to do it line by line. Those normalize once at the end instead,
/// which is both smaller and harder to get half-right: a threaded parameter
/// that one push forgets produces a mixed file, and mixed is the outcome
/// worth avoiding most (spec 051, scenario
/// `rewrites-preserve-line-endings`).
///
/// Idempotent: input already using `ending` comes back unchanged, because
/// each line is stripped of a trailing `\r` before the ending is applied.
/// The tail after the final `\n` is emitted as-is — it is an unterminated
/// line, not a line ending, so there is nothing to convert.
pub(crate) fn with_line_ending(text: &str, ending: &str) -> String {
    // Fast path: already LF-only and LF is what was asked for.
    if ending == "\n" && !text.contains('\r') {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len() + text.len() / 32);
    let mut rest = text;
    while let Some(idx) = rest.find('\n') {
        let line = &rest[..idx];
        out.push_str(line.strip_suffix('\r').unwrap_or(line));
        out.push_str(ending);
        rest = &rest[idx + 1..];
    }
    out.push_str(rest);
    out
}

/// Read a UTF-8 file, surfacing path context on failure.
pub(crate) fn read_text(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|source| PrimitiveError::Io {
        path: path.into(),
        source,
    })
}

/// Render a path as a repo-relative POSIX string, falling back to the
/// path's display form if it doesn't share a prefix with `repo`.
pub(crate) fn rel_path(path: &Path, repo: &Path) -> String {
    let display = path.strip_prefix(repo).unwrap_or(path);
    display.to_string_lossy().replace('\\', "/")
}

/// Read the frontmatter `status` value from a markdown document's content,
/// collapsing every unreadability (missing/unclosed frontmatter, invalid
/// YAML, frontmatter that doesn't parse as spec
/// [`Frontmatter`](crate::schema::primitives::Frontmatter), missing or
/// non-string `status`) to `None`.
///
/// The shared READ-only status reader: `traverse-deps` (dependency status),
/// `resolve-references` (linked-spec status, with its own membership policy
/// on top), and `check-stuck` (spec blobs from git history) all consume it.
/// The write path (`set-status`) keeps its span-preserving
/// `locate_status_field` instead — it must splice the value in place, not
/// just read it.
pub(crate) fn frontmatter_status(content: &str, path: &Path) -> Option<String> {
    let (fm_text, _body) = split_frontmatter(content, path).ok()?;
    serde_norway::from_str::<crate::schema::primitives::Frontmatter>(fm_text)
        .ok()
        .map(|fm| fm.status)
}

/// Atomically write `content` to `path` using `tempfile`'s create-then-rename
/// pattern. The tempfile is created in `path`'s parent directory so the rename
/// stays on the same filesystem (POSIX guarantee). A crash between creation
/// and persist leaves `path` unchanged; the orphaned tempfile is the only
/// recovery artifact.
pub(crate) fn write_atomic(path: &Path, content: &str) -> Result<()> {
    write_atomic_bytes(path, content.as_bytes())
}

/// Atomically write a byte slice to `path`. Same tempfile-plus-rename
/// pattern as [`write_atomic`]; used by primitives that produce binary
/// payloads (e.g., `fetch-archive` writing a downloaded tarball).
pub(crate) fn write_atomic_bytes(path: &Path, content: &[u8]) -> Result<()> {
    // Capture the destination's existing mode (Unix) so an in-place rewrite
    // preserves it. `NamedTempFile` is created 0600 and `persist` renames it
    // over the target, so without this every rewrite would narrow an existing
    // 0644 file to owner-only. New files keep the tempfile default; a
    // primitive that writes an *executable* re-applies its mode after this
    // returns (see `apply-manifest`'s `mirror_source_mode`).
    #[cfg(unix)]
    let prior_mode = {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path)
            .ok()
            .map(|meta| meta.permissions().mode())
    };
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|source| PrimitiveError::Io {
            path: parent.into(),
            source,
        })?;
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|source| PrimitiveError::Io {
        path: parent.into(),
        source,
    })?;
    tmp.as_file_mut()
        .write_all(content)
        .map_err(|source| PrimitiveError::Io {
            path: path.into(),
            source,
        })?;
    tmp.as_file_mut()
        .sync_all()
        .map_err(|source| PrimitiveError::Io {
            path: path.into(),
            source,
        })?;
    tmp.persist(path).map_err(|err| PrimitiveError::Io {
        path: path.into(),
        source: err.error,
    })?;
    #[cfg(unix)]
    if let Some(mode) = prior_mode {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).map_err(
            |source| PrimitiveError::Io {
                path: path.into(),
                source,
            },
        )?;
    }
    Ok(())
}

/// Shared helpers for identifying and flipping markdown task-list checkbox
/// lines (`- [ ] ...` / `- [x] ...`). Used by both `mark-task` and
/// `mark-criterion`; the regex is `^(\s*-\s+)\[([ xX])\](\s.*)?$`, expressed
/// directly via byte inspection to avoid pulling in `regex` for this hot path.
pub(crate) mod checkbox {
    /// Return `(prefix_end, marker_index)` when `line` is a task-list
    /// checkbox line. `prefix_end` is the byte index of the `[`; `marker_index`
    /// is the byte index of the space/x/X marker character.
    pub(crate) fn find_checkbox_line(line: &str) -> Option<(usize, usize)> {
        let bytes = line.as_bytes();
        let mut idx = 0;
        while idx < bytes.len() && matches!(bytes[idx], b' ' | b'\t') {
            idx += 1;
        }
        if bytes.get(idx) != Some(&b'-') {
            return None;
        }
        idx += 1;
        let mut saw_space = false;
        while idx < bytes.len() && matches!(bytes[idx], b' ' | b'\t') {
            saw_space = true;
            idx += 1;
        }
        if !saw_space {
            return None;
        }
        if bytes.get(idx) != Some(&b'[') {
            return None;
        }
        let bracket_idx = idx;
        let marker_idx = idx + 1;
        if !matches!(bytes.get(marker_idx), Some(&b' ' | &b'x' | &b'X')) {
            return None;
        }
        if bytes.get(marker_idx + 1) != Some(&b']') {
            return None;
        }
        match bytes.get(marker_idx + 2) {
            Some(&b' ' | &b'\t' | &b'\n' | &b'\r') | None => Some((bracket_idx, marker_idx)),
            _ => None,
        }
    }

    /// Return `(previous_state, rewritten_line)` after flipping the marker at
    /// `marker_idx` (obtained from [`find_checkbox_line`]) to `desired`.
    pub(crate) fn flip_checkbox_at(line: &str, marker_idx: usize, desired: bool) -> (bool, String) {
        let previous = matches!(line.as_bytes()[marker_idx], b'x' | b'X');
        let mut out = String::with_capacity(line.len());
        out.push_str(&line[..marker_idx]);
        out.push(if desired { 'x' } else { ' ' });
        out.push_str(&line[marker_idx + 1..]);
        (previous, out)
    }

    /// Parse a checkbox line into `(checked, text)`, where `text` is the
    /// trimmed content after the `]`. Recognition delegates to
    /// [`find_checkbox_line`], so the read side (`read-spec` criteria,
    /// `read-tasks` subtasks) and the mark side (`mark-task`,
    /// `mark-criterion`) share one grammar — the read/mark index contract
    /// requires that a checkbox counted by a reader is addressable by the
    /// matching marker, and vice versa.
    pub(crate) fn parse_checkbox_line(line: &str) -> Option<(bool, String)> {
        let (_bracket, marker_idx) = find_checkbox_line(line)?;
        let checked = matches!(line.as_bytes()[marker_idx], b'x' | b'X');
        let text = line[marker_idx + 2..].trim().to_string();
        Some((checked, text))
    }
}

/// The task "Done when" clause label, matched case-insensitively by
/// [`parse_done_when`]. `append-task` writes the canonical bold form
/// (`- **Done when**: …`); the template documents it and `/ductus:plan`'s
/// task-breakdown reference names it.
const DONE_WHEN_LABEL: &str = "Done when";

/// Recognize a task's "Done when" clause in any authoring form the writers
/// and the corpus produce, returning its trimmed body:
///
/// - `- **Done when**: <body>` — the form [`crate::primitives::append_task`]
///   emits and the `tasks.md` template documents (runtime-canonical);
/// - `- [x] Done when: <body>` / `- [ ] Done when: <body>` — the checkbox
///   form `/ductus:plan`'s LLM-authored task breakdown tends to produce (the
///   plan step fills the template directly, not via `append-task`);
/// - `Done when: <body>` — the bulletless form early specs used.
///
/// The leading list bullet, an optional task-list checkbox, the `**`
/// emphasis around the label, and the `:` separator are all optional (the
/// corpus carries both `Done when: …` and the colon-less `Done when <cond>`
/// forms); the label is matched case-insensitively. To keep a prose subtask
/// that merely opens with a longer word (`Done whenever …`) from being read
/// as a clause, the label must land on a **word boundary** — the character
/// after it must be the `:` separator, a closing `**`, whitespace, or the
/// end of the line.
///
/// Shared by [`crate::primitives::read_tasks`] (records the body) and
/// [`crate::primitives::mark_task`] (excludes the clause line from the
/// subtask index space) so a checkbox-form clause is treated as a clause by
/// both sides, never as an addressable subtask — the read/mark index contract.
pub(crate) fn parse_done_when(line: &str) -> Option<String> {
    // Peel an optional task-list checkbox via the shared grammar
    // (`- [x] Done when: …`); otherwise peel an optional plain list bullet
    // (`- Done when: …`); otherwise take the line for the bulletless form
    // (`Done when: …`).
    let after_marker = if let Some((_checked, text)) = checkbox::parse_checkbox_line(line) {
        text
    } else {
        let trimmed = line.trim_start();
        trimmed
            .strip_prefix("- ")
            .unwrap_or(trimmed)
            .trim()
            .to_string()
    };
    // Optional opening `**` emphasis, then the case-insensitive label.
    let label_start = after_marker
        .trim_start()
        .trim_start_matches('*')
        .trim_start();
    let rest = strip_label_ci(label_start, DONE_WHEN_LABEL)?;
    // Word-boundary guard: the label must be followed by a separator, not
    // more letters (rejects `Done whenever …`).
    match rest.chars().next() {
        Some(c) if !(c == ':' || c == '*' || c.is_whitespace()) => return None,
        _ => {}
    }
    // Strip the separator decoration — an optional `**` emphasis and an
    // optional `:` in either order (`**Done when**:`, `**Done when:**`,
    // `Done when:`, and the colon-less `Done when <cond>` all reduce
    // cleanly) — without eating a body that itself opens with `**emphasis**`.
    let sep = rest.strip_prefix("**").unwrap_or(rest);
    let sep = sep.strip_prefix(':').unwrap_or(sep);
    let sep = sep.strip_prefix("**").unwrap_or(sep);
    Some(sep.trim().to_string())
}

/// Case-insensitively strip an ASCII `label` prefix from `s`, returning the
/// remainder. Uses [`str::get`] so a multibyte-char boundary at
/// `label.len()` yields a clean `None` rather than a slice panic.
fn strip_label_ci<'a>(s: &'a str, label: &str) -> Option<&'a str> {
    let head = s.get(..label.len())?;
    head.eq_ignore_ascii_case(label)
        .then_some(&s[label.len()..])
}

/// Resolve a caller-supplied path argument against the repo root, accepting
/// absolute paths as-is.
///
/// This is the accept-absolute-paths counterpart to
/// [`validate_no_traversal`], for the paths that are bases rather than
/// arguments a primitive opens: `apply-manifest`'s `source-root` and
/// `target-root`, which its entries are checked against — `apply-manifest`
/// wraps this in [`resolve_root`] to bound the roots by the working directory
/// (spec 022 task 17) — and the sibling
/// checkouts committed config names (`resolve-constitutions`,
/// `resolve-references`; see **The config-sourced path boundary** on
/// [`validate_no_traversal`]). `run-generator` passes its script through
/// [`validate_contained`] first, so it never sees an absolute one. A path a
/// primitive opens goes through [`contained_path`] (or
/// [`validate_contained`]), and a staging path through
/// [`validate_in_temp_dir`], never through this alone.
/// `enforce-manifest` keeps its own stricter `resolve_contained_dir`
/// (absolute allowed only under the repo root) because its cleanup loop is
/// destructive.
pub(crate) fn resolve_path(repo: &Path, path_arg: &str) -> PathBuf {
    let candidate = Path::new(path_arg);
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        repo.join(candidate)
    }
}

/// Resolve and bound a manifest root, `apply-manifest`'s `source-root` or
/// `target-root`. The roots were absolute-capable bases; spec 022 task 17
/// (the owner's answer (a)) bounds them by the working directory `repo`, the
/// base every other path argument is held to. `target-root` (`allow_staging`
/// false) must resolve inside `repo`; `source-root` (`allow_staging` true)
/// inside `repo` or strictly inside a staging directory, as `extract-archive`
/// takes its paths ([`validate_in_temp_dir`]).
///
/// The bound is on where the root *resolves*, so an absolute root outside
/// `repo`, a relative one climbing out with `..`, and one through a symlink
/// that leaves `repo` are all refused — `resolve_symlinks` resolves the root
/// (and its existing ancestors) the same way [`validate_contained`] resolves
/// an entry, and the result is compared whole-component against the resolved
/// `repo` (`Path::starts_with`). A `source-root` is matched against the
/// staging roots *as given* ([`validate_in_temp_dir`] requires an absolute
/// path), so a relative `source-root` that links into a staging directory is
/// refused rather than followed — the staging arm is for the absolute staging
/// path `/ductus` passes, not a link into one.
///
/// Returns the resolved base (as [`resolve_path`] would), which the caller's
/// per-entry [`validate_contained`] then re-canonicalizes.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when the root resolves outside its allowed
/// bases; [`PrimitiveError::Io`] when `repo` or the root cannot be resolved.
pub(crate) fn resolve_root(
    repo: &Path,
    label: &str,
    root_arg: &str,
    allow_staging: bool,
) -> Result<PathBuf> {
    let resolved = resolve_path(repo, root_arg);
    let canonical_repo = canonical_base(repo)?;
    let canonical = resolve_symlinks(&resolved).map_err(|source| PrimitiveError::Io {
        path: resolved.clone(),
        source,
    })?;
    if canonical.starts_with(&canonical_repo) {
        return Ok(resolved);
    }
    if allow_staging && validate_in_temp_dir(root_arg).is_ok() {
        return Ok(resolved);
    }
    Err(PrimitiveError::InvalidPath {
        path: root_arg.into(),
        reason: if allow_staging {
            format!("{label} resolves outside the working directory and is not a staging directory")
        } else {
            format!("{label} resolves outside the working directory")
        },
    })
}

/// Run `cmd` to completion with stdout and stderr captured, as
/// [`std::process::Command::output`] does, but stop it once it has run for
/// `timeout`: the child is killed and `Ok(None)` returned. Both pipes are
/// drained on their own threads, so a child that fills a pipe cannot block
/// while this waits for it to exit. Stdin is closed, as `output` closes it.
///
/// # Errors
///
/// The spawn or wait error, as `output` would return it.
pub(crate) fn output_within(
    cmd: &mut std::process::Command,
    timeout: std::time::Duration,
) -> std::io::Result<Option<std::process::Output>> {
    use std::io::Read;
    use std::process::Stdio;
    fn drain<R: Read + Send + 'static>(pipe: Option<R>) -> std::thread::JoinHandle<Vec<u8>> {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut buf);
            }
            buf
        })
    }
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());
    let started = std::time::Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    Ok(Some(std::process::Output {
        status,
        stdout: stdout.join().unwrap_or_default(),
        stderr: stderr.join().unwrap_or_default(),
    }))
}

/// Reject caller-supplied paths that contain parent-directory components
/// (`..`) or absolute prefixes — the lexical half of the BE-INPUT-004 check.
/// It reads the text alone, so a symlink inside the project that points out
/// of it passes; a primitive that opens the path calls [`validate_contained`]
/// (or [`validate_feature_argument`]), which runs this first and then
/// resolves the links. A primitive that only records a path calls this alone.
///
/// # The config-sourced path boundary
///
/// This is the canonical statement of the converse, cited rather than
/// restated by the primitives that rely on it (`resolve-constitutions`
/// for `[constitutions.*]`, `resolve-references` for `[services]`).
///
/// A primitive that reads a path out of the project's **committed config**
/// may skip this check, and `..` is the normal case there: a sibling
/// checkout (`path = "../governance"`) is the layout the manual prescribes.
/// The trust does not come from anything the primitive can see in the
/// value — it comes from **how the value got there**. So it holds only
/// while every writer of that table records a path a human chose, into a
/// file a human reviews:
///
/// - A **hand-edit** is the origin for `[constitutions.*]` today.
/// - A command that *transcribes an operator's answer* does not break it.
///   `/{project}:link` writes `[services]` this way — it prompts for the
///   local checkout path and records what was typed verbatim — so the
///   framework performs the write while the operator remains the source.
/// - A writer that **originates** the value breaks it: a path derived from
///   a network response, an archive's contents, an LLM's output, or any
///   other input the operator never reviewed. Adding one moves that
///   table's paths into the validated tier, and the reading primitive
///   starts calling this function on them — in the same change as the
///   writer, never after it.
///
/// **Nothing enforces this.** No test fails when such a writer appears and
/// no check detects one; a value written by the wrong kind of writer is
/// indistinguishable, at read time, from a hand-edited one. This is a rule
/// for whoever adds the writer, and saying otherwise would imply
/// enforcement it does not have
/// (§grounding in `framework/constitution.md`).
pub(crate) fn validate_no_traversal(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(PrimitiveError::InvalidPath {
            path: path.into(),
            reason: "path is empty".into(),
        });
    }
    let p = Path::new(path);
    // `has_root()` in addition to `is_absolute()`: on Windows a `/`- or
    // `\`-rooted path without a drive letter is not "absolute", yet it
    // still escapes the repo (it resolves against the drive root). The
    // prefix check additionally rejects drive-relative forms (`C:foo`)
    // and UNC prefixes, which carry no root but name another location.
    let has_prefix = p
        .components()
        .next()
        .is_some_and(|c| matches!(c, std::path::Component::Prefix(_)));
    if p.is_absolute() || p.has_root() || has_prefix {
        return Err(PrimitiveError::InvalidPath {
            path: path.into(),
            reason: "absolute path not permitted".into(),
        });
    }
    for component in p.components() {
        if matches!(component, std::path::Component::ParentDir) {
            return Err(PrimitiveError::InvalidPath {
                path: path.into(),
                reason: "parent-directory component ('..') not permitted".into(),
            });
        }
    }
    Ok(())
}

/// Refuse `rel` when it leaves `base` through a symlink — the canonicalizing
/// half of BE-INPUT-004 that [`validate_no_traversal`] cannot do on text
/// alone (spec 022, scenario `a-path-argument-resolves-inside-its-base`).
///
/// `rel` is screened lexically first. Then each component of `base/rel` that
/// exists and is a symlink is resolved, and the path is refused when that
/// resolution does not lie inside the canonicalized `base`, compared by whole
/// components (`Path::starts_with`), never by string prefix. A link that stays
/// inside is followed and accepted. A component past the first one that does
/// not exist cannot be a link, so a path not created yet is judged by its
/// deepest existing ancestor; a dangling link is judged by where its target
/// would resolve ([`resolve_symlinks`]).
///
/// Every non-link component between two checked links lies inside `base` by
/// induction — its canonical form is its parent's plus a name that is not
/// `..` — so checking the links alone decides where the whole path resolves.
///
/// The check compares canonical forms only; the caller keeps using, and
/// naming, the path it was given. It runs once, so a link created between the
/// check and the open is not caught.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] for a path the lexical screen refuses, for
/// a link that resolves outside `base`, and for one that cannot be resolved
/// (a loop); [`PrimitiveError::Io`] when `base` itself cannot be resolved.
pub(crate) fn validate_contained(base: &Path, rel: &str) -> Result<()> {
    validate_no_traversal(rel)?;
    check_links_on(base, &canonical_base(base)?, rel)
}

/// `base` resolved through [`resolve_symlinks`], or the I/O error that
/// stopped it.
fn canonical_base(base: &Path) -> Result<PathBuf> {
    resolve_symlinks(base).map_err(|source| PrimitiveError::Io {
        path: base.into(),
        source,
    })
}

/// The link check [`validate_contained`] runs on an already screened `rel`.
fn check_links_on(base: &Path, canonical_base: &Path, rel: &str) -> Result<()> {
    let mut probe = base.to_path_buf();
    for component in Path::new(rel).components() {
        probe.push(component);
        match std::fs::symlink_metadata(&probe) {
            Ok(meta) if meta.file_type().is_symlink() => {
                refuse_outward_link(&probe, base, canonical_base, rel)?;
            }
            Ok(_) => {}
            // Absent, or unreachable: nothing beyond it can be followed, by
            // this check or by the open that comes after it.
            Err(_) => break,
        }
    }
    Ok(())
}

/// [`validate_contained`] for a feature argument, a name under the spec root
/// `root`, without the walk of [`validate_feature_argument`]: what
/// `check-artifacts` runs, since it skips a linked sibling as
/// `target-unparseable` and keeps running (spec 067, scenario
/// `sibling-symlink-trust-boundary`), which the walk would pre-empt by
/// refusing the whole call. The name is screened on its own first: joined to
/// the root, an absolute name would read as a relative one (`specs//etc` is
/// `specs/etc`).
pub(crate) fn validate_feature_contained(repo: &Path, root: &str, feature: &str) -> Result<()> {
    validate_no_traversal(feature)?;
    validate_contained(repo, &format!("{root}/{feature}"))
}

/// [`validate_feature_contained`] plus a walk of the feature directory
/// ([`validate_feature_path`]). `root` is the spec root the caller loaded.
pub(crate) fn validate_feature_argument(repo: &Path, root: &str, feature: &str) -> Result<()> {
    validate_no_traversal(feature)?;
    validate_feature_path(repo, &format!("{root}/{feature}"))
}

/// [`validate_contained`] for a project-relative feature directory, plus a
/// walk of everything under it: the call is refused when any symlink in the
/// directory resolves outside the project. That covers the files a primitive
/// then opens beneath the directory under fixed names (`spec.md`, `tasks.md`,
/// the `scenarios/` listing) without checking each open. A linked directory
/// that stays inside the project is walked too, each directory once.
///
/// A directory the walk cannot list is left to the primitive, which reports
/// it in its own terms (`check-artifact-size`'s `artifact-unreadable`, for
/// one), so a link inside a directory that is traversable but not readable
/// is not seen: the walk can check only what it can list.
///
/// # Errors
///
/// As [`validate_contained`].
pub(crate) fn validate_feature_path(repo: &Path, rel: &str) -> Result<()> {
    validate_no_traversal(rel)?;
    let canonical_base = canonical_base(repo)?;
    check_links_on(repo, &canonical_base, rel)?;
    walk_for_outward_links(repo, &canonical_base, repo.join(rel), rel)
}

/// A project-relative path argument a primitive opens, checked by
/// [`validate_contained`] against the project root and then joined to it.
/// The readers whose arguments once resolved through [`resolve_path`] take
/// their paths this way (spec 022 task 9, the owner's answer (a)), so an
/// absolute path is refused as it is at every other site that opens a path.
///
/// # Errors
///
/// As [`validate_contained`].
pub(crate) fn contained_path(repo: &Path, rel: &str) -> Result<PathBuf> {
    validate_contained(repo, rel)?;
    Ok(repo.join(rel))
}

/// [`validate_contained`] for a path or glob a tool expands itself, as
/// `lint-markdown` hands its paths to markdownlint-cli2 0.23.2, which
/// rewrites an argument before globby 16 and fast-glob 3 expand it (spec 022
/// task 9). The check judges the forms it can model and refuses the rest:
///
/// - Refused: a leading `:`, markdownlint-cli2's literal-path prefix, which
///   it resolves against its working directory; a leading `!` or `#`, its
///   negations; anywhere a `{`, `}`, `(`, `)` or `|`, micromatch's brace
///   expansion, extglobs and regex groups, any of which can spell a `..` or
///   an absolute alternative the lexical screen never sees; and a `\`,
///   which markdownlint-cli2 turns into `/` except before one of
///   `$()*+?[]^`, where fast-glob reads it as an escape on every platform,
///   while Rust on Windows reads it as a separator.
/// - A pattern with a `*`, `?` or `[` is checked as a path, and the directory
///   before its first such component is walked as a feature directory is
///   ([`validate_feature_path`]), the project root when the pattern starts
///   with one: fast-glob follows a linked directory it meets
///   (`followSymbolicLinks` is on unless set, and markdownlint-cli2 does not
///   set it).
/// - A path naming an existing directory, `.` among them, is walked the same
///   way: globby expands a directory to everything under it
///   (`expandDirectories` is on unless set), and markdownlint-cli2 reads `.`,
///   when it is the only path argument, as the Markdown files in the
///   directory.
/// - Any other path is checked as a path.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] for a refused form, naming it, and as
/// [`validate_contained`].
pub(crate) fn validate_glob_contained(repo: &Path, pattern: &str) -> Result<()> {
    let refuse = |what: String| PrimitiveError::InvalidPath {
        path: pattern.into(),
        reason: format!(
            "{what}, which this check does not model; name the files with a plain path \
             or a `*`, `?` or `[` glob"
        ),
    };
    match pattern.chars().next() {
        Some(':') => {
            return Err(refuse(
                "begins with `:`, markdownlint-cli2's literal-path prefix".into(),
            ));
        }
        Some(lead @ ('!' | '#')) => {
            return Err(refuse(format!(
                "begins with `{lead}`, a markdownlint-cli2 negation"
            )));
        }
        _ => {}
    }
    if let Some(syntax) = pattern.chars().find(|c| "{}()|".contains(*c)) {
        return Err(refuse(format!(
            "contains `{syntax}`, glob syntax that can spell a path the check does not see"
        )));
    }
    if pattern.contains('\\') {
        return Err(refuse(
            "contains `\\`, which markdownlint-cli2 reads as `/` or as an escape".into(),
        ));
    }
    validate_contained(repo, pattern)?;
    let mut prefix = PathBuf::new();
    for component in Path::new(pattern).components() {
        if component
            .as_os_str()
            .to_string_lossy()
            .contains(['*', '?', '['])
        {
            let canonical_base = canonical_base(repo)?;
            return walk_for_outward_links(repo, &canonical_base, repo.join(prefix), pattern);
        }
        prefix.push(component);
    }
    let named = repo.join(pattern);
    if named.is_dir() {
        let canonical_base = canonical_base(repo)?;
        return walk_for_outward_links(repo, &canonical_base, named, pattern);
    }
    Ok(())
}

/// The containment check for a staging path, which lies in the system temp
/// directory rather than in the project: `fetch-archive`'s destination and
/// `extract-archive`'s archive and destination, which `/ductus` passes under
/// the `{tempdir}` it makes with `mktemp -d -t` (spec 022 task 9, the owner's
/// answer (a)). The path must be absolute and lie strictly inside one of
/// [`staging_roots`], spelled as that root is or in its canonical form
/// (macOS's `/var` is `/private/var`); the rest of it goes through
/// [`validate_contained`] against that root, so a link on it that leaves the
/// root is refused. The root itself is refused: on Linux it is the shared
/// `/tmp`, and `/ductus` always passes a directory it made inside it.
/// Returns the path as given.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`], naming the argument, for a relative
/// path, a `..` component, a path outside every staging root or naming one,
/// or a link on it that resolves outside its root or cannot be resolved. A
/// root that cannot itself be resolved is skipped.
pub(crate) fn validate_in_temp_dir(path_arg: &str) -> Result<PathBuf> {
    validate_in_staging(staging_roots(), path_arg)
}

/// The text of the file a CLI `--{field}-json` flag names (048 scenario
/// `state-b-continues-in-session`): `apply-manifest`'s and
/// `enforce-manifest`'s array and map arguments, which `/ductus` writes into
/// its `{tempdir}` on the CLI path. The path is held to a staging directory,
/// as `extract-archive`'s archive is ([`validate_in_temp_dir`], `BE-INPUT-004`),
/// before it is opened.
///
/// # Errors
///
/// A message naming `--{flag}`: the path is not inside a staging directory,
/// or the file cannot be read. An unreadable file is never an empty default.
pub fn read_cli_json_file(flag: &str, path: &str) -> std::result::Result<String, String> {
    let shown = flatten_line(path);
    let resolved = validate_in_temp_dir(path).map_err(|err| format!("--{flag}: {err}"))?;
    std::fs::read_to_string(&resolved)
        .map_err(|err| format!("--{flag}: cannot read `{shown}`: {err}"))
}

/// [`validate_in_temp_dir`] plus the bound that the staging path lie *outside*
/// the working directory `repo`: a write destination for the archive
/// primitives (spec 022 task 17 question 2, the owner's answer (a)).
///
/// `fetch-archive`'s output and `extract-archive`'s destination are bounded to
/// a staging root alone, ignoring the project. When the project itself lies
/// inside a staging root — a checkout under `/tmp` on Linux, a `TMPDIR`
/// override, or Windows with `TMP` and `TEMP` both unset — that bound is met
/// by the project's own files, so the primitive would write them (a granting
/// `.claude/settings.local.json`, say) with no project bound and no host
/// prompt. This refuses a destination that resolves inside the working
/// directory, closing that write.
///
/// `/ductus` is untouched in every normal temp configuration: it extracts
/// into `{tempdir}` (a fresh `mktemp -d -t` directory) and writes every
/// download there, and under an unset or `/tmp`/per-user `TMPDIR` that
/// directory is a sibling of the project, outside it, so it passes. The one
/// configuration this refuses a real `/ductus` call is a `TMPDIR` pointing
/// *inside* the working tree (a workspace-local temp): `{tempdir}` then lands
/// inside the project and extraction is refused. That cannot be carved out
/// without reopening the gap — in that layout `/ductus`'s fresh `{tempdir}`
/// and a malicious dest are both inside the working directory with nothing to
/// distinguish them, so any bound that admits one admits the other. The
/// exclusion is by the working directory, not the outermost git work tree —
/// the opposite of [`resolve_root`]'s direction and for the opposite reason:
/// excluding the working directory refuses *less*, and `{tempdir}` is outside
/// the project under any normal temp configuration, whereas excluding the
/// outermost work tree would refuse `{tempdir}` when the temp directory sits
/// under a home work tree.
///
/// # Errors
///
/// As [`validate_in_temp_dir`]; additionally [`PrimitiveError::InvalidPath`]
/// when the staging path resolves inside the working directory, and
/// [`PrimitiveError::Io`] when `repo` cannot be resolved.
pub(crate) fn validate_staging_outside(repo: &Path, path_arg: &str) -> Result<PathBuf> {
    let resolved = validate_in_temp_dir(path_arg)?;
    let canonical_repo = canonical_base(repo)?;
    let canonical = resolve_symlinks(&resolved).map_err(|source| PrimitiveError::Io {
        path: resolved.clone(),
        source,
    })?;
    if canonical.starts_with(&canonical_repo) {
        return Err(PrimitiveError::InvalidPath {
            path: path_arg.into(),
            reason: "resolves inside the working directory; an archive staging path must lie \
                     outside the project"
                .into(),
        });
    }
    Ok(resolved)
}

/// The directories a staging path may lie in, read once per process:
/// [`staging_roots_for`] over [`std::env::temp_dir`] and, on macOS, the
/// per-user temp directory `getconf DARWIN_USER_TEMP_DIR` names.
fn staging_roots() -> &'static [PathBuf] {
    static ROOTS: std::sync::OnceLock<Vec<PathBuf>> = std::sync::OnceLock::new();
    ROOTS.get_or_init(|| {
        #[cfg(target_os = "macos")]
        let darwin = darwin_user_temp_dir();
        #[cfg(not(target_os = "macos"))]
        let darwin = None;
        staging_roots_for(std::env::temp_dir(), darwin)
    })
}

/// The staging roots for a given `temp_dir` and macOS per-user directory, in
/// order, each once, absolute ones only. `temp_dir` follows `TMPDIR` but is
/// an empty path for a `TMPDIR` set but empty, where GNU `mktemp -t` uses
/// `/tmp` (it reads `TMPDIR` only when non-empty), so off macOS an empty
/// `temp_dir` stands for `/tmp` on Unix. A relative one contributes no root:
/// `mktemp` would print a relative path, which the check refuses anyway. On
/// macOS `mktemp -d -t` uses `TMPDIR` only as a fallback: its manual says
/// the `-t` template is "based on the prefix and the
/// `_CS_DARWIN_USER_TEMP_DIR` configuration variable if available", so the
/// per-user directory is a root beside `temp_dir`.
fn staging_roots_for(temp_dir: PathBuf, darwin: Option<PathBuf>) -> Vec<PathBuf> {
    let temp_dir = if cfg!(all(unix, not(target_os = "macos"))) && temp_dir.as_os_str().is_empty() {
        PathBuf::from("/tmp")
    } else {
        temp_dir
    };
    let mut roots: Vec<PathBuf> = Vec::new();
    for root in std::iter::once(temp_dir).chain(darwin) {
        if root.is_absolute() && !roots.contains(&root) {
            roots.push(root);
        }
    }
    roots
}

/// How long `getconf DARWIN_USER_TEMP_DIR` may run before it is stopped
/// (`BE-TIMEOUT-001`). A `getconf` that has not answered by then is treated
/// as one that could not be read: the per-user directory is a staging root
/// only when [`std::env::temp_dir`] names it too, so the archive primitives
/// refuse a staging path that only it would admit rather than wait on it.
#[cfg(target_os = "macos")]
const GETCONF_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// The per-user temp directory macOS keeps whatever `TMPDIR` says, from
/// `getconf DARWIN_USER_TEMP_DIR`, or `None` when that cannot be read within
/// [`GETCONF_TIMEOUT`].
#[cfg(target_os = "macos")]
fn darwin_user_temp_dir() -> Option<PathBuf> {
    let mut getconf = std::process::Command::new("/usr/bin/getconf");
    getconf.arg("DARWIN_USER_TEMP_DIR");
    let output = output_within(&mut getconf, GETCONF_TIMEOUT).ok()??;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let dir = text.trim();
    (!dir.is_empty()).then(|| PathBuf::from(dir))
}

/// [`validate_in_temp_dir`] against named staging roots, the seam a test uses
/// to hold the check to directories it controls.
fn validate_in_staging(roots: &[PathBuf], path_arg: &str) -> Result<PathBuf> {
    let refuse = |reason: String| PrimitiveError::InvalidPath {
        path: path_arg.into(),
        reason,
    };
    let shown = roots
        .iter()
        .map(|root| root.display().to_string())
        .collect::<Vec<_>>()
        .join(" or ");
    let candidate = Path::new(path_arg);
    if !candidate.is_absolute() {
        return Err(refuse(format!(
            "must be an absolute path inside the system temp directory ({shown})"
        )));
    }
    if candidate
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(refuse(
            "parent-directory component ('..') not permitted".into(),
        ));
    }
    // A root that cannot be resolved is skipped: a path under it is then
    // judged by the others, and refused when none holds it.
    let resolved: Vec<(&PathBuf, PathBuf)> = roots
        .iter()
        .filter_map(|root| canonical_base(root).ok().map(|canonical| (root, canonical)))
        .collect();
    // Matched by text before the candidate is touched: on Windows resolving a
    // path opens it, and a candidate outside every root (a UNC path) must be
    // refused without that.
    let Some((base, rest)) = resolved.iter().find_map(|(root, canonical)| {
        candidate
            .strip_prefix(root)
            .map(|rest| (root.as_path(), rest))
            .or_else(|_| {
                candidate
                    .strip_prefix(canonical)
                    .map(|rest| (canonical.as_path(), rest))
            })
            .ok()
    }) else {
        return Err(refuse(format!(
            "is not inside the system temp directory ({shown})"
        )));
    };
    // The root itself is refused: a candidate that spells the root it
    // matched, or whose canonical form is any root's, a root inside another
    // (`TMPDIR` an ancestor of macOS's per-user directory) included. A
    // candidate that does not exist yet has no canonical form, so one naming
    // a root not created yet is refused only when that root is the one it
    // matched; a path accepted that way is still contained by its root.
    let names_a_root = rest.as_os_str().is_empty()
        || candidate
            .canonicalize()
            .is_ok_and(|real| resolved.iter().any(|(_, canonical)| *canonical == real));
    if names_a_root {
        return Err(refuse(format!(
            "names the system temp directory itself ({shown}); pass a directory made inside it"
        )));
    }
    validate_contained(base, &rest.to_string_lossy()).map_err(|err| match err {
        PrimitiveError::InvalidPath { reason, .. } => refuse(reason),
        other => other,
    })?;
    Ok(candidate.to_path_buf())
}

/// Walk `start` and everything under it, refusing the first symlink that
/// resolves outside `canonical_base`; `rel` is the argument named in the
/// error. A linked directory that stays inside is walked too, each directory
/// once, and a directory the walk cannot list is left to the caller.
fn walk_for_outward_links(
    repo: &Path,
    canonical_base: &Path,
    start: PathBuf,
    rel: &str,
) -> Result<()> {
    let mut visited = std::collections::BTreeSet::new();
    let mut pending = vec![start];
    while let Some(dir) = pending.pop() {
        let Ok(canonical) = dir.canonicalize() else {
            continue;
        };
        if !visited.insert(canonical) {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            // `DirEntry::file_type` does not follow a link.
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            let entry_path = entry.path();
            if kind.is_symlink() {
                refuse_outward_link(&entry_path, repo, canonical_base, rel)?;
                if entry_path.is_dir() {
                    pending.push(entry_path);
                }
            } else if kind.is_dir() {
                pending.push(entry_path);
            }
        }
    }
    Ok(())
}

/// The `base`-relative, `/`-separated path `rel` reaches once its symlinks
/// are resolved, or `None` when it reaches outside `base` or a link on the
/// way cannot be resolved. The `writeCode` boundary matches this against its
/// patterns as well as the path as given
/// ([`crate::schema::extensions::validate_write_code_resolution`]).
pub(crate) fn resolved_relative(base: &Path, rel: &str) -> Option<String> {
    let canonical_base = resolve_symlinks(base).ok()?;
    let resolved = resolve_symlinks(&base.join(rel)).ok()?;
    let inside = resolved.strip_prefix(&canonical_base).ok()?;
    let segments: Vec<String> = inside
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect();
    Some(segments.join("/"))
}

/// Refuse the symlink at `link` when it resolves outside `canonical_base`.
/// `rel` is the argument being checked, named in the error.
fn refuse_outward_link(link: &Path, base: &Path, canonical_base: &Path, rel: &str) -> Result<()> {
    let shown = link
        .strip_prefix(base)
        .unwrap_or(link)
        .display()
        .to_string();
    let resolved = resolve_symlinks(link).map_err(|err| PrimitiveError::InvalidPath {
        path: rel.into(),
        reason: format!("the symlink `{shown}` cannot be resolved: {err}"),
    })?;
    if resolved.starts_with(canonical_base) {
        return Ok(());
    }
    Err(PrimitiveError::InvalidPath {
        path: rel.into(),
        reason: format!(
            "the symlink `{shown}` resolves to {}, outside {}",
            resolved.display(),
            canonical_base.display()
        ),
    })
}

/// The canonical form of `path`, for a path that does not exist as well as
/// one that does, judged as the kernel would once every missing directory on
/// it exists. An existing path is canonicalized. Otherwise a dangling link is
/// judged by where its target would resolve, and a missing path by its
/// parent's resolution with the last component appended — `..` taking the
/// parent of that resolution, and an appended name that turns out to exist
/// (reached through a `..` after a missing directory) resolved in turn, since
/// it may be a link. The resolver follows at most [`MAX_SYMLINK_HOPS`] links
/// itself, from one budget for the whole resolution as the kernel keeps one
/// per lookup: a count kept per branch would let a chain of links that each
/// name the next twice cost time exponential in its length. Each
/// `canonicalize` it calls follows links of its own, up to the platform's
/// limit, and those are not counted here.
fn resolve_symlinks(path: &Path) -> std::io::Result<PathBuf> {
    let mut hops = 0;
    resolve_symlinks_within(path, &mut hops)
}

/// Linux's own limit on the links one lookup follows (`MAXSYMLINKS`).
const MAX_SYMLINK_HOPS: u32 = 40;

fn resolve_symlinks_within(path: &Path, hops: &mut u32) -> std::io::Result<PathBuf> {
    match path.canonicalize() {
        Ok(canonical) => return Ok(canonical),
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => return Err(err),
        Err(_) => {}
    }
    if std::fs::symlink_metadata(path).is_ok_and(|meta| meta.file_type().is_symlink()) {
        if *hops >= MAX_SYMLINK_HOPS {
            return Err(std::io::Error::other("too many levels of symbolic links"));
        }
        *hops += 1;
        let target = std::fs::read_link(path)?;
        // `join` replaces the parent when the target is absolute.
        let next = path
            .parent()
            .map_or_else(|| target.clone(), |parent| parent.join(&target));
        return resolve_symlinks_within(&next, hops);
    }
    let mut components = path.components();
    match components.next_back() {
        Some(std::path::Component::Normal(name)) => {
            let joined = resolve_symlinks_within(components.as_path(), hops)?.join(name);
            if joined != path && std::fs::symlink_metadata(&joined).is_ok() {
                return resolve_symlinks_within(&joined, hops);
            }
            Ok(joined)
        }
        Some(std::path::Component::ParentDir) => {
            let resolved = resolve_symlinks_within(components.as_path(), hops)?;
            Ok(resolved
                .parent()
                .map_or_else(|| resolved.clone(), Path::to_path_buf))
        }
        Some(std::path::Component::CurDir) => resolve_symlinks_within(components.as_path(), hops),
        // A root, a drive prefix, or nothing at all: there is no parent to
        // resolve it through.
        _ => Ok(path.to_path_buf()),
    }
}

/// Reject a text value carrying an embedded newline or carriage return, or
/// any other character that is not plain single-line text: a control
/// character other than tab, one of the Unicode line breaks YAML also treats
/// as a break, or a noncharacter the YAML reader refuses (see
/// [`is_line_hazard`]).
///
/// Such a value, interpolated verbatim into a markdown or YAML artifact,
/// would inject document structure (a phantom heading, a new frontmatter
/// key) or leave a file the YAML reader refuses outright — it rejects raw
/// control characters and noncharacters, so one stored decision would make a
/// whole record unreadable (spec 058). `primitive`/`argument` name the offending field.
/// Shared by every primitive that splices caller-supplied text into a file it
/// writes.
pub(crate) fn validate_single_line(primitive: &str, argument: &str, value: &str) -> Result<()> {
    let reject = |reason: &str| PrimitiveError::InvalidArgument {
        primitive: primitive.into(),
        argument: argument.into(),
        reason: reason.into(),
    };
    if value.contains('\n') || value.contains('\r') {
        return Err(reject(
            "embedded newlines would inject document structure; supply single-line text",
        ));
    }
    if value.chars().any(is_line_hazard) {
        return Err(reject(
            "control characters, Unicode line breaks, and noncharacters would inject document \
             structure or leave a record the YAML reader refuses; supply plain single-line text",
        ));
    }
    Ok(())
}

/// Reject inbox bullet text that is empty, whitespace-only, or spans lines:
/// the one rule `append-inbox` and `remove-inbox-item` both apply, so any
/// bullet the one writes, the other can remove.
///
/// Deliberately narrower than [`validate_single_line`], in the direction of
/// the inbox rather than the YAML records. `inbox.md` is markdown that no YAML
/// reader parses, so the control characters and noncharacters that rule
/// refuses leave nothing unreadable here, and markdown ends a line only at
/// `\n` or `\r`, so U+2028 and the other Unicode breaks inject no structure.
/// `remove-inbox-item` writes no caller text at all; it only matches it.
/// Holding removal to the YAML rule refused text `append-inbox` had written,
/// which left that bullet removable by hand only, and tightening
/// `append-inbox` instead would refuse a todo for no protection.
pub(crate) fn validate_inbox_text(primitive: &str, argument: &str, text: &str) -> Result<()> {
    let reject = |reason: String| PrimitiveError::InvalidArgument {
        primitive: primitive.into(),
        argument: argument.into(),
        reason,
    };
    if text.trim().is_empty() {
        return Err(reject(format!("{argument} is empty")));
    }
    if text.contains('\n') || text.contains('\r') {
        return Err(reject(
            "an inbox bullet is one line, and embedded newlines would inject markdown \
             structure into inbox.md; supply single-line text"
                .into(),
        ));
    }
    Ok(())
}

/// Where the directory `dir` sits inside `repository`'s work tree: its path
/// from the work tree, empty when `dir` is the work tree itself, or `None`
/// when the repository has no work tree, `dir` cannot be resolved, or it lies
/// outside the work tree.
///
/// Git history, the index, and a diff name every path from the work tree, and
/// a project need not sit at its repository's root. A project in a
/// subdirectory names its files from the project root, so it prefixes this
/// before asking history about them.
pub(crate) fn workdir_prefix(repository: &git2::Repository, dir: &Path) -> Option<PathBuf> {
    let workdir = repository.workdir()?.canonicalize().ok()?;
    Some(
        dir.canonicalize()
            .ok()?
            .strip_prefix(&workdir)
            .ok()?
            .to_path_buf(),
    )
}

/// A project's git repository, with the project's place in its work tree.
///
/// Every reader that asks git about a project's files opens the repository
/// through this (spec 059), save `inbox_standing`, which places the inbox
/// file's own directory rather than the project root. Git names each path in
/// its trees, index, status, and diffs from the work tree, and a project
/// names its files from the project root. The two agree only when the project
/// is the work tree's root, so a reader converts with [`Self::to_git`] on the
/// way in and [`Self::to_project`] on the way out.
pub(crate) struct ProjectRepository {
    pub(crate) repository: git2::Repository,
    /// The project root's path from the work tree, `/`-joined: empty at the
    /// work tree's root, else ending in `/`.
    prefix: String,
}

impl ProjectRepository {
    /// Discover the repository containing `project`, searching upward.
    ///
    /// # Errors
    ///
    /// No repository contains `project`; [`workdir_prefix`] cannot place it
    /// inside the repository's work tree — a bare repository, or one whose
    /// `core.worktree` moved the work tree elsewhere (`GIT_WORK_TREE` is not
    /// consulted: libgit2 reads it only when a repository is opened from the
    /// environment, which discovery does not do). Each is the no-repository
    /// case. Reading history from the work tree's root instead would find none
    /// and answer as for a spec never committed, which a caller cannot tell
    /// from a real answer.
    pub(crate) fn discover(project: &Path) -> std::result::Result<Self, git2::Error> {
        let repository = git2::Repository::discover(project)?;
        let Some(place) = workdir_prefix(&repository, project) else {
            return Err(git2::Error::from_str(
                "the project directory is not inside its repository's work tree",
            ));
        };
        // Joined by component rather than rendered as a path, so the prefix
        // uses git's `/` on every platform.
        let mut prefix = String::new();
        for part in &place {
            prefix.push_str(&part.to_string_lossy());
            prefix.push('/');
        }
        Ok(Self { repository, prefix })
    }

    /// `rel`, a `/`-separated path from the project root, as git names it.
    pub(crate) fn to_git(&self, rel: &str) -> String {
        format!("{}{rel}", self.prefix)
    }

    /// `git_path`, as git names it, from the project root, or `None` when it
    /// lies outside the project: another project's work, never this one's.
    pub(crate) fn to_project<'a>(&self, git_path: &'a str) -> Option<&'a str> {
        git_path.strip_prefix(self.prefix.as_str())
    }

    /// The text of `rel`, a `/`-separated path from the project root, as
    /// HEAD's tree holds it: `None` when that tree has no entry there or its
    /// blob is not UTF-8. A directory at `rel` is not a blob, and errors.
    ///
    /// # Errors
    ///
    /// [`PrimitiveError::Git`] when HEAD, its tree, or the blob cannot be
    /// read — an unborn HEAD among them.
    pub(crate) fn read_at_head(&self, rel: &str) -> Result<Option<String>> {
        let head = self.repository.head()?.peel_to_commit()?;
        blob_text(&self.repository, &head.tree()?, &self.to_git(rel))
    }
}

/// The UTF-8 text of the blob at `git_path` in `tree`, or `None` when the tree
/// has no entry there or the blob is not UTF-8. `git_path` is named from the
/// work tree, as git names every path in a tree.
///
/// # Errors
///
/// [`PrimitiveError::Git`] when the lookup fails for any reason other than an
/// absent entry, or the entry's blob cannot be read — so a tree that could
/// not be searched never reads as one without the file.
pub(crate) fn blob_text(
    repository: &git2::Repository,
    tree: &git2::Tree<'_>,
    git_path: &str,
) -> Result<Option<String>> {
    let entry = match tree.get_path(Path::new(git_path)) {
        Ok(entry) => entry,
        Err(err) if err.code() == git2::ErrorCode::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    };
    let blob = repository.find_blob(entry.id())?;
    Ok(std::str::from_utf8(blob.content()).ok().map(str::to_string))
}

/// A repository whose project sits in a subdirectory, for the tests of every
/// reader that converts through [`ProjectRepository`].
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
pub(crate) mod git_fixture {
    use std::path::{Path, PathBuf};

    /// Initialize a repository at `root` and a project at `root/proj`, with
    /// its own `.ductus/config.toml`. Returns the repository and the project
    /// root.
    pub(crate) fn subdirectory_project(root: &Path) -> (git2::Repository, PathBuf) {
        let repository = git2::Repository::init(root).unwrap();
        let project = root.join("proj");
        write(
            &project.join(".ductus/config.toml"),
            "[paths]\nspecs-root = \"specs\"\n",
        );
        (repository, project)
    }

    /// Write `body` to `path`, creating its parent directories.
    pub(crate) fn write(path: &Path, body: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, body).unwrap();
    }

    /// Stage every change in the work tree and commit it.
    pub(crate) fn commit_all(repository: &git2::Repository, message: &str) -> git2::Oid {
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let signature = git2::Signature::now("Test", "test@example.com").unwrap();
        let parent = repository
            .head()
            .ok()
            .and_then(|head| head.target())
            .and_then(|oid| repository.find_commit(oid).ok());
        let parents: Vec<&git2::Commit> = parent.as_ref().into_iter().collect();
        repository
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &parents,
            )
            .unwrap()
    }
}

/// A character that is not plain single-line text: a control character other
/// than tab (C0, DEL, C1 — `U+0085` among them), `U+2028` / `U+2029`, which
/// YAML treats as line breaks, or the noncharacters `U+FFFE` / `U+FFFF`. The
/// YAML reader refuses raw control characters and both noncharacters (they sit
/// outside its printable set), and a line break inside a plain scalar ends it.
pub(crate) fn is_line_hazard(c: char) -> bool {
    (c.is_control() && c != '\t') || matches!(c, '\u{2028}' | '\u{2029}' | '\u{FFFE}' | '\u{FFFF}')
}

/// `value` as one line of plain text: every [`is_line_hazard`] character
/// becomes a space, and the result is trimmed. The one normalization a stored
/// decision key and its companion text pass through, so the key a writer
/// stores and the key a host fires for the same finding agree (spec 058).
pub(crate) fn flatten_line(value: &str) -> String {
    value
        .chars()
        .map(|c| if is_line_hazard(c) { ' ' } else { c })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Extract an inbox/list bullet line's text: the trimmed content after the
/// `- ` marker, with an optional task-list checkbox (`[ ]`/`[x]`/`[X]`)
/// stripped via the shared [`checkbox::parse_checkbox_line`] grammar so the
/// plain `- text` and the checkbox `- [ ] text` forms both resolve to their
/// content. `None` for a non-bullet line. The one definition of bullet
/// identity: [`iter_bullets`] reads bullets through it, so the inbox count,
/// `remove-inbox-item`'s removal match, and the dashboard's inbox row agree on
/// what an item's text is.
pub(crate) fn bullet_text(line: &str) -> Option<String> {
    if let Some((_checked, text)) = checkbox::parse_checkbox_line(line) {
        return Some(text);
    }
    let rest = line.trim_start().strip_prefix("- ")?;
    Some(rest.trim().to_string())
}

/// Strip one leading list marker from caller-supplied bullet *content*.
///
/// Every `append-*` primitive renders its own marker (`- ` or `- [ ] `), so
/// a caller that includes one produces a doubled prefix (`- [ ] - [ ] text`).
/// Nothing catches that on the way in: the caller cannot see the rendering,
/// a doubled marker is valid markdown so `lint-markdown` passes, and the
/// write is atomic — it surfaces only when a human reads the file.
///
/// Delegates to [`bullet_text`] rather than matching separately, so the
/// write side and the read side — [`iter_bullets`], which `remove-inbox-item`
/// matches against — can never disagree about which inputs carry a marker. Strips **one** marker: the failure mode is a
/// single doubling, and stripping to exhaustion would eat legitimate content
/// from text that genuinely begins with a dash.
pub(crate) fn strip_bullet_marker(text: &str) -> String {
    bullet_text(text).unwrap_or_else(|| text.trim().to_string())
}

/// Iterate the real inbox/list bullets of `content` as `(line_index, text)`,
/// skipping fenced code blocks and HTML-comment regions via [`SkipScanner`].
/// The inbox template embeds `- ` lines inside its `<!-- Rules: … -->`
/// guidance comment; without comment-awareness those would be miscounted as
/// items and could even be matched for removal. Shared by the inbox count
/// `append-inbox` reports, `remove-inbox-item`'s removal, the dashboard's
/// inbox row, and `check-promotion-coverage`'s entry walk, so all agree on
/// which lines are real bullets.
pub(crate) fn iter_bullets(content: &str) -> impl Iterator<Item = (usize, String)> + '_ {
    let mut skip = SkipScanner::default();
    content.lines().enumerate().filter_map(move |(idx, line)| {
        if skip.skip(line) {
            return None;
        }
        bullet_text(line).map(|text| (idx, text))
    })
}

/// Collapse every run of whitespace in `text` to a single space, trimming the
/// ends — the normalization a single-line report needs before rendering
/// operator-authored free text.
///
/// The hazard is in the data rather than the display: TOML multi-line strings
/// and YAML block scalars both make an embedded newline reachable, and one
/// newline turns a single-line notice into two. Shared by the
/// disabled-rule-file notice, the shared-constitution `description`, and
/// `append-question`'s comparison key, which builds its case-folding on top of
/// this rather than beside it — three copies of one `split_whitespace` chain
/// had accumulated before they were collapsed here.
pub(crate) fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Count the real (comment/fence-aware) inbox/list bullets in `content`.
pub(crate) fn count_inbox_bullets(content: &str) -> u32 {
    u32::try_from(iter_bullets(content).count()).unwrap_or(u32::MAX)
}

/// Derive a kebab-case slug from free text: every ASCII alphanumeric
/// character is lowercased and kept; every run of other characters (spaces,
/// punctuation, non-ASCII) collapses to a single hyphen; leading and trailing
/// hyphens are trimmed. The result is empty, or matches [`is_slug_grammar`].
///
/// The one sanitizing rule for operator-supplied names: `create-feature`
/// applies it to a title and to a branch identifier, and the session core
/// to a session identity (spec 062). Sharing it is what keeps a name that
/// becomes a path segment on one grammar.
pub(crate) fn derive_slug(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut pending_hyphen = false;
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_hyphen && !out.is_empty() {
                out.push('-');
            }
            pending_hyphen = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending_hyphen = true;
        }
    }
    out
}

/// Validate a caller-supplied slug against the framework slug grammar
/// `^[a-z0-9]+(?:-[a-z0-9]+)*$`: one or more lowercase-alphanumeric
/// segments joined by single hyphens — exactly the alphabet
/// [`derive_slug`] emits. This is an allowlist
/// (BE-INPUT-002): every slug reaches a written filename
/// (`scenarios/{slug}.md`) and a rendered heading, so anything outside the
/// grammar — uppercase, `_`, `.`, path separators, whitespace, newlines,
/// or other control characters — is rejected before it can inject a path
/// segment or forge markdown structure.
pub(crate) fn validate_slug(slug: &str) -> Result<()> {
    if slug.is_empty() {
        return Err(PrimitiveError::InvalidSlug {
            slug: slug.into(),
            reason: "slug is empty".into(),
        });
    }
    if !is_slug_grammar(slug) {
        return Err(PrimitiveError::InvalidSlug {
            slug: slug.into(),
            reason: "slug must match ^[a-z0-9]+(?:-[a-z0-9]+)*$ \
                     (lowercase letters, digits, single hyphens)"
                .into(),
        });
    }
    Ok(())
}

/// Whether `s` matches the framework slug grammar
/// `^[a-z0-9]+(?:-[a-z0-9]+)*$`. The predicate behind [`validate_slug`],
/// shared with [`parse_feature_dir`]'s branch-scoped identifier check so
/// one definition of the alphabet serves both — a second copy is how a
/// grammar and the names it admits drift apart.
pub(crate) fn is_slug_grammar(s: &str) -> bool {
    // Allowlist, segment by segment: each `-`-delimited segment must be
    // non-empty (rejecting a leading/trailing hyphen and a `--` run) and
    // hold only lowercase ASCII letters and digits.
    !s.is_empty()
        && s.split('-').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

/// Per-line skip state shared by the `tasks.md` / spec structural walkers.
/// Content inside a fenced code block (` ``` `) or an HTML comment
/// (`<!-- … -->`) is not markdown structure — it must not yield headings,
/// task numbers, phase containers, or checkboxes. Feed every line of a
/// document in order; [`skip`](SkipScanner::skip) reports the lines to ignore.
///
/// This exists because `tasks.md`'s own template guidance comment embeds
/// example `## N.` task headings; without comment-awareness the tasks
/// parsers mis-read a reset (template-state) file as containing phantom
/// tasks, splitting the runtime/markdown two-paths guarantee.
#[derive(Default)]
pub(crate) struct SkipScanner {
    in_fence: bool,
    in_comment: bool,
}

impl SkipScanner {
    /// Advance over `line` (document order) and report whether its content
    /// must be skipped. Fence and multi-line-comment delimiter lines are
    /// themselves skipped, matching the pre-existing fenced-block handling.
    /// A comment that opens and closes on the same line is inline — its
    /// surrounding content is real markdown and the line is not skipped.
    /// A comment delimiter inside a backtick inline-code span is inert (it
    /// neither opens nor closes a region), so prose that merely mentions
    /// `<!--` in code font is not mistaken for a comment opener.
    pub(crate) fn skip(&mut self, line: &str) -> bool {
        if self.in_fence {
            if line.trim_start().starts_with("```") {
                self.in_fence = false;
            }
            return true;
        }
        if self.in_comment {
            if line.contains("-->") {
                self.in_comment = false;
            }
            return true;
        }
        if line.trim_start().starts_with("```") {
            self.in_fence = true;
            return true;
        }
        // Comment delimiters inside a backtick inline-code span are inert —
        // prose that merely mentions `<!--` in code font must not open a skip
        // region (scenarios/skipscanner-inline-code-exemption.md). Fence
        // delimiters are already line-anchored (a fence opens only when a line
        // *starts* with the run), so a mid-prose fence mention is inert too.
        if let Some(open) = find_outside_code(line, "<!--") {
            if find_outside_code(&line[open + 4..], "-->").is_some() {
                return false;
            }
            self.in_comment = true;
            return true;
        }
        false
    }

    /// `true` while the scanner sits inside an *open* fenced code block or
    /// HTML comment — content fed now would be skipped. Lets a caller detect
    /// an unterminated region that runs to EOF (e.g. `append-inbox` placing a
    /// new bullet where the comment/fence-aware read side will count it).
    pub(crate) fn in_region(&self) -> bool {
        self.in_fence || self.in_comment
    }

    /// `true` while the scanner sits inside an open fenced code block. Lets a
    /// caller that keeps fenced content, as `prune-project` keeps a fenced
    /// payload inside its entry, tell it from an HTML comment, which
    /// [`skip`](Self::skip) reports the same way.
    pub(crate) fn in_fence(&self) -> bool {
        self.in_fence
    }
}

/// Byte ranges of `line` that sit inside a markdown inline-code span — the
/// content between a matched pair of equal-length backtick runs, per the
/// `CommonMark` code-span rule. A backtick run with no later equal-length run
/// is a literal backtick and opens no span; runs of a different length
/// between a matched pair are ordinary span content.
///
/// Shared, so the consumers that must read *outside* code font and the ones
/// that must read *inside* it agree on where the spans are rather than each
/// rolling its own scan.
pub(crate) fn inline_code_spans(line: &str) -> Vec<std::ops::Range<usize>> {
    let bytes = line.as_bytes();
    // Backtick runs as (start, len), left to right.
    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'`' {
            let start = i;
            while i < bytes.len() && bytes[i] == b'`' {
                i += 1;
            }
            runs.push((start, i - start));
        } else {
            i += 1;
        }
    }
    // Match each opening run to the next run of equal length; the content
    // between them is a code span, and the closing run cannot re-open one.
    let mut spans = Vec::new();
    let mut idx = 0;
    while idx < runs.len() {
        let (open_start, open_len) = runs[idx];
        if let Some(offset) = runs[idx + 1..].iter().position(|&(_, len)| len == open_len) {
            let close_idx = idx + 1 + offset;
            spans.push(open_start + open_len..runs[close_idx].0);
            idx = close_idx + 1;
        } else {
            idx += 1;
        }
    }
    spans
}

/// First byte offset of `needle` in `line` that does not fall inside an
/// inline-code span — the occurrence a comment/fence-aware scanner acts on.
/// A delimiter mentioned inside backticks is skipped over. `needle` is
/// ASCII (`<!--` / `-->`), so each match starts on a char boundary.
fn find_outside_code(line: &str, needle: &str) -> Option<usize> {
    // Cheap pre-check: a line with no delimiter at all (the common case)
    // needs no inline-code-span computation.
    if !line.contains(needle) {
        return None;
    }
    let spans = inline_code_spans(line);
    let mut from = 0;
    while let Some(rel) = line[from..].find(needle) {
        let pos = from + rel;
        if !spans.iter().any(|span| span.contains(&pos)) {
            return Some(pos);
        }
        from = pos + 1;
    }
    None
}

/// One block-level element of a markdown document — the list item, table
/// row, or paragraph a single authorial claim occupies.
///
/// The unit exists because a claim and the link it is about must be judged
/// together, and markdown block boundaries are structural: the repo has no
/// sentence splitter, and one that survives version strings, `e.g.`, and
/// period-bearing code spans is its own project.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MarkdownBlock {
    /// 1-based line number the block starts on.
    pub(crate) line: usize,
    /// The block's lines, joined by `\n` exactly as they appear.
    pub(crate) text: String,
}

/// Split `content` into its block-level elements, in document order.
///
/// Three kinds, decided in this order: a line whose trimmed form starts with
/// `|` is a **table row** and is a block by itself; a line opening a list
/// marker starts a **list item**; any other run of non-blank, non-heading
/// lines is a **paragraph**. An open block ends at a blank line, a heading, a
/// blockquote, or the start of another block.
///
/// Four contexts never reach a block, from two sources. Fenced code blocks
/// and HTML comments are dropped by [`SkipScanner`], which every line passes
/// through first; blockquote lines (`>`) are dropped here. The split is
/// deliberately **not** pushed into `SkipScanner`: that scanner is shared by
/// the tasks primitives, `prune-plan`'s section segmentation, and the section
/// and bullet walkers, so teaching it a fourth region would change how each
/// of them reads a quoted line. (Inline code spans are the fourth exempt
/// context, but they are an *intra-line* concern — see [`inline_code_spans`]
/// — so a consumer applies them to a block's text rather than the splitter
/// dropping lines.)
///
/// This is the default [`BlockReading`]; [`split_blocks_reading`] keeps
/// blockquotes and headings as blocks instead of dropping them.
pub(crate) fn split_blocks(content: &str) -> Vec<MarkdownBlock> {
    split_blocks_reading(content, BlockReading::default())
}

/// What a split keeps beyond paragraphs, list items and table rows.
///
/// [`split_blocks`] keeps neither: the link-adjacent family exempts quoted
/// text and has no use for headings. The body path family reads both — a
/// post-completion note is a blockquote that names paths, and a heading can
/// name one too.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BlockReading {
    /// A blockquote paragraph is a block: its `>` lines, until a blank or
    /// empty `>` line; a list item inside the quote starts a new one. A plain
    /// line after a quote resumes as a new paragraph under either reading
    /// (scenario `block-element-scanner`).
    pub(crate) quotes: bool,
    /// A heading is a block by itself rather than only a boundary.
    pub(crate) headings: bool,
}

/// [`split_blocks`] under `reading`. A blockquote block's lines keep their
/// `>` markers, as every block's lines appear as written; [`unquote`] removes
/// them.
pub(crate) fn split_blocks_reading(content: &str, reading: BlockReading) -> Vec<MarkdownBlock> {
    let mut skip = SkipScanner::default();
    let mut blocks: Vec<MarkdownBlock> = Vec::new();
    // The open block: its first line, its lines, and whether it is a quote.
    let mut open: Option<(usize, Vec<String>, bool)> = None;

    for (idx, line) in content.lines().enumerate() {
        // `skip` runs first and unconditionally so the scanner advances over
        // every line in order, including the ones dropped below.
        let skipped = skip.skip(line);
        // A comment opening and closing on one line is *not* skipped by
        // `SkipScanner` — its surrounding content is real markdown — so the
        // commented text is removed here instead. Without this, a term
        // inside a one-line comment would reach a block.
        let content_line = if skipped {
            Cow::Borrowed("")
        } else {
            strip_inline_comments(line)
        };
        let trimmed = content_line.trim_start();
        if skipped || trimmed.is_empty() {
            close_block(&mut blocks, open.take());
            continue;
        }
        if trimmed.starts_with('>') {
            let inner = unquote(trimmed);
            // An empty `>` line ends a quote paragraph, as a blank line ends
            // any other.
            if !reading.quotes || inner.is_empty() {
                close_block(&mut blocks, open.take());
                continue;
            }
            match &mut open {
                Some((_, lines, true)) if !opens_list_item(inner) => {
                    lines.push(content_line.into_owned());
                }
                _ => {
                    close_block(&mut blocks, open.take());
                    open = Some((idx + 1, vec![content_line.into_owned()], true));
                }
            }
            continue;
        }
        if parse_atx_heading(&content_line).is_some() {
            close_block(&mut blocks, open.take());
            if reading.headings {
                blocks.push(MarkdownBlock {
                    line: idx + 1,
                    text: content_line.into_owned(),
                });
            }
            continue;
        }
        if trimmed.starts_with('|') {
            close_block(&mut blocks, open.take());
            blocks.push(MarkdownBlock {
                line: idx + 1,
                text: content_line.into_owned(),
            });
            continue;
        }
        if opens_list_item(trimmed) {
            close_block(&mut blocks, open.take());
            open = Some((idx + 1, vec![content_line.into_owned()], false));
            continue;
        }
        // A non-marker line continues an open paragraph or list item — the
        // indented and lazy continuation forms are the same claim either way.
        // After a quote it resumes as a new paragraph, as it does when the
        // default reading drops the quote.
        if let Some((_, lines, false)) = &mut open {
            lines.push(content_line.into_owned());
        } else {
            close_block(&mut blocks, open.take());
            open = Some((idx + 1, vec![content_line.into_owned()], false));
        }
    }
    close_block(&mut blocks, open.take());
    blocks
}

/// A blockquote line's content: every leading `>` marker, and the spaces
/// after each, removed. Any other line comes back trimmed.
pub(crate) fn unquote(line: &str) -> &str {
    let mut rest = line.trim();
    while let Some(inner) = rest.strip_prefix('>') {
        rest = inner.trim_start();
    }
    rest
}

/// Push the currently-open block, if any, onto `blocks`.
fn close_block(blocks: &mut Vec<MarkdownBlock>, open: Option<(usize, Vec<String>, bool)>) {
    if let Some((line, lines, _)) = open {
        blocks.push(MarkdownBlock {
            line,
            text: lines.join("\n"),
        });
    }
}

/// Remove every inline HTML-comment span (one that opens *and* closes on the
/// same line) from `line`, leaving the surrounding markdown intact.
///
/// [`SkipScanner`] deliberately reports such a line as *not* skipped, because
/// its surrounding content is real — which would let the commented text reach
/// a block. Delimiters inside a code span are inert here for the same reason
/// they are inert there.
fn strip_inline_comments(line: &str) -> Cow<'_, str> {
    if !line.contains("<!--") {
        return Cow::Borrowed(line);
    }
    let mut out = String::new();
    let mut rest = line;
    let mut stripped = false;
    while let Some(open) = find_outside_code(rest, "<!--") {
        let after = &rest[open + 4..];
        let Some(close) = find_outside_code(after, "-->") else {
            break;
        };
        out.push_str(&rest[..open]);
        rest = &after[close + 3..];
        stripped = true;
    }
    if !stripped {
        return Cow::Borrowed(line);
    }
    out.push_str(rest);
    Cow::Owned(out)
}

/// `true` when `trimmed` opens a list item — a `-`, `*`, or `+` bullet, or an
/// `N.` ordered marker — followed by a space or nothing. A nested marker at
/// deeper indentation still matches, so a sub-bullet is its own block rather
/// than part of its parent's claim.
fn opens_list_item(trimmed: &str) -> bool {
    for marker in ['-', '*', '+'] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            return rest.is_empty() || rest.starts_with(' ');
        }
    }
    let digits = trimmed.bytes().take_while(u8::is_ascii_digit).count();
    digits > 0
        && trimmed.as_bytes().get(digits) == Some(&b'.')
        && matches!(trimmed.as_bytes().get(digits + 1), None | Some(b' '))
}

/// Walk `content` line by line, yielding the numeric prefix of every ATX
/// heading at any of the given `levels` whose text begins with `N.`. Skips
/// headings inside fenced code blocks and HTML comments. Used by `tasks.md`
/// primitives to
/// compute the next task number in both flat (`## N.`) and phased
/// (`### N.` under `## Phase X`) structures — passing `&[2, 3]` produces
/// the union across both shapes.
pub(crate) fn iter_task_numbers_at_levels<'a>(
    content: &'a str,
    levels: &'a [u8],
) -> impl Iterator<Item = u32> + 'a {
    let mut skip = SkipScanner::default();
    content.lines().filter_map(move |line| {
        if skip.skip(line) {
            return None;
        }
        let (level, text) = parse_atx_heading(line)?;
        if !levels.contains(&level) {
            return None;
        }
        let dot = text.find('.')?;
        let num_part = &text[..dot];
        if num_part.is_empty() {
            return None;
        }
        num_part.parse::<u32>().ok()
    })
}

/// Phased vs flat structure of a `tasks.md` file.
///
/// A file is **phased** when it contains at least one `### N.` heading
/// outside of fenced blocks — meaning task entries live at level 3 under
/// `## …` phase containers (e.g., 023's `## Phase A — Refactor / ### 1.
/// Task`). Otherwise it is **flat** — task entries are `## N.` at level 2
/// (the original `tasks.md` shape).
///
/// Detection matches the [scenario][runtime-primitive-structural-bugs]
/// edge case "mixed structure → treat as phased": any `### N.` heading
/// anywhere in the file signals phased structure, even if `## N.` headings
/// are also present.
///
/// [runtime-primitive-structural-bugs]: <https://github.com/stonean/ductus/blob/main/specs/022-deterministic-runtime/scenarios/runtime-primitive-structural-bugs.md>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TasksStructure {
    /// No `### N.` headings present; task entries are flat (`## N.`).
    Flat,
    /// At least one `### N.` heading present; task entries live under
    /// `## …` phase containers.
    Phased,
}

/// Detect a `tasks.md` file's structure. Used by `append-task` (to choose
/// flat-append vs phase-append) and `read-tasks` (to walk the appropriate
/// heading levels).
pub(crate) fn detect_tasks_structure(content: &str) -> TasksStructure {
    if iter_task_numbers_at_levels(content, &[3]).next().is_some() {
        TasksStructure::Phased
    } else {
        TasksStructure::Flat
    }
}

/// One `## …` phase container in a phased `tasks.md`. `start_line` and
/// `end_line` are 1-based line numbers from the file's `lines()` iterator;
/// `end_line` is the last content line that belongs to this phase (the
/// line before the next `## …` heading, or the last line of the file
/// when this is the final phase).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PhaseRange {
    /// Full heading text (without the leading `## ` prefix), e.g.,
    /// "Phase A — Refactor" or "Phase C — Follow-on scenarios".
    pub heading: String,
    /// 1-based line number of the `## …` heading line itself.
    pub start_line: usize,
    /// 1-based line number of the last content line that belongs to this
    /// phase (inclusive).
    pub end_line: usize,
}

/// Walk a phased `tasks.md` body and yield each `## …` phase container's
/// heading text plus the line range it covers. `## N.` headings (numeric
/// flat-task remnants in a mixed-structure file) are NOT treated as
/// phase containers — only `## …` headings with non-numeric text qualify.
/// Behavior on a non-phased file is informational; callers should gate
/// on [`detect_tasks_structure`] before consuming this iterator.
pub(crate) fn iter_phase_ranges(content: &str) -> Vec<PhaseRange> {
    let mut phases: Vec<PhaseRange> = Vec::new();
    let mut skip = SkipScanner::default();
    let lines: Vec<&str> = content.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        if skip.skip(line) {
            continue;
        }
        if let Some((2, heading)) = parse_atx_heading(line) {
            // Skip numeric flat-task remnants: a heading whose text begins
            // with "N." (decimal digits, then dot) is a flat task, not a
            // phase container. Mixed files keep their phase set clean.
            if heading_is_numeric(&heading) {
                continue;
            }
            // 1-based line numbers; close out the previous phase before
            // opening the next.
            let one_based = idx + 1;
            if let Some(prev) = phases.last_mut() {
                prev.end_line = one_based.saturating_sub(1);
            }
            phases.push(PhaseRange {
                heading,
                start_line: one_based,
                end_line: lines.len(), // closed below or left at EOF
            });
        }
    }
    phases
}

/// Split a numbered task heading (`"12. Title"`) into its number and title,
/// both borrowed from `heading`. `None` when the heading does not open with
/// the `N.` grammar every tasks-file parser shares — decimal digits followed
/// by a literal dot. A prose heading like `## 3 quick wins` is deliberately
/// not a task, so it must not parse as one.
///
/// This is the single home for the grammar: `read-tasks`, `prune-tasks`, and
/// the phase scanner below all read the same tasks file, so a divergence here
/// would let one primitive see a task another does not.
pub(crate) fn split_numbered_heading(heading: &str) -> Option<(&str, &str)> {
    let end = heading
        .bytes()
        .position(|b| !b.is_ascii_digit())
        .unwrap_or(heading.len());
    if end == 0 {
        return None;
    }
    let (number, after) = heading.split_at(end);
    Some((number, after.strip_prefix('.')?.trim_start()))
}

/// `true` when `heading` opens with the `N.` task-heading grammar — the
/// predicate half of [`split_numbered_heading`], for callers that only need
/// to classify a heading rather than take it apart.
pub(crate) fn heading_is_numeric(heading: &str) -> bool {
    split_numbered_heading(heading).is_some()
}

/// Parse an ATX heading line and return `(level, text)` when the line matches
/// `# heading` through `###### heading`. Trims trailing `#` runs in the closed
/// form (`## Foo ##`).
pub(crate) fn parse_atx_heading(line: &str) -> Option<(u8, String)> {
    let trimmed = line.trim_start();
    let bytes = trimmed.as_bytes();
    let mut level: u8 = 0;
    while (level as usize) < bytes.len() && bytes[level as usize] == b'#' && level < 6 {
        level += 1;
    }
    if level == 0 {
        return None;
    }
    let after = &trimmed[level as usize..];
    if !after.starts_with(' ') && !after.is_empty() {
        return None;
    }
    let heading = after.trim().trim_end_matches('#').trim().to_string();
    Some((level, heading))
}

/// Yield the body lines inside the section with heading `heading`, in
/// document order. The section ends at the next ATX heading whose level
/// is `<=` the matched heading's level (a sibling or shallower heading),
/// or at EOF. Lines INSIDE the section — including blank lines and any
/// nested deeper-level headings — are yielded as-is so consumers can
/// apply their own filters. When the heading appears more than once,
/// lines from every matching section are yielded in document order.
///
/// It skips no fenced block or HTML comment; a reader that must is built on
/// [`section_line_indices`] instead, as `read_spec::parse_open_questions` and
/// `parse_checkboxes` are. `dashboard::context_summary` reads through this
/// one and filters comment lines itself.
pub(crate) fn section_lines<'a>(body: &'a str, heading: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut in_section = false;
    let mut section_level: u8 = 0;
    for line in body.lines() {
        if let Some((level, h)) = parse_atx_heading(line) {
            if in_section && level <= section_level {
                in_section = false;
            }
            if h == heading {
                in_section = true;
                section_level = level;
                continue;
            }
        }
        if in_section {
            out.push(line);
        }
    }
    out
}

/// Comment/fence-aware variant of [`section_lines`]: returns the 0-based
/// indices (into `lines`) of the content lines inside the section named
/// `heading`, applying [`SkipScanner`] semantics to the whole document.
/// Lines inside fenced code blocks or HTML comments are neither yielded
/// nor treated as section-boundary headings, so a template guidance
/// comment that embeds example checkboxes or headings contributes
/// nothing. As with [`section_lines`], every matching section's lines are
/// yielded in document order when the heading repeats.
///
/// This is the single source of truth for "structural lines inside
/// section X": `read-spec`'s acceptance-criteria walk and
/// `mark-criterion`'s checkbox addressing both consume it, which keeps
/// their criterion indexes in lockstep (the two-paths guarantee).
pub(crate) fn section_line_indices(lines: &[&str], heading: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut skip = SkipScanner::default();
    let mut in_section = false;
    let mut section_level: u8 = 0;
    for (idx, line) in lines.iter().enumerate() {
        if skip.skip(line) {
            continue;
        }
        if let Some((level, h)) = parse_atx_heading(line) {
            if in_section && level <= section_level {
                in_section = false;
            }
            if h == heading {
                in_section = true;
                section_level = level;
                continue;
            }
        }
        if in_section {
            out.push(idx);
        }
    }
    out
}

/// The two directory forms a feature can take under the spec root
/// (spec 051, constitution §numbering).
///
/// `Sequential` is the permanent `NNN-slug` form. `BranchScoped` is the
/// temporary `{identifier}.{n}-slug` staging form a story branch creates
/// when it cannot edit an upstream spec in place; it exists only until
/// fold-back discharges it into that spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FeatureForm {
    /// `NNN-slug` — three or more ASCII digits, a hyphen, and a slug.
    Sequential {
        /// The numeric prefix as a number.
        number: u32,
    },
    /// `{identifier}.{n}-slug` — a branch namespace and its counter.
    BranchScoped {
        /// The branch identifier, already in the slug grammar.
        identifier: String,
        /// The per-identifier counter, `1`-based.
        n: u32,
    },
}

impl FeatureForm {
    /// The sequential number, or `None` for a branch-scoped directory.
    ///
    /// The `None` arm is the point: a branch-scoped directory has no
    /// place in the global sequence, so no caller can read one out of it.
    /// The predecessor of this method parsed the first three bytes of any
    /// name, which answered `Some(123)` for `1234.1-slug` — a silent
    /// collision with `123-slug` in feature resolution and in the
    /// next-number computation.
    pub(crate) fn sequential_number(&self) -> Option<u32> {
        match *self {
            Self::Sequential { number } => Some(number),
            Self::BranchScoped { .. } => None,
        }
    }
}

/// Parse a directory name into its [`FeatureForm`], or `None` when it is
/// not a feature directory at all.
///
/// The single place either form is recognized. Primitives that walk the
/// spec root and must tell feature directories from sibling artifacts
/// (`templates/`, `inbox.md`, ad-hoc notes, dotfiles) reach the
/// filesystem through [`list_feature_dirs`] and [`is_spec_path`], both of
/// which delegate here — so widening the corpus happens once rather than
/// once per consumer. A duplicated membership rule is how the shell
/// versions of the frontmatter generators drifted.
///
/// The branch-scoped form splits on the **first** `.`, which is
/// unambiguous because the identifier is held to the slug grammar and so
/// cannot contain one.
pub(crate) fn parse_feature_dir(name: &str) -> Option<FeatureForm> {
    match name.split_once('.') {
        Some((identifier, rest)) => parse_branch_scoped(identifier, rest),
        None => parse_sequential(name),
    }
}

/// At least three digits, then `-`, then at least one character.
///
/// **Three is a minimum, not a width.** `create-feature` formats the
/// number with `{number:03}`, which pads *up to* three digits and then
/// keeps counting, so the 1000th spec in a corpus is named `1000-slug`.
/// A rule that demanded exactly three digits — as this one did — made
/// that directory invisible to every corpus reader the moment it was
/// created, by a predicate disagreeing with the formatter that produced
/// the name.
///
/// A run longer than three digits carrying a leading zero is rejected, so
/// the name/number mapping stays injective: `{number:03}` never emits
/// `0500-`, and accepting it would give `500` two spellings — one of
/// which `next_feature_number` and `resolve-feature` would then disagree
/// about.
///
/// Deliberately does **not** hold the trailing slug to the slug grammar:
/// this form predates the grammar's enforcement, and an adopter's spec
/// root may hold directories that would fail it. Tightening the rule here
/// would make those directories invisible to every corpus reader at once
/// — a silent regression rather than a reported one.
fn parse_sequential(name: &str) -> Option<FeatureForm> {
    let (digits, slug) = name.split_once('-')?;
    if slug.is_empty() || digits.len() < 3 {
        return None;
    }
    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if digits.len() > 3 && digits.starts_with('0') {
        return None;
    }
    let number = digits.parse::<u32>().ok()?;
    Some(FeatureForm::Sequential { number })
}

/// `{identifier}.{n}-{slug}`, given the halves either side of the first `.`.
///
/// The new form is machine-generated end to end — `create-feature`
/// sanitizes the identifier and derives the slug — so both halves are
/// held to the slug grammar. `n` is `[1-9][0-9]*`: a leading zero would
/// let `1234.01-x` and `1234.1-x` name the same counter value from two
/// directories.
fn parse_branch_scoped(identifier: &str, rest: &str) -> Option<FeatureForm> {
    if !is_slug_grammar(identifier) {
        return None;
    }
    let (digits, slug) = rest.split_once('-')?;
    if !is_slug_grammar(slug) {
        return None;
    }
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) || digits.starts_with('0') {
        return None;
    }
    let n = digits.parse::<u32>().ok()?;
    Some(FeatureForm::BranchScoped {
        identifier: identifier.to_string(),
        n,
    })
}

/// `true` when `name` is a feature directory in either form.
///
/// Retained as the reading the call sites want; the recognition itself
/// lives in [`parse_feature_dir`].
pub(crate) fn is_feature_slug(name: &str) -> bool {
    parse_feature_dir(name).is_some()
}

/// Refuse a feature name a session may not store (spec 062, `BE-INPUT-002`),
/// naming `argument`. The allowlist is a feature directory
/// [`parse_feature_dir`] recognizes, made only of visible ASCII with no path
/// separator: the grammar leaves a sequential directory's legacy slug
/// unchecked on purpose, and a session renders the name into the one-line
/// notices and dashboard lines another agent reads, where a control, bidi or
/// zero-width character would forge, reorder or hide text.
pub(crate) fn validate_session_feature(primitive: &str, argument: &str, name: &str) -> Result<()> {
    let visible = name
        .chars()
        .all(|c| c.is_ascii_graphic() && c != '/' && c != '\\');
    if is_feature_slug(name) && visible {
        return Ok(());
    }
    Err(PrimitiveError::InvalidArgument {
        primitive: primitive.into(),
        argument: argument.into(),
        reason: format!(
            "{name:?} is not a feature directory name (`NNN-slug` or \
             `{{branch}}.{{n}}-slug`, visible ASCII, no path separator)"
        ),
    })
}

/// Refuse a scenario name a session may not store (spec 062, `BE-INPUT-002`),
/// naming `argument`: the slug grammar `create-scenario` and `resolve-feature`
/// already hold scenarios to.
pub(crate) fn validate_session_scenario(primitive: &str, argument: &str, slug: &str) -> Result<()> {
    if is_slug_grammar(slug) {
        return Ok(());
    }
    Err(PrimitiveError::InvalidArgument {
        primitive: primitive.into(),
        argument: argument.into(),
        reason: format!(
            "{slug:?} is not a scenario slug (lowercase letters, digits, single hyphens)"
        ),
    })
}

/// Whether a repo-relative path is a feature spec under `specs_root`:
/// `{root}/NNN-slug/(spec|spec-and-plan).md`.
///
/// The shared membership rule for the two frontmatter-index generators. Both
/// enumerate the same corpus, and a second copy of this predicate is how the
/// shell versions drifted.
pub(crate) fn is_spec_path(path: &str, specs_root: &str) -> bool {
    let Some(rest) = path
        .strip_prefix(specs_root)
        .and_then(|r| r.strip_prefix('/'))
    else {
        return false;
    };
    let mut parts = rest.split('/');
    let (Some(feature), Some(file), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    is_feature_slug(feature) && matches!(file, "spec.md" | "spec-and-plan.md")
}

/// The `NNN-slug` feature directory owning a repo-relative spec path.
pub(crate) fn spec_feature_slug(path: &str, specs_root: &str) -> Option<String> {
    path.strip_prefix(specs_root)?
        .strip_prefix('/')?
        .split('/')
        .next()
        .map(str::to_string)
}

/// Every path the git index tracks inside the project, named from the project
/// root, sorted and deduplicated (an index in conflict holds one entry per
/// stage). `Ok(None)` when no work tree contains the project, the
/// no-repository case; an index that cannot be read in a repository that was
/// found is an error, never an empty index. The index names paths from the
/// work tree, so a project in a subdirectory of its repository reads its own
/// files by their names and leaves another project's out (spec 059).
///
/// # Errors
///
/// Returns [`PrimitiveError::Git`] when the repository's index cannot be read.
pub(crate) fn tracked_project_paths(repo: &Path) -> Result<Option<Vec<String>>> {
    let Ok(project) = ProjectRepository::discover(repo) else {
        return Ok(None);
    };
    let index = project.repository.index()?;
    let mut out: Vec<String> = index
        .iter()
        .filter_map(|entry| String::from_utf8(entry.path).ok())
        .filter_map(|path| project.to_project(&path).map(str::to_string))
        .collect();
    out.sort();
    out.dedup();
    Ok(Some(out))
}

/// Feature-spec paths tracked by git, named from the project root, sorted.
///
/// Scoped to the git index rather than a worktree glob so an untracked
/// in-progress draft is never rewritten and never enters a derived index
/// (spec 017, `tracked-specs-not-worktree`). Falls back to a worktree walk
/// only outside a git repo, where there is no index; a repository whose index
/// cannot be read is an error rather than that fallback, which would rewrite
/// the drafts the index exists to exclude.
///
/// # Errors
///
/// Returns [`PrimitiveError::Git`] when the repository's index cannot be read.
pub(crate) fn list_tracked_specs(repo: &Path, specs_root: &str) -> Result<Vec<String>> {
    if let Some(tracked) = tracked_project_paths(repo)? {
        return Ok(tracked
            .into_iter()
            .filter(|path| is_spec_path(path, specs_root))
            .collect());
    }
    let mut out = Vec::new();
    let specs_dir = repo.join(specs_root);
    for feature in list_feature_dirs(&specs_dir) {
        for name in ["spec.md", "spec-and-plan.md"] {
            if specs_dir.join(&feature).join(name).is_file() {
                out.push(format!("{specs_root}/{feature}/{name}"));
            }
        }
    }
    out.sort();
    Ok(out)
}

/// Feature-spec paths present in the worktree but not tracked by git —
/// exactly the set [`list_tracked_specs`] excludes by design — named from the
/// project root.
///
/// Exists so a generator can report what it did *not* examine. A zero rewrite
/// count means "I rewrote nothing", not "everything is in sync": an untracked
/// draft is never enumerated, so a bare in-sync claim would assert a property
/// of files the generator cannot vouch for. Empty outside a git repo, where
/// the fallback already walks everything.
///
/// # Errors
///
/// Returns [`PrimitiveError::Git`] when the status of a repository that was
/// found cannot be read: an empty list there would claim no untracked draft
/// exists when none was looked for (QUAL-CLAIM-001).
pub(crate) fn list_untracked_specs(repo: &Path, specs_root: &str) -> Result<Vec<String>> {
    let Ok(project) = ProjectRepository::discover(repo) else {
        return Ok(Vec::new());
    };
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false)
        // Bound the walk to the spec tree. Without a pathspec this is a full
        // worktree status on every run — including the pre-commit path, where
        // it would scan build output and vendored trees to answer a question
        // only about `{specs-root}/`. Literal, because the project's path from
        // the work tree may hold `*`, `?` or `[`, which as a glob would match
        // no file and report no untracked draft.
        .pathspec(project.to_git(specs_root))
        .disable_pathspec_match(true);
    let statuses = project.repository.statuses(Some(&mut opts))?;
    let mut out: Vec<String> = Vec::new();
    for entry in statuses.iter() {
        if !entry.status().contains(git2::Status::WT_NEW) {
            continue;
        }
        // `path()` errors on a non-UTF-8 path, which cannot be a spec under
        // the validated slug grammar anyway.
        let Ok(path) = entry.path() else { continue };
        if let Some(path) = project.to_project(path)
            && is_spec_path(path, specs_root)
        {
            out.push(path.to_string());
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

/// Feature-spec paths staged in the index for the pending commit, named from
/// the project root — the `--staged` rewrite set, so committing one spec
/// never rewrites the derived frontmatter of unrelated specs. Empty outside a
/// git repo.
///
/// # Errors
///
/// Returns [`PrimitiveError::Git`] when the staged changes of a repository
/// that was found cannot be read.
pub(crate) fn list_staged_specs(
    repo: &Path,
    specs_root: &str,
) -> Result<std::collections::BTreeSet<String>> {
    let Ok(project) = ProjectRepository::discover(repo) else {
        return Ok(std::collections::BTreeSet::new());
    };
    let repository = &project.repository;
    // HEAD tree against the index. An unborn HEAD (no commits yet) diffs the
    // index against nothing, which is the correct "everything staged" answer.
    let head_tree = repository
        .head()
        .ok()
        .and_then(|head| head.peel_to_tree().ok());
    // Bounded to the spec tree, as the untracked listing is, rather than the
    // whole index against HEAD — which in a large repository holding the
    // project is every other project's staged work too. Literal, for the
    // same reason.
    let mut opts = git2::DiffOptions::new();
    opts.pathspec(project.to_git(specs_root))
        .disable_pathspec_match(true);
    let diff = repository.diff_tree_to_index(head_tree.as_ref(), None, Some(&mut opts))?;
    let mut out = std::collections::BTreeSet::new();
    for delta in diff.deltas() {
        for file in [delta.new_file(), delta.old_file()] {
            if let Some(path) = file.path().and_then(Path::to_str)
                && let Some(path) = project.to_project(path)
                && is_spec_path(path, specs_root)
            {
                out.insert(path.to_string());
            }
        }
    }
    Ok(out)
}

/// List feature directories under the spec root — sequential (`NNN-slug`)
/// and branch-scoped (`{identifier}.{n}-slug`) alike — in corpus order
/// ([`feature_dir_cmp`]), which compares numbers numerically. Best-effort: a missing or unreadable spec root yields an empty
/// list — a repo without a spec root has no features by definition, and
/// every caller reports the empty case as "no features" rather than an
/// operational error.
pub(crate) fn list_feature_dirs(specs_dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(specs_dir) else {
        return Vec::new();
    };
    let mut features: Vec<String> = entries
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| is_feature_slug(name))
        .collect();
    features.sort_by(|a, b| feature_dir_cmp(a, b));
    features
}

/// Total order over a mixed spec root, shared by every surface that
/// presents features so a reader never sees two orders for one directory
/// — the guarantee [`scenario_name_cmp`] gives scenarios.
///
/// Sequential directories sort first, by number; branch-scoped ones
/// follow, grouped by identifier and then ordered by counter. Grouping
/// the transient form after the permanent one keeps the pipeline view's
/// familiar shape: a reader scanning for `037-` does not step over a
/// branch namespace to reach it.
///
/// The counter is compared **numerically**, which is the whole reason a
/// comparator is needed rather than a plain sort: `1234.10-x` precedes
/// `1234.2-x` lexicographically. Sequential names are compared by number
/// too — `999-x` precedes `1000-x`, which byte order inverts — so a corpus
/// of three-digit names comes back in the byte order it always did.
///
/// The name is the final tiebreak, so the order is total: two directories
/// that agree on form, number, identifier, and counter still sort
/// deterministically.
pub(crate) fn feature_dir_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    /// Sequential before branch-scoped; an unparseable name (which
    /// `list_feature_dirs` has already filtered out) sorts last rather
    /// than panicking.
    fn rank(form: Option<&FeatureForm>) -> u8 {
        match form {
            Some(FeatureForm::Sequential { .. }) => 0,
            Some(FeatureForm::BranchScoped { .. }) => 1,
            None => 2,
        }
    }

    let (fa, fb) = (parse_feature_dir(a), parse_feature_dir(b));
    let by_form = rank(fa.as_ref()).cmp(&rank(fb.as_ref()));
    if by_form != Ordering::Equal {
        return by_form;
    }
    let within = match (&fa, &fb) {
        (
            Some(FeatureForm::Sequential { number: x }),
            Some(FeatureForm::Sequential { number: y }),
        ) => x.cmp(y),
        (
            Some(FeatureForm::BranchScoped {
                identifier: ia,
                n: na,
            }),
            Some(FeatureForm::BranchScoped {
                identifier: ib,
                n: nb,
            }),
        ) => ia.cmp(ib).then_with(|| na.cmp(nb)),
        _ => Ordering::Equal,
    };
    within.then_with(|| a.cmp(b))
}

/// List the scenario markdown files directly under `scenarios_dir`, sorted
/// CASE-INSENSITIVELY by filename. Matches the `.md` extension
/// case-insensitively too, so `FOO.MD` and `foo.md` are both scenarios —
/// the consuming surfaces (`dashboard` counts, `check-artifacts` derives
/// slugs, `read-spec` collects scenario open questions) must agree on the
/// same set AND the same order. Subdirectories and non-markdown files are
/// excluded; an absent or unreadable directory yields an empty list.
///
/// Ordering uses [`scenario_name_cmp`] — one comparator shared by every
/// surface that presents scenarios, so a reader never sees two different
/// orders for the same directory (spec 046).
pub(crate) fn list_scenario_files(scenarios_dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(scenarios_dir) else {
        return Vec::new();
    };
    let mut files: Vec<String> = entries
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| {
            Path::new(name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        })
        .collect();
    files.sort_by(|a, b| scenario_name_cmp(a, b));
    files
}

/// Order two scenario names case-insensitively, breaking ties on the raw
/// bytes. The tiebreak is load-bearing: `read_dir` yields entries in
/// filesystem order, so `ALPHA.MD` and `alpha.md` — equal under a
/// lowercase-only key — would otherwise sort nondeterministically across
/// machines. Shared by every surface that orders scenarios so the rule is
/// stated once (spec 046).
pub(crate) fn scenario_name_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    a.to_lowercase()
        .cmp(&b.to_lowercase())
        .then_with(|| a.cmp(b))
}

/// Scenario frontmatter shape: `section` is the post-017 field; `spec-ref`
/// is the pre-017 legacy field still present on older scenarios. The single
/// shared definition for `resolve-feature`'s scenario detail and
/// `dashboard`'s session-target detail, so both surfaces fold
/// `section.or(spec-ref)` identically.
#[derive(serde::Deserialize)]
pub(crate) struct ScenarioFrontmatter {
    /// Post-017 section label the scenario belongs to.
    #[serde(default)]
    pub(crate) section: Option<String>,
    /// Pre-017 legacy field, read only as a fallback for `section`.
    #[serde(default, rename = "spec-ref")]
    pub(crate) spec_ref: Option<String>,
}

/// Best-effort read of a scenario file's `section` (or legacy `spec-ref`)
/// frontmatter field. `None` when the file is unreadable, has no
/// frontmatter, or the frontmatter fails to parse — every consumer degrades
/// an unreadable scenario to a detail-less entry rather than an error.
pub(crate) fn read_scenario_section(path: &Path) -> Option<String> {
    let content = read_text(path).ok()?;
    let (fm_text, _body) = split_frontmatter(&content, path).ok()?;
    let fm = serde_norway::from_str::<ScenarioFrontmatter>(fm_text).ok()?;
    fm.section.or(fm.spec_ref)
}

/// The two candidate locations for a spec-pipeline template file, in
/// resolution order: the installed adopter layout
/// `{specs-root}/templates/{file}` (what `/ductus` scaffolds and the
/// command prose names) first, then the framework source layout
/// `framework/templates/spec/{file}` (the ductus repo itself). Shared by
/// `create-feature`'s template copy and the `writeSpecBody` request builder
/// (`interpreter::payload::load_template`); each caller keeps its own
/// missing-template policy.
pub(crate) fn template_candidates(specs_root: &str, file: &str) -> [String; 2] {
    [
        format!("{specs_root}/templates/{file}"),
        format!("framework/templates/spec/{file}"),
    ]
}

/// Resolve a spec-pipeline template through [`template_candidates`],
/// returning `(repo-relative path, absolute path)` of the first candidate
/// on disk, or [`PrimitiveError::TemplateNotFound`] naming every location
/// tried. Shared by `create-feature` (spec template) and
/// `create-plan-artifacts` (plan/tasks/data-model templates).
pub(crate) fn resolve_template(
    repo: &Path,
    specs_root: &str,
    file: &str,
) -> Result<(String, PathBuf)> {
    let candidates = template_candidates(specs_root, file);
    for rel in &candidates {
        let abs = repo.join(rel);
        if abs.is_file() {
            return Ok((rel.clone(), abs));
        }
    }
    Err(PrimitiveError::TemplateNotFound {
        tried: candidates.join(", "),
    })
}

/// Parse the `## Affected Files` markdown table in a plan body and return
/// the first-column path entries in document order. Tolerates rows with
/// backtick-wrapped paths and skips the header separator row. Shared by the
/// writeCode plan reader (`interpreter::payload`) and `compute-review-scope`
/// so both readers agree on the one canonical plan format (a table; see spec
/// 022 task 47).
///
/// A table inside a fenced block or an HTML comment is not the plan's: the
/// plan template carries an example table inside its guidance comment, and a
/// scaffolded plan that keeps it must not put the example's paths in a review
/// scope.
pub(crate) fn parse_affected_files(plan_content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_section = false;
    let mut saw_header = false;
    let mut skip = SkipScanner::default();
    for line in plan_content.lines() {
        // `skip` runs first on every line, so the scanner advances in order.
        if skip.skip(line) {
            continue;
        }
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("## ") {
            // Heading boundary: enter the section when we hit its header,
            // exit on any other H2.
            in_section = rest.trim().eq_ignore_ascii_case("Affected Files");
            saw_header = false;
            continue;
        }
        if !in_section {
            continue;
        }
        if !trimmed.starts_with('|') {
            // A non-table line ends the current table. Resetting here is what
            // makes a section holding SEVERAL tables parse correctly: without
            // it `saw_header` stays true past the first separator, so every
            // later table's `| File | Action | … |` header is read as a data
            // row and the literal word "File" lands in the path list. Spec
            // 017's plan has eleven such tables, which is how it surfaced.
            saw_header = false;
            continue;
        }
        // Skip the separator row (e.g., `| --- | --- | --- |`).
        if trimmed
            .bytes()
            .all(|b| matches!(b, b'|' | b'-' | b':' | b' '))
        {
            saw_header = true;
            continue;
        }
        if !saw_header {
            // First row is the header (`| File | Action | ... |`) — skip
            // until the separator passes.
            continue;
        }
        // Strip the leading `|`, take the first cell.
        let after_pipe = trimmed.trim_start_matches('|');
        let Some((cell, _)) = after_pipe.split_once('|') else {
            continue;
        };
        // A cell may carry a qualifier after the path (`` `constitution.md`
        // (root) ``). When it holds a backticked span, that span IS the path;
        // trimming stray backticks off the whole cell would keep the trailing
        // prose and yield a path that cannot resolve.
        let cell = cell.trim();
        let path = match cell
            .split_once('`')
            .and_then(|(_, rest)| rest.split_once('`'))
        {
            Some((inner, _)) => inner.trim(),
            None => cell.trim_matches('`').trim(),
        };
        if path.is_empty() {
            continue;
        }
        out.push(path.to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    // --- project repository ----------------------------------------------------
    // Git names paths from the work tree and a project names them from its own
    // root; these pin the conversion between the two (spec 059).

    #[test]
    fn a_project_at_the_work_tree_root_converts_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        git2::Repository::init(tmp.path()).unwrap();
        let project = ProjectRepository::discover(tmp.path()).unwrap();
        assert_eq!(project.to_git("specs/001-x/spec.md"), "specs/001-x/spec.md");
        assert_eq!(project.to_project("src/a.rs"), Some("src/a.rs"));
    }

    #[test]
    fn a_project_in_a_subdirectory_converts_through_its_place() {
        let tmp = tempfile::tempdir().unwrap();
        let (_, root) = git_fixture::subdirectory_project(tmp.path());
        let project = ProjectRepository::discover(&root).unwrap();
        assert_eq!(
            project.to_git("specs/001-x/spec.md"),
            "proj/specs/001-x/spec.md"
        );
        assert_eq!(
            project.to_project("proj/specs/001-x/spec.md"),
            Some("specs/001-x/spec.md")
        );
    }

    #[test]
    fn a_path_outside_the_project_is_not_the_projects() {
        let tmp = tempfile::tempdir().unwrap();
        let (_, root) = git_fixture::subdirectory_project(tmp.path());
        let project = ProjectRepository::discover(&root).unwrap();
        assert_eq!(project.to_project("other/x.rs"), None);
        // A sibling whose name starts with the project's is another project.
        // Matching on the bare name rather than the name and its `/` would
        // claim it.
        assert_eq!(project.to_project("proj-other/x.rs"), None);
        assert_eq!(project.to_project("README.md"), None);
    }

    #[cfg(unix)]
    #[test]
    fn a_project_reached_through_a_symlink_takes_its_targets_place() {
        let tmp = tempfile::tempdir().unwrap();
        let repo_root = tmp.path().join("repo");
        let (_, root) = git_fixture::subdirectory_project(&repo_root);
        let link = tmp.path().join("link");
        std::os::unix::fs::symlink(&root, &link).unwrap();
        let project = ProjectRepository::discover(&link).unwrap();
        assert_eq!(project.to_git("specs/x.md"), "proj/specs/x.md");
    }

    #[test]
    fn a_repository_with_no_work_tree_is_no_repository() {
        // A bare repository has no work tree for the project to sit in, so
        // there is no place to convert through. Reading it as the work tree's
        // root would look up paths git never recorded and find no history.
        let tmp = tempfile::tempdir().unwrap();
        git2::Repository::init_bare(tmp.path()).unwrap();
        assert!(ProjectRepository::discover(tmp.path()).is_err());
    }

    #[test]
    fn a_directory_in_no_repository_is_no_repository() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(ProjectRepository::discover(tmp.path()).is_err());
    }

    // --- audit record loading -------------------------------------------------
    // Absent, unreadable and present are three answers, not two (spec 057 AC5).
    // Each case below pins one of them, because the failure this guards against
    // is precisely the collapse of the middle one into the first.

    #[test]
    fn record_load_reports_absent_when_the_artifact_is_not_there() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            load_review_record(dir.path()),
            RecordLoad::Absent,
            "a feature with no review.md has not been reviewed"
        );
        assert_eq!(load_analyze_record(dir.path()), RecordLoad::Absent);
    }

    #[test]
    fn record_load_reports_unreadable_rather_than_absent_on_a_broken_artifact() {
        let dir = tempfile::tempdir().unwrap();

        // Present, but no frontmatter fence at all.
        std::fs::write(dir.path().join("review.md"), "# Review\n\nno frontmatter\n").unwrap();
        assert!(
            matches!(load_review_record(dir.path()), RecordLoad::Unreadable(_)),
            "a review.md with no frontmatter says nothing about whether a review ran"
        );

        // Present, fenced, but the YAML will not deserialize into the record.
        std::fs::write(
            dir.path().join("analysis.md"),
            "---\nhard-fail: not-a-number\n---\n\n# Analysis\n",
        )
        .unwrap();
        assert!(
            matches!(load_analyze_record(dir.path()), RecordLoad::Unreadable(_)),
            "a record whose types do not parse is undeterminable, never never-run"
        );
    }

    #[test]
    fn record_load_carries_every_merged_field() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("review.md"),
            "---\n\
             spec: 057-analyze-artifact-and-record-relocation\n\
             last-run: 2026-09-15T00:00:00Z\n\
             reviewed-against: abc1234\n\
             diff-base: def5678\n\
             must-violations: 1\n\
             should-violations: 2\n\
             low-confidence: 3\n\
             captured-issues: 4\n\
             examined: 5\n\
             scope: 6\n\
             skipped-passes: [security, simplicity]\n\
             blocking: true\n\
             ---\n\n# Review\n",
        )
        .unwrap();

        let RecordLoad::Present(record) = load_review_record(dir.path()) else {
            panic!("expected a present record");
        };

        // The fields that existed on only one side before the merge are the
        // ones a careless merge drops, so they are asserted individually.
        assert_eq!(
            record.spec.as_deref(),
            Some("057-analyze-artifact-and-record-relocation")
        );
        assert_eq!(record.diff_base.as_deref(), Some("def5678"));
        // `captured-issues: 4` is a pre-058 field: still in the file, still
        // parsed past, no longer carried by the record.
        assert_eq!(record.dispositions, None);
        assert_eq!(record.skipped_passes, vec!["security", "simplicity"]);

        // ...and the fields that were duplicated still arrive.
        assert_eq!(record.last_run.as_deref(), Some("2026-09-15T00:00:00Z"));
        assert_eq!(record.reviewed_against.as_deref(), Some("abc1234"));
        assert_eq!(record.must_violations, 1);
        assert_eq!(record.should_violations, 2);
        assert_eq!(record.low_confidence, 3);
        assert_eq!(record.examined, Some(5));
        assert_eq!(record.scope, Some(6));
        assert!(record.blocking);
    }

    #[test]
    fn record_load_round_trips_the_analyze_record() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("analysis.md"),
            "---\n\
             spec: 057-analyze-artifact-and-record-relocation\n\
             last-run: 2026-09-15T00:00:00Z\n\
             analyzed-against: abc1234\n\
             hard-fail: 0\n\
             blocking-findings: 0\n\
             advisory: 2\n\
             unexamined: 1\n\
             blocking: false\n\
             analyzed-digest:\n\
             \u{20} spec.md: aaaa\n\
             ---\n\n# Analysis\n",
        )
        .unwrap();

        let RecordLoad::Present(record) = load_analyze_record(dir.path()) else {
            panic!("expected a present record");
        };
        assert_eq!(record.last_run.as_deref(), Some("2026-09-15T00:00:00Z"));
        assert_eq!(record.advisory, 2);
        assert_eq!(record.unexamined, 1);
        assert!(!record.blocking);
        assert_eq!(
            record.analyzed_digest.get("spec.md").map(String::as_str),
            Some("aaaa")
        );
    }

    // --- dispositions (spec 058) ---------------------------------------------
    // Absent and all-zero are two answers: a record that predates dispositions
    // did not disposition anything, while a record whose run found nothing
    // did. And a partial map is unreadable rather than defaulted, because a
    // defaulted `undispositioned: 0` would claim decisions nobody made.

    #[test]
    fn a_record_that_predates_dispositions_loads_with_none_and_keeps_parsing() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("review.md"),
            "---\nlast-run: 2026-09-15T00:00:00Z\ncaptured-issues: 2\nblocking: false\n---\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("analysis.md"),
            "---\nlast-run: 2026-09-15T00:00:00Z\ncaptured-issues: 3\nblocking: false\n---\n",
        )
        .unwrap();

        let RecordLoad::Present(review) = load_review_record(dir.path()) else {
            panic!("a pre-058 review.md must still parse");
        };
        let RecordLoad::Present(analysis) = load_analyze_record(dir.path()) else {
            panic!("a pre-058 analysis.md must still parse");
        };
        assert_eq!(review.dispositions, None);
        assert_eq!(analysis.dispositions, None);
    }

    #[test]
    fn an_all_zero_map_is_present_not_absent() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("analysis.md"),
            "---\nlast-run: 2026-09-25T00:00:00Z\nblocking: false\n\
             dispositions:\n  fixed: 0\n  routed: 0\n  discarded: 0\n  undispositioned: 0\n---\n",
        )
        .unwrap();
        let RecordLoad::Present(analysis) = load_analyze_record(dir.path()) else {
            panic!("expected a present record");
        };
        assert_eq!(
            analysis.dispositions,
            Some(crate::schema::primitives::Dispositions::default()),
            "a run that found nothing recorded that it found nothing"
        );
    }

    #[test]
    fn a_partial_dispositions_map_is_unreadable_not_defaulted() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("review.md"),
            "---\nlast-run: 2026-09-25T00:00:00Z\n\
             dispositions:\n  fixed: 1\n  routed: 0\n  discarded: 0\n---\n",
        )
        .unwrap();
        assert!(
            matches!(load_review_record(dir.path()), RecordLoad::Unreadable(_)),
            "a map missing `undispositioned` must not read as zero undispositioned"
        );
    }

    #[test]
    fn read_recorded_list_reads_any_key_and_refuses_a_malformed_one() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("analysis.md"),
            "---\nwaivers: [a, b]\ndecisions: {not: a-list}\n---\n",
        )
        .unwrap();
        let listed: Vec<String> =
            read_recorded_list(dir.path(), ANALYSIS_RECORD_FILE, "waivers").unwrap();
        assert_eq!(listed, vec!["a", "b"]);
        assert!(
            read_recorded_list::<String>(dir.path(), ANALYSIS_RECORD_FILE, "decisions").is_err()
        );
        let empty: Vec<String> =
            read_recorded_list(dir.path(), ANALYSIS_RECORD_FILE, "absent").unwrap();
        assert_eq!(empty, [] as [String; 0]);
    }

    // --- numbered task headings ----------------------------------------------
    // Moved here with the helper (scenario numbered-heading-grammar-single-source).
    // The cases below are the union of what the three former copies each
    // covered, so collapsing them cannot silently drop a contract.

    #[test]
    fn split_numbered_heading_extracts_number_and_title() {
        assert_eq!(
            split_numbered_heading("12. Implement the parser"),
            Some(("12", "Implement the parser"))
        );
        assert_eq!(
            split_numbered_heading("3. Wire CLI"),
            Some(("3", "Wire CLI"))
        );
        assert_eq!(split_numbered_heading("Not numbered"), None);
        // A prose heading whose digits are not followed by `.` is not a task.
        assert_eq!(split_numbered_heading("3 quick wins"), None);
        // Bare digits with no dot at all — the `prune-tasks` copy tolerated
        // this and returned an empty title; the shared grammar rejects it,
        // which is what its only caller (guarded by the task level) expects.
        assert_eq!(split_numbered_heading("12"), None);
        // A number with the dot but no title is a task with an empty title.
        assert_eq!(split_numbered_heading("12."), Some(("12", "")));
    }

    #[test]
    fn heading_is_numeric_agrees_with_the_splitter() {
        for heading in [
            "12. Implement the parser",
            "3 quick wins",
            "Not numbered",
            "12",
            "12.",
            "",
        ] {
            assert_eq!(
                heading_is_numeric(heading),
                split_numbered_heading(heading).is_some(),
                "predicate and splitter disagreed on {heading:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn write_atomic_preserves_existing_file_mode() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spec.md");
        std::fs::write(&path, "old").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        write_atomic(&path, "new").unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(
            mode, 0o644,
            "in-place rewrite must not narrow the file mode"
        );
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new");
    }

    #[test]
    fn validate_slug_accepts_normal_slugs() {
        validate_slug("retry-on-timeout").unwrap();
        validate_slug("a").unwrap();
        validate_slug("ask-consolidation").unwrap();
    }

    #[test]
    fn validate_slug_rejects_empty() {
        assert!(matches!(
            validate_slug("").unwrap_err(),
            PrimitiveError::InvalidSlug { .. }
        ));
    }

    #[test]
    fn validate_slug_rejects_path_separators() {
        for bad in &["a/b", "a\\b", "../escape", "..\\escape"] {
            assert!(
                matches!(
                    validate_slug(bad).unwrap_err(),
                    PrimitiveError::InvalidSlug { .. }
                ),
                "expected rejection for {bad:?}"
            );
        }
    }

    #[test]
    fn validate_slug_rejects_dotfile_prefix() {
        for bad in &[".hidden", "..", "."] {
            assert!(
                matches!(
                    validate_slug(bad).unwrap_err(),
                    PrimitiveError::InvalidSlug { .. }
                ),
                "expected rejection for {bad:?}"
            );
        }
    }

    #[test]
    fn validate_slug_rejects_newlines_and_control_chars() {
        // The denylist that BE-INPUT-002 replaced admitted these into a
        // written filename and a rendered heading; the allowlist rejects
        // every character outside `[a-z0-9-]`.
        for bad in &["a\nb", "a\rb", "a\tb", "a b", "a\u{7f}b", "a\0b"] {
            assert!(
                matches!(
                    validate_slug(bad).unwrap_err(),
                    PrimitiveError::InvalidSlug { .. }
                ),
                "expected rejection for {bad:?}"
            );
        }
    }

    #[test]
    fn validate_slug_rejects_non_grammar_shapes() {
        // Uppercase, underscores, dots, and leading/trailing/repeated
        // hyphens all fall outside ^[a-z0-9]+(?:-[a-z0-9]+)*$.
        for bad in &["Upper", "a_b", "-lead", "trail-", "a--b", "a.b", "café"] {
            assert!(
                matches!(
                    validate_slug(bad).unwrap_err(),
                    PrimitiveError::InvalidSlug { .. }
                ),
                "expected rejection for {bad:?}"
            );
        }
    }

    #[test]
    fn validate_slug_accepts_grammar_conformant_slugs() {
        for good in &["a", "022", "retry-on-timeout", "spec-042-foo", "x1-y2-z3"] {
            validate_slug(good).unwrap();
        }
    }

    /// The shell surfaces that cannot call the runtime carry a copy of the
    /// directory-membership rule, and this holds the copies to it.
    ///
    /// Three files match spec paths with a pattern rather than a primitive,
    /// each for the same reason: the pre-commit hooks run before any binary
    /// is resolvable, and `lint-frontmatter.sh` exists to find malformed
    /// frontmatter so it cannot depend on something that parses frontmatter.
    /// A copy is the right answer there; an *unwatched* copy is not. Both
    /// gaps this test now guards were live before it existed — a
    /// digits-only pattern skipped the 1000th spec, and it skipped every
    /// branch-scoped directory outright, so `label-criteria` never ran on
    /// one and spec 013's backstop was dead for it.
    ///
    /// The property is one-sided on purpose: **the pattern must accept
    /// everything `parse_feature_dir` accepts.** A false negative is a spec
    /// silently dropped from a hook or a lint; a false positive is a path
    /// handed to a primitive that resolves it and declines, which is noise
    /// rather than damage. The known divergences are pinned below so they
    /// read as decisions rather than oversights.
    #[test]
    fn the_shell_spec_pattern_accepts_what_the_grammar_accepts() {
        /// The alternation every shell copy carries, verbatim.
        const ALTERNATION: &str =
            r"(([0-9][0-9][0-9]|[1-9][0-9][0-9][0-9]+)|[a-z0-9]+(-[a-z0-9]+)*\.[1-9][0-9]*)";
        const CARRIERS: &[&str] = &[
            ".githooks/pre-commit",
            "framework/bootstrap/hooks/ductus-pre-commit",
            "scripts/lint-frontmatter.sh",
        ];

        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("runtime/ has a parent")
            .to_path_buf();

        for rel in CARRIERS {
            let path = repo.join(rel);
            let source =
                std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {rel}: {e}"));
            assert!(
                source.contains(ALTERNATION),
                "{rel} no longer carries the shared feature-directory alternation.\n\
                 Expected to find: {ALTERNATION}\n\
                 The two pre-commit hooks are separate files and both need the edit; \
                 a fix applied to one leaves the other — and every adopter — behind."
            );
        }

        let pattern = regex::Regex::new(&format!("^{ALTERNATION}-.+$"))
            .expect("the shared alternation compiles");

        // Names the runtime accepts. Every one must match, or a real spec is
        // invisible to a hook or a lint.
        for name in &[
            "000-skeleton",
            "050-alpha",
            "999-last-three-digit",
            "1000-thousandth",
            "12345-far-future",
            "1234.1-staged",
            "1234.10-late",
            "proj-1111.12-ticket-work",
            "1111-proj.1-item-work",
        ] {
            assert!(
                parse_feature_dir(name).is_some(),
                "table drift: the runtime no longer accepts {name:?}"
            );
            assert!(
                pattern.is_match(name),
                "the shell pattern drops {name:?}, which the runtime accepts — \
                 a spec invisible to the hooks and the frontmatter lint"
            );
        }

        // Names both must reject. A pattern looser than the grammar here
        // would put a directory no corpus reader can see in front of a lint.
        for name in &[
            "05-short",      // fewer than three digits
            "0500-padded",   // padding past the `{number:03}` minimum
            "1234.0-zero",   // the branch counter is 1-based
            "1234.01-alias", // a leading zero would alias `.1`
            "templates",     // a sibling directory, not a feature
            "inbox.md",
        ] {
            assert!(
                parse_feature_dir(name).is_none(),
                "table drift: the runtime now accepts {name:?}"
            );
            assert!(
                !pattern.is_match(name),
                "the shell pattern accepts {name:?}, which the runtime rejects"
            );
        }

        // Pinned divergences: the pattern is a superset in exactly these
        // ways, because a regex over a path segment cannot hold the slug to
        // the grammar `validate_slug` enforces. Harmless in the one-sided
        // direction — the primitive that receives the path declines it.
        for name in &["1234.1-Uppercase", "1234.1-under_score"] {
            assert!(
                parse_feature_dir(name).is_none(),
                "table drift: the runtime now accepts {name:?}"
            );
            assert!(
                pattern.is_match(name),
                "the documented superset narrowed; if that is deliberate, \
                 move {name:?} into the reject table above"
            );
        }
    }

    #[test]
    fn line_ending_of_reads_the_file_it_is_given() {
        assert_eq!(line_ending_of("a\r\nb\r\n"), "\r\n");
        assert_eq!(line_ending_of("a\nb\n"), "\n");
        // No line ending at all — empty, or one unterminated line — carries
        // no evidence either way, so the platform-neutral default applies.
        assert_eq!(line_ending_of(""), "\n");
        assert_eq!(line_ending_of("no newline here"), "\n");
    }

    /// Mixed endings have no state to preserve, so the dominant one governs
    /// and ties go to LF. `contains("\r\n")` — what the three retired copies
    /// each used — would have answered CRLF for a mostly-LF file carrying one
    /// stray CRLF.
    #[test]
    fn line_ending_of_resolves_a_mixed_file_to_the_dominant_ending() {
        assert_eq!(line_ending_of("a\nb\nc\nd\r\n"), "\n");
        assert_eq!(line_ending_of("a\r\nb\r\nc\r\nd\n"), "\r\n");
        // A tie is not evidence for CRLF.
        assert_eq!(line_ending_of("a\r\nb\n"), "\n");
    }

    #[test]
    fn with_line_ending_converts_and_is_idempotent() {
        assert_eq!(with_line_ending("a\nb\n", "\r\n"), "a\r\nb\r\n");
        assert_eq!(with_line_ending("a\r\nb\r\n", "\n"), "a\nb\n");
        // Applying an ending a file already uses changes nothing — the
        // property that lets writers normalize unconditionally.
        assert_eq!(with_line_ending("a\r\nb\r\n", "\r\n"), "a\r\nb\r\n");
        assert_eq!(with_line_ending("a\nb\n", "\n"), "a\nb\n");
    }

    /// The tail after the final `\n` is an unterminated line, not a line
    /// ending, so it survives verbatim and no terminator is invented for it.
    #[test]
    fn with_line_ending_leaves_an_unterminated_final_line_alone() {
        assert_eq!(with_line_ending("a\nb", "\r\n"), "a\r\nb");
        assert_eq!(with_line_ending("", "\r\n"), "");
    }

    #[test]
    fn join_lines_terminated_ends_with_exactly_one_ending() {
        assert_eq!(join_lines_terminated(&["a", "b"], "\r\n"), "a\r\nb\r\n");
        assert_eq!(join_lines_terminated(&["a"], "\n"), "a\n");
        assert_eq!(join_lines_terminated::<&str>(&[], "\n"), "");
    }

    #[test]
    fn parse_feature_dir_reads_the_sequential_form() {
        assert_eq!(
            parse_feature_dir("022-deterministic-runtime"),
            Some(FeatureForm::Sequential { number: 22 })
        );
        assert_eq!(
            parse_feature_dir("007-webhooks"),
            Some(FeatureForm::Sequential { number: 7 })
        );
        assert_eq!(parse_feature_dir("abc-nope"), None);
        assert_eq!(parse_feature_dir("22"), None); // fewer than three digits
        assert_eq!(parse_feature_dir("22-short"), None);
        assert_eq!(parse_feature_dir("050"), None); // digits but no slug
    }

    /// The three-digit pad is a minimum width, not a fixed one: the 1000th
    /// spec in a corpus is named `1000-slug` by `create-feature`'s
    /// `{number:03}` formatter, and a predicate demanding exactly three
    /// digits made that directory invisible to every corpus reader the
    /// moment it was created. The formatter and the membership rule have
    /// to agree across the whole range, not just the first 1000.
    #[test]
    fn parse_feature_dir_reads_a_sequential_number_past_999() {
        assert_eq!(
            parse_feature_dir("1000-thousandth"),
            Some(FeatureForm::Sequential { number: 1000 })
        );
        assert_eq!(
            parse_feature_dir("12345-far-future"),
            Some(FeatureForm::Sequential { number: 12345 })
        );
        // Injective by construction: `{number:03}` never emits a leading
        // zero past three digits, so accepting one would give 500 two
        // spellings that the counter and feature resolution would then
        // disagree about.
        assert_eq!(parse_feature_dir("0500-padded-twice"), None);
        // Still branch-scoped — the dot is examined before the digits are.
        assert_eq!(
            parse_feature_dir("1000.1-staged"),
            Some(FeatureForm::BranchScoped {
                identifier: "1000".into(),
                n: 1,
            })
        );
    }

    #[test]
    fn parse_feature_dir_reads_the_branch_scoped_form() {
        assert_eq!(
            parse_feature_dir("1234.1-thing"),
            Some(FeatureForm::BranchScoped {
                identifier: "1234".into(),
                n: 1,
            })
        );
        // The identifier is an opaque token, not a number: a Jira-style
        // `PROJ-1111` sanitizes to `proj-1111` before it reaches here.
        assert_eq!(
            parse_feature_dir("proj-1111.12-thing"),
            Some(FeatureForm::BranchScoped {
                identifier: "proj-1111".into(),
                n: 12,
            })
        );
    }

    #[test]
    fn parse_feature_dir_rejects_malformed_branch_scoped_names() {
        for bad in &[
            "1234.0-thing",   // n is 1-based
            "1234.01-thing",  // leading zero would alias 1234.1
            "1234.-thing",    // no counter
            "1234.1-",        // no slug
            "1234.1thing",    // no hyphen after the counter
            ".1-thing",       // no identifier
            "PROJ.1-thing",   // identifier outside the slug grammar
            "1234.1.2-thing", // a second dot leaves a non-numeric counter
            "inbox.md",       // sibling artifact, not a feature
            ".hidden",
        ] {
            assert_eq!(
                parse_feature_dir(bad),
                None,
                "expected rejection for {bad:?}"
            );
        }
    }

    #[test]
    fn feature_dir_cmp_orders_a_mixed_corpus() {
        let mut names = vec![
            "1234.10-late".to_string(),
            "051-b".to_string(),
            "proj-9.1-other".to_string(),
            "1234.2-early".to_string(),
            "1000-e".to_string(),
            "007-a".to_string(),
            "999-d".to_string(),
        ];
        names.sort_by(|a, b| feature_dir_cmp(a, b));
        assert_eq!(
            names,
            vec![
                // Sequential first, by number — 999 before 1000, which byte
                // order inverts.
                "007-a".to_string(),
                "051-b".to_string(),
                "999-d".to_string(),
                "1000-e".to_string(),
                // Then branch-scoped, grouped by identifier, counter
                // compared numerically — 2 before 10, which a plain
                // lexicographic sort would invert.
                "1234.2-early".to_string(),
                "1234.10-late".to_string(),
                "proj-9.1-other".to_string(),
            ]
        );
    }

    // Unix-only: `sh` is the portable way to emit on both streams and exit
    // non-zero from a test; the helper itself is platform-independent.
    #[cfg(unix)]
    #[test]
    fn output_within_captures_what_output_would() {
        let output = output_within(
            std::process::Command::new("sh")
                .arg("-c")
                .arg("echo out; echo err >&2; exit 3"),
            std::time::Duration::from_secs(30),
        )
        .unwrap()
        .expect("finished well inside its timeout");
        assert_eq!(output.stdout, b"out\n");
        assert_eq!(output.stderr, b"err\n");
        assert_eq!(output.status.code(), Some(3));
    }

    #[test]
    fn feature_dir_cmp_compares_a_sequential_prefix_numerically() {
        // Past 999 the prefix grows a digit, and byte order would put
        // `1000-x` first.
        let mut names = vec!["1000-x".to_string(), "999-y".to_string()];
        names.sort_by(|a, b| feature_dir_cmp(a, b));
        assert_eq!(names, ["999-y", "1000-x"]);
    }

    #[test]
    fn parse_affected_files_ignores_a_table_inside_an_html_comment() {
        let plan = "# P\n\n## Affected Files\n\n<!-- Example:\n\n| File | Action |\n| --- | --- |\n| `src/example` | Create |\n\n-->\n\n| File | Action |\n| --- | --- |\n| `src/real` | Edit |\n";
        assert_eq!(parse_affected_files(plan), ["src/real"]);
    }

    #[test]
    fn feature_dir_cmp_leaves_a_sequential_only_corpus_in_byte_order() {
        // Three-digit prefixes make byte order and numeric order agree,
        // so an existing spec root is presented exactly as before.
        let names = ["000-a", "007-b", "051-c", "999-d"];
        let mut by_cmp = names.to_vec();
        by_cmp.sort_by(|a, b| feature_dir_cmp(a, b));
        let mut by_bytes = names.to_vec();
        by_bytes.sort_unstable();
        assert_eq!(by_cmp, by_bytes);
    }

    #[test]
    fn only_the_sequential_form_carries_a_sequential_number() {
        let sequential = parse_feature_dir("123-thing").expect("sequential parses");
        assert_eq!(sequential.sequential_number(), Some(123));

        // The misparse this type exists to prevent: `1234.1-thing` must not
        // read as feature 123.
        let branch = parse_feature_dir("1234.1-thing").expect("branch-scoped parses");
        assert_eq!(branch.sequential_number(), None);
    }

    #[test]
    fn template_candidates_orders_adopter_layout_first() {
        assert_eq!(
            template_candidates("specs", "spec.md"),
            [
                "specs/templates/spec.md".to_string(),
                "framework/templates/spec/spec.md".to_string(),
            ]
        );
        // Honors a configured spec root in the first candidate only.
        assert_eq!(
            template_candidates("governance", "plan.md")[0],
            "governance/templates/plan.md"
        );
    }

    #[test]
    fn list_scenario_files_matches_md_case_insensitively() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("scenarios");
        std::fs::create_dir_all(&dir).unwrap();
        for name in ["alpha.md", "BETA.MD", "notes.txt", "README"] {
            std::fs::write(dir.join(name), "x").unwrap();
        }
        std::fs::create_dir_all(dir.join("nested.md")).unwrap(); // a dir, excluded
        // Case-insensitive ORDER as well as case-insensitive extension
        // matching: `alpha.md` precedes `BETA.MD` (spec 046). A byte-order
        // sort would put every uppercase name first.
        assert_eq!(
            list_scenario_files(&dir),
            vec!["alpha.md".to_string(), "BETA.MD".to_string()]
        );
        // Absent directory degrades to empty.
        assert_eq!(
            list_scenario_files(&tmp.path().join("missing")),
            [] as [String; 0]
        );
    }

    #[test]
    fn scenario_name_cmp_breaks_case_ties_deterministically() {
        use std::cmp::Ordering;
        // Case-insensitive primary ordering.
        assert_eq!(scenario_name_cmp("alpha.md", "BETA.MD"), Ordering::Less);
        assert_eq!(scenario_name_cmp("BETA.MD", "alpha.md"), Ordering::Greater);
        // Names differing only in case are ordered by raw bytes rather than
        // left to read_dir's filesystem-dependent order.
        assert_eq!(scenario_name_cmp("ALPHA.MD", "alpha.md"), Ordering::Less);
        assert_eq!(scenario_name_cmp("alpha.md", "ALPHA.MD"), Ordering::Greater);
        assert_eq!(scenario_name_cmp("alpha.md", "alpha.md"), Ordering::Equal);
    }

    #[test]
    fn validate_no_traversal_accepts_normal_paths() {
        validate_no_traversal("specs/042-foo").unwrap();
        validate_no_traversal("a/b/c").unwrap();
        validate_no_traversal("specs/022-deterministic-runtime").unwrap();
    }

    #[test]
    fn validate_no_traversal_rejects_empty() {
        assert!(matches!(
            validate_no_traversal("").unwrap_err(),
            PrimitiveError::InvalidPath { .. }
        ));
    }

    // --- scenario a-path-argument-resolves-inside-its-base (spec 022) ------

    /// A project and a directory outside it, both real directories.
    #[cfg(unix)]
    fn project_and_outside() -> (tempfile::TempDir, tempfile::TempDir) {
        let project = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(outside.path().join("dir")).unwrap();
        std::fs::write(outside.path().join("file.md"), "outside\n").unwrap();
        (project, outside)
    }

    #[cfg(unix)]
    fn link(target: impl AsRef<Path>, at: impl AsRef<Path>) {
        if let Some(parent) = at.as_ref().parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::os::unix::fs::symlink(target, at).unwrap();
    }

    #[cfg(unix)]
    fn refused_through(err: &PrimitiveError, shown: &str) -> bool {
        matches!(err, PrimitiveError::InvalidPath { reason, .. }
            if reason.contains(&format!("the symlink `{shown}`")))
    }

    #[cfg(unix)]
    #[test]
    fn a_link_that_leaves_the_base_is_refused_and_named() {
        let (project, outside) = project_and_outside();
        link(
            outside.path().join("dir"),
            project.path().join("specs/001-x"),
        );
        let err = validate_contained(project.path(), "specs/001-x/spec.md").unwrap_err();
        assert!(refused_through(&err, "specs/001-x"), "{err:?}");
        let PrimitiveError::InvalidPath { path, .. } = &err else {
            unreachable!()
        };
        assert_eq!(path, "specs/001-x/spec.md");
    }

    #[cfg(unix)]
    #[test]
    fn a_link_that_stays_inside_the_base_is_followed_and_accepted() {
        let (project, _outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("specs/002-real")).unwrap();
        link("002-real", project.path().join("specs/001-x"));
        validate_contained(project.path(), "specs/001-x/spec.md").unwrap();
    }

    /// The comparison is by whole components: a sibling whose name extends the
    /// base's (`proj2` beside `proj`) is outside it.
    #[cfg(unix)]
    #[test]
    fn containment_compares_whole_components_not_a_string_prefix() {
        let parent = tempfile::tempdir().unwrap();
        let base = parent.path().join("proj");
        std::fs::create_dir_all(&base).unwrap();
        std::fs::create_dir_all(parent.path().join("proj2")).unwrap();
        link(parent.path().join("proj2"), base.join("sibling"));
        let err = validate_contained(&base, "sibling/x.md").unwrap_err();
        assert!(refused_through(&err, "sibling"), "{err:?}");
    }

    /// A link below the first link is checked too, and a later component
    /// reached through an inside link is judged where it lands.
    #[cfg(unix)]
    #[test]
    fn a_link_reached_through_an_inside_link_is_checked() {
        let (project, outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("real")).unwrap();
        link("real", project.path().join("alias"));
        link(outside.path().join("dir"), project.path().join("real/out"));
        let err = validate_contained(project.path(), "alias/out/x.md").unwrap_err();
        assert!(refused_through(&err, "alias/out"), "{err:?}");
    }

    /// A path not created yet is judged by its deepest existing ancestor, so
    /// a new file under a real directory passes and one under an outward link
    /// does not.
    #[cfg(unix)]
    #[test]
    fn a_missing_path_is_judged_by_its_deepest_existing_ancestor() {
        let (project, outside) = project_and_outside();
        validate_contained(project.path(), "specs/001-x/scenarios/new.md").unwrap();
        link(
            outside.path().join("dir"),
            project.path().join("specs/001-x/scenarios"),
        );
        let err = validate_contained(project.path(), "specs/001-x/scenarios/new.md").unwrap_err();
        assert!(refused_through(&err, "specs/001-x/scenarios"), "{err:?}");
    }

    #[cfg(unix)]
    #[test]
    fn a_dangling_link_is_judged_by_where_its_target_would_resolve() {
        let (project, outside) = project_and_outside();
        link(
            outside.path().join("gone/x.md"),
            project.path().join("out.md"),
        );
        let err = validate_contained(project.path(), "out.md").unwrap_err();
        assert!(refused_through(&err, "out.md"), "{err:?}");
        link("gone/x.md", project.path().join("in.md"));
        validate_contained(project.path(), "in.md").unwrap();
    }

    /// A dangling target that climbs through a directory not created yet is
    /// judged as the kernel would once it exists, not by its text: a `..`
    /// after a link leaves the link's target, not the link. Each input below
    /// gets the opposite answer from a textual reading of its target.
    #[cfg(unix)]
    #[test]
    fn a_dangling_target_climbing_through_a_missing_directory_is_judged_physically() {
        let (project, _outside) = project_and_outside();
        // A canonical base, so a textual answer cannot differ merely for
        // spelling the base differently (macOS `/var` against `/private/var`).
        let base = project.path().canonicalize().unwrap();
        // `here` is the base itself, so `here/..` is the base's parent: the
        // target lands outside, where its text would stay inside.
        link(".", base.join("here"));
        link("missing/../here/../escape", base.join("out"));
        let err = validate_contained(&base, "out/x.md").unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                if reason.contains("the symlink `out` resolves to")
                    && reason.contains(", outside ")),
            "{err:?}"
        );
        // `deep` is three directories down, so two `..` after it stay
        // inside: the target lands inside, where its text would leave.
        std::fs::create_dir_all(base.join("a/b/c")).unwrap();
        link("a/b/c", base.join("deep"));
        link("missing/../deep/../../x.md", base.join("in.md"));
        validate_contained(&base, "in.md").unwrap();
    }

    /// A staging path is absolute and strictly inside a staging root, spelled
    /// as the root is or canonically (macOS's `/var` is `/private/var`); the
    /// root itself is refused, and a link on the path that leaves its root is
    /// refused, the error naming the whole argument (spec 022 task 9). A
    /// second root is honoured as the first is, as macOS's per-user temp
    /// directory is beside `TMPDIR`'s.
    #[cfg(unix)]
    #[test]
    fn a_staging_path_lies_inside_a_staging_root() {
        let (staging, outside) = project_and_outside();
        let real = staging.path().canonicalize().unwrap();
        let alias_parent = tempfile::tempdir().unwrap();
        let alias = alias_parent.path().join("alias");
        link(&real, &alias);
        let other = tempfile::tempdir().unwrap();
        let roots = [other.path().to_path_buf(), alias.clone()];
        validate_in_staging(&roots, &alias.join("run/a.tar.gz").to_string_lossy()).unwrap();
        validate_in_staging(&roots, &real.join("run/a.tar.gz").to_string_lossy()).unwrap();
        for arg in [
            "run/a.tar.gz".to_string(),
            alias.join("run/../a").to_string_lossy().into_owned(),
            outside.path().join("a").to_string_lossy().into_owned(),
            alias.to_string_lossy().into_owned(),
        ] {
            let err = validate_in_staging(&roots, &arg).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidPath { path, .. } if *path == arg),
                "{arg}: {err:?}"
            );
        }
        let err = validate_in_staging(&roots, &alias.to_string_lossy()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                if reason.contains("names the system temp directory itself")),
            "{err:?}"
        );
        // A root inside another is refused as itself, whichever comes first,
        // and a root that cannot be resolved is skipped, not fatal.
        let inner = real.join("inner");
        std::fs::create_dir_all(&inner).unwrap();
        let nested = [alias.clone(), inner.clone()];
        let err = validate_in_staging(&nested, &inner.to_string_lossy()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                if reason.contains("names the system temp directory itself")),
            "{err:?}"
        );
        // However the inner root and the candidate are spelled, the candidate
        // is that root itself, not a path inside the outer root.
        for (spelled_roots, candidate) in [
            ([alias.clone(), alias.join("inner")], inner.clone()),
            ([alias.clone(), inner.clone()], alias.join("inner")),
        ] {
            let err =
                validate_in_staging(&spelled_roots, &candidate.to_string_lossy()).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                    if reason.contains("names the system temp directory itself")),
                "{candidate:?} under {spelled_roots:?}: {err:?}"
            );
        }
        // A root not created yet is still refused as itself, by its spelling.
        let missing = [alias_parent.path().join("not-yet")];
        let err = validate_in_staging(&missing, &missing[0].to_string_lossy()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                if reason.contains("names the system temp directory itself")),
            "{err:?}"
        );
        // A link to itself is a loop, which no resolution gets past.
        link("loop", alias_parent.path().join("loop"));
        let unresolvable = [alias_parent.path().join("loop"), alias.clone()];
        validate_in_staging(&unresolvable, &alias.join("run/a.tar.gz").to_string_lossy()).unwrap();
        link(outside.path().join("dir"), real.join("out"));
        let arg = alias.join("out/a.tar.gz").to_string_lossy().into_owned();
        let err = validate_in_staging(&roots, &arg).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { path, reason }
                if *path == arg && reason.contains("the symlink `out`")),
            "{err:?}"
        );
    }

    /// 048's analysis (2026-10-29, `BE-INPUT-004`): a `--{field}-json` file
    /// is read only from a staging directory, and a path outside one is
    /// refused naming the flag before it is opened. Killed by M5.
    #[test]
    fn a_cli_json_file_is_read_only_from_a_staging_directory() {
        let staging = tempfile::tempdir().unwrap();
        let inside = staging.path().join("entries.json");
        std::fs::write(&inside, "[]").unwrap();
        assert_eq!(
            read_cli_json_file("entries-json", &inside.to_string_lossy()).unwrap(),
            "[]"
        );
        let outside = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        for arg in [
            outside.to_string_lossy().into_owned(),
            "entries.json".into(),
        ] {
            let err = read_cli_json_file("entries-json", &arg).unwrap_err();
            assert!(err.starts_with("--entries-json: "), "{arg}: {err}");
        }
    }

    /// The staging roots follow `temp_dir`, each once. Off macOS on Unix an
    /// empty `temp_dir` (a `TMPDIR` set but empty) stands for `/tmp`, where
    /// GNU `mktemp -t` then puts its directory; a relative one is no root.
    #[test]
    fn staging_roots_follow_temp_dir_and_fall_back_to_tmp() {
        let temp = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let other = temp.join("other");
        assert_eq!(staging_roots_for(temp.clone(), None), vec![temp.clone()]);
        assert_eq!(
            staging_roots_for(temp.clone(), Some(temp.clone())),
            vec![temp.clone()]
        );
        assert_eq!(
            staging_roots_for(temp.clone(), Some(other.clone())),
            vec![temp, other]
        );
        let fallback = if cfg!(all(unix, not(target_os = "macos"))) {
            vec![PathBuf::from("/tmp")]
        } else {
            Vec::new()
        };
        assert_eq!(staging_roots_for(PathBuf::new(), None), fallback);
        assert_eq!(
            staging_roots_for(PathBuf::from("relative"), None),
            Vec::<PathBuf>::new()
        );
    }

    /// `getconf DARWIN_USER_TEMP_DIR` names the directory `mktemp -d -t`
    /// makes its directory in on macOS, whatever `TMPDIR` says: the root the
    /// staging check takes beside `temp_dir()` there. `TMPDIR` here points under
    /// `/tmp`, outside the per-user directory, so a `mktemp` that honoured it
    /// would fail the test.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_darwin_user_temp_dir_is_where_mktemp_t_makes_a_directory() {
        let dir = darwin_user_temp_dir().expect("getconf DARWIN_USER_TEMP_DIR");
        let elsewhere = tempfile::Builder::new()
            .prefix("ductus-tmpdir-")
            .tempdir_in("/tmp")
            .unwrap();
        let made = std::process::Command::new("/usr/bin/mktemp")
            .args(["-d", "-t", "ductus-XXXXXX"])
            .env("TMPDIR", elsewhere.path())
            .output()
            .unwrap();
        assert!(made.status.success(), "mktemp failed: {made:?}");
        let staging = PathBuf::from(String::from_utf8(made.stdout).unwrap().trim());
        let inside = staging.starts_with(&dir) || staging.starts_with(dir.canonicalize().unwrap());
        std::fs::remove_dir(&staging).unwrap();
        assert!(
            inside,
            "{} is not under {}",
            staging.display(),
            dir.display()
        );
    }

    /// A glob is checked as a path and its literal prefix walked, and so is a
    /// directory a path names, because markdownlint-cli2 follows a linked
    /// directory it meets while expanding either; a file is checked as a
    /// path, and a link beside it is not walked.
    #[cfg(unix)]
    #[test]
    fn a_glob_walks_its_literal_prefix_for_a_link_out() {
        let (project, outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("docs/clean")).unwrap();
        std::fs::create_dir_all(project.path().join("specs/001-x")).unwrap();
        link(
            outside.path().join("dir"),
            project.path().join("specs/001-x/notes"),
        );
        for pattern in ["specs/**/*.md", "**/*.md"] {
            let err = validate_glob_contained(project.path(), pattern).unwrap_err();
            assert!(
                refused_through(&err, "specs/001-x/notes"),
                "{pattern}: {err:?}"
            );
        }
        validate_glob_contained(project.path(), "docs/**/*.md").unwrap();
        // A directory is walked as a glob's prefix is, `.` among them: globby
        // expands a directory to everything under it.
        for directory in ["specs/001-x", "specs", "."] {
            let err = validate_glob_contained(project.path(), directory).unwrap_err();
            assert!(
                refused_through(&err, "specs/001-x/notes"),
                "{directory}: {err:?}"
            );
        }
        validate_glob_contained(project.path(), "docs/clean").unwrap();
        // A file is checked as a path: a link beside it is not its concern.
        std::fs::write(project.path().join("specs/001-x/spec.md"), "x\n").unwrap();
        validate_glob_contained(project.path(), "specs/001-x/spec.md").unwrap();
    }

    /// The forms markdownlint-cli2 or its globbing rewrites in ways the check
    /// does not model are refused, each named in the error.
    #[test]
    fn a_glob_form_the_check_does_not_model_is_refused() {
        let project = tempfile::tempdir().unwrap();
        let mut refused = vec![
            (":x.md", "`:`"),
            ("!x.md", "`!`"),
            ("#x.md", "`#`"),
            ("{..,x}/y.md", "`{`"),
            ("x}.md", "`}`"),
            ("@(x)/y.md", "`(`"),
            ("x).md", "`)`"),
            ("a|b.md", "`|`"),
        ];
        // On every platform: before one of `$()*+?[]^` it is fast-glob's
        // escape, which Rust on Windows would read as a separator.
        refused.push(("link\\x.md", "`\\`"));
        refused.push(("a\\$b.md", "`\\`"));
        for (pattern, named) in refused {
            let err = validate_glob_contained(project.path(), pattern).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                    if reason.contains(named) && reason.contains("does not model")),
                "{pattern}: {err:?}"
            );
        }
        validate_glob_contained(project.path(), "x [draft].md").unwrap();
    }

    /// After a `..` past a missing directory, the next name can be a link
    /// that exists, and it is followed: `missing/../out` lands where `out`
    /// points, not at `out` itself.
    #[cfg(unix)]
    #[test]
    fn a_link_reached_after_a_missing_directory_is_followed() {
        let (project, outside) = project_and_outside();
        let base = project.path().canonicalize().unwrap();
        link(outside.path().join("dir"), base.join("out"));
        link("missing/../out/x.md", base.join("dangling.md"));
        let err = validate_contained(&base, "dangling.md").unwrap_err();
        assert!(refused_through(&err, "dangling.md"), "{err:?}");
    }

    /// Every link followed counts against one budget for the whole
    /// resolution. Twelve links that each name the next twice, through a
    /// missing directory so the kernel cannot resolve them itself, need 4,096
    /// follows: refused at the budget, not resolved after exponential work.
    #[cfg(unix)]
    #[test]
    fn links_that_fan_out_share_one_hop_budget() {
        let (project, _outside) = project_and_outside();
        let base = project.path().canonicalize().unwrap();
        std::fs::create_dir_all(base.join("real")).unwrap();
        let chain = 12;
        for i in 0..chain {
            let next = format!("L{}", i + 1);
            link(
                format!("m/../{next}/../m/../{next}"),
                base.join(format!("L{i}")),
            );
        }
        link("real", base.join(format!("L{chain}")));
        let err = validate_contained(&base, "L0/x.md").unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                if reason.contains("too many levels of symbolic links")),
            "{err:?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_link_loop_is_refused_rather_than_followed() {
        let (project, _outside) = project_and_outside();
        link("b", project.path().join("a"));
        link("a", project.path().join("b"));
        let err = validate_contained(project.path(), "a/x.md").unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                if reason.contains("the symlink `a` cannot be resolved")),
            "{err:?}"
        );
    }

    /// The base is canonicalized too, so a project reached through a link (as
    /// macOS reaches every tempdir, `/var` → `/private/var`) still contains
    /// its own files.
    #[cfg(unix)]
    #[test]
    fn a_base_reached_through_a_link_contains_its_own_files() {
        let (project, outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("real/specs")).unwrap();
        link(project.path().join("real"), outside.path().join("via"));
        std::fs::create_dir_all(project.path().join("real/specs/002-y")).unwrap();
        link("002-y", project.path().join("real/specs/001-x"));
        validate_contained(&outside.path().join("via"), "specs/001-x/spec.md").unwrap();
    }

    #[test]
    fn the_lexical_screen_still_runs_first() {
        let tmp = tempfile::tempdir().unwrap();
        for rel in ["", "/etc", "a/../b"] {
            assert!(
                matches!(
                    validate_contained(tmp.path(), rel),
                    Err(PrimitiveError::InvalidPath { .. })
                ),
                "{rel}"
            );
        }
    }

    /// The walk finds an outward link under a fixed name the primitive opens
    /// itself, which checking the argument alone cannot see.
    #[cfg(unix)]
    #[test]
    fn the_feature_walk_refuses_an_outward_link_beneath_the_directory() {
        let (project, outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("specs/001-x/scenarios")).unwrap();
        validate_feature_path(project.path(), "specs/001-x").unwrap();
        link(
            outside.path().join("file.md"),
            project.path().join("specs/001-x/scenarios/a.md"),
        );
        validate_contained(project.path(), "specs/001-x").unwrap();
        let err = validate_feature_path(project.path(), "specs/001-x").unwrap_err();
        assert!(
            refused_through(&err, "specs/001-x/scenarios/a.md"),
            "{err:?}"
        );
    }

    /// An inside link to a directory is walked into, so an outward link behind
    /// it is found; a link back up the tree is walked once, not forever.
    #[cfg(unix)]
    #[test]
    fn the_feature_walk_follows_inside_links_once() {
        let (project, outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("specs/001-x")).unwrap();
        std::fs::create_dir_all(project.path().join("shared")).unwrap();
        link("../..", project.path().join("specs/001-x/up"));
        link("../../shared", project.path().join("specs/001-x/shared"));
        validate_feature_path(project.path(), "specs/001-x").unwrap();
        link(
            outside.path().join("file.md"),
            project.path().join("shared/out.md"),
        );
        let err = validate_feature_path(project.path(), "specs/001-x").unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
    }

    /// A dangling link that would land inside is left to the primitive, the
    /// case `check-artifact-size` reports as `artifact-unreadable`.
    #[cfg(unix)]
    #[test]
    fn the_feature_walk_leaves_an_inside_dangling_link_to_the_primitive() {
        let (project, _outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("specs/001-x/scenarios")).unwrap();
        link(
            "../gone.md",
            project.path().join("specs/001-x/scenarios/b.md"),
        );
        validate_feature_path(project.path(), "specs/001-x").unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn a_feature_argument_is_screened_before_it_meets_the_root() {
        let tmp = tempfile::tempdir().unwrap();
        // Joined to the root, `/etc` would read `specs//etc`, which `Path`
        // collapses to the relative `specs/etc`.
        assert!(matches!(
            validate_feature_argument(tmp.path(), "specs", "/etc"),
            Err(PrimitiveError::InvalidPath { .. })
        ));
    }

    #[cfg(unix)]
    #[test]
    fn resolved_relative_names_where_a_path_lands() {
        let (project, outside) = project_and_outside();
        std::fs::create_dir_all(project.path().join("framework")).unwrap();
        link("../framework", project.path().join("runtime/link"));
        link(
            outside.path().join("dir"),
            project.path().join("runtime/out"),
        );
        assert_eq!(
            resolved_relative(project.path(), "runtime/link/new.md").as_deref(),
            Some("framework/new.md")
        );
        assert_eq!(
            resolved_relative(project.path(), "runtime/real.md").as_deref(),
            Some("runtime/real.md")
        );
        assert_eq!(resolved_relative(project.path(), "runtime/out/x.md"), None);
    }

    #[test]
    fn validate_no_traversal_rejects_absolute_paths() {
        for bad in &["/etc/passwd", "/tmp/x"] {
            assert!(
                matches!(
                    validate_no_traversal(bad).unwrap_err(),
                    PrimitiveError::InvalidPath { .. }
                ),
                "expected rejection for {bad:?}"
            );
        }
    }

    #[test]
    fn validate_no_traversal_rejects_parent_components() {
        for bad in &["../foo", "specs/../target", "a/b/../c"] {
            assert!(
                matches!(
                    validate_no_traversal(bad).unwrap_err(),
                    PrimitiveError::InvalidPath { .. }
                ),
                "expected rejection for {bad:?}"
            );
        }
    }

    #[test]
    fn resolve_path_joins_relative_and_passes_absolute_through() {
        let repo = Path::new("/repo");
        assert_eq!(
            resolve_path(repo, "specs/001-basic/spec.md"),
            Path::new("/repo/specs/001-basic/spec.md")
        );
        assert_eq!(
            resolve_path(repo, "/tmp/x/spec.md"),
            Path::new("/tmp/x/spec.md")
        );
    }

    #[test]
    fn frontmatter_status_reads_string_status() {
        let content = "---\nstatus: planned\ndependencies: []\n---\n\n# X\n";
        assert_eq!(
            frontmatter_status(content, Path::new("spec.md")).as_deref(),
            Some("planned")
        );
    }

    #[test]
    fn frontmatter_status_collapses_unreadable_shapes_to_none() {
        for content in &[
            "# No frontmatter\n",                           // missing block
            "---\nstatus: [unterminated\n---\n# X\n",       // invalid YAML
            "---\ndependencies: []\n---\n# X\n",            // status missing
            "---\nstatus: [a, b]\ndependencies: []\n---\n", // non-string status
        ] {
            assert_eq!(
                frontmatter_status(content, Path::new("spec.md")),
                None,
                "expected None for {content:?}"
            );
        }
    }

    #[test]
    fn parse_checkbox_line_matches_mark_side_grammar() {
        // Accepted: the exact grammar find_checkbox_line recognizes.
        assert_eq!(
            checkbox::parse_checkbox_line("- [ ] pending item"),
            Some((false, "pending item".to_string()))
        );
        assert_eq!(
            checkbox::parse_checkbox_line("  - [x] done item"),
            Some((true, "done item".to_string()))
        );
        assert_eq!(
            checkbox::parse_checkbox_line("- [X] done upper"),
            Some((true, "done upper".to_string()))
        );
        // Bare checkbox (nothing after `]`) is a checkbox with empty text —
        // the mark side can flip it, so the read side must count it.
        assert_eq!(
            checkbox::parse_checkbox_line("- [ ]"),
            Some((false, String::new()))
        );
        // Rejected: divergent-grammar shapes the mark side cannot address.
        for not_a_checkbox in &[
            "- [-] partial",
            "- [x]no-space",
            "* [ ] star bullet",
            "- foo",
        ] {
            assert_eq!(
                checkbox::parse_checkbox_line(not_a_checkbox),
                None,
                "expected rejection for {not_a_checkbox:?}"
            );
        }
    }

    #[test]
    fn iter_numbered_headings_extracts_atx2_numbers() {
        let content = "# Title\n\n## 1. First\n\n## 2. Second\n\n## 3. Third\n\nNot a heading.\n";
        let nums: Vec<u32> = iter_task_numbers_at_levels(content, &[2]).collect();
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn iter_numbered_headings_skips_non_atx2() {
        let content =
            "# 99. Not counted\n\n## 1. Counted\n\n### 2. Not counted (level 3)\n\n## 2. Counted\n";
        let nums: Vec<u32> = iter_task_numbers_at_levels(content, &[2]).collect();
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn iter_numbered_headings_skips_fenced_blocks() {
        let content = "## 1. Real\n\n```text\n## 99. Fake\n```\n\n## 2. Real\n";
        let nums: Vec<u32> = iter_task_numbers_at_levels(content, &[2]).collect();
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn iter_numbered_headings_handles_non_numeric_headings() {
        let content = "## Setup\n\n## 1. First\n\n## 7. Seventh\n";
        let nums: Vec<u32> = iter_task_numbers_at_levels(content, &[2]).collect();
        assert_eq!(nums, vec![1, 7]);
    }

    #[test]
    fn section_lines_yields_section_body_until_sibling_heading() {
        let body = "## A\n\nline A1\nline A2\n\n## B\n\nline B1\n";
        let a = section_lines(body, "A");
        assert_eq!(a, vec!["", "line A1", "line A2", ""]);
        let b = section_lines(body, "B");
        assert_eq!(b, vec!["", "line B1"]);
    }

    #[test]
    fn section_lines_yields_nothing_for_absent_heading() {
        let body = "## Other\n\nx\n";
        assert_eq!(section_lines(body, "Missing"), [] as [&str; 0]);
    }

    #[test]
    fn section_lines_keeps_deeper_nested_headings_as_body_content() {
        // A `### nested` heading INSIDE `## A` is body content, not a
        // section boundary — section ends only at <= same-level heading.
        let body = "## A\n\n### nested\n\nx\n\n## B\n";
        let a = section_lines(body, "A");
        assert_eq!(a, vec!["", "### nested", "", "x", ""]);
    }

    #[test]
    fn section_lines_handles_repeated_heading() {
        // When the same heading appears more than once, lines from every
        // matching section are yielded in document order.
        let body = "## A\n\nfirst\n\n## B\n\nx\n\n## A\n\nsecond\n";
        let a = section_lines(body, "A");
        assert_eq!(a, vec!["", "first", "", "", "second"]);
    }

    #[test]
    fn section_line_indices_skips_comment_and_fence_content() {
        let body = "## A\n\n<!--\n- [ ] fake\n-->\n- [ ] real\n```\n- [ ] fenced\n```\n\n## B\n";
        let lines: Vec<&str> = body.lines().collect();
        // Comment and fence content (delimiter lines included) is skipped;
        // only the blank lines and the real checkbox line survive.
        assert_eq!(section_line_indices(&lines, "A"), vec![1, 5, 9]);
    }

    #[test]
    fn section_line_indices_ignores_headings_inside_comments() {
        // A sibling heading inside a comment must not close the section.
        let body = "## A\n\n<!--\n## B\n-->\n- [ ] still in A\n";
        let lines: Vec<&str> = body.lines().collect();
        assert_eq!(section_line_indices(&lines, "A"), vec![1, 5]);
    }

    #[test]
    fn section_line_indices_keeps_inline_comment_lines() {
        // A comment that opens and closes on the same line is inline — the
        // line is real content (documented SkipScanner delimiter behavior).
        let body = "## A\n- [ ] real <!-- note -->\n";
        let lines: Vec<&str> = body.lines().collect();
        assert_eq!(section_line_indices(&lines, "A"), vec![1]);
    }

    #[test]
    fn split_frontmatter_offset_matches_lf_opener() {
        let content = "---\nstatus: x\n---\nbody\n";
        let (fm, body, offset) =
            split_frontmatter_with_offset(content, Path::new("spec.md")).unwrap();
        assert_eq!(fm, "status: x");
        assert_eq!(body, "body\n");
        assert_eq!(offset, "---\n".len());
    }

    #[test]
    fn split_frontmatter_offset_matches_crlf_opener() {
        let content = "---\r\nstatus: draft\r\n---\r\n\r\nbody\r\n";
        let (fm, body, offset) =
            split_frontmatter_with_offset(content, Path::new("spec.md")).unwrap();
        assert_eq!(fm, "status: draft\r");
        assert_eq!(body, "\r\nbody\r\n");
        assert_eq!(offset, "---\r\n".len());
    }

    #[test]
    fn split_frontmatter_accepts_empty_block() {
        let (fm, body, offset) =
            split_frontmatter_with_offset("---\n---\nbody\n", Path::new("spec.md")).unwrap();
        assert_eq!(fm, "");
        assert_eq!(body, "body\n");
        assert_eq!(offset, "---\n".len());

        let (fm, body, offset) =
            split_frontmatter_with_offset("---\r\n---\r\n", Path::new("spec.md")).unwrap();
        assert_eq!(fm, "");
        assert_eq!(body, "");
        assert_eq!(offset, "---\r\n".len());
    }

    /// A closing fence on the file's last line needs no newline after it: a
    /// line is a line whether or not the file ends in one.
    #[test]
    fn split_frontmatter_accepts_a_closing_fence_ending_the_file() {
        let path = Path::new("spec.md");
        assert_eq!(
            split_frontmatter("---\nstatus: x\n---", path).unwrap(),
            ("status: x", "")
        );
        assert_eq!(
            split_frontmatter("---\r\nstatus: x\r\n---", path).unwrap(),
            ("status: x\r", "")
        );
        assert_eq!(split_frontmatter("---\n---", path).unwrap(), ("", ""));
        // An earlier closing fence still closes the block.
        assert_eq!(
            split_frontmatter("---\nstatus: x\n---\nbody\n---", path).unwrap(),
            ("status: x", "body\n---")
        );
    }

    /// A file that opens no block and one whose block never closes are two
    /// defects, and each error names its own.
    #[test]
    fn a_missing_and_an_unclosed_frontmatter_are_named_apart() {
        let path = Path::new("spec.md");
        let missing = split_frontmatter("# Title\n", path).unwrap_err();
        assert!(
            missing.to_string().contains("no leading `---`"),
            "{missing}"
        );
        assert!(matches!(missing, PrimitiveError::MissingFrontmatter { .. }));
        let unclosed = split_frontmatter("---\nstatus: x\n\n# Title\n", path).unwrap_err();
        assert!(unclosed.to_string().contains("never closes"), "{unclosed}");
        assert!(matches!(
            unclosed,
            PrimitiveError::UnclosedFrontmatter { .. }
        ));
    }

    #[test]
    fn parse_affected_files_handles_several_tables_in_one_section() {
        // A section may hold more than one table, each with its own header.
        // `saw_header` used to survive the first separator, so every later
        // header row was read as a data row and the literal "File" landed in
        // the path list — eight times, against spec 017's plan.
        let plan = "## Affected Files\n\n\
                    Templates:\n\n\
                    | File | Action |\n\
                    | --- | --- |\n\
                    | `framework/templates/spec/spec.md` | Edit |\n\n\
                    Scripts:\n\n\
                    | File | Action |\n\
                    | --- | --- |\n\
                    | `scripts/gen-help-tables.sh` | Create |\n";
        let paths = parse_affected_files(plan);
        assert_eq!(
            paths,
            vec![
                "framework/templates/spec/spec.md".to_string(),
                "scripts/gen-help-tables.sh".to_string(),
            ],
            "a header row must never be parsed as a path"
        );
        assert!(!paths.iter().any(|p| p == "File"));
    }

    #[test]
    fn parse_affected_files_takes_the_backticked_path_from_a_qualified_cell() {
        // A cell may carry a qualifier after the path. Trimming stray
        // backticks off the whole cell kept the trailing prose and produced
        // "constitution.md` (root)", which resolves to nothing.
        let plan = "## Affected Files\n\n\
                    | File | Action |\n\
                    | --- | --- |\n\
                    | `constitution.md` (root) | Edit |\n\
                    | `specs/000-016/spec.md` (and one `spec-and-plan.md` if any) | Edit |\n";
        assert_eq!(
            parse_affected_files(plan),
            vec![
                "constitution.md".to_string(),
                "specs/000-016/spec.md".to_string(),
            ]
        );
    }

    #[test]
    fn parse_affected_files_extracts_first_column_paths() {
        let plan = "# Plan\n\n\
                    ## Affected Files\n\n\
                    | File | Action | Purpose |\n\
                    | --- | --- | --- |\n\
                    | `runtime/src/foo.rs` | Create | Foo |\n\
                    | `runtime/src/bar.rs` | Edit | Bar |\n\
                    | scripts/baz.sh | Create | Baz |\n\n\
                    ## Trade-offs\n\nIrrelevant.\n";
        // (the multi-table and qualified-cell cases are pinned separately below)
        let paths = parse_affected_files(plan);
        assert_eq!(
            paths,
            vec![
                "runtime/src/foo.rs".to_string(),
                "runtime/src/bar.rs".to_string(),
                "scripts/baz.sh".to_string()
            ]
        );
    }

    #[test]
    fn parse_affected_files_handles_missing_section() {
        let plan = "# Plan\n\n## Trade-offs\n\nNo affected files.\n";
        let paths = parse_affected_files(plan);
        assert_eq!(paths, [] as [String; 0]);
    }

    #[test]
    fn parse_affected_files_ignores_table_inside_fenced_block() {
        let plan = "# Plan\n\n\
                    ## Affected Files\n\n\
                    ```text\n\
                    | not | a | table |\n\
                    | --- | --- | --- |\n\
                    | `nope.md` | Create | Fake |\n\
                    ```\n\n\
                    | File | Action | Purpose |\n\
                    | --- | --- | --- |\n\
                    | `real.md` | Create | Real |\n";
        let paths = parse_affected_files(plan);
        assert_eq!(paths, vec!["real.md".to_string()]);
    }

    #[test]
    fn is_feature_slug_accepts_canonical_form() {
        for slug in &[
            "022-deterministic-runtime",
            "000-blocker",
            "999-foo",
            "1000-foo",
        ] {
            assert!(is_feature_slug(slug), "expected acceptance for {slug:?}");
        }
    }

    #[test]
    fn is_feature_slug_rejects_non_pattern() {
        for bad in &[
            "templates",
            "inbox.md",
            ".hidden",
            "022",
            "abc-something",
            "22-too-short",
            "0220-too-long-prefix", // four digits with a leading zero: not a `{number:03}` name
        ] {
            assert!(!is_feature_slug(bad), "expected rejection for {bad:?}");
        }
    }

    #[test]
    fn task_walkers_ignore_html_comment_headings() {
        // The tasks.md template guidance comment embeds `## 1.` example
        // headings; they must not be counted as tasks or flip structure
        // detection.
        let content = "# T\n\nIntro.\n\n<!-- Example:\n## 1. Not a task\n\n- [ ] not a subtask\n### 2. Also not\n-->\n\n## 1. Real task\n\n- [ ] real\n";
        let nums: Vec<u32> = iter_task_numbers_at_levels(content, &[2, 3]).collect();
        assert_eq!(
            nums,
            vec![1],
            "only the real `## 1.` outside the comment counts"
        );
        assert_eq!(detect_tasks_structure(content), TasksStructure::Flat);

        // A pure-comment example (no real tasks) yields nothing.
        let only_comment = "# T\n\n<!-- \n## 1. X\n### 2. Y\n-->\n";
        assert!(
            iter_task_numbers_at_levels(only_comment, &[2, 3])
                .next()
                .is_none()
        );

        // An inline self-closing comment on a heading line does not hide it.
        let inline = "## 3. Real <!-- note -->\n\n- [ ] x\n";
        let inline_nums: Vec<u32> = iter_task_numbers_at_levels(inline, &[2, 3]).collect();
        assert_eq!(inline_nums, vec![3]);
    }

    #[test]
    fn backticked_comment_delimiter_in_prose_does_not_hide_following_structure() {
        // scenarios/skipscanner-inline-code-exemption.md: a task's done-when
        // that mentions a comment-open delimiter inside backticks must not
        // open a skip region and hide the next task heading — the exact bug
        // task 67's done-when hit.
        let content = "## 1. First\n\n- **Done when**: handles a backticked `<!--` in prose.\n\n## 2. Second\n\n- [ ] x\n";
        let nums: Vec<u32> = iter_task_numbers_at_levels(content, &[2]).collect();
        assert_eq!(
            nums,
            vec![1, 2],
            "a backticked comment-open delimiter must not hide the following `## 2.`"
        );

        // Balanced backticked delimiters on one line are inert too.
        let balanced = "## 1. First\n\nMentions `<!--` and `-->` in code font.\n\n## 2. Second\n";
        let bnums: Vec<u32> = iter_task_numbers_at_levels(balanced, &[2]).collect();
        assert_eq!(bnums, vec![1, 2]);

        // A genuine (unbackticked) comment still hides its contents.
        let real = "## 1. First\n\n<!-- comment\n## 9. hidden\n-->\n\n## 2. Second\n";
        let rnums: Vec<u32> = iter_task_numbers_at_levels(real, &[2]).collect();
        assert_eq!(rnums, vec![1, 2], "the real comment still hides `## 9.`");
    }

    #[test]
    fn find_outside_code_ignores_delimiters_inside_backticks() {
        // Inside a single-backtick span → not found.
        assert_eq!(find_outside_code("a `<!--` b", "<!--"), None);
        // In ordinary text → found at its offset.
        assert_eq!(find_outside_code("a <!-- b", "<!--"), Some(2));
        // Double-backtick span containing the token still shields it.
        assert_eq!(find_outside_code("x ``<!--`` y", "<!--"), None);
        // A delimiter outside any span is found even when another sits inside one.
        assert_eq!(find_outside_code("`<!--` then <!--", "<!--"), Some(12));
    }

    // --- split_blocks ---------------------------------------------------------

    /// `(line, text)` pairs, the shape assertions read most clearly in.
    fn blocks(content: &str) -> Vec<(usize, String)> {
        split_blocks(content)
            .into_iter()
            .map(|b| (b.line, b.text))
            .collect()
    }

    #[test]
    fn each_block_kind_is_split_and_line_numbered() {
        let doc = "# Heading\n\
                   \n\
                   A paragraph that\n\
                   wraps onto two lines.\n\
                   \n\
                   - first bullet\n\
                   - second bullet\n\
                   \n\
                   | a | b |\n\
                   | --- | --- |\n";
        assert_eq!(
            blocks(doc),
            vec![
                (3, "A paragraph that\nwraps onto two lines.".to_string()),
                (6, "- first bullet".to_string()),
                (7, "- second bullet".to_string()),
                (9, "| a | b |".to_string()),
                (10, "| --- | --- |".to_string()),
            ],
            "headings are structure, not claims, and never yield a block"
        );
    }

    /// The reading the body family takes: a heading is its own block, and a
    /// blockquote paragraph is one — wrapped `>` lines joined, an empty `>`
    /// line ending it, a list item inside it starting a new one. A plain line
    /// after a quote resumes as a paragraph, as under the default reading,
    /// which drops the quote and the heading.
    #[test]
    fn a_reading_split_keeps_quotes_and_headings() {
        let doc = "## Notes\n\
                   \n\
                   > **Note:** a later spec retired\n\
                   > the hook,\n\
                   > - **AC7** named it\n\
                   >   on two lines.\n\
                   after the quote.\n\
                   >\n\
                   > A second paragraph.\n";
        let reading = BlockReading {
            quotes: true,
            headings: true,
        };
        let read: Vec<(usize, String)> = split_blocks_reading(doc, reading)
            .into_iter()
            .map(|b| (b.line, b.text))
            .collect();
        assert_eq!(
            read,
            vec![
                (1, "## Notes".to_string()),
                (
                    3,
                    "> **Note:** a later spec retired\n> the hook,".to_string()
                ),
                (5, "> - **AC7** named it\n>   on two lines.".to_string()),
                (7, "after the quote.".to_string()),
                (9, "> A second paragraph.".to_string()),
            ]
        );
        assert_eq!(blocks(doc), vec![(7, "after the quote.".to_string())]);
        assert_eq!(unquote(">   on two lines."), "on two lines.");
        assert_eq!(unquote("> > nested"), "nested");
    }

    #[test]
    fn every_exempt_context_is_dropped() {
        // One tell per exempt context plus one in live prose: only the live
        // one survives as block text (spec 045, AC13). The inline-code case
        // is intra-line and is asserted through `inline_code_spans` below.
        let doc = "```\nstill open in a fence\n```\n\
                   \n\
                   <!-- still open in a comment -->\n\
                   \n\
                   > still open in a blockquote\n\
                   \n\
                   still open in live prose\n";
        assert_eq!(
            blocks(doc),
            vec![(9, "still open in live prose".to_string())]
        );
    }

    #[test]
    fn an_inline_comment_is_stripped_but_its_line_survives() {
        // `SkipScanner` reports a line whose comment opens and closes inline
        // as *not* skipped, because the surrounding content is real markdown.
        // The commented text must still not reach a block.
        assert_eq!(
            blocks("real prose <!-- still open --> more prose\n"),
            vec![(1, "real prose  more prose".to_string())]
        );
        // A line that is nothing but a comment leaves no block at all.
        assert_eq!(blocks("<!-- still open -->\n"), [] as [(usize, String); 0]);
        // A delimiter inside a code span is inert, so nothing is stripped.
        assert_eq!(
            blocks("prose about `<!--` markers\n"),
            vec![(1, "prose about `<!--` markers".to_string())]
        );
    }

    #[test]
    fn an_unterminated_region_swallows_everything_after_it() {
        let fence = "before\n\n```\nafter the opener\n";
        assert_eq!(blocks(fence), vec![(1, "before".to_string())]);
        let comment = "before\n\n<!-- opener\nafter the opener\n";
        assert_eq!(blocks(comment), vec![(1, "before".to_string())]);
    }

    #[test]
    fn a_marker_ends_the_open_block_without_a_blank_line() {
        let doc = "a paragraph\n- a bullet right after it\n  continued\n1. an ordered item\n";
        assert_eq!(
            blocks(doc),
            vec![
                (1, "a paragraph".to_string()),
                (2, "- a bullet right after it\n  continued".to_string()),
                (4, "1. an ordered item".to_string()),
            ]
        );
    }

    #[test]
    fn a_nested_bullet_is_its_own_block() {
        // A claim in a sub-bullet is scoped to that sub-bullet, not merged
        // into its parent's.
        let doc = "- parent\n  - child\n";
        assert_eq!(
            blocks(doc),
            vec![(1, "- parent".to_string()), (2, "  - child".to_string()),]
        );
    }

    #[test]
    fn a_table_row_inside_a_list_item_is_a_row() {
        let doc = "- intro\n| a | b |\n";
        assert_eq!(
            blocks(doc),
            vec![(1, "- intro".to_string()), (2, "| a | b |".to_string())]
        );
    }

    #[test]
    fn a_blockquote_interrupting_a_paragraph_ends_it() {
        let doc = "before\n> quoted\nafter\n";
        assert_eq!(
            blocks(doc),
            vec![(1, "before".to_string()), (3, "after".to_string())]
        );
    }

    #[test]
    fn a_document_with_no_block_content_yields_nothing() {
        assert_eq!(
            blocks("# Only\n\n## Headings\n"),
            [] as [(usize, String); 0]
        );
        assert_eq!(blocks(""), [] as [(usize, String); 0]);
    }

    #[test]
    fn inline_code_spans_locate_the_fourth_exempt_context() {
        // The intra-line half of AC13: a term inside backticks is inside a
        // span, the same term outside them is not.
        let line = "the `still open` tell versus still open prose";
        let spans = inline_code_spans(line);
        let inside = line.find("still open").unwrap();
        let outside = line.rfind("still open").unwrap();
        assert!(spans.iter().any(|s| s.contains(&inside)));
        assert!(!spans.iter().any(|s| s.contains(&outside)));
    }

    /// Single-line text is plain text: a control character other than tab, a
    /// Unicode line break, or a noncharacter is refused — the YAML reader
    /// refuses a raw control character or noncharacter, so one stored in a
    /// record makes the record unreadable (spec 058). Tab stays legal.
    #[test]
    fn single_line_text_refuses_control_characters_and_line_breaks() {
        for bad in [
            "a\nb",
            "a\rb",
            "a\u{1b}b",
            "a\u{7f}b",
            "a\u{85}b",
            "a\u{2028}b",
            "a\u{2029}b",
            "a\u{fffe}b",
            "a\u{ffff}b",
        ] {
            assert!(
                validate_single_line("p", "arg", bad).is_err(),
                "{bad:?} must be refused"
            );
        }
        assert!(validate_single_line("p", "arg", "a\tb — `c`: d").is_ok());
        assert_eq!(flatten_line(" a\u{1b}b\nc\u{2028}d\t "), "a b c d");
        assert_eq!(flatten_line("non\u{ffff}char"), "non char");
    }
}

#[cfg(test)]
mod spec_corpus_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{
        git_fixture, is_spec_path, list_staged_specs, list_tracked_specs, list_untracked_specs,
        spec_feature_slug, tracked_project_paths,
    };
    use std::path::Path;

    #[test]
    fn spec_path_recognition_is_scoped_to_the_root_and_shape() {
        assert!(is_spec_path("specs/001-a/spec.md", "specs"));
        assert!(is_spec_path("specs/001-a/spec-and-plan.md", "specs"));
        assert!(!is_spec_path("specs/001-a/plan.md", "specs"));
        assert!(!is_spec_path("specs/001-a/scenarios/x.md", "specs"));
        assert!(!is_spec_path("specs/inbox.md", "specs"));
        assert!(!is_spec_path("other/001-a/spec.md", "specs"));
        assert!(!is_spec_path("specs/not-a-feature/spec.md", "specs"));
        // A non-default root (spec 040) is honored, and the default is not.
        assert!(is_spec_path("governance/001-a/spec.md", "governance"));
        assert!(!is_spec_path("specs/001-a/spec.md", "governance"));
    }

    #[test]
    fn feature_slug_is_the_directory_component() {
        assert_eq!(
            spec_feature_slug("specs/022-deterministic-runtime/spec.md", "specs").as_deref(),
            Some("022-deterministic-runtime")
        );
        assert_eq!(spec_feature_slug("other/001-a/spec.md", "specs"), None);
    }

    // --- spec listings -----------------------------------------------------------

    fn spec_body(title: &str) -> String {
        format!("---\nstatus: draft\ndependencies: []\n---\n\n# {title}\n")
    }

    /// A repository whose index cannot be read is not the no-repository case.
    /// `list_tracked_specs` fell back to the worktree walk there and so listed
    /// an untracked draft the index exists to exclude; the other two listings
    /// answered empty, as though they had looked (QUAL-CLAIM-001).
    #[test]
    fn an_unreadable_index_is_an_error_for_every_listing() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        git_fixture::write(&tmp.path().join("specs/001-a/spec.md"), &spec_body("A"));
        git_fixture::commit_all(&repository, "init");
        git_fixture::write(
            &tmp.path().join("specs/002-draft/spec.md"),
            &spec_body("Draft"),
        );
        std::fs::write(tmp.path().join(".git/index"), "not an index").unwrap();

        assert!(list_tracked_specs(tmp.path(), "specs").is_err());
        assert!(list_untracked_specs(tmp.path(), "specs").is_err());
        assert!(list_staged_specs(tmp.path(), "specs").is_err());
    }

    /// A project whose path from the work tree holds a glob character. As a
    /// glob the pathspec matched no file, so no untracked draft was reported
    /// and, once the staged listing is bounded the same way, no staged spec.
    #[test]
    fn a_project_path_with_a_glob_character_lists_its_specs_literally() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let project = tmp.path().join("app[v2]");
        git_fixture::write(&project.join("specs/001-a/spec.md"), &spec_body("A"));
        git_fixture::commit_all(&repository, "init");
        git_fixture::write(
            &project.join("specs/002-draft/spec.md"),
            &spec_body("Draft"),
        );
        git_fixture::write(
            &project.join("specs/001-a/spec.md"),
            &spec_body("A, edited"),
        );
        let mut index = repository.index().unwrap();
        index
            .add_path(Path::new("app[v2]/specs/001-a/spec.md"))
            .unwrap();
        index.write().unwrap();

        assert_eq!(
            list_tracked_specs(&project, "specs").unwrap(),
            vec!["specs/001-a/spec.md".to_string()]
        );
        assert_eq!(
            list_untracked_specs(&project, "specs").unwrap(),
            vec!["specs/002-draft/spec.md".to_string()]
        );
        assert_eq!(
            list_staged_specs(&project, "specs")
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["specs/001-a/spec.md".to_string()]
        );
    }

    /// An index in conflict holds one entry per stage for the same path; the
    /// tracked-paths walk names each path once.
    #[test]
    fn tracked_project_paths_names_a_conflicted_path_once() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        git_fixture::write(&tmp.path().join("a.md"), "base\n");
        git_fixture::commit_all(&repository, "init");
        let mut index = repository.index().unwrap();
        let entry = index.get_path(Path::new("a.md"), 0).unwrap();
        index.remove_path(Path::new("a.md")).unwrap();
        for stage in 1..=3_u16 {
            let staged = git2::IndexEntry {
                ctime: entry.ctime,
                mtime: entry.mtime,
                dev: entry.dev,
                ino: entry.ino,
                mode: entry.mode,
                uid: entry.uid,
                gid: entry.gid,
                file_size: entry.file_size,
                id: entry.id,
                flags: (entry.flags & !0x3000) | (stage << 12),
                flags_extended: entry.flags_extended,
                path: entry.path.clone(),
            };
            index.add(&staged).unwrap();
        }
        index.write().unwrap();
        assert!(repository.index().unwrap().has_conflicts());

        let tracked = tracked_project_paths(tmp.path()).unwrap().unwrap();
        assert_eq!(tracked, vec!["a.md".to_string()]);
    }
}
