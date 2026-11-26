//! `prune-project` — the deterministic half of `/{project}:prune --project`
//! (spec 041, scenario `prune-walks-the-project-files`, for spec 075).
//!
//! The four project files — `AGENTS.md` and the spec root's `system.md`,
//! `events.md` and `errors.md` — and each of their parts, the markdown files
//! directly under each one's directory in `.ductus/docs/` (spec 087), are
//! segmented into entries, a part by its own project file's grammar, each
//! entry reported by its locator, size and digest, never its text. The
//! integrity findings over the parts, and each directory there that could not
//! be listed, ride beside them for the preview to show. The host proposes each
//! entry's home and writes it there; an apply then removes or replaces the
//! entries it lists, refusing the whole write when any of them changed since
//! the preview, because the host judged its moves against the text it read.
//! An apply can also create a missing home from its template.
//!
//! For the two catalog files, a preview runs the literal search behind a
//! removal proposal, so whether an event type or error code still appears in
//! the code never rests on the operator's memory. It is a literal search: an
//! identifier built at runtime does not appear, which is why the walk words
//! zero hits as *no literal occurrence found*.
//!
//! A rewrite guards the window between its read and its rename
//! (`BE-TXN-002`): each file is read once into a [`ReadSnapshot`], and its
//! rewrite is refused, never forced, when the file changed since. The apply
//! goes on to the other files and reports which it wrote and which it
//! refused. A home to create is written create-only ([`write_new_atomic`]),
//! which refuses to replace whatever a writer put there first, so a created
//! home needs no snapshot.
//!
//! Every open the primitive makes goes through `validate_contained`
//! (BE-INPUT-004): a project file that is, or lies under, a symbolic link
//! resolving outside the project is `outside-project` and neither read nor
//! written, a home to create there refuses the whole apply before it writes,
//! and a tracked file the search would read through one is left unread and
//! named.

use std::path::Path;

use sha2::{Digest, Sha256};

use crate::primitives::check_artifact_size::{
    PROJECT_FILES_KEY, project_subjects, resolve_threshold,
};
use crate::primitives::{
    PrimitiveError, ReadSnapshot, Result, SkipScanner, bounded_lists, hex, inline_code_spans,
    line_ending_of, parse_atx_heading, project_parts, tracked_project_paths, validate_contained,
    write_new_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    EntryKind, EntryRemoval, ListsForm, ProjectEntry, ProjectFile, ProjectFileKind,
    ProjectFileState, ProjectHome, ProjectSearch, PruneProjectArgs, PruneProjectResult,
    SearchState, TailCause, UnexaminedTail,
};

const PRIMITIVE: &str = "prune-project";

/// `framework/templates/project/system.md`, compiled in because an adopter's
/// tree holds no copy of the project templates: `/ductus` installs each as
/// the file itself. [`tests::compiled_templates_match_the_framework`] holds
/// the three constants to the templates, so a template edit and its constant
/// change in one commit.
const SYSTEM_TEMPLATE: &str = "# System\n\n<!-- The constitution's §spec-phase directory tree (.ductus/constitution.md)\n     defines what this file holds. Establish it early, then update it as the\n     architecture evolves. Include only the sections that apply to your project\n     and remove the rest. -->\n\n## Configuration\n\n<!-- How the application is configured. Example:\n\nAll configuration is read from environment variables at startup.\nSecrets are injected via the deployment platform and never committed to source.\n\n| Variable | Required | Default | Purpose |\n| --- | --- | --- | --- |\n| `DATABASE_URL` | yes | — | Primary database connection string |\n| `LOG_LEVEL` | no | `info` | Logging verbosity |\n\n-->\n\n## Application Lifecycle\n\n<!-- Startup sequence, initialization order, and shutdown behavior. Example:\n\n1. Load configuration from environment\n2. Connect to database and run migrations\n3. Initialize cache connection\n4. Start background workers\n5. Begin accepting requests\n\n-->\n\n## Request Lifecycle\n\n<!-- The path a request or message takes through the system. Example:\n\n1. Request received\n2. Middleware chain: logging → authentication → rate limiting → routing\n3. Handler processes request\n4. Response returned\n\n-->\n\n## Multi-tenancy\n\n<!-- Remove this section if your project is not multi-tenant.\n\nDescribe how tenancy is scoped and enforced. Example:\n\nTenant is identified by the `X-Tenant-ID` header, validated against the session.\nAll database queries are scoped to the tenant via a shared query filter.\nBackground jobs carry tenant context through a metadata field.\n\n-->\n\n## Shared Infrastructure\n\n<!-- Packages, modules, or services shared across features. Example:\n\n| Package | Purpose |\n| --- | --- |\n| `shared/database` | Connection pool and query helpers |\n| `shared/middleware` | Common middleware stack |\n| `shared/logging` | Structured logger configuration |\n\n-->\n\n## Module Pattern\n\n<!-- How application modules are structured and isolated. Example:\n\nEach feature module exposes a public API through a single entry point.\nModules may not import from other modules directly — shared code lives\nin the shared infrastructure packages listed above.\nDependencies are injected at initialization, not imported globally.\n\n-->\n";

/// `framework/templates/project/events.md`; see [`SYSTEM_TEMPLATE`].
const EVENTS_TEMPLATE: &str = "# Events\n\n<!-- The constitution's §spec-phase directory tree (.ductus/constitution.md)\n     defines what this file holds. It starts mostly empty and grows as features\n     are built.\n\n     Consider specifying retry policy and dead-letter handling as dedicated\n     feature specs rather than embedding them here. -->\n\n## Envelope Format\n\n<!-- The standard wrapper structure for all events. Example:\n\nAll events use a common envelope:\n\n```json\n{\n  \"id\": \"evt_01J...\",\n  \"type\": \"order.created\",\n  \"timestamp\": \"2025-01-15T09:30:00Z\",\n  \"data\": {}\n}\n```\n\nThe `data` field contains the event-specific payload documented in the catalog below.\n\n-->\n\n## Naming Convention\n\n<!-- How event types and subjects are named. Example:\n\nEvent types use dot notation: `{resource}.{action}` (e.g., `order.created`,\n`user.email_changed`).\n\nSubject or topic names follow the pattern `{service}.{resource}.{action}`.\n\n-->\n\n## Event Catalog\n\n<!-- Registry of event types. Add entries as features are built.\n     Each entry documents the event type, its payload, and who publishes\n     and subscribes to it. Example:\n\n### order.created\n\nPublished when a new order is placed.\n\n- **Publisher**: orders module\n- **Subscribers**: notifications, billing\n- **Payload**:\n\n```json\n{\n  \"order_id\": \"ord_01J...\",\n  \"customer_id\": \"cus_01J...\",\n  \"total\": 4999,\n  \"currency\": \"usd\"\n}\n```\n\n### user.email_changed\n\nPublished when a user updates their email address.\n\n- **Publisher**: users module\n- **Subscribers**: notifications, audit\n- **Payload**:\n\n```json\n{\n  \"user_id\": \"usr_01J...\",\n  \"old_email\": \"old@example.com\",\n  \"new_email\": \"new@example.com\"\n}\n```\n\n-->\n";

/// `framework/templates/project/errors.md`; see [`SYSTEM_TEMPLATE`].
const ERRORS_TEMPLATE: &str = "# Errors\n\n<!-- The constitution's §spec-phase directory tree (.ductus/constitution.md)\n     defines what this file holds. Establish it early, then extend it as modules\n     are built. -->\n\n## Error Response Format\n\n<!-- The standard structure for error responses. Example:\n\nAll error responses use a consistent JSON envelope:\n\n```json\n{\n  \"error\": {\n    \"code\": \"validation_failed\",\n    \"message\": \"One or more fields are invalid.\",\n    \"request_id\": \"req_01J...\",\n    \"details\": []\n  }\n}\n```\n\nThe `request_id` is the request's correlation ID, so a caller can quote it\nwhen reporting the error. The `details` array is optional and used for\nvalidation errors (see below).\n\n-->\n\n## Error Codes\n\n<!-- Naming convention for error codes and a registry of codes in use. Example:\n\nError codes use `snake_case` and follow the pattern `{category}_{description}`.\n\n| Code | Category | Meaning |\n| --- | --- | --- |\n| `auth_token_expired` | auth | Access token has expired |\n| `validation_failed` | validation | Request body failed validation |\n| `not_found` | resource | Requested resource does not exist |\n\n-->\n\n## Status Mapping\n\n<!-- How error codes map to response status codes or equivalent. Example:\n\n| Category | Status Code |\n| --- | --- |\n| `auth_*` | 401 |\n| `forbidden_*` | 403 |\n| `not_found` | 404 |\n| `validation_*` | 422 |\n| `rate_*` | 429 |\n| `internal_*` | 500 |\n\n-->\n\n## Validation Errors\n\n<!-- How per-field validation errors are structured. Example:\n\nValidation errors populate the `details` array with one entry per invalid field:\n\n```json\n{\n  \"error\": {\n    \"code\": \"validation_failed\",\n    \"message\": \"One or more fields are invalid.\",\n    \"request_id\": \"req_01J...\",\n    \"details\": [\n      { \"field\": \"email\", \"code\": \"required\", \"message\": \"Email is required.\" },\n      { \"field\": \"age\", \"code\": \"out_of_range\", \"message\": \"Age must be between 0 and 150.\" }\n    ]\n  }\n}\n```\n\n-->\n\n## Logging\n\n<!-- How errors are logged and severity mapping. Example:\n\nAll errors are logged with structured fields including the error code,\nrequest ID, and stack trace (for unexpected errors only).\n\n| Severity | When |\n| --- | --- |\n| `warn` | Client errors (4xx) — expected, no action needed |\n| `error` | Server errors (5xx) — unexpected, requires investigation |\n\n-->\n\n## Internal vs External Errors\n\n<!-- Rules for what error information is exposed to callers vs kept internal. Example:\n\nExternal responses include the error code and a safe, human-readable message.\nStack traces, internal identifiers, and infrastructure details are never\nincluded in responses. They are logged server-side with the request ID,\nwhich the response also carries so a caller can quote it.\n\n-->\n";

/// A creatable home's project-relative path and template.
fn home(root: &str, home: ProjectHome) -> (String, &'static str) {
    match home {
        ProjectHome::System => (format!("{root}/system.md"), SYSTEM_TEMPLATE),
        ProjectHome::Events => (format!("{root}/events.md"), EVENTS_TEMPLATE),
        ProjectHome::Errors => (format!("{root}/errors.md"), ERRORS_TEMPLATE),
    }
}

/// Segment, search and — on `apply` — write the four project files and their
/// parts.
///
/// A rewrite whose file changed since it was read is not an error: it is
/// refused, left as the other writer made it, and named in the result's
/// `refused`, while the other files are written and named in `written`
/// (`BE-TXN-002`, `BE-TXN-004`).
///
/// # Errors
///
/// - [`PrimitiveError::MissingArgument`] when `apply` lists nothing to do.
/// - [`PrimitiveError::InvalidArgument`] when a list is given without
///   `apply`, or names a path that is neither one of the four project files
///   nor a part of one.
/// - [`PrimitiveError::InvalidReadSize`] / [`PrimitiveError::Toml`] when the
///   configured read size is invalid or the config does not parse, as
///   `check-artifact-size` reports them.
/// - [`PrimitiveError::Git`] when a repository is found but its index cannot
///   be read.
/// - [`PrimitiveError::InvalidPath`] when an apply lists a home to create
///   whose path is, or lies under, a symbolic link resolving outside the
///   project, checked before the apply writes anything; and from the
///   re-check just before each rewrite.
/// - [`PrimitiveError::Io`] when a write fails, or the list file past the read
///   size cannot be written, or the results directory resolves outside the
///   project (`bounded_lists::place`). An apply has written by then, so such an
///   error fails a call whose writes stand; running the preview again reads
///   the changed files.
pub fn run(args: &PruneProjectArgs, repo: &Path) -> Result<PruneProjectResult> {
    run_between(args, repo, || {})
}

/// [`run`], calling `before_write` after everything is read and computed and
/// before the first write, which a test replaces to change a file in that
/// window.
fn run_between(
    args: &PruneProjectArgs,
    repo: &Path,
    before_write: impl FnOnce(),
) -> Result<PruneProjectResult> {
    let root = paths::Paths::load(repo)?.specs_root;
    let subjects = project_subjects(&root);
    let parts = project_parts::project_parts(repo, &root);
    let all = subject_list(&subjects, &parts.parts);
    check_args(args, &all)?;
    let (threshold, notices) = resolve_threshold(repo)?;
    let (files, part_files, snapshots, mut segments) = read_subjects(repo, &all, threshold.bytes);

    let search = if args.apply {
        None
    } else {
        let excluded: Vec<&str> = all.iter().map(|(_, rel, _)| *rel).collect();
        Some(search(repo, &root, &excluded, &mut segments)?)
    };

    let (claimed, stale_entries) = claim(&segments, args);
    let applied = args.apply && stale_entries.is_empty();
    let mut created = Vec::new();
    let mut kept = Vec::new();
    let mut written = Vec::new();
    let mut refused = Vec::new();
    if applied {
        // Every home to create is checked before anything is written, so a
        // refused one never leaves half an apply on disk.
        for kind in &args.create {
            validate_contained(repo, &home(&root, *kind).0)?;
        }
        before_write();
        for (idx, (_, rel, _)) in all.iter().enumerate() {
            let Some(snapshot) = &snapshots[idx] else {
                continue;
            };
            let Some(content) = snapshot.text()? else {
                continue;
            };
            let edits: Vec<&(usize, Option<&str>)> = claimed
                .iter()
                .filter(|(seg, _)| segments[*seg].entry.path == *rel)
                .collect();
            if edits.is_empty() {
                continue;
            }
            match snapshot.write_if_unchanged(rewrite(content, &segments, &edits)) {
                Ok(()) => written.push((*rel).to_string()),
                // The file changed since it was read: leave it as the other
                // writer made it, and go on to the others (`BE-TXN-004`).
                Err(PrimitiveError::ChangedSinceRead { .. }) => refused.push((*rel).to_string()),
                Err(other) => return Err(other),
            }
        }
        for kind in &args.create {
            let (rel, body) = home(&root, *kind);
            if create_home(&repo.join(&rel), body)? {
                created.push(*kind);
            } else {
                kept.push(*kind);
            }
        }
    }
    // An apply that refused a file did not write everything it listed.
    let applied = applied && refused.is_empty();

    let entries: Vec<ProjectEntry> = segments.into_iter().map(|seg| seg.entry).collect();
    let mut result = PruneProjectResult {
        threshold,
        files,
        part_files_count: part_files.len(),
        part_files: Some(part_files),
        part_findings_count: parts.findings.len(),
        part_findings: Some(parts.findings),
        parts_unlisted_count: parts.unlisted.len(),
        parts_unlisted: Some(parts.unlisted),
        form: ListsForm::Inline,
        entries_count: entries.len(),
        entries: Some(entries),
        lists_file: None,
        search,
        applied,
        stale_entries,
        created,
        kept,
        written,
        refused,
        notices,
    };
    bound(repo, &mut result)?;
    Ok(result)
}

/// Every subject in the order it is read and written: the four project
/// files, then each part, which carries its project file's path and is
/// segmented by that file's grammar.
fn subject_list<'a>(
    subjects: &'a [(ProjectFileKind, String); 4],
    parts: &'a [project_parts::Part],
) -> Vec<(ProjectFileKind, &'a str, Option<&'a str>)> {
    subjects
        .iter()
        .map(|(kind, rel)| (*kind, rel.as_str(), None))
        .chain(parts.iter().map(|part| {
            (
                part.kind,
                part.path.as_str(),
                Some(part.project_file.as_str()),
            )
        }))
        .collect()
}

/// Read and segment every subject, in order: the four project files' records,
/// the parts' records, each subject's snapshot, and every entry.
fn read_subjects(
    repo: &Path,
    all: &[(ProjectFileKind, &str, Option<&str>)],
    limit: u64,
) -> (
    Vec<ProjectFile>,
    Vec<ProjectFile>,
    Vec<Option<ReadSnapshot>>,
    Vec<Segment>,
) {
    let mut files = Vec::new();
    let mut part_files = Vec::new();
    let mut snapshots = Vec::new();
    let mut segments: Vec<Segment> = Vec::new();
    for (kind, rel, part_of) in all {
        let (mut file, snapshot) = read_subject(repo, *kind, rel, limit);
        if let Some(text) = snapshot.as_ref().and_then(|s| s.text().ok().flatten()) {
            let (found, tail) = segment(rel, *kind, text);
            segments.extend(found);
            file.unexamined_tail = tail;
        }
        file.part_of = part_of.map(str::to_string);
        if part_of.is_some() {
            part_files.push(file);
        } else {
            files.push(file);
        }
        snapshots.push(snapshot);
    }
    (files, part_files, snapshots, segments)
}

/// Move the result's lists to the file past the read size.
///
/// `hits` ride inside their entries, so the one list moves whole: a hit
/// list is read only beside its entry, and `entries` alone grows past the
/// read size, since each entry is a record whether or not it has hits.
/// The search's `outside-project` grows with the tracked files, so it
/// moves with them, and the parts' three lists grow with the project's
/// parts (spec 087), so they move too.
fn bound(repo: &Path, result: &mut PruneProjectResult) -> Result<()> {
    let mut lists = serde_json::json!({
        "entries": result.entries,
        "part-files": result.part_files,
        "part-findings": result.part_findings,
        "parts-unlisted": result.parts_unlisted,
    });
    if let Some(outside) = result
        .search
        .as_ref()
        .and_then(|s| s.outside_project.as_ref())
    {
        lists["outside-project"] = serde_json::json!(outside);
    }
    if let Some(file) = bounded_lists::place(repo, PRIMITIVE, PROJECT_FILES_KEY, &result, &lists)? {
        result.form = ListsForm::File;
        result.entries = None;
        result.part_files = None;
        result.part_findings = None;
        result.parts_unlisted = None;
        if let Some(search) = result.search.as_mut() {
            search.outside_project = None;
        }
        result.lists_file = Some(file);
    }
    Ok(())
}

/// Refuse an argument set that names nothing to do, a list without an apply,
/// or a removal or replacement in a file that is neither one of the four
/// project files nor a part of one, before any subject is read.
fn check_args(
    args: &PruneProjectArgs,
    subjects: &[(ProjectFileKind, &str, Option<&str>)],
) -> Result<()> {
    let listed = !args.remove.is_empty() || !args.replace.is_empty() || !args.create.is_empty();
    if args.apply && !listed {
        return Err(PrimitiveError::MissingArgument {
            primitive: PRIMITIVE.into(),
            argument: "remove".into(),
            reason: "an apply writes what it lists, so list a removal, replacement or creation"
                .into(),
        });
    }
    if !args.apply && listed {
        return Err(PrimitiveError::InvalidArgument {
            primitive: PRIMITIVE.into(),
            argument: "apply".into(),
            reason: "only an apply removes, replaces or creates; pass apply with the lists".into(),
        });
    }
    let known = |path: &str| subjects.iter().any(|(_, rel, _)| *rel == path);
    let paths = args
        .remove
        .iter()
        .map(|removal| ("remove", removal.path.as_str()))
        .chain(args.replace.iter().map(|r| ("replace", r.path.as_str())));
    for (argument, path) in paths {
        if !known(path) {
            return Err(PrimitiveError::InvalidArgument {
                primitive: PRIMITIVE.into(),
                argument: argument.into(),
                reason: format!("{path} is not one of the four project files or a part of one"),
            });
        }
    }
    Ok(())
}

/// Read one subject. A file that is not there is `absent`; one that is, or
/// lies under, a symbolic link resolving outside the project is
/// `outside-project` and is never opened; one that is there but cannot be
/// read as text is `unreadable`. Neither contributes entries. A file read as
/// text comes back as the [`ReadSnapshot`] its rewrite compares against.
fn read_subject(
    repo: &Path,
    kind: ProjectFileKind,
    rel: &str,
    limit: u64,
) -> (ProjectFile, Option<ReadSnapshot>) {
    let file = |state, bytes: Option<u64>| ProjectFile {
        path: rel.to_string(),
        kind,
        state,
        bytes,
        over_limit: bytes.map(|bytes| bytes > limit),
        unexamined_tail: None,
        part_of: None,
    };
    // The snapshot's read goes through `validate_contained`, so a link
    // leaving the project is refused before any open.
    let snapshot = match ReadSnapshot::read(repo, rel) {
        Ok(snapshot) => snapshot,
        Err(PrimitiveError::InvalidPath { .. }) => {
            return (file(ProjectFileState::OutsideProject, None), None);
        }
        Err(_) => return (file(ProjectFileState::Unreadable, None), None),
    };
    match snapshot.bytes() {
        None => (file(ProjectFileState::Absent, None), None),
        Some(bytes) => {
            let size = bytes.len() as u64;
            match snapshot.text() {
                Ok(_) => (file(ProjectFileState::Present, Some(size)), Some(snapshot)),
                Err(_) => (file(ProjectFileState::Unreadable, None), None),
            }
        }
    }
}

/// One entry and the 0-based span of its lines: `first` to `end`, exclusive,
/// ending at its last non-blank line.
struct Segment {
    entry: ProjectEntry,
    first: usize,
    end: usize,
}

/// The entry being built, and the line it opened on.
#[derive(Clone, Copy)]
enum Open {
    Heading(u8),
    Item,
    Block,
}

/// Segment `content` into entries by the grammar in 041's `data-model.md`
/// §`prune-project`, with the tail no entry covers when the file ends inside an
/// open HTML comment or an unterminated managed region: its lines are neither
/// entries, a heading entry's included, nor silently dropped, since the file's
/// record names where they begin.
/// An unterminated fence is not such a tail; it becomes one block entry.
fn segment(
    rel: &str,
    file: ProjectFileKind,
    content: &str,
) -> (Vec<Segment>, Option<UnexaminedTail>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut segmenter = Segmenter {
        lines: lines.clone(),
        rel,
        file,
        section: None,
        open: None,
        managed: None,
        in_table: false,
        out: Vec::new(),
    };
    let mut skip = SkipScanner::default();
    // The line the comment still open opened on.
    let mut comment_from: Option<usize> = None;
    for (i, line) in lines.iter().copied().enumerate() {
        // The scanner sees every line, in order, whatever the step does next.
        let was_fence = skip.in_fence();
        let was_comment = skip.in_region() && !was_fence;
        let skipped = skip.skip(line);
        let fence = skipped && (was_fence || skip.in_fence());
        let comment = skip.in_region() && !skip.in_fence();
        comment_from = match (was_comment, comment) {
            (false, true) => Some(i),
            (_, false) => None,
            (true, true) => comment_from,
        };
        segmenter.step(i, line, skipped, fence);
    }
    // An entry still open when a comment never closes ends before it, so the
    // tail's lines are covered by no entry, a heading entry's included.
    segmenter.close(comment_from.unwrap_or(segmenter.lines.len()));
    // A managed region opens before any comment inside it, so it names the
    // earlier line.
    let tail = match (&segmenter.managed, comment_from) {
        (Some((_, from)), _) => Some((*from, TailCause::UnterminatedManagedRegion)),
        (None, Some(from)) => Some((from, TailCause::OpenComment)),
        (None, None) => None,
    }
    .map(|(from, cause)| UnexaminedTail {
        from_line: from + 1,
        cause,
    });
    (segmenter.out, tail)
}

/// The segmentation state over one file.
struct Segmenter<'a> {
    lines: Vec<&'a str>,
    rel: &'a str,
    file: ProjectFileKind,
    /// The enclosing `##` heading's text; `None` in the preamble.
    section: Option<String>,
    /// The entry being built, and the 0-based line it opened on.
    open: Option<(Open, usize)>,
    /// The marker of the managed region being skipped, if any, and the
    /// 0-based line it began on.
    managed: Option<(String, usize)>,
    /// A table's delimiter row has been seen, so `|` lines are body rows.
    in_table: bool,
    out: Vec<Segment>,
}

impl Segmenter<'_> {
    /// Advance over line `i`. `skipped` is the scanner's verdict, and `fence`
    /// says the skipped line belongs to a fenced block rather than a comment.
    fn step(&mut self, i: usize, line: &str, skipped: bool, fence: bool) {
        let trimmed = line.trim();
        if let Some((marker, _)) = &self.managed {
            if trimmed == format!("<!-- END {marker} -->") {
                self.managed = None;
            }
            return;
        }
        if !skipped && let Some(marker) = begin_marker(trimmed) {
            self.close(i);
            self.managed = Some((marker.to_string(), i));
            return;
        }
        let whole_comment = !skipped && trimmed.starts_with("<!--") && trimmed.ends_with("-->");
        if (skipped && !fence) || whole_comment {
            // Part of an open heading entry; otherwise it ends the entry and
            // is never one itself.
            if !self.in_heading() {
                self.close(i);
            }
            self.in_table = false;
        } else if skipped {
            // A fenced block belongs to the entry it sits in, or is one.
            if self.open.is_none() {
                self.open = Some((Open::Block, i));
            }
        } else if let Some((level, text)) = parse_atx_heading(line) {
            self.heading(i, level, text);
        } else if trimmed.is_empty() {
            self.blank(i);
        } else if !self.in_heading() {
            self.content(i, line, trimmed);
        }
    }

    fn in_heading(&self) -> bool {
        matches!(self.open, Some((Open::Heading(_), _)))
    }

    /// A `#` or `##` heading is structure; a deeper one opens a heading
    /// entry, unless it nests inside an open, shallower one.
    fn heading(&mut self, i: usize, level: u8, text: String) {
        self.in_table = false;
        if level <= 2 {
            self.close(i);
            self.section = (level == 2).then_some(text);
            return;
        }
        if let Some((Open::Heading(outer), _)) = self.open
            && outer < level
        {
            return;
        }
        self.close(i);
        self.open = Some((Open::Heading(level), i));
    }

    /// A blank line ends an item or block — not a loose list item whose next
    /// line is indented — and any table; a heading entry runs on.
    fn blank(&mut self, i: usize) {
        self.in_table = false;
        match self.open {
            Some((Open::Heading(_), _)) | None => {}
            Some((Open::Item, _)) if next_line_is_indented(&self.lines, i) => {}
            Some(_) => self.close(i),
        }
    }

    /// A non-blank, non-heading line outside a heading entry.
    fn content(&mut self, i: usize, line: &str, trimmed: &str) {
        if trimmed.starts_with('|') {
            self.table_line(i, trimmed);
            return;
        }
        self.in_table = false;
        if is_top_level_item(line) {
            self.close(i);
            self.open = Some((Open::Item, i));
        } else if self.open.is_none() {
            self.open = Some((Open::Block, i));
        }
    }

    /// A header row, when its delimiter follows, and the delimiter are
    /// structure; each body row is an entry of its own.
    fn table_line(&mut self, i: usize, trimmed: &str) {
        self.close(i);
        if is_delimiter_row(trimmed) {
            self.in_table = true;
            return;
        }
        let header = !self.in_table
            && self
                .lines
                .get(i + 1)
                .is_some_and(|next| is_delimiter_row(next.trim()));
        if !header {
            self.push(EntryKind::Row, i, i + 1);
        }
    }

    /// Record the open entry, ending before line `upto`.
    fn close(&mut self, upto: usize) {
        if let Some((shape, first)) = self.open.take() {
            let kind = match shape {
                Open::Heading(_) => EntryKind::Heading,
                Open::Item => EntryKind::Item,
                Open::Block => EntryKind::Block,
            };
            self.push(kind, first, upto);
        }
    }

    /// Record the entry spanning `first..upto`, trimmed to its last
    /// non-blank line. A span with no non-blank line records nothing.
    fn push(&mut self, kind: EntryKind, first: usize, upto: usize) {
        let Some(last) = self.lines[first..upto]
            .iter()
            .rposition(|line| !line.trim().is_empty())
        else {
            return;
        };
        let end = first + last + 1;
        let text = self.lines[first..end].join("\n");
        let identifier = match self.file {
            ProjectFileKind::Events | ProjectFileKind::Errors => {
                identifier(kind, self.lines[first])
            }
            ProjectFileKind::Agents | ProjectFileKind::System => None,
        };
        self.out.push(Segment {
            entry: ProjectEntry {
                path: self.rel.to_string(),
                section: self.section.clone(),
                kind,
                start_line: first + 1,
                end_line: end,
                bytes: text.len(),
                digest: hex(&Sha256::digest(text.as_bytes())),
                identifier,
                hits: None,
            },
            first,
            end,
        });
    }
}

/// The event type or error code a catalog entry names: the first code span
/// on its first line — for a row, in its first cell — or, for a heading with
/// none, the heading text. A block names none.
fn identifier(kind: EntryKind, first_line: &str) -> Option<String> {
    let span = |text: &str| {
        inline_code_spans(text)
            .first()
            .map(|range| text[range.clone()].trim().to_string())
    };
    let found = match kind {
        EntryKind::Heading => parse_atx_heading(first_line)
            .map(|(_, text)| span(&text).unwrap_or_else(|| text.trim().to_string())),
        EntryKind::Item => span(first_line),
        EntryKind::Row => first_line
            .trim()
            .trim_start_matches('|')
            .split('|')
            .next()
            .and_then(span),
        EntryKind::Block => None,
    };
    found.filter(|id| !id.is_empty())
}

/// The marker of a `<!-- BEGIN {marker} -->` line, as `merge-managed-block`
/// writes it.
fn begin_marker(trimmed: &str) -> Option<&str> {
    let marker = trimmed
        .strip_prefix("<!-- BEGIN ")?
        .strip_suffix(" -->")?
        .trim();
    (!marker.is_empty()).then_some(marker)
}

/// A table's delimiter row: pipes, dashes, colons and spaces, with a dash.
fn is_delimiter_row(trimmed: &str) -> bool {
    trimmed.starts_with('|')
        && trimmed.contains('-')
        && trimmed.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

/// A list item at the left margin: `-`, `*` or `+`, or `1.` / `1)`.
fn is_top_level_item(line: &str) -> bool {
    if line.starts_with("- ") || line.starts_with("* ") || line.starts_with("+ ") {
        return true;
    }
    let digits = line.bytes().take_while(u8::is_ascii_digit).count();
    digits > 0 && (line[digits..].starts_with(". ") || line[digits..].starts_with(") "))
}

/// Whether the next non-blank line after `i` is indented, so a blank line
/// inside a loose list item does not end it.
fn next_line_is_indented(lines: &[&str], i: usize) -> bool {
    lines[i + 1..]
        .iter()
        .find(|line| !line.trim().is_empty())
        .is_some_and(|line| line.starts_with(' ') || line.starts_with('\t'))
}

/// Run the literal search for every catalog identifier over the tracked files
/// outside the spec root, every subject excepted — the four project files and
/// each part, so a catalog entry is never a hit on its own file or its
/// index — setting each such entry's `hits`. Each file is read once, unless
/// it is, or lies under, a symbolic link resolving outside the project: that
/// one is not read and is named in `outside-project`. With no identifier to
/// look for, nothing is read.
fn search(
    repo: &Path,
    root: &str,
    excluded: &[&str],
    segments: &mut [Segment],
) -> Result<ProjectSearch> {
    let Some(tracked) = tracked_project_paths(repo)? else {
        return Ok(ProjectSearch {
            state: SearchState::Unavailable,
            files_searched: None,
            reason: Some(
                "no git repository contains the project, so no tracked file was read".into(),
            ),
            outside_project: None,
            outside_project_count: None,
        });
    };
    let prefix = format!("{root}/");
    let candidates: Vec<String> = tracked
        .into_iter()
        .filter(|path| !path.starts_with(&prefix))
        .filter(|path| !excluded.contains(&path.as_str()))
        .collect();

    let wanted: Vec<usize> = (0..segments.len())
        .filter(|idx| segments[*idx].entry.identifier.is_some())
        .collect();
    for idx in &wanted {
        segments[*idx].entry.hits = Some(Vec::new());
    }
    let mut files_searched = 0;
    let mut outside_project = Vec::new();
    if !wanted.is_empty() {
        for path in &candidates {
            match validate_contained(repo, path) {
                Ok(()) => {}
                Err(PrimitiveError::InvalidPath { .. }) => {
                    outside_project.push(path.clone());
                    continue;
                }
                Err(err) => return Err(err),
            }
            let Ok(bytes) = std::fs::read(repo.join(path)) else {
                continue;
            };
            files_searched += 1;
            let text = std::str::from_utf8(&bytes).ok();
            for idx in &wanted {
                let entry = &mut segments[*idx].entry;
                let Some(id) = &entry.identifier else {
                    continue;
                };
                if contains(text, &bytes, id)
                    && let Some(hits) = entry.hits.as_mut()
                {
                    hits.push(path.clone());
                }
            }
        }
    }
    Ok(ProjectSearch {
        state: SearchState::Ran,
        files_searched: Some(files_searched),
        reason: None,
        outside_project_count: Some(outside_project.len()),
        outside_project: Some(outside_project),
    })
}

/// Whether `needle` occurs in `hay` as a byte substring. `text` is `hay`
/// validated as UTF-8 once per file by the caller, or `None` for a file that
/// is not, which keeps the byte-window path.
fn contains(text: Option<&str>, hay: &[u8], needle: &str) -> bool {
    match text {
        Some(text) => text.contains(needle),
        None => hay
            .windows(needle.len().max(1))
            .any(|window| window == needle.as_bytes()),
    }
}

/// Claim one entry per listed removal or replacement, by path and digest.
/// Each listing with no unclaimed entry to match is stale.
fn claim<'a>(
    segments: &[Segment],
    args: &'a PruneProjectArgs,
) -> (Vec<(usize, Option<&'a str>)>, Vec<EntryRemoval>) {
    let listed = args
        .remove
        .iter()
        .map(|removal| (&removal.path, &removal.digest, None))
        .chain(
            args.replace
                .iter()
                .map(|r| (&r.path, &r.digest, Some(r.with.as_str()))),
        );
    let mut claimed: Vec<(usize, Option<&str>)> = Vec::new();
    let mut stale = Vec::new();
    for (path, digest, with) in listed {
        let found = segments.iter().enumerate().position(|(idx, seg)| {
            seg.entry.path == *path
                && seg.entry.digest == *digest
                && !claimed.iter().any(|(taken, _)| *taken == idx)
        });
        match found {
            Some(idx) => claimed.push((idx, with)),
            None => stale.push(EntryRemoval {
                path: path.clone(),
                digest: digest.clone(),
            }),
        }
    }
    (claimed, stale)
}

/// Apply `edits` to `content` bottom-up, so each span still names the lines
/// it was read at, and collapse the blank lines at each changed seam to at
/// most one — none at either end of the file. Line endings and a final
/// newline are kept as they were.
fn rewrite(content: &str, segments: &[Segment], edits: &[&(usize, Option<&str>)]) -> String {
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let mut ordered: Vec<&(usize, Option<&str>)> = edits.to_vec();
    ordered.sort_by_key(|(idx, _)| std::cmp::Reverse(segments[*idx].first));
    let blank = |line: &String| line.trim().is_empty();
    for (idx, with) in ordered {
        let seg = &segments[*idx];
        let replacement: Vec<String> = with
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default();
        let at = seg.first;
        let added = replacement.len();
        lines.splice(seg.first..seg.end, replacement);
        if added == 0 {
            if at > 0 && at < lines.len() && blank(&lines[at - 1]) && blank(&lines[at]) {
                lines.remove(at);
            }
            while at == 0 && lines.first().is_some_and(blank) {
                lines.remove(0);
            }
            while at >= lines.len() && lines.last().is_some_and(blank) {
                lines.pop();
            }
        }
    }
    let ending = line_ending_of(content);
    let mut out = lines.join(ending);
    if content.ends_with('\n') && !out.is_empty() {
        out.push_str(ending);
    }
    out
}

/// Create a missing home from its template, at mode `0644` on Unix
/// (`write_new_atomic` leaves a new file at its tempfile's `0600`). Returns
/// `false`, writing nothing, when the home already exists, including one a
/// writer created since the caller looked: a create never replaces a file,
/// so it needs no [`ReadSnapshot`] (`BE-TXN-002`).
fn create_home(path: &Path, body: &str) -> Result<bool> {
    if !write_new_atomic(path, body)? {
        return Ok(false);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o644)).map_err(
            |source| PrimitiveError::Io {
                path: path.into(),
                source,
            },
        )?;
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::fs;
    use std::path::PathBuf;

    use tempfile::TempDir;

    use super::*;
    use crate::primitives::git_fixture::{commit_all, write};
    use crate::schema::primitives::EntryReplacement;

    const AGENTS: &str = "\
# Project: demo

One-line description.

<!-- BEGIN ductus:memory-routing -->
> **Where a learning goes** — managed by /ductus.
<!-- END ductus:memory-routing -->

## Workflow

- Run the tests first.
  - A nested child stays with its item.
- Commit directly to main.

<!-- Guidance a template left behind.
     It spans lines. -->

## Tech Stack

| Layer | Technology |
| --- | --- |
| Language | Go |
| Database | PostgreSQL |

## Gotchas

A paragraph about the build.
It runs on.

### Deploys

The deploy script needs a token.

```sh
make deploy
```
";

    const EVENTS: &str = "\
# Events

## Event Catalog

### order.created

Published when an order is placed.

### `user.deleted`

Published when a user is removed.
";

    const ERRORS: &str = "\
# Errors

## Error Codes

Codes use `snake_case`.

| Code | Meaning |
| --- | --- |
| `auth_token_expired` | The token has expired |
| `not_found` | Nothing is there |
";

    fn project() -> TempDir {
        let dir = TempDir::new().unwrap();
        write(&dir.path().join("AGENTS.md"), AGENTS);
        write(&dir.path().join("specs/events.md"), EVENTS);
        write(&dir.path().join("specs/errors.md"), ERRORS);
        dir
    }

    fn preview(dir: &Path) -> PruneProjectResult {
        run(&PruneProjectArgs::default(), dir).unwrap()
    }

    fn entries_of<'a>(result: &'a PruneProjectResult, path: &str) -> Vec<&'a ProjectEntry> {
        result
            .entries
            .as_ref()
            .unwrap()
            .iter()
            .filter(|e| e.path == path)
            .collect()
    }

    #[test]
    fn agents_md_segments_into_each_entry_kind() {
        let dir = project();
        let result = preview(dir.path());
        let shape: Vec<(EntryKind, Option<&str>, usize, usize)> = entries_of(&result, "AGENTS.md")
            .into_iter()
            .map(|e| (e.kind, e.section.as_deref(), e.start_line, e.end_line))
            .collect();
        assert_eq!(
            shape,
            vec![
                (EntryKind::Block, None, 3, 3),
                (EntryKind::Item, Some("Workflow"), 11, 12),
                (EntryKind::Item, Some("Workflow"), 13, 13),
                (EntryKind::Row, Some("Tech Stack"), 22, 22),
                (EntryKind::Row, Some("Tech Stack"), 23, 23),
                (EntryKind::Block, Some("Gotchas"), 27, 28),
                (EntryKind::Heading, Some("Gotchas"), 30, 36),
            ]
        );
    }

    #[test]
    fn a_managed_region_and_a_comment_are_never_entries() {
        let dir = project();
        let result = preview(dir.path());
        for entry in entries_of(&result, "AGENTS.md") {
            let lines: Vec<&str> = AGENTS.lines().collect();
            let text = lines[entry.start_line - 1..entry.end_line].join("\n");
            assert!(!text.contains("Where a learning goes"), "{text}");
            assert!(!text.contains("Guidance a template"), "{text}");
        }
    }

    #[test]
    fn a_fenced_block_stays_inside_its_heading_entry() {
        let dir = project();
        let result = preview(dir.path());
        let deploys = entries_of(&result, "AGENTS.md")
            .into_iter()
            .find(|e| e.kind == EntryKind::Heading)
            .unwrap();
        // `### Deploys` through the fence's closing line.
        assert_eq!((deploys.start_line, deploys.end_line), (30, 36));
    }

    #[test]
    fn catalog_entries_carry_their_identifier() {
        let dir = project();
        let result = preview(dir.path());
        let ids = |path| -> Vec<Option<String>> {
            entries_of(&result, path)
                .into_iter()
                .map(|e| e.identifier.clone())
                .collect()
        };
        assert_eq!(
            ids("specs/events.md"),
            vec![Some("order.created".into()), Some("user.deleted".into())]
        );
        // The prose block names no identifier; each row names its code.
        assert_eq!(
            ids("specs/errors.md"),
            vec![
                None,
                Some("auth_token_expired".into()),
                Some("not_found".into())
            ]
        );
        assert!(ids("AGENTS.md").iter().all(Option::is_none));
    }

    #[test]
    fn the_literal_search_reports_each_tracked_hit_outside_the_spec_root() {
        let dir = project();
        let repository = git2::Repository::init(dir.path()).unwrap();
        write(
            &dir.path().join("src/orders.rs"),
            "emit(\"order.created\");\n",
        );
        // A mention under the spec root, or in a subject, is not code.
        write(&dir.path().join("specs/001-x/spec.md"), "user.deleted\n");
        write(
            &dir.path().join("AGENTS.md"),
            &format!("{AGENTS}\nThe user.deleted event is audited.\n"),
        );
        commit_all(&repository, "fixture");
        // An untracked file is not searched.
        write(&dir.path().join("scratch.rs"), "user.deleted\n");

        let result = preview(dir.path());
        let hits = |id: &str| {
            result
                .entries
                .as_ref()
                .unwrap()
                .iter()
                .find(|e| e.identifier.as_deref() == Some(id))
                .and_then(|e| e.hits.clone())
        };
        assert_eq!(
            hits("order.created"),
            Some(vec!["src/orders.rs".to_string()])
        );
        assert_eq!(hits("user.deleted"), Some(Vec::new()));
        let search = result.search.unwrap();
        assert_eq!(search.state, SearchState::Ran);
        assert_eq!(search.files_searched, Some(1));
    }

    #[test]
    fn with_no_repository_the_search_is_unavailable_never_zero_hits() {
        let dir = project();
        let result = preview(dir.path());
        assert_eq!(result.search.unwrap().state, SearchState::Unavailable);
        assert!(
            result
                .entries
                .as_ref()
                .unwrap()
                .iter()
                .all(|e| e.hits.is_none())
        );
    }

    #[test]
    fn each_file_reports_its_state_and_the_limit_is_a_maximum() {
        let dir = project();
        let agents = AGENTS.len();
        write(
            &dir.path().join(".ductus/config.toml"),
            &format!("[artifacts]\nread-size-bytes = {agents}\n"),
        );
        fs::write(dir.path().join("specs/errors.md"), [0xff, 0xfe]).unwrap();
        let result = preview(dir.path());
        let states: Vec<(ProjectFileState, Option<bool>)> = result
            .files
            .iter()
            .map(|f| (f.state, f.over_limit))
            .collect();
        assert_eq!(
            states,
            vec![
                // Exactly at the limit is not over it.
                (ProjectFileState::Present, Some(false)),
                (ProjectFileState::Absent, None),
                (ProjectFileState::Present, Some(false)),
                (ProjectFileState::Unreadable, None),
            ]
        );
        write(&dir.path().join("AGENTS.md"), &format!("{AGENTS}x"));
        assert_eq!(preview(dir.path()).files[0].over_limit, Some(true));
    }

    #[test]
    fn an_apply_removes_an_entry_and_collapses_its_seam() {
        let dir = project();
        let before = preview(dir.path());
        let paragraph = entries_of(&before, "AGENTS.md")
            .into_iter()
            .find(|e| e.kind == EntryKind::Block && e.section.as_deref() == Some("Gotchas"))
            .unwrap();
        let args = PruneProjectArgs {
            apply: true,
            remove: vec![EntryRemoval {
                path: "AGENTS.md".into(),
                digest: paragraph.digest.clone(),
            }],
            ..PruneProjectArgs::default()
        };
        let result = run(&args, dir.path()).unwrap();
        assert!(result.applied);
        let written = fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert!(!written.contains("A paragraph about the build."));
        assert!(written.contains("## Gotchas\n\n### Deploys"), "{written}");
        assert!(!written.contains("\n\n\n"));
    }

    #[test]
    fn an_apply_replaces_an_entry_with_its_text() {
        let dir = project();
        let before = preview(dir.path());
        let item = entries_of(&before, "AGENTS.md")
            .into_iter()
            .find(|e| e.kind == EntryKind::Item)
            .unwrap();
        let args = PruneProjectArgs {
            apply: true,
            replace: vec![EntryReplacement {
                path: "AGENTS.md".into(),
                digest: item.digest.clone(),
                with: "- See `docs/testing.md` before running the tests.".into(),
            }],
            ..PruneProjectArgs::default()
        };
        assert!(run(&args, dir.path()).unwrap().applied);
        let written = fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert!(written.contains(
            "## Workflow\n\n- See `docs/testing.md` before running the tests.\n- Commit directly to main."
        ));
        assert!(!written.contains("nested child"));
    }

    #[test]
    fn a_stale_listing_refuses_the_whole_apply() {
        let dir = project();
        let before = preview(dir.path());
        let item = entries_of(&before, "AGENTS.md")
            .into_iter()
            .find(|e| e.kind == EntryKind::Item)
            .unwrap();
        let args = PruneProjectArgs {
            apply: true,
            remove: vec![
                EntryRemoval {
                    path: "AGENTS.md".into(),
                    digest: item.digest.clone(),
                },
                EntryRemoval {
                    path: "AGENTS.md".into(),
                    digest: "0000".into(),
                },
            ],
            create: vec![ProjectHome::System],
            ..PruneProjectArgs::default()
        };
        let result = run(&args, dir.path()).unwrap();
        assert!(!result.applied);
        assert_eq!(result.stale_entries.len(), 1);
        assert_eq!(
            fs::read_to_string(dir.path().join("AGENTS.md")).unwrap(),
            AGENTS
        );
        assert!(!dir.path().join("specs/system.md").exists());
    }

    #[test]
    fn an_apply_creates_a_missing_home_and_keeps_a_present_one() {
        let dir = project();
        let args = PruneProjectArgs {
            apply: true,
            create: vec![ProjectHome::System, ProjectHome::Events],
            ..PruneProjectArgs::default()
        };
        let result = run(&args, dir.path()).unwrap();
        assert_eq!(result.created, vec![ProjectHome::System]);
        assert_eq!(result.kept, vec![ProjectHome::Events]);
        let path = dir.path().join("specs/system.md");
        assert_eq!(fs::read_to_string(&path).unwrap(), SYSTEM_TEMPLATE);
        assert_eq!(
            fs::read_to_string(dir.path().join("specs/events.md")).unwrap(),
            EVENTS
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o644);
        }
    }

    #[test]
    fn an_apply_keeps_crlf_line_endings() {
        let dir = project();
        let crlf = AGENTS.replace('\n', "\r\n");
        fs::write(dir.path().join("AGENTS.md"), &crlf).unwrap();
        let before = preview(dir.path());
        let item = entries_of(&before, "AGENTS.md")
            .into_iter()
            .find(|e| e.kind == EntryKind::Item)
            .unwrap();
        let args = PruneProjectArgs {
            apply: true,
            remove: vec![EntryRemoval {
                path: "AGENTS.md".into(),
                digest: item.digest.clone(),
            }],
            ..PruneProjectArgs::default()
        };
        run(&args, dir.path()).unwrap();
        let written = fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert_eq!(
            written.matches('\n').count(),
            written.matches("\r\n").count()
        );
        assert!(written.ends_with("\r\n"));
    }

    #[test]
    fn argument_sets_that_name_nothing_or_no_subject_are_refused() {
        let dir = project();
        let empty_apply = PruneProjectArgs {
            apply: true,
            ..PruneProjectArgs::default()
        };
        assert!(matches!(
            run(&empty_apply, dir.path()),
            Err(PrimitiveError::MissingArgument { .. })
        ));
        let list_only = PruneProjectArgs {
            create: vec![ProjectHome::System],
            ..PruneProjectArgs::default()
        };
        assert!(matches!(
            run(&list_only, dir.path()),
            Err(PrimitiveError::InvalidArgument { .. })
        ));
        let elsewhere = PruneProjectArgs {
            apply: true,
            remove: vec![EntryRemoval {
                path: "README.md".into(),
                digest: "00".into(),
            }],
            ..PruneProjectArgs::default()
        };
        assert!(matches!(
            run(&elsewhere, dir.path()),
            Err(PrimitiveError::InvalidArgument { .. })
        ));
    }

    #[test]
    fn a_project_in_a_repository_subdirectory_searches_its_own_files() {
        let root = TempDir::new().unwrap();
        let (repository, project) =
            crate::primitives::git_fixture::subdirectory_project(root.path());
        write(&project.join("specs/events.md"), EVENTS);
        write(&project.join("src/orders.rs"), "order.created\n");
        write(&root.path().join("other/orders.rs"), "user.deleted\n");
        commit_all(&repository, "fixture");
        let result = preview(&project);
        let hits: Vec<Option<Vec<String>>> = result
            .entries
            .as_ref()
            .unwrap()
            .iter()
            .map(|e| e.hits.clone())
            .collect();
        assert_eq!(
            hits,
            vec![Some(vec!["src/orders.rs".to_string()]), Some(Vec::new())]
        );
    }

    /// A project in a repository with one hit, so `hits` is non-empty.
    fn project_with_a_hit() -> TempDir {
        let dir = project();
        let repository = git2::Repository::init(dir.path()).unwrap();
        write(
            &dir.path().join("src/orders.rs"),
            "emit(\"order.created\");\n",
        );
        commit_all(&repository, "fixture");
        dir
    }

    fn read_size(dir: &Path, bytes: usize) {
        write(
            &dir.join(".ductus/config.toml"),
            &format!("[artifacts]\nread-size-bytes = {bytes}\n"),
        );
    }

    #[test]
    fn entries_are_inline_while_the_result_fits_and_in_a_file_past_it() {
        let dir = project_with_a_hit();
        read_size(dir.path(), 9_999);
        let whole = preview(dir.path());
        assert_eq!(whole.form, ListsForm::Inline);
        assert_eq!(whole.lists_file, None);
        let entries = whole.entries.clone().unwrap();
        assert_eq!(whole.entries_count, entries.len());
        assert!(entries.len() > 2, "{entries:?}");
        assert!(
            !dir.path().join(".ductus/results").exists(),
            "an inline result wrote a list file"
        );

        // A result exactly at the read size fits; one byte under it does not.
        // The threshold prints at four digits throughout, so the length does
        // not change with it.
        let size = serde_json::to_vec(&whole).unwrap().len();
        assert!((1_000..9_999).contains(&size), "{size}");
        read_size(dir.path(), size);
        assert_eq!(preview(dir.path()).form, ListsForm::Inline);
        read_size(dir.path(), size - 1);
        let bounded = preview(dir.path());
        assert_eq!(bounded.form, ListsForm::File);
        assert_eq!(bounded.entries, None);
        assert_eq!(bounded.entries_count, entries.len());
        let file = bounded.lists_file.clone().unwrap();
        assert!(
            file.starts_with(".ductus/results/prune-project/project-files/"),
            "{file}"
        );
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(dir.path().join(&file)).unwrap()).unwrap();
        // The file holds every entry whole, each with its `hits`.
        assert_eq!(lists["entries"], serde_json::to_value(&entries).unwrap());
        assert!(
            entries
                .iter()
                .any(|e| e.hits.as_deref() == Some(&["src/orders.rs".to_string()][..]))
        );
        bounded_lists::assert_counts_name_the_file(
            PRIMITIVE,
            &serde_json::to_value(&bounded).unwrap(),
            &lists,
        );
    }

    #[test]
    fn an_apply_bounds_its_entries_too() {
        let dir = project();
        read_size(dir.path(), 10);
        let applying = PruneProjectArgs {
            apply: true,
            create: vec![ProjectHome::System],
            ..PruneProjectArgs::default()
        };
        let result = run(&applying, dir.path()).unwrap();
        assert!(result.applied);
        assert_eq!(result.form, ListsForm::File);
        assert_eq!(result.entries, None);
        assert!(result.entries_count > 0);
        assert!(dir.path().join(result.lists_file.unwrap()).is_file());
    }

    #[test]
    fn a_non_utf8_tracked_file_is_searched_bytewise() {
        let dir = project();
        let repository = git2::Repository::init(dir.path()).unwrap();
        let mut bytes = vec![0xff, 0xfe];
        bytes.extend_from_slice(b"order.created");
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(dir.path().join("src/blob.bin"), bytes).unwrap();
        commit_all(&repository, "fixture");
        let result = preview(dir.path());
        let hits = entries_of(&result, "specs/events.md")
            .into_iter()
            .find(|e| e.identifier.as_deref() == Some("order.created"))
            .and_then(|e| e.hits.clone());
        assert_eq!(hits, Some(vec!["src/blob.bin".to_string()]));
    }

    #[cfg(unix)]
    fn link(target: &Path, at: &Path) {
        fs::create_dir_all(at.parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(target, at).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn a_linked_agents_md_is_outside_project_and_an_apply_writes_nothing() {
        let dir = project();
        let outside = TempDir::new().unwrap();
        let target = outside.path().join("elsewhere.md");
        write(&target, AGENTS);
        fs::remove_file(dir.path().join("AGENTS.md")).unwrap();
        link(&target, &dir.path().join("AGENTS.md"));

        let result = preview(dir.path());
        let agents = &result.files[0];
        assert_eq!(agents.state, ProjectFileState::OutsideProject);
        assert_eq!((agents.bytes, agents.over_limit), (None, None));
        assert_eq!(entries_of(&result, "AGENTS.md").len(), 0);

        // An entry digest from the target's own text claims nothing, so the
        // apply is refused as stale and the target is untouched.
        let digest = segment("AGENTS.md", ProjectFileKind::Agents, AGENTS).0[0]
            .entry
            .digest
            .clone();
        let applying = PruneProjectArgs {
            apply: true,
            remove: vec![EntryRemoval {
                path: "AGENTS.md".into(),
                digest,
            }],
            ..PruneProjectArgs::default()
        };
        let applied = run(&applying, dir.path()).unwrap();
        assert!(!applied.applied);
        assert_eq!(fs::read_to_string(&target).unwrap(), AGENTS);
    }

    /// An apply listing a removal in `AGENTS.md` and one in `specs/errors.md`.
    fn two_file_removal(dir: &Path) -> PruneProjectArgs {
        let before = preview(dir);
        let agents = entries_of(&before, "AGENTS.md")
            .into_iter()
            .find(|e| e.kind == EntryKind::Block && e.section.as_deref() == Some("Gotchas"))
            .unwrap();
        let errors = segment("specs/errors.md", ProjectFileKind::Errors, ERRORS).0[1]
            .entry
            .digest
            .clone();
        PruneProjectArgs {
            apply: true,
            remove: vec![
                EntryRemoval {
                    path: "AGENTS.md".into(),
                    digest: agents.digest.clone(),
                },
                EntryRemoval {
                    path: "specs/errors.md".into(),
                    digest: errors,
                },
            ],
            ..PruneProjectArgs::default()
        }
    }

    /// A file changed after the apply read it is refused and left as the other
    /// writer made it; the file nobody touched is still written, and the
    /// result names each (`BE-TXN-002`, `BE-TXN-004`).
    #[test]
    fn a_file_changed_after_the_apply_read_it_is_refused_and_the_rest_is_written() {
        let dir = project();
        let args = two_file_removal(dir.path());
        let theirs = format!("{AGENTS}- Added by another writer.\n");
        let result = run_between(&args, dir.path(), || {
            fs::write(dir.path().join("AGENTS.md"), &theirs).unwrap();
        })
        .unwrap();
        assert!(!result.applied, "a refused file means not fully applied");
        assert_eq!(result.refused, vec!["AGENTS.md".to_string()]);
        assert_eq!(result.written, vec!["specs/errors.md".to_string()]);
        assert_eq!(
            fs::read_to_string(dir.path().join("AGENTS.md")).unwrap(),
            theirs
        );
        assert!(
            !fs::read_to_string(dir.path().join("specs/errors.md"))
                .unwrap()
                .contains("auth_token_expired")
        );
    }

    /// With nothing changing between the read and the write, every file is
    /// written and none is refused.
    #[test]
    fn an_unchanged_apply_writes_every_file_and_refuses_none() {
        let dir = project();
        let args = two_file_removal(dir.path());
        let result = run(&args, dir.path()).unwrap();
        assert!(result.applied);
        assert_eq!(
            result.written,
            vec!["AGENTS.md".to_string(), "specs/errors.md".to_string()]
        );
        assert_eq!(result.refused, Vec::<String>::new());
    }

    /// A home another writer created since the apply looked is kept, never
    /// replaced by the template.
    #[test]
    fn a_home_another_writer_created_is_kept_and_never_replaced() {
        let dir = project();
        let args = PruneProjectArgs {
            apply: true,
            create: vec![ProjectHome::System],
            ..PruneProjectArgs::default()
        };
        let result = run_between(&args, dir.path(), || {
            fs::write(dir.path().join("specs/system.md"), "theirs\n").unwrap();
        })
        .unwrap();
        assert_eq!(result.kept, vec![ProjectHome::System]);
        assert_eq!(result.created, Vec::<ProjectHome>::new());
        assert_eq!(
            fs::read_to_string(dir.path().join("specs/system.md")).unwrap(),
            "theirs\n"
        );
    }

    /// The write re-checks containment: a project file swapped for a link
    /// leaving the project after it was read is refused, and the file the
    /// link names is untouched.
    #[cfg(unix)]
    #[test]
    fn a_file_swapped_for_an_outward_link_after_its_read_is_refused() {
        let dir = project();
        let args = two_file_removal(dir.path());
        let outside = TempDir::new().unwrap();
        let target = outside.path().join("elsewhere.md");
        write(&target, "before\n");
        let err = run_between(&args, dir.path(), || {
            fs::remove_file(dir.path().join("AGENTS.md")).unwrap();
            link(&target, &dir.path().join("AGENTS.md"));
        })
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
        assert_eq!(fs::read_to_string(&target).unwrap(), "before\n");
    }

    #[cfg(unix)]
    #[test]
    fn a_linked_spec_root_makes_its_files_outside_project_and_refuses_a_create() {
        let dir = TempDir::new().unwrap();
        write(&dir.path().join("AGENTS.md"), AGENTS);
        let outside = TempDir::new().unwrap();
        write(&outside.path().join("events.md"), EVENTS);
        link(outside.path(), &dir.path().join("specs"));

        let result = preview(dir.path());
        let states: Vec<ProjectFileState> = result.files.iter().map(|f| f.state).collect();
        assert_eq!(
            states,
            vec![
                ProjectFileState::Present,
                ProjectFileState::OutsideProject,
                ProjectFileState::OutsideProject,
                ProjectFileState::OutsideProject,
            ]
        );
        assert_eq!(entries_of(&result, "specs/events.md").len(), 0);

        // A home to create under the linked root is refused, and nothing is
        // written through the link.
        let applying = PruneProjectArgs {
            apply: true,
            create: vec![ProjectHome::System],
            ..PruneProjectArgs::default()
        };
        let err = run(&applying, dir.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
        assert!(!outside.path().join("system.md").exists());
    }

    #[cfg(unix)]
    #[test]
    fn a_refused_create_leaves_no_half_an_apply_on_disk() {
        let dir = project();
        let outside = TempDir::new().unwrap();
        // `specs/errors.md` is present and its listed removal would be
        // written; the home to create is a link out, so nothing is.
        link(
            &outside.path().join("sys.md"),
            &dir.path().join("specs/system.md"),
        );
        let digest = segment("specs/errors.md", ProjectFileKind::Errors, ERRORS).0[1]
            .entry
            .digest
            .clone();
        let applying = PruneProjectArgs {
            apply: true,
            remove: vec![EntryRemoval {
                path: "specs/errors.md".into(),
                digest,
            }],
            create: vec![ProjectHome::System],
            ..PruneProjectArgs::default()
        };
        let err = run(&applying, dir.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
        assert_eq!(
            fs::read_to_string(dir.path().join("specs/errors.md")).unwrap(),
            ERRORS
        );
        assert!(!outside.path().join("sys.md").exists());
    }

    #[cfg(unix)]
    #[test]
    fn the_search_names_a_tracked_link_out_and_does_not_read_it() {
        let dir = project();
        let repository = git2::Repository::init(dir.path()).unwrap();
        write(
            &dir.path().join("src/orders.rs"),
            "emit(\"order.created\");\n",
        );
        let outside = TempDir::new().unwrap();
        let target = outside.path().join("secret.rs");
        write(&target, "order.created and user.deleted\n");
        link(&target, &dir.path().join("src/linked.rs"));
        // A tracked file under a linked directory is named too.
        write(&outside.path().join("lib/deep.rs"), "order.created\n");
        link(&outside.path().join("lib"), &dir.path().join("vendor"));
        commit_all(&repository, "fixture");

        let result = preview(dir.path());
        let search = result.search.as_ref().unwrap();
        assert_eq!(search.state, SearchState::Ran);
        assert_eq!(search.files_searched, Some(1));
        let mut named = search.outside_project.clone().unwrap();
        named.sort();
        assert_eq!(
            named,
            vec!["src/linked.rs".to_string(), "vendor".to_string()]
        );
        assert_eq!(search.outside_project_count, Some(2));
        let hits = |id: &str| {
            entries_of(&result, "specs/events.md")
                .into_iter()
                .find(|e| e.identifier.as_deref() == Some(id))
                .and_then(|e| e.hits.clone())
        };
        assert_eq!(
            hits("order.created"),
            Some(vec!["src/orders.rs".to_string()])
        );
        assert_eq!(hits("user.deleted"), Some(Vec::new()));
    }

    #[cfg(unix)]
    #[test]
    fn outside_project_moves_to_the_lists_file_with_its_count_inline() {
        let dir = project();
        let repository = git2::Repository::init(dir.path()).unwrap();
        write(
            &dir.path().join("src/orders.rs"),
            "emit(\"order.created\");\n",
        );
        let outside = TempDir::new().unwrap();
        write(&outside.path().join("x.rs"), "order.created\n");
        link(
            &outside.path().join("x.rs"),
            &dir.path().join("src/linked.rs"),
        );
        commit_all(&repository, "fixture");
        read_size(dir.path(), 9_999);
        let whole = preview(dir.path());
        assert_eq!(whole.form, ListsForm::Inline);
        assert_eq!(
            whole.search.as_ref().unwrap().outside_project,
            Some(vec!["src/linked.rs".to_string()])
        );
        let size = serde_json::to_vec(&whole).unwrap().len();
        read_size(dir.path(), size - 1);
        let bounded = preview(dir.path());
        assert_eq!(bounded.form, ListsForm::File);
        let search = bounded.search.as_ref().unwrap();
        assert_eq!(search.outside_project, None);
        assert_eq!(search.outside_project_count, Some(1));
        let file = bounded.lists_file.clone().unwrap();
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(dir.path().join(file)).unwrap()).unwrap();
        assert_eq!(
            lists["outside-project"],
            serde_json::json!(["src/linked.rs"])
        );
        assert!(lists["entries"].is_array());
    }

    fn tail_of(content: &str) -> (Vec<(EntryKind, usize, usize)>, Option<UnexaminedTail>) {
        let (segments, tail) = segment("AGENTS.md", ProjectFileKind::Agents, content);
        let shape = segments
            .iter()
            .map(|seg| (seg.entry.kind, seg.entry.start_line, seg.entry.end_line))
            .collect();
        (shape, tail)
    }

    #[test]
    fn an_unterminated_comment_names_where_the_unexamined_tail_begins() {
        let content = "## Gotchas\n\n- first\n- second\n\n<!-- never closed\n- third\n- fourth\n";
        let (shape, tail) = tail_of(content);
        assert_eq!(
            shape,
            vec![(EntryKind::Item, 3, 3), (EntryKind::Item, 4, 4)]
        );
        assert_eq!(
            tail,
            Some(UnexaminedTail {
                from_line: 6,
                cause: TailCause::OpenComment
            })
        );
        // A heading entry open when the comment starts ends before it.
        let under_heading =
            "## Gotchas\n\n### Heading entry\nbody\n\n<!-- never closed\nmore text\n";
        let (shape, tail) = tail_of(under_heading);
        assert_eq!(shape, vec![(EntryKind::Heading, 3, 4)]);
        assert_eq!(tail.map(|t| t.from_line), Some(6));
        // A comment that closes leaves nothing unexamined.
        let closed =
            "## Gotchas\n\n- first\n\n<!-- closed -->\n\n<!-- spans\n   lines -->\n\n- last\n";
        assert_eq!(tail_of(closed).1, None);
    }

    #[test]
    fn an_unterminated_managed_region_names_where_the_unexamined_tail_begins() {
        let content = "## Gotchas\n\n- first\n\n<!-- BEGIN ductus:x -->\n- owned\n- also owned\n";
        let (shape, tail) = tail_of(content);
        assert_eq!(shape, vec![(EntryKind::Item, 3, 3)]);
        assert_eq!(
            tail,
            Some(UnexaminedTail {
                from_line: 5,
                cause: TailCause::UnterminatedManagedRegion
            })
        );
        // A comment left open inside the region does not move the tail off the
        // region's own opening line.
        let nested = "- a\n<!-- BEGIN m -->\n<!-- open\n- b\n";
        assert_eq!(
            tail_of(nested).1,
            Some(UnexaminedTail {
                from_line: 2,
                cause: TailCause::UnterminatedManagedRegion
            })
        );
        // A terminated region, and an unterminated fence, are no tail.
        let closed = "- a\n<!-- BEGIN m -->\nowned\n<!-- END m -->\n- b\n";
        assert_eq!(tail_of(closed).1, None);
        let (shape, tail) = tail_of("- a\n\n```\nnever closed\n- b\n");
        assert_eq!(tail, None);
        assert_eq!(
            shape,
            vec![(EntryKind::Item, 1, 1), (EntryKind::Block, 3, 5)]
        );
    }

    #[test]
    fn a_file_with_an_open_region_reads_present_and_carries_its_tail() {
        let dir = project();
        write(
            &dir.path().join("AGENTS.md"),
            "## Gotchas\n\n- first\n\n<!-- never closed\n- second\n",
        );
        let result = preview(dir.path());
        let agents = &result.files[0];
        assert_eq!(agents.state, ProjectFileState::Present);
        assert_eq!(
            agents.unexamined_tail,
            Some(UnexaminedTail {
                from_line: 5,
                cause: TailCause::OpenComment
            })
        );
        assert!(
            result.files[1..]
                .iter()
                .all(|file| file.unexamined_tail.is_none())
        );
        let value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["files"][0]["unexamined-tail"]["from-line"], 5);
        assert_eq!(
            value["files"][0]["unexamined-tail"]["cause"],
            "open-comment"
        );
    }

    const SYSTEM_INDEX: &str = "\
# System

## Storage

Read [Storage](../.ductus/docs/system/storage.md) before changing how anything is stored.
";

    const STORAGE_PART: &str = "\
# Storage

## Database

All state is kept in one SQLite database.

## Files

No component writes a file of its own.
";

    const STORAGE: &str = ".ductus/docs/system/storage.md";

    /// [`project`] with `system.md` an index linking one part.
    fn parted() -> TempDir {
        let dir = project();
        write(&dir.path().join("specs/system.md"), SYSTEM_INDEX);
        write(&dir.path().join(STORAGE), STORAGE_PART);
        dir
    }

    /// A part is a subject after the four files, segmented by its project
    /// file's grammar and recorded in `part-files`, and an apply removes one
    /// of its entries (spec 087, AC7).
    #[test]
    fn a_part_is_walked_after_the_four_files_and_an_apply_removes_its_entry() {
        let dir = parted();
        let before = preview(dir.path());
        assert!(before.files.iter().all(|f| f.part_of.is_none()));
        let records = before.part_files.clone().unwrap();
        assert_eq!((records.len(), before.part_files_count), (1, 1));
        assert_eq!(records[0].path, STORAGE);
        assert_eq!(records[0].kind, ProjectFileKind::System);
        assert_eq!(records[0].state, ProjectFileState::Present);
        assert_eq!(records[0].part_of.as_deref(), Some("specs/system.md"));
        let sections: Vec<Option<&str>> = entries_of(&before, STORAGE)
            .into_iter()
            .map(|e| e.section.as_deref())
            .collect();
        assert_eq!(sections, vec![Some("Database"), Some("Files")]);
        // The four files' entries come first, then the part's.
        let paths: Vec<&str> = before
            .entries
            .as_ref()
            .unwrap()
            .iter()
            .map(|e| e.path.as_str())
            .collect();
        assert_eq!(
            paths.iter().position(|p| *p == STORAGE),
            Some(paths.len() - 2)
        );

        let files = entries_of(&before, STORAGE)
            .into_iter()
            .find(|e| e.section.as_deref() == Some("Files"))
            .unwrap();
        let args = PruneProjectArgs {
            apply: true,
            remove: vec![EntryRemoval {
                path: STORAGE.into(),
                digest: files.digest.clone(),
            }],
            ..PruneProjectArgs::default()
        };
        let result = run(&args, dir.path()).unwrap();
        assert!(result.applied, "{result:?}");
        assert_eq!(result.written, vec![STORAGE.to_string()]);
        let written = fs::read_to_string(dir.path().join(STORAGE)).unwrap();
        assert!(!written.contains("No component writes"), "{written}");
        assert!(written.contains("All state is kept"), "{written}");
    }

    /// A part of `events.md` carries its catalog identifiers into the literal
    /// search, and no part is searched as a source: the part naming
    /// `invoice.paid` and `order.created` is a hit on neither (spec 087).
    #[test]
    fn a_part_of_events_md_joins_the_search_and_is_never_its_own_hit() {
        let dir = project();
        let repository = git2::Repository::init(dir.path()).unwrap();
        write(
            &dir.path().join(".ductus/docs/events/billing.md"),
            "# Billing\n\n## Event Catalog\n\n### invoice.paid\n\nPublished after order.created.\n",
        );
        write(
            &dir.path().join("src/billing.rs"),
            "emit(\"invoice.paid\");\n",
        );
        commit_all(&repository, "fixture");
        let result = preview(dir.path());
        let hits = |id: &str| {
            result
                .entries
                .as_ref()
                .unwrap()
                .iter()
                .find(|e| e.identifier.as_deref() == Some(id))
                .and_then(|e| e.hits.clone())
        };
        assert_eq!(
            hits("invoice.paid"),
            Some(vec!["src/billing.rs".to_string()])
        );
        assert_eq!(hits("order.created"), Some(Vec::new()));
        assert_eq!(result.search.unwrap().files_searched, Some(1));
    }

    /// A parts directory that cannot be listed is named in `parts-unlisted`,
    /// never read as holding no parts, and a part that is a link leaving the
    /// project is recorded in that state and contributes no entries
    /// (`QUAL-CLAIM-001`).
    #[cfg(unix)]
    #[test]
    fn an_unlistable_directory_and_an_outward_part_are_reported_never_read_as_none() {
        let dir = project();
        let outside = TempDir::new().unwrap();
        fs::write(outside.path().join("x.md"), "# X\n\n## A\n\nText.\n").unwrap();
        fs::create_dir_all(dir.path().join(".ductus/docs/system")).unwrap();
        std::os::unix::fs::symlink(
            outside.path().join("x.md"),
            dir.path().join(".ductus/docs/system/x.md"),
        )
        .unwrap();
        std::os::unix::fs::symlink(outside.path(), dir.path().join(".ductus/docs/events")).unwrap();
        let result = preview(dir.path());
        assert_eq!(
            result.parts_unlisted,
            Some(vec![".ductus/docs/events".to_string()])
        );
        assert_eq!(result.parts_unlisted_count, 1);
        let records = result.part_files.clone().unwrap();
        assert_eq!(records.len(), 1, "{records:?}");
        assert_eq!(records[0].state, ProjectFileState::OutsideProject);
        assert_eq!(
            entries_of(&result, ".ductus/docs/system/x.md"),
            Vec::<&ProjectEntry>::new()
        );
    }

    /// Past the read size the parts' three lists move to the file whole, each
    /// key present there, and each count stays inline (`BE-PAYLOAD-001`).
    #[test]
    fn past_the_read_size_the_parts_lists_are_in_the_file_with_their_counts_inline() {
        let dir = parted();
        write(&dir.path().join(".ductus/docs/system/old.md"), "# Old\n");
        let whole = preview(dir.path());
        assert_eq!(whole.form, ListsForm::Inline);
        let files = whole.part_files.clone().unwrap();
        let findings = whole.part_findings.clone().unwrap();
        assert_eq!((files.len(), findings.len()), (2, 1));
        // Over each part, so `over-limit` reads as it did, and under the
        // result.
        read_size(dir.path(), 500);
        let bounded = preview(dir.path());
        assert_eq!(bounded.form, ListsForm::File);
        assert_eq!(
            (
                &bounded.part_files,
                &bounded.part_findings,
                &bounded.parts_unlisted
            ),
            (&None, &None, &None)
        );
        assert_eq!(
            (
                bounded.part_files_count,
                bounded.part_findings_count,
                bounded.parts_unlisted_count
            ),
            (2, 1, 0)
        );
        let file = bounded.lists_file.clone().unwrap();
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(dir.path().join(&file)).unwrap()).unwrap();
        assert_eq!(lists["part-files"], serde_json::to_value(&files).unwrap());
        assert_eq!(
            lists["part-findings"],
            serde_json::to_value(&findings).unwrap()
        );
        assert_eq!(lists["parts-unlisted"], serde_json::json!([]));
        bounded_lists::assert_counts_name_the_file(
            PRIMITIVE,
            &serde_json::to_value(&bounded).unwrap(),
            &lists,
        );
    }

    /// A project whose four project files name no parts keeps every earlier
    /// key, its files' records carry no `part-of`, and the parts' lists are
    /// present, empty and counted 0 (spec 087, AC8).
    #[test]
    fn a_project_with_no_parts_keeps_every_earlier_key_with_the_parts_lists_empty() {
        let dir = project();
        let value = serde_json::to_value(preview(dir.path())).unwrap();
        let mut keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        let mut expected = vec![
            "applied",
            "created",
            "entries",
            "entries-count",
            "files",
            "form",
            "kept",
            "notices",
            "refused",
            "search",
            "stale-entries",
            "threshold",
            "written",
            "part-files",
            "part-files-count",
            "part-findings",
            "part-findings-count",
            "parts-unlisted",
            "parts-unlisted-count",
        ];
        expected.sort_unstable();
        assert_eq!(keys, expected);
        for list in ["part-files", "part-findings", "parts-unlisted"] {
            assert_eq!(value[list], serde_json::json!([]), "{list}");
            assert_eq!(value[format!("{list}-count")], 0, "{list}");
        }
        for file in value["files"].as_array().unwrap() {
            assert!(!file.as_object().unwrap().contains_key("part-of"), "{file}");
        }
    }

    #[test]
    fn compiled_templates_match_the_framework() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        for (constant, name) in [
            (SYSTEM_TEMPLATE, "system"),
            (EVENTS_TEMPLATE, "events"),
            (ERRORS_TEMPLATE, "errors"),
        ] {
            let template = fs::read_to_string(
                repo_root.join(format!("framework/templates/project/{name}.md")),
            )
            .unwrap();
            assert_eq!(
                constant, template,
                "the compiled {name}.md drifted from framework/templates/project/{name}.md"
            );
        }
    }
}
