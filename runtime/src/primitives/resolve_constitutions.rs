//! `resolve-constitutions` — resolve each registered shared constitution
//! (spec 055) to a document on local disk, and say what could not be resolved.
//!
//! Reads the project config's `[constitutions]` registry and, for each entry,
//! resolves the local `path` and looks for `constitution.md` inside it. The
//! `repo` URL is identity only and is **never fetched** — this primitive makes
//! no network call and has no transport, which is why "unreachable" collapses
//! to "not checked out" rather than being an error class.
//!
//! The registry read mirrors `resolve_references::load_services` and the
//! checkout resolution mirrors its `classify`, deliberately: `[services]`
//! already settled that a missing checkout is a state rather than a config
//! error, and reusing that path keeps the two registries legible to each other.
//!
//! Results split into `loaded` and `skipped` rather than returning only what
//! worked. That is `QUAL-CLAIM-001` at the primitive layer: a caller reporting
//! a clean result over a source in `skipped` has to drop a field it was handed,
//! instead of relying on remembering to check.
//!
//! Schema is canonical in `specs/055-shared-constitution/data-model.md`.

use std::path::Path;

use crate::primitives::{PrimitiveError, Result, read_text, rel_path, resolve_path};
use crate::schema::constitutions::Constitutions;
use crate::schema::paths;
use crate::schema::primitives::{
    ConstitutionOutcome, ConstitutionRecord, DuplicateConstitutionPath, ResolveConstitutionsArgs,
    ResolveConstitutionsResult,
};

/// The document filename looked for inside a registered checkout. Fixed rather
/// than configurable: a per-entry filename would let two projects registering
/// the same source load different documents from it, which AC7 forbids.
const DOCUMENT_NAME: &str = "constitution.md";

/// Execute the `resolve-constitutions` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::Toml`] when the project config is present but
/// malformed, or [`PrimitiveError::Io`] when it exists and cannot be read.
/// Per-entry resolution failures are outcomes, not errors — an absent checkout
/// is a state this primitive reports, never a condition it fails on.
pub fn run(_args: &ResolveConstitutionsArgs, repo: &Path) -> Result<ResolveConstitutionsResult> {
    let registry = load_constitutions(repo)?;

    let mut loaded = Vec::new();
    let mut skipped = Vec::new();

    // Alias order, from the registry's `BTreeMap`. Deterministic across
    // machines and immune to a TOML reformat — AC7's requirement.
    for (alias, entry) in &registry.0 {
        let (outcome, document) = classify(repo, &entry.path);
        let record = ConstitutionRecord {
            alias: alias.clone(),
            repo: entry.repo.clone(),
            path: entry.path.clone(),
            document,
            description: normalize_description(entry.description.as_deref()),
            outcome,
        };
        if outcome == ConstitutionOutcome::Loaded {
            loaded.push(record);
        } else {
            skipped.push(record);
        }
    }

    let duplicate_paths = registry
        .duplicate_paths()
        .into_iter()
        .map(|(path, aliases)| DuplicateConstitutionPath { path, aliases })
        .collect();

    Ok(ResolveConstitutionsResult {
        loaded,
        skipped,
        examined: registry.0.len(),
        duplicate_paths,
    })
}

/// Read the `[constitutions]` registry from the project config. An absent
/// config file is an empty registry; a malformed one is an operational error.
///
/// An absent file and an absent table both yield an empty registry, which is
/// what makes AC1 true by construction: nothing downstream can distinguish
/// "no config" from "no entries", so neither can produce a different behavior.
fn load_constitutions(repo: &Path) -> Result<Constitutions> {
    let toml_path = paths::config_path(repo);
    if !toml_path.exists() {
        return Ok(Constitutions::default());
    }
    let content = read_text(&toml_path)?;
    Constitutions::from_toml_str(&content).map_err(|source| PrimitiveError::Toml {
        path: toml_path,
        source,
    })
}

/// Collapse a registered source's `description` to the single-line form every
/// consumer renders, or `None` when there is nothing to say.
///
/// Collapsed **here** rather than at each renderer so the surfaces cannot
/// disagree, and because the hazard is in the data rather than in the display:
/// TOML multi-line strings make an embedded newline reachable, and one newline
/// in a description would break a single-line report into two. The same
/// posture the disabled-rule-file notice takes with its `reason`.
///
/// Whitespace-only is `None`, not `Some("")`: an empty description says
/// nothing, and rendering an empty parenthetical would be worse than rendering
/// none — absent is absent.
pub(crate) fn normalize_description(description: Option<&str>) -> Option<String> {
    let collapsed = description?
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    (!collapsed.is_empty()).then_some(collapsed)
}

/// Classify one entry's `path` against the local filesystem.
///
/// `..` is permitted and absolute paths are accepted — a sibling checkout
/// (`path = "../governance"`) is the normal case, and this value comes from
/// committed config, not from the host or an LLM. The reasoning is stated
/// once, under **The config-sourced path boundary** on
/// [`super::validate_no_traversal`], and cited here rather than restated;
/// `resolve-references` cites the same statement for `[services]`.
///
/// What that statement obliges of *this* table: `[constitutions.*]` is
/// registered by hand today (`framework/bootstrap/ductus.md`), so the
/// operator is the origin of every path here. A future command that
/// originates the value instead ships the `validate_no_traversal` call with
/// itself.
fn classify(repo: &Path, path_value: &str) -> (ConstitutionOutcome, Option<String>) {
    let checkout = resolve_path(repo, path_value);
    if !checkout.is_dir() {
        return (ConstitutionOutcome::NotCheckedOut, None);
    }
    let document = checkout.join(DOCUMENT_NAME);
    if document.is_file() {
        (ConstitutionOutcome::Loaded, Some(rel_path(&document, repo)))
    } else {
        (ConstitutionOutcome::NoConstitutionDocument, None)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Build a repo root with the given config contents (or none at all).
    fn repo_with_config(config: Option<&str>) -> TempDir {
        let dir = TempDir::new().unwrap();
        if let Some(contents) = config {
            let config_path = dir.path().join(".ductus");
            fs::create_dir_all(&config_path).unwrap();
            fs::write(config_path.join("config.toml"), contents).unwrap();
        }
        dir
    }

    fn run_in(dir: &TempDir) -> ResolveConstitutionsResult {
        run(&ResolveConstitutionsArgs {}, dir.path()).unwrap()
    }

    /// The description reaches the record rather than stopping at the parser —
    /// it is the field that answers what a source governs, and an alias does
    /// not.
    #[test]
    fn a_description_is_carried_through_to_the_record() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"gov\"\n\
             description = \"Acme platform engineering rules\"\n",
        ));
        fs::create_dir_all(dir.path().join("gov")).unwrap();
        fs::write(dir.path().join("gov/constitution.md"), "# Rules\n").unwrap();

        let result = run_in(&dir);
        assert_eq!(result.loaded.len(), 1);
        assert_eq!(
            result.loaded[0].description.as_deref(),
            Some("Acme platform engineering rules")
        );
    }

    /// The description comes from the config, not the checkout, so it is
    /// available precisely when the document is not — which is the case where
    /// it helps most.
    #[test]
    fn a_skipped_entry_still_carries_its_description() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"nowhere\"\n\
             description = \"Acme platform engineering rules\"\n",
        ));

        let result = run_in(&dir);
        assert_eq!(result.skipped.len(), 1);
        assert_eq!(
            result.skipped[0].description.as_deref(),
            Some("Acme platform engineering rules")
        );
    }

    /// Absent is absent: a project that never writes one sees no change.
    #[test]
    fn an_entry_without_a_description_carries_none() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"nowhere\"\n",
        ));
        assert_eq!(run_in(&dir).skipped[0].description, None);
    }

    /// A TOML multi-line string makes an embedded newline reachable, and one
    /// newline would break a single-line report into two.
    #[test]
    fn an_embedded_newline_is_collapsed_before_it_reaches_a_report() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"nowhere\"\n\
             description = \"\"\"\nplatform rules,\n  including security\n\"\"\"\n",
        ));
        assert_eq!(
            run_in(&dir).skipped[0].description.as_deref(),
            Some("platform rules, including security")
        );
    }

    /// Whitespace-only says nothing, and an empty parenthetical would render
    /// worse than none at all.
    #[test]
    fn a_whitespace_only_description_is_none_not_empty() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"nowhere\"\n\
             description = \"   \"\n",
        ));
        assert_eq!(run_in(&dir).skipped[0].description, None);
    }

    #[test]
    fn absent_config_resolves_nothing_and_reports_nothing() {
        let dir = repo_with_config(None);
        let result = run_in(&dir);
        assert_eq!(result.examined, 0);
        assert!(result.loaded.is_empty());
        assert!(
            result.skipped.is_empty(),
            "nothing registered is not the same as something unreadable"
        );
    }

    #[test]
    fn absent_table_and_empty_table_agree() {
        // AC1: the two inputs must be indistinguishable in the result.
        let absent = run_in(&repo_with_config(Some(
            "[review]\ntech-stack-verified = true\n",
        )));
        let empty = run_in(&repo_with_config(Some("[constitutions]\n")));
        assert_eq!(absent, empty);
        assert_eq!(empty.examined, 0);
    }

    #[test]
    fn resolved_entry_is_loaded_with_its_document() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"governance\"\n",
        ));
        let checkout = dir.path().join("governance");
        fs::create_dir_all(&checkout).unwrap();
        fs::write(checkout.join("constitution.md"), "# House rules\n").unwrap();

        let result = run_in(&dir);
        assert_eq!(result.examined, 1);
        assert_eq!(result.loaded.len(), 1);
        assert!(result.skipped.is_empty());

        let record = &result.loaded[0];
        assert_eq!(record.alias, "acme");
        assert_eq!(record.outcome, ConstitutionOutcome::Loaded);
        assert_eq!(
            record.document.as_deref(),
            Some("governance/constitution.md")
        );
    }

    #[test]
    fn missing_checkout_is_not_checked_out() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"nowhere\"\n",
        ));
        let result = run_in(&dir);
        assert_eq!(result.examined, 1);
        assert!(result.loaded.is_empty());
        assert_eq!(result.skipped.len(), 1);
        assert_eq!(
            result.skipped[0].outcome,
            ConstitutionOutcome::NotCheckedOut
        );
        assert!(result.skipped[0].document.is_none());
    }

    #[test]
    fn checkout_without_document_is_its_own_outcome() {
        // The spec's Edge Cases require this be distinguishable from a missing
        // checkout: cloning the wrong repository and cloning nothing are
        // different operator mistakes and get different messages.
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"governance\"\n",
        ));
        fs::create_dir_all(dir.path().join("governance")).unwrap();

        let result = run_in(&dir);
        assert_eq!(result.skipped.len(), 1);
        assert_eq!(
            result.skipped[0].outcome,
            ConstitutionOutcome::NoConstitutionDocument
        );
        assert_ne!(
            result.skipped[0].outcome,
            ConstitutionOutcome::NotCheckedOut
        );
    }

    #[test]
    fn two_entries_sharing_a_path_are_reported_as_duplicates() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/a\"\npath = \"governance\"\n\
             [constitutions.house]\nrepo = \"https://example.test/b\"\npath = \"governance\"\n",
        ));
        let checkout = dir.path().join("governance");
        fs::create_dir_all(&checkout).unwrap();
        fs::write(checkout.join("constitution.md"), "# House rules\n").unwrap();

        let result = run_in(&dir);
        assert_eq!(
            result.loaded.len(),
            2,
            "both aliases resolve; neither errors"
        );
        assert_eq!(result.duplicate_paths.len(), 1);
        assert_eq!(result.duplicate_paths[0].path, "governance");
        assert_eq!(
            result.duplicate_paths[0].aliases,
            vec!["acme".to_string(), "house".to_string()]
        );
    }

    #[test]
    fn multiple_entries_load_in_alias_order() {
        // AC11 (all registered entries load) and AC7 (deterministic order).
        let dir = repo_with_config(Some(
            "[constitutions.zulu]\nrepo = \"https://example.test/z\"\npath = \"z\"\n\
             [constitutions.alpha]\nrepo = \"https://example.test/a\"\npath = \"a\"\n",
        ));
        for name in ["z", "a"] {
            let checkout = dir.path().join(name);
            fs::create_dir_all(&checkout).unwrap();
            fs::write(checkout.join("constitution.md"), "# rules\n").unwrap();
        }

        let result = run_in(&dir);
        assert_eq!(result.examined, 2);
        let aliases: Vec<&str> = result.loaded.iter().map(|r| r.alias.as_str()).collect();
        assert_eq!(
            aliases,
            vec!["alpha", "zulu"],
            "alias order, not TOML order"
        );
    }

    #[test]
    fn mixed_registry_splits_loaded_from_skipped() {
        let dir = repo_with_config(Some(
            "[constitutions.good]\nrepo = \"https://example.test/a\"\npath = \"good\"\n\
             [constitutions.gone]\nrepo = \"https://example.test/b\"\npath = \"gone\"\n",
        ));
        let checkout = dir.path().join("good");
        fs::create_dir_all(&checkout).unwrap();
        fs::write(checkout.join("constitution.md"), "# rules\n").unwrap();

        let result = run_in(&dir);
        assert_eq!(result.examined, 2);
        assert_eq!(result.loaded.len(), 1);
        assert_eq!(result.skipped.len(), 1);
        assert_eq!(result.loaded[0].alias, "good");
        assert_eq!(result.skipped[0].alias, "gone");
    }

    #[test]
    fn malformed_config_is_an_error_not_an_empty_registry() {
        // A config that does not parse must not read as "nothing registered" —
        // that would silently drop every rule the project declared.
        let dir = repo_with_config(Some("[constitutions.acme]\nrepo = \n"));
        let err = run(&ResolveConstitutionsArgs {}, dir.path());
        assert!(err.is_err());
    }

    #[test]
    fn entry_missing_path_is_an_error() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\n",
        ));
        assert!(run(&ResolveConstitutionsArgs {}, dir.path()).is_err());
    }
}
