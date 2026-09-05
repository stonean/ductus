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
//! Three kinds, and the classification is what makes an unresolved reference
//! worth reading:
//!
//! - **`qualified`** — the reference's line names another document (a `.md`
//!   filename, backticked or bare, or a markdown link to one). It is a claim
//!   about *that* document's sections, and this primitive holds one file's
//!   markers, so it cannot be evaluated here. Excluded by construction and
//!   **counted** — the exclusion is what makes the rest readable, and an
//!   exclusion nobody counts is indistinguishable from having found nothing.
//!   Line-scoped rather than immediately-preceding on purpose: the dominant
//!   real shape is a table row or a clause naming the file once and then
//!   citing several of its sections (`review.md | edit — §Behavior step 5 and
//!   §Load`), where every reference after the first would otherwise slip
//!   through.
//! - **`intra-document`** — the anchor names a heading in the citing file
//!   (`per §Design above`). Resolved against that file's own headings, which
//!   is the question the author was actually asking.
//! - **`markers`** — everything else: a genuine claim about the markers file,
//!   and the only kind that can come back unresolved.
//!
//! What stays reported is deliberate. `the bootstrap's §Derived values` and
//! `spec 022 §Versioning` name another document in prose without naming the
//! file, so they cannot be verified *or* excluded; reporting them is the
//! honest answer, and 31 of the 112 are this shape. The rule is exact — it
//! excludes only references whose line demonstrably names another document —
//! rather than a heuristic that would have swallowed `010`'s real one.

#![allow(clippy::expect_used)]

use std::collections::HashSet;
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::{Result, inline_code_spans, read_text, resolve_path};
use crate::schema::primitives::{AnchorReference, ResolveAnchorArgs, ResolveAnchorResult};

/// Execute the `resolve-anchor` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::Io`] when the file cannot
/// be read.
pub fn run(args: &ResolveAnchorArgs, repo: &Path) -> Result<ResolveAnchorResult> {
    let path = resolve_path(repo, &args.path);
    let content = read_text(&path)?;

    // Markers come from `markers-path` when supplied, else from the scanned
    // file itself (same-file self-consistency check).
    let markers = match &args.markers_path {
        Some(markers_path) => {
            let marker_content = read_text(&resolve_path(repo, markers_path))?;
            collect_markers(&marker_content)
        }
        None => collect_markers(&content),
    };
    let own_headings = collect_headings(&content);
    let markers_names = markers_identifiers(args.markers_path.as_ref());
    let mut references: Vec<AnchorReference> = Vec::new();
    let mut unresolved: HashSet<String> = HashSet::new();
    let mut qualified = 0u32;
    let mut intra_document = 0u32;
    for (line_no, line) in content.lines().enumerate() {
        let line_no = u32::try_from(line_no + 1).unwrap_or(u32::MAX);
        let line_names_document = names_other_document(line, &markers_names);
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
            // Order matters: a line that names another document is qualified
            // even when the anchor happens to match a local heading, because
            // the author named the document they meant.
            let (kind, resolved) = if line_names_document {
                qualified = qualified.saturating_add(1);
                (KIND_QUALIFIED, true)
            } else if names_local_heading(&own_headings, &line[whole..]) {
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
    })
}

const KIND_MARKERS: &str = "markers";
const KIND_QUALIFIED: &str = "qualified";
const KIND_INTRA_DOCUMENT: &str = "intra-document";

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

/// ATX heading texts in the citing file, longest first so a `§Hook Installation`
/// is matched against `Hook Installation` rather than a shorter `Hook`.
fn collect_headings(content: &str) -> Vec<String> {
    let mut headings: Vec<String> = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start_matches('#');
            (trimmed.len() < line.len() && trimmed.starts_with(' '))
                .then(|| trimmed.trim().to_string())
        })
        .filter(|h| !h.is_empty())
        .collect();
    headings.sort_by_key(|h| std::cmp::Reverse(h.len()));
    headings.dedup();
    headings
}

/// Whether the text starting at the `§` names one of the citing file's own
/// headings. `rest` begins with the `§`, so the heading is compared against
/// what follows it.
fn names_local_heading(headings: &[String], rest: &str) -> bool {
    let after = rest.strip_prefix('§').unwrap_or(rest);
    headings.iter().any(|h| after.starts_with(h.as_str()))
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
        assert!(r.unresolved.is_empty());
        assert!(r.references.iter().all(|x| x.kind == "qualified"));
        assert!(r.references.iter().all(|x| x.resolved));
    }

    #[test]
    fn a_markdown_link_to_a_document_qualifies_its_line() {
        let r = classify("See [017](../017-derive-dont-ask/spec.md) §Generators for the rule.\n");
        assert_eq!(r.qualified, 1);
        assert!(r.unresolved.is_empty());
    }

    #[test]
    fn a_bare_md_filename_qualifies_its_line() {
        let r = classify("The rule is in AGENTS.md §Workflow.\n");
        assert_eq!(r.qualified, 1);
        assert!(r.unresolved.is_empty());
    }

    #[test]
    fn an_anchor_naming_a_heading_here_resolves_against_this_file() {
        let r = classify("## Design\n\nRewrite the ladder per §Design above.\n");
        assert_eq!(r.intra_document, 1);
        assert_eq!(r.qualified, 0);
        assert!(r.unresolved.is_empty());
        assert_eq!(r.references[0].kind, "intra-document");
    }

    /// Longest-heading-first matching: `§Hook Installation` must not be
    /// satisfied by a shorter `## Hook` heading elsewhere in the file when
    /// the longer one exists.
    #[test]
    fn the_longest_matching_heading_wins() {
        let r = classify("## Hook\n\n## Hook Installation\n\nSee §Hook Installation below.\n");
        assert_eq!(r.intra_document, 1);
        assert!(r.unresolved.is_empty());
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
        assert!(r.references.is_empty());
        assert!(r.unresolved.is_empty());
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
                path: path.to_string_lossy().into(),
                markers_path: None,
            },
            tmp.path(),
        )
        .unwrap();
        assert!(result.references.is_empty());
        assert!(result.unresolved.is_empty());
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
