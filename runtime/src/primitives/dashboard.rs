//! `dashboard` — single-call pipeline-state data, which `render-status` renders as
//! `/{project}:status`'s view. What it
//! returns is `schema::descriptions::DASHBOARD`. The project config and the session file
//! resolve through [`crate::schema::paths`]'s three-tier `CONFIG_CHAIN` / `SESSION_CHAIN`
//! ladders (`.ductus/` → `.govern/` → legacy repo root, newest-wins) — host-agnostic,
//! project-name-agnostic, no caller-supplied path.
//!
//! The session target is a peek through [`crate::session`] (spec 062): an identified
//! process sees its own target, with its session label and the notices its next command
//! will deliver. The peek pins no adoption, refreshes no `used-at` and consumes no notice,
//! so the session is read only, as the specs and config are, the list file the
//! file form writes aside — `dashboard` is also called for
//! data, and a delivering read there would take the notices from the command that owns
//! them.
//!
//! The pipeline view's model is built from the same load by [`view_model`], which
//! `render-status` and the hook surface render, each held to the read size measured on
//! the result it serializes (spec 080): the view travels without the structured
//! payload, and the payload without the view.
//!
//! Defined by `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::time::SystemTime;

use serde::Deserialize;

use crate::host::Host;
use crate::primitives::resolve_references::{self, load_services};
use crate::primitives::{
    PrimitiveError, Result, ScenarioFrontmatter, list_feature_dirs, list_scenario_files,
    pipeline_view, read_spec, read_text, rel_path, section_lines, split_frontmatter,
};
use crate::schema::config_keys;
use crate::schema::paths;
use crate::schema::primitives::{
    DashboardArgs, DashboardConfig, DashboardResult, DashboardScenarioDetail,
    DashboardSessionTarget, DashboardSpec, Frontmatter, InboxStanding, ListsForm, ReferenceOutcome,
    ReopeningCounts, ResolutionRecord, ResolveReferencesArgs, ScenarioOpenQuestion, SessionNotice,
    SessionSource,
};
use crate::schema::services::Services;
use crate::schema::status::UNBLOCKING_STATUSES;
use crate::session::{self, Identity};

/// Execute the `dashboard` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::MissingSpecFile`] when an `NNN-feature`
/// directory under `specs/` lacks a `spec.md` (the directory naming
/// convention promises one), [`PrimitiveError::Io`] on filesystem
/// failures, [`PrimitiveError::Yaml`] when any spec's frontmatter is
/// malformed, [`PrimitiveError::Toml`] when the resolved project config
/// or session file is malformed, an invalid `DUCTUS_SESSION`, and what
/// `bounded_lists::place` returns: [`PrimitiveError::InvalidReadSize`] for a
/// malformed `[artifacts] read-size-bytes`, or a list file that cannot be
/// written. A *targeted scenario* with
/// missing or malformed frontmatter is NOT an error — it degrades to a
/// detail-less session target (see [`load_scenario_detail`]).
pub fn run(args: &DashboardArgs, repo: &Path) -> Result<DashboardResult> {
    let identity = session::process_identity()?;
    run_as(args, repo, identity.as_ref(), SystemTime::now())
}

/// The dashboard as `identity` at `now`: the one path [`run`] and the tests
/// share.
///
/// The result is inline while it fits `[artifacts] read-size-bytes`, and past
/// it carries `specs` and `tags-union` in the file it names and only their
/// counts inline (`bounded_lists::place`; spec 080, AC7). A non-empty
/// `session-notices` or `session-unreadable` moves with them, its count
/// inline, since nothing else bounds an unparseable session file's entry.
pub(crate) fn run_as(
    _args: &DashboardArgs,
    repo: &Path,
    identity: Option<&Identity>,
    now: SystemTime,
) -> Result<DashboardResult> {
    let mut result = load(repo, identity, now)?.into_result();
    let mut lists = serde_json::json!({
        "specs": &result.specs,
        "tags-union": &result.tags_union,
    });
    if !result.session_notices.is_empty() {
        lists["session-notices"] = serde_json::json!(&result.session_notices);
    }
    if !result.session_unreadable.is_empty() {
        lists["session-unreadable"] = serde_json::json!(&result.session_unreadable);
    }
    if let Some(file) = super::bounded_lists::place(
        repo,
        "dashboard",
        super::bounded_lists::ALL_SPECS,
        &result,
        &lists,
    )? {
        result.form = ListsForm::File;
        result.specs = None;
        result.tags_union = None;
        if !result.session_notices.is_empty() {
            result.session_notices_count = Some(result.session_notices.len());
            result.session_notices = Vec::new();
        }
        if !result.session_unreadable.is_empty() {
            result.session_unreadable_count = Some(result.session_unreadable.len());
            result.session_unreadable = Vec::new();
        }
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// The pipeline view's model as `identity` at `now`, with the
/// slash-command namespace its hints name. Each surface renders it held to
/// the read size, measured on the result that surface serializes.
///
/// # Errors
///
/// Every error [`run`] returns but those of `bounded_lists::place`, and the
/// errors resolving a spec's `references:` index returns for the readout.
pub(crate) fn view_model(
    repo: &Path,
    identity: Option<&Identity>,
    now: SystemTime,
) -> Result<(pipeline_view::Model, String)> {
    load(repo, identity, now)?.model(repo)
}

/// Everything one read of the pipeline state holds: the payload's fields, and
/// beside them what only the view shows.
struct Loaded {
    specs: Vec<DashboardSpec>,
    /// Each spec's unresolved scenario questions, as the scenario carrying
    /// each one, for the view, which lists every one.
    questions: pipeline_view::QuestionsBySpec,
    tags_union: Vec<String>,
    config: DashboardConfig,
    /// The repo-relative name the config read actually resolved, carried
    /// into the view so its provenance tag cannot name a different file than
    /// the one parsed.
    config_name: &'static str,
    session_target: Option<DashboardSessionTarget>,
    session: SessionView,
    inbox_standing: InboxStanding,
    reopenings: ReopeningCounts,
}

fn load(repo: &Path, identity: Option<&Identity>, now: SystemTime) -> Result<Loaded> {
    let (specs, questions) = load_specs(repo)?;
    let tags_union = compute_tags_union(&specs);
    let (config, config_name) = load_config(repo)?;
    let (session_target, session) = load_session_target(repo, identity, now)?;
    Ok(Loaded {
        tags_union,
        specs,
        questions,
        config,
        config_name,
        session_target,
        session,
        inbox_standing: super::inbox_standing::standing(repo)?,
        reopenings: super::measure_reopenings::counts(repo, now),
    })
}

impl Loaded {
    fn into_result(self) -> DashboardResult {
        DashboardResult {
            session_target: self.session_target,
            session_identity: self.session.identity,
            session_source: self.session.source,
            session_notices: self.session.notices,
            session_notices_count: None,
            session_unreadable: self.session.unreadable,
            session_unreadable_count: None,
            specs_count: self.specs.len(),
            specs: Some(self.specs),
            tags_union_count: self.tags_union.len(),
            tags_union: Some(self.tags_union),
            config: self.config,
            inbox_standing: self.inbox_standing,
            reopenings: self.reopenings,
            form: ListsForm::Inline,
            lists_file: None,
        }
    }

    /// Build the view's model, with the project's slash-command namespace.
    /// The references readout is omitted entirely when no spec declares
    /// references (a single-service adopter sees no change). Returned data,
    /// never stdout printing: user-facing rendering stays with the host
    /// (§runtime-boundary).
    fn model(&self, repo: &Path) -> Result<(pipeline_view::Model, String)> {
        let project = Host::load(repo)?.project;
        let model = pipeline_view::build(pipeline_view::ViewData {
            specs: &self.specs,
            questions: &self.questions,
            tags_union: &self.tags_union,
            config: &self.config,
            config_name: self.config_name,
            inbox: &self.inbox_standing,
            reopenings: &self.reopenings,
            session_target: self.session_target.as_ref(),
            session_label: self.session.identity.as_deref(),
            notices: &self.session.notices,
            unreadable: &self.session.unreadable,
            references: resolve_references_readout(repo, &self.specs, &project)?,
            project: &project,
        });
        // The namespace as the view's hints spell it, so a reader respelling
        // those hints, as Pi's bridge does, matches every one (`BE-INPUT-011`).
        Ok((model, pipeline_view::view_text(&project)))
    }
}

/// The cross-service references readout: each spec whose derived
/// `references:` index is non-empty, with one line per reference, resolved
/// through the same classification `resolve-references` exposes (composed
/// here so one call renders the whole view). Empty when no spec declares
/// references.
fn resolve_references_readout(
    repo: &Path,
    specs: &[DashboardSpec],
    project: &str,
) -> Result<Vec<(String, Vec<String>)>> {
    let services = load_services(repo)?;
    let mut sections = Vec::new();
    for spec in specs {
        let resolved = resolve_references::resolve(
            &ResolveReferencesArgs {
                feature: spec.slug.clone(),
            },
            repo,
        )?;
        if resolved.references.is_empty() {
            continue;
        }
        let lines = resolved
            .references
            .iter()
            .map(|record| reference_line(record, &services, project))
            .collect();
        sections.push((spec.slug.clone(), lines));
    }
    Ok(sections)
}

/// One readout line per resolution record, with the matched service's
/// `description` appended for orientation when present.
fn reference_line(record: &ResolutionRecord, services: &Services, project: &str) -> String {
    let name = match &record.service {
        Some(service) => format!("{service}/{}", record.spec),
        None => record.spec.clone(),
    };
    let body = match record.outcome {
        ReferenceOutcome::Ok => {
            format!("{name} → {}", record.status.as_deref().unwrap_or("unknown"))
        }
        ReferenceOutcome::Unregistered => format!(
            "{name} — status not attempted (unregistered; run /{project}:link to register the \
             service)"
        ),
        ReferenceOutcome::NotCheckedOut => format!("{name} → unknown (service not checked out)"),
        ReferenceOutcome::StatusUnreadable => format!("{name} → unknown (status unreadable)"),
        ReferenceOutcome::Broken => format!(
            "{name} → broken reference (target spec missing; also reported by /{project}:analyze)"
        ),
    };
    match record
        .service
        .as_deref()
        .and_then(|alias| services.0.get(alias))
        .and_then(|entry| entry.description.as_deref())
    {
        Some(description) if !description.is_empty() => format!("{body} — {description}"),
        _ => body,
    }
}

/// Walk `specs/` and build the per-spec entry list. Non-`NNN-feature`
/// directories are skipped (via the shared [`list_feature_dirs`]); a
/// present-but-missing `spec.md` still halts with a structured error from
/// [`load_one_spec`]. After the first pass, walks the list a second time
/// to fill in each spec's `blocked-by` (needs every spec's status known).
/// Beside the entries, each spec carrying unresolved scenario questions with
/// the scenario of each one, for the view, which lists every one.
fn load_specs(repo: &Path) -> Result<(Vec<DashboardSpec>, pipeline_view::QuestionsBySpec)> {
    let layout = paths::Paths::load(repo)?;
    let specs_dir = repo.join(&layout.specs_root);
    let mut entries: Vec<DashboardSpec> = Vec::new();
    let mut questions = pipeline_view::QuestionsBySpec::new();

    for slug in list_feature_dirs(&specs_dir) {
        let (entry, carried) = load_one_spec(&specs_dir, &layout.specs_root, &slug)?;
        if !carried.is_empty() {
            let scenarios = carried.into_iter().map(|q| q.scenario).collect();
            questions.push((slug.clone(), scenarios));
        }
        entries.push(entry);
    }

    let status_by_slug: HashMap<String, String> = entries
        .iter()
        .map(|s| (s.slug.clone(), s.status.clone()))
        .collect();
    for spec in &mut entries {
        // Resolution is against the corpus just walked, which is the only
        // tree this call can see. `true` therefore means "not here", never
        // "does not exist" — before the merge, the target normally lives on
        // the upstream branch.
        spec.fold_target_missing = spec
            .folds_into
            .as_ref()
            .is_some_and(|target| !status_by_slug.contains_key(target));
        spec.blocked_by = spec
            .dependencies
            .iter()
            .filter(|dep| {
                let status = status_by_slug.get(dep.as_str()).map_or("", String::as_str);
                !UNBLOCKING_STATUSES.contains(&status)
            })
            .cloned()
            .collect();
    }
    Ok((entries, questions))
}

/// Load one spec's dashboard entry, with its unresolved scenario questions.
/// `blocked_by` is filled in by the caller after every spec's status has
/// been read.
fn load_one_spec(
    specs_dir: &Path,
    root: &str,
    slug: &str,
) -> Result<(DashboardSpec, Vec<ScenarioOpenQuestion>)> {
    let feature_dir = specs_dir.join(slug);
    let spec_path = feature_dir.join("spec.md");
    if !spec_path.is_file() {
        return Err(PrimitiveError::MissingSpecFile {
            root: root.to_owned(),
            feature: slug.to_string(),
        });
    }
    let content = read_text(&spec_path)?;
    let (fm_text, body) = split_frontmatter(&content, &spec_path)?;
    let frontmatter: Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;
    let open_question_count = count_open_questions(body);
    let scenarios_dir = feature_dir.join("scenarios");
    // Scenario questions come from `read-spec`'s collector so the glance
    // view, the completion gate, and the analyze finding can never report
    // different counts for the same feature (spec 046).
    // The dashboard is a glance, not a full pretty-printer: an unreadable
    // scenario is reported by `read-spec` and by `check-artifacts`' skipped
    // list, which is where a caller goes for that distinction. Named here so
    // the discard is a decision rather than an oversight.
    let scenario_questions = read_spec::collect_scenario_open_questions(&feature_dir).questions;
    let scenarios_with_questions: Vec<String> = read_spec::scenario_names(&scenario_questions)
        .into_iter()
        .map(ToString::to_string)
        .collect();
    let entry = DashboardSpec {
        slug: slug.to_string(),
        status: frontmatter.status,
        dependencies: frontmatter.dependencies,
        tags: frontmatter.tags,
        open_question_count,
        has_plan: feature_dir.join("plan.md").is_file(),
        has_tasks: feature_dir.join("tasks.md").is_file(),
        has_data_model: feature_dir.join("data-model.md").is_file(),
        scenarios_count: count_scenario_files(&scenarios_dir),
        scenario_open_question_count: u32::try_from(scenario_questions.len()).unwrap_or(u32::MAX),
        scenarios_with_questions,
        blocked_by: Vec::new(),
        folds_into: frontmatter.folds_into,
        // Filled in by the caller, which needs every slug known first.
        fold_target_missing: false,
    };
    Ok((entry, scenario_questions))
}

/// Count `*.md` files directly under `scenarios_dir` via the shared
/// case-insensitive [`list_scenario_files`] — the same set `check-artifacts`
/// derives scenario slugs from. Returns 0 when the directory is absent or
/// unreadable.
fn count_scenario_files(scenarios_dir: &Path) -> u32 {
    u32::try_from(list_scenario_files(scenarios_dir).len()).unwrap_or(u32::MAX)
}

/// Count unresolved entries in a spec or scenario body's `## Open Questions`
/// section, with `read-spec`'s own parser ([`read_spec::parse_open_questions`])
/// rather than a second one, so `/{project}:status` and `/{project}:clarify`
/// never disagree about whether a spec has open questions: the count drives
/// the recovery Next Action, and a second reader is a second answer.
fn count_open_questions(body: &str) -> u32 {
    u32::try_from(read_spec::parse_open_questions(body, "Open Questions").len()).unwrap_or(u32::MAX)
}

/// Compute the sorted, deduplicated union of every spec's `tags` array.
fn compute_tags_union(specs: &[DashboardSpec]) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    for spec in specs {
        for tag in &spec.tags {
            set.insert(tag.clone());
        }
    }
    set.into_iter().collect()
}

/// Minimal TOML shape: just enough of the resolved project config to extract
/// the `[[review.disabled-rule-files]]` entries' `file` basenames. Unknown
/// keys are accepted; the primitive only reports what it knows.
#[derive(Deserialize, Default)]
struct DuctusConfig {
    #[serde(default)]
    review: Option<ReviewConfig>,
}

#[derive(Deserialize, Default)]
struct ReviewConfig {
    #[serde(default, rename = "disabled-rule-files")]
    disabled_rule_files: Vec<DisabledRuleFile>,
}

#[derive(Deserialize)]
struct DisabledRuleFile {
    file: String,
}

/// Read the resolved config file and summarize the review-state for the
/// dashboard, returning the repo-relative name resolution picked alongside
/// the summary. The name travels with the parsed content so the callout's
/// provenance tag names the file that was actually read.
fn load_config(repo: &Path) -> Result<(DashboardConfig, &'static str)> {
    let (toml_path, name) = paths::resolve_config(repo);
    if !toml_path.is_file() {
        return Ok((
            DashboardConfig {
                present: false,
                disabled_rule_files: Vec::new(),
                unknown_keys: Vec::new(),
            },
            name,
        ));
    }
    let content = read_text(&toml_path)?;
    let toml_error = |source| PrimitiveError::Toml {
        path: toml_path.clone(),
        source,
    };
    // The whole file's unknown keys, whichever command reads each section,
    // so `/status` names every one (spec 019). Key names only; the typed
    // parse below is what reports a value of the wrong type, by its line.
    let table: toml::Table = toml::from_str(&content).map_err(toml_error)?;
    let unknown_keys = config_keys::unknown_keys(&table)
        .iter()
        .map(ToString::to_string)
        .collect();
    let parsed: DuctusConfig = toml::from_str(&content).map_err(toml_error)?;
    let disabled_rule_files = parsed
        .review
        .map(|r| r.disabled_rule_files)
        .unwrap_or_default()
        .into_iter()
        .map(|d| d.file)
        .collect();
    Ok((
        DashboardConfig {
            present: true,
            disabled_rule_files,
            unknown_keys,
        },
        name,
    ))
}

/// The session facts beside the target: who is asking, where the target came
/// from, and what to tell them. All empty for an unidentified process, so its
/// payload and render are unchanged by spec 062.
#[derive(Debug, Default)]
struct SessionView {
    identity: Option<String>,
    source: Option<SessionSource>,
    notices: Vec<SessionNotice>,
    unreadable: Vec<String>,
}

/// Peek the session target through the session core and populate the
/// session-target field. When the targeted scenario file exists, also reads
/// it to populate `scenario-detail`. The target is echoed as-recorded;
/// `/{project}:target` is the corrective action for stale slugs, not the
/// dashboard.
fn load_session_target(
    repo: &Path,
    identity: Option<&Identity>,
    now: SystemTime,
) -> Result<(Option<DashboardSessionTarget>, SessionView)> {
    let resolution = session::peek(repo, identity, now)?;
    let view = SessionView {
        source: resolution.identity.as_ref().map(|_| resolution.source),
        identity: resolution.identity,
        notices: resolution.notices,
        unreadable: resolution
            .unreadable
            .iter()
            .map(|path| rel_path(path, repo))
            .collect(),
    };
    let Some(target) = resolution.target else {
        return Ok((None, view));
    };
    let scenario_detail = match (&target.scenario, &target.scenario_path) {
        (Some(_), Some(rel_path)) => load_scenario_detail(repo, rel_path)?,
        _ => None,
    };
    Ok((
        Some(DashboardSessionTarget {
            feature: target.feature,
            scenario: target.scenario,
            scenario_detail,
        }),
        view,
    ))
}

/// Read a scenario file and extract its dashboard header detail. Returns
/// `Ok(None)` when the file doesn't exist OR exists with missing/
/// malformed frontmatter — a parse failure degrades to a detail-less
/// target exactly like the stale-path case, so one bad scenario cannot
/// brick the whole `/ductus:status` render (scenario
/// primitive-robustness-hardening). The corrective surface for both is
/// the same: re-target or fix the scenario file.
fn load_scenario_detail(repo: &Path, rel_path: &str) -> Result<Option<DashboardScenarioDetail>> {
    let path = repo.join(rel_path);
    if !path.is_file() {
        return Ok(None);
    }
    let content = read_text(&path)?;
    let Ok((fm_text, body)) = split_frontmatter(&content, &path) else {
        return Ok(None);
    };
    let Ok(frontmatter) = serde_norway::from_str::<ScenarioFrontmatter>(fm_text) else {
        return Ok(None);
    };
    let section = frontmatter
        .section
        .or(frontmatter.spec_ref)
        .unwrap_or_default();
    let context_summary = context_summary(body);
    let open_question_count = count_open_questions(body);
    Ok(Some(DashboardScenarioDetail {
        section,
        context_summary,
        open_question_count,
    }))
}

/// First non-blank, non-HTML-comment line of the scenario body's
/// `## Context` section, trimmed. Empty string when the section is
/// absent or contains only blanks and comments. Walks the section with
/// `section_lines`, which skips no comment or fence: a comment is filtered
/// here by its opening line.
fn context_summary(body: &str) -> String {
    for line in section_lines(body, "Context") {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("<!--") {
            return trimmed.to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::primitives::{InboxState, ReopeningState, SessionNoticeKind, SessionTarget};
    use std::time::{Duration, UNIX_EPOCH};
    use tempfile::TempDir;

    /// The payload and the view one load renders, so a test reads both as it
    /// did before spec 080 moved the view to `render-status`.
    #[derive(Debug, PartialEq)]
    struct Viewed {
        result: DashboardResult,
        rendered_markdown: String,
    }

    impl std::ops::Deref for Viewed {
        type Target = DashboardResult;
        fn deref(&self) -> &DashboardResult {
            &self.result
        }
    }

    /// Shadows [`super::run`] so every existing test names its process as
    /// **unidentified** explicitly, rather than relying on the test binary
    /// never capturing the identity at startup.
    fn run(args: &DashboardArgs, repo: &Path) -> Result<Viewed> {
        run_as(args, repo, None, SystemTime::now())
    }

    /// Shadows [`super::run_as`]: the payload, and the view rendered from the
    /// same load.
    fn run_as(
        _args: &DashboardArgs,
        repo: &Path,
        identity: Option<&Identity>,
        now: SystemTime,
    ) -> Result<Viewed> {
        let loaded = load(repo, identity, now)?;
        let (model, _) = loaded.model(repo)?;
        let rendered_markdown = pipeline_view::whole(&model, pipeline_view::Format::Markdown);
        Ok(Viewed {
            result: loaded.into_result(),
            rendered_markdown,
        })
    }

    /// The payload carries no view, which is `render-status`'s now, and keeps
    /// the per-spec status `/{project}:review --all` enumerates from (AC13).
    #[test]
    fn the_payload_carries_no_view_and_keeps_each_specs_status() {
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: in-progress\ndependencies: []\n",
            "",
        );
        let result =
            super::run_as(&DashboardArgs::default(), tmp.path(), None, fixed_now()).unwrap();
        let value = serde_json::to_value(&result).unwrap();
        assert!(value.get("rendered-markdown").is_none(), "{value}");
        assert_eq!(value["specs"][0]["status"], "in-progress");
    }

    /// The payload is held to `[artifacts] read-size-bytes` (spec 080, AC7),
    /// so a malformed one halts it, as it halts every reader of the setting,
    /// rather than choosing a form.
    #[test]
    fn a_malformed_read_size_halts_the_payload() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "[artifacts]\nread-size-bytes = 0\n");
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: draft\ndependencies: []\n",
            "",
        );
        assert!(matches!(
            super::run_as(&DashboardArgs::default(), tmp.path(), None, fixed_now()),
            Err(PrimitiveError::InvalidReadSize { .. })
        ));
    }

    /// Past the read size a non-empty `session-unreadable` moves to the list
    /// file with the specs, its count inline, so the file form stays within
    /// the bound however many session files fail to parse: the expiry sweep
    /// leaves each one in place (spec 080, AC7; `BE-PAYLOAD-001`).
    #[test]
    fn the_file_form_moves_the_unreadable_session_files() {
        let tmp = TempDir::new().unwrap();
        write_spec(tmp.path(), "055-a", "status: planned\n", "");
        write_ductus_session(tmp.path(), "feature = \"055-a\"\npath = \"specs/055-a\"\n");
        let review = Identity::named("review").unwrap();
        session::resolve(tmp.path(), Some(&review), fixed_now()).unwrap();
        for n in 0..200 {
            std::fs::write(
                tmp.path()
                    .join(format!(".ductus/sessions/broken-{n:03}.toml")),
                "feature = [",
            )
            .unwrap();
        }
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 2000\n",
        )
        .unwrap();

        let result = super::run_as(
            &DashboardArgs::default(),
            tmp.path(),
            Some(&review),
            fixed_now(),
        )
        .unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert!(result.session_unreadable.is_empty(), "{result:?}");
        assert_eq!(result.session_unreadable_count, Some(200));
        let size = serde_json::to_vec(&result).unwrap().len();
        assert!(size <= 2000, "{size} bytes");
        let file: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(tmp.path().join(result.lists_file.as_deref().unwrap()))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(file["session-unreadable"].as_array().unwrap().len(), 200);
    }

    fn fixed_now() -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(1_790_683_200)
    }

    /// The `.ductus/` layout — the one per-process targets need. (The
    /// suite's `write_session_toml` writes the legacy root tier, which the
    /// session core rightly treats as unidentified.)
    fn write_ductus_session(repo: &Path, body: &str) {
        std::fs::create_dir_all(repo.join(".ductus")).unwrap();
        std::fs::write(repo.join(".ductus/session.toml"), body).unwrap();
    }

    #[test]
    fn an_identified_process_sees_its_label_and_its_notices() {
        let tmp = TempDir::new().unwrap();
        write_spec(tmp.path(), "055-a", "status: planned\n", "");
        write_ductus_session(tmp.path(), "feature = \"055-a\"\npath = \"specs/055-a\"\n");
        let review = Identity::named("review").unwrap();
        let result = run_as(
            &DashboardArgs::default(),
            tmp.path(),
            Some(&review),
            fixed_now(),
        )
        .unwrap();

        assert_eq!(result.session_identity.as_deref(), Some("review"));
        assert_eq!(result.session_source, Some(SessionSource::Adopted));
        assert_eq!(result.session_notices.len(), 1);
        assert_eq!(result.session_notices[0].kind, SessionNoticeKind::Adopted);
        let rendered = &result.rendered_markdown;
        assert!(
            rendered.starts_with("Target: 055-a / planned / next: ")
                && rendered.contains(
                    " — session review\nNotice: Session review has no target of its own and \
                     will adopt 055-a"
                ),
            "{rendered}"
        );

        let again = run_as(
            &DashboardArgs::default(),
            tmp.path(),
            Some(&review),
            fixed_now(),
        )
        .unwrap();
        assert_eq!(again, result, "a read pins and consumes nothing");
        assert!(
            !tmp.path().join(".ductus/sessions").exists(),
            "no own file created"
        );

        let next = session::resolve(tmp.path(), Some(&review), fixed_now()).unwrap();
        assert_eq!(next.source, SessionSource::Adopted);
        assert_eq!(next.notices.len(), 1);
        assert!(
            next.notices[0]
                .message
                .starts_with("Session review had no target of its own and adopted 055-a"),
            "{:?}",
            next.notices
        );
    }

    #[test]
    fn a_cleared_process_sees_the_pointer_and_why() {
        let tmp = TempDir::new().unwrap();
        write_ductus_session(
            tmp.path(),
            "feature = \"058-gone\"\npath = \"specs/058-gone\"\n",
        );
        let review = Identity::named("review").unwrap();
        session::resolve(tmp.path(), Some(&review), fixed_now()).unwrap();
        session::retarget(
            tmp.path(),
            None,
            "058-gone",
            None,
            session::RemovalCause::Consolidate,
            fixed_now(),
        )
        .unwrap();

        let result = run_as(
            &DashboardArgs::default(),
            tmp.path(),
            Some(&review),
            fixed_now(),
        )
        .unwrap();
        assert!(result.session_target.is_none());
        assert_eq!(result.session_source, Some(SessionSource::Cleared));
        let rendered = &result.rendered_markdown;
        let (first, rest) = rendered.split_once('\n').unwrap();
        assert_eq!(first, "No session target.", "{rendered}");
        assert!(
            rest.starts_with("Notice: Target 058-gone was removed by a consolidation"),
            "{rendered}"
        );

        let again = run_as(
            &DashboardArgs::default(),
            tmp.path(),
            Some(&review),
            fixed_now(),
        )
        .unwrap();
        assert_eq!(again, result, "the notice stays pending across views");
    }

    #[test]
    fn the_next_resolution_delivers_what_the_dashboard_showed() {
        let tmp = TempDir::new().unwrap();
        write_spec(tmp.path(), "055-a", "status: planned\n", "");
        write_ductus_session(
            tmp.path(),
            "feature = \"1234.1-b\"\npath = \"specs/1234.1-b\"\n",
        );
        let (x, y) = (Identity::named("x").unwrap(), Identity::named("y").unwrap());
        session::resolve(tmp.path(), Some(&x), fixed_now()).unwrap();
        session::resolve(tmp.path(), Some(&y), fixed_now()).unwrap();
        let upstream = SessionTarget {
            feature: "055-a".into(),
            path: "specs/055-a".into(),
            scenario: None,
            scenario_path: None,
        };
        session::retarget(
            tmp.path(),
            Some(&y),
            "1234.1-b",
            Some(&upstream),
            session::RemovalCause::Fold,
            fixed_now(),
        )
        .unwrap();
        let own = session::own_path(tmp.path(), &x);
        let before = std::fs::read_to_string(&own).unwrap();

        let shown = run_as(&DashboardArgs::default(), tmp.path(), Some(&x), fixed_now())
            .unwrap()
            .result
            .session_notices;
        let kinds: Vec<_> = shown.iter().map(|notice| notice.kind).collect();
        assert_eq!(
            kinds,
            [SessionNoticeKind::Folded, SessionNoticeKind::CoTarget],
            "{shown:?}"
        );
        assert_eq!(
            std::fs::read_to_string(&own).unwrap(),
            before,
            "no used-at refresh, no notice consumed"
        );

        let delivered = session::resolve(tmp.path(), Some(&x), fixed_now()).unwrap();
        assert_eq!(delivered.notices, shown);
        let after = run_as(&DashboardArgs::default(), tmp.path(), Some(&x), fixed_now()).unwrap();
        assert!(after.session_notices.is_empty(), "{after:?}");
    }

    #[test]
    fn a_session_file_that_does_not_parse_is_named_under_the_target() {
        let tmp = TempDir::new().unwrap();
        write_spec(tmp.path(), "055-a", "status: planned\n", "");
        write_ductus_session(tmp.path(), "feature = \"055-a\"\npath = \"specs/055-a\"\n");
        let review = Identity::named("review").unwrap();
        session::resolve(tmp.path(), Some(&review), fixed_now()).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/sessions/broken.toml"),
            "feature = [",
        )
        .unwrap();

        let result = run_as(
            &DashboardArgs::default(),
            tmp.path(),
            Some(&review),
            fixed_now(),
        )
        .unwrap();
        assert_eq!(result.session_unreadable, [".ductus/sessions/broken.toml"]);
        assert!(
            result
                .rendered_markdown
                .contains("\nUnreadable: .ductus/sessions/broken.toml — "),
            "{}",
            result.rendered_markdown
        );
    }

    #[test]
    fn an_unidentified_payload_carries_no_session_fields() {
        let tmp = TempDir::new().unwrap();
        write_session_toml(tmp.path(), "feature = \"055-a\"\npath = \"specs/055-a\"\n");
        let value =
            serde_json::to_value(run(&DashboardArgs::default(), tmp.path()).unwrap().result)
                .unwrap();
        for key in [
            "session-identity",
            "session-source",
            "session-notices",
            "session-unreadable",
        ] {
            assert!(value.get(key).is_none(), "{key} leaked into {value}");
        }
    }

    /// Write a minimal spec.md to `repo/specs/{slug}/spec.md` with the
    /// given frontmatter body plus a `## Open Questions` section that
    /// stays empty unless `open_questions` overrides it.
    fn write_spec(repo: &Path, slug: &str, frontmatter: &str, open_questions: &str) {
        let dir = repo.join("specs").join(slug);
        std::fs::create_dir_all(&dir).unwrap();
        let content =
            format!("---\n{frontmatter}---\n\n# {slug}\n\n## Open Questions\n\n{open_questions}\n");
        std::fs::write(dir.join("spec.md"), content).unwrap();
    }

    /// The dashboard resolves every spec's references through
    /// `resolve-references`, which checks its feature, so one spec directory
    /// holding a link that leaves the project refuses the whole view
    /// (scenario `a-path-argument-resolves-inside-its-base`).
    #[cfg(unix)]
    #[test]
    fn a_spec_directory_holding_an_outward_link_refuses_the_dashboard() {
        let tmp = TempDir::new().unwrap();
        write_spec(tmp.path(), "001-a", "status: draft\ndependencies: []\n", "");
        write_spec(tmp.path(), "002-b", "status: draft\ndependencies: []\n", "");
        let outside = TempDir::new().unwrap();
        std::fs::write(outside.path().join("notes.md"), "outside\n").unwrap();
        std::os::unix::fs::symlink(
            outside.path().join("notes.md"),
            tmp.path().join("specs/002-b/notes.md"),
        )
        .unwrap();
        let err = run(&DashboardArgs::default(), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                if reason.contains("notes.md")),
            "{err:?}"
        );
    }

    /// Write a legacy root session TOML at `<repo>/.govern.session.toml` —
    /// the fallback path the resolver reads when `.ductus/session.toml`
    /// is absent, so these tests double as legacy-fallback coverage.
    fn write_session_toml(repo: &Path, body: &str) {
        std::fs::write(repo.join(".govern.session.toml"), body).unwrap();
    }

    /// AC34: a spec declaring `folds-into` is reported as carrying an
    /// outstanding fold, and at `done` the row's Next Action is the fold
    /// rather than "complete", since a spec at `done` still owes the fold
    /// until fold-back retires it.
    #[test]
    fn a_declared_fold_is_never_reported_as_done() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "050-upstream",
            "status: done\ndependencies: []\n",
            "",
        );
        write_spec(
            tmp.path(),
            "1234.1-staged",
            "status: done\ndependencies: []\nfolds-into: 050-upstream\n",
            "",
        );

        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();

        let staged = result
            .specs
            .as_deref()
            .unwrap()
            .iter()
            .find(|s| s.slug == "1234.1-staged")
            .expect("the branch-scoped spec is enumerated");
        assert_eq!(staged.folds_into.as_deref(), Some("050-upstream"));
        assert!(!staged.fold_target_missing);

        let row = row_for(&result.rendered_markdown, "1234.1-staged");
        assert!(row.contains("done (fold pending)"), "{row}");
        assert!(row.contains("/ductus:fold → 050-upstream"), "{row}");
        assert!(!row.contains("done (spec is complete)"), "{row}");

        // The upstream spec is untouched: it declares no fold.
        let upstream = row_for(&result.rendered_markdown, "050-upstream");
        assert!(upstream.contains("done (spec is complete)"), "{upstream}");
        assert!(!upstream.contains("fold pending"), "{upstream}");
    }

    /// Done first, then fold (spec 051 scenario
    /// `a-pending-fold-never-holds-done`). Below `done` a staging spec shows
    /// the ordinary next action for its status, even with its target in this
    /// tree, and still carries the `(fold pending)` qualification.
    #[test]
    fn below_done_a_declared_fold_leaves_the_ordinary_next_action() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "050-upstream",
            "status: done\ndependencies: []\n",
            "",
        );
        for (slug, status, action) in [
            ("1234.1-drafted", "draft", "/ductus:clarify"),
            ("1234.2-clarified", "clarified", "/ductus:plan"),
            ("1234.3-planned", "planned", "/ductus:implement"),
            ("1234.4-underway", "in-progress", "/ductus:implement"),
        ] {
            write_spec(
                tmp.path(),
                slug,
                &format!("status: {status}\ndependencies: []\nfolds-into: 050-upstream\n"),
                "",
            );
            let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
            let row = row_for(&result.rendered_markdown, slug);
            assert!(row.contains(&format!("{status} (fold pending)")), "{row}");
            assert!(row.contains(action), "{row}");
            assert!(!row.contains("/ductus:fold"), "{row}");
        }
    }

    /// A target absent from this tree is called out on the row the fold is
    /// offered from. It is a report, not a verdict: before the merge the
    /// target normally lives on the upstream branch, and this view cannot
    /// tell which tree it is looking at.
    #[test]
    fn an_unresolvable_fold_target_is_called_out_on_the_row() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "1234.1-staged",
            "status: done\ndependencies: []\nfolds-into: 099-elsewhere\n",
            "",
        );

        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();

        assert!(result.specs.as_deref().unwrap()[0].fold_target_missing);
        let row = row_for(&result.rendered_markdown, "1234.1-staged");
        assert!(row.contains("/ductus:fold → 099-elsewhere"), "{row}");
        assert!(row.contains("(target not in this tree)"), "{row}");
        // Reported, never blocked: the callout says so in the operator's
        // terms rather than leaving the absence to be read as a defect.
        assert!(
            result
                .rendered_markdown
                .contains("normal state before the merge"),
            "{}",
            result.rendered_markdown
        );
    }

    /// The backlog is collected into one callout line so it can be read
    /// without scanning the table, in the same shape as the blocked and
    /// scenario-question callouts.
    #[test]
    fn outstanding_folds_are_collected_into_a_callout() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "050-upstream",
            "status: done\ndependencies: []\n",
            "",
        );
        write_spec(
            tmp.path(),
            "1234.1-staged",
            "status: in-progress\ndependencies: []\nfolds-into: 050-upstream\n",
            "",
        );
        write_spec(
            tmp.path(),
            "1234.2-also",
            "status: draft\ndependencies: []\nfolds-into: 099-elsewhere\n",
            "",
        );

        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();

        let callout = result
            .rendered_markdown
            .lines()
            .find(|l| l.contains("outstanding fold"))
            .expect("the pending-fold callout is rendered");
        assert!(
            callout.starts_with("Folds: 2 spec(s) with an outstanding fold — "),
            "{callout}"
        );
        assert!(
            callout.contains("1234.1-staged → 050-upstream"),
            "{callout}"
        );
        assert!(
            callout.contains("1234.2-also → 099-elsewhere (not in this tree)"),
            "{callout}"
        );
    }

    /// A corpus with no fold declared renders exactly as before — the
    /// callout is absent rather than rendered empty.
    #[test]
    fn a_corpus_without_folds_renders_no_fold_callout() {
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "050-upstream",
            "status: done\ndependencies: []\n",
            "",
        );

        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();

        assert!(!result.rendered_markdown.contains("outstanding fold"));
        assert!(!result.rendered_markdown.contains("fold pending"));
        assert_eq!(result.specs.as_deref().unwrap()[0].folds_into, None);
        assert!(!result.specs.as_deref().unwrap()[0].fold_target_missing);
    }

    /// An unresolved scenario question is the more upstream defect, so it
    /// keeps the Next Action cell even at `done`, where the fold would
    /// otherwise take it — the content gets settled before it gets moved.
    #[test]
    fn a_scenario_question_outranks_a_pending_fold_in_the_next_action() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "1234.1-staged",
            "status: done\ndependencies: []\nfolds-into: 050-upstream\n",
            "",
        );
        let scenarios = tmp.path().join("specs/1234.1-staged/scenarios");
        std::fs::create_dir_all(&scenarios).unwrap();
        std::fs::write(
            scenarios.join("a.md"),
            "---\nsection: X\n---\n\n# A\n\n## Open Questions\n\n- Which way?\n",
        )
        .unwrap();

        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();

        let row = row_for(&result.rendered_markdown, "1234.1-staged");
        assert!(row.contains("clarify (scenario)"), "{row}");
        assert!(!row.contains("/ductus:fold"), "{row}");
        // The fold is still reported — it is outstanding either way.
        assert!(row.contains("fold pending"), "{row}");
    }

    /// The rendered table row naming `slug`, for the assertions above.
    fn row_for(markdown: &str, slug: &str) -> String {
        markdown
            .lines()
            .find(|l| {
                l.starts_with(&format!("| {slug} |")) || l.starts_with(&format!("| **{slug}** |"))
            })
            .unwrap_or_else(|| panic!("no table row for {slug} in:\n{markdown}"))
            .to_string()
    }

    #[test]
    fn empty_specs_dir_returns_empty_list() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("specs")).unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.specs.as_deref().unwrap(), [] as [DashboardSpec; 0]);
        assert_eq!(result.tags_union.as_deref().unwrap(), [] as [String; 0]);
        assert!(!result.config.present);
        assert!(result.session_target.is_none());
    }

    #[test]
    fn missing_specs_dir_returns_empty_list() {
        let tmp = TempDir::new().unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.specs.as_deref().unwrap(), [] as [DashboardSpec; 0]);
    }

    #[test]
    fn happy_path_aggregates_inventory() {
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: done\ndependencies: []\ntags: [format, pipeline]\n",
            "*None — all resolved.*",
        );
        write_spec(
            tmp.path(),
            "002-beta",
            "status: planned\ndependencies: [001-alpha]\ntags: [pipeline]\n",
            "- What about edge case Z?\n- And case W?",
        );
        // Add a plan.md and tasks.md for 002-beta to exercise artifact
        // existence; leave 001-alpha bare.
        std::fs::write(tmp.path().join("specs/002-beta/plan.md"), "# plan").unwrap();
        std::fs::write(tmp.path().join("specs/002-beta/tasks.md"), "# tasks").unwrap();
        std::fs::create_dir_all(tmp.path().join("specs/002-beta/scenarios")).unwrap();
        std::fs::write(tmp.path().join("specs/002-beta/scenarios/x.md"), "# x").unwrap();
        std::fs::write(tmp.path().join("specs/002-beta/scenarios/y.md"), "# y").unwrap();
        std::fs::write(tmp.path().join("specs/002-beta/scenarios/README"), "# r").unwrap();

        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.specs_count, 2);
        assert_eq!(result.specs.as_deref().unwrap()[0].slug, "001-alpha");
        assert_eq!(result.specs.as_deref().unwrap()[1].slug, "002-beta");

        let alpha = &result.specs.as_deref().unwrap()[0];
        assert_eq!(alpha.status, "done");
        assert_eq!(alpha.dependencies, [] as [String; 0]);
        assert_eq!(alpha.tags, vec!["format", "pipeline"]);
        assert_eq!(alpha.open_question_count, 0);
        assert!(!alpha.has_plan);
        assert!(!alpha.has_tasks);
        assert!(!alpha.has_data_model);
        assert_eq!(alpha.scenarios_count, 0);
        assert_eq!(alpha.blocked_by, [] as [String; 0]);

        let beta = &result.specs.as_deref().unwrap()[1];
        assert_eq!(beta.status, "planned");
        assert_eq!(beta.dependencies, vec!["001-alpha"]);
        assert_eq!(beta.tags, vec!["pipeline"]);
        assert_eq!(beta.open_question_count, 2);
        assert!(beta.has_plan);
        assert!(beta.has_tasks);
        assert!(!beta.has_data_model);
        assert_eq!(beta.scenarios_count, 2);
        assert_eq!(beta.blocked_by, [] as [String; 0]);

        assert_eq!(
            result.tags_union.as_deref().unwrap(),
            vec!["format", "pipeline"]
        );
    }

    #[test]
    fn non_pattern_dirs_skip_silently() {
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "001-real",
            "status: draft\ndependencies: []\n",
            "*None — all resolved.*",
        );
        std::fs::create_dir_all(tmp.path().join("specs/templates")).unwrap();
        std::fs::create_dir_all(tmp.path().join("specs/.hidden")).unwrap();
        std::fs::write(tmp.path().join("specs/inbox.md"), "# inbox").unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.specs_count, 1);
        assert_eq!(result.specs.as_deref().unwrap()[0].slug, "001-real");
    }

    #[test]
    fn missing_spec_md_is_operational_error() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("specs/099-broken")).unwrap();
        let err = run(&DashboardArgs::default(), tmp.path()).unwrap_err();
        match err {
            PrimitiveError::MissingSpecFile { feature, root } => {
                assert_eq!(feature, "099-broken");
                assert_eq!(root, "specs");
            }
            other => panic!("expected MissingSpecFile, got {other:?}"),
        }
    }

    #[test]
    fn blocked_by_computes_from_dependency_status() {
        let tmp = TempDir::new().unwrap();
        // 001 is draft → blocks anything depending on it.
        write_spec(
            tmp.path(),
            "001-draft",
            "status: draft\ndependencies: []\n",
            "*None — all resolved.*",
        );
        // 002 is clarified → unblocks.
        write_spec(
            tmp.path(),
            "002-ready",
            "status: clarified\ndependencies: []\n",
            "*None — all resolved.*",
        );
        // 003 depends on both: 001 blocks it, 002 doesn't.
        write_spec(
            tmp.path(),
            "003-dep",
            "status: planned\ndependencies: [001-draft, 002-ready]\n",
            "*None — all resolved.*",
        );
        // 004 names a nonexistent dep — treated as blocking (empty status).
        write_spec(
            tmp.path(),
            "004-ghost",
            "status: planned\ndependencies: [999-missing]\n",
            "*None — all resolved.*",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let by_slug: HashMap<_, _> = result
            .specs
            .as_deref()
            .unwrap()
            .iter()
            .map(|s| (s.slug.as_str(), s))
            .collect();
        assert_eq!(by_slug["001-draft"].blocked_by, [] as [String; 0]);
        assert_eq!(by_slug["002-ready"].blocked_by, [] as [String; 0]);
        assert_eq!(by_slug["003-dep"].blocked_by, vec!["001-draft"]);
        assert_eq!(by_slug["004-ghost"].blocked_by, vec!["999-missing"]);
    }

    #[test]
    fn ductus_toml_absent_returns_present_false() {
        let tmp = TempDir::new().unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert!(!result.config.present);
        assert_eq!(result.config.disabled_rule_files, [] as [String; 0]);
    }

    #[test]
    fn ductus_toml_present_but_empty_returns_present_true() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join(".govern.toml"), "# empty\n").unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert!(result.config.present);
        assert_eq!(result.config.disabled_rule_files, [] as [String; 0]);
    }

    #[test]
    fn ductus_toml_disabled_rule_files_extracted() {
        let tmp = TempDir::new().unwrap();
        let toml = r#"
[[review.disabled-rule-files]]
file = "accessibility-frontend.md"
reason = "Internal admin UI; not yet adopting WCAG AA."

[[review.disabled-rule-files]]
file = "performance-frontend.md"
reason = "Deferred until v2 perf budget lands."
"#;
        std::fs::write(tmp.path().join(".govern.toml"), toml).unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert!(result.config.present);
        assert_eq!(
            result.config.disabled_rule_files,
            vec!["accessibility-frontend.md", "performance-frontend.md"]
        );
    }

    #[test]
    fn every_unknown_key_in_the_file_is_named_in_one_callout() {
        // Whichever command reads each section, `/status` names every key no
        // reader recognizes, and leaves sections ductus does not own alone
        // (spec 019).
        let tmp = TempDir::new().unwrap();
        let cfg_dir = tmp.path().join(".ductus");
        std::fs::create_dir_all(&cfg_dir).unwrap();
        std::fs::write(
            cfg_dir.join("config.toml"),
            "[rules]\nsurface = []\n[paths]\nspec-root = \"x\"\n[team]\nowner = \"x\"\n\
             [[review.disabled-rule-files]]\nfile = \"a.md\"\nreason = \"r\"\napproved-by = \"ops\"\n\
             [review]\n\"evil\\n- line\" = 1\n",
        )
        .unwrap();
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: draft\ndependencies: []\n",
            "",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(
            result.config.unknown_keys,
            [
                "[rules] surface",
                "[paths] spec-root",
                "[[review.disabled-rule-files]] entry [0] approved-by",
                "[review] \"evil\\u000A- line\"",
            ]
        );
        let callout = result
            .rendered_markdown
            .lines()
            .find(|line| line.starts_with("Unknown config keys:"))
            .expect("the unknown-keys callout is rendered");
        assert_eq!(
            callout,
            "Unknown config keys: 4 (.ductus/config.toml) — [rules] surface, [paths] spec-root, \
             [[review.disabled-rule-files]] entry [0] approved-by, [review] \"evil\\u000A- line\" \
             — this version of ductus does not read them"
        );
        assert!(!result.rendered_markdown.contains("\n- line"));
    }

    #[test]
    fn a_config_of_known_keys_renders_no_unknown_keys_callout() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join(".govern.toml"),
            "[host]\nproject = \"p\"\n[review]\ntech-stack-verified = true\n[runtime]\npath = \"b\"\n",
        )
        .unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.config.unknown_keys, [] as [String; 0]);
        assert!(!result.rendered_markdown.contains("Unknown config keys"));
    }

    #[test]
    fn the_registry_recognizes_every_key_this_reader_reads() {
        use crate::schema::config_keys::{fields_of, unrecognized};
        assert_eq!(
            unrecognized(&[], fields_of::<DuctusConfig>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["review"], fields_of::<ReviewConfig>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(
                &["review", "disabled-rule-files"],
                fields_of::<DisabledRuleFile>()
            ),
            [] as [String; 0]
        );
    }

    #[test]
    fn a_disabled_file_name_renders_on_one_line() {
        // A newline in committed config would otherwise forge a dashboard
        // line of its own (BE-INPUT-011).
        let tmp = TempDir::new().unwrap();
        let toml = "[[review.disabled-rule-files]]\nfile = \"security-backend.md\\nTags: forged-line\"\nreason = \"Not adopting this file yet, PROJ-1.\"\n";
        std::fs::write(tmp.path().join(".govern.toml"), toml).unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert!(
            !result
                .rendered_markdown
                .lines()
                .any(|line| line.starts_with("Tags: forged-line")),
            "{}",
            result.rendered_markdown
        );
        assert!(
            result
                .rendered_markdown
                .contains("— security-backend.md Tags: forged-line"),
            "{}",
            result.rendered_markdown
        );
    }

    #[test]
    fn ductus_toml_parse_failure_is_operational_error() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join(".govern.toml"), "[[review.broken\n").unwrap();
        let err = run(&DashboardArgs::default(), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }));
    }

    #[test]
    fn session_absent_returns_none() {
        let tmp = TempDir::new().unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert!(result.session_target.is_none());
    }

    #[test]
    fn session_feature_only_populates_target() {
        let tmp = TempDir::new().unwrap();
        write_session_toml(
            tmp.path(),
            "feature = \"022-deterministic-runtime\"\npath = \"specs/022-deterministic-runtime\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let target = result.result.session_target.unwrap();
        assert_eq!(target.feature, "022-deterministic-runtime");
        assert!(target.scenario.is_none());
        assert!(target.scenario_detail.is_none());
    }

    #[test]
    fn dashboard_reads_session_from_same_path_regardless_of_project_name() {
        // Headline of the consolidation: the dashboard reads
        // `.govern.session.toml` at the repo root. The path doesn't depend
        // on project name (`gov` vs `acme`) or AI CLI (`.claude/` vs
        // `.augment/`). The legacy `.claude/{project}-session.json` files
        // are not consulted — adopters migrate via /ductus.
        let tmp = TempDir::new().unwrap();
        write_session_toml(
            tmp.path(),
            "feature = \"002-observability\"\npath = \"specs/002-observability\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        // No `.claude/` or `.augment/` directory needs to exist.
        assert!(!tmp.path().join(".claude").exists());
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let target = result.result.session_target.unwrap();
        assert_eq!(target.feature, "002-observability");
    }

    #[test]
    fn legacy_json_session_file_is_ignored() {
        // An adopter who hasn't yet run /ductus post-consolidation may
        // still have `.claude/gov-session.json` on disk. The dashboard
        // does not read it — only `.govern.session.toml`. Adopters in
        // this state see "no target" until they re-/ductus:target or
        // /ductus migrates them.
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude")).unwrap();
        std::fs::write(
            tmp.path().join(".claude/gov-session.json"),
            r#"{"feature":"legacy-feature","path":"specs/legacy"}"#,
        )
        .unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert!(result.session_target.is_none());
    }

    #[test]
    fn malformed_session_toml_is_operational_error() {
        let tmp = TempDir::new().unwrap();
        write_session_toml(tmp.path(), "feature = [unclosed array\n");
        let err = run(&DashboardArgs::default(), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }));
    }

    #[test]
    fn session_with_scenario_populates_detail() {
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "022-foo",
            "status: in-progress\ndependencies: []\n",
            "*None — all resolved.*",
        );
        let scenarios = tmp.path().join("specs/022-foo/scenarios");
        std::fs::create_dir_all(&scenarios).unwrap();
        std::fs::write(
            scenarios.join("widget.md"),
            "---\nsection: \"Follow-on scenarios\"\n---\n\n# Widget\n\n## Context\n\nWidget exists to demonstrate gizmos.\n\n## Open Questions\n\n- One unresolved item.\n",
        )
        .unwrap();
        write_session_toml(
            tmp.path(),
            "feature = \"022-foo\"\npath = \"specs/022-foo\"\nscenario = \"widget\"\nscenario-path = \"specs/022-foo/scenarios/widget.md\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let target = result.result.session_target.unwrap();
        assert_eq!(target.feature, "022-foo");
        assert_eq!(target.scenario.as_deref(), Some("widget"));
        let detail = target.scenario_detail.unwrap();
        assert_eq!(detail.section, "Follow-on scenarios");
        assert_eq!(
            detail.context_summary,
            "Widget exists to demonstrate gizmos."
        );
        assert_eq!(detail.open_question_count, 1);
    }

    #[test]
    fn session_with_stale_scenario_returns_target_without_detail() {
        let tmp = TempDir::new().unwrap();
        write_session_toml(
            tmp.path(),
            "feature = \"022-foo\"\npath = \"specs/022-foo\"\nscenario = \"ghost\"\nscenario-path = \"specs/022-foo/scenarios/ghost.md\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let target = result.result.session_target.unwrap();
        assert_eq!(target.feature, "022-foo");
        assert_eq!(target.scenario.as_deref(), Some("ghost"));
        assert!(target.scenario_detail.is_none());
    }

    #[test]
    fn open_question_count_ignores_continuation_lines() {
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "001-x",
            "status: draft\ndependencies: []\n",
            "- First question with\n  a continuation line.\n- Second question.",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.specs.as_deref().unwrap()[0].open_question_count, 2);
    }

    #[test]
    fn open_question_count_is_read_specs_count() {
        // Numbered entries, a nested option folding into its question, and an
        // example inside a comment: the dashboard counts exactly what
        // read-spec reports, so /status and /clarify agree.
        let tmp = TempDir::new().unwrap();
        // The old `-`-only counter read this as 2: the nested option and the
        // commented example.
        let questions = "1. Which store?\n   - option a\n2. Which cache?\n3. Which queue?\n\n<!--\n- An example, not asked\n-->";
        write_spec(
            tmp.path(),
            "001-x",
            "status: draft\ndependencies: []\n",
            questions,
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.specs.as_deref().unwrap()[0].open_question_count, 3);
        let body = format!("## Open Questions\n\n{questions}\n");
        assert_eq!(
            read_spec::parse_open_questions(&body, "Open Questions").len(),
            3,
            "the two readers agree"
        );
    }

    #[test]
    fn session_with_malformed_scenario_frontmatter_degrades_to_detail_less_target() {
        // Scenario primitive-robustness-hardening: a targeted scenario
        // whose frontmatter fails YAML parse must degrade exactly like
        // the missing-file case — target surfaced, no detail — instead
        // of erroring the whole dashboard (one bad scenario bricking
        // /ductus:status).
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "022-foo",
            "status: in-progress\ndependencies: []\n",
            "*None — all resolved.*",
        );
        let scenarios = tmp.path().join("specs/022-foo/scenarios");
        std::fs::create_dir_all(&scenarios).unwrap();
        std::fs::write(
            scenarios.join("broken.md"),
            "---\nsection: [unclosed\n---\n\n# Broken\n\n## Context\n\nStill here.\n",
        )
        .unwrap();
        write_session_toml(
            tmp.path(),
            "feature = \"022-foo\"\npath = \"specs/022-foo\"\nscenario = \"broken\"\nscenario-path = \"specs/022-foo/scenarios/broken.md\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let target = result.result.session_target.unwrap();
        assert_eq!(target.feature, "022-foo");
        assert_eq!(target.scenario.as_deref(), Some("broken"));
        assert!(
            target.scenario_detail.is_none(),
            "malformed frontmatter must degrade, not error"
        );
    }

    #[test]
    fn session_with_scenario_missing_frontmatter_degrades_to_detail_less_target() {
        // No `---` fence at all — the split itself fails. Same
        // degradation as the parse-failure case.
        let tmp = TempDir::new().unwrap();
        write_spec(
            tmp.path(),
            "022-foo",
            "status: in-progress\ndependencies: []\n",
            "*None — all resolved.*",
        );
        let scenarios = tmp.path().join("specs/022-foo/scenarios");
        std::fs::create_dir_all(&scenarios).unwrap();
        std::fs::write(
            scenarios.join("bare.md"),
            "# Bare\n\n## Context\n\nNo frontmatter here.\n",
        )
        .unwrap();
        write_session_toml(
            tmp.path(),
            "feature = \"022-foo\"\npath = \"specs/022-foo\"\nscenario = \"bare\"\nscenario-path = \"specs/022-foo/scenarios/bare.md\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let target = result.result.session_target.unwrap();
        assert_eq!(target.scenario.as_deref(), Some("bare"));
        assert!(target.scenario_detail.is_none());
    }

    /// Pin the slash-command namespace so rendered `/{project}:…` texts
    /// are deterministic (the default falls back to the tempdir's random
    /// basename).
    fn pin_project(repo: &Path, extra: &str) {
        std::fs::write(
            repo.join(".govern.toml"),
            format!("[host]\nproject = \"ductus\"\n{extra}"),
        )
        .unwrap();
    }

    #[test]
    fn rendered_no_target_preamble_table_and_counts() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: draft\ndependencies: []\n",
            "- Undecided thing?",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let rendered = &result.rendered_markdown;
        assert!(rendered.starts_with("No session target.\n\n"), "{rendered}");
        assert!(
            rendered.ends_with("\n\nRun /ductus:target <feature> to select one."),
            "the hint to set a target closes the view: {rendered}"
        );
        assert!(
            rendered.contains(
                "| Feature | Status | Plan | Tasks | Data-model | Scenarios | Dependencies | Next Action |"
            ),
            "{rendered}"
        );
        assert!(
            rendered.contains("| 001-alpha | draft | — | — | — | 0 | — | /ductus:clarify |"),
            "{rendered}"
        );
        assert!(rendered.contains("Counts: draft 1"), "{rendered}");
        assert!(
            !rendered.contains("References:"),
            "readout omitted with no references: {rendered}"
        );
    }

    #[test]
    fn rendered_target_bold_row_recovery_blocked_and_config_callouts() {
        let tmp = TempDir::new().unwrap();
        pin_project(
            tmp.path(),
            "\n[[review.disabled-rule-files]]\nfile = \"security-backend.md\"\nreason = \"n/a\"\n",
        );
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: draft\ndependencies: []\n",
            "*None — all resolved.*",
        );
        // planned + open questions → recovery override; depends on the
        // draft spec → blocked.
        write_spec(
            tmp.path(),
            "002-beta",
            "status: planned\ndependencies: [001-alpha]\ntags: [pipeline]\n",
            "- What about Z?",
        );
        write_session_toml(
            tmp.path(),
            "feature = \"002-beta\"\npath = \"specs/002-beta\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let rendered = &result.rendered_markdown;
        assert!(
            rendered.starts_with("Target: 002-beta / planned / next: clarify (recovery)"),
            "{rendered}"
        );
        assert!(
            rendered.contains("| **002-beta** | planned |"),
            "target row bolded: {rendered}"
        );
        assert!(rendered.contains("| 001-alpha | draft |"), "{rendered}");
        assert!(
            rendered.contains("| 001 | clarify (recovery) |"),
            "NNN dep prefix + recovery next action: {rendered}"
        );
        assert!(
            rendered.contains("Counts: draft 1 · planned 1"),
            "{rendered}"
        );
        assert!(
            rendered.contains("Blocked: 1 spec(s) — 002-beta"),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "Recovery: 1 spec(s) past clarification with open questions again — 002-beta. Run /ductus:clarify on each; the spec reverts to draft and advances forward again."
            ),
            "{rendered}"
        );
        assert!(rendered.contains("\nTags: pipeline\n"), "{rendered}");
        assert!(
            rendered.contains("Disabled rule files: 1 (.govern.toml) — security-backend.md"),
            "{rendered}"
        );
    }

    #[test]
    fn rendered_config_callout_names_new_layout_config() {
        // Same callout, config under `.ductus/config.toml` — the
        // provenance tag names the resolved file, not a legacy literal.
        let tmp = TempDir::new().unwrap();
        let cfg_dir = tmp.path().join(".ductus");
        std::fs::create_dir_all(&cfg_dir).unwrap();
        std::fs::write(
            cfg_dir.join("config.toml"),
            "[host]\nproject = \"ductus\"\n\n[[review.disabled-rule-files]]\nfile = \"security-backend.md\"\nreason = \"n/a\"\n",
        )
        .unwrap();
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: draft\ndependencies: []\n",
            "",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert!(
            result
                .rendered_markdown
                .contains("Disabled rule files: 1 (.ductus/config.toml) — security-backend.md"),
            "{}",
            result.rendered_markdown
        );
    }

    #[test]
    fn rendered_scenario_line_and_next_action_override() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "022-foo",
            "status: done\ndependencies: []\n",
            "*None — all resolved.*",
        );
        let scenarios = tmp.path().join("specs/022-foo/scenarios");
        std::fs::create_dir_all(&scenarios).unwrap();
        std::fs::write(
            scenarios.join("edge.md"),
            "---\nsection: \"Core\"\n---\n\n# Edge\n\n## Context\n\nA thing.\n\n## Open Questions\n\n- Unresolved?\n",
        )
        .unwrap();
        write_session_toml(
            tmp.path(),
            "feature = \"022-foo\"\npath = \"specs/022-foo\"\nscenario = \"edge\"\nscenario-path = \"specs/022-foo/scenarios/edge.md\"\nset-at = \"2026-05-23T12:34:56Z\"\n",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let rendered = &result.rendered_markdown;
        assert!(
            rendered.starts_with(
                "Target: 022-foo / done / next: /ductus:clarify (scenario-targeted)\nScenario: edge (Core) — open-questions: 1"
            ),
            "scenario with open questions owns the next action: {rendered}"
        );
    }

    #[test]
    fn rendered_references_readout_covers_outcomes_and_descriptions() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join(".govern.toml"),
            "[host]\nproject = \"ductus\"\n\n[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"checkouts/api\"\ndescription = \"Main API service\"\n",
        )
        .unwrap();
        // Linked checkout with one resolvable spec.
        let linked = tmp.path().join("checkouts/api/specs/003-user");
        std::fs::create_dir_all(&linked).unwrap();
        std::fs::write(
            linked.join("spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# 003\n",
        )
        .unwrap();
        write_spec(
            tmp.path(),
            "001-consumer",
            "status: in-progress\ndependencies: []\nreferences:\n  - service: api\n    spec: 003-user\n  - spec: 007-nav\n  - service: api\n    spec: 999-gone\n",
            "*None — all resolved.*",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        let rendered = &result.rendered_markdown;
        assert!(rendered.contains("References:"), "{rendered}");
        assert!(rendered.contains("**001-consumer**"), "{rendered}");
        assert!(
            rendered.contains("- api/003-user → done — Main API service"),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "- 007-nav — status not attempted (unregistered; run /ductus:link to register the service)"
            ),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "- api/999-gone → broken reference (target spec missing; also reported by /ductus:analyze) — Main API service"
            ),
            "{rendered}"
        );
    }

    // --- scenario open questions (spec 046) -----------------------------------

    /// Seed a scenario under `slug` carrying `questions` verbatim inside
    /// its `## Open Questions` section.
    fn write_scenario(repo: &Path, slug: &str, name: &str, questions: &str) {
        let dir = repo.join("specs").join(slug).join("scenarios");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join(name),
            format!("---\nsection: Behavior\n---\n\n## Open Questions\n\n{questions}\n"),
        )
        .unwrap();
    }

    const PLANNED_FM: &str = "status: planned\ndependencies: []\n";

    #[test]
    fn scenarios_cell_gains_a_suffix_only_when_questions_exist() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "001-quiet", PLANNED_FM, "");
        write_scenario(
            tmp.path(),
            "001-quiet",
            "settled.md",
            "*None — all resolved.*",
        );
        write_spec(tmp.path(), "002-asking", PLANNED_FM, "");
        write_scenario(
            tmp.path(),
            "002-asking",
            "wire.md",
            "- Which operator?\n- Which encoding?",
        );

        let result = run(&DashboardArgs {}, tmp.path()).unwrap();
        let rendered = result.rendered_markdown;
        // Unchanged cell for the quiet spec: one scenario, no suffix.
        assert!(
            rendered.contains("| 001-quiet | planned | — | — | — | 1 |"),
            "{rendered}"
        );
        // Suffixed cell, and the Next Action overridden.
        assert!(
            rendered.contains("| 002-asking | planned | — | — | — | 1 (2 open) |"),
            "{rendered}"
        );
        assert!(rendered.contains("clarify (scenario)"), "{rendered}");
    }

    #[test]
    fn the_summary_lists_every_scenario_question_beside_its_spec() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "002-asking", PLANNED_FM, "");
        write_scenario(tmp.path(), "002-asking", "alpha.md", "- One?");
        write_scenario(tmp.path(), "002-asking", "beta.md", "- Two?");

        let rendered = run(&DashboardArgs {}, tmp.path())
            .unwrap()
            .rendered_markdown;
        let lines: Vec<&str> = rendered.lines().collect();
        assert!(
            lines
                .iter()
                .any(|l| l.starts_with("Scenario questions: 2 in 1 spec(s);")),
            "{rendered}"
        );
        assert!(lines.contains(&"  002-asking · alpha"), "{rendered}");
        assert!(lines.contains(&"  002-asking · beta"), "{rendered}");
    }

    #[test]
    fn recovery_wins_the_cell_but_both_callouts_render() {
        let tmp = tempfile::tempdir().unwrap();
        // Spec-body questions at `planned` (recovery) AND a scenario
        // question: the cell is exclusive, the callouts are not.
        write_spec(tmp.path(), "003-both", PLANNED_FM, "- A body question?");
        write_scenario(tmp.path(), "003-both", "wire.md", "- A scenario question?");

        let rendered = run(&DashboardArgs {}, tmp.path())
            .unwrap()
            .rendered_markdown;
        assert!(rendered.contains("clarify (recovery)"), "{rendered}");
        assert!(
            !rendered.contains("clarify (scenario)"),
            "the cell is exclusive: {rendered}"
        );
        assert!(rendered.contains("\nRecovery: "), "{rendered}");
        assert!(
            rendered.contains("\nScenario questions: "),
            "both callouts render: {rendered}"
        );
    }

    #[test]
    fn a_spec_without_scenario_questions_renders_exactly_as_before() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "001-quiet", PLANNED_FM, "");
        // Pin the command namespace: it otherwise defaults to the
        // tempdir's random basename.
        std::fs::write(
            tmp.path().join(".govern.toml"),
            "[host]\nproject = \"ductus\"\n",
        )
        .unwrap();
        let rendered = run(&DashboardArgs {}, tmp.path())
            .unwrap()
            .rendered_markdown;
        assert!(
            rendered.contains("| 001-quiet | planned | — | — | — | 0 |"),
            "{rendered}"
        );
        assert!(rendered.contains("/ductus:implement"), "{rendered}");
        assert!(
            !rendered.contains("Scenario questions:"),
            "no callout when nothing is outstanding: {rendered}"
        );
    }

    // -- the reopenings row (spec 079) ------------------------------------------

    /// Never omitted, and last but the inbox, which keeps its place at the end. With
    /// no repository the state says the measurement did not run.
    #[test]
    fn the_reopenings_row_renders_on_every_run_just_before_the_inbox_row() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "001-alpha",
            "status: draft\ndependencies: []\n",
            "",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.reopenings.state, ReopeningState::NotMeasured);
        let rendered = &result.rendered_markdown;
        let lines: Vec<&str> = rendered.lines().collect();
        let row = lines
            .iter()
            .position(|line| line.starts_with("Reopenings: ? not measured — "))
            .unwrap_or_else(|| panic!("{rendered}"));
        assert!(lines[row + 1].starts_with("Inbox:"), "{rendered}");
    }

    /// The block and the line are the measurement's counts at `now`.
    #[test]
    fn the_reopenings_block_carries_the_measurement() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let spec = tmp.path().join("specs/001-alpha/spec.md");
        for status in ["in-progress", "done"] {
            crate::primitives::git_fixture::write(
                &spec,
                &format!("---\nstatus: {status}\ndependencies: []\n---\n\n# A\n"),
            );
            crate::primitives::git_fixture::commit_all(&repository, "work(001): step");
        }
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.reopenings.state, ReopeningState::Measured);
        assert_eq!(result.reopenings.closed_count, 1);
        assert_eq!(result.reopenings.not_counted_count, 1);
        assert!(
            result.rendered_markdown.contains("1 closed, 0 over"),
            "{}",
            result.rendered_markdown
        );
    }

    // -- the standing inbox row (spec 058) --------------------------------------

    fn inbox_line(rendered: &str) -> Option<&str> {
        rendered.lines().find(|line| line.starts_with("Inbox:"))
    }

    #[test]
    fn the_inbox_row_renders_even_when_there_is_no_inbox() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        write_spec(
            tmp.path(),
            "001-x",
            "status: draft\ndependencies: []\n",
            "*None.*",
        );
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.inbox_standing.state, InboxState::NoFile);
        assert_eq!(
            inbox_line(&result.rendered_markdown),
            Some("Inbox: ? no readable specs/inbox.md — nothing examined"),
            "an absent inbox is its own state, never clean:\n{}",
            result.rendered_markdown
        );
    }

    #[test]
    fn a_clean_inbox_renders_a_clean_row_rather_than_nothing() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        std::fs::create_dir_all(tmp.path().join("specs")).unwrap();
        std::fs::write(
            tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n<!-- Rules:\n     - guidance, not an item -->\n",
        )
        .unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(
            inbox_line(&result.rendered_markdown),
            Some("Inbox: ✓ clean")
        );
    }

    #[test]
    fn an_outstanding_inbox_without_history_reports_its_age_as_undeterminable() {
        let tmp = TempDir::new().unwrap();
        pin_project(tmp.path(), "");
        std::fs::create_dir_all(tmp.path().join("specs")).unwrap();
        std::fs::write(
            tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n- [ ] one todo\n- [ ] and another\n",
        )
        .unwrap();
        let result = run(&DashboardArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.inbox_standing.outstanding, 2);
        assert_eq!(
            inbox_line(&result.rendered_markdown),
            Some("Inbox: 2 item(s) outstanding, age undeterminable — run /ductus:groom to route"),
            "an age blame cannot determine is unknown, never today"
        );
    }
}
