//! `backfill-section-digest` — give each analysis record written before
//! spec 082's AC7, and still current, the `section-digest` its run would
//! have written (spec 088, AC3).
//!
//! A record with no `section-digest` makes its spec's next analysis judge
//! every loaded rule after any design-subject edit, since nothing says which
//! sections changed. A record whose `analyzed-digest` still matches its
//! spec's design subjects was judged against exactly the sections they now
//! have, so their digests are the ones its run would have written. A `done`
//! spec's analysis ran before the done flip rewrote `spec.md`'s `status:`
//! line, so a record also matches when `spec.md` with its `status:` set to
//! another lifecycle status hashes to the recorded digest, which proves the
//! text the run read; the digests written are then that text's. Nothing is
//! written on any other ground, so the backfill asserts no analysis that did
//! not happen.
//!
//! The field is rendered by `write-analysis`' own renderer and placed before
//! the frontmatter's closing fence, every other byte of the record as it was,
//! and each write is guarded (`BE-TXN-002`): a record another writer changed
//! since it was read is left as that writer made it and named. A record that
//! carries the field already is left, so a second run writes nothing.

use std::path::Path;

use crate::primitives::analyze_subjects::section_digest_with;
use crate::primitives::rule_verdicts::design_subjects_match;
use crate::primitives::set_status::locate_status_field;
use crate::primitives::write_analysis::render_section_digest;
use crate::primitives::{
    ANALYSIS_RECORD_FILE, PrimitiveError, Result, bounded_lists, guarded, line_ending_of,
    list_feature_dirs, parse_record, read_text, split_frontmatter_with_offset,
    validate_feature_argument, with_line_ending,
};
use crate::schema::paths;
use crate::schema::primitives::{
    AnalyzeBlock, BackfillSectionDigestArgs, BackfillSectionDigestResult, BackfilledRecord,
    ListsForm, UnreadableAnalysisRecord,
};
use crate::schema::status::ALLOWED_STATUSES;

/// Backfill every current record under the spec root, then hold the result
/// to the read size.
///
/// # Errors
///
/// As [`compute`] and [`bound`].
pub fn run(args: &BackfillSectionDigestArgs, repo: &Path) -> Result<BackfillSectionDigestResult> {
    bound(repo, compute(args, repo)?)
}

/// Backfill every current record under the spec root, every list whole, so
/// the CLI names each refused record before the result is bounded.
///
/// # Errors
///
/// A project config that does not parse, a spec root that cannot be listed,
/// and a write that fails for a reason other than another writer's change.
pub fn compute(
    args: &BackfillSectionDigestArgs,
    repo: &Path,
) -> Result<BackfillSectionDigestResult> {
    let root = paths::Paths::load(repo)?.specs_root;
    let pinned: Vec<&str> = args
        .pinned
        .iter()
        .map(|path| path.trim().trim_start_matches("./"))
        .collect();
    let mut result = BackfillSectionDigestResult {
        specs_root: root.clone(),
        written: Some(Vec::new()),
        unmatched: Some(Vec::new()),
        ..BackfillSectionDigestResult::default()
    };
    let mut pinned_records = Vec::new();
    let mut unreadable = Vec::new();
    let mut refused = Vec::new();
    for feature in list_feature_dirs(repo, &root)? {
        let feature_dir = repo.join(&root).join(&feature);
        let rel = format!("{root}/{feature}/{ANALYSIS_RECORD_FILE}");
        if !feature_dir.join(ANALYSIS_RECORD_FILE).is_file() {
            continue;
        }
        result.examined += 1;
        if pinned.contains(&rel.as_str()) {
            pinned_records.push(feature);
            continue;
        }
        let unread = |reason: String| UnreadableAnalysisRecord {
            feature: feature.clone(),
            reason,
        };
        // A feature directory, or a file in it, that is a link leaving the
        // project is named and never read (`BE-INPUT-004`); the record is read
        // through the guard, so the write below compares against these bytes.
        if let Err(error) = validate_feature_argument(repo, &root, &feature) {
            unreadable.push(unread(error.to_string()));
            continue;
        }
        let (snapshot, content) = match guarded::read_required(repo, &rel) {
            Ok(read) => read,
            Err(error) => {
                unreadable.push(unread(error.to_string()));
                continue;
            }
        };
        let record: AnalyzeBlock = match parse_record(&content, &repo.join(&rel)) {
            Ok(record) => record,
            Err(reason) => {
                unreadable.push(unread(reason));
                continue;
            }
        };
        if !record.section_digest.is_empty() {
            result.has_section_digest += 1;
            continue;
        }
        let spec = read_text(&feature_dir.join("spec.md")).ok();
        let Some((matched_at, spec_text)) = matching_text(&feature_dir, &record, spec.as_deref())
        else {
            result.unmatched.get_or_insert_with(Vec::new).push(feature);
            continue;
        };
        let sections = section_digest_with(&feature_dir, spec_text.as_deref());
        if sections.is_empty() {
            result.no_design_subject += 1;
            continue;
        }
        let mut field = String::new();
        render_section_digest(&mut field, &sections);
        // The field takes the record's own line ending, so a CRLF record stays
        // uniformly CRLF (spec 051, scenario `rewrites-preserve-line-endings`).
        let field = with_line_ending(&field, line_ending_of(&content));
        let backfilled = with_field(&content, &field, &repo.join(&rel))?;
        match guarded::write_guarded(&snapshot, &backfilled) {
            Ok(()) => result
                .written
                .get_or_insert_with(Vec::new)
                .push(BackfilledRecord {
                    feature,
                    matched_at,
                }),
            Err(PrimitiveError::ChangedSinceRead { .. }) => refused.push(feature),
            Err(error) => return Err(error),
        }
    }
    result.written_count = result.written.as_ref().map_or(0, Vec::len);
    result.unmatched_count = result.unmatched.as_ref().map_or(0, Vec::len);
    result.pinned_count = pinned_records.len();
    result.unreadable_count = unreadable.len();
    result.refused_count = refused.len();
    result.pinned = (!pinned_records.is_empty()).then_some(pinned_records);
    result.unreadable = (!unreadable.is_empty()).then_some(unreadable);
    result.refused = (!refused.is_empty()).then_some(refused);
    Ok(result)
}

/// The status of the `spec.md` text a record's design subjects match, and
/// that text when it is not the one on disk: as it stands, or with its
/// `status:` set to each other lifecycle status in canonical order. `None`
/// when they match at none. A `spec.md` that cannot be read, or names no
/// status, is tried as it stands alone.
fn matching_text(
    feature_dir: &Path,
    record: &AnalyzeBlock,
    spec: Option<&str>,
) -> Option<(Option<String>, Option<String>)> {
    let status_at = spec.and_then(|text| status_span(text, feature_dir));
    if design_subjects_match(feature_dir, record, None) == Some(true) {
        return Some((status_at.map(|(status, _)| status), None));
    }
    let (spec, (current, span)) = spec.zip(status_at)?;
    ALLOWED_STATUSES
        .iter()
        .filter(|status| **status != current)
        .find_map(|status| {
            let text = format!("{}{status}{}", &spec[..span.start], &spec[span.end..]);
            (design_subjects_match(feature_dir, record, Some(&text)) == Some(true))
                .then(|| (Some((*status).to_string()), Some(text)))
        })
}

/// The `status:` value of `spec`, `spec.md`'s text, and its byte range in
/// that text, read as `set-status` reads it, so a substituted text is the one
/// `set-status` would write.
fn status_span(spec: &str, feature_dir: &Path) -> Option<(String, std::ops::Range<usize>)> {
    let path = feature_dir.join("spec.md");
    let (frontmatter, _body, offset) = split_frontmatter_with_offset(spec, &path).ok()?;
    let (value, range) = locate_status_field(frontmatter, "", "").ok()?;
    Some((value, offset + range.start..offset + range.end))
}

/// `content`, a record's text, with `field` placed before its frontmatter's
/// closing fence and every other byte as it was.
fn with_field(content: &str, field: &str, path: &Path) -> Result<String> {
    let (frontmatter, _body, offset) = split_frontmatter_with_offset(content, path)?;
    // The frontmatter text stops before the newline that ends its last line,
    // unless it is empty, when the closing fence follows the opener at once.
    let at = if frontmatter.is_empty() {
        offset
    } else {
        offset + frontmatter.len() + 1
    };
    Ok(format!("{}{field}{}", &content[..at], &content[at..]))
}

/// `result` held to `[artifacts] read-size-bytes`: inline while it fits, and
/// past it with every list in the file it names, their counts inline
/// (`bounded_lists::place`).
///
/// # Errors
///
/// A malformed read size, and a lists file that cannot be written.
pub fn bound(
    repo: &Path,
    mut result: BackfillSectionDigestResult,
) -> Result<BackfillSectionDigestResult> {
    result.form = ListsForm::Inline;
    let lists = serde_json::json!({
        "written": result.written.clone().unwrap_or_default(),
        "unmatched": result.unmatched.clone().unwrap_or_default(),
        "pinned": result.pinned.clone().unwrap_or_default(),
        "unreadable": result.unreadable.clone().unwrap_or_default(),
        "refused": result.refused.clone().unwrap_or_default(),
    });
    if let Some(file) = bounded_lists::place(
        repo,
        "backfill-section-digest",
        bounded_lists::ALL_SPECS,
        &result,
        &lists,
    )? {
        result.form = ListsForm::File;
        result.written = None;
        result.unmatched = None;
        result.pinned = None;
        result.unreadable = None;
        result.refused = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::analyze_subjects::section_digest;
    use crate::primitives::{RecordLoad, load_analyze_record};
    use crate::schema::primitives::WriteAnalysisArgs;
    use std::fs;
    use std::path::PathBuf;

    const SPEC: &str =
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# 042 — Demo\n\n## Behavior\n\nText.\n";
    const RECORD: &str = "specs/042-demo/analysis.md";

    /// Feature `042-demo` under `project`, holding `SPEC` and a plan, analyzed
    /// by `write-analysis` and its record's `section-digest` then removed, as
    /// a record written before spec 082's AC7 reads.
    fn analyzed_in(project: &Path) -> PathBuf {
        let dir = project.join("specs/042-demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("spec.md"), SPEC).unwrap();
        fs::write(dir.join("plan.md"), "# Plan\n\n## Decisions\n\nOne.\n").unwrap();
        let args = WriteAnalysisArgs {
            feature: "042-demo".into(),
            analyzed_at: "2026-11-26T00:00:00Z".into(),
            analyzed_against: "abc".into(),
            ..WriteAnalysisArgs::default()
        };
        crate::primitives::write_analysis::run(&args, project).unwrap();
        let record = dir.join("analysis.md");
        let written = fs::read_to_string(&record).unwrap();
        assert!(written.contains("section-digest:\n"), "{written}");
        fs::write(&record, without_section_digest(&written)).unwrap();
        dir
    }

    fn analyzed() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let dir = analyzed_in(tmp.path());
        (tmp, dir)
    }

    /// `text` with its `section-digest` field and its entries removed.
    fn without_section_digest(text: &str) -> String {
        let mut out = String::new();
        let mut skipping = false;
        for line in text.split_inclusive('\n') {
            if line.starts_with("section-digest:") {
                skipping = true;
                continue;
            }
            if skipping && line.starts_with("  ") {
                continue;
            }
            skipping = false;
            out.push_str(line);
        }
        out
    }

    fn text_of(dir: &Path) -> String {
        fs::read_to_string(dir.join("analysis.md")).unwrap()
    }

    fn record_of(dir: &Path) -> AnalyzeBlock {
        let RecordLoad::Present(record) = load_analyze_record(dir) else {
            panic!("the record did not load");
        };
        record
    }

    fn backfill(project: &Path, pinned: &[&str]) -> BackfillSectionDigestResult {
        let args = BackfillSectionDigestArgs {
            pinned: pinned.iter().map(|path| (*path).to_string()).collect(),
        };
        run(&args, project).unwrap()
    }

    fn written(feature: &str, matched_at: &str) -> BackfilledRecord {
        BackfilledRecord {
            feature: feature.into(),
            matched_at: Some(matched_at.into()),
        }
    }

    /// Spec 088, AC3: a record whose `analyzed-digest` matches its spec's
    /// design subjects as they stand gains the `section-digest` its run would
    /// have written, and every other byte of it stands.
    #[test]
    fn a_current_record_gains_its_digest_and_every_other_byte_stands() {
        let (tmp, dir) = analyzed();
        let before = text_of(&dir);
        let result = backfill(tmp.path(), &[]);
        assert_eq!(result.examined, 1);
        assert_eq!(
            result.written.unwrap(),
            [written("042-demo", "in-progress")]
        );
        assert_eq!(result.written_count, 1);
        assert_eq!(result.unmatched.unwrap(), [] as [String; 0]);
        let after = text_of(&dir);
        assert_ne!(after, before);
        assert_eq!(without_section_digest(&after), before);
        let sections = record_of(&dir).section_digest;
        assert!(!sections.is_empty());
        assert_eq!(sections, section_digest(&dir));
    }

    /// Spec 088, AC3: a `done` spec's record, its analysis read at
    /// `in-progress` before the done flip, matches with `spec.md` at that
    /// status and gains that text's digests, never the done text's; the
    /// spec itself is not touched.
    #[test]
    fn a_record_analyzed_at_another_status_gains_that_text_s_digest() {
        let (tmp, dir) = analyzed();
        let done = SPEC.replace("status: in-progress", "status: done");
        fs::write(dir.join("spec.md"), &done).unwrap();
        let result = backfill(tmp.path(), &[]);
        assert_eq!(
            result.written.unwrap(),
            [written("042-demo", "in-progress")]
        );
        let sections = record_of(&dir).section_digest;
        assert_eq!(sections, section_digest_with(&dir, Some(SPEC)));
        assert_ne!(sections, section_digest(&dir));
        assert_eq!(fs::read_to_string(dir.join("spec.md")).unwrap(), done);
    }

    /// Spec 088, AC3: a record whose design subjects changed since its run
    /// matches at no status, and is named and left as it was.
    #[test]
    fn a_stale_record_is_named_and_left_as_it_was() {
        let (tmp, dir) = analyzed();
        fs::write(dir.join("plan.md"), "# Plan\n\n## Decisions\n\nTwo.\n").unwrap();
        let before = text_of(&dir);
        let result = backfill(tmp.path(), &[]);
        assert_eq!(result.unmatched.unwrap(), ["042-demo"]);
        assert_eq!(result.unmatched_count, 1);
        assert_eq!(result.written_count, 0);
        assert_eq!(text_of(&dir), before);
    }

    /// Spec 088, AC3: a second run writes nothing, since the record now
    /// carries `section-digest`.
    #[test]
    fn a_second_run_writes_nothing() {
        let (tmp, dir) = analyzed();
        backfill(tmp.path(), &[]);
        let once = text_of(&dir);
        let again = backfill(tmp.path(), &[]);
        assert_eq!(again.written_count, 0);
        assert_eq!(again.has_section_digest, 1);
        assert_eq!(text_of(&dir), once);
    }

    /// A pinned record is reported and left; a record that does not parse is
    /// named with its reason and left; each count is inline, and an empty
    /// list is absent.
    #[test]
    fn a_pinned_record_and_an_unreadable_one_are_named_and_left_untouched() {
        let (tmp, dir) = analyzed();
        let before = text_of(&dir);
        let broken = tmp.path().join("specs/043-other");
        fs::create_dir_all(&broken).unwrap();
        fs::write(broken.join("analysis.md"), "---\nlast-run: [\n---\n").unwrap();
        let result = backfill(tmp.path(), &[RECORD]);
        assert_eq!(result.examined, 2);
        assert_eq!(result.pinned.unwrap(), ["042-demo"]);
        assert_eq!(result.pinned_count, 1);
        let unreadable = result.unreadable.unwrap();
        assert_eq!(unreadable.len(), 1);
        assert_eq!(unreadable[0].feature, "043-other");
        assert_ne!(unreadable[0].reason, "");
        assert_eq!(result.unreadable_count, 1);
        assert_eq!(result.written_count, 0);
        assert_eq!(text_of(&dir), before);
        assert_eq!(text_of(&broken), "---\nlast-run: [\n---\n");

        let clean = serde_json::to_value(backfill(tmp.path(), &[])).unwrap();
        assert!(
            clean.get("pinned").is_none() && clean["pinned-count"] == 0,
            "{clean}"
        );
        assert!(
            clean.get("refused").is_none() && clean["refused-count"] == 0,
            "{clean}"
        );
    }

    /// `BE-INPUT-004`: a feature directory holding a file that is a link
    /// leaving the project is named under `unreadable` with its reason and
    /// never read, its record left as it was.
    #[cfg(unix)]
    #[test]
    fn a_feature_holding_a_link_leaving_the_project_is_named_and_left() {
        let (tmp, dir) = analyzed();
        let before = text_of(&dir);
        let outside = tempfile::tempdir().unwrap();
        let target = outside.path().join("spec.md");
        fs::write(&target, SPEC).unwrap();
        fs::remove_file(dir.join("spec.md")).unwrap();
        std::os::unix::fs::symlink(&target, dir.join("spec.md")).unwrap();
        let result = backfill(tmp.path(), &[]);
        let unreadable = result.unreadable.unwrap();
        assert_eq!(unreadable.len(), 1);
        assert_eq!(unreadable[0].feature, "042-demo");
        assert!(
            unreadable[0].reason.contains("outside"),
            "{}",
            unreadable[0].reason
        );
        assert_eq!(result.written_count, 0);
        assert_eq!(text_of(&dir), before);
    }

    /// `BE-TXN-002`: a record another writer changed between this call's read
    /// and its write is named under `refused` and left as that writer made
    /// it.
    #[test]
    fn a_record_changed_since_it_was_read_is_refused() {
        let (tmp, dir) = analyzed();
        let before = text_of(&dir);
        let result = guarded::between_read_and_write(
            guarded::theirs_lands(&dir.join("analysis.md")),
            || backfill(tmp.path(), &[]),
        );
        assert_eq!(result.refused.unwrap(), ["042-demo"]);
        assert_eq!(result.refused_count, 1);
        assert_eq!(result.written_count, 0);
        assert_eq!(text_of(&dir), format!("{before}\nTheirs.\n"));
    }

    /// A feature whose design subjects hold no section has nothing to
    /// write: counted, and left.
    #[test]
    fn a_record_with_no_design_subject_is_counted_and_left() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/044-bare");
        fs::create_dir_all(&dir).unwrap();
        let record = "---\nspec: 044-bare\nlast-run: 2026-11-26T00:00:00Z\n---\n\n# Analysis\n";
        fs::write(dir.join("analysis.md"), record).unwrap();
        let result = backfill(tmp.path(), &[]);
        assert_eq!(result.no_design_subject, 1);
        assert_eq!(result.written_count, 0);
        assert_eq!(text_of(&dir), record);
    }

    /// Spec 059's layout: a project in a subdirectory of its repository is
    /// walked from its own root.
    #[test]
    fn a_project_in_a_subdirectory_is_backfilled() {
        let tmp = tempfile::tempdir().unwrap();
        let (_repository, project) =
            crate::primitives::git_fixture::subdirectory_project(tmp.path());
        let dir = analyzed_in(&project);
        let result = backfill(&project, &[]);
        assert_eq!(
            result.written.unwrap(),
            [written("042-demo", "in-progress")]
        );
        assert_eq!(record_of(&dir).section_digest, section_digest(&dir));
    }

    /// Past the read size every list moves to the file, its count inline.
    #[test]
    fn past_the_read_size_the_lists_move_to_the_file() {
        let (tmp, _dir) = analyzed();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 200\n",
        )
        .unwrap();
        let result = backfill(tmp.path(), &[]);
        assert_eq!(result.form, ListsForm::File);
        assert!(result.written.is_none() && result.unmatched.is_none());
        assert_eq!(result.written_count, 1);
        let file = result.lists_file.clone().unwrap();
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(tmp.path().join(&file)).unwrap()).unwrap();
        assert_eq!(lists["written"][0]["feature"], "042-demo");
        bounded_lists::assert_counts_name_the_file(
            "backfill-section-digest",
            &serde_json::to_value(&result).unwrap(),
            &lists,
        );
    }
}
