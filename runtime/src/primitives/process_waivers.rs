//! `process-waivers` — deterministic per-run waiver processing for `/ductus:review`.
//!
//! Reads the `waivers` list from `review.md`'s frontmatter and classifies each
//! anchor against the currently-firing `(rule, file)` findings. An entry's
//! `file` is one path or a list of them, and each listed path is its own
//! anchor, so one entry can land in several of these buckets:
//!
//! - **apply** — the anchored file exists AND the rule still fires there.
//! - **expire** — the file is gone OR the rule no longer fires there. The
//!   path drops on the next frontmatter write, and the entry with its last
//!   path (write-review's job); this primitive emits the `waiver expired: …`
//!   notice and reports the anchor.
//! - **retain** — a non-firing waiver on a **dimension-restricted** run
//!   (`skipped-passes` non-empty). The run lacks the full-review picture, so
//!   the waiver is left untouched (never expired/pruned) — this honors the
//!   contract that waivers anchored to skipped dimensions apply unchanged.
//!   Only an unrestricted run can expire a waiver.
//! - **malformed** — a field is missing/empty; warn and skip, and never
//!   report it expired. `write-review` compares anchors only, so a malformed
//!   waiver still loses each path whose `(rule, file)` an expired one shares.
//! - **duplicate** — a repeated `(rule, file)` pair, within an entry or across
//!   entries; only the first claim applies, and the same entry's other paths
//!   are still classified.
//!
//! The anchor is the `(rule, file)` pair only — line numbers are not part of
//! it, so code moving within a file does not expire a waiver. Read-only:
//! frontmatter mutation belongs to `write-review`.
//!
//! Defined by
//! `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`;
//! the per-path anchors by
//! `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`.

use std::path::Path;

use serde::Deserialize;

use crate::primitives::{PrimitiveError, Result, WaiverPaths};
use crate::schema::paths;
use crate::schema::primitives::{ProcessWaiversArgs, ProcessWaiversResult, WaiverRef};

/// Required waiver fields, checked in this order; the first missing/empty one
/// names the `malformed …` diagnostic.
const REQUIRED_FIELDS: &[&str] = &["rule", "file", "reason", "waived-at", "waived-by"];

/// Execute the `process-waivers` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute,
/// or carries a parent-directory component, [`PrimitiveError::FeatureNotFound`]
/// when the feature has no `spec.md`, [`PrimitiveError::UnclosedFrontmatter`]
/// when `review.md`'s frontmatter block never closes, [`PrimitiveError::Yaml`]
/// when its frontmatter or `waivers:` list fails to parse, or
/// [`PrimitiveError::Io`] on read failure. An absent `review.md`, or one that
/// opens no frontmatter block, records no waivers.
pub fn run(args: &ProcessWaiversArgs, repo: &Path) -> Result<ProcessWaiversResult> {
    super::validate_feature_argument(repo, &args.feature)?;
    let layout = paths::Paths::load(repo);
    let spec_path = repo
        .join(&layout.specs_root)
        .join(&args.feature)
        .join("spec.md");
    if !spec_path.is_file() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root,
            feature: args.feature.clone(),
        });
    }
    // The waivers live in `review.md`'s frontmatter, beside the rest of the
    // review record (spec 057). `spec.md` is still read above, because its
    // existence is what makes this a feature at all.
    let waivers: Vec<RawWaiver> =
        super::read_recorded_waivers(spec_path.parent().unwrap_or_else(|| Path::new(".")))?;

    // A dimension-restricted run (any pass skipped) cannot see the full set
    // of findings, so it must not expire a waiver on the strength of "the
    // rule didn't fire" — the rule's dimension may simply not have run. This
    // is deliberately the *coarse* invariant: a restricted run defers ALL
    // waiver garbage-collection to the next unrestricted run, including a
    // waiver whose anchored file is gone (which is dimension-independent and
    // could in principle expire safely). Retaining it too keeps the rule
    // simple — a scoped review never mutates the durable `review.waivers`
    // list — and the only cost is a provably-dead waiver lingering until the
    // next full run, which then prunes it.
    let restricted = !args.skipped_passes.is_empty();

    let mut applied: Vec<WaiverRef> = Vec::new();
    let mut expired: Vec<WaiverRef> = Vec::new();
    let mut retained: Vec<WaiverRef> = Vec::new();
    let mut notices: Vec<String> = Vec::new();
    let mut seen: Vec<(String, String)> = Vec::new();

    for (index, waiver) in waivers.iter().enumerate() {
        if let Some(field) = first_missing_field(waiver) {
            notices.push(format!(
                "malformed waiver at review.waivers[{index}]: missing '{field}'"
            ));
            continue;
        }
        // Safe: `first_missing_field` returned `None`, so each is present and
        // non-empty.
        let rule = waiver.rule.clone().unwrap_or_default();
        let reason = waiver.reason.clone().unwrap_or_default();
        let paths = waiver.file.as_ref().map_or(&[][..], WaiverPaths::as_slice);

        // Each listed path is its own anchor, so one entry can apply at one
        // path, expire at another, and repeat an anchor an earlier entry
        // already claimed at a third.
        for file in paths {
            if seen.iter().any(|(r, f)| r == &rule && f == file) {
                notices.push(format!(
                    "duplicate waiver: rule {rule} at {file} — entry [{index}] ignored"
                ));
                continue;
            }
            seen.push((rule.clone(), file.clone()));

            let file_exists = repo.join(file).exists();
            let rule_fires = args
                .fired
                .iter()
                .any(|finding| finding.rule == rule && &finding.file == file);
            let anchor = WaiverRef {
                rule: rule.clone(),
                file: file.clone(),
                reason: reason.clone(),
            };

            if file_exists && rule_fires {
                applied.push(anchor);
            } else if restricted {
                notices.push(format!(
                    "waiver retained: rule {rule} at {file} — dimension not evaluated this run ({reason})"
                ));
                retained.push(anchor);
            } else {
                notices.push(format!("waiver expired: rule {rule} at {file} ({reason})"));
                expired.push(anchor);
            }
        }
    }

    Ok(ProcessWaiversResult {
        applied,
        expired,
        retained,
        notices,
    })
}

/// Return the first required field that is absent or empty, or `None` when the
/// waiver is well-formed.
///
/// A `file` is missing when it is absent or names no usable anchor — an empty
/// list, or a list holding a blank path.
fn first_missing_field(waiver: &RawWaiver) -> Option<&'static str> {
    let blank = |value: Option<&str>| value.unwrap_or("").trim().is_empty();
    let missing = [
        blank(waiver.rule.as_deref()),
        waiver.file.as_ref().is_none_or(WaiverPaths::is_blank),
        blank(waiver.reason.as_deref()),
        blank(waiver.waived_at.as_deref()),
        blank(waiver.waived_by.as_deref()),
    ];
    REQUIRED_FIELDS
        .iter()
        .zip(missing)
        .find(|(_, missing)| *missing)
        .map(|(field, _)| *field)
}

/// One waiver entry, parsed loosely so a malformed entry (missing field) is a
/// reportable warning rather than a whole-frontmatter parse failure.
#[derive(Deserialize)]
struct RawWaiver {
    #[serde(default)]
    rule: Option<String>,
    #[serde(default)]
    file: Option<WaiverPaths>,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default, rename = "waived-at")]
    waived_at: Option<String>,
    #[serde(default, rename = "waived-by")]
    waived_by: Option<String>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::primitives::FiredFinding;
    use tempfile::TempDir;

    /// Write `specs/{feature}/review.md` with the given waivers YAML
    /// (already indented under `waivers:`), and touch each path in
    /// `existing_files` relative to the repo root.
    fn setup(feature: &str, waivers_yaml: &str, existing_files: &[&str]) -> TempDir {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("specs").join(feature);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# Spec\n",
        )
        .unwrap();
        // The waivers live in `review.md` now (spec 057 task 5). Callers still
        // pass entries indented for the old nested position, so they are
        // de-indented one level here rather than at eleven call sites — the
        // fixtures pin waiver *semantics*, and rewriting them all would risk
        // changing a case while moving it.
        let mut deindented = String::new();
        for line in waivers_yaml.lines() {
            deindented.push_str(line.strip_prefix("  ").unwrap_or(line));
            deindented.push('\n');
        }
        let waivers_yaml = deindented;
        std::fs::write(
            dir.join("review.md"),
            format!(
                "---\nspec: {feature}\nlast-run: 2026-01-01T00:00:00Z\nblocking: false\nwaivers:\n{waivers_yaml}---\n\n# Review\n"
            ),
        )
        .unwrap();
        for rel in existing_files {
            let path = tmp.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, "code\n").unwrap();
        }
        tmp
    }

    fn fired(pairs: &[(&str, &str)]) -> Vec<FiredFinding> {
        pairs
            .iter()
            .map(|(rule, file)| FiredFinding {
                rule: (*rule).to_string(),
                file: (*file).to_string(),
            })
            .collect()
    }

    fn args(feature: &str, fired_pairs: &[(&str, &str)]) -> ProcessWaiversArgs {
        ProcessWaiversArgs {
            feature: feature.to_string(),
            fired: fired(fired_pairs),
            skipped_passes: Vec::new(),
        }
    }

    fn restricted_args(
        feature: &str,
        fired_pairs: &[(&str, &str)],
        skipped: &[&str],
    ) -> ProcessWaiversArgs {
        ProcessWaiversArgs {
            feature: feature.to_string(),
            fired: fired(fired_pairs),
            skipped_passes: skipped.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    const ONE_WAIVER: &str = "    - rule: SEC-BE-014\n      file: src/api/internal.ts\n      reason: Endpoint is internal-only behind mTLS.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";

    #[test]
    fn applies_when_file_exists_and_rule_fires() {
        let tmp = setup("001-x", ONE_WAIVER, &["src/api/internal.ts"]);
        let result = run(
            &args("001-x", &[("SEC-BE-014", "src/api/internal.ts")]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 1);
        assert_eq!(result.applied[0].rule, "SEC-BE-014");
        assert_eq!(result.applied[0].file, "src/api/internal.ts");
        assert_eq!(result.expired, [] as [WaiverRef; 0]);
        assert_eq!(result.notices, [] as [String; 0]);
    }

    #[test]
    fn expires_when_file_is_gone() {
        // File not created → anchor no longer exists.
        let tmp = setup("001-x", ONE_WAIVER, &[]);
        let result = run(
            &args("001-x", &[("SEC-BE-014", "src/api/internal.ts")]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied, [] as [WaiverRef; 0]);
        assert_eq!(result.expired.len(), 1);
        assert_eq!(
            result.notices,
            vec![
                "waiver expired: rule SEC-BE-014 at src/api/internal.ts (Endpoint is internal-only behind mTLS.)"
            ]
        );
    }

    #[test]
    fn expires_when_rule_no_longer_fires() {
        // File exists but the rule is not in the fired set.
        let tmp = setup("001-x", ONE_WAIVER, &["src/api/internal.ts"]);
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(result.applied, [] as [WaiverRef; 0]);
        assert_eq!(result.expired.len(), 1);
        assert!(result.notices[0].starts_with("waiver expired: rule SEC-BE-014"));
    }

    #[test]
    fn restricted_run_retains_non_firing_waiver_instead_of_expiring() {
        // A dimension-restricted run (a pass was skipped) must not expire a
        // waiver just because its rule did not fire — that dimension may not
        // have run. The waiver is retained (kept in frontmatter), not pruned.
        let tmp = setup("001-x", ONE_WAIVER, &["src/api/internal.ts"]);
        let result = run(
            &restricted_args("001-x", &[], &["simplicity", "reuse"]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied, [] as [WaiverRef; 0]);
        assert!(result.expired.is_empty(), "restricted runs never expire");
        assert_eq!(result.retained.len(), 1);
        assert!(result.notices[0].starts_with("waiver retained: rule SEC-BE-014"));
    }

    #[test]
    fn restricted_run_still_applies_a_firing_waiver() {
        // Retention only affects non-firing waivers; one whose rule fires in
        // the passes that DID run still applies (suppresses the finding).
        let tmp = setup("001-x", ONE_WAIVER, &["src/api/internal.ts"]);
        let result = run(
            &restricted_args(
                "001-x",
                &[("SEC-BE-014", "src/api/internal.ts")],
                &["simplicity"],
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 1);
        assert_eq!(result.expired, [] as [WaiverRef; 0]);
        assert_eq!(result.retained, [] as [WaiverRef; 0]);
    }

    #[test]
    fn does_not_extend_to_a_different_file() {
        // The rule fires, but at a different file than the waiver's anchor.
        let tmp = setup(
            "001-x",
            ONE_WAIVER,
            &["src/api/internal.ts", "src/api/other.ts"],
        );
        let result = run(
            &args("001-x", &[("SEC-BE-014", "src/api/other.ts")]),
            tmp.path(),
        )
        .unwrap();
        // The waiver anchors (SEC-BE-014, internal.ts), which does not fire →
        // it expires; other.ts is a separate finding, not covered here.
        assert_eq!(result.applied, [] as [WaiverRef; 0]);
        assert_eq!(result.expired.len(), 1);
    }

    #[test]
    fn code_moving_within_file_does_not_expire() {
        // The anchor is (rule, file) only — no line number. The rule still
        // fires in the same file, so the waiver applies regardless of line.
        let tmp = setup("001-x", ONE_WAIVER, &["src/api/internal.ts"]);
        let result = run(
            &args("001-x", &[("SEC-BE-014", "src/api/internal.ts")]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 1);
    }

    #[test]
    fn malformed_missing_reason_is_skipped_with_warning() {
        let waiver = "    - rule: SEC-BE-014\n      file: src/api/internal.ts\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";
        let tmp = setup("001-x", waiver, &["src/api/internal.ts"]);
        let result = run(
            &args("001-x", &[("SEC-BE-014", "src/api/internal.ts")]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied, [] as [WaiverRef; 0]);
        assert_eq!(result.expired, [] as [WaiverRef; 0]);
        assert_eq!(
            result.notices,
            vec!["malformed waiver at review.waivers[0]: missing 'reason'"]
        );
    }

    #[test]
    fn malformed_missing_waived_by_names_that_field() {
        let waiver = "    - rule: SEC-BE-014\n      file: src/api/internal.ts\n      reason: Internal-only endpoint behind mTLS.\n      waived-at: 2026-05-10T14:40:00Z\n";
        let tmp = setup("001-x", waiver, &["src/api/internal.ts"]);
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(
            result.notices,
            vec!["malformed waiver at review.waivers[0]: missing 'waived-by'"]
        );
    }

    #[test]
    fn duplicate_first_applies_rest_warn() {
        let waivers = format!(
            "{ONE_WAIVER}    - rule: SEC-BE-014\n      file: src/api/internal.ts\n      reason: Duplicate entry that should be ignored.\n      waived-at: 2026-05-11T00:00:00Z\n      waived-by: dev@example.com\n"
        );
        let tmp = setup("001-x", &waivers, &["src/api/internal.ts"]);
        let result = run(
            &args("001-x", &[("SEC-BE-014", "src/api/internal.ts")]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 1);
        assert_eq!(
            result.notices,
            vec!["duplicate waiver: rule SEC-BE-014 at src/api/internal.ts — entry [1] ignored"]
        );
    }

    #[test]
    fn no_waivers_yields_empty_result() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# Spec\n",
        )
        .unwrap();
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(result.applied, [] as [WaiverRef; 0]);
        assert_eq!(result.expired, [] as [WaiverRef; 0]);
        assert_eq!(result.notices, [] as [String; 0]);
    }

    #[test]
    fn missing_feature_is_operational_error() {
        let tmp = TempDir::new().unwrap();
        let err = run(&args("999-nope", &[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    // -- a `file` listing several paths (020's waiver-file-lists) -------------

    /// One `SEC-BE-014` waiver whose `file` lists `paths`.
    fn list_waiver(paths: &[&str]) -> String {
        let items: String = paths.iter().fold(String::new(), |mut items, path| {
            items.push_str("        - ");
            items.push_str(path);
            items.push('\n');
            items
        });
        format!(
            "    - rule: SEC-BE-014\n      file:\n{items}      reason: Shared justification.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n"
        )
    }

    fn anchors(refs: &[WaiverRef]) -> Vec<(&str, &str)> {
        refs.iter()
            .map(|r| (r.rule.as_str(), r.file.as_str()))
            .collect()
    }

    #[test]
    fn each_listed_path_is_classified_as_its_own_anchor() {
        // a fires and exists → applied; b exists but does not fire and c is
        // gone → each expires alone, with its own notice and the shared reason.
        let tmp = setup(
            "001-x",
            &list_waiver(&["src/a.ts", "src/b.ts", "src/c.ts"]),
            &["src/a.ts", "src/b.ts"],
        );
        let result = run(&args("001-x", &[("SEC-BE-014", "src/a.ts")]), tmp.path()).unwrap();
        assert_eq!(anchors(&result.applied), vec![("SEC-BE-014", "src/a.ts")]);
        assert_eq!(
            anchors(&result.expired),
            vec![("SEC-BE-014", "src/b.ts"), ("SEC-BE-014", "src/c.ts")]
        );
        assert!(
            result
                .applied
                .iter()
                .chain(&result.expired)
                .all(|r| r.reason == "Shared justification.")
        );
        assert_eq!(
            result.notices,
            vec![
                "waiver expired: rule SEC-BE-014 at src/b.ts (Shared justification.)",
                "waiver expired: rule SEC-BE-014 at src/c.ts (Shared justification.)",
            ]
        );
    }

    #[test]
    fn restricted_run_splits_one_entry_across_applied_and_retained() {
        let tmp = setup(
            "001-x",
            &list_waiver(&["src/a.ts", "src/b.ts"]),
            &["src/a.ts", "src/b.ts"],
        );
        let result = run(
            &restricted_args("001-x", &[("SEC-BE-014", "src/a.ts")], &["simplicity"]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(anchors(&result.applied), vec![("SEC-BE-014", "src/a.ts")]);
        assert_eq!(anchors(&result.retained), vec![("SEC-BE-014", "src/b.ts")]);
        assert_eq!(result.expired, [] as [WaiverRef; 0]);
    }

    #[test]
    fn a_duplicate_path_ignores_only_that_pair_of_the_entry() {
        // Entry [0] claims a; entry [1] lists a and b. Only (rule, a) of entry
        // [1] is ignored — its b is still classified, and applies.
        let waivers = format!(
            "{ONE_WAIVER}{}",
            list_waiver(&["src/api/internal.ts", "src/b.ts"])
        );
        let tmp = setup("001-x", &waivers, &["src/api/internal.ts", "src/b.ts"]);
        let result = run(
            &args(
                "001-x",
                &[
                    ("SEC-BE-014", "src/api/internal.ts"),
                    ("SEC-BE-014", "src/b.ts"),
                ],
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            anchors(&result.applied),
            vec![
                ("SEC-BE-014", "src/api/internal.ts"),
                ("SEC-BE-014", "src/b.ts")
            ]
        );
        // The first claim's reason wins for the shared anchor.
        assert_eq!(
            result.applied[0].reason,
            "Endpoint is internal-only behind mTLS."
        );
        assert_eq!(
            result.notices,
            vec!["duplicate waiver: rule SEC-BE-014 at src/api/internal.ts — entry [1] ignored"]
        );
    }

    #[test]
    fn the_same_path_twice_in_one_entry_is_a_duplicate_claim() {
        let tmp = setup(
            "001-x",
            &list_waiver(&["src/a.ts", "src/a.ts"]),
            &["src/a.ts"],
        );
        let result = run(&args("001-x", &[("SEC-BE-014", "src/a.ts")]), tmp.path()).unwrap();
        assert_eq!(result.applied.len(), 1);
        assert_eq!(
            result.notices,
            vec!["duplicate waiver: rule SEC-BE-014 at src/a.ts — entry [0] ignored"]
        );
    }

    #[test]
    fn an_empty_list_or_a_blank_path_is_a_missing_file() {
        for file in ["[]", "\n        - src/a.ts\n        - \"\""] {
            let waiver = format!(
                "    - rule: SEC-BE-014\n      file: {file}\n      reason: Shared justification.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n"
            );
            let tmp = setup("001-x", &waiver, &["src/a.ts"]);
            let result = run(&args("001-x", &[("SEC-BE-014", "src/a.ts")]), tmp.path()).unwrap();
            assert!(result.applied.is_empty(), "{file}");
            assert_eq!(
                result.notices,
                vec!["malformed waiver at review.waivers[0]: missing 'file'"],
                "{file}"
            );
        }
    }

    #[test]
    fn a_list_item_that_is_not_a_string_fails_the_parse() {
        // An error, never an empty list: a waivers list that silently read as
        // empty would re-block every finding it waives.
        let tmp = setup(
            "001-x",
            &list_waiver(&["src/a.ts", "{nested: map}"]),
            &["src/a.ts"],
        );
        let err = run(&args("001-x", &[("SEC-BE-014", "src/a.ts")]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Yaml { .. }), "{err}");
    }
}
