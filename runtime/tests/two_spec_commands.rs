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
        // Steps 2 and 3 are prose on purpose: enumerating what the removal
        // destroys is a directory walk, and settling a `supersedes:` edge is
        // a decision only the operator may make — re-pointing one silently
        // would give the declaring spec a claim nobody made.
        ("2", "prose"),
        ("3", "prose"),
        ("4", "primitive:gate-confirm"),
        ("5", "primitive:rewrite-spec-links"),
        ("6", "primitive:retire-feature"),
        // Steps 7 and 8 are prose: clearing the session is conditional on the
        // session having named the source, and the report is a host
        // responsibility. Both come *after* the removal — the session cannot
        // be stranded until the directory is gone.
        ("7", "prose"),
        ("8", "prose"),
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
fn supersede_confirms_before_it_annotates_the_other_spec() {
    let steps = steps("supersede");
    let expected: Vec<(String, String)> = [
        ("1", "primitive:read-spec"),
        ("2", "primitive:gate-confirm"),
        // Step 3 is prose: appending to `supersedes:` is a frontmatter edit
        // with no primitive of its own.
        ("3", "prose"),
        ("4", "primitive:write-supersession-annotation"),
        // Reconciliation (spec 053): the bounded read, the judgment at the
        // boundary, then the criterion-level annotations it authorizes.
        ("5", "primitive:read-supersession-pair"),
        ("6", "extension:classifyClaims"),
        ("7", "primitive:write-supersession-annotation"),
        // Step 8 dispatches a second `gate-confirm`: surfacing conflicts
        // resolves nothing, but the one edit reconciliation may make — body
        // prose on a `done` spec — needs its own confirmation naming the
        // reopen. Step 9's report is a host responsibility.
        ("8", "primitive:gate-confirm"),
        ("9", "prose"),
    ]
    .into_iter()
    .map(|(n, k)| (n.to_string(), k.to_string()))
    .collect();
    assert_eq!(steps, expected);

    // `position` finds the *first* gate — the declaration's.
    let gate = position(&steps, "primitive:gate-confirm");
    assert_eq!(
        gate, 1,
        "the declaration gate must be the first one: {steps:?}"
    );
    assert!(
        position(&steps, "primitive:write-supersession-annotation") > gate,
        "the annotation writes to a second spec and must not precede the confirmation"
    );
    // Reconciliation reads and classifies before it annotates, and every
    // part of it sits after the confirmation — it writes to the second spec
    // too, one criterion at a time.
    let read = position(&steps, "primitive:read-supersession-pair");
    let classify = position(&steps, "extension:classifyClaims");
    assert!(
        read > gate,
        "the reconciliation read must follow the confirmation"
    );
    assert!(
        classify > read,
        "classification needs the pair; it cannot precede the read"
    );
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

#[test]
fn supersede_reports_all_three_reconciliation_outcomes() {
    // The host half of AC3/AC11/AC12. `read-supersession-pair` makes the
    // three states pairwise distinguishable in the *data* — that is pinned in
    // the primitive's own tests — but the obligation to actually tell them
    // apart in the report lives in prose, and prose is what drifts. This
    // asserts the command still carries all three.
    let source =
        std::fs::read_to_string(workspace_root().join("framework/commands/supersede.md")).unwrap();
    for state in [
        "conflicts to settle",
        "nothing to reconcile",
        "could not fully examine",
    ] {
        assert!(
            source.contains(state),
            "supersede.md's report must name the {state:?} outcome — three states that read \
             alike are the QUAL-CLAIM-001 failure this reconciliation is built to avoid"
        );
    }
}

#[test]
fn nothing_between_classification_and_the_report_can_resolve_a_conflict() {
    // The structural half of AC2. A conflict is surfaced and never resolved,
    // and the cheapest way to keep that true is to give the walk no primitive
    // that could do it: between `classifyClaims` and the report, the only
    // dispatches are the criterion annotation (which takes a label, not a
    // conflict) and the gate that guards a body-prose edit.
    let steps = steps("supersede");
    let classify = position(&steps, "extension:classifyClaims");
    let report = steps.len() - 1;

    let between: Vec<&str> = steps[classify + 1..report]
        .iter()
        .map(|(_, kind)| kind.as_str())
        .collect();
    for kind in &between {
        assert!(
            matches!(
                *kind,
                "prose" | "primitive:write-supersession-annotation" | "primitive:gate-confirm"
            ),
            "unexpected dispatch between classification and the report: {kind} — a primitive \
             here is a primitive that could settle a conflict the operator has not"
        );
    }
    assert!(
        between.contains(&"primitive:write-supersession-annotation"),
        "the superseded claims must still be annotated: {between:?}"
    );
}
