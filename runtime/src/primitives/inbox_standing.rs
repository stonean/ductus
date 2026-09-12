//! The **standing** inbox backlog — how deep the queue is and how old its
//! oldest item is — shared by `write-review` and `diff-cross-spec`.
//!
//! Every surface that showed the inbox before this was **window-scoped**.
//! `/{project}:review`'s **Captured issues** section lists bullets added since
//! its diff base; `/{project}:implement`'s completion summary lists
//! `diff-cross-spec`'s `inbox-additions`, computed from the feature's first
//! commit. Both answer *"what was captured while this feature was open"*.
//! Neither answers *"what is outstanding"*, so an item older than the current
//! feature was invisible by construction — six stood in the inbox when
//! `ductus-v0.47.0` was cut, and they appeared in that feature's reports only
//! because all six happened to land inside its window.
//!
//! The window keeps its own role: it is what ties a finding to the work that
//! produced it. This is the other number, and neither stands in for the other
//! (spec 022, scenario `the-inbox-row`).
//!
//! **A notice, never a gate.** §brownfield-inbox's whole design rests on
//! capture being free — *"the honest choice between a growing backlog and a
//! silent one would push toward silence"* — so gating `done` or a release on
//! inbox depth would make capture expensive. The row changes what the operator
//! knows at the moment they decide, and withholds nothing.

use std::path::Path;

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
        oldest: oldest_bullet_date(repo, &rel, &bullets),
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
/// `None` when it cannot be determined — a shallow clone with no history to
/// blame, a file not yet committed, a blame that fails for any other reason.
/// That is reported as *undeterminable* by the caller rather than silently
/// dropped or defaulted to today. **This reads git history**, so a CI job
/// running it needs `fetch-depth: 0` (§design-principles).
fn oldest_bullet_date(repo: &Path, rel: &str, bullets: &[usize]) -> Option<String> {
    let repository = Repository::discover(repo).ok()?;
    let blame = repository.blame_file(Path::new(rel), None).ok()?;
    bullets
        .iter()
        // `iter_bullets` yields 0-based line indexes; blame is 1-based.
        .filter_map(|idx| blame.get_line(idx + 1))
        .filter_map(|hunk| hunk.final_signature().map(|sig| sig.when().seconds()))
        .min()
        .map(format_utc_date)
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

    #[test]
    fn the_civil_date_conversion_matches_known_timestamps() {
        assert_eq!(format_utc_date(0), "1970-01-01");
        // A leap day, the case the March-based shift exists to get right.
        assert_eq!(format_utc_date(1_709_164_800), "2024-02-29");
        assert_eq!(format_utc_date(1_747_612_800), "2025-05-19");
    }
}
