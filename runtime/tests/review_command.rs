//! Integration coverage for the `/ductus:review` command procedure.
//!
//! Parses the shipped `framework/commands/review.md` and asserts it walks the
//! review-runtime-acceleration procedure end-to-end: `compute-review-scope` →
//! `discover-rule-files` → five `performReview` passes → `process-waivers` →
//! two host-responsibility steps (process decisions, fix and route; spec 058)
//! → `write-review` → a third, the fix-round loop (spec 078). The three are
//! prose the exec walker no-ops by design, which
//! is what records an exec run's observations as undispositioned rather than
//! deciding them with nobody to confirm. `process-waivers` runs after the
//! passes so waivers are classified against real findings (spec 022 scenario
//! waiver-processing-order) — the pre-pass ordering mass-expired every waiver
//! against an empty `fired` set. This is the parity check for the command
//! rewrite (spec 022 scenario review-runtime-acceleration, tasks 45g/45i) —
//! it fails if a future edit drops a primitive, reorders the passes, or lets
//! the file regress to legacy prose.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;

use ductus::parser::parse;
use ductus::schema::procedure::Step;

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is the runtime crate; the workspace root is its parent.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Reduce a step to `(number, kind:name)` for order-preserving comparison.
fn label(step: &Step) -> (String, String) {
    let num = |n: &ductus::schema::procedure::StepNumber| {
        n.0.iter().map(u32::to_string).collect::<Vec<_>>().join(".")
    };
    match step {
        Step::Primitive { number, name, .. } => (num(number), format!("primitive:{name}")),
        Step::Extension {
            number, identifier, ..
        } => (num(number), format!("extension:{identifier}")),
        Step::Prose { number, .. } => (num(number), "prose".to_string()),
    }
}

#[test]
fn review_command_parses_to_the_expected_procedure() {
    let source =
        std::fs::read_to_string(workspace_root().join("framework/commands/review.md")).unwrap();
    let procedure = parse(&source, "review").expect("review.md must parse as a Procedure");

    let steps: Vec<(String, String)> = procedure.steps.iter().map(label).collect();
    let expected: Vec<(String, String)> = [
        ("1", "primitive:compute-review-scope"),
        ("2", "primitive:discover-rule-files"),
        ("3", "extension:performReview"),
        ("4", "extension:performReview"),
        ("5", "extension:performReview"),
        ("6", "extension:performReview"),
        ("7", "extension:performReview"),
        ("8", "primitive:process-waivers"),
        ("9", "prose"),
        ("10", "prose"),
        ("11", "primitive:write-review"),
        ("12", "prose"),
    ]
    .into_iter()
    .map(|(n, k)| (n.to_string(), k.to_string()))
    .collect();

    assert_eq!(steps, expected);
}

/// The text of `command`'s `step`, from its shipped source, once the parser has
/// held it to be prose, which `ductus exec` no-ops.
fn step_prose(command: &str, step: &str) -> String {
    let source =
        std::fs::read_to_string(workspace_root().join(format!("framework/commands/{command}.md")))
            .unwrap();
    let procedure = parse(&source, command).unwrap();
    assert!(
        procedure.steps.iter().any(|candidate| {
            let (number, kind) = label(candidate);
            number == step && kind == "prose"
        }),
        "{command} step {step} is not prose"
    );
    source
        .lines()
        .find(|line| line.starts_with(&format!("{step}. ")))
        .unwrap_or_else(|| panic!("{command} has no step {step}"))
        .to_string()
}

/// Spec 079, AC17: in each command's fix-and-route step, the one sentence that
/// reaches `gate-confirm` is about a gating finding's deferral. The step is
/// prose the host walks, so its text is what there is to hold. It stays prose,
/// which `ductus exec` no-ops, so an exec run writes no disposition.
#[test]
fn a_fix_and_route_step_asks_a_person_only_about_a_gating_deferral() {
    for (command, step, gating) in [
        ("review", "10", "contract"),
        ("analyze", "19", "`blocking`"),
    ] {
        let prose = step_prose(command, step);
        assert!(prose.contains("`by-rule`"), "{command}: {prose}");
        let confirming: Vec<&str> = prose
            .split(". ")
            .filter(|sentence| sentence.contains("gate-confirm"))
            .collect();
        assert!(
            !confirming.is_empty(),
            "{command}: nothing reaches a person"
        );
        for sentence in &confirming {
            assert!(
                sentence.contains("deferr") && sentence.contains(gating),
                "{command}: {sentence}"
            );
        }
    }
}
