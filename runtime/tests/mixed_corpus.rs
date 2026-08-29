//! Integration test (spec 051): a spec root holding **both** directory forms
//! is enumerated whole by every surface that reads the corpus.
//!
//! The regression this guards is not a wrong answer but an invisible one.
//! Before branch-scoped numbering, the membership predicate accepted only
//! `NNN-`, so a `1234.1-slug` directory was not mishandled by these
//! surfaces — it was absent from them. A test that only asserted the parse
//! would not have caught that, because the parse is not where the corpus is
//! assembled; `list_feature_dirs` and `is_spec_path` are.
//!
//! `dashboard` reads the working tree, while the two frontmatter generators
//! enumerate *tracked* files, so the fixture is committed.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

use git2::{IndexAddOption, Repository, Signature};

use ductus::primitives;
use ductus::schema::primitives::{
    DashboardArgs, DeriveDependenciesArgs, DeriveReferencesArgs, ResolveFeatureArgs,
    ResolveFeatureOutcome,
};

/// The branch-scoped directory every assertion below looks for.
const BRANCH_SCOPED: &str = "1234.1-staged-change";
/// A sequential sibling, so each assertion distinguishes "saw both" from
/// "saw everything" — a surface that ignored the predicate entirely would
/// pass a fixture holding only the new form.
const SEQUENTIAL: &str = "007-existing";

fn write(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

fn spec_body(title: &str, links: &str) -> String {
    format!("---\nstatus: draft\ndependencies: []\n---\n\n# {title}\n\n{links}\n")
}

/// A committed spec root holding one directory of each form.
fn seed(dir: &Path) {
    Repository::init(dir).unwrap();
    write(
        &dir.join("specs").join(SEQUENTIAL).join("spec.md"),
        &spec_body("Existing", "No links here."),
    );
    write(
        &dir.join("specs").join(BRANCH_SCOPED).join("spec.md"),
        // A sibling link, so the dependency generator has an edge to
        // harvest *from* the branch-scoped spec as well as a file to see.
        &spec_body(
            "Staged change",
            &format!("Builds on [existing](../{SEQUENTIAL}/spec.md)."),
        ),
    );
    let repository = Repository::open(dir).unwrap();
    let mut index = repository.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repository.find_tree(tree_id).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    repository
        .commit(Some("HEAD"), &sig, &sig, "fixture", &tree, &[])
        .unwrap();
}

#[test]
fn dashboard_lists_both_directory_forms() {
    let tmp = tempfile::tempdir().unwrap();
    seed(tmp.path());
    let result = primitives::dashboard::run(&DashboardArgs::default(), tmp.path()).unwrap();
    let features: Vec<&str> = result.specs.iter().map(|s| s.slug.as_str()).collect();
    assert!(
        features.contains(&BRANCH_SCOPED),
        "pipeline view omitted the branch-scoped spec: {features:?}"
    );
    assert!(
        features.contains(&SEQUENTIAL),
        "pipeline view omitted the sequential spec: {features:?}"
    );
    // The sequential form sorts ahead of the transient one.
    assert_eq!(features, vec![SEQUENTIAL, BRANCH_SCOPED]);
}

#[test]
fn derive_dependencies_examines_both_directory_forms() {
    let tmp = tempfile::tempdir().unwrap();
    seed(tmp.path());
    let result = primitives::derive_dependencies::run(
        &DeriveDependenciesArgs {
            write: true,
            staged: false,
        },
        tmp.path(),
    )
    .unwrap();
    assert_eq!(
        result.examined, 2,
        "both forms should be enumerated, got {}",
        result.examined
    );
    // The edge is harvested from the branch-scoped spec's body, which is
    // only reachable if the file was recognized as a spec at all.
    let rewritten =
        fs::read_to_string(tmp.path().join("specs").join(BRANCH_SCOPED).join("spec.md")).unwrap();
    assert!(
        rewritten.contains(&format!("dependencies: [{SEQUENTIAL}]")),
        "branch-scoped spec's dependencies were not derived:\n{rewritten}"
    );
}

#[test]
fn derive_references_examines_both_directory_forms() {
    let tmp = tempfile::tempdir().unwrap();
    seed(tmp.path());
    let result = primitives::derive_references::run(
        &DeriveReferencesArgs {
            write: false,
            staged: false,
        },
        tmp.path(),
    )
    .unwrap();
    assert_eq!(
        result.examined, 2,
        "both forms should be enumerated, got {}",
        result.examined
    );
}

#[test]
fn resolve_feature_finds_a_branch_scoped_directory_by_name() {
    let tmp = tempfile::tempdir().unwrap();
    seed(tmp.path());
    let result = primitives::resolve_feature::run(
        &ResolveFeatureArgs {
            identifier: BRANCH_SCOPED.to_string(),
            scenario: None,
        },
        tmp.path(),
    )
    .unwrap();
    assert!(
        matches!(result.outcome, ResolveFeatureOutcome::Resolved),
        "branch-scoped directory did not resolve: {:?}",
        result.outcome
    );
    assert_eq!(result.feature.as_deref(), Some(BRANCH_SCOPED));
}

/// A declared `folds-into` must survive every primitive that rewrites spec
/// frontmatter.
///
/// The field is safe only because none of these writers re-serializes the
/// block — each splices its own key and leaves the rest byte-identical. That
/// is a property of their implementations rather than a guarantee any of them
/// states, so it is pinned here: if one ever switches to a parse-and-emit
/// round trip, a branch-scoped spec would silently lose the only record of
/// where it belongs, and the loss would surface as a fold-back that cannot
/// find its target.
#[test]
fn the_fold_target_survives_every_frontmatter_writer() {
    use ductus::schema::primitives::{LabelCriteriaArgs, SetStatusArgs, ValidateFrontmatterArgs};

    let tmp = tempfile::tempdir().unwrap();
    seed(tmp.path());
    let spec_path = tmp.path().join("specs").join(BRANCH_SCOPED).join("spec.md");

    // Declare the fold target, plus a criterion for label-criteria to
    // label and a sibling link for derive-dependencies to harvest.
    let original = fs::read_to_string(&spec_path).unwrap();
    let with_target = original.replace(
        "dependencies: []",
        &format!("dependencies: []\nfolds-into: {SEQUENTIAL}"),
    ) + "\n## Acceptance Criteria\n\n- [ ] A criterion to label.\n";
    fs::write(&spec_path, &with_target).unwrap();

    let carries_target = |stage: &str| {
        let body = fs::read_to_string(&spec_path).unwrap();
        assert!(
            body.contains(&format!("folds-into: {SEQUENTIAL}")),
            "fold target lost after {stage}:\n{body}"
        );
    };

    primitives::set_status::run(
        &SetStatusArgs {
            feature: BRANCH_SCOPED.into(),
            from: "draft".into(),
            to: "clarified".into(),
        },
        tmp.path(),
    )
    .unwrap();
    carries_target("set-status");

    primitives::derive_dependencies::run(
        &DeriveDependenciesArgs {
            write: true,
            staged: false,
        },
        tmp.path(),
    )
    .unwrap();
    carries_target("derive-dependencies");

    primitives::label_criteria::run(
        &LabelCriteriaArgs {
            feature: BRANCH_SCOPED.into(),
        },
        tmp.path(),
    )
    .unwrap();
    carries_target("label-criteria");

    // Every writer ran, and each one's own key landed — so the survival
    // above is not the vacuous result of nothing having been written.
    let final_body = fs::read_to_string(&spec_path).unwrap();
    assert!(final_body.contains("status: clarified"), "{final_body}");
    assert!(
        final_body.contains(&format!("dependencies: [{SEQUENTIAL}]")),
        "{final_body}"
    );
    assert!(final_body.contains("next-criterion:"), "{final_body}");

    // And the result still validates: a fold target naming a spec that is
    // present here is as acceptable as one that is not.
    let validated = primitives::validate_frontmatter::run(
        &ValidateFrontmatterArgs {
            path: format!("specs/{BRANCH_SCOPED}/spec.md"),
        },
        tmp.path(),
    )
    .unwrap();
    assert!(validated.clean, "{:?}", validated.findings);
}
