//! `rewrite-spec-links` — re-point every inbound pointer to a retiring or
//! renamed feature directory at its fold target.
//!
//! Fold-back moves a branch-scoped spec's content into its upstream home and
//! then retires the directory (spec 051). Every pointer that named the
//! retiring directory has to move in the same action, or the retirement
//! strands them — which is the one failure `check-orphaned-references`
//! deliberately will not repair, because a migration knows only its own hop
//! and a rewrite that guessed wrong is worse than a precise report. Fold-back
//! knows both endpoints, so it rewrites and the check confirms the result is
//! clean (AC22).
//!
//! Two pointer kinds move together:
//!
//! - **Body links** — any markdown link whose target carries the retiring
//!   directory as a whole path segment, at whatever depth the referring file
//!   sits (`../050-a/spec.md` from a sibling spec, `../../050-a/spec.md` from
//!   a scenario, `specs/050-a/spec.md` from a root-level file).
//! - **The `folds-into` frontmatter field** — included deliberately rather
//!   than left to a later pass. It is the one pointer whose whole job is to
//!   survive until the merge, so a rename that repaired body links and left
//!   it behind would break exactly the thing the field exists for (AC33).
//!
//! `dependencies:` and `references:` are **not** touched. They are derived
//! from body links, and `derive-dependencies` / `derive-references`
//! regenerate them from the rewritten bodies on the next commit through the
//! pre-commit hook, so hand-editing them here would be a second writer for a
//! value that already has one (AC23).
//!
//! Defined by `specs/051-branch-scoped-spec-numbering/spec.md` (AC22, AC23,
//! AC33).

use std::path::Path;

use walkdir::WalkDir;

use crate::primitives::{PrimitiveError, Result, read_text, spec_links, write_atomic};
use crate::schema::paths;
use crate::schema::primitives::{RewriteSpecLinksArgs, RewriteSpecLinksResult, RewrittenFile};

/// Where the pointers are being sent.
struct FoldTarget {
    /// The upstream feature directory name.
    feature: String,
    /// The scenario slug within it, when the fold routed the content into
    /// a scenario rather than the spec body.
    scenario: Option<String>,
}

impl FoldTarget {
    /// Parse `{feature}` or `{feature}/{scenario}`. Splitting on the first
    /// `/` is unambiguous: a feature directory name cannot contain one.
    fn parse(to: &str) -> Result<Self> {
        let (feature, scenario) = match to.split_once('/') {
            Some((feature, scenario)) => (feature, Some(scenario)),
            None => (to, None),
        };
        if feature.is_empty() || scenario.is_some_and(str::is_empty) {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "rewrite-spec-links".into(),
                argument: "to".into(),
                reason: format!(
                    "{to:?} is not a fold target: expected `<feature>` or \
                     `<feature>/<scenario>` with both parts non-empty"
                ),
            });
        }
        Ok(Self {
            feature: feature.to_string(),
            scenario: scenario.map(ToString::to_string),
        })
    }
}

/// Execute the `rewrite-spec-links` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidArgument`] when `from` is empty or `to`
/// does not parse as a fold target, or [`PrimitiveError::Io`] when a markdown
/// file under the spec root cannot be read or written.
pub fn run(args: &RewriteSpecLinksArgs, repo: &Path) -> Result<RewriteSpecLinksResult> {
    if args.from.is_empty() {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "rewrite-spec-links".into(),
            argument: "from".into(),
            reason: "must name the retiring or renamed feature directory".into(),
        });
    }
    let target = FoldTarget::parse(&args.to)?;
    let layout = paths::Paths::load(repo);
    let specs_dir = repo.join(&layout.specs_root);

    let mut rewritten: Vec<RewrittenFile> = Vec::new();
    let mut examined: u32 = 0;

    // Sorted so `rewritten` is in a stable path order rather than whatever
    // order the filesystem hands back — the same reason `list_feature_dirs`
    // sorts.
    let walk = WalkDir::new(&specs_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"));

    for entry in walk {
        let path = entry.path();
        // The retiring directory's own files are leaving with it. Rewriting
        // them would write into a tree about to be removed, and counting
        // them would overstate what was actually repaired. On a rename the
        // directory is already gone, so this skip is a no-op there.
        if path
            .strip_prefix(&specs_dir)
            .ok()
            .and_then(|rel| rel.components().next())
            .is_some_and(|first| first.as_os_str() == args.from.as_str())
        {
            continue;
        }
        examined = examined.saturating_add(1);

        let content = read_text(path)?;
        let (updated, count) = rewrite_content(&content, &args.from, &target);
        if count == 0 {
            continue;
        }
        write_atomic(path, &updated)?;
        rewritten.push(RewrittenFile {
            path: repo_relative(repo, path),
            count,
        });
    }

    Ok(RewriteSpecLinksResult {
        rewritten,
        examined,
    })
}

/// `path` relative to the repo root, with forward slashes.
fn repo_relative(repo: &Path, path: &Path) -> String {
    path.strip_prefix(repo)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Rewrite one file's pointers, returning the new content and how many moved.
///
/// The frontmatter block is walked separately from the body: only
/// `folds-into` moves there, so `dependencies:` and `references:` are left
/// untouched for the generators that own them.
fn rewrite_content(content: &str, from: &str, target: &FoldTarget) -> (String, u32) {
    let mut out = String::with_capacity(content.len());
    let mut count = 0;
    // A trailing newline is not a line, and `lines()` drops it; re-add it
    // only if the input had one, so a file's final byte is preserved.
    let ends_with_newline = content.ends_with('\n');

    let mut in_frontmatter = false;
    for (index, line) in content.lines().enumerate() {
        let is_fence = spec_links::is_frontmatter_fence(line);
        if index == 0 && is_fence {
            in_frontmatter = true;
            push_line(&mut out, line);
            continue;
        }
        if in_frontmatter && is_fence {
            in_frontmatter = false;
            push_line(&mut out, line);
            continue;
        }

        let (rewritten, moved) = if in_frontmatter {
            rewrite_folds_into(line, from, &target.feature)
        } else {
            rewrite_body_line(line, from, target)
        };
        count += moved;
        push_line(&mut out, &rewritten);
    }

    if !ends_with_newline {
        out.pop();
    }
    (out, count)
}

/// Append `line` and a newline.
fn push_line(out: &mut String, line: &str) {
    out.push_str(line);
    out.push('\n');
}

/// Re-point a `folds-into:` line naming `from`, leaving every other
/// frontmatter key — `dependencies:` and `references:` included — alone.
///
/// A fold target is always a feature, never a scenario: `validate-frontmatter`
/// requires the sequential `NNN-slug` form, which is what forbids chaining one
/// staged spec into another. So the scenario half of the target is dropped
/// here even when the body links collapse onto it.
fn rewrite_folds_into(line: &str, from: &str, to_feature: &str) -> (String, u32) {
    let Some(value) = line.strip_prefix("folds-into:") else {
        return (line.to_string(), 0);
    };
    // Tolerate quoting and surrounding whitespace, since a hand-edited
    // frontmatter may carry either.
    let bare = value.trim().trim_matches(['"', '\'']);
    if bare != from {
        return (line.to_string(), 0);
    }
    (format!("folds-into: {to_feature}"), 1)
}

/// Re-point every markdown link on one body line whose target names `from`
/// as a whole path segment.
fn rewrite_body_line(line: &str, from: &str, target: &FoldTarget) -> (String, u32) {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    let mut count = 0;

    while let Some(pos) = rest.find("](") {
        let (head, tail) = rest.split_at(pos + 2);
        out.push_str(head);
        // A link with no closing paren is not a link; emit the remainder as
        // written rather than scanning to end of line for one.
        let Some(close) = tail.find(')') else {
            rest = tail;
            break;
        };
        let (target_text, after) = tail.split_at(close);
        match rewrite_link_target(target_text, from, target) {
            Some(moved) => {
                out.push_str(&moved);
                count += 1;
            }
            None => out.push_str(target_text),
        }
        rest = after;
    }
    out.push_str(rest);
    (out, count)
}

/// The rewritten form of one link target, or `None` when it does not name
/// `from`.
///
/// Matching is by **whole path segment**, so `1234.1-widget` never matches
/// inside `1234.1-widget-cache` — a prefix match here would silently re-point
/// a link at a different spec, which is worse than leaving it dangling where
/// the orphan check can see it.
fn rewrite_link_target(link: &str, from: &str, target: &FoldTarget) -> Option<String> {
    // A cross-service link names another repository's spec. Its resolution is
    // that repo's business and this rename says nothing about it (spec 030).
    if link.contains("://") {
        return None;
    }
    let (path, suffix) = match link.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (link, None),
    };
    let segments: Vec<&str> = path.split('/').collect();
    let index = segments.iter().position(|segment| *segment == from)?;

    let mut rebuilt: Vec<String> = segments[..index]
        .iter()
        .map(|segment| (*segment).to_string())
        .collect();
    rebuilt.push(target.feature.clone());

    match &target.scenario {
        // A rename: the directory's files map across one for one, so the
        // tail — and any fragment naming a heading inside it — survives.
        None => {
            rebuilt.extend(segments[index + 1..].iter().map(|s| (*s).to_string()));
            let mut out = rebuilt.join("/");
            if let Some(fragment) = suffix {
                out.push('#');
                out.push_str(fragment);
            }
            Some(out)
        }
        // A fold into a scenario: the retiring directory's files did not
        // survive individually — their content landed in this one scenario —
        // so the tail names nothing and is replaced rather than carried. Any
        // fragment goes with it, because it named a heading in a file that no
        // longer exists; keeping it would produce a link that resolves to a
        // file and lands nowhere in it.
        Some(slug) => {
            rebuilt.push("scenarios".to_string());
            rebuilt.push(format!("{slug}.md"));
            Some(rebuilt.join("/"))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::run;
    use crate::schema::primitives::RewriteSpecLinksArgs;
    use std::fs;
    use std::path::Path;

    fn args(from: &str, to: &str) -> RewriteSpecLinksArgs {
        RewriteSpecLinksArgs {
            from: from.into(),
            to: to.into(),
        }
    }

    fn write(repo: &Path, rel: &str, content: &str) {
        let path = repo.join("specs").join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn read(repo: &Path, rel: &str) -> String {
        fs::read_to_string(repo.join("specs").join(rel)).unwrap()
    }

    #[test]
    fn sibling_links_are_re_pointed_at_the_fold_target() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n\
             See [staged](../1234.1-staged/spec.md) and [its plan](../1234.1-staged/plan.md).\n",
        );
        write(
            tmp.path(),
            "1234.1-staged/spec.md",
            "---\nstatus: in-progress\ndependencies: []\nfolds-into: 050-alpha\n---\n\n# staged\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(result.rewritten.len(), 1);
        assert_eq!(result.rewritten[0].path, "specs/050-alpha/spec.md");
        assert_eq!(result.rewritten[0].count, 2);
        // The retiring directory's own files are not examined: they leave
        // with it, so counting them would overstate the repair.
        assert_eq!(result.examined, 1);

        let alpha = read(tmp.path(), "050-alpha/spec.md");
        assert!(alpha.contains("[staged](../050-alpha/spec.md)"), "{alpha}");
        assert!(
            alpha.contains("[its plan](../050-alpha/plan.md)"),
            "{alpha}"
        );
    }

    /// A scenario sits one level deeper, so its sibling links carry `../../`.
    /// Matching by path segment rather than by a fixed prefix is what makes
    /// that work without a second rule.
    #[test]
    fn a_link_from_a_scenario_is_re_pointed_at_its_own_depth() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/scenarios/eviction.md",
            "---\nsection: Cache\n---\n\nPer [staged](../../1234.1-staged/spec.md).\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(result.rewritten[0].count, 1);
        let scenario = read(tmp.path(), "050-alpha/scenarios/eviction.md");
        assert!(
            scenario.contains("[staged](../../050-alpha/spec.md)"),
            "{scenario}"
        );
    }

    /// A link naming a scenario *of* the retiring directory, and a target
    /// that routes the content into a scenario: both tails are replaced,
    /// because neither file survives the fold.
    #[test]
    fn a_scenario_target_collapses_every_inbound_link_onto_it() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n\
             See [spec](../1234.1-staged/spec.md) and \
             [a scenario](../1234.1-staged/scenarios/inner.md#behavior).\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha/eviction"), tmp.path()).unwrap();

        assert_eq!(result.rewritten[0].count, 2);
        let alpha = read(tmp.path(), "050-alpha/spec.md");
        assert!(
            alpha.contains("[spec](../050-alpha/scenarios/eviction.md)"),
            "{alpha}"
        );
        // The fragment named a heading in a file that no longer exists, so it
        // goes with the tail rather than resolving to nothing.
        assert!(
            alpha.contains("[a scenario](../050-alpha/scenarios/eviction.md)"),
            "{alpha}"
        );
        assert!(!alpha.contains("#behavior"), "{alpha}");
    }

    /// AC33: the frontmatter pointer moves in the same action as the body
    /// links, not after them.
    #[test]
    fn a_folds_into_naming_the_moved_directory_moves_with_the_links() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "1234.1-other/spec.md",
            "---\nstatus: draft\ndependencies: [050-alpha]\nfolds-into: 050-alpha\n---\n\n# other\n",
        );

        let result = run(&args("050-alpha", "060-beta"), tmp.path()).unwrap();

        assert_eq!(result.rewritten[0].count, 1);
        let other = read(tmp.path(), "1234.1-other/spec.md");
        assert!(other.contains("folds-into: 060-beta"), "{other}");
        // The derived index is left to the generators, which regenerate it
        // from body links on the next commit.
        assert!(other.contains("dependencies: [050-alpha]"), "{other}");
    }

    /// A fold target that names a scenario still sends `folds-into` to the
    /// feature: the field requires the sequential feature form, which is what
    /// forbids chaining one staged spec into another.
    #[test]
    fn folds_into_takes_the_feature_even_when_the_target_names_a_scenario() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "1234.1-other/spec.md",
            "---\nstatus: draft\ndependencies: []\nfolds-into: 050-alpha\n---\n\n# other\n",
        );

        run(&args("050-alpha", "060-beta/eviction"), tmp.path()).unwrap();

        let other = read(tmp.path(), "1234.1-other/spec.md");
        assert!(other.contains("folds-into: 060-beta"), "{other}");
    }

    #[test]
    fn a_corpus_with_no_inbound_pointers_is_examined_and_unchanged() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\nNo pointers here.\n",
        );
        write(
            tmp.path(),
            "060-beta/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\nNor here.\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert!(result.rewritten.is_empty());
        // The distinction the count exists for.
        assert_eq!(result.examined, 2);
        assert!(read(tmp.path(), "050-alpha/spec.md").contains("No pointers here."));
    }

    /// A prefix match would silently re-point a link at a different spec,
    /// which is worse than leaving it where the orphan check can see it.
    #[test]
    fn a_directory_sharing_a_prefix_is_left_alone() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n\
             See [longer](../1234.1-staged-cache/spec.md).\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert!(result.rewritten.is_empty());
        assert!(read(tmp.path(), "050-alpha/spec.md").contains("../1234.1-staged-cache/spec.md"));
    }

    /// A cross-service link names another repository's spec; this rename
    /// says nothing about how it resolves there.
    #[test]
    fn a_cross_service_url_is_not_rewritten() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n\
             See [api](https://github.com/acme/api/blob/main/specs/1234.1-staged/spec.md).\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert!(result.rewritten.is_empty());
        assert!(
            read(tmp.path(), "050-alpha/spec.md")
                .contains("acme/api/blob/main/specs/1234.1-staged")
        );
    }

    #[test]
    fn a_root_relative_link_is_re_pointed_too() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n\
             See [staged](specs/1234.1-staged/spec.md).\n",
        );

        run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert!(read(tmp.path(), "050-alpha/spec.md").contains("](specs/050-alpha/spec.md)"));
    }

    #[test]
    fn a_malformed_to_is_refused_before_anything_is_written() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n[x](../1234.1-staged/spec.md)\n",
        );

        assert!(run(&args("1234.1-staged", "050-alpha/"), tmp.path()).is_err());
        assert!(read(tmp.path(), "050-alpha/spec.md").contains("../1234.1-staged/spec.md"));
    }
}
