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
//! A command's reference carries sub-procedures that restart
//! at 1. It lives in its own file under `framework/reference/` (spec 071),
//! so it is outside the step subject, but a numbered list can still sit in a
//! command file outside its Instructions. Those are separate numbered lists,
//! so resolving an Instructions reference against them — or the reverse —
//! would manufacture findings. The subject is therefore the **Instructions**
//! section alone, and [`references_out_of_subject`] counts the `step N`
//! mentions elsewhere in each file so a zero `findings-count` is never read as
//! *every reference in the file resolves* (`QUAL-CLAIM-001`).
//!
//! A qualified reference — `groom's step 3` — names another file's procedure
//! and is excluded by construction rather than filtered afterward: resolving
//! it against the citing file's own step set is exactly how a check like this
//! invents findings.
//!
//! ## Reference sections a command reaches (spec 026, from 071's scenario)
//!
//! A command's reference is read by section, only where a step names it, so a
//! `##` section no numbered step of the command's `## Instructions` reaches
//! is never read ([`reach`]): a citation in a purpose, a scope list, a flags
//! table, the preamble or a note after a step reaches nothing. Each
//! `framework/reference/<command>.md` is judged against
//! `framework/commands/<command>.md` through the walk spec 080's tool loading
//! makes ([`cited_sections`]), so one walker decides what a citation reaches.
//! A section kept for a reader outside the command carries
//! `<!-- audit:kept <reason> -->` on the first non-blank line beneath its heading: it is
//! reported exempt with its reason, and the marker is a finding when its
//! reason is empty, when its section is reached anyway, or when it sits
//! anywhere else, so an exemption cannot outlive its cause. A reference with
//! no `##` section is counted apart, never a pass.
//!
//! [`references_out_of_subject`]: crate::schema::primitives::CheckStepReferencesResult::references_out_of_subject

use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::resolve_anchor::heading_lines;
use crate::primitives::tool_loading::cited_sections;
use crate::primitives::{Result, SkipScanner, read_text, rel_path, section_line_indices};
use crate::schema::primitives::{
    CheckStepReferencesArgs, CheckStepReferencesResult, ExemptSection, ListsForm, ReferenceReach,
    StepReferenceFinding, StepReferenceSkip,
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

/// The references whose sections the commands reach.
const REFERENCE_DIR: &str = "framework/reference";

/// The heading whose numbered list is the subject.
const SUBJECT_SECTION: &str = "Instructions";

/// Execute the `check-step-references` primitive: [`compute`], returned
/// through [`bound`].
///
/// # Errors
///
/// Only those [`bound`] returns. An unreadable individual file is a skip, not
/// an error: nothing can be proven about a file that will not open, and a
/// read that died on one would say nothing about the files it could read.
pub fn run(args: &CheckStepReferencesArgs, repo: &Path) -> Result<CheckStepReferencesResult> {
    bound(compute(args, repo), repo)
}

/// A step-reference scan, whole: what `check-step-references` reports before
/// its result is bounded. Each field is [`CheckStepReferencesResult`]'s of
/// the same name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct StepReferences {
    findings: Vec<StepReferenceFinding>,
    examined: Vec<String>,
    with_steps: Vec<String>,
    not_a_procedure: Vec<String>,
    skipped: Vec<StepReferenceSkip>,
    reachability: Vec<ReferenceReach>,
    sections_unreached: u32,
    references_out_of_subject: u32,
    guidance: String,
}

/// `scan` as the primitive's result: inline while it fits `[artifacts]
/// read-size-bytes`, and past it with its six lists in the file it names and
/// only their counts inline (`bounded_lists::place`; 026 task 6,
/// `BE-PAYLOAD-001`).
///
/// # Errors
///
/// Those `bounded_lists::place` returns for a malformed read size or a list
/// file that cannot be written.
fn bound(scan: StepReferences, repo: &Path) -> Result<CheckStepReferencesResult> {
    let lists = serde_json::json!({
        "findings": scan.findings,
        "examined": scan.examined,
        "with-steps": scan.with_steps,
        "not-a-procedure": scan.not_a_procedure,
        "skipped": scan.skipped,
        "reachability": scan.reachability,
    });
    let mut result = CheckStepReferencesResult {
        findings_count: scan.findings.len(),
        findings: Some(scan.findings),
        examined_count: scan.examined.len(),
        examined: Some(scan.examined),
        with_steps_count: scan.with_steps.len(),
        with_steps: Some(scan.with_steps),
        not_a_procedure_count: scan.not_a_procedure.len(),
        not_a_procedure: Some(scan.not_a_procedure),
        skipped_count: scan.skipped.len(),
        skipped: Some(scan.skipped),
        reachability_count: scan.reachability.len(),
        sections_examined: scan.reachability.iter().map(|r| r.sections).sum(),
        sections_exempt: scan
            .reachability
            .iter()
            .map(|r| u32::try_from(r.exempt.len()).unwrap_or(u32::MAX))
            .sum(),
        sections_unreached: scan.sections_unreached,
        references_without_sections: u32::try_from(
            scan.reachability.iter().filter(|r| r.sections == 0).count(),
        )
        .unwrap_or(u32::MAX),
        reachability: Some(scan.reachability),
        references_out_of_subject: scan.references_out_of_subject,
        guidance: scan.guidance,
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) = super::bounded_lists::place(
        repo,
        "check-step-references",
        super::bounded_lists::ALL_SPECS,
        &result,
        &lists,
    )? {
        result.form = ListsForm::File;
        result.findings = None;
        result.examined = None;
        result.with_steps = None;
        result.not_a_procedure = None;
        result.skipped = None;
        result.reachability = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// Scan every subject file, every list whole.
// One flat pass over the subject files: enumerate, classify, resolve. Split
// into helpers the branches would only be threaded back through — the same
// reason `main`'s CLI match and `dispatch_primitive` carry this allow.
#[allow(clippy::too_many_lines)]
fn compute(_args: &CheckStepReferencesArgs, repo: &Path) -> StepReferences {
    let mut examined = Vec::new();
    let mut with_steps = Vec::new();
    let mut skipped = Vec::new();
    let mut findings = Vec::new();
    let mut not_a_procedure = Vec::new();
    let mut references_out_of_subject: u32 = 0;

    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    // A source directory or file that is, or lies under, a link leaving the
    // project is not listed or read: a skip names it, never a clean result
    // (`BE-INPUT-004`).
    let listing = if super::leaves_project(repo, COMMANDS_DIR) {
        Err(std::io::Error::other("it leaves the project"))
    } else {
        std::fs::read_dir(repo.join(COMMANDS_DIR))
    };
    let commands_listed = listing.is_ok();
    match listing {
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
        if super::leaves_project(repo, &rel) {
            skipped.push(StepReferenceSkip {
                path: rel,
                reason: "could not be read: it leaves the project".to_string(),
            });
            continue;
        }
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

        // References outside the subject are counted, never resolved — a
        // numbered list elsewhere in the file restarts at 1 and is a
        // different list.
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

    // Where there are no command sources there are no sections for them to
    // reach, and the skip above already says nothing was examined.
    let mut reachability = Vec::new();
    let mut sections_unreached = 0;
    if commands_listed {
        let reached = reach(repo);
        sections_unreached = reached.unreached;
        findings.extend(reached.findings);
        skipped.extend(reached.skipped);
        reachability = reached.reachability;
    }

    let guidance = if examined.is_empty() {
        format!("no command sources found under {COMMANDS_DIR} — nothing was examined")
    } else {
        String::new()
    };

    StepReferences {
        findings,
        examined,
        with_steps,
        not_a_procedure,
        skipped,
        reachability,
        sections_unreached,
        references_out_of_subject,
        guidance,
    }
}

/// The reference marker: a line holding only
/// `<!-- audit:kept <reason> -->`, the reason optional so an empty one can be
/// found.
#[allow(clippy::expect_used)] // A hard-coded pattern that compiles.
fn kept_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"^\s*<!--\s*audit:kept(\s.*?)?\s*-->\s*$").expect("hard-coded regex compiles")
    })
}

/// [`reach`]'s findings, skips and per-reference record.
#[derive(Default)]
struct Reach {
    findings: Vec<StepReferenceFinding>,
    skipped: Vec<StepReferenceSkip>,
    reachability: Vec<ReferenceReach>,
    unreached: u32,
}

/// Judge every `framework/reference/<command>.md` against its command's
/// citations ([`cited_sections`]). A reference whose command or text cannot be
/// read, or that cites a reference that cannot be, is a skip: nothing is
/// judged of it.
fn reach(repo: &Path) -> Reach {
    let mut out = Reach::default();
    if super::leaves_project(repo, REFERENCE_DIR) {
        out.skipped.push(StepReferenceSkip {
            path: REFERENCE_DIR.to_string(),
            reason: "reference directory could not be listed: it leaves the project".to_string(),
        });
        return out;
    }
    let mut references: Vec<std::path::PathBuf> = match std::fs::read_dir(repo.join(REFERENCE_DIR))
    {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
            .collect(),
        Err(err) => {
            out.skipped.push(StepReferenceSkip {
                path: REFERENCE_DIR.to_string(),
                reason: format!("reference directory could not be listed: {err}"),
            });
            return out;
        }
    };
    references.sort();
    let read = |rel: &str| -> std::result::Result<String, String> {
        if super::leaves_project(repo, rel) {
            return Err(format!("{rel} leaves the project"));
        }
        read_text(&repo.join(rel)).map_err(|err| format!("{rel} could not be read: {err}"))
    };
    for path in &references {
        let rel = rel_path(path, repo);
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let judged = read(&rel).and_then(|reference| {
            let command = read(&format!("{COMMANDS_DIR}/{stem}.md"))?;
            let reached = cited_sections(&command, stem, |name| {
                read(&format!("{REFERENCE_DIR}/{name}.md"))
            })?;
            Ok((reference, reached))
        });
        match judged {
            Ok((reference, reached)) => {
                let own: std::collections::HashSet<&str> = reached
                    .iter()
                    .filter(|(name, _)| name == stem)
                    .map(|(_, heading)| heading.as_str())
                    .collect();
                judge_sections(&rel, &reference, &own, &mut out);
            }
            Err(reason) => out.skipped.push(StepReferenceSkip {
                path: rel,
                reason: format!("could not be judged: {reason}"),
            }),
        }
    }
    out
}

/// One reference's `##` sections against the headings its command reaches.
fn judge_sections(
    rel: &str,
    reference: &str,
    reached: &std::collections::HashSet<&str>,
    out: &mut Reach,
) {
    let lines: Vec<&str> = reference.lines().collect();
    let finding = |line: usize, kind: &str, message: String| StepReferenceFinding {
        file: rel.to_string(),
        line,
        kind: kind.into(),
        reference: 0,
        message,
    };
    let sections: Vec<_> = heading_lines(reference)
        .into_iter()
        .filter(|h| h.level == 2)
        .collect();
    let mut attached = std::collections::HashSet::new();
    let mut exempt = Vec::new();
    for section in &sections {
        let marker = (section.line + 1..lines.len())
            .find(|&i| !lines[i].trim().is_empty())
            .and_then(|i| kept_regex().captures(lines[i]).map(|cap| (i, cap)));
        let at = marker.as_ref().map_or(section.line + 2, |(i, _)| i + 1);
        let reason = marker.as_ref().map(|(i, cap)| {
            attached.insert(*i);
            cap.get(1).map_or("", |m| m.as_str()).trim().to_string()
        });
        let is_reached = reached.contains(section.text.as_str());
        let mut exempted = false;
        match reason {
            Some(reason) if reason.is_empty() => out.findings.push(finding(
                at,
                "empty-exemption",
                format!(
                    "{rel}: the `audit:kept` marker under `{}` states no reason, so it exempts \
                     nothing — state why a reader outside the command keeps the section",
                    section.text
                ),
            )),
            Some(_) if is_reached => out.findings.push(finding(
                at,
                "stale-exemption",
                format!(
                    "{rel}: `{}` is reached by its command, so its `audit:kept` marker exempts \
                     nothing — remove the marker",
                    section.text
                ),
            )),
            Some(reason) => {
                exempted = true;
                exempt.push(ExemptSection {
                    section: section.text.clone(),
                    reason,
                });
            }
            None => {}
        }
        if !is_reached && !exempted {
            out.unreached += 1;
            out.findings.push(finding(
                section.line + 1,
                "unreached-section",
                format!(
                    "{rel}: `## {}` is reached by no step of its command, so no agent reads it — \
                     cite it from the step that needs it, delete it if it restates, or mark it \
                     `<!-- audit:kept <reason> -->` beneath the heading",
                    section.text
                ),
            ));
        }
    }
    let mut skip = SkipScanner::default();
    for (i, line) in lines.iter().enumerate() {
        if !skip.skip(line) && kept_regex().is_match(line) && !attached.contains(&i) {
            out.findings.push(finding(
                i + 1,
                "misplaced-exemption",
                format!(
                    "{rel}: an `audit:kept` marker that is not the first line beneath a `##` \
                     heading exempts nothing — move it, or remove it"
                ),
            ));
        }
    }
    out.reachability.push(ReferenceReach {
        reference: rel.to_string(),
        sections: u32::try_from(sections.len()).unwrap_or(u32::MAX),
        exempt,
    });
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
/// A **qualified** reference — `groom's step 3`, or `step 5 of **§Walk
/// through tasks** in …` — is excluded: it names another procedure's list,
/// and resolving it against the citing file's own step set is how a check
/// like this manufactures findings. A line that opens
/// a numbered step contributes its own leading number as a step, never as a
/// reference.
fn step_references(line: &str) -> Vec<u32> {
    let lower = line.to_lowercase();
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
        let mut numbers = Vec::new();
        loop {
            let rest = &lower[walk..];
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            if digits.is_empty() {
                break;
            }
            if let Ok(n) = digits.parse::<u32>() {
                numbers.push(n);
            }
            walk += digits.len();
            let rest = &lower[walk..];
            let sep = ["–", "—", "-", " and ", ", and ", ", "];
            let Some(found) = sep.iter().find(|s| rest.starts_with(**s)) else {
                break;
            };
            walk += found.len();
        }
        // `step 5 of **§Walk through tasks** in …` numbers a named section's
        // own list, so it is qualified like the possessive form.
        let after = &lower[walk..];
        if !(after.starts_with(" of §") || after.starts_with(" of **§")) {
            out.extend(numbers);
        }
        i = walk.max(start + 4);
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

## A later list

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
            "a later list must not join the step set"
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

    /// A pointer into a reference section numbers that section's own list
    /// (spec 071): `implement.md` writes "step 5 of **§Walk through tasks (per
    /// task, in order)** in `.ductus/reference/implement.md`", which is not
    /// the command's step 5.
    #[test]
    fn a_step_of_a_named_section_is_qualified() {
        assert_eq!(
            step_references("step 5 of **§Walk through tasks** in `.ductus/reference/x.md`"),
            Vec::<u32>::new()
        );
        assert_eq!(step_references("per step 2 of §Load"), Vec::<u32>::new());
        // The unqualified form still counts.
        assert_eq!(step_references("see step 5 of the walk"), vec![5]);
    }

    /// A project with the given command and reference files, each as
    /// `(name, text)`.
    fn reach_project(commands: &[(&str, &str)], references: &[(&str, &str)]) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        for (dir, files) in [(COMMANDS_DIR, commands), (REFERENCE_DIR, references)] {
            std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
            for (name, text) in files {
                std::fs::write(tmp.path().join(dir).join(format!("{name}.md")), text).unwrap();
            }
        }
        tmp
    }

    /// A command whose one step names its reference `x` and cites `pointer`.
    fn command(pointer: &str) -> String {
        format!(
            "# X\n\n## Instructions\n\n1. One, per {pointer} in `.ductus/reference/x.md`.\n\
             2. Two.\n3. Three.\n"
        )
    }

    fn kinds(found: &StepReferences) -> Vec<&str> {
        found.findings.iter().map(|f| f.kind.as_str()).collect()
    }

    /// A section no step reaches is a finding that names the reference, the
    /// section and its line, and counts as unreached.
    #[test]
    fn an_unreached_section_is_reported_and_named() {
        let tmp = reach_project(
            &[("x", &command("**§Reached**"))],
            &[("x", "# X\n\n## Reached\n\nText.\n\n## Orphan\n\nText.\n")],
        );
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert_eq!(kinds(&found), vec!["unreached-section"], "{found:?}");
        let finding = &found.findings[0];
        assert_eq!(finding.file, "framework/reference/x.md");
        assert_eq!(finding.line, 7);
        assert!(finding.message.contains("## Orphan"), "{}", finding.message);
        assert_eq!(found.sections_unreached, 1);
        assert_eq!(found.reachability[0].sections, 2);
    }

    /// A section a reached section names by `§`, and one only a markdown
    /// anchor link in a reached section names, are reached.
    #[test]
    fn a_section_reached_through_a_reached_one_or_by_an_anchor_link_is_reached() {
        let tmp = reach_project(
            &[("x", &command("**§Top**"))],
            &[(
                "x",
                "## Top\n\nSee §Through and [the output](#the-output-row).\n\n\
                 ## Through\n\nText.\n\n## The `output` row\n\nText.\n",
            )],
        );
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert!(found.findings.is_empty(), "{found:?}");
        assert_eq!(found.reachability[0].sections, 3);
    }

    /// A heading inside a fenced block is not a section, so it is neither
    /// counted nor reported.
    #[test]
    fn a_heading_in_a_fenced_block_is_not_a_section() {
        let tmp = reach_project(
            &[("x", &command("**§Top**"))],
            &[("x", "## Top\n\n```text\n## Example\n```\n")],
        );
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert!(found.findings.is_empty(), "{found:?}");
        assert_eq!(found.reachability[0].sections, 1);
    }

    /// A subsection cited by name reaches itself and not its `##` parent.
    #[test]
    fn a_cited_subsection_does_not_reach_its_parent_section() {
        let tmp = reach_project(
            &[("x", &command("**§Child**"))],
            &[(
                "x",
                "## Parent\n\n### Child\n\nText.\n\n### Spare\n\nText.\n",
            )],
        );
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert_eq!(kinds(&found), vec!["unreached-section"], "{found:?}");
        assert!(found.findings[0].message.contains("## Parent"));
        assert_eq!(
            found.reachability[0].sections, 1,
            "only a `##` heading is a section"
        );
    }

    /// A section with a marker stating its reason is exempt: listed with the
    /// reason and counted, and it is no finding.
    #[test]
    fn an_exempt_section_with_a_reason_is_exempt_and_counted() {
        let tmp = reach_project(
            &[("x", &command("**§Top**"))],
            &[(
                "x",
                "## Top\n\nText.\n\n## Kept\n\n<!-- audit:kept the constitution names it -->\n\nText.\n",
            )],
        );
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert!(found.findings.is_empty(), "{found:?}");
        assert_eq!(
            found.reachability[0].exempt,
            vec![ExemptSection {
                section: "Kept".into(),
                reason: "the constitution names it".into(),
            }]
        );
        let result = bound(found, tmp.path()).unwrap();
        assert_eq!(result.sections_exempt, 1);
        assert_eq!(result.sections_examined, 2);
        assert_eq!(result.sections_unreached, 0);
    }

    /// An empty reason exempts nothing, a marker on a reached section is
    /// stale, and a marker anywhere but beneath a `##` heading is misplaced:
    /// each a finding, so an exemption cannot rot.
    #[test]
    fn an_empty_stale_or_misplaced_exemption_is_a_finding() {
        let tmp = reach_project(
            &[("x", &command("**§Top**"))],
            &[(
                "x",
                "## Top\n\n<!-- audit:kept it is cited anyway -->\n\nText.\n\n## Empty\n\n\
                 <!-- audit:kept -->\n\nText.\n\n## Loose\n\nText.\n<!-- audit:kept stray -->\n",
            )],
        );
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        let mut got = kinds(&found);
        got.sort_unstable();
        assert_eq!(
            got,
            vec![
                "empty-exemption",
                "misplaced-exemption",
                "stale-exemption",
                "unreached-section",
                "unreached-section",
            ],
            "{found:?}"
        );
        assert_eq!(found.reachability[0].exempt, vec![]);
        assert_eq!(found.sections_unreached, 2);
        // A finding names its marker's own line, a blank line or not between
        // the heading and the marker.
        let line_of = |kind: &str| found.findings.iter().find(|f| f.kind == kind).unwrap().line;
        assert_eq!(line_of("stale-exemption"), 3);
        assert_eq!(line_of("empty-exemption"), 9);
    }

    /// Only a numbered step starts the walk: a citation in the Instructions
    /// preamble, in a note after the steps or in another section of the
    /// command (Scope Boundaries) reaches nothing, and one in a bullet inside
    /// a step does.
    #[test]
    fn only_a_numbered_step_reaches_a_section() {
        let command = "# X\n\n## Scope Boundaries\n\n- See **§InScope** in `.ductus/reference/x.md`.\n\n\
                       ## Instructions\n\n> Preamble cites **§InPreamble**.\n\n\
                       1. One, per **§Top** in `.ductus/reference/x.md`.\n   - a bullet citing **§InBullet**\n\
                       2. Two.\n3. Three.\n\nA note after the steps cites **§InNote**.\n";
        let reference = "## Top\n\n## InBullet\n\n## InPreamble\n\n## InNote\n\n## InScope\n";
        let tmp = reach_project(&[("x", command)], &[("x", reference)]);
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        let mut unreached: Vec<&str> = found
            .findings
            .iter()
            .map(|f| f.message.split('`').nth(1).unwrap())
            .collect();
        unreached.sort_unstable();
        assert_eq!(
            unreached,
            vec!["## InNote", "## InPreamble", "## InScope"],
            "{found:?}"
        );
    }

    /// A reference with no `##` section is counted as having none: neither a
    /// finding, a skip nor a silent pass.
    #[test]
    fn a_reference_with_no_sections_is_reported_as_having_none() {
        let tmp = reach_project(
            &[("x", &command("**§Top**"))],
            &[("x", "# X\n\nOpening.\n")],
        );
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert!(
            found.findings.is_empty() && found.skipped.is_empty(),
            "{found:?}"
        );
        assert_eq!(found.reachability[0].sections, 0);
        let result = bound(found, tmp.path()).unwrap();
        assert_eq!(result.references_without_sections, 1);
        assert_eq!(result.sections_examined, 0);
    }

    /// A reference with no command, one that is not readable, and one whose
    /// command names a reference that is missing are skips with their
    /// reasons, never a pass.
    #[test]
    fn an_unreadable_reference_is_a_skip() {
        let tmp = reach_project(
            &[("x", &command("**§Top**")), ("y", &command("**§Top**"))],
            &[("x", "## Top\n"), ("nocommand", "## Top\n")],
        );
        // `y`'s reference is a directory, which cannot be read as text.
        std::fs::create_dir(tmp.path().join(REFERENCE_DIR).join("y.md")).unwrap();
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        let skipped: Vec<&str> = found.skipped.iter().map(|s| s.path.as_str()).collect();
        assert_eq!(
            skipped,
            vec![
                "framework/reference/nocommand.md",
                "framework/reference/y.md"
            ],
            "{found:?}"
        );
        assert!(
            found
                .skipped
                .iter()
                .all(|s| s.reason.contains("could not be judged"))
        );
        assert_eq!(found.reachability.len(), 1, "only x was judged");
    }

    /// A command that names another reference it cannot read is a skip too,
    /// since what it reaches there is unknown.
    #[test]
    fn a_cited_reference_that_cannot_be_read_is_a_skip() {
        let command = "# X\n\n## Instructions\n\n1. See **§Top** in `.ductus/reference/gone.md`.\n";
        let tmp = reach_project(&[("x", command)], &[("x", "## Top\n")]);
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert_eq!(found.skipped.len(), 1, "{found:?}");
        assert!(found.skipped[0].reason.contains("gone.md"), "{found:?}");
        assert_eq!(found.reachability, vec![]);
    }

    /// In a project with no `framework/` — an adopter's — nothing is examined,
    /// the existing skip says so, and no reachability is claimed.
    #[test]
    fn a_project_without_framework_sources_judges_no_sections() {
        let tmp = tempfile::tempdir().unwrap();
        let found = compute(&CheckStepReferencesArgs {}, tmp.path());
        assert!(
            found.examined.is_empty() && found.reachability.is_empty(),
            "{found:?}"
        );
        assert_eq!(found.skipped.len(), 1, "{found:?}");
        assert!(found.guidance.contains("nothing was examined"));
    }

    /// The shipped tree: every `##` section of every reference is reached,
    /// except the four a stated reason keeps. Built from the real files, so it
    /// fails when a pointer a command needs is removed (spec 071).
    #[test]
    fn the_shipped_references_are_reached_except_the_four_kept_sections() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let found = compute(&CheckStepReferencesArgs {}, root);
        assert!(found.skipped.is_empty(), "{:?}", found.skipped);
        let unreached: Vec<&str> = found.findings.iter().map(|f| f.message.as_str()).collect();
        assert!(unreached.is_empty(), "{unreached:#?}");
        let exempt: Vec<(&str, &str)> = found
            .reachability
            .iter()
            .flat_map(|r| {
                r.exempt
                    .iter()
                    .map(move |e| (r.reference.as_str(), e.section.as_str()))
            })
            .collect();
        assert_eq!(
            exempt,
            vec![
                ("framework/reference/analyze.md", "Severity tiers"),
                ("framework/reference/analyze.md", "Record fields"),
                (
                    "framework/reference/plan.md",
                    "Cross-spec context checklist"
                ),
                ("framework/reference/review.md", "Blocking semantics"),
            ]
        );
        assert!(
            found.reachability.iter().map(|r| r.sections).sum::<u32>() > 50,
            "the walk judged too few sections to be reading the references"
        );
    }

    /// A project with a clean command file and one whose Instructions
    /// refers to step 9, which it lacks, and a reference directory that holds
    /// no file.
    fn project() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(REFERENCE_DIR)).unwrap();
        let dir = tmp.path().join(COMMANDS_DIR);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("cmd.md"),
            "# Cmd\n\n## Instructions\n\n1. One, see step 2.\n2. Two.\n3. Three.\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("bad.md"),
            "# Bad\n\n## Instructions\n\n1. One, see step 9.\n2. Two.\n3. Three.\n",
        )
        .unwrap();
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

    /// Under the read size every list is in the result with its count, and
    /// no list file is written.
    #[test]
    fn the_lists_are_inline_while_the_result_fits() {
        let tmp = project();
        let result = run(&CheckStepReferencesArgs {}, tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        assert_eq!(result.lists_file, None);
        let findings = result.findings.unwrap();
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].kind, "unresolved");
        assert_eq!(result.findings_count, 1);
        assert_eq!(result.examined.as_ref().unwrap().len(), 2);
        assert_eq!(result.examined_count, 2);
        assert_eq!(result.with_steps.as_ref().unwrap().len(), 2);
        assert_eq!(result.with_steps_count, 2);
        assert_eq!(result.not_a_procedure, Some(vec![]));
        assert_eq!(result.not_a_procedure_count, 0);
        assert_eq!(result.skipped, Some(vec![]));
        assert_eq!(result.skipped_count, 0);
        assert_eq!(result.reachability, Some(vec![]));
        assert_eq!(result.reachability_count, 0);
        assert!(!tmp.path().join(".ductus/results").exists());
    }

    /// A result exactly at the read size fits; one byte under it moves the
    /// six lists whole to the file the result names, their counts staying.
    #[test]
    fn the_lists_are_in_a_file_past_the_read_size() {
        let tmp = project();
        let whole = run(&CheckStepReferencesArgs {}, tmp.path()).unwrap();
        let size = serde_json::to_vec(&whole).unwrap().len();
        read_size(tmp.path(), size);
        assert_eq!(
            run(&CheckStepReferencesArgs {}, tmp.path()).unwrap().form,
            ListsForm::Inline
        );
        read_size(tmp.path(), size - 1);
        let bounded = run(&CheckStepReferencesArgs {}, tmp.path()).unwrap();
        assert_eq!(bounded.form, ListsForm::File);
        assert_eq!(bounded.findings, None);
        assert_eq!(bounded.examined, None);
        assert_eq!(bounded.with_steps, None);
        assert_eq!(bounded.not_a_procedure, None);
        assert_eq!(bounded.skipped, None);
        assert_eq!(bounded.reachability, None);
        assert_eq!(bounded.findings_count, 1);
        assert_eq!(bounded.examined_count, 2);
        assert_eq!(bounded.with_steps_count, 2);
        assert_eq!(bounded.not_a_procedure_count, 0);
        assert_eq!(bounded.skipped_count, 0);
        assert_eq!(bounded.reachability_count, 0);
        assert_eq!(
            bounded.references_out_of_subject,
            whole.references_out_of_subject
        );
        let file = bounded.lists_file.clone().unwrap();
        assert!(
            file.starts_with(".ductus/results/check-step-references/all-specs/"),
            "{file}"
        );
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(&file)).unwrap())
                .unwrap();
        assert_eq!(
            lists["findings"],
            serde_json::to_value(&whole.findings).unwrap()
        );
        assert_eq!(
            lists["examined"],
            serde_json::to_value(&whole.examined).unwrap()
        );
        assert_eq!(
            lists["with-steps"],
            serde_json::to_value(&whole.with_steps).unwrap()
        );
        assert_eq!(lists["not-a-procedure"], serde_json::json!([]));
        assert_eq!(lists["skipped"], serde_json::json!([]));
        assert_eq!(lists["reachability"], serde_json::json!([]));
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "check-step-references",
            &serde_json::to_value(&bounded).unwrap(),
            &lists,
        );
    }

    /// A command source directory, a command file and a bootstrap file that
    /// are links leaving the project are skipped by name and never read, so
    /// the scan cannot read as clean over them (`BE-INPUT-004`; 022 task 42).
    #[cfg(unix)]
    #[test]
    fn a_source_linked_outside_the_project_is_skipped_not_read() {
        let steps = "# C\n\n## Instructions\n\n1. a\n2. b\n3. c, see step 9.\n";
        let args = CheckStepReferencesArgs {};

        let repo = tempfile::tempdir().unwrap();
        let _dir = crate::primitives::outward::dir(repo.path(), COMMANDS_DIR, |dir| {
            std::fs::write(dir.join("c.md"), steps).unwrap();
        });
        let found = compute(&args, repo.path());
        assert!(found.examined.is_empty(), "{found:?}");
        assert_eq!(found.skipped[0].path, COMMANDS_DIR);

        let repo = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(repo.path().join(COMMANDS_DIR)).unwrap();
        let _file =
            crate::primitives::outward::file(repo.path(), &format!("{COMMANDS_DIR}/c.md"), steps);
        let found = compute(&args, repo.path());
        assert!(found.examined.is_empty(), "{found:?}");
        assert!(found.findings.is_empty(), "{found:?}");
        assert!(
            found.skipped.iter().any(|s| s.path.ends_with("c.md")),
            "{found:?}"
        );

        let repo = tempfile::tempdir().unwrap();
        let _file = crate::primitives::outward::file(repo.path(), BOOTSTRAP_FILES[0], steps);
        let found = compute(&args, repo.path());
        assert!(found.examined.is_empty(), "{found:?}");
        assert!(
            found.skipped.iter().any(|s| s.path == BOOTSTRAP_FILES[0]),
            "{found:?}"
        );
    }
}
