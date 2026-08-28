//! `check-command-flags` — every flag a command's Flags table documents also
//! appears in that command's `argument-hint:` frontmatter.
//!
//! `argument-hint` is the surface a host renders when it offers the command,
//! so a flag absent from it is a flag the operator is never shown. An adopter
//! hit exactly that: `--since` was documented in `review.md`'s Flags table and
//! accepted by `compute-review-scope` as `since`, yet reported as "doesn't
//! show as an option" — the hint read `[--all] [--fix] [feature]` while the
//! table listed eight entries.
//!
//! Measured against that state: 6 findings — `--security`, `--simplicity`,
//! `--quality`, `--since`, `--waive`, and `--reason`, the last because the
//! waiver row names both halves of the pair. 0 once the hint was corrected.
//!
//! ## Why the subject is `framework/commands/`
//!
//! The sources, not the generated copies under a host's commands directory.
//! A generated copy carries whatever its source carries, so checking both
//! would report every finding twice and neither copy would be the one to fix.
//! More to the point, an adopter told their installed `review.md` disagrees
//! with itself cannot act on it — the file is regenerated from ductus and the
//! repair is a ductus release. The divergence originates here, so it is caught
//! here, before it ships.
//!
//! ## Scope, narrowly
//!
//! Only a `Flags` **section** counts, and only its table rows. A command that
//! documents a flag in prose is not a subject: `implement.md` describes
//! `--auto` under a `### Flags` heading with no table, and its hint names it —
//! correct, and invisible to a table-shaped check. Widening to "any `--x` in
//! any body" would report every prose mention of another command's flags.
//!
//! That narrowing is why an empty `findings` means *every tabled flag is
//! surfaced*, never *every documented flag is surfaced*. [`examined`] and
//! [`with_flags_table`] are what let a caller say which of those it is
//! (`QUAL-CLAIM-001`).
//!
//! [`examined`]: crate::schema::primitives::CheckCommandFlagsResult::examined
//! [`with_flags_table`]: crate::schema::primitives::CheckCommandFlagsResult::with_flags_table
//!
//! Section membership comes from [`super::section_line_indices`], the shared
//! fence- and comment-aware scanner, rather than a second heading walk here:
//! these command bodies embed example output and artifact fragments, and a
//! table row inside a fence is an illustration, not the command's contract.

use std::path::Path;

use crate::primitives::{Result, read_text, rel_path, section_line_indices};
use crate::schema::primitives::{
    CheckCommandFlagsArgs, CheckCommandFlagsResult, CommandFlagFinding, CommandFlagSkip,
};

/// Directory holding the command sources this check reads.
const COMMANDS_DIR: &str = "framework/commands";

/// Execute the `check-command-flags` primitive.
///
/// # Errors
///
/// Never returns an error for an unreadable command file — that is a
/// [`CommandFlagSkip`], so an empty `findings` is not mistaken for a verified
/// tree. Returns [`super::PrimitiveError::Io`] only when the directory listing
/// itself fails.
pub fn run(_args: &CheckCommandFlagsArgs, repo: &Path) -> Result<CheckCommandFlagsResult> {
    let dir = repo.join(COMMANDS_DIR);

    let mut examined = Vec::new();
    let mut with_flags_table = Vec::new();
    let mut skipped = Vec::new();
    let mut findings = Vec::new();

    let mut paths: Vec<_> = match std::fs::read_dir(&dir) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
            .collect(),
        Err(err) => {
            // The subject does not exist. Reported as a skip rather than an
            // error: a caller running outside the ductus repo gets a result
            // that says "examined nothing", which is the truth.
            skipped.push(CommandFlagSkip {
                path: COMMANDS_DIR.to_string(),
                reason: format!("command source directory could not be listed: {err}"),
            });
            return Ok(CheckCommandFlagsResult {
                findings,
                examined,
                with_flags_table,
                skipped,
                commands_dir: COMMANDS_DIR.to_string(),
                guidance: String::new(),
            });
        }
    };
    paths.sort();

    for path in &paths {
        let rel = rel_path(path, repo);
        let content = match read_text(path) {
            Ok(text) => text,
            Err(err) => {
                skipped.push(CommandFlagSkip {
                    path: rel,
                    reason: format!("could not be read: {err}"),
                });
                continue;
            }
        };
        examined.push(rel.clone());

        let flags = tabled_flags(&content);
        if flags.is_empty() {
            continue;
        }
        with_flags_table.push(rel.clone());

        let Some(hint) = argument_hint(&content, path) else {
            findings.push(CommandFlagFinding {
                command: rel,
                flag: String::new(),
                reason:
                    "documents a Flags table but declares no argument-hint, so no flag is surfaced"
                        .to_string(),
            });
            continue;
        };

        for flag in flags {
            if !hint_names(&hint, &flag) {
                findings.push(CommandFlagFinding {
                    command: rel.clone(),
                    reason: format!(
                        "Flags table documents {flag} but argument-hint omits it, so it is never surfaced"
                    ),
                    flag,
                });
            }
        }
    }

    // An empty derivation over a non-empty subject means the extraction broke,
    // not that the corpus is clean. Say so rather than returning the payload of
    // a clean run.
    let guidance = if with_flags_table.is_empty() && !examined.is_empty() {
        format!(
            "no Flags table found in any of {} command file(s) — treat this as an extraction failure, not a clean result",
            examined.len()
        )
    } else {
        String::new()
    };

    Ok(CheckCommandFlagsResult {
        findings,
        examined,
        with_flags_table,
        skipped,
        commands_dir: COMMANDS_DIR.to_string(),
        guidance,
    })
}

/// The `argument-hint:` value from the leading frontmatter block, with
/// surrounding quotes stripped.
///
/// Where the frontmatter block ends is [`super::split_frontmatter`]'s
/// question, not this function's — it already handles the CRLF opener and the
/// empty-block (`---\n---\n`) case, and a second definition here would be a
/// second place for that boundary to drift. A file with no frontmatter is
/// `None` rather than an error, which is why the result is discarded with
/// `.ok()`: an absent block is a legitimate state for a command file, not a
/// failure to report.
///
/// Scans the frontmatter only. An `argument-hint:` line in the body is prose
/// about the field — several command files discuss it — and is not the
/// declaration a host reads.
fn argument_hint(content: &str, path: &Path) -> Option<String> {
    let (frontmatter, _body) = super::split_frontmatter(content, path).ok()?;
    for raw in frontmatter.lines() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if let Some(value) = line.strip_prefix("argument-hint:") {
            let trimmed = value.trim();
            let unquoted = trimmed
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .or_else(|| {
                    trimmed
                        .strip_prefix('\'')
                        .and_then(|v| v.strip_suffix('\''))
                })
                .unwrap_or(trimmed);
            return Some(unquoted.to_string());
        }
    }
    None
}

/// Every distinct `--flag` named in the first cell of a table row inside a
/// `Flags` section, in first-appearance order.
///
/// The first cell only: later cells are the behavior prose, which routinely
/// names other flags (`Composes with all other flags`, cross-references to
/// `--waive`) and would manufacture findings against whichever row mentioned
/// one. A single row may still name more than one flag — `--waive <rule-id>
/// --reason "<text>"` is one row and two flags — so the cell is scanned rather
/// than matched once.
fn tabled_flags(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut out: Vec<String> = Vec::new();
    for idx in section_line_indices(&lines, "Flags") {
        let line = lines[idx].trim_start();
        if !line.starts_with('|') {
            continue;
        }
        let cell = line
            .trim_start_matches('|')
            .split('|')
            .next()
            .unwrap_or_default();
        for flag in flags_in(cell) {
            if !out.contains(&flag) {
                out.push(flag);
            }
        }
    }
    out
}

/// Every `--flag` token in `text`: two hyphens, an ASCII lowercase letter,
/// then lowercase letters, digits, and hyphens.
///
/// The value form (`--since=<ref>`) stops at the `=`, so the token compared
/// against the hint is the flag name and a hint spelling the value
/// differently is not a false finding.
fn flags_in(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 2 < bytes.len() {
        if bytes[i] == b'-' && bytes[i + 1] == b'-' && bytes[i + 2].is_ascii_lowercase() {
            // A `---` run is a rule or fence, never a flag.
            if i > 0 && bytes[i - 1] == b'-' {
                i += 1;
                continue;
            }
            let start = i;
            i += 2;
            while i < bytes.len()
                && (bytes[i].is_ascii_lowercase() || bytes[i].is_ascii_digit() || bytes[i] == b'-')
            {
                i += 1;
            }
            out.push(text[start..i].to_string());
            continue;
        }
        i += 1;
    }
    out
}

/// Whether `hint` names `flag` as a whole token.
///
/// Word-boundary matching in both directions: `--since` must not be satisfied
/// by `--since-ish`, and `--all` must not be satisfied by `--install-all`. A
/// bare substring test passes both, which would report a hint as complete
/// while the flag it names is a different one.
fn hint_names(hint: &str, flag: &str) -> bool {
    let mut rest = hint;
    while let Some(pos) = rest.find(flag) {
        let before = rest[..pos].chars().next_back();
        let after = rest[pos + flag.len()..].chars().next();
        let boundary_before = before.is_none_or(|c| !c.is_ascii_lowercase() && c != '-');
        let boundary_after = after.is_none_or(|c| !c.is_ascii_lowercase() && c != '-');
        if boundary_before && boundary_after {
            return true;
        }
        rest = &rest[pos + flag.len()..];
    }
    false
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn command(hint: Option<&str>, body: &str) -> String {
        let hint_line = hint.map_or(String::new(), |h| format!("argument-hint: \"{h}\"\n"));
        format!("---\ndescription: x\n{hint_line}---\n\n{body}\n")
    }

    #[test]
    fn reads_the_hint_from_frontmatter_only() {
        let content = command(Some("[--all] [feature]"), "argument-hint: not frontmatter");
        assert_eq!(
            argument_hint(&content, Path::new("review.md")).unwrap(),
            "[--all] [feature]"
        );
    }

    #[test]
    fn a_file_without_frontmatter_has_no_hint() {
        assert!(
            argument_hint(
                "# Title\n\nargument-hint: \"[--x]\"\n",
                Path::new("review.md")
            )
            .is_none()
        );
    }

    #[test]
    fn an_empty_frontmatter_block_yields_no_hint_without_erroring() {
        // `---\n---\n` is the case a hand-rolled scan gets wrong: the closing
        // fence is the very next line. split_frontmatter handles it, so this
        // is None (no hint declared), not a swallowed error.
        assert!(argument_hint("---\n---\n\n# Title\n", Path::new("x.md")).is_none());
    }

    #[test]
    fn a_crlf_frontmatter_opener_is_read() {
        let content = "---\r\ndescription: x\r\nargument-hint: \"[--all]\"\r\n---\r\n\r\nbody\r\n";
        assert_eq!(
            argument_hint(content, Path::new("x.md")).unwrap(),
            "[--all]"
        );
    }

    #[test]
    fn harvests_every_flag_from_a_flags_table() {
        let content = command(
            Some("[--all]"),
            "## Flags\n\n| Flag | Behavior |\n| --- | --- |\n\
             | `--all` | Review everything |\n\
             | `--since=<ref>` | Override the diff base |\n",
        );
        assert_eq!(tabled_flags(&content), vec!["--all", "--since"]);
    }

    #[test]
    fn one_row_may_name_two_flags() {
        let content = command(
            Some("[--waive]"),
            "## Flags\n\n| Flag | Behavior |\n| --- | --- |\n\
             | `--waive <rule-id> --reason \"<text>\"` | Record a waiver |\n",
        );
        assert_eq!(tabled_flags(&content), vec!["--waive", "--reason"]);
    }

    #[test]
    fn only_the_first_cell_is_scanned() {
        // The behavior column routinely names other flags; harvesting it
        // would report a finding against whichever row mentioned one.
        let content = command(
            Some("[--all]"),
            "## Flags\n\n| Flag | Behavior |\n| --- | --- |\n\
             | `--all` | Composes with `--fix` and `--since` |\n",
        );
        assert_eq!(tabled_flags(&content), vec!["--all"]);
    }

    #[test]
    fn a_flags_section_without_a_table_yields_nothing() {
        // implement.md's shape: prose under `### Flags`, no table.
        let content = command(
            Some("[--auto] [feature]"),
            "### Flags\n\n`$ARGUMENTS` may include the `--auto` flag in any position.\n",
        );
        assert!(tabled_flags(&content).is_empty());
    }

    #[test]
    fn a_table_outside_the_flags_section_is_not_a_subject() {
        let content = command(
            Some("[--all]"),
            "## Inputs\n\n| Field | Note |\n| --- | --- |\n| `--nope` | not a flag table |\n",
        );
        assert!(tabled_flags(&content).is_empty());
    }

    #[test]
    fn a_fenced_example_table_is_skipped() {
        let content = command(
            Some("[--all]"),
            "## Flags\n\n| Flag | Behavior |\n| --- | --- |\n| `--all` | real |\n\n\
             ```text\n| `--example` | illustration |\n```\n",
        );
        assert_eq!(tabled_flags(&content), vec!["--all"]);
    }

    #[test]
    fn the_section_ends_at_a_sibling_heading() {
        let content = command(
            Some("[--all]"),
            "## Flags\n\n| Flag | Behavior |\n| --- | --- |\n| `--all` | real |\n\n\
             ## Pipeline position\n\n| Flag | Behavior |\n| `--after` | not a flag |\n",
        );
        assert_eq!(tabled_flags(&content), vec!["--all"]);
    }

    #[test]
    fn the_none_row_contributes_no_flag() {
        let content = command(
            Some("[--all]"),
            "## Flags\n\n| Flag | Behavior |\n| --- | --- |\n\
             | _(none)_ | Review the current target |\n| `--all` | Everything |\n",
        );
        assert_eq!(tabled_flags(&content), vec!["--all"]);
    }

    #[test]
    fn the_separator_row_contributes_no_flag() {
        // `| --- | --- |` is three hyphens, not a `--f` flag.
        assert!(flags_in(" --- ").is_empty());
    }

    #[test]
    fn hint_matching_is_whole_token() {
        assert!(hint_names("[--all] [--fix]", "--all"));
        assert!(hint_names("[--since=<ref>]", "--since"));
        assert!(hint_names("[--a|--b]", "--b"));
        // Prefix and suffix collisions must not count as present.
        assert!(!hint_names("[--since-ish]", "--since"));
        assert!(!hint_names("[--install-all]", "--all"));
        assert!(!hint_names("[--fix]", "--f"));
    }

    #[test]
    fn a_repeated_flag_is_reported_once() {
        let content = command(
            Some("[--all]"),
            "## Flags\n\n| `--all` | a |\n| `--all` | again |\n",
        );
        assert_eq!(tabled_flags(&content), vec!["--all"]);
    }
}
