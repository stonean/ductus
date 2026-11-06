//! `classify-spec-edit` — whether an edit under a spec reopens it (spec 079, AC15). What
//! it answers is `schema::descriptions::CLASSIFY_SPEC_EDIT`.
//!
//! Decided from the diff, since a rule an author must remember is skipped when it matters
//! most (§design-principles). Each side goes through the parser `read-spec` or
//! `read-tasks` uses, so this cannot disagree with them about what a criterion, a question
//! or a task is. Criterion ticks are not compared, as the digests normalize them
//! (`analyze_subjects::without_criterion_ticks`).
//!
//! A run's record is no edit, since `/{project}:analyze` may run on a `done` spec and leave
//! it `done`. A deleted `plan.md` or `tasks.md` is no correction, as analyze reports a
//! missing one as blocking.
//!
//! A contract edit is `mechanical` when it only adds, removes or rewrites a cross-service
//! reference, or is a sweep's: one file's diff cannot show a sweep, so a snapshot of the
//! working tree goes through `mechanical_sweep`'s repo-wide index, the rule the review
//! gate's staleness uses, so the two cannot disagree. Its blind spot comes with it: a real
//! change made with the same token in two files reads as a sweep.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use git2::{Delta, DiffOptions, Oid, Tree};

use crate::primitives::analyze_subjects::is_review_contract;
use crate::primitives::derive_references::ReferenceScrub;
use crate::primitives::fix_rounds::snapshot;
use crate::primitives::mechanical_sweep::SweepIndex;
use crate::primitives::read_spec::{parse_checkboxes, parse_open_questions};
use crate::primitives::read_tasks::parse_tasks;
use crate::primitives::{
    ANALYSIS_RECORD_FILE, PrimitiveError, ProjectRepository, REVIEW_RECORD_FILE, Result, blob_text,
    read_text, split_frontmatter,
};
use crate::schema::paths;
use crate::schema::primitives::{
    ClassifySpecEditArgs, ClassifySpecEditResult, SpecEditKind, SpecEditReason,
};

/// The ref the working tree is compared with when the caller names none.
const DEFAULT_BASE: &str = "HEAD";

/// Execute the `classify-spec-edit` primitive.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when `feature` would leave the spec root, or
/// the feature directory or a link inside it resolves outside the project;
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent;
/// [`PrimitiveError::Git`] for any libgit2 failure, a `base` that names no commit
/// and a project no work tree contains among them; and what reading or splitting
/// a changed `spec.md` or `tasks.md` returns, so an edit that could not be judged
/// never reads as a correction.
pub fn run(args: &ClassifySpecEditArgs, repo: &Path) -> Result<ClassifySpecEditResult> {
    let layout = paths::Paths::load(repo);
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
    let rel_dir = format!("{}/{}", layout.specs_root, args.feature);
    if !repo.join(&rel_dir).is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root,
            feature: args.feature.clone(),
        });
    }
    let project = ProjectRepository::discover(repo)?;
    let base = project
        .repository
        .revparse_single(args.base.as_deref().unwrap_or(DEFAULT_BASE))?
        .peel_to_commit()?
        .tree()?;
    let side = Side {
        project: &project,
        base: &base,
        repo,
        rel_dir: &rel_dir,
        scrub: ReferenceScrub::new(repo),
    };

    let mut edited = false;
    let mut reasons = BTreeSet::new();
    let mut contracts = Vec::new();
    for (file, deleted) in changed_files(&project, &rel_dir, &base)? {
        if file == REVIEW_RECORD_FILE || file == ANALYSIS_RECORD_FILE {
            continue;
        }
        edited = true;
        match file.as_str() {
            "spec.md" => spec_reasons(&side, deleted, &mut reasons)?,
            "plan.md" | "tasks.md" if deleted => {
                reasons.insert(SpecEditReason::OtherFile);
            }
            "plan.md" => {}
            "tasks.md" => tasks_reasons(&side, &mut reasons)?,
            "data-model.md" => contracts.push((file, deleted, SpecEditReason::DataModel)),
            other if is_review_contract(other) => {
                contracts.push((file, deleted, SpecEditReason::Scenario));
            }
            _ => {
                reasons.insert(SpecEditReason::OtherFile);
            }
        }
    }

    let mechanical = contract_reasons(&side, contracts, &mut reasons)?;
    let kind = if !reasons.is_empty() {
        SpecEditKind::Reopens
    } else if mechanical {
        SpecEditKind::Mechanical
    } else if edited {
        SpecEditKind::Correction
    } else {
        SpecEditKind::None
    };
    Ok(ClassifySpecEditResult {
        kind,
        reasons: reasons.into_iter().collect(),
    })
}

/// Put each contract edit beyond a mechanical one in `reasons`, and say whether any
/// was mechanical. The sweep index walks the whole diff, so it is built only when an
/// edit is more than a reference's.
fn contract_reasons(
    side: &Side<'_>,
    contracts: Vec<(String, bool, SpecEditReason)>,
    reasons: &mut BTreeSet<SpecEditReason>,
) -> Result<bool> {
    let mut mechanical = false;
    let mut sweep: Option<SweepIndex> = None;
    for (file, deleted, reason) in contracts {
        if !deleted && side.only_references_differ(&file)? {
            mechanical = true;
            continue;
        }
        let index = match &sweep {
            Some(index) => index,
            None => sweep.insert(side.sweep_index()?),
        };
        let git_path = side.project.to_git(&format!("{}/{file}", side.rel_dir));
        if index.changed_beyond_spelling(&git_path) {
            reasons.insert(reason);
        } else {
            mechanical = true;
        }
    }
    Ok(mechanical)
}

/// Where one file under the spec directory reads from on each side of the edit.
struct Side<'a> {
    project: &'a ProjectRepository,
    base: &'a Tree<'a>,
    repo: &'a Path,
    rel_dir: &'a str,
    scrub: ReferenceScrub,
}

impl Side<'_> {
    /// `file`'s text at the base. `None` when it is absent there or not UTF-8.
    fn at_base(&self, file: &str) -> Result<Option<String>> {
        let git_path = self.project.to_git(&format!("{}/{file}", self.rel_dir));
        blob_text(&self.project.repository, self.base, &git_path)
    }

    /// `file`'s text in the working tree.
    fn now(&self, file: &str) -> Result<String> {
        read_text(&self.repo.join(self.rel_dir).join(file))
    }

    /// Whether `file` is the same on both sides once its references are out, a line
    /// left holding nothing or only a list marker counting as none.
    fn only_references_differ(&self, file: &str) -> Result<bool> {
        let Some(at_base) = self.at_base(file)? else {
            return Ok(false);
        };
        let now = self.now(file)?;
        let lines = |text: &str| -> Vec<String> {
            self.scrub
                .without_references(text)
                .lines()
                .map(str::trim_end)
                .filter(|line| !matches!(line.trim_start(), "" | "-" | "*" | "+"))
                .map(str::to_string)
                .collect()
        };
        Ok(lines(&at_base) == lines(&now))
    }

    /// The sweep rule over the diff from the base to a snapshot of the working tree.
    fn sweep_index(&self) -> Result<SweepIndex> {
        let repository = &self.project.repository;
        let (snapshot, _) = snapshot(self.project)?;
        let head = repository.find_commit(Oid::from_str(&snapshot)?)?.tree()?;
        Ok(SweepIndex::build(repository, self.base, &head))
    }
}

/// Each file under `rel_dir` that differs between `base` and the working tree,
/// from the feature directory, with whether it is deleted. An untracked file
/// counts: a scenario written by hand and not yet committed is the reopen this
/// exists to see. The pathspec is literal, since the project's path may hold a
/// glob character, which would match no file and read as no edit
/// (`QUAL-CLAIM-001`).
fn changed_files(
    project: &ProjectRepository,
    rel_dir: &str,
    base: &Tree<'_>,
) -> Result<BTreeMap<String, bool>> {
    let mut options = DiffOptions::new();
    options
        .pathspec(project.to_git(rel_dir))
        .disable_pathspec_match(true)
        .include_untracked(true)
        .recurse_untracked_dirs(true);
    let diff = project
        .repository
        .diff_tree_to_workdir_with_index(Some(base), Some(&mut options))?;
    let prefix = format!("{rel_dir}/");
    let mut files = BTreeMap::new();
    for delta in diff.deltas() {
        let Some(path) = delta.new_file().path().or_else(|| delta.old_file().path()) else {
            continue;
        };
        let path = path.to_string_lossy().replace('\\', "/");
        if let Some(file) = project
            .to_project(&path)
            .and_then(|inside| inside.strip_prefix(&prefix))
        {
            files.insert(file.to_string(), delta.status() == Delta::Deleted);
        }
    }
    Ok(files)
}

/// Why a `spec.md` edit reopens: a `spec.md` absent on either side is not a
/// correction to it, and otherwise what its criteria and questions say decides.
fn spec_reasons(
    side: &Side<'_>,
    deleted: bool,
    reasons: &mut BTreeSet<SpecEditReason>,
) -> Result<()> {
    let at_base = side.at_base("spec.md")?;
    let (Some(at_base), false) = (at_base, deleted) else {
        reasons.insert(SpecEditReason::OtherFile);
        return Ok(());
    };
    let now = side.now("spec.md")?;
    let path = Path::new("spec.md");
    let (_, base_body) = split_frontmatter(&at_base, path)?;
    let (_, now_body) = split_frontmatter(&now, path)?;
    // A reference is informative, so one in a criterion or question changes neither.
    let base_body = side.scrub.without_references(base_body);
    let now_body = side.scrub.without_references(now_body);
    if criterion_texts(&base_body) != criterion_texts(&now_body) {
        reasons.insert(SpecEditReason::Criterion);
    }
    if question_texts(&base_body) != question_texts(&now_body) {
        reasons.insert(SpecEditReason::OpenQuestion);
    }
    Ok(())
}

/// What the criteria say, sorted, so moving one changes nothing, and with each
/// `AC{n}:` label removed, so assigning a label stays a correction (AC15).
fn criterion_texts(body: &str) -> Vec<String> {
    let mut texts: Vec<String> = parse_checkboxes(body, "Acceptance Criteria")
        .into_iter()
        .map(|criterion| {
            let unlabelled = criterion
                .label
                .as_deref()
                .and_then(|label| criterion.text.strip_prefix(label))
                .and_then(|rest| rest.strip_prefix(':'))
                .map(str::trim_start);
            unlabelled.map_or_else(|| criterion.text.clone(), str::to_string)
        })
        .collect();
    texts.sort();
    texts
}

/// What the open questions say, sorted.
fn question_texts(body: &str) -> Vec<String> {
    let mut texts: Vec<String> = parse_open_questions(body, "Open Questions")
        .into_iter()
        .map(|question| question.text)
        .collect();
    texts.sort();
    texts
}

/// Why a `tasks.md` edit reopens: an unchecked item the base does not hold, so
/// one added, reworded or unticked. Ticking or removing one adds none, which is
/// what `/{project}:prune` does.
fn tasks_reasons(side: &Side<'_>, reasons: &mut BTreeSet<SpecEditReason>) -> Result<()> {
    let at_base = unchecked_items(&side.at_base("tasks.md")?.unwrap_or_default());
    let now = unchecked_items(&side.now("tasks.md")?);
    if now
        .iter()
        .any(|(item, count)| at_base.get(item).copied().unwrap_or(0) < *count)
    {
        reasons.insert(SpecEditReason::UncheckedTask);
    }
    Ok(())
}

/// Each unchecked box with its task's heading, which carries no number, so
/// renumbering a task changes nothing. A checkbox-form `Done when` counts, since
/// a reader sees its box.
pub(crate) fn unchecked_items(content: &str) -> BTreeMap<(String, String), usize> {
    let mut items = BTreeMap::new();
    for task in parse_tasks(content) {
        let clause = task
            .done_when
            .filter(|_| task.done_when_checked == Some(false));
        let subtasks = task
            .subtasks
            .into_iter()
            .filter(|subtask| !subtask.checked)
            .map(|subtask| subtask.text);
        for text in subtasks.chain(clause) {
            *items.entry((task.heading.clone(), text)).or_insert(0) += 1;
        }
    }
    items
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, subdirectory_project, write};
    use git2::Repository;
    use std::path::PathBuf;

    const SPEC: &str = "---\nstatus: done\ndependencies: []\nnext-criterion: 3\n---\n\n# 012\n\n\
        Prose.\n\n## Acceptance Criteria\n\n- [x] AC1: First thing.\n- [x] AC2: Second thing.\n\n\
        ## Open Questions\n\n*None — all resolved.*\n\n## Resolved Questions\n\n- **Why?** Because.\n";
    const TASKS: &str =
        "# 012 Tasks\n\n## 1. Build\n\n- [x] The thing\n\n- **Done when**: built.\n";
    const FEATURE: &str = "012-a";

    /// A project holding spec 012 with every artifact committed, and a sibling.
    fn project(root: &Path) -> Repository {
        let repository = Repository::init(root).unwrap();
        for (file, body) in [
            ("spec.md", SPEC),
            ("plan.md", "# Plan\n\nThe approach.\n"),
            ("tasks.md", TASKS),
            ("data-model.md", "# Data\n\nA shape.\n"),
            ("research.md", "# Research\n\nA note.\n"),
            ("scenarios/s.md", "---\nsection: x\n---\n\n# s\n"),
        ] {
            write(&root.join("specs").join(FEATURE).join(file), body);
        }
        write(&root.join("specs/034-b/spec.md"), SPEC);
        commit_all(&repository, "chore: seed");
        repository
    }

    fn edit(root: &Path, file: &str, body: &str) {
        write(&root.join("specs").join(FEATURE).join(file), body);
    }

    fn remove(root: &Path, file: &str) {
        std::fs::remove_file(root.join("specs").join(FEATURE).join(file)).unwrap();
    }

    fn classify_from(root: &Path, base: Option<&str>) -> Result<ClassifySpecEditResult> {
        run(
            &ClassifySpecEditArgs {
                feature: FEATURE.into(),
                base: base.map(str::to_string),
            },
            root,
        )
    }

    fn classify(root: &Path) -> ClassifySpecEditResult {
        classify_from(root, None).unwrap()
    }

    fn assert_reopens(root: &Path, expected: &[SpecEditReason]) {
        let result = classify(root);
        assert_eq!(result.kind, SpecEditKind::Reopens);
        assert_eq!(result.reasons, expected);
    }

    fn assert_kind(root: &Path, expected: SpecEditKind) {
        let result = classify(root);
        assert_eq!(result.kind, expected, "{:?}", result.reasons);
    }

    /// A contract and a sibling spec's file holding `old-label`, committed, so a sweep
    /// can rewrite it across files.
    fn swept_project(root: &Path) {
        let repository = Repository::open(root).unwrap();
        edit(
            root,
            "scenarios/s.md",
            "---\nsection: x\n---\n\n# s\n\nUses `old-label`.\n",
        );
        edit(root, "data-model.md", "# Data\n\nUses `old-label`.\n");
        write(
            &root.join("specs/034-b/data-model.md"),
            "# Data\n\nUses `old-label`.\n",
        );
        commit_all(&repository, "chore: the label");
    }

    #[test]
    fn a_token_swept_across_files_is_mechanical() {
        let (_dir, root) = fixture();
        swept_project(&root);
        edit(
            &root,
            "scenarios/s.md",
            "---\nsection: x\n---\n\n# s\n\nUses `new-label`.\n",
        );
        edit(&root, "data-model.md", "# Data\n\nUses `new-label`.\n");
        write(
            &root.join("specs/034-b/data-model.md"),
            "# Data\n\nUses `new-label`.\n",
        );
        assert_kind(&root, SpecEditKind::Mechanical);
    }

    /// One file's token edit is how a real change looks, so it needs a second file.
    #[test]
    fn a_token_changed_in_one_file_reopens() {
        let (_dir, root) = fixture();
        swept_project(&root);
        edit(&root, "data-model.md", "# Data\n\nUses `new-label`.\n");
        assert_reopens(&root, &[SpecEditReason::DataModel]);
    }

    /// A sweep does not excuse the real edit beside it.
    #[test]
    fn a_real_contract_edit_beside_a_sweep_reopens() {
        let (_dir, root) = fixture();
        swept_project(&root);
        edit(
            &root,
            "scenarios/s.md",
            "---\nsection: x\n---\n\n# s\n\nUses `new-label`.\n",
        );
        write(
            &root.join("specs/034-b/data-model.md"),
            "# Data\n\nUses `new-label`.\n",
        );
        edit(
            &root,
            "data-model.md",
            "# Data\n\nUses `new-label`, and a new claim.\n",
        );
        assert_reopens(&root, &[SpecEditReason::DataModel]);
    }

    /// `mechanical` still gives way to what else the edit reopens.
    #[test]
    fn a_sweep_beside_a_changed_criterion_reopens() {
        let (_dir, root) = fixture();
        swept_project(&root);
        edit(
            &root,
            "scenarios/s.md",
            "---\nsection: x\n---\n\n# s\n\nUses `new-label`.\n",
        );
        edit(&root, "data-model.md", "# Data\n\nUses `new-label`.\n");
        edit(&root, "spec.md", &SPEC.replace("First thing.", "First."));
        assert_reopens(&root, &[SpecEditReason::Criterion]);
    }

    const LINK: &str = "[api](https://example.invalid/acme/api/blob/main/specs/003-user/spec.md)";

    /// A contract holding a line that names a reference, committed.
    fn linked_project(root: &Path) {
        let repository = Repository::open(root).unwrap();
        let body = format!("---\nsection: x\n---\n\n# s\n\nSee {LINK} for it.\n\nProse.\n");
        edit(root, "scenarios/s.md", &body);
        commit_all(&repository, "chore: a reference");
    }

    #[test]
    fn a_rewritten_reference_is_mechanical() {
        let (_dir, root) = fixture();
        linked_project(&root);
        let moved = LINK.replace("/acme/api/", "/acme/users/");
        let body = format!("---\nsection: x\n---\n\n# s\n\nSee {moved} for it.\n\nProse.\n");
        edit(&root, "scenarios/s.md", &body);
        assert_kind(&root, SpecEditKind::Mechanical);
    }

    /// Added as its own list item, as a reference usually is, and taken away again.
    #[test]
    fn a_reference_added_or_removed_as_a_list_item_is_mechanical() {
        let (_dir, root) = fixture();
        linked_project(&root);
        let with_item =
            format!("---\nsection: x\n---\n\n# s\n\nSee {LINK} for it.\n\nProse.\n\n- {LINK}\n");
        edit(&root, "scenarios/s.md", &with_item);
        assert_kind(&root, SpecEditKind::Mechanical);
    }

    /// Prose beside the link is a claim, whatever the link does.
    #[test]
    fn prose_changed_beside_a_reference_reopens() {
        let (_dir, root) = fixture();
        linked_project(&root);
        let body = format!("---\nsection: x\n---\n\n# s\n\nSee {LINK} for that.\n\nProse.\n");
        edit(&root, "scenarios/s.md", &body);
        assert_reopens(&root, &[SpecEditReason::Scenario]);
    }

    /// A relative link is a sibling spec's, the blocking kind, never a reference.
    #[test]
    fn a_relative_link_is_no_reference() {
        let (_dir, root) = fixture();
        let repository = Repository::open(&root).unwrap();
        let body = "---\nsection: x\n---\n\n# s\n\nSee [b](../../034-b/spec.md).\n";
        edit(&root, "scenarios/s.md", body);
        commit_all(&repository, "chore: a sibling link");
        edit(
            &root,
            "scenarios/s.md",
            "---\nsection: x\n---\n\n# s\n\nSee [b](../../035-c/spec.md).\n",
        );
        assert_reopens(&root, &[SpecEditReason::Scenario]);
    }

    /// A reference is informative wherever it sits, a criterion included.
    #[test]
    fn a_reference_rewritten_in_a_criterion_is_a_correction() {
        let (_dir, root) = fixture();
        let repository = Repository::open(&root).unwrap();
        edit(
            &root,
            "spec.md",
            &SPEC.replace("First thing.", &format!("First, see {LINK}.")),
        );
        commit_all(&repository, "chore: a reference in a criterion");
        let moved = LINK.replace("/acme/api/", "/acme/users/");
        edit(
            &root,
            "spec.md",
            &SPEC.replace("First thing.", &format!("First, see {moved}.")),
        );
        assert_correction(&root);
    }

    fn assert_correction(root: &Path) {
        assert_eq!(
            classify(root),
            ClassifySpecEditResult {
                kind: SpecEditKind::Correction,
                reasons: Vec::new(),
            }
        );
    }

    /// A temporary project with spec 012 committed. The guard keeps it alive.
    fn fixture() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().to_path_buf();
        project(&root);
        (dir, root)
    }

    /// A temporary project holding only `files` under spec 012, committed.
    fn committed(files: &[(&str, &str)]) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let repository = Repository::init(&root).unwrap();
        for (file, body) in files {
            edit(&root, file, body);
        }
        commit_all(&repository, "chore: seed");
        (dir, root)
    }

    #[test]
    fn nothing_changed_is_none() {
        let (_dir, root) = fixture();
        assert_eq!(
            classify(&root),
            ClassifySpecEditResult {
                kind: SpecEditKind::None,
                reasons: Vec::new(),
            }
        );
    }

    #[test]
    fn spec_body_prose_is_a_correction() {
        let (_dir, root) = fixture();
        edit(
            &root,
            "spec.md",
            &SPEC.replace("Prose.", "Corrected prose."),
        );
        assert_correction(&root);
    }

    #[test]
    fn a_resolved_question_edit_is_a_correction() {
        let (_dir, root) = fixture();
        edit(&root, "spec.md", &SPEC.replace("Because.", "Since."));
        assert_correction(&root);
    }

    #[test]
    fn a_plan_edit_is_a_correction() {
        let (_dir, root) = fixture();
        edit(&root, "plan.md", "# Plan\n\nThe corrected approach.\n");
        assert_correction(&root);
    }

    #[test]
    fn prose_and_plan_together_are_a_correction() {
        let (_dir, root) = fixture();
        edit(
            &root,
            "spec.md",
            &SPEC.replace("Prose.", "Corrected prose."),
        );
        edit(&root, "plan.md", "# Plan\n\nThe corrected approach.\n");
        assert_correction(&root);
    }

    #[test]
    fn labelling_is_a_correction() {
        let unlabelled = SPEC
            .replace("next-criterion: 3\n", "")
            .replace("AC1: ", "")
            .replace("AC2: ", "");
        let (_dir, root) = committed(&[("spec.md", &unlabelled)]);
        edit(&root, "spec.md", SPEC);
        assert_correction(&root);
    }

    #[test]
    fn a_tick_either_way_is_a_correction() {
        let (_dir, root) = fixture();
        edit(&root, "spec.md", &SPEC.replace("- [x] AC2", "- [ ] AC2"));
        assert_correction(&root);
    }

    #[test]
    fn moving_an_open_question_is_a_correction() {
        let asked = |questions: &str| SPEC.replace("*None — all resolved.*", questions);
        let (_dir, root) = committed(&[("spec.md", &asked("- Should this?\n- Must that?"))]);
        edit(&root, "spec.md", &asked("- Must that?\n- Should this?"));
        assert_correction(&root);
    }

    #[test]
    fn moving_a_criterion_is_a_correction() {
        let (_dir, root) = fixture();
        let moved = SPEC.replace(
            "- [x] AC1: First thing.\n- [x] AC2: Second thing.\n",
            "- [x] AC2: Second thing.\n- [x] AC1: First thing.\n",
        );
        edit(&root, "spec.md", &moved);
        assert_correction(&root);
    }

    #[test]
    fn rewriting_every_line_ending_is_a_correction() {
        let (_dir, root) = fixture();
        edit(&root, "spec.md", &SPEC.replace('\n', "\r\n"));
        assert_correction(&root);
    }

    #[test]
    fn a_changed_criterion_reopens() {
        let (_dir, root) = fixture();
        edit(&root, "spec.md", &SPEC.replace("First thing.", "First."));
        assert_reopens(&root, &[SpecEditReason::Criterion]);
    }

    #[test]
    fn an_added_criterion_reopens() {
        let (_dir, root) = fixture();
        let added = SPEC.replace(
            "- [x] AC2: Second thing.\n",
            "- [x] AC2: Second thing.\n- [ ] Third thing.\n",
        );
        edit(&root, "spec.md", &added);
        assert_reopens(&root, &[SpecEditReason::Criterion]);
    }

    #[test]
    fn a_removed_criterion_reopens() {
        let (_dir, root) = fixture();
        edit(
            &root,
            "spec.md",
            &SPEC.replace("- [x] AC2: Second thing.\n", ""),
        );
        assert_reopens(&root, &[SpecEditReason::Criterion]);
    }

    #[test]
    fn an_added_question_reopens() {
        let (_dir, root) = fixture();
        let added = SPEC.replace("*None — all resolved.*", "- Should this change?");
        edit(&root, "spec.md", &added);
        assert_reopens(&root, &[SpecEditReason::OpenQuestion]);
    }

    #[test]
    fn a_changed_question_reopens() {
        let asked = SPEC.replace("*None — all resolved.*", "- Should this change?");
        let (_dir, root) = committed(&[("spec.md", &asked)]);
        edit(
            &root,
            "spec.md",
            &asked.replace("Should this change?", "Must this change?"),
        );
        assert_reopens(&root, &[SpecEditReason::OpenQuestion]);
    }

    #[test]
    fn an_added_unchecked_task_reopens() {
        let (_dir, root) = fixture();
        let added = format!("{TASKS}\n## 2. More\n\n- [ ] The other thing\n");
        edit(&root, "tasks.md", &added);
        assert_reopens(&root, &[SpecEditReason::UncheckedTask]);
    }

    #[test]
    fn unticking_a_task_reopens() {
        let (_dir, root) = fixture();
        edit(&root, "tasks.md", &TASKS.replace("- [x]", "- [ ]"));
        assert_reopens(&root, &[SpecEditReason::UncheckedTask]);
    }

    #[test]
    fn a_new_unchecked_clause_reopens() {
        let (_dir, root) = fixture();
        let added = format!("{TASKS}\n## 2. More\n\n- [ ] Done when: more is done.\n");
        edit(&root, "tasks.md", &added);
        assert_reopens(&root, &[SpecEditReason::UncheckedTask]);
    }

    /// Two boxes with the same text under one task are two items, so a second
    /// one reopens though the text is already pending.
    #[test]
    fn a_repeated_unchecked_item_reopens() {
        let pending = "# Tasks\n\n## 1. Build\n\n- [ ] The thing\n";
        let (_dir, root) = committed(&[("spec.md", SPEC), ("tasks.md", pending)]);
        edit(&root, "tasks.md", &format!("{pending}- [ ] The thing\n"));
        assert_reopens(&root, &[SpecEditReason::UncheckedTask]);
    }

    #[test]
    fn a_task_edit_adding_a_checked_item_is_a_correction() {
        let (_dir, root) = fixture();
        let added = TASKS.replace("- [x] The thing\n", "- [x] The thing\n- [x] Another\n");
        edit(&root, "tasks.md", &added);
        assert_correction(&root);
    }

    /// Pending work reworded, in its item or its heading, is changed work.
    #[test]
    fn a_reworded_pending_task_reopens() {
        for reworded in [
            "# Tasks\n\n## 1. Build\n\n- [ ] A thing\n",
            "# Tasks\n\n## 1. Make\n\n- [ ] The thing\n",
        ] {
            let (_dir, root) = committed(&[
                ("spec.md", SPEC),
                ("tasks.md", "# Tasks\n\n## 1. Build\n\n- [ ] The thing\n"),
            ]);
            edit(&root, "tasks.md", reworded);
            assert_reopens(&root, &[SpecEditReason::UncheckedTask]);
        }
    }

    #[test]
    fn a_task_edit_adding_no_unchecked_task_is_a_correction() {
        let (_dir, root) = fixture();
        edit(&root, "tasks.md", &TASKS.replace("built.", "it is built."));
        assert_correction(&root);
    }

    #[test]
    fn ticking_renumbering_or_pruning_a_pending_task_is_a_correction() {
        let pending = "# Tasks\n\n## 1. Build\n\n- [ ] The thing\n\n## 2. Ship\n\n- [ ] It\n";
        for edited in [
            "# Tasks\n\n## 1. Build\n\n- [x] The thing\n\n## 2. Ship\n\n- [ ] It\n",
            "# Tasks\n\n## 3. Build\n\n- [ ] The thing\n\n## 2. Ship\n\n- [ ] It\n",
            "# Tasks\n\n## 2. Ship\n\n- [ ] It\n",
        ] {
            let (_dir, root) = committed(&[("spec.md", SPEC), ("tasks.md", pending)]);
            edit(&root, "tasks.md", edited);
            assert_correction(&root);
        }
    }

    #[test]
    fn a_scenario_edit_reopens() {
        let (_dir, root) = fixture();
        edit(&root, "scenarios/s.md", "---\nsection: y\n---\n\n# s\n");
        assert_reopens(&root, &[SpecEditReason::Scenario]);
    }

    #[test]
    fn an_untracked_scenario_reopens() {
        let (_dir, root) = fixture();
        edit(&root, "scenarios/new.md", "---\nsection: y\n---\n\n# new\n");
        assert_reopens(&root, &[SpecEditReason::Scenario]);
    }

    #[test]
    fn an_untracked_scenario_in_a_new_directory_reopens() {
        let (_dir, root) = committed(&[("spec.md", SPEC)]);
        edit(&root, "scenarios/new.md", "---\nsection: y\n---\n\n# new\n");
        assert_reopens(&root, &[SpecEditReason::Scenario]);
    }

    #[test]
    fn a_deleted_scenario_reopens() {
        let (_dir, root) = fixture();
        remove(&root, "scenarios/s.md");
        assert_reopens(&root, &[SpecEditReason::Scenario]);
    }

    #[test]
    fn a_data_model_edit_reopens() {
        let (_dir, root) = fixture();
        edit(&root, "data-model.md", "# Data\n\nAnother shape.\n");
        assert_reopens(&root, &[SpecEditReason::DataModel]);
    }

    #[test]
    fn another_file_reopens() {
        let (_dir, root) = fixture();
        edit(&root, "research.md", "# Research\n\nAnother note.\n");
        assert_reopens(&root, &[SpecEditReason::OtherFile]);
    }

    #[test]
    fn a_record_write_alone_is_none() {
        let (_dir, root) = fixture();
        edit(&root, "review.md", "---\nspec: 012-a\n---\n");
        edit(&root, "analysis.md", "---\nspec: 012-a\n---\n");
        assert_eq!(classify(&root).kind, SpecEditKind::None);
    }

    #[test]
    fn a_record_write_beside_a_correction_stays_a_correction() {
        let (_dir, root) = fixture();
        edit(&root, "analysis.md", "---\nspec: 012-a\n---\n");
        edit(
            &root,
            "spec.md",
            &SPEC.replace("Prose.", "Corrected prose."),
        );
        assert_correction(&root);
    }

    #[test]
    fn a_deleted_plan_or_tasks_reopens() {
        for file in ["plan.md", "tasks.md"] {
            let (_dir, root) = fixture();
            remove(&root, file);
            assert_reopens(&root, &[SpecEditReason::OtherFile]);
        }
    }

    #[test]
    fn a_deleted_spec_reopens() {
        let (_dir, root) = fixture();
        remove(&root, "spec.md");
        assert_reopens(&root, &[SpecEditReason::OtherFile]);
    }

    #[test]
    fn a_spec_new_since_the_base_reopens() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let repository = Repository::init(root).unwrap();
        write(&root.join("README.md"), "seed\n");
        commit_all(&repository, "chore: seed");
        edit(root, "spec.md", SPEC);
        assert_reopens(root, &[SpecEditReason::OtherFile]);
    }

    #[test]
    fn reasons_come_in_one_order_without_repeats() {
        let (_dir, root) = fixture();
        edit(&root, "research.md", "# Research\n\nSome note.\n");
        edit(&root, "data-model.md", "# Data\n\nAnother shape.\n");
        edit(
            &root,
            "scenarios/s.md",
            "---\nsection: x\n---\n\n# s\n\nAdded.\n",
        );
        edit(&root, "scenarios/t.md", "---\nsection: y\n---\n\n# t\n");
        edit(&root, "tasks.md", &TASKS.replace("- [x]", "- [ ]"));
        let both = SPEC
            .replace("First thing.", "First.")
            .replace("*None — all resolved.*", "- Should this change?");
        edit(&root, "spec.md", &both);
        assert_reopens(
            &root,
            &[
                SpecEditReason::Criterion,
                SpecEditReason::OpenQuestion,
                SpecEditReason::UncheckedTask,
                SpecEditReason::Scenario,
                SpecEditReason::DataModel,
                SpecEditReason::OtherFile,
            ],
        );
    }

    #[test]
    fn another_specs_edit_is_not_this_ones() {
        let (_dir, root) = fixture();
        write(&root.join("specs/034-b/spec.md"), "changed\n");
        write(&root.join("specs/012-ab/spec.md"), "a longer neighbour\n");
        write(&root.join("runtime/lib.rs"), "// code\n");
        assert_eq!(classify(&root).kind, SpecEditKind::None);
    }

    /// The default base is `HEAD`: a committed edit is nothing to classify until
    /// `base` reaches behind it.
    #[test]
    fn base_decides_where_the_comparison_starts() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let repository = project(root);
        let before = repository.head().unwrap().peel_to_commit().unwrap().id();
        edit(root, "spec.md", &SPEC.replace("First thing.", "First."));
        commit_all(&repository, "chore: change a criterion");
        assert_eq!(classify(root).kind, SpecEditKind::None);
        let result = classify_from(root, Some(&before.to_string())).unwrap();
        assert_eq!(result.kind, SpecEditKind::Reopens);
        assert_eq!(result.reasons, vec![SpecEditReason::Criterion]);
    }

    #[test]
    fn a_base_naming_no_commit_is_an_error() {
        let (_dir, root) = fixture();
        assert!(matches!(
            classify_from(&root, Some("no-such-ref")),
            Err(PrimitiveError::Git(_))
        ));
    }

    #[test]
    fn a_spec_that_does_not_split_is_an_error_not_a_correction() {
        let (_dir, root) = fixture();
        edit(&root, "spec.md", "no frontmatter\n");
        assert!(matches!(
            classify_from(&root, None),
            Err(PrimitiveError::MissingFrontmatter { .. })
        ));
    }

    #[test]
    fn a_feature_that_is_absent_or_escapes_is_refused() {
        let (_dir, root) = fixture();
        let attempt = |feature: &str| {
            run(
                &ClassifySpecEditArgs {
                    feature: feature.into(),
                    base: None,
                },
                &root,
            )
        };
        assert!(matches!(
            attempt("999-none"),
            Err(PrimitiveError::FeatureNotFound { .. })
        ));
        assert!(matches!(
            attempt("../../etc"),
            Err(PrimitiveError::InvalidPath { .. })
        ));
    }

    #[test]
    fn a_project_in_a_subdirectory_reads_its_own_paths() {
        let dir = tempfile::tempdir().unwrap();
        let (repository, project_root) = subdirectory_project(dir.path());
        edit(&project_root, "spec.md", SPEC);
        edit(&project_root, "tasks.md", TASKS);
        commit_all(&repository, "chore: seed");
        assert_eq!(classify(&project_root).kind, SpecEditKind::None);

        edit(
            &project_root,
            "spec.md",
            &SPEC.replace("Prose.", "Corrected prose."),
        );
        assert_correction(&project_root);

        edit(
            &project_root,
            "spec.md",
            &SPEC.replace("First thing.", "First."),
        );
        edit(&project_root, "tasks.md", &TASKS.replace("- [x]", "- [ ]"));
        assert_reopens(
            &project_root,
            &[SpecEditReason::Criterion, SpecEditReason::UncheckedTask],
        );
    }
}
