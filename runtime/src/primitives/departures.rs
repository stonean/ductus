//! A spec's decided rule departures: the entry reader and the history
//! attribution every consumer shares (spec 073).
//!
//! An entry is a list item under `## Applicable Rules` that reads `departs:`
//! and one backticked rule ID, with a `reason:` child and an optional `cited:`
//! child (`specs/073-decided-rule-departures-are-recorded/data-model.md`,
//! §Departure entry). `check-departures`, `write-analysis` and the exec
//! walker's tally, and `write-review` with `process-waivers`, all read the
//! grammar and the attribution here, so none restates either.
//!
//! Section, fence and comment handling is the corpus's own: the section is
//! found by [`heading_lines`] and [`section_range`], and fenced code and HTML
//! comments are skipped by [`SkipScanner`], as `read-section` and every
//! `tasks.md` walker do. The legacy citation form (`` - `ID` — reason ``)
//! opens with no `departs:`, so it yields no entry and no problem.
//!
//! Attribution reads git history, so a CI job running it needs
//! `fetch-depth: 0` (§design-principles).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

use crate::primitives::resolve_anchor::{heading_lines, section_range};
use crate::primitives::{
    FeatureForm, ProjectRepository, SkipScanner, parse_feature_dir, without_inline_comments,
};

/// The heading a spec's departure entries sit under.
const SECTION: &str = "Applicable Rules";

/// One well-formed departure entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The rule the spec deliberately does not satisfy.
    pub rule: String,
    /// Why, in the spec author's words; never empty.
    pub reason: String,
    /// The `§anchor` reference that decided the departure in advance. A
    /// `cited:` child with no text is absent, as the author named none.
    pub cited: Option<String>,
    /// The 1-based line of the `departs:` item.
    pub line: usize,
}

/// What is wrong with an entry, or with the set of entries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProblemKind {
    /// The `departs:` item has no backticked rule ID.
    MissingRule,
    /// The `departs:` item names more than one backticked rule ID.
    SecondRule,
    /// The entry has no `reason:` child.
    MissingReason,
    /// The entry's `reason:` child has no text.
    EmptyReason,
    /// A child key other than `reason` and `cited`.
    UnknownKey(String),
    /// A `reason` or `cited` child given twice.
    DuplicateKey(String),
    /// A second entry for a rule an earlier entry names. Both entries stay in
    /// [`Parsed::entries`], so a consumer sees which rules are named twice
    /// from this problem and attributes each rule once.
    DuplicateRule(String),
}

/// A structural problem and the line it is reported at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    /// The 1-based line: the `departs:` item, or the offending child.
    pub line: usize,
    /// What is wrong.
    pub kind: ProblemKind,
}

/// A spec's departure entries and the problems found reading them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parsed {
    /// Every well-formed entry, in document order. A malformed entry is not
    /// here, so it waives nothing.
    pub entries: Vec<Entry>,
    /// Every structural problem, in document order.
    pub problems: Vec<Problem>,
}

/// An entry being read: its item, and its children so far.
struct Open {
    indent: usize,
    line: usize,
    rule: Option<String>,
    reason: Option<String>,
    cited: Option<String>,
    /// The key of the child the next continuation line extends.
    last_key: Option<String>,
    problems: Vec<Problem>,
}

/// Read the departure entries of a spec's `content`.
#[must_use]
pub fn parse(content: &str) -> Parsed {
    let lines: Vec<&str> = content.lines().collect();
    let mut parsed = Parsed::default();
    let Some(range) = section_range(&heading_lines(content), SECTION, lines.len()) else {
        return parsed;
    };
    let mut skip = SkipScanner::default();
    let mut open: Option<Open> = None;
    // Whether the previous line ended a paragraph: blank, skipped, or the
    // heading. A line that follows text directly can continue it lazily.
    let mut loose = true;
    for (index, raw) in lines.iter().enumerate() {
        if skip.skip(raw) || !range.contains(&index) || index == range.start {
            loose = true;
            continue;
        }
        let text = without_inline_comments(raw);
        if text.trim().is_empty() {
            loose = true;
            continue;
        }
        let after_break = std::mem::replace(&mut loose, false);
        let indent = text.len() - text.trim_start().len();
        let item = list_item(&text);
        if let Some(entry) = open.as_mut() {
            if indent > entry.indent {
                read_child(entry, index + 1, item, text.trim());
                continue;
            }
            // CommonMark's lazy continuation: unindented text directly after a
            // child's text is still that child's paragraph.
            if item.is_none()
                && !after_break
                && !text.trim_start().starts_with('#')
                && entry.last_key.is_some()
            {
                read_child(entry, index + 1, None, text.trim());
                continue;
            }
            finish(open.take(), &mut parsed);
        }
        if let Some(rest) = item.and_then(|rest| rest.strip_prefix("departs:")) {
            open = Some(start_entry(indent, index + 1, rest));
        }
    }
    finish(open, &mut parsed);
    parsed
}

/// The text of a list item after its marker, or `None` for any other line.
fn list_item(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    ["- ", "* ", "+ "]
        .iter()
        .find_map(|marker| trimmed.strip_prefix(marker))
        .map(str::trim)
}

fn start_entry(indent: usize, line: usize, rest: &str) -> Open {
    let ids: Vec<&str> = rest
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .collect();
    let mut problems = Vec::new();
    if ids.is_empty() {
        problems.push(Problem {
            line,
            kind: ProblemKind::MissingRule,
        });
    }
    if ids.len() > 1 {
        problems.push(Problem {
            line,
            kind: ProblemKind::SecondRule,
        });
    }
    Open {
        indent,
        line,
        rule: ids.first().map(|id| (*id).to_string()),
        reason: None,
        cited: None,
        last_key: None,
        problems,
    }
}

/// Read one line under an entry: a `key: value` child item, or the wrapped
/// continuation of the previous child's value.
fn read_child(entry: &mut Open, line: usize, item: Option<&str>, text: &str) {
    let Some(item) = item else {
        let Some(key) = entry.last_key.clone() else {
            return;
        };
        let slot = if key == "reason" {
            &mut entry.reason
        } else {
            &mut entry.cited
        };
        if let Some(value) = slot.as_mut() {
            if !value.is_empty() {
                value.push(' ');
            }
            value.push_str(text);
        }
        return;
    };
    let (key, value) = item.split_once(':').unwrap_or((item, ""));
    let key = key.trim();
    let slot = match key {
        "reason" => &mut entry.reason,
        "cited" => &mut entry.cited,
        other => {
            entry.last_key = None;
            entry.problems.push(Problem {
                line,
                kind: ProblemKind::UnknownKey(other.to_string()),
            });
            return;
        }
    };
    if slot.is_some() {
        entry.problems.push(Problem {
            line,
            kind: ProblemKind::DuplicateKey(key.to_string()),
        });
        entry.last_key = None;
        return;
    }
    *slot = Some(value.trim().to_string());
    entry.last_key = Some(key.to_string());
}

/// Close an entry: record its problems, and the entry itself when none is
/// structural.
fn finish(open: Option<Open>, parsed: &mut Parsed) {
    let Some(mut entry) = open else {
        return;
    };
    match entry.reason.as_deref() {
        None => entry.problems.push(Problem {
            line: entry.line,
            kind: ProblemKind::MissingReason,
        }),
        Some("") => entry.problems.push(Problem {
            line: entry.line,
            kind: ProblemKind::EmptyReason,
        }),
        Some(_) => {}
    }
    let well_formed = entry.problems.is_empty();
    parsed.problems.append(&mut entry.problems);
    let (true, Some(rule), Some(reason)) = (well_formed, entry.rule, entry.reason) else {
        return;
    };
    if parsed.entries.iter().any(|earlier| earlier.rule == rule) {
        parsed.problems.push(Problem {
            line: entry.line,
            kind: ProblemKind::DuplicateRule(rule.clone()),
        });
    }
    parsed.entries.push(Entry {
        rule,
        reason,
        cited: entry.cited.filter(|cited| !cited.is_empty()),
        line: entry.line,
    });
}

/// Who decided a departure, and when.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attribution {
    /// The first commit of the unbroken run ending at `HEAD` in which the
    /// spec holds an entry for the rule.
    Attributed {
        /// The commit's author email.
        decided_by: String,
        /// The commit's author time, RFC 3339 UTC to the second with `Z`.
        decided_at: String,
    },
    /// No history answers: the repository is shallow or absent, the entry is
    /// not in `HEAD`'s spec, or a revision cannot be read. A state of its own,
    /// never read as a person or a time.
    Unattributed,
}

/// Attribute each of `rules`, as named by entries in the spec at `spec_rel`
/// (a `/`-separated path from `project_root`), from the project's git history.
///
/// The walk follows first parents from `HEAD` and stops a rule's run at the
/// first commit whose spec holds no entry for it, so an entry removed and
/// added back is introduced by the re-adding commit and an edit to a kept
/// entry changes nothing. A spec directory renamed is followed with similarity
/// detection on the commit that adds the file; where none is found the add is
/// the introduction.
#[must_use]
pub fn attribute(
    project_root: &Path,
    spec_rel: &str,
    rules: &[String],
) -> BTreeMap<String, Attribution> {
    let mut result: BTreeMap<String, Attribution> = rules
        .iter()
        .map(|rule| (rule.clone(), Attribution::Unattributed))
        .collect();
    let Ok(project) = ProjectRepository::discover(project_root) else {
        return result;
    };
    if project.repository.is_shallow() {
        return result;
    }
    let introduced = introducing_commits(&project, spec_rel, rules);
    for (rule, oid) in introduced {
        if let Some(attribution) = describe(&project.repository, oid) {
            result.insert(rule, attribution);
        }
    }
    result
}

/// The introducing commit of each rule whose run is fully known.
fn introducing_commits(
    project: &ProjectRepository,
    spec_rel: &str,
    rules: &[String],
) -> BTreeMap<String, git2::Oid> {
    let repository = &project.repository;
    let mut introduced = BTreeMap::new();
    let Ok(mut commit) = repository.head().and_then(|head| head.peel_to_commit()) else {
        return introduced;
    };
    let mut open: BTreeSet<&String> = rules.iter().collect();
    let mut git_path = project.to_git(spec_rel);
    let mut cache: Option<(git2::Oid, BTreeSet<String>)> = None;
    loop {
        let Ok(held) = rules_at(repository, &commit, &git_path, &mut cache) else {
            for rule in &open {
                introduced.remove(*rule);
            }
            return introduced;
        };
        open.retain(|rule| {
            let present = held.contains(*rule);
            if present {
                introduced.insert((*rule).clone(), commit.id());
            }
            present
        });
        if open.is_empty() {
            return introduced;
        }
        let Ok(parent) = commit.parent(0) else {
            return introduced;
        };
        if let Some(earlier) = moved_from(repository, &parent, &commit, &git_path) {
            git_path = earlier;
        }
        commit = parent;
    }
}

/// The rule IDs the spec at `git_path` names in `commit`'s tree: none when the
/// tree has no such file, an error when it cannot be read as text. `cache`
/// keeps the last blob read, since most commits leave the spec as it was.
fn rules_at(
    repository: &git2::Repository,
    commit: &git2::Commit<'_>,
    git_path: &str,
    cache: &mut Option<(git2::Oid, BTreeSet<String>)>,
) -> Result<BTreeSet<String>, git2::Error> {
    let entry = match commit.tree()?.get_path(Path::new(git_path)) {
        Ok(entry) => entry,
        Err(err) if err.code() == git2::ErrorCode::NotFound => return Ok(BTreeSet::new()),
        Err(err) => return Err(err),
    };
    if let Some((_, held)) = cache.as_ref().filter(|(oid, _)| *oid == entry.id()) {
        return Ok(held.clone());
    }
    let blob = repository.find_blob(entry.id())?;
    let text = std::str::from_utf8(blob.content())
        .map_err(|_| git2::Error::from_str("the spec is not UTF-8 text"))?;
    let held: BTreeSet<String> = parse(text)
        .entries
        .into_iter()
        .map(|entry| entry.rule)
        .collect();
    *cache = Some((entry.id(), held.clone()));
    Ok(held)
}

/// Where `git_path`, which `commit` holds and `parent` does not, was in
/// `parent`: the old path of a rename the diff detects, else `None`. The one
/// delta whose new path is `git_path` is the file's add, or its rename once
/// similarity detection has paired it with a delete.
fn moved_from(
    repository: &git2::Repository,
    parent: &git2::Commit<'_>,
    commit: &git2::Commit<'_>,
    git_path: &str,
) -> Option<String> {
    let parent_tree = parent.tree().ok()?;
    if parent_tree.get_path(Path::new(git_path)).is_ok() {
        return None;
    }
    let mut diff = repository
        .diff_tree_to_tree(Some(&parent_tree), Some(&commit.tree().ok()?), None)
        .ok()?;
    diff.find_similar(None).ok()?;
    let old = diff
        .deltas()
        .find(|delta| {
            delta
                .new_file()
                .path()
                .is_some_and(|new| new == Path::new(git_path))
        })
        .and_then(|delta| delta.old_file().path())
        .map(|old| old.to_string_lossy().replace('\\', "/"))?;
    // Similarity detection pairs any deleted file with a similar added one, so
    // a rename is followed only within one spec: the same feature directory
    // identity, which a number never reissued makes the same spec.
    (spec_form(&old).is_some() && spec_form(&old) == spec_form(git_path)).then_some(old)
}

/// The feature a spec file's directory names, by the corpus's one membership
/// rule.
fn spec_form(git_path: &str) -> Option<FeatureForm> {
    Path::new(git_path)
        .parent()?
        .file_name()?
        .to_str()
        .and_then(parse_feature_dir)
}

/// The attribution a commit gives: its author's email and time, or `None`
/// when either cannot be read.
fn describe(repository: &git2::Repository, oid: git2::Oid) -> Option<Attribution> {
    let commit = repository.find_commit(oid).ok()?;
    let author = commit.author();
    let email = author.email().ok()?.to_string();
    let seconds = u64::try_from(author.when().seconds()).ok()?;
    Some(Attribution::Attributed {
        decided_by: email,
        decided_at: crate::session::iso8601_utc(UNIX_EPOCH + Duration::from_secs(seconds)),
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{subdirectory_project, write};
    use git2::{IndexAddOption, Repository, Signature, Time};
    use std::path::PathBuf;

    const SPEC: &str = "specs/001-x/spec.md";

    fn spec_with(entries: &str) -> String {
        format!("# Spec\n\n## Applicable Rules\n\n{entries}\n## Next\n\ntext\n")
    }

    fn entry(rule: &str) -> String {
        format!("- departs: `{rule}`\n  - reason: because\n")
    }

    // -- the parser ---------------------------------------------------------

    #[test]
    fn a_well_formed_entry_is_read_with_its_reason_citation_and_line() {
        let parsed = parse(&spec_with(
            "- departs: `BE-X-001`\n  - reason: the field is removed\n  - cited: §shared-exceptions\n",
        ));
        assert_eq!(parsed.problems, vec![]);
        assert_eq!(
            parsed.entries,
            vec![Entry {
                rule: "BE-X-001".into(),
                reason: "the field is removed".into(),
                cited: Some("§shared-exceptions".into()),
                line: 5,
            }]
        );
    }

    #[test]
    fn cited_is_optional_and_a_wrapped_reason_is_joined() {
        let parsed = parse(&spec_with(
            "- departs: `BE-X-001`\n  - reason: first half\n    second half\n",
        ));
        assert_eq!(parsed.problems, vec![]);
        assert_eq!(parsed.entries[0].reason, "first half second half");
        assert_eq!(parsed.entries[0].cited, None);
    }

    #[test]
    fn a_cited_child_with_no_text_names_no_citation() {
        let parsed = parse(&spec_with("- departs: `A-1`\n  - reason: r\n  - cited:\n"));
        assert_eq!(parsed.problems, vec![]);
        assert_eq!(parsed.entries[0].cited, None);
    }

    #[test]
    fn star_and_plus_list_markers_open_entries_and_children() {
        let parsed = parse(&spec_with(
            "* departs: `A-1`\n  + reason: r\n  - cited: §x\n+ departs: `B-1`\n  * reason: s\n",
        ));
        assert_eq!(parsed.problems, vec![]);
        assert_eq!(
            parsed
                .entries
                .iter()
                .map(|e| (e.rule.as_str(), e.reason.as_str()))
                .collect::<Vec<_>>(),
            vec![("A-1", "r"), ("B-1", "s")]
        );
    }

    #[test]
    fn an_unindented_line_after_the_reason_continues_it_as_commonmark_renders() {
        let parsed = parse(&spec_with(
            "- departs: `A-1`\n  - reason: first half\nsecond half\n",
        ));
        assert_eq!(parsed.problems, vec![]);
        assert_eq!(parsed.entries[0].reason, "first half second half");
    }

    #[test]
    fn an_unindented_paragraph_after_a_blank_line_is_not_part_of_the_reason() {
        let parsed = parse(&spec_with(
            "- departs: `A-1`\n  - reason: r\n\nunrelated prose\n",
        ));
        assert_eq!(parsed.problems, vec![]);
        assert_eq!(parsed.entries[0].reason, "r");
    }

    #[test]
    fn the_legacy_citation_bullet_is_no_entry_and_no_problem() {
        let parsed = parse(&spec_with(
            "- `BE-X-001` — reason it applies\n- plain text\n",
        ));
        assert_eq!(parsed, Parsed::default());
    }

    #[test]
    fn an_entry_in_a_comment_or_a_fence_is_no_entry() {
        let parsed = parse(&spec_with(
            "<!--\n- departs: `A-1`\n  - reason: r\n-->\n```markdown\n- departs: `B-1`\n  - reason: r\n```\n",
        ));
        assert_eq!(parsed, Parsed::default());
    }

    #[test]
    fn an_entry_outside_applicable_rules_is_no_entry() {
        let before = parse(&format!(
            "{}- departs: `A-1`\n  - reason: r\n",
            spec_with("")
        ));
        assert_eq!(before, Parsed::default());
        let parsed = parse("# Spec\n\n## Other\n\n- departs: `A-1`\n  - reason: r\n");
        assert_eq!(parsed, Parsed::default());
    }

    #[test]
    fn an_entry_with_no_rule_id_is_a_problem_and_no_entry() {
        let parsed = parse(&spec_with("- departs: nothing\n  - reason: r\n"));
        assert_eq!(parsed.entries, vec![]);
        assert_eq!(
            parsed.problems,
            vec![Problem {
                line: 5,
                kind: ProblemKind::MissingRule
            }]
        );
    }

    #[test]
    fn a_second_rule_id_makes_the_entry_malformed() {
        let parsed = parse(&spec_with("- departs: `A-1` and `B-1`\n  - reason: r\n"));
        assert_eq!(parsed.entries, vec![]);
        assert_eq!(
            parsed.problems,
            vec![Problem {
                line: 5,
                kind: ProblemKind::SecondRule
            }]
        );
    }

    #[test]
    fn a_missing_reason_makes_the_entry_malformed() {
        let parsed = parse(&spec_with("- departs: `A-1`\n  - cited: §x\n"));
        assert_eq!(parsed.entries, vec![]);
        assert_eq!(
            parsed.problems,
            vec![Problem {
                line: 5,
                kind: ProblemKind::MissingReason
            }]
        );
    }

    #[test]
    fn an_empty_reason_makes_the_entry_malformed() {
        let parsed = parse(&spec_with("- departs: `A-1`\n  - reason:\n"));
        assert_eq!(parsed.entries, vec![]);
        assert_eq!(
            parsed.problems,
            vec![Problem {
                line: 5,
                kind: ProblemKind::EmptyReason
            }]
        );
    }

    #[test]
    fn another_child_key_makes_the_entry_malformed_at_the_child_line() {
        let parsed = parse(&spec_with("- departs: `A-1`\n  - reason: r\n  - why: r\n"));
        assert_eq!(parsed.entries, vec![]);
        assert_eq!(
            parsed.problems,
            vec![Problem {
                line: 7,
                kind: ProblemKind::UnknownKey("why".into())
            }]
        );
    }

    #[test]
    fn a_repeated_child_key_makes_the_entry_malformed() {
        let parsed = parse(&spec_with(
            "- departs: `A-1`\n  - reason: r\n  - reason: s\n",
        ));
        assert_eq!(parsed.entries, vec![]);
        assert_eq!(
            parsed.problems,
            vec![Problem {
                line: 7,
                kind: ProblemKind::DuplicateKey("reason".into())
            }]
        );
    }

    #[test]
    fn a_rule_two_entries_name_is_reported_and_both_entries_stay() {
        let parsed = parse(&spec_with(&format!("{}{}", entry("A-1"), entry("A-1"))));
        assert_eq!(parsed.entries.len(), 2);
        assert_eq!(
            parsed.problems,
            vec![Problem {
                line: 7,
                kind: ProblemKind::DuplicateRule("A-1".into())
            }]
        );
    }

    #[test]
    fn a_malformed_entry_does_not_swallow_the_next_entry() {
        let parsed = parse(&spec_with(&format!("- departs: `A-1`\n{}", entry("B-1"))));
        assert_eq!(
            parsed
                .entries
                .iter()
                .map(|e| e.rule.as_str())
                .collect::<Vec<_>>(),
            vec!["B-1"]
        );
    }

    // -- attribution --------------------------------------------------------

    /// Commit the whole work tree (deletions included) as `email` at `time`.
    fn commit_as(repository: &Repository, email: &str, time: i64) -> git2::Oid {
        let mut index = repository.index().unwrap();
        index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
        index.update_all(["*"], None).unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let signature = Signature::new("N", email, &Time::new(time, 0)).unwrap();
        let parent = repository
            .head()
            .ok()
            .and_then(|head| head.peel_to_commit().ok());
        let parents: Vec<&git2::Commit> = parent.iter().collect();
        repository
            .commit(Some("HEAD"), &signature, &signature, "c", &tree, &parents)
            .unwrap()
    }

    /// Commit the work tree with explicit signatures and parents; `HEAD`
    /// moves only when `advance` is set.
    fn commit_with(
        repository: &Repository,
        author: &Signature<'_>,
        committer: &Signature<'_>,
        parents: &[git2::Oid],
        advance: bool,
    ) -> git2::Oid {
        let mut index = repository.index().unwrap();
        index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
        index.update_all(["*"], None).unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let parents: Vec<git2::Commit> = parents
            .iter()
            .map(|oid| repository.find_commit(*oid).unwrap())
            .collect();
        let parents: Vec<&git2::Commit> = parents.iter().collect();
        repository
            .commit(
                advance.then_some("HEAD"),
                author,
                committer,
                "c",
                &tree,
                &parents,
            )
            .unwrap()
    }

    fn sig(email: &str, time: i64, offset_minutes: i32) -> Signature<'static> {
        Signature::new("N", email, &Time::new(time, offset_minutes)).unwrap()
    }

    fn who(rule: &str, project: &Path, spec_rel: &str) -> Attribution {
        attribute(project, spec_rel, &[rule.to_string()])
            .remove(rule)
            .unwrap()
    }

    fn by(email: &str, at: &str) -> Attribution {
        Attribution::Attributed {
            decided_by: email.into(),
            decided_at: at.into(),
        }
    }

    /// 2021-01-01T00:00:00Z, one day later each step.
    const T1: i64 = 1_609_459_200;
    const DAY: i64 = 86_400;

    fn repository() -> (tempfile::TempDir, Repository) {
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        (tmp, repository)
    }

    #[test]
    fn an_entry_is_attributed_to_the_commit_that_introduced_it() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(""));
        commit_as(&repository, "zed@example.com", T1);
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1 + DAY);
        write(&tmp.path().join("other.txt"), "x");
        commit_as(&repository, "bob@example.com", T1 + 2 * DAY);
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("alice@example.com", "2021-01-02T00:00:00Z")
        );
    }

    #[test]
    fn an_edit_after_the_commit_keeps_the_introducing_commit() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1);
        write(
            &tmp.path().join(SPEC),
            &spec_with(&format!("{}  - cited: §x\n", entry("A-1"))),
        );
        commit_as(&repository, "bob@example.com", T1 + DAY);
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("alice@example.com", "2021-01-01T00:00:00Z")
        );
    }

    #[test]
    fn an_entry_removed_and_added_back_is_introduced_by_the_re_adding_commit() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1);
        write(&tmp.path().join(SPEC), &spec_with(""));
        commit_as(&repository, "bob@example.com", T1 + DAY);
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "carol@example.com", T1 + 2 * DAY);
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("carol@example.com", "2021-01-03T00:00:00Z")
        );
    }

    #[test]
    fn each_rule_is_attributed_by_its_own_run() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1);
        write(
            &tmp.path().join(SPEC),
            &spec_with(&format!("{}{}", entry("A-1"), entry("B-1"))),
        );
        commit_as(&repository, "bob@example.com", T1 + DAY);
        let both = attribute(tmp.path(), SPEC, &["A-1".into(), "B-1".into()]);
        assert_eq!(both["A-1"], by("alice@example.com", "2021-01-01T00:00:00Z"));
        assert_eq!(both["B-1"], by("bob@example.com", "2021-01-02T00:00:00Z"));
    }

    #[test]
    fn a_side_branchs_commit_is_no_part_of_the_first_parent_run() {
        let (tmp, repository) = repository();
        let spec = tmp.path().join(SPEC);
        write(&spec, &spec_with(""));
        let base = commit_as(&repository, "zed@example.com", T1);
        // The entry is introduced on a side branch...
        write(&spec, &spec_with(&entry("A-1")));
        let side = commit_with(
            &repository,
            &sig("alice@example.com", T1 + DAY, 0),
            &sig("alice@example.com", T1 + DAY, 0),
            &[base],
            false,
        );
        // ...while the first-parent line carries on without it...
        write(&spec, &spec_with(""));
        write(&tmp.path().join("other.txt"), "x");
        let main = commit_with(
            &repository,
            &sig("bob@example.com", T1 + 2 * DAY, 0),
            &sig("bob@example.com", T1 + 2 * DAY, 0),
            &[base],
            true,
        );
        // ...and a merge by another author brings the entry in.
        write(&spec, &spec_with(&entry("A-1")));
        commit_with(
            &repository,
            &sig("carol@example.com", T1 + 3 * DAY, 0),
            &sig("carol@example.com", T1 + 3 * DAY, 0),
            &[main, side],
            true,
        );
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("carol@example.com", "2021-01-04T00:00:00Z")
        );
    }

    #[test]
    fn the_author_and_the_utc_instant_are_recorded_not_the_committer_or_the_offset() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        // Authored 2021-01-01T00:00:00Z at +05:30 (local 05:30); committed by
        // someone else a day later at another offset.
        commit_with(
            &repository,
            &sig("alice@example.com", T1, 330),
            &sig("dan@example.com", T1 + DAY, -120),
            &[],
            true,
        );
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("alice@example.com", "2021-01-01T00:00:00Z")
        );
    }

    #[test]
    fn two_renames_in_one_commit_are_each_followed_to_their_own_spec() {
        let (tmp, repository) = repository();
        let a = format!("{}  - cited: §a\n", entry("A-1"));
        let b = format!("{}  - cited: §b\n", entry("B-1"));
        write(&tmp.path().join("specs/001-a/spec.md"), &spec_with(&a));
        write(&tmp.path().join("specs/002-b/spec.md"), &spec_with(&b));
        commit_as(&repository, "alice@example.com", T1);
        std::fs::remove_dir_all(tmp.path().join("specs/001-a")).unwrap();
        std::fs::remove_dir_all(tmp.path().join("specs/002-b")).unwrap();
        write(&tmp.path().join("specs/001-a2/spec.md"), &spec_with(&a));
        write(&tmp.path().join("specs/002-b2/spec.md"), &spec_with(&b));
        commit_as(&repository, "bob@example.com", T1 + DAY);
        assert_eq!(
            who("A-1", tmp.path(), "specs/001-a2/spec.md"),
            by("alice@example.com", "2021-01-01T00:00:00Z")
        );
        assert_eq!(
            who("B-1", tmp.path(), "specs/002-b2/spec.md"),
            by("alice@example.com", "2021-01-01T00:00:00Z")
        );
    }

    #[test]
    fn a_similar_file_of_another_spec_is_not_followed() {
        let (tmp, repository) = repository();
        write(
            &tmp.path().join("specs/002-other/spec.md"),
            &spec_with(&entry("A-1")),
        );
        commit_as(&repository, "alice@example.com", T1);
        // Spec 002 is deleted and 001 added with the same text, which
        // similarity detection pairs; they are two specs, so the add stands.
        std::fs::remove_dir_all(tmp.path().join("specs/002-other")).unwrap();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "bob@example.com", T1 + DAY);
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("bob@example.com", "2021-01-02T00:00:00Z")
        );
    }

    #[test]
    fn an_uncommitted_entry_is_unattributed() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(""));
        commit_as(&repository, "alice@example.com", T1);
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        assert_eq!(who("A-1", tmp.path(), SPEC), Attribution::Unattributed);
    }

    #[test]
    fn an_entry_removed_from_head_is_unattributed() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1);
        write(&tmp.path().join(SPEC), &spec_with(""));
        commit_as(&repository, "bob@example.com", T1 + DAY);
        assert_eq!(who("A-1", tmp.path(), SPEC), Attribution::Unattributed);
    }

    #[test]
    fn a_malformed_entry_at_head_is_unattributed() {
        let (tmp, repository) = repository();
        write(
            &tmp.path().join(SPEC),
            &spec_with("- departs: `A-1`\n  - cited: §x\n"),
        );
        commit_as(&repository, "alice@example.com", T1);
        assert_eq!(who("A-1", tmp.path(), SPEC), Attribution::Unattributed);
    }

    #[test]
    fn a_project_with_no_repository_or_no_commit_is_unattributed() {
        let tmp = tempfile::tempdir().unwrap();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        assert_eq!(who("A-1", tmp.path(), SPEC), Attribution::Unattributed);
        Repository::init(tmp.path()).unwrap();
        assert_eq!(who("A-1", tmp.path(), SPEC), Attribution::Unattributed);
    }

    #[test]
    fn a_renamed_spec_directory_keeps_the_introducing_commit() {
        let (tmp, repository) = repository();
        let old = "specs/001-old-name/spec.md";
        write(&tmp.path().join(old), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1);
        std::fs::remove_dir_all(tmp.path().join("specs/001-old-name")).unwrap();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "bob@example.com", T1 + DAY);
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("alice@example.com", "2021-01-01T00:00:00Z")
        );
    }

    #[test]
    fn a_file_added_with_no_rename_to_follow_is_introduced_by_the_add() {
        let (tmp, repository) = repository();
        write(
            &tmp.path().join("specs/001-old-name/spec.md"),
            "# Unrelated\n",
        );
        commit_as(&repository, "alice@example.com", T1);
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "bob@example.com", T1 + DAY);
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("bob@example.com", "2021-01-02T00:00:00Z")
        );
    }

    #[test]
    fn a_project_in_a_repository_subdirectory_is_attributed_through_its_prefix() {
        let tmp = tempfile::tempdir().unwrap();
        let (repository, project): (Repository, PathBuf) = subdirectory_project(tmp.path());
        // The same spec path at the work tree's root is another project's.
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "other@example.com", T1);
        write(&project.join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1 + DAY);
        assert_eq!(
            who("A-1", &project, SPEC),
            by("alice@example.com", "2021-01-02T00:00:00Z")
        );
    }

    #[test]
    fn a_shallow_repository_is_unattributed() {
        let (tmp, repository) = repository();
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "alice@example.com", T1);
        assert_eq!(
            who("A-1", tmp.path(), SPEC),
            by("alice@example.com", "2021-01-01T00:00:00Z")
        );
        let head = repository.head().unwrap().target().unwrap();
        std::fs::write(tmp.path().join(".git/shallow"), format!("{head}\n")).unwrap();
        assert!(Repository::open(tmp.path()).unwrap().is_shallow());
        assert_eq!(who("A-1", tmp.path(), SPEC), Attribution::Unattributed);
    }

    #[test]
    fn an_unreadable_revision_leaves_the_run_unattributed() {
        let (tmp, repository) = repository();
        std::fs::create_dir_all(tmp.path().join("specs/001-x")).unwrap();
        std::fs::write(tmp.path().join(SPEC), [0xff, 0xfe, 0x00]).unwrap();
        commit_as(&repository, "alice@example.com", T1);
        write(&tmp.path().join(SPEC), &spec_with(&entry("A-1")));
        commit_as(&repository, "bob@example.com", T1 + DAY);
        assert_eq!(who("A-1", tmp.path(), SPEC), Attribution::Unattributed);
    }
}
