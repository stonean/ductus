//! `process-waivers` — deterministic per-run waiver processing for `/ductus:review`.
//!
//! Reads the `waivers` list from `review.md`'s frontmatter and classifies each
//! anchor against the currently-firing findings. An entry's `anchors` list, or a
//! departure waiver's `file` (one path or a list), is read as one anchor per item, so one
//! entry can land in several of these buckets:
//!
//! - **apply** — the anchored file exists, and the rule fires at the anchored site,
//!   with no more findings there than the anchor covers.
//! - **expire** — the file is gone, or the rule no longer fires at the site in a
//!   file the run read, or more findings fire at the site than the anchor covers (on a
//!   dimension-restricted run too: a partial run's surplus is still a surplus), or the
//!   entry was recorded before sites. The anchor drops on the next frontmatter write, and
//!   the entry with its last anchor (write-review's job); this primitive emits the
//!   `waiver expired: …` notice and reports the anchor.
//! - **retain** — a non-firing waiver on a **dimension-restricted** run
//!   (`skipped-passes` non-empty). The run lacks the full-review picture, so
//!   the waiver is left untouched (never expired/pruned) — this honors the
//!   contract that waivers anchored to skipped dimensions apply unchanged.
//!   Only an unrestricted run can expire a waiver for not firing. So too a non-firing
//!   waiver on a file still on disk that the run's scope does not hold, which it did
//!   not read (spec 079).
//! - **malformed** — a field is missing/empty, `anchors` included; warn and skip, and
//!   never report it expired. `write-review` compares anchors only, so a malformed
//!   waiver still loses each anchor an expired one shares.
//! - **duplicate** — a repeated `(rule, file, site)` anchor, within an entry or across
//!   entries (`(rule, file)` for an anchor with no site); only the first claim applies,
//!   and the same entry's other anchors are still classified.
//!
//! An anchor is a `(rule, file, site)` triple with the count of findings it covers
//! (spec 020's `waiver-expiry`); line numbers are not part of it, so code moving within
//! its site does not expire a waiver. The findings at an anchor are counted after
//! collapsing those of one rule, file and site whose line ranges overlap, never across
//! sites. A waiver `write-review` derived from a spec's departure entry carries
//! `departure: true`, keeps `file`, and covers every finding of its rule at each file,
//! at any site, with no count. Any other entry holding `file` and no `anchors` was
//! recorded before sites: it covers nothing and always expires.
//!
//! A departure waiver is expired whatever the run's restrictions when the spec's
//! `## Applicable Rules` holds no well-formed entry for its rule, so a removed entry
//! stops waiving at once (spec 073); a waiver with the marker and an entry is classified
//! as any other. Read-only: frontmatter mutation belongs to `write-review`.
//!
//! The four lists grow with a spec's waivers, and a departure entry adds one per rule and
//! in-scope file, so [`run`] bounds them as `process-decisions` does (`bounded_lists::place`;
//! spec 073, BE-PAYLOAD-001): inline while the result fits `[artifacts] read-size-bytes`, and
//! past it in the file `lists-file` names, with each list's count inline in both forms. A
//! result with nothing in any list reads no setting.
//!
//! Defined by
//! `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`;
//! the per-path anchors by
//! `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`.

use std::cell::OnceCell;
use std::path::Path;

use serde::Deserialize;

use crate::primitives::compute_review_scope::RunScope;
use crate::primitives::write_review::collapse_at_sites;
use crate::primitives::{
    PrimitiveError, Result, StoredAnchor, WaiverPaths, anchors_unusable, bounded_lists, departures,
};
use crate::schema::paths;
use crate::schema::primitives::{
    FiredFinding, ListsForm, ProcessWaiversArgs, ProcessWaiversResult, WaiverRef,
};

/// What [`compute`] classifies, whole: the four lists of [`ProcessWaiversResult`] before
/// [`run`] decides where they go.
#[derive(Debug)]
pub(crate) struct Classified {
    pub(crate) applied: Vec<WaiverRef>,
    pub(crate) expired: Vec<WaiverRef>,
    pub(crate) retained: Vec<WaiverRef>,
    pub(crate) notices: Vec<String>,
}

/// Required waiver fields, checked in this order; the first missing/empty one
/// names the `malformed …` diagnostic.
const REQUIRED_FIELDS: &[&str] = &["rule", "anchors", "reason", "waived-at", "waived-by"];

/// Execute the `process-waivers` primitive: [`compute`], returned with its lists inline while
/// the result fits `[artifacts] read-size-bytes`, and past it with the lists in the file
/// `lists-file` names and only their counts inline (`bounded_lists::place`). A result with
/// every list empty is inline and reads no setting, so a malformed `read-size-bytes` halts
/// only a result with a list to bound (`QUAL-DELEG-001`).
///
/// # Errors
///
/// Every error [`compute`] returns, and those `bounded_lists::place` returns for a malformed
/// read size or a list file that cannot be written.
pub fn run(args: &ProcessWaiversArgs, repo: &Path) -> Result<ProcessWaiversResult> {
    let classified = compute(args, repo)?;
    let lists = serde_json::json!({
        "applied": &classified.applied,
        "expired": &classified.expired,
        "retained": &classified.retained,
        "notices": &classified.notices,
    });
    let nothing = classified.applied.is_empty()
        && classified.expired.is_empty()
        && classified.retained.is_empty()
        && classified.notices.is_empty();
    let mut result = ProcessWaiversResult {
        form: ListsForm::Inline,
        applied_count: classified.applied.len(),
        expired_count: classified.expired.len(),
        retained_count: classified.retained.len(),
        notices_count: classified.notices.len(),
        applied: Some(classified.applied),
        expired: Some(classified.expired),
        retained: Some(classified.retained),
        notices: Some(classified.notices),
        lists_file: None,
    };
    if nothing {
        return Ok(result);
    }
    if let Some(file) =
        bounded_lists::place(repo, "process-waivers", &args.feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.applied = None;
        result.expired = None;
        result.retained = None;
        result.notices = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// Classify the recorded waivers, whole.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute,
/// or carries a parent-directory component,
/// or when the feature directory, or a link inside it, resolves outside the
/// project, [`PrimitiveError::FeatureNotFound`]
/// when the feature has no `spec.md`, [`PrimitiveError::UnclosedFrontmatter`]
/// when `review.md`'s frontmatter block never closes, [`PrimitiveError::Yaml`]
/// when its frontmatter or `waivers:` list fails to parse, or
/// [`PrimitiveError::Io`] on read failure. An absent `review.md`, or one that
/// opens no frontmatter block, records no waivers.
pub(crate) fn compute(args: &ProcessWaiversArgs, repo: &Path) -> Result<Classified> {
    let layout = paths::Paths::load(repo)?;
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
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
    // A file the run did not read says nothing about a rule's firing there either,
    // so such a waiver stays too, read only when one did not fire (spec 079).
    // One rule's findings at one site of a file whose line ranges overlap are one, and
    // findings at different sites are counted apart.
    let pass = Pass {
        args,
        repo,
        counted: collapse_at_sites(&args.fired),
        restricted: !args.skipped_passes.is_empty(),
        scope: OnceCell::new(),
    };
    let mut classified = Classified {
        applied: Vec::new(),
        expired: Vec::new(),
        retained: Vec::new(),
        notices: Vec::new(),
    };
    let mut seen: Vec<(String, String, Option<String>)> = Vec::new();
    // Read only when a waiver carries the marker, so a record with none depends on nothing
    // else. The first marked waiver reads the spec, which this function has found to exist.
    let mut entry_rules: Option<Vec<String>> = None;

    for (index, waiver) in waivers.iter().enumerate() {
        let located = read_anchors(waiver);
        if let Some(field) = first_missing_field(waiver, located.is_some()) {
            classified.notices.push(format!(
                "malformed waiver at review.waivers[{index}]: missing '{field}'"
            ));
            continue;
        }
        // Safe: `first_missing_field` returned `None`, so each is present and
        // non-empty, and the anchors were read.
        let rule = waiver.rule.clone().unwrap_or_default();
        let reason = waiver.reason.clone().unwrap_or_default();
        // A notice prints each stored value flattened to one line, so a
        // control character a hand edit left in the record never reaches the
        // operator raw; classification reads the values as written (066
        // scenario `process-waivers-notices-flatten-stored-text`).
        let shown_rule = super::flatten_line(&rule);
        let shown_reason = super::flatten_line(&reason);

        // Each anchor is classified alone, so one entry can apply at one anchor,
        // expire at another, and repeat an anchor an earlier entry already claimed at a
        // third.
        for located in located.unwrap_or_default() {
            let file = located.file;
            let site = match located.kind {
                Kind::Site { site, .. } => Some(site),
                Kind::Departure | Kind::PreSite => None,
            };
            let shown_file = super::flatten_line(file);
            let shown_site = site.map(super::flatten_line);
            // ` in {site}` after the file, for an anchor that has a site.
            let at = shown_site.as_ref().map_or_else(
                || shown_file.clone(),
                |site| format!("{shown_file} in {site}"),
            );
            let claim = (rule.clone(), file.to_string(), site.map(str::to_string));
            if seen.contains(&claim) {
                classified.notices.push(format!(
                    "duplicate waiver: rule {shown_rule} at {at} — entry [{index}] ignored"
                ));
                continue;
            }
            seen.push(claim);
            let anchor = WaiverRef {
                rule: rule.clone(),
                file: file.to_string(),
                site: site.map(str::to_string),
                reason: reason.clone(),
            };

            if located.kind == Kind::PreSite {
                classified.notices.push(format!(
                    "waiver expired: rule {shown_rule} at {shown_file} — recorded before sites ({shown_reason})"
                ));
                classified.expired.push(anchor);
                continue;
            }
            if located.kind == Kind::Departure
                && departure_gone(waiver, &rule, &spec_path, &mut entry_rules)?
            {
                classified.notices.push(format!(
                    "waiver expired: rule {shown_rule} at {shown_file} ({shown_reason})"
                ));
                classified.expired.push(anchor);
                continue;
            }

            let shown = Shown {
                rule: &shown_rule,
                at: &at,
                reason: &shown_reason,
            };
            pass.place(located.kind, anchor, &shown, &mut classified);
        }
    }

    Ok(classified)
}

/// The text a notice prints of an anchor, each flattened to one line.
struct Shown<'a> {
    rule: &'a str,
    /// The file, then ` in {site}` for an anchor that has a site.
    at: &'a str,
    reason: &'a str,
}

/// What classifying one run's anchors reads: the findings as counted, whether the run was
/// dimension-restricted, and the run's scope, resolved when an anchor first needs it.
struct Pass<'a> {
    args: &'a ProcessWaiversArgs,
    repo: &'a Path,
    counted: Vec<&'a FiredFinding>,
    restricted: bool,
    scope: OnceCell<RunScope>,
}

impl Pass<'_> {
    /// Place one anchor that is not a duplicate, not recorded before sites and not a
    /// departure waiver whose entry is gone: apply it, expire it for a count it does not
    /// cover, or, when it does not fire, retain or expire it.
    fn place(&self, kind: Kind<'_>, anchor: WaiverRef, shown: &Shown<'_>, out: &mut Classified) {
        let Shown { rule, at, reason } = shown;
        let file_exists = self.repo.join(&anchor.file).exists();
        let count = self
            .counted
            .iter()
            .filter(|finding| {
                finding.rule == anchor.rule
                    && finding.file == anchor.file
                    && anchor.site.as_deref().is_none_or(|site| {
                        finding.site.as_deref().map(str::trim) == Some(site.trim())
                    })
            })
            .count();
        if let Kind::Site { covers, .. } = kind
            && file_exists
            && count > covers as usize
        {
            // A finding beyond the count is one nobody judged, and a partial run's
            // surplus is still a surplus, so a restricted run expires it too.
            out.notices.push(format!(
                "waiver expired: rule {rule} at {at} — {count} findings fire where it covers {covers} ({reason})"
            ));
            out.expired.push(anchor);
        } else if file_exists && count > 0 {
            out.applied.push(anchor);
        } else if self.restricted {
            out.notices.push(format!(
                "waiver retained: rule {rule} at {at} — dimension not evaluated this run ({reason})"
            ));
            out.retained.push(anchor);
        } else if let Some(why) = self
            .scope
            .get_or_init(|| {
                RunScope::resolve(&self.args.feature, self.args.since.as_deref(), self.repo)
            })
            .why_unread(&anchor.file, self.repo)
        {
            // `why` interpolates the stored file, so it is flattened with the rest.
            let shown_why = super::flatten_line(&why);
            out.notices.push(format!(
                "waiver retained: rule {rule} at {at} — {shown_why} ({reason})"
            ));
            out.retained.push(anchor);
        } else {
            out.notices
                .push(format!("waiver expired: rule {rule} at {at} ({reason})"));
            out.expired.push(anchor);
        }
    }
}

/// Whether `waiver` carries the departure marker and the spec's `## Applicable Rules`
/// holds no well-formed entry for `rule`. `entry_rules` caches the entries' rules, read
/// from the spec at `spec_path` by the first marked waiver, so a record with no marker
/// reads nothing else.
fn departure_gone(
    waiver: &RawWaiver,
    rule: &str,
    spec_path: &Path,
    entry_rules: &mut Option<Vec<String>>,
) -> Result<bool> {
    if waiver.departure != Some(true) {
        return Ok(false);
    }
    if entry_rules.is_none() {
        let content = super::read_text(spec_path)?;
        *entry_rules = Some(
            departures::parse(&content)
                .entries
                .into_iter()
                .map(|entry| entry.rule)
                .collect(),
        );
    }
    Ok(!entry_rules
        .as_deref()
        .unwrap_or_default()
        .iter()
        .any(|held| held == rule))
}

/// Return the first required field that is absent or empty, or `None` when the
/// waiver is well-formed. `anchors_read` is whether [`read_anchors`] found a usable anchor.
///
/// `anchors` is missing when the entry names no usable one: neither an `anchors` list
/// of anchors that each hold a file, a site and a positive `covers`, nor a usable `file`
/// (an empty list, or one holding a blank path, is not).
fn first_missing_field(waiver: &RawWaiver, anchors_read: bool) -> Option<&'static str> {
    let blank = |value: Option<&str>| value.unwrap_or("").trim().is_empty();
    let missing = [
        blank(waiver.rule.as_deref()),
        !anchors_read,
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

/// How an anchor is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind<'a> {
    /// A `(file, site)` anchor covering up to `covers` findings.
    Site { site: &'a str, covers: u32 },
    /// A departure waiver's file, covering every finding of its rule there.
    Departure,
    /// A path of an entry recorded before sites: it covers nothing.
    PreSite,
}

/// One anchor of an entry, as [`read_anchors`] reads it.
struct Located<'a> {
    file: &'a str,
    kind: Kind<'a>,
}

/// The entry's anchors, or `None` when it names no usable one. A departure waiver is read
/// by its `file`, one anchor per path covering every site, whatever else it holds, as
/// `write-review` refreshes it. Any other entry holding `anchors` is read from them alone,
/// and one holding a blank or unusable anchor, or none, is malformed; otherwise a usable
/// `file` is one anchor per path, recorded before sites.
fn read_anchors(waiver: &RawWaiver) -> Option<Vec<Located<'_>>> {
    let departure = waiver.departure == Some(true);
    if !departure && let Some(anchors) = &waiver.anchors {
        if anchors_unusable(anchors) {
            return None;
        }
        return Some(
            anchors
                .iter()
                .filter_map(StoredAnchor::usable)
                .map(|(file, site, covers)| Located {
                    file,
                    kind: Kind::Site { site, covers },
                })
                .collect(),
        );
    }
    let paths = waiver.file.as_ref().filter(|paths| !paths.is_blank())?;
    Some(
        paths
            .as_slice()
            .iter()
            .map(|file| Located {
                file,
                kind: if departure {
                    Kind::Departure
                } else {
                    Kind::PreSite
                },
            })
            .collect(),
    )
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
    anchors: Option<Vec<StoredAnchor>>,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default, rename = "waived-at")]
    waived_at: Option<String>,
    #[serde(default, rename = "waived-by")]
    waived_by: Option<String>,
    /// Set by `write-review` on a waiver derived from a departure entry.
    #[serde(default)]
    departure: Option<bool>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::primitives::FiredFinding;
    use std::fmt::Write as _;
    use tempfile::TempDir;

    /// The classification, whole: these tests read the lists as the host reads them inline.
    fn run(args: &ProcessWaiversArgs, repo: &Path) -> Result<Classified> {
        compute(args, repo)
    }

    /// Write `specs/{feature}/review.md` with the given waivers YAML
    /// (already indented under `waivers:`), and touch each path in
    /// `existing_files` relative to the repo root.
    fn setup(feature: &str, waivers_yaml: &str, existing_files: &[&str]) -> TempDir {
        setup_after(feature, waivers_yaml, &[], existing_files)
    }

    /// `setup`, with the spec reopened in a repository: each of `before` is committed
    /// before its window starts, so the run's scope does not hold it, and each of
    /// `existing_files` is uncommitted, so the scope does.
    fn setup_after(
        feature: &str,
        waivers_yaml: &str,
        before: &[&str],
        existing_files: &[&str],
    ) -> TempDir {
        use crate::primitives::git_fixture::commit_all;
        let tmp = TempDir::new().unwrap();
        let repo = git2::Repository::init(tmp.path()).unwrap();
        let dir = tmp.path().join("specs").join(feature);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: planned\ndependencies: []\n---\n\n# Spec\n",
        )
        .unwrap();
        for rel in before {
            let path = tmp.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, "code\n").unwrap();
        }
        commit_all(&repo, "chore: seed");
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# Spec\n",
        )
        .unwrap();
        commit_all(&repo, "status: in-progress");
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

    /// Findings at the site every fixture waiver anchors to, `s`, one per pair.
    fn fired(pairs: &[(&str, &str)]) -> Vec<FiredFinding> {
        pairs
            .iter()
            .map(|(rule, file)| fired_at(rule, file, Some("s"), ""))
            .collect()
    }

    fn fired_at(rule: &str, file: &str, site: Option<&str>, range: &str) -> FiredFinding {
        FiredFinding {
            rule: rule.to_string(),
            file: file.to_string(),
            site: site.map(str::to_string),
            line_range: range.to_string(),
        }
    }

    fn args(feature: &str, fired_pairs: &[(&str, &str)]) -> ProcessWaiversArgs {
        ProcessWaiversArgs {
            feature: feature.to_string(),
            fired: fired(fired_pairs),
            skipped_passes: Vec::new(),
            since: None,
            ..ProcessWaiversArgs::default()
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
            since: None,
            ..ProcessWaiversArgs::default()
        }
    }

    const ONE_WAIVER: &str = "    - rule: SEC-BE-014\n      anchors:\n        - file: src/api/internal.ts\n          site: s\n          covers: 1\n      reason: Endpoint is internal-only behind mTLS.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";

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
                "waiver expired: rule SEC-BE-014 at src/api/internal.ts in s (Endpoint is internal-only behind mTLS.)"
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
        assert_eq!(
            result.notices,
            vec![
                "waiver expired: rule SEC-BE-014 at src/api/internal.ts in s (Endpoint is internal-only behind mTLS.)"
            ]
        );
    }

    /// The rule did not fire because the run never opened the file, so that says
    /// nothing about the waiver, and a person's reason stays for the run that reads it.
    #[test]
    fn a_waiver_on_a_file_the_run_did_not_read_is_retained() {
        let tmp = setup_after("001-x", ONE_WAIVER, &["src/api/internal.ts"], &[]);
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(result.expired, [] as [WaiverRef; 0]);
        assert_eq!(result.retained.len(), 1);
        assert!(
            result.notices[0].contains("this run did not read `src/api/internal.ts`"),
            "{:?}",
            result.notices
        );
    }

    /// A narrowed window reads less, so the same file is unread under it.
    #[test]
    fn the_runs_since_decides_what_it_read() {
        let tmp = setup("001-x", ONE_WAIVER, &[]);
        let path = tmp.path().join("src/api/internal.ts");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "code\n").unwrap();
        let repo = git2::Repository::open(tmp.path()).unwrap();
        crate::primitives::git_fixture::commit_all(&repo, "feat: the file");
        assert_eq!(
            run(&args("001-x", &[]), tmp.path()).unwrap().expired.len(),
            1
        );

        let mut narrowed = args("001-x", &[]);
        narrowed.since = Some("HEAD".to_string());
        let result = run(&narrowed, tmp.path()).unwrap();
        assert_eq!(result.retained.len(), 1, "{:?}", result.notices);
    }

    /// No window to read is no evidence the file was read, and the notice says which.
    #[test]
    fn a_scope_that_cannot_be_read_retains_and_says_so() {
        let tmp = setup("001-x", ONE_WAIVER, &["src/api/internal.ts"]);
        std::fs::remove_dir_all(tmp.path().join(".git")).unwrap();
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(result.retained.len(), 1);
        assert!(
            result.notices[0].contains("this run's scope could not be read"),
            "{:?}",
            result.notices
        );
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
        // The anchor is (rule, file, site) only — no line number. The rule still
        // fires at the same site, so the waiver applies regardless of line.
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
        let waiver = "    - rule: SEC-BE-014\n      anchors:\n        - file: src/api/internal.ts\n          site: s\n          covers: 1\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";
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
        let waiver = "    - rule: SEC-BE-014\n      anchors:\n        - file: src/api/internal.ts\n          site: s\n          covers: 1\n      reason: Internal-only endpoint behind mTLS.\n      waived-at: 2026-05-10T14:40:00Z\n";
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
            "{ONE_WAIVER}    - rule: SEC-BE-014\n      anchors:\n        - file: src/api/internal.ts\n          site: s\n          covers: 1\n      reason: Duplicate entry that should be ignored.\n      waived-at: 2026-05-11T00:00:00Z\n      waived-by: dev@example.com\n"
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
            vec![
                "duplicate waiver: rule SEC-BE-014 at src/api/internal.ts in s — entry [1] ignored"
            ]
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

    /// One `SEC-BE-014` waiver anchored at site `s` of each of `paths`, covering one finding
    /// at each.
    fn list_waiver(paths: &[&str]) -> String {
        let items: String = paths.iter().fold(String::new(), |mut items, path| {
            let _ = write!(
                items,
                "        - file: {path}\n          site: s\n          covers: 1\n"
            );
            items
        });
        format!(
            "    - rule: SEC-BE-014\n      anchors:\n{items}      reason: Shared justification.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n"
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
                "waiver expired: rule SEC-BE-014 at src/b.ts in s (Shared justification.)",
                "waiver expired: rule SEC-BE-014 at src/c.ts in s (Shared justification.)",
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
            vec![
                "duplicate waiver: rule SEC-BE-014 at src/api/internal.ts in s — entry [1] ignored"
            ]
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
            vec!["duplicate waiver: rule SEC-BE-014 at src/a.ts in s — entry [0] ignored"]
        );
    }

    /// Every notice prints a stored waiver's rule, file and reason flattened
    /// to one line, while classification reads them as written: an escape
    /// planted in each field of an expired, a retained and a duplicate waiver
    /// reaches no notice, and a tab is kept (066 scenario
    /// `process-waivers-notices-flatten-stored-text`).
    #[test]
    fn notices_flatten_stored_text_and_classification_reads_it_as_written() {
        // ESC in the rule, NEL in the file, U+2028 and a tab in the reason.
        let planted = "    - rule: \"R\\x1b[2J1\"\n      anchors:\n        - file: \"src/gone\\x85.ts\"\n          site: s\n          covers: 1\n      reason: \"why\\u2028so\\tthen\"\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";
        let (rule, file, reason) = ("R\u{1b}[2J1", "src/gone\u{85}.ts", "why\u{2028}so\tthen");

        let tmp = setup("001-x", planted, &[]);
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(
            result.notices,
            ["waiver expired: rule R [2J1 at src/gone .ts in s (why so\tthen)"]
        );
        let anchor = &result.expired[0];
        assert_eq!(
            (
                anchor.rule.as_str(),
                anchor.file.as_str(),
                anchor.reason.as_str()
            ),
            (rule, file, reason)
        );

        let result = run(&restricted_args("001-x", &[], &["security"]), tmp.path()).unwrap();
        assert_eq!(
            result.notices,
            [
                "waiver retained: rule R [2J1 at src/gone .ts in s — dimension not evaluated this run (why so\tthen)"
            ]
        );
        assert_eq!(result.retained[0].file, file);

        let tmp = setup("001-x", &format!("{planted}{planted}"), &[]);
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(
            result.notices[1],
            "duplicate waiver: rule R [2J1 at src/gone .ts in s — entry [1] ignored"
        );
        for notice in &result.notices {
            assert!(
                !notice
                    .chars()
                    .any(|c| c != '\t' && crate::primitives::is_line_hazard(c)),
                "{notice:?}"
            );
        }

        // An existing file outside the window reaches the `why_unread` branch, whose
        // text names the stored file.
        let tmp = setup_after("001-x", planted, &["src/gone\u{85}.ts"], &[]);
        let result = run(&args("001-x", &[]), tmp.path()).unwrap();
        assert_eq!(
            result.notices,
            [
                "waiver retained: rule R [2J1 at src/gone .ts in s — this run did not read `src/gone .ts` (why so\tthen)"
            ]
        );
        assert_eq!(result.retained[0].file, file);
    }

    /// The notice for a departure waiver whose spec holds no entry for its rule prints
    /// the stored rule, file and reason flattened to one line, as the other notices do:
    /// an escape planted in each field reaches no notice, and the anchor keeps the stored
    /// text (066 scenario `process-waivers-notices-flatten-stored-text`).
    #[test]
    fn the_departure_expiry_notice_flattens_stored_text() {
        let planted = "    - rule: \"R\\x1b[2J1\"\n      file: \"src/gone\\x85.ts\"\n      reason: \"why\\u2028so\\tthen\"\n      departure: true\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";
        let tmp = setup("001-x", planted, &["src/gone\u{85}.ts"]);
        let result = run(
            &args("001-x", &[("R\u{1b}[2J1", "src/gone\u{85}.ts")]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            result.notices,
            ["waiver expired: rule R [2J1 at src/gone .ts (why so\tthen)"]
        );
        assert!(result.applied.is_empty(), "{result:?}");
        let anchor = &result.expired[0];
        assert_eq!(
            (
                anchor.rule.as_str(),
                anchor.file.as_str(),
                anchor.reason.as_str()
            ),
            ("R\u{1b}[2J1", "src/gone\u{85}.ts", "why\u{2028}so\tthen")
        );
    }

    /// An entry naming no usable anchor is malformed under `anchors`, whichever spelling
    /// it uses: an empty list, a blank file or site, a `covers` that is not a positive
    /// integer, and a departure waiver's empty `file` list or blank path.
    #[test]
    fn an_entry_naming_no_usable_anchor_is_missing_its_anchors() {
        let tail = "      reason: Shared justification.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";
        let cases = [
            "      anchors: []\n".to_string(),
            "      anchors:\n        - file: \"\"\n          site: s\n          covers: 1\n"
                .to_string(),
            "      anchors:\n        - file: src/a.ts\n          site: \"\"\n          covers: 1\n"
                .to_string(),
            "      anchors:\n        - file: src/a.ts\n          covers: 1\n".to_string(),
            "      anchors:\n        - file: src/a.ts\n          site: s\n          covers: 0\n"
                .to_string(),
            "      anchors:\n        - file: src/a.ts\n          site: s\n          covers: two\n"
                .to_string(),
            "      anchors:\n        - file: src/a.ts\n          site: s\n".to_string(),
            "      file: []\n".to_string(),
            "      departure: true\n      file: []\n".to_string(),
            "      departure: true\n      file:\n        - src/a.ts\n        - \"\"\n".to_string(),
            "      departure: true\n".to_string(),
        ];
        for body in cases {
            let waiver = format!("    - rule: SEC-BE-014\n{body}{tail}");
            let tmp = setup("001-x", &waiver, &["src/a.ts"]);
            let result = run(&args("001-x", &[("SEC-BE-014", "src/a.ts")]), tmp.path()).unwrap();
            assert!(result.applied.is_empty(), "{body}");
            assert_eq!(
                result.notices,
                vec!["malformed waiver at review.waivers[0]: missing 'anchors'"],
                "{body}"
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

    // -- an anchor covers the findings at its site, up to its count (020 AC19) -----

    /// One `SEC-BE-014` waiver anchored at `site` of `src/a.ts`, covering `covers` findings.
    fn site_waiver(site: &str, covers: u32) -> String {
        format!(
            "    - rule: SEC-BE-014\n      anchors:\n        - file: src/a.ts\n          site: {site}\n          covers: {covers}\n      reason: Decided for this site.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n"
        )
    }

    fn firing(findings: Vec<FiredFinding>) -> ProcessWaiversArgs {
        ProcessWaiversArgs {
            feature: "001-x".to_string(),
            fired: findings,
            ..ProcessWaiversArgs::default()
        }
    }

    /// A finding of the waived rule at another site in the same file is not covered: the
    /// anchor's own site fires nowhere, so it expires, and the finding at the other site is
    /// left to count.
    #[test]
    fn a_finding_at_another_site_in_the_file_is_not_covered() {
        let tmp = setup("001-x", &site_waiver("alpha", 1), &["src/a.ts"]);
        let result = run(
            &firing(vec![fired_at("SEC-BE-014", "src/a.ts", Some("beta"), "")]),
            tmp.path(),
        )
        .unwrap();
        assert!(result.applied.is_empty(), "{result:?}");
        assert_eq!(
            result.notices,
            vec!["waiver expired: rule SEC-BE-014 at src/a.ts in alpha (Decided for this site.)"]
        );

        // The finding at the anchored site, beside one at another, is the only one covered.
        let result = run(
            &firing(vec![
                fired_at("SEC-BE-014", "src/a.ts", Some("beta"), "1-2"),
                fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "10-12"),
            ]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 1, "{result:?}");
        assert_eq!(result.applied[0].site.as_deref(), Some("alpha"));
    }

    /// A finding whose pass named no site is covered by no site anchor.
    #[test]
    fn a_finding_that_names_no_site_is_not_covered() {
        let tmp = setup("001-x", &site_waiver("alpha", 1), &["src/a.ts"]);
        let result = run(
            &firing(vec![fired_at("SEC-BE-014", "src/a.ts", None, "")]),
            tmp.path(),
        )
        .unwrap();
        assert!(result.applied.is_empty(), "{result:?}");
        assert_eq!(result.expired.len(), 1, "{result:?}");
    }

    /// More findings fire at the site than the anchor covers: it covers none of them and
    /// expires with the count notice, on a dimension-restricted run too.
    #[test]
    fn more_findings_than_the_anchor_covers_expire_it_with_the_count_notice() {
        let tmp = setup("001-x", &site_waiver("alpha", 1), &["src/a.ts"]);
        let two = vec![
            fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "1-2"),
            fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "10-12"),
        ];
        let expected = vec![
            "waiver expired: rule SEC-BE-014 at src/a.ts in alpha — 2 findings fire where it covers 1 (Decided for this site.)",
        ];
        let result = run(&firing(two.clone()), tmp.path()).unwrap();
        assert!(result.applied.is_empty(), "{result:?}");
        assert_eq!(result.expired.len(), 1);
        assert_eq!(result.notices, expected);

        let mut restricted = firing(two);
        restricted.skipped_passes = vec!["simplicity".to_string()];
        let result = run(&restricted, tmp.path()).unwrap();
        assert!(result.retained.is_empty(), "{result:?}");
        assert_eq!(result.notices, expected);
    }

    /// Two reports of one finding at a site whose line ranges overlap count once, and a count
    /// of two covers two separate ones.
    #[test]
    fn findings_with_overlapping_ranges_count_once() {
        let tmp = setup("001-x", &site_waiver("alpha", 1), &["src/a.ts"]);
        let result = run(
            &firing(vec![
                fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "10-20"),
                fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "15"),
            ]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 1, "{result:?}");

        let tmp = setup("001-x", &site_waiver("alpha", 2), &["src/a.ts"]);
        let result = run(
            &firing(vec![
                fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "1-2"),
                fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "10-12"),
            ]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 1, "{result:?}");
        assert_eq!(result.notices, [] as [String; 0]);
    }

    /// The collapse never crosses sites: a whole-file finding of the rule at another site,
    /// listed first, overlaps every range and still absorbs none of the anchored site's, so
    /// the waiver applies while its finding fires (spec 020 task 21).
    #[test]
    fn a_finding_at_another_site_absorbs_none_at_the_anchored_site() {
        let tmp = setup("001-x", &site_waiver("alpha", 1), &["src/a.ts"]);
        for whole_file_site in [Some("(top level)"), None] {
            let result = run(
                &firing(vec![
                    fired_at("SEC-BE-014", "src/a.ts", whole_file_site, ""),
                    fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "10-12"),
                ]),
                tmp.path(),
            )
            .unwrap();
            assert_eq!(result.applied.len(), 1, "{whole_file_site:?}: {result:?}");
            assert_eq!(result.applied[0].site.as_deref(), Some("alpha"));
            assert_eq!(result.notices, [] as [String; 0], "{whole_file_site:?}");
        }
    }

    /// An entry recorded before sites, a `file` and no `anchors` that is not a departure
    /// waiver's, covers nothing: it expires whatever fired and on a restricted run too, with
    /// the notice naming that.
    #[test]
    fn an_entry_recorded_before_sites_covers_nothing_and_always_expires() {
        let legacy = "    - rule: SEC-BE-014\n      file:\n        - src/a.ts\n        - src/b.ts\n      reason: Decided for the file.\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";
        let tmp = setup("001-x", legacy, &["src/a.ts", "src/b.ts"]);
        let expected = vec![
            "waiver expired: rule SEC-BE-014 at src/a.ts — recorded before sites (Decided for the file.)",
            "waiver expired: rule SEC-BE-014 at src/b.ts — recorded before sites (Decided for the file.)",
        ];
        let mut fires = firing(vec![
            fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), ""),
            fired_at("SEC-BE-014", "src/b.ts", None, ""),
        ]);
        let result = run(&fires, tmp.path()).unwrap();
        assert!(result.applied.is_empty(), "{result:?}");
        assert_eq!(result.notices, expected);
        assert_eq!(
            anchors(&result.expired),
            vec![("SEC-BE-014", "src/a.ts"), ("SEC-BE-014", "src/b.ts")]
        );
        assert!(result.expired.iter().all(|anchor| anchor.site.is_none()));

        fires.skipped_passes = vec!["security".to_string()];
        let result = run(&fires, tmp.path()).unwrap();
        assert!(result.retained.is_empty(), "{result:?}");
        assert_eq!(result.notices, expected);
    }

    /// The same file at two sites is two anchors, not a duplicate; the same triple twice is.
    #[test]
    fn duplicates_compare_by_site() {
        let both = format!("{}{}", site_waiver("alpha", 1), site_waiver("beta", 1));
        let tmp = setup("001-x", &both, &["src/a.ts"]);
        let result = run(
            &firing(vec![
                fired_at("SEC-BE-014", "src/a.ts", Some("alpha"), "1"),
                fired_at("SEC-BE-014", "src/a.ts", Some("beta"), "9"),
            ]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.applied.len(), 2, "{result:?}");
        assert_eq!(result.notices, [] as [String; 0]);
    }

    // -- the bound on the result (073, BE-PAYLOAD-001) ------------------------

    fn read_size(repo: &Path, value: &str) {
        std::fs::create_dir_all(repo.join(".ductus")).unwrap();
        std::fs::write(
            repo.join(".ductus/config.toml"),
            format!("[artifacts]\nread-size-bytes = {value}\n"),
        )
        .unwrap();
    }

    /// Past `[artifacts] read-size-bytes` the four lists leave the result for the one file
    /// `lists-file` names, each count stays inline in both forms, and a result exactly at the
    /// size fits. A result with every list empty reads no setting.
    #[test]
    fn the_lists_past_the_read_size_name_a_file_holding_them_whole() {
        let tmp = setup(
            "001-x",
            &list_waiver(&["src/a.ts", "src/b.ts", "src/gone.ts"]),
            &["src/a.ts", "src/b.ts"],
        );
        let firing = args("001-x", &[("SEC-BE-014", "src/a.ts")]);

        read_size(tmp.path(), "1000000");
        let inline = super::run(&firing, tmp.path()).unwrap();
        assert_eq!(inline.form, ListsForm::Inline, "{inline:?}");
        assert_eq!(inline.lists_file, None);
        assert_eq!(inline.applied.as_ref().unwrap().len(), 1);
        assert_eq!(inline.expired.as_ref().unwrap().len(), 2);
        assert_eq!(inline.retained, Some(Vec::new()));
        assert_eq!(inline.notices.as_ref().unwrap().len(), 2);
        assert_eq!(
            (
                inline.applied_count,
                inline.expired_count,
                inline.retained_count,
                inline.notices_count
            ),
            (1, 2, 0, 2)
        );

        // The bound is on the whole result: at its compact size it fits, a byte under it
        // does not.
        let at = serde_json::to_vec(&inline).unwrap().len();
        read_size(tmp.path(), &at.to_string());
        let fits = super::run(&firing, tmp.path()).unwrap();
        assert_eq!(fits.form, ListsForm::Inline, "{fits:?}");
        read_size(tmp.path(), &(at - 1).to_string());
        let past = super::run(&firing, tmp.path()).unwrap();
        assert_eq!(past.form, ListsForm::File, "{past:?}");

        read_size(tmp.path(), "10");
        let file = super::run(&firing, tmp.path()).unwrap();
        assert_eq!(file.form, ListsForm::File, "{file:?}");
        assert!(file.applied.is_none() && file.expired.is_none(), "{file:?}");
        assert!(
            file.retained.is_none() && file.notices.is_none(),
            "{file:?}"
        );
        assert_eq!((file.applied_count, file.expired_count), (1, 2));
        assert_eq!((file.retained_count, file.notices_count), (0, 2));
        let listed: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(tmp.path().join(file.lists_file.as_ref().unwrap())).unwrap(),
        )
        .unwrap();
        assert!(
            file.lists_file
                .as_deref()
                .unwrap()
                .starts_with(".ductus/results/process-waivers/001-x/"),
        );
        assert_eq!(listed["applied"], serde_json::json!(inline.applied));
        assert_eq!(listed["expired"], serde_json::json!(inline.expired));
        assert_eq!(listed["retained"], serde_json::json!([]));
        assert_eq!(listed["notices"], serde_json::json!(inline.notices));
        let wire = serde_json::to_value(&file).unwrap();
        for list in ["applied", "expired", "retained", "notices"] {
            assert!(wire.get(list).is_none(), "{list} stays in the file: {wire}");
        }
        bounded_lists::assert_counts_name_the_file("process-waivers", &wire, &listed);

        // A retained anchor, on a dimension-restricted run, moves with the rest and is in
        // the file whole.
        read_size(tmp.path(), "10");
        let held = super::run(
            &restricted_args("001-x", &[("SEC-BE-014", "src/a.ts")], &["security"]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(held.form, ListsForm::File, "{held:?}");
        assert_eq!((held.applied_count, held.retained_count), (1, 2));
        let held_lists: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(tmp.path().join(held.lists_file.as_ref().unwrap())).unwrap(),
        )
        .unwrap();
        assert_eq!(held_lists["retained"].as_array().unwrap().len(), 2);
        assert_eq!(held_lists["expired"], serde_json::json!([]));

        // Each list alone is enough to move: a result of one applied anchor and no notice,
        // and one of a notice and no anchor (a malformed waiver).
        let only_applied = setup("003-z", &list_waiver(&["src/a.ts"]), &["src/a.ts"]);
        read_size(only_applied.path(), "10");
        let moved = super::run(
            &args("003-z", &[("SEC-BE-014", "src/a.ts")]),
            only_applied.path(),
        )
        .unwrap();
        assert_eq!(moved.form, ListsForm::File, "{moved:?}");
        assert_eq!((moved.applied_count, moved.notices_count), (1, 0));
        let malformed = "    - rule: SEC-BE-014\n      file: src/a.ts\n      waived-at: 2026-05-10T14:40:00Z\n      waived-by: dev@example.com\n";
        let only_notice = setup("004-w", malformed, &["src/a.ts"]);
        read_size(only_notice.path(), "10");
        let moved = super::run(&args("004-w", &[]), only_notice.path()).unwrap();
        assert_eq!(moved.form, ListsForm::File, "{moved:?}");
        assert_eq!((moved.applied_count, moved.notices_count), (0, 1));

        // Nothing to move reads no setting: a malformed read size does not halt it, and a
        // result with a list to bound does.
        read_size(tmp.path(), "\"big\"");
        let empty = setup("002-y", "", &[]);
        read_size(empty.path(), "\"big\"");
        let none = super::run(&args("002-y", &[]), empty.path()).unwrap();
        assert_eq!(none.form, ListsForm::Inline);
        let err = super::run(&firing, tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidReadSize { .. }),
            "{err}"
        );
    }
}
