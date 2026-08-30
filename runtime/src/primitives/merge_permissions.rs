//! `merge-permissions` — idempotently install or update a canonical
//! permission allow/deny set into a JSON file, removing exact-match
//! duplicates from each array. The destination path is host-supplied
//! (typically the bootstrap-substituted `{cli-config-dir}/settings.local.json`,
//! e.g. `.claude/settings.local.json` on Claude Code or
//! `.augment/settings.json` on Auggie); no default — `path` is required.
//!
//! The primitive is the deterministic surface `/configure` calls; see
//! spec 022's `framework-list-dedup` scenario for the contract.
//!
//! Behavior summary:
//!
//! - **File does not exist** → write
//!   `{ "permissions": { "allow": [...canonical], "deny": [...canonical] } }`
//!   and emit `created`.
//! - **File exists, parses as JSON** → dedup exact-match entries in
//!   `permissions.allow` and `permissions.deny`, remove any `revoke`
//!   entry from `permissions.allow`, then ensure every canonical entry
//!   is present (append at end, preserving prior order). If the
//!   post-merge value equals the pre-merge value structurally, emit
//!   `unchanged` and skip the write (preserves mtime for build-tool
//!   idempotency).
//! - **File exists, malformed JSON** → return
//!   [`PrimitiveError::Json`]; do not write.
//! - **`permissions.allow` / `permissions.deny` field exists but is
//!   not an array** → return [`PrimitiveError::JsonSchema`]; do not
//!   write.
//!
//! Atomic writes use the project-wide tempfile + rename helper. Field
//! order under `permissions` follows insertion order via
//! `serde_json`'s `preserve_order` feature.
//!
//! # Retirement (`revoke`)
//!
//! `merge-permissions` installs and dedups; on its own it never removes
//! a non-canonical entry an adopter owns. That is deliberate, and it is
//! also why an entry the framework *once shipped* and has since retired
//! survives in every adopter tree indefinitely — the removal has no
//! owner. `revoke` gives it one: the caller passes the explicit list of
//! formerly-canonical entries, and only exact matches from that list are
//! removed. Shape-matching is not offered, so an adopter who authored
//! their own copy of a retired pattern keeps it.
//!
//! Retirement is **allow-side only**, enforced by the absence of a
//! deny-side counterpart rather than by a documented convention: an
//! over-broad deny entry refuses more rather than approving more, so
//! sweeping both arrays under one argument would invite narrowing the
//! deny set into holes. See spec 027's `retired-permission-entry-cleanup`
//! scenario.

#![allow(clippy::expect_used)]

use std::path::Path;

use serde_json::{Map, Value, json};

use crate::primitives::{PrimitiveError, Result, read_text, validate_no_traversal, write_atomic};
use crate::schema::primitives::{MergePermissionsArgs, MergePermissionsResult};

/// Execute the `merge-permissions` primitive.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `path` is absolute, empty, or
///   contains a parent-directory component — the BE-INPUT-004
///   defense-in-depth check every path-taking primitive applies before
///   filesystem operations. `path` is repo-relative (the
///   bootstrap-substituted `{cli-config-dir}/settings.local.json`); no
///   caller needs an out-of-repo destination.
/// - [`PrimitiveError::Io`] on local filesystem failures.
/// - [`PrimitiveError::Json`] when an existing file fails JSON parse.
/// - [`PrimitiveError::JsonSchema`] when `permissions.allow` /
///   `permissions.deny` exists but is not an array (e.g., null,
///   object, string).
/// - [`PrimitiveError::ConflictingRevoke`] when an entry appears in
///   both `allow` and `revoke`. Checked before any filesystem read,
///   so a contradictory call never leaves a partial write.
pub fn run(args: &MergePermissionsArgs, repo: &Path) -> Result<MergePermissionsResult> {
    validate_no_traversal(&args.path)?;
    validate_revoke_disjoint(&args.allow, &args.revoke)?;
    let target_path = repo.join(&args.path);

    let existing = match target_path.try_exists() {
        Ok(true) => Some(read_text(&target_path)?),
        Ok(false) => None,
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: target_path.clone(),
                source,
            });
        }
    };

    let MergeOutcome {
        post_value,
        action,
        allow_added,
        allow_deduped,
        allow_revoked,
        deny_added,
        deny_deduped,
    } = compute_merge(
        existing.as_deref(),
        &args.allow,
        &args.deny,
        &args.revoke,
        &target_path,
    )?;

    if action != "unchanged" {
        let serialized = serialize_pretty(&post_value);
        write_atomic(&target_path, &serialized)?;
    }

    Ok(MergePermissionsResult {
        path: target_path.to_string_lossy().into_owned(),
        action: action.into(),
        allow_added,
        allow_deduped,
        allow_revoked,
        deny_added,
        deny_deduped,
    })
}

/// Reject a call whose `allow` and `revoke` sets intersect.
///
/// The two passes are irreconcilable for such an entry: revoke removes
/// it, the canonical-presence pass appends it back. Whichever runs last
/// wins, so the primitive would silently honour half the request and —
/// worse — could never emit `unchanged`, defeating the mtime-preserving
/// short-circuit callers rely on for idempotency. Reporting every
/// conflicting entry at once (rather than the first) means a caller
/// fixing a retired-entry list sees the whole overlap in one run.
fn validate_revoke_disjoint(allow: &[String], revoke: &[String]) -> Result<()> {
    // Deduped: a canonical set that repeats an entry would otherwise make
    // the message count one conflict twice and name it twice, reporting a
    // scale of problem the caller does not have.
    let mut conflicts: Vec<&str> = Vec::new();
    for entry in allow {
        if revoke.contains(entry) && !conflicts.contains(&entry.as_str()) {
            conflicts.push(entry.as_str());
        }
    }
    if conflicts.is_empty() {
        return Ok(());
    }
    Err(PrimitiveError::ConflictingRevoke {
        count: conflicts.len(),
        plural: if conflicts.len() == 1 { "y" } else { "ies" },
        entries: conflicts.join(", "),
    })
}

/// Pretty-print with 2-space indent and a trailing newline. The
/// `serde_json` `preserve_order` feature keeps `Map`'s insertion
/// order intact across serialization. `to_string_pretty` is
/// infallible on `serde_json::Value` (no non-string keys, no I/O),
/// so the `.expect` documents the invariant rather than handling a
/// reachable failure mode.
fn serialize_pretty(value: &Value) -> String {
    let mut out =
        serde_json::to_string_pretty(value).expect("serde_json::Value serializes infallibly");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

struct MergeOutcome {
    post_value: Value,
    action: &'static str,
    allow_added: u32,
    allow_deduped: u32,
    allow_revoked: u32,
    deny_added: u32,
    deny_deduped: u32,
}

fn compute_merge(
    existing: Option<&str>,
    canonical_allow: &[String],
    canonical_deny: &[String],
    revoke: &[String],
    path: &Path,
) -> Result<MergeOutcome> {
    match existing {
        // Nothing on disk to retire: a fresh file is written from the
        // canonical sets, which `validate_revoke_disjoint` has already
        // proven share no member with `revoke`.
        None => Ok(fresh_merge(canonical_allow, canonical_deny)),
        Some(text) => existing_merge(text, canonical_allow, canonical_deny, revoke, path),
    }
}

fn fresh_merge(canonical_allow: &[String], canonical_deny: &[String]) -> MergeOutcome {
    let allow_added = u32::try_from(canonical_allow.len()).unwrap_or(u32::MAX);
    let deny_added = u32::try_from(canonical_deny.len()).unwrap_or(u32::MAX);
    let post_value = json!({
        "permissions": {
            "allow": canonical_allow,
            "deny": canonical_deny,
        }
    });
    MergeOutcome {
        post_value,
        action: "created",
        allow_added,
        allow_deduped: 0,
        allow_revoked: 0,
        deny_added,
        deny_deduped: 0,
    }
}

fn existing_merge(
    text: &str,
    canonical_allow: &[String],
    canonical_deny: &[String],
    revoke: &[String],
    path: &Path,
) -> Result<MergeOutcome> {
    let original: Value = serde_json::from_str(text).map_err(|source| PrimitiveError::Json {
        path: path.into(),
        source,
    })?;

    let mut post_value = original.clone();
    let permissions = ensure_permissions_object(&mut post_value, path)?;

    let ArrayOutcome {
        added: allow_added,
        deduped: allow_deduped,
        revoked: allow_revoked,
    } = merge_array(permissions, "allow", canonical_allow, revoke, path)?;
    // Deny is never a retirement subject; see the module's Retirement note.
    let ArrayOutcome {
        added: deny_added,
        deduped: deny_deduped,
        revoked: _,
    } = merge_array(permissions, "deny", canonical_deny, &[], path)?;

    let action = if post_value == original {
        "unchanged"
    } else {
        "updated"
    };

    Ok(MergeOutcome {
        post_value,
        action,
        allow_added,
        allow_deduped,
        allow_revoked,
        deny_added,
        deny_deduped,
    })
}

/// Ensure the top-level value has a `permissions` object, returning a
/// mutable reference to its `Map`. If the top-level value is not an
/// object, return a schema error rather than silently overwriting.
fn ensure_permissions_object<'a>(
    value: &'a mut Value,
    path: &Path,
) -> Result<&'a mut Map<String, Value>> {
    let Some(root) = value.as_object_mut() else {
        return Err(PrimitiveError::JsonSchema {
            path: path.into(),
            reason: "top-level value is not a JSON object".into(),
        });
    };
    let permissions = root.entry("permissions").or_insert_with(|| json!({}));
    match permissions.as_object_mut() {
        Some(map) => Ok(map),
        None => Err(PrimitiveError::JsonSchema {
            path: path.into(),
            reason: "`permissions` field exists but is not a JSON object".into(),
        }),
    }
}

/// Per-array counts returned by [`merge_array`].
struct ArrayOutcome {
    added: u32,
    deduped: u32,
    revoked: u32,
}

/// Apply the revoke + dedup + canonical-presence passes to one array
/// field on `permissions`. `field` is `"allow"` or `"deny"`; `revoke`
/// is always empty for `"deny"` (retirement is allow-side only).
///
/// Pass order is revoke-first, and it is load-bearing for the counts.
/// Deduping first would remove the second copy of a doubled retired
/// entry as a *duplicate* and the first as a *retirement*, splitting one
/// cause across two counters and crediting the dedup pass with work it
/// did not conceptually do. Revoking first attributes every copy to the
/// reason it actually went, and leaves `deduped` to mean what it says:
/// duplicates among the entries that survive.
fn merge_array(
    permissions: &mut Map<String, Value>,
    field: &str,
    canonical: &[String],
    revoke: &[String],
    path: &Path,
) -> Result<ArrayOutcome> {
    let Some(array_value) = permissions.get_mut(field) else {
        permissions.insert(
            field.into(),
            Value::Array(canonical.iter().map(|s| Value::String(s.clone())).collect()),
        );
        let added = u32::try_from(canonical.len()).unwrap_or(u32::MAX);
        // No array on disk means nothing to retire, so `revoke` has no
        // subject here even when non-empty.
        return Ok(ArrayOutcome {
            added,
            deduped: 0,
            revoked: 0,
        });
    };

    let Some(arr) = array_value.as_array_mut() else {
        return Err(PrimitiveError::JsonSchema {
            path: path.into(),
            reason: format!("`permissions.{field}` exists but is not an array"),
        });
    };

    // Revoke pass: drop every copy of a formerly-canonical entry, by
    // exact string match. Runs before dedup so a doubled retired entry
    // is attributed wholly to retirement (see the note above).
    let mut revoked = 0u32;
    if !revoke.is_empty() {
        let mut idx = 0;
        while idx < arr.len() {
            let retire = arr[idx]
                .as_str()
                .is_some_and(|entry| revoke.iter().any(|r| r == entry));
            if retire {
                arr.remove(idx);
                revoked = revoked.saturating_add(1);
                continue;
            }
            idx += 1;
        }
    }

    // Dedup pass: first occurrence wins; later duplicates removed in place.
    let mut seen: Vec<String> = Vec::with_capacity(arr.len());
    let mut deduped = 0u32;
    let mut idx = 0;
    while idx < arr.len() {
        if let Some(s) = arr[idx].as_str() {
            let s_owned = s.to_string();
            if seen.contains(&s_owned) {
                arr.remove(idx);
                deduped = deduped.saturating_add(1);
                continue;
            }
            seen.push(s_owned);
        }
        // Non-string entries are preserved verbatim and not considered
        // for dedup. The canonical set is string-valued; non-string
        // entries don't collide.
        idx += 1;
    }

    // Canonical-presence pass: append any canonical entry not already
    // present (by string-equality), preserving canonical-set order.
    let mut added = 0u32;
    for entry in canonical {
        if !seen.contains(entry) {
            arr.push(Value::String(entry.clone()));
            seen.push(entry.clone());
            added = added.saturating_add(1);
        }
    }

    Ok(ArrayOutcome {
        added,
        deduped,
        revoked,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;

    fn args(path: &str, allow: &[&str], deny: &[&str]) -> MergePermissionsArgs {
        args_revoking(path, allow, deny, &[])
    }

    fn args_revoking(
        path: &str,
        allow: &[&str],
        deny: &[&str],
        revoke: &[&str],
    ) -> MergePermissionsArgs {
        MergePermissionsArgs {
            path: path.to_string(),
            allow: allow.iter().map(|s| (*s).to_string()).collect(),
            deny: deny.iter().map(|s| (*s).to_string()).collect(),
            revoke: revoke.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    /// Read back `permissions.allow` as owned strings.
    fn allow_of(path: &std::path::Path) -> Vec<String> {
        let v: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        v["permissions"]["allow"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e.as_str().unwrap().to_string())
            .collect()
    }

    /// The nine entries the `retired-permission-entries` migration
    /// retires: seven wildcard-before-subcommand git allows (spec 023
    /// task 21) and two inert `Write(path)` allows (spec 023's
    /// `configure-inert-write-path-entries`).
    const RETIRED: &[&str] = &[
        "Bash(git -C * add *)",
        "Bash(git -C * commit *)",
        "Bash(git -C * push *)",
        "Bash(git -C * log *)",
        "Bash(git -C * diff *)",
        "Bash(git -C * status *)",
        "Bash(git -C * show *)",
        "Write(.ductus/session.toml)",
        "Write(.ductus/config.toml)",
    ];

    #[test]
    fn creates_file_when_absent_with_canonical_set() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(".claude/settings.local.json");
        let result = run(
            &args(
                ".claude/settings.local.json",
                &["Edit", "Bash(ls *)"],
                &["Bash(rm -rf *)"],
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "created");
        assert_eq!(result.allow_added, 2);
        assert_eq!(result.deny_added, 1);
        assert_eq!(result.allow_deduped, 0);
        assert_eq!(result.deny_deduped, 0);

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["permissions"]["allow"][0], "Edit");
        assert_eq!(parsed["permissions"]["allow"][1], "Bash(ls *)");
        assert_eq!(parsed["permissions"]["deny"][0], "Bash(rm -rf *)");
    }

    #[test]
    fn writes_to_host_supplied_path_for_non_claude_adopter() {
        // Auggie keeps its permissions at `.augment/settings.json`; the
        // primitive writes wherever the caller says without baking in a
        // Claude-shaped default.
        let tmp = tempfile::tempdir().unwrap();
        let result = run(&args(".augment/settings.json", &["Edit"], &[]), tmp.path()).unwrap();
        assert!(result.path.ends_with(".augment/settings.json"));
        assert_eq!(result.action, "created");
        assert!(tmp.path().join(".augment/settings.json").is_file());
        assert!(
            !tmp.path().join(".claude").exists(),
            "Auggie write must not create a Claude-shaped sibling"
        );
    }

    #[test]
    fn dedups_existing_duplicates() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("settings.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["Edit","Bash(ls *)","Edit","Write","Edit"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(
            &args("settings.json", &["Edit", "Bash(ls *)"], &[]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_added, 0);
        assert_eq!(
            result.allow_deduped, 2,
            "two extra 'Edit' entries should be removed"
        );

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        assert_eq!(allow.len(), 3, "Edit + Bash(ls *) + Write after dedup");
        assert_eq!(allow[0], "Edit", "first occurrence wins");
        assert_eq!(allow[1], "Bash(ls *)");
        assert_eq!(allow[2], "Write");
    }

    #[test]
    fn dedup_includes_non_canonical_entries() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["UserAdded","UserAdded","Other","UserAdded"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &[], &[]), tmp.path()).unwrap();
        assert_eq!(result.allow_deduped, 2);
        assert_eq!(result.allow_added, 0);
        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        assert_eq!(allow.len(), 2);
        assert_eq!(allow[0], "UserAdded");
        assert_eq!(allow[1], "Other");
    }

    #[test]
    fn appends_missing_canonical_at_end_preserving_order() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["UserA","UserB"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(
            &args("s.json", &["Canonical1", "Canonical2"], &[]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_added, 2);
        assert_eq!(result.allow_deduped, 0);

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        assert_eq!(
            allow
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["UserA", "UserB", "Canonical1", "Canonical2"]
        );
    }

    #[test]
    fn canonical_present_is_not_re_appended() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["Canonical1","UserA"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(
            &args("s.json", &["Canonical1", "Canonical2"], &[]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.allow_added, 1, "only Canonical2 was missing");
        assert_eq!(result.allow_deduped, 0);
    }

    #[test]
    fn unchanged_when_canonical_present_and_no_duplicates() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        let original = r#"{
  "permissions": {
    "allow": [
      "Edit",
      "Bash(ls *)"
    ],
    "deny": [
      "Bash(rm -rf *)"
    ]
  }
}
"#;
        fs::write(&path, original).unwrap();
        let mtime_before = fs::metadata(&path).unwrap().modified().unwrap();

        let result = run(
            &args("s.json", &["Edit", "Bash(ls *)"], &["Bash(rm -rf *)"]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "unchanged");
        assert_eq!(result.allow_added, 0);
        assert_eq!(result.allow_deduped, 0);

        // mtime preserved (no write happened).
        assert_eq!(
            fs::metadata(&path).unwrap().modified().unwrap(),
            mtime_before
        );

        // File content untouched byte-for-byte.
        assert_eq!(fs::read_to_string(&path).unwrap(), original);
    }

    #[test]
    fn preserves_untouched_top_level_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{
  "permissions": {
    "allow": ["UserA"],
    "deny": [],
    "additionalDirectories": ["/foo", "/bar"]
  },
  "defaultMode": "default",
  "customField": {"nested": "value"}
}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &["Canonical1"], &[]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["defaultMode"], "default");
        assert_eq!(parsed["customField"]["nested"], "value");
        assert_eq!(parsed["permissions"]["additionalDirectories"][0], "/foo");
        assert_eq!(parsed["permissions"]["additionalDirectories"][1], "/bar");
    }

    #[test]
    fn missing_permissions_object_is_added() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, r#"{"defaultMode": "default"}"#).unwrap();
        let result = run(&args("s.json", &["Edit"], &["Bash(rm -rf *)"]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["defaultMode"], "default");
        assert_eq!(parsed["permissions"]["allow"][0], "Edit");
        assert_eq!(parsed["permissions"]["deny"][0], "Bash(rm -rf *)");
    }

    #[test]
    fn missing_allow_array_is_seeded() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, r#"{"permissions":{"deny":["Existing"]}}"#).unwrap();
        let result = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_added, 1);

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["permissions"]["allow"][0], "Edit");
        assert_eq!(parsed["permissions"]["deny"][0], "Existing");
    }

    #[test]
    fn absolute_path_is_rejected_before_any_write() {
        let tmp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let target = outside.path().join("settings.json");
        let err = run(&args(&target.to_string_lossy(), &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidPath { .. }),
            "expected InvalidPath, got {err:?}"
        );
        assert!(!target.exists(), "nothing may be written outside the repo");
    }

    #[test]
    fn traversal_path_is_rejected_before_any_write() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(&args("../settings.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }

    #[test]
    fn malformed_json_returns_parse_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, r#"{"permissions": {"allow": [oops}"#).unwrap();
        let err = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Json { .. }));
        // File should be unchanged.
        let body = fs::read_to_string(&path).unwrap();
        assert!(body.contains("oops"));
    }

    #[test]
    fn non_array_allow_returns_schema_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions": {"allow": "not-an-array", "deny": []}}"#,
        )
        .unwrap();
        let err = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        match err {
            PrimitiveError::JsonSchema { reason, .. } => {
                assert!(reason.contains("permissions.allow"));
                assert!(reason.contains("not an array"));
            }
            other => panic!("expected JsonSchema, got {other:?}"),
        }
    }

    #[test]
    fn non_object_top_level_returns_schema_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, "[]").unwrap();
        let err = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::JsonSchema { .. }));
    }

    #[test]
    fn deny_array_is_independently_handled() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":[],"deny":["Bash(rm -rf *)","Bash(rm -rf *)","Other"]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &[], &["Bash(rm -rf *)"]), tmp.path()).unwrap();
        assert_eq!(result.deny_deduped, 1);
        assert_eq!(result.deny_added, 0);
        assert_eq!(result.allow_added, 0);
        assert_eq!(result.allow_deduped, 0);
    }

    #[test]
    fn non_string_entries_preserved_and_ignored_for_dedup() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["Edit",42,"Edit",null,"Edit"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &[], &[]), tmp.path()).unwrap();
        assert_eq!(result.allow_deduped, 2, "duplicate 'Edit' entries removed");

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        // After dedup: "Edit", 42, null
        assert_eq!(allow.len(), 3);
        assert_eq!(allow[0], "Edit");
        assert_eq!(allow[1], 42);
        assert_eq!(allow[2], Value::Null);
    }
    // -- retirement (`revoke`) ------------------------------------------

    #[test]
    fn revoke_removes_every_retired_entry_reported_by_the_host_linter() {
        // The exact nine entries Claude Code warns about at session start
        // in a tree configured before spec 023 task 21 and before
        // `configure-inert-write-path-entries`.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join(".claude/settings.local.json");
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        let mut pre: Vec<&str> = vec!["Edit", "Edit(.ductus/session.toml)", "Bash(ls *)"];
        pre.extend_from_slice(RETIRED);
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": { "allow": pre, "deny": ["Bash(rm -rf *)"] }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking(
                ".claude/settings.local.json",
                &["Edit", "Edit(.ductus/session.toml)", "Bash(ls *)"],
                &["Bash(rm -rf *)"],
                RETIRED,
            ),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_revoked, 9, "all nine retired entries removed");
        assert_eq!(result.allow_added, 0, "canonical set was already present");
        assert_eq!(result.allow_deduped, 0);
        assert_eq!(
            allow_of(&target),
            vec!["Edit", "Edit(.ductus/session.toml)", "Bash(ls *)"],
            "survivors keep their original order"
        );
    }

    #[test]
    fn revoke_never_reaches_the_deny_array() {
        // Retirement is allow-side only *by construction*: the deny array
        // is merged with an empty revoke list, so even an entry named in
        // `revoke` survives there. An over-broad deny refuses more rather
        // than approving more, so narrowing it would open a hole.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": {
                    "allow": ["Bash(git -C * rm *)"],
                    "deny": ["Bash(git -C * rm *)"],
                }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking(
                "settings.json",
                &[],
                &["Bash(git -C * rm *)"],
                &["Bash(git -C * rm *)"],
            ),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 1, "removed from allow");
        let v: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        assert_eq!(
            v["permissions"]["deny"][0], "Bash(git -C * rm *)",
            "the identical string is a correct deny entry and must survive"
        );
        assert_eq!(
            v["permissions"]["allow"].as_array().unwrap().len(),
            0,
            "and must be gone from allow"
        );
    }

    #[test]
    fn revoke_matches_exactly_and_spares_adopter_authored_entries() {
        // Shape-matching is deliberately not offered. An adopter who wrote
        // their own near-miss of a retired pattern keeps it; only what the
        // framework itself shipped is removed.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        let adopter = [
            "Bash(git -C * status)",         // no trailing wildcard
            "Bash(git -C ~/other status *)", // pinned path, not a wildcard
            "bash(git -C * status *)",       // different tool casing
            "Write(.ductus/session.toml )",  // stray space
        ];
        let mut pre = vec!["Bash(git -C * status *)"];
        pre.extend_from_slice(&adopter);
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({ "permissions": { "allow": pre } })).unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking("settings.json", &[], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 1, "only the exact match went");
        assert_eq!(
            allow_of(&target),
            adopter.to_vec(),
            "every adopter-authored near-miss survives untouched"
        );
    }

    #[test]
    fn revoke_removes_every_copy_of_a_doubled_retired_entry() {
        // Revoke runs before dedup, so both copies are attributed to the
        // retirement rather than one being credited to the dedup pass.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": {
                    "allow": [
                        "Write(.ductus/config.toml)",
                        "Edit",
                        "Write(.ductus/config.toml)",
                    ]
                }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 2, "both copies count as retired");
        assert_eq!(
            result.allow_deduped, 0,
            "and neither is miscounted as a duplicate"
        );
        assert_eq!(allow_of(&target), vec!["Edit"]);
    }

    #[test]
    fn revoke_is_a_silent_no_op_on_an_already_clean_file() {
        // The second `/ductus` run, and every fresh adopter. `unchanged`
        // skips the write, preserving mtime.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        let clean = json!({ "permissions": { "allow": ["Edit"], "deny": [] } });
        fs::write(&target, serialize_pretty(&clean)).unwrap();
        let before = fs::metadata(&target).unwrap().modified().unwrap();

        let result = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.action, "unchanged");
        assert_eq!(result.allow_revoked, 0);
        assert_eq!(
            fs::metadata(&target).unwrap().modified().unwrap(),
            before,
            "an unchanged merge must not rewrite the file"
        );
    }

    #[test]
    fn revoke_reaches_a_fixed_point_on_the_second_run() {
        // Idempotency is what makes the migration safe to re-run: the
        // adopter's next `/ductus` must report `unchanged`, not churn.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        let mut pre = vec!["Bash(ls *)"];
        pre.extend_from_slice(RETIRED);
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({ "permissions": { "allow": pre } })).unwrap(),
        )
        .unwrap();
        let call = args_revoking("settings.json", &["Bash(ls *)", "Edit"], &[], RETIRED);

        let first = run(&call, tmp.path()).unwrap();
        assert_eq!(first.action, "updated");
        assert_eq!(first.allow_revoked, 9);
        assert_eq!(first.allow_added, 1, "`Edit` appended");

        let second = run(&call, tmp.path()).unwrap();
        assert_eq!(second.action, "unchanged");
        assert_eq!(second.allow_revoked, 0);
        assert_eq!(second.allow_added, 0);
    }

    #[test]
    fn revoke_on_an_absent_file_writes_the_canonical_set_only() {
        let tmp = tempfile::tempdir().unwrap();
        let result = run(
            &args_revoking("settings.json", &["Edit"], &["Bash(rm -rf *)"], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.action, "created");
        assert_eq!(
            result.allow_revoked, 0,
            "a file that did not exist has nothing to retire"
        );
        assert_eq!(allow_of(&tmp.path().join("settings.json")), vec!["Edit"]);
    }

    #[test]
    fn revoke_preserves_unrelated_keys_and_sibling_permission_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "$schema": "https://example.test/schema.json",
                "model": "opus",
                "permissions": {
                    "allow": ["Write(.ductus/config.toml)", "Edit"],
                    "ask": ["Bash(curl *)"],
                    "additionalDirectories": ["/abs/specs"],
                }
            }))
            .unwrap(),
        )
        .unwrap();

        run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        let v: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        assert_eq!(v["$schema"], "https://example.test/schema.json");
        assert_eq!(v["model"], "opus");
        assert_eq!(v["permissions"]["ask"][0], "Bash(curl *)");
        assert_eq!(v["permissions"]["additionalDirectories"][0], "/abs/specs");
        assert_eq!(allow_of(&target), vec!["Edit"]);
    }

    #[test]
    fn an_entry_in_both_allow_and_revoke_is_rejected_before_any_write() {
        // The two passes would fight, so the merge could never reach a
        // fixed point. Caught before the file is read, so a contradictory
        // call cannot leave a partial write behind.
        let tmp = tempfile::tempdir().unwrap();
        let err = run(
            &args_revoking(
                "settings.json",
                &["Edit", "Write(.ductus/config.toml)"],
                &[],
                RETIRED,
            ),
            tmp.path(),
        )
        .unwrap_err();

        let message = err.to_string();
        assert!(
            message.contains("Write(.ductus/config.toml)"),
            "names the offending entry: {message}"
        );
        assert!(message.contains("entry appear"), "singular form: {message}");
        assert!(
            !tmp.path().join("settings.json").exists(),
            "a rejected call writes nothing"
        );
    }

    #[test]
    fn conflicting_revoke_reports_every_overlap_at_once() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(
            &args_revoking(
                "settings.json",
                &["Write(.ductus/config.toml)", "Write(.ductus/session.toml)"],
                &[],
                RETIRED,
            ),
            tmp.path(),
        )
        .unwrap_err();

        let message = err.to_string();
        assert!(message.contains("2 entries appear"), "plural: {message}");
        assert!(message.contains("Write(.ductus/config.toml)"), "{message}");
        assert!(message.contains("Write(.ductus/session.toml)"), "{message}");
    }

    #[test]
    fn revoke_survives_non_string_entries_in_the_allow_array() {
        // The array is adopter-owned; a hand-edited file can hold a
        // non-string. Retirement must skip it rather than panic.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": {
                    "allow": [42, "Write(.ductus/config.toml)", Value::Null, "Edit"]
                }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 1);
        let v: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        let allow = v["permissions"]["allow"].as_array().unwrap();
        assert_eq!(allow[0], 42, "non-strings preserved verbatim");
        assert_eq!(allow[1], Value::Null);
        assert_eq!(allow[2], "Edit");
    }

    #[test]
    fn revoke_on_a_malformed_file_refuses_without_writing() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(&target, "{ not json").unwrap();

        let err = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap_err();

        assert!(matches!(err, PrimitiveError::Json { .. }));
        assert_eq!(
            fs::read_to_string(&target).unwrap(),
            "{ not json",
            "a hand-edited file is reported, never silently rewritten"
        );
    }
}
