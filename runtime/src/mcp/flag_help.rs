//! The shape of a `--{name}-json` flag's file, rendered from the input schema
//! the MCP tool publishes for the argument (spec 022, scenario
//! `every-mcp-argument-has-a-cli-form`), so a host on the CLI builds the file
//! from `--help` alone and the help cannot disagree with the tool.

use serde_json::{Map, Value};

/// How deep a nested type is expanded; a recursive type stops here.
const MAX_DEPTH: usize = 6;

/// The help text for the flag that carries the tool argument `property`, or
/// `None` when the schema has no such property. `root` is the tool's whole
/// input schema, which holds the `$defs` its `$ref`s name.
#[must_use]
pub fn shape_help(root: &Map<String, Value>, property: &str) -> Option<String> {
    let node = root.get("properties")?.get(property)?;
    let mut lines = vec![format!(
        "The file holds `{property}`: {}.",
        type_label(root, node)
    )];
    expand(root, node, 1, &mut lines);
    Some(lines.join("\n"))
}

/// Follow a `$ref` into the schema's `$defs`; any other node is returned as is.
fn resolve<'a>(root: &'a Map<String, Value>, node: &'a Value) -> &'a Value {
    node.get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| reference.strip_prefix("#/$defs/"))
        .and_then(|name| root.get("$defs")?.get(name))
        .unwrap_or(node)
}

fn ref_name(node: &Value) -> Option<&str> {
    node.get("$ref")?.as_str()?.rsplit('/').next()
}

/// The variants of a nullable or untagged node (`anyOf`, `oneOf`, or an
/// `allOf` wrapping one `$ref`), each still unresolved.
fn variants(node: &Value) -> Vec<&Value> {
    ["anyOf", "oneOf", "allOf"]
        .iter()
        .filter_map(|key| node.get(key)?.as_array())
        .flatten()
        .collect()
}

fn is_null(node: &Value) -> bool {
    node.get("type").and_then(Value::as_str) == Some("null")
}

/// The string values a node enumerates: an `enum` list, a `const`, or a
/// `oneOf`/`anyOf` whose every variant is one of those.
fn enum_values(node: &Value) -> Option<Vec<String>> {
    if let Some(values) = node.get("enum").and_then(Value::as_array) {
        return Some(values.iter().map(Value::to_string).collect());
    }
    if let Some(value) = node.get("const") {
        return Some(vec![value.to_string()]);
    }
    let parts = variants(node);
    if parts.is_empty() {
        return None;
    }
    let mut all = Vec::new();
    for part in parts {
        all.extend(enum_values(part)?);
    }
    Some(all)
}

/// A one-phrase type for a node: `string`, `integer`, `array of ReviewObservation`,
/// `object mapping string to string`, `string or null`, `one of "a", "b"`.
fn type_label(root: &Map<String, Value>, node: &Value) -> String {
    let resolved = resolve(root, node);
    if let Some(values) = enum_values(resolved) {
        return format!("one of {}", values.join(", "));
    }
    if let Some(name) = ref_name(node)
        && (resolved.get("properties").is_some() || !variants(resolved).is_empty())
    {
        return name.to_owned();
    }
    let parts = variants(resolved);
    if !parts.is_empty() {
        let nullable = parts.iter().any(|part| is_null(part));
        let labels: Vec<String> = parts
            .iter()
            .filter(|part| !is_null(part))
            .map(|part| type_label(root, part))
            .collect();
        return with_null(labels.join(" or "), nullable);
    }
    let types: Vec<&str> = match resolved.get("type") {
        Some(Value::Array(types)) => types.iter().filter_map(Value::as_str).collect(),
        Some(Value::String(ty)) => vec![ty.as_str()],
        _ => return "any JSON value".to_owned(),
    };
    let nullable = types.contains(&"null");
    let labels: Vec<String> = types
        .iter()
        .filter(|ty| **ty != "null")
        .map(|ty| match *ty {
            "array" => resolved.get("items").map_or_else(
                || "array".to_owned(),
                |items| format!("array of {}", type_label(root, items)),
            ),
            "object" => match resolved.get("additionalProperties") {
                Some(values) if values.is_object() => {
                    format!("object mapping string to {}", type_label(root, values))
                }
                _ => "object".to_owned(),
            },
            other => other.to_owned(),
        })
        .collect();
    with_null(labels.join(" or "), nullable)
}

fn with_null(label: String, nullable: bool) -> String {
    if nullable {
        format!("{label} or null")
    } else {
        label
    }
}

/// The object types a node holds, directly or inside an array, a map's values
/// or a nullable wrapper, each resolved.
fn object_types<'a>(root: &'a Map<String, Value>, node: &'a Value, out: &mut Vec<&'a Value>) {
    let resolved = resolve(root, node);
    if resolved.get("properties").is_some() {
        out.push(resolved);
        return;
    }
    if let Some(items) = resolved.get("items") {
        object_types(root, items, out);
    }
    if let Some(values) = resolved
        .get("additionalProperties")
        .filter(|v| v.is_object())
    {
        object_types(root, values, out);
    }
    for part in variants(resolved) {
        object_types(root, part, out);
    }
}

/// One line per field of each object type under `node`, nested types indented
/// below their field.
fn expand(root: &Map<String, Value>, node: &Value, depth: usize, lines: &mut Vec<String>) {
    if depth > MAX_DEPTH {
        return;
    }
    let mut objects = Vec::new();
    object_types(root, node, &mut objects);
    let indent = "  ".repeat(depth);
    for object in objects {
        let required: Vec<&str> = object
            .get("required")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let Some(properties) = object.get("properties").and_then(Value::as_object) else {
            continue;
        };
        for (name, field) in properties {
            let need = if required.contains(&name.as_str()) {
                "required"
            } else {
                "optional"
            };
            lines.push(format!(
                "{indent}{name}: {} ({need})",
                type_label(root, field)
            ));
            expand(root, field, depth + 1, lines);
        }
    }
}
