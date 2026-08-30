//! `read-supersession-pair` — the bounded read reconciliation is built on.
//!
//! Spec 053 bounds reconciliation's read to the two specs a declared
//! supersession names, plus the superseded spec's scenarios. Nothing else: no
//! plan, no data model, no tasks file, no source tree, no third spec.
//!
//! **The bound lives here rather than in the command prose, and that is the
//! whole reason this primitive exists.** A rule the host is asked to remember
//! is a diligence dependency, which §design-principles rejects outright.
//! There is no argument by which a caller could ask for any of the excluded
//! paths, so the bound is a property of the code.
//!
//! It is load-bearing rather than tidy. Without a bound, reconciliation is
//! the corpus-wide criterion-supersession check that was measured and
//! rejected — 455 pairs tested, 215 firing, every sampled one a false
//! positive. A *declared* edge is what collapses the search to two specs and
//! changes what a false positive costs: not an unsolicited finding on a
//! `done` spec, but a candidate offered to an operator who has already said
//! these two conflict.
//!
//! Reading two full specs on the authority of a declared pointer is not a
//! widening of what the pipeline permits — fold-back already declares exactly
//! this bound, reading both specs while excluding plans, data models, and
//! source.
//!
//! Defined by `specs/053-supersession-reconciliation/spec.md`.

use std::path::Path;

use crate::primitives::read_spec::{parse_checkboxes, parse_sections};
use crate::primitives::{
    Result, list_scenario_files, read_text, rel_path, split_frontmatter, validate_no_traversal,
};
use crate::schema::paths;
use crate::schema::primitives::{
    Frontmatter, ReadSupersessionPairArgs, ReadSupersessionPairResult, ScenarioRead, SpecRead,
};

/// Load the declared pair and the superseded spec's scenarios.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::InvalidPath`] when either
/// feature name is empty, absolute, or carries a parent-directory component,
/// and [`crate::primitives::PrimitiveError::InvalidArgument`] when the two
/// name the same feature — a spec cannot supersede itself, and classifying a
/// spec against its own claims would report every one of them as conflicting.
///
/// A spec or scenario that cannot be read is **not** an error. It lands in
/// `unreadable` and is excluded from `examined`, because nothing can be
/// proven about a file that will not parse — and a read that died on one
/// would tell the caller nothing about the files it *could* read.
pub fn run(args: &ReadSupersessionPairArgs, repo: &Path) -> Result<ReadSupersessionPairResult> {
    validate_no_traversal(&args.feature)?;
    validate_no_traversal(&args.superseded_by)?;

    if args.feature == args.superseded_by {
        return Err(crate::primitives::PrimitiveError::InvalidArgument {
            primitive: "read-supersession-pair".into(),
            argument: "superseded-by".into(),
            reason: "a spec cannot supersede itself; classifying a spec against its own claims \
                     would report every one of them as conflicting"
                .into(),
        });
    }

    let layout = paths::Paths::load(repo);
    let specs_root = layout.specs_root.clone();
    let mut result = ReadSupersessionPairResult::default();

    result.superseded = read_one(
        repo,
        &specs_root,
        &args.feature,
        &mut result.unreadable,
        &mut result.examined,
    );
    result.superseding = read_one(
        repo,
        &specs_root,
        &args.superseded_by,
        &mut result.unreadable,
        &mut result.examined,
    );

    // Scenarios of the **superseded** spec only. The superseding spec's
    // scenarios describe what it delivers, which is not what reconciliation
    // classifies — it walks the *predecessor's* claims.
    let scenarios_dir = repo.join(&specs_root).join(&args.feature).join("scenarios");
    for name in list_scenario_files(&scenarios_dir) {
        let path = scenarios_dir.join(&name);
        let relative = rel_path(&path, repo);
        let slug = Path::new(&name)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(name.as_str())
            .to_string();
        let Ok(content) = read_text(&path) else {
            result.unreadable.push(relative);
            continue;
        };
        // A hand-written scenario may carry no frontmatter; fall back to the
        // whole file rather than dropping it, the way the shared scenario
        // scan already does.
        let body = split_frontmatter(&content, &path)
            .map_or(content.as_str(), |(_, body)| body)
            .to_string();
        result.examined.push(relative.clone());
        result.scenarios.push(ScenarioRead {
            slug,
            path: relative,
            body,
        });
    }

    // Nothing to classify is a distinct outcome from nothing to report, and
    // the two must not share a shape. A brownfield sketch spec with no
    // criteria and no sections is legitimately the former (AC11).
    if result.superseded.acceptance_criteria.is_empty()
        && result.superseded.sections.is_empty()
        && result.scenarios.is_empty()
    {
        result.guidance = format!(
            "`{}` carries no criteria, no body sections, and no scenarios — there is nothing to \
             reconcile, which is not the same as reconciling and finding no conflicts",
            args.feature
        );
    }

    Ok(result)
}

/// Read one spec into the shape reconciliation classifies over.
///
/// An unreadable or unparseable spec yields an empty [`SpecRead`] and a
/// recorded path, never an error: the caller needs to know which half of the
/// pair it could not examine, and dying here would hide that the other half
/// was read fine.
fn read_one(
    repo: &Path,
    specs_root: &str,
    feature: &str,
    unreadable: &mut Vec<String>,
    examined: &mut Vec<String>,
) -> SpecRead {
    let path = repo.join(specs_root).join(feature).join("spec.md");
    let relative = rel_path(&path, repo);
    let empty = SpecRead {
        feature: feature.to_string(),
        path: relative.clone(),
        ..SpecRead::default()
    };

    let Ok(content) = read_text(&path) else {
        unreadable.push(relative);
        return empty;
    };
    let Ok((frontmatter, body)) = split_frontmatter(&content, &path) else {
        unreadable.push(relative);
        return empty;
    };
    // Unparseable frontmatter yields an empty status rather than an error:
    // the body still carries claims worth classifying, and the status only
    // colours the guidance about whether the target shipped.
    let status = serde_norway::from_str::<Frontmatter>(frontmatter)
        .map(|fm| fm.status)
        .unwrap_or_default();

    examined.push(relative.clone());
    SpecRead {
        feature: feature.to_string(),
        path: relative,
        status,
        sections: parse_sections(body, true),
        acceptance_criteria: parse_checkboxes(body, "Acceptance Criteria"),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::PrimitiveError;
    use std::fs;
    use tempfile::tempdir;

    fn write(repo: &Path, rel: &str, body: &str) {
        let path = repo.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    fn spec(status: &str, criteria: &str) -> String {
        format!(
            "---\nstatus: {status}\ndependencies: []\n---\n\n# A Spec\n\nLead.\n\n\
             ## Behavior\n\nIt does a thing.\n\n## Acceptance Criteria\n\n{criteria}"
        )
    }

    fn args(feature: &str, by: &str) -> ReadSupersessionPairArgs {
        ReadSupersessionPairArgs {
            feature: feature.into(),
            superseded_by: by.into(),
        }
    }

    fn pair(repo: &Path) -> ReadSupersessionPairResult {
        run(&args("005-workflows", "043-sunset"), repo).unwrap()
    }

    fn seed(repo: &Path) {
        write(
            repo,
            "specs/005-workflows/spec.md",
            &spec(
                "done",
                "- [x] AC1: Workflow files are scaffolded\n- [x] AC2: Something else\n",
            ),
        );
        write(
            repo,
            "specs/043-sunset/spec.md",
            &spec("done", "- [ ] AC1: The workflow scaffolding is removed\n"),
        );
    }

    #[test]
    fn reads_both_specs_and_the_superseded_spec_scenarios() {
        let tmp = tempdir().unwrap();
        seed(tmp.path());
        write(
            tmp.path(),
            "specs/005-workflows/scenarios/one.md",
            "---\nsection: \"Behavior\"\n---\n\n# One\n\nA claim.\n",
        );
        let result = pair(tmp.path());

        assert_eq!(result.superseded.feature, "005-workflows");
        assert_eq!(result.superseded.status, "done");
        assert_eq!(result.superseded.acceptance_criteria.len(), 2);
        assert_eq!(
            result.superseded.acceptance_criteria[0].label.as_deref(),
            Some("AC1")
        );
        assert!(
            result
                .superseded
                .sections
                .iter()
                .any(|s| s.heading == "Behavior")
        );
        assert_eq!(result.superseding.feature, "043-sunset");
        assert_eq!(result.scenarios.len(), 1);
        assert_eq!(result.scenarios[0].slug, "one");
        assert!(result.scenarios[0].body.contains("A claim."));
        assert!(result.unreadable.is_empty());
        assert!(result.guidance.is_empty());
        assert_eq!(result.examined.len(), 3);
    }

    #[test]
    fn the_superseding_spec_scenarios_are_not_read() {
        // Reconciliation walks the *predecessor's* claims. The superseding
        // spec's scenarios describe what it delivers, which is a different
        // question and outside the declared bound.
        let tmp = tempdir().unwrap();
        seed(tmp.path());
        write(
            tmp.path(),
            "specs/043-sunset/scenarios/theirs.md",
            "---\nsection: \"Behavior\"\n---\n\n# Theirs\n\nNot a subject.\n",
        );
        let result = pair(tmp.path());
        assert!(result.scenarios.is_empty(), "{:?}", result.scenarios);
        assert!(
            !result
                .examined
                .iter()
                .any(|p| p.contains("043-sunset/scenarios")),
            "{:?}",
            result.examined
        );
    }

    #[test]
    fn a_pair_with_no_scenarios_is_read_cleanly() {
        let tmp = tempdir().unwrap();
        seed(tmp.path());
        let result = pair(tmp.path());
        assert!(result.scenarios.is_empty());
        assert!(result.unreadable.is_empty());
        assert!(result.guidance.is_empty());
    }

    #[test]
    fn an_absent_spec_is_named_unreadable_and_excluded_from_examined() {
        // A domain outcome, not an error: the caller needs to know which half
        // it could not examine, and dying would hide that the other half read
        // fine.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/005-workflows/spec.md",
            &spec("done", "- [x] AC1: A claim\n"),
        );
        let result = pair(tmp.path());
        assert_eq!(
            result.unreadable,
            vec!["specs/043-sunset/spec.md".to_string()]
        );
        assert!(
            !result.examined.iter().any(|p| p.contains("043-sunset")),
            "{:?}",
            result.examined
        );
        assert_eq!(result.superseding.acceptance_criteria.len(), 0);
    }

    #[test]
    fn an_unreadable_scenario_is_named_and_excluded_but_never_fatal() {
        let tmp = tempdir().unwrap();
        seed(tmp.path());
        let dir = tmp.path().join("specs/005-workflows/scenarios");
        fs::create_dir_all(&dir).unwrap();
        // Invalid UTF-8: listed by the shared scenario scan, and rejected by
        // the read. A directory named `*.md` would not do — the listing
        // filters to files — so this is the shape that actually reaches it.
        fs::write(dir.join("broken.md"), [0xff, 0xfe, 0x00]).unwrap();
        fs::write(
            dir.join("good.md"),
            "---\nsection: \"Behavior\"\n---\n\n# Good\n\nReadable.\n",
        )
        .unwrap();
        let result = pair(tmp.path());
        assert_eq!(result.scenarios.len(), 1, "{:?}", result.scenarios);
        assert!(
            result.unreadable.iter().any(|p| p.ends_with("broken.md")),
            "{:?}",
            result.unreadable
        );
    }

    #[test]
    fn a_superseded_spec_with_nothing_to_classify_reports_guidance() {
        // Examined-and-empty is not examined-and-clean, and a brownfield
        // sketch is legitimately the former.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/005-workflows/spec.md",
            "---\nstatus: draft\ndependencies: []\n---\n\n# Sketch\n",
        );
        write(
            tmp.path(),
            "specs/043-sunset/spec.md",
            &spec("done", "- [ ] AC1: A claim\n"),
        );
        let result = pair(tmp.path());
        assert!(
            result.guidance.contains("nothing to reconcile"),
            "{}",
            result.guidance
        );
    }

    #[test]
    fn a_spec_cannot_supersede_itself() {
        let tmp = tempdir().unwrap();
        seed(tmp.path());
        let err = run(&args("005-workflows", "005-workflows"), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidArgument { .. }),
            "{err:?}"
        );
    }

    #[test]
    fn a_traversing_feature_name_is_refused() {
        let tmp = tempdir().unwrap();
        seed(tmp.path());
        let err = run(&args("../etc", "043-sunset"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
    }

    #[test]
    fn a_configured_spec_root_is_honored() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[paths]\nspecs-root = \"requirements\"\n",
        )
        .unwrap();
        write(
            tmp.path(),
            "requirements/005-workflows/spec.md",
            &spec("done", "- [x] AC1: A claim\n"),
        );
        write(
            tmp.path(),
            "requirements/043-sunset/spec.md",
            &spec("done", "- [ ] AC1: Removed\n"),
        );
        let result = pair(tmp.path());
        assert_eq!(result.superseded.path, "requirements/005-workflows/spec.md");
        assert!(result.unreadable.is_empty());
    }
}
