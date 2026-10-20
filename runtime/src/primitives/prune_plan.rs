//! `prune-plan` — find the `plan.md` sections outside the design record, and
//! remove the ones the host lists.
//!
//! A plan records the design as it stands (§plan-phase), and its design
//! record is the plan template's own `##` sections, compared ASCII
//! case-insensitively ([`DESIGN_RECORD`]). A preview reports every other
//! `##` section with its size, a digest, the advisory `/{project}:analyze`
//! records for it, and whether `analysis.md` already stores a discard for
//! that advisory. An apply removes the sections the host lists by heading
//! and digest, refusing the whole write when any of them changed since the
//! preview, because the host judged its moves against the text it read.
//!
//! Which pieces of a section last, and where each goes, is the host's
//! judgment. Everything here is deterministic, and no section's text ever
//! leaves the runtime.

use std::collections::HashMap;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::primitives::decisions::{RawDecision, read_decisions, same_key};
use crate::primitives::prune_tasks::{one_feature_or_all, read_status, size_of};
use crate::primitives::write_analysis::finding_key_of;
use crate::primitives::{
    ANALYSIS_RECORD_FILE, PrimitiveError, ProjectRepository, Result, SkipScanner, checkbox, hex,
    join_blocks, line_ending_of, list_feature_dirs, parse_atx_heading, read_text, rel_path,
    write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    DecisionOutcome, PlanFinding, PlanRemoval, PlanSection, PrunePlanArgs, PrunePlanResult,
    PrunePlanSummary, PruneWalk, PruneWalkEntry, SkipReason, SkippedFeature,
};

/// The plan template's `##` headings, in template order — the design record.
/// A unit test ([`tests::design_record_matches_the_plan_template`]) holds this
/// to `framework/templates/spec/plan.md`, as `prune-tasks` holds its reset
/// body to the tasks template, so an adopter who customizes the template sees
/// their own set only on the markdown-only path.
const DESIGN_RECORD: [&str; 7] = [
    "Overview",
    "Technical Decisions",
    "Affected Files",
    "Data Model",
    "Trade-offs",
    "Open Questions Resolved",
    "Cross-spec impact",
];

/// The analyze family a section outside the design record is reported under.
const FAMILY: &str = "plan-record";

/// Whether `heading` names a design-record section.
fn is_design_record(heading: &str) -> bool {
    let heading = heading.trim();
    DESIGN_RECORD
        .iter()
        .any(|record| record.eq_ignore_ascii_case(heading))
}

/// The advisory for a section outside the design record. The one function
/// that builds it, so the finding `/{project}:analyze` records and the key
/// prune looks its stored decision up by cannot disagree.
pub(crate) fn plan_section_finding(heading: &str) -> PlanFinding {
    PlanFinding {
        family: FAMILY.to_string(),
        message: format!("plan.md §{heading} is outside the design record"),
    }
}

/// One segmented block of a plan: the preamble, a level-1 heading group, or
/// a `##` section.
struct Block {
    /// The heading of a `##` section; `None` for the preamble and a level-1
    /// heading group, which are always kept.
    heading: Option<String>,
    lines: Vec<String>,
}

impl Block {
    /// The section's text: its lines from the heading to the last non-blank
    /// line, joined by `\n`. The digest, the size, and the reopen comparison
    /// all read this, so a blank line at a seam never changes any of them.
    fn text(&self) -> String {
        let end = self
            .lines
            .iter()
            .rposition(|line| !line.trim().is_empty())
            .map_or(0, |last| last + 1);
        self.lines[..end].join("\n")
    }
}

/// Segment `content` into blocks. A block opens at every heading of level 2
/// or above outside a fenced block or an HTML comment, so a `##` in either is
/// not structure, and a `###` stays inside the section that holds it.
fn segment(content: &str) -> Vec<Block> {
    let mut skip = SkipScanner::default();
    let mut blocks = vec![Block {
        heading: None,
        lines: Vec::new(),
    }];
    for line in content.lines() {
        // `skip` runs first on every line, so the scanner advances in order.
        if !skip.skip(line)
            && let Some((level, heading)) = parse_atx_heading(line)
            && level <= 2
        {
            blocks.push(Block {
                heading: (level == 2).then_some(heading),
                lines: vec![line.to_string()],
            });
            continue;
        }
        if let Some(block) = blocks.last_mut() {
            block.lines.push(line.to_string());
        }
    }
    blocks
}

/// The design-record sections of `content`, in order, as lowercased heading
/// and text — the subject of the reopen trigger's plan half.
fn design_record(content: &str) -> Vec<(String, String)> {
    segment(content)
        .iter()
        .filter_map(|block| {
            let heading = block.heading.as_deref()?;
            is_design_record(heading).then(|| (heading.trim().to_ascii_lowercase(), block.text()))
        })
        .collect()
}

/// Each unchecked task-list checkbox line in `content`, outside fenced blocks
/// and HTML comments, trimmed, with how many times it occurs — the subject of
/// the reopen trigger's tasks half.
fn unchecked_lines(content: &str) -> HashMap<&str, usize> {
    let mut skip = SkipScanner::default();
    let mut lines = HashMap::new();
    for line in content.lines().filter(|line| !skip.skip(line)) {
        if checkbox::find_checkbox_line(line)
            .is_some_and(|(_bracket, marker)| line.as_bytes()[marker] == b' ')
        {
            *lines.entry(line.trim()).or_insert(0) += 1;
        }
    }
    lines
}

/// Whether `now` adds an unchecked checkbox to `head`: holds an unchecked
/// checkbox line more times than `head` does. A line moved, or only
/// re-indented, adds nothing; a line added, or reworded, does.
fn adds_unchecked_box(now: &str, head: &str) -> bool {
    let before = unchecked_lines(head);
    unchecked_lines(now)
        .into_iter()
        .any(|(line, count)| count > before.get(line).copied().unwrap_or(0))
}

/// Execute the `prune-plan` primitive against the given project root: one
/// feature's preview or apply, or every feature's preview under `all`.
///
/// # Errors
///
/// - [`PrimitiveError::MissingArgument`] when neither `feature` nor `all` is
///   given, or when `apply` lists nothing to remove.
/// - [`PrimitiveError::InvalidArgument`] when both `feature` and `all` are
///   given, when `all` comes with `apply`, and when `remove` is given without
///   `apply` or names a design-record section.
/// - [`PrimitiveError::InvalidPath`] when `feature` would escape the spec root,
///   or when the feature directory, or a link inside it, resolves outside the
///   project.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory is absent.
/// - [`PrimitiveError::MissingSpecFile`] / [`PrimitiveError::StatusFieldMissing`]
///   / [`PrimitiveError::MissingFrontmatter`] /
///   [`PrimitiveError::UnclosedFrontmatter`] / [`PrimitiveError::Yaml`] when
///   the spec status cannot be read.
/// - [`PrimitiveError::Yaml`] when `analysis.md`'s `decisions:` list does not
///   parse, and [`PrimitiveError::UnclosedFrontmatter`] when its frontmatter
///   never closes — read as empty, either would propose every section already
///   decided.
/// - [`PrimitiveError::Git`] when a call against a `done` spec cannot read
///   HEAD for the reopen trigger.
/// - [`PrimitiveError::Io`] on filesystem failure.
///
/// Each error writes nothing.
pub fn run(args: &PrunePlanArgs, repo: &Path) -> Result<PrunePlanResult> {
    let root = paths::Paths::load(repo).specs_root;
    let feature = one_feature_or_all("prune-plan", args.feature.as_deref(), args.all)?;
    // Refused before the `remove` list is checked, so a walk asked to apply is
    // told why it cannot rather than to list a section it then may not remove.
    if feature.is_none() && args.apply {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "prune-plan".into(),
            argument: "apply".into(),
            reason: "each plan section is a judgment about one spec's content; \
                     apply one spec at a time"
                .into(),
        });
    }
    check_removals(args)?;
    let Some(feature) = feature else {
        return Ok(PrunePlanResult {
            summary: None,
            walk: Some(walk(args, repo, &root)?),
        });
    };
    Ok(PrunePlanResult {
        summary: Some(summarize(feature, args, repo, &root)?),
        walk: None,
    })
}

/// Every feature's preview, in corpus order. A feature is listed when its
/// plan holds a section outside the record, or when it is a `done` spec whose
/// tree already requires a reopen — the one thing a walk must not drop, since
/// `/{project}:prune` names that reopen on the spec's row. Any other feature
/// is examined and left out, and one with no `plan.md` is skipped with that
/// reason.
fn walk(args: &PrunePlanArgs, repo: &Path, root: &str) -> Result<PruneWalk<PrunePlanSummary>> {
    let names = list_feature_dirs(&repo.join(root));
    let mut features = Vec::new();
    let mut skipped = Vec::new();
    for feature in &names {
        let summary = summarize(feature, args, repo, root)?;
        if !summary.sections.is_empty() || summary.reopen_required == Some(true) {
            features.push(PruneWalkEntry {
                feature: feature.clone(),
                summary,
            });
        } else if summary.missing {
            skipped.push(SkippedFeature {
                feature: feature.clone(),
                reason: SkipReason::NoPlanFile,
            });
        }
    }
    Ok(PruneWalk {
        examined: u32::try_from(names.len()).unwrap_or(u32::MAX),
        features,
        skipped,
    })
}

/// One feature's preview or apply.
fn summarize(
    feature: &str,
    args: &PrunePlanArgs,
    repo: &Path,
    root: &str,
) -> Result<PrunePlanSummary> {
    super::validate_feature_argument(repo, root, feature)?;
    let feature_dir = repo.join(root).join(feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    }
    let status = read_status(&feature_dir, root, feature)?;
    let plan_path = feature_dir.join("plan.md");
    let plan_rel = format!("{root}/{feature}/plan.md");
    let tasks_rel = format!("{root}/{feature}/tasks.md");

    let content = if plan_path.is_file() {
        Some(read_text(&plan_path)?)
    } else {
        None
    };
    let blocks = content.as_deref().map(segment).unwrap_or_default();
    let stored = read_decisions(&feature_dir, ANALYSIS_RECORD_FILE)?;
    let (examined, outside) = outside_record(&blocks, &stored);
    let (claimed, stale_sections) = claim(&outside, &args.remove);

    let before = content.as_deref().unwrap_or_default();
    let ending = line_ending_of(before);
    let without = |drop: &[usize]| -> String {
        if drop.is_empty() {
            return before.to_string();
        }
        let kept: Vec<&[String]> = blocks
            .iter()
            .enumerate()
            .filter(|(idx, _)| !drop.contains(idx))
            .map(|(_, block)| block.lines.as_slice())
            .collect();
        join_blocks(&kept, ending)
    };

    // With no plan every listed removal is stale, so an apply over a missing
    // plan is refused by the same test.
    let applied = args.apply && stale_sections.is_empty();
    let after = if applied {
        without(&claimed)
    } else if args.apply {
        before.to_string()
    } else {
        let proposed: Vec<usize> = outside
            .iter()
            .filter(|(_, section)| !section.decided)
            .map(|(idx, _)| *idx)
            .collect();
        without(&proposed)
    };

    // Computed before the write, so an apply that cannot read HEAD writes
    // nothing. A preview reads the tree as it stands — removing sections
    // outside the record changes no design-record section — so a reopen the
    // tree already carries, from edits made before the run, is known before
    // the host writes anything.
    let reopen_required = if status == "done" {
        Some(reopen_required(repo, &plan_rel, &tasks_rel, &after)?)
    } else {
        None
    };
    if applied {
        write_atomic(&plan_path, &after)?;
    }

    Ok(PrunePlanSummary {
        path: rel_path(&plan_path, repo),
        missing: content.is_none(),
        status,
        sections_examined: examined,
        sections: outside.into_iter().map(|(_, section)| section).collect(),
        applied,
        stale_sections,
        size_before: size_of(before),
        size_after: size_of(&after),
        reopen_required,
    })
}

/// Every section of `blocks` outside the design record, with the index of its
/// block, and the count of `##` sections read. A section is decided when
/// `stored` holds a well-formed discard under its finding's exact key.
fn outside_record(blocks: &[Block], stored: &[RawDecision]) -> (u32, Vec<(usize, PlanSection)>) {
    let decided = |finding: &PlanFinding| {
        let key = finding_key_of(&finding.family, &finding.message);
        stored.iter().any(|entry| {
            entry.to_ref().is_ok_and(|decision| {
                decision.outcome == DecisionOutcome::Discarded && same_key(&decision.key, &key)
            })
        })
    };
    let mut examined = 0u32;
    let mut outside = Vec::new();
    for (idx, block) in blocks.iter().enumerate() {
        let Some(heading) = block.heading.as_deref() else {
            continue;
        };
        examined += 1;
        if is_design_record(heading) {
            continue;
        }
        let text = block.text();
        let finding = plan_section_finding(heading);
        outside.push((
            idx,
            PlanSection {
                heading: heading.to_string(),
                ordinal: examined,
                lines: text.lines().count(),
                bytes: text.len(),
                digest: hex(&Sha256::digest(text.as_bytes())),
                decided: decided(&finding),
                finding,
            },
        ));
    }
    (examined, outside)
}

/// Match each removal to a distinct section of that heading and digest.
/// Returns the block indexes claimed and the heading of each removal left
/// unmatched — the section changed or went since the preview.
fn claim(outside: &[(usize, PlanSection)], remove: &[PlanRemoval]) -> (Vec<usize>, Vec<String>) {
    let mut claimed: Vec<usize> = Vec::new();
    let mut stale = Vec::new();
    for removal in remove {
        match outside.iter().find(|(idx, section)| {
            !claimed.contains(idx)
                && section.heading == removal.heading
                && section.digest == removal.digest
        }) {
            Some((idx, _)) => claimed.push(*idx),
            None => stale.push(removal.heading.clone()),
        }
    }
    (claimed, stale)
}

/// Refuse a `remove` list the call cannot honor: an apply that lists nothing,
/// a list with no apply to act on it, or a design-record section, which prune
/// never removes.
fn check_removals(args: &PrunePlanArgs) -> Result<()> {
    if args.apply && args.remove.is_empty() {
        return Err(PrimitiveError::MissingArgument {
            primitive: "prune-plan".into(),
            argument: "remove".into(),
            reason: "an apply removes the sections it lists, so list at least one".into(),
        });
    }
    if !args.apply && !args.remove.is_empty() {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "prune-plan".into(),
            argument: "remove".into(),
            reason: "only an apply removes sections; pass apply with the list".into(),
        });
    }
    if let Some(record) = args
        .remove
        .iter()
        .find(|removal| is_design_record(&removal.heading))
    {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "prune-plan".into(),
            argument: "remove".into(),
            reason: format!(
                "§{} is a design-record section, which prune never removes",
                record.heading
            ),
        });
    }
    Ok(())
}

/// Whether the tree, with `plan` as its plan, takes a `done` spec's back-edge
/// against HEAD: any design-record section of `plan` differs from HEAD's, or
/// `tasks.md` adds an unchecked checkbox to HEAD's. An artifact absent at HEAD
/// triggers nothing — a `done` spec's artifacts are committed.
fn reopen_required(repo: &Path, plan_rel: &str, tasks_rel: &str, plan: &str) -> Result<bool> {
    let project = ProjectRepository::discover(repo)?;
    if let Some(head) = project.read_at_head(plan_rel)?
        && design_record(plan) != design_record(&head)
    {
        return Ok(true);
    }
    if let Some(head) = project.read_at_head(tasks_rel)? {
        let tasks = repo.join(tasks_rel);
        let now = if tasks.is_file() {
            read_text(&tasks)?
        } else {
            String::new()
        };
        return Ok(adds_unchecked_box(&now, &head));
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, write};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    const FEATURE: &str = "041-task-pruning";

    fn feature_dir(repo: &Path) -> PathBuf {
        repo.join("specs").join(FEATURE)
    }

    /// A project with `spec.md` at `status` and, when given, `plan.md`.
    fn project(repo: &Path, status: &str, plan: Option<&str>) {
        write(
            &feature_dir(repo).join("spec.md"),
            &format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Spec\n"),
        );
        if let Some(plan) = plan {
            write(&feature_dir(repo).join("plan.md"), plan);
        }
    }

    fn preview() -> PrunePlanArgs {
        PrunePlanArgs {
            feature: Some(FEATURE.into()),
            all: false,
            apply: false,
            remove: Vec::new(),
        }
    }

    /// One feature's summary from a single-feature call.
    fn run(args: &PrunePlanArgs, repo: &Path) -> Result<PrunePlanSummary> {
        super::run(args, repo).map(|result| result.summary.expect("a single-feature summary"))
    }

    fn apply(sections: &[&PlanSection]) -> PrunePlanArgs {
        PrunePlanArgs {
            feature: Some(FEATURE.into()),
            all: false,
            apply: true,
            remove: sections
                .iter()
                .map(|section| PlanRemoval {
                    heading: section.heading.clone(),
                    digest: section.digest.clone(),
                })
                .collect(),
        }
    }

    fn plan_text(repo: &Path) -> String {
        fs::read_to_string(feature_dir(repo).join("plan.md")).unwrap()
    }

    const PLAN: &str = "# 041 — Plan\n\nImplements the spec.\n\n## Overview\n\nThe approach.\n\n## Technical Decisions\n\n### A decision\n\nWhy.\n\n## Implementation notes\n\nA journal entry.\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n\n## Trade-offs\n\nRejected X.\n";

    #[test]
    fn a_preview_reports_each_section_outside_the_record_and_writes_nothing() {
        let tmp = tempdir().unwrap();
        project(tmp.path(), "in-progress", Some(PLAN));
        let result = run(&preview(), tmp.path()).unwrap();
        assert!(!result.missing);
        assert_eq!(result.sections_examined, 5);
        assert_eq!(result.sections.len(), 1);
        let section = &result.sections[0];
        assert_eq!(section.heading, "Implementation notes");
        assert_eq!(section.ordinal, 3);
        assert_eq!(section.lines, 3);
        assert_eq!(section.finding.family, "plan-record");
        assert_eq!(
            section.finding.message,
            "plan.md §Implementation notes is outside the design record"
        );
        assert!(!section.decided);
        assert!(!result.applied);
        assert!(result.reopen_required.is_none());
        assert!(result.size_after.bytes < result.size_before.bytes);
        assert_eq!(plan_text(tmp.path()), PLAN, "a preview must not write");
    }

    #[test]
    fn a_heading_inside_a_fence_or_a_comment_is_not_a_section() {
        let tmp = tempdir().unwrap();
        let plan = "# P\n\n## Overview\n\n```markdown\n## Not a section\n```\n\n<!--\n## Nor this\n-->\n\n## Technical Decisions\n\nText.\n";
        project(tmp.path(), "planned", Some(plan));
        let result = run(&preview(), tmp.path()).unwrap();
        assert_eq!(result.sections_examined, 2);
        assert!(result.sections.is_empty(), "{:?}", result.sections);
    }

    #[test]
    fn headings_compare_case_insensitively() {
        let tmp = tempdir().unwrap();
        let plan = "# P\n\n## overview\n\nA.\n\n## TECHNICAL DECISIONS\n\nB.\n\n## Cross-Spec Impact\n\nC.\n\n## Tradeoffs\n\nD.\n";
        project(tmp.path(), "planned", Some(plan));
        let result = run(&preview(), tmp.path()).unwrap();
        let headings: Vec<&str> = result.sections.iter().map(|s| s.heading.as_str()).collect();
        // Case is ignored; punctuation is not.
        assert_eq!(headings, ["Tradeoffs"]);
    }

    #[test]
    fn an_apply_removes_the_listed_section_and_normalizes_the_seam() {
        let tmp = tempdir().unwrap();
        // No blank line between the section before and the one removed, so a
        // plain join would butt its last line against the next heading.
        let plan = PLAN.replace(
            "Why.\n\n## Implementation notes",
            "Why.\n## Implementation notes",
        );
        project(tmp.path(), "in-progress", Some(&plan));
        let previewed = run(&preview(), tmp.path()).unwrap();
        let result = run(&apply(&[&previewed.sections[0]]), tmp.path()).unwrap();
        assert!(result.applied);
        assert_eq!(result.stale_sections, [] as [String; 0]);
        let written = plan_text(tmp.path());
        assert!(!written.contains("Implementation notes"));
        assert!(!written.contains("A journal entry."));
        assert!(written.contains("### A decision\n\nWhy.\n\n## Affected Files"));
        assert!(!written.contains("\n\n\n"));
        assert_eq!(result.size_after.bytes, written.len());
        // Only a call against a `done` spec computes the trigger.
        assert!(result.reopen_required.is_none());
    }

    #[test]
    fn a_digest_mismatch_refuses_the_whole_apply() {
        let tmp = tempdir().unwrap();
        let plan = format!("{PLAN}\n## Resuming from here\n\nOrder.\n");
        project(tmp.path(), "in-progress", Some(&plan));
        let previewed = run(&preview(), tmp.path()).unwrap();
        assert_eq!(previewed.sections.len(), 2);
        // The journal changes after the preview; the other section does not.
        let edited = plan.replace("A journal entry.", "A journal entry, edited.");
        write(&feature_dir(tmp.path()).join("plan.md"), &edited);
        let result = run(
            &apply(&[&previewed.sections[0], &previewed.sections[1]]),
            tmp.path(),
        )
        .unwrap();
        assert!(!result.applied);
        assert_eq!(result.stale_sections, ["Implementation notes"]);
        assert_eq!(
            plan_text(tmp.path()),
            edited,
            "a refused apply writes nothing, not even the unchanged section's removal"
        );
    }

    #[test]
    fn a_removal_naming_no_section_is_stale() {
        let tmp = tempdir().unwrap();
        project(tmp.path(), "in-progress", Some(PLAN));
        let args = PrunePlanArgs {
            feature: Some(FEATURE.into()),
            all: false,
            apply: true,
            remove: vec![PlanRemoval {
                heading: "Gone".into(),
                digest: "00".into(),
            }],
        };
        let result = run(&args, tmp.path()).unwrap();
        assert!(!result.applied);
        assert_eq!(result.stale_sections, ["Gone"]);
        assert_eq!(plan_text(tmp.path()), PLAN);
    }

    #[test]
    fn a_design_record_section_is_never_removed() {
        let tmp = tempdir().unwrap();
        project(tmp.path(), "in-progress", Some(PLAN));
        let args = PrunePlanArgs {
            feature: Some(FEATURE.into()),
            all: false,
            apply: true,
            remove: vec![PlanRemoval {
                heading: "overview".into(),
                digest: "00".into(),
            }],
        };
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidArgument { .. }),
            "{err}"
        );
        assert_eq!(plan_text(tmp.path()), PLAN);
    }

    #[test]
    fn remove_and_apply_come_together() {
        let tmp = tempdir().unwrap();
        project(tmp.path(), "in-progress", Some(PLAN));
        let nothing_listed = PrunePlanArgs {
            feature: Some(FEATURE.into()),
            all: false,
            apply: true,
            remove: Vec::new(),
        };
        assert!(matches!(
            run(&nothing_listed, tmp.path()).unwrap_err(),
            PrimitiveError::MissingArgument { .. }
        ));
        let no_apply = PrunePlanArgs {
            feature: Some(FEATURE.into()),
            all: false,
            apply: false,
            remove: vec![PlanRemoval {
                heading: "Implementation notes".into(),
                digest: "00".into(),
            }],
        };
        assert!(matches!(
            run(&no_apply, tmp.path()).unwrap_err(),
            PrimitiveError::InvalidArgument { .. }
        ));
    }

    #[test]
    fn a_stored_discard_sets_decided() {
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let plan = format!("{PLAN}\n## Known limitations\n\nDesign.\n");
        project(tmp.path(), "done", Some(&plan));
        // A discard of the journal's finding, a route of the limitations':
        // only the discard marks its section decided.
        write(
            &feature_dir(tmp.path()).join("analysis.md"),
            "---\nspec: 041-task-pruning\ndecisions:\n  - key: \"plan-record — plan.md §Implementation notes is outside the design record\"\n    outcome: discarded\n    reason: deliberate\n    decided-at: 2026-09-29T00:00:00Z\n    decided-by: a@b.c\n  - key: \"plan-record — plan.md §Known limitations is outside the design record\"\n    outcome: routed\n    target: specs/041-task-pruning/tasks.md\n    decided-at: 2026-09-29T00:00:00Z\n    decided-by: a@b.c\n---\n\n# Analysis\n",
        );
        // A `done` spec's preview reads HEAD for the reopen trigger.
        commit_all(&repository, "done");
        let result = run(&preview(), tmp.path()).unwrap();
        let decided: Vec<(&str, bool)> = result
            .sections
            .iter()
            .map(|s| (s.heading.as_str(), s.decided))
            .collect();
        assert_eq!(
            decided,
            [("Implementation notes", true), ("Known limitations", false)]
        );
        // The preview's size-after proposes only the undecided section.
        let only_limitations = size_of(&plan.replace("\n## Known limitations\n\nDesign.\n", ""));
        assert_eq!(result.size_after, only_limitations);
    }

    #[test]
    fn a_missing_plan_is_reported_not_an_error() {
        let tmp = tempdir().unwrap();
        project(tmp.path(), "clarified", None);
        let result = run(&preview(), tmp.path()).unwrap();
        assert!(result.missing);
        assert_eq!(result.sections_examined, 0);
        assert_eq!(result.sections, [] as [PlanSection; 0]);
        assert!(!feature_dir(tmp.path()).join("plan.md").exists());
    }

    fn all() -> PrunePlanArgs {
        PrunePlanArgs {
            feature: None,
            all: true,
            apply: false,
            remove: Vec::new(),
        }
    }

    #[test]
    fn a_walk_lists_plans_with_sections_outside_the_record_and_skips_missing_ones() {
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let clean = "# P\n\n## Overview\n\nA.\n";
        for (name, status, plan) in [
            ("1234.10-late", "done", Some(PLAN)),
            ("1234.2-early", "done", Some(PLAN)),
            ("002-first", "in-progress", Some(PLAN)),
            ("003-clean", "done", Some(clean)),
            ("004-draft", "draft", None),
        ] {
            let dir = tmp.path().join("specs").join(name);
            write(
                &dir.join("spec.md"),
                &format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Spec\n"),
            );
            if let Some(plan) = plan {
                write(&dir.join("plan.md"), plan);
            }
        }
        // Each `done` spec's preview reads HEAD for the reopen trigger.
        commit_all(&repository, "corpus");
        let walk = super::run(&all(), tmp.path())
            .unwrap()
            .walk
            .expect("an all walk");
        let order: Vec<(&str, &str)> = walk
            .features
            .iter()
            .map(|f| (f.feature.as_str(), f.summary.status.as_str()))
            .collect();
        assert_eq!(
            order,
            [
                ("002-first", "in-progress"),
                ("1234.2-early", "done"),
                ("1234.10-late", "done")
            ]
        );
        assert_eq!(
            walk.skipped,
            [SkippedFeature {
                feature: "004-draft".into(),
                reason: SkipReason::NoPlanFile,
            }]
        );
        assert_eq!(walk.examined, 5);
    }

    #[test]
    fn a_plan_walk_is_preview_only() {
        let tmp = tempdir().unwrap();
        project(tmp.path(), "done", Some(PLAN));
        let mut applying = all();
        applying.apply = true;
        applying.remove = vec![PlanRemoval {
            heading: "Implementation notes".into(),
            digest: "00".into(),
        }];
        let err = super::run(&applying, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "apply"),
            "{err}"
        );
        assert_eq!(plan_text(tmp.path()), PLAN);
        // Refused for the walk even with nothing listed, rather than being
        // told to list a section it then may not remove.
        let mut nothing_listed = all();
        nothing_listed.apply = true;
        let err = super::run(&nothing_listed, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "apply"),
            "{err}"
        );
        let mut neither = all();
        neither.all = false;
        assert!(matches!(
            super::run(&neither, tmp.path()).unwrap_err(),
            PrimitiveError::MissingArgument { .. }
        ));
    }

    #[test]
    fn a_walk_lists_a_done_spec_whose_tree_requires_a_reopen() {
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let clean = "# P\n\n## Overview\n\nA.\n\n## Technical Decisions\n\nWhy.\n";
        let spec = |name: &str, status: &str| {
            write(
                &tmp.path().join("specs").join(name).join("spec.md"),
                &format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Spec\n"),
            );
        };
        spec("001-edited", "done");
        write(&tmp.path().join("specs/001-edited/plan.md"), clean);
        spec("002-no-plan", "done");
        write(
            &tmp.path().join("specs/002-no-plan/tasks.md"),
            "# Tasks\n\n## 1. Done\n\n- [x] a\n",
        );
        spec("003-quiet", "done");
        write(&tmp.path().join("specs/003-quiet/plan.md"), clean);
        spec("004-draft", "draft");
        commit_all(&repository, "corpus");
        // Edits made before the run: a design-record change, and owed work on
        // a spec with no plan. Neither spec has a section to prune.
        write(
            &tmp.path().join("specs/001-edited/plan.md"),
            &clean.replace("Why.", "Why, as it now stands."),
        );
        write(
            &tmp.path().join("specs/002-no-plan/tasks.md"),
            "# Tasks\n\n## 1. Done\n\n- [x] a\n\n## 2. Owed\n\n- [ ] b\n",
        );
        let walk = super::run(&all(), tmp.path())
            .unwrap()
            .walk
            .expect("an all walk");
        let listed: Vec<(&str, Option<bool>)> = walk
            .features
            .iter()
            .map(|f| (f.feature.as_str(), f.summary.reopen_required))
            .collect();
        assert_eq!(
            listed,
            [("001-edited", Some(true)), ("002-no-plan", Some(true))]
        );
        assert_eq!(
            walk.skipped,
            [SkippedFeature {
                feature: "004-draft".into(),
                reason: SkipReason::NoPlanFile,
            }]
        );
        assert_eq!(walk.examined, 4);
    }

    #[test]
    fn a_section_listed_twice_is_claimed_once() {
        let tmp = tempdir().unwrap();
        project(tmp.path(), "in-progress", Some(PLAN));
        let previewed = run(&preview(), tmp.path()).unwrap();
        let journal = &previewed.sections[0];
        let result = run(&apply(&[journal, journal]), tmp.path()).unwrap();
        assert!(!result.applied);
        assert_eq!(result.stale_sections, ["Implementation notes"]);
        assert_eq!(plan_text(tmp.path()), PLAN);
    }

    #[test]
    fn an_apply_keeps_crlf_line_endings() {
        let tmp = tempdir().unwrap();
        let crlf = PLAN.replace('\n', "\r\n");
        project(tmp.path(), "in-progress", Some(&crlf));
        let previewed = run(&preview(), tmp.path()).unwrap();
        let result = run(&apply(&[&previewed.sections[0]]), tmp.path()).unwrap();
        assert!(result.applied);
        let written = plan_text(tmp.path());
        assert!(!written.contains("Implementation notes"));
        assert_eq!(
            written.matches('\n').count(),
            written.matches("\r\n").count(),
            "every line ending stays CRLF: {written:?}"
        );
    }

    #[test]
    fn design_record_matches_the_plan_template() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let template =
            fs::read_to_string(repo_root.join("framework/templates/spec/plan.md")).unwrap();
        let headings: Vec<String> = segment(&template)
            .into_iter()
            .filter_map(|block| block.heading)
            .collect();
        assert_eq!(
            headings, DESIGN_RECORD,
            "DESIGN_RECORD drifted from framework/templates/spec/plan.md"
        );
    }

    // --- the reopen trigger ---------------------------------------------

    /// A committed `done` spec with `PLAN` and a fully checked `tasks.md`.
    fn committed_done_spec(root: &Path) -> git2::Repository {
        let repository = git2::Repository::init(root).unwrap();
        project(root, "done", Some(PLAN));
        write(
            &feature_dir(root).join("tasks.md"),
            "# Tasks\n\n## 1. Done\n\n- [x] a\n",
        );
        commit_all(&repository, "done");
        repository
    }

    fn remove_journal(root: &Path) -> PrunePlanSummary {
        let previewed = run(&preview(), root).unwrap();
        run(&apply(&[&previewed.sections[0]]), root).unwrap()
    }

    #[test]
    fn a_removal_only_diff_does_not_reopen() {
        let tmp = tempdir().unwrap();
        let _repository = committed_done_spec(tmp.path());
        let result = remove_journal(tmp.path());
        assert!(result.applied);
        assert_eq!(result.reopen_required, Some(false));
    }

    #[test]
    fn an_artifact_absent_at_head_triggers_nothing() {
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        project(tmp.path(), "done", Some(PLAN));
        commit_all(&repository, "done, with no task list");
        // A task list written since HEAD holds an unchecked box, but HEAD has
        // no task list to compare it with.
        write(
            &feature_dir(tmp.path()).join("tasks.md"),
            "# Tasks\n\n## 1. Owed\n\n- [ ] b\n",
        );
        let result = remove_journal(tmp.path());
        assert_eq!(result.reopen_required, Some(false));
    }

    #[test]
    fn a_changed_design_record_section_reopens() {
        let tmp = tempdir().unwrap();
        let _repository = committed_done_spec(tmp.path());
        // The host moves a decision home before the removal.
        let moved = PLAN.replace("Why.\n", "Why, as it now stands.\n");
        write(&feature_dir(tmp.path()).join("plan.md"), &moved);
        let result = remove_journal(tmp.path());
        assert_eq!(result.reopen_required, Some(true));
    }

    #[test]
    fn an_added_unchecked_task_reopens() {
        let tmp = tempdir().unwrap();
        let _repository = committed_done_spec(tmp.path());
        write(
            &feature_dir(tmp.path()).join("tasks.md"),
            "# Tasks\n\n## 1. Done\n\n- [x] a\n\n## 2. Owed\n\n- [ ] b\n",
        );
        let result = remove_journal(tmp.path());
        assert_eq!(result.reopen_required, Some(true));
    }

    #[test]
    fn an_unchecked_box_in_a_comment_or_a_fence_adds_nothing() {
        // The tasks template's guidance comment carries example `- [ ]` lines,
        // so a reset body holds them: they are not owed work.
        let tmp = tempdir().unwrap();
        let _repository = committed_done_spec(tmp.path());
        write(
            &feature_dir(tmp.path()).join("tasks.md"),
            "# Tasks\n\n## 1. Done\n\n- [x] a\n\n<!--\n- [ ] an example\n-->\n\n```markdown\n- [ ] another\n```\n",
        );
        let result = remove_journal(tmp.path());
        assert_eq!(result.reopen_required, Some(false));
    }

    #[test]
    fn adding_one_unchecked_box_and_checking_another_reopens() {
        // The count of unchecked boxes is unchanged; the diff still adds one.
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        project(tmp.path(), "done", Some(PLAN));
        write(
            &feature_dir(tmp.path()).join("tasks.md"),
            "# Tasks\n\n## 1. Owed\n\n- [ ] a\n",
        );
        commit_all(&repository, "done, carrying an unchecked box");
        write(
            &feature_dir(tmp.path()).join("tasks.md"),
            "# Tasks\n\n## 1. Owed\n\n- [x] a\n\n## 2. More\n\n- [ ] b\n",
        );
        let result = remove_journal(tmp.path());
        assert_eq!(result.reopen_required, Some(true));
    }

    #[test]
    fn moving_or_reindenting_an_unchecked_box_does_not_reopen() {
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        project(tmp.path(), "done", Some(PLAN));
        write(
            &feature_dir(tmp.path()).join("tasks.md"),
            "# Tasks\n\n## 1. One\n\n- [ ] a\n\n## 2. Two\n\n- [x] b\n",
        );
        commit_all(&repository, "done, carrying an unchecked box");
        // The unchecked box moves under task 2 and is nested there.
        write(
            &feature_dir(tmp.path()).join("tasks.md"),
            "# Tasks\n\n## 1. One\n\nProse.\n\n## 2. Two\n\n- [x] b\n  - [ ] a\n",
        );
        let result = remove_journal(tmp.path());
        assert_eq!(result.reopen_required, Some(false));
    }

    #[test]
    fn a_preview_reports_the_reopen_the_tree_already_carries() {
        let tmp = tempdir().unwrap();
        let _repository = committed_done_spec(tmp.path());
        let clean = run(&preview(), tmp.path()).unwrap();
        assert_eq!(clean.reopen_required, Some(false));
        // A design-record edit made before the run is known at the preview.
        let edited = PLAN.replace("Why.\n", "Why, corrected by hand.\n");
        write(&feature_dir(tmp.path()).join("plan.md"), &edited);
        let result = run(&preview(), tmp.path()).unwrap();
        assert_eq!(result.reopen_required, Some(true));
        assert!(!result.applied);
        assert_eq!(plan_text(tmp.path()), edited, "a preview must not write");
    }

    #[test]
    fn an_apply_that_cannot_read_head_writes_nothing() {
        // No repository: the trigger cannot be computed, and the removal must
        // not land ahead of the error.
        let tmp = tempdir().unwrap();
        project(tmp.path(), "in-progress", Some(PLAN));
        let previewed = run(&preview(), tmp.path()).unwrap();
        project(tmp.path(), "done", None);
        let err = run(&apply(&[&previewed.sections[0]]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Git(..)), "{err}");
        assert_eq!(plan_text(tmp.path()), PLAN);
    }

    #[test]
    fn the_reopen_trigger_reads_head_from_a_project_in_a_repository_subdirectory() {
        let tmp = tempdir().unwrap();
        let (repository, root) = crate::primitives::git_fixture::subdirectory_project(tmp.path());
        project(&root, "done", Some(PLAN));
        commit_all(&repository, "done");
        let moved = PLAN.replace("Why.\n", "Why, as it now stands.\n");
        write(&feature_dir(&root).join("plan.md"), &moved);
        let result = remove_journal(&root);
        assert_eq!(result.reopen_required, Some(true));
    }
}
