//! `check-step-references` — every `step N` reference in a command file
//! resolves to a numbered step that file actually has.
//!
//! A command file numbers its Instructions steps and then refers to them in
//! prose — "settled in step 4", "the confirmation in step 3 carries",
//! "refused by the primitive in step 5". The numbers are the only binding
//! between a reference and its target, and nothing checked it.
//!
//! They drift the moment a step is inserted or removed, because removing one
//! renumbers every step after it while the prose keeps the old numbers. Spec
//! 054 removed two steps from `specify.md` and one from `consolidate.md` and
//! left four stale references behind:
//!
//! - `specify.md` named `create-feature` at step 6 in two places after it
//!   moved to step 5.
//! - `consolidate.md`'s step 2 said "the confirmation in step 4 carries"
//!   after the confirmation moved to step 3.
//! - `consolidate.md`'s step 5 said "even though step 5 established the same
//!   fact", which had been step 6 pointing at step 5 and silently became a
//!   **self-reference** — the degenerate case, and the one least likely to be
//!   caught by reading, since the sentence still parses.
//!
//! Measured against that state: 4 findings before the repair, 0 after.
//!
//! All four were found by a human re-reading the files during an unrelated
//! review. Nothing in the test suite, the pre-commit hook, or the other audit
//! families would have reported any of them, and the procedure parser is no
//! help — it reads the numbered steps to build a `Procedure` and never looks
//! at the prose between them. That makes this a diligence dependency, which
//! §design-principles rejects.
//!
//! ## Three findings, not one
//!
//! **Unresolved** is the obvious one: a reference naming a number the file
//! does not have. **Self-reference** is not caught by existence — it resolves,
//! so a resolution check passes it — and is almost always renumbering residue.
//! **Discontinuous** numbering is what a partial removal leaves; it makes
//! every reference after the gap ambiguous rather than wrong, so it is
//! reported separately from the two that are definitely wrong.
//!
//! ## Scope: the Instructions section, and that bound is in the result
//!
//! Several command files carry a `## Markdown-only reference` whose
//! sub-procedures restart at 1. Those are separate numbered lists, so
//! resolving an Instructions reference against them — or the reverse — would
//! manufacture findings. The subject is therefore the **Instructions**
//! section alone, and [`references_out_of_subject`] counts the `step N`
//! mentions elsewhere in each file so an empty `findings` is never read as
//! *every reference in the file resolves* (`QUAL-CLAIM-001`).
//!
//! A qualified reference — `groom's step 3` — names another file's procedure
//! and is excluded by construction rather than filtered afterward: resolving
//! it against the citing file's own step set is exactly how a check like this
//! invents findings.
//!
//! [`references_out_of_subject`]: crate::schema::primitives::CheckStepReferencesResult::references_out_of_subject

use std::path::Path;

use crate::primitives::{Result, read_text, rel_path, section_line_indices};
use crate::schema::primitives::{
    CheckStepReferencesArgs, CheckStepReferencesResult, StepReferenceFinding, StepReferenceSkip,
};

/// Command sources. The generated copies under a host's commands directory
/// carry whatever their source carries, so checking both would report every
/// finding twice and neither copy would be the one to fix.
const COMMANDS_DIR: &str = "framework/commands";

/// The bootstrap procedures, which number their steps the same way and drift
/// the same way.
const BOOTSTRAP_FILES: [&str; 2] = [
    "framework/bootstrap/ductus.md",
    "framework/bootstrap/govern.md",
];

/// The heading whose numbered list is the subject.
const SUBJECT_SECTION: &str = "Instructions";

/// Execute the `check-step-references` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError`] only on a failure that makes
/// the whole run meaningless. An unreadable individual file is a skip, not an
/// error: nothing can be proven about a file that will not open, and a read
/// that died on one would say nothing about the files it could read.
// One flat pass over the subject files: enumerate, classify, resolve. Split
// into helpers the branches would only be threaded back through — the same
// reason `main`'s CLI match and `dispatch_primitive` carry this allow.
#[allow(clippy::too_many_lines)]
pub fn run(_args: &CheckStepReferencesArgs, repo: &Path) -> Result<CheckStepReferencesResult> {
    let mut examined = Vec::new();
    let mut with_steps = Vec::new();
    let mut skipped = Vec::new();
    let mut findings = Vec::new();
    let mut not_a_procedure = Vec::new();
    let mut references_out_of_subject: u32 = 0;

    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    match std::fs::read_dir(repo.join(COMMANDS_DIR)) {
        Ok(entries) => {
            let mut dir_paths: Vec<_> = entries
                .filter_map(std::result::Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
                .collect();
            dir_paths.sort();
            paths.extend(dir_paths);
        }
        Err(err) => {
            skipped.push(StepReferenceSkip {
                path: COMMANDS_DIR.to_string(),
                reason: format!("command source directory could not be listed: {err}"),
            });
        }
    }
    for rel in BOOTSTRAP_FILES {
        let p = repo.join(rel);
        if p.is_file() {
            paths.push(p);
        }
    }

    for path in &paths {
        let rel = rel_path(path, repo);
        let content = match read_text(path) {
            Ok(text) => text,
            Err(err) => {
                skipped.push(StepReferenceSkip {
                    path: rel,
                    reason: format!("could not be read: {err}"),
                });
                continue;
            }
        };
        examined.push(rel.clone());

        let lines: Vec<&str> = content.lines().collect();
        let subject: Vec<usize> = section_line_indices(&lines, SUBJECT_SECTION);

        // References outside the subject are counted, never resolved — the
        // markdown-only sub-procedures restart at 1 and are a different list.
        let in_subject: std::collections::HashSet<usize> = subject.iter().copied().collect();
        for (idx, line) in lines.iter().enumerate() {
            if !in_subject.contains(&idx) {
                references_out_of_subject +=
                    u32::try_from(step_references(line).len()).unwrap_or(u32::MAX);
            }
        }

        if subject.is_empty() {
            // No Instructions section: not a subject. Examined, contributing
            // nothing, which is correct rather than a gap.
            continue;
        }

        let steps = step_numbers(&lines, &subject);

        // Is this one procedure, or several lists sharing a heading?
        //
        // Only a single ascending run starting at 1 is a procedure whose
        // numbers a reference can be resolved against. `amend.md` restarts at
        // 1 under each `###` subsection and `status.md` uses three separate
        // one-item lists; both are legitimate authoring, and MD029 is
        // disabled for these files so nothing pushes them to be otherwise.
        // Resolving a reference against a merged set of those would invent
        // findings, which is the failure this family must not have.
        let nums: Vec<u32> = steps.iter().map(|(n, _)| *n).collect();
        let is_procedure =
            nums.first() == Some(&1) && nums.windows(2).all(|w| w[1] > w[0]) && nums.len() >= 3;
        if !is_procedure {
            not_a_procedure.push(rel.clone());
            continue;
        }
        with_steps.push(rel.clone());

        let max = *nums.last().unwrap_or(&0);
        let count = u32::try_from(nums.len()).unwrap_or(u32::MAX);
        if max != count {
            // Ascending from 1 but with gaps — the residue of a partial
            // removal, which makes every reference after the gap ambiguous
            // rather than wrong.
            let present: Vec<String> = nums.iter().map(ToString::to_string).collect();
            findings.push(StepReferenceFinding {
                file: rel.clone(),
                line: 0,
                kind: "discontinuous".into(),
                reference: 0,
                message: format!(
                    "{rel}'s Instructions steps ascend from 1 but skip numbers — found {}, so the \
                     list runs to {max} with only {count} steps",
                    present.join(", ")
                ),
            });
        }

        let known: std::collections::HashSet<u32> = nums.iter().copied().collect();

        // Which step each subject line belongs to, so a self-reference is
        // attributable. A line before the first numbered step belongs to no
        // step and can never be one.
        let mut owner: u32 = 0;
        for &idx in &subject {
            let line = lines[idx];
            if let Some(n) = leading_step_number(line) {
                owner = n;
            }
            for r in step_references(line) {
                if !known.contains(&r) {
                    findings.push(StepReferenceFinding {
                        file: rel.clone(),
                        line: idx + 1,
                        kind: "unresolved".into(),
                        reference: r,
                        message: format!(
                            "{rel}:{} references step {r}, which does not exist — this file's \
                             Instructions has steps 1..{max}",
                            idx + 1
                        ),
                    });
                } else if owner != 0 && r == owner {
                    findings.push(StepReferenceFinding {
                        file: rel.clone(),
                        line: idx + 1,
                        kind: "self-reference".into(),
                        reference: r,
                        message: format!(
                            "{rel}:{} is inside step {owner} and refers to step {r} — a \
                             self-reference, which resolves and so is invisible to an existence \
                             check; it is almost always renumbering residue",
                            idx + 1
                        ),
                    });
                }
            }
        }
    }

    let guidance = if examined.is_empty() {
        format!("no command sources found under {COMMANDS_DIR} — nothing was examined")
    } else {
        String::new()
    };

    Ok(CheckStepReferencesResult {
        findings,
        examined,
        with_steps,
        not_a_procedure,
        skipped,
        references_out_of_subject,
        guidance,
    })
}

/// The step number a line opens with, when it opens a numbered step.
fn leading_step_number(line: &str) -> Option<u32> {
    let digits: String = line.chars().take_while(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return None;
    }
    let rest = &line[digits.len()..];
    if rest.starts_with(". ") {
        digits.parse().ok()
    } else {
        None
    }
}

/// The numbered steps inside the subject lines, in body order.
fn step_numbers(lines: &[&str], subject: &[usize]) -> Vec<(u32, usize)> {
    subject
        .iter()
        .filter_map(|&idx| leading_step_number(lines[idx]).map(|n| (n, idx)))
        .collect()
}

/// Every `step N` this line refers to.
///
/// Handles `step 4`, `steps 2 and 3`, and `steps 1–3` / `steps 1-3` (both
/// endpoints, not the span — naming an endpoint that does not exist is the
/// defect, and a range whose interior is missing is the discontinuity finding).
///
/// A **qualified** reference — `groom's step 3` — is excluded: it names
/// another file's procedure, and resolving it against the citing file's own
/// step set is how a check like this manufactures findings. A line that opens
/// a numbered step contributes its own leading number as a step, never as a
/// reference.
fn step_references(line: &str) -> Vec<u32> {
    let lower = line.to_lowercase();
    let bytes = lower.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;

    while let Some(rel) = lower[i..].find("step") {
        let start = i + rel;
        let mut cursor = start + 4;
        // `steps` as well as `step`.
        if lower[cursor..].starts_with('s') {
            cursor += 1;
        }
        // Must be followed by whitespace then a digit; `stepping` is not one.
        let after = &lower[cursor..];
        let trimmed = after.trim_start();
        let consumed = after.len() - trimmed.len();
        if consumed == 0 || !trimmed.starts_with(|c: char| c.is_ascii_digit()) {
            i = start + 4;
            continue;
        }

        // Qualified? Look at the token immediately before.
        let prefix = lower[..start].trim_end();
        let qualified = prefix.ends_with("'s") || prefix.ends_with('\u{2019}');
        if qualified {
            i = start + 4;
            continue;
        }

        // Collect the number list: digits separated by `and`, `,`, `-`, `–`.
        let mut walk = cursor + consumed;
        loop {
            let rest = &lower[walk..];
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            if digits.is_empty() {
                break;
            }
            if let Ok(n) = digits.parse::<u32>() {
                out.push(n);
            }
            walk += digits.len();
            let rest = &lower[walk..];
            let sep = ["–", "—", "-", " and ", ", and ", ", "];
            let Some(found) = sep.iter().find(|s| rest.starts_with(**s)) else {
                break;
            };
            walk += found.len();
        }
        i = walk.max(start + 4);
        let _ = bytes;
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    const DOC: &str = "\
# Cmd

## Instructions

1. First, see step 3.
2. Second.
3. Third, and this sentence refers to step 3.

## Markdown-only reference

1. Restarts here, mentioning step 9.
";

    #[test]
    fn the_subject_scan_finds_the_instructions_section() {
        let lines: Vec<&str> = DOC.lines().collect();
        let subject = section_line_indices(&lines, SUBJECT_SECTION);
        assert!(
            !subject.is_empty(),
            "Instructions section not found; scan returned nothing"
        );
        let steps = step_numbers(&lines, &subject);
        assert_eq!(
            steps.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            vec![1, 2, 3],
            "the markdown-only list must not join the step set"
        );
    }

    #[test]
    fn a_reference_list_is_extracted_in_every_shape() {
        assert_eq!(step_references("see step 4."), vec![4]);
        assert_eq!(step_references("skip steps 2 and 3 and go on"), vec![2, 3]);
        assert_eq!(
            step_references("the routing gate (steps 1-3) writes"),
            vec![1, 3]
        );
        assert_eq!(step_references("stepping carefully"), Vec::<u32>::new());
    }

    #[test]
    fn the_real_command_corpus_is_classified_as_it_reads() {
        // The guard against the subject silently narrowing. A change that
        // broke the section scan would leave every file unexamined and the
        // family would report clean over nothing.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let text = std::fs::read_to_string(root.join("framework/commands/specify.md")).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        let subject = section_line_indices(&lines, SUBJECT_SECTION);
        assert!(
            !subject.is_empty(),
            "no Instructions lines found in specify.md"
        );
        let nums: Vec<u32> = step_numbers(&lines, &subject)
            .iter()
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(
            nums,
            (1..=u32::try_from(nums.len()).unwrap()).collect::<Vec<_>>(),
            "specify.md's Instructions must be one contiguous procedure"
        );
    }

    #[test]
    fn a_self_reference_resolves_and_so_needs_its_own_finding() {
        // The 054 case: a step whose prose refers to its own number. It
        // resolves, so an existence check alone passes it.
        let lines: Vec<&str> = DOC.lines().collect();
        let subject = section_line_indices(&lines, SUBJECT_SECTION);
        let steps = step_numbers(&lines, &subject);
        let known: std::collections::HashSet<u32> = steps.iter().map(|(n, _)| *n).collect();
        let mut owner = 0;
        let mut self_refs = 0;
        for &idx in &subject {
            if let Some(n) = leading_step_number(lines[idx]) {
                owner = n;
            }
            for r in step_references(lines[idx]) {
                assert!(known.contains(&r), "step {r} should resolve in the fixture");
                if owner != 0 && r == owner {
                    self_refs += 1;
                }
            }
        }
        assert_eq!(self_refs, 1, "step 3 refers to itself in the fixture");
    }

    #[test]
    fn a_qualified_reference_names_another_files_procedure_and_is_excluded() {
        assert_eq!(step_references("as groom's step 3 does"), Vec::<u32>::new());
    }
}
