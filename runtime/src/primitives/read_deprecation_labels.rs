//! `read-deprecation-labels` — the deprecation labels of rule files, one entry
//! for each line containing `**DEPRECATED` (spec 067, scenario
//! deprecation-labels-are-read-by-the-runtime).
//!
//! Which lines label which rules is [`section_label_lines`], the reader
//! `check-rule-ids` calls a rule deprecated by, so the two cannot disagree.
//! What this adds is Family 40's grammar for the label itself
//! (`specs/008-security-rules/data-model.md`, the Deprecated field), which
//! the release audit's `scripts/audit/deprecation-removal.sh` used to apply
//! in its own parse.

#![allow(clippy::expect_used)]

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::rule_sections::{heading_id_regex, section_label_lines};
use crate::primitives::{Result, contained_path, parse_atx_heading, read_text};
use crate::schema::primitives::{
    DeprecationLabel, LabelFollows, ListsForm, MalformedLabel, ReadDeprecationLabelsArgs,
    ReadDeprecationLabelsResult,
};

/// Where the form of a label is stated, cited by every fix.
const FORM: &str = "specs/008-security-rules/data-model.md, the Deprecated field";

/// Execute the `read-deprecation-labels` primitive: [`compute`], returned
/// through [`bound`].
///
/// # Errors
///
/// Those [`compute`] and [`bound`] return.
pub fn run(args: &ReadDeprecationLabelsArgs, repo: &Path) -> Result<ReadDeprecationLabelsResult> {
    bound(compute(args, repo)?, repo)
}

/// A read of rule files' labels, whole: what `read-deprecation-labels`
/// reports before its result is bounded. Each field is
/// [`ReadDeprecationLabelsResult`]'s of the same name.
#[derive(Clone, Debug, PartialEq)]
pub struct Labels {
    /// See [`ReadDeprecationLabelsResult::labels`].
    pub labels: Vec<DeprecationLabel>,
    /// See [`ReadDeprecationLabelsResult::unreadable`].
    pub unreadable: Vec<String>,
    /// See [`ReadDeprecationLabelsResult::examined`].
    pub examined: usize,
}

/// `read` as the primitive's result: inline while it fits `[artifacts]
/// read-size-bytes`, and past it with `labels` and `unreadable` in the file it
/// names and only their counts inline (`bounded_lists::place`; spec 067 task
/// 35, `BE-PAYLOAD-001`).
///
/// # Errors
///
/// Those `bounded_lists::place` returns for a malformed read size or a list
/// file that cannot be written.
pub fn bound(read: Labels, repo: &Path) -> Result<ReadDeprecationLabelsResult> {
    let lists = serde_json::json!({
        "labels": read.labels,
        "unreadable": read.unreadable,
    });
    let mut result = ReadDeprecationLabelsResult {
        labels_count: read.labels.len(),
        labels: Some(read.labels),
        unreadable_count: read.unreadable.len(),
        unreadable: Some(read.unreadable),
        examined: read.examined,
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) = super::bounded_lists::place(
        repo,
        "read-deprecation-labels",
        LISTS_SCOPE,
        &result,
        &lists,
    )? {
        result.form = ListsForm::File;
        result.labels = None;
        result.unreadable = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// The path segment this primitive's list file sits under, in place of the
/// feature `bounded_lists::place` keys others by, since a read takes paths.
const LISTS_SCOPE: &str = "paths";

/// Read the labels of every rule file `args` names, every list whole.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::InvalidPath`], before any file
/// is read, when a rule file path is absolute, climbs out with `..` or leaves
/// the project through a symlink ([`contained_path`]). A file that cannot be
/// read is reported in `unreadable`, not as an error.
pub fn compute(args: &ReadDeprecationLabelsArgs, repo: &Path) -> Result<Labels> {
    let paths = args
        .rule_files
        .iter()
        .map(|rule_file| contained_path(repo, rule_file))
        .collect::<Result<Vec<_>>>()?;
    let mut labels = Vec::new();
    let mut unreadable = Vec::new();
    let mut examined = 0;
    for (name, path) in args.rule_files.iter().zip(&paths) {
        let Ok(content) = read_text(path) else {
            unreadable.push(name.clone());
            continue;
        };
        examined += 1;
        labels.extend(read_file(name, &content));
    }
    Ok(Labels {
        labels,
        unreadable,
        examined,
    })
}

/// The labels of one rule file, by line.
fn read_file(name: &str, content: &str) -> Vec<DeprecationLabel> {
    let lines: Vec<&str> = content.lines().collect();
    // Line index -> the rules it is the first label of, and those it follows.
    let mut owners: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let mut follows: BTreeMap<usize, Vec<LabelFollows>> = BTreeMap::new();
    for cap in heading_id_regex().captures_iter(content) {
        let (Some(whole), Some(id)) = (cap.get(0), cap.get(1)) else {
            continue;
        };
        let in_section = section_label_lines(content, whole.start());
        let Some((&first, rest)) = in_section.split_first() else {
            continue;
        };
        owners
            .entry(first)
            .or_default()
            .push(id.as_str().to_string());
        for &extra in rest {
            follows.entry(extra).or_default().push(LabelFollows {
                rule: id.as_str().to_string(),
                first_line: first + 1,
            });
        }
    }
    lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("**DEPRECATED"))
        .map(|(index, _)| {
            let rules = owners.remove(&index).unwrap_or_default();
            let mut label = DeprecationLabel {
                file: name.to_string(),
                line: index + 1,
                follows: follows.remove(&index).unwrap_or_default(),
                rules: Vec::new(),
                deprecated_in: None,
                removal_target: None,
                malformed: None,
            };
            if !rules.is_empty() {
                grade(&mut label, &lines, index, &rules);
            }
            label.rules = rules;
            label
        })
        .collect()
}

/// Apply the label grammar to the line at `index`, which labels `rules`.
fn grade(label: &mut DeprecationLabel, lines: &[&str], index: usize, rules: &[String]) {
    let who = format!("{}'s", rules.join(" and "));
    let line = lines[index];
    let malformed = |reason: String, fix: String| Some(MalformedLabel { reason, fix });
    let Some(opening) = label_regex().captures(line) else {
        label.malformed = if line.starts_with("**DEPRECATED") {
            malformed(
                format!("{who} deprecation label is not in the form **DEPRECATED in X.Y.Z:**"),
                format!(
                    "write the label as **DEPRECATED in X.Y.Z:**, naming the deprecating release ({FORM})"
                ),
            )
        } else {
            malformed(
                format!("{who} deprecation label does not open its line"),
                format!("start the line with the label, outside any blockquote or indent ({FORM})"),
            )
        };
        return;
    };
    let deprecated_in = opening[1].trim_matches('`').to_string();
    if !semver_regex().is_match(&deprecated_in) {
        label.malformed = malformed(
            format!(
                "{who} label reads DEPRECATED in {deprecated_in}, which is not MAJOR.MINOR.PATCH"
            ),
            format!("name the deprecating release as MAJOR.MINOR.PATCH ({FORM})"),
        );
        return;
    }
    label.deprecated_in = Some(deprecated_in);
    let mut paragraph = vec![line];
    for following in &lines[index + 1..] {
        let trimmed = following.trim_matches(char::is_whitespace);
        if trimmed.is_empty()
            || parse_atx_heading(following).is_some()
            || trimmed.starts_with('>')
            || field_regex().is_match(trimmed)
            || block_regex().is_match(trimmed)
        {
            break;
        }
        paragraph.push(following);
    }
    let targets: Vec<String> = target_regex()
        .captures_iter(&paragraph.join(" "))
        .map(|cap| cap[1].trim_end_matches('.').to_string())
        .collect();
    let named_once = format!("name the target once, as removed in X.Y.Z ({FORM})");
    label.malformed = match targets.as_slice() {
        [] => malformed(
            format!(
                "{who} label names no removal target, so nothing holds a release to removing it"
            ),
            format!("add removed in X.Y.Z to the label ({FORM})"),
        ),
        [first, rest @ ..] if !rest.is_empty() && rest.iter().all(|t| t == first) => malformed(
            format!("{who} label names its removal target {first} more than once"),
            named_once,
        ),
        [_, _, ..] => malformed(
            format!(
                "{who} label names more than one removal target ({})",
                targets.join(", ")
            ),
            named_once,
        ),
        [only] if !semver_regex().is_match(only) => malformed(
            format!("{who} removal target {only} is not MAJOR.MINOR.PATCH"),
            format!("name the target as removed in X.Y.Z ({FORM})"),
        ),
        [only] => {
            label.removal_target = Some(only.clone());
            None
        }
    };
}

/// `**DEPRECATED in X:**` opening the line.
fn label_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"^\*\*DEPRECATED in ([^:*]*):\*\*").expect("hard-coded regex compiles")
    })
}

/// A `**Field:**` line, which ends a paragraph.
fn field_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^\*\*[^*]+:\*\*").expect("hard-coded regex compiles"))
}

/// A line that opens a list item or a fence, either of which ends a paragraph.
fn block_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"^(?:(?:[-*+]|1[.)])(?:[ \t]|$)|```|~~~)").expect("hard-coded regex compiles")
    })
}

/// `removed in` followed by something that starts like a version, in any case;
/// "removed in favour of" is prose.
fn target_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"(?i)\bremoved\s+in\s+`?(v?[0-9][0-9A-Za-z.+-]*)")
            .expect("hard-coded regex compiles")
    })
}

fn semver_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[0-9]+\.[0-9]+\.[0-9]+$").expect("hard-coded regex compiles"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn read(files: &[(&str, &str)]) -> Labels {
        let tmp = tempfile::tempdir().unwrap();
        for (name, body) in files {
            std::fs::write(tmp.path().join(name), body).unwrap();
        }
        compute(
            &ReadDeprecationLabelsArgs {
                rule_files: files.iter().map(|(n, _)| (*n).to_string()).collect(),
            },
            tmp.path(),
        )
        .unwrap()
    }

    const DUE: &str = "# Rules\n\n### BE-OLD-001\n\n**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`; removed in 0.62.0.\n\n> A MUST.\n";

    #[test]
    fn a_well_formed_label_gives_its_versions() {
        let result = read(&[("r.md", DUE)]);
        assert_eq!(result.examined, 1);
        let [label] = result.labels.as_slice() else {
            panic!("{:?}", result.labels);
        };
        assert_eq!(label.line, 5);
        assert_eq!(label.rules, ["BE-OLD-001"]);
        assert_eq!(label.deprecated_in.as_deref(), Some("0.60.0"));
        assert_eq!(label.removal_target.as_deref(), Some("0.62.0"));
        assert!(label.malformed.is_none() && label.follows.is_empty());
    }

    /// The shared reader: `check-rule-ids` and this primitive call a rule
    /// deprecated on the same lines, a blockquoted label included.
    #[test]
    fn the_primitive_labels_what_check_rule_ids_calls_deprecated() {
        let body = "### BE-OLD-001\n\n> **DEPRECATED in 0.60.0:** x; removed in 0.62.0.\n\n### BE-LIVE-002\n\n> A MUST.\n";
        let result = read(&[("r.md", body)]);
        let [label] = result.labels.as_slice() else {
            panic!("{:?}", result.labels);
        };
        assert_eq!(label.rules, ["BE-OLD-001"]);
        // A blockquoted label labels the rule but does not open its line.
        assert!(
            label
                .malformed
                .as_ref()
                .is_some_and(|m| m.reason.contains("does not open its line"))
        );
    }

    #[test]
    fn a_label_in_no_section_has_neither_list() {
        let result = read(&[(
            "r.md",
            "# Rules\n\n**DEPRECATED in 0.60.0:** x; removed in 0.62.0.\n",
        )]);
        let [label] = result.labels.as_slice() else {
            panic!("{:?}", result.labels);
        };
        assert!(label.rules.is_empty() && label.follows.is_empty() && label.malformed.is_none());
        assert_eq!(label.line, 3);
    }

    #[test]
    fn a_second_label_follows_the_first() {
        let body = "### BE-OLD-001\n\n**DEPRECATED in 0.60.0:** x; removed in 0.62.0.\n\n**DEPRECATED in 0.60.0:** y; removed in 0.63.0.\n";
        let result = read(&[("r.md", body)]);
        assert_eq!(result.labels.len(), 2);
        assert_eq!(result.labels[0].rules, ["BE-OLD-001"]);
        assert_eq!(result.labels[1].rules, [] as [String; 0]);
        assert_eq!(
            result.labels[1].follows,
            [LabelFollows {
                rule: "BE-OLD-001".into(),
                first_line: 3
            }]
        );
    }

    #[test]
    fn each_malformation_is_a_result_and_hides_no_other() {
        let body = "### BE-AA-001\n\n**DEPRECATED in 0.60.0:** moved.\n\n### BE-BB-002\n\n**DEPRECATED in 0.60:** x; removed in 0.62.0.\n\n### BE-CC-003\n\n**DEPRECATED in 0.60.0:** x; removed in 0.62.\n\n### BE-DD-004\n\n**DEPRECATED in 0.60.0:** x; removed in 0.62.0 and removed in 0.63.0.\n\n### BE-EE-005\n\n**DEPRECATED in 0.60.0:** removed in favour of BE-X-001.\n";
        let result = read(&[("r.md", body)]);
        let reasons: Vec<&str> = result
            .labels
            .iter()
            .map(|l| l.malformed.as_ref().unwrap().reason.as_str())
            .collect();
        assert_eq!(
            reasons,
            [
                "BE-AA-001's label names no removal target, so nothing holds a release to removing it",
                "BE-BB-002's label reads DEPRECATED in 0.60, which is not MAJOR.MINOR.PATCH",
                "BE-CC-003's removal target 0.62 is not MAJOR.MINOR.PATCH",
                "BE-DD-004's label names more than one removal target (0.62.0, 0.63.0)",
                "BE-EE-005's label names no removal target, so nothing holds a release to removing it",
            ]
        );
    }

    #[test]
    fn an_unreadable_file_is_reported_and_not_examined() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("bad.md"), b"### BE-X-001\n\xff\n").unwrap();
        std::fs::write(tmp.path().join("good.md"), DUE).unwrap();
        let result = compute(
            &ReadDeprecationLabelsArgs {
                rule_files: vec!["bad.md".into(), "good.md".into(), "missing.md".into()],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unreadable, ["bad.md", "missing.md"]);
        assert_eq!(result.examined, 1);
        assert_eq!(result.labels.len(), 1);
    }

    #[test]
    fn a_path_leaving_the_project_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let err = compute(
            &ReadDeprecationLabelsArgs {
                rule_files: vec!["../x.md".into()],
            },
            tmp.path(),
        );
        assert!(err.is_err());
    }

    /// Two rule files, one labelled and one that does not exist, so each
    /// list holds an entry.
    fn bounded_args() -> ReadDeprecationLabelsArgs {
        ReadDeprecationLabelsArgs {
            rule_files: vec!["r.md".into(), "gone.md".into()],
        }
    }

    fn bounded_project() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("r.md"), DUE).unwrap();
        tmp
    }

    fn read_size(repo: &Path, bytes: usize) {
        std::fs::create_dir_all(repo.join(".ductus")).unwrap();
        std::fs::write(
            repo.join(".ductus/config.toml"),
            format!("[artifacts]\nread-size-bytes = {bytes}\n"),
        )
        .unwrap();
    }

    /// Under the read size both lists are in the result with their counts,
    /// and no list file is written.
    #[test]
    fn the_lists_are_inline_while_the_result_fits() {
        let tmp = bounded_project();
        let result = run(&bounded_args(), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        assert_eq!(result.lists_file, None);
        assert_eq!(result.labels.as_ref().unwrap().len(), 1);
        assert_eq!(result.labels_count, 1);
        assert_eq!(result.unreadable, Some(vec!["gone.md".to_string()]));
        assert_eq!(result.unreadable_count, 1);
        assert_eq!(result.examined, 1);
        assert!(!tmp.path().join(".ductus/results").exists());
    }

    /// A result exactly at the read size fits; one byte under it moves both
    /// lists whole to the file the result names, their counts staying.
    #[test]
    fn the_lists_are_in_a_file_past_the_read_size() {
        let tmp = bounded_project();
        let whole = run(&bounded_args(), tmp.path()).unwrap();
        let size = serde_json::to_vec(&whole).unwrap().len();
        read_size(tmp.path(), size);
        assert_eq!(
            run(&bounded_args(), tmp.path()).unwrap().form,
            ListsForm::Inline
        );
        read_size(tmp.path(), size - 1);
        let bounded = run(&bounded_args(), tmp.path()).unwrap();
        assert_eq!(bounded.form, ListsForm::File);
        assert_eq!(bounded.labels, None);
        assert_eq!(bounded.unreadable, None);
        assert_eq!(bounded.labels_count, 1);
        assert_eq!(bounded.unreadable_count, 1);
        assert_eq!(bounded.examined, 1);
        let file = bounded.lists_file.clone().unwrap();
        assert!(
            file.starts_with(".ductus/results/read-deprecation-labels/paths/"),
            "{file}"
        );
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(&file)).unwrap())
                .unwrap();
        assert_eq!(
            lists["labels"],
            serde_json::to_value(&whole.labels).unwrap()
        );
        assert_eq!(lists["unreadable"], serde_json::json!(["gone.md"]));
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "read-deprecation-labels",
            &serde_json::to_value(&bounded).unwrap(),
            &lists,
        );
    }
}
