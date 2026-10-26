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
//!   maps to a source this primitive can read, so it *can* be judged, and is.
//!   Two shapes reach it. An explicit pair — a bold `§X` followed by `in` and
//!   a shipped document in a code span, as in **§Grounding (advisory)** in
//!   `.ductus/reference/analyze.md` — resolves only when `X` *equals* a
//!   heading of that document: the bold delimits the name, so a pointer that
//!   outlives a shortened heading is caught rather than prefix-matched.
//!   Otherwise a line naming **exactly one** document, which the manifest
//!   maps, judges each `§` on it: resolved when it begins with a heading of
//!   that source, a heading of the citing file, or names a marker. The marker
//!   fallback is required — a line naming a shipped document such as
//!   `specs/inbox.md` usually cites a constitution section beside it; the
//!   citing-file fallback lets a pointer line also cite a section of its own
//!   file, and no reference in the corpus needs it today. Pairs exist because
//!   most command steps name `plan.md` or `spec.md` beside their pointer:
//!   measured 2026-10-25, 59 of 100 pointers sat on such lines, which the line
//!   rule leaves unjudged. The manifest bounds all of it — it is the loading
//!   policy, the source path is the document root, and an unreadable source
//!   leaves the reference `qualified` — so this is not the general link
//!   resolver `anchor-reference-kinds` declined to become. Where the
//!   bootstrap file is absent, as in every adopter, the map is empty and
//!   nothing is judged this way.
//! - **`qualified`** — the reference's line names another document (a `.md`
//!   filename, backticked or bare, or a markdown link to one) that the rule
//!   above does not judge. It is a claim about *that* document's sections,
//!   and this primitive holds one file's markers, so it is not evaluated
//!   here. Excluded by construction and **counted** — the exclusion is what
//!   makes the rest readable, and an exclusion nobody counts is
//!   indistinguishable from having found nothing. Line-scoped rather than
//!   immediately-preceding on purpose: the dominant real shape is a table row
//!   or a clause naming the file once and then citing several of its sections
//!   (`review.md | edit — §Behavior step 5 and §Load`), where every reference
//!   after the first would otherwise slip through.
//! - **`intra-document`** — the anchor names a heading in the citing file
//!   (`per §Design above`). Resolved against that file's own headings, which
//!   is the question the author was actually asking.
//! - **`markers`** — everything else: a genuine claim about the markers file.
//!
//! Only `cross-document` and `markers` references can come back unresolved.
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
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::{Result, SkipScanner, contained_path, inline_code_spans, read_text};
use crate::schema::primitives::{AnchorReference, ResolveAnchorArgs, ResolveAnchorResult};

/// Execute the `resolve-anchor` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::Io`] when the file cannot
/// be read, and [`crate::primitives::PrimitiveError::InvalidPath`], before
/// anything is read, when `path` or `markers-path` is absolute, climbs out
/// with `..`, or leaves the project through a symlink
/// ([`crate::primitives::contained_path`]).
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
    let markers_names = markers_identifiers(args.markers_path.as_ref());
    let mut shipped = Shipped::load(repo);
    let mut references: Vec<AnchorReference> = Vec::new();
    let mut unresolved: HashSet<String> = HashSet::new();
    let mut qualified = 0u32;
    let mut intra_document = 0u32;
    let mut cross_document = 0u32;
    for (line_no, line) in content.lines().enumerate() {
        let line_no = u32::try_from(line_no + 1).unwrap_or(u32::MAX);
        let line_names_document = names_other_document(line, &markers_names);
        // The one shipped document this line names, when it names exactly one
        // and its source can be read: the line's references can be judged.
        let judged_against = if line_names_document {
            shipped.line_headings(line, &markers_names)
        } else {
            None
        };
        let pairs = shipped.pairs(line);
        let code_spans = inline_code_spans(line);
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
            // `check-corpus-links` applies to link targets in code spans.
            if code_spans.iter().any(|span| span.contains(&whole)) {
                continue;
            }
            // Order matters. A reference that points into a shipped document
            // is judged as cross-document first. Otherwise a line that names
            // another document is qualified even when the anchor happens to
            // match a local heading, because the author named the document
            // they meant.
            let rest = &line[whole..];
            // An explicit pair is judged against its document alone, by
            // equality; a line naming one shipped document also accepts a
            // local heading or a marker, since a pointer line can cite either
            // beside it.
            let cross = pairs.get(&whole).copied().or_else(|| {
                judged_against.as_ref().map(|h| {
                    names_local_heading(h, rest)
                        || names_local_heading(&own_headings, rest)
                        || markers.contains(&anchor)
                })
            });
            let (kind, resolved) = if let Some(resolved) = cross {
                cross_document = cross_document.saturating_add(1);
                if !resolved {
                    unresolved.insert(anchor.clone());
                }
                (KIND_CROSS_DOCUMENT, resolved)
            } else if line_names_document {
                qualified = qualified.saturating_add(1);
                (KIND_QUALIFIED, true)
            } else if names_local_heading(&own_headings, rest) {
                intra_document = intra_document.saturating_add(1);
                (KIND_INTRA_DOCUMENT, true)
            } else {
                let resolved = markers.contains(&anchor);
                if !resolved {
                    unresolved.insert(anchor.clone());
                }
                (KIND_MARKERS, resolved)
            };
            references.push(AnchorReference {
                anchor,
                line: line_no,
                resolved,
                kind: kind.to_string(),
            });
        }
    }

    let mut unresolved: Vec<String> = unresolved.into_iter().collect();
    unresolved.sort();
    Ok(ResolveAnchorResult {
        references,
        unresolved,
        qualified,
        intra_document,
        cross_document,
    })
}

const KIND_MARKERS: &str = "markers";
const KIND_QUALIFIED: &str = "qualified";
const KIND_INTRA_DOCUMENT: &str = "intra-document";
const KIND_CROSS_DOCUMENT: &str = "cross-document";

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

    /// The headings of `source`, or `None` when it cannot be read — which
    /// leaves the references that would be judged against it `qualified`.
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

    /// The headings of the one shipped document `line` names, when it names
    /// exactly one document other than the markers file.
    fn line_headings(&mut self, line: &str, markers_names: &[String]) -> Option<Vec<String>> {
        let source = single_shipped_source(line, markers_names, &self.sources)?;
        self.headings_of(&source)
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

/// The source of the one shipped document `line` names, when the line names
/// exactly one document other than the markers file and the manifest maps it.
/// A line naming two documents is left `qualified`: which one a `§` means is
/// a judgment this primitive does not make.
fn single_shipped_source(
    line: &str,
    markers_names: &[String],
    shipped: &HashMap<String, String>,
) -> Option<String> {
    if shipped.is_empty() {
        return None;
    }
    let mut named: Vec<&str> = document_regex()
        .find_iter(line)
        .map(|m| m.as_str())
        .filter(|token| {
            !markers_names
                .iter()
                .any(|name| token.contains(name.as_str()))
        })
        .map(document_path)
        .collect();
    named.sort_unstable();
    named.dedup();
    match named.as_slice() {
        [only] => shipped.get(*only).cloned(),
        _ => None,
    }
}

/// The path a [`document_regex`] match names: a code span's contents, a link
/// target without its fragment, or a bare token without its leading delimiter.
fn document_path(token: &str) -> &str {
    let token = token.trim_start_matches(|c: char| c.is_whitespace() || c == '(' || c == '[');
    if let Some(inner) = token.strip_prefix('`').and_then(|t| t.strip_suffix('`')) {
        return inner;
    }
    if let Some(target) = token.strip_prefix("](").and_then(|t| t.strip_suffix(')')) {
        return target.split('#').next().unwrap_or(target);
    }
    token
}

/// Whether `line` names a markdown document **other than the markers file** —
/// a backticked path, a markdown link target, or a bare `*.md` token.
///
/// The "other than" is load-bearing and was not in the first draft: a line
/// citing the markers file itself (`[§known](constitution.md#known)`, or
/// `framework/constitution.md §grounding`) names a document, but the
/// document it names is precisely the one whose markers are in hand. Treating
/// it as qualified would exclude the single kind of reference this primitive
/// exists to check. An existing test caught it, which is the argument for
/// having had one.
///
/// Line-scoped by design; see the module docs for why immediately-preceding
/// is the wrong window.
fn names_other_document(line: &str, markers_names: &[String]) -> bool {
    document_regex().find_iter(line).any(|m| {
        let token = m.as_str();
        !markers_names
            .iter()
            .any(|name| token.contains(name.as_str()))
    })
}

/// The path and basename by which a citation might name the markers file.
/// Both forms occur in the corpus — a spec writes the repo-relative path, a
/// scenario one directory deeper writes a `../` path whose basename is all
/// that reliably matches.
fn markers_identifiers(markers_path: Option<&String>) -> Vec<String> {
    let Some(path) = markers_path else {
        return Vec::new();
    };
    let mut names = vec![path.clone()];
    if let Some(base) = Path::new(path).file_name().and_then(|n| n.to_str()) {
        names.push(base.to_string());
    }
    names.sort_by_key(|n| std::cmp::Reverse(n.len()));
    names.dedup();
    names
}

fn document_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"`[^`]*\.md[^`]*`|\]\([^)]*\.md[^)]*\)|(?:^|[\s(\[])[\w.-]+\.md\b")
            .expect("hard-coded regex compiles")
    })
}

/// ATX heading texts of a document. A line inside a fenced code block or an
/// HTML comment is content, not structure ([`SkipScanner`]): the fenced
/// example record in review's reference holds `## Summary`, which names no
/// section a pointer can reach.
fn collect_headings(content: &str) -> Vec<String> {
    let mut skip = SkipScanner::default();
    let mut headings: Vec<String> = content
        .lines()
        .filter(|line| !skip.skip(line))
        .filter_map(|line| {
            let trimmed = line.trim_start_matches('#');
            (trimmed.len() < line.len() && trimmed.starts_with(' '))
                .then(|| trimmed.trim().to_string())
        })
        .filter(|h| !h.is_empty())
        .collect();
    headings.sort_unstable();
    headings.dedup();
    headings
}

/// Whether the text starting at the `§` begins with one of `headings` — the
/// citing file's own, or a shipped source's under the line rule. A `§` in
/// running prose has nothing marking where its name ends, so a heading the
/// text begins with resolves it when the heading ends where a word ends; an
/// explicit pair, whose bold does mark the end, is compared by equality
/// instead (`Shipped::pairs`). `rest` begins with
/// the `§`, so the heading is compared against what follows it.
fn names_local_heading(headings: &[String], rest: &str) -> bool {
    let after = rest.strip_prefix('§').unwrap_or(rest);
    headings.iter().any(|h| {
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
    /// document is excluded — the author named the document they meant, and
    /// guessing otherwise is how a rule starts inventing findings.
    #[test]
    fn a_resolvable_anchor_on_a_qualified_line_is_still_qualified() {
        let r = classify("Per §known, see `other.md` for the detail.\n");
        assert_eq!(r.qualified, 1);
        assert_eq!(r.references[0].kind, "qualified");
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

    #[test]
    fn a_pair_naming_an_unmapped_document_stays_qualified() {
        let r = judge("Read `plan.md`, then **§Nope** in `.ductus/reference/zzz.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
    }

    #[test]
    fn a_line_naming_two_documents_stays_qualified() {
        // Not a pair: without the `**§X** in` form nothing says which of
        // the two documents the section belongs to.
        let r = judge("See §Nope, in `.ductus/reference/x.md` and `other.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
        assert_eq!((r.cross_document, r.qualified), (0, 1));
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

    #[test]
    fn a_mapped_source_that_cannot_be_read_stays_qualified() {
        let r = judge("**§Nope** in `.ductus/reference/missing.md`.\n");
        assert_eq!(kinds(&r), ["qualified"]);
        assert_eq!(r.unresolved, [] as [String; 0]);
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
