//! `rewrite-spec-links` — re-point every inbound pointer to a retiring or
//! renamed feature directory at its fold target.
//!
//! Fold-back moves a branch-scoped spec's content into its upstream home and
//! then retires the directory (spec 051), and `/{project}:consolidate` removes
//! a directory whose content belongs with another (spec 052). Every pointer
//! that named the retiring directory has to move in the same action, or the
//! retirement strands them — which is the one failure
//! `check-orphaned-references` deliberately will not repair, because a
//! migration knows only its own hop and a rewrite that guessed wrong is worse
//! than a precise report. Both commands know both endpoints, so this rewrites
//! and the check confirms the result is clean (AC22).
//!
//! Three pointer kinds move together:
//!
//! - **Body links** in every markdown file under the spec root — any markdown
//!   link whose target carries the retiring directory as a whole path
//!   segment, at whatever depth the referring file sits (`../050-a/spec.md`
//!   from a sibling spec, `../../050-a/spec.md` from a scenario).
//! - **Path references in the referrers `check-orphaned-references` reads**
//!   — `CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit` and the
//!   spec root's `system.md`, from that check's own `referrers` list rather
//!   than a copy. From those files a reference reads
//!   `{specs-root}/{from}/…` as a markdown link, a code span, prose or a shell
//!   argument, so they are scanned with that check's own token scanner
//!   (`managed_path_spans`), anchored at the configured spec root, and a
//!   token is rewritten exactly when its first segment past the root is the
//!   retiring directory. Reusing the scanner is what keeps the two in step:
//!   every token the check would report as naming the retiring directory is
//!   re-pointed, and what it does not treat as a reference — a revision-
//!   qualified path (`git show abc1234:specs/{from}/spec.md`), a pattern, a
//!   URL — is not rewritten here either. A re-pointed token keeps its tail on
//!   a rename, as a body link does, so one naming a file the target does not
//!   hold still dangles, and the check still reports it. `system.md` sits
//!   under the spec root, so it takes both passes.
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
//! A referrer that is a symbolic link is not followed. The atomic write
//! replaces the file it is given, so writing through `CLAUDE.md` linked to
//! `AGENTS.md` would turn the link into a copy; the file it names is
//! rewritten in its own right when it is itself a referrer, and a reference
//! reached only through a link is left for `check-orphaned-references`, which
//! reads through it, to report. The spec-root walk skips links the same way.
//!
//! Defined by `specs/051-branch-scoped-spec-numbering/spec.md` (AC6, AC22,
//! AC23, AC33).

use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::primitives::{
    PrimitiveError, Result, check_orphaned_references, read_text, spec_links, write_atomic,
};
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

/// Which pointer forms one file is rewritten for.
#[derive(Clone, Copy)]
enum Passes {
    /// Markdown under the spec root: body links and `folds-into`.
    Links,
    /// A referrer outside the spec root: path tokens behind the spec root.
    Tokens,
    /// A referrer under the spec root (`system.md`): both, links first, so a
    /// link the first pass re-pointed no longer names the retiring directory
    /// and is not counted again by the second.
    Both,
}

impl Passes {
    fn links(self) -> bool {
        matches!(self, Self::Links | Self::Both)
    }

    fn tokens(self) -> bool {
        matches!(self, Self::Tokens | Self::Both)
    }
}

/// One run's rewrite rule and what it has done so far.
struct Sweep<'a> {
    repo: &'a Path,
    /// The retiring or renamed directory.
    from: &'a str,
    target: FoldTarget,
    /// The configured spec root as a prefix, `specs/` by default — the one
    /// managed root a feature directory can be named behind.
    root: String,
    rewritten: Vec<RewrittenFile>,
    examined: u32,
}

impl Sweep<'_> {
    /// Rewrite one file in place, recording it when anything in it moved.
    fn visit(&mut self, path: &Path, passes: Passes) -> Result<()> {
        self.examined = self.examined.saturating_add(1);
        let content = read_text(path)?;
        let (updated, count) =
            rewrite_content(&content, self.from, &self.target, &self.root, passes);
        if count == 0 {
            return Ok(());
        }
        write_atomic(path, &updated)?;
        self.rewritten.push(RewrittenFile {
            path: repo_relative(self.repo, path),
            count,
        });
        Ok(())
    }
}

/// Execute the `rewrite-spec-links` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidArgument`] when `from` is empty or `to`
/// does not parse as a fold target, or [`PrimitiveError::Io`] when a markdown
/// file under the spec root, or a referrer outside it, cannot be read as
/// UTF-8 text or written.
pub fn run(args: &RewriteSpecLinksArgs, repo: &Path) -> Result<RewriteSpecLinksResult> {
    if args.from.is_empty() {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "rewrite-spec-links".into(),
            argument: "from".into(),
            reason: "must name the retiring or renamed feature directory".into(),
        });
    }
    // Both arguments reach a filesystem path below — `from` as a path
    // component to skip, `to` as the directory whose existence is checked —
    // so neither may carry a parent-directory escape (BE-INPUT-004).
    let layout = paths::Paths::load(repo);
    super::validate_feature_argument(repo, &layout.specs_root, &args.from)?;
    super::validate_feature_argument(repo, &layout.specs_root, &args.to)?;
    let target = FoldTarget::parse(&args.to)?;
    let specs_dir = repo.join(&layout.specs_root);

    // Refuse before writing anything when the destination does not exist.
    //
    // This primitive re-points pointers across the whole corpus, so a run
    // against a target that is not there leaves every inbound link naming a
    // spec nobody can open — and the rewrite is idempotent in the *wrong*
    // direction for recovery: once re-pointed, nothing names the original
    // any more, so a second run finds nothing to repair.
    //
    // The check belongs here rather than in the calling command. Today
    // `/{project}:fold` happens to establish the same fact one step earlier,
    // because `invalidate-review` errors on a missing feature — but that is
    // a side effect of a step added for an unrelated reason (spec 051 task
    // 21), not a guarantee. A primitive that re-points links at a
    // destination is the thing that should refuse an absent one, on both
    // paths and however it is called.
    //
    // Same terms `retire-feature` enforces: a directory holding a `spec.md`,
    // not merely a directory — a directory with no spec is not a home
    // content can have landed in. The scenario half of a `{feature}/{slug}`
    // target is deliberately not checked, because the feature's existence is
    // what makes the link resolvable at all and the scenario is written by
    // an earlier step of the same fold.
    if !specs_dir.join(&target.feature).join("spec.md").is_file() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: target.feature.clone(),
        });
    }

    // The files `check-orphaned-references` reads, from its own list: a
    // reference this leaves behind in one of them is a finding that check
    // reports the moment the directory is retired (AC22).
    let referrers: Vec<PathBuf> = check_orphaned_references::referrers(&layout)
        .iter()
        .map(|referrer| repo.join(referrer))
        .collect();
    let mut sweep = Sweep {
        repo,
        from: &args.from,
        target,
        root: check_orphaned_references::spec_root_prefix(&layout),
        rewritten: Vec::new(),
        examined: 0,
    };

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
        let passes = if referrers.iter().any(|referrer| referrer == path) {
            Passes::Both
        } else {
            Passes::Links
        };
        sweep.visit(path, passes)?;
    }

    // The referrers outside the spec root, in the order the check reads them.
    for referrer in &referrers {
        // Under the spec root (`system.md`): walked above, with both passes.
        if referrer.starts_with(&specs_dir) {
            continue;
        }
        // Not every project has every referrer, and an absent one is neither
        // examined nor rewritten. Nor is a symbolic link followed: the atomic
        // write would replace the link with a copy of what it names.
        let regular =
            std::fs::symlink_metadata(referrer).is_ok_and(|meta| meta.file_type().is_file());
        if !regular {
            continue;
        }
        sweep.visit(referrer, Passes::Tokens)?;
    }

    Ok(RewriteSpecLinksResult {
        rewritten: sweep.rewritten,
        examined: sweep.examined,
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
/// The link pass walks the frontmatter block separately from the body: only
/// `folds-into` moves there, so `dependencies:` and `references:` are left
/// untouched for the generators that own them. The token pass reads every
/// line, frontmatter included, because `check-orphaned-references` does.
fn rewrite_content(
    content: &str,
    from: &str,
    target: &FoldTarget,
    root: &str,
    passes: Passes,
) -> (String, u32) {
    // The file's own ending, not a bare `\n`. `lines()` strips a trailing
    // `\r`, so re-joining with `\n` would convert a CRLF checkout's file to
    // LF as a side effect of re-pointing one link.
    let ending = crate::primitives::line_ending_of(content);
    let mut out = String::with_capacity(content.len());
    let mut count = 0;
    // A trailing newline is not a line, and `lines()` drops it; re-add it
    // only if the input had one, so a file's final byte is preserved.
    let ends_with_newline = content.ends_with('\n');

    let mut in_frontmatter = false;
    for (index, line) in content.lines().enumerate() {
        let mut current = line.to_string();
        if passes.links() {
            let is_fence = spec_links::is_frontmatter_fence(line);
            if index == 0 && is_fence {
                in_frontmatter = true;
            } else if in_frontmatter && is_fence {
                in_frontmatter = false;
            } else {
                let (rewritten, moved) = if in_frontmatter {
                    rewrite_folds_into(line, from, &target.feature)
                } else {
                    rewrite_body_line(line, from, target)
                };
                count += moved;
                current = rewritten;
            }
        }
        if passes.tokens() {
            let (rewritten, moved) = rewrite_path_tokens(&current, from, target, root);
            count += moved;
            current = rewritten;
        }
        push_line(&mut out, &current, ending);
    }

    if !ends_with_newline && out.len() >= ending.len() {
        // Drop the ending this loop added past the final line — the whole
        // ending, not one byte, or a CRLF file is left with a stray `\r`.
        out.truncate(out.len() - ending.len());
    }
    (out, count)
}

/// Append `line` and a newline.
fn push_line(out: &mut String, line: &str, ending: &str) {
    out.push_str(line);
    out.push_str(ending);
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

/// Re-point every path token on one referrer line that names `from` as its
/// first segment past the spec root.
///
/// The tokens are the ones `check-orphaned-references` finds, by its own
/// scanner given the spec root as the only managed root, so this rewrites
/// exactly what that check would report once the directory is gone — the
/// same reference written as a link, a code span, prose or a shell argument.
fn rewrite_path_tokens(line: &str, from: &str, target: &FoldTarget, root: &str) -> (String, u32) {
    let roots = [root.to_string()];
    let mut out = String::with_capacity(line.len());
    let mut copied = 0;
    let mut count = 0;
    for span in check_orphaned_references::managed_path_spans(line, &roots) {
        // `#` is not a path character, so a link's fragment sits just past
        // the token. It is taken along so that a fold into a scenario drops
        // it, as it drops a body link's.
        let end = span.end + fragment_len(&line[span.end..]);
        let Some(moved) = rewrite_root_token(&line[span.start..end], from, target) else {
            continue;
        };
        out.push_str(&line[copied..span.start]);
        out.push_str(&moved);
        copied = end;
        count += 1;
    }
    out.push_str(&line[copied..]);
    (out, count)
}

/// The byte length of a `#fragment` opening `rest`, or zero when there is
/// none. A heading anchor is letters, digits, `-` and `_`; anything after it
/// is prose.
fn fragment_len(rest: &str) -> usize {
    let Some(anchor) = rest.strip_prefix('#') else {
        return 0;
    };
    1 + anchor
        .chars()
        .take_while(|c| c.is_alphanumeric() || matches!(c, '-' | '_'))
        .map(char::len_utf8)
        .sum::<usize>()
}

/// The rewritten form of one token that starts with the spec root, or `None`
/// when its first segment past the root is not `from`.
///
/// Anchored at that segment rather than searched for: `specs/050-a/x/{from}`
/// names something inside another spec, not the retiring directory.
fn rewrite_root_token(token: &str, from: &str, target: &FoldTarget) -> Option<String> {
    let (path, fragment) = split_fragment(token);
    let segments: Vec<&str> = path.split('/').collect();
    // `segments[0]` is the spec root itself: the scanner returns no token
    // that does not start with it, and the root is one segment, since
    // `validate_specs_root` refuses a nested one.
    (segments.get(1) == Some(&from)).then(|| rebuild(&segments, 1, fragment, target))
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
    let (path, fragment) = split_fragment(link);
    let segments: Vec<&str> = path.split('/').collect();
    let index = segments.iter().position(|segment| *segment == from)?;
    Some(rebuild(&segments, index, fragment, target))
}

/// A link or token split at its first `#` into the path and the fragment.
fn split_fragment(link: &str) -> (&str, Option<&str>) {
    match link.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (link, None),
    }
}

/// `segments` re-pointed at the fold target, the one at `index` being the
/// retiring directory.
fn rebuild(segments: &[&str], index: usize, fragment: Option<&str>, target: &FoldTarget) -> String {
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
            if let Some(fragment) = fragment {
                out.push('#');
                out.push_str(fragment);
            }
            out
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
            rebuilt.join("/")
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::run;
    use crate::primitives::PrimitiveError;
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

    /// Give the fold target a `spec.md`. The primitive refuses to re-point
    /// links at a destination that is not a home, so a fixture that omits
    /// this is exercising a state the fold cannot reach.
    fn seed_target(repo: &Path, feature: &str) {
        write(
            repo,
            &format!("{feature}/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# target\n",
        );
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
        seed_target(tmp.path(), "050-alpha");

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
        seed_target(tmp.path(), "060-beta");

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
        seed_target(tmp.path(), "060-beta");

        run(&args("050-alpha", "060-beta/eviction"), tmp.path()).unwrap();

        let other = read(tmp.path(), "1234.1-other/spec.md");
        assert!(other.contains("folds-into: 060-beta"), "{other}");
    }

    /// AC29: the corpus is left untouched when the destination does not
    /// exist. Recovery is not available here — the rewrite is idempotent in
    /// the wrong direction, so once links are re-pointed nothing names the
    /// original and a second run finds nothing to repair.
    #[test]
    fn an_absent_fold_target_refuses_with_the_corpus_untouched() {
        let tmp = tempfile::tempdir().unwrap();
        let before = "---\nstatus: draft\ndependencies: []\n---\n\nSee [staged](../1234.1-staged/spec.md).\n";
        write(tmp.path(), "060-beta/spec.md", before);

        let err = run(&args("1234.1-staged", "050-nowhere"), tmp.path()).unwrap_err();

        assert!(
            matches!(err, PrimitiveError::FeatureNotFound { ref feature, .. } if feature == "050-nowhere"),
            "unexpected error: {err:?}"
        );
        assert_eq!(
            read(tmp.path(), "060-beta/spec.md"),
            before,
            "the inbound link must not have moved"
        );
    }

    /// Same terms `retire-feature` enforces: a directory with no `spec.md` is
    /// not a home content can have landed in, so it does not satisfy the
    /// check merely by existing.
    #[test]
    fn a_target_directory_without_a_spec_is_not_a_home() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "060-beta/spec.md",
            "---\nstatus: draft\ndependencies: []\n---\n\n[x](../1234.1-staged/spec.md)\n",
        );
        fs::create_dir_all(tmp.path().join("specs/050-alpha")).unwrap();

        let err = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap_err();

        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    /// A scenario-targeted `to` is checked on its **feature** half — that is
    /// what makes the rewritten link resolvable, and the scenario is written
    /// by an earlier step of the same fold.
    #[test]
    fn a_scenario_target_is_checked_on_its_feature() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "060-beta/spec.md",
            "---\nstatus: draft\ndependencies: []\n---\n\n[x](../1234.1-staged/spec.md)\n",
        );

        // Feature absent: refused, scenario half notwithstanding.
        let err = run(&args("1234.1-staged", "050-alpha/eviction"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));

        // Feature present: accepted, without the scenario file existing yet.
        seed_target(tmp.path(), "050-alpha");
        let result = run(&args("1234.1-staged", "050-alpha/eviction"), tmp.path()).unwrap();
        assert_eq!(result.rewritten.len(), 1);
    }

    /// BE-INPUT-004: neither argument may escape the spec root, since both
    /// reach a filesystem path.
    #[test]
    fn a_traversing_argument_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");

        for (from, to) in [("../../etc", "050-alpha"), ("1234.1-staged", "../../etc")] {
            let err = run(&args(from, to), tmp.path()).unwrap_err();
            assert!(
                matches!(err, PrimitiveError::InvalidPath { .. }),
                "expected a path refusal for ({from:?}, {to:?}), got {err:?}"
            );
        }
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

        assert_eq!(
            result.rewritten,
            [] as [crate::schema::primitives::RewrittenFile; 0]
        );
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

        assert_eq!(
            result.rewritten,
            [] as [crate::schema::primitives::RewrittenFile; 0]
        );
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

        assert_eq!(
            result.rewritten,
            [] as [crate::schema::primitives::RewrittenFile; 0]
        );
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

    // -- referrers outside the spec root (spec 051 task 29) -------------------

    /// Write a file at a repo-relative path — a referrer, not a spec file.
    fn write_root(repo: &Path, rel: &str, content: &str) {
        let path = repo.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn read_root(repo: &Path, rel: &str) -> String {
        fs::read_to_string(repo.join(rel)).unwrap()
    }

    fn rewritten_paths(result: &crate::schema::primitives::RewriteSpecLinksResult) -> Vec<&str> {
        result.rewritten.iter().map(|f| f.path.as_str()).collect()
    }

    /// AC6 and AC22: every file `check-orphaned-references` reads is
    /// re-pointed, in whichever form it names the retiring directory — an
    /// `@` import, a code span and bare prose, a markdown link, a shell
    /// argument — and `system.md`, which the spec-root walk already reached,
    /// gets its code span rewritten as well as its relative link.
    #[test]
    fn every_referrer_the_orphan_check_reads_is_re_pointed() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(
            tmp.path(),
            "CLAUDE.md",
            "# CLAUDE.md\n\n@specs/1234.1-staged/spec.md\n",
        );
        write_root(
            tmp.path(),
            "AGENTS.md",
            "See `specs/1234.1-staged/plan.md` and specs/1234.1-staged.\n",
        );
        write_root(
            tmp.path(),
            "README.md",
            "Read [the staged spec](specs/1234.1-staged/spec.md).\n",
        );
        write_root(
            tmp.path(),
            ".githooks/pre-commit",
            "#!/usr/bin/env bash\nbash specs/1234.1-staged/check.sh --staged\n",
        );
        write(
            tmp.path(),
            "system.md",
            "# System\n\nSee [staged](1234.1-staged/spec.md) and `specs/1234.1-staged/plan.md`.\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        let rewritten: Vec<(&str, u32)> = result
            .rewritten
            .iter()
            .map(|f| (f.path.as_str(), f.count))
            .collect();
        assert_eq!(
            rewritten,
            vec![
                ("specs/system.md", 2),
                ("CLAUDE.md", 1),
                ("AGENTS.md", 2),
                ("README.md", 1),
                (".githooks/pre-commit", 1),
            ]
        );
        // The two spec-root files plus the four referrers outside it.
        assert_eq!(result.examined, 6);

        assert_eq!(
            read_root(tmp.path(), "CLAUDE.md"),
            "# CLAUDE.md\n\n@specs/050-alpha/spec.md\n"
        );
        assert_eq!(
            read_root(tmp.path(), "AGENTS.md"),
            "See `specs/050-alpha/plan.md` and specs/050-alpha.\n"
        );
        assert_eq!(
            read_root(tmp.path(), "README.md"),
            "Read [the staged spec](specs/050-alpha/spec.md).\n"
        );
        assert_eq!(
            read_root(tmp.path(), ".githooks/pre-commit"),
            "#!/usr/bin/env bash\nbash specs/050-alpha/check.sh --staged\n"
        );
        assert_eq!(
            read(tmp.path(), "system.md"),
            "# System\n\nSee [staged](050-alpha/spec.md) and `specs/050-alpha/plan.md`.\n"
        );
    }

    /// AC22 as the check states it: `check-orphaned-references` reports the
    /// references a retired directory left in every referrer, and after the
    /// rewrite it reports none. The retiring directory is absent, as it is
    /// once `retire-feature` has run, because the check tests existence.
    #[test]
    fn the_orphan_check_reports_clean_where_it_reported_before() {
        use crate::primitives::check_orphaned_references;
        use crate::schema::primitives::CheckOrphanedReferencesArgs;

        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        for referrer in ["CLAUDE.md", "AGENTS.md", "README.md"] {
            write_root(
                tmp.path(),
                referrer,
                "See `specs/1234.1-staged/spec.md` and [it](specs/1234.1-staged).\n",
            );
        }
        write_root(
            tmp.path(),
            ".githooks/pre-commit",
            "#!/usr/bin/env bash\ngit add specs/1234.1-staged/spec.md\n",
        );
        write(
            tmp.path(),
            "system.md",
            "# System\n\nSee `specs/1234.1-staged/spec.md`.\n",
        );

        let before =
            check_orphaned_references::run(&CheckOrphanedReferencesArgs {}, tmp.path()).unwrap();
        let mut flagged: Vec<&str> = before
            .findings
            .iter()
            .map(|f| f.referrer.as_str())
            .collect();
        flagged.dedup();
        assert_eq!(
            flagged,
            vec![
                "CLAUDE.md",
                "AGENTS.md",
                "README.md",
                ".githooks/pre-commit",
                "specs/system.md"
            ],
            "the fixture must hold a finding in every referrer: {:?}",
            before.findings
        );

        run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        let after =
            check_orphaned_references::run(&CheckOrphanedReferencesArgs {}, tmp.path()).unwrap();
        assert!(after.findings.is_empty(), "{:?}", after.findings);
        assert_eq!(after.examined.len(), 5, "{:?}", after.examined);
    }

    /// Whole-segment matching holds outside the spec root too: a directory
    /// whose name merely starts with the retiring one is a different spec,
    /// and so is a directory of that name inside another spec — the match is
    /// anchored at the first segment past the root.
    #[test]
    fn a_referrer_naming_a_directory_sharing_the_prefix_is_left_alone() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        let line = "Not `specs/1234.1-staged-cache/spec.md`, nor specs/060-beta/1234.1-staged/x.md, \
                    but specs/1234.1-staged/spec.md.\n";
        write_root(tmp.path(), "README.md", line);

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(result.rewritten[0].count, 1, "{:?}", result.rewritten);
        assert_eq!(
            read_root(tmp.path(), "README.md"),
            "Not `specs/1234.1-staged-cache/spec.md`, nor specs/060-beta/1234.1-staged/x.md, \
             but specs/050-alpha/spec.md.\n"
        );
    }

    /// A URL naming another repository's spec is that repository's business,
    /// in a referrer as in a spec body.
    #[test]
    fn a_cross_repository_url_in_a_referrer_is_left_alone() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        let before =
            "See [api](https://github.com/acme/api/blob/main/specs/1234.1-staged/spec.md).\n";
        write_root(tmp.path(), "README.md", before);

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(rewritten_paths(&result), [] as [&str; 0]);
        assert_eq!(read_root(tmp.path(), "README.md"), before);
    }

    /// A revision-qualified path names a blob at that commit, where the
    /// directory still has its old name; the check does not report it, so
    /// rewriting it would falsify a record. The bare path beside it moves.
    #[test]
    fn a_revision_qualified_path_in_a_referrer_is_left_alone() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(
            tmp.path(),
            "AGENTS.md",
            "Recover it with `git show abc1234:specs/1234.1-staged/spec.md`.\n\
             Its home: specs/1234.1-staged/spec.md\n",
        );

        run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(
            read_root(tmp.path(), "AGENTS.md"),
            "Recover it with `git show abc1234:specs/1234.1-staged/spec.md`.\n\
             Its home: specs/050-alpha/spec.md\n"
        );
    }

    /// A project without some referrers: the absent ones are neither an
    /// error nor examined, and the present one is still rewritten.
    #[test]
    fn an_absent_referrer_is_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(tmp.path(), "AGENTS.md", "See specs/1234.1-staged/spec.md\n");

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(rewritten_paths(&result), vec!["AGENTS.md"]);
        // The target's `spec.md` and the one referrer that exists.
        assert_eq!(result.examined, 2);
    }

    /// `[paths] specs-root` decides the prefix a referrer names a feature
    /// directory behind; `specs/` on a project that renamed its root is not
    /// a reference to one of its specs.
    #[test]
    fn a_referrer_is_matched_behind_the_configured_spec_root() {
        let tmp = tempfile::tempdir().unwrap();
        write_root(
            tmp.path(),
            ".ductus/config.toml",
            "[paths]\nspecs-root = \"governance\"\n",
        );
        write_root(
            tmp.path(),
            "governance/050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n# target\n",
        );
        write_root(
            tmp.path(),
            "README.md",
            "See governance/1234.1-staged/spec.md, not specs/1234.1-staged/spec.md.\n",
        );

        run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(
            read_root(tmp.path(), "README.md"),
            "See governance/050-alpha/spec.md, not specs/1234.1-staged/spec.md.\n"
        );
    }

    /// A fold into a scenario collapses a referrer's reference onto it and
    /// drops the fragment, which named a heading in a file that is gone —
    /// the rule a body link follows. A rename keeps the fragment.
    #[test]
    fn a_referrer_fragment_follows_the_body_link_rule() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        let before = "See [it](specs/1234.1-staged/spec.md#behavior).\n";
        write_root(tmp.path(), "README.md", before);

        run(&args("1234.1-staged", "050-alpha/eviction"), tmp.path()).unwrap();
        assert_eq!(
            read_root(tmp.path(), "README.md"),
            "See [it](specs/050-alpha/scenarios/eviction.md).\n"
        );

        write_root(tmp.path(), "README.md", before);
        run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();
        assert_eq!(
            read_root(tmp.path(), "README.md"),
            "See [it](specs/050-alpha/spec.md#behavior).\n"
        );
    }

    /// The hook is rewritten in place, not recreated: a hook that lost its
    /// executable bit is one git skips, which is a silent failure at the
    /// next commit rather than a loud one now.
    #[cfg(unix)]
    #[test]
    fn a_rewritten_hook_keeps_its_executable_bit() {
        use std::os::unix::fs::PermissionsExt;

        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(
            tmp.path(),
            ".githooks/pre-commit",
            "#!/bin/sh\ngit add specs/1234.1-staged/spec.md\n",
        );
        let hook = tmp.path().join(".githooks/pre-commit");
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(rewritten_paths(&result), vec![".githooks/pre-commit"]);
        let mode = fs::metadata(&hook).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o755);
    }

    /// `CLAUDE.md` linked to `AGENTS.md` is a common layout. The atomic write
    /// replaces the path it is given, so following the link would turn it
    /// into a copy; the file it names is rewritten once, in its own right.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_referrer_stays_a_link() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(tmp.path(), "AGENTS.md", "See specs/1234.1-staged/spec.md\n");
        std::os::unix::fs::symlink("AGENTS.md", tmp.path().join("CLAUDE.md")).unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(rewritten_paths(&result), vec!["AGENTS.md"]);
        let claude = fs::symlink_metadata(tmp.path().join("CLAUDE.md")).unwrap();
        assert!(
            claude.file_type().is_symlink(),
            "CLAUDE.md was replaced by a copy"
        );
        assert_eq!(
            read_root(tmp.path(), "CLAUDE.md"),
            "See specs/050-alpha/spec.md\n"
        );
    }
}
