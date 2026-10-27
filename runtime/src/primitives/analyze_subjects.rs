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

use crate::primitives::{hex, read_text};
use crate::schema::primitives::{AnalyzeBlock, RecordFreshness};

/// Any `.md` artifact under the feature except the analyze record itself —
/// the analyze record's subject set.
///
/// Deliberately wider than `check_review_gate`'s durable contracts, and the
/// two must not be merged: a review reads *code*, so `review.md` is its output
/// rather than its input, while an analysis reads *artifacts*, and `review.md`
/// is among the ones its families assert on. Excluding `review.md` would
/// exempt the single edit that most often invalidates an analyze record.
///
/// ## `analysis.md` is excluded outright, and excising its frontmatter was not
/// enough
///
/// `/{project}:analyze` writes this file, so it is the command's **output**,
/// exactly as `review.md` is `/{project}:review`'s and is excluded from
/// [`is_review_contract`] for the identical stated reason. Nothing reads its
/// body: its readers — the pre-done gate, `write-review`'s analyze-freshness
/// row, `read-spec`, `validate-frontmatter`, `process-decisions`,
/// `write-analysis` and `prune-plan` reading its stored decisions, and
/// `/{project}:analyze` judging drift from the record it has just written —
/// read its frontmatter.
///
/// Spec 057 first excluded only that frontmatter, which is the half the record
/// moved with. That left the **body** digested, and `write-analysis` rewrites
/// the body in the same call — so any run whose Summary or counts differed from
/// the previous one digested the pre-write body, wrote a different one, and
/// reported itself stale. Measured on `020-code-review` on 2026-09-16: the
/// first run left the gate answering `blocked: analysis is stale — 1
/// artifact(s) changed since it ran: specs/020-code-review/analysis.md`, and a
/// second run with identical arguments converged. Two runs to record one
/// analysis is a false block, and the plan's own note says a gate with false
/// positives is the one people route around.
///
/// The whole-file exclusion makes 057's AC7 unconditionally true rather than
/// true-when-the-body-happens-not-to-change.
pub(crate) fn is_analyze_subject(rel_within_feature: &str) -> bool {
    if rel_within_feature == crate::primitives::ANALYSIS_RECORD_FILE {
        return false;
    }
    Path::new(rel_within_feature)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// An analyze subject a `done` spec's record is still judged on: every one but
/// `plan.md` and `tasks.md`.
///
/// An edit confined to those two takes no back-edge on a `done` spec
/// (§spec-lifecycle), so it must not stale the record either: a stale
/// analysis on a spec nobody may reopen asks for a re-run that re-verifies
/// nothing, as the review record's narrower set already concluded for every
/// non-contract artifact. A spec short of `done` keeps both, because the
/// pre-`done` gate reads its analysis against them (spec 041,
/// `a-plan-edit-does-not-reopen-a-done-spec`).
pub(crate) fn is_done_analyze_subject(rel_within_feature: &str) -> bool {
    is_analyze_subject(rel_within_feature)
        && rel_within_feature != "plan.md"
        && rel_within_feature != "tasks.md"
}

/// A scenario or the data model — the artifacts a **review** reads, and the
/// subject set of the review record.
///
/// Deliberately narrower than [`is_analyze_subject`], and the two must not be
/// merged. A review reads *code*, so `review.md` is its output rather than its
/// input — counting it would stale every review the instant it was recorded.
/// An analysis reads *artifacts*, so that same file is among its subjects. Same
/// mechanism, different claims.
///
/// ## `spec.md`'s exclusion now rests on nothing, and is kept anyway
///
/// `spec.md` was excluded for the same stated reason as `review.md`:
/// `write-review` touched both, so digesting either would have staled the
/// record it was writing. Spec 057 moved the record to `review.md`, and
/// `write-review` no longer touches `spec.md` at all — so for that half the
/// reason has lapsed, and the exclusion is now a bare fact rather than a
/// consequence.
///
/// It is left in place deliberately. Adding `spec.md` to this set would make
/// every recorded review stale on any spec-body edit, which is a change to what
/// the gate *means* — defensible on its own merits, and not something to slip
/// into a relocation whose whole claim (AC4) is that the gate's verdicts are
/// unchanged. Whoever revisits it should argue it as its own change, and should
/// know that this comment, not a stale rationale, is what they are overturning.
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
/// `analysis.md` is not digested at all: [`is_analyze_subject`] excludes the
/// whole file, and every other subject is digested whole. That exclusion is
/// load-bearing rather than tidy: the record is written after the subjects are
/// read, so a digest covering it could never match on the next comparison and
/// every run would stale itself. Measured on the sha-diff design the same
/// exclusion was what took the flagged population from 54 of 54 specs down
/// to 1.
///
/// It began as an excision of `spec.md`'s `analyze:` block and moved with the
/// record (spec 057), first as an excision of `analysis.md`'s frontmatter —
/// which left the body, rewritten by the same call, inside the digest — and
/// then as the whole-file exclusion. `spec.md` is now digested whole, which
/// means a frontmatter edit there moves the digest — visible where the old
/// excision hid it.
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
                // Every subject is digested whole. A record's own artifact is
                // excluded by the membership predicate rather than partially
                // excised here: excising only the frontmatter left the body —
                // which the same call rewrites — inside the digest, so a run
                // whose report changed staled itself (spec 057, and see
                // `is_analyze_subject`). `spec.md` is digested whole too, with
                // the block gone there is nothing in it to excise.
                let mut hasher = Sha256::new();
                hasher.update(text.as_bytes());
                out.digests.insert(rel, hex(&hasher.finalize()));
            }
            Err(_) => out.unreadable.push(rel),
        }
    }
    out.unreadable.sort();
    out
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
    // deleted scenario alters what an analysis would find. A recorded path the
    // predicate no longer counts is not a subject, so it is not compared.
    for path in recorded.keys().filter(|path| is_subject(path)) {
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
    // The sweep index is keyed by git's names, from the work tree; the
    // candidates are the project's. In a project in a subdirectory of its
    // repository every lookup missed, and a missing path reads as a real
    // change, so the exemption never ran (spec 059).
    let Ok(project) = crate::primitives::ProjectRepository::discover(repo) else {
        return candidates;
    };
    let repository = &project.repository;
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
        crate::primitives::mechanical_sweep::SweepIndex::build(repository, &base_tree, &head_tree);
    candidates
        .into_iter()
        .filter(|path| index.changed_beyond_spelling(&project.to_git(path)))
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
    let spec_path = repo.join(rel_dir).join("spec.md");
    let done = read_text(&spec_path)
        .ok()
        .and_then(|content| crate::primitives::frontmatter_status(&content, &spec_path))
        .is_some_and(|status| status == "done");
    freshness_of(
        repo,
        rel_dir,
        analyze.last_run.as_deref(),
        analyze.analyzed_against.as_deref(),
        // Empty is treated as absent here: `spec.md` is always a subject, so
        // an empty analyze digest can only mean the record predates the field.
        Some(&analyze.analyzed_digest).filter(|d| !d.is_empty()),
        if done {
            is_done_analyze_subject
        } else {
            is_analyze_subject
        },
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
    use std::path::PathBuf;
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

    /// The exclusion the whole comparison rests on, at its new address: the
    /// record is written after the subjects are read, so a digest covering it
    /// could never match and every run would stale itself. It moved with the
    /// record — from `spec.md`'s `analyze:` block to `analysis.md` (spec 057
    /// AC7).
    ///
    /// **It covers the whole file, not just the frontmatter**, and this test
    /// pins that rather than the narrower rule it replaced. `write-analysis`
    /// rewrites the report body in the same call that writes the record, so
    /// excising the frontmatter alone left every run whose counts or Summary
    /// changed digesting a body it was about to replace — reporting itself
    /// stale, and converging only on a second identical run.
    #[test]
    fn the_record_is_not_a_subject_of_its_own_digest() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), "");
        fs::write(
            tmp.path().join("analysis.md"),
            "---\nspec: 001-x\nlast-run: null\n---\n\n# Analysis\n\nOne finding.\n",
        )
        .unwrap();
        let before = subject_digest(tmp.path(), is_analyze_subject);

        // A second run records different counts, a different timestamp, *and*
        // a different report body — which is what every run with a changed
        // result does.
        fs::write(
            tmp.path().join("analysis.md"),
            "---\nspec: 001-x\nlast-run: 2026-09-07T00:00:00Z\nadvisory: 4\nblocking: false\n---\n\n# Analysis\n\nTwelve findings.\n",
        )
        .unwrap();
        let after = subject_digest(tmp.path(), is_analyze_subject);

        assert!(
            !before.digests.contains_key("analysis.md"),
            "the analyze record is its own command's output and must not be a subject"
        );
        assert_eq!(
            before.digests, after.digests,
            "rewriting the record — frontmatter or body — must not stale it"
        );
    }

    /// The other half: with the block gone, `spec.md` is digested whole, so a
    /// frontmatter edit there **does** move the digest. The old excision would
    /// have hidden it.
    #[test]
    fn a_spec_frontmatter_edit_does_change_its_digest() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), "");
        let before = subject_digest(tmp.path(), is_analyze_subject);
        fs::write(
            tmp.path().join("spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# Spec\n",
        )
        .unwrap();
        let after = subject_digest(tmp.path(), is_analyze_subject);
        assert_ne!(before.digests["spec.md"], after.digests["spec.md"]);
    }

    /// The exclusion is the record's alone — the *review* record stays a
    /// subject, body and frontmatter both, because `review.md` is an input to
    /// an analysis rather than its output. Excluding it would exempt the single
    /// edit that most often invalidates an analyze record, which is the
    /// asymmetry `is_analyze_subject` and `is_review_contract` exist to keep.
    #[test]
    fn the_review_record_remains_a_subject() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), "");
        fs::write(
            tmp.path().join("review.md"),
            "---\nspec: 001-x\nlast-run: null\n---\n\n# Review\n\nOne finding.\n",
        )
        .unwrap();
        let before = subject_digest(tmp.path(), is_analyze_subject);
        fs::write(
            tmp.path().join("review.md"),
            "---\nspec: 001-x\nlast-run: 2026-09-07T00:00:00Z\n---\n\n# Review\n\nTwelve findings.\n",
        )
        .unwrap();
        let after = subject_digest(tmp.path(), is_analyze_subject);
        assert_ne!(before.digests["review.md"], after.digests["review.md"]);
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

    /// A record taken over `seed`'s artifacts, with the spec at `status`.
    fn recorded_at(dir: &Path, status: &str) -> AnalyzeBlock {
        fs::write(
            dir.join("spec.md"),
            format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Spec\n"),
        )
        .unwrap();
        fs::write(dir.join("plan.md"), "# Plan\n\n## Overview\n\nA.\n").unwrap();
        AnalyzeBlock {
            last_run: Some("2026-10-26T00:00:00Z".into()),
            analyzed_digest: subject_digest(dir, is_analyze_subject).digests,
            ..AnalyzeBlock::default()
        }
    }

    /// The `done` half of spec 041's `a-plan-edit-does-not-reopen-a-done-spec`:
    /// a plan or tasks edit takes no back-edge, so it does not stale the record.
    #[test]
    fn a_plan_or_tasks_edit_does_not_stale_a_done_specs_record() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let analyze = recorded_at(&dir, "done");
        fs::write(dir.join("plan.md"), "# Plan\n\n## Overview\n\nB.\n").unwrap();
        fs::write(dir.join("tasks.md"), "# Tasks\n").unwrap();
        let result = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));
        assert!(
            matches!(result, RecordFreshness::Current { .. }),
            "{result:?}"
        );
    }

    /// The other direction: the same edit on a spec short of `done` stales it,
    /// and a `done` spec's other subjects still do.
    #[test]
    fn a_plan_edit_stales_an_open_spec_and_a_spec_edit_stales_a_done_one() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let analyze = recorded_at(&dir, "in-progress");
        fs::write(dir.join("plan.md"), "# Plan\n\n## Overview\n\nB.\n").unwrap();
        let RecordFreshness::Stale { paths, .. } =
            analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze))
        else {
            panic!("a plan edit on an in-progress spec must stale its record");
        };
        assert_eq!(paths, vec!["specs/001-x/plan.md".to_string()]);

        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let analyze = recorded_at(&dir, "done");
        fs::write(dir.join("scenarios/a.md"), "# A, changed\n").unwrap();
        let RecordFreshness::Stale { paths, .. } =
            analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze))
        else {
            panic!("a scenario edit on a done spec must stale its record");
        };
        assert_eq!(paths, vec!["specs/001-x/scenarios/a.md".to_string()]);
    }

    /// A recorded `plan.md` the `done` predicate no longer counts is not read as
    /// a deleted subject.
    #[test]
    fn a_done_specs_recorded_plan_is_not_a_deleted_subject() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        let analyze = recorded_at(&dir, "done");
        fs::remove_file(dir.join("plan.md")).unwrap();
        let result = analyze_freshness(tmp.path(), "specs/001-x", Some(&analyze));
        assert!(
            matches!(result, RecordFreshness::Current { .. }),
            "{result:?}"
        );
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

    /// A project at `proj/` in a subdirectory of its repository, with one
    /// scenario and a data model, and a document in another project; each
    /// names `/govern`. Commits that as the base, records both digests
    /// against it, then rewrites each file as `edit` renders it and commits
    /// again. Returns the project root and the analyze and review records.
    fn swept_subdirectory_project(
        root: &Path,
        edit: impl Fn(&str, &str) -> String,
    ) -> (
        PathBuf,
        AnalyzeBlock,
        crate::schema::primitives::ReviewBlock,
    ) {
        use crate::primitives::git_fixture;
        let (repository, project) = git_fixture::subdirectory_project(root);
        let dir = project.join("specs/001-x");
        seed(&dir, "");
        let files = [
            (
                dir.join("scenarios/a.md"),
                "# A\n\nRun `/govern`. Timeout is 30s.\n",
            ),
            (
                dir.join("data-model.md"),
                "# Model\n\nWritten by `/govern`.\n",
            ),
            (root.join("other/docs/x.md"), "Run `/govern` to sync.\n"),
        ];
        for (path, body) in &files {
            git_fixture::write(path, body);
        }
        let base = git_fixture::commit_all(&repository, "base").to_string();
        let analyze = AnalyzeBlock {
            last_run: Some("2026-09-26T00:00:00Z".into()),
            analyzed_against: Some(base.clone()),
            analyzed_digest: subject_digest(&dir, is_analyze_subject).digests,
            ..AnalyzeBlock::default()
        };
        let review = crate::schema::primitives::ReviewBlock {
            last_run: Some("2026-09-26T00:00:00Z".into()),
            reviewed_against: Some(base),
            reviewed_digest: Some(subject_digest(&dir, is_review_contract).digests),
            ..crate::schema::primitives::ReviewBlock::default()
        };
        for (path, body) in &files {
            let name = path.file_name().unwrap().to_str().unwrap();
            git_fixture::write(path, &edit(name, body));
        }
        git_fixture::commit_all(&repository, "sweep");
        (project, analyze, review)
    }

    /// A uniform repo-wide rename, across two projects, in a project in a
    /// subdirectory of its repository. The sweep index names paths from the
    /// work tree, so looked up by the project's names every subject missed,
    /// read as changed, and both records went stale over a rename (spec 059).
    #[test]
    fn a_rename_sweep_leaves_a_subdirectory_projects_records_current() {
        let tmp = tempdir().unwrap();
        let (project, analyze, review) =
            swept_subdirectory_project(tmp.path(), |_, body| body.replace("/govern", "/ductus"));
        let result = analyze_freshness(&project, "specs/001-x", Some(&analyze));
        assert!(
            matches!(result, RecordFreshness::Current { .. }),
            "{result:?}"
        );
        let result = review_freshness(&project, "specs/001-x", Some(&review));
        assert!(
            matches!(result, RecordFreshness::Current { .. }),
            "{result:?}"
        );
    }

    #[test]
    fn a_real_change_inside_a_sweep_still_stales_a_subdirectory_project() {
        let tmp = tempdir().unwrap();
        let (project, analyze, review) = swept_subdirectory_project(tmp.path(), |name, body| {
            let swept = body.replace("/govern", "/ductus");
            if name == "a.md" {
                swept.replace("30s", "60s")
            } else {
                swept
            }
        });
        let expected = vec!["specs/001-x/scenarios/a.md".to_string()];
        match analyze_freshness(&project, "specs/001-x", Some(&analyze)) {
            RecordFreshness::Stale { paths, .. } => assert_eq!(paths, expected),
            other => panic!("expected stale, got {other:?}"),
        }
        match review_freshness(&project, "specs/001-x", Some(&review)) {
            RecordFreshness::Stale { paths, .. } => assert_eq!(paths, expected),
            other => panic!("expected stale, got {other:?}"),
        }
    }

    /// The reach failure, not the read failure — the one that used to be
    /// dropped by `filter_map(Result::ok)`.
    ///
    /// Unix only: the test makes `scenarios/` untraversable with a mode
    /// bit, which Windows has no equivalent of, so there it would assert
    /// nothing.
    #[cfg(unix)]
    #[test]
    fn a_subject_the_walk_cannot_reach_is_recorded() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        seed(&dir, "");
        // Make `scenarios/` untraversable so the walk errors on it rather
        // than on any single file.
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
