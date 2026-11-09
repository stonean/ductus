//! Which `ductus` tools a slash command's steps name (spec 080). The MCP
//! server flags the installed `target` command's tools to load up front
//! ([`AlwaysLoaded`]), and every other Claude Code copy loads its own in one
//! search through the line [`add_tool_search_line`] writes. Both read the
//! same derivation, [`step_tools`].

use std::collections::BTreeSet;
use std::path::Path;

use crate::host::Host;
use crate::parser::{self, ParseError};
use crate::primitives::line_ending_of;
use crate::schema::procedure::{Procedure, Step};
use crate::schema::registry;

/// The command whose step tools a session loads up front: the one that opens
/// a session.
pub const ALWAYS_LOADED_COMMAND: &str = "target";

/// The served tools the numbered steps of `procedure` name: each served
/// registry name occurring as a whole token in a step's prose, backticked or
/// bare, since a per-item primitive is written bare. A CLI-only name is left
/// out, since a host cannot load it, and the `## Instructions` preamble is no
/// step.
#[must_use]
pub fn step_tools(procedure: &Procedure) -> BTreeSet<&'static str> {
    let mut tools = BTreeSet::new();
    for step in &procedure.steps {
        let text: &str = match step {
            Step::Primitive { prose, .. } | Step::Extension { prose, .. } => prose,
            Step::Prose { text, .. } => text,
        };
        tools.extend(registry::served_primitives().filter(|name| names_token(text, name)));
    }
    tools
}

/// Whether `name` occurs in `text` with no name character on either side, so
/// `dashboard` is not found in `dashboard-primitive`, nor `read-spec` in
/// `unread-spec`.
fn names_token(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(start, _)| {
        let before = text[..start].chars().next_back();
        let after = text[start + name.len()..].chars().next();
        !before.is_some_and(is_name_char) && !after.is_some_and(is_name_char)
    })
}

/// A character a registry name, or a longer identifier holding one, can
/// continue with.
fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

/// The step tools of a command's text. For a file the parser reads as legacy
/// prose they are [`section_tools`], since the parser ends the
/// `## Instructions` section at its first sub-heading, so a command whose
/// steps all sit under sub-headings has no parsed step (owner, 2026-11-09).
///
/// # Errors
///
/// The parser's error for a malformed procedure.
pub fn text_step_tools(text: &str) -> Result<BTreeSet<&'static str>, ParseError> {
    match parser::parse(text, "") {
        Ok(procedure) => Ok(step_tools(&procedure)),
        Err(ParseError::LegacyProse) => Ok(section_tools(text)),
        Err(err) => Err(err),
    }
}

/// The served names occurring as whole tokens in a command's
/// `## Instructions` section, sub-headings included, past its preamble
/// blockquote and up to the next heading of the same or a higher level. None
/// when the file has no such section.
#[must_use]
pub fn section_tools(text: &str) -> BTreeSet<&'static str> {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let headings = atx_headings(&lines);
    let Some(&(start, level, _)) = headings.iter().find(|h| h.2 == "Instructions") else {
        return BTreeSet::new();
    };
    let end = headings
        .iter()
        .find(|h| h.0 > start && h.1 <= level)
        .map_or(lines.len(), |h| h.0);
    let mut first = start + 1;
    while first < end && lines[first].trim().is_empty() {
        first += 1;
    }
    while first < end && lines[first].starts_with('>') {
        first += 1;
    }
    let body: String = lines[first..end].concat();
    registry::served_primitives()
        .filter(|name| names_token(&body, name))
        .collect()
}

/// `text`, a command copy after substitution, with a last paragraph added to
/// its `## Instructions` preamble blockquote that loads its step tools, less
/// `always_loaded`, in one Claude Code `ToolSearch` (spec 080, AC29). The
/// names are sorted, as one `select:` query, and the line takes the file's
/// own line ending. A preamble is opened when the section has none. `text`
/// comes back unchanged when no tool remains.
///
/// The parser reads a preamble code span as a reference, never a dispatch,
/// so the copy parses to its source's procedure.
///
/// # Errors
///
/// The parser's error for a malformed procedure, and an
/// [`ParseError::Invalid`] when the steps' `## Instructions` heading is not an
/// ATX heading this can find.
pub fn add_tool_search_line(
    text: &str,
    always_loaded: &BTreeSet<&'static str>,
) -> Result<String, ParseError> {
    let remaining: Vec<&str> = text_step_tools(text)?
        .difference(always_loaded)
        .copied()
        .collect();
    if remaining.is_empty() {
        return Ok(text.to_owned());
    }
    let query = remaining
        .iter()
        .map(|name| format!("mcp__ductus__{name}"))
        .collect::<Vec<_>>()
        .join(",");
    let eol = line_ending_of(text);
    let line = format!(
        "> **Claude Code:** load this command's `ductus` tools in one `ToolSearch` call before \
         step 1: `select:{query}`"
    );

    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let heading = instructions_heading(&lines).ok_or_else(|| ParseError::Invalid {
        message: "no ATX `## Instructions` heading to hold the tool-search line".into(),
        location: None,
    })?;
    let mut first = heading + 1;
    while lines.get(first).is_some_and(|l| l.trim().is_empty()) {
        first += 1;
    }
    let mut out = String::with_capacity(text.len() + line.len() + 8);
    if lines.get(first).is_some_and(|l| l.starts_with('>')) {
        let mut last = first;
        while lines.get(last + 1).is_some_and(|l| l.starts_with('>')) {
            last += 1;
        }
        for l in &lines[..=last] {
            out.push_str(l);
        }
        if !lines[last].ends_with('\n') {
            out.push_str(eol);
        }
        out.push('>');
        out.push_str(eol);
        out.push_str(&line);
        out.push_str(eol);
        for l in &lines[last + 1..] {
            out.push_str(l);
        }
    } else {
        for l in &lines[..first] {
            out.push_str(l);
        }
        out.push_str(&line);
        out.push_str(eol);
        out.push_str(eol);
        for l in &lines[first..] {
            out.push_str(l);
        }
    }
    Ok(out)
}

/// The index of the first ATX heading reading `Instructions`, at any level,
/// as the parser takes it.
fn instructions_heading(lines: &[&str]) -> Option<usize> {
    atx_headings(lines)
        .into_iter()
        .find(|h| h.2 == "Instructions")
        .map(|h| h.0)
}

/// Each ATX heading outside a fenced code block, as its line index, its
/// level and its text.
fn atx_headings<'a>(lines: &[&'a str]) -> Vec<(usize, usize, &'a str)> {
    let mut headings = Vec::new();
    let mut fence: Option<&str> = None;
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim_end_matches(['\n', '\r']).trim_start();
        if let Some(open) = fence {
            if trimmed.starts_with(open) {
                fence = None;
            }
            continue;
        }
        if trimmed.starts_with("```") {
            fence = Some("```");
            continue;
        }
        if trimmed.starts_with("~~~") {
            fence = Some("~~~");
            continue;
        }
        let level = trimmed.chars().take_while(|&c| c == '#').count();
        let rest = &trimmed[level..];
        if (1..=6).contains(&level) && rest.starts_with([' ', '\t']) {
            headings.push((index, level, rest.trim().trim_end_matches('#').trim()));
        }
    }
    headings
}

/// What the MCP server found for [`ALWAYS_LOADED_COMMAND`]'s installed copy,
/// the first of [`Host::command_file_candidates`] that exists.
#[derive(Debug, PartialEq, Eq)]
pub enum AlwaysLoaded {
    /// The copy at `path` parsed, and its steps name `tools`.
    Derived {
        /// The copy, repo-relative.
        path: String,
        /// Its step tools.
        tools: BTreeSet<&'static str>,
    },
    /// No candidate holds a copy, so no tool is flagged, as before spec 080.
    NoCopy {
        /// The paths tried, repo-relative, in resolution order.
        candidates: Vec<String>,
    },
    /// The copy at `path` could not be read or parsed, so no tool is flagged.
    Unreadable {
        /// The copy, repo-relative.
        path: String,
        /// Why.
        reason: String,
    },
}

impl AlwaysLoaded {
    /// Find and derive the installed copy under `repo`.
    #[must_use]
    pub fn resolve(repo: &Path) -> Self {
        let candidates = Host::load(repo).command_file_candidates(ALWAYS_LOADED_COMMAND);
        for path in &candidates {
            let source = match std::fs::read_to_string(repo.join(path)) {
                Ok(source) => source,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
                Err(err) => {
                    return Self::Unreadable {
                        path: path.clone(),
                        reason: err.to_string(),
                    };
                }
            };
            return match parser::parse(&source, ALWAYS_LOADED_COMMAND) {
                Ok(procedure) => Self::Derived {
                    path: path.clone(),
                    tools: step_tools(&procedure),
                },
                Err(err) => Self::Unreadable {
                    path: path.clone(),
                    reason: err.to_string(),
                },
            };
        }
        Self::NoCopy { candidates }
    }

    /// The tools to flag: none unless the copy was derived.
    #[must_use]
    pub fn tools(&self) -> BTreeSet<&'static str> {
        match self {
            Self::Derived { tools, .. } => tools.clone(),
            Self::NoCopy { .. } | Self::Unreadable { .. } => BTreeSet::new(),
        }
    }

    /// The line the server writes to stderr when it flags nothing because it
    /// found no usable copy, so that state is never silent. `None` once
    /// derived.
    #[must_use]
    pub fn notice(&self) -> Option<String> {
        match self {
            Self::Derived { .. } => None,
            Self::NoCopy { candidates } => Some(format!(
                "ductus: no installed `{ALWAYS_LOADED_COMMAND}` command at {}, so no tool is \
                 flagged to load up front (spec 080)",
                candidates.join(", ")
            )),
            Self::Unreadable { path, reason } => Some(format!(
                "ductus: the installed `{ALWAYS_LOADED_COMMAND}` command at {path} could not be \
                 read ({reason}), so no tool is flagged to load up front (spec 080)"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::fs;

    use super::*;
    use crate::schema::procedure::{SourceRange, StepNumber};

    fn location() -> SourceRange {
        SourceRange {
            start_line: 1,
            start_col: 1,
            end_line: 1,
            end_col: 1,
        }
    }

    fn prose_step(text: &str) -> Step {
        Step::Prose {
            number: StepNumber(vec![1]),
            text: text.into(),
            location: location(),
        }
    }

    fn tools_of(steps: Vec<Step>) -> Vec<&'static str> {
        step_tools(&Procedure {
            command: "x".into(),
            steps,
        })
        .into_iter()
        .collect()
    }

    #[test]
    fn a_name_counts_backticked_or_bare_and_never_inside_a_longer_token() {
        assert_eq!(
            tools_of(vec![prose_step(
                "Invoke `read-spec`, then enumerate the dashboard payload's `specs[].slug`."
            )]),
            vec!["dashboard", "read-spec"]
        );
        assert_eq!(
            tools_of(vec![prose_step("See the dashboard-primitive scenario.")]),
            Vec::<&str>::new(),
            "a name followed by a name character is part of a longer token"
        );
        assert_eq!(
            tools_of(vec![prose_step("Keep the unread-spec notes.")]),
            Vec::<&str>::new(),
            "a name preceded by a name character is part of a longer token"
        );
    }

    #[test]
    fn a_cli_only_name_is_left_out() {
        assert_eq!(
            tools_of(vec![prose_step("Invoke `run-generator`, then read-tasks.")]),
            vec!["read-tasks"]
        );
    }

    #[test]
    fn every_kind_of_step_contributes_its_prose() {
        let steps = vec![
            Step::Primitive {
                number: StepNumber(vec![1]),
                name: "read-spec".into(),
                prose: "Invoke `read-spec`.".into(),
                location: location(),
            },
            Step::Extension {
                number: StepNumber(vec![2]),
                identifier: "writeCode".into(),
                prose: "<!-- llm:writeCode --> Then mark-task.".into(),
                location: location(),
            },
            prose_step("Render the view, as dashboard does."),
        ];
        assert_eq!(tools_of(steps), vec!["dashboard", "mark-task", "read-spec"]);
    }

    #[test]
    fn the_preamble_names_no_step_tool() {
        let source = "## Instructions\n\n> Invoke `resolve-session` first.\n\n\
                      1. Invoke `read-spec` against the feature.\n";
        let procedure = parser::parse(source, "x").unwrap();
        assert_eq!(
            step_tools(&procedure).into_iter().collect::<Vec<_>>(),
            vec!["read-spec"]
        );
    }

    const LINE: &str = "> **Claude Code:** load this command's `ductus` tools in one `ToolSearch` \
                        call before step 1: ";

    #[test]
    fn the_line_closes_the_preamble_naming_the_tools_left_sorted() {
        let text = "# X\n\n## Instructions\n\n> **For agent runtimes**: a.\n>\n> b.\n\n\
                    1. Invoke `write-session`.\n2. Invoke `read-spec`.\n3. Then mark-task.\n";
        let out = add_tool_search_line(text, &BTreeSet::from(["write-session"])).unwrap();
        assert_eq!(
            out,
            format!(
                "# X\n\n## Instructions\n\n> **For agent runtimes**: a.\n>\n> b.\n>\n{LINE}\
                 `select:mcp__ductus__mark-task,mcp__ductus__read-spec`\n\n\
                 1. Invoke `write-session`.\n2. Invoke `read-spec`.\n3. Then mark-task.\n"
            )
        );
    }

    #[test]
    fn no_tool_left_and_no_procedure_leave_the_text_as_it_is() {
        let text = "## Instructions\n\n> a.\n\n1. Invoke `write-session`.\n";
        assert_eq!(
            add_tool_search_line(text, &BTreeSet::from(["write-session"])).unwrap(),
            text
        );
        let prose = "# Help\n\nNo steps, though resolve-session is named.\n";
        assert_eq!(
            add_tool_search_line(prose, &BTreeSet::new()).unwrap(),
            prose
        );
    }

    #[test]
    fn a_section_with_no_preamble_gets_one() {
        let text = "## Instructions\n\n1. Invoke `read-spec`.\n";
        assert_eq!(
            add_tool_search_line(text, &BTreeSet::new()).unwrap(),
            format!(
                "## Instructions\n\n{LINE}`select:mcp__ductus__read-spec`\n\n1. Invoke `read-spec`.\n"
            )
        );
    }

    #[test]
    fn the_line_takes_the_files_line_ending() {
        let text = "## Instructions\r\n\r\n> a.\r\n\r\n1. Invoke `read-spec`.\r\n";
        let out = add_tool_search_line(text, &BTreeSet::new()).unwrap();
        assert!(
            out.contains(&format!(
                "> a.\r\n>\r\n{LINE}`select:mcp__ductus__read-spec`\r\n\r\n1."
            )),
            "{out:?}"
        );
        assert!(
            !out.replace("\r\n", "").contains('\n'),
            "no bare LF: {out:?}"
        );
    }

    /// A command whose steps all sit under sub-headings, as amend's do,
    /// parses as legacy prose.
    const LEGACY: &str = "# X\n\n## Instructions\n\n> Invoke `resolve-session` first.\n\n\
                          ### Confirm\n\n1. Invoke `read-spec`.\n\n### Record\n\n\
                          1. Then append-task.\n\n## Notes\n\nmark-task is named past the section.\n";

    #[test]
    fn a_legacy_prose_command_names_the_tools_under_its_sub_headings() {
        assert_eq!(parser::parse(LEGACY, "x"), Err(ParseError::LegacyProse));
        assert_eq!(
            text_step_tools(LEGACY)
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["append-task", "read-spec"],
            "neither the preamble's tool nor one past the section's end"
        );
        let out = add_tool_search_line(LEGACY, &BTreeSet::new()).unwrap();
        assert!(
            out.contains(&format!(
                "> Invoke `resolve-session` first.\n>\n{LINE}\
                 `select:mcp__ductus__append-task,mcp__ductus__read-spec`\n\n### Confirm"
            )),
            "{out}"
        );
    }

    #[test]
    fn a_heading_inside_a_fence_is_not_the_section() {
        let text = "# X\n\n```markdown\n## Instructions\n```\n\n## Instructions\n\n\
                    1. Invoke `read-spec`.\n";
        let out = add_tool_search_line(text, &BTreeSet::new()).unwrap();
        assert_eq!(
            out,
            format!(
                "# X\n\n```markdown\n## Instructions\n```\n\n## Instructions\n\n{LINE}\
                 `select:mcp__ductus__read-spec`\n\n1. Invoke `read-spec`.\n"
            )
        );
    }

    /// A project named `acme`, with `target.md` installed at Claude's
    /// layout when `target` is given.
    fn project(target: Option<&str>) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[host]\nproject = \"acme\"\n",
        )
        .unwrap();
        if let Some(source) = target {
            let dir = tmp.path().join(".claude/commands/acme");
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("target.md"), source).unwrap();
        }
        tmp
    }

    #[test]
    fn an_installed_copy_is_derived_and_needs_no_notice() {
        let tmp = project(Some(
            "## Instructions\n\n1. Invoke `resolve-session`.\n2. Invoke `write-session`.\n",
        ));
        let found = AlwaysLoaded::resolve(tmp.path());
        assert_eq!(
            found,
            AlwaysLoaded::Derived {
                path: ".claude/commands/acme/target.md".into(),
                tools: BTreeSet::from(["resolve-session", "write-session"]),
            }
        );
        assert_eq!(
            found.tools(),
            BTreeSet::from(["resolve-session", "write-session"])
        );
        assert_eq!(found.notice(), None);
    }

    #[test]
    fn no_copy_flags_nothing_and_names_every_path_tried() {
        let tmp = project(None);
        let found = AlwaysLoaded::resolve(tmp.path());
        let AlwaysLoaded::NoCopy { candidates } = &found else {
            panic!("expected no copy, got {found:?}");
        };
        assert_eq!(
            candidates,
            &Host::load(tmp.path()).command_file_candidates("target")
        );
        assert!(found.tools().is_empty());
        let notice = found.notice().unwrap();
        assert!(
            notice.contains("no installed `target` command at .claude/commands/acme/target.md"),
            "{notice}"
        );
    }

    #[test]
    fn a_copy_that_does_not_parse_flags_nothing_and_says_why() {
        let tmp = project(Some("# Target\n\nNo steps here.\n"));
        let found = AlwaysLoaded::resolve(tmp.path());
        let AlwaysLoaded::Unreadable { path, .. } = &found else {
            panic!("expected an unreadable copy, got {found:?}");
        };
        assert_eq!(path, ".claude/commands/acme/target.md");
        assert!(found.tools().is_empty());
        let notice = found.notice().unwrap();
        assert!(
            notice.contains("at .claude/commands/acme/target.md could not be read"),
            "{notice}"
        );
        assert!(
            notice.contains("legacy prose"),
            "the parse error is the reason: {notice}"
        );
    }
}
