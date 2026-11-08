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
//! Each rendering is held to a bound in bytes, the configured read size
//! (AC24): past it, the `done (spec is complete)` rows collapse to a count
//! line, and if it is still over, the rows past the bound are cut and a line
//! counts the specs not shown, so a partial view never reads as complete.
//! Paging is no option, since the hook path shows one message.

use std::fmt::Write as _;

use crate::primitives::{flatten_line, parse_feature_dir};
use crate::schema::primitives::{
    DashboardConfig, DashboardSessionTarget, DashboardSpec, InboxStanding, InboxState,
    ReopeningCounts, ReopeningState, SessionNotice,
};
use crate::schema::status::ALLOWED_STATUSES;

/// Which rendering to produce.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Format {
    /// The pipe table and list summary the model path's hosts render.
    Markdown,
    /// Fixed-width plain text for a terminal that renders no markdown.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the hook surface (spec 080 task 6) renders it; until then only the tests do"
        )
    )]
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
}

/// The view, ready to render either way.
#[derive(Debug)]
pub(crate) struct Model {
    target: Vec<String>,
    rows: Vec<Row>,
    summary: Vec<Item>,
    references: Vec<(String, Vec<String>)>,
    next_actions: Vec<(String, Vec<String>)>,
    prompt: String,
}

/// Build the view's model.
pub(crate) fn build(data: ViewData<'_>) -> Model {
    let project = data.project;
    let target_slug = data.session_target.map(|t| t.feature.as_str());
    let rows = data
        .specs
        .iter()
        .map(|spec| Row {
            cells: [
                spec.slug.clone(),
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
            Some((_, slugs)) => slugs.push(spec.slug.clone()),
            None => next_actions.push((action, vec![spec.slug.clone()])),
        }
    }
    Model {
        target: target_lines(&data),
        rows,
        summary: summary(&data),
        references: data.references,
        next_actions,
        prompt: format!("Run /{project}:target <feature> to select one."),
    }
}

/// Render the model as `format`, held to `bound` bytes.
///
/// Within the bound the whole view renders. Past it, the
/// `done (spec is complete)` rows collapse to one count line; if it is still
/// over, the most rows that fit are kept and a line counts the specs not
/// shown. A view whose summary alone is over the bound keeps no row and says
/// how many it cut, which is the most it can do.
pub(crate) fn render(model: &Model, format: Format, bound: u64) -> String {
    let all: Vec<&Row> = model.rows.iter().collect();
    let full = compose(model, format, &all, &[]);
    if fits(&full, bound) {
        return full;
    }
    let open: Vec<&Row> = model.rows.iter().filter(|row| !row.complete).collect();
    let collapsed = model.rows.len() - open.len();
    let mut trailer = Vec::new();
    if collapsed > 0 {
        trailer.push(format!("{COMPLETE}: {collapsed} spec(s)"));
        let view = compose(model, format, &open, &trailer);
        if fits(&view, bound) {
            return view;
        }
    }
    // Keeping more rows only lengthens the view, so the largest count that
    // fits is found by bisection.
    let with = |kept: usize| {
        let mut lines = trailer.clone();
        lines.push(format!(
            "{} spec(s) not shown — the view is over the read size",
            open.len() - kept
        ));
        compose(model, format, &open[..kept], &lines)
    };
    let (mut lo, mut hi) = (0, open.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if fits(&with(mid), bound) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    with(lo)
}

fn fits(view: &str, bound: u64) -> bool {
    u64::try_from(view.len()).unwrap_or(u64::MAX) <= bound
}

/// The view with `rows` in its table, and `trailer` as a block of its own
/// under it: a line directly under a markdown table would read as a row.
fn compose(model: &Model, format: Format, rows: &[&Row], trailer: &[String]) -> String {
    let mut blocks = vec![
        model.target.join("\n"),
        match format {
            Format::Markdown => markdown_table(rows),
            Format::Text => text_table(rows),
        },
        trailer.join("\n"),
        summary_block(&model.summary),
    ];
    if !model.references.is_empty() {
        blocks.push(references_block(&model.references, format));
    }
    if !model.next_actions.is_empty() {
        blocks.push(next_actions_block(&model.next_actions));
    }
    blocks.push(model.prompt.clone());
    blocks.retain(|b| !b.is_empty());
    blocks.join("\n\n")
}

/// The target line, then the scenario, notice and unreadable-session lines.
/// Without a target, the first line says so; the hint to set one is the
/// view's closing prompt.
fn target_lines(data: &ViewData<'_>) -> Vec<String> {
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
                    |s| (s.status.clone(), next_action(s, data.project)),
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
            let mut line = format!("Target: {} / {status} / next: {next}", target.feature);
            if let Some(label) = data.session_label {
                let _ = write!(line, " — session {label}");
            }
            lines.push(line);
            if let Some(scenario) = &target.scenario {
                let (section, open) = target
                    .scenario_detail
                    .as_ref()
                    .map(|d| (d.section.clone(), d.open_question_count))
                    .unwrap_or_default();
                lines.push(format!(
                    "Scenario: {scenario} ({section}) — open-questions: {open}"
                ));
            }
        }
    }
    // Notices follow the no-target line too, since a consolidation's notice is
    // exactly what explains a target that has gone (spec 062).
    for notice in data.notices {
        lines.push(format!("Notice: {}", notice.message));
    }
    for path in data.unreadable {
        lines.push(format!(
            "Unreadable: {path} — a session file that does not parse, so whether it shares \
             this feature was not checked"
        ));
    }
    lines
}

/// The summary items, one fact each, in a fixed order. `Counts:`,
/// `Reopenings` and `Inbox:` render on every run; the rest when they apply.
fn summary(data: &ViewData<'_>) -> Vec<Item> {
    let (specs, project) = (data.specs, data.project);
    let item = |line: String| Item {
        line,
        details: Vec::new(),
    };
    let mut items = Vec::new();
    if !specs.is_empty() {
        items.push(item(counts_line(specs)));
    }
    let blocked: Vec<&str> = specs
        .iter()
        .filter(|s| !s.blocked_by.is_empty())
        .map(|s| s.slug.as_str())
        .collect();
    if !blocked.is_empty() {
        items.push(item(format!(
            "Blocked: {} spec(s) — {}",
            blocked.len(),
            blocked.join(", ")
        )));
    }
    let recovery: Vec<&str> = specs
        .iter()
        .filter(|s| in_recovery(s))
        .map(|s| s.slug.as_str())
        .collect();
    if !recovery.is_empty() {
        items.push(item(format!(
            "Recovery: {} spec(s) past clarification with open questions again — {}. Run \
             /{project}:clarify on each; the spec reverts to draft and advances forward again.",
            recovery.len(),
            recovery.join(", ")
        )));
    }
    items.extend(questions_item(data.questions, project));
    if let Some(line) = folds_line(specs, project) {
        items.push(item(line));
    }
    if !data.tags_union.is_empty() {
        items.push(item(format!("Tags: {}", data.tags_union.join(", "))));
    }
    let config = data.config;
    if config.present && !config.disabled_rule_files.is_empty() {
        // Each basename is committed config, rendered on one line so a
        // newline in it cannot forge a view line (`BE-INPUT-011`), as
        // `discover-rule-files` renders the same values.
        let shown: Vec<String> = config
            .disabled_rule_files
            .iter()
            .map(|file| flatten_line(file))
            .collect();
        items.push(item(format!(
            "Disabled rule files: {} ({}) — {}",
            config.disabled_rule_files.len(),
            data.config_name,
            shown.join(", ")
        )));
    }
    if !config.unknown_keys.is_empty() {
        // Each entry is already one line: a key that needs it is quoted and
        // escaped by the registry (`BE-INPUT-011`).
        items.push(item(format!(
            "Unknown config keys: {} ({}) — {} — this version of ductus does not read them",
            config.unknown_keys.len(),
            data.config_name,
            config.unknown_keys.join(", ")
        )));
    }
    items.push(item(reopenings_line(data.reopenings)));
    items.push(item(inbox_line(data.inbox, project)));
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
/// listed, with no cap: a truncated list reads as "these are the ones needing
/// attention" while hiding others.
fn questions_item(questions: &QuestionsBySpec, project: &str) -> Option<Item> {
    let carrying: Vec<&(String, Vec<String>)> =
        questions.iter().filter(|(_, q)| !q.is_empty()).collect();
    if carrying.is_empty() {
        return None;
    }
    let total: usize = carrying.iter().map(|(_, q)| q.len()).sum();
    let details = carrying
        .iter()
        .flat_map(|(slug, scenarios)| scenarios.iter().map(move |s| format!("{slug} · {s}")))
        .collect();
    Some(Item {
        line: format!(
            "Scenario questions: {total} in {} spec(s); a spec is not complete while its \
             scenarios carry questions, so clarify each with /{project}:clarify.",
            carrying.len()
        ),
        details,
    })
}

/// The pending-fold item: the backlog the rows' qualification marks, on one
/// line so it reads without scanning the table.
fn folds_line(specs: &[DashboardSpec], project: &str) -> Option<String> {
    let pending: Vec<&DashboardSpec> = specs.iter().filter(|s| s.folds_into.is_some()).collect();
    if pending.is_empty() {
        return None;
    }
    let detail: Vec<String> = pending
        .iter()
        .map(|s| {
            let target = s.folds_into.as_deref().unwrap_or("—");
            if s.fold_target_missing {
                format!("{} → {target} (not in this tree)", s.slug)
            } else {
                format!("{} → {target}", s.slug)
            }
        })
        .collect();
    Some(format!(
        "Folds: {} spec(s) with an outstanding fold — {}. Run /{project}:fold on each once it is \
         done; a fold never holds done, and fold-back retires the branch-scoped spec. A target \
         reported as not in this tree is the normal state before the merge — it lives on the \
         upstream branch, and the fold waits for the tree that holds both — so correct it only \
         when it is a typo.",
        pending.len(),
        detail.join("; ")
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
/// hand-edited frontmatter) still counts, appended after the known tiers.
fn counts_line(specs: &[DashboardSpec]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut counted: Vec<&str> = Vec::new();
    for status in ALLOWED_STATUSES {
        let n = specs.iter().filter(|s| s.status == *status).count();
        if n > 0 {
            parts.push(format!("{status} {n}"));
            counted.push(status);
        }
    }
    for spec in specs {
        if !counted.contains(&spec.status.as_str()) {
            let n = specs.iter().filter(|s| s.status == spec.status).count();
            parts.push(format!("{} {n}", spec.status));
            counted.push(spec.status.as_str());
        }
    }
    format!("Counts: {}", parts.join(" · "))
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

/// One line per item, each item's details indented two spaces beneath it.
fn summary_block(items: &[Item]) -> String {
    let mut lines = Vec::new();
    for item in items {
        lines.push(item.line.clone());
        lines.extend(item.details.iter().map(|d| format!("  {d}")));
    }
    lines.join("\n")
}

fn references_block(sections: &[(String, Vec<String>)], format: Format) -> String {
    let rendered: Vec<String> = sections
        .iter()
        .map(|(slug, lines)| {
            let heading = match format {
                Format::Markdown => format!("**{slug}**"),
                Format::Text => slug.clone(),
            };
            let body: Vec<String> = lines.iter().map(|l| format!("- {l}")).collect();
            format!("{heading}\n\n{}", body.join("\n"))
        })
        .collect();
    format!("References:\n\n{}", rendered.join("\n\n"))
}

fn next_actions_block(groups: &[(String, Vec<String>)]) -> String {
    let lines: Vec<String> = groups
        .iter()
        .map(|(action, slugs)| format!("  {action}: {}", slugs.join(", ")))
        .collect();
    format!("Next actions:\n{}", lines.join("\n"))
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
        let mut action = format!("/{project}:fold → {target}");
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
        other => other.to_string(),
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
    if spec.folds_into.is_some() {
        return format!("{} (fold pending)", spec.status);
    }
    spec.status.clone()
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
                .map_or_else(|| dep.clone(), |n| format!("{n:03}"))
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
            gamma.blocked_by = Vec::new();
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
        let rendered = render(
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
        let markdown = render(&model, Format::Markdown, u64::MAX);
        let text = render(&model, Format::Text, u64::MAX);
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
            let rendered = render(&model, format, u64::MAX);
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
        let rendered = render(
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
        let rendered = render(&Fixture::new(None).model(), Format::Markdown, u64::MAX);
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

    #[test]
    fn a_view_at_the_bound_renders_whole() {
        let model = generated(60).model();
        for format in [Format::Markdown, Format::Text] {
            let full = render(&model, format, u64::MAX);
            assert_eq!(render(&model, format, size(&full)), full);
            assert_eq!(table_rows(&full, format).len(), 60);
        }
    }

    #[test]
    fn past_the_bound_the_done_rows_collapse_to_a_count_line() {
        let model = generated(60).model();
        for format in [Format::Markdown, Format::Text] {
            let full = render(&model, format, u64::MAX);
            let bound = size(&full) - 1;
            let view = render(&model, format, bound);
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
            let full = render(&model, format, u64::MAX);
            let collapsed = render(&model, format, size(&full) - 1);
            let bound = size(&collapsed) - 500;
            let view = render(&model, format, bound);
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
