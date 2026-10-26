//! The **standing** inbox backlog — how deep the queue is and how old its
//! oldest item is — rendered by `dashboard` as `/{project}:status`'s `Inbox:`
//! line.
//!
//! It began as the other half of a pair. `write-review` and `diff-cross-spec`
//! each showed a **window** — the bullets captured while the feature in hand
//! was open — and a window cannot show an item older than that feature: six
//! stood in the inbox when `ductus-v0.47.0` was cut, and they appeared in that
//! feature's reports only because all six happened to land inside its window.
//! Spec 058 retired the windows along with the capture that fed them — no
//! finding a run produces reaches the inbox now — and moved this row to
//! `dashboard`, because a count of the todos a person has logged has no
//! bearing on the spec a review or implementation run is working on.
//!
//! **A notice, never a gate.** §brownfield-inbox states the standing count
//! is a notice, never a gate: capture stays free, so gating `done` or a
//! release on inbox depth would make logging a todo expensive. The row changes what the operator knows at
//! the moment they decide, and withholds nothing.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use git2::Repository;

use crate::schema::paths;
use crate::schema::primitives::{InboxStanding, InboxState};

/// Resolve the standing inbox backlog at `{specs-root}/inbox.md`.
///
/// Never fails: every unknown is a *state*, because a row that silently
/// disappeared or reported a confident zero over a file it could not read
/// would be the `QUAL-CLAIM-001` conflation the row exists to remove from the
/// report's own surface.
pub(crate) fn standing(repo: &Path) -> InboxStanding {
    let specs_root = paths::Paths::load(repo).specs_root;
    let rel = format!("{specs_root}/inbox.md");
    let Ok(content) = std::fs::read_to_string(repo.join(&rel)) else {
        // No file, or one that is not readable UTF-8. Distinct from clean: a
        // project with no `inbox.md` has not been examined-and-found-empty,
        // and the two must not render alike.
        return InboxStanding {
            state: InboxState::NoFile,
            outstanding: 0,
            oldest: None,
            path: rel,
        };
    };

    // The shared comment- and fence-aware bullet grammar, not a second parser:
    // the inbox template embeds `- ` lines inside its `<!-- Rules: … -->`
    // guidance block, and counting those once reported ~30 phantom items.
    let bullets: Vec<usize> = super::iter_bullets(&content).map(|(idx, _)| idx).collect();
    if bullets.is_empty() {
        return InboxStanding {
            state: InboxState::Clean,
            outstanding: 0,
            oldest: None,
            path: rel,
        };
    }

    InboxStanding {
        outstanding: u32::try_from(bullets.len()).unwrap_or(u32::MAX),
        oldest: oldest_bullet_date(repo, &rel, &content, &bullets),
        state: InboxState::Outstanding,
        path: rel,
    }
}

/// The `YYYY-MM-DD` (UTC) date of the oldest surviving bullet, by `git blame`.
///
/// Age is what separates a working queue from a rotting one, and it requires
/// no authored state — nothing is added to the file and no author has to
/// remember anything, which is what keeps this clear of the diligence
/// dependency §design-principles rejects.
///
/// Blame is **content-based**, which is the reason it is used rather than the
/// file's own mtime or its first commit: `append-inbox` and `remove-inbox-item`
/// rewrite the whole file atomically on every call, so any whole-file signal
/// would reset a surviving line's date on the next unrelated capture.
///
/// Each working-tree bullet is dated **by its text** against the committed
/// file's blame, never by its line number. The bullets come from the working
/// tree and the blame from `HEAD`, so pairing them by position shifted every
/// bullet below an uncommitted removal onto the line above it — right after
/// `/{project}:groom`, the row reported the removed item's date. Blaming the
/// working-tree bytes as a buffer fixed that and broke two other cases: under
/// `core.autocrlf` the working file is CRLF and the blob LF, so every line
/// read as uncommitted, and libgit2's buffer blame misplaces a deletion that
/// follows an insertion (spec 058). A text match is immune to both: a trailing
/// `\r` is trimmed on each side, and a line whose text `HEAD` does not hold is
/// uncommitted and carries no date (it is newer than every committed line, so
/// it cannot be the oldest).
///
/// A text the working tree holds `k` times and `HEAD` holds `m` times keeps
/// the dates of the **last** `min(k, m)` committed occurrences. That follows
/// the two writers: `remove-inbox-item` removes the first matching bullet, so
/// after an uncommitted removal the survivors are the later occurrences, and
/// `append-inbox` appends after every bullet, so an uncommitted duplicate is
/// the one beyond the `m` committed ones, undated.
///
/// A **shallow clone** attributes every line older than its cut to the
/// boundary commit, whose date is then a confident answer for lines it did
/// not write. So when any surviving bullet maps to a committed line whose
/// date cannot be read — a boundary line in a shallow clone, or a commit that
/// cannot be looked up — the age is undeterminable, because that bullet may be
/// the oldest; reporting the oldest date after the cut instead would claim a
/// queue fresher than it is. In a full clone the boundary is the root commit
/// and its date is real. The blamed path is relative to the git work tree,
/// which is not the project root when the project lives in a subdirectory of
/// its repository.
///
/// `None` when it cannot be determined — no repository, a file not yet
/// committed, every bullet uncommitted, a surviving bullet behind a shallow
/// cut, a blame that fails for any other reason. That is reported as
/// *undeterminable* by the caller rather than silently dropped or defaulted to
/// today. **This reads git history**, so a CI job running it needs
/// `fetch-depth: 0` (§design-principles).
fn oldest_bullet_date(repo: &Path, rel: &str, content: &str, bullets: &[usize]) -> Option<String> {
    let repository = Repository::discover(repo).ok()?;
    let blamed = workdir_relative(&repository, &repo.join(rel))?;
    let committed = committed_text(&repository, &blamed)?;
    let blame = repository.blame_file(&blamed, None).ok()?;
    let shallow = repository.is_shallow();

    // `None` is a committed line whose date cannot be read.
    let mut line_dates: Vec<Option<i64>> = vec![None; committed.lines().count()];
    for hunk in blame.iter() {
        if shallow && hunk.is_boundary() {
            continue;
        }
        let date = hunk.final_signature().map_or_else(
            || {
                repository
                    .find_commit(hunk.final_commit_id())
                    .ok()
                    .map(|commit| commit.author().when().seconds())
            },
            |signature| Some(signature.when().seconds()),
        );
        // `final_start_line` is 1-based.
        let first = hunk.final_start_line().saturating_sub(1);
        for slot in line_dates.iter_mut().skip(first).take(hunk.lines_in_hunk()) {
            *slot = date;
        }
    }

    // Each committed text's dates, in file order.
    let mut by_text: HashMap<&str, Vec<Option<i64>>> = HashMap::new();
    for (line, date) in committed.lines().zip(line_dates) {
        by_text
            .entry(line.trim_end_matches('\r'))
            .or_default()
            .push(date);
    }
    let lines: Vec<&str> = content.lines().collect();
    let mut working: HashMap<&str, usize> = HashMap::new();
    for &idx in bullets {
        if let Some(line) = lines.get(idx) {
            *working.entry(line.trim_end_matches('\r')).or_default() += 1;
        }
    }

    let mut oldest: Option<i64> = None;
    for (text, count) in working {
        // A text `HEAD` does not hold is uncommitted: newer than every
        // committed line, so never the oldest.
        let Some(dates) = by_text.get(text) else {
            continue;
        };
        for date in &dates[dates.len() - count.min(dates.len())..] {
            // A surviving bullet whose date cannot be read may be the oldest.
            let date = (*date)?;
            oldest = Some(oldest.map_or(date, |current| current.min(date)));
        }
    }
    oldest.map(format_utc_date)
}

/// `path` relative to the repository's work tree — what `blame_file` and the
/// tree lookup take — or `None` when it lies outside it or cannot be resolved.
/// The parent is canonicalized rather than the file, so an inbox that is a
/// symlink is blamed under its own name.
fn workdir_relative(repository: &Repository, path: &Path) -> Option<PathBuf> {
    Some(super::workdir_prefix(repository, path.parent()?)?.join(path.file_name()?))
}

/// The file's text at `HEAD`, or `None` when `HEAD` does not hold it.
fn committed_text(repository: &Repository, path: &Path) -> Option<String> {
    let tree = repository.head().ok()?.peel_to_tree().ok()?;
    let blob = tree
        .get_path(path)
        .ok()?
        .to_object(repository)
        .ok()?
        .peel_to_blob()
        .ok()?;
    String::from_utf8(blob.content().to_vec()).ok()
}

/// Format a Unix timestamp as `YYYY-MM-DD` in UTC.
///
/// Hand-rolled rather than pulling a date crate for one format: the runtime
/// has no date dependency, and the civil-from-days algorithm is exact for
/// every timestamp this can see.
fn format_utc_date(seconds: i64) -> String {
    let days = seconds.div_euclid(86_400);
    // Howard Hinnant's `civil_from_days`, shifted to a March-based year so
    // the leap day lands at the end and the month-length series is uniform.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn seed_inbox(repo: &Path, content: &str) {
        fs::create_dir_all(repo.join("specs")).unwrap();
        fs::write(repo.join("specs/inbox.md"), content).unwrap();
    }

    #[test]
    fn an_absent_inbox_is_its_own_state_not_a_clean_one() {
        let tmp = tempdir().unwrap();
        let result = standing(tmp.path());
        assert_eq!(result.state, InboxState::NoFile);
        assert_eq!(result.outstanding, 0);
        assert_eq!(result.oldest, None);
    }

    #[test]
    fn an_inbox_holding_only_the_template_comment_is_clean() {
        let tmp = tempdir().unwrap();
        seed_inbox(
            tmp.path(),
            "# Inbox\n\n<!-- Rules:\n- do not frontfill\n- groom regularly\n-->\n",
        );
        let result = standing(tmp.path());
        assert_eq!(result.state, InboxState::Clean);
        assert_eq!(
            result.outstanding, 0,
            "the shared bullet grammar ignores comment regions"
        );
    }

    #[test]
    fn outstanding_items_are_counted() {
        let tmp = tempdir().unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- one\n- two\n- [ ] three\n");
        let result = standing(tmp.path());
        assert_eq!(result.state, InboxState::Outstanding);
        assert_eq!(result.outstanding, 3);
    }

    /// An uncommitted inbox has no blame to read. The count still renders and
    /// the age reports undeterminable — never defaulted to today, which would
    /// make a rotting queue look fresh.
    #[test]
    fn an_unblameable_inbox_still_counts_and_reports_no_age() {
        let tmp = tempdir().unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- one\n");
        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 1);
        assert_eq!(result.oldest, None);
    }

    /// Commit everything in `repo` with a fixed author time, so the blame the
    /// assertions read is deterministic.
    fn commit_all(repo: &Repository, message: &str, epoch_seconds: i64) {
        let mut index = repo.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let sig = git2::Signature::new(
            "Test",
            "test@example.com",
            &git2::Time::new(epoch_seconds, 0),
        )
        .unwrap();
        let parent = repo
            .head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| repo.find_commit(oid).ok());
        let parents: Vec<&git2::Commit> = parent.as_ref().into_iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .unwrap();
    }

    /// The age is what separates a working queue from a rotting one, and it
    /// has to survive the atomic whole-file rewrites `append-inbox` and
    /// `remove-inbox-item` perform on every capture — which is why blame is
    /// read per surviving line rather than any whole-file signal.
    #[test]
    fn the_oldest_date_survives_a_later_whole_file_rewrite() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();

        // 2025-05-19: the first item lands.
        seed_inbox(tmp.path(), "# Inbox\n\n- the old one\n");
        commit_all(&repo, "capture the first item", 1_747_612_800);

        // 2026-09-12: a later capture rewrites the whole file, as the atomic
        // append does. The surviving line keeps its own date.
        seed_inbox(tmp.path(), "# Inbox\n\n- the old one\n- a fresh one\n");
        commit_all(&repo, "capture a second item", 1_789_516_800);

        let result = standing(tmp.path());
        assert_eq!(result.state, InboxState::Outstanding);
        assert_eq!(result.outstanding, 2);
        assert_eq!(
            result.oldest.as_deref(),
            Some("2025-05-19"),
            "a whole-file rewrite must not reset a surviving line's age"
        );
    }

    /// The bullets come from the working tree and the blame from `HEAD`, so
    /// they are paired by text, never by line number. Pairing by position
    /// shifted every bullet below an uncommitted removal onto the line above
    /// it — right after `/groom` removes the oldest item, the row reported that
    /// removed item's date as the queue's oldest (spec 058).
    #[test]
    fn an_uncommitted_removal_does_not_shift_the_oldest_date() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- the old one\n");
        commit_all(&repo, "capture the first item", 1_747_612_800);
        seed_inbox(tmp.path(), "# Inbox\n\n- the old one\n- a fresh one\n");
        commit_all(&repo, "capture a second item", 1_789_516_800);

        // Groomed away, not yet committed.
        seed_inbox(tmp.path(), "# Inbox\n\n- a fresh one\n");
        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 1);
        assert_eq!(
            result.oldest,
            Some(format_utc_date(1_789_516_800)),
            "the surviving item's own date, not the removed item's"
        );
    }

    /// A bullet not yet committed has no date, and it is newer than every
    /// committed one, so it is skipped rather than read as the oldest. Paired
    /// by position, the committed `- y` sat at the uncommitted bullet's line
    /// and the older comment below it at `- y`'s, so the row reported the
    /// comment's date.
    #[test]
    fn an_uncommitted_bullet_takes_no_committed_line_s_date() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n<!-- end -->\n");
        commit_all(&repo, "the template", 1_560_000_000);
        seed_inbox(tmp.path(), "# Inbox\n\n- y\n<!-- end -->\n");
        commit_all(&repo, "capture y", 1_747_612_800);

        seed_inbox(
            tmp.path(),
            "# Inbox\n\n- logged just now\n- y\n<!-- end -->\n",
        );
        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 2);
        assert_eq!(result.oldest, Some(format_utc_date(1_747_612_800)));
    }

    /// libgit2's buffer blame misplaces a deletion that follows an insertion:
    /// a new line above `newer` with `old` removed dated the queue by the
    /// removed `old`. Matching by text never consults the buffer diff.
    #[test]
    fn an_insertion_above_a_removal_dates_the_surviving_item() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- old\n");
        commit_all(&repo, "capture old", 1_577_836_800);
        seed_inbox(tmp.path(), "# Inbox\n\n- newer\n- old\n");
        commit_all(&repo, "capture newer", 1_609_459_200);

        seed_inbox(tmp.path(), "# Inbox\n\n- just logged\n- newer\n");
        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 2);
        assert_eq!(result.oldest, Some(format_utc_date(1_609_459_200)));
    }

    /// Under `core.autocrlf` the working file is CRLF while the committed blob
    /// is LF. Blaming the working-tree bytes read every line as uncommitted,
    /// so a clean, fully committed inbox reported its age undeterminable.
    #[test]
    fn a_crlf_working_tree_over_an_lf_blob_is_dated() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        repo.config()
            .unwrap()
            .set_bool("core.autocrlf", true)
            .unwrap();
        seed_inbox(tmp.path(), "# Inbox\r\n\r\n- old item\r\n");
        commit_all(&repo, "capture", 1_577_836_800);
        let blob = committed_text(&repo, Path::new("specs/inbox.md")).unwrap();
        assert!(!blob.contains('\r'), "autocrlf must store the blob as LF");

        let result = standing(tmp.path());
        assert_eq!(result.oldest, Some(format_utc_date(1_577_836_800)));
    }

    /// A shallow clone attributes every line older than its cut to the
    /// boundary commit, so that commit's date would be reported with
    /// confidence for lines it did not write. It is undeterminable instead.
    #[test]
    fn a_shallow_clone_does_not_date_by_its_boundary_commit() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- old item\n");
        commit_all(&repo, "capture", 1_577_836_800);
        fs::write(tmp.path().join("other.txt"), "x").unwrap();
        commit_all(&repo, "unrelated", 1_709_164_800);
        let head = repo.head().unwrap().target().unwrap();
        fs::write(tmp.path().join(".git/shallow"), format!("{head}\n")).unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        assert!(repo.is_shallow());

        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 1);
        assert_eq!(result.oldest, None, "the cut hides the real date");
    }

    /// Mark `repo`'s current `HEAD` as the cut of a shallow clone, as
    /// `git clone --depth N` records it, and reopen it.
    fn cut_shallow_at_head(path: &Path, repo: &Repository) -> Repository {
        let cut = repo.head().unwrap().target().unwrap();
        fs::write(path.join(".git/shallow"), format!("{cut}\n")).unwrap();
        let repo = Repository::open(path).unwrap();
        assert!(repo.is_shallow());
        repo
    }

    /// `git clone --depth 2` over three commits — the old item (2020), an
    /// unrelated change (2021), a new item (2022) — keeps the new item's real
    /// date and cuts the old one's. Dropping the item behind the cut reported
    /// the new item's 2022 as the queue's oldest, a confident date for a queue
    /// holding something older; the age is undeterminable instead.
    #[test]
    fn one_bullet_behind_a_shallow_cut_makes_the_age_undeterminable() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- [ ] ancient\n");
        commit_all(&repo, "capture ancient", 1_577_836_800);
        fs::write(tmp.path().join("other.txt"), "x").unwrap();
        commit_all(&repo, "unrelated", 1_609_459_200);
        let repo = cut_shallow_at_head(tmp.path(), &repo);
        seed_inbox(tmp.path(), "# Inbox\n\n- [ ] ancient\n- [ ] recent\n");
        commit_all(&repo, "capture recent", 1_640_995_200);

        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 2);
        assert_eq!(
            result.oldest, None,
            "a surviving bullet behind the cut may be the oldest"
        );
    }

    /// Only a *surviving bullet* behind the cut hides the age. The heading
    /// lines behind it are not items, and a bullet not yet committed is newer
    /// than every committed one, so neither makes the age undeterminable.
    #[test]
    fn a_shallow_clone_dates_items_written_after_its_cut() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n");
        commit_all(&repo, "the template", 1_577_836_800);
        fs::write(tmp.path().join("other.txt"), "x").unwrap();
        commit_all(&repo, "unrelated", 1_609_459_200);
        let repo = cut_shallow_at_head(tmp.path(), &repo);
        seed_inbox(tmp.path(), "# Inbox\n\n- [ ] recent\n");
        commit_all(&repo, "capture recent", 1_640_995_200);
        seed_inbox(
            tmp.path(),
            "# Inbox\n\n- [ ] recent\n- [ ] logged just now\n",
        );

        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 2);
        assert_eq!(result.oldest, Some(format_utc_date(1_640_995_200)));
    }

    /// `remove-inbox-item` removes the **first** of two identical bullets, so
    /// the one left is the later capture. Pairing repeated texts front-first
    /// gave it the removed bullet's older date.
    #[test]
    fn removing_one_of_two_identical_bullets_leaves_the_survivor_its_own_date() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- [ ] flaky test\n");
        commit_all(&repo, "capture it", 1_577_836_800);
        seed_inbox(
            tmp.path(),
            "# Inbox\n\n- [ ] flaky test\n- [ ] flaky test\n",
        );
        commit_all(&repo, "capture it again", 1_640_995_200);

        let removed = crate::primitives::remove_inbox_item::run(
            &crate::schema::primitives::RemoveInboxItemArgs {
                item: "flaky test".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(removed.removed);

        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 1);
        assert_eq!(
            result.oldest,
            Some(format_utc_date(1_640_995_200)),
            "the survivor's own date, not the removed duplicate's"
        );
    }

    /// `append-inbox` appends after every existing bullet, so a duplicate not
    /// yet committed is the last occurrence: undated, while the committed one
    /// keeps its date.
    #[test]
    fn a_duplicate_appended_uncommitted_leaves_the_committed_date() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        seed_inbox(tmp.path(), "# Inbox\n\n- [ ] flaky test\n");
        commit_all(&repo, "capture it", 1_577_836_800);

        crate::primitives::append_inbox::run(
            &crate::schema::primitives::AppendInboxArgs {
                text: "flaky test".into(),
            },
            tmp.path(),
        )
        .unwrap();

        let result = standing(tmp.path());
        assert_eq!(result.outstanding, 2);
        assert_eq!(result.oldest, Some(format_utc_date(1_577_836_800)));
    }

    /// A project living in a subdirectory of its repository is blamed by its
    /// path from the work tree, not from the project root.
    #[test]
    fn a_project_in_a_repository_subdirectory_is_dated() {
        let tmp = tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let project = tmp.path().join("service");
        seed_inbox(&project, "# Inbox\n\n- old item\n");
        commit_all(&repo, "capture", 1_577_836_800);

        let result = standing(&project);
        assert_eq!(result.oldest, Some(format_utc_date(1_577_836_800)));
    }

    #[test]
    fn the_civil_date_conversion_matches_known_timestamps() {
        assert_eq!(format_utc_date(0), "1970-01-01");
        // A leap day, the case the March-based shift exists to get right.
        assert_eq!(format_utc_date(1_709_164_800), "2024-02-29");
        assert_eq!(format_utc_date(1_747_612_800), "2025-05-19");
    }
}
