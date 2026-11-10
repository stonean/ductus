//! `resolve-references` — classify each cross-service reference (spec 030).
//!
//! Reads the consumer feature's derived `references:` index, resolves each
//! entry's service through the project config's `[services]` registry, and
//! reads the linked spec's live `status` from the registered local checkout.
//! It also reports each repo the registry holds under two or more aliases.
//! Every reference is classified into the closed [`ReferenceOutcome`] enum by
//! deterministic predicates — no prose is read for intent. Reuses
//! `read_text` / `split_frontmatter` and the allowed-status set that
//! `validate-frontmatter` enforces.
//!
//! The index stores `{service, spec}` pairs (the raw URL is not persisted),
//! so URL-level distinctions collapse here: a malformed URL that yielded no
//! valid spec was never harvested, and a resolved-but-absent target surfaces
//! as `broken`; a scenario link was normalized to its feature slug at harvest
//! time. Schema is canonical in
//! `specs/030-cross-service-references/data-model.md`.

use std::path::Path;

use serde::Deserialize;

use crate::primitives::{
    PrimitiveError, Result, frontmatter_status, read_text, rel_path, resolve_path,
    split_frontmatter,
};
use crate::schema::config_keys;
use crate::schema::paths;
use crate::schema::primitives::{
    DuplicateRepo, ListsForm, ReferenceOutcome, ResolutionRecord, ResolveReferencesArgs,
    ResolveReferencesResult,
};
use crate::schema::services::{Services, ServicesError};
use crate::schema::status::ALLOWED_STATUSES;

/// The subset of the consumer spec's frontmatter this primitive reads: the
/// derived `references:` index. Other fields are ignored.
#[derive(Debug, Default, Deserialize)]
struct ConsumerFrontmatter {
    #[serde(default)]
    references: Vec<IndexEntry>,
}

/// One `references:` index entry. `service` is null for an `unregistered`
/// reference (the harvester matched no registry repo).
#[derive(Debug, Deserialize)]
struct IndexEntry {
    #[serde(default)]
    service: Option<String>,
    spec: String,
}

/// Execute the `resolve-references` primitive.
///
/// The result is bounded (`BE-PAYLOAD-001`): its two lists are inline while
/// the serialized result fits `[artifacts] read-size-bytes`, and past it they
/// move whole to the file `lists-file` names, with their counts inline, as
/// `compute-review-scope`'s do (066 scenario `scope-results-have-a-bounded-size`).
///
/// # Errors
///
/// Every error [`resolve`] returns, and those `bounded_lists::place` returns
/// for a malformed read size or a list file that cannot be written.
pub fn run(args: &ResolveReferencesArgs, repo: &Path) -> Result<ResolveReferencesResult> {
    let resolved = resolve(args, repo)?;
    let mut result = ResolveReferencesResult {
        references_count: resolved.references.len(),
        duplicate_repos_count: resolved.duplicate_repos.len(),
        references: Some(resolved.references.clone()),
        duplicate_repos: Some(resolved.duplicate_repos.clone()),
        form: ListsForm::Inline,
        lists_file: None,
        path: resolved.path,
    };
    let lists = serde_json::json!({
        "references": resolved.references,
        "duplicate-repos": resolved.duplicate_repos,
    });
    if let Some(file) =
        super::bounded_lists::place(repo, "resolve-references", &args.feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.references = None;
        result.duplicate_repos = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// A feature's references resolved, every list whole: what `resolve-references`
/// returns before its result is bounded, and what `dashboard`'s readout renders.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Resolved {
    /// One record per `references:` entry, in index order.
    pub(crate) references: Vec<ResolutionRecord>,
    /// Each repo the registry holds under two or more aliases.
    pub(crate) duplicate_repos: Vec<DuplicateRepo>,
    /// Repo-relative path to the consumer spec file.
    pub(crate) path: String,
}

/// Resolve a feature's `references:` index against the registry, unbounded.
///
/// # Errors
///
/// Returns [`PrimitiveError::Io`] when the consumer spec or the project config
/// cannot be read, [`PrimitiveError::MissingFrontmatter`] when the spec has no
/// `---` block, [`PrimitiveError::UnclosedFrontmatter`] when its block never
/// closes, [`PrimitiveError::Yaml`] when its frontmatter is not valid YAML,
/// [`PrimitiveError::Toml`] when the resolved project config is present but
/// does not parse, and [`PrimitiveError::InvalidServiceEntry`] when a
/// `[services.<alias>]` entry's `repo` or `path` is absent or empty.
/// A reachable service checkout whose own project config cannot be read or
/// parsed, or whose `[paths] specs-root` is invalid, halts the same way.
/// Per-reference resolution failures are outcomes, not errors.
pub(crate) fn resolve(args: &ResolveReferencesArgs, repo: &Path) -> Result<Resolved> {
    let root = paths::Paths::load(repo)?.specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    let spec_path = repo.join(&root).join(&args.feature).join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, _body) = split_frontmatter(&content, &spec_path)?;
    let frontmatter: ConsumerFrontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;

    let services = load_services(repo)?;

    let references = frontmatter
        .references
        .iter()
        .map(|entry| {
            let (outcome, status) = classify(repo, &services, entry)?;
            Ok(ResolutionRecord {
                service: entry.service.clone(),
                spec: entry.spec.clone(),
                outcome,
                status,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // A repo under two aliases resolves through one of them silently, so the
    // registry's duplicates are part of the answer (spec 030's data model).
    let duplicate_repos = services
        .duplicate_repos()
        .into_iter()
        .map(|(repo, aliases)| DuplicateRepo { repo, aliases })
        .collect();

    Ok(Resolved {
        references,
        duplicate_repos,
        path: rel_path(&spec_path, repo),
    })
}

/// Read `[services]` from the resolved project config — whichever tier
/// `paths::config_path` returns (`.ductus/config.toml`, `.govern/config.toml`,
/// or the legacy root `.govern.toml`), so this never spells a tier itself.
///
/// An absent file is an empty registry. A document that does not parse is
/// [`PrimitiveError::Toml`], and an entry whose `repo` or `path` is absent or
/// empty is [`PrimitiveError::InvalidServiceEntry`]: both halt, naming the
/// file, since reading past either would report every reference through the
/// registry wrongly (`CFG-ENV-003`, `CFG-ENV-008`). `pub(crate)`: the one
/// reader of the registry, shared with `dashboard`, whose readout appends the
/// matched service's `description`, and `derive-references`.
pub(crate) fn load_services(repo: &Path) -> Result<Services> {
    let toml_path = paths::config_path(repo);
    if !toml_path.exists() {
        return Ok(Services::default());
    }
    let content = read_text(&toml_path)?;
    config_keys::warn_unknown_keys(&toml_path, &content, &["services"]);
    Services::from_toml_str(&content).map_err(|source| match source {
        ServicesError::Parse(source) => PrimitiveError::Toml {
            path: toml_path,
            source,
        },
        ServicesError::Invalid {
            alias,
            field,
            reason,
        } => PrimitiveError::InvalidServiceEntry {
            path: toml_path,
            alias,
            field,
            reason,
        },
    })
}

/// Classify one reference against the registry and the local checkout.
///
/// The `[services]` `path` is read without `validate_no_traversal`; the
/// reasoning is stated once, under **The config-sourced path boundary** on
/// [`super::validate_no_traversal`], and cited here rather than restated.
/// `resolve-constitutions` cites the same statement for `[constitutions.*]`.
///
/// What that statement obliges of *this* table: `/{project}:link` already
/// writes `[services]`, so the converse obligation is live here rather than
/// latent. It is satisfied because that command *transcribes* — it prompts
/// for the local checkout path, records what the operator typed verbatim,
/// and warns without blocking when it does not resolve — so the operator is
/// still the origin of the value. A change that had `/{project}:link`
/// derive the path from anything else would move this read into the
/// validated tier.
fn classify(
    repo: &Path,
    services: &Services,
    entry: &IndexEntry,
) -> Result<(ReferenceOutcome, Option<String>)> {
    // Null service (or an alias no longer in the registry) → unregistered:
    // a plain navigational link, status not attempted.
    let Some(alias) = entry.service.as_deref() else {
        return Ok((ReferenceOutcome::Unregistered, None));
    };
    let Some(service) = services.0.get(alias) else {
        return Ok((ReferenceOutcome::Unregistered, None));
    };

    // Resolve the local checkout (relative to the repo root, or absolute).
    // `..` is permitted — a sibling checkout is the normal case
    // (`path = "../api"`), and this is committed config, not an LLM-supplied
    // path; see the boundary statement cited on this function. A
    // missing/unusable path can prove nothing, so it is `not-checked-out`,
    // never `broken`.
    let checkout = resolve_path(repo, &service.path);
    if !checkout.is_dir() {
        return Ok((ReferenceOutcome::NotCheckedOut, None));
    }

    // Reachable checkout: the target spec either resolves or is provably broken.
    let target = paths::specs_dir(&checkout)?
        .join(&entry.spec)
        .join("spec.md");
    if !target.is_file() {
        return Ok((ReferenceOutcome::Broken, None));
    }

    Ok(match read_target_status(&target) {
        Some(status) => (ReferenceOutcome::Ok, Some(status)),
        None => (ReferenceOutcome::StatusUnreadable, None),
    })
}

/// Read a reachable target spec's `status`, returning `Some(status)` only when
/// the frontmatter parses (shared [`frontmatter_status`] reader) and the value
/// is in the allowed set. Any unreadability (no frontmatter, malformed YAML,
/// missing/non-string/out-of-set `status`) collapses to `None` →
/// `status-unreadable`.
fn read_target_status(target: &Path) -> Option<String> {
    let content = std::fs::read_to_string(target).ok()?;
    let status = frontmatter_status(&content, target)?;
    ALLOWED_STATUSES
        .contains(&status.as_str())
        .then_some(status)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::path::Path;

    /// Write `content` to `path`, creating parent directories.
    fn write_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    /// Seed a consumer repo: `.govern.toml` from `toml`, and a consumer
    /// `specs/001-consumer/spec.md` whose frontmatter carries `references`.
    fn seed_consumer(repo: &Path, toml: &str, references_block: &str) {
        write_file(&repo.join(".govern.toml"), toml);
        let spec = format!(
            "---\nstatus: in-progress\ndependencies: []\n{references_block}---\n\n# Consumer\n"
        );
        write_file(&repo.join("specs/001-consumer/spec.md"), &spec);
    }

    /// Write a fake registered checkout's spec at
    /// `<repo>/<checkout>/specs/<slug>/spec.md` with the given raw content.
    fn write_checkout_spec(repo: &Path, checkout: &str, slug: &str, content: &str) {
        write_file(
            &repo.join(checkout).join("specs").join(slug).join("spec.md"),
            content,
        );
    }

    fn consumer_args() -> ResolveReferencesArgs {
        ResolveReferencesArgs {
            feature: "001-consumer".into(),
        }
    }

    /// The consumer's references resolved whole, before the result is bounded.
    fn run_consumer(repo: &Path) -> Resolved {
        resolve(&consumer_args(), repo).unwrap()
    }

    const API_TOML: &str =
        "[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"checkouts/api\"\n";

    #[test]
    fn ok_when_registered_reachable_and_status_in_set() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        write_checkout_spec(
            repo,
            "checkouts/api",
            "003-user",
            "---\nstatus: clarified\n---\n# U\n",
        );

        let result = run_consumer(repo);
        assert_eq!(result.references.len(), 1);
        let rec = &result.references[0];
        assert_eq!(rec.service.as_deref(), Some("api"));
        assert_eq!(rec.spec, "003-user");
        assert_eq!(rec.outcome, ReferenceOutcome::Ok);
        assert_eq!(rec.status.as_deref(), Some("clarified"));
        assert_eq!(result.path, "specs/001-consumer/spec.md");
    }

    #[test]
    fn resolves_under_configured_specs_root_on_both_sides() {
        // Spec 040: the consumer's spec path and the registered checkout's
        // target spec path both resolve `[paths] specs-root`, so a reference
        // resolves when either side renames its spec root to `governance`.
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        write_file(
            &repo.join(".govern.toml"),
            "[paths]\nspecs-root = \"governance\"\n\n[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"checkouts/api\"\n",
        );
        write_file(
            &repo.join("governance/001-consumer/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\nreferences:\n  - service: api\n    spec: 003-user\n---\n\n# Consumer\n",
        );
        // The registered checkout also renames its spec root.
        write_file(
            &repo.join("checkouts/api/.govern.toml"),
            "[paths]\nspecs-root = \"governance\"\n",
        );
        write_file(
            &repo.join("checkouts/api/governance/003-user/spec.md"),
            "---\nstatus: clarified\n---\n# U\n",
        );

        let result = run_consumer(repo);
        assert_eq!(result.path, "governance/001-consumer/spec.md");
        let rec = &result.references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::Ok);
        assert_eq!(rec.status.as_deref(), Some("clarified"));
    }

    #[test]
    fn unregistered_when_service_is_null() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        // Null service in the index → unregistered, status not attempted.
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: null\n    spec: 004-orders\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.service, None);
        assert_eq!(rec.outcome, ReferenceOutcome::Unregistered);
        assert_eq!(rec.status, None);
    }

    #[test]
    fn unregistered_when_alias_absent_from_registry() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        // Index names `ghost`, but the registry only knows `api`.
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: ghost\n    spec: 003-user\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::Unregistered);
    }

    #[test]
    fn not_checked_out_when_path_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        // Registered, but no checkout directory exists at `checkouts/api`.
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: api\n    spec: 003-user\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::NotCheckedOut);
        assert_eq!(rec.status, None);
    }

    #[test]
    fn broken_when_checkout_reachable_but_target_absent() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        // Checkout exists (some other spec present) but 003-user does not —
        // renamed / deleted / mistyped upstream, or a malformed URL's slug.
        write_checkout_spec(
            repo,
            "checkouts/api",
            "009-other",
            "---\nstatus: done\n---\n# O\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::Broken);
        assert_eq!(rec.status, None);
    }

    #[test]
    fn status_unreadable_when_no_frontmatter() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        write_checkout_spec(
            repo,
            "checkouts/api",
            "003-user",
            "# User\n\nNo frontmatter here.\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::StatusUnreadable);
    }

    #[test]
    fn status_unreadable_when_yaml_malformed() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        // Unbalanced bracket → YAML parse failure inside the frontmatter.
        write_checkout_spec(
            repo,
            "checkouts/api",
            "003-user",
            "---\nstatus: [unterminated\n---\n# U\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::StatusUnreadable);
    }

    #[test]
    fn status_unreadable_when_status_out_of_set() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        write_checkout_spec(
            repo,
            "checkouts/api",
            "003-user",
            "---\nstatus: wibble\n---\n# U\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::StatusUnreadable);
    }

    #[test]
    fn self_reference_resolves_like_any_registered_service() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        // The consumer's own repo is registered with `path = "."`, and the
        // reference points back at one of its own specs. No special-casing:
        // it resolves `ok` like any registered service.
        let toml = "[services.self]\nrepo = \"https://github.com/acme/consumer\"\npath = \".\"\n";
        seed_consumer(
            repo,
            toml,
            "references:\n  - service: self\n    spec: 002-local\n",
        );
        write_file(
            &repo.join("specs/002-local/spec.md"),
            "---\nstatus: planned\ndependencies: []\n---\n# Local\n",
        );

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.service.as_deref(), Some("self"));
        assert_eq!(rec.outcome, ReferenceOutcome::Ok);
        assert_eq!(rec.status.as_deref(), Some("planned"));
    }

    #[test]
    fn empty_index_yields_no_records() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        // No `references:` field at all.
        seed_consumer(repo, API_TOML, "");

        let result = run_consumer(repo);
        assert_eq!(result.references, [] as [ResolutionRecord; 0]);
        assert_eq!(result.duplicate_repos, [] as [DuplicateRepo; 0]);
    }

    /// A repo registered under two aliases, by spellings the harvester treats
    /// as one, is reported with its aliases: resolution uses one of them, and
    /// without this nothing says which or that there was a choice.
    #[test]
    fn a_repo_under_two_aliases_is_reported_with_both() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let toml = "[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"checkouts/api\"\n\n\
                    [services.mirror]\nrepo = \"https://github.com/acme/api.git\"\npath = \"checkouts/mirror\"\n\n\
                    [services.web]\nrepo = \"https://github.com/acme/web\"\npath = \"checkouts/web\"\n";
        seed_consumer(
            repo,
            toml,
            "references:\n  - service: api\n    spec: 003-user\n",
        );

        let result = run_consumer(repo);
        assert_eq!(
            result.duplicate_repos,
            [DuplicateRepo {
                repo: "https://github.com/acme/api".into(),
                aliases: vec!["api".into(), "mirror".into()],
            }]
        );
        // The references resolve as before.
        assert_eq!(
            result.references[0].outcome,
            ReferenceOutcome::NotCheckedOut
        );
    }

    /// Distinct repos report no duplicate, and the key is present and empty
    /// on the wire rather than absent.
    #[test]
    fn distinct_repos_report_an_empty_duplicate_list() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            API_TOML,
            "references:\n  - service: api\n    spec: 003-user\n",
        );

        let result = run(&consumer_args(), repo).unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        assert_eq!(result.duplicate_repos, Some(Vec::new()));
        assert_eq!(result.duplicate_repos_count, 0);
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["duplicate-repos"], serde_json::json!([]));
    }

    /// An entry with no `path` halts the run naming the alias, the field and
    /// the file, rather than resolving the service against this repository
    /// (`CFG-ENV-003`).
    #[test]
    fn an_entry_without_a_path_halts_naming_the_alias_the_field_and_the_file() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            "[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"\"\n",
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        let err = run(&consumer_args(), repo).unwrap_err();
        let PrimitiveError::InvalidServiceEntry { alias, field, .. } = &err else {
            panic!("expected an invalid entry, got {err}");
        };
        assert_eq!((alias.as_str(), field.as_str()), ("api", "path"));
        assert!(err.to_string().contains(".govern.toml"), "{err}");
    }

    /// A config that does not parse halts, naming the file (`CFG-ENV-008`).
    #[test]
    fn a_config_that_does_not_parse_halts_naming_the_file() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        seed_consumer(
            repo,
            "[services.api\n",
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        let err = run(&consumer_args(), repo).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }), "{err}");
        assert!(err.to_string().contains(".govern.toml"), "{err}");
    }

    /// Past `[artifacts] read-size-bytes` both lists move whole to the file the
    /// result names, neither inline, and the counts still say how many
    /// (`BE-PAYLOAD-001`).
    #[test]
    fn a_result_past_the_read_size_names_a_file_holding_both_lists() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let toml = "[artifacts]\nread-size-bytes = 10\n\n\
                    [services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"checkouts/api\"\n\n\
                    [services.mirror]\nrepo = \"https://github.com/acme/api/\"\npath = \"checkouts/api\"\n";
        seed_consumer(
            repo,
            toml,
            "references:\n  - service: api\n    spec: 003-user\n",
        );
        let whole = run_consumer(repo);

        let bounded = run(&consumer_args(), repo).unwrap();
        assert_eq!(bounded.form, ListsForm::File);
        assert_eq!(
            (&bounded.references, &bounded.duplicate_repos),
            (&None, &None)
        );
        assert_eq!(
            (bounded.references_count, bounded.duplicate_repos_count),
            (1, 1)
        );
        let file = bounded.lists_file.expect("the file form names its file");
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(repo.join(&file)).unwrap()).unwrap();
        assert_eq!(
            lists["references"],
            serde_json::to_value(&whole.references).unwrap()
        );
        assert_eq!(
            lists["duplicate-repos"],
            serde_json::to_value(&whole.duplicate_repos).unwrap()
        );
    }

    #[test]
    fn absent_ductus_toml_makes_named_service_unregistered() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        // No `.govern.toml` → empty registry → a named alias resolves to
        // unregistered rather than erroring.
        let spec = "---\nstatus: draft\ndependencies: []\nreferences:\n  - service: api\n    spec: 003-user\n---\n\n# Consumer\n";
        write_file(&repo.join("specs/001-consumer/spec.md"), spec);

        let rec = &run_consumer(repo).references[0];
        assert_eq!(rec.outcome, ReferenceOutcome::Unregistered);
    }
}
