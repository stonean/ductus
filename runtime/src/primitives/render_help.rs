//! `render-help` — the guide `/{project}:help` prints, read from the installed
//! help copy, so a typed help is answered with no model turn: by
//! `ductus hook help` in Claude Code and by the bridge's registered command in
//! Pi (spec 080, scenario `help-answers-with-no-model-turn`). What it returns is
//! `schema::descriptions::RENDER_HELP`.
//!
//! The guide comes from the installed copy, not from the framework source: the
//! crate is published from `runtime/` alone, so a build cannot embed
//! `framework/commands/help.md`, and reading the copy follows a pinned or edited
//! `help.md` and carries the project's name in each command, as the copy the
//! model path reads does.

use std::path::Path;

use crate::host::Host;
use crate::primitives::{PrimitiveError, Result};
use crate::schema::primitives::{RenderHelpArgs, RenderHelpResult};

/// The command whose installed copy holds the guide.
const HELP_COMMAND: &str = "help";

/// The heading the guide follows.
const INSTRUCTIONS_HEADING: &str = "## Instructions";

/// The line that opens and closes the guide.
const RULE: &str = "---";

/// Execute the `render-help` primitive: the guide, and the namespace Pi's
/// bridge names its commands from, which reads no spec, so a view that
/// cannot be rendered does not stop the bridge's registration (spec 080's
/// plan, *Pi registers the command in the bridge*).
///
/// # Errors
///
/// Every error [`guide`] returns.
pub fn run(_args: &RenderHelpArgs, repo: &Path) -> Result<RenderHelpResult> {
    Ok(RenderHelpResult {
        guide: guide(repo)?,
        project: Host::load(repo).project,
    })
}

/// The guide in the first installed help copy
/// [`Host::command_file_candidates`] finds under `repo`.
///
/// # Errors
///
/// - [`PrimitiveError::CommandCopyNotFound`] when no candidate holds a copy.
/// - [`PrimitiveError::Io`] when a copy exists but cannot be read.
/// - [`PrimitiveError::MalformedCommandCopy`] when the copy holds no guide.
pub fn guide(repo: &Path) -> Result<String> {
    let candidates = Host::load(repo).command_file_candidates(HELP_COMMAND);
    for path in &candidates {
        let full = repo.join(path);
        let source = match std::fs::read_to_string(&full) {
            Ok(source) => source,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(source) => return Err(PrimitiveError::Io { path: full, source }),
        };
        return extract(&source).ok_or_else(|| PrimitiveError::MalformedCommandCopy {
            path: path.clone(),
            reason: format!(
                "no guide: fewer than two `{RULE}` lines after its `{INSTRUCTIONS_HEADING}` heading"
            ),
        });
    }
    Err(PrimitiveError::CommandCopyNotFound {
        command: HELP_COMMAND.to_owned(),
        tried: candidates.join(", "),
    })
}

/// The guide in a help copy's text: the lines between the first line after its
/// `## Instructions` heading that is exactly `---` and the last such line. Each
/// line that is wholly an HTML comment, as the generator's table markers are,
/// is removed, together with a blank line the removal would leave beside
/// another, and the leading and trailing blank lines are trimmed. `None` when
/// the copy has no such heading or fewer than two such lines after it.
fn extract(source: &str) -> Option<String> {
    let lines: Vec<&str> = source.lines().collect();
    let heading = lines
        .iter()
        .position(|line| line.trim_end() == INSTRUCTIONS_HEADING)?;
    let rules: Vec<usize> = (heading + 1..lines.len())
        .filter(|&index| lines[index].trim_end() == RULE)
        .collect();
    let (&open, &close) = (rules.first()?, rules.last()?);
    if open == close {
        return None;
    }
    let mut kept: Vec<&str> = Vec::new();
    let mut skip_blank = false;
    for &line in &lines[open + 1..close] {
        let trimmed = line.trim();
        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") {
            skip_blank = kept.last().is_none_or(|last| last.trim().is_empty());
            continue;
        }
        if skip_blank && trimmed.is_empty() {
            skip_blank = false;
            continue;
        }
        skip_blank = false;
        kept.push(line);
    }
    let start = kept.iter().position(|line| !line.trim().is_empty())?;
    let end = kept.iter().rposition(|line| !line.trim().is_empty())?;
    Some(kept[start..=end].join("\n"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::fs;

    use super::*;

    const COPY: &str = "\
---
description: Display an overview.
---

# Help

## Purpose

Printed verbatim.

## Instructions

Print the following guide exactly:

---

## acme — Pipeline

### Commands

<!-- generated:commands-orient:start -->

| Command | Description |
| --- | --- |
| `/acme:help` | Display an overview. |

<!-- generated:commands-orient:end -->

### Typical Session

```text
/acme:status


/acme:target 000
```

---
";

    /// The guide is the text between the rules after `## Instructions`: the
    /// frontmatter's `---` lines are not among them, the table markers go with
    /// the blank line each would leave doubled, and a blank run inside a fence
    /// stays as written.
    #[test]
    fn the_guide_is_the_text_between_the_rules_without_its_comment_lines() {
        assert_eq!(
            extract(COPY).unwrap(),
            "\
## acme — Pipeline

### Commands

| Command | Description |
| --- | --- |
| `/acme:help` | Display an overview. |

### Typical Session

```text
/acme:status


/acme:target 000
```"
        );
    }

    /// A copy whose rules are missing, single, or above the heading holds no
    /// guide.
    #[test]
    fn a_copy_without_two_rules_after_the_heading_holds_no_guide() {
        for source in [
            "---\nx: y\n---\n\n## Instructions\n\nPrint it.\n",
            "## Instructions\n\n---\n\nguide\n",
            "---\n\nguide\n\n---\n\n## Instructions\n\nPrint it.\n",
            "## Purpose\n\n---\n\nguide\n\n---\n",
        ] {
            assert_eq!(extract(source), None, "{source:?}");
        }
    }

    fn project(cli_config_dir: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[host]\nproject = \"acme\"\n",
        )
        .unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            format!("cli-config-dir = \"{cli_config_dir}\"\n"),
        )
        .unwrap();
        tmp
    }

    /// The guide is read from the copy the recorded agent's layout installs:
    /// Claude Code's command directory, or Pi's prompts.
    #[test]
    fn the_guide_is_read_from_the_installed_copy_of_each_layout() {
        for (dir, copy) in [
            (".claude", ".claude/commands/acme/help.md"),
            (".pi", ".pi/prompts/acme-help.md"),
        ] {
            let tmp = project(dir);
            let path = tmp.path().join(copy);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, COPY).unwrap();
            let result = run(&RenderHelpArgs {}, tmp.path()).unwrap();
            assert!(result.guide.starts_with("## acme — Pipeline"), "{copy}");
            assert_eq!(
                result.project, "acme",
                "the namespace Pi's bridge names its commands from"
            );
        }
    }

    /// The namespace comes without reading a spec, so a spec whose frontmatter
    /// does not parse, which fails the view, leaves the result whole (spec
    /// 080's plan, *Pi registers the command in the bridge*).
    #[test]
    fn a_spec_that_does_not_parse_leaves_the_namespace_readable() {
        let tmp = project(".pi");
        let path = tmp.path().join(".pi/prompts/acme-help.md");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, COPY).unwrap();
        let spec = tmp.path().join("specs/001-broken/spec.md");
        fs::create_dir_all(spec.parent().unwrap()).unwrap();
        fs::write(&spec, "---\nstatus: [\n---\n").unwrap();
        let args = crate::schema::primitives::DashboardArgs::default();
        assert!(crate::primitives::dashboard::run(&args, tmp.path()).is_err());
        assert_eq!(run(&RenderHelpArgs {}, tmp.path()).unwrap().project, "acme");
    }

    /// No installed copy is an error naming every path tried, never an empty
    /// guide.
    #[test]
    fn no_installed_copy_is_an_error_naming_the_paths_tried() {
        let tmp = project(".claude");
        let err = guide(tmp.path()).unwrap_err();
        let PrimitiveError::CommandCopyNotFound { command, tried } = &err else {
            panic!("{err:?}");
        };
        assert_eq!(command, "help");
        assert!(tried.contains(".claude/commands/acme/help.md"), "{tried}");
        assert!(tried.contains(".claude/prompts/acme-help.md"), "{tried}");
    }

    /// A copy that holds no guide is an error naming the copy.
    #[test]
    fn a_copy_holding_no_guide_is_an_error_naming_it() {
        let tmp = project(".claude");
        let path = tmp.path().join(".claude/commands/acme/help.md");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "## Instructions\n\nPrint nothing.\n").unwrap();
        let err = guide(tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::MalformedCommandCopy { path, .. } if path == ".claude/commands/acme/help.md"),
            "{err:?}"
        );
    }

    /// The guide this repository installs for Claude Code, from the source the
    /// generator copies, is non-empty and carries no generator marker.
    #[test]
    fn the_shipped_help_source_holds_a_guide() {
        let source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/commands/help.md"),
        )
        .unwrap();
        let guide = extract(&source).unwrap();
        assert!(guide.starts_with("## {project} — "), "{guide}");
        assert!(
            guide.ends_with("Depth-first keeps context focused."),
            "{guide}"
        );
        assert!(!guide.contains("<!--"), "{guide}");
    }
}
