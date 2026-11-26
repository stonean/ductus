//! `prune-project`'s move of a project-file pointer's target into a parts
//! directory (spec 087, plan *`prune-project` walks parts and moves a
//! pointer's target*).
//!
//! Every call lists the move candidates: the markdown files a project file or
//! part links to that sit outside the parts directories, the spec root and
//! `.ductus/`. A preview naming one move runs the literal search for its
//! `from` over every tracked file but `from`, the spec root included, and
//! lists each referrer with its occurrences. An apply naming the move and the
//! referrers the operator confirmed checks each against the tree as it now
//! is, then makes the move under one hold of the compare-and-set lock
//! ([`commit_if_unchanged`]): `to` written create-only with the moved file's
//! relative links rewritten for its new directory, each confirmed referrer's
//! occurrences rewritten, and `from` removed — or, when any of those files
//! changed since the apply read it, nothing at all (the agent's
//! recommendation, adopted by the operator, 2026-11-26).
//!
//! An occurrence is `from`'s project-relative path with a boundary on each
//! side ([`occurrences`]), and one function finds those a preview counts and
//! an apply rewrites, so the two cannot disagree.

use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;

use super::{Outcome, PRIMITIVE};
use crate::primitives::check_corpus_links::{
    LinkFilePart, checked_lines, lexical_join, link_file_part, link_target_ranges,
};
use crate::primitives::{
    Change, PrimitiveError, ReadSnapshot, Result, commit_if_unchanged, leaves_project,
    project_parts, rel_path, tracked_project_paths, validate_contained,
};
use crate::schema::paths::DUCTUS_DIR;
use crate::schema::primitives::{
    FileMove, MoveCandidate, MoveReferrer, ProjectFileKind, ProjectSearch, SearchState,
};

/// The move candidates among the links of every subject read as text: each
/// markdown file inside the project that exists, outside every parts
/// directory, the spec root and `.ductus/`, and none of the four project
/// files, with the subjects linking it in subject order, sorted by path. A
/// link in a fenced block or an HTML comment names no candidate, as it names
/// no part ([`project_parts::index_links`]), and a target that is, or lies
/// under, a symbolic link leaving the project is never probed.
pub(super) fn candidates(
    repo: &Path,
    root: &str,
    subjects: &[(ProjectFileKind, &str, Option<&str>)],
    texts: &[Option<&str>],
) -> Vec<MoveCandidate> {
    let project_file = |path: &str| {
        subjects
            .iter()
            .any(|(_, rel, part_of)| part_of.is_none() && *rel == path)
    };
    let mut found: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ((_, rel, _), text) in subjects.iter().zip(texts) {
        let Some(text) = text else {
            continue;
        };
        for link in project_parts::index_links(text, rel, repo) {
            let name = link.rsplit('/').next().unwrap_or(&link);
            let movable = project_parts::is_part_name(name)
                && !project_file(&link)
                && !project_parts::in_dir(&link, root)
                && !project_parts::in_dir(&link, DUCTUS_DIR)
                && !leaves_project(repo, &link)
                && repo.join(&link).is_file();
            if movable {
                let linked_from = found.entry(link).or_default();
                if !linked_from.iter().any(|from| from == rel) {
                    linked_from.push((*rel).to_string());
                }
            }
        }
    }
    found
        .into_iter()
        .map(|(path, linked_from)| MoveCandidate { path, linked_from })
        .collect()
}

/// Refuse a move whose `to` is not a new markdown file directly under a parts
/// directory, or whose `from` is no move candidate, before its search or its
/// apply reads anything.
pub(super) fn check(repo: &Path, mv: &FileMove, candidates: &[MoveCandidate]) -> Result<()> {
    let invalid = |reason: String| PrimitiveError::InvalidArgument {
        primitive: PRIMITIVE.into(),
        argument: "move".into(),
        reason,
    };
    if project_parts::part_of(&mv.to).is_none() {
        return Err(invalid(format!(
            "`{}` is not a markdown file directly under a parts directory ({})",
            mv.to,
            project_parts::KINDS
                .map(|kind| format!("`{}/`", project_parts::parts_dir(kind)))
                .join(", ")
        )));
    }
    validate_contained(repo, &mv.to)?;
    if repo.join(&mv.to).symlink_metadata().is_ok() {
        return Err(invalid(format!(
            "`{}` exists: a move writes a new part",
            mv.to
        )));
    }
    if !candidates.iter().any(|candidate| candidate.path == mv.from) {
        return Err(invalid(format!(
            "`{}` is not a move candidate: no project file or part links it, it does not exist, \
             or it lies in a parts directory, the spec root or `.ductus/`",
            mv.from
        )));
    }
    Ok(())
}

/// Run the literal search for `from` over every tracked file but `from`, the
/// spec root included, and list each file holding it with its occurrences,
/// in path order. A file that is, or lies under, a symbolic link resolving
/// outside the project is not read and is named in `outside-project`; with
/// no repository the search is `unavailable` and lists no referrer, never
/// zero of them.
pub(super) fn search(
    repo: &Path,
    from: &str,
) -> Result<(ProjectSearch, Option<Vec<MoveReferrer>>)> {
    let Some(tracked) = tracked_project_paths(repo)? else {
        return Ok((
            ProjectSearch {
                state: SearchState::Unavailable,
                files_searched: None,
                reason: Some(
                    "no git repository contains the project, so no tracked file was read".into(),
                ),
                outside_project: None,
                outside_project_count: None,
            },
            None,
        ));
    };
    let mut referrers = Vec::new();
    let mut outside_project = Vec::new();
    let mut files_searched = 0;
    for path in tracked.iter().filter(|path| *path != from) {
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
        let found = occurrences(&bytes, from.as_bytes()).len();
        if found > 0 {
            referrers.push(MoveReferrer {
                path: path.clone(),
                occurrences: found,
            });
        }
    }
    Ok((
        ProjectSearch {
            state: SearchState::Ran,
            files_searched: Some(files_searched),
            reason: None,
            outside_project_count: Some(outside_project.len()),
            outside_project: Some(outside_project),
        },
        Some(referrers),
    ))
}

/// Make the move with the referrers the operator confirmed.
///
/// Each confirmed referrer is checked first, against the tree as it now is:
/// one that is untracked, `from` itself, outside the project, unreadable or
/// holding no occurrence is stale, and any stale one refuses the whole apply
/// before anything is written. Then, under one hold of the compare-and-set
/// lock, `to`, each referrer and `from` are read again; one that changed
/// since this apply read it refuses the whole move, and each that changed is
/// named. Otherwise `to` is created with the moved file's relative links
/// rewritten, at `from`'s permissions, each referrer is rewritten, and
/// `from` is removed.
pub(super) fn apply(repo: &Path, mv: &FileMove, before_write: impl FnOnce()) -> Result<Outcome> {
    let from = ReadSnapshot::read(repo, &mv.from)?;
    let Some(text) = from.text()? else {
        // Gone since `check` found it a candidate: the tree changed.
        return Ok(Outcome {
            refused: vec![mv.from.clone()],
            stale_referrers: Some(Vec::new()),
            ..Outcome::default()
        });
    };
    let to = ReadSnapshot::read(repo, &mv.to)?;
    let tracked = tracked_project_paths(repo)?.unwrap_or_default();
    let mut confirmed: Vec<&str> = mv.referrers.iter().map(String::as_str).collect();
    confirmed.sort_unstable();
    confirmed.dedup();
    let mut stale = Vec::new();
    let mut referrers = Vec::new();
    for path in confirmed {
        let found = (path != mv.from && tracked.binary_search_by(|t| t.as_str().cmp(path)).is_ok())
            .then(|| ReadSnapshot::read(repo, path).ok())
            .flatten()
            .filter(|snapshot| {
                snapshot
                    .bytes()
                    .is_some_and(|bytes| !occurrences(bytes, mv.from.as_bytes()).is_empty())
            });
        match found {
            Some(snapshot) => {
                let rewritten = substitute(snapshot.bytes().unwrap_or_default(), &mv.from, &mv.to);
                referrers.push((path.to_string(), snapshot, rewritten));
            }
            None => stale.push(path.to_string()),
        }
    }
    if !stale.is_empty() {
        return Ok(Outcome {
            stale_referrers: Some(stale),
            ..Outcome::default()
        });
    }

    let moved = relocate_links(text, &mv.from, &mv.to, repo);
    // Not `mirror_source_mode`: the commit below removes `from`, so its
    // permissions are read here, before it.
    let permissions = std::fs::metadata(repo.join(&mv.from))
        .ok()
        .map(|meta| meta.permissions());
    before_write();
    let mut changes = vec![(&to, Change::Create(&moved))];
    changes.extend(
        referrers
            .iter()
            .map(|(_, snapshot, bytes)| (snapshot, Change::Write(bytes))),
    );
    changes.push((&from, Change::Remove));
    let refused = commit_if_unchanged(&changes)?;
    if !refused.is_empty() {
        return Ok(Outcome {
            refused,
            stale_referrers: Some(Vec::new()),
            ..Outcome::default()
        });
    }
    if let Some(permissions) = permissions {
        // [`commit_if_unchanged`] creates `to` at its tempfile's `0600`.
        let to_path = repo.join(&mv.to);
        std::fs::set_permissions(&to_path, permissions).map_err(|source| PrimitiveError::Io {
            path: to_path,
            source,
        })?;
    }
    let mut written = vec![mv.to.clone()];
    written.extend(referrers.into_iter().map(|(path, _, _)| path));
    Ok(Outcome {
        applied: true,
        written,
        stale_referrers: Some(Vec::new()),
        ..Outcome::default()
    })
}

/// The byte offset of each occurrence of `needle` in `hay`, left to right and
/// never overlapping, with a boundary on each side (the agent's
/// recommendation, adopted by the operator, 2026-11-26). Before it, once a
/// run of `./` and `../` segments and a root `/` are set aside, no path
/// character joins it, so `foo/docs/x.md` holds no `docs/x.md` while
/// `../../docs/x.md` does. After it no path character follows, and a `.`
/// followed by one is a longer name, so `docs/x.mdx` and `docs/x.md.orig`
/// hold none while `docs/x.md#part` and a sentence's closing `docs/x.md.`
/// do.
pub(super) fn occurrences(hay: &[u8], needle: &[u8]) -> Vec<usize> {
    let mut out = Vec::new();
    if needle.is_empty() {
        return out;
    }
    let mut at = 0;
    while let Some(pos) = hay[at..]
        .windows(needle.len())
        .position(|window| window == needle)
    {
        let start = at + pos;
        let end = start + needle.len();
        if bounded_before(hay, start) && bounded_after(hay, end) {
            out.push(start);
            at = end;
        } else {
            at = start + 1;
        }
    }
    out
}

/// A byte that can continue a file name: a letter, a digit, `_`, `-`, or any
/// byte of a multi-byte character.
fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-' || byte >= 0x80
}

/// Whether the occurrence starting at `start` has a boundary before it.
fn bounded_before(hay: &[u8], start: usize) -> bool {
    let mut at = start;
    loop {
        if hay[..at].ends_with(b"../") {
            at -= 3;
        } else if hay[..at].ends_with(b"./") {
            at -= 2;
        } else {
            break;
        }
    }
    if hay[..at].ends_with(b"/") {
        at -= 1;
    }
    at == 0 || !(is_name_byte(hay[at - 1]) || matches!(hay[at - 1], b'.' | b'/'))
}

/// Whether the occurrence ending at `end` has a boundary after it.
fn bounded_after(hay: &[u8], end: usize) -> bool {
    match hay.get(end) {
        None => true,
        Some(b'.') => !hay.get(end + 1).copied().is_some_and(is_name_byte),
        Some(&byte) => !is_name_byte(byte) && byte != b'/',
    }
}

/// `hay` with each of `from`'s [`occurrences`] replaced by `to`, so the text
/// before each, a relative prefix included, is kept.
fn substitute(hay: &[u8], from: &str, to: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(hay.len());
    let mut last = 0;
    for start in occurrences(hay, from.as_bytes()) {
        out.extend_from_slice(&hay[last..start]);
        out.extend_from_slice(to.as_bytes());
        last = start + from.len();
    }
    out.extend_from_slice(&hay[last..]);
    out
}

/// `text`, the moved file's body at `from`, with each relative link
/// `check-corpus-links` reads ([`checked_lines`], [`link_target_ranges`])
/// rewritten for `to`'s directory: resolved against `from`'s, a link to
/// `from` itself taken to `to`, and made relative again, its fragment kept.
/// A fragment alone, a scheme, a documentation shape, a root-absolute target
/// and one that leaves the project are left as they are, and so is a literal
/// mention of `from`'s path, which the search excepts.
fn relocate_links(text: &str, from: &str, to: &str, repo: &Path) -> String {
    let here = repo.join(from);
    let here = here.parent().unwrap_or(repo);
    let to_dir = to.rsplit_once('/').map_or("", |(dir, _)| dir);
    let mut edits: Vec<(Range<usize>, String)> = Vec::new();
    for (_, line) in checked_lines(text) {
        let body = &text[line.clone()];
        for target in link_target_ranges(body) {
            let LinkFilePart::File(rel) = link_file_part(&body[target.clone()]) else {
                continue;
            };
            if rel.starts_with('/') {
                continue;
            }
            let resolved = lexical_join(here, rel);
            if resolved.strip_prefix(repo).is_err() {
                continue;
            }
            let mut path = rel_path(&resolved, repo);
            if path == from {
                path = to.to_string();
            }
            let mut moved = relative(to_dir, &path);
            if rel.ends_with('/') && !moved.ends_with('/') {
                moved.push('/');
            }
            if moved != rel {
                let at = line.start + target.start;
                edits.push((at..at + rel.len(), moved));
            }
        }
    }
    let mut out = text.to_string();
    for (range, with) in edits.into_iter().rev() {
        out.replace_range(range, &with);
    }
    out
}

/// The relative path from the project-relative directory `dir` to the
/// project-relative `target`, `.` for `dir` itself.
fn relative(dir: &str, target: &str) -> String {
    let dir: Vec<&str> = dir.split('/').filter(|part| !part.is_empty()).collect();
    let target: Vec<&str> = target.split('/').filter(|part| !part.is_empty()).collect();
    let common = dir.iter().zip(&target).take_while(|(a, b)| a == b).count();
    let mut parts = vec![".."; dir.len() - common];
    parts.extend(&target[common..]);
    if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn found(hay: &str, needle: &str) -> Vec<usize> {
        occurrences(hay.as_bytes(), needle.as_bytes())
    }

    /// A longer path ending in `from` holds no occurrence, while a relative
    /// prefix, a root `/`, a quote, a backtick and a parenthesis each bound
    /// one (spec 087, AC14's search).
    #[test]
    fn an_occurrence_needs_a_boundary_before_it() {
        let from = "docs/x.md";
        for hay in [
            "foo/docs/x.md",
            "xdocs/x.md",
            "my-docs/x.md",
            ".ductus/docs/x.md",
            "a../docs/x.md",
            "é/docs/x.md",
        ] {
            assert_eq!(found(hay, from), Vec::<usize>::new(), "{hay}");
        }
        for (hay, at) in [
            ("docs/x.md", 0),
            ("[a](../../docs/x.md)", 10),
            ("[a](./docs/x.md)", 6),
            ("[a](/docs/x.md)", 5),
            ("see `docs/x.md`", 5),
            ("(docs/x.md)", 1),
        ] {
            assert_eq!(found(hay, from), vec![at], "{hay}");
        }
    }

    /// A longer path starting with `from` holds no occurrence, while a
    /// fragment, a sentence's closing `.` and a closing delimiter each end
    /// one (the agent's recommendation, adopted by the operator, 2026-11-26).
    #[test]
    fn an_occurrence_needs_a_boundary_after_it() {
        let from = "docs/x.md";
        for hay in ["docs/x.mdx", "docs/x.md.orig", "docs/x.md/y", "docs/x.md_b"] {
            assert_eq!(found(hay, from), Vec::<usize>::new(), "{hay}");
        }
        for hay in [
            "docs/x.md#part",
            "docs/x.md.",
            "docs/x.md)",
            "docs/x.md`",
            "docs/x.md:3",
        ] {
            assert_eq!(found(hay, from), vec![0], "{hay}");
        }
    }

    /// The rewrite replaces exactly the occurrences the count finds, keeping
    /// each relative prefix, and leaves a longer path alone.
    #[test]
    fn a_substitution_keeps_each_prefix_and_rewrites_only_occurrences() {
        let hay = "[a](../../docs/x.md) `docs/x.md` foo/docs/x.md docs/x.mdx";
        let out = substitute(hay.as_bytes(), "docs/x.md", ".ductus/docs/agents/x.md");
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "[a](../../.ductus/docs/agents/x.md) `.ductus/docs/agents/x.md` foo/docs/x.md docs/x.mdx"
        );
        assert_eq!(found(hay, "docs/x.md").len(), 2);
    }

    /// The moved file's relative links resolve to what they named before the
    /// move, a self-link follows the file, and a fragment, a root-absolute
    /// target, a scheme and a link in a fenced block are left as they are.
    #[test]
    fn the_moved_files_relative_links_are_rewritten_for_its_new_directory() {
        let repo = Path::new("/project");
        let text = "\
[agents](../AGENTS.md) and [spec](../specs/001-x/spec.md#ac1)
[sibling](other.md) [self](x.md#top) [here](#top)
[root](/README.md) [web](https://example.com/a.md)

```md
[fenced](../AGENTS.md)
```
";
        let out = relocate_links(text, "docs/x.md", ".ductus/docs/agents/x.md", repo);
        assert_eq!(
            out,
            "\
[agents](../../../AGENTS.md) and [spec](../../../specs/001-x/spec.md#ac1)
[sibling](../../../docs/other.md) [self](x.md#top) [here](#top)
[root](/README.md) [web](https://example.com/a.md)

```md
[fenced](../AGENTS.md)
```
"
        );
    }

    #[test]
    fn a_relative_path_climbs_to_the_common_directory() {
        assert_eq!(
            relative(".ductus/docs/agents", "AGENTS.md"),
            "../../../AGENTS.md"
        );
        assert_eq!(
            relative(".ductus/docs/agents", ".ductus/docs/agents/y.md"),
            "y.md"
        );
        assert_eq!(relative("", "docs/y.md"), "docs/y.md");
        assert_eq!(relative("docs", "docs"), ".");
    }
}
