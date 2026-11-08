//! The model-free result-size measurement (spec 080, AC14, AC23), run by
//! `scripts/measure-result-sizes.sh`. Ignored, so `cargo test` and CI never
//! run it.
//!
//! Every name in `PRIMITIVE_REGISTRY` is called on two generated corpora,
//! [`corpus::SMALL`] and [`corpus::LARGE`], through the built binary's MCP
//! server over stdio, each call on a fresh copy so a writing primitive
//! cannot change what the next one reads. What is recorded is the
//! characters of the result's text, the form a host reads it in. A name
//! the fixture cannot exercise gets a *not measured* row with its reason,
//! never an omitted one, and a registry name with no case fails the run.
//!
//! A primitive **grows with the corpus** when its result on the large
//! corpus is at least twice its result on the small one and at least 200
//! characters longer: a count's extra digits pass neither test, and a list
//! that scales with the corpus passes both.
//!
//! The same run records each served tool's loaded schema — its description
//! plus its input schema serialized as compact JSON, in characters — from
//! `tools/list`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Write as _};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Stdio};

use common::corpus::{self, CorpusSize, SUBJECT};
use serde_json::{Value, json};

/// The work a registry name gets on the fixture.
enum Case {
    /// Call the tool with these arguments.
    Call(Value),
    /// Call it with these arguments after linking this checkout's
    /// `node_modules` into the copy, since it runs markdownlint.
    CallWithNodeModules(Value),
    /// Not measured, for this reason.
    NotMeasured(&'static str),
}

/// What a run on one corpus needs to build a case's arguments.
struct Context {
    size: CorpusSize,
    base: String,
    head: String,
}

fn subject_path() -> String {
    format!("specs/{SUBJECT}")
}

/// Why a registry name is not measured on the fixture, when it is not.
fn not_measured(name: &str) -> Option<&'static str> {
    Some(match name {
        "apply-manifest" => {
            "its result follows the manifest the host builds and needs a staged framework tree; a spec corpus holds neither"
        }
        "check-promotion-coverage" => {
            "it audits the framework's own rule file against command sections, which a spec corpus does not hold"
        }
        "enforce-manifest" => {
            "its result follows the expected-file list the host passes, not the spec corpus"
        }
        "extract-archive" => "it needs a release archive, which a spec corpus does not hold",
        "fetch-archive" => "it needs the network",
        "run-generator" => {
            "CLI-only: it runs a project script and is no MCP tool, so no host reads its result as a tool result"
        }
        _ => return None,
    })
}

fn case(name: &str, ctx: &Context) -> Case {
    if let Some(reason) = not_measured(name) {
        return Case::NotMeasured(reason);
    }
    let f = SUBJECT;
    let p = subject_path();
    let spec = format!("{p}/spec.md");
    let base = ctx.base.as_str();
    let head = ctx.head.as_str();
    let call = Case::Call;
    match name {
        "check-command-flags"
        | "check-corpus-links"
        | "check-orphaned-references"
        | "check-step-references"
        | "check-unfolded-specs"
        | "dashboard"
        | "derive-dependencies"
        | "derive-references"
        | "discover-rule-files"
        | "measure-reopenings"
        | "prune-project"
        | "render-status"
        | "resolve-constitutions"
        | "resolve-session" => call(json!({})),
        "check-artifact-size"
        | "check-artifacts"
        | "check-review-freshness"
        | "check-review-gate"
        | "classify-spec-edit"
        | "compute-review-scope"
        | "create-plan-artifacts"
        | "derive-boundary"
        | "diff-cross-spec"
        | "invalidate-review"
        | "label-criteria"
        | "process-waivers"
        | "prune-plan"
        | "prune-tasks"
        | "read-spec"
        | "read-tasks"
        | "relocate-audit-records"
        | "resolve-references"
        | "start-fix-rounds" => call(json!({"feature": f})),
        "brief-re-read" | "check-prescriptions" | "read-re-read" => {
            call(json!({"feature": f, "base": base}))
        }
        "resolve-anchor" | "validate-frontmatter" => call(json!({"path": spec})),
        "append-inbox" => call(json!({"text": "A new corpus todo"})),
        "append-question" => call(json!({"feature": f, "question": "Is case zero handled?"})),
        "append-task" => call(
            json!({"feature-path": p, "title": "Handle case group zero", "done-when": "case group zero's tests pass", "body": ["Write case group zero, with its test"]}),
        ),
        "check-rule-ids" => {
            call(json!({"path": spec, "rule-files": ["specs/rules/quality-cross.md"]}))
        }
        "check-stuck" => call(json!({"feature": f, "threshold": 3})),
        "create-feature" => call(json!({"title": "A new corpus feature"})),
        "create-scenario" => call(
            json!({"feature-path": p, "slug": "a-new-case", "section": "Behavior", "body": "## Context\n\nA new case.\n"}),
        ),
        "derive-routing-candidates" => {
            call(json!({"description": "handle a case the general path does not expect"}))
        }
        "gate-confirm" => call(json!({"gate": "corpus", "prompt": "Approve the corpus?"})),
        "lint-markdown" => Case::CallWithNodeModules(json!({"paths": ["specs"]})),
        "mark-criterion" => call(json!({"feature": f, "checked": true, "criterion-index": 0})),
        "mark-task" => call(
            json!({"feature": f, "task-number": ctx.size.tasks.to_string(), "subtask-index": 0, "checked": true}),
        ),
        "merge-managed-block" => call(json!({"path": "AGENTS.md", "block": "corpus block"})),
        "merge-permissions" => call(json!({"path": ".claude/settings.local.json"})),
        "migrate-session-file" => call(json!({"legacy-path": ".claude/corpus-session.json"})),
        "process-decisions" => call(json!({"feature": f, "record": "review"})),
        "remove-inbox-item" => call(json!({"item": "Corpus inbox item 001, logged by hand"})),
        "resolve-feature" => call(json!({"identifier": f})),
        "retarget-sessions" => call(json!({"from": f, "cause": "consolidate", "clear": true})),
        "retire-feature" => {
            call(json!({"feature": corpus::slug(2), "fold-target": f, "allow-sequential": true}))
        }
        "rewrite-spec-links" => call(json!({"from": corpus::slug(2), "to": f})),
        "set-status" => call(json!({"feature": f, "from": "in-progress", "to": "done"})),
        // The last spec's chain runs through every spec, so its traversal is
        // the one that scales.
        "traverse-deps" => call(json!({"feature": corpus::slug(ctx.size.specs)})),
        "write-analysis" => call(
            json!({"feature": f, "analyzed-at": "2026-11-08T00:00:00Z", "analyzed-against": head}),
        ),
        "write-review" => call(
            json!({"feature": f, "reviewed-at": "2026-11-08T00:00:00Z", "reviewed-against": head, "diff-base": base}),
        ),
        "write-session" => call(json!({"feature": f, "path": p})),
        other => panic!(
            "`{other}` is in PRIMITIVE_REGISTRY with no case here; add one, or a not-measured row with its reason"
        ),
    }
}

/// The MCP server of the built binary, rooted at `repo`, initialized.
struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl Server {
    fn start(repo: &Path) -> Self {
        let command = common::ductus_command(env!("CARGO_BIN_EXE_ductus"));
        let mut command = common::without_git_hook_variables(command);
        let mut child = command
            .arg("mcp")
            .current_dir(repo)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        let mut server = Self {
            child,
            stdin,
            stdout,
            next_id: 1,
        };
        server.request(
            "initialize",
            &json!({"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "result-sizes", "version": "0"}}),
        );
        server.send(&json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
        server
    }

    fn send(&mut self, message: &Value) {
        writeln!(self.stdin, "{message}").unwrap();
        self.stdin.flush().unwrap();
    }

    fn request(&mut self, method: &str, params: &Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        self.send(&json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}));
        loop {
            let mut line = String::new();
            assert!(
                self.stdout.read_line(&mut line).unwrap() > 0,
                "the server closed stdout before answering `{method}`"
            );
            let message: Value = serde_json::from_str(&line).unwrap();
            if message.get("id") == Some(&json!(id)) {
                return message;
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// One call's outcome.
enum Outcome {
    /// The characters of the result's text, and whether the result carries
    /// `bounded_lists`' `form` field, so is bounded already.
    Measured { chars: usize, bounded: bool },
    /// The tool returned an error, whose text is kept.
    Errored(String),
    /// Not measured, for this reason.
    NotMeasured(&'static str),
}

fn run_case(template: &Path, name: &str, ctx: &Context) -> Outcome {
    let (args, link) = match case(name, ctx) {
        Case::Call(args) => (args, false),
        Case::CallWithNodeModules(args) => (args, true),
        Case::NotMeasured(reason) => return Outcome::NotMeasured(reason),
    };
    let copy = tempfile::tempdir().unwrap();
    common::copy_dir_recursive(template, copy.path());
    if link {
        link_node_modules(copy.path());
    }
    let mut server = Server::start(copy.path());
    let response = server.request("tools/call", &json!({"name": name, "arguments": args}));
    if let Some(error) = response.get("error") {
        return Outcome::Errored(error.to_string());
    }
    let result = &response["result"];
    let text: String = result["content"]
        .as_array()
        .map(|parts| {
            parts
                .iter()
                .filter_map(|part| part["text"].as_str())
                .collect()
        })
        .unwrap_or_default();
    if result["isError"] == json!(true) {
        return Outcome::Errored(text);
    }
    let bounded =
        serde_json::from_str::<Value>(&text).is_ok_and(|value| value.get("form").is_some());
    Outcome::Measured {
        chars: text.chars().count(),
        bounded,
    }
}

/// Link this checkout's `node_modules` into a copy, so `lint-markdown`
/// runs the real markdownlint against the project's own configuration.
fn link_node_modules(root: &Path) {
    let checkout = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    std::fs::copy(
        checkout.join(".markdownlint-cli2.jsonc"),
        root.join(".markdownlint-cli2.jsonc"),
    )
    .unwrap();
    // The configuration lints `**/*.md` and honours `.gitignore`, so the link
    // is ignored as an adopter's `node_modules` is. No trailing slash: that
    // form matches no symbolic link.
    std::fs::write(root.join(".gitignore"), "node_modules\n").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(checkout.join("node_modules"), root.join("node_modules")).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(checkout.join("node_modules"), root.join("node_modules"))
        .unwrap();
}

fn commit_shas(root: &Path) -> (String, String) {
    let repo = git2::Repository::open(root).unwrap();
    let mut walk = repo.revwalk().unwrap();
    walk.push_head().unwrap();
    walk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)
        .unwrap();
    let shas: Vec<String> = walk.map(|oid| oid.unwrap().to_string()).collect();
    (shas[0].clone(), shas[shas.len() - 1].clone())
}

/// Every registry name's outcome on a corpus of `size`.
fn measure(size: CorpusSize) -> Vec<(&'static str, Outcome)> {
    let template = tempfile::tempdir().unwrap();
    corpus::generate(template.path(), &size);
    let (base, head) = commit_shas(template.path());
    let ctx = Context { size, base, head };
    ductus::schema::registry::PRIMITIVE_REGISTRY
        .iter()
        .map(|name| (*name, run_case(template.path(), name, &ctx)))
        .collect()
}

fn cell(outcome: &Outcome) -> String {
    match outcome {
        Outcome::Measured { chars, .. } => chars.to_string(),
        Outcome::Errored(text) => {
            let first: String = text
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(120)
                .collect();
            format!("error: {}", first.replace('|', "\\|"))
        }
        Outcome::NotMeasured(_) => "—".to_string(),
    }
}

fn grows(small: &Outcome, large: &Outcome) -> &'static str {
    match (small, large) {
        (Outcome::Measured { chars: s, .. }, Outcome::Measured { chars: l, .. }) => {
            if *l >= 2 * *s && *l >= *s + 200 {
                "yes"
            } else {
                "no"
            }
        }
        (Outcome::NotMeasured(_), _) | (_, Outcome::NotMeasured(_)) => "not measured",
        _ => "not determined: a call errored",
    }
}

fn describe(size: &CorpusSize) -> String {
    let defects = if size.defects {
        ", one markdown-lint violation and one broken link"
    } else {
        ""
    };
    format!(
        "{} specs, each with {} criteria, {} tasks of {} subtasks and {} scenarios{defects}; {} inbox items",
        size.specs, size.criteria, size.tasks, size.subtasks, size.scenarios, size.inbox_items
    )
}

fn result_table() -> String {
    let small = measure(corpus::SMALL);
    let large = measure(corpus::LARGE);
    let mut out = format!(
        "Small corpus: {}. Large corpus: {}.\n\n\
         | Primitive | Small (chars) | Large (chars) | Grows | Bounded already | Note |\n| --- | --- | --- | --- | --- | --- |\n",
        describe(&corpus::SMALL),
        describe(&corpus::LARGE)
    );
    for ((name, s), (_, l)) in small.iter().zip(&large) {
        // An empty cell is one space, as markdownlint's compact table style
        // (MD060) requires.
        let note = match s {
            Outcome::NotMeasured(reason) => *reason,
            _ => "",
        };
        let note = if note.is_empty() {
            String::new()
        } else {
            format!("{note} ")
        };
        let bounded = match l {
            Outcome::Measured { bounded: true, .. } => "yes",
            Outcome::Measured { bounded: false, .. } => "no",
            _ => "—",
        };
        let _ = writeln!(
            out,
            "| `{name}` | {} | {} | {} | {bounded} | {note}|",
            cell(s),
            cell(l),
            grows(s, l)
        );
    }
    out
}

fn schema_table() -> String {
    let template = tempfile::tempdir().unwrap();
    corpus::generate(template.path(), &corpus::SMALL);
    let mut server = Server::start(template.path());
    let response = server.request("tools/list", &json!({}));
    let tools = response["result"]["tools"].as_array().unwrap().clone();
    let mut rows: Vec<(String, usize, usize)> = tools
        .iter()
        .map(|tool| {
            let name = tool["name"].as_str().unwrap().to_string();
            let description = tool["description"].as_str().unwrap_or("").chars().count();
            let input = serde_json::to_string(&tool["inputSchema"])
                .unwrap()
                .chars()
                .count();
            (name, description, input)
        })
        .collect();
    rows.sort_by(|a, b| (b.1 + b.2).cmp(&(a.1 + a.2)).then(a.0.cmp(&b.0)));
    let total: usize = rows.iter().map(|(_, d, i)| d + i).sum();
    let mut out = format!(
        "{} tools, {total} characters in all.\n\n\
         | Tool | Description | Input schema | Loaded |\n| --- | --- | --- | --- |\n",
        rows.len()
    );
    for (name, d, i) in &rows {
        let _ = writeln!(out, "| `{name}` | {d} | {i} | {} |", d + i);
    }
    out
}

/// Every registry name has a case, so a primitive added without one fails
/// here, in `cargo test`, rather than only when the measurement runs.
#[test]
fn every_registry_name_has_a_case() {
    let ctx = Context {
        size: corpus::SMALL,
        base: String::new(),
        head: String::new(),
    };
    for name in ductus::schema::registry::PRIMITIVE_REGISTRY {
        let _ = case(name, &ctx);
    }
}

/// The generator scales every dimension it is given, and builds the
/// three-commit history, with the subject `in-progress` at its end.
#[test]
fn the_corpus_scales_every_dimension() {
    let size = CorpusSize {
        specs: 4,
        criteria: 3,
        tasks: 6,
        subtasks: 2,
        scenarios: 5,
        inbox_items: 7,
        defects: true,
    };
    let tmp = tempfile::tempdir().unwrap();
    corpus::generate(tmp.path(), &size);
    let specs = tmp.path().join("specs");
    let count = |marker: &str| {
        std::fs::read_dir(&specs)
            .unwrap()
            .filter(|entry| {
                let name = entry.as_ref().unwrap().file_name();
                name.to_string_lossy().contains(marker)
            })
            .count()
    };
    assert_eq!(count("-feature-"), size.specs);
    assert_eq!(count("-staged-"), 1);
    let staged =
        std::fs::read_to_string(specs.join(corpus::branch_slug(1)).join("spec.md")).unwrap();
    assert!(
        staged.contains(&format!("folds-into: {}\n", corpus::slug(size.specs))),
        "{staged}"
    );
    let subject = specs.join(SUBJECT);
    let spec = std::fs::read_to_string(subject.join("spec.md")).unwrap();
    assert!(spec.starts_with("---\nstatus: in-progress\n"), "{spec}");
    assert_eq!(spec.matches("] AC").count(), size.criteria);
    assert!(
        spec.contains("## Notes:\n\nSee [the guide](../../docs/missing-guide.md)."),
        "{spec}"
    );
    let tasks = std::fs::read_to_string(subject.join("tasks.md")).unwrap();
    assert_eq!(tasks.matches("\n## ").count(), size.tasks);
    assert_eq!(tasks.matches("- [x] ").count(), size.tasks * size.subtasks);
    let scenarios = std::fs::read_dir(subject.join("scenarios"))
        .unwrap()
        .count();
    assert_eq!(scenarios, size.scenarios);
    let inbox = std::fs::read_to_string(specs.join("inbox.md")).unwrap();
    assert_eq!(inbox.matches("- [ ] ").count(), size.inbox_items);
    let last =
        std::fs::read_to_string(specs.join(corpus::slug(size.specs)).join("spec.md")).unwrap();
    assert!(
        last.contains(&format!("](../{}/spec.md)", corpus::slug(size.specs - 1))),
        "{last}"
    );
    let repo = git2::Repository::open(tmp.path()).unwrap();
    let mut walk = repo.revwalk().unwrap();
    walk.push_head().unwrap();
    assert_eq!(walk.count(), 3);
}

#[test]
#[ignore = "a measurement, run by scripts/measure-result-sizes.sh"]
fn result_sizes() {
    let report = format!(
        "## Result sizes\n\n{}\n## Loaded schema sizes\n\n{}",
        result_table(),
        schema_table()
    );
    println!("{report}");
    if let Some(path) = std::env::var_os("DUCTUS_RESULT_SIZES_OUT") {
        std::fs::write(path, &report).unwrap();
    }
}
