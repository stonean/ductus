//! `discover-rule-files` — deterministic rule-file selection for `/ductus:review`,
//! and with `all` for `/ductus:analyze`, which loads every file.
//!
//! With `rules` it also lists each selected file's rules, as the shared
//! `rule_sections` parser reads them, so neither command reads a rule file
//! through the shell; under `all` each file's headings that open no rule come
//! back too, for analyze's integrity checks (spec 080).
//!
//! Owns `/ductus:review`'s rule-file selection end-to-end: list the rule-file
//! directory
//! (`framework/rules/` in ductus's own repo, `{specs-root}/rules/` in
//! adopters), classify each file by basename suffix, apply the
//! `[rules] surfaces` selection, then the `[[review.disabled-rule-files]]`
//! filter — returning the selected set plus the ordered stdout notice lines
//! the command must emit verbatim.
//!
//! The `[rules] surfaces` key, when set, is authoritative (and validated with
//! the fail-fast semantics review documents). When unset, selection falls back
//! to the host-detected stack passed in `detected-surfaces` — stack detection
//! itself is a semantic step that stays with the host/LLM; the primitive only
//! consumes the resolved surfaces. When neither is supplied, every recognized
//! surface is loaded (conservative: review everything).
//!
//! Defined by
//! `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::primitives::rule_sections::{other_headings, parse_rule_sections};
use crate::primitives::rule_verdicts::{self, LoadedRuleFile};
use crate::primitives::{PrimitiveError, Result, flatten_line, read_text};
use crate::schema::config_keys::{self, UnknownKey};
use crate::schema::paths;
use crate::schema::primitives::{
    DiscoverRuleFilesArgs, DiscoverRuleFilesResult, ListsForm, LoadedRule, OtherHeading,
    ReusableVerdict, RuleFileRules, RuleVerdictsBlock, UnreadableRuleFile,
};
use crate::schema::severity::RuleSeverity;

/// Recognized, selectable surface members for `[rules] surfaces`.
/// Cross-cutting (`-cross.md`) files are unconditional and are not a
/// selectable surface — listing `"cross"` is rejected.
const VALID_SURFACES: &[&str] = &["backend", "frontend"];

/// Minimum trimmed length (Unicode codepoints) of a disabled-rule-file
/// `reason`. Mirrors the review contract's audit-trail requirement.
pub(crate) const MIN_REASON_LEN: usize = 16;

/// Suffix classification of a rule file.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Surface {
    /// `*-backend.md`.
    Backend,
    /// `*-frontend.md`.
    Frontend,
    /// `*-cross.md` — loaded unconditionally.
    Cross,
    /// Any other suffix — loaded for every stack with a warning.
    Unrecognized,
}

/// Classify a rule-file basename by its suffix.
fn classify(basename: &str) -> Surface {
    if basename.ends_with("-backend.md") {
        Surface::Backend
    } else if basename.ends_with("-frontend.md") {
        Surface::Frontend
    } else if basename.ends_with("-cross.md") {
        Surface::Cross
    } else {
        Surface::Unrecognized
    }
}

/// Execute the `discover-rule-files` primitive.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidArgument`] when `verification` or
///   `extra-files` is given without `rules`, which it would add to.
/// - [`PrimitiveError::Toml`] when the resolved config file is malformed,
///   [`PrimitiveError::Io`] when it cannot be read, and
///   [`PrimitiveError::InvalidSpecsRoot`] when its `[paths] specs-root` is
///   invalid.
/// - Without `all`, which selects past them: [`PrimitiveError::InvalidSurfacesMember`]
///   when `[rules] surfaces` or `detected-surfaces` names a value outside
///   `{backend, frontend}`, [`PrimitiveError::InvalidSurfacesType`] when the
///   key is not a list of strings, and [`PrimitiveError::InvalidDisabledRuleFile`]
///   when a `[[review.disabled-rule-files]]` entry is malformed.
/// - [`PrimitiveError::Io`] on filesystem failures listing the rule-file
///   directory.
/// - With `rules`, [`PrimitiveError::InvalidPath`] for an `extra-files` entry
///   leaving the project, and every error `bounded_lists::place` returns: a
///   malformed read size, or a list file that cannot be written.
/// - With `all`, `rules` and `feature`, [`PrimitiveError::FeatureNotFound`] and the
///   feature-argument refusals of `validate_feature_argument`.
pub fn run(args: &DiscoverRuleFilesArgs, repo: &Path) -> Result<DiscoverRuleFilesResult> {
    for (argument, given) in [
        ("verification", args.verification),
        ("extra-files", !args.extra_files.is_empty()),
    ] {
        if given && !args.rules {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "discover-rule-files".into(),
                argument: argument.into(),
                reason: "it adds to the rules `rules` lists, so without `rules` it does nothing"
                    .into(),
            });
        }
    }
    // Resolved once: the same probe backs both the read below and the
    // provenance tag in the disabled-rule-file notice.
    let (config, unknown, config_name, config_path) = load_ductus_toml(repo)?;
    // Every entry is validated before any rule file is selected, so a
    // malformed one halts the run instead of leaving a partial selection.
    // Under `all` the entries select nothing, so a malformed review setting
    // cannot halt an analysis.
    let disabled = if args.all {
        Vec::new()
    } else {
        validate_disabled_entries(config.review.as_ref(), &config_path)?
    };
    let (rules_dir_path, rules_dir_rel) = resolve_rules_dir(repo)?;

    let mut notices: Vec<String> = unknown.iter().map(|key| key.notice(config_name)).collect();
    let mut selected: Vec<String> = Vec::new();

    if let Some(dir) = rules_dir_path {
        let all = list_rule_files(&dir)?;
        let surfaces = if args.all {
            Vec::new()
        } else {
            resolve_surfaces(config.rules.as_ref(), &args.detected_surfaces)?
        };

        for name in &all {
            if args.all {
                selected.push(name.clone());
                if classify(name) == Surface::Unrecognized {
                    notices.push(unrecognized_suffix_notice(name));
                }
                continue;
            }
            match classify(name) {
                Surface::Backend => {
                    if has(&surfaces, "backend") {
                        selected.push(name.clone());
                    }
                }
                Surface::Frontend => {
                    if has(&surfaces, "frontend") {
                        selected.push(name.clone());
                    }
                }
                Surface::Cross => selected.push(name.clone()),
                Surface::Unrecognized => {
                    selected.push(name.clone());
                    notices.push(unrecognized_suffix_notice(name));
                }
            }
        }

        // Under `all` no entry was validated, so `disabled` is empty and the
        // filter removes nothing.
        apply_disabled_filter(&disabled, &all, &mut selected, &mut notices, config_name);
        // A `*.md` entry that is no regular file, a symlink among them, is
        // not loaded: named, so a file that was never examined does not read
        // as one that does not exist (`QUAL-CLAIM-001`).
        for name in non_file_rule_entries(&dir) {
            notices.push(format!(
                "rule file {} is not a regular file — not loaded; replace it with the file itself",
                flatten_line(&name)
            ));
        }
    }

    selected.sort();
    notices.push(format!("loading rule files: {}", selected.join(", ")));

    let mut result = DiscoverRuleFilesResult {
        rules_dir: rules_dir_rel,
        selected,
        notices,
        ..DiscoverRuleFilesResult::default()
    };
    if args.rules {
        list_rules(args, repo, &mut result)?;
    }
    Ok(result)
}

/// With `rules`, each selected file's rules and then each extra file's,
/// parsed by [`parse_rule_sections`], the reading `check-rule-ids` and the
/// exec walker share; a file that cannot be read is listed under
/// `unreadable`. The list is returned inline while the result fits
/// `[artifacts] read-size-bytes`, and past it in the file the result names
/// (`bounded_lists::place`; spec 080, AC7).
fn list_rules(
    args: &DiscoverRuleFilesArgs,
    repo: &Path,
    result: &mut DiscoverRuleFilesResult,
) -> Result<()> {
    let mut files: Vec<String> = result
        .selected
        .iter()
        .map(|name| format!("{}/{name}", result.rules_dir))
        .collect();
    for extra in &args.extra_files {
        super::validate_contained(repo, extra)?;
        files.push(extra.clone());
    }
    let mut rule_files = Vec::new();
    // The files a run records its rule verdicts over: the selected ones, which
    // under `all` is every file in the directory, never the extra files.
    let mut loaded: Vec<LoadedRuleFile> = Vec::new();
    let selected = result.selected.len();
    for (index, file) in files.into_iter().enumerate() {
        let content = match std::fs::read_to_string(repo.join(&file)) {
            Ok(content) => content,
            Err(err) => {
                result.unreadable.push(UnreadableRuleFile {
                    file,
                    reason: err.to_string(),
                });
                continue;
            }
        };
        if index < selected
            && let Some(name) = Path::new(&file).file_name().and_then(|name| name.to_str())
        {
            loaded.push(LoadedRuleFile::read(name, &content));
        }
        let rules = parse_rule_sections(&content)
            .into_iter()
            .map(|section| LoadedRule {
                tier: match section.tier() {
                    Some(RuleSeverity::Must) => Some("must".to_owned()),
                    Some(RuleSeverity::Should) => Some("should".to_owned()),
                    _ => None,
                },
                verification: section.verification.clone().filter(|_| args.verification),
                id: section.id,
                heading: section.heading,
                statement: section.statement,
            })
            .collect();
        // What the shared parser gives up, a malformed rule heading, which
        // opens no rule there, a tab-separated one among them, which the
        // shared heading parser does not take: listed with the lines under
        // each heading, so analyze's integrity checks judge it without
        // reading the file.
        let other_headings = args.all.then(|| {
            other_headings(&content)
                .into_iter()
                .map(|(line, heading, body)| OtherHeading {
                    line,
                    heading,
                    body,
                })
                .collect()
        });
        rule_files.push(RuleFileRules {
            file,
            rules,
            other_headings,
        });
    }
    result.rule_files_count = Some(rule_files.len());
    result.rule_files = Some(rule_files);
    result.form = Some(ListsForm::Inline);
    if args.all
        && let Some(feature) = &args.feature
    {
        let block = verdicts_block(repo, feature, &loaded, &mut result.notices)?;
        result.verdicts = Some(block);
    }
    let mut lists = serde_json::json!({ "rule-files": &result.rule_files });
    if let Some(block) = &result.verdicts {
        lists["reusable"] = serde_json::json!(&block.reusable);
        lists["judge"] = serde_json::json!(&block.judge);
    }
    if let Some(file) = super::bounded_lists::place(
        repo,
        "discover-rule-files",
        super::bounded_lists::ALL_SPECS,
        &*result,
        &lists,
    )? {
        result.form = Some(ListsForm::File);
        result.rule_files = None;
        if let Some(block) = &mut result.verdicts {
            block.reusable = None;
            block.judge = None;
        }
        result.lists_file = Some(file);
    }
    Ok(())
}

/// The stored rule verdicts for `feature` over `loaded`: which rules to
/// judge and which fired verdicts to report again (spec 082). A record that
/// does not parse is named in a notice, so judging every rule is never
/// mistaken for holding no verdicts.
fn verdicts_block(
    repo: &Path,
    feature: &str,
    loaded: &[LoadedRuleFile],
    notices: &mut Vec<String>,
) -> Result<RuleVerdictsBlock> {
    let root = paths::Paths::load(repo)?.specs_root;
    super::validate_feature_argument(repo, &root, feature)?;
    let feature_dir = repo.join(&root).join(feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: feature.to_string(),
        });
    }
    let standing = rule_verdicts::standing(&feature_dir, loaded);
    if let Some(rule_verdicts::NoReuse::RecordUnreadable(reason)) = &standing.no_reuse {
        notices.push(format!(
            "{feature}'s analysis.md carries no readable record ({}) — every rule is to be judged",
            flatten_line(reason)
        ));
    }
    let tier = |tier: RuleSeverity| match tier {
        RuleSeverity::Must => "must",
        _ => "should",
    };
    let reusable: Vec<ReusableVerdict> = standing
        .reusable()
        .map(|(rule, verdict)| ReusableVerdict {
            id: rule.id.clone(),
            tier: tier(rule.tier).to_string(),
            verdict: verdict.clone(),
        })
        .collect();
    let judge: Vec<String> = standing.judge().map(|rule| rule.id.clone()).collect();
    Ok(RuleVerdictsBlock {
        reason: standing.no_reuse.as_ref().map(|why| why.code().to_string()),
        reusable_count: reusable.len(),
        judge_count: judge.len(),
        reusable: Some(reusable),
        judge: Some(judge),
    })
}

/// `true` when `surfaces` contains `member`.
fn has(surfaces: &[String], member: &str) -> bool {
    surfaces.iter().any(|s| s == member)
}

/// Locate the rule-file directory. Adopters keep rules under
/// `{specs-root}/rules/`; ductus's own repo keeps them under
/// `framework/rules/`. Prefer the adopter path when it exists (that is the
/// discriminator — a ductus checkout has `framework/rules/` but no
/// `specs/rules/`). Returns `(None, "")` when neither exists, and an error
/// when the project config cannot be read or parsed.
pub(crate) fn resolve_rules_dir(repo: &Path) -> Result<(Option<PathBuf>, String)> {
    let layout = paths::Paths::load(repo)?;
    let adopter = repo.join(&layout.specs_root).join("rules");
    if adopter.is_dir() {
        return Ok((Some(adopter), format!("{}/rules", layout.specs_root)));
    }
    let own = repo.join("framework").join("rules");
    if own.is_dir() {
        return Ok((Some(own), "framework/rules".to_string()));
    }
    Ok((None, String::new()))
}

/// The notice for a rule file whose suffix names no surface.
fn unrecognized_suffix_notice(name: &str) -> String {
    format!(
        "rule file {name} has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md"
    )
}

/// The `*.md` entries directly under `dir` that [`list_rule_files`] leaves
/// out because they are not regular files, sorted. Best effort: an entry that
/// cannot be inspected is left out, since the listing already read the
/// directory.
fn non_file_rule_entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|kind| !kind.is_file()))
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        })
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

/// List `*.md` basenames directly under `dir`, sorted for determinism.
pub(crate) fn list_rule_files(dir: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let read_dir = std::fs::read_dir(dir).map_err(|source| PrimitiveError::Io {
        path: dir.into(),
        source,
    })?;
    for entry in read_dir {
        let entry = entry.map_err(|source| PrimitiveError::Io {
            path: dir.into(),
            source,
        })?;
        let file_type = entry.file_type().map_err(|source| PrimitiveError::Io {
            path: entry.path(),
            source,
        })?;
        if !file_type.is_file() {
            continue;
        }
        if entry
            .path()
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}

/// Resolve the effective surfaces. Config `[rules] surfaces`, when present, is
/// authoritative (and validated); when unset, fall back to `detected`; when
/// that is also empty, load every recognized surface.
fn resolve_surfaces(rules: Option<&RulesSection>, detected: &[String]) -> Result<Vec<String>> {
    match rules.and_then(|r| r.surfaces.as_ref()) {
        Some(value) => validate_surfaces(value),
        None => {
            if detected.is_empty() {
                Ok(VALID_SURFACES.iter().map(|s| (*s).to_string()).collect())
            } else {
                // The `detected-surfaces` arg crosses the MCP boundary, so
                // validate its members the same way a `[rules] surfaces`
                // config value is validated. An unrecognized member (e.g.
                // "Backend" or "back-end") would otherwise select neither
                // surface and silently drop that surface's rule files from
                // the review set — fail fast instead.
                for member in detected {
                    validate_surface_member(member)?;
                }
                Ok(detected.to_vec())
            }
        }
    }
}

/// Reject a surface name outside `{backend, frontend}`. Shared by the
/// `[rules] surfaces` config check and the MCP `detected-surfaces` check so
/// the two agree on the accepted set.
fn validate_surface_member(member: &str) -> Result<()> {
    if VALID_SURFACES.contains(&member) {
        Ok(())
    } else {
        Err(PrimitiveError::InvalidSurfacesMember {
            value: member.to_string(),
        })
    }
}

/// Validate a `[rules] surfaces` value: it must be a list of strings, each a
/// member of `{backend, frontend}`. The empty list is valid (cross-only).
fn validate_surfaces(value: &toml::Value) -> Result<Vec<String>> {
    let toml::Value::Array(items) = value else {
        return Err(PrimitiveError::InvalidSurfacesType {
            got: toml_type_name(value).to_string(),
        });
    };
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let toml::Value::String(member) = item else {
            return Err(PrimitiveError::InvalidSurfacesType {
                got: format!("a list containing {}", toml_type_name(item)),
            });
        };
        validate_surface_member(member)?;
        out.push(member.clone());
    }
    Ok(out)
}

/// Human-readable TOML type name for the type-mismatch diagnostic.
pub(crate) fn toml_type_name(value: &toml::Value) -> &'static str {
    match value {
        toml::Value::String(_) => "a string",
        toml::Value::Integer(_) => "an integer",
        toml::Value::Float(_) => "a float",
        toml::Value::Boolean(_) => "a boolean",
        toml::Value::Datetime(_) => "a datetime",
        toml::Value::Array(_) => "an array",
        toml::Value::Table(_) => "a table",
    }
}

/// A well-formed `[[review.disabled-rule-files]]` entry.
struct DisabledEntry<'a> {
    /// The rule-file basename the entry disables.
    file: &'a str,
    /// The audit-trail justification, at least [`MIN_REASON_LEN`] codepoints
    /// once trimmed.
    reason: &'a str,
}

/// Validate every `[[review.disabled-rule-files]]` entry, in config order,
/// and return them all or halt on the first malformed one. `config` is the
/// resolved config file the entries were read from, named in the error.
///
/// A malformed entry is a configuration value that does not parse into its
/// declared shape, so it fails fast rather than being skipped (`CFG-ENV-008`):
/// skipping it would leave the rule file enforced while the operator believes
/// it disabled, or the reverse after a later fix, with nothing to say which.
/// A duplicate is well-formed and is not judged here.
fn validate_disabled_entries<'a>(
    review: Option<&'a ReviewSection>,
    config: &Path,
) -> Result<Vec<DisabledEntry<'a>>> {
    let Some(review) = review else {
        return Ok(Vec::new());
    };
    review
        .disabled_rule_files
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            validate_disabled_entry(entry).map_err(|problem| {
                PrimitiveError::InvalidDisabledRuleFile {
                    path: config.to_path_buf(),
                    index,
                    problem,
                }
            })
        })
        .collect()
}

/// The two fields of a `[[review.disabled-rule-files]]` entry this primitive
/// reads; the shared registry declares the same pair (spec 019).
const FILE: &str = "file";
const REASON: &str = "reason";

/// One entry as a [`DisabledEntry`], or the problem naming the field that
/// failed and the offending value, rendered as TOML on one line
/// ([`rendered`]).
fn validate_disabled_entry(entry: &toml::Value) -> std::result::Result<DisabledEntry<'_>, String> {
    let toml::Value::Table(table) = entry else {
        return Err(format!(
            "the entry is {} ({}), not a table",
            toml_type_name(entry),
            rendered(entry)
        ));
    };
    let string_field = |name: &str| match table.get(name) {
        None => Err(format!("`{name}` is missing")),
        Some(toml::Value::String(value)) => Ok(value.as_str()),
        Some(other) => Err(format!(
            "`{name}` is {} ({}), not a string",
            toml_type_name(other),
            rendered(other)
        )),
    };
    let file = string_field(FILE)?;
    let reason = string_field(REASON)?;
    let length = reason.trim().chars().count();
    if length < MIN_REASON_LEN {
        return Err(format!(
            "`reason` {} is {length} characters once trimmed, under the minimum of {MIN_REASON_LEN}",
            rendered(&toml::Value::String(reason.to_owned()))
        ));
    }
    Ok(DisabledEntry { file, reason })
}

/// A config value as TOML renders it, flattened to one line: committed
/// config can carry a newline or an escape sequence, and an operator-facing
/// line must not let it forge a second line or restyle the terminal
/// (`BE-INPUT-011`).
pub(crate) fn rendered(value: &toml::Value) -> String {
    flatten_line(&value.to_string())
}

/// Apply the validated `[[review.disabled-rule-files]]` entries to the
/// post-selection set, appending the ordered notice lines for each entry.
/// Entries are processed in config order; the first entry for a given `file`
/// applies, later duplicates warn. `config_name` is the repo-relative
/// resolved config file the disable came from, rendered in the drop notice's
/// provenance tag — the newest existing of `.ductus/config.toml`,
/// `.govern/config.toml`, or the legacy root `.govern.toml`, per
/// `schema::paths`'s `CONFIG_CHAIN`.
fn apply_disabled_filter(
    disabled: &[DisabledEntry<'_>],
    all: &[String],
    selected: &mut Vec<String>,
    notices: &mut Vec<String>,
    config_name: &str,
) {
    let mut seen: Vec<&str> = Vec::new();
    for (index, entry) in disabled.iter().enumerate() {
        let (file, reason) = (entry.file, entry.reason);
        // Matched as written, rendered on one line (`BE-INPUT-011`): the
        // notices are emitted verbatim, so a raw newline in either value
        // would forge a notice line of its own.
        let shown = flatten_line(file);
        if seen.contains(&file) {
            notices.push(format!(
                "duplicate disabled-rule-file: {shown} — entry [{index}] ignored"
            ));
            continue;
        }
        seen.push(file);

        if let Some(pos) = selected.iter().position(|s| s == file) {
            selected.remove(pos);
            notices.push(format!(
                "disabled-rule-file: {shown} — {} ({config_name})",
                super::collapse_whitespace(&flatten_line(reason))
            ));
        } else if all.iter().any(|s| s == file) {
            notices.push(format!(
                "disabled-rule-file (no-op): {shown} not selected by stack detection"
            ));
        } else {
            notices.push(format!(
                "unknown disabled-rule-file: {shown} (no such file in the rule-file directory)"
            ));
        }
    }
}

/// Parse the resolved config file, returning defaults when it is absent, plus
/// the unknown keys in the two sections this primitive reads, the
/// repo-relative name that resolution picked, and its path. Returning the name
/// here — rather than re-probing at render time — is what keeps the parsed
/// content, its provenance tag and the path an error names from disagreeing
/// across a concurrent migration.
fn load_ductus_toml(repo: &Path) -> Result<(DuctusToml, Vec<UnknownKey>, &'static str, PathBuf)> {
    let (path, name) = paths::resolve_config(repo);
    if !path.is_file() {
        return Ok((DuctusToml::default(), Vec::new(), name, path));
    }
    let content = read_text(&path)?;
    let parsed = toml::from_str(&content).map_err(|source| PrimitiveError::Toml {
        path: path.clone(),
        source,
    })?;
    let unknown = config_keys::unknown_keys_in_text(&content, &["rules", "review"]);
    Ok((parsed, unknown, name, path))
}

/// Minimal `.ductus/config.toml` shape: the `[rules]` and `[review]` sections
/// this primitive consults. A key outside the shared registry is reported as a
/// notice, never an error (spec 019). Resolution is the newest-wins ladder in
/// `schema::paths` (`.ductus/` → `.govern/` → legacy root), so an older tier
/// still parses into this same shape.
#[derive(Deserialize, Default)]
struct DuctusToml {
    #[serde(default)]
    rules: Option<RulesSection>,
    #[serde(default)]
    review: Option<ReviewSection>,
}

/// `[rules]` — `surfaces` is kept as a raw `toml::Value` so the primitive can
/// distinguish unset (None) from an empty list, and produce the contract's
/// type/member diagnostics rather than a generic parse error.
#[derive(Deserialize, Default)]
struct RulesSection {
    #[serde(default)]
    surfaces: Option<toml::Value>,
}

/// `[review]` — the `disabled-rule-files` entries, kept as raw values so a
/// malformed entry (not a table, a missing or mistyped `file`/`reason`, a
/// short `reason`) halts naming its index and field rather than surfacing as
/// a generic parse error.
#[derive(Deserialize, Default)]
struct ReviewSection {
    #[serde(default, rename = "disabled-rule-files")]
    disabled_rule_files: Vec<toml::Value>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use tempfile::TempDir;

    /// Build a repo with `framework/rules/{files}` and an optional
    /// `.govern.toml`. Returns the tempdir.
    fn setup(files: &[&str], ductus_toml: Option<&str>) -> TempDir {
        let tmp = TempDir::new().unwrap();
        let rules = tmp.path().join("framework/rules");
        std::fs::create_dir_all(&rules).unwrap();
        for f in files {
            std::fs::write(rules.join(f), "# rule\n").unwrap();
        }
        if let Some(body) = ductus_toml {
            std::fs::write(tmp.path().join(".govern.toml"), body).unwrap();
        }
        tmp
    }

    fn args(detected: &[&str]) -> DiscoverRuleFilesArgs {
        DiscoverRuleFilesArgs {
            detected_surfaces: detected.iter().map(|s| (*s).to_string()).collect(),
            ..DiscoverRuleFilesArgs::default()
        }
    }

    const THREE: &[&str] = &[
        "security-backend.md",
        "accessibility-frontend.md",
        "quality-cross.md",
    ];

    #[test]
    fn rejects_unrecognized_detected_surface_member() {
        // The MCP-boundary `detected-surfaces` arg is validated like a config
        // value: an unrecognized member fails fast rather than silently
        // selecting neither surface and dropping that surface's rule files.
        let tmp = setup(THREE, None);
        let err = run(&args(&["Backend"]), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidSurfacesMember { value } if value == "Backend"),
            "expected InvalidSurfacesMember, got {err:?}"
        );
    }

    #[test]
    fn unset_surfaces_no_detection_loads_all_recognized() {
        let tmp = setup(THREE, None);
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert_eq!(
            result.selected,
            vec![
                "accessibility-frontend.md",
                "quality-cross.md",
                "security-backend.md"
            ]
        );
        assert_eq!(result.rules_dir, "framework/rules");
        assert_eq!(
            result.notices,
            vec![
                "loading rule files: accessibility-frontend.md, quality-cross.md, security-backend.md"
            ]
        );
    }

    #[test]
    fn surfaces_backend_keeps_backend_and_cross_drops_frontend() {
        let tmp = setup(THREE, Some("[rules]\nsurfaces = [\"backend\"]\n"));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert_eq!(
            result.selected,
            vec!["quality-cross.md", "security-backend.md"]
        );
    }

    #[test]
    fn empty_surfaces_is_cross_only() {
        let tmp = setup(THREE, Some("[rules]\nsurfaces = []\n"));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert_eq!(result.selected, vec!["quality-cross.md"]);
    }

    #[test]
    fn unset_surfaces_falls_back_to_detected_stack() {
        let tmp = setup(THREE, None);
        let result = run(&args(&["frontend"]), tmp.path()).unwrap();
        assert_eq!(
            result.selected,
            vec!["accessibility-frontend.md", "quality-cross.md"]
        );
    }

    #[test]
    fn config_surfaces_override_detected_stack() {
        // Config wins even when a detected stack is supplied.
        let tmp = setup(THREE, Some("[rules]\nsurfaces = [\"backend\"]\n"));
        let result = run(&args(&["frontend"]), tmp.path()).unwrap();
        assert_eq!(
            result.selected,
            vec!["quality-cross.md", "security-backend.md"]
        );
    }

    #[test]
    fn unrecognized_member_fails_fast() {
        let tmp = setup(THREE, Some("[rules]\nsurfaces = [\"fullstack\"]\n"));
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        match err {
            PrimitiveError::InvalidSurfacesMember { value } => assert_eq!(value, "fullstack"),
            other => panic!("expected InvalidSurfacesMember, got {other:?}"),
        }
    }

    #[test]
    fn mixed_valid_and_invalid_members_fail_on_the_invalid_one() {
        let tmp = setup(
            THREE,
            Some("[rules]\nsurfaces = [\"backend\", \"fullstack\"]\n"),
        );
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        assert!(matches!(
            err,
            PrimitiveError::InvalidSurfacesMember { value } if value == "fullstack"
        ));
    }

    #[test]
    fn cross_is_not_a_selectable_surface() {
        let tmp = setup(THREE, Some("[rules]\nsurfaces = [\"cross\"]\n"));
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        assert!(matches!(
            err,
            PrimitiveError::InvalidSurfacesMember { value } if value == "cross"
        ));
    }

    #[test]
    fn non_list_surfaces_is_a_type_error() {
        let tmp = setup(THREE, Some("[rules]\nsurfaces = \"backend\"\n"));
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        match err {
            PrimitiveError::InvalidSurfacesType { got } => assert_eq!(got, "a string"),
            other => panic!("expected InvalidSurfacesType, got {other:?}"),
        }
    }

    #[test]
    fn unrecognized_suffix_loads_for_all_stacks_with_warning() {
        let tmp = setup(&["security-backend.md", "internal-notes.md"], None);
        let result = run(&args(&["backend"]), tmp.path()).unwrap();
        assert!(result.selected.contains(&"internal-notes.md".to_string()));
        assert_eq!(
            result.notices[0],
            "rule file internal-notes.md has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md"
        );
    }

    #[test]
    fn disabled_rule_file_drops_selected_file_with_notice() {
        let toml = "[[review.disabled-rule-files]]\nfile = \"accessibility-frontend.md\"\nreason = \"Internal admin UI; not yet adopting WCAG AA.\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(
            !result
                .selected
                .contains(&"accessibility-frontend.md".to_string())
        );
        assert!(result.notices.contains(
            &"disabled-rule-file: accessibility-frontend.md — Internal admin UI; not yet adopting WCAG AA. (.govern.toml)".to_string()
        ));
    }

    #[test]
    fn disabled_notice_names_new_layout_config() {
        // Same drop, config under `.ductus/config.toml` — the provenance
        // tag names the resolved file, not a hardcoded legacy literal.
        let toml = "[[review.disabled-rule-files]]\nfile = \"accessibility-frontend.md\"\nreason = \"Internal admin UI; not yet adopting WCAG AA.\"\n";
        let tmp = setup(THREE, None);
        let cfg_dir = tmp.path().join(".ductus");
        std::fs::create_dir_all(&cfg_dir).unwrap();
        std::fs::write(cfg_dir.join("config.toml"), toml).unwrap();
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(result.notices.contains(
            &"disabled-rule-file: accessibility-frontend.md — Internal admin UI; not yet adopting WCAG AA. (.ductus/config.toml)".to_string()
        ));
    }

    #[test]
    fn disabled_rule_file_non_selected_is_a_no_op_notice() {
        // Backend-only selection; disabling a frontend file that isn't selected.
        let toml = "[rules]\nsurfaces = [\"backend\"]\n\n[[review.disabled-rule-files]]\nfile = \"accessibility-frontend.md\"\nreason = \"Not adopting WCAG AA on this surface yet.\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(result.notices.contains(
            &"disabled-rule-file (no-op): accessibility-frontend.md not selected by stack detection".to_string()
        ));
    }

    #[test]
    fn disabled_rule_file_unknown_warns() {
        let toml = "[[review.disabled-rule-files]]\nfile = \"nonexistent-cross.md\"\nreason = \"This file was renamed or moved elsewhere.\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(result.notices.contains(
            &"unknown disabled-rule-file: nonexistent-cross.md (no such file in the rule-file directory)".to_string()
        ));
    }

    /// The error a malformed disabled entry halts with: its index, the
    /// problem it names, and the rendered message.
    fn malformed(toml: &str) -> (usize, String, String) {
        let tmp = setup(THREE, Some(toml));
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        let message = err.to_string();
        let PrimitiveError::InvalidDisabledRuleFile {
            path,
            index,
            problem,
        } = err
        else {
            panic!("expected InvalidDisabledRuleFile, got {err:?}");
        };
        assert!(path.ends_with(".govern.toml"), "{}", path.display());
        (index, problem, message)
    }

    #[test]
    fn every_malformed_entry_halts_naming_its_index_and_field() {
        // Each shape fails fast (CFG-ENV-008) rather than being skipped: a
        // skipped entry would leave the file enforced, or unenforced after a
        // later fix, with nothing to say which.
        for (entry, problem) in [
            (
                "\"quality-cross.md\"",
                "the entry is a string (\"quality-cross.md\"), not a table",
            ),
            (
                "{ reason = \"Not adopting this file yet, see PROJ-1.\" }",
                "`file` is missing",
            ),
            (
                "{ file = 5, reason = \"Not adopting this file yet, see PROJ-1.\" }",
                "`file` is an integer (5), not a string",
            ),
            ("{ file = \"quality-cross.md\" }", "`reason` is missing"),
            (
                "{ file = \"quality-cross.md\", reason = true }",
                "`reason` is a boolean (true), not a string",
            ),
            (
                "{ file = \"quality-cross.md\", reason = \"  too short     \" }",
                "`reason` \"  too short     \" is 9 characters once trimmed, under the minimum of 16",
            ),
        ] {
            let toml = format!("[review]\ndisabled-rule-files = [{entry}]\n");
            let (index, found, message) = malformed(&toml);
            assert_eq!(index, 0, "{entry}");
            assert_eq!(found, problem, "{entry}");
            assert!(
                message.starts_with("invalid [[review.disabled-rule-files]] entry [0] in "),
                "{message}"
            );
            assert!(message.contains(problem), "{message}");
            assert!(message.contains("at least 16 characters"), "{message}");
        }
    }

    #[test]
    fn a_malformed_entry_halts_after_well_formed_ones() {
        // The second entry is malformed; the first, valid, entry is not
        // applied, because the run halts before any rule file is selected.
        let toml = "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"First entry, this one is fine.\"\n\n[[review.disabled-rule-files]]\nfile = \"security-backend.md\"\nreason = \"short\"\n";
        let (index, problem, _) = malformed(toml);
        assert_eq!(index, 1);
        assert!(problem.starts_with("`reason` \"short\""), "{problem}");
    }

    #[test]
    fn a_malformed_entry_halts_with_no_rule_directory() {
        // Validation does not wait for a rule file to select.
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join(".govern.toml"),
            "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\n",
        )
        .unwrap();
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        assert!(
            matches!(
                err,
                PrimitiveError::InvalidDisabledRuleFile { index: 0, .. }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn a_short_reason_is_named_on_one_line() {
        // TOML renders a string holding a newline as a multi-line string,
        // raw newlines and all; the problem flattens it.
        let (_, problem, message) = malformed(
            "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"a\\nb\"\n",
        );
        assert!(
            problem.contains("a b") && !problem.contains('\n'),
            "{problem:?}"
        );
        assert!(!message.contains('\n'), "{message:?}");
    }

    #[test]
    fn a_non_ascii_reason_is_shown_as_written() {
        let (_, problem, _) = malformed(
            "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"हिन्दी कारण\"\n",
        );
        assert!(
            problem.starts_with("`reason` \"हिन्दी कारण\" is "),
            "{problem}"
        );
    }

    #[test]
    fn notices_render_config_values_on_one_line() {
        // A newline in `file` would forge a notice line, and an escape
        // sequence in `reason` would restyle the terminal (BE-INPUT-011).
        let toml = "[[review.disabled-rule-files]]\nfile = \"nope.md\\nloading rule files: (forged)\"\nreason = \"Renamed upstream, see PROJ-1.\"\n\n[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"\\u001b[31mNot adopting this file yet\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert_eq!(result.notices.len(), 3, "{:?}", result.notices);
        assert_eq!(
            result.notices[0],
            "unknown disabled-rule-file: nope.md loading rule files: (forged) (no such file in the rule-file directory)"
        );
        assert!(
            result.notices[1].starts_with("disabled-rule-file: quality-cross.md — "),
            "{:?}",
            result.notices[1]
        );
        for notice in &result.notices {
            assert!(
                !notice.contains('\n') && !notice.contains('\u{1b}'),
                "{notice:?}"
            );
        }
    }

    #[test]
    fn an_adopter_field_on_an_entry_is_accepted() {
        // The open-schema rule: a field an adopter adds, such as an approval,
        // is neither an error nor a reason to skip the entry.
        let toml = "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"Not adopting this file yet, PROJ-1.\"\napproved-by = \"ops@example.com\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(!result.selected.contains(&"quality-cross.md".to_string()));
        // Reported as informational all the same (spec 019).
        assert_eq!(
            result.notices[0],
            "unknown config key: [[review.disabled-rule-files]] entry [0] approved-by (.govern.toml) — this version of ductus does not read it"
        );
    }

    #[test]
    fn a_mistyped_key_is_named_ahead_of_the_other_notices() {
        // `surface` for `surfaces` would otherwise be dropped, and the
        // selection fall back to stack detection without a word (spec 019).
        let toml = "[rules]\nsurface = [\"backend\"]\n[review]\ndisabled-rule-file = \"x\"\ntech-stack-verified = true\n[paths]\nroot = \"x\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&["frontend"]), tmp.path()).unwrap();
        assert_eq!(
            result.notices,
            [
                "unknown config key: [rules] surface (.govern.toml) — this version of ductus does not read it",
                "unknown config key: [review] disabled-rule-file (.govern.toml) — this version of ductus does not read it",
                "loading rule files: accessibility-frontend.md, quality-cross.md",
            ]
        );
    }

    #[test]
    fn the_registry_recognizes_every_key_this_reader_reads() {
        use crate::schema::config_keys::{fields_of, unrecognized};
        assert_eq!(
            unrecognized(&[], fields_of::<DuctusToml>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["rules"], fields_of::<RulesSection>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["review"], fields_of::<ReviewSection>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["review", "disabled-rule-files"], &[FILE, REASON]),
            [] as [String; 0]
        );
    }

    #[test]
    fn the_reason_length_counts_codepoints_not_bytes() {
        // Eight codepoints in twenty-four bytes is under the minimum, which a
        // byte count would accept.
        let (_, problem, _) = malformed(
            "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"対応は第三四半期\"\n",
        );
        assert!(problem.contains("is 8 characters"), "{problem}");
        // Sixteen codepoints in forty-eight bytes meets it.
        let toml = "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"アクセシビリティ対応は第三四半期\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(!result.selected.contains(&"quality-cross.md".to_string()));
    }

    #[test]
    fn duplicate_disabled_rule_file_first_applies_rest_warn() {
        let toml = "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"First entry, this one applies.\"\n\n[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"Second entry, should be ignored.\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(!result.selected.contains(&"quality-cross.md".to_string()));
        assert!(result.notices.contains(
            &"duplicate disabled-rule-file: quality-cross.md — entry [1] ignored".to_string()
        ));
    }

    #[test]
    fn reason_whitespace_is_collapsed_in_the_notice() {
        let toml = "[[review.disabled-rule-files]]\nfile = \"quality-cross.md\"\nreason = \"\"\"\nInternal admin UI;\nnot adopting WCAG AA yet.\n\"\"\"\n";
        let tmp = setup(THREE, Some(toml));
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert!(result.notices.contains(
            &"disabled-rule-file: quality-cross.md — Internal admin UI; not adopting WCAG AA yet. (.govern.toml)".to_string()
        ));
    }

    #[test]
    fn notice_ordering_unrecognized_then_disabled_then_loading() {
        let toml = "[[review.disabled-rule-files]]\nfile = \"security-backend.md\"\nreason = \"Backend rules disabled for this probe.\"\n";
        let tmp = setup(&["security-backend.md", "internal-notes.md"], Some(toml));
        let result = run(&args(&["backend"]), tmp.path()).unwrap();
        assert_eq!(result.notices.len(), 3);
        assert!(
            result.notices[0].starts_with("rule file internal-notes.md has unrecognized suffix")
        );
        assert!(result.notices[1].starts_with("disabled-rule-file: security-backend.md"));
        assert!(result.notices[2].starts_with("loading rule files:"));
    }

    #[test]
    fn adopter_layout_uses_specs_rules_directory() {
        let tmp = TempDir::new().unwrap();
        let rules = tmp.path().join("specs/rules");
        std::fs::create_dir_all(&rules).unwrap();
        std::fs::write(rules.join("quality-cross.md"), "# rule\n").unwrap();
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert_eq!(result.rules_dir, "specs/rules");
        assert_eq!(result.selected, vec!["quality-cross.md"]);
    }

    #[test]
    fn no_rule_directory_yields_empty_selection() {
        let tmp = TempDir::new().unwrap();
        let result = run(&args(&[]), tmp.path()).unwrap();
        assert_eq!(result.rules_dir, "");
        assert_eq!(result.selected, [] as [String; 0]);
        assert_eq!(result.notices, vec!["loading rule files: "]);
    }

    #[test]
    fn malformed_ductus_toml_is_operational_error() {
        let tmp = setup(THREE, Some("[[review.broken\n"));
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }));
    }

    /// Three rules: a MUST with a Verification, one with no tier, and a SHOULD.
    const RULES: &str = "# Rules\n\n### QUAL-TEST-001\n\n> A test MUST fail without its behavior.\n\n\
                         **Verification:** Read each test.\n\n### QUAL-TEST-002\n\n> Prefer small tests.\n\n\
                         ### QUAL-TEST-003\n\n> A test SHOULD name the behavior it holds.\n";

    fn with_rules(args: DiscoverRuleFilesArgs) -> DiscoverRuleFilesArgs {
        DiscoverRuleFilesArgs {
            rules: true,
            ..args
        }
    }

    /// Scenario `rules-load-without-shell-reads`: with `rules`, each selected
    /// file's rules come back, ID, tier and Statement, and the Verification
    /// only with `verification`. Without `rules` the result keeps its old
    /// shape.
    #[test]
    fn rules_lists_each_selected_files_rules() {
        let tmp = setup(&["quality-cross.md"], None);
        std::fs::write(tmp.path().join("framework/rules/quality-cross.md"), RULES).unwrap();

        let plain = run(&args(&[]), tmp.path()).unwrap();
        let value = serde_json::to_value(&plain).unwrap();
        assert!(
            value.get("rule-files").is_none() && value.get("form").is_none(),
            "{value}"
        );

        let result = run(&with_rules(args(&[])), tmp.path()).unwrap();
        assert_eq!(result.form, Some(ListsForm::Inline));
        assert_eq!(result.rule_files_count, Some(1));
        let files = result.rule_files.unwrap();
        assert_eq!(files[0].file, "framework/rules/quality-cross.md");
        let rules = &files[0].rules;
        assert_eq!(rules.len(), 3);
        assert_eq!(rules[0].id, "QUAL-TEST-001");
        assert_eq!(rules[0].heading, "### QUAL-TEST-001");
        assert_eq!(rules[0].tier.as_deref(), Some("must"));
        assert_eq!(rules[0].statement, "A test MUST fail without its behavior.");
        assert_eq!(
            rules[0].verification, None,
            "no Verification without the flag"
        );
        assert_eq!(
            rules[1].tier, None,
            "a Statement with no keyword has no tier"
        );
        assert_eq!(
            rules[2].tier.as_deref(),
            Some("should"),
            "a SHOULD Statement is should-tier"
        );
        assert_eq!(
            files[0].other_headings, None,
            "no other headings without `all`"
        );

        let verified = run(
            &DiscoverRuleFilesArgs {
                verification: true,
                ..with_rules(args(&[]))
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            verified.rule_files.unwrap()[0].rules[0]
                .verification
                .as_deref(),
            Some("Read each test.")
        );
    }

    /// `all` selects every file in the directory, past `[rules] surfaces` and
    /// the disabled-rule-files filter, as analyze loads them.
    #[test]
    fn all_selects_every_file_past_surfaces_and_the_disabled_filter() {
        let toml = "[rules]\nsurfaces = [\"backend\"]\n\n[[review.disabled-rule-files]]\n\
                    file = \"security-backend.md\"\nreason = \"Not adopting it on this surface yet.\"\n";
        let tmp = setup(THREE, Some(toml));
        let review = run(&args(&[]), tmp.path()).unwrap();
        assert_eq!(review.selected, vec!["quality-cross.md"]);
        let all = run(
            &DiscoverRuleFilesArgs {
                all: true,
                ..args(&[])
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            all.selected,
            vec![
                "accessibility-frontend.md",
                "quality-cross.md",
                "security-backend.md"
            ]
        );
        assert_eq!(
            all.notices.last().unwrap(),
            "loading rule files: accessibility-frontend.md, quality-cross.md, security-backend.md"
        );
    }

    /// A `*.md` entry that is no regular file is named in a notice rather than
    /// dropped, so a rule file never examined does not read as one that does
    /// not exist (`QUAL-CLAIM-001`).
    #[cfg(unix)]
    #[test]
    fn a_rule_entry_that_is_no_regular_file_is_named() {
        let tmp = setup(&["quality-cross.md"], None);
        std::fs::write(tmp.path().join("elsewhere.md"), RULES).unwrap();
        std::os::unix::fs::symlink(
            tmp.path().join("elsewhere.md"),
            tmp.path().join("framework/rules/linked-cross.md"),
        )
        .unwrap();
        let all = run(
            &DiscoverRuleFilesArgs {
                all: true,
                ..args(&[])
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(all.selected, vec!["quality-cross.md"]);
        assert!(
            all.notices.iter().any(|notice| notice
                == "rule file linked-cross.md is not a regular file — not loaded; replace it with the file itself"),
            "{:?}",
            all.notices
        );
    }

    /// Under `all` each file also lists the headings that open no rule, a
    /// malformed rule heading among them, since the parser sees no rule there
    /// and analyze's integrity checks must (spec 080's plan).
    #[test]
    fn all_lists_each_files_headings_that_open_no_rule() {
        let tmp = setup(&["quality-cross.md"], None);
        std::fs::write(
            tmp.path().join("framework/rules/quality-cross.md"),
            format!(
                "{RULES}\n## Testing\n\n### qual-test-004\n  \n> Lowercase.\n\n\
                 **Verification:** Read it.\n\n   \n"
            ),
        )
        .unwrap();
        let result = run(
            &DiscoverRuleFilesArgs {
                all: true,
                ..with_rules(args(&[]))
            },
            tmp.path(),
        )
        .unwrap();
        let files = result.rule_files.unwrap();
        let others: Vec<(usize, &str, &str)> = files[0]
            .other_headings
            .as_ref()
            .unwrap()
            .iter()
            .map(|h| (h.line, h.heading.as_str(), h.body.as_str()))
            .collect();
        // Each heading carries the lines under it up to the next heading,
        // blank lines at either end dropped, so the malformed rule's
        // Statement and Verification can be judged without reading the file.
        assert_eq!(
            others,
            [
                (1, "# Rules", ""),
                (17, "## Testing", ""),
                (
                    19,
                    "### qual-test-004",
                    "> Lowercase.\n\n**Verification:** Read it."
                )
            ]
        );
        assert_eq!(files[0].rules.len(), 3, "the malformed heading is no rule");
    }

    /// Under `all`, `[rules] surfaces` and the disabled entries select
    /// nothing, so a malformed one halts review's call but not analyze's.
    #[test]
    fn all_does_not_validate_review_only_settings() {
        let toml =
            "[rules]\nsurfaces = [\"middle\"]\n\n[[review.disabled-rule-files]]\nfile = \"x.md\"\n";
        let tmp = setup(THREE, Some(toml));
        assert!(run(&args(&[]), tmp.path()).is_err());
        let all = run(
            &DiscoverRuleFilesArgs {
                all: true,
                ..args(&[])
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(all.selected.len(), 3);
    }

    /// `verification` and `extra-files` add to what `rules` lists, so either
    /// without it is refused rather than ignored.
    #[test]
    fn verification_or_extra_files_without_rules_is_refused() {
        let tmp = setup(&["quality-cross.md"], None);
        for (call, argument) in [
            (
                DiscoverRuleFilesArgs {
                    verification: true,
                    ..args(&[])
                },
                "verification",
            ),
            (
                DiscoverRuleFilesArgs {
                    extra_files: vec!["docs/rules/x.md".to_string()],
                    ..args(&[])
                },
                "extra-files",
            ),
        ] {
            let err = run(&call, tmp.path()).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidArgument { argument: a, .. } if a == argument),
                "{err:?}"
            );
        }
    }

    /// A rule file outside the directory is listed after the selected ones,
    /// one that cannot be read is named under `unreadable`, and a path
    /// leaving the project is refused.
    #[test]
    fn extra_files_are_listed_and_an_unreadable_file_is_named() {
        let tmp = setup(&["quality-cross.md"], None);
        std::fs::create_dir_all(tmp.path().join("docs/rules")).unwrap();
        std::fs::write(tmp.path().join("docs/rules/internal-api.md"), RULES).unwrap();
        std::fs::write(
            tmp.path().join("framework/rules/quality-cross.md"),
            [0xff, 0xfe],
        )
        .unwrap();
        let result = run(
            &DiscoverRuleFilesArgs {
                extra_files: vec!["docs/rules/internal-api.md".into()],
                ..with_rules(args(&[]))
            },
            tmp.path(),
        )
        .unwrap();
        let files = result.rule_files.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file, "docs/rules/internal-api.md");
        assert_eq!(files[0].rules.len(), 3);
        assert_eq!(result.unreadable.len(), 1);
        assert_eq!(
            result.unreadable[0].file,
            "framework/rules/quality-cross.md"
        );

        let err = run(
            &DiscoverRuleFilesArgs {
                extra_files: vec!["../outside.md".into()],
                ..with_rules(args(&[]))
            },
            tmp.path(),
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
    }

    /// Past the read size the rules move to the file the result names, with
    /// their count inline (AC7).
    #[test]
    fn past_the_read_size_the_rules_move_to_a_file() {
        let tmp = setup(&["quality-cross.md"], None);
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 300\n",
        )
        .unwrap();
        std::fs::write(tmp.path().join("framework/rules/quality-cross.md"), RULES).unwrap();
        let result = run(&with_rules(args(&[])), tmp.path()).unwrap();
        assert_eq!(result.form, Some(ListsForm::File));
        assert!(result.rule_files.is_none());
        assert_eq!(result.rule_files_count, Some(1));
        let file = result.lists_file.clone().unwrap();
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(&file)).unwrap())
                .unwrap();
        assert_eq!(lists["rule-files"][0]["rules"][0]["id"], "QUAL-TEST-001");
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "discover-rule-files",
            &serde_json::to_value(&result).unwrap(),
            &lists,
        );
    }

    // -- stored rule verdicts (spec 082) ----------------------------------

    const VERDICT_RULES: &str = "### TS-RULE-001\n\n> A spec MUST name its owner.\n\n**Verification:** Look for an owner.\n\n\
        ### TS-RULE-002\n\n> A spec MUST name its constants.\n\n**Verification:** Look for constants.\n\n\
        ### TS-SHLD-001\n\n> A spec SHOULD name its cursor.\n\n**Verification:** Look for a cursor.\n\n\
        ### TS-NONE-001\n\n> A note about owners.\n\n**Verification:** None.\n";

    /// A project with one feature, `001-x`, and the rules above.
    fn verdict_project() -> TempDir {
        let tmp = setup(&["quality-cross.md"], None);
        std::fs::write(
            tmp.path().join("framework/rules/quality-cross.md"),
            VERDICT_RULES,
        )
        .unwrap();
        let feature = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&feature).unwrap();
        std::fs::write(
            feature.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# X\n\n## Goal\n\nText.\n",
        )
        .unwrap();
        tmp
    }

    fn with_feature(extra: DiscoverRuleFilesArgs) -> DiscoverRuleFilesArgs {
        DiscoverRuleFilesArgs {
            rules: true,
            all: true,
            feature: Some("001-x".into()),
            ..extra
        }
    }

    fn record(tmp: &TempDir, judged: &[&str], finding: Option<&str>) {
        let mut a = crate::schema::primitives::WriteAnalysisArgs {
            feature: "001-x".into(),
            analyzed_at: "2026-11-12T00:00:00Z".into(),
            analyzed_against: "abc".into(),
            judged_rules: judged.iter().map(|id| (*id).to_string()).collect(),
            ..crate::schema::primitives::WriteAnalysisArgs::default()
        };
        if let Some(id) = finding {
            a.blocking_findings = 1;
            a.rule_verdicts = std::collections::BTreeMap::from([(
                id.to_string(),
                crate::schema::primitives::RuleVerdict::Finding {
                    at: "spec.md#Goal".into(),
                    message: "no owner".into(),
                },
            )]);
        }
        crate::primitives::write_analysis::run(&a, tmp.path()).unwrap();
    }

    const ALL_JUDGED: &[&str] = &["TS-RULE-001", "TS-RULE-002", "TS-SHLD-001"];

    /// Spec 082, AC1 on the interactive path: with no record every assessable
    /// rule is to be judged, and with a record over unchanged subjects none
    /// is, the current finding coming back to report again, in its tier; an
    /// edited design subject judges every rule again, an edited `tasks.md`
    /// none.
    #[test]
    fn a_second_call_over_an_unchanged_spec_lists_no_rule_to_judge() {
        let tmp = verdict_project();
        let first = run(&with_feature(args(&[])), tmp.path()).unwrap();
        let block = first.verdicts.unwrap();
        assert_eq!(block.reason.as_deref(), Some("no-prior-verdicts"));
        assert_eq!(block.judge.unwrap(), ALL_JUDGED);
        assert_eq!(block.reusable.unwrap(), []);

        record(&tmp, ALL_JUDGED, Some("TS-RULE-001"));
        let second = run(&with_feature(args(&[])), tmp.path()).unwrap();
        let block = second.verdicts.unwrap();
        assert_eq!(block.reason, None);
        assert_eq!(block.judge_count, 0);
        assert_eq!(block.judge.unwrap(), [] as [String; 0]);
        let reusable = block.reusable.unwrap();
        assert_eq!(reusable.len(), 1);
        assert_eq!(reusable[0].id, "TS-RULE-001");
        assert_eq!(reusable[0].tier, "must");
        assert_eq!(
            reusable[0].verdict,
            crate::schema::primitives::RuleVerdict::Finding {
                at: "spec.md#Goal".into(),
                message: "no owner".into()
            }
        );
        assert_eq!(block.reusable_count, 1);

        std::fs::write(tmp.path().join("specs/001-x/tasks.md"), "# T\n\n- [x] a\n").unwrap();
        let ticked = run(&with_feature(args(&[])), tmp.path()).unwrap();
        assert_eq!(ticked.verdicts.unwrap().judge_count, 0);

        std::fs::write(tmp.path().join("specs/001-x/plan.md"), "# Plan\n").unwrap();
        let edited = run(&with_feature(args(&[])), tmp.path()).unwrap();
        let block = edited.verdicts.unwrap();
        assert_eq!(block.reason.as_deref(), Some("subjects-changed"));
        assert_eq!(block.judge.unwrap(), ALL_JUDGED);
    }

    /// A rule the prior run left unjudged is the only one listed to judge.
    #[test]
    fn a_rule_the_prior_run_left_unjudged_is_listed_to_judge() {
        let tmp = verdict_project();
        record(&tmp, &["TS-RULE-001", "TS-SHLD-001"], None);
        let result = run(&with_feature(args(&[])), tmp.path()).unwrap();
        assert_eq!(result.verdicts.unwrap().judge.unwrap(), ["TS-RULE-002"]);
    }

    /// A rule edit judges the file's rules again over an unchanged spec.
    #[test]
    fn an_edited_rule_file_is_listed_to_judge_again() {
        let tmp = verdict_project();
        record(&tmp, ALL_JUDGED, None);
        std::fs::write(
            tmp.path().join("framework/rules/quality-cross.md"),
            VERDICT_RULES.replace("Look for constants.", "Look for named constants."),
        )
        .unwrap();
        let result = run(&with_feature(args(&[])), tmp.path()).unwrap();
        assert_eq!(result.verdicts.unwrap().judge.unwrap(), ALL_JUDGED);
    }

    /// `feature` reads nothing without `all` and `rules`, which select the
    /// files an analysis records over: the block is absent, not empty, and a
    /// feature that does not exist is refused.
    #[test]
    fn a_feature_without_all_and_rules_adds_no_verdicts_and_a_missing_one_is_refused() {
        let tmp = verdict_project();
        for given in [
            DiscoverRuleFilesArgs {
                all: false,
                ..with_feature(args(&[]))
            },
            DiscoverRuleFilesArgs {
                rules: false,
                ..with_feature(args(&[]))
            },
        ] {
            assert_eq!(run(&given, tmp.path()).unwrap().verdicts, None);
        }
        let missing = DiscoverRuleFilesArgs {
            feature: Some("009-none".into()),
            ..with_feature(args(&[]))
        };
        assert!(matches!(
            run(&missing, tmp.path()).unwrap_err(),
            PrimitiveError::FeatureNotFound { .. }
        ));
    }

    /// A record that does not parse is named, and every rule is listed to
    /// judge: it is never read as holding no verdicts.
    #[test]
    fn an_unreadable_record_is_named_and_judges_every_rule() {
        let tmp = verdict_project();
        std::fs::write(
            tmp.path().join("specs/001-x/analysis.md"),
            "---\nlast-run: 2026-11-12T00:00:00Z\nrule-verdicts: [TS-RULE-001]\n---\n",
        )
        .unwrap();
        let result = run(&with_feature(args(&[])), tmp.path()).unwrap();
        assert!(
            result
                .notices
                .iter()
                .any(|notice| notice.contains("carries no readable record")),
            "{:?}",
            result.notices
        );
        let block = result.verdicts.unwrap();
        assert_eq!(block.reason.as_deref(), Some("record-unreadable"));
        assert_eq!(block.judge.unwrap(), ALL_JUDGED);
    }

    /// Past the read size the lists move to the file the result names, with
    /// their counts inline, `reusable` and `judge` beside `rule-files`.
    #[test]
    fn past_the_read_size_the_verdict_lists_move_to_the_file() {
        let tmp = verdict_project();
        record(&tmp, &["TS-RULE-002", "TS-SHLD-001"], None);
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 300\n",
        )
        .unwrap();
        let result = run(&with_feature(args(&[])), tmp.path()).unwrap();
        assert_eq!(result.form, Some(ListsForm::File));
        let block = result.verdicts.clone().unwrap();
        assert_eq!((block.reusable_count, block.judge_count), (0, 1));
        assert!(block.reusable.is_none() && block.judge.is_none());
        let file = result.lists_file.clone().unwrap();
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(&file)).unwrap())
                .unwrap();
        assert_eq!(lists["judge"], serde_json::json!(["TS-RULE-001"]));
        assert_eq!(lists["reusable"], serde_json::json!([]));
        assert!(lists["rule-files"].is_array());
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "discover-rule-files",
            &serde_json::to_value(&result).unwrap(),
            &lists,
        );
    }
}
