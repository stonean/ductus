//! Step-order coverage for the two commands spec 052 adds.
//!
//! Both write **two** specs, and both put an irreversible or
//! second-spec-touching action behind a single confirmation. That ordering is
//! the whole safety property and it is invisible in the command prose to
//! anyone who does not already know to look — so it is pinned here, against
//! the shipped `framework/commands/*.md`, exactly as spec 022's
//! `specify_command.rs` pins the routing gate.
//!
//! `/{project}:consolidate` is the sharper case. It is the only command that
//! removes a durable artifact, its confirmation is the operator's only look at
//! what they are losing, and `rewrite-spec-links` is idempotent in the *wrong*
//! direction — once inbound links are re-pointed, nothing names the source and
//! a re-run finds nothing to repair. A gate that drifted below either writer
//! would therefore not merely be a weaker prompt; it would be unrecoverable
//! (spec 052, AC8 and AC10).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;

use ductus::parser::parse;
use ductus::schema::procedure::Step;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn procedure(command: &str) -> ductus::schema::procedure::Procedure {
    let path = workspace_root()
        .join("framework/commands")
        .join(format!("{command}.md"));
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    parse(&source, command).unwrap_or_else(|err| panic!("{command}.md must parse: {err:?}"))
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

fn steps(command: &str) -> Vec<(String, String)> {
    procedure(command).steps.iter().map(label).collect()
}

/// Index of the first step dispatching `kind`, or a panic naming what was
/// searched — a silent `None` here would make the ordering assertions below
/// vacuously true.
fn position(steps: &[(String, String)], kind: &str) -> usize {
    steps
        .iter()
        .position(|(_, k)| k == kind)
        .unwrap_or_else(|| panic!("expected a {kind} step, found: {steps:?}"))
}

#[test]
fn consolidate_confirms_before_it_rewrites_or_removes_anything() {
    let steps = steps("consolidate");
    let expected: Vec<(String, String)> = [
        ("1", "primitive:read-spec"),
        // Step 2 is prose on purpose: enumerating what the removal destroys
        // is a directory walk, not a decision.
        ("2", "prose"),
        ("3", "primitive:gate-confirm"),
        ("4", "primitive:rewrite-spec-links"),
        ("5", "primitive:retire-feature"),
        // Steps 6 and 7 are prose: clearing the session is conditional on the
        // session having named the source, and the report is a host
        // responsibility. Both come *after* the removal — the session cannot
        // be stranded until the directory is gone.
        ("6", "prose"),
        ("7", "prose"),
    ]
    .into_iter()
    .map(|(n, k)| (n.to_string(), k.to_string()))
    .collect();
    assert_eq!(steps, expected);

    // The ordering claim as a property rather than a list: every step that
    // writes comes after the confirmation.
    let gate = position(&steps, "primitive:gate-confirm");
    for writer in ["primitive:rewrite-spec-links", "primitive:retire-feature"] {
        assert!(
            position(&steps, writer) > gate,
            "{writer} must not run before the confirmation"
        );
    }
}

#[test]
fn fold_never_reaches_the_sequential_opt_in() {
    // The refusal spec 052 gated is only safe because the one caller that
    // does not opt in cannot. `fold.md` describes `retire-feature`, so a
    // future edit could plausibly add the argument there; this is what would
    // catch it. Prose-level, deliberately: the opt-in is an argument the
    // command file names or does not, and there is no parsed representation
    // of a primitive's arguments to assert against.
    let path = workspace_root().join("framework/commands/fold.md");
    let source = std::fs::read_to_string(&path).unwrap();
    assert!(
        !source.contains("allow-sequential") && !source.contains("allow_sequential"),
        "fold.md must not pass retire-feature's sequential opt-in — the refusal it \
         bypasses is what keeps an irreversible removal out of reach of a typo"
    );
    // And it must still say so, so a reader of fold's own account of
    // `retire-feature` is not left with the pre-052 description.
    assert!(
        source.contains("opt-in"),
        "fold.md must record that it never passes the opt-in, so its account of \
         retire-feature matches the primitive's behavior"
    );
}
