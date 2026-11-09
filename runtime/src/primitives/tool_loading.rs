//! Which `ductus` tools a slash command's steps name (spec 080). The MCP
//! server flags the installed `target` command's tools to load up front
//! ([`AlwaysLoaded`]), and every other command loads its own in one search.
//! Both read the same derivation, [`step_tools`].

use std::collections::BTreeSet;
use std::path::Path;

use crate::host::Host;
use crate::parser;
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
