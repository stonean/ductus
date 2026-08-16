//! Integration coverage for the `/ductus:specify` routing gate.
//!
//! Spec 022 scenario `specify-routes-before-scaffolding`: the routing decision
//! runs *before* `create-feature` writes anything, because creating a spec is
//! the one action the two routing rules exist to prevent. A gate that sits
//! after the scaffold is no gate at all, and the ordering is invisible in the
//! command prose to anyone who does not already know to look — so it is pinned
//! here, against the shipped `framework/commands/specify.md`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::io::Cursor;
use std::path::PathBuf;

use ductus::interpreter::{WalkOutcome, Walker};
use ductus::parser::parse;
use ductus::schema::procedure::Step;
use serde_json::{Map, Value};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn specify_procedure() -> ductus::schema::procedure::Procedure {
    let source =
        std::fs::read_to_string(workspace_root().join("framework/commands/specify.md")).unwrap();
    parse(&source, "specify").expect("specify.md must parse as a Procedure")
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
fn routing_gate_precedes_create_feature() {
    let steps: Vec<(String, String)> = specify_procedure().steps.iter().map(label).collect();
    let expected: Vec<(String, String)> = [
        ("1", "primitive:derive-routing-candidates"),
        ("2", "extension:routeInboxItem"),
        ("3", "primitive:gate-confirm"),
        ("4", "primitive:create-feature"),
        ("5", "extension:writeSpecBody"),
        ("6", "primitive:label-criteria"),
        ("7", "primitive:lint-markdown"),
        ("8", "primitive:gate-confirm"),
        ("9", "primitive:write-session"),
    ]
    .into_iter()
    .map(|(n, k)| (n.to_string(), k.to_string()))
    .collect();

    assert_eq!(steps, expected);

    // The ordering claim, stated as the property rather than as a list: every
    // write-performing step comes after the routing gate.
    let gate = steps
        .iter()
        .position(|(_, kind)| kind == "primitive:gate-confirm")
        .expect("a gate-confirm step");
    for writer in [
        "primitive:create-feature",
        "primitive:label-criteria",
        "primitive:write-session",
    ] {
        let at = steps
            .iter()
            .position(|(_, kind)| kind == writer)
            .unwrap_or_else(|| panic!("{writer} step present"));
        assert!(at > gate, "{writer} must not run before the routing gate");
    }
}

#[test]
fn denying_the_routing_gate_scaffolds_nothing() {
    // The operator taking a candidate instead of a new spec is expressed as a
    // denial: the walk ends clean and no feature directory exists. If the gate
    // ever moved after `create-feature`, this would leave a stray spec behind
    // — the exact failure the scenario was written from.
    let tmp = tempfile::tempdir().unwrap();
    let templates = tmp.path().join("specs/templates");
    std::fs::create_dir_all(&templates).unwrap();
    std::fs::write(
        templates.join("spec.md"),
        "---\nstatus: draft\ndependencies: []\n---\n\n# {NNN}\n",
    )
    .unwrap();

    let mut context = Map::new();
    context.insert(
        "title".into(),
        Value::String("check-artifacts family for stale reviews".into()),
    );

    let responses = concat!(
        r#"{"type":"llm-response","request-id":"req-1","response":{"route":"scenario","feature":"022-deterministic-runtime"}}"#,
        "\n",
        r#"{"type":"gate-response","request-id":"req-2","confirmed":false}"#,
        "\n",
    );
    let mut reader = Cursor::new(responses.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let procedure = specify_procedure();
    let mut walker = Walker::new(
        &procedure,
        tmp.path().to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let created: Vec<String> = std::fs::read_dir(tmp.path().join("specs"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name != "templates")
        .collect();
    assert!(
        created.is_empty(),
        "a denied routing gate must scaffold nothing, found {created:?}"
    );
    assert!(
        !tmp.path().join(".ductus/session.toml").exists(),
        "a denied routing gate must not write the session target"
    );
}
