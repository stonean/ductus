//! `check-command-flags` — a command's declared interface agrees with the
//! body a host substitutes into. Two directions, checked in one pass: every
//! flag a `Flags` table documents appears in the `argument-hint:` frontmatter
//! (direction 1), and a command that declares an `argument-hint` carries a
//! substitution token in its body (direction 3) — without one the host drops
//! the argument and the declared interface is unreachable.
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
//! Direction 3 is the converse defect, and it is silent in the other
//! direction: a host injects the invocation's arguments through substitution
//! alone (Pi's `substituteArgs` has no fallback that appends an unreferenced
//! argument), so a command whose declared hint outruns its body reads its
//! no-argument branch however the operator invokes it. `target`, `link`, and
//! `prune` each carried a hint and no token. The subject is every examined
//! command that declares a hint, whether or not it has a `Flags` table;
//! [`with_argument_hint`] is that direction's denominator.
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
//! [`with_argument_hint`]: crate::schema::primitives::CheckCommandFlagsResult::with_argument_hint
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
    let mut with_argument_hint = Vec::new();
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
                with_argument_hint,
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
        check_one(
            &rel,
            &content,
            path,
            &mut findings,
            &mut with_flags_table,
            &mut with_argument_hint,
        );
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
        with_argument_hint,
        skipped,
        commands_dir: COMMANDS_DIR.to_string(),
        guidance,
    })
}

/// The two directions for one command file, appending each result to the
/// run's accumulators.
///
/// Split out of [`run`] so the loop stays a loop. The two directions are
/// genuinely separate checks that happen to share a file, and extracting them
/// is the right answer where a `#[allow(clippy::too_many_lines)]` would have
/// hidden a body that had outgrown one screen.
fn check_one(
    rel: &str,
    content: &str,
    path: &Path,
    findings: &mut Vec<CommandFlagFinding>,
    with_flags_table: &mut Vec<String>,
    with_argument_hint: &mut Vec<String>,
) {
    let declared = hint_and_body(content, path);
    let hint = declared.as_ref().map(|(hint, _)| hint.clone());

    // Direction 3: a declared interface the host cannot reach. The argument
    // arrives through substitution alone, so a hint with no token in the body
    // is an interface that silently discards its argument. The subject is any
    // command declaring a hint, table or not, so this runs before the
    // Flags-table narrowing below.
    if let Some((_, body)) = declared.as_ref() {
        with_argument_hint.push(rel.to_string());
        if !has_substitution_token(body) {
            findings.push(CommandFlagFinding {
                command: rel.to_string(),
                direction: "hint-unreachable".to_string(),
                flag: String::new(),
                reason: "declares an argument-hint but its body carries no substitution token ($ARGUMENTS, $@, $1, ${N:-\u{2026}}, ${@:-\u{2026}}, ${@:N}, ${@:N:L}), so the host silently discards the argument".to_string(),
            });
        }
    }

    // Direction 1: a tabled flag the hint omits.
    let flags = tabled_flags(content);
    if flags.is_empty() {
        return;
    }
    with_flags_table.push(rel.to_string());

    let Some(hint) = hint else {
        findings.push(CommandFlagFinding {
            command: rel.to_string(),
            direction: "flag-unsurfaced".to_string(),
            flag: String::new(),
            reason: "documents a Flags table but declares no argument-hint, so no flag is surfaced"
                .to_string(),
        });
        return;
    };

    for flag in flags {
        if !hint_names(&hint, &flag) {
            findings.push(CommandFlagFinding {
                command: rel.to_string(),
                direction: "flag-unsurfaced".to_string(),
                reason: format!(
                    "Flags table documents {flag} but argument-hint omits it, so it is never surfaced"
                ),
                flag,
            });
        }
    }
}

/// The `argument-hint:` value from the leading frontmatter block, with
/// surrounding quotes stripped, and the body after that block — the text a
/// host substitutes into, and so the subject of the token direction.
///
/// Where the frontmatter block ends is [`super::split_frontmatter`]'s
/// question, not this function's — it already handles the CRLF opener and the
/// empty-block (`---\n---\n`) case, and a second definition here would be a
/// second place for that boundary to drift. A file with no frontmatter is
/// `None` rather than an error, which is why the result is discarded with
/// `.ok()`: a command file that declares no hint is not a subject for either
/// direction, rather than a failure to report.
///
/// Scans the frontmatter only. An `argument-hint:` line in the body is prose
/// about the field — several command files discuss it — and is not the
/// declaration a host reads.
fn hint_and_body(content: &str, path: &Path) -> Option<(String, String)> {
    let (frontmatter, body) = super::split_frontmatter(content, path).ok()?;
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
            return Some((unquoted.to_string(), body.to_string()));
        }
    }
    None
}

/// Whether `text` carries a prompt-template substitution token a host
/// expands — `$ARGUMENTS`, `$@`, `$<digit>`, or a `${…}` whose first
/// character is `@` or a digit (`${1:-default}`, `${@:2}`, `${@:2:3}`).
///
/// Deliberately permissive about the braced form's tail, because the token
/// set is the union of what the supported hosts expand and the check must not
/// report a command that legitimately uses `$1` or `${@:N}`. A token the
/// scanner misses makes the check silent, never falsely loud — the safe
/// direction for a gate, and the reason the scan is a character walk rather
/// than a list of complete spellings.
fn has_substitution_token(text: &str) -> bool {
    if text.contains("$ARGUMENTS") {
        return true;
    }
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' {
            let next = bytes[i + 1];
            if next == b'@' || next.is_ascii_digit() {
                return true;
            }
            if next == b'{' && i + 2 < bytes.len() {
                let first = bytes[i + 2];
                if first == b'@' || first.is_ascii_digit() {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
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

    fn hint(content: &str, path: &str) -> Option<String> {
        hint_and_body(content, Path::new(path)).map(|(hint, _)| hint)
    }

    #[test]
    fn reads_the_hint_from_frontmatter_only() {
        let content = command(Some("[--all] [feature]"), "argument-hint: not frontmatter");
        assert_eq!(hint(&content, "review.md").unwrap(), "[--all] [feature]");
    }

    #[test]
    fn a_file_without_frontmatter_has_no_hint() {
        assert!(hint("# Title\n\nargument-hint: \"[--x]\"\n", "review.md").is_none());
    }

    #[test]
    fn an_empty_frontmatter_block_yields_no_hint_without_erroring() {
        // `---\n---\n` is the case a hand-rolled scan gets wrong: the closing
        // fence is the very next line. split_frontmatter handles it, so this
        // is None (no hint declared), not a swallowed error.
        assert!(hint("---\n---\n\n# Title\n", "x.md").is_none());
    }

    #[test]
    fn a_crlf_frontmatter_opener_is_read() {
        let content = "---\r\ndescription: x\r\nargument-hint: \"[--all]\"\r\n---\r\n\r\nbody\r\n";
        assert_eq!(hint(content, "x.md").unwrap(), "[--all]");
    }

    #[test]
    fn the_token_scanner_accepts_the_whole_set() {
        for token in [
            "$ARGUMENTS",
            "$@",
            "$1",
            "${1:-x}",
            "${@:-x}",
            "${@:2}",
            "${@:2:3}",
        ] {
            assert!(has_substitution_token(token), "{token} must be accepted");
        }
    }

    #[test]
    fn a_tokenless_body_is_the_direction_three_defect() {
        // `target.md`'s shape before the fix: a declared hint, prose that
        // names the argument, and no token the host can substitute into.
        assert!(!has_substitution_token(
            "# Target\n\nSet the feature when the invocation has an argument.\n"
        ));
        assert!(has_substitution_token(
            "# Target\n\nInvocation arguments: `$ARGUMENTS` (empty when none).\n"
        ));
        // A bare `$` before a word is prose, not a token. Reporting it as one
        // would make the check miss a genuinely tokenless command.
        assert!(!has_substitution_token("costs $x and $foo"));
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
