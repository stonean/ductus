//! `resolve-session` — resolve the calling process's session target
//! (spec 062).
//!
//! Before spec 062 every targeted command read `.ductus/session.toml` with
//! the host's file tool. That read cannot be made per-process: the identity
//! lives in the environment, which only the runtime process can see on every
//! path. So resolution is a primitive, and the commands invoke it.
//!
//! The read has side effects by necessity — pinning an adopted default,
//! refreshing `used-at`, and consuming once-only notices are all state
//! changes — and every one of them lives in [`crate::session::resolve`].
//! This primitive reads the identity at the edge and reports.

use std::path::Path;
use std::time::SystemTime;

use crate::primitives::{Result, rel_path};
use crate::schema::primitives::{ResolveSessionArgs, ResolveSessionResult};
use crate::session::{self, Identity};

/// Execute the `resolve-session` primitive against `repo`, as the process
/// whose environment this runtime inherited.
///
/// # Errors
///
/// [`crate::primitives::PrimitiveError::InvalidArgument`] when
/// `DUCTUS_SESSION` sanitizes to nothing;
/// [`crate::primitives::PrimitiveError::Toml`] naming the file when the
/// process's own target, or the default it would adopt, does not parse;
/// [`crate::primitives::PrimitiveError::Io`] on a failed read, lock or write.
pub fn run(_args: &ResolveSessionArgs, repo: &Path) -> Result<ResolveSessionResult> {
    let identity = session::process_identity()?;
    run_as(repo, identity.as_ref(), SystemTime::now())
}

/// Resolve as `identity` at `now`: the one path [`run`] and the tests share.
pub(crate) fn run_as(
    repo: &Path,
    identity: Option<&Identity>,
    now: SystemTime,
) -> Result<ResolveSessionResult> {
    let resolution = session::resolve(repo, identity, now)?;
    Ok(ResolveSessionResult {
        identity: resolution.identity,
        source: resolution.source,
        target: resolution.target,
        notices: resolution.notices,
        unreadable: resolution
            .unreadable
            .iter()
            .map(|path| rel_path(path, repo))
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::primitives::{SessionNoticeKind, SessionSource};
    use std::fs;
    use std::time::{Duration, UNIX_EPOCH};
    use tempfile::tempdir;

    fn now() -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(1_790_683_200)
    }

    fn repo_with_default(body: &str) -> tempfile::TempDir {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(tmp.path().join(".ductus/session.toml"), body).unwrap();
        tmp
    }

    #[test]
    fn an_unidentified_process_reads_the_default() {
        let tmp = repo_with_default(
            "feature = \"055-a\"\npath = \"specs/055-a\"\nscenario = \"s\"\n\
             scenario-path = \"specs/055-a/scenarios/s.md\"\n",
        );
        let result = run_as(tmp.path(), None, now()).unwrap();
        assert_eq!(result.identity, None);
        assert_eq!(result.source, SessionSource::Default);
        let target = result.target.unwrap();
        assert_eq!(target.display(), "055-a/s");
        assert_eq!(
            target.scenario_path.as_deref(),
            Some("specs/055-a/scenarios/s.md")
        );
        assert_eq!(
            result.notices,
            [] as [crate::schema::primitives::SessionNotice; 0]
        );
    }

    #[test]
    fn an_identified_process_adopts_and_reports_the_notice() {
        let tmp = repo_with_default("feature = \"055-a\"\npath = \"specs/055-a\"\n");
        let review = Identity::named("review").unwrap();
        let result = run_as(tmp.path(), Some(&review), now()).unwrap();
        assert_eq!(result.identity.as_deref(), Some("review"));
        assert_eq!(result.source, SessionSource::Adopted);
        assert_eq!(result.notices[0].kind, SessionNoticeKind::Adopted);
    }

    #[test]
    fn a_peer_file_that_does_not_parse_is_named_repo_relative() {
        let tmp = repo_with_default("feature = \"055-a\"\npath = \"specs/055-a\"\n");
        let review = Identity::named("review").unwrap();
        run_as(tmp.path(), Some(&review), now()).unwrap();
        fs::write(
            tmp.path().join(".ductus/sessions/broken.toml"),
            "feature = [",
        )
        .unwrap();
        let result = run_as(tmp.path(), Some(&review), now()).unwrap();
        assert_eq!(result.unreadable, [".ductus/sessions/broken.toml"]);
        let value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["unreadable"][0], ".ductus/sessions/broken.toml");
    }

    #[test]
    fn no_target_anywhere_is_none_not_an_error() {
        let tmp = tempdir().unwrap();
        let result = run_as(tmp.path(), None, now()).unwrap();
        assert_eq!((result.source, result.target), (SessionSource::None, None));
    }

    #[test]
    fn the_result_serializes_to_the_data_model_shape() {
        let tmp = repo_with_default("feature = \"055-a\"\npath = \"specs/055-a\"\n");
        let review = Identity::named("review").unwrap();
        let value =
            serde_json::to_value(run_as(tmp.path(), Some(&review), now()).unwrap()).unwrap();
        assert_eq!(value["identity"], "review");
        assert_eq!(value["source"], "adopted");
        assert_eq!(value["target"]["feature"], "055-a");
        assert_eq!(value["target"]["scenario"], serde_json::Value::Null);
        assert_eq!(value["notices"][0]["kind"], "adopted");
        assert!(value["notices"][0]["message"].is_string());
    }
}
