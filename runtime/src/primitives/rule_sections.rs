//! The rule sections of a rule file — one reading of what a rule is.
//!
//! `check-rule-ids` and the exec `/{project}:analyze` walker both read rule
//! files, and they must agree on which headings define a rule. The walker
//! used to treat any level-3 heading as one while `check-rule-ids` matched
//! the rule-ID grammar, so the rules one assessed and the IDs the other knew
//! could differ over the same file. Both now take the heading grammar from
//! here (spec 060).
//!
//! A section's tier is derived from its Statement's RFC 2119 keyword, as the
//! rule-format data models state (`specs/008-security-rules/data-model.md`
//! §Severity classification): MUST or MUST NOT is MUST-tier, SHOULD or SHOULD
//! NOT is SHOULD-tier. The format keeps a Statement's keywords in one tier,
//! and the shipped rule files are held to it by a test below. A Statement in
//! a rule file a project authors or pins is not, so one carrying both still
//! states a blocking obligation and is read as MUST-tier. One carrying
//! neither has no tier, and the walker records it unexamined rather than
//! assessing it in a tier nobody stated.

#![allow(clippy::expect_used)]

use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::parse_atx_heading;
use crate::schema::severity::RuleSeverity;

/// One rule section of a rule file.
#[derive(Debug)]
pub(crate) struct RuleSection {
    /// The rule ID its heading carries (e.g. `CFG-CONST-001`).
    pub(crate) id: String,
    /// Its heading line, trimmed, so a reader can check its level and that
    /// it holds the ID alone (spec 080).
    pub(crate) heading: String,
    /// The section's first block quote, `>` markers stripped and lines
    /// joined with single spaces. Empty when the section has none.
    pub(crate) statement: String,
    /// The section's first `**Verification:**` paragraph, wrapped lines
    /// joined with single spaces. `None` when absent or empty.
    pub(crate) verification: Option<String>,
}

impl RuleSection {
    /// The rule's tier, from the RFC 2119 keyword in its Statement: MUST-tier
    /// when the Statement carries `MUST` (so `MUST NOT` too), otherwise
    /// SHOULD-tier when it carries `SHOULD`, otherwise `None`.
    ///
    /// Keywords are whole tokens matched case-sensitively. The rule files
    /// write them in capitals, and a lowercase "must" in a Statement's prose
    /// is not an obligation keyword.
    pub(crate) fn tier(&self) -> Option<RuleSeverity> {
        let mut should = false;
        for token in self.tokens() {
            match token {
                "MUST" => return Some(RuleSeverity::Must),
                "SHOULD" => should = true,
                _ => {}
            }
        }
        should.then_some(RuleSeverity::Should)
    }

    /// The Statement's whole tokens, split on every non-alphanumeric
    /// character. Every keyword check reads the Statement through this, so
    /// two checks cannot disagree about what counts as a keyword.
    fn tokens(&self) -> impl Iterator<Item = &str> {
        self.statement.split(|c: char| !c.is_ascii_alphanumeric())
    }
}

/// `### BE-AUTHN-001` — matches rule-ID headings inside a rule file. The
/// category segment follows the schema grammar `[A-Z][A-Z0-9]*` (digits are
/// permitted after the first letter, e.g. `FE-A11YFORM-001`), per
/// `specs/008-security-rules/data-model.md`.
///
/// What this gives up: a heading the grammar rejects opens no rule, so a
/// malformed rule heading, such as `### be-authn-001` or a rule at level 5,
/// is no rule to [`parse_rule_sections`] and no reader of it sees one.
/// Neither does a heading [`parse_atx_heading`] rejects, though `CommonMark`
/// renders it as one: a tab after the `#` run, as in `###\tBE-AUTHN-001`.
/// [`other_headings`] lists both kinds, so analyze's integrity checks can
/// find one (spec 080).
pub(crate) fn heading_id_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"(?m)^#{2,4}\s+([A-Z]{2,5}-[A-Z][A-Z0-9]+-\d{3,4})\b")
            .expect("hard-coded regex compiles")
    })
}

/// An ATX heading whose `#` run, one to six long, is followed by a tab.
fn tab_heading_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[ \t]*#{1,6}\t").expect("hard-coded regex compiles"))
}

/// Each ATX heading in `content` that opens no rule section, with its
/// 1-based line, its text, trimmed, and its body: what [`parse_rule_sections`]
/// gives up, a malformed rule heading among them. The body is the lines under
/// the heading up to the next heading of any level, blank lines at either end
/// dropped, so a reader judges whether the heading heads a rule without
/// reading the file (spec 080, task 57).
pub(crate) fn other_headings(content: &str) -> Vec<(usize, String, String)> {
    let lines: Vec<&str> = content.lines().collect();
    // A heading here is what `CommonMark` renders as one, which is wider than
    // what the shared parser takes: it also counts a `#` run followed by a
    // tab, which `parse_atx_heading` rejects, so that heading opens no rule
    // and is listed rather than seen nowhere. Kept local, since widening the
    // shared parser would change every reader of it.
    let headings: Vec<usize> = (0..lines.len())
        .filter(|&i| {
            parse_atx_heading(lines[i]).is_some() || tab_heading_regex().is_match(lines[i])
        })
        .collect();
    let opens_rule =
        |line: &str| parse_atx_heading(line).is_some() && heading_id_regex().is_match(line);
    headings
        .iter()
        .enumerate()
        .filter(|&(_, &i)| !opens_rule(lines[i]))
        .map(|(k, &i)| {
            let end = headings.get(k + 1).copied().unwrap_or(lines.len());
            let mut under = &lines[i + 1..end];
            while let [first, rest @ ..] = under
                && first.trim().is_empty()
            {
                under = rest;
            }
            while let [rest @ .., last] = under
                && last.trim().is_empty()
            {
                under = rest;
            }
            (i + 1, lines[i].trim().to_owned(), under.join("\n"))
        })
        .collect()
}

/// Every rule section in `content`, in heading order.
///
/// A section opens at a heading [`heading_id_regex`] matches and runs to the
/// next rule heading, or to the next heading of the same or higher level,
/// the boundary `check-rule-ids` uses to scope a deprecation label. A deeper
/// heading that is not a rule's is a subsection and stays in scope. The
/// Statement is the section's first block quote. The Verification paragraph
/// ends at a blank line, the next `**Field:**`, or any heading.
pub(crate) fn parse_rule_sections(content: &str) -> Vec<RuleSection> {
    let mut sections = Vec::new();
    let mut open: Option<OpenSection> = None;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some((level, _)) = parse_atx_heading(line) {
            let rule_id = heading_id_regex()
                .captures(line)
                .map(|cap| cap[1].to_string());
            let closes = rule_id.is_some() || open.as_ref().is_some_and(|s| level <= s.level);
            if closes && let Some(section) = open.take() {
                sections.push(section.finish());
            }
            match (rule_id, open.as_mut()) {
                (Some(id), _) => open = Some(OpenSection::new(id, line.trim().to_owned(), level)),
                // A subsection heading ends a paragraph in progress.
                (None, Some(section)) => section.end_paragraphs(),
                (None, None) => {}
            }
            continue;
        }
        if let Some(section) = open.as_mut() {
            section.read(trimmed);
        }
    }
    if let Some(section) = open {
        sections.push(section.finish());
    }
    sections
}

/// Where a section's reader stands in its Statement block quote.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Quote {
    NotStarted,
    Reading,
    Done,
}

/// Where a section's reader stands in its Verification paragraph.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Paragraph {
    NotStarted,
    Reading,
    Done,
}

/// A rule section being read, line by line.
struct OpenSection {
    id: String,
    heading: String,
    level: u8,
    statement: Vec<String>,
    quote: Quote,
    verification: Vec<String>,
    paragraph: Paragraph,
}

impl OpenSection {
    fn new(id: String, heading: String, level: u8) -> Self {
        Self {
            id,
            heading,
            level,
            statement: Vec::new(),
            quote: Quote::NotStarted,
            verification: Vec::new(),
            paragraph: Paragraph::NotStarted,
        }
    }

    /// Read one non-heading line of the section, trimmed.
    fn read(&mut self, trimmed: &str) {
        let quoted = trimmed.strip_prefix('>');
        match (self.quote, quoted) {
            (Quote::NotStarted | Quote::Reading, Some(rest)) => {
                self.quote = Quote::Reading;
                let rest = rest.trim();
                if !rest.is_empty() {
                    self.statement.push(rest.to_string());
                }
            }
            (Quote::Reading, None) => self.quote = Quote::Done,
            _ => {}
        }
        match self.paragraph {
            Paragraph::NotStarted => {
                if let Some(rest) = trimmed.strip_prefix("**Verification:**") {
                    self.paragraph = Paragraph::Reading;
                    self.push_verification(rest);
                }
            }
            Paragraph::Reading => {
                if trimmed.is_empty() || trimmed.starts_with("**") {
                    self.paragraph = Paragraph::Done;
                } else {
                    self.push_verification(trimmed);
                }
            }
            Paragraph::Done => {}
        }
    }

    fn push_verification(&mut self, text: &str) {
        let text = text.trim();
        if !text.is_empty() {
            self.verification.push(text.to_string());
        }
    }

    /// A heading inside the section ends the Statement and any Verification
    /// paragraph in progress.
    fn end_paragraphs(&mut self) {
        if self.quote == Quote::Reading {
            self.quote = Quote::Done;
        }
        if self.paragraph == Paragraph::Reading {
            self.paragraph = Paragraph::Done;
        }
    }

    fn finish(self) -> RuleSection {
        let verification = self.verification.join(" ");
        RuleSection {
            id: self.id,
            heading: self.heading,
            statement: self.statement.join(" "),
            verification: (!verification.is_empty()).then_some(verification),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn rule(id: &str, statement: &str, verification: &str) -> String {
        format!(
            "### {id}\n\n> {statement}\n\n**Rationale:** Why.\n\n**Verification:** {verification}\n"
        )
    }

    fn only(content: &str) -> RuleSection {
        let sections = parse_rule_sections(content);
        assert_eq!(sections.len(), 1, "{sections:?}");
        sections.into_iter().next().unwrap()
    }

    fn tier_of(statement: &str) -> Option<RuleSeverity> {
        only(&rule("TS-RULE-001", statement, "v")).tier()
    }

    /// Whether a section's Statement carries keywords of both tiers, read
    /// through the same tokens [`RuleSection::tier`] reads.
    fn mixes_tiers(section: &RuleSection) -> bool {
        section.tokens().any(|token| token == "MUST")
            && section.tokens().any(|token| token == "SHOULD")
    }

    fn statement_mixes_tiers(statement: &str) -> bool {
        mixes_tiers(&only(&rule("TS-RULE-001", statement, "v")))
    }

    /// Every rule file shipped under `framework/rules/`, sorted.
    fn shipped_rule_files() -> Vec<std::path::PathBuf> {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/rules");
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no rule files under {}", dir.display());
        files
    }

    /// A rule heading with a tab after its `#` run renders as a heading but
    /// opens no rule, since the shared heading parser rejects it, so
    /// `other_headings` lists it with its body rather than leaving it in no
    /// part of the result (spec 080; `QUAL-DELEG-001`).
    #[test]
    fn a_tab_separated_rule_heading_is_an_other_heading() {
        let content = format!(
            "{}\n###\tTS-RULE-002\n\n> Tests MUST run.\n",
            rule("TS-RULE-001", "Tests SHOULD fail.", "v")
        );
        assert_eq!(parse_rule_sections(&content).len(), 1);
        let other = other_headings(&content);
        assert!(
            other
                .iter()
                .any(|(_, heading, body)| heading == "###\tTS-RULE-002"
                    && body == "> Tests MUST run."),
            "{other:?}"
        );
    }

    #[test]
    fn a_statement_s_keyword_gives_its_tier() {
        assert_eq!(tier_of("Tokens MUST expire."), Some(RuleSeverity::Must));
        assert_eq!(
            tier_of("Tokens MUST NOT be logged."),
            Some(RuleSeverity::Must)
        );
        assert_eq!(tier_of("Tokens SHOULD rotate."), Some(RuleSeverity::Should));
        assert_eq!(
            tier_of("Tokens SHOULD NOT be reused."),
            Some(RuleSeverity::Should)
        );
    }

    /// A Statement carrying both keywords states a blocking obligation, in
    /// either order. No shipped rule has one; this is the reading for a rule
    /// file a project authors or pins.
    #[test]
    fn a_statement_carrying_both_keywords_is_must_tier() {
        assert_eq!(
            tier_of("Lists MUST paginate and SHOULD use cursors."),
            Some(RuleSeverity::Must)
        );
        assert_eq!(
            tier_of("Images SHOULD lazy-load; scripts MUST be split."),
            Some(RuleSeverity::Must)
        );
    }

    /// No keyword, or only a lowercase one, is no tier — never a guessed one.
    #[test]
    fn a_statement_with_no_keyword_has_no_tier() {
        assert_eq!(tier_of("Constants live in one module."), None);
        assert_eq!(tier_of("Callers must not drift apart."), None);
        assert_eq!(tier_of("MUSTARD and SHOULDERS are not keywords."), None);
    }

    #[test]
    fn a_prefixed_statement_is_read_after_its_label() {
        let section = only(
            "### CFG-CONST-001\n\n> **Statement:** Constants MUST live in a single\n> central module.\n\n**Verification:** v\n",
        );
        assert_eq!(
            section.statement,
            "**Statement:** Constants MUST live in a single central module."
        );
        assert_eq!(section.tier(), Some(RuleSeverity::Must));
    }

    #[test]
    fn a_wrapped_verification_is_joined() {
        let section = only(
            "### TS-RULE-001\n\n> A MUST.\n\n**Verification:** Every literal\nthat varies across\n  environments is named.\n\n**Source:** x\n",
        );
        assert_eq!(
            section.verification.as_deref(),
            Some("Every literal that varies across environments is named.")
        );
    }

    #[test]
    fn a_verification_ends_at_the_next_field() {
        let section = only("### TS-RULE-001\n\n> A MUST.\n\n**Verification:** v\n**Source:** s\n");
        assert_eq!(section.verification.as_deref(), Some("v"));
    }

    #[test]
    fn a_missing_or_empty_verification_is_none() {
        assert_eq!(
            only("### TS-RULE-001\n\n> A MUST.\n\n**Rationale:** r\n").verification,
            None
        );
        assert_eq!(
            only("### TS-RULE-001\n\n> A MUST.\n\n**Verification:**\n\n**Source:** s\n")
                .verification,
            None
        );
    }

    /// Deprecation does not change what a section is: the walker still asks
    /// about a deprecated rule, which binds until it is removed.
    #[test]
    fn a_deprecated_section_parses_like_any_other() {
        let section = only(
            "### TS-OLD-001\n\n**DEPRECATED in 0.5.0:** replaced by TS-NEW-002.\n\n> Old MUST.\n\n**Verification:** v\n",
        );
        assert_eq!(section.id, "TS-OLD-001");
        assert_eq!(section.tier(), Some(RuleSeverity::Must));
        assert_eq!(section.verification.as_deref(), Some("v"));
    }

    /// A category heading carries no rule ID, and it ends the rule before it
    /// rather than folding the next category's prose into that rule.
    #[test]
    fn a_category_heading_is_not_a_rule_and_ends_the_one_before_it() {
        let content = format!(
            "# Rules\n\n## CFG-CONST — Constants\n\n{}\n## CFG-ENV — Environment\n\n> Stray MUST quote.\n\n**Verification:** stray\n\n{}",
            rule("CFG-CONST-001", "A MUST.", "first"),
            rule("CFG-ENV-001", "An env SHOULD.", "second"),
        );
        let sections = parse_rule_sections(&content);
        let ids: Vec<&str> = sections.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["CFG-CONST-001", "CFG-ENV-001"]);
        assert_eq!(sections[0].verification.as_deref(), Some("first"));
        assert_eq!(sections[1].tier(), Some(RuleSeverity::Should));
    }

    /// A deeper heading is a subsection: it ends the paragraph in progress
    /// but not the rule, while a same-level heading ends the rule.
    #[test]
    fn a_section_ends_at_the_next_same_or_higher_heading() {
        let sections = parse_rule_sections(
            "### TS-RULE-001\n\n> A MUST.\n\n#### Notes\n\n**Verification:** in the subsection\n\n### Not a rule\n\n**Verification:** outside\n",
        );
        assert_eq!(sections.len(), 1, "{sections:?}");
        assert_eq!(
            sections[0].verification.as_deref(),
            Some("in the subsection")
        );
    }

    /// Only the first block quote is the Statement; a later one — a signpost,
    /// a note — is not folded into it.
    #[test]
    fn only_the_first_block_quote_is_the_statement() {
        let section = only(
            "### TS-RULE-001\n\n> Keys MUST rotate.\n\n**Verification:** v\n\n> Note: SHOULD is discussed elsewhere.\n",
        );
        assert_eq!(section.statement, "Keys MUST rotate.");
    }

    /// Every rule `check-rule-ids` recognizes in the shipped rule files is a
    /// section the walker can assess: it parses, carries a Verification, and
    /// has a tier. A shipped rule failing any of the three would be recorded
    /// unexamined on every exec analyze.
    #[test]
    fn every_shipped_rule_is_assessable() {
        let mut total = 0;
        for file in &shipped_rule_files() {
            let content = std::fs::read_to_string(file).unwrap();
            let headings = heading_id_regex().captures_iter(&content).count();
            let sections = parse_rule_sections(&content);
            assert_eq!(sections.len(), headings, "{}", file.display());
            for section in &sections {
                assert!(
                    section.verification.is_some() && section.tier().is_some(),
                    "{} {} is not assessable: {section:?}",
                    file.display(),
                    section.id
                );
            }
            total += sections.len();
        }
        assert!(total > 0, "no rule sections in the shipped rule files");
    }

    /// The check below keeps a subject once the shipped files are clean: a
    /// Statement spanning both tiers is caught in either order, and one that
    /// pairs two keywords of a single tier, or adds MAY, is not.
    #[test]
    fn a_statement_mixing_tiers_is_caught_and_a_single_tier_one_is_not() {
        assert!(statement_mixes_tiers(
            "Lists MUST paginate and SHOULD use cursors."
        ));
        assert!(statement_mixes_tiers(
            "Images SHOULD lazy-load; scripts MUST NOT block."
        ));
        assert!(!statement_mixes_tiers(
            "Cookies MUST be Secure and MUST NOT be readable."
        ));
        assert!(!statement_mixes_tiers(
            "Fonts SHOULD be subset and SHOULD NOT be inlined."
        ));
        assert!(!statement_mixes_tiers(
            "Traffic MUST use TLS; loopback MAY use plaintext."
        ));
    }

    /// No shipped rule's Statement spans both tiers. The rule format keeps a
    /// Statement's keywords in one tier (`specs/008-security-rules/data-model.md`
    /// §Severity classification) because a review waiver and cross-pass dedup
    /// both key on the rule ID, so a mixed rule would waive or collapse a
    /// blocking clause together with an advisory one. Every offender is named.
    #[test]
    fn no_shipped_statement_mixes_tiers() {
        let mut total = 0;
        let mut mixed = Vec::new();
        for file in &shipped_rule_files() {
            let content = std::fs::read_to_string(file).unwrap();
            for section in parse_rule_sections(&content) {
                total += 1;
                if mixes_tiers(&section) {
                    mixed.push(section.id);
                }
            }
        }
        assert!(total > 0, "no rule sections in the shipped rule files");
        assert!(
            mixed.is_empty(),
            "{} shipped Statement(s) carry both MUST and SHOULD: {mixed:?}",
            mixed.len()
        );
    }

    /// The list of values a log rule's Statement bars: from the word after
    /// the one word following the first `MUST NOT `, read as the verb,
    /// through the closing "other sensitive field values". `None` when
    /// `MUST NOT ` or the closing phrase is missing.
    fn barred_values(statement: &str) -> Option<&str> {
        const END: &str = "other sensitive field values";
        let (_, after) = statement.split_once("MUST NOT ")?;
        let (_, list) = after.split_once(' ')?;
        let end = list.find(END)? + END.len();
        Some(&list[..end])
    }

    /// The rule section `id` in the shipped rule file `file`, which must
    /// exist: a rule renamed or removed fails the test that reads it.
    fn shipped_section(file: &str, id: &str) -> RuleSection {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../framework/rules")
            .join(file);
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("{}: {err}", path.display()));
        parse_rule_sections(&content)
            .into_iter()
            .find(|section| section.id == id)
            .unwrap_or_else(|| panic!("{id} not found in {}", path.display()))
    }

    /// The check below keeps a subject once the shipped lists agree: two
    /// lists that match are equal whatever verb precedes them, one that
    /// lacks a value is caught, and a Statement with no list reads as none.
    #[test]
    fn a_barred_value_list_is_read_and_a_differing_one_is_caught() {
        let logs =
            "Logs MUST NOT contain passwords, session IDs, or other sensitive field values. More.";
        let carriers = "Logs and spans MUST NOT carry passwords, session IDs, or other sensitive field values, wherever.";
        let fewer = "Logs MUST NOT contain passwords, or other sensitive field values.";
        assert_eq!(
            barred_values(logs),
            Some("passwords, session IDs, or other sensitive field values")
        );
        assert_eq!(barred_values(logs), barred_values(carriers));
        assert_ne!(barred_values(logs), barred_values(fewer));
        assert_eq!(barred_values("Logs MUST be structured."), None);
        assert_eq!(barred_values("Logs MUST NOT contain passwords."), None);
    }

    /// `security-cross.md` `SEC-LOG-001` copies `security-backend.md`
    /// `BE-LOG-002`'s list of the values kept out of logs, so a project that
    /// does not install `security-backend.md` is held to the same set (spec
    /// 077's plan, Trade-offs). Neither rule's text points an editor of
    /// `BE-LOG-002` at the copy, so this keeps an edit to one list from
    /// reaching only some project shapes.
    #[test]
    fn sec_log_001_bars_the_values_be_log_002_bars() {
        let backend = shipped_section("security-backend.md", "BE-LOG-002");
        let cross = shipped_section("security-cross.md", "SEC-LOG-001");
        let backend_list = barred_values(&backend.statement)
            .unwrap_or_else(|| panic!("no value list in BE-LOG-002: {}", backend.statement));
        let cross_list = barred_values(&cross.statement)
            .unwrap_or_else(|| panic!("no value list in SEC-LOG-001: {}", cross.statement));
        assert_eq!(
            cross_list, backend_list,
            "SEC-LOG-001's value list has drifted from BE-LOG-002's"
        );
    }
}
