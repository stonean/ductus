//! A command step never states a bounded list's emptiness by the list (spec
//! 080, the scenario `a-step-gates-on-a-bounded-list-by-its-count`).
//!
//! Past `[artifacts] read-size-bytes` a bounded result has `form: "file"`:
//! each moved list is absent and only its `{list}-count` stays inline. A step
//! that reads "an empty `unfolded`" there reads an absent list as an empty one
//! and stops with "nothing to fold" while thousands are pending, so a step
//! that asks whether a list is empty reads the `-count`.
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

fn is_array(property: &Value) -> bool {
    property.to_string().contains("\"array\"")
}

/// The names that are a list with an inline `-count` in a tool whose result
/// can move to a file.
fn bounded_list_names(schemas: &[(String, serde_json::Map<String, Value>)]) -> BTreeSet<String> {
    let mut lists = BTreeSet::new();
    for (_, schema) in schemas {
        let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
            continue;
        };
        for (name, property) in properties {
            if is_array(property)
                && properties.contains_key("lists-file")
                && properties.contains_key(&format!("{name}-count"))
            {
                lists.insert(name.clone());
            }
        }
    }
    lists
}

/// Every phrase in `text` that states a bounded list's emptiness by the list:
/// `empty`, `non-empty`, `no` or `zero` before a backticked name, or a
/// backticked name followed by `is`/`are` `empty`/`absent`. A hyphenated
/// `non-zero` is a count's reading and does not match.
fn emptiness_by_list(text: &str, names: &BTreeSet<String>) -> Vec<String> {
    let alternation = names
        .iter()
        .map(|name| regex::escape(name))
        .collect::<Vec<_>>()
        .join("|");
    let pattern = Regex::new(&format!(
        "(?i)(?:^|[^-\\w])(?:empty|non-empty|no|zero)\\s+`(?:{alternation})`\
         |`(?:{alternation})`\\s+(?:is|are)\\s+(?:empty|absent)"
    ))
    .unwrap();
    pattern
        .find_iter(text)
        .map(|found| {
            let phrase = found.as_str();
            let phrase = phrase.trim_start_matches(|c: char| !c.is_alphanumeric() && c != '`');
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
        emptiness_by_list(stated, &names),
        [
            "empty `unfolded`",
            "empty `rewritten`",
            "`unfolded` is empty"
        ]
    );
    let by_count = "A zero `unfolded-count` with a non-zero `examined` means nothing; \
                    reading a zero `rewritten-count`, and the `unfolded` list is read.";
    let none: Vec<String> = Vec::new();
    assert_eq!(emptiness_by_list(by_count, &names), none);
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
        for phrase in emptiness_by_list(&fs::read_to_string(&path).unwrap(), &names) {
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
