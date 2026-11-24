//! `check-owed-work` — whether a spec owes work, by the one test the rest of the
//! runtime reads ([`super::owed_work`], spec 084): an unchecked box in its
//! `tasks.md` or an unticked acceptance criterion in its `spec.md`.
//!
//! A command that edits a spec reads this to decide a reopen, so the decision comes
//! from the spec's state, never from an edit's diff. Under `all` it reports every
//! `done` spec that owes work, which is what a CI step and the framework's own audit
//! call in place of parsing the corpus themselves (§runtime-boundary, principle 3).
//!
//! Both forms read the working tree. A spec the corpus form cannot judge is named in
//! `skipped` with its reason, never counted as owing nothing (`QUAL-CLAIM-001`).

use std::collections::BTreeSet;
use std::path::Path;

use crate::primitives::owed_work::{OwedWork, owed_work};
use crate::primitives::{
    PrimitiveError, Result, frontmatter_status, list_feature_dirs, list_untracked_specs, read_text,
    split_frontmatter, validate_feature_argument,
};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckOwedWorkArgs, CheckOwedWorkResult, ListsForm, OwedSpec, OwedWorkOfCorpus, OwedWorkOfSpec,
    OwedWorkSkip, OwedWorkSkipReason, UncheckedBox, UntickedCriterion,
};

const PRIMITIVE: &str = "check-owed-work";

/// The status the corpus form reads.
const DONE: &str = "done";

/// Execute the `check-owed-work` primitive: one spec's owed work for `feature`, or
/// every `done` spec's under `all`, returned inline while it fits `[artifacts]
/// read-size-bytes` and past it with its lists in the file `lists-file` names
/// (`bounded_lists::place`; `BE-PAYLOAD-001`).
///
/// # Errors
///
/// [`PrimitiveError::MissingArgument`] when the call names neither `feature` nor
/// `all`, and [`PrimitiveError::InvalidArgument`] when it names both, before
/// anything is read. For `feature`: the feature argument's refusals
/// ([`validate_feature_argument`]: a traversal, or a link under the directory that
/// leaves the project), [`PrimitiveError::FeatureNotFound`],
/// [`PrimitiveError::MissingSpecFile`], [`PrimitiveError::Io`] when `spec.md` or a
/// present `tasks.md` cannot be read, and `spec.md`'s frontmatter errors from
/// [`split_frontmatter`]: a missing `spec.md` is an error, never `owes: false`.
/// Under `all`: [`PrimitiveError::Git`] when the repository's status cannot be
/// read, so which specs are untracked has no answer. Both: those
/// `bounded_lists::place` returns.
pub fn run(args: &CheckOwedWorkArgs, repo: &Path) -> Result<CheckOwedWorkResult> {
    let root = paths::Paths::load(repo)?.specs_root;
    match (&args.feature, args.all) {
        (Some(feature), false) => {
            let (record, owes) = spec_owed(repo, &root, feature)?;
            bound_spec(repo, record, owes)
        }
        (None, true) => bound_corpus(repo, corpus_owed(repo, &root)?),
        (None, false) => Err(PrimitiveError::MissingArgument {
            primitive: PRIMITIVE.into(),
            argument: "feature".into(),
            reason: "name a feature for one spec, or set all for every done spec".into(),
        }),
        (Some(_), true) => Err(PrimitiveError::InvalidArgument {
            primitive: PRIMITIVE.into(),
            argument: "all".into(),
            reason: "all reads every done spec in place of one; drop feature".into(),
        }),
    }
}

/// One spec's owed work, its lists whole, and whether it owes any.
fn spec_owed(repo: &Path, root: &str, feature: &str) -> Result<(OwedSpec, bool)> {
    validate_feature_argument(repo, root, feature)?;
    let dir = repo.join(root).join(feature);
    if !dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    }
    let spec_path = dir.join("spec.md");
    if !spec_path.is_file() {
        return Err(PrimitiveError::MissingSpecFile {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    }
    let spec = read_text(&spec_path)?;
    let (_frontmatter, body) = split_frontmatter(&spec, &spec_path)?;
    let tasks_path = dir.join("tasks.md");
    let tasks = if tasks_path.is_file() {
        Some(read_text(&tasks_path)?)
    } else {
        None
    };
    let work = owed_work(body, tasks.as_deref());
    Ok((owed_spec(feature, &work), work.owes()))
}

/// What the corpus form found: every list whole.
struct CorpusOwed {
    specs: Vec<OwedSpec>,
    examined: usize,
    skipped: Vec<OwedWorkSkip>,
}

/// Every `done` spec's owed work, in corpus order, with each spec it could not
/// judge, at any status, in `skipped`.
fn corpus_owed(repo: &Path, root: &str) -> Result<CorpusOwed> {
    let untracked: BTreeSet<String> = list_untracked_specs(repo, root)?.into_iter().collect();
    let mut found = CorpusOwed {
        specs: Vec::new(),
        examined: 0,
        skipped: Vec::new(),
    };
    for feature in list_feature_dirs(repo, root)? {
        match judge(repo, root, &feature, &untracked)? {
            Judged::NotDone => {}
            Judged::Skipped(reason) => found.skipped.push(OwedWorkSkip { feature, reason }),
            Judged::Done(record, owes) => {
                found.examined += 1;
                if owes {
                    found.specs.push(record);
                }
            }
        }
    }
    Ok(found)
}

/// What the corpus form made of one spec.
enum Judged {
    /// Read, and not `done`.
    NotDone,
    /// `done`, and read whole: its record, and whether it owes work.
    Done(OwedSpec, bool),
    /// Not judged, for this reason.
    Skipped(OwedWorkSkipReason),
}

/// One spec of the corpus, judged by the same reads as the named form, each that
/// cannot be made a skip with its reason rather than an error, so one spec cannot
/// stop the walk.
fn judge(repo: &Path, root: &str, feature: &str, untracked: &BTreeSet<String>) -> Result<Judged> {
    match validate_feature_argument(repo, root, feature) {
        Ok(()) => {}
        Err(PrimitiveError::InvalidPath { .. }) => {
            return Ok(Judged::Skipped(OwedWorkSkipReason::OutsideProject));
        }
        Err(other) => return Err(other),
    }
    if untracked.contains(&format!("{root}/{feature}/spec.md")) {
        return Ok(Judged::Skipped(OwedWorkSkipReason::Untracked));
    }
    let dir = repo.join(root).join(feature);
    let spec_path = dir.join("spec.md");
    if !spec_path.is_file() {
        return Ok(Judged::Skipped(OwedWorkSkipReason::NoSpecFile));
    }
    let Ok(spec) = read_text(&spec_path) else {
        return Ok(Judged::Skipped(OwedWorkSkipReason::Unreadable));
    };
    let Some(status) = frontmatter_status(&spec, &spec_path) else {
        return Ok(Judged::Skipped(OwedWorkSkipReason::StatusUnreadable));
    };
    if status != DONE {
        return Ok(Judged::NotDone);
    }
    // The status parsed, so the frontmatter splits.
    let (_frontmatter, body) = split_frontmatter(&spec, &spec_path)?;
    let tasks_path = dir.join("tasks.md");
    let tasks = if tasks_path.is_file() {
        let Ok(tasks) = read_text(&tasks_path) else {
            return Ok(Judged::Skipped(OwedWorkSkipReason::Unreadable));
        };
        Some(tasks)
    } else {
        None
    };
    let work = owed_work(body, tasks.as_deref());
    Ok(Judged::Done(owed_spec(feature, &work), work.owes()))
}

/// The result's record of one spec's owed work.
fn owed_spec(feature: &str, owed: &OwedWork<'_>) -> OwedSpec {
    OwedSpec {
        feature: feature.to_string(),
        unchecked_boxes: owed
            .unchecked_boxes
            .iter()
            .map(|(line, count)| UncheckedBox {
                line: (*line).to_string(),
                count: *count,
            })
            .collect(),
        unticked_criteria: owed
            .unticked_criteria
            .iter()
            .map(|(_, criterion)| UntickedCriterion {
                label: criterion.label.clone(),
                text: criterion.text.clone(),
            })
            .collect(),
    }
}

/// The named form, held to the read size: past it, its two lists go to the file
/// the result names, with their counts inline.
fn bound_spec(repo: &Path, record: OwedSpec, owes: bool) -> Result<CheckOwedWorkResult> {
    let lists = serde_json::json!({
        "unchecked-boxes": record.unchecked_boxes,
        "unticked-criteria": record.unticked_criteria,
    });
    let feature = record.feature.clone();
    let mut result = CheckOwedWorkResult {
        spec: Some(OwedWorkOfSpec {
            owes,
            unchecked_boxes_count: record.unchecked_boxes.len(),
            unticked_criteria_count: record.unticked_criteria.len(),
            unchecked_boxes: Some(record.unchecked_boxes),
            unticked_criteria: Some(record.unticked_criteria),
            feature: record.feature,
        }),
        corpus: None,
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) = super::bounded_lists::place(repo, PRIMITIVE, &feature, &result, &lists)? {
        if let Some(spec) = result.spec.as_mut() {
            spec.unchecked_boxes = None;
            spec.unticked_criteria = None;
        }
        result.form = ListsForm::File;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// The corpus form, held to the read size: past it, `specs` and `skipped` go to the
/// file the result names, with their counts inline.
fn bound_corpus(repo: &Path, found: CorpusOwed) -> Result<CheckOwedWorkResult> {
    let lists = serde_json::json!({
        "specs": found.specs,
        "skipped": found.skipped,
    });
    let mut result = CheckOwedWorkResult {
        spec: None,
        corpus: Some(OwedWorkOfCorpus {
            specs_count: found.specs.len(),
            specs: Some(found.specs),
            examined_count: found.examined,
            skipped_count: found.skipped.len(),
            skipped: Some(found.skipped),
        }),
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) = super::bounded_lists::place(
        repo,
        PRIMITIVE,
        super::bounded_lists::ALL_SPECS,
        &result,
        &lists,
    )? {
        if let Some(corpus) = result.corpus.as_mut() {
            corpus.specs = None;
            corpus.skipped = None;
        }
        result.form = ListsForm::File;
        result.lists_file = Some(file);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use tempfile::TempDir;

    const OWING_SPEC: &str = "---\nstatus: done\ndependencies: []\n---\n\n# 001\n\n## Acceptance Criteria\n\n- [x] AC1: one\n- [ ] AC2: two\n";
    const TICKED_SPEC: &str = "---\nstatus: done\ndependencies: []\n---\n\n# 002\n\n## Acceptance Criteria\n\n- [x] AC1: one\n";
    const OPEN_TASKS: &str = "# T\n\n## 1. One\n\n- [x] a\n- [ ] b\n";
    const SPENT_TASKS: &str = "# T\n\n## 1. One\n\n- [x] a\n\n<!--\n- [ ] an example\n-->\n";

    fn project() -> TempDir {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("specs")).unwrap();
        tmp
    }

    fn write(repo: &Path, feature: &str, file: &str, text: &str) {
        let dir = repo.join("specs").join(feature);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(file), text).unwrap();
    }

    fn one(repo: &Path, feature: &str) -> OwedWorkOfSpec {
        let args = CheckOwedWorkArgs {
            feature: Some(feature.into()),
            all: false,
        };
        let result = run(&args, repo).unwrap();
        assert!(result.corpus.is_none());
        result.spec.unwrap()
    }

    fn all(repo: &Path) -> OwedWorkOfCorpus {
        let args = CheckOwedWorkArgs {
            feature: None,
            all: true,
        };
        let result = run(&args, repo).unwrap();
        assert!(result.spec.is_none());
        result.corpus.unwrap()
    }

    #[test]
    fn a_spec_reports_its_unchecked_box_and_its_unticked_criterion() {
        let tmp = project();
        write(tmp.path(), "001-owing", "spec.md", OWING_SPEC);
        write(tmp.path(), "001-owing", "tasks.md", OPEN_TASKS);
        let owed = one(tmp.path(), "001-owing");
        assert!(owed.owes);
        assert_eq!(
            owed.unchecked_boxes.unwrap(),
            vec![UncheckedBox {
                line: "- [ ] b".into(),
                count: 1
            }]
        );
        assert_eq!(
            owed.unticked_criteria.unwrap(),
            vec![UntickedCriterion {
                label: Some("AC2".into()),
                text: "AC2: two".into()
            }]
        );
        assert_eq!(
            (owed.unchecked_boxes_count, owed.unticked_criteria_count),
            (1, 1)
        );
    }

    #[test]
    fn a_spec_with_neither_owes_nothing() {
        let tmp = project();
        write(tmp.path(), "002-ticked", "spec.md", TICKED_SPEC);
        write(tmp.path(), "002-ticked", "tasks.md", SPENT_TASKS);
        let owed = one(tmp.path(), "002-ticked");
        assert!(!owed.owes, "{owed:?}");
        assert_eq!(
            (owed.unchecked_boxes_count, owed.unticked_criteria_count),
            (0, 0)
        );
    }

    /// Each half alone owes work, so neither condition is read through the other.
    #[test]
    fn a_box_alone_or_a_criterion_alone_owes_work() {
        let tmp = project();
        write(tmp.path(), "001-box", "spec.md", TICKED_SPEC);
        write(tmp.path(), "001-box", "tasks.md", OPEN_TASKS);
        write(tmp.path(), "002-criterion", "spec.md", OWING_SPEC);
        let boxed = one(tmp.path(), "001-box");
        assert!(boxed.owes);
        assert_eq!(boxed.unticked_criteria_count, 0);
        let criterion = one(tmp.path(), "002-criterion");
        assert!(criterion.owes);
        assert_eq!(criterion.unchecked_boxes_count, 0);
    }

    /// A `done` spec reopens from its state, not from an edit's diff (spec 084,
    /// AC2): an edit to its scenario's text, its data model, its prose or another
    /// file owes nothing, so every command deciding on this answer leaves it
    /// `done`, even when the edited text quotes an unchecked box; an added
    /// criterion or an appended unchecked task owes work, so it reopens.
    #[test]
    fn only_an_added_criterion_or_an_appended_task_makes_a_done_spec_owe_work() {
        let tmp = project();
        let feature = "003-done";
        let dir = tmp.path().join("specs").join(feature);
        std::fs::create_dir_all(dir.join("scenarios")).unwrap();
        write(tmp.path(), feature, "spec.md", TICKED_SPEC);
        write(tmp.path(), feature, "tasks.md", SPENT_TASKS);
        let scenario = "---\nsection: Behavior\n---\n\n# S\n\n## Context\n\nc\n";
        write(tmp.path(), feature, "scenarios/s.md", scenario);
        assert!(!one(tmp.path(), feature).owes);

        write(
            tmp.path(),
            feature,
            "scenarios/s.md",
            &format!("{scenario}\n## Behavior\n\n- [ ] a box the scenario quotes\n"),
        );
        write(
            tmp.path(),
            feature,
            "data-model.md",
            "# DM\n\n- [ ] a box the data model quotes\n",
        );
        write(
            tmp.path(),
            feature,
            "spec.md",
            &format!("{TICKED_SPEC}\n## Resolved Questions\n\n- Prose, edited.\n"),
        );
        write(
            tmp.path(),
            feature,
            "notes.md",
            "- [ ] another file's box\n",
        );
        let edited = one(tmp.path(), feature);
        assert!(
            !edited.owes,
            "no edit but a criterion or a task owes work: {edited:?}"
        );

        write(
            tmp.path(),
            feature,
            "spec.md",
            &format!("{TICKED_SPEC}- [ ] AC2: added\n"),
        );
        let criterion = one(tmp.path(), feature);
        assert!(criterion.owes);
        assert_eq!(
            (
                criterion.unchecked_boxes_count,
                criterion.unticked_criteria_count
            ),
            (0, 1)
        );

        write(tmp.path(), feature, "spec.md", TICKED_SPEC);
        write(
            tmp.path(),
            feature,
            "tasks.md",
            &format!(
                "{SPENT_TASKS}\n## 2. Scenario s\n\n- [ ] Implement the behavior described in scenarios/s.md\n"
            ),
        );
        let task = one(tmp.path(), feature);
        assert!(task.owes);
        assert_eq!(
            (task.unchecked_boxes_count, task.unticked_criteria_count),
            (1, 0)
        );
    }

    #[test]
    fn a_missing_spec_file_is_an_error_never_owing_nothing() {
        let tmp = project();
        write(tmp.path(), "001-bare", "tasks.md", SPENT_TASKS);
        let args = CheckOwedWorkArgs {
            feature: Some("001-bare".into()),
            all: false,
        };
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::MissingSpecFile { .. }),
            "{err:?}"
        );
        let args = CheckOwedWorkArgs {
            feature: Some("009-absent".into()),
            all: false,
        };
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::FeatureNotFound { .. }),
            "{err:?}"
        );
    }

    #[test]
    fn a_call_names_a_feature_or_all_never_both_or_neither() {
        let tmp = project();
        write(tmp.path(), "001-owing", "spec.md", OWING_SPEC);
        let neither = run(&CheckOwedWorkArgs::default(), tmp.path()).unwrap_err();
        assert!(
            matches!(neither, PrimitiveError::MissingArgument { .. }),
            "{neither:?}"
        );
        let both = CheckOwedWorkArgs {
            feature: Some("001-owing".into()),
            all: true,
        };
        let both = run(&both, tmp.path()).unwrap_err();
        assert!(
            matches!(both, PrimitiveError::InvalidArgument { .. }),
            "{both:?}"
        );
    }

    /// The corpus form names only the `done` specs that owe work, and counts every
    /// `done` spec it read: one owing, one clean, and an owing spec that is not
    /// `done`, which is neither named nor counted.
    #[test]
    fn the_corpus_form_names_only_the_owing_done_specs_and_counts_each_examined() {
        let tmp = project();
        write(tmp.path(), "001-owing", "spec.md", OWING_SPEC);
        write(tmp.path(), "001-owing", "tasks.md", OPEN_TASKS);
        write(tmp.path(), "002-ticked", "spec.md", TICKED_SPEC);
        write(tmp.path(), "002-ticked", "tasks.md", SPENT_TASKS);
        write(
            tmp.path(),
            "003-open",
            "spec.md",
            &OWING_SPEC.replace("status: done", "status: in-progress"),
        );
        write(tmp.path(), "003-open", "tasks.md", OPEN_TASKS);
        let corpus = all(tmp.path());
        let named: Vec<&str> = corpus
            .specs
            .as_ref()
            .unwrap()
            .iter()
            .map(|spec| spec.feature.as_str())
            .collect();
        assert_eq!(named, vec!["001-owing"]);
        assert_eq!(corpus.specs_count, 1);
        assert_eq!(corpus.examined_count, 2);
        assert_eq!(corpus.skipped_count, 0);
        let owing = &corpus.specs.unwrap()[0];
        assert_eq!(
            (owing.unchecked_boxes.len(), owing.unticked_criteria.len()),
            (1, 1)
        );
    }

    /// A spec the walk cannot judge is named with its reason, never dropped and
    /// never counted as owing nothing (`QUAL-CLAIM-001`).
    #[test]
    fn a_spec_the_corpus_form_cannot_judge_is_skipped_with_its_reason() {
        let tmp = project();
        write(
            tmp.path(),
            "001-no-status",
            "spec.md",
            "---\nfoo: [\n---\n\n# x\n",
        );
        write(tmp.path(), "002-no-frontmatter", "spec.md", "# x\n");
        std::fs::create_dir_all(tmp.path().join("specs/003-empty")).unwrap();
        write(tmp.path(), "004-binary", "spec.md", OWING_SPEC);
        std::fs::write(
            tmp.path().join("specs/004-binary/tasks.md"),
            [0xff_u8, 0xfe, 0x00],
        )
        .unwrap();
        let corpus = all(tmp.path());
        assert_eq!(corpus.examined_count, 0);
        assert_eq!(
            corpus.skipped.unwrap(),
            vec![
                OwedWorkSkip {
                    feature: "001-no-status".into(),
                    reason: OwedWorkSkipReason::StatusUnreadable
                },
                OwedWorkSkip {
                    feature: "002-no-frontmatter".into(),
                    reason: OwedWorkSkipReason::StatusUnreadable
                },
                OwedWorkSkip {
                    feature: "003-empty".into(),
                    reason: OwedWorkSkipReason::NoSpecFile
                },
                OwedWorkSkip {
                    feature: "004-binary".into(),
                    reason: OwedWorkSkipReason::Unreadable
                },
            ]
        );
    }

    /// A spec whose directory links out of the project is skipped, never read; the
    /// named form refuses it.
    #[cfg(unix)]
    #[test]
    fn a_spec_linked_out_of_the_project_is_skipped_and_refused() {
        let tmp = project();
        let outside = TempDir::new().unwrap();
        std::fs::write(outside.path().join("spec.md"), OWING_SPEC).unwrap();
        std::os::unix::fs::symlink(outside.path(), tmp.path().join("specs/001-out")).unwrap();
        let corpus = all(tmp.path());
        assert_eq!(
            corpus.skipped.unwrap(),
            vec![OwedWorkSkip {
                feature: "001-out".into(),
                reason: OwedWorkSkipReason::OutsideProject
            }]
        );
        assert_eq!((corpus.examined_count, corpus.specs_count), (0, 0));
        let args = CheckOwedWorkArgs {
            feature: Some("001-out".into()),
            all: false,
        };
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
    }

    /// An untracked spec is skipped and named, as `derive-dependencies` skips one:
    /// a draft never committed is no part of what the corpus asserts.
    #[test]
    fn an_untracked_spec_is_skipped_and_named() {
        let tmp = project();
        let repo = git2::Repository::init(tmp.path()).unwrap();
        write(tmp.path(), "001-owing", "spec.md", OWING_SPEC);
        let mut index = repo.index().unwrap();
        index
            .add_path(Path::new("specs/001-owing/spec.md"))
            .unwrap();
        index.write().unwrap();
        write(tmp.path(), "002-draft", "spec.md", OWING_SPEC);
        let corpus = all(tmp.path());
        assert_eq!(
            corpus.skipped.unwrap(),
            vec![OwedWorkSkip {
                feature: "002-draft".into(),
                reason: OwedWorkSkipReason::Untracked
            }]
        );
        assert_eq!((corpus.examined_count, corpus.specs_count), (1, 1));
    }

    /// Each shape round-trips to itself, and neither half reads the other's keys
    /// (`docs/developing-ductus.md`, *A result with two shapes*).
    #[test]
    fn each_result_shape_round_trips() {
        let tmp = project();
        write(tmp.path(), "001-owing", "spec.md", OWING_SPEC);
        write(tmp.path(), "001-owing", "tasks.md", OPEN_TASKS);
        for args in [
            CheckOwedWorkArgs {
                feature: Some("001-owing".into()),
                all: false,
            },
            CheckOwedWorkArgs {
                feature: None,
                all: true,
            },
        ] {
            let result = run(&args, tmp.path()).unwrap();
            let text = serde_json::to_string(&result).unwrap();
            let back: CheckOwedWorkResult = serde_json::from_str(&text).unwrap();
            assert_eq!(back, result, "{text}");
        }
    }

    /// Past the read size each form's lists move to the file it names, with their
    /// counts inline, so a file-form result never reads as owing nothing.
    #[test]
    fn past_the_read_size_the_lists_move_to_a_file() {
        let tmp = project();
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 100\n",
        )
        .unwrap();
        write(tmp.path(), "001-owing", "spec.md", OWING_SPEC);
        write(tmp.path(), "001-owing", "tasks.md", OPEN_TASKS);
        let args = CheckOwedWorkArgs {
            feature: Some("001-owing".into()),
            all: false,
        };
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        let shown = serde_json::to_value(&result).unwrap();
        let spec = result.spec.unwrap();
        assert!(spec.owes);
        assert!(spec.unchecked_boxes.is_none() && spec.unticked_criteria.is_none());
        assert_eq!(
            (spec.unchecked_boxes_count, spec.unticked_criteria_count),
            (1, 1)
        );
        let file = tmp.path().join(result.lists_file.unwrap());
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        assert_eq!(lists["unchecked-boxes"][0]["line"], "- [ ] b");
        assert_eq!(lists["unticked-criteria"][0]["label"], "AC2");
        // `examined-count`, the irregular count, is the corpus form's alone, so the
        // named form is held to the lists its counts name.
        let counted =
            super::super::bounded_lists::counted_lists(PRIMITIVE, shown.as_object().unwrap());
        assert_eq!(counted.len(), 2, "{counted:?}");
        for list in counted {
            assert!(lists[&list].is_array(), "no `{list}` list");
        }

        let args = CheckOwedWorkArgs {
            feature: None,
            all: true,
        };
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        let shown = serde_json::to_value(&result).unwrap();
        let corpus = result.corpus.unwrap();
        assert!(corpus.specs.is_none() && corpus.skipped.is_none());
        assert_eq!((corpus.specs_count, corpus.examined_count), (1, 1));
        let file = tmp.path().join(result.lists_file.unwrap());
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        assert_eq!(lists["specs"][0]["feature"], "001-owing");
        super::super::bounded_lists::assert_counts_name_the_file(PRIMITIVE, &shown, &lists);
    }
}
