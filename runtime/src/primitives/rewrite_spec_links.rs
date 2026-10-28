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
//!   hold still dangles, and the check still reports it; one naming a file
//!   the target holds under the same name resolves, and the check passes it
//!   (see *Kept tails* below). `system.md` sits under the spec root, so it
//!   takes both passes.
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
//! **Every referrer is read before anything is written**, `system.md`
//! included, so a referrer that stops the run stops it with the tree
//! untouched. Each is classified first, and only a readable one is
//! rewritten — `system.md` by the spec-root walk, from the content read
//! here, and the rest after it:
//!
//! - **Absent** — not every project has every referrer — it is neither
//!   examined nor reported, as the check ignores it.
//! - **A symbolic link** is not followed. The atomic write replaces the file
//!   it is given, so writing through `CLAUDE.md` linked to `AGENTS.md` would
//!   turn the link into a copy. The file it names is rewritten in its own
//!   right when it is itself a referrer; the link is listed in `skipped`, so
//!   a reference reached only through it is reported unexamined rather than
//!   passed over, and `check-orphaned-references`, which reads through it,
//!   reports that reference once the directory is gone.
//! - **Reached through a directory that resolves outside the project** —
//!   `.githooks` linked to a shared hooks directory — it is listed in
//!   `skipped` and not written: the write would land in a file this project
//!   does not hold.
//! - **Not UTF-8 text** — listed in `skipped` with the reason the check
//!   gives when it skips the same file
//!   ([`check_orphaned_references::UNREADABLE_AS_TEXT`]), so the two agree on
//!   it.
//! - **Unreadable for any other reason, or not a regular file** — the run
//!   errors, before any write.
//!
//! A referrer under the spec root that is listed in `skipped` is passed over
//! by the walk, which neither reads nor writes it. Otherwise the walk reads
//! and writes one file at a time, and skips a symbolic link without listing
//! it. A referrer outside the spec root is never a file the walk writes, so
//! its content read beforehand is still its content when it is rewritten:
//! the walk writes markdown under the spec root, and a referrer outside it
//! could reach one only through a link — its own, which is skipped, or
//! `.githooks`, whose `pre-commit` is not markdown.
//!
//! **Kept tails.** A feature-only target keeps each reference's tail, which
//! is right for `spec.md`, where a body-edit fold writes, and for
//! `scenarios/{slug}.md`, since the fold carries each scenario across under
//! its own slug. Any other tail — `plan.md`, `tasks.md`, `data-model.md`,
//! `research.md` — lands on the upstream spec's own file of that name, which
//! the fold never wrote, and `check-orphaned-references` tests existence
//! only, so it stays clean. The design stays: such a reference is still
//! re-pointed, and listed in `kept-tails` with its file and line so the
//! caller can name it for the operator to judge. A scenario target keeps a
//! tail naming one of the retiring directory's scenarios, fragment and all,
//! since that scenario crosses over under its own slug on either route, and
//! replaces any other tail with the routed scenario. On either target a
//! `scenarios/{name}` tail the retiring directory does not hold, read before
//! anything is written, is listed as well: no fold carried that scenario, so
//! the reference lands on whatever the target holds under the name. The
//! exclusions are a fold's:
//! `/{project}:consolidate` carries no scenario across, so under it a kept
//! `scenarios/` tail resolves only where the target has a scenario of that
//! slug of its own, and is not listed either.
//!
//! Defined by `specs/051-branch-scoped-spec-numbering/spec.md` (AC6, AC22,
//! AC23, AC33).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::primitives::{
    PrimitiveError, Result, check_orphaned_references, read_text, spec_links, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    KeptTail, ReferrerSkip, RewriteSpecLinksArgs, RewriteSpecLinksResult, RewrittenFile,
};

/// The `skipped` reason for a referrer that is a symbolic link.
const SKIP_LINK: &str = "symbolic link, not followed";

/// The `skipped` reason for a referrer reached through a directory that
/// resolves outside the project.
const SKIP_OUTSIDE: &str = "resolves outside the project through a linked directory, not written";

/// Where the pointers are being sent.
struct FoldTarget {
    /// The upstream feature directory name.
    feature: String,
    /// The scenario slug within it, when the fold routed the content into
    /// a scenario rather than the spec body.
    scenario: Option<String>,
    /// The file names under the retiring directory's `scenarios/`, read
    /// before anything is written, or `None` when the directory is already
    /// gone (a rename). A kept `scenarios/{name}` tail the directory does not
    /// hold names a scenario the fold could not carry — one renamed before
    /// the fold, or never there — so it is listed in `kept-tails` rather than
    /// landing silently on whatever the target holds under that name.
    retiring_scenarios: Option<BTreeSet<String>>,
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
            retiring_scenarios: None,
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

/// A referrer read before anything is written, and the passes it takes.
struct PendingReferrer {
    path: PathBuf,
    content: String,
    passes: Passes,
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
    skipped: Vec<ReferrerSkip>,
    kept_tails: Vec<KeptTail>,
}

impl Sweep<'_> {
    /// Rewrite one file's content in place, recording it when anything in it
    /// moved.
    fn rewrite(&mut self, path: &Path, content: &str, passes: Passes) -> Result<()> {
        self.examined = self.examined.saturating_add(1);
        let rewrite = rewrite_content(content, self.from, &self.target, &self.root, passes);
        if rewrite.count == 0 {
            return Ok(());
        }
        write_atomic(path, &rewrite.text)?;
        let shown = repo_relative(self.repo, path);
        self.kept_tails.extend(
            rewrite
                .kept_tails
                .into_iter()
                .map(|(line, reference)| KeptTail {
                    path: shown.clone(),
                    line,
                    reference,
                }),
        );
        self.rewritten.push(RewrittenFile {
            path: shown,
            count: rewrite.count,
        });
        Ok(())
    }

    /// Walk the spec root, rewriting each markdown file outside the retiring
    /// directory as it is read. A referrer under the spec root is not read
    /// here: one in `pending` is taken out of it and rewritten from the
    /// content read before anything was written, and one that is not there
    /// was listed in `skipped` and is passed over.
    fn walk_spec_root(
        &mut self,
        specs_dir: &Path,
        referrers: &[PathBuf],
        pending: &mut Vec<PendingReferrer>,
    ) -> Result<()> {
        // Sorted so `rewritten` is in a stable path order rather than
        // whatever order the filesystem hands back — the same reason
        // `list_feature_dirs` sorts. A symbolic link is neither followed nor
        // listed: `file_type` does not follow one, so it is not `is_file`.
        let walk = WalkDir::new(specs_dir)
            .sort_by_file_name()
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"));

        for entry in walk {
            let path = entry.path();
            // The retiring directory's own files are leaving with it.
            // Rewriting them would write into a tree about to be removed, and
            // counting them would overstate what was actually repaired. On a
            // rename the directory is already gone, so this skip is a no-op
            // there.
            if path
                .strip_prefix(specs_dir)
                .ok()
                .and_then(|rel| rel.components().next())
                .is_some_and(|first| first.as_os_str() == self.from)
            {
                continue;
            }
            if let Some(index) = pending.iter().position(|referrer| referrer.path == path) {
                let referrer = pending.remove(index);
                self.rewrite(path, &referrer.content, referrer.passes)?;
            } else if !referrers.iter().any(|referrer| referrer == path) {
                // Not a referrer. One that is but is not pending was listed in
                // `skipped`, and is passed over unread.
                let content = read_text(path)?;
                self.rewrite(path, &content, Passes::Links)?;
            }
        }
        Ok(())
    }

    /// Read every referrer, classifying each, before anything is written;
    /// return the ones read, in the order the check reads them — one under
    /// the spec root (`system.md`) to take both passes, the rest the token
    /// pass. See the module doc for the classes.
    ///
    /// The outside-project test is [`super::validate_contained`], the
    /// containment rule the primitives apply to a project-relative path they
    /// open, rather than a narrow check of its own: it resolves the project
    /// root and every link on the path (`resolve_symlinks`, so macOS's
    /// `/tmp` and `/private/tmp` compare alike) and compares them by
    /// component. What else it enforces by construction is vacuous here — its
    /// traversal screen (the referrer names are fixed), its refusal of a link
    /// that cannot be resolved (`symlink_metadata` has already reached the
    /// file through every one), and its judgment of a final-component link
    /// (one is skipped above before it runs). `host_config::check_destination`
    /// does not fit: it judges whether a destination is a host's
    /// configuration, and its module doc says a destination that resolves
    /// outside the project is not judged there.
    fn read_referrers(
        &mut self,
        referrers: &[String],
        specs_dir: &Path,
    ) -> Result<Vec<PendingReferrer>> {
        let mut pending = Vec::new();
        for shown in referrers {
            let path = self.repo.join(shown);
            // `symlink_metadata` judges the final component as given, which is
            // where the atomic write lands, and follows every directory link
            // before it. A missing directory on the path (`.githooks` absent,
            // or a file) is as absent as a missing file.
            let meta = match std::fs::symlink_metadata(&path) {
                Ok(meta) => meta,
                Err(err)
                    if matches!(
                        err.kind(),
                        std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                    ) =>
                {
                    continue;
                }
                Err(source) => return Err(PrimitiveError::Io { path, source }),
            };
            if meta.file_type().is_symlink() {
                self.skip(shown, SKIP_LINK);
                continue;
            }
            match super::validate_contained(self.repo, shown) {
                Ok(()) => {}
                Err(PrimitiveError::InvalidPath { .. }) => {
                    self.skip(shown, SKIP_OUTSIDE);
                    continue;
                }
                Err(other) => return Err(other),
            }
            // Refused before it is opened: a FIFO would block the read.
            if !meta.is_file() {
                return Err(PrimitiveError::Io {
                    path,
                    source: std::io::Error::other("exists but is not a regular file"),
                });
            }
            let bytes = std::fs::read(&path).map_err(|source| PrimitiveError::Io {
                path: path.clone(),
                source,
            })?;
            let Ok(content) = String::from_utf8(bytes) else {
                self.skip(shown, check_orphaned_references::UNREADABLE_AS_TEXT);
                continue;
            };
            // Under the spec root (`system.md`), the walk rewrites it, with
            // its body links as well as its path tokens.
            let passes = if path.starts_with(specs_dir) {
                Passes::Both
            } else {
                Passes::Tokens
            };
            pending.push(PendingReferrer {
                path,
                content,
                passes,
            });
        }
        Ok(pending)
    }

    fn skip(&mut self, shown: &str, reason: &str) {
        self.skipped.push(ReferrerSkip {
            path: shown.to_string(),
            reason: reason.to_string(),
        });
    }
}

/// Execute the `rewrite-spec-links` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidArgument`] when `from` is empty or `to`
/// does not parse as a fold target, [`PrimitiveError::InvalidPath`] when
/// either is absolute, climbs with `..`, or has a symbolic link on or under
/// its directory that resolves outside the project, and
/// [`PrimitiveError::FeatureNotFound`] when the target holds no `spec.md` —
/// each before anything is written. Returns [`PrimitiveError::Io`] when a
/// referrer, `system.md` included, cannot be examined for a reason other
/// than its absence or its encoding — its metadata or its bytes cannot be
/// read, or it is not a regular file — before anything is written; when a
/// markdown file under the spec root that is not a referrer cannot be read
/// as UTF-8 text, part-way through the walk and so after the files before it
/// were written; or when a file cannot be written. A referrer that is not
/// UTF-8 text is not an error: it is listed in `skipped`, as a symbolic link
/// and a referrer resolving outside the project are.
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
    let mut target = FoldTarget::parse(&args.to)?;
    let specs_dir = repo.join(&layout.specs_root);
    target.retiring_scenarios = retiring_scenarios(&specs_dir.join(&args.from))?;

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
    let referrers = check_orphaned_references::referrers(&layout);
    let referrer_paths: Vec<PathBuf> = referrers
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
        skipped: Vec::new(),
        kept_tails: Vec::new(),
    };

    // Every referrer is read first, so one that stops the run stops it
    // before anything is written. The walk then takes `system.md` out of
    // `pending` where it meets it, and what is left — the referrers outside
    // the spec root, and `system.md` only where the walk could not list the
    // directory holding it — is rewritten after it, each with its own
    // passes, in the order the check reads them.
    let mut pending = sweep.read_referrers(&referrers, &specs_dir)?;
    sweep.walk_spec_root(&specs_dir, &referrer_paths, &mut pending)?;
    for referrer in pending {
        sweep.rewrite(&referrer.path, &referrer.content, referrer.passes)?;
    }

    Ok(RewriteSpecLinksResult {
        rewritten: sweep.rewritten,
        examined: sweep.examined,
        skipped: sweep.skipped,
        kept_tails: sweep.kept_tails,
    })
}

/// `path` relative to the repo root, with forward slashes.
fn repo_relative(repo: &Path, path: &Path) -> String {
    path.strip_prefix(repo)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// One file's content with its pointers re-pointed.
struct FileRewrite {
    text: String,
    /// Pointers moved.
    count: u32,
    /// `(line, reference)` for each moved reference whose kept tail names a
    /// file the fold did not write ([`names_a_file_the_fold_did_not_write`]),
    /// the line 1-based.
    kept_tails: Vec<(u32, String)>,
}

/// One re-pointed reference.
struct Moved {
    /// The reference as rewritten.
    text: String,
    /// Whether a feature-only target kept a tail naming a file the fold did
    /// not write.
    kept_tail: bool,
}

/// Rewrite one file's pointers.
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
) -> FileRewrite {
    // The file's own ending, not a bare `\n`. `lines()` strips a trailing
    // `\r`, so re-joining with `\n` would convert a CRLF checkout's file to
    // LF as a side effect of re-pointing one link.
    let ending = crate::primitives::line_ending_of(content);
    let mut out = String::with_capacity(content.len());
    let mut count = 0;
    let mut kept_tails = Vec::new();
    // A trailing newline is not a line, and `lines()` drops it; re-add it
    // only if the input had one, so a file's final byte is preserved.
    let ends_with_newline = content.ends_with('\n');

    let mut in_frontmatter = false;
    for (index, line) in content.lines().enumerate() {
        let mut current = line.to_string();
        let mut kept = Vec::new();
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
                    rewrite_body_line(line, from, target, &mut kept)
                };
                count += moved;
                current = rewritten;
            }
        }
        if passes.tokens() {
            let (rewritten, moved) = rewrite_path_tokens(&current, from, target, root, &mut kept);
            count += moved;
            current = rewritten;
        }
        let line_number = u32::try_from(index + 1).unwrap_or(u32::MAX);
        kept_tails.extend(kept.into_iter().map(|reference| (line_number, reference)));
        push_line(&mut out, &current, ending);
    }

    if !ends_with_newline && out.len() >= ending.len() {
        // Drop the ending this loop added past the final line — the whole
        // ending, not one byte, or a CRLF file is left with a stray `\r`.
        out.truncate(out.len() - ending.len());
    }
    FileRewrite {
        text: out,
        count,
        kept_tails,
    }
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
/// as a whole path segment, pushing each kept-tail reference onto `kept`.
fn rewrite_body_line(
    line: &str,
    from: &str,
    target: &FoldTarget,
    kept: &mut Vec<String>,
) -> (String, u32) {
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
                out.push_str(&moved.text);
                if moved.kept_tail {
                    kept.push(moved.text);
                }
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
/// Each kept-tail reference is pushed onto `kept`.
fn rewrite_path_tokens(
    line: &str,
    from: &str,
    target: &FoldTarget,
    root: &str,
    kept: &mut Vec<String>,
) -> (String, u32) {
    let roots = [root.to_string()];
    let mut out = String::with_capacity(line.len());
    let mut copied = 0;
    let mut count = 0;
    for span in check_orphaned_references::managed_path_spans(line, &roots) {
        // A span that starts inside the text the previous rewrite consumed.
        // A heading anchor's characters include none of `/`, so in
        // `spec.md#specs/{from}/plan.md` the first token takes `#specs`
        // along and ends inside the second, and splicing the second would
        // slice backwards. It is left as written. On a rename the kept
        // fragment restores the `specs` it took, so the token still names the
        // retiring directory; on a scenario target the fragment is dropped
        // when the first token collapses onto the routed scenario, joining the
        // rest to that scenario's path, which names no file, and kept when the
        // first token names a carried scenario, which restores the `specs` as
        // on a rename. Either way `check-orphaned-references` reports it once
        // the directory is gone.
        if span.start < copied {
            continue;
        }
        // `#` is not a path character, so a link's fragment sits just past
        // the token. It is taken along so that a fold into a scenario drops
        // it, as it drops a body link's.
        let end = span.end + fragment_len(&line[span.end..]);
        let Some(moved) = rewrite_root_token(&line[span.start..end], from, target) else {
            continue;
        };
        out.push_str(&line[copied..span.start]);
        out.push_str(&moved.text);
        if moved.kept_tail {
            kept.push(moved.text);
        }
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
fn rewrite_root_token(token: &str, from: &str, target: &FoldTarget) -> Option<Moved> {
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
fn rewrite_link_target(link: &str, from: &str, target: &FoldTarget) -> Option<Moved> {
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
fn rebuild(segments: &[&str], index: usize, fragment: Option<&str>, target: &FoldTarget) -> Moved {
    let mut rebuilt: Vec<String> = segments[..index]
        .iter()
        .map(|segment| (*segment).to_string())
        .collect();
    rebuilt.push(target.feature.clone());
    let tail = &segments[index + 1..];

    match &target.scenario {
        // A fold into a scenario, and a tail naming no scenario of the
        // retiring directory: that file did not survive individually — its
        // content landed in the routed scenario — so the tail names nothing
        // and is replaced rather than carried. Any fragment goes with it,
        // because it named a heading in a file that no longer exists;
        // keeping it would produce a link that resolves to a file and lands
        // nowhere in it.
        Some(slug) if !names_a_scenario(tail) => {
            rebuilt.push("scenarios".to_string());
            rebuilt.push(format!("{slug}.md"));
            Moved {
                text: rebuilt.join("/"),
                kept_tail: false,
            }
        }
        // Otherwise the tail is kept, and any fragment naming a heading
        // inside it.
        //
        // On a scenario target the tail names one of the retiring
        // directory's scenarios, which crosses over under its own slug on
        // either route (`/{project}:fold` step 7, which stops the fold when
        // that slug is already taken by a file the fold did not write), so
        // it lands on the carried scenario. Such a tail is never a kept tail.
        //
        // On a feature-only target the directory's files map across one for
        // one on a rename; a body-edit fold writes only some of them and a
        // consolidation none, so a kept tail naming a file the target was not
        // given is flagged for `kept-tails`
        // (`names_a_file_the_fold_did_not_write`).
        //
        // On either target, a scenario tail the retiring directory does not
        // hold is flagged too: no fold carried that scenario, so the
        // reference lands on whatever the target holds under the name.
        _ => {
            rebuilt.extend(tail.iter().map(|s| (*s).to_string()));
            let mut out = rebuilt.join("/");
            if let Some(fragment) = fragment {
                out.push('#');
                out.push_str(fragment);
            }
            Moved {
                text: out,
                kept_tail: names_a_file_the_fold_did_not_write(tail)
                    || target.does_not_hold_scenario(tail),
            }
        }
    }
}

impl FoldTarget {
    /// Whether `tail` names a scenario the retiring directory, read before
    /// the run wrote anything, does not hold. Unknown when the directory was
    /// already gone, and then never flagged.
    fn does_not_hold_scenario(&self, tail: &[&str]) -> bool {
        match (tail, &self.retiring_scenarios) {
            (["scenarios", name, ..], Some(held)) if !name.is_empty() => !held.contains(*name),
            _ => false,
        }
    }
}

/// The file names under `dir/scenarios/`, or `None` when `dir` does not
/// exist. A `dir` with no `scenarios/` holds none.
///
/// # Errors
///
/// [`PrimitiveError::Io`] when `dir/scenarios/` exists and cannot be listed.
fn retiring_scenarios(dir: &Path) -> Result<Option<BTreeSet<String>>> {
    if !dir.is_dir() {
        return Ok(None);
    }
    let scenarios = dir.join("scenarios");
    let entries = match std::fs::read_dir(&scenarios) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Some(BTreeSet::new()));
        }
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: scenarios,
                source,
            });
        }
    };
    let mut held = BTreeSet::new();
    for entry in entries {
        let entry = entry.map_err(|source| PrimitiveError::Io {
            path: scenarios.clone(),
            source,
        })?;
        held.insert(entry.file_name().to_string_lossy().into_owned());
    }
    Ok(Some(held))
}

/// Whether `tail`, the segments after the retiring directory, names one of
/// its scenarios: `scenarios/{name}` with a name. A reference to the
/// `scenarios` directory itself names none.
fn names_a_scenario(tail: &[&str]) -> bool {
    matches!(tail, ["scenarios", name, ..] if !name.is_empty())
}

/// Whether `tail`, the segments a feature-only target keeps after the
/// retiring directory, names a file the fold did not write in the target.
///
/// None does when it is empty — the directory itself — or `spec.md`, where a
/// body-edit fold writes, or under `scenarios/`, since the fold carries each
/// scenario across under its own slug (`/{project}:fold` step 7). Any other
/// file is the upstream spec's own of that name, so a reference to it
/// resolves to content the fold never wrote, and nothing else reports it:
/// `check-orphaned-references` tests existence only.
fn names_a_file_the_fold_did_not_write(tail: &[&str]) -> bool {
    !matches!(tail, [] | ["" | "spec.md"] | ["scenarios", ..])
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{SKIP_LINK, SKIP_OUTSIDE, run};
    use crate::primitives::PrimitiveError;
    use crate::schema::primitives::{ReferrerSkip, RewriteSpecLinksArgs};
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

    /// A target that routes the content into a scenario: a link to the
    /// spec or to any other file of the retiring directory collapses onto the
    /// routed scenario, because that file does not survive the fold, and so
    /// does one to its `scenarios` directory, which names no one scenario. A
    /// link naming a scenario *of* the retiring
    /// directory keeps its tail and fragment, because the fold carries that
    /// scenario across under its own slug on either route.
    #[test]
    fn a_scenario_target_collapses_the_rest_and_keeps_a_carried_scenario() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "050-alpha/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n\
             See [spec](../1234.1-staged/spec.md), [plan](../1234.1-staged/plan.md#steps), \
             [the scenarios](../1234.1-staged/scenarios/) and \
             [a scenario](../1234.1-staged/scenarios/inner.md#behavior).\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha/eviction"), tmp.path()).unwrap();

        assert_eq!(result.rewritten[0].count, 4);
        let alpha = read(tmp.path(), "050-alpha/spec.md");
        assert!(
            alpha.contains("[spec](../050-alpha/scenarios/eviction.md)"),
            "{alpha}"
        );
        // The `scenarios` directory itself names no scenario, so it collapses
        // with the rest.
        assert!(
            alpha.contains("[the scenarios](../050-alpha/scenarios/eviction.md)"),
            "{alpha}"
        );
        // The fragment named a heading in a file that no longer exists, so it
        // goes with the tail rather than resolving to nothing.
        assert!(
            alpha.contains("[plan](../050-alpha/scenarios/eviction.md)"),
            "{alpha}"
        );
        assert!(!alpha.contains("#steps"), "{alpha}");
        assert!(
            alpha.contains("[a scenario](../050-alpha/scenarios/inner.md#behavior)"),
            "{alpha}"
        );
        assert!(result.kept_tails.is_empty(), "{:?}", result.kept_tails);
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

    /// A `.githooks` that is a regular file holds no `pre-commit`: the path
    /// names nothing, as when `.githooks` is absent, so it is neither an
    /// error nor examined nor skipped, and the rest is still rewritten. The
    /// check reads it the same way (`exists` is false through a file).
    #[test]
    fn a_referrer_under_a_file_where_its_directory_should_be_is_absent() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(tmp.path(), ".githooks", "not a directory\n");
        write_root(tmp.path(), "AGENTS.md", "See specs/1234.1-staged/spec.md\n");

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(rewritten_paths(&result), vec!["AGENTS.md"]);
        assert_eq!(result.skipped, [] as [ReferrerSkip; 0]);
        assert_eq!(result.examined, 2);
        assert_eq!(read_root(tmp.path(), ".githooks"), "not a directory\n");
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

    /// `CLAUDE.md` linked to `AGENTS.md` is a layout an adopter may choose.
    /// The atomic write replaces the path it is given, so following the link
    /// would turn it into a copy; the file it names is rewritten once, in
    /// its own right.
    /// The link is listed in `skipped`, not passed over: a link to a file
    /// that is not itself a referrer would otherwise hide every reference in
    /// it behind a clean result.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_referrer_stays_a_link() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(tmp.path(), "AGENTS.md", "See specs/1234.1-staged/spec.md\n");
        std::os::unix::fs::symlink("AGENTS.md", tmp.path().join("CLAUDE.md")).unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(rewritten_paths(&result), vec!["AGENTS.md"]);
        assert_eq!(
            result.skipped,
            vec![ReferrerSkip {
                path: "CLAUDE.md".into(),
                reason: SKIP_LINK.into(),
            }]
        );
        // The target's `spec.md` and `AGENTS.md`; the link is not counted.
        assert_eq!(result.examined, 2);
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

    // -- skipped referrers, kept tails, a fragment over the next token -------

    /// A referrer that is not UTF-8 text is reported, not raised: the rest
    /// is still rewritten, the spec root among it, and the file is named in
    /// the words `check-orphaned-references` uses when it skips the same
    /// one, so the two primitives agree about it. The reason is one shared
    /// constant, so the comparison holds the rest of that agreement: it
    /// fails if the rewrite stops skipping the file, spells its path
    /// differently, or gives a reason of its own.
    #[test]
    fn a_non_utf8_referrer_is_skipped_and_the_rest_is_still_rewritten() {
        use crate::primitives::check_orphaned_references;
        use crate::schema::primitives::CheckOrphanedReferencesArgs;

        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write(
            tmp.path(),
            "060-beta/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\nSee [s](../1234.1-staged/spec.md).\n",
        );
        write_root(tmp.path(), "AGENTS.md", "See specs/1234.1-staged/spec.md\n");
        // Latin-1 `é` (0xE9) makes the file invalid UTF-8.
        let latin1: &[u8] = b"Caf\xe9 notes: specs/1234.1-staged/spec.md\n";
        fs::write(tmp.path().join("README.md"), latin1).unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(
            rewritten_paths(&result),
            vec!["specs/060-beta/spec.md", "AGENTS.md"]
        );
        assert!(
            read(tmp.path(), "060-beta/spec.md").contains("[s](../050-alpha/spec.md)"),
            "the spec root was not rewritten"
        );
        assert_eq!(fs::read(tmp.path().join("README.md")).unwrap(), latin1);
        // The two spec-root files and `AGENTS.md`; `README.md` is listed in
        // `skipped`, not counted.
        assert_eq!(result.examined, 3);

        let check =
            check_orphaned_references::run(&CheckOrphanedReferencesArgs {}, tmp.path()).unwrap();
        let theirs: Vec<(&str, &str)> = check
            .skipped
            .iter()
            .map(|s| (s.path.as_str(), s.reason.as_str()))
            .collect();
        let ours: Vec<(&str, &str)> = result
            .skipped
            .iter()
            .map(|s| (s.path.as_str(), s.reason.as_str()))
            .collect();
        assert_eq!(ours, theirs);
        assert_eq!(
            ours.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
            vec!["README.md"]
        );
    }

    /// A referrer that cannot be read for any reason but its encoding stops
    /// the run, and stops it before anything is written — the spec root, and
    /// a referrer read before it, alike — because every referrer is read
    /// before the walk. A directory where `AGENTS.md` should be is the
    /// portable unreadable file, and it is refused as not a regular file
    /// before it is opened, which is what keeps a FIFO from blocking the
    /// read; the error's text is what separates that refusal from the read
    /// error a directory would raise anyway.
    #[test]
    fn an_unreadable_referrer_stops_the_run_before_anything_is_written() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        let beta =
            "---\nstatus: done\ndependencies: []\n---\n\nSee [s](../1234.1-staged/spec.md).\n";
        write(tmp.path(), "060-beta/spec.md", beta);
        let claude = "@specs/1234.1-staged/spec.md\n";
        write_root(tmp.path(), "CLAUDE.md", claude);
        fs::create_dir_all(tmp.path().join("AGENTS.md")).unwrap();

        let err = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap_err();

        assert!(
            matches!(
                err,
                PrimitiveError::Io { ref path, ref source }
                    if path.ends_with("AGENTS.md")
                        && source.to_string() == "exists but is not a regular file"
            ),
            "unexpected error: {err:?}"
        );
        assert_eq!(
            read(tmp.path(), "060-beta/spec.md"),
            beta,
            "the spec root was written before the error"
        );
        assert_eq!(read_root(tmp.path(), "CLAUDE.md"), claude);
    }

    /// `.githooks` linked to a directory outside the project: the hook there
    /// is not this project's file, so it is listed in `skipped` and left
    /// untouched. The same link to a directory inside the project is
    /// followed and the hook rewritten — the refusal asserted in both
    /// directions, which also proves the project root and the link are
    /// resolved alike (a fixture under macOS's `/var` resolves to
    /// `/private/var`).
    #[cfg(unix)]
    #[test]
    fn a_referrer_behind_a_directory_linked_outside_the_project_is_not_written() {
        let tmp = tempfile::tempdir().unwrap();
        let shared = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        let hook = "#!/bin/sh\ntest -f specs/1234.1-staged/spec.md\n";
        fs::write(shared.path().join("pre-commit"), hook).unwrap();
        std::os::unix::fs::symlink(shared.path(), tmp.path().join(".githooks")).unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(
            result.skipped,
            vec![ReferrerSkip {
                path: ".githooks/pre-commit".into(),
                reason: SKIP_OUTSIDE.into(),
            }]
        );
        assert_eq!(rewritten_paths(&result), [] as [&str; 0]);
        assert_eq!(
            fs::read_to_string(shared.path().join("pre-commit")).unwrap(),
            hook,
            "a file outside the project was written"
        );

        fs::remove_file(tmp.path().join(".githooks")).unwrap();
        write_root(tmp.path(), "tools/hooks/pre-commit", hook);
        std::os::unix::fs::symlink("tools/hooks", tmp.path().join(".githooks")).unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(result.skipped, [] as [ReferrerSkip; 0]);
        assert_eq!(rewritten_paths(&result), vec![".githooks/pre-commit"]);
        assert_eq!(
            read_root(tmp.path(), "tools/hooks/pre-commit"),
            "#!/bin/sh\ntest -f specs/050-alpha/spec.md\n"
        );
    }

    /// A feature-only target keeps each tail. `spec.md`, a scenario and the
    /// directory itself land on what the fold wrote; `plan.md` and
    /// `research.md` land on the upstream spec's own files, which the fold
    /// never wrote and the orphan check, testing existence only, passes — so
    /// those are listed, with file and line, for body links and referrer
    /// tokens alike, and still re-pointed. A scenario target keeps only the
    /// scenario's tail, replaces the rest, and lists none.
    #[test]
    fn a_kept_tail_naming_a_file_the_fold_did_not_write_is_listed() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        let beta = "---\nstatus: done\ndependencies: []\n---\n\n\
                    See [s](../1234.1-staged/spec.md), [d](../1234.1-staged),\n\
                    [x](../1234.1-staged/scenarios/x.md) and [p](../1234.1-staged/plan.md#steps).\n";
        write(tmp.path(), "060-beta/spec.md", beta);
        let agents =
            "Spec: specs/1234.1-staged/spec.md\nResearch: `specs/1234.1-staged/research.md`\n";
        write_root(tmp.path(), "AGENTS.md", agents);

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        let kept: Vec<(&str, u32, &str)> = result
            .kept_tails
            .iter()
            .map(|k| (k.path.as_str(), k.line, k.reference.as_str()))
            .collect();
        assert_eq!(
            kept,
            vec![
                ("specs/060-beta/spec.md", 7, "../050-alpha/plan.md#steps"),
                ("AGENTS.md", 2, "specs/050-alpha/research.md"),
            ]
        );
        assert!(
            read(tmp.path(), "060-beta/spec.md").contains("[p](../050-alpha/plan.md#steps)"),
            "a listed reference must still be re-pointed"
        );

        write(tmp.path(), "060-beta/spec.md", beta);
        write_root(tmp.path(), "AGENTS.md", agents);
        let result = run(&args("1234.1-staged", "050-alpha/eviction"), tmp.path()).unwrap();

        assert_eq!(
            rewritten_paths(&result),
            vec!["specs/060-beta/spec.md", "AGENTS.md"]
        );
        assert!(result.kept_tails.is_empty(), "{:?}", result.kept_tails);
    }

    /// A reference to a scenario the retiring directory does not hold — one
    /// renamed before the fold, or never there — is listed on either target,
    /// and still re-pointed, since no fold carried it and the target's file of
    /// that name is not its content. A scenario the directory holds is not.
    #[test]
    fn a_scenario_the_retiring_directory_does_not_hold_is_listed() {
        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write(
            tmp.path(),
            "1234.1-staged/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n",
        );
        write(
            tmp.path(),
            "1234.1-staged/scenarios/inner.md",
            "---\nsection: x\n---\n",
        );
        let beta = "---\nstatus: done\ndependencies: []\n---\n\n\
                    [held](../1234.1-staged/scenarios/inner.md) and \
                    [gone](../1234.1-staged/scenarios/edge-cases.md#behavior)\n";
        let agents = "Old: `specs/1234.1-staged/scenarios/edge-cases.md`\n";

        for to in ["050-alpha", "050-alpha/eviction"] {
            write(tmp.path(), "060-beta/spec.md", beta);
            write_root(tmp.path(), "AGENTS.md", agents);

            let result = run(&args("1234.1-staged", to), tmp.path()).unwrap();

            let kept: Vec<(&str, &str)> = result
                .kept_tails
                .iter()
                .map(|k| (k.path.as_str(), k.reference.as_str()))
                .collect();
            assert_eq!(
                kept,
                vec![
                    (
                        "specs/060-beta/spec.md",
                        "../050-alpha/scenarios/edge-cases.md#behavior"
                    ),
                    ("AGENTS.md", "specs/050-alpha/scenarios/edge-cases.md"),
                ],
                "target {to}"
            );
            let beta_now = read(tmp.path(), "060-beta/spec.md");
            assert!(
                beta_now.contains("[held](../050-alpha/scenarios/inner.md)"),
                "{beta_now}"
            );
        }
    }

    /// A fragment that runs into a second spec-root token on one line
    /// (`#specs/…`): the first token takes `#specs` along and ends inside
    /// the second, which is left as written rather than spliced backwards —
    /// once a panic, raised after the spec root had been written. The token
    /// left behind still names the retiring directory, and the orphan check
    /// reports it; the retiring directory is absent here, as once retired.
    #[test]
    fn a_fragment_running_into_the_next_token_does_not_panic() {
        use crate::primitives::check_orphaned_references;
        use crate::schema::primitives::CheckOrphanedReferencesArgs;

        let tmp = tempfile::tempdir().unwrap();
        seed_target(tmp.path(), "050-alpha");
        write_root(
            tmp.path(),
            "README.md",
            "odd: specs/1234.1-staged/spec.md#specs/1234.1-staged/plan.md\n",
        );

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(rewritten_paths(&result), vec!["README.md"]);
        assert_eq!(result.rewritten[0].count, 1);
        assert_eq!(
            read_root(tmp.path(), "README.md"),
            "odd: specs/050-alpha/spec.md#specs/1234.1-staged/plan.md\n"
        );

        let check =
            check_orphaned_references::run(&CheckOrphanedReferencesArgs {}, tmp.path()).unwrap();
        let targets: Vec<&str> = check.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(targets, vec!["specs/1234.1-staged/plan.md"]);
    }

    /// A spec root holding a sibling spec that names the retiring directory,
    /// for the `system.md` tests: `060-beta`'s link and `AGENTS.md`'s token
    /// are the rest that must still be rewritten.
    fn seed_beside_system_md(repo: &Path) {
        seed_target(repo, "050-alpha");
        write(
            repo,
            "060-beta/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\nSee [s](../1234.1-staged/spec.md).\n",
        );
        write_root(repo, "AGENTS.md", "See specs/1234.1-staged/spec.md\n");
    }

    /// `system.md` is a referrer under the spec root, and is classified with
    /// the others before anything is written: as a symbolic link it is listed
    /// in `skipped` — where the walk alone would pass over it without a word
    /// — the link stays a link, the file it names is untouched, and the rest
    /// is still rewritten.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_system_md_is_skipped_and_the_rest_is_still_rewritten() {
        let tmp = tempfile::tempdir().unwrap();
        seed_beside_system_md(tmp.path());
        let system =
            "# System\n\nSee [staged](1234.1-staged/spec.md) and `specs/1234.1-staged/plan.md`.\n";
        write_root(tmp.path(), "docs/system.md", system);
        std::os::unix::fs::symlink("../docs/system.md", tmp.path().join("specs/system.md"))
            .unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(
            result.skipped,
            vec![ReferrerSkip {
                path: "specs/system.md".into(),
                reason: SKIP_LINK.into(),
            }]
        );
        assert_eq!(
            rewritten_paths(&result),
            vec!["specs/060-beta/spec.md", "AGENTS.md"]
        );
        // The two spec files and `AGENTS.md`.
        assert_eq!(result.examined, 3);
        assert_eq!(read_root(tmp.path(), "docs/system.md"), system);
        assert!(
            fs::symlink_metadata(tmp.path().join("specs/system.md"))
                .unwrap()
                .file_type()
                .is_symlink(),
            "specs/system.md was replaced by a copy"
        );
    }

    /// A `system.md` that is not UTF-8 text is listed in `skipped` in the
    /// check's words, before anything is written, and the walk passes over
    /// it rather than reading it — which raised an error part-way through
    /// the walk, after the files before it had been written. Its bytes are
    /// unchanged and the rest is still rewritten.
    #[test]
    fn a_non_utf8_system_md_is_skipped_and_the_rest_is_still_rewritten() {
        use crate::primitives::check_orphaned_references;
        use crate::schema::primitives::CheckOrphanedReferencesArgs;

        let tmp = tempfile::tempdir().unwrap();
        seed_beside_system_md(tmp.path());
        // Latin-1 `è` (0xE8) makes the file invalid UTF-8.
        let latin1: &[u8] =
            b"# Syst\xe8me\n\nSee [staged](1234.1-staged/spec.md) and `specs/1234.1-staged/plan.md`.\n";
        fs::write(tmp.path().join("specs/system.md"), latin1).unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert_eq!(
            result.skipped,
            vec![ReferrerSkip {
                path: "specs/system.md".into(),
                reason: check_orphaned_references::UNREADABLE_AS_TEXT.into(),
            }]
        );
        assert_eq!(
            rewritten_paths(&result),
            vec!["specs/060-beta/spec.md", "AGENTS.md"]
        );
        assert_eq!(result.examined, 3);
        assert_eq!(
            fs::read(tmp.path().join("specs/system.md")).unwrap(),
            latin1
        );

        let check =
            check_orphaned_references::run(&CheckOrphanedReferencesArgs {}, tmp.path()).unwrap();
        assert_eq!(check.skipped.len(), 1, "{:?}", check.skipped);
        assert_eq!(
            (
                check.skipped[0].path.as_str(),
                check.skipped[0].reason.as_str()
            ),
            (
                result.skipped[0].path.as_str(),
                result.skipped[0].reason.as_str()
            )
        );
    }
}
