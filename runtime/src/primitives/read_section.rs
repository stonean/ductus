//! `read-section` — the sections of a markdown document a step points at
//! (spec 080, scenario `reference-reads-follow-the-cited-sections`).
//!
//! A command's reference is read where a step points at one of its
//! sections, and reading the whole file put every section in context for
//! the rest of the run. This returns only the sections named, each cut by
//! the boundary the batching line's citations use
//! ([`resolve_anchor::section_range`]), so what a command's line expects a
//! section to call and what the model reads of it are the same text.

use std::path::Path;

use crate::primitives::resolve_anchor::{heading_lines, section_range};
use crate::primitives::{Result, read_text};
use crate::schema::primitives::{ListsForm, ReadSectionArgs, ReadSectionResult, Section};

/// Execute the `read-section` primitive: each heading asked for, in order,
/// with its section, and those the document lacks under `not-found`. The
/// sections are returned inline while the result fits `[artifacts]
/// read-size-bytes`, and past it in the file the result names, with their
/// count inline (`bounded_lists::place`; AC7).
///
/// # Errors
///
/// [`crate::primitives::PrimitiveError::InvalidArgument`] when `headings` is
/// empty, as the CLI requires one `--heading`, since such a call reads
/// nothing; [`crate::primitives::PrimitiveError::InvalidPath`] when `path`
/// leaves the project; [`crate::primitives::PrimitiveError::Io`] when the
/// document cannot be read, or is not UTF-8 text; and what
/// `bounded_lists::place` returns for a malformed read size or a list file
/// that cannot be written.
pub fn run(args: &ReadSectionArgs, repo: &Path) -> Result<ReadSectionResult> {
    if args.headings.is_empty() {
        return Err(crate::primitives::PrimitiveError::InvalidArgument {
            primitive: "read-section".into(),
            argument: "headings".into(),
            reason: "it names no section, so the call would read nothing".into(),
        });
    }
    super::validate_contained(repo, &args.path)?;
    let content = read_text(&repo.join(&args.path))?;
    let lines: Vec<&str> = content.lines().collect();
    let headings = heading_lines(&content);
    let mut sections = Vec::new();
    let mut not_found = Vec::new();
    for heading in &args.headings {
        match section_range(&headings, heading, lines.len()) {
            Some(range) => sections.push(Section {
                heading: heading.clone(),
                text: lines[range].join("\n"),
            }),
            None => not_found.push(heading.clone()),
        }
    }
    let mut result = ReadSectionResult {
        path: args.path.clone(),
        sections_count: sections.len(),
        sections: Some(sections),
        not_found,
        form: ListsForm::Inline,
        lists_file: None,
    };
    let lists = serde_json::json!({ "sections": &result.sections });
    let key = Path::new(&args.path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| {
            !stem.is_empty()
                && stem
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        })
        .unwrap_or("document");
    if let Some(file) = super::bounded_lists::place(repo, "read-section", key, &result, &lists)? {
        result.form = ListsForm::File;
        result.sections = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::fs;

    use super::*;
    use crate::primitives::PrimitiveError;

    const REFERENCE: &str = "# plan — reference\n\n## Detect\n\nDetect text.\n\n\
                             ### Detail\n\nNested text.\n\n## Validation gate\n\n\
                             Invoke `derive-boundary`.\n\n```text\n## Not a heading\n```\n\n\
                             ## Finalize\n\nFinal text.\n";

    fn project(reference: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus/reference")).unwrap();
        fs::write(tmp.path().join(".ductus/reference/plan.md"), reference).unwrap();
        tmp
    }

    fn args(headings: &[&str]) -> ReadSectionArgs {
        ReadSectionArgs {
            path: ".ductus/reference/plan.md".into(),
            headings: headings.iter().map(|h| (*h).to_owned()).collect(),
        }
    }

    /// No heading is refused on every surface, as the CLI's required
    /// `--heading` refuses it, rather than read as a call for nothing.
    #[test]
    fn no_heading_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("doc.md"), "# Doc\n").unwrap();
        let err = run(&args(&[]), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, crate::primitives::PrimitiveError::InvalidArgument { argument, .. } if argument == "headings"),
            "{err:?}"
        );
    }

    /// Each heading asked for comes back with its section, subsections
    /// included and up to the next heading of its level, in the order asked;
    /// a heading inside a fence is content; one the document lacks is named.
    #[test]
    fn each_heading_asked_for_comes_back_with_its_section_in_order() {
        let tmp = project(REFERENCE);
        let result = run(
            &args(&["Validation gate", "Detect", "Not a heading", "Missing"]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        assert_eq!(result.sections_count, 2);
        let sections = result.sections.unwrap();
        assert_eq!(sections[0].heading, "Validation gate");
        assert_eq!(
            sections[0].text,
            "## Validation gate\n\nInvoke `derive-boundary`.\n\n```text\n## Not a heading\n```\n"
        );
        assert_eq!(
            sections[1].text,
            "## Detect\n\nDetect text.\n\n### Detail\n\nNested text.\n"
        );
        assert_eq!(result.not_found, vec!["Not a heading", "Missing"]);
    }

    /// The boundary is the batching line's: the section `read-section`
    /// returns is the text `tool_loading::cited_tools` reads for the same
    /// citation, so a tool the line loads for a section is in what is read.
    #[test]
    fn the_section_is_the_text_the_line_reads() {
        let tmp = project(REFERENCE);
        let result = run(&args(&["Validation gate"]), tmp.path()).unwrap();
        let text = &result.sections.unwrap()[0].text;
        let command = "This command's reference is `.ductus/reference/plan.md`.\n\n\
                       ## Instructions\n\n1. Run **§Validation gate** in \
                       `.ductus/reference/plan.md`.\n";
        let cited = crate::primitives::tool_loading::cited_tools(command, "plan", |_| {
            Ok::<String, String>(REFERENCE.to_owned())
        })
        .unwrap();
        assert!(cited.contains("derive-boundary"));
        assert!(text.contains("Invoke `derive-boundary`"));
    }

    /// A path leaving the project is refused before anything is read.
    #[test]
    fn a_path_outside_the_project_is_refused() {
        let tmp = project(REFERENCE);
        let err = run(
            &ReadSectionArgs {
                path: "../outside.md".into(),
                headings: vec!["Detect".into()],
            },
            tmp.path(),
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
    }

    /// Past the read size the sections move to the file the result names,
    /// with their count inline (AC7).
    #[test]
    fn past_the_read_size_the_sections_move_to_a_file() {
        let big = format!("## Big\n\n{}\n", "word ".repeat(400));
        let tmp = project(&big);
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 1000\n",
        )
        .unwrap();
        let result = run(&args(&["Big"]), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert!(result.sections.is_none());
        assert_eq!(result.sections_count, 1);
        let file = result.lists_file.clone().unwrap();
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(tmp.path().join(&file)).unwrap()).unwrap();
        assert_eq!(lists["sections"][0]["heading"], "Big");
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "read-section",
            &serde_json::to_value(&result).unwrap(),
            &lists,
        );
    }
}
