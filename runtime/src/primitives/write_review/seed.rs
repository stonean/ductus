//! The project record's seed (spec 084): a review record no review wrote, so
//! the window for code no spec claims has a start.
//!
//! An install or an update writes it once. The window starts after the commit
//! that adds the record to history (`check_unreviewed_code::project_window`),
//! so `reviewed-against` here is provenance: the `HEAD` the seed was written
//! at, substituted by the runtime and never passed in.

use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::review_subject::ReviewSubject;
use crate::primitives::{PrimitiveError, ProjectRepository, Result};
use crate::schema::primitives::{SeedOutcome, WriteReviewArgs, WriteReviewResult};

/// Write the seed at `{root}/review.md`, or leave the record already there as
/// it is, a seed or a review's, and say which: a second run writes nothing.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] naming `seed` when any other argument
/// is given, a feature or a finding among them; [`PrimitiveError::InvalidPath`]
/// when the spec root or the record resolves outside the project;
/// [`PrimitiveError::Git`] for a project no work tree contains or a `HEAD` that
/// cannot be read; [`PrimitiveError::ChangedSinceRead`] when a record appeared
/// after this call found none (`BE-TXN-002`); and [`PrimitiveError::Io`] on a
/// read or write failure.
pub(super) fn write(args: &WriteReviewArgs, repo: &Path, root: &str) -> Result<WriteReviewResult> {
    let alone = WriteReviewArgs {
        seed: true,
        ..WriteReviewArgs::default()
    };
    if *args != alone {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "write-review".into(),
            argument: "seed".into(),
            reason: "a seed is the project record's alone and takes no other argument: no \
                     feature, no run's fields and no finding"
                .into(),
        });
    }
    let rel = ReviewSubject::Project.record_rel(root);
    let record = crate::primitives::ReadSnapshot::read(repo, &rel)?;
    let outcome = if record.bytes().is_some() {
        SeedOutcome::Kept
    } else {
        let project = ProjectRepository::discover(repo)?;
        record.write_if_unchanged(render(head_of(&project)?.as_deref()))?;
        SeedOutcome::Written
    };
    Ok(WriteReviewResult {
        path: rel,
        seed: Some(outcome),
        ..WriteReviewResult::default()
    })
}

/// `HEAD`'s sha, or `None` on an unborn branch, where no commit exists yet.
fn head_of(project: &ProjectRepository) -> Result<Option<String>> {
    match project.repository.head() {
        Ok(head) => Ok(Some(head.peel_to_commit()?.id().to_string())),
        Err(err) if err.code() == git2::ErrorCode::UnbornBranch => Ok(None),
        Err(err) => Err(err.into()),
    }
}

/// The seed's text. `reviewed-against` is absent when the repository has no
/// commit: there is nothing to name, and the window's start does not read it.
fn render(head: Option<&str>) -> String {
    let name = ReviewSubject::Project.name();
    let mut text = format!("---\nspec: {name}\nlast-run: null\n");
    if let Some(head) = head {
        let _ = writeln!(text, "reviewed-against: {head}");
    }
    let before = head.map_or_else(
        || "No commit existed when it was written, so no code came before it.".to_string(),
        |head| format!("Code no spec claimed before `{head}` was not reviewed."),
    );
    let _ = write!(
        text,
        "must-violations: 0\nshould-violations: 0\nlow-confidence: 0\nscope: 0\n\
         blocking: false\n---\n\n# Review — {name}\n\n## Summary\n\n\
         No review ran: this record is a seed, written so the window for code no spec claims \
         has a start, which is after the commit that adds this file. {before}\n"
    );
    text
}
