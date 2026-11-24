//! Whose review a record holds: a feature's, or the project's (spec 084).
//!
//! A review primitive called with no `feature` reviews the code no spec
//! claims, under `{specs-root}/review.md`. The subject is resolved once from
//! the argument, and every place a record, a round directory or a list file is
//! named takes it from here, so the label `project` never reaches a builder of
//! feature paths.

use std::path::{Path, PathBuf};

use super::{PrimitiveError, REVIEW_RECORD_FILE, Result};

/// What the project record states as its `spec`, and the name of its round
/// directory and its list files: a name no feature directory can take, since
/// every feature directory carries a number (`parse_feature_dir`).
pub(crate) const PROJECT_SUBJECT: &str = "project";

/// The subject of a review: one feature, or the code no spec claims.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReviewSubject<'a> {
    /// The feature directory of this name.
    Feature(&'a str),
    /// The code no spec claims.
    Project,
}

impl<'a> ReviewSubject<'a> {
    /// The subject a primitive's `feature` argument names. Only an absent
    /// argument is the project: an empty one is a feature name, which every
    /// caller's validation refuses.
    pub(crate) fn of(feature: Option<&'a str>) -> Self {
        feature.map_or(Self::Project, Self::Feature)
    }

    /// The feature directory's name, or none for the project.
    pub(crate) fn feature(self) -> Option<&'a str> {
        match self {
            Self::Feature(feature) => Some(feature),
            Self::Project => None,
        }
    }

    /// The name the subject's record states as its `spec`, and that names its
    /// round directory and its list files.
    pub(crate) fn name(self) -> &'a str {
        self.feature().unwrap_or(PROJECT_SUBJECT)
    }

    /// The directory holding the subject's record, from the project root: the
    /// feature directory, or the spec root itself.
    pub(crate) fn record_dir_rel(self, specs_root: &str) -> String {
        match self {
            Self::Feature(feature) => format!("{specs_root}/{feature}"),
            Self::Project => specs_root.to_string(),
        }
    }

    /// [`Self::record_dir_rel`], on disk under the project root `repo`.
    pub(crate) fn record_dir(self, repo: &Path, specs_root: &str) -> PathBuf {
        repo.join(self.record_dir_rel(specs_root))
    }

    /// The subject a record primitive's `feature` argument names, screened, and
    /// the directory its record lives in. A feature is one whose directory
    /// holds a `spec.md`, which is what makes it a feature; the project's
    /// record lives in the spec root.
    ///
    /// # Errors
    ///
    /// [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute, or
    /// carries a parent-directory component, when the feature directory or a
    /// link inside it resolves outside the project, or when the spec root
    /// does; [`PrimitiveError::FeatureNotFound`] when the feature has no
    /// `spec.md`.
    pub(crate) fn resolve(
        feature: Option<&'a str>,
        repo: &Path,
        specs_root: &str,
    ) -> Result<(Self, PathBuf)> {
        let subject = Self::of(feature);
        match subject {
            Self::Feature(feature) => {
                super::validate_feature_argument(repo, specs_root, feature)?;
            }
            Self::Project => super::validate_contained(repo, specs_root)?,
        }
        let dir = subject.record_dir(repo, specs_root);
        if let Self::Feature(feature) = subject
            && !dir.join("spec.md").is_file()
        {
            return Err(PrimitiveError::FeatureNotFound {
                root: specs_root.to_string(),
                feature: feature.to_string(),
            });
        }
        Ok((subject, dir))
    }

    /// The subject's review record, from the project root.
    pub(crate) fn record_rel(self, specs_root: &str) -> String {
        format!("{}/{REVIEW_RECORD_FILE}", self.record_dir_rel(specs_root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_absent_feature_is_the_project() {
        assert_eq!(ReviewSubject::of(None), ReviewSubject::Project);
        assert_eq!(ReviewSubject::of(Some("")), ReviewSubject::Feature(""));
        assert_eq!(
            ReviewSubject::of(Some("project")),
            ReviewSubject::Feature("project")
        );
    }

    #[test]
    fn the_project_record_sits_in_the_spec_root_and_a_features_in_its_directory() {
        assert_eq!(
            ReviewSubject::Project.record_rel("specs"),
            "specs/review.md"
        );
        assert_eq!(
            ReviewSubject::Feature("001-x").record_rel("docs/specs"),
            "docs/specs/001-x/review.md"
        );
        assert_eq!(ReviewSubject::Project.name(), "project");
        assert_eq!(ReviewSubject::Feature("001-x").name(), "001-x");
        assert!(crate::primitives::parse_feature_dir(PROJECT_SUBJECT).is_none());
    }
}
