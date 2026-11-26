//! The parts of a project file (spec 087).
//!
//! A project file that has outgrown `[artifacts] read-size-bytes` may keep an
//! index and move its sections into **parts**: the markdown files directly
//! under its parts directory, `.ductus/docs/{agents,system,events,errors}/`
//! under the project root. This module is the only enumeration of them, so
//! `check-artifact-size`, `prune-project` and `resolve-constitutions` cannot
//! disagree about what a part is (spec 087 plan, *One definition of the part
//! set*).
//!
//! It computes three things and opens nothing but the four project files: the
//! part set, each part's linkage from its own project file, and the integrity
//! findings ([`PartFindingKind`]). A part is a part whether or not its index
//! links it. What could not be looked at — a directory that is, or lies
//! under, a symbolic link leaving the project, one `read_dir` could not list,
//! a project file that could not be read — is reported as such (`unlisted`, a
//! part's `outside`, a `linked` of `None`), never read as no parts or as an
//! unlinked one (`QUAL-CLAIM-001`).

use std::collections::BTreeSet;
use std::path::Path;

use crate::primitives::check_corpus_links::{
    LinkFilePart, lexical_join, link_base, link_file_part, link_targets,
};
use crate::primitives::{SkipScanner, leaves_project, rel_path, scenario_name_cmp};
use crate::schema::primitives::{PartFinding, PartFindingKind, ProjectFileKind};

/// The directory under the project root that holds every parts directory,
/// and nothing else.
pub(crate) const DOCS_DIR: &str = ".ductus/docs";

/// The parts directory of a registered shared constitution, under its
/// checkout, beside the `constitution.md` read from there (spec 087,
/// *Parts*). In a project it is no project file's, yet it is a parts
/// directory, so it is no `not-a-parts-directory` finding there (the owner,
/// 2026-11-25).
pub(crate) const CONSTITUTION_PARTS_DIR: &str = ".ductus/docs/constitution";

/// The project files that may have parts, in `project_subjects` order.
pub(crate) const KINDS: [ProjectFileKind; 4] = [
    ProjectFileKind::Agents,
    ProjectFileKind::System,
    ProjectFileKind::Events,
    ProjectFileKind::Errors,
];

/// How deep the nested-file walk descends before it records the directory
/// as unlisted and stops: far past any real tree, so that a link to an
/// ancestor ends as a report rather than a stack overflow.
const MAX_NESTED_DEPTH: u32 = 32;

/// `kind`'s parts directory, project-relative.
pub(crate) fn parts_dir(kind: ProjectFileKind) -> &'static str {
    match kind {
        ProjectFileKind::Agents => ".ductus/docs/agents",
        ProjectFileKind::System => ".ductus/docs/system",
        ProjectFileKind::Events => ".ductus/docs/events",
        ProjectFileKind::Errors => ".ductus/docs/errors",
    }
}

/// Whether `name`, a file name directly under a parts directory, names a
/// part: a markdown file. Any other file there is neither measured nor
/// reported (AC11).
pub(crate) fn is_part_name(name: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// The project file whose part `rel`, a project-relative path, would be: the
/// one whose parts directory holds it directly, when it names a markdown
/// file. A staged check, which reads the index rather than the disk,
/// classifies an entry by this.
pub(crate) fn part_of(rel: &str) -> Option<ProjectFileKind> {
    let (dir, name) = rel.rsplit_once('/')?;
    if !is_part_name(name) {
        return None;
    }
    KINDS.into_iter().find(|kind| parts_dir(*kind) == dir)
}

/// A markdown file directly under a listed directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ListedPart {
    /// Its path relative to the listing's base.
    pub(crate) path: String,
    /// Whether it is a symbolic link resolving outside the base
    /// (`BE-INPUT-004`): unreadable, and never opened.
    pub(crate) outside: bool,
}

/// One directory as [`list_dir`] listed it. A directory that does not exist
/// lists empty: it holds no parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Listing {
    /// Its markdown files, in `scenario_name_cmp` order.
    pub(crate) parts: Vec<ListedPart>,
    /// Its subdirectories' base-relative paths, sorted.
    pub(crate) subdirs: Vec<String>,
    /// What could not be looked at: the directory itself, when it is or lies
    /// under a symbolic link resolving outside the base or could not be
    /// listed; or an entry that is a link out of the base and not markdown,
    /// so whether it is a directory is never probed.
    pub(crate) unlisted: Vec<String>,
}

/// List `dir_rel`, a directory relative to `base`, without following a link
/// that leaves `base`. `base` is the project root for a project file's
/// parts, and a shared constitution's checkout for its parts.
pub(crate) fn list_dir(base: &Path, dir_rel: &str) -> Listing {
    let mut listing = Listing::default();
    if leaves_project(base, dir_rel) {
        listing.unlisted.push(dir_rel.to_string());
        return listing;
    }
    let dir = base.join(dir_rel);
    if !dir.is_dir() {
        return listing;
    }
    let Ok(entries) = std::fs::read_dir(&dir) else {
        listing.unlisted.push(dir_rel.to_string());
        return listing;
    };
    for entry in entries {
        let Ok(entry) = entry else {
            // An entry that cannot be read leaves the listing incomplete,
            // and a shorter list is indistinguishable from a full one.
            listing.unlisted.push(dir_rel.to_string());
            continue;
        };
        let name = entry.file_name().to_string_lossy().into_owned();
        let rel = format!("{dir_rel}/{name}");
        // A link out of the base is judged before it is followed, so the type
        // of what it points at is never probed.
        let linked = entry.file_type().is_ok_and(|kind| kind.is_symlink());
        if linked && leaves_project(base, &rel) {
            if is_part_name(&name) {
                listing.parts.push(ListedPart {
                    path: rel,
                    outside: true,
                });
            } else {
                listing.unlisted.push(rel);
            }
            continue;
        }
        if entry.path().is_dir() {
            listing.subdirs.push(rel);
        } else if is_part_name(&name) {
            listing.parts.push(ListedPart {
                path: rel,
                outside: false,
            });
        }
    }
    listing
        .parts
        .sort_by(|a, b| scenario_name_cmp(&a.path, &b.path));
    listing.subdirs.sort();
    listing.unlisted.sort();
    listing.unlisted.dedup();
    listing
}

/// One part of a project file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Part {
    /// Its project-relative path.
    pub(crate) path: String,
    /// The project file it is a part of.
    pub(crate) kind: ProjectFileKind,
    /// That project file's project-relative path.
    pub(crate) project_file: String,
    /// Whether it is a symbolic link resolving outside the project: it is
    /// unreadable, and never opened.
    pub(crate) outside: bool,
    /// Whether its own project file links it. `None` when that file exists
    /// but could not be read, so its links are unknown; an absent project
    /// file links nothing.
    pub(crate) linked: Option<bool>,
}

/// The project files' parts, and what is wrong with them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ProjectParts {
    /// Every part, by project file in `project_subjects` order, then in
    /// `scenario_name_cmp` order.
    pub(crate) parts: Vec<Part>,
    /// The integrity findings, sorted by path, then kind.
    pub(crate) findings: Vec<PartFinding>,
    /// What under `.ductus/docs/` could not be looked at (see
    /// [`Listing::unlisted`]), sorted: a part or a finding it holds is
    /// unknown, never absent.
    pub(crate) unlisted: Vec<String>,
}

/// The project files' parts under `repo`, whose spec root is `root`, with
/// each part's linkage and the integrity findings over them.
pub(crate) fn project_parts(repo: &Path, root: &str) -> ProjectParts {
    let mut out = ProjectParts::default();
    let subjects = super::check_artifact_size::project_subjects(root);
    let links: Vec<Option<Vec<String>>> = subjects
        .iter()
        .map(|(_, file)| project_file_links(repo, file))
        .collect();
    missing_parts(repo, &subjects, &links, &mut out.findings);

    for (((kind, file), file_links), dir) in subjects.iter().zip(&links).zip(KINDS.map(parts_dir)) {
        let listing = list_dir(repo, dir);
        out.unlisted.extend(listing.unlisted);
        for listed in listing.parts {
            let linked = file_links
                .as_ref()
                .map(|file_links| file_links.contains(&listed.path));
            if linked == Some(false) {
                out.findings.push(PartFinding {
                    kind: PartFindingKind::UnlinkedPart,
                    message: format!(
                        "`{}` is a part of `{file}`, which does not link it: add a pointer to it \
                         in `{file}` saying when to read it, or remove the part",
                        listed.path
                    ),
                    path: listed.path.clone(),
                });
            }
            out.parts.push(Part {
                path: listed.path,
                kind: *kind,
                project_file: file.clone(),
                outside: listed.outside,
                linked,
            });
        }
        let mut nested = Vec::new();
        for subdir in listing.subdirs {
            nested_files(
                repo,
                &subdir,
                MAX_NESTED_DEPTH,
                &mut nested,
                &mut out.unlisted,
            );
        }
        for path in nested {
            out.findings.push(PartFinding {
                kind: PartFindingKind::NestedFile,
                message: format!(
                    "`{path}` is in a subdirectory of `{dir}/`, and parts do not nest: move it \
                     directly under `{dir}/` as a sibling part, named by `{file}`"
                ),
                path,
            });
        }
    }

    strays(repo, &mut out);
    out.findings
        .sort_by(|a, b| a.path.cmp(&b.path).then(a.kind.cmp(&b.kind)));
    out.unlisted.sort();
    out.unlisted.dedup();
    out
}

/// A `missing-part` finding for each distinct link of a project file that
/// resolves into a parts directory, to no file. A link that is, or passes
/// through, a symbolic link leaving the project is never probed: the
/// directory it enters is reported unlisted instead.
fn missing_parts(
    repo: &Path,
    subjects: &[(ProjectFileKind, String)],
    links: &[Option<Vec<String>>],
    findings: &mut Vec<PartFinding>,
) {
    let mut missing = BTreeSet::new();
    for ((_, file), file_links) in subjects.iter().zip(links) {
        for link in file_links.iter().flatten() {
            if KINDS.iter().any(|kind| in_dir(link, parts_dir(*kind)))
                && !leaves_project(repo, link)
                && !repo.join(link).exists()
            {
                missing.insert((link.clone(), file.clone()));
            }
        }
    }
    for (link, file) in missing {
        findings.push(PartFinding {
            kind: PartFindingKind::MissingPart,
            message: format!(
                "`{file}` links `{link}`, which does not exist: restore the part, or remove \
                 the link from `{file}`"
            ),
            path: link,
        });
    }
}

/// The `not-a-part` and `not-a-parts-directory` findings: what sits
/// directly under `.ductus/docs/` beside the parts directories.
fn strays(repo: &Path, out: &mut ProjectParts) {
    let docs = list_dir(repo, DOCS_DIR);
    out.unlisted.extend(docs.unlisted);
    for stray in docs.parts {
        out.findings.push(PartFinding {
            kind: PartFindingKind::NotAPart,
            message: format!(
                "`{}` is directly under `{DOCS_DIR}/`, which holds parts directories alone: move \
                 it into the parts directory of the project file it belongs to",
                stray.path
            ),
            path: stray.path,
        });
    }
    for dir in docs.subdirs {
        if KINDS.iter().any(|kind| parts_dir(*kind) == dir) || dir == CONSTITUTION_PARTS_DIR {
            continue;
        }
        out.findings.push(PartFinding {
            kind: PartFindingKind::NotAPartsDirectory,
            message: format!(
                "`{dir}/` is no project file's parts directory ({}): rename it to the one its \
                 content belongs to, or move its content out of `{DOCS_DIR}/`",
                KINDS
                    .map(|kind| format!("`{}/`", parts_dir(kind)))
                    .join(", ")
            ),
            path: dir,
        });
    }
}

/// Whether `path` lies under `dir`, both project-relative, by whole
/// components.
fn in_dir(path: &str, dir: &str) -> bool {
    path.strip_prefix(dir)
        .is_some_and(|rest| rest.starts_with('/'))
}

/// The links of the project file at `file`, project-relative, as
/// [`index_links`] resolves them: empty when it does not exist, `None` when
/// it exists but could not be read or is, or lies under, a symbolic link
/// leaving the project, which is never opened (`BE-INPUT-004`).
fn project_file_links(repo: &Path, file: &str) -> Option<Vec<String>> {
    if leaves_project(repo, file) {
        return None;
    }
    match std::fs::read_to_string(repo.join(file)) {
        Ok(text) => Some(index_links(&text, file, repo)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Some(Vec::new()),
        Err(_) => None,
    }
}

/// Every inline markdown link in `text`, the body of the project-relative
/// file `file`, resolved to a project-relative path, in order of appearance.
///
/// Read through `check-corpus-links`' scan (`link_targets`,
/// `link_file_part`, `link_base`, `lexical_join`), so a fragment is
/// stripped, a scheme-bearing target, a bare fragment and a documentation
/// shape name no file, and a root-absolute target resolves against the
/// project root. Unlike that scan, a link inside an HTML comment is skipped
/// with one in a fenced block: a commented-out pointer names no part (spec
/// 087 plan, *Linkage*). A target that leaves the project lexically is
/// dropped, since no part lies there.
pub(crate) fn index_links(text: &str, file: &str, repo: &Path) -> Vec<String> {
    let here = repo.join(file);
    let here = here.parent().unwrap_or(repo);
    let mut skip = SkipScanner::default();
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if skip.skip(line) {
            continue;
        }
        for target in link_targets(line) {
            let LinkFilePart::File(rel) = link_file_part(target) else {
                continue;
            };
            let resolved = lexical_join(link_base(rel, here, repo), rel);
            if resolved.strip_prefix(repo).is_ok() {
                out.push(rel_path(&resolved, repo));
            }
        }
    }
    out
}

/// Append every markdown file at any depth under `dir`, a subdirectory of a
/// parts directory, to `out`, and every directory it could not look at to
/// `unlisted`.
fn nested_files(
    repo: &Path,
    dir: &str,
    depth_budget: u32,
    out: &mut Vec<String>,
    unlisted: &mut Vec<String>,
) {
    if depth_budget == 0 {
        unlisted.push(dir.to_string());
        return;
    }
    let listing = list_dir(repo, dir);
    out.extend(listing.parts.into_iter().map(|part| part.path));
    unlisted.extend(listing.unlisted);
    for subdir in listing.subdirs {
        nested_files(repo, &subdir, depth_budget - 1, out, unlisted);
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write(repo: &Path, rel: &str, body: &str) {
        let path = repo.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }

    /// A clean project: each project file is an index linking its one part.
    fn clean(repo: &Path) {
        write(
            repo,
            "AGENTS.md",
            "# Agents\n\nRead [the procedure](.ductus/docs/agents/procedure.md) for that task.\n",
        );
        write(
            repo,
            "specs/system.md",
            "# System\n\n## Storage\n\nSee [storage](../.ductus/docs/system/storage.md).\n",
        );
        write(
            repo,
            "specs/events.md",
            "# Events\n\nSee [orders](../.ductus/docs/events/orders.md).\n",
        );
        write(
            repo,
            "specs/errors.md",
            "# Errors\n\nSee [auth](../.ductus/docs/errors/auth.md).\n",
        );
        for part in [
            ".ductus/docs/agents/procedure.md",
            ".ductus/docs/system/storage.md",
            ".ductus/docs/events/orders.md",
            ".ductus/docs/errors/auth.md",
        ] {
            write(repo, part, "# Part\n");
        }
    }

    fn kinds(parts: &ProjectParts) -> Vec<(PartFindingKind, &str)> {
        parts
            .findings
            .iter()
            .map(|finding| (finding.kind, finding.path.as_str()))
            .collect()
    }

    #[test]
    fn a_markdown_file_directly_under_each_parts_directory_is_a_part_linked_or_not() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        // One unlinked part per directory, beside the linked one.
        for dir in KINDS.map(parts_dir) {
            write(repo, &format!("{dir}/zz-unlinked.md"), "# Unlinked\n");
        }
        let parts = project_parts(repo, "specs");
        let listed: Vec<(&str, ProjectFileKind, &str, Option<bool>)> = parts
            .parts
            .iter()
            .map(|part| {
                (
                    part.path.as_str(),
                    part.kind,
                    part.project_file.as_str(),
                    part.linked,
                )
            })
            .collect();
        assert_eq!(
            listed,
            vec![
                (
                    ".ductus/docs/agents/procedure.md",
                    ProjectFileKind::Agents,
                    "AGENTS.md",
                    Some(true)
                ),
                (
                    ".ductus/docs/agents/zz-unlinked.md",
                    ProjectFileKind::Agents,
                    "AGENTS.md",
                    Some(false)
                ),
                (
                    ".ductus/docs/system/storage.md",
                    ProjectFileKind::System,
                    "specs/system.md",
                    Some(true)
                ),
                (
                    ".ductus/docs/system/zz-unlinked.md",
                    ProjectFileKind::System,
                    "specs/system.md",
                    Some(false)
                ),
                (
                    ".ductus/docs/events/orders.md",
                    ProjectFileKind::Events,
                    "specs/events.md",
                    Some(true)
                ),
                (
                    ".ductus/docs/events/zz-unlinked.md",
                    ProjectFileKind::Events,
                    "specs/events.md",
                    Some(false)
                ),
                (
                    ".ductus/docs/errors/auth.md",
                    ProjectFileKind::Errors,
                    "specs/errors.md",
                    Some(true)
                ),
                (
                    ".ductus/docs/errors/zz-unlinked.md",
                    ProjectFileKind::Errors,
                    "specs/errors.md",
                    Some(false)
                ),
            ]
        );
        assert!(parts.parts.iter().all(|part| !part.outside));
        assert_eq!(parts.unlisted, Vec::<String>::new());
    }

    #[test]
    fn a_directory_s_parts_are_in_scenario_order_not_byte_order() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        write(
            repo,
            "specs/system.md",
            "# System\n\n[b](../.ductus/docs/system/B.md) [a](../.ductus/docs/system/a.md)\n",
        );
        write(repo, ".ductus/docs/system/B.md", "# B\n");
        write(repo, ".ductus/docs/system/a.md", "# a\n");
        let parts = project_parts(repo, "specs");
        let paths: Vec<&str> = parts.parts.iter().map(|part| part.path.as_str()).collect();
        assert_eq!(
            paths,
            vec![".ductus/docs/system/a.md", ".ductus/docs/system/B.md"]
        );
    }

    #[test]
    fn a_non_markdown_file_in_a_parts_directory_is_neither_a_part_nor_a_finding() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(repo, ".ductus/docs/system/diagram.png", "png");
        write(repo, ".ductus/docs/system/notes.txt", "notes");
        let parts = project_parts(repo, "specs");
        assert_eq!(parts.parts.len(), 4);
        assert!(
            parts.parts.iter().all(|part| is_part_name(&part.path)),
            "{:?}",
            parts.parts
        );
        assert_eq!(kinds(&parts), Vec::new());
    }

    #[test]
    fn a_clean_project_has_no_finding() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        let parts = project_parts(repo, "specs");
        assert_eq!(kinds(&parts), Vec::new());
        assert_eq!(parts.parts.len(), 4);
        assert!(parts.parts.iter().all(|part| part.linked == Some(true)));
    }

    #[test]
    fn a_link_to_a_part_that_does_not_exist_is_a_missing_part() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(
            repo,
            "specs/system.md",
            "# System\n\nSee [storage](../.ductus/docs/system/storage.md) and \
             [gone](../.ductus/docs/system/gone.md).\n",
        );
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![(PartFindingKind::MissingPart, ".ductus/docs/system/gone.md")]
        );
        let message = &parts.findings[0].message;
        assert!(message.contains("`specs/system.md`"), "{message}");
    }

    #[test]
    fn a_part_its_project_file_does_not_link_is_unlinked() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(repo, ".ductus/docs/errors/orphan.md", "# Orphan\n");
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![(
                PartFindingKind::UnlinkedPart,
                ".ductus/docs/errors/orphan.md"
            )]
        );
    }

    #[test]
    fn a_link_from_another_project_file_does_not_link_a_part() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(repo, ".ductus/docs/system/shared.md", "# Shared\n");
        write(
            repo,
            "AGENTS.md",
            "# Agents\n\nRead [the procedure](.ductus/docs/agents/procedure.md) and \
             [shared](.ductus/docs/system/shared.md).\n",
        );
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![(
                PartFindingKind::UnlinkedPart,
                ".ductus/docs/system/shared.md"
            )]
        );
    }

    #[test]
    fn a_markdown_file_in_a_subdirectory_of_a_parts_directory_is_nested() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(repo, ".ductus/docs/system/deeper/inner.md", "# Inner\n");
        write(
            repo,
            ".ductus/docs/system/deeper/deepest/core.md",
            "# Core\n",
        );
        write(repo, ".ductus/docs/system/deeper/image.png", "png");
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![
                (
                    PartFindingKind::NestedFile,
                    ".ductus/docs/system/deeper/deepest/core.md"
                ),
                (
                    PartFindingKind::NestedFile,
                    ".ductus/docs/system/deeper/inner.md"
                ),
            ]
        );
        assert_eq!(parts.parts.len(), 4, "a nested file is no part");
    }

    #[test]
    fn a_directory_under_docs_that_is_no_parts_directory_is_reported() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(repo, ".ductus/docs/sytem/storage.md", "# Misplaced\n");
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![(PartFindingKind::NotAPartsDirectory, ".ductus/docs/sytem")]
        );
    }

    #[test]
    fn a_shared_constitution_parts_directory_is_no_stray_directory() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(
            repo,
            ".ductus/docs/constitution/security.md",
            "# Security\n",
        );
        let parts = project_parts(repo, "specs");
        assert_eq!(kinds(&parts), Vec::new());
        assert_eq!(
            parts.parts.len(),
            4,
            "a constitution's part is no project part"
        );
    }

    #[test]
    fn a_markdown_file_directly_under_docs_is_not_a_part() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(repo, ".ductus/docs/stray.md", "# Stray\n");
        write(repo, ".ductus/docs/README.txt", "not markdown");
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![(PartFindingKind::NotAPart, ".ductus/docs/stray.md")]
        );
    }

    #[test]
    fn a_part_that_is_a_link_leaving_the_project_is_unreadable() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        let _outside = crate::primitives::outward::file(
            repo,
            ".ductus/docs/system/elsewhere.md",
            "# Outside\n",
        );
        let parts = project_parts(repo, "specs");
        let part = parts
            .parts
            .iter()
            .find(|part| part.path == ".ductus/docs/system/elsewhere.md")
            .expect("the linked file is listed as a part");
        assert!(part.outside, "{part:?}");
        let storage = parts
            .parts
            .iter()
            .find(|part| part.path == ".ductus/docs/system/storage.md")
            .unwrap();
        assert!(!storage.outside, "a part inside the project is readable");
    }

    #[test]
    fn a_parts_directory_that_is_a_link_leaving_the_project_is_unlisted_never_empty() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        fs::remove_dir_all(repo.join(".ductus/docs/events")).unwrap();
        let _outside = crate::primitives::outward::dir(repo, ".ductus/docs/events", |dir| {
            fs::write(dir.join("orders.md"), "# Orders\n").unwrap();
        });
        let parts = project_parts(repo, "specs");
        assert!(
            parts.unlisted.contains(&".ductus/docs/events".to_string()),
            "{:?}",
            parts.unlisted
        );
        assert!(
            !parts
                .parts
                .iter()
                .any(|part| part.kind == ProjectFileKind::Events),
            "nothing is listed through a link out of the project"
        );
        assert!(
            !parts
                .findings
                .iter()
                .any(|finding| finding.kind == PartFindingKind::MissingPart),
            "a link into an unlisted directory is not probed: {:?}",
            parts.findings
        );
    }

    #[test]
    fn a_project_file_that_cannot_be_read_leaves_its_parts_linkage_unknown() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        fs::remove_file(repo.join("specs/system.md")).unwrap();
        let _outside = crate::primitives::outward::file(repo, "specs/system.md", "# System\n");
        let parts = project_parts(repo, "specs");
        let storage = parts
            .parts
            .iter()
            .find(|part| part.path == ".ductus/docs/system/storage.md")
            .unwrap();
        assert_eq!(storage.linked, None);
        assert_eq!(kinds(&parts), Vec::new(), "unknown is no unlinked finding");
    }

    #[test]
    fn a_part_whose_project_file_is_absent_is_unlinked() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        fs::remove_file(repo.join("specs/events.md")).unwrap();
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![(
                PartFindingKind::UnlinkedPart,
                ".ductus/docs/events/orders.md"
            )]
        );
    }

    #[test]
    fn a_project_with_no_docs_directory_has_no_part_and_no_finding() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        write(
            repo,
            "AGENTS.md",
            "# Agents\n\nSee [the spec](specs/001-x/spec.md).\n",
        );
        write(
            repo,
            "specs/system.md",
            "# System\n\nSee [README](../README.md).\n",
        );
        write(repo, "specs/events.md", "# Events\n");
        write(repo, "specs/errors.md", "# Errors\n");
        let parts = project_parts(repo, "specs");
        assert_eq!(parts, ProjectParts::default());
    }

    #[test]
    fn a_link_with_a_fragment_or_rooted_at_the_project_links_its_part() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(
            repo,
            "AGENTS.md",
            "# Agents\n\nRead [one](.ductus/docs/agents/procedure.md#its-steps).\n",
        );
        write(
            repo,
            "specs/system.md",
            "# System\n\nSee [storage](/.ductus/docs/system/storage.md).\n",
        );
        let parts = project_parts(repo, "specs");
        assert_eq!(kinds(&parts), Vec::new());
        assert!(parts.parts.iter().all(|part| part.linked == Some(true)));
    }

    #[test]
    fn a_link_inside_an_html_comment_or_a_fence_links_no_part() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        clean(repo);
        write(
            repo,
            "specs/system.md",
            "# System\n\n<!--\nSee [storage](../.ductus/docs/system/storage.md).\n-->\n\n```\n\
             [errors](../.ductus/docs/system/gone.md)\n```\n",
        );
        let parts = project_parts(repo, "specs");
        assert_eq!(
            kinds(&parts),
            vec![(
                PartFindingKind::UnlinkedPart,
                ".ductus/docs/system/storage.md"
            )],
            "neither the commented-out pointer links nor the fenced one is missing"
        );
    }

    #[test]
    fn part_of_names_the_project_file_whose_directory_holds_a_markdown_file_directly() {
        assert_eq!(
            part_of(".ductus/docs/system/storage.md"),
            Some(ProjectFileKind::System)
        );
        assert_eq!(
            part_of(".ductus/docs/agents/procedure.MD"),
            Some(ProjectFileKind::Agents)
        );
        assert_eq!(part_of(".ductus/docs/system/diagram.png"), None);
        assert_eq!(part_of(".ductus/docs/system/deeper/inner.md"), None);
        assert_eq!(part_of(".ductus/docs/stray.md"), None);
        assert_eq!(part_of(".ductus/docs/constitution/security.md"), None);
        assert_eq!(part_of("docs/system/storage.md"), None);
    }
}
