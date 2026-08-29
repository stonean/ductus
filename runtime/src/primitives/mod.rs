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

pub mod append_inbox;
pub mod append_question;
pub mod append_task;
pub mod apply_manifest;
pub mod check_artifacts;
pub mod check_command_flags;
pub mod check_orphaned_references;
pub mod check_review_agreement;
pub mod check_review_gate;
pub mod check_rule_ids;
pub mod check_stuck;
pub mod check_unfolded_specs;
pub mod compute_review_scope;
pub mod create_feature;
pub mod create_plan_artifacts;
pub mod create_scenario;
pub mod dashboard;
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
pub mod label_criteria;
pub mod lint_markdown;
pub mod mark_criterion;
pub mod mark_task;
pub mod mechanical_sweep;
pub mod merge_managed_block;
pub mod merge_permissions;
pub mod migrate_session_file;
pub mod process_waivers;
pub mod prune_tasks;
pub mod read_spec;
pub mod read_tasks;
pub mod remove_inbox_item;
pub mod resolve_anchor;
pub mod resolve_feature;
pub mod resolve_references;
pub mod retire_feature;
pub mod rewrite_spec_links;
pub mod run_generator;
pub mod set_status;
pub(crate) mod spec_links;
pub mod traverse_deps;
pub mod validate_frontmatter;
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
    /// Archive entry path escapes the destination directory (`..`, absolute).
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
    /// Manifest entry referenced an unknown strategy. Valid values are
    /// `update`, `create`, and `skip-if-conflict`.
    #[error(
        "unknown manifest strategy '{strategy}' (expected 'update', 'create', or 'skip-if-conflict')"
    )]
    UnknownManifestStrategy {
        /// Strategy string as it appeared in the manifest entry.
        strategy: String,
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
    /// TOML parse failure (e.g., `dashboard` reading a malformed
    /// `.govern.toml`).
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
    /// this so the command can direct the user to run the plan phase.
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
}

/// Convenience alias for primitive return values.
pub type Result<T> = std::result::Result<T, PrimitiveError>;

/// Split a markdown file's content into its frontmatter YAML block and the
/// body that follows. Returns an error if no `---` opening fence is present
/// or no closing fence is found.
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
    Err(PrimitiveError::MissingFrontmatter { path: path.into() })
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
/// [`validate_no_traversal`]: primitives whose path arguments are
/// operator/machine-local (fixture specs in temp dirs, generator scripts,
/// downloaded archives, sibling-service checkouts) resolve through this
/// helper; primitives that must stay inside the repo root
/// (`merge-managed-block`, `merge-permissions`, …) call
/// [`validate_no_traversal`] first and never accept absolute input.
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

/// Reject caller-supplied paths that contain parent-directory components
/// (`..`) or absolute prefixes — the BE-INPUT-004 defense-in-depth check.
/// Primitives that accept paths from the host or LLM call this before any
/// filesystem operation to guarantee the resolved path stays inside the
/// repo root.
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

/// Reject a text value carrying an embedded newline or carriage return.
/// Such a value, interpolated verbatim into a markdown or YAML artifact,
/// would inject document structure (a phantom heading, a new frontmatter
/// key); `primitive`/`argument` name the offending field. Shared by every
/// primitive that splices caller-supplied text into a file it writes.
pub(crate) fn validate_single_line(primitive: &str, argument: &str, value: &str) -> Result<()> {
    if value.contains('\n') || value.contains('\r') {
        return Err(PrimitiveError::InvalidArgument {
            primitive: primitive.into(),
            argument: argument.into(),
            reason: "embedded newlines would inject document structure; \
                     supply single-line text"
                .into(),
        });
    }
    Ok(())
}

/// Extract an inbox/list bullet line's text: the trimmed content after the
/// `- ` marker, with an optional task-list checkbox (`[ ]`/`[x]`/`[X]`)
/// stripped via the shared [`checkbox::parse_checkbox_line`] grammar so the
/// plain `- text` and the checkbox `- [ ] text` forms both resolve to their
/// content. `None` for a non-bullet line. Shared by `append-inbox` (dedup
/// match) and `remove-inbox-item` (removal match) so the two agree on bullet
/// identity.
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
/// write side and `append-inbox`'s dedup read side can never disagree about
/// which inputs carry a marker. Strips **one** marker: the failure mode is a
/// single doubling, and stripping to exhaustion would eat legitimate content
/// from text that genuinely begins with a dash.
pub(crate) fn strip_bullet_marker(text: &str) -> String {
    bullet_text(text).unwrap_or_else(|| text.trim().to_string())
}

/// Iterate the real inbox/list bullets of `content` as `(line_index, text)`,
/// skipping fenced code blocks and HTML-comment regions via [`SkipScanner`].
/// The inbox template embeds `- ` lines inside its `<!-- Rules: … -->`
/// guidance comment; without comment-awareness those would be miscounted as
/// items and could even be matched for removal. Shared by `append-inbox`
/// (dedup + count) and `remove-inbox-item` (removal + count) so both agree
/// on which lines are real bullets.
pub(crate) fn iter_bullets(content: &str) -> impl Iterator<Item = (usize, String)> + '_ {
    let mut skip = SkipScanner::default();
    content.lines().enumerate().filter_map(move |(idx, line)| {
        if skip.skip(line) {
            return None;
        }
        bullet_text(line).map(|text| (idx, text))
    })
}

/// Count the real (comment/fence-aware) inbox/list bullets in `content`.
pub(crate) fn count_inbox_bullets(content: &str) -> u32 {
    u32::try_from(iter_bullets(content).count()).unwrap_or(u32::MAX)
}

/// Validate a caller-supplied slug against the framework slug grammar
/// `^[a-z0-9]+(?:-[a-z0-9]+)*$`: one or more lowercase-alphanumeric
/// segments joined by single hyphens — exactly the alphabet
/// `create_feature::derive_slug` emits. This is an allowlist
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
/// `read-tasks`, `mark-task`, `prune-tasks`, and the task-number walkers, so
/// teaching it a fourth region would change how each of them reads a quoted
/// task line. (Inline code spans are the fourth exempt context, but they are
/// an *intra-line* concern — see [`inline_code_spans`] — so a consumer
/// applies them to a block's text rather than the splitter dropping lines.)
pub(crate) fn split_blocks(content: &str) -> Vec<MarkdownBlock> {
    let mut skip = SkipScanner::default();
    let mut blocks: Vec<MarkdownBlock> = Vec::new();
    let mut open: Option<(usize, Vec<String>)> = None;

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
        if skipped
            || trimmed.is_empty()
            || trimmed.starts_with('>')
            || parse_atx_heading(&content_line).is_some()
        {
            close_block(&mut blocks, open.take());
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
            open = Some((idx + 1, vec![content_line.into_owned()]));
            continue;
        }
        match &mut open {
            // A non-marker line continues whatever is open — the indented
            // and lazy continuation forms are the same claim either way.
            Some((_, lines)) => lines.push(content_line.into_owned()),
            None => open = Some((idx + 1, vec![content_line.into_owned()])),
        }
    }
    close_block(&mut blocks, open.take());
    blocks
}

/// Push the currently-open block, if any, onto `blocks`.
fn close_block(blocks: &mut Vec<MarkdownBlock>, open: Option<(usize, Vec<String>)>) {
    if let Some((line, lines)) = open {
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
/// Shared between `read_spec::parse_open_questions` (returns
/// `Vec<OpenQuestion>`) and `dashboard::{count_open_questions,
/// context_summary}` (return a `u32` count and a `String` summary
/// respectively). The iteration semantics are the single source of
/// truth for "lines inside section X"; consumers diverge only in how
/// they fold the yielded lines into their result shape.
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
    /// `NNN-slug` — three ASCII digits, a hyphen, and a slug.
    Sequential {
        /// The three-digit prefix as a number.
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

/// List feature directories (`NNN-slug`) under the spec root, sorted by
/// name. Best-effort: a missing or unreadable spec root yields an empty
/// list — a repo without a spec root has no features by definition, and
/// the primitives that consume this (`resolve-feature`, `create-feature`,
/// `dashboard`, and `interpreter::payload`'s inbox router) all report the
/// empty case as "no features" rather than an operational error.
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

/// Feature-spec paths tracked by git, repo-relative, sorted.
///
/// Scoped to the git index rather than a worktree glob so an untracked
/// in-progress draft is never rewritten and never enters a derived index
/// (spec 017, `tracked-specs-not-worktree`). Falls back to a worktree walk
/// only outside a git repo, where there is no index.
pub(crate) fn list_tracked_specs(repo: &Path, specs_root: &str) -> Vec<String> {
    if let Ok(repository) = git2::Repository::open(repo)
        && let Ok(index) = repository.index()
    {
        let mut out: Vec<String> = index
            .iter()
            .filter_map(|entry| String::from_utf8(entry.path).ok())
            .filter(|path| is_spec_path(path, specs_root))
            .collect();
        out.sort();
        out.dedup();
        return out;
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
    out
}

/// Feature-spec paths present in the worktree but not tracked by git —
/// exactly the set [`list_tracked_specs`] excludes by design.
///
/// Exists so a generator can report what it did *not* examine. A zero rewrite
/// count means "I rewrote nothing", not "everything is in sync": an untracked
/// draft is never enumerated, so a bare in-sync claim would assert a property
/// of files the generator cannot vouch for. Empty outside a git repo, where
/// the fallback already walks everything.
pub(crate) fn list_untracked_specs(repo: &Path, specs_root: &str) -> Vec<String> {
    let Ok(repository) = git2::Repository::open(repo) else {
        return Vec::new();
    };
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false)
        // Bound the walk to the spec tree. Without a pathspec this is a full
        // worktree status on every run — including the pre-commit path, where
        // it would scan build output and vendored trees to answer a question
        // only about `{specs-root}/`.
        .pathspec(specs_root);
    let Ok(statuses) = repository.statuses(Some(&mut opts)) else {
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for entry in statuses.iter() {
        if !entry.status().contains(git2::Status::WT_NEW) {
            continue;
        }
        // `path()` errors on a non-UTF-8 path, which cannot be a spec under
        // the validated slug grammar anyway.
        let Ok(path) = entry.path() else { continue };
        if is_spec_path(path, specs_root) {
            out.push(path.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Feature-spec paths staged in the index for the pending commit — the
/// `--staged` rewrite set, so committing one spec never rewrites the derived
/// frontmatter of unrelated specs. Empty outside a git repo.
pub(crate) fn list_staged_specs(
    repo: &Path,
    specs_root: &str,
) -> std::collections::BTreeSet<String> {
    let Ok(repository) = git2::Repository::open(repo) else {
        return std::collections::BTreeSet::new();
    };
    // HEAD tree against the index. An unborn HEAD (no commits yet) diffs the
    // index against nothing, which is the correct "everything staged" answer.
    let head_tree = repository
        .head()
        .ok()
        .and_then(|head| head.peel_to_tree().ok());
    let Ok(diff) = repository.diff_tree_to_index(head_tree.as_ref(), None, None) else {
        return std::collections::BTreeSet::new();
    };
    let mut out = std::collections::BTreeSet::new();
    for delta in diff.deltas() {
        for file in [delta.new_file(), delta.old_file()] {
            if let Some(path) = file.path().and_then(Path::to_str)
                && is_spec_path(path, specs_root)
            {
                out.insert(path.to_string());
            }
        }
    }
    out
}

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
/// `1234.2-x` lexicographically. Sequential names are unaffected — their
/// prefix is exactly three digits, so byte order and numeric order agree
/// — and a corpus holding only sequential directories comes back in the
/// order it always did.
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
/// `{specs-root}/templates/{file}` (what `/ductus:init` scaffolds and the
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
pub(crate) fn parse_affected_files(plan_content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_section = false;
    let mut in_fence = false;
    let mut saw_header = false;
    for line in plan_content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
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
            "007-a".to_string(),
        ];
        names.sort_by(|a, b| feature_dir_cmp(a, b));
        assert_eq!(
            names,
            vec![
                // Sequential first, by number.
                "007-a".to_string(),
                "051-b".to_string(),
                // Then branch-scoped, grouped by identifier, counter
                // compared numerically — 2 before 10, which a plain
                // lexicographic sort would invert.
                "1234.2-early".to_string(),
                "1234.10-late".to_string(),
                "proj-9.1-other".to_string(),
            ]
        );
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
        assert!(list_scenario_files(&tmp.path().join("missing")).is_empty());
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
        assert!(section_lines(body, "Missing").is_empty());
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
        assert!(paths.is_empty());
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
        assert!(blocks("<!-- still open -->\n").is_empty());
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
        assert!(blocks("# Only\n\n## Headings\n").is_empty());
        assert!(blocks("").is_empty());
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
}

#[cfg(test)]
mod spec_corpus_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{is_spec_path, spec_feature_slug};

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
}
