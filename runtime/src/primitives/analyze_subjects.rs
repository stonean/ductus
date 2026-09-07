//! The analyze record's subject set, its digest, and the freshness comparison
//! every surface reads.
//!
//! **One reference point, in one place.** `/{project}:analyze` reads a spec's
//! artifacts from disk, so what it examined is a *tree state*, not a commit.
//! The first implementation of this check compared `analyze.analyzed-against`
//! — a commit sha — against `HEAD`, which silently redefined the record's
//! subject from "what the analysis read" to "what happened to be committed at
//! the time". Those coincide only when the working tree is clean, and at the
//! moment analyze runs it usually is not: `/{project}:review` has just written
//! `review.md` and the spec's `review:` block, and `mark-task` and
//! `mark-criterion` rewrote `tasks.md` and `spec.md` minutes before that. The
//! consequence was a gate that blocked records it should have passed — the
//! paths the analysis *had* read came back as changes on the next commit — and
//! a gate with false positives is the one people route around, which is the
//! failure `check_review_gate`'s own history records twice.
//!
//! So staleness is a **content** comparison: the record carries a per-path
//! digest of the subject set as the run read it, and freshness is a digest
//! match. `analyzed-against` survives as provenance and is read for exactly
//! one thing — the mechanical-sweep rename exemption, which genuinely needs
//! two trees.
//!
//! The second thing this buys is that the two surfaces no longer need
//! different reference points. A commit sha cannot describe a working tree, so
//! the gate had to read commits while `/{project}:review`'s row read the tree,
//! and one implementation could still return two answers. A digest answers the
//! same way wherever it is asked (spec 047, `analyze-record-freshness`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::primitives::{read_text, split_frontmatter, write_review};
use crate::schema::primitives::{AnalyzeBlock, RecordFreshness};

/// Any `.md` artifact under the feature — the analyze record's subject set.
///
/// Deliberately wider than `check_review_gate`'s durable contracts, and the
/// two must not be merged: a review reads *code*, so `review.md` is its output
/// rather than its input, while an analysis reads *artifacts*, and `review.md`
/// plus the `review:` block are among the ones its families assert on.
/// Excluding `review.md` would exempt the single edit that most often
/// invalidates an analyze record.
pub(crate) fn is_analyze_subject(rel_within_feature: &str) -> bool {
    Path::new(rel_within_feature)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// A scenario or the data model — the artifacts a **review** reads, and the
/// subject set of the `review:` record.
///
/// Deliberately narrower than [`is_analyze_subject`], and the two must not be
/// merged. A review reads *code*, so `review.md` and `spec.md` are its outputs
/// rather than its inputs — `write-review` touches both, and counting them
/// would stale every review the instant it was recorded. An analysis reads
/// *artifacts*, so those same files are among its subjects. Same mechanism,
/// different claims.
///
/// Mirrors `scripts/audit/review-freshness.sh`'s rule exactly.
pub(crate) fn is_review_contract(rel_within_feature: &str) -> bool {
    let is_md = Path::new(rel_within_feature)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
    (rel_within_feature.starts_with("scenarios/") && is_md) || rel_within_feature == "data-model.md"
}

/// The subject set's digest, plus the subjects that could not be read.
///
/// Paths are relative to the feature directory, so the map is short and stays
/// meaningful if the spec is ever moved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SubjectDigest {
    /// path-within-feature → lowercase-hex sha256 of the digested content.
    pub digests: BTreeMap<String, String>,
    /// Subjects that exist but could not be read, recorded rather than
    /// digested as empty — an unreadable subject is not a matching one.
    pub unreadable: Vec<String>,
}

/// Digest every analyze subject under `feature_dir`.
///
/// `spec.md` is digested with its own `analyze:` frontmatter block **excised**.
/// That exclusion is load-bearing rather than tidy: the record is written after
/// the subjects are read, so a digest covering the block could never match on
/// the next comparison and every run would stale itself. Measured on the
/// sha-diff design the same exclusion was what took the flagged population
/// from 54 of 54 specs down to 1.
///
/// Both failure modes are reported, and they are one line apart: a subject the
/// walk could not **reach** and one it opened and could not **read** both land
/// in `unreadable`. Only reporting the second would let an unreachable
/// subdirectory shrink the subject set silently, so a digest taken after it
/// became unreadable would compare clean against one taken before.
pub(crate) fn subject_digest(feature_dir: &Path, is_subject: fn(&str) -> bool) -> SubjectDigest {
    let mut out = SubjectDigest::default();
    for entry in walkdir::WalkDir::new(feature_dir).follow_links(false) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                // A subject the walk could not *reach* is recorded, not
                // dropped. Discarding it would shrink the subject set with no
                // trace — a permission-denied `scenarios/` would digest clean
                // against a record taken when its files were readable, which
                // is the same conflation the `unreadable` field exists to
                // prevent one line down. The path may be a directory rather
                // than a subject; it is recorded anyway, because what is
                // unreachable is everything beneath it.
                let path = error
                    .path()
                    .and_then(|path| path.strip_prefix(feature_dir).ok())
                    .map(|path| path.to_string_lossy().replace('\\', "/"))
                    .filter(|path| !path.is_empty())
                    .unwrap_or_else(|| ".".to_string());
                out.unreadable.push(path);
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(rel) = entry.path().strip_prefix(feature_dir) else {
            continue;
        };
        let rel = rel.to_string_lossy().replace('\\', "/");
        if !is_subject(&rel) {
            continue;
        }
        match read_text(entry.path()) {
            Ok(text) => {
                let digested = if rel == "spec.md" {
                    strip_analyze_block(&text, entry.path()).unwrap_or(text)
                } else {
                    text
                };
                let mut hasher = Sha256::new();
                hasher.update(digested.as_bytes());
                out.digests.insert(rel, hex(&hasher.finalize()));
            }
            Err(_) => out.unreadable.push(rel),
        }
    }
    out.unreadable.sort();
    out
}

/// `spec.md` with its `analyze:` frontmatter block removed, or `None` when the
/// frontmatter will not split (in which case the caller digests the file whole
/// — a spec whose frontmatter is unparseable is a real difference, not one to
/// normalize away).
///
/// Reuses [`write_review::splice_top_level_block`] with an empty replacement
/// rather than re-deriving "find the top-level key and its extent". Sharing
/// that is deliberate for the reason the splice was generalized in the first
/// place: two implementations of the same region logic agree until one meets a
/// frontmatter shape the other has not, and here disagreement would mean a
/// record that can never match.
fn strip_analyze_block(text: &str, path: &Path) -> Option<String> {
    let (fm_text, body) = split_frontmatter(text, path).ok()?;
    Some(format!(
        "{}\n---\n{body}",
        write_review::splice_top_level_block(fm_text, "analyze", "")
    ))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut acc, byte| {
        use std::fmt::Write as _;
        let _ = write!(acc, "{byte:02x}");
        acc
    })
}

/// Compare a spec's recorded analyze digest against its subjects as they are
/// now.
///
/// The single implementation behind both surfaces (spec 047 AC14) — the
/// completion gate and `/{project}:review`'s `analyze` row both call this, and
/// because the comparison is of content rather than commits there is no
/// reference point left for them to differ on.
///
/// Ordering of the arms matters. A record with no digest is
/// [`RecordFreshness::Undeterminable`], never `Current`: nothing on disk says
/// what those runs examined, and every record written before this change is in
/// that state. It does not block, so the gate stops *enforcing* freshness for
/// a spec until its next analyze writes a digest — self-healing, and
/// deliberately not a grandfather clause. The record is not exempt, it is
/// unreadable, and the result says which.
pub(crate) fn freshness_of(
    repo: &Path,
    rel_dir: &str,
    last_run: Option<&str>,
    recorded_against: Option<&str>,
    recorded: Option<&BTreeMap<String, String>>,
    is_subject: fn(&str) -> bool,
    digest_field: &str,
) -> RecordFreshness {
    let Some(last_run) = last_run.map(str::to_string) else {
        return RecordFreshness::NeverRun;
    };
    let analyzed_against = recorded_against.unwrap_or_default().to_string();
    // `None` is a record that never took a digest. An empty map is a digest
    // that was taken over a subject set with nothing in it — a spec with no
    // scenarios and no data model — which is current, not unjudgeable.
    let Some(recorded) = recorded else {
        return RecordFreshness::Undeterminable {
            reason: format!("the record carries no {digest_field}, so what it examined is unknown"),
        };
    };

    let current = subject_digest(&repo.join(rel_dir), is_subject);
    let mut changed: BTreeSet<String> = BTreeSet::new();
    for (path, digest) in &current.digests {
        if recorded.get(path) != Some(digest) {
            changed.insert(format!("{rel_dir}/{path}"));
        }
    }
    // A subject the record covered that is now gone is a change too — a
    // deleted scenario alters what an analysis would find.
    for path in recorded.keys() {
        if !current.digests.contains_key(path) {
            changed.insert(format!("{rel_dir}/{path}"));
        }
    }
    // An unreadable subject is not a matching one. It cannot be compared, so
    // it is reported as changed rather than passed over — the safe direction,
    // and re-running a read-only analyze costs nothing.
    for path in &current.unreadable {
        changed.insert(format!("{rel_dir}/{path}"));
    }

    let changed = exempt_renames(repo, &analyzed_against, changed);
    if changed.is_empty() {
        return RecordFreshness::Current {
            last_run,
            analyzed_against,
        };
    }
    RecordFreshness::Stale {
        last_run,
        analyzed_against,
        paths: changed.into_iter().collect(),
    }
}

/// Drop the candidates whose change was a uniform repo-wide rename.
///
/// The one place `analyzed-against` is still read. A contract that changed only
/// in spelling states what it stated before, so a rename sweep must not stale
/// the record — the §spec-lifecycle case (a) rule, and the same exemption
/// `stale_review_block` applies. Measured on the review half before it had the
/// exemption, the un-exempted rule called 19 of 46 `done` specs stale, every
/// one a consequence of 049's `govern → ductus` sweep and none a real change.
///
/// When the trees are unavailable — no repo, an unresolvable `analyzed-against`
/// from a rebase or shallow clone, an uncommitted change with no committed
/// counterpart — the exemption cannot run and every candidate is **kept**.
/// Reporting a rename as stale over-reports; dropping a real change under
/// cover of an exemption that never ran would not.
fn exempt_renames(
    repo: &Path,
    analyzed_against: &str,
    candidates: BTreeSet<String>,
) -> BTreeSet<String> {
    if candidates.is_empty() || analyzed_against.trim().is_empty() {
        return candidates;
    }
    let Ok(repository) = git2::Repository::discover(repo) else {
        return candidates;
    };
    let base_tree = repository
        .revparse_single(analyzed_against.trim())
        .and_then(|object| object.peel_to_commit())
        .and_then(|commit| commit.tree());
    let head_tree = repository
        .head()
        .and_then(|head| head.peel_to_commit())
        .and_then(|commit| commit.tree());
    let (Ok(base_tree), Ok(head_tree)) = (base_tree, head_tree) else {
        return candidates;
    };
    let index =
        crate::primitives::mechanical_sweep::SweepIndex::build(&repository, &base_tree, &head_tree);
    candidates
        .into_iter()
        .filter(|path| index.changed_beyond_spelling(path))
        .collect()
}

/// The freshness of a spec's `analyze:` record.
pub(crate) fn analyze_freshness(
    repo: &Path,
    rel_dir: &str,
    analyze: Option<&AnalyzeBlock>,
) -> RecordFreshness {
    let Some(analyze) = analyze else {
        return RecordFreshness::NeverRun;
    };
    freshness_of(
        repo,
        rel_dir,
        analyze.last_run.as_deref(),
        analyze.analyzed_against.as_deref(),
        // Empty is treated as absent here: `spec.md` is always a subject, so
        // an empty analyze digest can only mean the record predates the field.
        Some(&analyze.analyzed_digest).filter(|d| !d.is_empty()),
        is_analyze_subject,
        "analyzed-digest",
    )
}

/// The freshness of a spec's `review:` record.
///
/// The same call as [`analyze_freshness`] over a narrower subject set, which is
/// the whole of the difference between the two records' freshness. It replaced
/// a commit-range diff for the reason that one is documented at the top of this
/// module: `/{project}:review` reads the working tree and recorded a *commit*,
/// so a scenario written in-session came back as a durable contract that had
/// changed since the review, when it had changed only since the commit the
/// review was labelled with.
pub(crate) fn review_freshness(
    repo: &Path,
    rel_dir: &str,
    review: Option<&crate::schema::primitives::ReviewBlock>,
) -> RecordFreshness {
    let Some(review) = review else {
        return RecordFreshness::NeverRun;
    };
    freshness_of(
        repo,
        rel_dir,
        review.last_run.as_deref(),
        review.reviewed_against.as_deref(),
        review.reviewed_digest.as_ref(),
        is_review_contract,
        "reviewed-digest",
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn seed(dir: &Path, analyze_block: &str) {
        fs::create_dir_all(dir.join("scenarios")).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\nstatus: in-progress\ndependencies: []\n{analyze_block}---\n\n# Spec\n"),
        )
        .unwrap();
        fs::write(dir.join("tasks.md"), "# Tasks\n\n- [ ] work\n").unwrap();
        fs::write(dir.join("scenarios/a.md"), "# A\n").unwrap();
        fs::write(dir.join("notes.txt"), "not a subject\n").unwrap();
    }

    #[test]
    fn only_markdown_under_the_feature_is_a_subject() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), "");
        let digest = subject_digest(tmp.path(), is_analyze_subject);
        let paths: Vec<&str> = digest.digests.keys().map(String::as_str).collect();
        assert_eq!(paths, vec!["scenarios/a.md", "spec.md", "tasks.md"]);
    }

    /// The exclusion the whole comparison rests on: the record is written after
    /// the subjects are read, so a digest covering the `analyze:` block could
    /// never match and every run would stale itself.
    #[test]
    fn the_analyze_block_does_not_change_the_spec_digest() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), "");
        let before = subject_digest(tmp.path(), is_analyze_subject);

        let block = "analyze:\n  last-run: 2026-09-07T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 0\n  unexamined: 0\n  blocking: false\n";
        seed(tmp.path(), block);
        let after = subject_digest(tmp.path(), is_analyze_subject);

        assert_eq!(before.digests["spec.md"], after.digests["spec.md"]);
    }

    #[test]
    fn a_body_edit_does_change_the_spec_digest() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), "");
        let before = subject_digest(tmp.path(), is_analyze_subject);
        fs::write(
            tmp.path().join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# Spec\n\nNew scope.\n",
        )
        .unwrap();
        let after = subject_digest(tmp.path(), is_analyze_subject);
        assert_ne!(before.digests["spec.md"], after.digests["spec.md"]);
    }

    fn block_with(digests: &[(&str, &str)]) -> AnalyzeBlock {
        AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_against: None,
            analyzed_digest: digests
                .iter()
                .map(|(p, d)| ((*p).to_string(), (*d).to_string()))
                .collect(),
            ..AnalyzeBlock::default()
        }
    }

    #[test]
    fn a_matching_digest_is_current() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let recorded = subject_digest(&dir, is_analyze_subject);
        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_digest: recorded.digests,
            ..AnalyzeBlock::default()
        };
        let result = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));
        assert!(
            matches!(result, RecordFreshness::Current { .. }),
            "{result:?}"
        );
    }

    #[test]
    fn a_changed_subject_is_stale_and_named() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let recorded = subject_digest(&dir, is_analyze_subject);
        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_digest: recorded.digests,
            ..AnalyzeBlock::default()
        };
        fs::write(dir.join("tasks.md"), "# Tasks\n\n- [x] work\n").unwrap();

        let RecordFreshness::Stale { paths, .. } =
            analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze))
        else {
            panic!("a changed subject must be stale");
        };
        assert_eq!(paths, vec!["specs/001-x/tasks.md".to_string()]);
    }

    /// A deleted scenario changes what an analysis would find, so a subject the
    /// record covered and that is now gone counts.
    #[test]
    fn a_deleted_subject_is_stale() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let recorded = subject_digest(&dir, is_analyze_subject);
        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_digest: recorded.digests,
            ..AnalyzeBlock::default()
        };
        fs::remove_file(dir.join("scenarios/a.md")).unwrap();

        let RecordFreshness::Stale { paths, .. } =
            analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze))
        else {
            panic!("a deleted subject must be stale");
        };
        assert!(
            paths.iter().any(|p| p.ends_with("scenarios/a.md")),
            "{paths:?}"
        );
    }

    /// Every record written before this change. Not current, not stale — the
    /// gate stops enforcing freshness until the next analyze writes a digest.
    #[test]
    fn a_record_with_no_digest_is_undeterminable() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_against: Some("abc123".into()),
            ..AnalyzeBlock::default()
        };
        let result = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));
        let RecordFreshness::Undeterminable { reason } = result else {
            panic!("a digest-less record cannot be judged: {result:?}");
        };
        assert!(reason.contains("no analyzed-digest"), "{reason}");
    }

    #[test]
    fn an_absent_block_is_never_analyzed() {
        let tmp = tempdir().unwrap();
        assert_eq!(
            analyze_freshness(tmp.path(), "specs/001-x", None),
            RecordFreshness::NeverRun
        );
        let no_run = block_with(&[("spec.md", "deadbeef")]);
        let no_run = AnalyzeBlock {
            last_run: None,
            ..no_run
        };
        assert_eq!(
            analyze_freshness(tmp.path(), "specs/001-x", Some(&no_run)),
            RecordFreshness::NeverRun
        );
    }

    /// The false-positive path the sha diff had, as a test. An analysis that
    /// read a dirty tree and recorded what it read must not stale when that
    /// same content is committed — the content never changed, only its commit
    /// status did, and the digest does not look at commits.
    #[test]
    fn committing_content_the_analysis_already_read_is_not_stale() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let repository = git2::Repository::init(tmp.path()).unwrap();

        // The analysis reads the tree — including an uncommitted edit — and
        // records exactly that.
        fs::write(dir.join("tasks.md"), "# Tasks\n\n- [x] work\n").unwrap();
        let recorded = subject_digest(&dir, is_analyze_subject);

        // Now commit it. Under the sha diff this became `analyze-stale`.
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let sha = repository
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                "commit the analyzed content",
                &tree,
                &[],
            )
            .unwrap()
            .to_string();

        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_against: Some(sha),
            analyzed_digest: recorded.digests,
            ..AnalyzeBlock::default()
        };
        let result = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));
        assert!(
            matches!(result, RecordFreshness::Current { .. }),
            "committing what the analysis read must not stale it: {result:?}"
        );
    }

    /// The same answer before and after a commit — what collapsing the two
    /// reference points into one buys.
    #[test]
    fn the_answer_does_not_depend_on_commit_status() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        git2::Repository::init(tmp.path()).unwrap();
        let recorded = subject_digest(&dir, is_analyze_subject);
        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_digest: recorded.digests,
            ..AnalyzeBlock::default()
        };

        // Uncommitted change: stale.
        fs::write(dir.join("scenarios/a.md"), "# A\n\nChanged.\n").unwrap();
        let before = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));

        // Commit it: still stale, and the same paths.
        let repository = git2::Repository::open(tmp.path()).unwrap();
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        repository
            .commit(Some("HEAD"), &sig, &sig, "commit the change", &tree, &[])
            .unwrap();
        let after = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));

        assert_eq!(
            before, after,
            "the comparison must not depend on commit status"
        );
    }

    /// The reach failure, not the read failure — the one that used to be
    /// dropped by `filter_map(Result::ok)`.
    #[test]
    fn a_subject_the_walk_cannot_reach_is_recorded() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        // Make `scenarios/` untraversable so the walk errors on it rather
        // than on any single file.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let scenarios = dir.join("scenarios");
            fs::set_permissions(&scenarios, fs::Permissions::from_mode(0o000)).unwrap();
            let digest = subject_digest(&dir, is_analyze_subject);
            // Restore before asserting, so a failure cannot leave the tempdir
            // undeletable.
            fs::set_permissions(&scenarios, fs::Permissions::from_mode(0o755)).unwrap();
            assert!(
                !digest.unreadable.is_empty(),
                "an unreachable subdirectory must not shrink the subject set silently: {digest:?}"
            );
            assert!(
                !digest.digests.contains_key("scenarios/a.md"),
                "the unreachable file is not digested: {digest:?}"
            );
        }
    }

    // --- the review's narrower subject set ----------------------------------

    #[test]
    fn the_review_contract_set_is_scenarios_and_the_data_model() {
        assert!(is_review_contract("scenarios/a.md"));
        assert!(is_review_contract("data-model.md"));
        // A review's own outputs are not its inputs: counting them would stale
        // every review the instant it was recorded.
        assert!(!is_review_contract("review.md"));
        assert!(!is_review_contract("spec.md"));
        // Ephemeral by construction, and churning — the scoping that keeps
        // this gate off the path people route around.
        assert!(!is_review_contract("tasks.md"));
        assert!(!is_review_contract("plan.md"));
        // All four of those *are* analyze subjects. Same mechanism, different
        // claims.
        for path in ["review.md", "spec.md", "tasks.md", "plan.md"] {
            assert!(is_analyze_subject(path), "{path}");
        }
    }

    fn review_block(digest: Option<&SubjectDigest>) -> crate::schema::primitives::ReviewBlock {
        crate::schema::primitives::ReviewBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            reviewed_digest: digest.map(|d| d.digests.clone()),
            ..crate::schema::primitives::ReviewBlock::default()
        }
    }

    #[test]
    fn a_matching_contract_digest_is_current() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let recorded = subject_digest(&dir, is_review_contract);
        let result = review_freshness(
            tmp.path(),
            "specs/001-x",
            Some(&review_block(Some(&recorded))),
        );
        assert!(
            matches!(result, RecordFreshness::Current { .. }),
            "{result:?}"
        );
    }

    /// Editing `review.md` or `spec.md` stales the *analyze* record and not
    /// the review's — the asymmetry stated as a test, since it is the one
    /// thing a reader is most likely to take for an omission.
    #[test]
    fn a_review_output_stales_the_analysis_and_not_the_review() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let contracts = subject_digest(&dir, is_review_contract);
        let subjects = subject_digest(&dir, is_analyze_subject);
        fs::write(dir.join("review.md"), "# Review\n\nNo findings.\n").unwrap();

        let review = review_freshness(
            tmp.path(),
            "specs/001-x",
            Some(&review_block(Some(&contracts))),
        );
        assert!(
            matches!(review, RecordFreshness::Current { .. }),
            "{review:?}"
        );

        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_digest: subjects.digests,
            ..AnalyzeBlock::default()
        };
        let analyze = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));
        assert!(
            matches!(analyze, RecordFreshness::Stale { .. }),
            "{analyze:?}"
        );
    }

    /// A spec with no scenarios and no data model records an **empty** digest,
    /// which is current. Absent is unjudgeable; empty is examined-and-clean,
    /// and a bare map cannot tell them apart.
    #[test]
    fn an_empty_contract_digest_is_current_but_an_absent_one_is_not() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# S\n",
        )
        .unwrap();

        let empty = subject_digest(&dir, is_review_contract);
        assert!(empty.digests.is_empty());
        let current =
            review_freshness(tmp.path(), "specs/001-x", Some(&review_block(Some(&empty))));
        assert!(
            matches!(current, RecordFreshness::Current { .. }),
            "{current:?}"
        );

        let absent = review_freshness(tmp.path(), "specs/001-x", Some(&review_block(None)));
        let RecordFreshness::Undeterminable { reason } = absent else {
            panic!("a record that took no digest cannot be judged: {absent:?}");
        };
        assert!(reason.contains("no reviewed-digest"), "{reason}");
    }

    #[test]
    fn an_unreadable_subject_is_reported_not_matched() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let recorded = subject_digest(&dir, is_analyze_subject);
        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-07T00:00:00Z".into()),
            analyzed_digest: recorded.digests,
            ..AnalyzeBlock::default()
        };
        // Invalid UTF-8 makes the subject unreadable rather than absent.
        fs::write(dir.join("scenarios/a.md"), [0xff, 0xfe, 0x00]).unwrap();

        let result = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));
        let RecordFreshness::Stale { paths, .. } = result else {
            panic!("an unreadable subject is not a matching one: {result:?}");
        };
        assert!(
            paths.iter().any(|p| p.ends_with("scenarios/a.md")),
            "{paths:?}"
        );
    }
}
