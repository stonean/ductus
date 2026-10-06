//! Extension-point request builders.
//!
//! Bundles the static context (`constitution-excerpts`, `plan-relevant-files`,
//! `write-boundary`) that LLM extension points need into the typed
//! [`WriteCodeRequest`] / [`WriteSpecBodyRequest`] /
//! [`AssessSpecQualityRequest`] / [`AskClarifyQuestionRequest`] /
//! [`RouteInboxItemRequest`] / [`VerifyCriteriaRequest`] shapes defined in
//! [`crate::schema::extensions`]. The interpreter calls
//! [`build_extension_request`] just before emitting an `llm-request` envelope;
//! the result replaces the previous "dump the walker context as the request"
//! behavior with a payload whose field order is cache-anchored per the
//! spec 022 LLM extension points contract (stable prefix front, per-task
//! variable suffix last).
//!
//! Per the extension-request-hygiene scenario, walker-internal accumulator
//! keys (prior `llm:*` response echoes and the cross-pass `findings`
//! array) are filtered from every legacy-compat context merge, and an
//! unknown extension identifier is a structured error rather than a raw
//! context dump.
//!
//! [`WriteCodeRequest`]: crate::schema::extensions::WriteCodeRequest
//! [`WriteSpecBodyRequest`]: crate::schema::extensions::WriteSpecBodyRequest
//! [`AssessSpecQualityRequest`]: crate::schema::extensions::AssessSpecQualityRequest
//! [`AskClarifyQuestionRequest`]: crate::schema::extensions::AskClarifyQuestionRequest
//! [`RouteInboxItemRequest`]: crate::schema::extensions::RouteInboxItemRequest
//! [`VerifyCriteriaRequest`]: crate::schema::extensions::VerifyCriteriaRequest

#![allow(clippy::expect_used)]

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use regex::Regex;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::host::Host;
use crate::primitives::read_tasks;
use crate::primitives::rule_sections::parse_rule_sections;
use crate::schema::extensions::{
    AskClarifyQuestionRequest, AssessSpecQualityRequest, AssessSpecQualityRule, ClarifyQuestion,
    FoldSourceScenario, PerformReviewRequest, PlanRelevantFile, ReviewRuleFile, ReviewScopeFile,
    RouteFoldRequest, RouteInboxItemRequest, RouteInboxSpec, VerifyCriteriaRequest,
    VerifyCriterion, WriteCodeRequest, WriteCodeTask, WriteSpecBodyRequest,
};
use crate::schema::paths;
use crate::schema::primitives::ReadTasksArgs;
use crate::schema::severity::RuleSeverity;

/// Errors that abort payload construction. The interpreter surfaces these
/// as structured `error` envelopes (e.g.,
/// [`SecretExfiltration`](PayloadError::SecretExfiltration) → code
/// `secret-exfiltration-blocked`).
#[derive(Debug, thiserror::Error)]
pub enum PayloadError {
    /// A path listed in the plan's Affected Files matched a secret-bearing
    /// pattern (`.env`, `credentials*`, etc.), was marked ignored by
    /// `.gitignore`, or canonicalized to a location outside the repo root
    /// (path traversal — pattern label `out-of-repo`). The root here is the
    /// project root, which is a subdirectory of the git repository when the
    /// project sits in one (spec 059); the label keeps its name because it is
    /// part of the envelope a host already parses.
    #[error("secret-exfiltration-blocked: '{path}' matches pattern '{pattern}'")]
    SecretExfiltration {
        /// Offending repo-relative path.
        path: String,
        /// Pattern that matched: a glob name (`.env`, `.env.*`,
        /// `*-secrets.*`, `credentials*`), `.gitignore`, or `out-of-repo`
        /// for paths whose canonical form escapes the repo root.
        pattern: String,
    },
    /// The extension identifier has no typed request builder in this
    /// runtime version. Emitting the raw walker context (the pre-hygiene
    /// fallback) would leak accumulator state to the host, so the walk
    /// halts with a structured error instead
    /// (extension-request-hygiene scenario).
    #[error("unknown extension point `{identifier}`: no typed request builder in this runtime")]
    UnknownExtension {
        /// The unrecognized extension identifier from the step marker.
        identifier: String,
    },
}

/// Machine-readable code emitted in the `error` envelope when payload
/// construction fails. Kept stable for host integrations.
impl PayloadError {
    /// Return the envelope code that corresponds to this variant.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::SecretExfiltration { .. } => "secret-exfiltration-blocked",
            // Matches the code the walker emits when a response arrives
            // for an identifier `validate_response` does not know.
            Self::UnknownExtension { .. } => "unknown-extension",
        }
    }
}

/// Build the request payload for an extension point, returning a JSON value
/// the walker can drop straight into the `llm-request.request` field.
///
/// Behavior by `identifier`:
///
/// - `writeCode` — builds [`WriteCodeRequest`] from the targeted feature's
///   `plan.md` (for `plan-relevant-files`), the command file's `Reference:`
///   line (for `constitution-excerpts`), the walker context's
///   `write-boundary`, and the current task pulled from `tasks.md` (using
///   `feature` + `task-number` from the walker context). Legacy context
///   fields are appended after the typed prefix for backward compatibility
///   with hosts that already parse them (walker-internal accumulator keys
///   filtered — see [`is_walker_internal_key`]).
/// - `writeSpecBody` — builds [`WriteSpecBodyRequest`]: the template the
///   running command fills (`/ductus:plan` → the plan template, `/ductus:specify`
///   → the spec template), the section named by the step prose, the
///   `feature-description` context key, and `existing-content` when the
///   named section already has body content in the file the running
///   command owns. Filtered legacy context fields follow the typed prefix.
/// - `performReview` — builds [`PerformReviewRequest`] (see
///   [`build_perform_review_request`]); filtered legacy context fields
///   follow the typed prefix, preserving the primitive results the pass
///   needs (`scope`/`diff-base`, `selected`/`rules-dir`/`notices`).
/// - `assessSpecQuality` — builds [`AssessSpecQualityRequest`] from the
///   walker context's `path` (the spec under review, read off disk for
///   `spec-content`) and the one rule the walker seeded under
///   [`ASSESSED_RULE_KEY`], carrying the tier its own Statement states. The
///   walker sends one such request per loaded rule of a step's tier.
///   Emits the documented typed shape only — the data model sanctions no
///   legacy context dump here.
/// - `askClarifyQuestion` — builds [`AskClarifyQuestionRequest`] from the
///   spec resolved via `path`/`feature` and the question from the
///   `question` context value (falling back to the first merged
///   `open-questions` entry). Typed shape only.
/// - `routeInboxItem` — builds [`RouteInboxItemRequest`] from the
///   `item-text` context key, the fixed route vocabulary, and a scan of
///   the spec root for available features. Typed shape only.
/// - `routeFold` — builds [`RouteFoldRequest`] from the branch-scoped spec
///   resolved via `path`/`feature`, its own scenarios, and the upstream
///   spec its `folds-into` names (see [`build_route_fold_request`]). Typed
///   shape only.
/// - `verifyCriteria` — builds [`VerifyCriteriaRequest`] from the spec
///   resolved via `path`/`feature` and the completion gate's merged
///   `acceptance-criteria` result (see
///   [`build_verify_criteria_request`]). Typed shape only.
/// - any other identifier — [`PayloadError::UnknownExtension`]; the raw
///   context dump fallback was removed by the extension-request-hygiene
///   scenario.
///
/// # Errors
///
/// Returns [`PayloadError::SecretExfiltration`] when any path in
/// `plan-relevant-files` matches a secret-bearing pattern or `.gitignore`,
/// and [`PayloadError::UnknownExtension`] for an identifier with no typed
/// builder.
pub fn build_extension_request(
    identifier: &str,
    context: &Map<String, Value>,
    repo: &Path,
    command_name: &str,
    step_prose: &str,
) -> Result<Value, PayloadError> {
    match identifier {
        "writeCode" => build_write_code_request(context, repo, command_name),
        "writeSpecBody" => Ok(build_write_spec_body_request(
            context,
            repo,
            command_name,
            step_prose,
        )),
        "performReview" => Ok(build_perform_review_request(context, repo)),
        "assessSpecQuality" => Ok(build_assess_spec_quality_request(context, repo)),
        "askClarifyQuestion" => Ok(build_ask_clarify_question_request(context, repo)),
        "routeInboxItem" => Ok(build_route_inbox_item_request(context, repo)),
        "routeFold" => Ok(build_route_fold_request(context, repo)),
        "verifyCriteria" => Ok(build_verify_criteria_request(context, repo)),
        other => Err(PayloadError::UnknownExtension {
            identifier: other.to_string(),
        }),
    }
}

/// Context keys the walker accumulates across `performReview` passes and hands
/// to `write-review` as a single union each. Both are pass *outputs*, so a
/// later pass must never see an earlier one's — they are filtered out of every
/// outbound request by [`is_walker_internal_key`], which reads this same list.
pub(crate) const PERFORM_REVIEW_ACCUMULATORS: &[&str] = &["findings", "observations"];

/// `true` for walker-internal accumulator keys that never belong in an
/// outbound request: prior `llm:<identifier>` response echoes and the
/// cross-pass [`PERFORM_REVIEW_ACCUMULATORS`] arrays the walker accumulates
/// for `write-review`. Primitive results threaded through the context
/// (`scope`, `diff-base`, `selected`, `rules-dir`, `notices`, …) are NOT
/// accumulator state and pass through the merge untouched.
fn is_walker_internal_key(key: &str) -> bool {
    PERFORM_REVIEW_ACCUMULATORS.contains(&key) || key.starts_with("llm:")
}

/// Append the legacy-compat context fields after a typed prefix, skipping
/// walker-internal accumulator keys ([`is_walker_internal_key`]) and any
/// key the typed prefix already emitted (typed values win).
fn merge_legacy_context(object: &mut Map<String, Value>, context: &Map<String, Value>) {
    for (key, value) in context {
        if is_walker_internal_key(key) {
            continue;
        }
        object.entry(key.clone()).or_insert_with(|| value.clone());
    }
}

/// Serialize a typed request and append the filtered legacy-compat context
/// fields after it ([`merge_legacy_context`]). The typed fields lead in
/// declaration order (the cache anchor); walker-internal accumulator keys
/// are dropped and keys the typed prefix already emitted win. Shared epilogue
/// for the builders that carry a legacy tail (`writeCode`, `writeSpecBody`,
/// `performReview`).
fn typed_with_legacy_context<T: Serialize>(typed: &T, context: &Map<String, Value>) -> Value {
    let typed_value = serde_json::to_value(typed).unwrap_or(Value::Null);
    let mut object = match typed_value {
        Value::Object(map) => map,
        _ => Map::new(),
    };
    merge_legacy_context(&mut object, context);
    Value::Object(object)
}

/// Serialize a typed request as the entire payload, with no legacy context
/// tail. Shared epilogue for the builders the data model documents as
/// bare-typed (`assessSpecQuality`, `askClarifyQuestion`, `routeInboxItem`,
/// `routeFold`, `verifyCriteria`) — the previous raw walker-context dump is exactly the
/// behavior those builders replace.
fn typed_only<T: Serialize>(typed: &T) -> Value {
    serde_json::to_value(typed).unwrap_or(Value::Null)
}

/// Build the `performReview` request for one pass. Loads the in-scope files
/// (`scope`, from `compute-review-scope`) and the pass's rule files
/// (`selected` basenames under `rules-dir`, from `discover-rule-files`) off
/// disk, and pairs them with the `pass` name. Missing/unreadable files are
/// skipped rather than erroring — the pass reviews what it can read. The
/// typed prefix leads (cache-anchor order: `scope-files` is stable across
/// passes); legacy context fields follow for hosts that already parse them.
fn build_perform_review_request(context: &Map<String, Value>, repo: &Path) -> Value {
    let pass = context
        .get("pass")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let scope_files = load_scope_files(context, repo);
    let rule_files = load_rule_files(context, repo);

    let typed = PerformReviewRequest {
        scope_files,
        rule_files,
        pass,
    };
    // Filtered merge: pass N must not see passes 1..N-1's accumulated
    // `findings` or raw `llm:performReview` echoes, but keeps the
    // primitive results it legitimately needs (`scope`/`diff-base` from
    // compute-review-scope, `selected`/`rules-dir`/`notices` from
    // discover-rule-files).
    typed_with_legacy_context(&typed, context)
}

/// Classification of a repo-relative path against the canonicalized repo root
/// — the BE-INPUT-004 containment primitive shared by the scope, rule, and
/// plan file readers.
enum Contained {
    /// Canonical absolute path that stays inside the repo root.
    Inside(PathBuf),
    /// Path does not resolve to an existing file (`canonicalize` failed).
    Missing,
    /// Canonical path escapes the repo root — an absolute joinee, a `..`
    /// traversal, or a symlink whose target lands outside the repo.
    Outside,
}

/// Resolve `rel` against the already-canonicalized `canon_repo` and classify
/// whether its canonical form stays within the repo. Callers decide how to
/// treat an `Outside` path: the best-effort review readers skip it, while the
/// writeCode plan reader treats it as an exfiltration attempt and errors.
fn classify_contained(canon_repo: &Path, rel: &Path) -> Contained {
    match canon_repo.join(rel).canonicalize() {
        Ok(abs) if abs.starts_with(canon_repo) => Contained::Inside(abs),
        Ok(_) => Contained::Outside,
        Err(_) => Contained::Missing,
    }
}

/// Load `scope` paths (from `compute-review-scope`) into `ReviewScopeFile`
/// records, reading each file's content.
///
/// BE-INPUT-004: `scope` originates from plan-authored `## Affected Files`
/// entries (via `compute-review-scope`'s `read_plan_affected`), so each path
/// is canonicalized and confined to the repo root before it is opened — an
/// absolute or traversing entry is skipped, never read into the review
/// payload. Missing and unreadable paths are likewise skipped (best-effort).
fn load_scope_files(context: &Map<String, Value>, repo: &Path) -> Vec<ReviewScopeFile> {
    let Ok(canon_repo) = repo.canonicalize() else {
        return Vec::new();
    };
    string_array(context, "scope")
        .into_iter()
        .filter_map(
            |path| match classify_contained(&canon_repo, Path::new(&path)) {
                Contained::Inside(abs) => std::fs::read_to_string(&abs)
                    .ok()
                    .map(|content| ReviewScopeFile { path, content }),
                Contained::Missing | Contained::Outside => None,
            },
        )
        .collect()
}

/// Load the pass's `selected` rule basenames (from `discover-rule-files`)
/// under `rules-dir` into `ReviewRuleFile` records. Unreadable files are
/// skipped.
fn load_rule_files(context: &Map<String, Value>, repo: &Path) -> Vec<ReviewRuleFile> {
    let rules_dir = context
        .get("rules-dir")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let Ok(canon_repo) = repo.canonicalize() else {
        return Vec::new();
    };
    // `rules-dir` and `selected` come from `discover-rule-files` (a directory
    // walk of trusted basenames), but the same BE-INPUT-004 containment check
    // is applied defensively so the reader cannot escape the repo regardless.
    string_array(context, "selected")
        .into_iter()
        .filter_map(|name| {
            let rel = Path::new(rules_dir).join(&name);
            match classify_contained(&canon_repo, &rel) {
                Contained::Inside(abs) => std::fs::read_to_string(&abs)
                    .ok()
                    .map(|content| ReviewRuleFile { name, content }),
                Contained::Missing | Contained::Outside => None,
            }
        })
        .collect()
}

/// Read a context key as a `Vec<String>`, dropping non-string members.
/// Empty when the key is absent or not an array.
fn string_array(context: &Map<String, Value>, key: &str) -> Vec<String> {
    context
        .get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn build_write_code_request(
    context: &Map<String, Value>,
    repo: &Path,
    command_name: &str,
) -> Result<Value, PayloadError> {
    let feature = context
        .get("feature")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let plan_relevant_files = load_plan_relevant_files(&feature, repo)?;
    let excerpts = load_constitution_excerpts(command_name, repo);
    let write_boundary = string_array(context, "write-boundary");
    let task = load_current_task(&feature, context, repo);

    let typed = WriteCodeRequest {
        constitution_excerpts: excerpts.excerpts,
        constitution_excerpts_unexaminable: excerpts.unexaminable,
        plan_relevant_files,
        write_boundary,
        task,
    };

    // Merge: typed prefix first (declaration order = cache-anchor order),
    // then legacy context fields that hosts may already parse (filtered:
    // no `llm:*` echoes or accumulated `findings` — the whole-session dump
    // eroded the cache anchor). Keys present in both prefer the typed value.
    Ok(typed_with_legacy_context(&typed, context))
}

/// Build the `writeSpecBody` request. All documented typed fields are
/// populated from the walker context and disk (mirroring
/// `build_assess_spec_quality_request`'s "typed even when bare"
/// discipline — a field the run cannot derive is an empty string, never a
/// dropped key):
///
/// - `template-path` / `template-content` — the template the running
///   command fills, resolved by [`load_template`].
/// - `section` — the section heading named by the step prose; empty when
///   the step fills a whole body rather than one section (`/ductus:specify`).
/// - `feature-description` — the `feature-description` context key the
///   host seeds from the slash command's `$ARGUMENTS`; empty when unset.
/// - `existing-content` — the section's current body in the file the
///   running command owns (plan.md for `/ductus:plan`, spec.md for
///   `/ductus:specify`); omitted when absent or empty.
///
/// Filtered legacy context fields follow the typed prefix for hosts that
/// already parse them.
fn build_write_spec_body_request(
    context: &Map<String, Value>,
    repo: &Path,
    command_name: &str,
    step_prose: &str,
) -> Value {
    let section = extract_section_name(step_prose).unwrap_or_default();
    let (template_path, template_content) = load_template(command_name, repo);
    let feature_description = context
        .get("feature-description")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let existing_content = if section.is_empty() {
        None
    } else {
        let feature = context.get("feature").and_then(Value::as_str);
        let path_hint = context.get("path").and_then(Value::as_str);
        read_existing_section(&section, feature, path_hint, repo, command_name)
    };
    let typed = WriteSpecBodyRequest {
        template_path,
        template_content,
        section,
        feature_description,
        existing_content,
    };
    typed_with_legacy_context(&typed, context)
}

/// Resolve the template file the running command fills. `/ductus:plan` fills
/// plan sections from the plan template; `/ductus:specify` fills the spec
/// body from the spec template. Candidates, in order: the installed
/// adopter layout `{specs-root}/templates/<file>` (what `/ductus`
/// scaffolds and the command prose names), then the framework source
/// layout `framework/templates/spec/<file>` (the ductus repo itself).
/// Returns `(repo-relative path, content)` for the first candidate on
/// disk, or empty strings when the command fills no template or none
/// exists.
fn load_template(command_name: &str, repo: &Path) -> (String, String) {
    let file = match command_name {
        "plan" => "plan.md",
        "specify" => "spec.md",
        _ => return (String::new(), String::new()),
    };
    let specs_root = crate::schema::paths::Paths::load(repo).specs_root;
    for rel in crate::primitives::template_candidates(&specs_root, file) {
        if let Some(content) = read_repo_file(repo, &rel) {
            return (rel, content);
        }
    }
    (String::new(), String::new())
}

/// Build the `askClarifyQuestion` request (reserved by the
/// clarify-command-acceleration scenario). Typed shape only, mirroring
/// `assessSpecQuality` — no legacy context dump:
///
/// - `spec-path` / `spec-content` — the spec under clarification,
///   resolved by [`resolve_spec_path`] and read repo-confined
///   (BE-INPUT-004); content empty when missing.
/// - `question` — an explicit `question` context value when the walker
///   seeds one (string, or object with `text` / optional `section`),
///   falling back to the first entry of `read-spec`'s merged
///   `open-questions` result.
fn build_ask_clarify_question_request(context: &Map<String, Value>, repo: &Path) -> Value {
    let spec_path = resolve_spec_path(context, repo);
    let spec_content = read_repo_file(repo, &spec_path).unwrap_or_default();
    let question = resolve_clarify_question(context);
    let typed = AskClarifyQuestionRequest {
        spec_path,
        spec_content,
        question,
    };
    typed_only(&typed)
}

/// Resolve the repo-relative path of the spec file the walker targets.
/// Preference order: an explicit `.md`-shaped `path` context value (the
/// analyze fixtures seed the spec file directly); `feature` joined under
/// the configured specs root; a directory-shaped `path` (the session
/// target's feature directory) joined with `spec.md`. Empty when the
/// context carries neither.
fn resolve_spec_path(context: &Map<String, Value>, repo: &Path) -> String {
    let path = context
        .get("path")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let is_markdown_file = Path::new(path)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
    if is_markdown_file {
        return path.to_string();
    }
    let feature = context
        .get("feature")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !feature.is_empty() {
        let specs_root = crate::schema::paths::Paths::load(repo).specs_root;
        return format!("{specs_root}/{feature}/spec.md");
    }
    if !path.is_empty() {
        return format!("{path}/spec.md");
    }
    String::new()
}

/// The walker-context key a clarify walk seeds with the question each
/// `askClarifyQuestion` round trip asks: one per entry of `read-spec`'s
/// `open-questions`, removed after its round trip (spec 022, scenario
/// `exec-clarify-asks-each-open-question`).
pub(crate) const CLARIFY_QUESTION_KEY: &str = "question";

/// Resolve the question an `askClarifyQuestion` round trip carries. The
/// question the walker seeded under [`CLARIFY_QUESTION_KEY`] wins — the
/// clarify walk seeds one per open question; otherwise the first entry of
/// the merged `open-questions` result, which is what a walk with no
/// question list to fan out over sends. The typed shape is always present —
/// an unseeded context yields an empty question text.
fn resolve_clarify_question(context: &Map<String, Value>) -> ClarifyQuestion {
    if let Some(question) = context.get(CLARIFY_QUESTION_KEY) {
        if let Some(text) = question.as_str() {
            return ClarifyQuestion {
                text: text.to_string(),
                section: None,
            };
        }
        if let Some(object) = question.as_object() {
            return ClarifyQuestion {
                text: object
                    .get("text")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                section: object
                    .get("section")
                    .and_then(Value::as_str)
                    .map(str::to_string),
            };
        }
    }
    let text = context
        .get("open-questions")
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .and_then(|q| q.get("text"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    ClarifyQuestion {
        text,
        section: None,
    }
}

/// The groom decision tree's route vocabulary, in walk order (see the
/// groom-command-acceleration scenario: rule promotion → missing spec →
/// spec edit [`spec`] → scenario vs. chore → discard).
const INBOX_ROUTES: [&str; 5] = ["rule", "spec", "scenario", "chore", "discard"];

/// Build the `routeInboxItem` request (reserved by the
/// groom-command-acceleration scenario). Typed shape only — no legacy
/// context dump:
///
/// - `item-text` — the `item-text` context key (seeded per inbox item by
///   the groom walk); empty when unseeded.
/// - `routes` — the fixed [`INBOX_ROUTES`] vocabulary, so hosts need not
///   parse the command prose to learn the closed decision set.
/// - `available-specs` — `NNN-slug` directories under the spec root with
///   each spec's frontmatter `status` (status drives the
///   done → in-progress reopen consent on a scenario route).
/// - `candidates` — `derive-routing-candidates`' result threaded through the
///   walker context, present on the `/ductus:specify` path and absent on the
///   groom path. Omitted from the payload when empty, so groom's request is
///   byte-unchanged and the two entry points share one routing point rather
///   than growing a second tree.
///
/// `item-text` falls back to `description` — the key `/ductus:specify` seeds —
/// so the specify path needs no second context key for the same value.
fn build_route_inbox_item_request(context: &Map<String, Value>, repo: &Path) -> Value {
    let item_text = context
        .get("item-text")
        .or_else(|| context.get("description"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let candidates = context
        .get("candidates")
        .cloned()
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default();
    let typed = RouteInboxItemRequest {
        item_text,
        routes: INBOX_ROUTES.iter().map(ToString::to_string).collect(),
        available_specs: load_available_specs(repo),
        candidates,
    };
    typed_only(&typed)
}

/// Scan the configured spec root for `NNN-slug` feature directories and
/// read each spec's frontmatter `status`. Best-effort: an unreadable
/// directory yields an empty list, an unreadable or malformed `spec.md`
/// yields an empty status (the feature's existence still matters to the
/// router). Sorted by slug for deterministic payloads.
fn load_available_specs(repo: &Path) -> Vec<RouteInboxSpec> {
    let specs_dir = crate::schema::paths::specs_dir(repo);
    crate::primitives::list_feature_dirs(&specs_dir)
        .into_iter()
        .map(|slug| {
            let status = read_spec_status(&specs_dir.join(&slug).join("spec.md"));
            RouteInboxSpec {
                feature: slug,
                status,
            }
        })
        .collect()
}

/// The fold decision's route vocabulary. Two leaves, and deliberately not
/// [`INBOX_ROUTES`]: that set answers "where in the corpus does this work
/// belong", while this one answers "what shape does content whose home is
/// already known take once it arrives". Widening the inbox set to carry a
/// second question would break the closedness its other callers rely on.
const FOLD_ROUTES: [&str; 2] = ["body-edit", "scenario"];

/// Build the `routeFold` request for one branch-scoped spec. Typed shape
/// only — no legacy context dump:
///
/// - `feature` / `spec-content` — the branch-scoped spec resolved through
///   the same `path`/`feature` preference every other builder uses, read
///   off disk.
/// - `scenarios` — the scenarios it carries, sorted by slug. Context rather
///   than subjects: a scenario is already an organizational split and
///   crosses over unchanged, so what the route decides is the body's shape.
/// - `fold-target` — read from the spec's own `folds-into` frontmatter, not
///   from a context key. The field *is* the record of where this spec
///   belongs, so taking the target from anywhere else would let a walker
///   fold a spec somewhere it never claimed to go.
/// - `target-content` / `target-status` / `target-sections` — the upstream
///   spec, without which "which section" has no answer the router could
///   give. The section list is what keeps it naming a heading the file
///   actually has.
///
/// Best-effort throughout: an unreadable file yields an empty string rather
/// than an error, matching every sibling builder. A missing target is not
/// special-cased here — `retire-feature` refuses on it at the end of the
/// fold, which is the moment that check can be made honestly.
fn build_route_fold_request(context: &Map<String, Value>, repo: &Path) -> Value {
    let spec_path = resolve_spec_path(context, repo);
    let spec_content = read_repo_file(repo, &spec_path).unwrap_or_default();
    let specs_root = crate::schema::paths::Paths::load(repo).specs_root;
    let feature = spec_path
        .strip_prefix(&format!("{specs_root}/"))
        .and_then(|rest| rest.split('/').next())
        .unwrap_or_default()
        .to_string();

    let feature_dir = repo.join(&specs_root).join(&feature);
    let scenarios = load_fold_source_scenarios(&feature_dir);

    let fold_target = folds_into(&spec_content).unwrap_or_default();
    let target_spec = repo.join(&specs_root).join(&fold_target).join("spec.md");
    // Read through the contained helper, not `fs::read_to_string`.
    // `folds-into` is hand-authored frontmatter, so it is an artifact-supplied
    // value flowing into a filesystem path — the case BE-INPUT-004 governs and
    // the case `read_repo_file`'s canonicalize-and-contain check exists for.
    // Nothing guarantees `validate-frontmatter` ran first, and the content
    // read here is shipped to the host, so a `folds-into` carrying `../`
    // would exfiltrate a `spec.md` from outside the repo. The source spec two
    // lines above already goes through this helper; this is the same read.
    let target_content = if fold_target.is_empty() {
        String::new()
    } else {
        read_repo_file(repo, &format!("{specs_root}/{fold_target}/spec.md")).unwrap_or_default()
    };
    let target_status =
        crate::primitives::frontmatter_status(&target_content, &target_spec).unwrap_or_default();
    let target_sections = section_headings(&target_content);

    let typed = RouteFoldRequest {
        feature,
        spec_content,
        scenarios,
        fold_target,
        target_content,
        target_status,
        target_sections,
        routes: FOLD_ROUTES.iter().map(ToString::to_string).collect(),
    };
    typed_only(&typed)
}

/// The scenarios a branch-scoped spec carries, sorted by slug through the
/// shared [`crate::primitives::list_scenario_files`] listing — the same set
/// `check-artifacts` derives scenario slugs from, so the fold sees exactly
/// what the artifact checks see.
fn load_fold_source_scenarios(feature_dir: &Path) -> Vec<FoldSourceScenario> {
    let scenarios_dir = feature_dir.join("scenarios");
    crate::primitives::list_scenario_files(&scenarios_dir)
        .into_iter()
        .map(|name| FoldSourceScenario {
            slug: name.trim_end_matches(".md").to_string(),
            section: crate::primitives::read_scenario_section(&scenarios_dir.join(&name))
                .unwrap_or_default(),
        })
        .collect()
}

/// A spec's declared `folds-into` target, read from its frontmatter block.
///
/// Scanned rather than deserialized so a frontmatter carrying an unrelated
/// malformed key still yields the target: this builder feeds a decision the
/// operator confirms, and refusing to route because some other field is
/// wrong would be a worse failure than routing on the field that parsed.
fn folds_into(spec_content: &str) -> Option<String> {
    use crate::primitives::spec_links::is_frontmatter_fence;

    let mut lines = spec_content.lines();
    if lines
        .next()
        .is_none_or(|first| !is_frontmatter_fence(first))
    {
        return None;
    }
    for line in lines {
        if is_frontmatter_fence(line) {
            return None;
        }
        if let Some(value) = line.strip_prefix("folds-into:") {
            let bare = value.trim().trim_matches(['"', '\'']);
            if !bare.is_empty() {
                return Some(bare.to_string());
            }
        }
    }
    None
}

/// The `##` headings of a spec body, in body order, with fenced regions
/// skipped — a `## ` inside a code block is an example, not a section, and
/// offering one as a fold destination would name a place that does not
/// exist.
fn section_headings(content: &str) -> Vec<String> {
    let mut headings = Vec::new();
    let mut fenced = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        if let Some(text) = line.strip_prefix("## ") {
            headings.push(text.trim().to_string());
        }
    }
    headings
}

/// Read a spec file's frontmatter `status`, best-effort: empty string
/// when the file is missing or the frontmatter does not parse.
fn read_spec_status(spec_path: &Path) -> String {
    let Ok(content) = std::fs::read_to_string(spec_path) else {
        return String::new();
    };
    crate::primitives::frontmatter_status(&content, spec_path).unwrap_or_default()
}

/// Build the `verifyCriteria` request for `/ductus:implement`'s completion
/// gate. Typed shape only, mirroring `askClarifyQuestion` — no legacy
/// context dump:
///
/// - `spec-path` / `spec-content` — the spec under verification,
///   resolved by [`resolve_spec_path`] and read repo-confined
///   (BE-INPUT-004); content empty when missing.
/// - `criteria` — the merged `acceptance-criteria` result of the
///   completion gate's `read-spec` step, indexed in body order (the same
///   0-based addressing `mark-criterion` consumes). Empty when the
///   context carries no criteria — the typed shape is always present.
fn build_verify_criteria_request(context: &Map<String, Value>, repo: &Path) -> Value {
    let spec_path = resolve_spec_path(context, repo);
    let spec_content = read_repo_file(repo, &spec_path).unwrap_or_default();
    let criteria = context
        .get("acceptance-criteria")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .enumerate()
                .map(|(index, criterion)| VerifyCriterion {
                    index: u32::try_from(index).unwrap_or(u32::MAX),
                    text: criterion
                        .get("text")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    checked: criterion
                        .get("checked")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                })
                .collect()
        })
        .unwrap_or_default();
    let typed = VerifyCriteriaRequest {
        spec_path,
        spec_content,
        criteria,
    };
    typed_only(&typed)
}

/// The walker-context key an `/analyze` walk seeds with the rule each
/// `assessSpecQuality` round trip assesses (spec 060). The walker loops a
/// step over the loaded rules of its tier and seeds one rule per request, as
/// a clarify loop seeds one `question` per round trip, then removes the key.
pub(crate) const ASSESSED_RULE_KEY: &str = "assessed-rule";

/// Build the `assessSpecQuality` request for one rule's Verification read
/// (`/ductus:analyze` steps 11–12). Typed fields sourced from the walker
/// context and disk:
///
/// - `spec-path` — the spec file the context's `path` names (seeded by
///   `/ductus:target` as the spec directory, echoed by `read-spec` as the
///   file).
/// - `spec-content` — the spec read off disk, repo-confined
///   (BE-INPUT-004); empty when missing or out of repo.
/// - `rule` — the rule the walker seeded under [`ASSESSED_RULE_KEY`]: its
///   ID, its Verification phrase, and the tier its Statement carries (see
///   [`load_rules`]). The walker seeds one for every request it sends, so an
///   unseeded context yields the empty rule only when this builder is
///   called outside a walk.
///
/// Unlike `writeCode`, no legacy context fields are appended: the data
/// model documents the bare typed shape for this point, and the previous
/// raw walker-context dump is exactly the behavior this builder replaces.
fn build_assess_spec_quality_request(context: &Map<String, Value>, repo: &Path) -> Value {
    let spec_path = context
        .get("path")
        .and_then(Value::as_str)
        .map(|path| super::spec_file(repo, path))
        .unwrap_or_default();
    let spec_content = read_repo_file(repo, &spec_path).unwrap_or_default();
    let rule = context
        .get(ASSESSED_RULE_KEY)
        .and_then(|rule| serde_json::from_value::<AssessSpecQualityRule>(rule.clone()).ok())
        .unwrap_or_else(|| AssessSpecQualityRule {
            id: String::new(),
            verification: String::new(),
            severity: RuleSeverity::Unspecified,
        });
    let typed = AssessSpecQualityRequest {
        spec_path,
        spec_content,
        rule,
    };
    typed_only(&typed)
}

/// Read a repo-relative file with the BE-INPUT-004 containment check.
/// `None` when `rel` is empty, the repo cannot be canonicalized, the path
/// escapes the repo, or the file is missing/unreadable.
fn read_repo_file(repo: &Path, rel: &str) -> Option<String> {
    if rel.is_empty() {
        return None;
    }
    let canon_repo = repo.canonicalize().ok()?;
    match classify_contained(&canon_repo, Path::new(rel)) {
        Contained::Inside(abs) => std::fs::read_to_string(abs).ok(),
        Contained::Missing | Contained::Outside => None,
    }
}

/// Map an `assessSpecQuality` step's rule-tier phrase to the tier of rules
/// the step asks about: `MUST-tier` → `Must`, `SHOULD-tier` → `Should`,
/// `INFO-tier` → `Info` (case-insensitive). [`RuleSeverity::Unspecified`]
/// when the prose names no tier. The tier selects which loaded rules a step
/// covers; it never assigns a tier to a rule, which carries its own
/// (spec 060, AC1).
pub(crate) fn severity_from_step_prose(prose: &str) -> RuleSeverity {
    let lower = prose.to_lowercase();
    for tier in [RuleSeverity::Must, RuleSeverity::Should, RuleSeverity::Info] {
        if lower.contains(&format!("{}-tier", tier.as_str())) {
            return tier;
        }
    }
    RuleSeverity::Unspecified
}

/// The rules an `/analyze` walk assesses, read once from its `rule-files`
/// at the walk's first `assessSpecQuality` step (spec 060).
#[derive(Default)]
pub(crate) struct LoadedRules {
    /// Rules carrying a tier and a Verification, in `rule-files` order and
    /// heading order within a file — the order requests go out in.
    pub(crate) assessable: Vec<AssessSpecQualityRule>,
    /// Rule sections with no Verification, or whose Statement carries no
    /// RFC 2119 keyword: loaded, and never asked about.
    pub(crate) unassessable: u32,
    /// Listed rule files that could not be read, whose rules cannot even be
    /// counted.
    pub(crate) unreadable_files: u32,
}

impl LoadedRules {
    /// Whether the walk loaded no rule section at all — a rule set with only
    /// unassessable rules is not empty, since each is recorded on its own.
    pub(crate) fn is_empty(&self) -> bool {
        self.assessable.is_empty() && self.unassessable == 0
    }
}

/// Read every file the context's `rule-files` lists, repo-confined
/// (BE-INPUT-004), and sort its rule sections into those a request can
/// assess and those it cannot. A rule's tier is its Statement's RFC 2119
/// keyword, never the asking step's ([`RuleSection::tier`]).
pub(crate) fn load_rules(context: &Map<String, Value>, repo: &Path) -> LoadedRules {
    let mut loaded = LoadedRules::default();
    for rel in string_array(context, "rule-files") {
        let Some(rule_file) = read_repo_file(repo, &rel) else {
            loaded.unreadable_files += 1;
            continue;
        };
        for section in parse_rule_sections(&rule_file) {
            let tier = section.tier();
            match (tier, section.verification) {
                (Some(severity), Some(verification)) => {
                    loaded.assessable.push(AssessSpecQualityRule {
                        id: section.id,
                        verification,
                        severity,
                    });
                }
                _ => loaded.unassessable += 1,
            }
        }
    }
    loaded
}

fn load_current_task(feature: &str, context: &Map<String, Value>, repo: &Path) -> WriteCodeTask {
    let task_number = context
        .get("task-number")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if feature.is_empty() {
        return WriteCodeTask {
            number: task_number,
            heading: String::new(),
            subtasks: Vec::new(),
        };
    }
    let args = ReadTasksArgs {
        feature: feature.to_string(),
    };
    let Ok(result) = read_tasks::run(&args, repo) else {
        return WriteCodeTask {
            number: task_number,
            heading: String::new(),
            subtasks: Vec::new(),
        };
    };
    // Locate by explicit task-number; fall back to the first incomplete
    // task when the context did not seed one.
    let task = if task_number.is_empty() {
        result
            .tasks
            .iter()
            .find(|t| t.subtasks.iter().any(|s| !s.checked))
            .or_else(|| result.tasks.first())
    } else {
        result.tasks.iter().find(|t| t.number == task_number)
    };
    match task {
        Some(t) => WriteCodeTask {
            number: t.number.clone(),
            heading: t.heading.clone(),
            subtasks: t.subtasks.iter().map(|s| s.text.clone()).collect(),
        },
        None => WriteCodeTask {
            number: task_number,
            heading: String::new(),
            subtasks: Vec::new(),
        },
    }
}

fn load_plan_relevant_files(
    feature: &str,
    repo: &Path,
) -> Result<Vec<PlanRelevantFile>, PayloadError> {
    if feature.is_empty() {
        return Ok(Vec::new());
    }
    // Canonicalize repo once so the containment check below operates on the
    // resolved form (e.g., macOS `/var/folders/...` → `/private/var/...`).
    // A non-canonicalizable repo path mirrors the "no plan, no files" posture.
    let Ok(canon_repo) = repo.canonicalize() else {
        return Ok(Vec::new());
    };
    let plan_path = crate::schema::paths::specs_dir(&canon_repo)
        .join(feature)
        .join("plan.md");
    let Ok(plan_content) = std::fs::read_to_string(&plan_path) else {
        return Ok(Vec::new());
    };
    let paths = crate::primitives::parse_affected_files(&plan_content);
    // Discover the git repo once, above the loop (BE-QUERY-001): the prior
    // per-path `Repository::discover` re-walked the filesystem for every
    // Affected Files entry. A project no work tree contains (discover fails:
    // no repository, a bare one, or a work tree `core.worktree` moved) yields
    // no repository, and every path is treated as not-ignored — the same
    // degradation the per-path form gave. Git reads ignore rules by the path
    // from the work tree, which is not the project root when the project sits
    // in a subdirectory of its repository: asked by the project-relative path,
    // the project's own `.gitignore` never applied, and a root pattern could
    // refuse a path it does not name (spec 059).
    let git_repo = crate::primitives::ProjectRepository::discover(&canon_repo).ok();
    let mut out = Vec::new();
    for rel in paths {
        if let Some(pattern) = secret_pattern(&rel) {
            return Err(PayloadError::SecretExfiltration {
                path: rel,
                pattern: pattern.into(),
            });
        }
        // Containment runs before the gitignore query. libgit2 answers
        // "ignored" for any path containing `..`, so asked first it refused
        // an escape under the `.gitignore` label in every git repository, and
        // the `out-of-repo` label this check exists to give never appeared
        // there. The refusal held either way; the label named the wrong cause.
        let canon_abs = match classify_contained(&canon_repo, Path::new(&rel)) {
            // Planned-new file or rename target — omit, don't error.
            // (`canonicalize` errors on missing files; existing behavior
            // preserved.) Nothing is read, so there is nothing to ask git
            // about.
            Contained::Missing => continue,
            // Path traversal: `../foo`, absolute path, or symlink whose
            // canonical target escapes the repo root. BE-INPUT-004
            // defense-in-depth — the basename-only secret-pattern check
            // above doesn't catch this class.
            Contained::Outside => {
                return Err(PayloadError::SecretExfiltration {
                    path: rel,
                    pattern: "out-of-repo".into(),
                });
            }
            Contained::Inside(abs) => abs,
        };
        // The checks below ask about the file that is read, by its name from
        // the project root, never by the plan's spelling of it. The plan may
        // write an absolute path, a `.`, `..` or `//` segment, or a symlink
        // with a harmless name: asked by the raw text, libgit2 matched no
        // anchored pattern against `proj//<absolute>` and refused `src/../x`
        // as ignored, and a link named `notes.txt` sent the `.env` it points
        // at. The entry's own name was checked for a secret pattern above.
        let Some(target) = project_relative(&canon_repo, &canon_abs) else {
            // A name that is not UTF-8 cannot be asked about, so nothing is
            // read rather than something sent unexamined.
            continue;
        };
        if let Some(pattern) = secret_pattern(&target) {
            return Err(PayloadError::SecretExfiltration {
                path: rel,
                pattern: pattern.into(),
            });
        }
        if let Some(project) = git_repo.as_ref()
            && is_gitignored(&project.repository, &project.to_git(&target))
        {
            return Err(PayloadError::SecretExfiltration {
                path: rel,
                pattern: ".gitignore".into(),
            });
        }
        let Ok(content) = std::fs::read_to_string(&canon_abs) else {
            continue;
        };
        out.push(PlanRelevantFile { path: rel, content });
    }
    Ok(out)
}

/// `abs`, a path inside `canon_repo`, named from the project root with `/`
/// between components. `None` when a component is not UTF-8, which no
/// gitignore pattern could be asked about.
fn project_relative(canon_repo: &Path, abs: &Path) -> Option<String> {
    let inside = abs.strip_prefix(canon_repo).ok()?;
    let parts: Option<Vec<&str>> = inside
        .components()
        .map(|part| part.as_os_str().to_str())
        .collect();
    Some(parts?.join("/"))
}

fn load_constitution_excerpts(command_name: &str, repo: &Path) -> ConstitutionExcerptScan {
    let Some(command_path) = locate_command_file(command_name, repo) else {
        return ConstitutionExcerptScan::unexaminable(format!(
            "command-file-missing: {command_name}"
        ));
    };
    let Ok(command_content) = std::fs::read_to_string(&command_path) else {
        // Repo-relative: the label rides in an outbound payload, and an
        // absolute path would carry the contributor's home directory with it.
        let shown = command_path.strip_prefix(repo).unwrap_or(&command_path);
        return ConstitutionExcerptScan::unexaminable(format!(
            "command-file-unreadable: {}",
            shown.display()
        ));
    };
    let anchors = parse_command_references(&command_content);
    if anchors.is_empty() {
        // The one honest empty case: the command declares no `Reference:`
        // line, so there is nothing to load and nothing went unexamined.
        return ConstitutionExcerptScan::default();
    }
    let Some(constitution_rel) = paths::constitution_path(repo) else {
        return ConstitutionExcerptScan::unexaminable(format!(
            "constitution-unreadable: {}",
            paths::CONSTITUTION_CHAIN.join(" or ")
        ));
    };
    let Ok(constitution) = std::fs::read_to_string(repo.join(constitution_rel)) else {
        return ConstitutionExcerptScan::unexaminable(format!(
            "constitution-unreadable: {constitution_rel}"
        ));
    };
    let mut scan = ConstitutionExcerptScan::default();
    for anchor in anchors {
        match extract_anchor_body(&constitution, &anchor) {
            Some(body) => scan.excerpts.push(body),
            // Dropping the anchor silently would leave a populated array
            // that reads as complete while a section the command declared
            // relevant is missing from it.
            None => scan
                .unexaminable
                .push(format!("anchor-unresolved: §{anchor}")),
        }
    }
    scan
}

fn locate_command_file(command_name: &str, repo: &Path) -> Option<PathBuf> {
    let host = Host::load(repo);
    let mut rels = vec![format!("framework/commands/{command_name}.md")];
    // Installed command file — `commands/` (claude-style), singular
    // `command/` (opencode), or flat `prompts/{project}-{name}.md` (pi);
    // see `Host::command_file_candidates`.
    rels.extend(host.command_file_candidates(command_name));
    rels.push(format!("framework/bootstrap/{command_name}.md"));
    for rel in rels {
        let candidate = repo.join(rel);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

/// The outcome of a constitution-excerpt load: what was read, and what could
/// not be. Two fields rather than one `Vec<String>` because five distinct
/// states used to collapse into the same empty vec — an absent command file,
/// an unreadable one, a command with no `Reference:` line, an unreadable
/// constitution, and an anchor that resolves to nothing — and a host reading
/// the resulting `writeCode` payload could not tell "no constitutional
/// context applies" from "the context could not be loaded" (`QUAL-CLAIM-001`).
#[derive(Debug, Default, PartialEq)]
struct ConstitutionExcerptScan {
    /// Resolved section bodies, in the order the command's `Reference:` line
    /// named their anchors.
    excerpts: Vec<String>,
    /// One entry per cause the load could not examine; empty when every
    /// declared anchor resolved.
    unexaminable: Vec<String>,
}

impl ConstitutionExcerptScan {
    /// A scan that read nothing, for the reason given.
    fn unexaminable(reason: String) -> Self {
        Self {
            excerpts: Vec::new(),
            unexaminable: vec![reason],
        }
    }
}

/// Extract anchor names from a command file's `Reference: §a, §b, §c` line
/// under the `Scope Boundaries` section. The line may carry trailing
/// parenthetical prose; only `§<name>` tokens are returned.
///
/// # Panics
///
/// Panics only if the hard-coded anchor regex fails to compile — which
/// would indicate a corrupt `regex` crate, not user input.
#[must_use]
pub fn parse_command_references(command_content: &str) -> Vec<String> {
    static R: OnceLock<Regex> = OnceLock::new();
    let anchor_re = R.get_or_init(|| {
        Regex::new(r"§([A-Za-z][A-Za-z0-9_-]*)").expect("hard-coded regex compiles")
    });
    for line in command_content.lines() {
        let trimmed = line.trim_start_matches(|c: char| c == '-' || c.is_whitespace());
        if !trimmed.starts_with("Reference:") {
            continue;
        }
        let mut seen = HashSet::new();
        // Order-preserving dedup: `Vec::dedup` drops only *consecutive*
        // repeats, so a `Reference:` line naming one anchor twice with
        // another between them emitted its excerpt twice. First-occurrence
        // order is kept so the cache-anchored prefix stays byte-stable for
        // inputs that were already duplicate-free.
        let anchors: Vec<String> = anchor_re
            .captures_iter(trimmed)
            .map(|c| c[1].to_string())
            .filter(|anchor| seen.insert(anchor.clone()))
            .collect();
        return anchors;
    }
    Vec::new()
}

/// Return the body of an anchored section. The body is the content between
/// `<!-- §<anchor> -->` and the next `<!-- §<other> -->` marker (or EOF),
/// with the marker line itself excluded. Returns `None` when the anchor is
/// not present in `content`.
///
/// # Panics
///
/// Panics only if the hard-coded next-marker regex fails to compile —
/// which would indicate a corrupt `regex` crate, not user input.
#[must_use]
pub fn extract_anchor_body(content: &str, anchor: &str) -> Option<String> {
    static NEXT: OnceLock<Regex> = OnceLock::new();
    let marker = format!("<!-- §{anchor} -->");
    let start = content.find(&marker)?;
    // Skip past the marker line itself (find end of that line).
    let after_marker_line = {
        let rel = content[start..].find('\n')?;
        start + rel + 1
    };
    let rest = &content[after_marker_line..];
    // Find the next anchor marker (any name) and cut there.
    let next_re = NEXT.get_or_init(|| {
        Regex::new(r"<!--\s*§[A-Za-z][A-Za-z0-9_-]*\s*-->").expect("hard-coded regex compiles")
    });
    let end = match next_re.find(rest) {
        Some(m) => m.start(),
        None => rest.len(),
    };
    Some(rest[..end].trim_end_matches('\n').to_string())
}

/// Read a section body from a spec or plan file. The running command
/// selects the file explicitly (extension-request-hygiene — a
/// `/ductus:specify` re-run on a feature that has since gained a plan must
/// not read plan.md's section):
///
/// - `plan` (`/ductus:plan`) → `specs/{feature}/plan.md`
/// - `specify` (`/ductus:specify`) → `specs/{feature}/spec.md`
///
/// Any other command yields `None`: only `/ductus:plan` and `/ductus:specify`
/// carry the `writeSpecBody` marker. Returns `None` too when the file does
/// not exist or the section is absent or empty. Whitespace-only bodies count
/// as empty.
fn read_existing_section(
    section: &str,
    feature: Option<&str>,
    path_hint: Option<&str>,
    repo: &Path,
    command_name: &str,
) -> Option<String> {
    let feature_dir = match feature {
        Some(f) if !f.is_empty() => crate::schema::paths::specs_dir(repo).join(f),
        _ => repo.join(path_hint.unwrap_or_default()),
    };
    let filenames: &[&str] = match command_name {
        "plan" => &["plan.md"],
        "specify" => &["spec.md"],
        _ => return None,
    };
    // BE-INPUT-004: `feature` and `path` arrive in the walker context, so the
    // file is confined to the project before it is read, as every other reader
    // here is — a `../` feature or an absolute path never reaches the payload.
    let canon_repo = repo.canonicalize().ok()?;
    for filename in filenames {
        let candidate = feature_dir.join(filename);
        let Contained::Inside(abs) = classify_contained(&canon_repo, &candidate) else {
            continue;
        };
        let Ok(content) = std::fs::read_to_string(&abs) else {
            continue;
        };
        if let Some(body) = extract_section_body(&content, section) {
            let trimmed = body.trim();
            if trimmed.is_empty() {
                return None;
            }
            return Some(trimmed.to_string());
        }
    }
    None
}

/// Pull a level-2 (`## …`) section body from a markdown file. The body runs
/// from after the heading line to the next level-1 or level-2 heading (or
/// EOF). Fenced code blocks inside the body do not terminate it.
fn extract_section_body(content: &str, section: &str) -> Option<String> {
    let mut in_fence = false;
    let mut collected: Option<Vec<&str>> = None;
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            if let Some(acc) = collected.as_mut() {
                acc.push(line);
            }
            continue;
        }
        if !in_fence {
            if let Some(rest) = trimmed.strip_prefix("## ") {
                if collected.is_some() {
                    break;
                }
                if rest.trim().eq_ignore_ascii_case(section) {
                    collected = Some(Vec::new());
                    continue;
                }
            } else if trimmed.starts_with("# ")
                && !trimmed.starts_with("## ")
                && collected.is_some()
            {
                break;
            }
        }
        if let Some(acc) = collected.as_mut() {
            acc.push(line);
        }
    }
    collected.map(|lines| lines.join("\n"))
}

/// Extract the section name from a step prose like
/// `Fill the Technical Decisions section of the plan.` Returns `None` when
/// no such phrase is present. The name charset is restricted to plain
/// heading words (`[A-Za-z0-9 _/-]`) so a whole-body fill step whose prose
/// happens to mention "… section" much later — `/ductus:specify`'s "Fill the
/// new spec body following §spec-requirements: a Motivation section …" —
/// does not smuggle intervening punctuation into the typed `section` field.
fn extract_section_name(prose: &str) -> Option<String> {
    static R: OnceLock<Regex> = OnceLock::new();
    let re = R.get_or_init(|| {
        Regex::new(r"(?i)Fill\s+the\s+([A-Za-z][A-Za-z0-9 _/-]*?)\s+section")
            .expect("hard-coded regex compiles")
    });
    re.captures(prose).map(|c| c[1].trim().to_string())
}

/// Match a path against the v1 secret-exfiltration patterns. Returns the
/// matched pattern label when blocked; `None` otherwise. Matching is
/// ASCII-case-insensitive on the basename so a plan entry of `.ENV` or
/// `Credentials.json` cannot bypass the guard on a case-insensitive
/// filesystem (macOS APFS by default). Patterns:
///
/// - `.env` and `.env.*` (e.g., `.env.production`)
/// - `*-secrets.*` (e.g., `db-secrets.yaml`)
/// - `credentials*` (any extension)
fn secret_pattern(path: &str) -> Option<&'static str> {
    let basename = Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if basename == ".env" {
        return Some(".env");
    }
    if basename.starts_with(".env.") {
        return Some(".env.*");
    }
    if basename.starts_with("credentials") {
        return Some("credentials*");
    }
    // *-secrets.* — split into stem and extension; the stem must end with
    // `-secrets` and there must be at least one extension.
    if let Some((stem, _ext)) = basename.rsplit_once('.')
        && stem.ends_with("-secrets")
    {
        return Some("*-secrets.*");
    }
    None
}

/// Ask libgit2 whether `path`, named from the work tree, is gitignored from
/// `repository`'s perspective. Returns `false` when libgit2 errors — the
/// secret-pattern check above is the floor; gitignore is an opt-in second
/// layer. The caller hoists `ProjectRepository::discover` above the Affected
/// Files loop (BE-QUERY-001) and only calls this when discovery succeeded,
/// so a project no work tree contains — no repository, a bare one, or a work
/// tree `core.worktree` moved elsewhere — skips the query entirely and every
/// path stays not-ignored.
fn is_gitignored(repository: &git2::Repository, path: &str) -> bool {
    repository
        .status_should_ignore(Path::new(path))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parse_command_references_extracts_anchor_names() {
        let cmd = "## Scope Boundaries\n\n\
                   - The runtime write boundary is derived in step 2.\n\
                   - Do NOT read source code speculatively.\n\
                   - Reference: §implement-phase, §pipeline-boundaries, §text-first-artifacts, plus extras.\n";
        let anchors = parse_command_references(cmd);
        assert_eq!(
            anchors,
            vec![
                "implement-phase".to_string(),
                "pipeline-boundaries".to_string(),
                "text-first-artifacts".to_string()
            ]
        );
    }

    #[test]
    fn parse_command_references_dedups_non_adjacent_repeats() {
        let cmd = "## Scope Boundaries\n\n\
                   - Reference: \u{a7}spec-phase, \u{a7}text-first-artifacts, \u{a7}spec-phase.\n";
        let anchors = parse_command_references(cmd);
        assert_eq!(
            anchors,
            vec!["spec-phase".to_string(), "text-first-artifacts".to_string()],
            "a repeated anchor must be emitted once, in first-occurrence order"
        );
    }

    #[test]
    fn parse_command_references_empty_when_absent() {
        let cmd = "## Scope Boundaries\n\nNo reference line here.\n";
        assert!(parse_command_references(cmd).is_empty());
    }

    #[test]
    fn extract_anchor_body_returns_section_between_markers() {
        let constitution = "<!-- §alpha -->\n\
                            ### Alpha\n\nBody of alpha.\n\n\
                            <!-- §beta -->\n\
                            ### Beta\n\nBody of beta.\n";
        let alpha = extract_anchor_body(constitution, "alpha").unwrap();
        assert!(alpha.contains("Body of alpha."));
        assert!(!alpha.contains("Body of beta."));
        assert!(!alpha.contains("<!-- §beta -->"));
    }

    #[test]
    fn extract_anchor_body_reads_until_eof_for_last_marker() {
        let content = "<!-- §only -->\n\nfinal body content\n";
        let body = extract_anchor_body(content, "only").unwrap();
        assert_eq!(body.trim(), "final body content");
    }

    #[test]
    fn extract_anchor_body_returns_none_when_anchor_missing() {
        let content = "<!-- §other -->\nbody\n";
        assert!(extract_anchor_body(content, "missing").is_none());
    }

    #[test]
    fn secret_pattern_matches_dotenv_family() {
        assert_eq!(secret_pattern(".env"), Some(".env"));
        assert_eq!(secret_pattern(".env.production"), Some(".env.*"));
        assert_eq!(secret_pattern("path/to/.env.local"), Some(".env.*"));
    }

    #[test]
    fn secret_pattern_matches_secrets_files() {
        assert_eq!(secret_pattern("db-secrets.yaml"), Some("*-secrets.*"));
        assert_eq!(
            secret_pattern("path/to/api-secrets.json"),
            Some("*-secrets.*")
        );
    }

    #[test]
    fn secret_pattern_matches_credentials_files() {
        assert_eq!(secret_pattern("credentials"), Some("credentials*"));
        assert_eq!(secret_pattern("credentials.json"), Some("credentials*"));
        assert_eq!(
            secret_pattern("path/to/credentials.gpg"),
            Some("credentials*")
        );
    }

    #[test]
    fn secret_pattern_passes_through_normal_files() {
        assert_eq!(secret_pattern("runtime/src/main.rs"), None);
        assert_eq!(secret_pattern("README.md"), None);
        assert_eq!(secret_pattern("framework/constitution.md"), None);
    }

    /// Write a command file declaring `anchors` on its `Reference:` line.
    fn write_command_file(repo: &std::path::Path, name: &str, anchors: &str) {
        let dir = repo.join("framework/commands");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(format!("{name}.md")),
            format!("## Scope Boundaries\n\n- Reference: {anchors}.\n"),
        )
        .unwrap();
    }

    fn write_constitution(repo: &std::path::Path, body: &str) {
        let dir = repo.join("framework");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("constitution.md"), body).unwrap();
    }

    #[test]
    fn excerpt_scan_reports_an_anchor_that_resolves_to_nothing() {
        let tmp = tempdir().unwrap();
        write_command_file(tmp.path(), "implement", "§alpha, §ghost");
        write_constitution(tmp.path(), "<!-- §alpha -->\n\nBody of alpha.\n");

        let scan = load_constitution_excerpts("implement", tmp.path());
        assert_eq!(
            scan.excerpts.len(),
            1,
            "the resolved anchor is still loaded"
        );
        assert!(scan.excerpts[0].contains("Body of alpha."));
        assert_eq!(
            scan.unexaminable,
            vec!["anchor-unresolved: §ghost".to_string()],
            "a dropped anchor must be named, not silently absent"
        );
    }

    #[test]
    fn excerpt_scan_reports_an_unreadable_constitution() {
        let tmp = tempdir().unwrap();
        write_command_file(tmp.path(), "implement", "§alpha");
        // No constitution written at all.

        let scan = load_constitution_excerpts("implement", tmp.path());
        assert!(scan.excerpts.is_empty());
        assert_eq!(scan.unexaminable.len(), 1);
        assert_eq!(
            scan.unexaminable,
            vec![
                "constitution-unreadable: .ductus/constitution.md or framework/constitution.md"
                    .to_string()
            ],
            "the label names every path tried, repo-relative — it rides in an outbound payload"
        );
    }

    /// An adopter's constitution is the one its bootstrap installs under
    /// `.ductus/`; a project holding only that copy loads its excerpts rather
    /// than reporting the constitution unreadable.
    #[test]
    fn excerpt_scan_reads_an_adopter_constitution() {
        let tmp = tempdir().unwrap();
        write_command_file(tmp.path(), "implement", "§alpha");
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/constitution.md"),
            "<!-- §alpha -->\n\nBody of alpha.\n",
        )
        .unwrap();

        let scan = load_constitution_excerpts("implement", tmp.path());
        assert!(scan.unexaminable.is_empty(), "{:?}", scan.unexaminable);
        assert_eq!(scan.excerpts.len(), 1);
        assert!(scan.excerpts[0].contains("Body of alpha."));
    }

    #[test]
    fn excerpt_scan_reports_a_missing_command_file() {
        let tmp = tempdir().unwrap();
        let scan = load_constitution_excerpts("nonesuch", tmp.path());
        assert!(scan.excerpts.is_empty());
        assert_eq!(
            scan.unexaminable,
            vec!["command-file-missing: nonesuch".to_string()]
        );
    }

    #[test]
    fn excerpt_scan_is_empty_both_ways_when_no_reference_line_is_declared() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("framework/commands");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("implement.md"),
            "## Scope Boundaries\n\nNothing.\n",
        )
        .unwrap();

        let scan = load_constitution_excerpts("implement", tmp.path());
        assert_eq!(
            scan,
            ConstitutionExcerptScan::default(),
            "a command with no Reference: line is the one honest empty case"
        );
    }

    #[test]
    fn clean_write_code_request_omits_the_unexaminable_field() {
        let request = WriteCodeRequest {
            constitution_excerpts: vec!["body".to_string()],
            constitution_excerpts_unexaminable: vec![],
            plan_relevant_files: vec![],
            write_boundary: vec!["runtime/**".to_string()],
            task: WriteCodeTask {
                number: "1".to_string(),
                heading: "A task".to_string(),
                subtasks: vec![],
            },
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(
            !json.contains("constitution-excerpts-unexaminable"),
            "an empty field must stay off the wire so clean payloads are \
             byte-identical to pre-field ones: {json}"
        );

        let flagged = WriteCodeRequest {
            constitution_excerpts_unexaminable: vec!["anchor-unresolved: §ghost".to_string()],
            ..request
        };
        assert!(
            serde_json::to_string(&flagged)
                .unwrap()
                .contains("constitution-excerpts-unexaminable")
        );
    }

    #[test]
    fn load_plan_relevant_files_omits_absent_paths_without_error() {
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `existing.txt` | Edit |\n\
             | `planned-but-absent.txt` | Create |\n",
        )
        .unwrap();
        fs::write(tmp.path().join("existing.txt"), "hello").unwrap();

        let files = load_plan_relevant_files("123-foo", tmp.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "existing.txt");
        assert_eq!(files[0].content, "hello");
    }

    #[test]
    fn load_plan_relevant_files_rejects_secret_pattern() {
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `.env.production` | Edit |\n",
        )
        .unwrap();

        let err = load_plan_relevant_files("123-foo", tmp.path()).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, ".env.production");
                assert_eq!(pattern, ".env.*");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    #[test]
    fn load_plan_relevant_files_rejects_gitignored_path() {
        let tmp = tempdir().unwrap();
        // Init a git repo so libgit2 can answer the gitignore query.
        let repo = git2::Repository::init(tmp.path()).unwrap();
        let _ = repo; // dropped — `discover` reopens later
        let feature_dir = tmp.path().join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(tmp.path().join(".gitignore"), "secret-config.toml\n").unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `secret-config.toml` | Edit |\n",
        )
        .unwrap();
        fs::write(tmp.path().join("secret-config.toml"), "key=value").unwrap();

        let err = load_plan_relevant_files("123-foo", tmp.path()).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, "secret-config.toml");
                assert_eq!(pattern, ".gitignore");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    /// A project at `root/proj`, in a subdirectory of its repository, whose
    /// feature `123-foo` plans `entry` as its one affected file. Returns the
    /// project root.
    fn subdirectory_plan(root: &Path, entry: &str) -> PathBuf {
        let (_, project) = crate::primitives::git_fixture::subdirectory_project(root);
        let feature_dir = project.join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            format!(
                "## Affected Files\n\n\
                 | File | Action |\n| --- | --- |\n\
                 | `{entry}` | Edit |\n"
            ),
        )
        .unwrap();
        project
    }

    /// A project in a subdirectory of its repository plans a file its own
    /// `.gitignore` ignores through an anchored pattern, under a name no
    /// secret pattern matches. Git reads ignore rules by the path from the
    /// work tree, so asked by the project-relative path the project's
    /// `.gitignore` never applied and the file went into the payload
    /// (spec 059).
    #[test]
    fn a_subdirectory_projects_own_gitignore_refuses_its_file() {
        let tmp = tempdir().unwrap();
        let project = subdirectory_plan(tmp.path(), "config/local.toml");
        fs::write(project.join(".gitignore"), "/config/local.toml\n").unwrap();
        fs::create_dir_all(project.join("config")).unwrap();
        fs::write(project.join("config/local.toml"), "token=value").unwrap();
        assert_eq!(secret_pattern("config/local.toml"), None);

        let err = load_plan_relevant_files("123-foo", &project).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, "config/local.toml");
                assert_eq!(pattern, ".gitignore");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    #[test]
    fn a_root_pattern_does_not_refuse_a_subdirectory_file_it_does_not_name() {
        // `/config/` ignores the work tree's own `config/`, not the project's.
        // Read from the work tree's root, the project-relative path matched
        // it and a file git tracks was refused.
        let tmp = tempdir().unwrap();
        let project = subdirectory_plan(tmp.path(), "config/app.toml");
        fs::write(tmp.path().join(".gitignore"), "/config/\n").unwrap();
        fs::create_dir_all(project.join("config")).unwrap();
        fs::write(project.join("config/app.toml"), "name=app").unwrap();

        let files = load_plan_relevant_files("123-foo", &project).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "config/app.toml");
    }

    /// Converting the path for git must not open a way out of the project.
    /// `../outside.txt` from a project at `proj/` names a file the repository
    /// holds but the project does not, and the containment check refuses it
    /// as `out-of-repo`, the label its cause warrants. The check runs before
    /// the gitignore query, which libgit2 answers "ignored" for any path
    /// containing `..` and would otherwise label the escape `.gitignore`.
    #[test]
    fn a_subdirectory_plan_path_escaping_the_project_is_still_refused() {
        let tmp = tempdir().unwrap();
        let project = subdirectory_plan(tmp.path(), "../outside.txt");
        fs::write(tmp.path().join("outside.txt"), "leaked").unwrap();

        let err = load_plan_relevant_files("123-foo", &project).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, "../outside.txt");
                assert_eq!(pattern, "out-of-repo");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    /// BE-INPUT-004: a context `feature` that climbs out of the spec root, or
    /// an absolute `path` hint, is not read into the `writeSpecBody` payload.
    #[test]
    fn read_existing_section_reads_nothing_outside_the_project() {
        let outer = tempdir().unwrap();
        let repo = outer.path().join("repo");
        fs::create_dir_all(repo.join("specs")).unwrap();
        fs::write(
            outer.path().join("spec.md"),
            "# Outside\n\n## Motivation\n\nOutside text.\n",
        )
        .unwrap();
        let climbing = read_existing_section("Motivation", Some("../.."), None, &repo, "specify");
        assert_eq!(climbing, None);
        let absolute = outer.path().canonicalize().unwrap();
        let hinted = read_existing_section(
            "Motivation",
            None,
            Some(absolute.to_str().unwrap()),
            &repo,
            "specify",
        );
        assert_eq!(hinted, None);
    }

    /// The refusal a subdirectory project's plan entry produced, by pattern.
    fn refusal_pattern(project: &Path) -> Option<String> {
        match load_plan_relevant_files("123-foo", project) {
            Err(PayloadError::SecretExfiltration { pattern, .. }) => Some(pattern),
            Err(other @ PayloadError::UnknownExtension { .. }) => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
            Ok(_) => None,
        }
    }

    /// An absolute spelling of an ignored file inside a subdirectory project.
    /// Converted for git by prefixing it, it became `proj//<absolute>`, which
    /// no anchored pattern matches, so the file went into the payload.
    #[test]
    fn an_absolute_spelling_of_an_ignored_file_is_refused() {
        let tmp = tempdir().unwrap();
        // The fixture places the project at `proj/`.
        let absolute = tmp
            .path()
            .canonicalize()
            .unwrap()
            .join("proj/config/local.toml")
            .to_string_lossy()
            .into_owned();
        let project = subdirectory_plan(tmp.path(), &absolute);
        fs::write(project.join(".gitignore"), "/config/local.toml\n").unwrap();
        fs::create_dir_all(project.join("config")).unwrap();
        fs::write(project.join("config/local.toml"), "token=value").unwrap();

        assert_eq!(refusal_pattern(&project).as_deref(), Some(".gitignore"));
    }

    /// A `..` spelling of a file git tracks is asked about by the file's own
    /// name. Asked by the raw text, libgit2 answered "ignored" for the `..`
    /// and refused a file nothing ignores.
    #[test]
    fn a_dot_dot_spelling_of_a_tracked_file_is_not_refused() {
        let tmp = tempdir().unwrap();
        let project = subdirectory_plan(tmp.path(), "src/../app.toml");
        fs::create_dir_all(project.join("src")).unwrap();
        fs::write(project.join("app.toml"), "name=app").unwrap();

        assert_eq!(refusal_pattern(&project), None);
        let files = load_plan_relevant_files("123-foo", &project).unwrap();
        assert_eq!(files.len(), 1);
    }

    /// A symlink with a harmless name pointing at an ignored file: the file
    /// read is the target, so the target's name is what git is asked about.
    #[cfg(unix)]
    #[test]
    fn a_harmless_named_link_to_an_ignored_file_is_refused() {
        let tmp = tempdir().unwrap();
        let project = subdirectory_plan(tmp.path(), "notes.txt");
        fs::write(project.join(".gitignore"), "/config/local.toml\n").unwrap();
        fs::create_dir_all(project.join("config")).unwrap();
        fs::write(project.join("config/local.toml"), "token=value").unwrap();
        std::os::unix::fs::symlink("config/local.toml", project.join("notes.txt")).unwrap();

        assert_eq!(refusal_pattern(&project).as_deref(), Some(".gitignore"));
    }

    /// A symlink with a harmless name pointing at a secret-named file: the
    /// basename check reads the target's name as well as the entry's.
    #[cfg(unix)]
    #[test]
    fn a_harmless_named_link_to_a_secret_file_is_refused() {
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `readme.txt` | Edit |\n",
        )
        .unwrap();
        fs::write(tmp.path().join(".env"), "TOKEN=value").unwrap();
        std::os::unix::fs::symlink(".env", tmp.path().join("readme.txt")).unwrap();

        assert_eq!(refusal_pattern(tmp.path()).as_deref(), Some(".env"));
    }

    /// The same escape from a project at the root of its git repository,
    /// where libgit2's "ignored" answer for a `..` path labelled it
    /// `.gitignore` before spec 059's task 16 put containment first. The
    /// existing escape tests run outside any repository, where there is no
    /// gitignore query to answer first, so they could not see it.
    #[test]
    fn a_root_project_in_a_git_repository_refuses_an_escape_as_out_of_repo() {
        let outer = tempdir().unwrap();
        let repo = outer.path().join("repo");
        fs::create_dir_all(&repo).unwrap();
        git2::Repository::init(&repo).unwrap();
        fs::write(outer.path().join("outside.txt"), "leaked").unwrap();
        let feature_dir = repo.join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `../outside.txt` | Edit |\n",
        )
        .unwrap();

        let err = load_plan_relevant_files("123-foo", &repo).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, "../outside.txt");
                assert_eq!(pattern, "out-of-repo");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    #[test]
    fn secret_pattern_is_case_insensitive() {
        // BE-INPUT-004 — case-fold bypass on case-insensitive filesystems.
        // `.ENV` on macOS APFS resolves to `.env` on disk; the basename
        // check must match regardless of case.
        assert_eq!(secret_pattern(".ENV"), Some(".env"));
        assert_eq!(secret_pattern(".Env.Production"), Some(".env.*"));
        assert_eq!(secret_pattern("Credentials.JSON"), Some("credentials*"));
        assert_eq!(secret_pattern("DB-Secrets.YAML"), Some("*-secrets.*"));
        assert_eq!(secret_pattern("README.md"), None);
    }

    #[test]
    fn load_plan_relevant_files_rejects_relative_escape() {
        // BE-INPUT-004 — a plan entry of `../outside.txt` resolves outside
        // the repo. Basename `outside.txt` does not match any secret pattern,
        // but the canonical-containment check catches it.
        let outer = tempdir().unwrap();
        let repo = outer.path().join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        std::fs::write(outer.path().join("outside.txt"), "leaked").unwrap();
        let feature_dir = repo.join("specs/123-foo");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `../outside.txt` | Edit |\n",
        )
        .unwrap();

        let err = load_plan_relevant_files("123-foo", &repo).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, "../outside.txt");
                assert_eq!(pattern, "out-of-repo");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    #[test]
    fn load_plan_relevant_files_rejects_absolute_escape() {
        // BE-INPUT-004 — `Path::join` lets an absolute joinee replace the
        // base, so `/etc/hosts` (or a sibling tempdir absolute path) would
        // be read without the containment check. Use a sibling tempdir
        // instead of /etc/hosts so the test is hermetic.
        let outer = tempdir().unwrap();
        let repo = outer.path().join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        let sibling = outer.path().join("sibling.txt");
        std::fs::write(&sibling, "leaked").unwrap();
        // Resolve to the canonical absolute form so the test is robust
        // against tempdir symlinks (macOS `/var` → `/private/var`).
        let abs_str = sibling
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let feature_dir = repo.join("specs/123-foo");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::write(
            feature_dir.join("plan.md"),
            format!(
                "## Affected Files\n\n\
                 | File | Action |\n| --- | --- |\n\
                 | `{abs_str}` | Edit |\n"
            ),
        )
        .unwrap();

        let err = load_plan_relevant_files("123-foo", &repo).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, abs_str);
                assert_eq!(pattern, "out-of-repo");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    #[test]
    fn load_plan_relevant_files_admits_in_repo_relative_path() {
        // Happy path — a normal in-repo relative entry resolves under
        // canon_repo and is bundled into the payload as today.
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src/lib.rs"), "fn main() {}").unwrap();
        std::fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `src/lib.rs` | Edit |\n",
        )
        .unwrap();

        let files = load_plan_relevant_files("123-foo", tmp.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "src/lib.rs");
        assert_eq!(files[0].content, "fn main() {}");
    }

    #[test]
    fn load_plan_relevant_files_skips_case_fold_bypass() {
        // BE-INPUT-004 — `.ENV` lowercased to `.env` matches the pattern
        // and is rejected before the containment check runs. Important on
        // case-insensitive filesystems where the on-disk file is `.env`
        // but the plan author can spell it `.ENV` (or `.Env`) to bypass.
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::write(
            feature_dir.join("plan.md"),
            "## Affected Files\n\n\
             | File | Action |\n| --- | --- |\n\
             | `.ENV` | Edit |\n",
        )
        .unwrap();

        let err = load_plan_relevant_files("123-foo", tmp.path()).unwrap_err();
        match err {
            PayloadError::SecretExfiltration { path, pattern } => {
                assert_eq!(path, ".ENV");
                // Pattern label is the canonical lowercase form regardless
                // of which case the author spelled.
                assert_eq!(pattern, ".env");
            }
            other @ PayloadError::UnknownExtension { .. } => {
                panic!("expected SecretExfiltration, got {other:?}")
            }
        }
    }

    #[test]
    fn extract_section_name_pulls_section_from_step_prose() {
        let prose = "Fill the Technical Decisions section of the plan. The host returns the markdown body for the section; the walker forwards the response through the context.";
        assert_eq!(
            extract_section_name(prose).as_deref(),
            Some("Technical Decisions")
        );
    }

    #[test]
    fn extract_section_name_returns_none_when_phrase_absent() {
        let prose = "Do the thing. No section here.";
        assert!(extract_section_name(prose).is_none());
    }

    #[test]
    fn extract_section_body_pulls_body_until_next_h2() {
        let plan = "# Title\n\n\
                    ## Motivation\n\nWhy.\n\n\
                    ## Technical Decisions\n\nFirst decision.\n\nSecond decision.\n\n\
                    ## Affected Files\n\n| File | Action |\n";
        let body = extract_section_body(plan, "Technical Decisions").unwrap();
        assert!(body.contains("First decision."));
        assert!(body.contains("Second decision."));
        assert!(!body.contains("## Affected Files"));
    }

    #[test]
    fn build_write_spec_body_request_inlines_existing_section_content() {
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            "# Plan\n\n\
             ## Technical Decisions\n\n\
             Use the standard library.\n\n\
             ## Affected Files\n\n| File | Action |\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("123-foo".into()));
        let prose = "Fill the Technical Decisions section of the plan.";
        let value = build_write_spec_body_request(&ctx, tmp.path(), "plan", prose);
        let obj = value.as_object().unwrap();
        assert_eq!(obj["section"], "Technical Decisions");
        assert_eq!(
            obj["existing-content"].as_str().unwrap(),
            "Use the standard library."
        );
        // The typed prefix leads in declaration order even when the
        // template fields could not be derived (no template on disk).
        let prefix: Vec<&str> = obj.keys().map(String::as_str).take(4).collect();
        assert_eq!(
            prefix,
            vec![
                "template-path",
                "template-content",
                "section",
                "feature-description"
            ]
        );
        assert_eq!(obj["template-path"], "");
    }

    #[test]
    fn build_write_spec_body_request_stays_typed_when_no_existing_content() {
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("999-empty".into()));
        let prose = "Fill the Motivation section of the spec.";
        let value = build_write_spec_body_request(&ctx, tmp.path(), "specify", prose);
        let obj = value.as_object().unwrap();
        // No existing content found → no `existing-content` key added.
        assert!(!obj.contains_key("existing-content"));
        // Typed fields are still emitted (empty, not dropped).
        assert_eq!(obj["section"], "Motivation");
        assert_eq!(obj["feature-description"], "");
        // Context fields are preserved after the typed prefix.
        assert_eq!(obj["feature"], "999-empty");
    }

    #[test]
    fn build_write_spec_body_request_selects_file_by_running_command() {
        // A /ductus:specify re-run on a feature that has since gained a plan
        // must read spec.md's section, not plan.md's (and vice versa).
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("plan.md"),
            "# Plan\n\n## Motivation\n\nPlan motivation.\n",
        )
        .unwrap();
        fs::write(
            feature_dir.join("spec.md"),
            "# Spec\n\n## Motivation\n\nSpec motivation.\n",
        )
        .unwrap();
        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("123-foo".into()));
        let prose = "Fill the Motivation section of the file.";
        let plan = build_write_spec_body_request(&ctx, tmp.path(), "plan", prose);
        assert_eq!(plan["existing-content"], "Plan motivation.");
        let spec = build_write_spec_body_request(&ctx, tmp.path(), "specify", prose);
        assert_eq!(spec["existing-content"], "Spec motivation.");
    }

    #[test]
    fn build_write_spec_body_request_populates_template_and_description() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs/templates")).unwrap();
        fs::write(
            tmp.path().join("specs/templates/plan.md"),
            "# Plan template\n",
        )
        .unwrap();
        let mut ctx = Map::new();
        ctx.insert(
            "feature-description".into(),
            Value::String("webhook delivery".into()),
        );
        let prose = "Fill the Technical Decisions section of the plan.";
        let value = build_write_spec_body_request(&ctx, tmp.path(), "plan", prose);
        assert_eq!(value["template-path"], "specs/templates/plan.md");
        assert_eq!(value["template-content"], "# Plan template\n");
        assert_eq!(value["feature-description"], "webhook delivery");
        assert_eq!(value["section"], "Technical Decisions");
    }

    #[test]
    fn load_template_falls_back_to_framework_source_layout() {
        // No installed {specs-root}/templates/ → the framework source
        // layout (the ductus repo itself) is the second candidate.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("framework/templates/spec")).unwrap();
        fs::write(
            tmp.path().join("framework/templates/spec/spec.md"),
            "# Spec template\n",
        )
        .unwrap();
        let (path, content) = load_template("specify", tmp.path());
        assert_eq!(path, "framework/templates/spec/spec.md");
        assert_eq!(content, "# Spec template\n");
        // A command that fills no template resolves nothing.
        assert_eq!(
            load_template("review", tmp.path()),
            (String::new(), String::new())
        );
    }

    #[test]
    fn extract_section_name_ignores_whole_body_fill_prose() {
        // /ductus:specify's step 2 fills the whole spec body; its prose
        // mentions "a Motivation section" behind punctuation the heading
        // charset excludes, so no section is extracted.
        let prose = "Fill the new spec body following §spec-requirements: a \
                     Motivation section, Acceptance Criteria with concrete and \
                     testable checkboxes, and Open Questions.";
        assert!(extract_section_name(prose).is_none());
    }

    #[test]
    fn build_write_code_request_emits_typed_prefix_in_declaration_order() {
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/123-foo");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("tasks.md"),
            "# Tasks\n\n## 1. Stub a module\n\n- [ ] Create stub\n- **Done when**: file exists.\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("123-foo".into()));
        ctx.insert("task-number".into(), Value::String("1".into()));
        ctx.insert(
            "write-boundary".into(),
            Value::Array(vec![Value::String("runtime/**".into())]),
        );
        // A legacy field that should appear in the merged output AFTER the
        // typed prefix.
        ctx.insert("legacy-extra".into(), Value::String("kept".into()));

        let value = build_write_code_request(&ctx, tmp.path(), "implement").unwrap();
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        // Cache-anchor order: constitution-excerpts, plan-relevant-files,
        // write-boundary, task — then legacy keys.
        // The fixture repo carries no command file, so the excerpt load
        // reports `command-file-missing` and the unexaminable key rides in
        // the stable prefix beside the excerpts it explains.
        let prefix: Vec<&str> = keys.iter().take(5).copied().collect();
        assert_eq!(
            prefix,
            vec![
                "constitution-excerpts",
                "constitution-excerpts-unexaminable",
                "plan-relevant-files",
                "write-boundary",
                "task"
            ]
        );
        assert!(keys.contains(&"legacy-extra"));
        assert_eq!(value["task"]["number"], "1");
        assert_eq!(value["task"]["heading"], "Stub a module");
    }

    /// Stage a repo with a spec file, for the assessSpecQuality builder tests.
    fn stage_assess_fixture(repo: &Path) {
        fs::create_dir_all(repo.join("specs/003-analyze")).unwrap();
        fs::write(repo.join("specs/003-analyze/spec.md"), "# Spec body\n").unwrap();
    }

    /// A walker context seeded with the rule one round trip assesses, as the
    /// walker seeds it (spec 060).
    fn assess_context() -> Map<String, Value> {
        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("003-analyze".into()));
        ctx.insert(
            "path".into(),
            Value::String("specs/003-analyze/spec.md".into()),
        );
        ctx.insert(
            ASSESSED_RULE_KEY.into(),
            serde_json::json!({
                "id": "CFG-CONST-001",
                "verification": "Every constant is sourced from the central module.",
                "severity": "must"
            }),
        );
        // A legacy dump key that must NOT leak into the typed payload.
        ctx.insert("stdout".into(), Value::String("noise".into()));
        ctx
    }

    #[test]
    fn build_assess_spec_quality_request_emits_documented_typed_shape() {
        let tmp = tempdir().unwrap();
        stage_assess_fixture(tmp.path());
        let value = build_assess_spec_quality_request(&assess_context(), tmp.path());
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        // The typed fields lead — and, per the data model, are the entire
        // payload: no raw walker-context dump trails them.
        assert_eq!(keys, vec!["spec-path", "spec-content", "rule"]);
        assert_eq!(value["spec-path"], "specs/003-analyze/spec.md");
        assert_eq!(value["spec-content"], "# Spec body\n");
        assert_eq!(value["rule"]["id"], "CFG-CONST-001");
        assert_eq!(value["rule"]["severity"], "must");
        assert_eq!(
            value["rule"]["verification"],
            "Every constant is sourced from the central module."
        );
    }

    /// A session target's `path` is the spec directory, as `write-session`
    /// records it; the request carries the spec file inside it, never the
    /// directory with nothing read from it.
    #[test]
    fn build_assess_spec_quality_request_reads_the_spec_a_directory_path_names() {
        let tmp = tempdir().unwrap();
        stage_assess_fixture(tmp.path());
        let mut context = assess_context();
        context.insert("path".into(), Value::String("specs/003-analyze".into()));
        let value = build_assess_spec_quality_request(&context, tmp.path());
        assert_eq!(value["spec-path"], "specs/003-analyze/spec.md");
        assert_eq!(value["spec-content"], "# Spec body\n");
    }

    #[test]
    fn build_assess_spec_quality_request_is_typed_even_when_context_is_bare() {
        // Missing spec file, no seeded rule: the typed shape still leads
        // with empty fields rather than reverting to a dump.
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert("path".into(), Value::String("specs/absent/spec.md".into()));
        let value = build_assess_spec_quality_request(&ctx, tmp.path());
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, vec!["spec-path", "spec-content", "rule"]);
        assert_eq!(value["spec-content"], "");
        assert_eq!(value["rule"]["id"], "");
        assert_eq!(value["rule"]["severity"], "");
    }

    #[test]
    fn build_extension_request_routes_assess_spec_quality_to_typed_builder() {
        let tmp = tempdir().unwrap();
        stage_assess_fixture(tmp.path());
        let prose = "For every loaded MUST-tier rule, request an assessment.";
        let value = build_extension_request(
            "assessSpecQuality",
            &assess_context(),
            tmp.path(),
            "analyze",
            prose,
        )
        .unwrap();
        let obj = value.as_object().unwrap();
        assert!(obj.contains_key("spec-path"));
        assert_eq!(value["rule"]["id"], "CFG-CONST-001");
        // The raw context dump no longer reaches the host.
        assert!(!obj.contains_key("stdout"));
    }

    /// The step's phrase selects the tier of rules it asks about.
    #[test]
    fn a_step_s_phrase_selects_the_tier_it_asks_about() {
        assert_eq!(
            severity_from_step_prose("For every loaded MUST-tier rule"),
            RuleSeverity::Must
        );
        assert_eq!(
            severity_from_step_prose("For every loaded SHOULD-tier rule"),
            RuleSeverity::Should
        );
        assert_eq!(
            severity_from_step_prose("For every loaded rule"),
            RuleSeverity::Unspecified
        );
    }

    /// `load_rules` reads the listed files in order and sorts each rule by
    /// whether a request can assess it: a tier and a Verification make it
    /// assessable, in the tier its own Statement carries; lacking either, it
    /// is counted, never dropped. A file it cannot read — missing, or outside
    /// the repo — is counted too.
    #[test]
    fn load_rules_sorts_each_rule_by_whether_it_can_be_assessed() {
        let outer = tempdir().unwrap();
        let repo = outer.path().join("repo");
        fs::create_dir_all(repo.join("framework/rules")).unwrap();
        fs::write(
            outer.path().join("outside.md"),
            "### TO-OUT-001\n\n> MUST.\n\n**Verification:** v\n",
        )
        .unwrap();
        fs::write(
            repo.join("framework/rules/a.md"),
            "### TA-MUST-001\n\n> A MUST.\n\n**Verification:** a must.\n\n\
             ### TA-SHOULD-001\n\n> A SHOULD.\n\n**Verification:** a should.\n\n\
             ### TA-NONE-001\n\n> No keyword.\n\n**Verification:** a none.\n\n\
             ### TA-NOVER-001\n\n> A MUST with nothing to verify by.\n",
        )
        .unwrap();
        fs::write(
            repo.join("framework/rules/b.md"),
            "### TB-BOTH-001\n\n> Lists MUST paginate and SHOULD use cursors.\n\n**Verification:** b both.\n",
        )
        .unwrap();
        let mut ctx = Map::new();
        ctx.insert(
            "rule-files".into(),
            serde_json::json!([
                "framework/rules/a.md",
                "framework/rules/missing.md",
                "framework/rules/b.md",
                "../outside.md"
            ]),
        );
        let loaded = load_rules(&ctx, &repo);
        let assessable: Vec<(&str, RuleSeverity)> = loaded
            .assessable
            .iter()
            .map(|rule| (rule.id.as_str(), rule.severity))
            .collect();
        assert_eq!(
            assessable,
            [
                ("TA-MUST-001", RuleSeverity::Must),
                ("TA-SHOULD-001", RuleSeverity::Should),
                ("TB-BOTH-001", RuleSeverity::Must),
            ]
        );
        assert_eq!(loaded.assessable[0].verification, "a must.");
        assert_eq!(loaded.unassessable, 2);
        assert_eq!(loaded.unreadable_files, 2);
        assert!(!loaded.is_empty());
    }

    /// No rule file is an empty rule set; a set of only unassessable rules is
    /// not, since each of those is recorded on its own.
    #[test]
    fn a_rule_set_is_empty_only_when_it_holds_no_rule_at_all() {
        let tmp = tempdir().unwrap();
        assert!(load_rules(&Map::new(), tmp.path()).is_empty());
        fs::write(
            tmp.path().join("r.md"),
            "### TR-NONE-001\n\n> No keyword.\n\n**Verification:** v\n",
        )
        .unwrap();
        let mut ctx = Map::new();
        ctx.insert("rule-files".into(), serde_json::json!(["r.md"]));
        let loaded = load_rules(&ctx, tmp.path());
        assert!(loaded.assessable.is_empty());
        assert!(!loaded.is_empty());
    }

    #[test]
    fn build_perform_review_request_bundles_scope_and_rules() {
        let tmp = tempdir().unwrap();
        // In-scope file and a rule file the pass should read off disk.
        fs::create_dir_all(tmp.path().join("runtime/src")).unwrap();
        fs::write(tmp.path().join("runtime/src/main.rs"), "fn main() {}").unwrap();
        fs::create_dir_all(tmp.path().join("framework/rules")).unwrap();
        fs::write(
            tmp.path().join("framework/rules/security-backend.md"),
            "# Security\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert("pass".into(), Value::String("security".into()));
        ctx.insert(
            "scope".into(),
            Value::Array(vec![
                Value::String("runtime/src/main.rs".into()),
                // An absent path is skipped, not an error.
                Value::String("runtime/src/absent.rs".into()),
            ]),
        );
        ctx.insert("rules-dir".into(), Value::String("framework/rules".into()));
        ctx.insert(
            "selected".into(),
            Value::Array(vec![Value::String("security-backend.md".into())]),
        );

        let value = build_perform_review_request(&ctx, tmp.path());
        let obj = value.as_object().unwrap();
        // Typed prefix leads in cache-anchor order.
        let prefix: Vec<&str> = obj.keys().map(String::as_str).take(3).collect();
        assert_eq!(prefix, vec!["scope-files", "rule-files", "pass"]);
        assert_eq!(value["pass"], "security");
        // Only the readable scope file is bundled.
        assert_eq!(value["scope-files"].as_array().unwrap().len(), 1);
        assert_eq!(value["scope-files"][0]["path"], "runtime/src/main.rs");
        assert_eq!(value["scope-files"][0]["content"], "fn main() {}");
        assert_eq!(value["rule-files"][0]["name"], "security-backend.md");
        assert_eq!(value["rule-files"][0]["content"], "# Security\n");
    }

    #[test]
    fn build_perform_review_request_is_empty_without_scope_or_rules() {
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert("pass".into(), Value::String("reuse".into()));
        let value = build_perform_review_request(&ctx, tmp.path());
        assert_eq!(value["pass"], "reuse");
        assert!(value["scope-files"].as_array().unwrap().is_empty());
        assert!(value["rule-files"].as_array().unwrap().is_empty());
    }

    #[test]
    fn load_scope_files_confines_reads_to_the_repo_root() {
        // BE-INPUT-004: a `scope` entry that escapes the repo (absolute or
        // `..` traversal) or does not exist is skipped, never read into the
        // performReview payload; only the in-repo file is bundled.
        let outer = tempdir().unwrap();
        let repo = outer.path().join("repo");
        fs::create_dir_all(repo.join("src")).unwrap();
        fs::write(repo.join("src/in.rs"), "fn a() {}").unwrap();
        // A secret file OUTSIDE the repo the traversal/absolute entries target.
        fs::write(outer.path().join("secret.txt"), "leaked").unwrap();
        let abs_secret = outer
            .path()
            .join("secret.txt")
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        let mut ctx = Map::new();
        ctx.insert(
            "scope".into(),
            Value::Array(vec![
                Value::String("src/in.rs".into()),     // in-repo → read
                Value::String("../secret.txt".into()), // traversal → skipped
                Value::String(abs_secret),             // absolute → skipped
                Value::String("src/absent.rs".into()), // missing → skipped
            ]),
        );

        let files = load_scope_files(&ctx, &repo);
        assert_eq!(files.len(), 1, "only the in-repo scope file is read");
        assert_eq!(files[0].path, "src/in.rs");
        assert_eq!(files[0].content, "fn a() {}");
    }

    #[test]
    fn write_code_merge_filters_walker_accumulator_keys() {
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("123-foo".into()));
        ctx.insert("legacy-extra".into(), Value::String("kept".into()));
        // Walker-internal accumulator state that must not ride along.
        ctx.insert(
            "llm:writeSpecBody".into(),
            serde_json::json!({ "content": "prior response" }),
        );
        ctx.insert("findings".into(), serde_json::json!([{ "rule": "X" }]));

        let value = build_write_code_request(&ctx, tmp.path(), "implement").unwrap();
        let obj = value.as_object().unwrap();
        assert!(!obj.contains_key("llm:writeSpecBody"));
        assert!(!obj.contains_key("findings"));
        assert_eq!(obj["legacy-extra"], "kept");
        // The cache-anchor prefix is untouched by the filtering.
        let prefix: Vec<&str> = obj.keys().map(String::as_str).take(5).collect();
        assert_eq!(
            prefix,
            vec![
                "constitution-excerpts",
                "constitution-excerpts-unexaminable",
                "plan-relevant-files",
                "write-boundary",
                "task"
            ]
        );
    }

    #[test]
    fn perform_review_merge_keeps_primitive_results_drops_accumulators() {
        // Pass N must not see passes 1..N-1's findings, observations, or
        // llm:* echoes, but keeps the task-46 result threading: scope/diff-base
        // from compute-review-scope, selected/rules-dir/notices from
        // discover-rule-files.
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert("pass".into(), Value::String("reuse".into()));
        ctx.insert("scope".into(), serde_json::json!(["runtime/src/a.rs"]));
        ctx.insert("diff-base".into(), Value::String("abc1234".into()));
        ctx.insert("rules-dir".into(), Value::String("framework/rules".into()));
        ctx.insert("selected".into(), serde_json::json!(["reuse.md"]));
        ctx.insert("notices".into(), serde_json::json!(["fallback notice"]));
        ctx.insert(
            "findings".into(),
            serde_json::json!([{ "rule": "SEC-BE-001" }]),
        );
        ctx.insert(
            "observations".into(),
            serde_json::json!([{ "text": "perf: repeated scan" }]),
        );
        ctx.insert(
            "llm:performReview".into(),
            serde_json::json!({ "findings": [] }),
        );

        let value = build_perform_review_request(&ctx, tmp.path());
        let obj = value.as_object().unwrap();
        assert!(!obj.contains_key("findings"));
        assert!(!obj.contains_key("observations"));
        assert!(!obj.contains_key("llm:performReview"));
        for kept in ["scope", "diff-base", "rules-dir", "selected", "notices"] {
            assert!(obj.contains_key(kept), "primitive result `{kept}` dropped");
        }
    }

    /// The payload's shape, over a branch-scoped spec that carries its own
    /// scenarios — the case the source scenarios exist to cover.
    #[test]
    fn build_route_fold_request_emits_typed_shape_with_source_scenarios() {
        let tmp = tempdir().unwrap();
        let staged = tmp.path().join("specs/1234.1-widget-cache");
        fs::create_dir_all(staged.join("scenarios")).unwrap();
        fs::write(
            staged.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\nfolds-into: 050-alpha\n---\n\n# staged\n",
        )
        .unwrap();
        fs::write(
            staged.join("scenarios/eviction.md"),
            "---\nsection: Behavior\n---\n\n# Eviction\n",
        )
        .unwrap();
        fs::write(
            staged.join("scenarios/warmup.md"),
            "---\nsection: Motivation\n---\n\n# Warmup\n",
        )
        .unwrap();
        let target = tmp.path().join("specs/050-alpha");
        fs::create_dir_all(&target).unwrap();
        fs::write(
            target.join("spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# alpha\n\n## Motivation\n\nx\n\n             ```text\n## Not A Section\n```\n\n## Behavior\n\ny\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert(
            "feature".into(),
            Value::String("1234.1-widget-cache".into()),
        );
        // A dump key that must NOT leak into the typed payload.
        ctx.insert("stdout".into(), Value::String("noise".into()));

        let value = build_route_fold_request(&ctx, tmp.path());
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            vec![
                "feature",
                "spec-content",
                "scenarios",
                "fold-target",
                "target-content",
                "target-status",
                "target-sections",
                "routes",
            ]
        );
        assert_eq!(value["feature"], "1234.1-widget-cache");
        assert!(
            value["spec-content"]
                .as_str()
                .unwrap()
                .contains("folds-into: 050-alpha")
        );
        // Source scenarios are context, sorted by slug.
        assert_eq!(value["scenarios"][0]["slug"], "eviction");
        assert_eq!(value["scenarios"][0]["section"], "Behavior");
        assert_eq!(value["scenarios"][1]["slug"], "warmup");
        // The target is taken from the spec's own `folds-into`, not a
        // context key.
        assert_eq!(value["fold-target"], "050-alpha");
        assert_eq!(value["target-status"], "done");
        // A `## ` inside a fence is an example, not a fold destination.
        assert_eq!(
            value["target-sections"],
            serde_json::json!(["Motivation", "Behavior"])
        );
        assert_eq!(
            value["routes"],
            serde_json::json!(["body-edit", "scenario"])
        );
    }

    /// A spec with no scenarios and an unresolvable target still produces a
    /// payload: the target's absence is `retire-feature`'s refusal to make
    /// at the end of the fold, not this builder's.
    #[test]
    fn build_route_fold_request_degrades_rather_than_failing() {
        let tmp = tempdir().unwrap();
        let staged = tmp.path().join("specs/1234.1-staged");
        fs::create_dir_all(&staged).unwrap();
        fs::write(
            staged.join("spec.md"),
            "---\nstatus: draft\ndependencies: []\nfolds-into: 099-elsewhere\n---\n\n# staged\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert("path".into(), Value::String("specs/1234.1-staged".into()));

        let value = build_route_fold_request(&ctx, tmp.path());
        assert_eq!(value["feature"], "1234.1-staged");
        assert_eq!(value["fold-target"], "099-elsewhere");
        assert_eq!(value["target-content"], "");
        assert_eq!(value["target-status"], "");
        assert!(value.get("scenarios").is_none());
        assert!(value.get("target-sections").is_none());
    }

    /// A spec declaring no fold target yields an empty one rather than
    /// inventing a destination.
    /// BE-INPUT-004: `folds-into` is hand-authored, so it must not reach the
    /// filesystem uncontained. A traversing target reads nothing rather than
    /// lifting a `spec.md` from outside the repo into a payload bound for the
    /// host.
    #[test]
    fn build_route_fold_request_refuses_a_traversing_fold_target() {
        let tmp = tempdir().unwrap();
        // A `spec.md` outside the repo root, reachable only by traversal.
        let outside = tmp.path().join("outside");
        fs::create_dir_all(&outside).unwrap();
        fs::write(outside.join("spec.md"), "SECRET-OUTSIDE-THE-REPO\n").unwrap();

        let repo = tmp.path().join("repo");
        let staged = repo.join("specs/1234.1-staged");
        fs::create_dir_all(&staged).unwrap();
        fs::write(
            staged.join("spec.md"),
            "---\nstatus: draft\ndependencies: []\nfolds-into: ../../outside\n---\n\n# staged\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("1234.1-staged".into()));

        let value = build_route_fold_request(&ctx, &repo);
        assert_eq!(value["fold-target"], "../../outside");
        assert_eq!(
            value["target-content"], "",
            "a traversing fold target must read nothing"
        );
        assert_eq!(value["target-status"], "");
    }

    #[test]
    fn build_route_fold_request_leaves_an_undeclared_target_empty() {
        let tmp = tempdir().unwrap();
        let staged = tmp.path().join("specs/1234.1-standalone");
        fs::create_dir_all(&staged).unwrap();
        fs::write(
            staged.join("spec.md"),
            "---\nstatus: draft\ndependencies: []\n---\n\n# standalone\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("1234.1-standalone".into()));

        let value = build_route_fold_request(&ctx, tmp.path());
        assert_eq!(value["fold-target"], "");
        assert_eq!(value["target-content"], "");
    }

    #[test]
    fn build_ask_clarify_question_request_emits_typed_shape() {
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/007-clarify");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(feature_dir.join("spec.md"), "# Spec body\n").unwrap();
        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("007-clarify".into()));
        // Session-style directory path (not the spec file).
        ctx.insert("path".into(), Value::String("specs/007-clarify".into()));
        // Merged read-spec result.
        ctx.insert(
            "open-questions".into(),
            serde_json::json!([{ "text": "Which auth mode?" }, { "text": "Second?" }]),
        );
        // A dump key that must NOT leak into the typed payload.
        ctx.insert("stdout".into(), Value::String("noise".into()));

        let value = build_ask_clarify_question_request(&ctx, tmp.path());
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, vec!["spec-path", "spec-content", "question"]);
        assert_eq!(value["spec-path"], "specs/007-clarify/spec.md");
        assert_eq!(value["spec-content"], "# Spec body\n");
        assert_eq!(value["question"]["text"], "Which auth mode?");
        assert!(
            value["question"].get("section").is_none(),
            "unattributed question omits `section`"
        );
    }

    #[test]
    fn ask_clarify_question_prefers_explicit_question_value() {
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert(
            "question".into(),
            serde_json::json!({ "text": "Cap at 60s?", "section": "Behavior" }),
        );
        ctx.insert(
            "open-questions".into(),
            serde_json::json!([{ "text": "ignored fallback" }]),
        );
        let value = build_ask_clarify_question_request(&ctx, tmp.path());
        assert_eq!(value["question"]["text"], "Cap at 60s?");
        assert_eq!(value["question"]["section"], "Behavior");
    }

    #[test]
    fn build_route_inbox_item_request_lists_available_specs() {
        let tmp = tempdir().unwrap();
        for (slug, status) in [("001-alpha", "done"), ("002-beta", "draft")] {
            let dir = tmp.path().join("specs").join(slug);
            fs::create_dir_all(&dir).unwrap();
            fs::write(
                dir.join("spec.md"),
                format!("---\nstatus: {status}\ndependencies: []\n---\n\n# {slug}\n"),
            )
            .unwrap();
        }
        // Non-feature siblings are skipped.
        fs::create_dir_all(tmp.path().join("specs/templates")).unwrap();

        let mut ctx = Map::new();
        ctx.insert(
            "item-text".into(),
            Value::String("Bug: retry loop never backs off".into()),
        );
        ctx.insert("stdout".into(), Value::String("noise".into()));

        let value = build_route_inbox_item_request(&ctx, tmp.path());
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, vec!["item-text", "routes", "available-specs"]);
        assert_eq!(value["item-text"], "Bug: retry loop never backs off");
        assert_eq!(
            value["routes"],
            serde_json::json!(["rule", "spec", "scenario", "chore", "discard"])
        );
        assert_eq!(
            value["available-specs"],
            serde_json::json!([
                { "feature": "001-alpha", "status": "done" },
                { "feature": "002-beta", "status": "draft" }
            ])
        );
    }

    #[test]
    fn route_inbox_item_carries_specify_description_and_derived_candidates() {
        // The `/ductus:specify` entry point reaches the SAME routing point as
        // groom: `description` stands in for `item-text`, and
        // `derive-routing-candidates`' output rides along as evidence. A
        // second routing tree is what this avoids.
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/022-deterministic-runtime");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# 022\n",
        )
        .unwrap();

        let mut ctx = Map::new();
        ctx.insert(
            "description".into(),
            Value::String("a new check-artifacts family".into()),
        );
        ctx.insert(
            "candidates".into(),
            serde_json::json!([{
                "route": "scenario",
                "source": "runtime-work",
                "target": "022-deterministic-runtime",
                "path": "specs/022-deterministic-runtime",
                "status": "in-progress",
                "reopens": false,
                "reason": "names the primitive `check-artifacts`"
            }]),
        );

        let value = build_route_inbox_item_request(&ctx, tmp.path());
        assert_eq!(value["item-text"], "a new check-artifacts family");
        assert_eq!(
            value["candidates"][0]["target"],
            "022-deterministic-runtime"
        );
        assert_eq!(value["candidates"][0]["reopens"], false);
        // The vocabulary is the same closed set groom routes against.
        assert_eq!(
            value["routes"],
            serde_json::json!(["rule", "spec", "scenario", "chore", "discard"])
        );
    }

    #[test]
    fn route_inbox_item_prefers_item_text_over_description() {
        // Groom seeds `item-text`; a stale `description` in context must not
        // displace the item actually under decision.
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert("item-text".into(), Value::String("the inbox item".into()));
        ctx.insert("description".into(), Value::String("something else".into()));
        let value = build_route_inbox_item_request(&ctx, tmp.path());
        assert_eq!(value["item-text"], "the inbox item");
        assert!(value.as_object().unwrap().get("candidates").is_none());
    }

    #[test]
    fn build_verify_criteria_request_emits_typed_shape() {
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/004-implement");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(feature_dir.join("spec.md"), "# Spec body\n").unwrap();
        let mut ctx = Map::new();
        ctx.insert("feature".into(), Value::String("004-implement".into()));
        // Session-style directory path (not the spec file).
        ctx.insert("path".into(), Value::String("specs/004-implement".into()));
        // Merged read-spec result from the completion gate's read step.
        ctx.insert(
            "acceptance-criteria".into(),
            serde_json::json!([
                { "checked": false, "text": "The walker completes." },
                { "checked": true, "text": "Boundary edits are rejected." }
            ]),
        );
        // A dump key that must NOT leak into the typed payload.
        ctx.insert("stdout".into(), Value::String("noise".into()));

        let value = build_verify_criteria_request(&ctx, tmp.path());
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, vec!["spec-path", "spec-content", "criteria"]);
        assert_eq!(value["spec-path"], "specs/004-implement/spec.md");
        assert_eq!(value["spec-content"], "# Spec body\n");
        // Indexes follow body order — the same 0-based addressing
        // mark-criterion consumes.
        assert_eq!(
            value["criteria"],
            serde_json::json!([
                { "index": 0, "text": "The walker completes.", "checked": false },
                { "index": 1, "text": "Boundary edits are rejected.", "checked": true }
            ])
        );
    }

    #[test]
    fn build_verify_criteria_request_is_typed_even_when_context_is_bare() {
        // Missing spec file and no criteria: the typed shape still leads
        // with empty fields rather than reverting to a dump.
        let tmp = tempdir().unwrap();
        let value = build_verify_criteria_request(&Map::new(), tmp.path());
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, vec!["spec-path", "spec-content", "criteria"]);
        assert_eq!(value["spec-path"], "");
        assert_eq!(value["spec-content"], "");
        assert!(value["criteria"].as_array().unwrap().is_empty());
    }

    #[test]
    fn build_extension_request_routes_verify_criteria_to_typed_builder() {
        let tmp = tempdir().unwrap();
        let mut ctx = Map::new();
        ctx.insert(
            "acceptance-criteria".into(),
            serde_json::json!([{ "checked": false, "text": "c" }]),
        );
        ctx.insert("stdout".into(), Value::String("noise".into()));
        let value =
            build_extension_request("verifyCriteria", &ctx, tmp.path(), "implement", "").unwrap();
        let obj = value.as_object().unwrap();
        assert!(obj.contains_key("criteria"));
        // The raw context dump does not reach the host.
        assert!(!obj.contains_key("stdout"));
    }

    #[test]
    fn build_extension_request_errors_on_unknown_identifier() {
        // The raw-context-dump fallback is gone: an identifier without a
        // typed builder is a structured error (extension-request-hygiene).
        let tmp = tempdir().unwrap();
        let err = build_extension_request("mysteryPoint", &Map::new(), tmp.path(), "test", "")
            .unwrap_err();
        assert_eq!(err.code(), "unknown-extension");
        assert!(err.to_string().contains("mysteryPoint"));
    }

    #[test]
    fn an_unknown_extension_identifier_still_errors() {
        // The registry stays closed. A typo'd marker must not silently
        // produce an empty request the host then answers about nothing.
        let tmp = tempdir().unwrap();
        let ctx = Map::new();
        assert!(
            build_extension_request("classifyClaimz", &ctx, tmp.path(), "consolidate", "").is_err()
        );
    }
}
