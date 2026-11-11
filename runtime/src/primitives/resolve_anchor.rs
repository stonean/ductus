//! `resolve-anchor` — verify every `§<anchor>` reference in a file resolves
//! to a `<!-- §anchor -->` marker. By default the markers are collected from
//! the same file (the constitution self-consistency check); pass
//! `markers-path` to resolve a file's references against a *different* file's
//! markers — e.g. a spec's `§` references against the constitution — so a
//! renamed constitution section surfaces as an unresolved reference instead
//! of every reference firing as unresolved noise.
//!
//! NOT EVERY `§X` IS A CLAIM ABOUT THE MARKERS FILE, and treating them all as
//! one was this primitive's defect. `§` is the corpus-wide notation for "a
//! section", not for "a constitution section", so a spec writes
//! `AGENTS.md §Workflow`, `[review.md](...) §Behavior`, and `per §Design
//! above` in the same breath as a genuine constitution citation. Resolving
//! all of them against one file reported 112 unresolved anchors across the
//! spec corpus — and the cost was not noise for its own sake. Spec 023
//! deleted `§lightweight-track` from the constitution and its own task said
//! "verify the anchor is no longer referenced anywhere"; `010`'s spec body
//! still cited it, and the check had been reporting that correctly for four
//! specs' worth of history, invisible among 111 lines that were not defects.
//!
//! Four kinds, and the classification is what makes an unresolved reference
//! worth reading. A reference is classified in this order, the first that
//! applies winning:
//!
//! - **`cross-document`** (spec 071, 067 scenario
//!   `pointers-into-shipped-files-are-judged`) — the reference points into a
//!   document the `## Shared Files` section of `framework/bootstrap/ductus.md`
//!   maps to a source this primitive can read. Two shapes reach it. An
//!   explicit pair — a bold `§X` followed by `in` and a shipped document in a
//!   code span, as in **§Grounding (advisory)** in
//!   `.ductus/reference/analyze.md` — resolves only when `X` *equals* a
//!   heading of that document: the bold delimits the name, so a pointer that
//!   outlives a shortened heading is caught rather than prefix-matched.
//!   Otherwise a line naming **exactly one** document, which the manifest
//!   maps, judges each `§` on it: resolved when it begins with a heading of
//!   that source, a heading of the citing file, or names a marker. Pairs
//!   exist because most command steps name `plan.md` or `spec.md` beside
//!   their pointer: measured 2026-10-25, 59 of 100 pointers sat on such
//!   lines. Where the bootstrap file is absent, as in every adopter, the map
//!   is empty and nothing is judged this way.
//! - **`qualified`** — the reference's line names another document, or it is
//!   a code span holding a path and the reference
//!   (`` `../shared/constitution.md §rule` ``). Judged against every document
//!   the line names (067 scenario `shared-constitution-references-are-judged`):
//!   resolved when its text begins with a heading of a readable markdown
//!   document named, names a marker of one — a registered shared
//!   constitution's among them — begins with a heading of the citing file, or
//!   names a marker of the markers file; otherwise **unresolved**, since a
//!   check that names the document and then ignores it misses every renamed
//!   section. What it cannot judge it says so about, never reports broken:
//!   a line naming only documents absent from the tree, or a file that is
//!   not markdown and holds no section, leaves the reference
//!   `anchor-target-unjudgeable` (excluded by construction); a document that
//!   exists and cannot be read, or a registered `[constitutions.*]` entry that
//!   did not resolve, leaves it `anchor-target-unreadable` (could not be
//!   read). Both are counted, and `/{project}:analyze` records them unexamined.
//!   Line-scoped rather than immediately-preceding on purpose: the dominant
//!   real shape is a table row or a clause naming the file once and then
//!   citing several of its sections (`review.md | edit — §Behavior step 5
//!   and §Load`).
//! - **`intra-document`** — the anchor names a heading in the citing file
//!   (`per §Design above`). Resolved against that file's own headings, which
//!   is the question the author was actually asking.
//! - **`markers`** — everything else: a genuine claim about the markers file.
//!
//! A document is named by a backticked or linked path or a bare `name.md`,
//! resolved from the citing file's directory and then from the project root.
//! The markers file is recognized by where such a token lands, never by its
//! basename, so a registered shared constitution or a `team-constitution.md`
//! is another document; a Shared Files destination whose source is the
//! markers file (`.ductus/constitution.md` cited from `framework/`) is the
//! markers file. Registered documents are read through `resolve-constitutions`'
//! loader, outside the tree where their paths lead — the config-sourced path
//! boundary on [`super::validate_no_traversal`].
//!
//! A heading is an ATX heading line outside a ` ``` ` fenced block and outside
//! an HTML comment: a `## Summary` inside a fenced example record is content,
//! not a section a pointer can name. Outside a pair, a reference names a
//! heading its text begins with, ending where a word ends.
//!
//! What stays reported is deliberate. `the bootstrap's §Derived values` and
//! `spec 022 §Versioning` name another document in prose without naming the
//! file, so they cannot be verified *or* excluded; reporting them is the
//! honest answer, and 31 of the 112 are this shape. The rule is exact — it
//! excludes only references whose line demonstrably names another document —
//! rather than a heuristic that would have swallowed `010`'s real one.

#![allow(clippy::expect_used)]

use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::{
    Result, SkipScanner, contained_path, inline_code_spans, read_text, resolve_path,
};
use crate::schema::primitives::{
    AnchorReference, ConstitutionOutcome, ResolveAnchorArgs, ResolveAnchorResult,
};

/// Execute the `resolve-anchor` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::Io`] when the file cannot
/// be read, and [`crate::primitives::PrimitiveError::InvalidPath`], before
/// anything is read, when `path` or `markers-path` is absolute, climbs out
/// with `..`, or leaves the project through a symlink
/// ([`crate::primitives::contained_path`]). A `[constitutions.*]` registry
/// that does not parse, or names an entry the schema forbids, halts the run
/// with `resolve-constitutions`' own error: the registry is the loading policy
/// for registered documents, and a malformed one must never read as an empty
/// one.
pub fn run(args: &ResolveAnchorArgs, repo: &Path) -> Result<ResolveAnchorResult> {
    let path = contained_path(repo, &args.path)?;
    let markers_file = args
        .markers_path
        .as_deref()
        .map(|markers_path| contained_path(repo, markers_path))
        .transpose()?;
    let content = read_text(&path)?;

    // Markers come from `markers-path` when supplied, else from the scanned
    // file itself (same-file self-consistency check).
    let markers = match &markers_file {
        Some(markers_file) => collect_markers(&read_text(markers_file)?),
        None => collect_markers(&content),
    };
    let own_headings = collect_headings(&content);
    let mut documents = Documents::load(repo, &args.path, args.markers_path.as_deref())?;
    let ctx = Fallbacks {
        own_headings: &own_headings,
        markers: &markers,
    };
    let mut tally = Tally::default();
    for (line_no, line) in content.lines().enumerate() {
        let line_no = u32::try_from(line_no + 1).unwrap_or(u32::MAX);
        let code_spans = inline_code_spans(line);
        // Every document the line names other than the markers file, each
        // resolved once: what a reference on this line can be judged against.
        let named = documents.named_on(line, &code_spans);
        // The one shipped document the line names, when it names exactly one
        // and its source can be read: 071's line rule, unchanged.
        let single_shipped = match named.as_slice() {
            [(_, Named::Sections(sections))] if sections.shipped => Some(&sections.headings),
            _ => None,
        };
        let pairs = documents.shipped.pairs(line);
        let mut on_line: Vec<(usize, AnchorReference)> = Vec::new();
        for cap in reference_regex().captures_iter(line) {
            let whole = cap.get(0).map_or(0, |m| m.start());
            let anchor = cap[1].to_string();
            if is_within_marker_comment(line, whole) {
                continue;
            }
            // A `§anchor` inside a code span is notation being *described*,
            // not a citation — the constitution writes ``§anchor`` when
            // defining what an anchor reference looks like. Same class as the
            // marker-comment exclusion above, and the same rule
            // `check-corpus-links` applies to link targets in code spans. The
            // one span that *is* a citation, a path followed by a reference,
            // is read below.
            if code_spans.iter().any(|span| span.contains(&whole)) {
                continue;
            }
            let rest = &line[whole..];
            // Order matters. An explicit pair into a shipped document is
            // judged against that document alone, by equality; a line naming
            // exactly one shipped document judges against its source, the
            // citing file and the markers (both 071's); a line naming any
            // other document is judged against every readable one it names,
            // with the same two fallbacks; only then a local heading, and
            // last the markers file.
            let cross = pairs.get(&whole).copied().or_else(|| {
                single_shipped.map(|headings| {
                    names_local_heading(headings, rest)
                        || names_local_heading(&own_headings, rest)
                        || markers.contains(&anchor)
                })
            });
            let reference = if let Some(resolved) = cross {
                tally.cross(&anchor, resolved, line_no)
            } else if !named.is_empty() {
                let verdict = judge(&named, rest, &anchor, &ctx);
                tally.qualified(&anchor, verdict, line_no)
            } else if names_local_heading(&own_headings, rest) {
                tally.intra(&anchor, line_no)
            } else {
                tally.markers(&anchor, markers.contains(&anchor), line_no)
            };
            on_line.push((whole, reference));
        }
        // A code span whose whole content is a path followed by one `§`
        // reference is a citation of that path's section (scenario
        // `shared-constitution-references-are-judged`, decision (c)): judged
        // against that document, with the line rule's two fallbacks.
        for span in &code_spans {
            let Some((token, offset)) = span_citation(&line[span.clone()]) else {
                continue;
            };
            let whole = span.start + offset;
            let rest = &line[whole..span.end];
            let Some(cap) = reference_regex().captures(rest) else {
                continue;
            };
            let anchor = cap[1].to_string();
            let reference = match documents.resolve(&token) {
                Named::Markers => tally.markers(&anchor, markers.contains(&anchor), line_no),
                target => {
                    let verdict = judge(&[(token, target)], rest, &anchor, &ctx);
                    tally.qualified(&anchor, verdict, line_no)
                }
            };
            on_line.push((whole, reference));
        }
        on_line.sort_by_key(|(at, _)| *at);
        tally
            .references
            .extend(on_line.into_iter().map(|(_, reference)| reference));
    }

    let mut unresolved: Vec<String> = tally.unresolved.into_iter().collect();
    unresolved.sort();
    Ok(ResolveAnchorResult {
        references: tally.references,
        unresolved,
        qualified: tally.qualified,
        intra_document: tally.intra_document,
        cross_document: tally.cross_document,
        anchor_target_unjudgeable: tally.unjudgeable,
        anchor_target_unreadable: tally.unreadable,
    })
}

const KIND_MARKERS: &str = "markers";
const KIND_QUALIFIED: &str = "qualified";
const KIND_INTRA_DOCUMENT: &str = "intra-document";
const KIND_CROSS_DOCUMENT: &str = "cross-document";

/// The unexamined reason for a reference this primitive could not judge
/// because nothing it names holds a section it can read: every document its
/// line names is absent from the tree, or one is a file that is not markdown.
/// Excluded by construction — it recurs on every run.
pub(crate) const UNJUDGEABLE: &str = "anchor-target-unjudgeable";

/// The unexamined reason for a reference this primitive could not judge
/// because a document its line names exists and could not be read — a shipped
/// source, an in-tree file, a path leaving the project through a symlink — or
/// is a registered `[constitutions.*]` entry that did not resolve. Could not
/// be read: something the run meant to see went unseen.
pub(crate) const UNREADABLE: &str = "anchor-target-unreadable";

/// The citing file's own headings and the markers file's markers: the two
/// fallbacks every judged reference outside an explicit pair may resolve by.
struct Fallbacks<'a> {
    own_headings: &'a [String],
    markers: &'a HashSet<String>,
}

/// How a judged reference came out.
#[derive(Clone, Copy)]
enum Verdict {
    Resolved,
    Unresolved,
    /// Not judged, with the unexamined reason it is counted under.
    Unjudged(&'static str),
}

/// Judge `rest` (the text from the `§`) and its `anchor` against every
/// document a line names. It resolves when it begins with a heading of a
/// readable document named, names one of its markers, begins with a heading
/// of the citing file, or names a marker of the markers file. Otherwise what
/// cannot be judged is said so rather than reported broken: a document that
/// exists unreadable, or a skipped registered entry, makes it
/// [`UNREADABLE`]; a file that is not markdown, which holds no section to
/// read, or a line naming only documents absent from the tree, makes it
/// [`UNJUDGEABLE`]. Only a reference that every judgeable document named
/// failed is unresolved.
fn judge(named: &[(String, Named)], rest: &str, anchor: &str, ctx: &Fallbacks<'_>) -> Verdict {
    let mut unreadable = false;
    let mut non_markdown = false;
    let mut all_absent = true;
    for (_, target) in named {
        match target {
            Named::Sections(sections) => {
                if names_local_heading(&sections.headings, rest)
                    || sections.markers.contains(anchor)
                {
                    return Verdict::Resolved;
                }
                all_absent = false;
            }
            Named::Unreadable => {
                unreadable = true;
                all_absent = false;
            }
            Named::NonMarkdown => {
                non_markdown = true;
                all_absent = false;
            }
            Named::Absent => {}
            // Filtered out of a line's documents; a span citation of the
            // markers file never reaches here.
            Named::Markers => all_absent = false,
        }
    }
    if names_local_heading(ctx.own_headings, rest) || ctx.markers.contains(anchor) {
        return Verdict::Resolved;
    }
    if unreadable {
        Verdict::Unjudged(UNREADABLE)
    } else if non_markdown || all_absent {
        Verdict::Unjudged(UNJUDGEABLE)
    } else {
        Verdict::Unresolved
    }
}

/// The references and counts a scan accumulates.
#[derive(Default)]
struct Tally {
    references: Vec<AnchorReference>,
    unresolved: HashSet<String>,
    qualified: u32,
    intra_document: u32,
    cross_document: u32,
    unjudgeable: u32,
    unreadable: u32,
}

impl Tally {
    fn record(
        &mut self,
        anchor: &str,
        line: u32,
        kind: &str,
        resolved: bool,
        unjudged: Option<&str>,
    ) -> AnchorReference {
        if !resolved {
            self.unresolved.insert(anchor.to_string());
        }
        AnchorReference {
            anchor: anchor.to_string(),
            line,
            resolved,
            kind: kind.to_string(),
            unjudged: unjudged.map(str::to_string),
        }
    }

    fn cross(&mut self, anchor: &str, resolved: bool, line: u32) -> AnchorReference {
        self.cross_document = self.cross_document.saturating_add(1);
        self.record(anchor, line, KIND_CROSS_DOCUMENT, resolved, None)
    }

    fn qualified(&mut self, anchor: &str, verdict: Verdict, line: u32) -> AnchorReference {
        self.qualified = self.qualified.saturating_add(1);
        match verdict {
            Verdict::Resolved => self.record(anchor, line, KIND_QUALIFIED, true, None),
            Verdict::Unresolved => self.record(anchor, line, KIND_QUALIFIED, false, None),
            Verdict::Unjudged(reason) => {
                if reason == UNREADABLE {
                    self.unreadable = self.unreadable.saturating_add(1);
                } else {
                    self.unjudgeable = self.unjudgeable.saturating_add(1);
                }
                self.record(anchor, line, KIND_QUALIFIED, true, Some(reason))
            }
        }
    }

    fn intra(&mut self, anchor: &str, line: u32) -> AnchorReference {
        self.intra_document = self.intra_document.saturating_add(1);
        self.record(anchor, line, KIND_INTRA_DOCUMENT, true, None)
    }

    fn markers(&mut self, anchor: &str, resolved: bool, line: u32) -> AnchorReference {
        self.record(anchor, line, KIND_MARKERS, resolved, None)
    }
}

/// What a document token on a line names, once resolved.
#[derive(Clone)]
enum Named {
    /// The markers file itself, under any path that resolves to it.
    Markers,
    /// A document whose sections this primitive read.
    Sections(Sections),
    /// A markdown path no file in the tree holds, or one leaving the project.
    Absent,
    /// A file that is not markdown, which holds no section to read.
    NonMarkdown,
    /// A document that exists and could not be read, or a registered
    /// `[constitutions.*]` entry that did not resolve.
    Unreadable,
}

/// The sections a named document offers a reference: its ATX headings and its
/// `<!-- §anchor -->` markers. `shipped` marks a Shared Files destination read
/// through its framework source, which 071's line rule judges as
/// `cross-document` when it is the only document a line names.
#[derive(Clone)]
struct Sections {
    headings: Vec<String>,
    markers: HashSet<String>,
    shipped: bool,
}

/// Every document a scan can be asked about, each resolved and read at most
/// once: the markers file, the registered shared constitutions, the shipped
/// documents, and the project's own files.
struct Documents<'a> {
    repo: &'a Path,
    /// The project root made absolute, for comparing a resolved token with a
    /// registered document that sits outside it.
    repo_abs: PathBuf,
    /// The citing file's directory, relative to the project root.
    citing_dir: PathBuf,
    /// `markers-path`, lexically normalized; `None` without one.
    markers_rel: Option<PathBuf>,
    shipped: Shipped<'a>,
    /// Each registered document's absolute path, with its markers when it
    /// loaded and could be read.
    registered: Vec<(PathBuf, Option<HashSet<String>>)>,
    resolved: HashMap<String, Named>,
}

impl<'a> Documents<'a> {
    fn load(repo: &'a Path, citing: &str, markers_path: Option<&str>) -> Result<Self> {
        let repo_abs = std::path::absolute(repo).unwrap_or_else(|_| repo.to_path_buf());
        let citing_dir = Path::new(citing)
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let markers_rel = markers_path.and_then(|m| normalize_rel(Path::new(""), m));
        // The registry is read through `resolve-constitutions` itself, the
        // loading policy for these documents (scenario
        // `shared-constitution-references-are-judged`). A registered path
        // routinely leaves the project (`../shared`); reading it is the
        // config-sourced path boundary stated on
        // [`super::validate_no_traversal`].
        let registry = super::resolve_constitutions::run(repo)?;
        let mut registered = Vec::new();
        for record in registry.loaded.iter().chain(&registry.skipped) {
            let document = normalize_abs(
                &resolve_path(&repo_abs, &record.path)
                    .join(super::resolve_constitutions::DOCUMENT_NAME),
            );
            let markers = (record.outcome == ConstitutionOutcome::Loaded)
                .then(|| read_text(&document).ok().map(|text| collect_markers(&text)))
                .flatten();
            registered.push((document, markers));
        }
        Ok(Self {
            repo,
            repo_abs,
            citing_dir,
            markers_rel,
            shipped: Shipped::load(repo),
            registered,
            resolved: HashMap::new(),
        })
    }

    /// The documents `line` names other than the markers file, each with what
    /// it resolved to, deduplicated by token and in order of appearance.
    fn named_on(
        &mut self,
        line: &str,
        code_spans: &[std::ops::Range<usize>],
    ) -> Vec<(String, Named)> {
        let mut named: Vec<(String, Named)> = Vec::new();
        for token in document_tokens(line, code_spans) {
            if named.iter().any(|(seen, _)| *seen == token) {
                continue;
            }
            match self.resolve(&token) {
                Named::Markers => {}
                target => named.push((token, target)),
            }
        }
        named
    }

    /// Resolve one document token: the markers file first, then a registered
    /// document, then a shipped destination, then a file of the project
    /// resolved from the citing file's directory and then from the root.
    fn resolve(&mut self, token: &str) -> Named {
        if let Some(found) = self.resolved.get(token) {
            return found.clone();
        }
        let found = self.resolve_uncached(token);
        self.resolved.insert(token.to_string(), found.clone());
        found
    }

    fn resolve_uncached(&mut self, token: &str) -> Named {
        let markdown = is_markdown(token);
        // A remote document is never read: absent from the tree.
        if is_url(token) {
            return Named::Absent;
        }
        let bases = [self.citing_dir.clone(), PathBuf::new()];
        let relatives: Vec<PathBuf> = bases
            .iter()
            .filter_map(|base| normalize_rel(base, token))
            .collect();
        if self
            .markers_rel
            .as_ref()
            .is_some_and(|markers| relatives.contains(markers))
        {
            return Named::Markers;
        }
        // A registered document matches by where the token lands, from the
        // citing file's directory or the root, however it is spelled.
        let absolutes: Vec<PathBuf> = bases
            .iter()
            .map(|base| normalize_abs(&self.repo_abs.join(base).join(token)))
            .collect();
        if let Some((_, markers)) = self
            .registered
            .iter()
            .find(|(document, _)| absolutes.contains(document))
        {
            return markers.clone().map_or(Named::Unreadable, |markers| {
                Named::Sections(Sections {
                    headings: Vec::new(),
                    markers,
                    shipped: false,
                })
            });
        }
        if let Some(source) = self.shipped.source_of(token) {
            if self.markers_rel.as_ref() == normalize_rel(Path::new(""), &source).as_ref() {
                return Named::Markers;
            }
            return match self.shipped.headings_of(&source) {
                Some(headings) => Named::Sections(Sections {
                    headings,
                    markers: HashSet::new(),
                    shipped: true,
                }),
                None => Named::Unreadable,
            };
        }
        for relative in &relatives {
            let relative = relative.to_string_lossy().replace('\\', "/");
            // `contained_path` refuses a symlink leaving the project, which is
            // a document this primitive will not read: unreadable, not absent.
            let Ok(path) = contained_path(self.repo, &relative) else {
                return Named::Unreadable;
            };
            if !path.exists() {
                continue;
            }
            if !markdown {
                return Named::NonMarkdown;
            }
            return read_text(&path).map_or(Named::Unreadable, |text| {
                Named::Sections(Sections {
                    headings: collect_headings(&text),
                    markers: collect_markers(&text),
                    shipped: false,
                })
            });
        }
        if markdown {
            Named::Absent
        } else {
            Named::NonMarkdown
        }
    }
}

/// The installer whose **Shared Files** manifest maps each shipped document to
/// its framework source. Present only in a framework checkout.
const BOOTSTRAP: &str = "framework/bootstrap/ductus.md";

/// The documents the **Shared Files** manifest ships, and each one's
/// headings, read at most once per scan.
struct Shipped<'a> {
    repo: &'a Path,
    sources: HashMap<String, String>,
    headings: HashMap<String, Option<Vec<String>>>,
}

impl<'a> Shipped<'a> {
    fn load(repo: &'a Path) -> Self {
        Self {
            repo,
            sources: shipped_sources(repo),
            headings: HashMap::new(),
        }
    }

    /// The framework source of the shipped destination `token` names, as the
    /// manifest writes it or as it normalizes from the root.
    fn source_of(&self, token: &str) -> Option<String> {
        if let Some(source) = self.sources.get(token) {
            return Some(source.clone());
        }
        let normalized = normalize_rel(Path::new(""), token)?;
        let normalized = normalized.to_string_lossy().replace('\\', "/");
        self.sources.get(&normalized).cloned()
    }

    /// The headings of `source`, or `None` when it cannot be read — which
    /// leaves a reference judged against it unexamined, as could not be read.
    fn headings_of(&mut self, source: &str) -> Option<Vec<String>> {
        let repo = self.repo;
        self.headings
            .entry(source.to_string())
            .or_insert_with(|| {
                contained_path(repo, source)
                    .ok()
                    .and_then(|path| read_text(&path).ok())
                    .map(|text| collect_headings(&text))
            })
            .clone()
    }

    /// The explicit pairs on `line`: each pair's `§` offset → whether its
    /// bold name equals a heading of the shipped document it names, for
    /// each pair whose document can be read. Equality, not the line rule's
    /// prefix match: the bold delimits the name, so a pointer longer than
    /// any heading — one that outlived a shortened heading — is unresolved.
    fn pairs(&mut self, line: &str) -> HashMap<usize, bool> {
        let mut pairs = HashMap::new();
        for cap in pair_regex().captures_iter(line) {
            let Some(source) = self.sources.get(&cap[2]).cloned() else {
                continue;
            };
            if let (Some(whole), Some(headings)) = (cap.get(0), self.headings_of(&source)) {
                let name = cap[1].trim();
                // The pair regex matches from the `**`; the `§` follows it.
                pairs.insert(whole.start() + 2, headings.iter().any(|h| h == name));
            }
        }
        pairs
    }
}

/// Destination → source for every row of the `## Shared Files` section of
/// [`BOOTSTRAP`] whose first two cells are each one backticked span. Read from
/// that section alone: the slash-command table below it maps per-agent paths
/// with placeholders, which no citing line names. Empty when the file is
/// absent or unreadable, which disables the judgment rather than guessing.
fn shipped_sources(repo: &Path) -> HashMap<String, String> {
    let Ok(text) = read_text(&repo.join(BOOTSTRAP)) else {
        return HashMap::new();
    };
    let mut map = HashMap::new();
    let mut in_section = false;
    for line in text.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            in_section = heading.trim() == "Shared Files";
            continue;
        }
        if !in_section || !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if let [source, destination, ..] = cells.as_slice()
            && let (Some(source), Some(destination)) = (sole_span(source), sole_span(destination))
        {
            map.insert(destination.to_string(), source.to_string());
        }
    }
    map
}

/// The text of a cell that is exactly one backticked span, else `None`.
fn sole_span(cell: &str) -> Option<&str> {
    let inner = cell.strip_prefix('`')?.strip_suffix('`')?;
    (!inner.is_empty() && !inner.contains('`')).then_some(inner)
}

/// The document tokens `line` names, in order: each word of a code span that
/// ends in `.md`, a code span that is one path with an interior `/` and a file
/// extension (a document that may not be markdown), a markdown link's target
/// with its fragment stripped, and a bare `name.md` outside the spans. A URL
/// to a markdown document names one absent from the tree; any other URL, a
/// link to a fragment of this file, and a link to a directory name nothing.
fn document_tokens(line: &str, code_spans: &[std::ops::Range<usize>]) -> Vec<String> {
    let mut found: Vec<(usize, String)> = Vec::new();
    for span in code_spans {
        let content = &line[span.clone()];
        let words: Vec<&str> = content.split_whitespace().collect();
        for word in &words {
            let word = strip_fragment(word);
            if is_markdown(word) {
                found.push((span.start, word.to_string()));
            }
        }
        if let [word] = words.as_slice() {
            let word = strip_fragment(word);
            if !is_markdown(word) && word.contains('/') && has_extension(word) && !is_url(word) {
                found.push((span.start, word.to_string()));
            }
        }
    }
    let in_span = |at: usize| code_spans.iter().any(|span| span.contains(&at));
    for cap in link_regex().captures_iter(line) {
        let Some(target) = cap.get(1) else { continue };
        if in_span(target.start()) {
            continue;
        }
        let path = strip_fragment(target.as_str());
        // A URL to a markdown document names a document, one this tree does
        // not hold; a URL to anything else names none.
        let names = if is_url(path) {
            is_markdown(path)
        } else {
            is_markdown(path) || has_extension(path)
        };
        if path.is_empty() || !names {
            continue;
        }
        found.push((target.start(), path.to_string()));
    }
    for cap in bare_markdown_regex().captures_iter(line) {
        let Some(token) = cap.get(1) else { continue };
        if in_span(token.start()) {
            continue;
        }
        found.push((token.start(), token.as_str().to_string()));
    }
    found.sort_by_key(|(at, _)| *at);
    found.into_iter().map(|(_, token)| token).collect()
}

/// A code span that is a citation: its whole content a path, a space, then one
/// `§` reference, as in `` `../shared/constitution.md §rule` ``. Returns the
/// path and the byte offset of the `§` within the span.
fn span_citation(content: &str) -> Option<(String, usize)> {
    if content.matches('§').count() != 1 {
        return None;
    }
    let at = content.find('§')?;
    let (head, _) = content.split_at(at);
    let path = head.trim();
    if path.is_empty() || path.contains(char::is_whitespace) || !head.ends_with(char::is_whitespace)
    {
        return None;
    }
    let path = strip_fragment(path);
    (is_markdown(path) || (path.contains('/') && has_extension(path) && !is_url(path)))
        .then(|| (path.to_string(), at))
}

fn strip_fragment(token: &str) -> &str {
    token.split('#').next().unwrap_or(token)
}

fn is_markdown(token: &str) -> bool {
    Path::new(token)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// Whether the last segment of `token` carries a file extension.
fn has_extension(token: &str) -> bool {
    let name = token.rsplit('/').next().unwrap_or(token);
    name.rsplit_once('.').is_some_and(|(stem, ext)| {
        !stem.is_empty() && !ext.is_empty() && ext.chars().all(|c| c.is_ascii_alphanumeric())
    })
}

fn is_url(token: &str) -> bool {
    token.contains("://") || token.starts_with("mailto:")
}

/// `token` joined to `base` with `.` and `..` resolved lexically, relative to
/// the project root; `None` when it is absolute or climbs out of the root.
fn normalize_rel(base: &Path, token: &str) -> Option<PathBuf> {
    let mut parts: Vec<std::ffi::OsString> = Vec::new();
    for component in base.join(token).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                parts.pop()?;
            }
            Component::Normal(part) => parts.push(part.to_os_string()),
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(parts.iter().collect())
}

/// An absolute path with `.` and `..` resolved lexically, symlinks left alone:
/// both sides of a registered-document comparison are built the same way.
fn normalize_abs(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn link_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\]\(([^)\s]*)\)").expect("hard-coded regex compiles"))
}

fn bare_markdown_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"(?:^|[\s(\[])([\w.-]+\.md)\b").expect("hard-coded regex compiles")
    })
}

/// ATX heading texts of a document. A line inside a fenced code block or an
/// HTML comment is content, not structure ([`SkipScanner`]): the fenced
/// example record in review's reference holds `## Summary`, which names no
/// section a pointer can reach.
fn collect_headings(content: &str) -> Vec<String> {
    let mut headings: Vec<String> = heading_lines(content)
        .into_iter()
        .map(|heading| heading.text)
        .collect();
    headings.sort_unstable();
    headings.dedup();
    headings
}

/// One ATX heading of a document, as [`heading_lines`] finds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HeadingLine {
    /// Its 0-based index among the document's [`str::lines`].
    pub(crate) line: usize,
    /// The number of `#` that open it.
    pub(crate) level: usize,
    /// Its text, trimmed.
    pub(crate) text: String,
}

/// Each ATX heading of a document in order. A line inside a fenced code
/// block or an HTML comment is content, not structure ([`SkipScanner`]).
/// The heading rules every `§` judgement here uses, shared with the
/// batching line's citations (spec 080, `tool_loading::cited_tools`).
pub(crate) fn heading_lines(content: &str) -> Vec<HeadingLine> {
    let mut skip = SkipScanner::default();
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| !skip.skip(line))
        .filter_map(|(index, line)| {
            let trimmed = line.trim_start_matches('#');
            let text = trimmed.trim();
            (trimmed.len() < line.len() && trimmed.starts_with(' ') && !text.is_empty()).then(
                || HeadingLine {
                    line: index,
                    level: line.len() - trimmed.len(),
                    text: text.to_string(),
                },
            )
        })
        .collect()
}

/// The lines of the section under the first of `headings` reading `heading`:
/// its heading line up to the next heading of the same or a higher level, or
/// to `line_count`, the document's line count. `None` when no heading reads
/// it. One boundary for every reader of a section: the batching line's
/// citations and `read-section` (spec 080).
pub(crate) fn section_range(
    headings: &[HeadingLine],
    heading: &str,
    line_count: usize,
) -> Option<std::ops::Range<usize>> {
    let at = headings.iter().position(|h| h.text == heading)?;
    let start = &headings[at];
    let end = headings[at + 1..]
        .iter()
        .find(|h| h.level <= start.level)
        .map_or(line_count, |h| h.line);
    Some(start.line..end)
}

/// Whether the text starting at the `§` begins with one of `headings` — the
/// citing file's own, or a shipped source's under the line rule. A `§` in
/// running prose has nothing marking where its name ends, so a heading the
/// text begins with resolves it when the heading ends where a word ends; an
/// explicit pair, whose bold does mark the end, is compared by equality
/// instead (`Shipped::pairs`). `rest` begins with
/// the `§`, so the heading is compared against what follows it.
fn names_local_heading(headings: &[String], rest: &str) -> bool {
    cited_heading(headings, rest).is_some()
}

/// The heading of `headings` that the text starting at the `§` names, under
/// [`names_local_heading`]'s rule: the longest the text begins with, ending
/// where a word ends, so `§Setup details` names `Setup details` over
/// `Setup`. `None` when it names none of them.
pub(crate) fn cited_heading<'h>(headings: &'h [String], rest: &str) -> Option<&'h str> {
    let after = rest.strip_prefix('§').unwrap_or(rest);
    headings
        .iter()
        .filter(|h| {
            after.strip_prefix(h.as_str()).is_some_and(|tail| {
                // The heading must end where a word ends: `§Groundingx` does
                // not name `Grounding`, while `§Hook Installation` may still
                // name a heading `Hook`, since nothing marks where prose ends.
                !tail
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '-')
            })
        })
        .max_by_key(|h| h.len())
        .map(String::as_str)
}

fn collect_markers(content: &str) -> HashSet<String> {
    marker_regex()
        .captures_iter(content)
        .map(|c| c[1].to_string())
        .collect()
}

fn marker_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"<!--\s*§([A-Za-z][A-Za-z0-9_-]*)\s*-->").expect("hard-coded regex compiles")
    })
}

/// An explicit pointer pair — a bold `§X`, then `in`, then a document in a
/// code span — capturing the bold name and the document.
fn pair_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"\*\*§([^*]+)\*\* in `([^`]+)`").expect("hard-coded regex compiles")
    })
}

fn reference_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"§([A-Za-z][A-Za-z0-9_-]*)").expect("hard-coded regex compiles"))
}

fn is_within_marker_comment(line: &str, match_start: usize) -> bool {
    let before = &line[..match_start];
    let after = &line[match_start..];
    before.contains("<!--") && after.contains("-->")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::path::PathBuf;

    fn fixture_repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/primitives/sample-repo")
    }

    #[test]
    fn resolves_constitution_anchors() {
        let repo = fixture_repo();
        let result = run(
            &ResolveAnchorArgs {
                path: "framework/constitution.md".into(),
                markers_path: None,
            },
            &repo,
        )
        .unwrap();

        let resolved_refs: Vec<&str> = result
            .references
            .iter()
            .filter(|r| r.resolved)
            .map(|r| r.anchor.as_str())
            .collect();
        assert!(resolved_refs.contains(&"runtime-boundary"));
        assert!(resolved_refs.contains(&"spec-phase"));

        assert_eq!(result.unresolved, vec!["unknown-anchor".to_string()]);
    }

    /// Helper: scan `body` against a markers file declaring `§known`.
    fn classify(body: &str) -> ResolveAnchorResult {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("markers.md"), "<!-- §known -->\n").unwrap();
        std::fs::write(tmp.path().join("citing.md"), body).unwrap();
        run(
            &ResolveAnchorArgs {
                path: "citing.md".into(),
                markers_path: Some("markers.md".into()),
            },
            tmp.path(),
        )
        .unwrap()
    }

    /// The dominant real shape, and the one an "immediately preceding"
    /// rule misses: a table row naming the file once and then citing several
    /// of its sections. Every reference on the line is qualified, not just
    /// the first.
    #[test]
    fn every_reference_on_a_line_naming_a_document_is_qualified() {
        let r = classify(
            "| `framework/commands/review.md` | edit — §Behavior step 5 and §Load and §Notes |\n",
        );
        assert_eq!(r.qualified, 3);
        assert_eq!(r.unresolved, [] as [String; 0]);
        assert!(r.references.iter().all(|x| x.kind == "qualified"));
        assert!(r.references.iter().all(|x| x.resolved));
    }

    #[test]
    fn a_markdown_link_to_a_document_qualifies_its_line() {
        let r = classify("See [017](../017-derive-dont-ask/spec.md) §Generators for the rule.\n");
        assert_eq!(r.qualified, 1);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    #[test]
    fn a_bare_md_filename_qualifies_its_line() {
        let r = classify("The rule is in AGENTS.md §Workflow.\n");
        assert_eq!(r.qualified, 1);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    #[test]
    fn an_anchor_naming_a_heading_here_resolves_against_this_file() {
        let r = classify("## Design\n\nRewrite the ladder per §Design above.\n");
        assert_eq!(r.intra_document, 1);
        assert_eq!(r.qualified, 0);
        assert_eq!(r.unresolved, [] as [String; 0]);
        assert_eq!(r.references[0].kind, "intra-document");
    }

    /// A `§` in prose resolves against a heading its text begins with:
    /// nothing marks where the name ends, so `§Hook Installation` resolves
    /// whichever of `## Hook` and `## Hook Installation` exists.
    #[test]
    fn a_reference_beginning_with_a_heading_resolves_against_it() {
        let r = classify("## Hook\n\n## Hook Installation\n\nSee §Hook Installation below.\n");
        assert_eq!(r.intra_document, 1);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// THE CONTROL. `010`'s real dangling reference — an anchor deleted from
    /// the constitution by spec 023, on a line naming no document. It stayed
    /// invisible among 111 non-defects for four specs' worth of history, and
    /// the whole point of the classification is that it survives it.
    #[test]
    fn a_genuine_dangling_anchor_is_still_reported() {
        let r = classify("- Lightweight track (§lightweight-track) — skip the plan phase.\n");
        assert_eq!(r.qualified, 0);
        assert_eq!(r.intra_document, 0);
        assert_eq!(r.unresolved, vec!["lightweight-track".to_string()]);
        assert_eq!(r.references[0].kind, "markers");
    }

    /// Notation being described, not cited. The constitution defines what an
    /// anchor reference looks like by writing one in a code span; reporting
    /// that as a dangling reference is manufacturing a finding out of prose.
    #[test]
    fn an_anchor_inside_a_code_span_is_not_a_reference() {
        let r = classify("An anchor reference (`§anchor`) links to a section.\n");
        assert_eq!(r.references, [] as [AnchorReference; 0]);
        assert_eq!(r.unresolved, [] as [String; 0]);
        assert_eq!(r.qualified, 0);
    }

    /// A line citing the MARKERS file is not qualified — the document it
    /// names is the one whose markers are in hand, so it is exactly the kind
    /// of reference this primitive exists to check. Caught by an existing
    /// test on the first draft of the rule.
    #[test]
    fn a_line_citing_the_markers_file_is_not_qualified() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("markers.md"), "<!-- §known -->\n").unwrap();
        std::fs::write(
            tmp.path().join("citing.md"),
            "See [§known](markers.md#known) and `markers.md` §missing.\n",
        )
        .unwrap();
        let r = run(
            &ResolveAnchorArgs {
                path: "citing.md".into(),
                markers_path: Some("markers.md".into()),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            r.qualified, 0,
            "citing the markers file is not qualification"
        );
        assert_eq!(r.unresolved, vec!["missing".to_string()]);
    }

    /// A markers file below the root is cited from a sibling directory by a
    /// `../` path, which never contains the repo-relative `markers-path`:
    /// only its basename identifies the line as citing the markers file, so
    /// the reference is judged against the markers rather than excused as
    /// qualified.
    #[test]
    fn a_line_citing_a_nested_markers_file_by_a_relative_path_is_not_qualified() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("framework/reference")).unwrap();
        std::fs::write(
            tmp.path().join("framework/constitution.md"),
            "<!-- §known -->\n",
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("framework/reference/citing.md"),
            "See [§known](../constitution.md#known) and [§gone](../constitution.md#gone).\n",
        )
        .unwrap();
        let r = run(
            &ResolveAnchorArgs {
                path: "framework/reference/citing.md".into(),
                markers_path: Some("framework/constitution.md".into()),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(r.qualified, 0, "{:?}", r.references);
        let judged: Vec<(&str, &str, bool)> = r
            .references
            .iter()
            .map(|x| (x.anchor.as_str(), x.kind.as_str(), x.resolved))
            .collect();
        assert_eq!(
            judged,
            [("known", "markers", true), ("gone", "markers", false)]
        );
        assert_eq!(r.unresolved, vec!["gone".to_string()]);
    }

    /// Prose that names another document without naming the file can be
    /// neither verified nor excluded, so it stays reported. Stated as a test
    /// because it is a deliberate limit, not an oversight.
    #[test]
    fn prose_qualification_without_a_filename_stays_reported() {
        let r = classify("Deferred to the runtime per spec 022 §Versioning.\n");
        assert_eq!(r.qualified, 0);
        assert_eq!(r.unresolved, vec!["Versioning".to_string()]);
    }

    /// A genuine constitution citation on a line that happens to name a
    /// document is still `qualified`, and resolves by the markers fallback:
    /// a line naming a document often cites a constitution section beside it.
    #[test]
    fn a_resolvable_anchor_on_a_qualified_line_is_still_qualified() {
        let r = classify("Per §known, see `other.md` for the detail.\n");
        assert_eq!(r.qualified, 1);
        assert_eq!(r.references[0].kind, "qualified");
        assert!(r.references[0].resolved);
        assert_eq!(r.references[0].unjudged, None);
    }

    #[test]
    fn markers_are_excluded_from_references() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("only-markers.md");
        std::fs::write(&path, "# only markers\n\n<!-- §foo -->\n<!-- §bar -->\n").unwrap();
        let result = run(
            &ResolveAnchorArgs {
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                markers_path: None,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.references, [] as [AnchorReference; 0]);
        assert_eq!(result.unresolved, [] as [String; 0]);
    }

    /// A repo with a Shared Files manifest mapping `.ductus/reference/x.md`
    /// to a readable source and `.ductus/reference/missing.md` to an absent
    /// one, and a slash-command row below the section mapping
    /// `.ductus/reference/y.md` — which must not count, being outside it.
    /// `bootstrap` false leaves the installer out, as in an adopter.
    fn judge_in(bootstrap: bool, body: &str) -> ResolveAnchorResult {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("markers.md"), "<!-- §known -->\n").unwrap();
        std::fs::create_dir_all(root.join("framework/bootstrap")).unwrap();
        std::fs::create_dir_all(root.join("framework/reference")).unwrap();
        if bootstrap {
            std::fs::write(
                root.join(BOOTSTRAP),
                "# ductus\n\n## Shared Files\n\n\
                 | Source Path | Destination Path |\n| --- | --- |\n\
                 | `framework/reference/x.md` | `.ductus/reference/x.md` |\n\
                 | `framework/reference/missing.md` | `.ductus/reference/missing.md` |\n\n\
                 ## Per-Agent Scaffolding\n\n\
                 | `framework/reference/x.md` | `.ductus/reference/y.md` |\n",
            )
            .unwrap();
        }
        std::fs::write(
            root.join("framework/reference/x.md"),
            "# x\n\n## Grounding\n\n## Rules (blocking and advisory)\n\n```markdown\n## Fenced\n```\n",
        )
        .unwrap();
        std::fs::write(root.join("citing.md"), body).unwrap();
        run(
            &ResolveAnchorArgs {
                path: "citing.md".into(),
                markers_path: Some("markers.md".into()),
            },
            root,
        )
        .unwrap()
    }

    fn judge(body: &str) -> ResolveAnchorResult {
        judge_in(true, body)
    }

    fn kinds(r: &ResolveAnchorResult) -> Vec<&str> {
        r.references.iter().map(|x| x.kind.as_str()).collect()
    }

    #[test]
    fn a_pointer_naming_a_heading_of_the_shipped_source_resolves() {
        let r = judge("Apply **§Grounding** in `.ductus/reference/x.md`.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert!(r.references[0].resolved);
        assert_eq!((r.cross_document, r.qualified), (1, 0));
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// The line rule, without the pair form: a line naming one shipped
    /// document has its `§` judged against that document's headings.
    #[test]
    fn a_reference_on_a_line_naming_one_shipped_document_resolves_against_it() {
        let r = judge("See §Grounding, in `.ductus/reference/x.md`.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    #[test]
    fn a_pointer_naming_no_heading_of_the_shipped_source_is_unresolved() {
        let r = judge("Apply **§Groundng** in `.ductus/reference/x.md`.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert_eq!(r.unresolved, ["Groundng".to_string()]);
    }

    /// A pair names its heading exactly, so a shortened pointer is reported
    /// rather than guessed at.
    #[test]
    fn a_pointer_must_spell_the_whole_heading() {
        let r = judge("See **§Rules** in `.ductus/reference/x.md`.\n");
        assert_eq!(r.unresolved, ["Rules".to_string()]);
        let r = judge("See **§Rules (blocking and advisory)** in `.ductus/reference/x.md`.\n");
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// A line naming a shipped document usually cites a constitution section
    /// beside it (`specs/inbox.md` … §brownfield-inbox); a headings-only rule
    /// would report every one.
    #[test]
    fn a_marker_on_a_line_naming_a_shipped_document_resolves() {
        let r = judge("Append to `.ductus/reference/x.md` per §known.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// A pointer line can also cite a section of its own file. No line in the
    /// corpus needs this today; it keeps the line rule from misreporting one.
    #[test]
    fn a_heading_of_the_citing_file_on_a_pointer_line_resolves() {
        let r = judge(
            "# Doc\n\n## Local\n\nSee §Local and **§Grounding** in `.ductus/reference/x.md`.\n",
        );
        assert_eq!(kinds(&r), ["cross-document", "cross-document"]);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// A command step names `plan.md` beside its pointer; the explicit pair
    /// still says which document the section belongs to.
    #[test]
    fn an_explicit_pair_is_judged_on_a_line_naming_other_documents() {
        let r = judge("Read `plan.md`, then apply **§Grounding** in `.ductus/reference/x.md`.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert_eq!(r.unresolved, [] as [String; 0]);
        let r = judge("Read `plan.md`, then apply **§Groundng** in `.ductus/reference/x.md`.\n");
        assert_eq!(r.unresolved, ["Groundng".to_string()]);
    }

    /// A pair names its document, so it is judged against that document
    /// alone: a marker or a local heading does not rescue a wrong pointer.
    #[test]
    fn an_explicit_pair_is_judged_against_its_document_alone() {
        let r = judge(
            "# Doc\n\n## Local\n\n`plan.md` and **§known** in `.ductus/reference/x.md`, **§Local** in `.ductus/reference/x.md`.\n",
        );
        assert_eq!(kinds(&r), ["cross-document", "cross-document"]);
        assert_eq!(r.unresolved, ["Local".to_string(), "known".to_string()]);
    }

    /// On a line naming only its own document a pair is still judged by its
    /// document alone — the line rule's marker and local-heading fallbacks
    /// never reach it.
    #[test]
    fn a_pair_on_a_line_naming_only_its_document_takes_no_fallback() {
        let r = judge("**§known** in `.ductus/reference/x.md`.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert_eq!(r.unresolved, ["known".to_string()]);
        let r = judge("# Doc\n\n## Local\n\n**§Local** in `.ductus/reference/x.md`.\n");
        assert_eq!(r.unresolved, ["Local".to_string()]);
    }

    /// The bold delimits a pair's name, so a pointer longer than any heading
    /// — one that outlived a shortened heading — is not prefix-matched.
    #[test]
    fn a_pair_longer_than_its_heading_is_unresolved() {
        let r = judge("Apply **§Grounding checks** in `.ductus/reference/x.md`.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert_eq!(r.unresolved, ["Grounding".to_string()]);
    }

    /// A heading line inside a fenced block is example content: review's
    /// reference carries a fenced record holding `## Summary`, which is no
    /// section a pointer can name.
    #[test]
    fn a_heading_inside_a_fenced_block_is_not_a_section() {
        let r = judge("Apply **§Fenced** in `.ductus/reference/x.md`.\n");
        assert_eq!(r.unresolved, ["Fenced".to_string()]);
        let r = judge("See §Fenced, in `.ductus/reference/x.md`.\n");
        assert_eq!(r.unresolved, ["Fenced".to_string()]);
        let r = classify("```markdown\n## Hidden\n```\n\nSee §Hidden above.\n");
        assert_eq!(
            (r.intra_document, r.unresolved.clone()),
            (0, vec!["Hidden".to_string()])
        );
    }

    /// A heading inside an HTML comment is no section either.
    #[test]
    fn a_heading_inside_an_html_comment_is_not_a_section() {
        let r = classify("<!--\n## Hidden\n-->\n\nSee §Hidden above.\n");
        assert_eq!(
            (r.intra_document, r.unresolved.clone()),
            (0, vec!["Hidden".to_string()])
        );
    }

    /// Outside a pair the heading must end where a word ends: `§Groundingx`
    /// and `§Hooks` name no heading `Grounding` or `Hook`.
    #[test]
    fn a_heading_must_end_where_a_word_ends() {
        let r = judge("See §Groundingx, in `.ductus/reference/x.md`.\n");
        assert_eq!(r.unresolved, ["Groundingx".to_string()]);
        let r = classify("## Hook\n\nSee §Hooks above.\n");
        assert_eq!(
            (r.intra_document, r.unresolved.clone()),
            (0, vec!["Hooks".to_string()])
        );
        let r = classify("## Hook\n\nSee §Hook-like reasoning above.\n");
        assert_eq!(r.unresolved, ["Hook-like".to_string()]);
        let r = classify("## Hook\n\nSee §Hook_two above.\n");
        assert_eq!(r.unresolved, ["Hook_two".to_string()]);
        // A heading at the very end of a line still ends a word.
        let r = classify("## Hook\n\nSee §Hook\n");
        assert_eq!(
            (r.intra_document, r.unresolved.clone()),
            (1, Vec::<String>::new())
        );
    }

    /// A pair naming a document the manifest does not map is no 071 pair: it
    /// is a reference on a line naming documents, and here both are absent,
    /// so it is unexamined, excluded by construction.
    #[test]
    fn a_pair_naming_an_unmapped_document_stays_qualified() {
        let r = judge("Read `plan.md`, then **§Nope** in `.ductus/reference/zzz.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
        assert_eq!(r.references[0].unjudged.as_deref(), Some(UNJUDGEABLE));
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// Not a pair, and two documents: 071's line rule does not judge it, so
    /// it is `qualified` — and judged against every readable document the
    /// line names (decision (j)): a heading of either resolves it, and one
    /// that matches neither is unresolved, since one of them is readable.
    #[test]
    fn a_line_naming_two_documents_is_judged_against_both() {
        let r = judge("See §Nope, in `.ductus/reference/x.md` and `other.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
        assert_eq!((r.cross_document, r.qualified), (0, 1));
        assert_eq!(r.unresolved, ["Nope".to_string()]);
        assert_eq!(r.references[0].unjudged, None);
        let r = judge("See §Grounding, in `.ductus/reference/x.md` and `other.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    #[test]
    fn a_document_the_manifest_does_not_map_stays_qualified() {
        let r = judge("**§Nope** in `.ductus/reference/zzz.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
    }

    #[test]
    fn a_row_outside_the_shared_files_section_does_not_map() {
        let r = judge("**§Nope** in `.ductus/reference/y.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
    }

    /// A shipped source that cannot be read is a document the run meant to
    /// see and could not (decision (y)): the reference is unexamined, could
    /// not be read, never reported broken and never folded into a pass.
    #[test]
    fn a_mapped_source_that_cannot_be_read_is_unexamined_as_unreadable() {
        let r = judge("**§Nope** in `.ductus/reference/missing.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
        assert_eq!(r.unresolved, [] as [String; 0]);
        assert_eq!(r.references[0].unjudged.as_deref(), Some(UNREADABLE));
        assert_eq!(
            (r.anchor_target_unreadable, r.anchor_target_unjudgeable),
            (1, 0)
        );
    }

    /// Every adopter: no installer source, so nothing is judged this way.
    #[test]
    fn without_the_installer_nothing_is_judged_across_documents() {
        let r = judge_in(false, "**§Nope** in `.ductus/reference/x.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
        assert_eq!(r.cross_document, 0);
    }

    #[test]
    fn the_markers_file_named_on_the_line_is_not_the_one_document() {
        // Not a pair, so only the line rule can judge it — and only when
        // `markers.md` is not counted as a second document.
        let r = judge("See §Groundng, in `.ductus/reference/x.md`; see `markers.md`.\n");
        assert_eq!(kinds(&r), ["cross-document"]);
        assert_eq!(r.unresolved, ["Groundng".to_string()]);
    }

    /// Corpus conformance (spec 071): every pointer from a shipped command or
    /// reference file into a reference file names a heading that file has.
    /// `/{project}:audit` Family 39 runs the same rule over all of
    /// `framework/` at audit time; this runs it on every `cargo test`, so an
    /// edit to a command or reference file that breaks a pointer fails the
    /// runtime suite too. The floor keeps it from passing on an empty subject.
    #[test]
    fn every_shipped_pointer_into_a_reference_file_resolves() {
        // The repository root, as `check_artifacts`' corpus test takes it:
        // the crate's parent, never canonicalized (a Windows verbatim path).
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let mut judged = 0u32;
        let mut broken = Vec::new();
        for dir in ["framework/commands", "framework/reference"] {
            let mut paths: Vec<PathBuf> = std::fs::read_dir(repo.join(dir))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("md"))
                .collect();
            paths.sort();
            for path in paths {
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                let rel = format!("{dir}/{name}");
                let r = run(
                    &ResolveAnchorArgs {
                        path: rel.clone(),
                        markers_path: Some("framework/constitution.md".into()),
                    },
                    repo,
                )
                .unwrap();
                judged += r.cross_document;
                broken.extend(
                    r.references
                        .iter()
                        .filter(|x| x.kind == KIND_CROSS_DOCUMENT && !x.resolved)
                        .map(|x| format!("{rel}:{} §{}", x.line, x.anchor)),
                );
            }
        }
        assert!(broken.is_empty(), "pointers naming no heading: {broken:#?}");
        assert!(
            judged >= 50,
            "only {judged} pointers judged — the subject went vacuous"
        );
    }

    /// A project at `<tmp>/project` whose `.ductus/config.toml` registers the
    /// sibling checkout `<tmp>/shared` (`config` is the registry text), whose
    /// `constitution.md` carries the markers `§a-b` and `§a-b-c`; the project's
    /// own markers file is `.ductus/constitution.md` with `§known`. `body` is
    /// written to `specs/001-x/spec.md` and scanned.
    fn scan_registered(config: &str, body: &str) -> ResolveAnchorResult {
        let tmp = tempfile::tempdir().unwrap();
        let project = tmp.path().join("project");
        let shared = tmp.path().join("shared");
        std::fs::create_dir_all(project.join(".ductus")).unwrap();
        std::fs::create_dir_all(project.join("specs/001-x")).unwrap();
        std::fs::create_dir_all(&shared).unwrap();
        std::fs::write(
            shared.join("constitution.md"),
            "# Shared\n\n<!-- §a-b -->\n\n## A b\n\n<!-- §a-b-c -->\n\n## A b c\n",
        )
        .unwrap();
        std::fs::write(project.join(".ductus/config.toml"), config).unwrap();
        std::fs::write(project.join(".ductus/constitution.md"), "<!-- §known -->\n").unwrap();
        std::fs::write(project.join("specs/001-x/spec.md"), body).unwrap();
        run(
            &ResolveAnchorArgs {
                path: "specs/001-x/spec.md".into(),
                markers_path: Some(".ductus/constitution.md".into()),
            },
            &project,
        )
        .unwrap()
    }

    const REGISTERED: &str =
        "[constitutions.shared]\nrepo = \"https://example.test/shared\"\npath = \"../shared\"\n";

    /// The scenario's minimal fixture: two references in the same qualified
    /// form, path and anchor on one line, into a registered shared
    /// constitution carrying both markers. They classify alike, whatever
    /// their anchors' shapes, and both resolve against that document.
    #[test]
    fn same_form_references_into_a_registered_constitution_classify_alike() {
        let r = scan_registered(
            REGISTERED,
            "Per (`../shared/constitution.md` §a-b) and (`../shared/constitution.md` §a-b-c).\n\
             See `../shared/constitution.md` §a-b-c.\n",
        );
        let shapes: Vec<(&str, bool, Option<&str>)> = r
            .references
            .iter()
            .map(|x| (x.kind.as_str(), x.resolved, x.unjudged.as_deref()))
            .collect();
        assert_eq!(shapes.len(), 3, "{:?}", r.references);
        assert!(
            shapes.windows(2).all(|w| w[0] == w[1]),
            "same form, different classification: {:?}",
            r.references
        );
        // Resolved by judgment against the registered document, not passed
        // over as a reference nothing could judge.
        assert!(
            shapes
                .iter()
                .all(|(_, resolved, unjudged)| *resolved && unjudged.is_none()),
            "{:?}",
            r.references
        );
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// A registered document's markers decide its references, whatever path
    /// reaches it: a present anchor resolves and a missing one is unresolved.
    #[test]
    fn a_reference_into_a_registered_constitution_is_judged_against_its_markers() {
        let r = scan_registered(
            REGISTERED,
            "See `../shared/constitution.md` §a-b and §gone.\n\
             From here: [the rule](../../../shared/constitution.md#a-b-c) §a-b-c.\n",
        );
        assert_eq!(r.unresolved, ["gone".to_string()]);
        assert!(
            r.references
                .iter()
                .all(|x| x.kind == KIND_QUALIFIED && x.unjudged.is_none()),
            "{:?}",
            r.references
        );
    }

    /// A registered entry that did not resolve is a document the config
    /// declares and the run could not read: unexamined, could not be read.
    #[test]
    fn a_reference_naming_a_skipped_registered_entry_is_unexamined_as_unreadable() {
        let config =
            "[constitutions.gone]\nrepo = \"https://example.test/gone\"\npath = \"../absent\"\n";
        let r = scan_registered(config, "See `../absent/constitution.md` §anything.\n");
        assert_eq!(r.references[0].unjudged.as_deref(), Some(UNREADABLE));
        assert_eq!(
            (r.anchor_target_unreadable, r.anchor_target_unjudgeable),
            (1, 0)
        );
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// The markers file is recognized by where a token resolves, never by a
    /// basename substring: a file sharing the basename, or one merely
    /// containing it, is another document.
    #[test]
    fn a_lookalike_of_the_markers_file_is_another_document() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("framework")).unwrap();
        std::fs::write(root.join("framework/constitution.md"), "<!-- §known -->\n").unwrap();
        std::fs::write(root.join("team-constitution.md"), "# Team\n\n## Rule\n").unwrap();
        std::fs::write(
            root.join("citing.md"),
            "See `team-constitution.md` §Rule.\n\
             See `notes/constitution.md.bak` §y.\n\
             See `../shared/constitution.md` §z.\n\
             See `framework/constitution.md` §gone.\n",
        )
        .unwrap();
        let r = run(
            &ResolveAnchorArgs {
                path: "citing.md".into(),
                markers_path: Some("framework/constitution.md".into()),
            },
            root,
        )
        .unwrap();
        let shapes: Vec<(&str, &str, Option<&str>)> = r
            .references
            .iter()
            .map(|x| (x.anchor.as_str(), x.kind.as_str(), x.unjudged.as_deref()))
            .collect();
        assert_eq!(
            shapes,
            [
                ("Rule", KIND_QUALIFIED, None),
                ("y", KIND_QUALIFIED, Some(UNJUDGEABLE)),
                ("z", KIND_QUALIFIED, Some(UNJUDGEABLE)),
                ("gone", KIND_MARKERS, None),
            ]
        );
        assert_eq!(r.unresolved, ["gone".to_string()]);
    }

    /// A tree holding `docs/guide.md` (heading `Install`), `docs/locked.md`,
    /// `scripts/run.sh` and the markers file `markers.md` (`§known`); `body`
    /// is written to `citing.md` and scanned.
    fn scan_tree(body: &str) -> ResolveAnchorResult {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::create_dir_all(root.join("scripts")).unwrap();
        std::fs::write(root.join("markers.md"), "<!-- §known -->\n").unwrap();
        std::fs::write(root.join("docs/guide.md"), "# Guide\n\n## Install\n").unwrap();
        std::fs::write(root.join("docs/locked.md"), "# Locked\n\n## Install\n").unwrap();
        std::fs::write(root.join("scripts/run.sh"), "#!/bin/sh\n").unwrap();
        std::fs::write(root.join("citing.md"), body).unwrap();
        run(
            &ResolveAnchorArgs {
                path: "citing.md".into(),
                markers_path: Some("markers.md".into()),
            },
            root,
        )
        .unwrap()
    }

    fn verdicts(r: &ResolveAnchorResult) -> Vec<(&str, bool, Option<&str>)> {
        r.references
            .iter()
            .map(|x| (x.anchor.as_str(), x.resolved, x.unjudged.as_deref()))
            .collect()
    }

    /// Decision (j): a reference on a line naming a readable document is
    /// judged against that document's headings, so a renamed section is
    /// caught rather than counted and passed over.
    #[test]
    fn a_reference_on_a_line_naming_a_readable_document_is_judged_against_it() {
        let r = scan_tree("See `docs/guide.md` §Install.\nSee `docs/guide.md` §Missing.\n");
        assert_eq!(
            verdicts(&r),
            [("Install", true, None), ("Missing", false, None)]
        );
        assert_eq!(r.unresolved, ["Missing".to_string()]);
        assert_eq!(r.qualified, 2);
    }

    /// A link to a markdown document by URL names a document this tree does
    /// not hold: the reference beside it is unexamined, not a claim about the
    /// markers file reported broken.
    #[test]
    fn a_url_to_a_markdown_document_names_an_absent_document() {
        let r = scan_tree(
            "See [spec 042](https://example.test/specs/042/spec.md) §Transition.\n\
             See [the site](https://example.test/page) §Elsewhere.\n",
        );
        let shapes: Vec<(&str, &str, Option<&str>)> = r
            .references
            .iter()
            .map(|x| (x.anchor.as_str(), x.kind.as_str(), x.unjudged.as_deref()))
            .collect();
        assert_eq!(
            shapes,
            [
                ("Transition", KIND_QUALIFIED, Some(UNJUDGEABLE)),
                ("Elsewhere", KIND_MARKERS, None),
            ]
        );
        assert_eq!(r.unresolved, ["Elsewhere".to_string()]);
    }

    /// A line naming only documents absent from the tree cannot be judged:
    /// unexamined, excluded by construction. Beside a readable document, an
    /// absent one does not excuse a reference neither carries.
    #[test]
    fn a_reference_naming_only_absent_documents_is_unexamined() {
        let r = scan_tree(
            "See `docs/none.md` §Whatever.\nSee `docs/none.md` and `docs/guide.md` §Gone.\n",
        );
        assert_eq!(
            verdicts(&r),
            [("Whatever", true, Some(UNJUDGEABLE)), ("Gone", false, None)]
        );
        assert_eq!(r.anchor_target_unjudgeable, 1);
        assert_eq!(r.unresolved, ["Gone".to_string()]);
    }

    /// A document that exists and cannot be read is unexamined, could not be
    /// read.
    #[cfg(unix)]
    #[test]
    fn a_reference_naming_an_unreadable_document_is_unexamined_as_unreadable() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("markers.md"), "<!-- §known -->\n").unwrap();
        std::fs::write(root.join("locked.md"), "## Install\n").unwrap();
        std::fs::set_permissions(
            root.join("locked.md"),
            std::fs::Permissions::from_mode(0o000),
        )
        .unwrap();
        if std::fs::read_to_string(root.join("locked.md")).is_ok() {
            // Running as root: permissions do not bind, so nothing to test.
            return;
        }
        std::fs::write(root.join("citing.md"), "See `locked.md` §Install.\n").unwrap();
        let r = run(
            &ResolveAnchorArgs {
                path: "citing.md".into(),
                markers_path: Some("markers.md".into()),
            },
            root,
        )
        .unwrap();
        assert_eq!(r.references[0].unjudged.as_deref(), Some(UNREADABLE));
        assert_eq!(r.anchor_target_unreadable, 1);
        assert_eq!(r.unresolved, [] as [String; 0]);
    }

    /// Decision (10a): a file that is not markdown holds no section, so a
    /// reference beside one that matches no marker or heading is unexamined,
    /// excluded by construction, never the false alarm it was.
    #[test]
    fn a_reference_beside_a_non_markdown_file_is_unexamined() {
        let r = scan_tree(
            "Set it in `.ductus/config.toml` §constitutions.\n\
             And [the table](config.toml) §artifacts.\n\
             See `docs/guide.md` and `scripts/run.sh` §Install.\n\
             See `docs/guide.md` and `scripts/run.sh` §Nope.\n\
             Per `scripts/run.sh` §known.\n",
        );
        assert_eq!(
            verdicts(&r),
            [
                ("constitutions", true, Some(UNJUDGEABLE)),
                ("artifacts", true, Some(UNJUDGEABLE)),
                ("Install", true, None),
                ("Nope", true, Some(UNJUDGEABLE)),
                ("known", true, None),
            ]
        );
        assert_eq!(r.unresolved, [] as [String; 0]);
        assert_eq!(r.anchor_target_unjudgeable, 3);
    }

    /// Decision (c): a code span whose whole content is a path followed by one
    /// reference is a citation of that path's section, judged; any other span
    /// still holds no reference.
    #[test]
    fn a_span_holding_a_path_and_one_reference_is_judged() {
        let r = scan_tree(
            "Per `docs/guide.md §Install` and `docs/guide.md §Missing`.\n\
             Not one: `docs/guide.md §Install and §Other`, `see docs/guide.md §Install`.\n",
        );
        assert_eq!(
            verdicts(&r),
            [("Install", true, None), ("Missing", false, None)]
        );
        assert_eq!(r.unresolved, ["Missing".to_string()]);
    }

    /// A shipped destination whose source is the markers file is the markers
    /// file: framework files cite the constitution by the path it lands at
    /// in an adopter (`.ductus/constitution.md`), and those references stay
    /// claims about the markers.
    #[test]
    fn a_shipped_destination_of_the_markers_file_is_the_markers_file() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("framework/bootstrap")).unwrap();
        std::fs::write(
            root.join(BOOTSTRAP),
            "# ductus\n\n## Shared Files\n\n| Source | Destination |\n| --- | --- |\n\
             | `framework/constitution.md` | `.ductus/constitution.md` |\n",
        )
        .unwrap();
        std::fs::write(root.join("framework/constitution.md"), "<!-- §known -->\n").unwrap();
        std::fs::write(
            root.join("citing.md"),
            "Per `.ductus/constitution.md` §known and §gone.\n",
        )
        .unwrap();
        let r = run(
            &ResolveAnchorArgs {
                path: "citing.md".into(),
                markers_path: Some("framework/constitution.md".into()),
            },
            root,
        )
        .unwrap();
        assert_eq!(kinds(&r), [KIND_MARKERS, KIND_MARKERS]);
        assert_eq!(r.unresolved, ["gone".to_string()]);
    }

    #[test]
    fn resolves_references_against_a_separate_markers_file() {
        // A spec cites `§known` and `§renamed`; markers live only in a
        // separate constitution file. With `markers-path` pointed at it,
        // `§known` resolves and `§renamed` is the only unresolved one —
        // rather than every reference firing as unresolved noise.
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("spec.md"),
            "# Spec\n\nSee [§known](constitution.md#known) and [§renamed](constitution.md#renamed).\n",
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("constitution.md"),
            "# Constitution\n\n<!-- §known -->\n## Known\n",
        )
        .unwrap();
        let result = run(
            &ResolveAnchorArgs {
                path: "spec.md".into(),
                markers_path: Some("constitution.md".into()),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unresolved, vec!["renamed".to_string()]);
        assert!(
            result
                .references
                .iter()
                .any(|r| r.anchor == "known" && r.resolved)
        );
    }
}
