//! `check-promotion-coverage` — how much of a rules file has been routed to
//! the canonical source it belongs in.
//!
//! [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s
//! *Shared knowledge stays in git* routes a learning by **population**, and
//! the first destination is the constitution. Nothing reported how much of the
//! rules file had actually gone there — and *not having promoted* is invisible
//! by construction, because a universal entry still sitting in the project's
//! own file looks exactly like one correctly judged project-only. Both are
//! bullets in the same section, and neither carries a marker, since a
//! per-entry marker is the authored input §design-principles rejects.
//!
//! The measurement needs no authored input at all:
//!
//! ```text
//! unclassified = rule-bearing − (table-keyed ∪ pointer-citing)
//! ```
//!
//! **Why a primitive rather than a hand derivation.** The figure has been
//! re-derived by hand repeatedly and disagreed with itself every time, and not
//! once was the cause decay — each disagreement was a difference of *method*.
//! Counting the pointer side as anchor-bearing lines in the whole file rather
//! than as rule-bearing bullets in the named sections gives two honest answers
//! to one question; and a matcher that does not strip a trailing parenthetical
//! from a table key fails to match every key carrying one, reporting a
//! coverage gap that is an artifact of the instrument. Both were caught only
//! because someone happened to re-derive a second time. A recorded figure
//! decays and a hand method drifts; this does neither, which is why all four
//! terms are reported rather than just the difference.
//!
//! **A notice, never a gate** (spec 067, scenario `the-promotion-coverage-line`).
//! Promotion has to stay free for the reason §brownfield-inbox gives for
//! capture: gating a release on how much is unrouted would make the honest
//! choice between a growing backlog and a silent one push toward silence.

use std::collections::BTreeSet;
use std::path::Path;

use crate::primitives::{Result, iter_bullets, parse_atx_heading, read_text, resolve_path};
use crate::schema::primitives::{
    CheckPromotionCoverageArgs, CheckPromotionCoverageResult, PromotionTableState,
};

/// Execute the `check-promotion-coverage` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::Io`] when `rules-file`, or a
/// `table-file` that *was* supplied, cannot be read. A supplied-but-unreadable
/// table is an error rather than the `no-table` state on purpose: treating it
/// as absent would report full coverage over a file nobody could read, which
/// is `QUAL-CLAIM-001` on this measurement's own surface.
pub fn run(args: &CheckPromotionCoverageArgs, repo: &Path) -> Result<CheckPromotionCoverageResult> {
    let rules = read_text(&resolve_path(repo, &args.rules_file))?;

    // The sections are a set: naming one twice must not double its bullets.
    let wanted: BTreeSet<&str> = args.sections.iter().map(String::as_str).collect();
    let (entries, present) = rule_bearing_entries(&rules, &wanted);
    let missing_sections: Vec<String> = wanted
        .iter()
        .filter(|s| !present.contains(**s as &str))
        .map(|s| (*s).to_string())
        .collect();

    let leads: Vec<String> = entries.iter().map(|line| lead_phrase(line)).collect();

    let pointer_citing: BTreeSet<usize> = entries
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains(&args.pointer_link))
        .map(|(i, _)| i)
        .collect();

    let mut table_keyed: BTreeSet<usize> = BTreeSet::new();
    let mut unmatched_keys: Vec<String> = Vec::new();
    let mut table_rows = 0usize;
    let mut table_state = PromotionTableState::NoTable;

    if let Some(table_file) = &args.table_file {
        let table = read_text(&resolve_path(repo, table_file))?;
        table_state = PromotionTableState::Read;
        for key in table_keys(&table, args.table_section.as_deref()) {
            table_rows += 1;
            match match_key(&key, &leads) {
                Some(idx) => {
                    table_keyed.insert(idx);
                }
                None => unmatched_keys.push(key),
            }
        }
    }

    let classified: BTreeSet<usize> = table_keyed.union(&pointer_citing).copied().collect();
    let unclassified_entries: Vec<String> = leads
        .iter()
        .enumerate()
        .filter(|(i, _)| !classified.contains(i))
        .map(|(_, lead)| lead.clone())
        .collect();

    Ok(CheckPromotionCoverageResult {
        rule_bearing: entries.len(),
        table_keyed: table_keyed.len(),
        pointer_citing: pointer_citing.len(),
        classified: classified.len(),
        unclassified: unclassified_entries.len(),
        unclassified_entries,
        table_rows,
        unmatched_keys,
        missing_sections,
        table_state,
    })
}

/// Top-level bullets under any of the `wanted` level-2 headings, plus the set
/// of those headings actually present.
///
/// **Reuses `iter_bullets` for its comment- and fence-awareness and keeps the
/// top-level constraint local, deliberately.** That helper trims leading
/// whitespace before matching, because it was written for inbox bullets where
/// nesting does not occur; delegating wholesale would silently widen this
/// count to nested list items and inflate the denominator. The property the
/// shared helper *does* provide is the valuable half — a rules file's guidance
/// blocks legitimately contain `- ` lines inside HTML comments, and counting
/// those once inflated an unrelated count by roughly thirty. So the skip
/// scanner is shared and the indentation test is not. See `QUAL-DELEG-001`.
fn rule_bearing_entries<'a>(
    content: &'a str,
    wanted: &BTreeSet<&str>,
) -> (Vec<&'a str>, BTreeSet<String>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut section_of: Vec<Option<&str>> = Vec::with_capacity(lines.len());
    let mut present: BTreeSet<String> = BTreeSet::new();
    let mut current: Option<&str> = None;

    for line in &lines {
        if let Some((level, text)) = parse_atx_heading(line)
            && level <= 2
        {
            current = wanted.get(text.as_str()).copied();
            if let Some(name) = current {
                present.insert(name.to_string());
            }
        }
        section_of.push(current);
    }

    let entries = iter_bullets(content)
        .filter(|(idx, _)| {
            // Top-level only: the shared bullet grammar accepts an indented
            // marker and this count must not.
            lines.get(*idx).is_some_and(|l| l.starts_with("- "))
                && section_of.get(*idx).copied().flatten().is_some()
        })
        .filter_map(|(idx, _)| lines.get(idx).copied())
        .collect();

    (entries, present)
}

/// The bolded span opening a bullet, normalized; the whole bullet when it
/// carries no bolded lead.
fn lead_phrase(line: &str) -> String {
    let body = line.strip_prefix("- ").unwrap_or(line);
    if let Some(rest) = body.strip_prefix("**")
        && let Some(end) = rest.find("**")
    {
        return normalize(&rest[..end]);
    }
    normalize(body)
}

/// Strip inline markup and collapse whitespace, so a key and a lead phrase are
/// compared on their words rather than on their emphasis.
fn normalize(text: &str) -> String {
    let stripped: String = text
        .chars()
        .filter(|c| !matches!(c, '`' | '*' | '\\'))
        .collect();
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Drop one trailing parenthetical from a table key.
///
/// A table may qualify a key with a parenthetical the entry itself does not
/// carry — a retained positional identifier, say. Without this, every such key
/// fails to match and the result reports a coverage gap that is an artifact of
/// the matcher rather than a property of the corpus. That exact mistake has
/// been made by hand against this very measurement.
fn strip_trailing_parenthetical(key: &str) -> &str {
    let trimmed = key.trim_end();
    let Some(stripped) = trimmed.strip_suffix(')') else {
        return trimmed;
    };
    match stripped.rfind('(') {
        // Only a parenthetical with no nesting inside it, which is what a
        // qualifier is; anything else is left alone rather than guessed at.
        Some(open) if !stripped[open..].contains(')') => stripped[..open].trim_end(),
        _ => trimmed,
    }
}

/// Match a key against the lead phrases, by prefix in either direction.
///
/// Either direction, because a table may key an entry by its full lead phrase
/// or by a shortened form of it, and both are the same intent.
fn match_key(key: &str, leads: &[String]) -> Option<usize> {
    let needle = normalize(strip_trailing_parenthetical(key));
    if needle.is_empty() {
        return None;
    }
    leads
        .iter()
        .position(|lead| lead.starts_with(&needle) || needle.starts_with(lead.as_str()))
}

/// First cells of every markdown table row, optionally scoped to one level-2
/// section. Header and delimiter rows are skipped.
fn table_keys(content: &str, section: Option<&str>) -> Vec<String> {
    let mut inside = section.is_none();
    let mut keys = Vec::new();
    let mut header_pending = false;

    for line in content.lines() {
        if let Some((level, text)) = parse_atx_heading(line) {
            if level <= 2 {
                if let Some(name) = section {
                    inside = text == name;
                }
                header_pending = false;
            }
            continue;
        }
        if !inside {
            continue;
        }
        let Some(rest) = line.strip_prefix("| ") else {
            header_pending = false;
            continue;
        };
        let Some(cell) = rest.split('|').next().map(str::trim) else {
            continue;
        };
        // A delimiter row (`| --- |`) marks the end of the header; the row
        // before it was the header, and neither is a key.
        if !cell.is_empty() && cell.chars().all(|c| matches!(c, '-' | ':' | ' ')) {
            if let Some(last) = keys.pop() {
                debug_assert!(header_pending, "popped a key that was not a header: {last}");
            }
            header_pending = false;
            continue;
        }
        if cell.is_empty() {
            continue;
        }
        header_pending = true;
        keys.push(cell.to_string());
    }
    keys
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    fn seed(rules: &str, table: Option<&str>) -> TempDir {
        let tmp = tempdir().unwrap();
        fs::write(tmp.path().join("RULES.md"), rules).unwrap();
        if let Some(t) = table {
            fs::write(tmp.path().join("plan.md"), t).unwrap();
        }
        tmp
    }

    fn args(table: bool) -> CheckPromotionCoverageArgs {
        CheckPromotionCoverageArgs {
            rules_file: "RULES.md".into(),
            sections: vec!["Workflow".into(), "Gotchas".into()],
            table_file: table.then(|| "plan.md".to_string()),
            table_section: Some("Classification".into()),
            pointer_link: "constitution.md#".into(),
        }
    }

    const TABLE: &str = "# Plan\n\n## Classification\n\n| Entry | Reason |\n| --- | --- |\n| A routed rule | promoted |\n";

    #[test]
    fn the_four_terms_and_their_difference_are_reported() {
        let rules = "# R\n\n## Workflow\n\n\
             - **A routed rule.** The rule is [x](constitution.md#grounding) — pointer.\n\
             - **An unrouted rule.** States something of its own.\n\n\
             ## Gotchas\n\n\
             - **A third rule.** Also unrouted.\n";
        let tmp = seed(rules, Some(TABLE));
        let r = run(&args(true), tmp.path()).unwrap();
        assert_eq!(r.rule_bearing, 3);
        assert_eq!(r.table_keyed, 1);
        assert_eq!(r.pointer_citing, 1);
        // The routed entry is in both sets and counted once.
        assert_eq!(r.classified, 1);
        assert_eq!(r.unclassified, 2);
        assert_eq!(r.table_state, PromotionTableState::Read);
        assert!(
            r.unclassified_entries
                .iter()
                .any(|e| e == "An unrouted rule.")
        );
    }

    /// Absence of a table is a *state*. Reporting zero coverage over a project
    /// that keeps no classification table would be the conflation the whole
    /// measurement exists to remove.
    #[test]
    fn no_table_is_a_state_not_a_coverage_of_zero() {
        let rules = "# R\n\n## Workflow\n\n- **Only rule.** Text.\n";
        let tmp = seed(rules, None);
        let r = run(&args(false), tmp.path()).unwrap();
        assert_eq!(r.table_state, PromotionTableState::NoTable);
        assert_eq!(r.table_rows, 0);
        assert_eq!(r.rule_bearing, 1);
    }

    /// A supplied-but-unreadable table errors rather than degrading to the
    /// absent state, which would report full coverage over an unread file.
    #[test]
    fn a_supplied_but_missing_table_is_an_error_not_the_absent_state() {
        let rules = "# R\n\n## Workflow\n\n- **Only rule.** Text.\n";
        let tmp = seed(rules, None);
        assert!(run(&args(true), tmp.path()).is_err());
    }

    /// The narrowed direction: a nested bullet is not an entry. The shared
    /// bullet grammar accepts an indented marker, so delegating to it
    /// wholesale would inflate the denominator.
    #[test]
    fn nested_bullets_and_comment_bullets_are_not_entries() {
        let rules = "# R\n\n## Workflow\n\n\
             <!-- guidance:\n     - not an entry\n-->\n\n\
             - **A real entry.** Text.\n  - a nested detail\n\n\
             ```text\n- not an entry either\n```\n";
        let tmp = seed(rules, None);
        let r = run(&args(false), tmp.path()).unwrap();
        assert_eq!(r.rule_bearing, 1, "only the top-level, uncommented bullet");
    }

    /// Bullets outside the named sections do not count, and a section named
    /// twice does not double.
    #[test]
    fn only_named_sections_count_and_the_set_does_not_double() {
        let rules = "# R\n\n## Workflow\n\n- **In scope.** Text.\n\n\
             ## Tech Stack\n\n- **Out of scope.** Text.\n";
        let mut a = args(false);
        a.sections = vec!["Workflow".into(), "Workflow".into()];
        let tmp = seed(rules, None);
        let r = run(&a, tmp.path()).unwrap();
        assert_eq!(r.rule_bearing, 1);
    }

    /// A renamed section must not quietly shrink the denominator — that moves
    /// coverage in the flattering direction.
    #[test]
    fn an_absent_section_is_reported_rather_than_contributing_zero() {
        let rules = "# R\n\n## Workflow\n\n- **In scope.** Text.\n";
        let tmp = seed(rules, None);
        let r = run(&args(false), tmp.path()).unwrap();
        assert_eq!(r.missing_sections, vec!["Gotchas".to_string()]);
    }

    /// The widened direction for the key matcher: a trailing parenthetical on
    /// a key must not prevent the match.
    #[test]
    fn a_trailing_parenthetical_on_a_key_still_matches() {
        let rules = "# R\n\n## Workflow\n\n- **A routed rule.** Text.\n";
        let table = "# Plan\n\n## Classification\n\n| Entry | Reason |\n| --- | --- |\n| A routed rule (survey W11) | promoted |\n";
        let tmp = seed(rules, Some(table));
        let r = run(&args(true), tmp.path()).unwrap();
        assert_eq!(r.table_keyed, 1);
        assert_eq!(r.unmatched_keys, [] as [String; 0]);
    }

    /// The narrowed direction: a key matching nothing is reported, because
    /// that is how a *reworded* entry surfaces.
    #[test]
    fn a_key_matching_no_entry_is_reported_loudly() {
        let rules = "# R\n\n## Workflow\n\n- **The entry as reworded.** Text.\n";
        let table = "# Plan\n\n## Classification\n\n| Entry | Reason |\n| --- | --- |\n| The entry as it used to read | promoted |\n";
        let tmp = seed(rules, Some(table));
        let r = run(&args(true), tmp.path()).unwrap();
        assert_eq!(r.table_keyed, 0);
        assert_eq!(r.unmatched_keys.len(), 1);
    }

    /// Tables outside the named section are not read.
    #[test]
    fn table_extraction_is_scoped_to_the_named_section() {
        let rules = "# R\n\n## Workflow\n\n- **A routed rule.** Text.\n";
        let table = "# Plan\n\n## Affected Files\n\n| File | Why |\n| --- | --- |\n| A routed rule | noise |\n\n## Classification\n\n| Entry | Reason |\n| --- | --- |\n";
        let tmp = seed(rules, Some(table));
        let r = run(&args(true), tmp.path()).unwrap();
        assert_eq!(r.table_rows, 0, "the affected-files table is out of scope");
        assert_eq!(r.unclassified, 1);
    }

    #[test]
    fn header_and_delimiter_rows_are_not_keys() {
        let rules = "# R\n\n## Workflow\n\n- **Entry.** Text.\n";
        let tmp = seed(rules, Some(TABLE));
        let r = run(&args(true), tmp.path()).unwrap();
        // "Entry" is the header and must not be taken as a key.
        assert!(!r.unmatched_keys.iter().any(|k| k == "Entry"));
        assert_eq!(r.table_rows, 1);
    }

    #[test]
    fn a_rules_file_with_no_entries_reports_a_visible_denominator() {
        let rules = "# R\n\n## Workflow\n\n";
        let tmp = seed(rules, None);
        let r = run(&args(false), tmp.path()).unwrap();
        assert_eq!(r.rule_bearing, 0);
        assert_eq!(r.unclassified, 0);
    }
}
