//! `check-unfolded-specs` — branch-scoped specs still present in the tree.
//!
//! A branch-scoped directory (`1234.1-slug`) is a staging form: it exists so
//! two branches can each create a spec without claiming the same sequential
//! number, and it is discharged by fold-back into the upstream spec its
//! `folds-into:` names (spec 051). Nothing runs on the merge itself, so the
//! merge that ends a branch's life leaves its staging directories behind and
//! says nothing about them. This check is what notices.
//!
//! Read-only, and it **reports rather than repairs**: which upstream section
//! a spec's content belongs in is a judgement `/{project}:fold` puts to the
//! operator, not one a scan can make.
//!
//! A `folds-into` naming a spec absent from this working tree is reported as
//! declared, not as broken. That absence is the feature's normal case before
//! a merge — a branch-scoped spec exists *because* upstream moved, so its
//! target usually lives on the upstream branch — and this check cannot see
//! the tree that would settle it. Existence is enforced at fold-back by
//! `retire-feature`, which runs after the merge, in the first tree holding
//! both.
//!
//! Defined by `specs/051-branch-scoped-spec-numbering/spec.md` (AC21).

use std::path::Path;

use crate::primitives::{
    FeatureForm, PrimitiveError, Result, list_feature_dirs, parse_feature_dir, read_text,
    split_frontmatter,
};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckUnfoldedSpecsArgs, CheckUnfoldedSpecsResult, Frontmatter, UnfoldedSpec,
};

/// Execute the `check-unfolded-specs` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::MissingSpecFile`] when a branch-scoped feature
/// directory lacks a `spec.md` (the naming convention promises one),
/// [`PrimitiveError::Io`] when that file is unreadable, or
/// [`PrimitiveError::Yaml`] when its frontmatter is malformed. Halting there
/// rather than skipping is deliberate: this check's whole output is what each
/// surviving spec declares, so a directory whose declaration could not be read
/// has no honest row — and dropping it silently would understate the very
/// backlog the check exists to report.
pub fn run(_args: &CheckUnfoldedSpecsArgs, repo: &Path) -> Result<CheckUnfoldedSpecsResult> {
    let layout = paths::Paths::load(repo);
    let specs_dir = repo.join(&layout.specs_root);

    let mut unfolded: Vec<UnfoldedSpec> = Vec::new();
    let mut examined: u32 = 0;

    for slug in list_feature_dirs(&specs_dir) {
        // Counted before the form check: `examined` bounds the claim by the
        // corpus that was walked, so a sequential directory contributes to
        // "we looked at 47 features and none of them staged" even though it
        // can never be a finding.
        examined = examined.saturating_add(1);
        let Some(FeatureForm::BranchScoped { identifier, .. }) = parse_feature_dir(&slug) else {
            continue;
        };

        let spec_path = specs_dir.join(&slug).join("spec.md");
        if !spec_path.is_file() {
            return Err(PrimitiveError::MissingSpecFile {
                root: layout.specs_root.clone(),
                feature: slug,
            });
        }
        let content = read_text(&spec_path)?;
        let (fm_text, _body) = split_frontmatter(&content, &spec_path)?;
        let frontmatter: Frontmatter =
            serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
                path: spec_path.clone(),
                source,
            })?;

        unfolded.push(UnfoldedSpec {
            feature: slug,
            identifier,
            folds_into: frontmatter.folds_into,
            status: frontmatter.status,
        });
    }

    Ok(CheckUnfoldedSpecsResult { unfolded, examined })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::run;
    use crate::primitives::PrimitiveError;
    use crate::schema::primitives::CheckUnfoldedSpecsArgs;
    use std::fs;
    use std::path::Path;

    /// Write a feature directory with a `spec.md` carrying `frontmatter`.
    fn write_spec(repo: &Path, feature: &str, frontmatter: &str) {
        let dir = repo.join("specs").join(feature);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\n{frontmatter}---\n\n# {feature}\n"),
        )
        .unwrap();
    }

    #[test]
    fn a_corpus_without_branch_scoped_specs_is_empty_but_examined() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", "status: done\ndependencies: []\n");
        write_spec(tmp.path(), "051-beta", "status: draft\ndependencies: []\n");

        let result = run(&CheckUnfoldedSpecsArgs {}, tmp.path()).unwrap();

        assert!(result.unfolded.is_empty());
        // The distinction the count exists for: examined-and-clean, not
        // nothing-was-looked-at.
        assert_eq!(result.examined, 2);
    }

    #[test]
    fn an_empty_corpus_is_distinguishable_from_a_clean_one() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();

        let result = run(&CheckUnfoldedSpecsArgs {}, tmp.path()).unwrap();

        assert!(result.unfolded.is_empty());
        assert_eq!(result.examined, 0);
    }

    #[test]
    fn each_surviving_branch_scoped_spec_reports_its_declared_target() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", "status: done\ndependencies: []\n");
        write_spec(
            tmp.path(),
            "1234.1-widget-cache",
            "status: in-progress\ndependencies: []\nfolds-into: 050-alpha\n",
        );
        write_spec(
            tmp.path(),
            "proj-77.2-eviction",
            "status: draft\ndependencies: []\nfolds-into: 099-not-in-this-tree\n",
        );

        let result = run(&CheckUnfoldedSpecsArgs {}, tmp.path()).unwrap();

        assert_eq!(result.examined, 3);
        let rows: Vec<(&str, &str, Option<&str>, &str)> = result
            .unfolded
            .iter()
            .map(|s| {
                (
                    s.feature.as_str(),
                    s.identifier.as_str(),
                    s.folds_into.as_deref(),
                    s.status.as_str(),
                )
            })
            .collect();
        assert_eq!(
            rows,
            vec![
                (
                    "1234.1-widget-cache",
                    "1234",
                    Some("050-alpha"),
                    "in-progress"
                ),
                // A target absent from this tree is still a declaration, not
                // a finding: the tree that would resolve it is the one this
                // branch forked from.
                (
                    "proj-77.2-eviction",
                    "proj-77",
                    Some("099-not-in-this-tree"),
                    "draft"
                ),
            ]
        );
    }

    #[test]
    fn a_spec_that_declares_no_target_reports_none_rather_than_vanishing() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(
            tmp.path(),
            "1234.1-standalone",
            "status: draft\ndependencies: []\n",
        );

        let result = run(&CheckUnfoldedSpecsArgs {}, tmp.path()).unwrap();

        assert_eq!(result.unfolded.len(), 1);
        assert_eq!(result.unfolded[0].folds_into, None);
    }

    #[test]
    fn a_branch_scoped_directory_without_a_spec_halts_rather_than_underreporting() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs/1234.1-empty")).unwrap();

        let err = run(&CheckUnfoldedSpecsArgs {}, tmp.path()).unwrap_err();

        assert!(
            matches!(err, PrimitiveError::MissingSpecFile { ref feature, .. } if feature == "1234.1-empty"),
            "unexpected error: {err:?}"
        );
    }
}
