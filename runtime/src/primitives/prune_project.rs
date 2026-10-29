//! `prune-project` — the deterministic half of `/{project}:prune --project`
//! (spec 041, scenario `prune-walks-the-project-files`, for spec 075).
//!
//! The four project files — `AGENTS.md` and the spec root's `system.md`,
//! `events.md` and `errors.md` — are segmented into entries, each reported by
//! its locator, size and digest, never its text. The host proposes each
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

use std::path::Path;

use sha2::{Digest, Sha256};

use crate::primitives::check_artifact_size::resolve_threshold;
use crate::primitives::{
    PrimitiveError, Result, SkipScanner, hex, inline_code_spans, line_ending_of, parse_atx_heading,
    tracked_project_paths, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    EntryKind, EntryRemoval, ProjectEntry, ProjectFile, ProjectFileKind, ProjectFileState,
    ProjectHome, ProjectSearch, PruneProjectArgs, PruneProjectResult, SearchState,
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
const ERRORS_TEMPLATE: &str = "# Errors\n\n<!-- The constitution's §spec-phase directory tree (.ductus/constitution.md)\n     defines what this file holds. Establish it early, then extend it as modules\n     are built. -->\n\n## Error Response Format\n\n<!-- The standard structure for error responses. Example:\n\nAll error responses use a consistent JSON envelope:\n\n```json\n{\n  \"error\": {\n    \"code\": \"validation_failed\",\n    \"message\": \"One or more fields are invalid.\",\n    \"details\": []\n  }\n}\n```\n\nThe `details` array is optional and used for validation errors (see below).\n\n-->\n\n## Error Codes\n\n<!-- Naming convention for error codes and a registry of codes in use. Example:\n\nError codes use `snake_case` and follow the pattern `{category}_{description}`.\n\n| Code | Category | Meaning |\n| --- | --- | --- |\n| `auth_token_expired` | auth | Access token has expired |\n| `validation_failed` | validation | Request body failed validation |\n| `not_found` | resource | Requested resource does not exist |\n\n-->\n\n## Status Mapping\n\n<!-- How error codes map to response status codes or equivalent. Example:\n\n| Category | Status Code |\n| --- | --- |\n| `auth_*` | 401 |\n| `forbidden_*` | 403 |\n| `not_found` | 404 |\n| `validation_*` | 422 |\n| `rate_*` | 429 |\n| `internal_*` | 500 |\n\n-->\n\n## Validation Errors\n\n<!-- How per-field validation errors are structured. Example:\n\nValidation errors populate the `details` array with one entry per invalid field:\n\n```json\n{\n  \"error\": {\n    \"code\": \"validation_failed\",\n    \"message\": \"One or more fields are invalid.\",\n    \"details\": [\n      { \"field\": \"email\", \"code\": \"required\", \"message\": \"Email is required.\" },\n      { \"field\": \"age\", \"code\": \"out_of_range\", \"message\": \"Age must be between 0 and 150.\" }\n    ]\n  }\n}\n```\n\n-->\n\n## Logging\n\n<!-- How errors are logged and severity mapping. Example:\n\nAll errors are logged with structured fields including the error code,\nrequest ID, and stack trace (for unexpected errors only).\n\n| Severity | When |\n| --- | --- |\n| `warn` | Client errors (4xx) — expected, no action needed |\n| `error` | Server errors (5xx) — unexpected, requires investigation |\n\n-->\n\n## Internal vs External Errors\n\n<!-- Rules for what error information is exposed to callers vs kept internal. Example:\n\nExternal responses include the error code and a safe, human-readable message.\nStack traces, internal identifiers, and infrastructure details are never\nincluded in responses. They are logged server-side with the request ID\nfor correlation.\n\n-->\n";

/// The four subjects, in subject order, each with its project-relative path.
fn subjects(root: &str) -> [(ProjectFileKind, String); 4] {
    [
        (ProjectFileKind::Agents, "AGENTS.md".to_string()),
        (ProjectFileKind::System, format!("{root}/system.md")),
        (ProjectFileKind::Events, format!("{root}/events.md")),
        (ProjectFileKind::Errors, format!("{root}/errors.md")),
    ]
}

/// A creatable home's project-relative path and template.
fn home(root: &str, home: ProjectHome) -> (String, &'static str) {
    match home {
        ProjectHome::System => (format!("{root}/system.md"), SYSTEM_TEMPLATE),
        ProjectHome::Events => (format!("{root}/events.md"), EVENTS_TEMPLATE),
        ProjectHome::Errors => (format!("{root}/errors.md"), ERRORS_TEMPLATE),
    }
}

/// Segment, search and — on `apply` — write the four project files.
///
/// # Errors
///
/// - [`PrimitiveError::MissingArgument`] when `apply` lists nothing to do.
/// - [`PrimitiveError::InvalidArgument`] when a list is given without
///   `apply`, or names a path that is not one of the four subjects.
/// - [`PrimitiveError::InvalidReadSize`] / [`PrimitiveError::Toml`] when the
///   configured read size is invalid or the config does not parse, as
///   `check-artifact-size` reports them.
/// - [`PrimitiveError::Git`] when a repository is found but its index cannot
///   be read.
/// - [`PrimitiveError::Io`] when a write fails.
pub fn run(args: &PruneProjectArgs, repo: &Path) -> Result<PruneProjectResult> {
    let root = paths::Paths::load(repo).specs_root;
    let subjects = subjects(&root);
    check_args(args, &subjects)?;
    let (threshold, notices) = resolve_threshold(repo)?;

    let mut files = Vec::new();
    let mut contents = Vec::new();
    let mut segments: Vec<Segment> = Vec::new();
    for (kind, rel) in &subjects {
        let (file, content) = read_subject(repo, *kind, rel, threshold.bytes);
        if let Some(text) = &content {
            segments.extend(segment(rel, *kind, text));
        }
        files.push(file);
        contents.push(content);
    }

    let search = if args.apply {
        None
    } else {
        Some(search(repo, &root, &subjects, &mut segments)?)
    };

    let (claimed, stale_entries) = claim(&segments, args);
    let applied = args.apply && stale_entries.is_empty();
    let mut created = Vec::new();
    let mut kept = Vec::new();
    if applied {
        for (idx, (_, rel)) in subjects.iter().enumerate() {
            let Some(content) = &contents[idx] else {
                continue;
            };
            let edits: Vec<&(usize, Option<&str>)> = claimed
                .iter()
                .filter(|(seg, _)| segments[*seg].entry.path == *rel)
                .collect();
            if edits.is_empty() {
                continue;
            }
            write_atomic(&repo.join(rel), &rewrite(content, &segments, &edits))?;
        }
        for kind in &args.create {
            let (rel, body) = home(&root, *kind);
            let path = repo.join(&rel);
            if path.exists() {
                kept.push(*kind);
                continue;
            }
            create_home(&path, body)?;
            created.push(*kind);
        }
    }

    Ok(PruneProjectResult {
        threshold,
        files,
        entries: segments.into_iter().map(|seg| seg.entry).collect(),
        search,
        applied,
        stale_entries,
        created,
        kept,
        notices,
    })
}

/// Refuse an argument set that names nothing to do, or a list without an
/// apply, before anything is read.
fn check_args(args: &PruneProjectArgs, subjects: &[(ProjectFileKind, String); 4]) -> Result<()> {
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
    let known = |path: &str| subjects.iter().any(|(_, rel)| rel == path);
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
                reason: format!("{path} is not one of the four project files"),
            });
        }
    }
    Ok(())
}

/// Read one subject. A file that is not there is `absent`; one that is there
/// but cannot be read as text is `unreadable` and contributes no entries.
fn read_subject(
    repo: &Path,
    kind: ProjectFileKind,
    rel: &str,
    limit: u64,
) -> (ProjectFile, Option<String>) {
    let file = |state, bytes: Option<u64>| ProjectFile {
        path: rel.to_string(),
        kind,
        state,
        bytes,
        over_limit: bytes.map(|bytes| bytes > limit),
    };
    match std::fs::read(repo.join(rel)) {
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            (file(ProjectFileState::Absent, None), None)
        }
        Err(_) => (file(ProjectFileState::Unreadable, None), None),
        Ok(bytes) => {
            let size = bytes.len() as u64;
            match String::from_utf8(bytes) {
                Ok(text) => (file(ProjectFileState::Present, Some(size)), Some(text)),
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
/// §`prune-project`.
fn segment(rel: &str, file: ProjectFileKind, content: &str) -> Vec<Segment> {
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
    for (i, line) in lines.iter().copied().enumerate() {
        // The scanner sees every line, in order, whatever the step does next.
        let was_fence = skip.in_fence();
        let skipped = skip.skip(line);
        let fence = skipped && (was_fence || skip.in_fence());
        segmenter.step(i, line, skipped, fence);
    }
    segmenter.close(segmenter.lines.len());
    segmenter.out
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
    /// The marker of the managed region being skipped, if any.
    managed: Option<String>,
    /// A table's delimiter row has been seen, so `|` lines are body rows.
    in_table: bool,
    out: Vec<Segment>,
}

impl Segmenter<'_> {
    /// Advance over line `i`. `skipped` is the scanner's verdict, and `fence`
    /// says the skipped line belongs to a fenced block rather than a comment.
    fn step(&mut self, i: usize, line: &str, skipped: bool, fence: bool) {
        let trimmed = line.trim();
        if let Some(marker) = &self.managed {
            if trimmed == format!("<!-- END {marker} -->") {
                self.managed = None;
            }
            return;
        }
        if !skipped && let Some(marker) = begin_marker(trimmed) {
            self.close(i);
            self.managed = Some(marker.to_string());
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
/// outside the spec root, the four subjects excepted, setting each such
/// entry's `hits`. Each file is read once. With no identifier to look for,
/// nothing is read.
fn search(
    repo: &Path,
    root: &str,
    subjects: &[(ProjectFileKind, String); 4],
    segments: &mut [Segment],
) -> Result<ProjectSearch> {
    let Some(tracked) = tracked_project_paths(repo)? else {
        return Ok(ProjectSearch {
            state: SearchState::Unavailable,
            files_searched: None,
            reason: Some(
                "no git repository contains the project, so no tracked file was read".into(),
            ),
        });
    };
    let prefix = format!("{root}/");
    let candidates: Vec<String> = tracked
        .into_iter()
        .filter(|path| !path.starts_with(&prefix))
        .filter(|path| subjects.iter().all(|(_, rel)| rel != path))
        .collect();

    let wanted: Vec<usize> = (0..segments.len())
        .filter(|idx| segments[*idx].entry.identifier.is_some())
        .collect();
    for idx in &wanted {
        segments[*idx].entry.hits = Some(Vec::new());
    }
    let mut files_searched = 0;
    if !wanted.is_empty() {
        for path in &candidates {
            let Ok(bytes) = std::fs::read(repo.join(path)) else {
                continue;
            };
            files_searched += 1;
            for idx in &wanted {
                let entry = &mut segments[*idx].entry;
                let Some(id) = &entry.identifier else {
                    continue;
                };
                if contains(&bytes, id.as_bytes())
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
    })
}

/// Whether `needle` occurs in `hay` as a byte substring.
fn contains(hay: &[u8], needle: &[u8]) -> bool {
    match std::str::from_utf8(hay) {
        Ok(text) => std::str::from_utf8(needle).is_ok_and(|needle| text.contains(needle)),
        Err(_) => hay
            .windows(needle.len().max(1))
            .any(|window| window == needle),
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

/// Write a missing home from its template, at mode `0644` on Unix:
/// `write_atomic` leaves a new file at its tempfile's `0600`.
fn create_home(path: &Path, body: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| PrimitiveError::Io {
            path: parent.into(),
            source,
        })?;
    }
    write_atomic(path, body)?;
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
    Ok(())
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
        result.entries.iter().filter(|e| e.path == path).collect()
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
        assert!(result.entries.iter().all(|e| e.hits.is_none()));
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
        let hits: Vec<Option<Vec<String>>> =
            result.entries.iter().map(|e| e.hits.clone()).collect();
        assert_eq!(
            hits,
            vec![Some(vec!["src/orders.rs".to_string()]), Some(Vec::new())]
        );
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
