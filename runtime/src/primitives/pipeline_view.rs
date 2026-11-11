//! The pipeline view `/{project}:status` shows (spec 080): one [`Model`] built
//! from the dashboard's data, rendered as markdown for the model path and as
//! fixed-width text for the hook path (AC17, AC19).
//!
//! The two renderings differ in the table and its emphasis only. Markdown keeps
//! the pipe table with the target's row in bold. Text pads each column to its
//! widest cell with two-space gaps and marks the target's row with `→`, since
//! the hook path shows it in a terminal that renders no markdown. Every other
//! line is the same in both: the summary one item per line, the specs with
//! work left listed by next action, and the view's one `/{project}:target`
//! hint at the end (AC6, AC18).
//!
//! Each rendering is held to a bound in bytes, the configured read size,
//! measured on the result its caller serializes (AC24). Past it, the
//! `done (spec is complete)` rows collapse to a count line; then the rows past
//! the bound are cut and counted; then, with no row left, the `Next actions:`
//! block keeps the leading names that fit; then each other list collapses to
//! its count, in [`CUT_ORDER`]. So a partial view never reads as complete, and
//! each line standing for what was cut can name the file holding the whole
//! view, which `render-status` writes for the model path. The hook path shows
//! one message, so it cannot page and writes no file.

use std::fmt::Write as _;

use crate::primitives::{flatten_line, parse_feature_dir};
use crate::schema::primitives::{
    DashboardConfig, DashboardSessionTarget, DashboardSpec, InboxStanding, InboxState,
    ReopeningCounts, ReopeningState, SessionNotice,
};
use crate::schema::status::ALLOWED_STATUSES;

/// `value` as text in one view line: each tab and every [`flatten_line`]
/// hazard a space, trimmed. Every value the view takes from a file name or
/// from frontmatter passes through it, so none can add a line, carry an
/// escape sequence, or shift a fixed-width column (`BE-INPUT-011`). The tab
/// is replaced here rather than in [`flatten_line`], since 058's stored
/// decision keys normalize through that.
pub(crate) fn view_text(value: &str) -> String {
    flatten_line(&value.replace('\t', " "))
}

/// Which rendering to produce.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Format {
    /// The pipe table and list summary the model path's hosts render.
    Markdown,
    /// Fixed-width plain text for a terminal that renders no markdown, which
    /// `ductus hook status` shows.
    Text,
}

/// The Next Action of a spec whose pipeline is complete.
pub(crate) const COMPLETE: &str = "done (spec is complete)";

/// The table's column headings.
const HEADINGS: [&str; 8] = [
    "Feature",
    "Status",
    "Plan",
    "Tasks",
    "Data-model",
    "Scenarios",
    "Dependencies",
    "Next Action",
];

/// Each spec carrying unresolved scenario questions, by slug, with the
/// scenario of each question, in the shared scenario-file order.
pub(crate) type QuestionsBySpec = Vec<(String, Vec<String>)>;

/// What the view is built from.
pub(crate) struct ViewData<'a> {
    pub specs: &'a [DashboardSpec],
    /// The specs carrying unresolved scenario questions; a spec with none has
    /// no entry.
    pub questions: &'a QuestionsBySpec,
    pub tags_union: &'a [String],
    pub config: &'a DashboardConfig,
    /// The repo-relative config file the read resolved, for provenance tags.
    pub config_name: &'a str,
    pub inbox: &'a InboxStanding,
    pub reopenings: &'a ReopeningCounts,
    pub session_target: Option<&'a DashboardSessionTarget>,
    /// This process's session label, when it has an identity.
    pub session_label: Option<&'a str>,
    pub notices: &'a [SessionNotice],
    pub unreadable: &'a [String],
    /// The cross-service readout: each spec declaring references, with one
    /// line per resolved reference.
    pub references: Vec<(String, Vec<String>)>,
    pub project: &'a str,
}

/// One table row.
#[derive(Debug)]
struct Row {
    cells: [String; 8],
    target: bool,
    /// Whether the spec's Next Action is [`COMPLETE`], so its row is the
    /// first to collapse past the bound.
    complete: bool,
}

/// One summary item, with the lines that detail it.
#[derive(Debug)]
struct Item {
    line: String,
    details: Vec<String>,
    /// The list the item collapses as past the bound, with its lines once
    /// collapsed, the last of which the not-shown words end. `None` for an
    /// item the bound never cuts.
    cut: Option<(Cut, Vec<String>)>,
}

/// A list the bound collapses to its count once no row and no next-action
/// name is left, one at a time in [`CUT_ORDER`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cut {
    References,
    DisabledRuleFiles,
    UnknownConfigKeys,
    Tags,
    Folds,
    Recovery,
    Blocked,
    ScenarioQuestions,
    /// The out-of-set statuses in `Counts:`.
    OtherStatuses,
    /// The `Next actions:` block's lines, once they name no spec.
    NextActionGroups,
    /// The `Notice:` and `Unreadable:` lines.
    SessionLines,
}

/// The order the lists collapse in (spec 080's plan, *Bound (AC24)*). The
/// session lines go last, since they explain the target the view opens with.
const CUT_ORDER: [Cut; 11] = [
    Cut::References,
    Cut::DisabledRuleFiles,
    Cut::UnknownConfigKeys,
    Cut::Tags,
    Cut::Folds,
    Cut::Recovery,
    Cut::Blocked,
    Cut::ScenarioQuestions,
    Cut::OtherStatuses,
    Cut::NextActionGroups,
    Cut::SessionLines,
];

/// How much of the model a rendering shows.
#[derive(Clone, Copy, Debug)]
struct Shown {
    /// Whether the `done (spec is complete)` rows are collapsed to a count
    /// line.
    done_collapsed: bool,
    /// The leading rows kept, of those the table holds.
    rows: usize,
    /// The leading next-action names kept, `None` for every one.
    names: Option<usize>,
    /// How many lists of [`CUT_ORDER`], from its start, are collapsed.
    collapsed: usize,
}

impl Shown {
    /// The whole view.
    fn all(model: &Model) -> Self {
        Self {
            done_collapsed: false,
            rows: model.rows.len(),
            names: None,
            collapsed: 0,
        }
    }

    fn cuts(&self, cut: Cut) -> bool {
        CUT_ORDER[..self.collapsed].contains(&cut)
    }
}

/// The view, ready to render either way.
#[derive(Debug)]
pub(crate) struct Model {
    /// The target line, then the scenario line when one is targeted.
    target: Vec<String>,
    /// One `Notice:` line per session notice.
    notices: Vec<String>,
    /// One `Unreadable:` line per session file that does not parse.
    unreadable: Vec<String>,
    rows: Vec<Row>,
    summary: Vec<Item>,
    references: Vec<(String, Vec<String>)>,
    next_actions: Vec<(String, Vec<String>)>,
    prompt: String,
}

/// Build the view's model.
pub(crate) fn build(data: ViewData<'_>) -> Model {
    // The namespace is `[host] project`, or the project directory's name when
    // that key is absent, so it is flattened like any other file-derived value
    // before a line names it (`BE-INPUT-011`).
    let project_text = view_text(data.project);
    let data = ViewData {
        project: &project_text,
        ..data
    };
    let project = data.project;
    let target_slug = data.session_target.map(|t| t.feature.as_str());
    let rows = data
        .specs
        .iter()
        .map(|spec| Row {
            cells: [
                view_text(&spec.slug),
                status_cell(spec),
                mark(spec.has_plan).to_string(),
                mark(spec.has_tasks).to_string(),
                mark(spec.has_data_model).to_string(),
                scenarios_cell(spec),
                dependency_prefixes(&spec.dependencies),
                next_action(spec, project),
            ],
            target: Some(spec.slug.as_str()) == target_slug,
            complete: next_action(spec, project) == COMPLETE,
        })
        .collect();
    let mut next_actions: Vec<(String, Vec<String>)> = Vec::new();
    for spec in data.specs {
        let action = next_action(spec, project);
        if action == COMPLETE || Some(spec.slug.as_str()) == target_slug {
            continue;
        }
        match next_actions.iter_mut().find(|(a, _)| *a == action) {
            Some((_, slugs)) => slugs.push(view_text(&spec.slug)),
            None => next_actions.push((action, vec![view_text(&spec.slug)])),
        }
    }
    let (target, notices, unreadable) = target_lines(&data);
    Model {
        target,
        notices,
        unreadable,
        rows,
        summary: summary(&data),
        references: data.references,
        next_actions,
        prompt: format!("Run /{project}:target <feature> to select one."),
    }
}

/// The whole view as `format`, nothing cut.
pub(crate) fn whole(model: &Model, format: Format) -> String {
    compose(model, format, &Shown::all(model), None)
}

/// Render the model as `format`: the most of it for which `fits` holds,
/// `fits` measuring the result the caller serializes the rendering in.
/// `whole_at`, when the caller has written the whole view to a file, names
/// that file, and every line standing for what was cut names it too.
///
/// Within the bound the whole view renders. Past it, spec 080's plan,
/// *Bound (AC24)*, applies its steps in turn until the view fits: the
/// `done (spec is complete)` rows collapse to a count line; the rows past
/// the bound are cut and counted; with no row left, the `Next actions:` block
/// keeps the leading names that fit; and each list of [`CUT_ORDER`] collapses
/// to its count, the rows and then the names being fitted again after each.
/// What is left then is the floor, which a read size below it gets, over the
/// bound, since it is the least a view can say.
pub(crate) fn render(
    model: &Model,
    format: Format,
    fits: &dyn Fn(&str) -> bool,
    whole_at: Option<&str>,
) -> String {
    let view = |shown: &Shown| compose(model, format, shown, whole_at);
    let whole = view(&Shown::all(model));
    if fits(&whole) {
        return whole;
    }
    let open = model.rows.iter().filter(|row| !row.complete).count();
    let names: usize = model
        .next_actions
        .iter()
        .map(|(_, slugs)| slugs.len())
        .sum();
    for collapsed in 0..=CUT_ORDER.len() {
        let rowless = Shown {
            done_collapsed: true,
            rows: 0,
            names: None,
            collapsed,
        };
        if fits(&view(&rowless)) {
            let rows = largest(open, |rows| fits(&view(&Shown { rows, ..rowless })));
            return view(&Shown { rows, ..rowless });
        }
        if fits(&view(&Shown {
            names: Some(0),
            ..rowless
        })) {
            let names = Some(largest(names, |kept| {
                fits(&view(&Shown {
                    names: Some(kept),
                    ..rowless
                }))
            }));
            return view(&Shown { names, ..rowless });
        }
    }
    view(&Shown {
        done_collapsed: true,
        rows: 0,
        names: Some(0),
        collapsed: CUT_ORDER.len(),
    })
}

/// The largest count up to `max` that `holds`, given that zero holds. Keeping
/// more only lengthens the view, so it is found by bisection.
fn largest(max: usize, holds: impl Fn(usize) -> bool) -> usize {
    let (mut lo, mut hi) = (0, max);
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if holds(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

/// The words that end each line standing for what the bound cut.
fn not_shown(whole_at: Option<&str>) -> String {
    match whole_at {
        Some(file) => {
            format!("not shown — the view is over the read size; the whole view is in {file}")
        }
        None => "not shown — the view is over the read size".to_string(),
    }
}

/// The view as `shown` cuts it. The lines standing for the cut rows sit in a
/// block of their own under the table, since a line directly under a
/// markdown table would read as a row.
fn compose(model: &Model, format: Format, shown: &Shown, whole_at: Option<&str>) -> String {
    let marker = not_shown(whole_at);
    let mut target = model.target.clone();
    if shown.cuts(Cut::SessionLines) {
        if !model.notices.is_empty() {
            target.push(format!(
                "Notice: {} notice(s) {marker}",
                model.notices.len()
            ));
        }
        if !model.unreadable.is_empty() {
            target.push(format!(
                "Unreadable: {} session file(s) {marker}",
                model.unreadable.len()
            ));
        }
    } else {
        target.extend(model.notices.iter().cloned());
        target.extend(model.unreadable.iter().cloned());
    }
    let table: Vec<&Row> = model
        .rows
        .iter()
        .filter(|row| !(shown.done_collapsed && row.complete))
        .collect();
    let rows = &table[..shown.rows.min(table.len())];
    let mut trailer = Vec::new();
    let done = model.rows.len() - table.len();
    if done > 0 {
        trailer.push(format!("{COMPLETE}: {done} spec(s)"));
    }
    if rows.len() < table.len() {
        trailer.push(format!("{} spec(s) {marker}", table.len() - rows.len()));
    }
    let mut blocks = vec![
        target.join("\n"),
        match format {
            Format::Markdown => markdown_table(rows),
            Format::Text => text_table(rows),
        },
        trailer.join("\n"),
        summary_block(&model.summary, shown, &marker),
    ];
    if !model.references.is_empty() {
        blocks.push(if shown.cuts(Cut::References) {
            format!("References: {} spec(s) {marker}", model.references.len())
        } else {
            references_block(&model.references, format)
        });
    }
    if !model.next_actions.is_empty() {
        blocks.push(next_actions_block(&model.next_actions, shown, &marker));
    }
    blocks.push(model.prompt.clone());
    blocks.retain(|b| !b.is_empty());
    blocks.join("\n\n")
}

/// The target block, in three parts: the target line and the scenario line,
/// then the notice lines, then the unreadable-session lines. Without a
/// target, the first line says so; the hint to set one is the view's closing
/// prompt.
fn target_lines(data: &ViewData<'_>) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut lines = Vec::new();
    match data.session_target {
        None => lines.push("No session target.".to_string()),
        Some(target) => {
            // A target naming a spec that is not on disk degrades to
            // placeholders rather than failing the whole render.
            let (status, mut next) = data
                .specs
                .iter()
                .find(|s| s.slug == target.feature)
                .map_or_else(
                    || ("unknown".to_string(), "—".to_string()),
                    |s| (view_text(&s.status), next_action(s, data.project)),
                );
            if target
                .scenario_detail
                .as_ref()
                .is_some_and(|d| d.open_question_count >= 1)
            {
                // A targeted scenario with unresolved questions owns the
                // next action regardless of the parent spec's status.
                next = format!("/{}:clarify (scenario-targeted)", data.project);
            }
            // Hand-written values, each flattened onto one line, so a newline in
            // one cannot forge a view line (`BE-INPUT-011`).
            let mut line = format!(
                "Target: {} / {status} / next: {next}",
                view_text(&target.feature)
            );
            if let Some(label) = data.session_label {
                let _ = write!(line, " — session {label}");
            }
            lines.push(line);
            if let Some(scenario) = &target.scenario {
                let scenario = view_text(scenario);
                let (section, open) = target
                    .scenario_detail
                    .as_ref()
                    .map(|d| (view_text(&d.section), d.open_question_count))
                    .unwrap_or_default();
                lines.push(format!(
                    "Scenario: {scenario} ({section}) — open-questions: {open}"
                ));
            }
        }
    }
    // Notices follow the no-target line too, since a consolidation's notice is
    // exactly what explains a target that has gone (spec 062).
    // A notice can carry a session file's hand-written feature, and an
    // unreadable path is a file name, so each is flattened onto one line
    // too (`BE-INPUT-011`).
    let notices = data
        .notices
        .iter()
        .map(|notice| format!("Notice: {}", view_text(&notice.message)))
        .collect();
    let unreadable = data
        .unreadable
        .iter()
        .map(|path| {
            format!(
                "Unreadable: {} — a session file that does not parse, so whether it shares \
                 this feature was not checked",
                view_text(path)
            )
        })
        .collect();
    (lines, notices, unreadable)
}

/// The summary items, one fact each, in a fixed order. `Counts:` renders
/// when a spec exists, and `Reopenings` and `Inbox:` on every run; the rest
/// when they apply.
fn summary(data: &ViewData<'_>) -> Vec<Item> {
    let (specs, project) = (data.specs, data.project);
    let whole = |line: String| Item {
        line,
        details: Vec::new(),
        cut: None,
    };
    let cut = |line: String, cut: Cut, collapsed: String| Item {
        line,
        details: Vec::new(),
        cut: Some((cut, vec![collapsed])),
    };
    let mut items = Vec::new();
    if !specs.is_empty() {
        items.push(counts_item(specs));
    }
    let blocked: Vec<String> = specs
        .iter()
        .filter(|s| !s.blocked_by.is_empty())
        .map(|s| view_text(&s.slug))
        .collect();
    if !blocked.is_empty() {
        items.push(cut(
            format!(
                "Blocked: {} spec(s) — {}",
                blocked.len(),
                blocked.join(", ")
            ),
            Cut::Blocked,
            format!("Blocked: {} spec(s)", blocked.len()),
        ));
    }
    let recovery: Vec<String> = specs
        .iter()
        .filter(|s| in_recovery(s))
        .map(|s| view_text(&s.slug))
        .collect();
    if !recovery.is_empty() {
        items.push(cut(
            format!(
                "Recovery: {} spec(s) past clarification with open questions again — {}. Run \
                 /{project}:clarify on each; the spec reverts to draft and advances forward \
                 again.",
                recovery.len(),
                recovery.join(", ")
            ),
            Cut::Recovery,
            format!("Recovery: {} spec(s)", recovery.len()),
        ));
    }
    items.extend(questions_item(data.questions, project));
    if let Some((line, pending)) = folds_line(specs, project) {
        items.push(cut(line, Cut::Folds, format!("Folds: {pending} spec(s)")));
    }
    if !data.tags_union.is_empty() {
        // A tag is hand-written frontmatter, rendered on one line so a
        // newline in it cannot forge a view line (`BE-INPUT-011`).
        let tags: Vec<String> = data.tags_union.iter().map(|t| view_text(t)).collect();
        items.push(cut(
            format!("Tags: {}", tags.join(", ")),
            Cut::Tags,
            format!("Tags: {}", tags.len()),
        ));
    }
    items.extend(config_items(data));
    items.push(whole(reopenings_line(data.reopenings)));
    items.push(whole(inbox_line(data.inbox, project)));
    items
}

/// The config items: the disabled rule files and the unknown config keys,
/// each when it applies, each collapsing to its count past the bound.
fn config_items(data: &ViewData<'_>) -> Vec<Item> {
    let cut = |line: String, cut: Cut, collapsed: String| Item {
        line,
        details: Vec::new(),
        cut: Some((cut, vec![collapsed])),
    };
    let mut items = Vec::new();
    let config = data.config;
    if config.present && !config.disabled_rule_files.is_empty() {
        // Each basename is committed config, rendered on one line so a
        // newline in it cannot forge a view line (`BE-INPUT-011`), as
        // `discover-rule-files` renders the same values.
        let shown: Vec<String> = config
            .disabled_rule_files
            .iter()
            .map(|file| view_text(file))
            .collect();
        items.push(cut(
            format!(
                "Disabled rule files: {} ({}) — {}",
                config.disabled_rule_files.len(),
                data.config_name,
                shown.join(", ")
            ),
            Cut::DisabledRuleFiles,
            format!(
                "Disabled rule files: {} ({})",
                config.disabled_rule_files.len(),
                data.config_name
            ),
        ));
    }
    if !config.unknown_keys.is_empty() {
        // Each entry is already one line: a key that needs it is quoted and
        // escaped by the registry (`BE-INPUT-011`).
        items.push(cut(
            format!(
                "Unknown config keys: {} ({}) — {} — this version of ductus does not read them",
                config.unknown_keys.len(),
                data.config_name,
                config.unknown_keys.join(", ")
            ),
            Cut::UnknownConfigKeys,
            format!(
                "Unknown config keys: {} ({})",
                config.unknown_keys.len(),
                data.config_name
            ),
        ));
    }
    items
}

/// The scenario-questions item, with one detail line per question naming its
/// spec and scenario, so a scenario with three questions has three lines. The
/// question's text is left to `/{project}:clarify`: the model path re-types
/// the view, and the text took this repository's view from 10,904 characters
/// to 16,434 (owner, 2026-11-08; spec 080's plan).
///
/// Rendered whether or not a spec is also in recovery: only the Next Action
/// cell is exclusive, and suppressing this item would hide questions that
/// still need resolving after the recovery walk (spec 046). Every question is
/// listed, with no cap, within the read size: a truncated list reads as
/// "these are the ones needing attention" while hiding others. Past it, the
/// lines collapse to one counting them, which names the file holding them on
/// the model path, so none is hidden (046's AC24; spec 080's plan).
fn questions_item(questions: &QuestionsBySpec, project: &str) -> Option<Item> {
    let carrying: Vec<&(String, Vec<String>)> =
        questions.iter().filter(|(_, q)| !q.is_empty()).collect();
    if carrying.is_empty() {
        return None;
    }
    let total: usize = carrying.iter().map(|(_, q)| q.len()).sum();
    let details = carrying
        .iter()
        .flat_map(|(slug, scenarios)| {
            scenarios
                .iter()
                .map(move |s| format!("{} · {}", view_text(slug), view_text(s)))
        })
        .collect();
    let line = format!(
        "Scenario questions: {total} in {} spec(s); a spec is not complete while its \
         scenarios carry questions, so clarify each with /{project}:clarify.",
        carrying.len()
    );
    Some(Item {
        cut: Some((
            Cut::ScenarioQuestions,
            vec![line.clone(), format!("  {total} question line(s)")],
        )),
        line,
        details,
    })
}

/// The pending-fold item, with how many specs it names: the backlog the
/// rows' qualification marks, on one line so it reads without scanning the
/// table.
fn folds_line(specs: &[DashboardSpec], project: &str) -> Option<(String, usize)> {
    let pending: Vec<&DashboardSpec> = specs.iter().filter(|s| s.folds_into.is_some()).collect();
    if pending.is_empty() {
        return None;
    }
    let detail: Vec<String> = pending
        .iter()
        .map(|s| {
            let target = view_text(s.folds_into.as_deref().unwrap_or("—"));
            if s.fold_target_missing {
                format!("{} → {target} (not in this tree)", view_text(&s.slug))
            } else {
                format!("{} → {target}", view_text(&s.slug))
            }
        })
        .collect();
    Some((
        format!(
            "Folds: {} spec(s) with an outstanding fold — {}. Run /{project}:fold on each once it \
             is done; a fold never holds done, and fold-back retires the branch-scoped spec. A \
             target reported as not in this tree is the normal state before the merge — it lives \
             on the upstream branch, and the fold waits for the tree that holds both — so \
             correct it only when it is a typo.",
            pending.len(),
            detail.join("; ")
        ),
        pending.len(),
    ))
}

/// The reopenings item, rendered on **every** run (spec 079, AC19). A state the
/// count cannot speak to renders its own words, so nothing reads as a clean zero
/// over a history that was not examined.
fn reopenings_line(reopenings: &ReopeningCounts) -> String {
    let since = reopenings.since.as_deref().unwrap_or("the window's start");
    let waiting = format!(
        "{} in-progress spec(s) with no unchecked task",
        reopenings.waiting_count
    );
    match reopenings.state {
        ReopeningState::Measured => format!(
            "Reopenings since {since}: {} closed, {} over the review and analysis bound, \
             {} not counted · {waiting}",
            reopenings.closed_count, reopenings.over_bound_count, reopenings.not_counted_count,
        ),
        ReopeningState::NothingToCompare => {
            format!("Reopenings since {since}: none closed, nothing to compare · {waiting}")
        }
        ReopeningState::ShallowHistory => {
            format!("Reopenings: ? history too shallow to measure · {waiting}")
        }
        ReopeningState::NotMeasured => format!(
            "Reopenings: ? not measured — {}",
            reopenings.reason.as_deref().unwrap_or("no reason given")
        ),
    }
}

/// The `Counts:` item, in lifecycle order. An out-of-set status (a
/// hand-edited frontmatter) still counts, appended after the known tiers on
/// one line, so a newline in it cannot forge a view line (`BE-INPUT-011`).
/// Past the bound the out-of-set statuses collapse to one part counting
/// them, since every distinct one adds a part.
fn counts_item(specs: &[DashboardSpec]) -> Item {
    let mut known: Vec<String> = Vec::new();
    for status in ALLOWED_STATUSES {
        let n = specs.iter().filter(|s| s.status == *status).count();
        if n > 0 {
            known.push(format!("{status} {n}"));
        }
    }
    let mut other: Vec<(&str, usize)> = Vec::new();
    for spec in specs {
        if ALLOWED_STATUSES.contains(&spec.status.as_str()) {
            continue;
        }
        match other.iter_mut().find(|(status, _)| *status == spec.status) {
            Some((_, n)) => *n += 1,
            None => other.push((spec.status.as_str(), 1)),
        }
    }
    let parts: Vec<String> = known
        .iter()
        .cloned()
        .chain(
            other
                .iter()
                .map(|(status, n)| format!("{} {n}", view_text(status))),
        )
        .collect();
    let cut = (!other.is_empty()).then(|| {
        let specs_in_other: usize = other.iter().map(|(_, n)| n).sum();
        let mut collapsed = known.clone();
        collapsed.push(format!(
            "{specs_in_other} spec(s) in {} other status(es)",
            other.len()
        ));
        (
            Cut::OtherStatuses,
            vec![format!("Counts: {}", collapsed.join(" · "))],
        )
    });
    Item {
        line: format!("Counts: {}", parts.join(" · ")),
        details: Vec::new(),
        cut,
    }
}

/// The standing inbox item, rendered on **every** run in one of four states.
///
/// Never omitted: a clean inbox renders a clean row and an absent one renders
/// its own state, because examined-and-empty, not-computed, and nothing-there
/// must not share an output. An age that `git blame` could not determine is
/// reported as undeterminable — never as today.
fn inbox_line(inbox: &InboxStanding, project: &str) -> String {
    match inbox.state {
        InboxState::NoFile => format!("Inbox: ? no readable {} — nothing examined", inbox.path),
        InboxState::Clean => "Inbox: ✓ clean".to_string(),
        InboxState::Outstanding => match inbox.oldest.as_deref() {
            Some(oldest) => format!(
                "Inbox: {} item(s) outstanding, oldest {oldest} — run /{project}:groom to route",
                inbox.outstanding
            ),
            None => format!(
                "Inbox: {} item(s) outstanding, age undeterminable — run /{project}:groom to route",
                inbox.outstanding
            ),
        },
    }
}

fn markdown_table(rows: &[&Row]) -> String {
    let mut lines = vec![
        format!("| {} |", HEADINGS.join(" | ")),
        format!("|{}", " --- |".repeat(HEADINGS.len())),
    ];
    for row in rows {
        let mut cells: Vec<String> = row.cells.to_vec();
        if row.target {
            cells[0] = format!("**{}**", cells[0]);
        }
        lines.push(format!("| {} |", cells.join(" | ")));
    }
    lines.join("\n")
}

/// Each column padded to its widest cell, two spaces between columns, and a
/// two-character gutter that marks the target's row. The last column is not
/// padded, so no line ends in spaces.
fn text_table(rows: &[&Row]) -> String {
    let mut widths: Vec<usize> = HEADINGS.iter().map(|h| h.chars().count()).collect();
    for row in rows {
        for (width, cell) in widths.iter_mut().zip(&row.cells) {
            *width = (*width).max(cell.chars().count());
        }
    }
    let line = |gutter: &str, cells: &[&str]| {
        let mut out = gutter.to_string();
        for (i, cell) in cells.iter().enumerate() {
            out.push_str(cell);
            if i + 1 < cells.len() {
                let pad = widths[i] - cell.chars().count() + 2;
                out.push_str(&" ".repeat(pad));
            }
        }
        out
    };
    let mut lines = vec![line("  ", &HEADINGS)];
    for row in rows {
        let cells: Vec<&str> = row.cells.iter().map(String::as_str).collect();
        lines.push(line(if row.target { "→ " } else { "  " }, &cells));
    }
    lines.join("\n")
}

/// One line per item, each item's details indented two spaces beneath it,
/// or the item's collapsed lines when `shown` cuts its list.
fn summary_block(items: &[Item], shown: &Shown, marker: &str) -> String {
    let mut lines = Vec::new();
    for item in items {
        match &item.cut {
            Some((cut, collapsed)) if shown.cuts(*cut) => {
                if let Some((last, leading)) = collapsed.split_last() {
                    lines.extend(leading.iter().cloned());
                    lines.push(format!("{last} {marker}"));
                }
            }
            _ => {
                lines.push(item.line.clone());
                lines.extend(item.details.iter().map(|d| format!("  {d}")));
            }
        }
    }
    lines.join("\n")
}

fn references_block(sections: &[(String, Vec<String>)], format: Format) -> String {
    let rendered: Vec<String> = sections
        .iter()
        .map(|(slug, lines)| {
            let heading = match format {
                Format::Markdown => format!("**{}**", view_text(slug)),
                Format::Text => view_text(slug),
            };
            let body: Vec<String> = lines
                .iter()
                .map(|l| format!("- {}", view_text(l)))
                .collect();
            format!("{heading}\n\n{}", body.join("\n"))
        })
        .collect();
    format!("References:\n\n{}", rendered.join("\n\n"))
}

/// The `Next actions:` block with `shown`'s leading names: a group cut short
/// ends `… N more`, a group with none left counts its specs, and a line
/// counts the names not shown. Once its list is cut, one line counts them
/// all.
fn next_actions_block(groups: &[(String, Vec<String>)], shown: &Shown, marker: &str) -> String {
    let total: usize = groups.iter().map(|(_, slugs)| slugs.len()).sum();
    if shown.cuts(Cut::NextActionGroups) {
        return format!(
            "Next actions: {total} spec(s) under {} next action(s) {marker}",
            groups.len()
        );
    }
    let named = shown.names.unwrap_or(total).min(total);
    let mut left = named;
    let mut lines = vec!["Next actions:".to_string()];
    for (action, slugs) in groups {
        let kept = left.min(slugs.len());
        left -= kept;
        lines.push(if kept == slugs.len() {
            format!("  {action}: {}", slugs.join(", "))
        } else if kept == 0 {
            format!("  {action}: {} spec(s)", slugs.len())
        } else {
            format!(
                "  {action}: {}, … {} more",
                slugs[..kept].join(", "),
                slugs.len() - kept
            )
        });
    }
    if named < total {
        lines.push(format!("  {} of {total} spec(s) {marker}", total - named));
    }
    lines.join("\n")
}

/// The Status → next action mapping, with the `clarify (recovery)`
/// override for a non-draft spec that has re-acquired open questions.
/// An out-of-set status echoes as-is (`validate-frontmatter` owns
/// flagging it).
fn next_action(spec: &DashboardSpec, project: &str) -> String {
    if in_recovery(spec) {
        // Recovery wins the cell when both apply: spec-body questions at
        // `clarified` or later are the more upstream defect, and clearing
        // them reverts the spec to `draft` with the scenario questions
        // still there to resolve afterward (spec 046).
        return "clarify (recovery)".to_string();
    }
    if spec.scenario_open_question_count >= 1 {
        return "clarify (scenario)".to_string();
    }
    if let (Some(target), "done") = (&spec.folds_into, spec.status.as_str()) {
        // Done first, then fold (spec 051 scenario
        // `a-pending-fold-never-holds-done`). A staging spec completes its own
        // pipeline like any other, so below `done` the status-driven match
        // keeps the cell; at `done` the fold is what remains, and it takes
        // the cell instead of "complete". It sits *below* the two overrides
        // above because an open question is the more upstream defect: the
        // content gets settled before it gets moved.
        let mut action = format!("/{project}:fold → {}", view_text(target));
        if spec.fold_target_missing {
            // Called out on the row the fold is offered from, as well as in
            // the outstanding-fold item. A report, not a verdict — see
            // `DashboardSpec::fold_target_missing`.
            action.push_str(" (target not in this tree)");
        }
        return action;
    }
    match spec.status.as_str() {
        "draft" => format!("/{project}:clarify"),
        "clarified" => format!("/{project}:plan"),
        "planned" | "in-progress" => format!("/{project}:implement"),
        "done" => COMPLETE.to_string(),
        // Hand-written frontmatter, on one line so a newline in it cannot
        // forge a view line (`BE-INPUT-011`).
        other => view_text(other),
    }
}

/// The Status cell. A spec declaring `folds-into` is qualified as carrying
/// a pending fold, so the view never reports it as simply `done` — the fold
/// is owed whatever its status, until fold-back retires it (spec 051 AC34).
/// It never holds the spec short of `done`: the fold is run after it.
///
/// The frontmatter value is kept alongside the qualification rather than
/// replaced by it: the view's job is to show where a spec sits, and
/// `in-progress` and `done` are different situations for the operator even
/// though both still owe the fold — the first has its own pipeline to
/// finish, the second has only the fold left.
fn status_cell(spec: &DashboardSpec) -> String {
    let status = view_text(&spec.status);
    if spec.folds_into.is_some() {
        return format!("{status} (fold pending)");
    }
    status
}

/// Recovery state: a spec past clarification whose body has re-acquired
/// open questions (usually a manual frontmatter edit).
fn in_recovery(spec: &DashboardSpec) -> bool {
    matches!(
        spec.status.as_str(),
        "clarified" | "planned" | "in-progress"
    ) && spec.open_question_count >= 1
}

/// The Scenarios cell: the count, suffixed with the outstanding
/// question total when there is one. Reusing the existing column keeps the
/// glance table from growing a ninth one; a spec whose scenarios carry no
/// questions renders exactly as before (spec 046).
fn scenarios_cell(spec: &DashboardSpec) -> String {
    if spec.scenario_open_question_count == 0 {
        return spec.scenarios_count.to_string();
    }
    format!(
        "{} ({} open)",
        spec.scenarios_count, spec.scenario_open_question_count
    )
}

/// Artifact-existence table mark.
fn mark(present: bool) -> &'static str {
    if present { "✓" } else { "—" }
}

/// The Dependencies column: sorted three-digit `NNN` prefixes from the
/// dependency slugs (`—` when empty; a slug without an `NNN-` prefix
/// passes through raw rather than vanishing).
///
/// A branch-scoped dependency takes the pass-through arm: it carries no
/// sequential number, so it renders as its own name. Abbreviating it to
/// the first three digits would print `123` for `1234.1-thing`, naming a
/// different spec.
fn dependency_prefixes(dependencies: &[String]) -> String {
    if dependencies.is_empty() {
        return "—".to_string();
    }
    let mut prefixes: Vec<String> = dependencies
        .iter()
        .map(|dep| {
            parse_feature_dir(dep)
                .and_then(|form| form.sequential_number())
                .map_or_else(|| view_text(dep), |n| format!("{n:03}"))
        })
        .collect();
    prefixes.sort();
    prefixes.join(", ")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn spec(slug: &str, status: &str) -> DashboardSpec {
        DashboardSpec {
            slug: slug.to_string(),
            status: status.to_string(),
            dependencies: Vec::new(),
            tags: Vec::new(),
            open_question_count: 0,
            has_plan: status != "draft" && status != "clarified",
            has_tasks: status != "draft" && status != "clarified",
            has_data_model: false,
            scenarios_count: 0,
            scenario_open_question_count: 0,
            scenarios_with_questions: Vec::new(),
            blocked_by: Vec::new(),
            folds_into: None,
            fold_target_missing: false,
        }
    }

    /// Specs at every status, a target, scenario questions (one spanning two
    /// lines), a dependency, a tag and a references readout.
    struct Fixture {
        specs: Vec<DashboardSpec>,
        questions: QuestionsBySpec,
        tags: Vec<String>,
        config: DashboardConfig,
        inbox: InboxStanding,
        reopenings: ReopeningCounts,
        target: Option<DashboardSessionTarget>,
    }

    impl Fixture {
        fn new(target: Option<&str>) -> Self {
            let mut gamma = spec("003-gamma", "planned");
            gamma.dependencies = vec!["002-beta".to_string()];
            let mut delta = spec("004-delta", "clarified");
            delta.scenarios_count = 2;
            delta.scenario_open_question_count = 2;
            delta.scenarios_with_questions = vec!["s1".to_string()];
            Self {
                specs: vec![
                    spec("001-alpha", "done"),
                    spec("002-beta", "in-progress"),
                    gamma,
                    delta,
                    spec("005-epsilon", "draft"),
                    spec("006-zeta", "done"),
                ],
                questions: vec![(
                    "004-delta".to_string(),
                    vec!["s1".to_string(), "s1".to_string()],
                )],
                tags: vec!["pipeline".to_string()],
                config: DashboardConfig {
                    present: true,
                    disabled_rule_files: Vec::new(),
                    unknown_keys: Vec::new(),
                },
                inbox: InboxStanding {
                    state: InboxState::Clean,
                    outstanding: 0,
                    oldest: None,
                    path: "specs/inbox.md".to_string(),
                },
                reopenings: ReopeningCounts {
                    state: ReopeningState::NotMeasured,
                    reason: Some("no repo".to_string()),
                    ..ReopeningCounts::default()
                },
                target: target.map(|feature| DashboardSessionTarget {
                    feature: feature.to_string(),
                    scenario: None,
                    scenario_detail: None,
                }),
            }
        }

        fn model(&self) -> Model {
            build(ViewData {
                specs: &self.specs,
                questions: &self.questions,
                tags_union: &self.tags,
                config: &self.config,
                config_name: ".ductus/config.toml",
                inbox: &self.inbox,
                reopenings: &self.reopenings,
                session_target: self.target.as_ref(),
                session_label: None,
                notices: &[],
                unreadable: &[],
                references: vec![(
                    "003-gamma".to_string(),
                    vec!["svc/010-upstream → done".to_string()],
                )],
                project: "ductus",
            })
        }
    }

    fn blocks(rendered: &str) -> Vec<&str> {
        rendered.split("\n\n").collect()
    }

    /// The table, rows in order, as the cells each rendering shows: the
    /// markdown split on its pipes with the bold removed, the text split on
    /// its runs of two or more spaces after the gutter.
    fn markdown_rows(rendered: &str) -> Vec<Vec<String>> {
        blocks(rendered)[1]
            .lines()
            .filter(|line| !line.starts_with("| ---"))
            .map(|line| {
                line.trim_matches('|')
                    .split('|')
                    .map(|cell| cell.trim().trim_matches('*').to_string())
                    .collect()
            })
            .collect()
    }

    fn text_rows(rendered: &str) -> Vec<Vec<String>> {
        blocks(rendered)[1]
            .lines()
            .map(|line| {
                let body: String = line.chars().skip(2).collect();
                body.split("  ")
                    .map(str::trim)
                    .filter(|cell| !cell.is_empty())
                    .map(str::to_string)
                    .collect()
            })
            .collect()
    }

    /// Every line outside the table, with the markdown's emphasis removed.
    fn lines_outside_the_table(rendered: &str) -> Vec<String> {
        let mut parts = blocks(rendered);
        parts.remove(1);
        parts
            .join("\n\n")
            .lines()
            .map(|line| line.replace("**", ""))
            .collect()
    }

    #[test]
    fn the_text_table_aligns_its_columns_with_two_space_gaps_and_no_markdown() {
        let rendered = within(
            &Fixture::new(Some("002-beta")).model(),
            Format::Text,
            u64::MAX,
        );
        assert!(!rendered.contains("**"), "{rendered}");
        let table: Vec<&str> = blocks(&rendered)[1].lines().collect();
        assert_eq!(table.len(), 7, "{rendered}");
        let header: Vec<char> = table[0].chars().collect();
        let starts: Vec<usize> = HEADINGS
            .iter()
            .map(|heading| {
                table[0]
                    .find(heading)
                    .map(|byte| table[0][..byte].chars().count())
                    .unwrap()
            })
            .collect();
        for line in &table {
            assert!(!line.contains('|'), "{line}");
            assert!(!line.ends_with(' '), "{line:?}");
            let chars: Vec<char> = line.chars().collect();
            for &start in &starts[1..] {
                assert_eq!(&chars[start - 2..start], &[' ', ' '], "{line:?}");
                assert_ne!(chars[start], ' ', "{line:?}");
            }
        }
        assert_eq!(header[..2], [' ', ' ']);
        let target: Vec<&&str> = table.iter().filter(|l| l.starts_with("→ ")).collect();
        assert_eq!(target.len(), 1, "{rendered}");
        assert!(target[0].contains("002-beta"), "{rendered}");
    }

    #[test]
    fn both_renderings_carry_the_same_facts_from_one_model() {
        let model = Fixture::new(Some("002-beta")).model();
        let markdown = within(&model, Format::Markdown, u64::MAX);
        let text = within(&model, Format::Text, u64::MAX);
        assert!(markdown.contains("| **002-beta** |"), "{markdown}");
        let rows = markdown_rows(&markdown);
        assert_eq!(rows, text_rows(&text));
        assert_eq!(rows.len(), 7);
        assert_eq!(rows[0], HEADINGS.map(str::to_string));
        assert_eq!(
            rows[3],
            [
                "003-gamma",
                "planned",
                "✓",
                "✓",
                "—",
                "0",
                "002",
                "/ductus:implement"
            ]
            .map(str::to_string)
        );
        assert_eq!(
            lines_outside_the_table(&markdown),
            lines_outside_the_table(&text)
        );
    }

    #[test]
    fn the_summary_is_one_item_per_line_and_ends_with_the_one_target_hint() {
        let model = Fixture::new(Some("002-beta")).model();
        for format in [Format::Markdown, Format::Text] {
            let rendered = within(&model, format, u64::MAX);
            let lines: Vec<&str> = rendered.lines().collect();
            for item in [
                "Counts: draft 1 · clarified 1 · planned 1 · in-progress 1 · done 2",
                "Tags: pipeline",
                "Reopenings: ? not measured — no repo",
                "Inbox: ✓ clean",
            ] {
                assert!(lines.contains(&item), "{item:?} in:\n{rendered}");
            }
            assert_eq!(
                lines.iter().filter(|l| **l == "  004-delta · s1").count(),
                2,
                "one line per question, its scenario's twice:\n{rendered}"
            );
            assert!(
                lines
                    .iter()
                    .any(|l| l.starts_with("Scenario questions: 2 in 1 spec(s)")),
                "{rendered}"
            );
            assert_eq!(rendered.matches(":target").count(), 1, "{rendered}");
            assert_eq!(
                lines.last(),
                Some(&"Run /ductus:target <feature> to select one.")
            );
        }
    }

    #[test]
    fn the_specs_with_work_left_are_grouped_by_next_action_without_the_target() {
        let rendered = within(
            &Fixture::new(Some("002-beta")).model(),
            Format::Text,
            u64::MAX,
        );
        let block = blocks(&rendered)
            .into_iter()
            .find(|b| b.starts_with("Next actions:"))
            .unwrap_or_else(|| panic!("{rendered}"));
        assert_eq!(
            block,
            "Next actions:\n  /ductus:implement: 003-gamma\n  clarify (scenario): 004-delta\n  \
             /ductus:clarify: 005-epsilon"
        );
    }

    #[test]
    fn with_no_target_the_view_says_so_and_still_closes_with_the_one_hint() {
        let rendered = within(&Fixture::new(None).model(), Format::Markdown, u64::MAX);
        assert!(rendered.starts_with("No session target.\n\n"), "{rendered}");
        assert_eq!(rendered.matches(":target").count(), 1, "{rendered}");
        assert!(
            rendered.contains("  /ductus:implement: 002-beta, 003-gamma"),
            "{rendered}"
        );
    }

    /// A generated corpus of `n` specs, alternately `planned` and `done`, with
    /// no target and no scenario questions.
    fn generated(n: usize) -> Fixture {
        let mut fixture = Fixture::new(None);
        fixture.specs = (1..=n)
            .map(|i| {
                let status = if i % 2 == 0 { "done" } else { "planned" };
                spec(&format!("{i:03}-generated-{i:03}"), status)
            })
            .collect();
        fixture.questions = Vec::new();
        fixture
    }

    /// The table's data rows, without its header (and separator).
    fn table_rows(rendered: &str, format: Format) -> Vec<&str> {
        let skip = match format {
            Format::Markdown => 2,
            Format::Text => 1,
        };
        blocks(rendered)[1].lines().skip(skip).collect()
    }

    fn size(view: &str) -> u64 {
        u64::try_from(view.len()).unwrap()
    }

    /// `render` held to `bound` bytes of the view itself, with no file
    /// holding the whole view.
    fn within(model: &Model, format: Format, bound: u64) -> String {
        render(model, format, &|view| size(view) <= bound, None)
    }

    #[test]
    fn a_view_at_the_bound_renders_whole() {
        let model = generated(60).model();
        for format in [Format::Markdown, Format::Text] {
            let full = within(&model, format, u64::MAX);
            assert_eq!(within(&model, format, size(&full)), full);
            assert_eq!(table_rows(&full, format).len(), 60);
        }
    }

    #[test]
    fn past_the_bound_the_done_rows_collapse_to_a_count_line() {
        let model = generated(60).model();
        for format in [Format::Markdown, Format::Text] {
            let full = within(&model, format, u64::MAX);
            let bound = size(&full) - 1;
            let view = within(&model, format, bound);
            assert!(size(&view) <= bound, "{view}");
            let rows = table_rows(&view, format);
            assert_eq!(rows.len(), 30, "{view}");
            assert!(rows.iter().all(|row| !row.contains("done")), "{view}");
            assert_eq!(blocks(&view)[2], "done (spec is complete): 30 spec(s)");
            assert!(!view.contains("not shown"), "{view}");
        }
    }

    #[test]
    fn still_over_the_bound_the_rows_past_it_are_cut_and_counted() {
        let model = generated(60).model();
        for format in [Format::Markdown, Format::Text] {
            let full = within(&model, format, u64::MAX);
            let collapsed = within(&model, format, size(&full) - 1);
            let bound = size(&collapsed) - 500;
            let view = within(&model, format, bound);
            assert!(size(&view) <= bound, "{view}");
            let rows = table_rows(&view, format);
            let trailer: Vec<&str> = blocks(&view)[2].lines().collect();
            assert_eq!(trailer[0], "done (spec is complete): 30 spec(s)");
            let cut: usize = trailer[1]
                .strip_suffix(" spec(s) not shown — the view is over the read size")
                .unwrap_or_else(|| panic!("{view}"))
                .parse()
                .unwrap();
            assert!(cut > 0, "{view}");
            assert_eq!(rows.len() + cut, 30, "every open spec is shown or counted");
            assert!(rows[0].contains("001-generated-001"), "{view}");
            // The most rows that fit are kept: the room left is less than one
            // more row and its newline would take.
            let longest = rows.iter().map(|row| row.len()).max().unwrap();
            let room = usize::try_from(bound - size(&view)).unwrap();
            assert!(room <= longest, "{room} bytes left, a row takes {longest}");
        }
    }

    /// With no row left, the `Next actions:` block keeps its leading names,
    /// the group cut short says how many more it holds, a line counts the
    /// names not shown, and one more name would not fit.
    #[test]
    fn with_no_row_left_the_next_actions_keep_the_leading_names_that_fit() {
        let mut fixture = Fixture::new(None);
        fixture.specs = (1..=120)
            .map(|i| spec(&format!("{i:03}-named-{i:03}"), "planned"))
            .collect();
        fixture.questions = Vec::new();
        let model = fixture.model();
        for format in [Format::Markdown, Format::Text] {
            let shown = |names| Shown {
                done_collapsed: true,
                rows: 0,
                names,
                collapsed: 0,
            };
            let bound = size(&compose(&model, format, &shown(None), None)) - 1;
            let view = within(&model, format, bound);
            assert!(size(&view) <= bound, "{view}");
            assert!(table_rows(&view, format).is_empty(), "{view}");
            assert_eq!(
                blocks(&view)[2],
                "120 spec(s) not shown — the view is over the read size"
            );
            let block = blocks(&view)
                .into_iter()
                .find(|b| b.starts_with("Next actions:"))
                .unwrap_or_else(|| panic!("{view}"));
            let lines: Vec<&str> = block.lines().collect();
            let (kept, more) = lines[1]
                .strip_prefix("  /ductus:implement: ")
                .and_then(|names| names.rsplit_once(", … "))
                .unwrap_or_else(|| panic!("{block}"));
            assert!(kept.starts_with("001-named-001, 002-named-002"), "{kept}");
            let kept = kept.split(", ").count();
            let more: usize = more.strip_suffix(" more").unwrap().parse().unwrap();
            assert!(kept > 0 && more > 0, "{block}");
            assert_eq!(kept + more, 120, "every spec is named or counted");
            assert_eq!(
                lines[2],
                format!("  {more} of 120 spec(s) not shown — the view is over the read size")
            );
            let one_more = compose(&model, format, &shown(Some(kept + 1)), None);
            assert!(size(&one_more) > bound, "the most names that fit are kept");
        }
    }

    /// A view carrying every list [`CUT_ORDER`] names, and done rows.
    fn every_list() -> Model {
        let mut specs: Vec<DashboardSpec> = (1..=40)
            .map(|i| {
                let status = ["done", "planned", "draft", "clarified"][i % 4];
                let mut s = spec(&format!("{i:03}-listed-{i:03}"), status);
                s.tags = vec![format!("tag-{i:03}")];
                s
            })
            .collect();
        for s in specs.iter_mut().filter(|s| s.status == "planned").take(4) {
            s.open_question_count = 1;
        }
        for s in specs.iter_mut().filter(|s| s.status == "clarified").take(4) {
            s.blocked_by = vec!["002-listed-002".to_string()];
        }
        for s in specs.iter_mut().filter(|s| s.status == "done").take(3) {
            s.folds_into = Some("099-upstream".to_string());
            s.fold_target_missing = true;
        }
        specs.push(spec("041-odd", "weird-a"));
        specs.push(spec("042-odd", "weird-b"));
        let questions: QuestionsBySpec = specs
            .iter()
            .filter(|s| s.status == "draft")
            .take(5)
            .map(|s| {
                (
                    s.slug.clone(),
                    vec!["a-scenario".to_string(), "b-scenario".to_string()],
                )
            })
            .collect();
        let tags: Vec<String> = specs.iter().flat_map(|s| s.tags.clone()).collect();
        let fixture = Fixture::new(Some("002-listed-002"));
        build(ViewData {
            specs: &specs,
            questions: &questions,
            tags_union: &tags,
            config: &DashboardConfig {
                present: true,
                disabled_rule_files: vec!["a-cross.md".to_string(), "b-cross.md".to_string()],
                unknown_keys: vec!["[review] stray".to_string()],
            },
            config_name: ".ductus/config.toml",
            inbox: &fixture.inbox,
            reopenings: &fixture.reopenings,
            session_target: fixture.target.as_ref(),
            session_label: Some("fx"),
            notices: &[SessionNotice {
                kind: crate::schema::primitives::SessionNoticeKind::CoTarget,
                message: "Session fx-2 also targets 002-listed-002.".to_string(),
            }],
            unreadable: &[".ductus/sessions/broken.toml".to_string()],
            references: vec![(
                "003-listed-003".to_string(),
                vec!["svc/010-upstream → done".to_string()],
            )],
            project: "ductus",
        })
    }

    /// The lists a rendering shows collapsed, read from its lines.
    fn collapsed(view: &str) -> Vec<Cut> {
        let cut = |prefix: &str| {
            view.lines()
                .any(|line| line.starts_with(prefix) && line.contains(" not shown — "))
        };
        CUT_ORDER
            .into_iter()
            .filter(|list| match list {
                Cut::References => cut("References: "),
                Cut::DisabledRuleFiles => cut("Disabled rule files: "),
                Cut::UnknownConfigKeys => cut("Unknown config keys: "),
                Cut::Tags => cut("Tags: "),
                Cut::Folds => cut("Folds: "),
                Cut::Recovery => cut("Recovery: "),
                Cut::Blocked => cut("Blocked: "),
                Cut::ScenarioQuestions => cut("  10 question line(s)"),
                Cut::OtherStatuses => cut("Counts: "),
                Cut::NextActionGroups => cut("Next actions: "),
                Cut::SessionLines => cut("Notice: ") && cut("Unreadable: "),
            })
            .collect()
    }

    /// Past the rows and the names, the lists collapse one at a time in
    /// [`CUT_ORDER`], each smaller bound collapsing no fewer, every view
    /// above the floor fits, and the floor collapses them all and still
    /// opens with the target and closes with the one hint.
    #[test]
    fn past_the_rows_and_names_each_list_collapses_in_its_order() {
        let model = every_list();
        for format in [Format::Markdown, Format::Text] {
            let floor = within(&model, format, 1);
            assert_eq!(collapsed(&floor), CUT_ORDER, "{floor}");
            assert!(
                floor.starts_with("Target: 002-listed-002 / draft / "),
                "{floor}"
            );
            assert!(
                floor.ends_with("\n\nRun /ductus:target <feature> to select one."),
                "{floor}"
            );
            let mut last: Vec<Cut> = Vec::new();
            let mut bound = size(&whole(&model, format));
            while bound > size(&floor) {
                let view = within(&model, format, bound);
                assert!(size(&view) <= bound, "over {bound}:\n{view}");
                let cuts = collapsed(&view);
                assert_eq!(cuts, CUT_ORDER[..cuts.len()], "{view}");
                assert!(cuts.len() >= last.len(), "{view}");
                last = cuts;
                bound -= 23;
            }
            assert!(
                last.len() >= CUT_ORDER.len() - 1,
                "the sweep reached the floor's lists"
            );
        }
    }

    /// When the caller holds the whole view in a file, every line standing
    /// for what was cut names it.
    #[test]
    fn each_line_standing_for_a_cut_names_the_file_holding_the_whole_view() {
        let file = ".ductus/results/render-status/all-specs/0a.json";
        let floor = render(&every_list(), Format::Markdown, &|_| false, Some(file));
        let ending =
            format!("not shown — the view is over the read size; the whole view is in {file}");
        let cut: Vec<&str> = floor.lines().filter(|l| l.contains("not shown")).collect();
        assert_eq!(cut.len(), CUT_ORDER.len() + 2, "{floor}");
        for line in cut {
            assert!(line.ends_with(&ending), "{line}");
        }
    }

    /// A status or a tag is hand-written frontmatter, so each is flattened
    /// onto one line before it joins the view (`BE-INPUT-011`).
    #[test]
    fn a_newline_in_a_status_or_a_tag_cannot_forge_a_view_line() {
        let mut fixture = Fixture::new(Some("007-odd"));
        fixture.specs.push(spec("007-odd", "weird\nNotice: forged"));
        fixture.tags = vec!["a\nNotice: forged".to_string()];
        let model = fixture.model();
        for format in [Format::Markdown, Format::Text] {
            let view = within(&model, format, u64::MAX);
            assert!(!view.lines().any(|l| l.starts_with("Notice:")), "{view}");
            assert!(view.contains("Tags: a Notice: forged"), "{view}");
            assert!(view.contains("· weird Notice: forged 1"), "{view}");
        }
    }

    /// The other hand-written values that join a view line, the session's
    /// feature and scenario, a scenario's `section`, a `folds-into`, a
    /// dependency with no sequential number, a references line, a session
    /// notice, which can carry the session's feature, and an unreadable
    /// session file's path, are flattened onto one line too (`BE-INPUT-011`).
    #[test]
    fn a_newline_in_any_hand_written_value_cannot_forge_a_view_line() {
        let mut fixture = Fixture::new(None);
        fixture.target = Some(DashboardSessionTarget {
            feature: "002-beta\nNotice: forged feature".to_string(),
            scenario: Some("sc\nNotice: forged scenario".to_string()),
            scenario_detail: Some(crate::schema::primitives::DashboardScenarioDetail {
                section: "Behavior\nNotice: forged section".to_string(),
                context_summary: String::new(),
                open_question_count: 0,
            }),
        });
        let mut staged = spec("1.1-stage", "done");
        staged.folds_into = Some("001-alpha\nNotice: forged fold".to_string());
        staged.fold_target_missing = true;
        fixture.specs.push(staged);
        fixture.specs[2].dependencies = vec!["x\nNotice: forged dependency".to_string()];
        let references = vec![(
            "003-gamma".to_string(),
            vec!["svc\nNotice: forged reference".to_string()],
        )];
        let notices = [crate::schema::primitives::SessionNotice {
            kind: crate::schema::primitives::SessionNoticeKind::Adopted,
            message: "Session s will adopt 002-beta\nNotice: forged notice".to_string(),
        }];
        let unreadable = [".ductus/sessions/x\nNotice: forged unreadable.toml".to_string()];
        let model = build(ViewData {
            specs: &fixture.specs,
            questions: &fixture.questions,
            tags_union: &fixture.tags,
            config: &fixture.config,
            config_name: ".ductus/config.toml",
            inbox: &fixture.inbox,
            reopenings: &fixture.reopenings,
            session_target: fixture.target.as_ref(),
            session_label: None,
            notices: &notices,
            unreadable: &unreadable,
            references,
            project: "ductus",
        });
        for format in [Format::Markdown, Format::Text] {
            let view = within(&model, format, u64::MAX);
            assert_eq!(
                view.lines().filter(|l| l.starts_with("Notice:")).count(),
                1,
                "{view}"
            );
            for forged in [
                "feature",
                "scenario",
                "section",
                "fold",
                "dependency",
                "reference",
                "notice",
                "unreadable",
            ] {
                assert!(
                    view.contains(&format!("Notice: forged {forged}")),
                    "{forged}: {view}"
                );
            }
        }
    }

    /// A spec's directory name and a scenario's file name join the view as
    /// written, and a sequential directory name is not held to the slug
    /// grammar (`parse_sequential`), so each is flattened as frontmatter is,
    /// wherever it appears: the table, `Next actions:`, `Blocked:`,
    /// `Recovery:`, `Folds:`, the scenario-question lines and the References
    /// block. So is the namespace, which falls back to the project directory's
    /// name. A tab becomes a space as well, since it would shift a fixed-width
    /// column (`BE-INPUT-011`).
    #[test]
    fn a_line_break_or_tab_in_a_file_name_cannot_forge_a_line_or_shift_a_column() {
        let mut fixture = Fixture::new(None);
        let slug = "008-odd\nNotice: forged slug\tx";
        let mut odd = spec(slug, "planned");
        odd.open_question_count = 1;
        odd.blocked_by = vec!["001-alpha".to_string()];
        odd.folds_into = Some("001-alpha".to_string());
        fixture.specs.push(odd);
        fixture.questions.push((
            slug.to_string(),
            vec!["sc\nNotice: forged scenario\ty".to_string()],
        ));
        fixture.tags = vec!["a\tb".to_string()];
        let model = build(ViewData {
            specs: &fixture.specs,
            questions: &fixture.questions,
            tags_union: &fixture.tags,
            config: &fixture.config,
            config_name: ".ductus/config.toml",
            inbox: &fixture.inbox,
            reopenings: &fixture.reopenings,
            session_target: None,
            session_label: None,
            notices: &[],
            unreadable: &[],
            references: vec![(slug.to_string(), vec!["svc".to_string()])],
            project: "pro\nNotice: forged project\tz",
        });
        for format in [Format::Markdown, Format::Text] {
            let view = within(&model, format, u64::MAX);
            assert!(!view.lines().any(|l| l.starts_with("Notice:")), "{view}");
            assert!(!view.contains('\t'), "{view}");
            for line in [
                "Blocked: 1 spec(s) — 008-odd Notice: forged slug x",
                "008-odd Notice: forged slug x → 001-alpha",
                "008-odd Notice: forged slug x · sc Notice: forged scenario y",
                "Tags: a b",
                "Run /pro Notice: forged project z:target <feature> to select one.",
            ] {
                assert!(view.contains(line), "{line}: {view}");
            }
        }
    }

    #[test]
    fn the_reopenings_line_says_each_state_in_its_own_words() {
        let counts = |state, since: Option<&str>, reason: Option<&str>| ReopeningCounts {
            state,
            since: since.map(str::to_string),
            closed_count: 12,
            over_bound_count: 1,
            not_counted_count: 2,
            waiting_count: 3,
            runs: 20,
            reason: reason.map(str::to_string),
        };
        let since = Some("2026-10-06");
        let waiting = "3 in-progress spec(s) with no unchecked task";
        assert_eq!(
            reopenings_line(&counts(ReopeningState::Measured, since, None)),
            format!(
                "Reopenings since 2026-10-06: 12 closed, 1 over the review and analysis bound, \
                 2 not counted · {waiting}"
            )
        );
        assert_eq!(
            reopenings_line(&counts(ReopeningState::NothingToCompare, since, None)),
            format!("Reopenings since 2026-10-06: none closed, nothing to compare · {waiting}")
        );
        assert_eq!(
            reopenings_line(&counts(ReopeningState::ShallowHistory, since, None)),
            format!("Reopenings: ? history too shallow to measure · {waiting}")
        );
        assert_eq!(
            reopenings_line(&counts(ReopeningState::NotMeasured, None, Some("no repo"))),
            "Reopenings: ? not measured — no repo"
        );
    }

    #[test]
    fn a_dated_outstanding_inbox_names_its_oldest_item() {
        let standing = InboxStanding {
            state: InboxState::Outstanding,
            outstanding: 6,
            oldest: Some("2026-05-19".into()),
            path: "specs/inbox.md".into(),
        };
        assert_eq!(
            inbox_line(&standing, "ductus"),
            "Inbox: 6 item(s) outstanding, oldest 2026-05-19 — run /ductus:groom to route"
        );
    }
}
