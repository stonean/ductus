//! A command step never states a bounded list's emptiness by the list (spec
//! 080, the scenario `a-step-gates-on-a-bounded-list-by-its-count`).
//!
//! Past `[artifacts] read-size-bytes` a bounded result has `form: "file"`:
//! each moved list is absent and only its `{list}-count` stays inline. A step
//! that reads "an empty `unfolded`" there reads an absent list as an empty one
//! and stops with "nothing to fold" while thousands are pending, so a step
//! that asks whether a list is empty reads the `-count`.
//!
//! Every served primitive's description and schema doc comments are held to
//! the same rule against that primitive's own bounded lists, since a caller
//! reads them as the contract for the same results.
//!
//! The bounded list names are derived from the served tools' output schemas
//! (the property `X` beside `X-count`, in a tool that also has `lists-file`),
//! never listed here, so a primitive that gains a bounded list gains the check.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use ductus::mcp::server::GovRuntimeServer;
use regex::Regex;
use rmcp::ServiceExt;
use serde_json::Value;

/// Phrases a command may keep although they name a bounded list's emptiness,
/// each with the reason it is not the defect: (file, matched phrase, reason).
const ALLOWED: &[(&str, &str, &str)] = &[
    (
        "amend.md",
        "no `dependencies`",
        "names a scenario target's missing frontmatter field, not a result's list",
    ),
    (
        "analyze.md",
        "non-empty `cycles`",
        "an entry of the list the step has just read from the file `lists-file` names",
    ),
    (
        "target.md",
        "empty `skipped`",
        "`resolve-constitutions`' `skipped`, which has no `-count` and is not bounded",
    ),
];

/// Phrases a description may keep although they name a bounded list's
/// emptiness (a bare name too), each with the reason it is not the defect:
/// (tool, matched phrase, reason).
const ALLOWED_DESCRIPTIONS: &[(&str, &str, &str)] = &[
    (
        "compute-review-scope",
        "empty scope",
        "defined by a zero `scope-count`, \"never an absent list\": the count is the reading",
    ),
    (
        "discover-rule-files",
        "non-empty unreadable",
        "names what the file form moves out of the result, not how to judge a result",
    ),
    (
        "lint-markdown",
        "no violations",
        "what a clean lint is, not a reading of the `violations` list",
    ),
    (
        "prune-project",
        "no entries",
        "an unreadable file contributes no entries: a fact about the file, not the list",
    ),
];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// The served tools' output schemas, from a server rooted at the repository.
async fn served_output_schemas() -> Vec<(String, serde_json::Map<String, Value>)> {
    let (server_side, client_side) = tokio::io::duplex(64 * 1024);
    let server = GovRuntimeServer::new(workspace_root());
    tokio::spawn(async move {
        if let Ok(service) = server.serve(server_side).await {
            let _ = service.waiting().await;
        }
    });
    let client = ().serve(client_side).await.unwrap();
    client
        .list_tools(Option::default())
        .await
        .unwrap()
        .tools
        .into_iter()
        .filter_map(|tool| {
            let schema = tool.output_schema.as_ref()?;
            Some((tool.name.to_string(), (**schema).clone()))
        })
        .collect()
}

/// Every tool's description and every `description` string in its input and
/// output schemas, as (tool, text).
async fn served_descriptions() -> Vec<(String, String)> {
    fn collect(tool: &str, value: &Value, out: &mut Vec<(String, String)>) {
        match value {
            Value::Object(map) => {
                for (key, child) in map {
                    match (key.as_str(), child.as_str()) {
                        ("description", Some(text)) => out.push((tool.to_string(), text.into())),
                        _ => collect(tool, child, out),
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|item| collect(tool, item, out)),
            _ => {}
        }
    }
    let (server_side, client_side) = tokio::io::duplex(64 * 1024);
    let server = GovRuntimeServer::new(workspace_root());
    tokio::spawn(async move {
        if let Ok(service) = server.serve(server_side).await {
            let _ = service.waiting().await;
        }
    });
    let client = ().serve(client_side).await.unwrap();
    let mut out = Vec::new();
    for tool in client.list_tools(Option::default()).await.unwrap().tools {
        let name = tool.name.to_string();
        if let Some(description) = &tool.description {
            out.push((name.clone(), description.to_string()));
        }
        collect(
            &name,
            &Value::Object((*tool.input_schema).clone()),
            &mut out,
        );
        if let Some(schema) = &tool.output_schema {
            collect(&name, &Value::Object((**schema).clone()), &mut out);
        }
    }
    out
}

fn is_array(property: &Value) -> bool {
    property.to_string().contains("\"array\"")
}

/// The names that are a list with an inline `-count` in a tool whose result
/// can move to a file.
fn bounded_list_names(schemas: &[(String, serde_json::Map<String, Value>)]) -> BTreeSet<String> {
    schemas
        .iter()
        .flat_map(|(_, schema)| bounded_lists_of(schema))
        .collect()
}

/// One tool's own bounded lists: the same rule over its one output schema.
fn bounded_lists_of(schema: &serde_json::Map<String, Value>) -> BTreeSet<String> {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return BTreeSet::new();
    };
    properties
        .iter()
        .filter(|(name, property)| {
            is_array(property)
                && properties.contains_key("lists-file")
                && properties.contains_key(&format!("{name}-count"))
        })
        .map(|(name, _)| name.clone())
        .collect()
}

/// Every phrase in `text` that states a bounded list's emptiness by the list:
/// `empty`, `non-empty`, `no` or `zero` before a backticked name (before the
/// name bare too, when `bare`), or a backticked name followed by `is`/`are`
/// `empty`/`absent`. A hyphenated `non-zero` or a `-count` is a count's
/// reading and does not match.
fn emptiness_by_list(text: &str, names: &BTreeSet<String>, bare: bool) -> Vec<String> {
    let alternation = names
        .iter()
        .map(|name| regex::escape(name))
        .collect::<Vec<_>>()
        .join("|");
    let tick = if bare { "`?" } else { "`" };
    let end = if bare { "(?:`|[\\s,.;:)]|$)" } else { "`" };
    let pattern = Regex::new(&format!(
        "(?i)(?:^|[^-\\w])(?:empty|non-empty|no|zero)\\s+{tick}(?:{alternation}){end}\
         |`(?:{alternation})`\\s+(?:is|are)\\s+(?:empty|absent)"
    ))
    .unwrap();
    pattern
        .find_iter(text)
        .map(|found| {
            let phrase = found.as_str();
            let phrase = phrase
                .trim_start_matches(|c: char| !c.is_alphanumeric() && c != '`')
                .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '`');
            phrase.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .collect()
}

#[tokio::test]
async fn the_bounded_list_names_come_from_the_served_schemas() {
    let names = bounded_list_names(&served_output_schemas().await);
    for expected in [
        "unfolded",
        "rewritten",
        "updated",
        "open-questions",
        "tasks",
    ] {
        assert!(
            names.contains(expected),
            "`{expected}` is a bounded list but the schemas do not say so: {names:?}"
        );
    }
    assert!(
        !names.contains("loaded"),
        "`loaded` is `resolve-constitutions`' list, which has no `lists-file`: not bounded"
    );
}

#[test]
fn the_phrase_scan_finds_an_emptiness_stated_by_the_list_and_only_that() {
    let names: BTreeSet<String> = ["unfolded", "rewritten"].map(String::from).into();
    let stated = "An empty `unfolded` with a non-zero `rewritten` means nothing; reading an \n\
                  empty `rewritten` against it, and `unfolded` is empty too.";
    assert_eq!(
        emptiness_by_list(stated, &names, false),
        [
            "empty `unfolded`",
            "empty `rewritten`",
            "`unfolded` is empty"
        ]
    );
    let by_count = "A zero `unfolded-count` with a non-zero `examined` means nothing; \
                    reading a zero `rewritten-count`, and the `unfolded` list is read.";
    let none: Vec<String> = Vec::new();
    assert_eq!(emptiness_by_list(by_count, &names, false), none);
    assert_eq!(
        emptiness_by_list("empty unfolded and a zero unfolded-count", &names, true),
        ["empty unfolded"]
    );
    assert_eq!(emptiness_by_list("empty unfolded", &names, false), none);
}

#[tokio::test]
async fn no_command_step_states_a_bounded_lists_emptiness_by_the_list() {
    let names = bounded_list_names(&served_output_schemas().await);
    let commands = workspace_root().join("framework/commands");
    let mut examined = 0;
    let mut found: BTreeMap<(String, String), usize> = BTreeMap::new();
    for entry in fs::read_dir(&commands).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        examined += 1;
        let file = path.file_name().unwrap().to_string_lossy().to_string();
        for phrase in emptiness_by_list(&fs::read_to_string(&path).unwrap(), &names, false) {
            *found.entry((file.clone(), phrase)).or_default() += 1;
        }
    }
    assert!(examined > 0, "no command file was read under {commands:?}");

    let unlisted: Vec<_> = found
        .keys()
        .filter(|(file, phrase)| {
            !ALLOWED
                .iter()
                .any(|(f, p, _)| f == file && p.eq_ignore_ascii_case(phrase))
        })
        .collect();
    assert!(
        unlisted.is_empty(),
        "a step states a bounded list's emptiness by the list, which is absent when the result is \
         `form: \"file\"`; gate on its `-count` instead: {unlisted:?}"
    );

    let stale: Vec<_> = ALLOWED
        .iter()
        .filter(|(f, p, _)| {
            !found
                .keys()
                .any(|(file, phrase)| file == f && phrase.eq_ignore_ascii_case(p))
        })
        .collect();
    assert!(
        stale.is_empty(),
        "an allowed phrase no longer appears, so its entry hides nothing: {stale:?}"
    );
}

#[tokio::test]
async fn no_primitive_description_states_a_bounded_lists_emptiness_by_the_list() {
    let schemas = served_output_schemas().await;
    let descriptions = served_descriptions().await;
    let mut bounded_tools = 0;
    let mut found: BTreeMap<(String, String), usize> = BTreeMap::new();
    for (tool, schema) in &schemas {
        let names = bounded_lists_of(schema);
        if names.is_empty() {
            continue;
        }
        bounded_tools += 1;
        let texts: Vec<_> = descriptions
            .iter()
            .filter(|(name, _)| name == tool)
            .collect();
        assert!(!texts.is_empty(), "no description was read for `{tool}`");
        for (_, text) in texts {
            for phrase in emptiness_by_list(text, &names, true) {
                *found.entry((tool.clone(), phrase)).or_default() += 1;
            }
        }
    }
    assert!(
        bounded_tools > 20,
        "only {bounded_tools} tools have a bounded list, so the scan went quiet"
    );
    let unlisted: Vec<_> = found
        .keys()
        .filter(|(tool, phrase)| {
            !ALLOWED_DESCRIPTIONS
                .iter()
                .any(|(t, p, _)| t == tool && p.eq_ignore_ascii_case(phrase))
        })
        .collect();
    assert!(
        unlisted.is_empty(),
        "a description states a bounded list's emptiness by the list, which is absent when the \
         result is `form: \"file\"`; say it by the `-count`, which both forms carry: {unlisted:?}"
    );
    let stale: Vec<_> = ALLOWED_DESCRIPTIONS
        .iter()
        .filter(|(t, p, _)| {
            !found
                .keys()
                .any(|(tool, phrase)| tool == t && phrase.eq_ignore_ascii_case(p))
        })
        .collect();
    assert!(
        stale.is_empty(),
        "an allowed phrase no longer appears, so its entry hides nothing: {stale:?}"
    );
}
