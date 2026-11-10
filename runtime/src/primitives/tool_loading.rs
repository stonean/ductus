//! Which `ductus` tools a slash command's steps name (spec 080). The MCP
//! server flags the installed `target` command's tools to load up front
//! ([`AlwaysLoaded`]), and every other Claude Code copy loads its own in one
//! search through the line [`add_tool_search_line`] writes. Both read the
//! same derivation, [`text_step_tools`].

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::host::Host;
use crate::parser::{self, ParseError};
use crate::primitives::resolve_anchor::{HeadingLine, cited_heading, heading_lines, section_range};
use crate::primitives::{SkipScanner, line_ending_of, without_inline_comments};
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

/// Where a command names a reference: `.ductus/reference/<name>.md`, as its
/// "Where a named section lives" paragraph and its qualified citations write
/// it.
#[allow(clippy::expect_used)] // A hard-coded pattern that compiles.
fn reference_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"\.ductus/reference/([A-Za-z0-9_-]+)\.md").expect("hard-coded regex compiles")
    })
}

/// A bold span, which cites a section when its text equals a heading.
#[allow(clippy::expect_used)] // A hard-coded pattern that compiles.
fn bold_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\*\*([^*]+)\*\*").expect("hard-coded regex compiles"))
}

/// A name written as an invocation: `invoke`, `invokes` or `invoking`, an
/// optional `the`, then the backticked name.
#[allow(clippy::expect_used)] // A hard-coded pattern that compiles.
fn invoked_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"\b[Ii]nvok(?:e|es|ing)\s+(?:the\s+)?`([A-Za-z0-9_-]+)`")
            .expect("hard-coded regex compiles")
    })
}

/// The served names `text` invokes ([`invoked_regex`]).
fn invoked(text: &str) -> Vec<&'static str> {
    invoked_regex()
        .captures_iter(text)
        .filter_map(|cap| registry::served_primitives().find(|name| *name == &cap[1]))
        .collect()
}

/// The documents a citation can reach: the command file, or a reference it
/// names by `.ductus/reference/<name>.md`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Doc {
    Command,
    Reference(String),
}

/// A document's lines and headings, for the citation walk.
struct Indexed {
    lines: Vec<String>,
    headings: Vec<HeadingLine>,
    texts: Vec<String>,
}

impl Indexed {
    fn new(text: &str) -> Self {
        let headings = heading_lines(text);
        let mut texts: Vec<String> = headings.iter().map(|h| h.text.clone()).collect();
        texts.sort_unstable();
        texts.dedup();
        Self {
            lines: text.lines().map(str::to_owned).collect(),
            headings,
            texts,
        }
    }

    /// The section under the first heading reading `heading`: its heading
    /// line up to the next heading of the same or a higher level.
    fn section(&self, heading: &str) -> Option<String> {
        let range = section_range(&self.headings, heading, self.lines.len())?;
        Some(self.lines[range].join("\n"))
    }
}

/// The served tools a command file and its cited sections invoke (spec 080,
/// scenarios `one-search-covers-referenced-sections` and
/// `reference-reads-follow-the-cited-sections`). A name invoked anywhere in
/// the command file counts, so a paragraph outside the steps that invokes a
/// tool, as "Where a named section lives" invokes `read-section`, loads it.
/// A line of the command file
/// cites a section with a `§` followed by its heading, matched longest
/// first and ending where a word ends ([`cited_heading`]), or with a bold
/// span equal to its heading. A line naming `.ductus/reference/<name>.md`
/// resolves only against the references it names; any other line against
/// its own document, then the command file, then the command's own
/// reference, the one its text names as `.ductus/reference/<command>.md`.
/// Each cited section's own citations are followed in turn. Inside a cited
/// section a name counts only where it is invoked ([`invoked_regex`]), so a
/// section that only mentions a tool adds nothing. Lines inside fenced
/// blocks and HTML comments cite nothing.
///
/// `read_reference` returns a named reference's text, after the same
/// substitutions as the command's.
///
/// # Errors
///
/// What `read_reference` returns for a reference it cannot read, so a named
/// reference that is missing is never a line that silently lacks its tools.
pub fn cited_tools<E>(
    text: &str,
    command: &str,
    mut read_reference: impl FnMut(&str) -> Result<String, E>,
) -> Result<BTreeSet<&'static str>, E> {
    // A comment is content, not an invocation or a citation, whether it spans
    // lines or sits on one (plan *Batched (AC29)*).
    let mut skip = SkipScanner::default();
    let lines: Vec<std::borrow::Cow<'_, str>> = text
        .lines()
        .filter(|line| !skip.skip(line))
        .map(without_inline_comments)
        .collect();
    let own_reference = lines
        .iter()
        .any(|line| {
            reference_regex()
                .captures_iter(line)
                .any(|cap| &cap[1] == command)
        })
        .then(|| Doc::Reference(command.to_owned()));
    let mut docs: BTreeMap<Doc, Indexed> = BTreeMap::new();
    docs.insert(Doc::Command, Indexed::new(text));
    let mut seen: BTreeSet<(Doc, String)> = BTreeSet::new();
    let mut queue: Vec<(Doc, String)> = Vec::new();
    let mut tools: BTreeSet<&'static str> = lines.iter().flat_map(|line| invoked(line)).collect();

    let mut cite = |from: &Doc,
                    body: &str,
                    docs: &mut BTreeMap<Doc, Indexed>,
                    queue: &mut Vec<(Doc, String)>|
     -> Result<(), E> {
        let mut skip = SkipScanner::default();
        for line in body.lines() {
            if skip.skip(line) {
                continue;
            }
            let line = without_inline_comments(line);
            let line: &str = &line;
            let named: Vec<Doc> = reference_regex()
                .captures_iter(line)
                .map(|cap| Doc::Reference(cap[1].to_owned()))
                .collect();
            let candidates: Vec<Doc> = if named.is_empty() {
                let mut c = vec![from.clone(), Doc::Command];
                c.extend(own_reference.clone());
                c.dedup();
                c
            } else {
                named
            };
            for doc in &candidates {
                if let Doc::Reference(name) = doc
                    && !docs.contains_key(doc)
                {
                    let reference = read_reference(name)?;
                    docs.insert(doc.clone(), Indexed::new(&reference));
                }
            }
            for (at, _) in line.match_indices('§') {
                if let Some((doc, heading)) = candidates.iter().find_map(|doc| {
                    cited_heading(&docs[doc].texts, &line[at..]).map(|h| (doc, h.to_owned()))
                }) {
                    queue.push((doc.clone(), heading));
                }
            }
            for cap in bold_regex().captures_iter(line) {
                let inner = &cap[1];
                if inner.starts_with('§') {
                    continue;
                }
                if let Some(doc) = candidates
                    .iter()
                    .find(|doc| docs[*doc].texts.iter().any(|h| h == inner))
                {
                    queue.push((doc.clone(), inner.to_owned()));
                }
            }
        }
        Ok(())
    };

    cite(&Doc::Command, text, &mut docs, &mut queue)?;
    while let Some((doc, heading)) = queue.pop() {
        if !seen.insert((doc.clone(), heading.clone())) {
            continue;
        }
        let Some(body) = docs[&doc].section(&heading) else {
            continue;
        };
        // A fenced block or a comment in the section is content, not an
        // invocation, as in the command file.
        let mut skip = SkipScanner::default();
        tools.extend(
            body.lines()
                .filter(|line| !skip.skip(line))
                .flat_map(|line| invoked(&without_inline_comments(line))),
        );
        cite(&doc, &body, &mut docs, &mut queue)?;
    }
    Ok(tools)
}

/// `text`, a command copy after substitution, with a last paragraph added to
/// its `## Instructions` preamble blockquote that loads its run tools, less
/// `always_loaded`, in one Claude Code `ToolSearch`, asking for the command's
/// first `ductus` call to go in the same message (spec 080, AC29). The run
/// tools are its step tools and `cited`, the tools the command file or a
/// section it cites invokes ([`cited_tools`]). The names are sorted, as one `select:` query, and the
/// line takes the file's own line ending. A preamble is opened when the
/// section has none. `text` comes back unchanged when no tool remains.
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
    cited: &BTreeSet<&'static str>,
) -> Result<String, ParseError> {
    let mut run_tools = text_step_tools(text)?;
    run_tools.extend(cited);
    let remaining: Vec<&str> = run_tools.difference(always_loaded).copied().collect();
    if remaining.is_empty() {
        return Ok(text.to_owned());
    }
    let query = remaining
        .iter()
        .map(|name| registry::claude_code_tool_name(name))
        .collect::<Vec<_>>()
        .join(",");
    let eol = line_ending_of(text);
    let line = format!(
        "> **Claude Code:** load this command's `ductus` tools in one `ToolSearch` call, and send \
         the command's first `ductus` call in that same message, since a tool can be called beside \
         the search that loads it: `select:{query}`"
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

/// Each ATX heading outside a fenced code block and an HTML comment, as its
/// line index, its level and its text ([`SkipScanner`], the rule the
/// citations use).
fn atx_headings<'a>(lines: &[&'a str]) -> Vec<(usize, usize, &'a str)> {
    let mut headings = Vec::new();
    let mut skip = SkipScanner::default();
    for (index, line) in lines.iter().enumerate() {
        let content = line.trim_end_matches(['\n', '\r']);
        if skip.skip(content) {
            continue;
        }
        let trimmed = content.trim_start();
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
    /// The copy at `path` parsed, and its steps name `tools`, through
    /// [`text_step_tools`], as `apply-manifest` derives them.
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
    /// The host layout that locates the copy could not be resolved, because
    /// the project config or a session file does not read or parse, so no
    /// copy was looked for and no tool is flagged. A state of its own, never
    /// a default layout in its place (`CFG-ENV-008`): the server still starts,
    /// so `write-session` can repair a malformed session file, and every other
    /// primitive that reads the file halts naming it.
    HostUnresolved {
        /// The error, which names the file.
        reason: String,
    },
}

impl AlwaysLoaded {
    /// Find and derive the installed copy under `repo`.
    #[must_use]
    pub fn resolve(repo: &Path) -> Self {
        let host = match Host::load(repo) {
            Ok(host) => host,
            Err(err) => {
                return Self::HostUnresolved {
                    reason: err.to_string(),
                };
            }
        };
        let candidates = host.command_file_candidates(ALWAYS_LOADED_COMMAND);
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
            // The one derivation `apply-manifest` makes too, so a legacy-prose
            // copy flags what its line leaves out (spec 080's plan).
            return match text_step_tools(&source) {
                Ok(tools) => Self::Derived {
                    path: path.clone(),
                    tools,
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
            Self::NoCopy { .. } | Self::Unreadable { .. } | Self::HostUnresolved { .. } => {
                BTreeSet::new()
            }
        }
    }

    /// The line the server writes to stderr when it flags nothing, because
    /// it found no usable copy or one naming no tool, so that state is never
    /// silent. `None` once a copy names a tool.
    #[must_use]
    pub fn notice(&self) -> Option<String> {
        match self {
            Self::Derived { path, tools } if tools.is_empty() => Some(format!(
                "ductus: the installed `{ALWAYS_LOADED_COMMAND}` command at {path} names no \
                 tool in its steps, so no tool is flagged to load up front (spec 080)"
            )),
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
            Self::HostUnresolved { reason } => Some(format!(
                "ductus: the host layout could not be resolved ({reason}), so the installed \
                 `{ALWAYS_LOADED_COMMAND}` command was not looked for and no tool is flagged \
                 to load up front (spec 080)"
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
                        call, and send the command's first `ductus` call in that same message, \
                        since a tool can be called beside the search that loads it: ";

    #[test]
    fn the_line_closes_the_preamble_naming_the_tools_left_sorted() {
        let text = "# X\n\n## Instructions\n\n> **For agent runtimes**: a.\n>\n> b.\n\n\
                    1. Invoke `write-session`.\n2. Invoke `read-spec`.\n3. Then mark-task.\n";
        let out = add_tool_search_line(text, &BTreeSet::from(["write-session"]), &BTreeSet::new())
            .unwrap();
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
            add_tool_search_line(text, &BTreeSet::from(["write-session"]), &BTreeSet::new())
                .unwrap(),
            text
        );
        let prose = "# Help\n\nNo steps, though resolve-session is named.\n";
        assert_eq!(
            add_tool_search_line(prose, &BTreeSet::new(), &BTreeSet::new()).unwrap(),
            prose
        );
    }

    #[test]
    fn a_section_with_no_preamble_gets_one() {
        let text = "## Instructions\n\n1. Invoke `read-spec`.\n";
        assert_eq!(
            add_tool_search_line(text, &BTreeSet::new(), &BTreeSet::new()).unwrap(),
            format!(
                "## Instructions\n\n{LINE}`select:mcp__ductus__read-spec`\n\n1. Invoke `read-spec`.\n"
            )
        );
    }

    #[test]
    fn the_line_takes_the_files_line_ending() {
        let text = "## Instructions\r\n\r\n> a.\r\n\r\n1. Invoke `read-spec`.\r\n";
        let out = add_tool_search_line(text, &BTreeSet::new(), &BTreeSet::new()).unwrap();
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
        let out = add_tool_search_line(LEGACY, &BTreeSet::new(), &BTreeSet::new()).unwrap();
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
        let out = add_tool_search_line(text, &BTreeSet::new(), &BTreeSet::new()).unwrap();
        assert_eq!(
            out,
            format!(
                "# X\n\n```markdown\n## Instructions\n```\n\n## Instructions\n\n{LINE}\
                 `select:mcp__ductus__read-spec`\n\n1. Invoke `read-spec`.\n"
            )
        );
    }

    /// A heading inside an HTML comment is no heading either, as the
    /// citations read it.
    #[test]
    fn a_heading_inside_a_comment_is_not_the_section() {
        let text = "# X\n\n<!--\n## Instructions\n-->\n\n## Instructions\n\n\
                    1. Invoke `read-spec`.\n";
        let out = add_tool_search_line(text, &BTreeSet::new(), &BTreeSet::new()).unwrap();
        assert_eq!(
            out,
            format!(
                "# X\n\n<!--\n## Instructions\n-->\n\n## Instructions\n\n{LINE}\
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

    /// A session file that does not parse leaves the host layout unresolved:
    /// no copy is looked for, no tool is flagged, and the notice names the
    /// file, rather than a default layout standing in (`CFG-ENV-008`). The
    /// installed copy here would otherwise derive two tools.
    #[test]
    fn an_unresolved_host_flags_nothing_and_names_the_file() {
        let tmp = project(Some(
            "## Instructions\n\n1. Invoke `resolve-session`.\n2. Invoke `write-session`.\n",
        ));
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "cli-config-dir = [\n",
        )
        .unwrap();
        let found = AlwaysLoaded::resolve(tmp.path());
        let AlwaysLoaded::HostUnresolved { reason } = &found else {
            panic!("expected an unresolved host, got {found:?}");
        };
        assert!(reason.contains("session.toml"), "{reason}");
        assert!(found.tools().is_empty());
        let notice = found.notice().unwrap();
        assert!(
            notice.contains("could not be resolved") && notice.contains("session.toml"),
            "{notice}"
        );
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
            &Host::load(tmp.path())
                .unwrap()
                .command_file_candidates("target")
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
        let tmp = project(Some(
            "## Instructions\n\n1. Invoke `read-spec` and `write-session`.\n",
        ));
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
            notice.contains("names 2 primitives"),
            "the parse error is the reason: {notice}"
        );
    }

    /// A copy whose steps name no tool flags none, and says so, so an empty
    /// derivation is never silent.
    #[test]
    fn a_copy_naming_no_tool_flags_nothing_and_says_so() {
        let tmp = project(Some("# Target\n\nNo steps here.\n"));
        let found = AlwaysLoaded::resolve(tmp.path());
        assert!(found.tools().is_empty(), "{found:?}");
        let notice = found.notice().unwrap();
        assert!(
            notice.contains("at .claude/commands/acme/target.md names no tool"),
            "{notice}"
        );
    }

    /// A legacy-prose copy gives the server the tools `apply-manifest`
    /// leaves out of each line, since both derive through one function.
    #[test]
    fn a_legacy_prose_copy_derives_as_apply_manifest_derives_it() {
        let tmp = project(Some(LEGACY));
        let found = AlwaysLoaded::resolve(tmp.path());
        let expected = text_step_tools(LEGACY).unwrap();
        assert!(!expected.is_empty());
        assert_eq!(found.tools(), expected);
        assert_eq!(found.notice(), None);
    }

    /// `cited_tools` over `command`, as command `x`, reading references from
    /// `refs` by name and failing the test on any other.
    fn cited(command: &str, refs: &[(&str, &str)]) -> Vec<&'static str> {
        cited_tools(command, "x", |name: &str| -> Result<String, String> {
            refs.iter()
                .find(|(n, _)| *n == name)
                .map(|(_, text)| (*text).to_owned())
                .ok_or_else(|| format!("no reference {name}"))
        })
        .unwrap()
        .into_iter()
        .collect()
    }

    /// Names `.ductus/reference/x.md` as the command's reference.
    const OWN: &str = "This command's reference is `.ductus/reference/x.md`.\n\n";

    /// Scenario `one-search-covers-referenced-sections`: a section a step
    /// cites adds the tool it invokes, and not one it only names.
    #[test]
    fn a_cited_section_adds_the_tools_it_invokes_and_not_those_it_names() {
        let command = format!(
            "{OWN}## Instructions\n\n1. Run **§Validation gate** in `.ductus/reference/x.md`.\n"
        );
        let reference = "## Validation gate\n\nInvoke `derive-boundary` to evaluate it. \
                         Then `write-review` records a later run.\n\n## Other\n\n\
                         Invoke `mark-task`.\n";
        assert_eq!(
            cited(&command, &[("x", reference)]),
            vec!["derive-boundary"],
            "the cited section's invocation counts, its mention and an uncited section do not"
        );
    }

    /// An invocation inside a fenced block or an HTML comment of a cited
    /// section is content, so it adds nothing.
    #[test]
    fn a_cited_sections_fenced_or_commented_invocation_adds_nothing() {
        let command = format!(
            "{OWN}## Instructions\n\n1. Run **§Validation gate** in `.ductus/reference/x.md`.\n"
        );
        let reference = "## Validation gate\n\nInvoke `derive-boundary`.\n\n```text\n\
                         Invoke `mark-task`.\n```\n\n<!--\nInvoke `append-task`.\n-->\n";
        assert_eq!(
            cited(&command, &[("x", reference)]),
            vec!["derive-boundary"]
        );
    }

    /// A comment that opens and closes on one line is content too, whether it
    /// holds an invocation in the command file or in a cited section, or a
    /// citation: none joins the line (plan *Batched (AC29)*: "outside fenced
    /// blocks and HTML comments").
    #[test]
    fn a_single_line_comments_invocation_or_citation_adds_nothing() {
        let command = format!(
            "{OWN}<!-- Invoke `append-task` here later. -->\n\n## Instructions\n\n\
             1. Run **§Validation gate** in `.ductus/reference/x.md`. <!-- and §Other -->\n"
        );
        let reference = "## Validation gate\n\nInvoke `derive-boundary`. <!-- invoke `mark-task` -->\n\n\
                         ## Other\n\nInvoke `label-criteria`.\n";
        assert_eq!(
            cited(&command, &[("x", reference)]),
            vec!["derive-boundary"]
        );
    }

    /// A bold span equal to a heading cites that section, with no `§`, and
    /// the walk reads citations anywhere in the file, not only in the steps.
    #[test]
    fn a_bold_heading_cites_a_section() {
        let command = "Under `--project`, work **Project files** in `.ductus/reference/x.md`.\n\n\
                       ## Instructions\n\n1. Run prune-tasks.\n";
        let reference = "## Project files\n\n- Invoke `prune-project` with no `apply`.\n";
        assert_eq!(cited(command, &[("x", reference)]), vec!["prune-project"]);
    }

    /// Scenario `reference-reads-follow-the-cited-sections`: a name the
    /// command file invokes outside its steps and outside any cited section,
    /// as the "Where a named section lives" paragraph invokes `read-section`,
    /// joins the line; one it only names does not.
    #[test]
    fn a_name_invoked_outside_the_steps_counts() {
        let command = "## Instructions\n\n> a.\n\n**Where a named section lives.** Invoke \
                       `read-section` with the reference and each section a step names; \
                       `write-review` records a later run.\n\n1. Run mark-task.\n";
        assert_eq!(cited(command, &[]), vec!["read-section"]);
    }

    /// A line naming a reference resolves only against it, even where the
    /// command's own reference has a heading of the same name.
    #[test]
    fn a_line_naming_a_reference_resolves_only_against_it() {
        let command = format!(
            "{OWN}## Instructions\n\n1. Walk **§Groom decision tree** in \
             `.ductus/reference/groom.md`.\n"
        );
        let own = "## Groom decision tree\n\nInvoke `mark-task`.\n";
        let groom = "## Groom decision tree\n\nInvoke `remove-inbox-item`.\n";
        assert_eq!(
            cited(&command, &[("x", own), ("groom", groom)]),
            vec!["remove-inbox-item"]
        );
    }

    /// A cited section's own citations are followed, against its own
    /// document first, and a heading is matched longest first.
    #[test]
    fn citations_are_followed_and_matched_longest_first() {
        let command = format!("{OWN}## Instructions\n\n1. Follow §Walk through tasks.\n");
        let reference = "## Walk through tasks\n\nEach finding follows §Setup details.\n\n\
                         ## Setup\n\nInvoke `mark-task`.\n\n\
                         ## Setup details\n\nInvoke `append-task`.\n";
        assert_eq!(cited(&command, &[("x", reference)]), vec!["append-task"]);
    }

    /// With no `.ductus/reference/<command>.md` in its text, a command has no
    /// own reference, so a bare `§` never reads one; and a citation inside a
    /// fenced block cites nothing.
    #[test]
    fn no_named_reference_is_read_and_a_fenced_citation_cites_nothing() {
        let command = "## Instructions\n\n1. Follow §Gate.\n\n```text\n**§Fenced** in \
                       `.ductus/reference/x.md`\n```\n";
        assert_eq!(cited(command, &[]), Vec::<&str>::new());
    }

    /// A named reference that cannot be read is an error, never a line that
    /// lacks its tools.
    #[test]
    fn a_named_reference_that_cannot_be_read_is_an_error() {
        let command = "## Instructions\n\n1. Follow **§Gate** in `.ductus/reference/gone.md`.\n";
        let err = cited_tools(command, "x", |name: &str| -> Result<String, String> {
            Err(format!("no reference {name}"))
        })
        .unwrap_err();
        assert_eq!(err, "no reference gone");
    }

    /// The line names the cited tools beside the step tools.
    #[test]
    fn the_line_names_the_cited_tools_beside_the_step_tools() {
        let text = "## Instructions\n\n> a.\n\n1. Invoke `read-spec`.\n";
        let out =
            add_tool_search_line(text, &BTreeSet::new(), &BTreeSet::from(["derive-boundary"]))
                .unwrap();
        assert!(
            out.contains("`select:mcp__ductus__derive-boundary,mcp__ductus__read-spec`"),
            "{out}"
        );
    }
}
