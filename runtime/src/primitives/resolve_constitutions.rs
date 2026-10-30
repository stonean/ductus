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
//! **A malformed entry halts; an unresolved checkout warns.** The two are
//! deliberately different severities, stated once in
//! `framework/bootstrap/ductus.md` §Validating the registry and mirrored here
//! rather than restated: a malformed entry is a mistake in the project's own
//! committed config and is always wrong, while a `path` that does not resolve
//! is a machine-local state that is correct for any contributor who has not
//! cloned the governance repository yet — blocking on it would make the
//! pipeline a hard dependency on someone else's repo state. The value checks
//! themselves live in [`crate::schema::constitutions`], inside the parser, so
//! no caller can obtain an unvalidated registry.
//!
//! Results split into `loaded` and `skipped` rather than returning only what
//! worked. That is `QUAL-CLAIM-001` at the primitive layer: a caller reporting
//! a clean result over a source in `skipped` has to drop a field it was handed,
//! instead of relying on remembering to check.
//!
//! **The primitive entry measures; the shared resolution does not.** The
//! MCP tool, the CLI subcommand and the exec walker call [`run_measured`],
//! which adds each loaded document's size against `[artifacts]
//! read-size-bytes` (scenario `target-reports-each-constitutions-size`).
//! `write-review` and `write-analysis` call [`run`], so a malformed read size,
//! which halts the measurement, never newly halts the two records that must
//! be written whatever the config holds.
//!
//! Schema is canonical in `specs/055-shared-constitution/data-model.md`.

use std::io::Write as _;
use std::path::Path;

use crate::primitives::check_artifact_size::resolve_threshold;
use crate::primitives::{PrimitiveError, Result, flatten_line, read_text, rel_path, resolve_path};
use crate::schema::config_keys;
use crate::schema::constitutions::{Constitutions, ConstitutionsError};
use crate::schema::paths;
use crate::schema::primitives::{
    ConstitutionOutcome, ConstitutionRecord, DuplicateConstitutionPath, ResolveConstitutionsArgs,
    ResolveConstitutionsResult,
};

/// The document filename looked for inside a registered checkout. Fixed rather
/// than configurable: a per-entry filename would let two projects registering
/// the same source load different documents from it, which AC7 forbids.
const DOCUMENT_NAME: &str = "constitution.md";

/// Resolve the registry without measuring anything: the resolution
/// `write-review` and `write-analysis` read for what went unexamined. The
/// primitive entry is [`run_measured`].
///
/// # Errors
///
/// Returns [`PrimitiveError::Toml`] when the project config is present but
/// will not parse, [`PrimitiveError::InvalidConstitutionEntry`] when an entry
/// parses but carries a value the schema forbids, or [`PrimitiveError::Io`]
/// when the config exists and cannot be read. Per-entry *resolution* failures
/// are outcomes, not errors — an absent checkout is a state this primitive
/// reports, never a condition it fails on.
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
            bytes: None,
            over_limit: None,
            warning: None,
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
        threshold: None,
    })
}

/// Execute the `resolve-constitutions` primitive: [`run`]'s resolution, with
/// each `loaded` document measured against the read size (spec 055, scenario
/// `target-reports-each-constitutions-size`).
///
/// With nothing registered nothing new is read — the threshold is not
/// resolved and the result is [`run`]'s, with no `threshold` — so a project
/// without the feature reads exactly what it did before (AC1), and a
/// malformed read size halts only a run that has something to measure. The
/// `unmeasured` argument returns [`run`]'s result the same way, for `/ductus`,
/// which writes the imports block and uses no size.
///
/// # Errors
///
/// [`run`]'s errors, and, when an entry is registered,
/// [`PrimitiveError::InvalidReadSize`] / [`PrimitiveError::Toml`] for an
/// invalid read size or a config that does not parse, as
/// `check-artifact-size` reports them, from the one reader of that setting.
pub fn run_measured(
    args: &ResolveConstitutionsArgs,
    repo: &Path,
) -> Result<ResolveConstitutionsResult> {
    let mut result = run(args, repo)?;
    if args.unmeasured || result.examined == 0 {
        return Ok(result);
    }
    let (threshold, notices) = resolve_threshold(repo)?;
    warn(&notices);
    for record in &mut result.loaded {
        measure(repo, record, threshold.bytes);
    }
    result.threshold = Some(threshold);
    Ok(result)
}

/// Write each `[artifacts]` notice to stderr, the channel this primitive
/// names an unknown `[constitutions]` key on, since its result carries no
/// notices. A write that fails is dropped: a warning must never fail the
/// run it warns about.
fn warn(notices: &[String]) {
    let mut stderr = std::io::stderr().lock();
    for notice in notices {
        let _ = writeln!(stderr, "ductus: {notice}");
    }
}

/// Measure one loaded record's document against `limit`; a document at the
/// limit is not over it. One that cannot be opened is left without a size, which
/// `/{project}:target` reports as not measured, never as under the limit
/// (`QUAL-CLAIM-001`). Two aliases on one checkout each measure the same
/// document, so both report its size.
fn measure(repo: &Path, record: &mut ConstitutionRecord, limit: u64) {
    let document = resolve_path(repo, &record.path).join(DOCUMENT_NAME);
    let Some(bytes) = length_on_disk(&document) else {
        return;
    };
    let over = bytes > limit;
    let warning = over.then(|| over_limit_warning(record, bytes, limit));
    record.bytes = Some(bytes);
    record.over_limit = Some(over);
    record.warning = warning;
}

/// A file's length on disk, read from its metadata without its content, as
/// `check-artifact-size` measures a spec artifact: the bytes an agent's
/// reader receives. `None` when it cannot be opened or is not a regular file.
fn length_on_disk(path: &Path) -> Option<u64> {
    std::fs::File::open(path)
        .and_then(|file| file.metadata())
        .ok()
        .filter(std::fs::Metadata::is_file)
        .map(|metadata| metadata.len())
}

/// The warning for a document over the limit. The fix is a cut in the
/// repository that owns it, because nothing in this project can shorten it;
/// that is also why it is reported here and never as an `/{project}:analyze`
/// finding, which a project must be able to fix, route or discard. The path
/// and the URL come from committed config, so each is flattened to keep the
/// warning one line.
fn over_limit_warning(record: &ConstitutionRecord, bytes: u64, limit: u64) -> String {
    let document = record.document.as_deref().unwrap_or(DOCUMENT_NAME);
    format!(
        "shared constitution `{}` (`{}`) is {bytes} bytes, over the {limit}-byte read size, \
         so an agent may not read it in one call. Fix: cut it in its own repository, {}; \
         nothing in this project can shorten it.",
        record.alias,
        flatten_line(document),
        flatten_line(&record.repo),
    )
}

/// Read the `[constitutions]` registry from the project config. An absent
/// config file is an empty registry; a malformed one is an operational error.
///
/// An absent file and an absent table both yield an empty registry, which is
/// what makes AC1 true by construction: nothing downstream can distinguish
/// "no config" from "no entries", so neither can produce a different behavior.
///
/// The two failure kinds are kept apart on the way out. A document that will
/// not parse is [`PrimitiveError::Toml`], whose message is serde's; an entry
/// that parsed and holds a forbidden value is
/// [`PrimitiveError::InvalidConstitutionEntry`], which names the alias and the
/// field because nothing else in the run ever will.
fn load_constitutions(repo: &Path) -> Result<Constitutions> {
    let toml_path = paths::config_path(repo);
    if !toml_path.exists() {
        return Ok(Constitutions::default());
    }
    let content = read_text(&toml_path)?;
    config_keys::warn_unknown_keys(&toml_path, &content, &["constitutions"]);
    Constitutions::from_toml_str(&content).map_err(|source| match source {
        ConstitutionsError::Parse(source) => PrimitiveError::Toml {
            path: toml_path,
            source,
        },
        ConstitutionsError::Invalid {
            alias,
            field,
            reason,
        } => PrimitiveError::InvalidConstitutionEntry {
            path: toml_path,
            alias,
            field,
            reason,
        },
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
    let collapsed = super::collapse_whitespace(description?);
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
/// registered by hand today (`docs/shared-constitution.md`), so the
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
    use crate::schema::primitives::{ReadSizeThreshold, ThresholdSource};
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
        run(&ResolveConstitutionsArgs::default(), dir.path()).unwrap()
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
        assert_eq!(result.loaded, [] as [ConstitutionRecord; 0]);
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
        assert_eq!(result.skipped, [] as [ConstitutionRecord; 0]);

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
        assert_eq!(result.loaded, [] as [ConstitutionRecord; 0]);
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
        let err = run(&ResolveConstitutionsArgs::default(), dir.path());
        assert!(err.is_err());
    }

    #[test]
    fn entry_missing_path_is_an_error() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\n",
        ));
        assert!(run(&ResolveConstitutionsArgs::default(), dir.path()).is_err());
    }

    /// The bug this validation closes, first half. Reproduced against the
    /// `ductus-v0.49.2` release binary: with a resolvable checkout, a `repo`
    /// that is not a URL resolved `loaded` with no complaint at all, so the
    /// operator had nothing to read and nothing to fix.
    #[test]
    fn a_repo_that_is_not_url_shaped_halts_instead_of_resolving_loaded() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"not-a-url-at-all\"\npath = \"gov\"\n",
        ));
        let checkout = dir.path().join("gov");
        fs::create_dir_all(&checkout).unwrap();
        fs::write(checkout.join("constitution.md"), "# House rules\n").unwrap();

        let err = run(&ResolveConstitutionsArgs::default(), dir.path()).expect_err("halts");
        assert!(
            matches!(
                &err,
                PrimitiveError::InvalidConstitutionEntry { alias, field, .. }
                    if alias == "acme" && field == "repo"
            ),
            "names the offending alias and field: {err}"
        );
    }

    /// Second half, and the worse one. `path = ""` resolved to the repository
    /// root — which exists and holds no `constitution.md` — so the run
    /// reported `no-constitution-document`: a **confidently wrong** reason,
    /// telling the operator their checkout lacks the document when the real
    /// mistake is an empty path in their own config. That is the exact
    /// conflation 055's Edge Cases forbid between the two failure states.
    #[test]
    fn an_empty_path_halts_instead_of_blaming_the_checkout() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"\"\n",
        ));
        let err = run(&ResolveConstitutionsArgs::default(), dir.path()).expect_err("halts");
        let PrimitiveError::InvalidConstitutionEntry { alias, field, .. } = &err else {
            panic!("expected a value rejection, got: {err}");
        };
        assert_eq!((alias.as_str(), field.as_str()), ("acme", "path"));
    }

    /// The asymmetry, asserted rather than assumed: a `path` that does not
    /// **resolve** must stay a warning, because it is a machine-local state
    /// that is correct for a contributor who has not cloned the governance
    /// repository yet. Blocking on it would make the pipeline a hard
    /// dependency on someone else's repo state.
    #[test]
    fn a_path_that_does_not_resolve_still_only_warns() {
        let dir = repo_with_config(Some(
            "[constitutions.acme]\n\
             repo = \"https://example.test/g\"\npath = \"../nowhere-at-all\"\n",
        ));
        let result = run_in(&dir);
        assert_eq!(result.examined, 1);
        assert_eq!(
            result.skipped[0].outcome,
            ConstitutionOutcome::NotCheckedOut,
            "an unresolved checkout is a state, never a config error"
        );
    }

    /// A quoted alias is legal TOML and reaches the registry carrying
    /// whitespace; the alias is what every report names the source by.
    #[test]
    fn an_alias_that_is_not_a_bare_toml_key_halts() {
        let dir = repo_with_config(Some(
            "[constitutions.\"my org\"]\nrepo = \"https://example.test/g\"\npath = \"gov\"\n",
        ));
        let err = run(&ResolveConstitutionsArgs::default(), dir.path()).expect_err("halts");
        let PrimitiveError::InvalidConstitutionEntry { alias, field, .. } = &err else {
            panic!("expected a value rejection, got: {err}");
        };
        assert_eq!((alias.as_str(), field.as_str()), ("my org", "alias"));
    }

    /// A document that will not parse and an entry that parses but is invalid
    /// are different operator mistakes, so they stay different variants — the
    /// first carries serde's location, the second names the alias and field.
    #[test]
    fn an_unparseable_config_stays_a_toml_error_not_a_value_rejection() {
        let dir = repo_with_config(Some("[constitutions.acme]\nrepo = \n"));
        let err = run(&ResolveConstitutionsArgs::default(), dir.path()).expect_err("halts");
        assert!(
            matches!(err, PrimitiveError::Toml { .. }),
            "unparseable is not a value rejection: {err}"
        );
    }

    // -- measurement (scenario `target-reports-each-constitutions-size`) -----

    /// A checkout at `name` holding a `constitution.md` of exactly `bytes`
    /// bytes.
    fn checkout_of_size(dir: &TempDir, name: &str, bytes: usize) {
        let checkout = dir.path().join(name);
        fs::create_dir_all(&checkout).unwrap();
        fs::write(checkout.join("constitution.md"), "x".repeat(bytes)).unwrap();
    }

    fn measured_in(dir: &TempDir) -> Result<ResolveConstitutionsResult> {
        run_measured(&ResolveConstitutionsArgs::default(), dir.path())
    }

    /// The scenario's runtime test: a document one byte over the limit is
    /// named as over it, with its owning repository as the fix; one at the
    /// limit is not; and the run continues, resolving both.
    #[test]
    fn a_constitution_one_byte_over_the_limit_is_named_with_its_repository_as_the_fix() {
        let dir = repo_with_config(Some(
            "[artifacts]\nread-size-bytes = 100\n\
             [constitutions.at]\nrepo = \"https://example.test/at\"\npath = \"at\"\n\
             [constitutions.over]\nrepo = \"https://example.test/over\"\npath = \"over\"\n",
        ));
        checkout_of_size(&dir, "at", 100);
        checkout_of_size(&dir, "over", 101);

        let result = measured_in(&dir).expect("an oversized source never halts the run");
        assert_eq!(
            result.threshold,
            Some(ReadSizeThreshold {
                bytes: 100,
                source: ThresholdSource::Config,
            })
        );
        let [at, over] = result.loaded.as_slice() else {
            panic!("both resolve: {result:?}");
        };
        assert_eq!(
            (
                at.alias.as_str(),
                at.bytes,
                at.over_limit,
                at.warning.as_deref()
            ),
            ("at", Some(100), Some(false), None),
            "a document at the limit is not over it"
        );
        assert_eq!(
            (over.alias.as_str(), over.bytes, over.over_limit),
            ("over", Some(101), Some(true))
        );
        assert_eq!(
            over.warning.as_deref(),
            Some(
                "shared constitution `over` (`over/constitution.md`) is 101 bytes, over the \
                 100-byte read size, so an agent may not read it in one call. Fix: cut it in its \
                 own repository, https://example.test/over; nothing in this project can shorten it."
            )
        );
    }

    /// A skipped entry has no document to measure, so it carries no size and
    /// reads as not measured, never as a document under the limit. The limit
    /// is still stated, since something was registered.
    #[test]
    fn a_skipped_entry_carries_no_size() {
        let dir = repo_with_config(Some(
            "[constitutions.empty]\nrepo = \"https://example.test/e\"\npath = \"empty\"\n\
             [constitutions.gone]\nrepo = \"https://example.test/g\"\npath = \"gone\"\n",
        ));
        fs::create_dir_all(dir.path().join("empty")).unwrap();

        let result = measured_in(&dir).unwrap();
        assert_eq!(result.skipped.len(), 2, "{result:?}");
        for record in &result.skipped {
            let json = serde_json::to_value(record).unwrap();
            for key in ["bytes", "over-limit", "warning"] {
                assert!(json.get(key).is_none(), "{key} on a skipped record: {json}");
            }
        }
        assert_eq!(result.threshold.map(|t| t.bytes), Some(50_000));
    }

    /// A loaded document that cannot be opened is not measured: no size and
    /// no verdict, never a document under the limit (`QUAL-CLAIM-001`).
    #[cfg(unix)]
    #[test]
    fn a_loaded_document_that_cannot_be_opened_is_not_measured() {
        use std::os::unix::fs::PermissionsExt;
        let dir = repo_with_config(Some(
            "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"gov\"\n",
        ));
        checkout_of_size(&dir, "gov", 10);
        let document = dir.path().join("gov/constitution.md");
        fs::set_permissions(&document, fs::Permissions::from_mode(0o000)).unwrap();
        if fs::File::open(&document).is_ok() {
            return; // running as root: permissions do not bind
        }
        let result = measured_in(&dir);
        fs::set_permissions(&document, fs::Permissions::from_mode(0o644)).unwrap();
        let result = result.unwrap();
        let [record] = result.loaded.as_slice() else {
            panic!("the document still resolves: {result:?}");
        };
        assert_eq!(
            (record.bytes, record.over_limit, record.warning.as_deref()),
            (None, None, None)
        );
    }

    /// With nothing registered nothing new is read: even a malformed read
    /// size does not halt, and the result is the unmeasured one, with no
    /// `threshold` key (AC1).
    #[test]
    fn with_nothing_registered_the_result_carries_no_new_key() {
        for config in [None, Some("[artifacts]\nread-size-bytes = \"lots\"\n")] {
            let dir = repo_with_config(config);
            let measured = measured_in(&dir).expect("nothing registered, nothing measured");
            assert_eq!(measured, run_in(&dir), "{config:?}");
            let json = serde_json::to_value(&measured).unwrap();
            assert!(json.get("threshold").is_none(), "{config:?}: {json}");
        }
    }

    /// Once something is registered, a malformed read size halts the
    /// measured entry, naming the setting and its value, as it halts
    /// `check-artifact-size`.
    #[test]
    fn a_malformed_read_size_halts_the_measured_entry() {
        let dir = repo_with_config(Some(
            "[artifacts]\nread-size-bytes = \"lots\"\n\
             [constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"gov\"\n",
        ));
        checkout_of_size(&dir, "gov", 10);
        let err = measured_in(&dir).expect_err("halts");
        assert!(
            matches!(
                &err,
                PrimitiveError::InvalidReadSize { setting, value, .. }
                    if setting == "[artifacts] read-size-bytes" && value == "\"lots\""
            ),
            "{err}"
        );
    }

    /// `/ductus` passes `unmeasured`, so a malformed read size that halts the
    /// measured entry never halts it, and nothing is measured (task 15).
    #[test]
    fn an_unmeasured_call_reads_no_read_size() {
        let dir = repo_with_config(Some(
            "[artifacts]\nread-size-bytes = \"lots\"\n\
             [constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"gov\"\n",
        ));
        checkout_of_size(&dir, "gov", 10);
        let args = ResolveConstitutionsArgs { unmeasured: true };
        let result = run_measured(&args, dir.path()).expect("an unmeasured call never halts");
        assert_eq!(result.threshold, None, "{result:?}");
        let [acme] = result.loaded.as_slice() else {
            panic!("the entry still resolves: {result:?}");
        };
        assert_eq!((acme.bytes, acme.over_limit), (None, None), "{acme:?}");
    }

    /// `write-review` and `write-analysis` resolve through [`run`], so a
    /// malformed read size, which halts the measured entry, leaves their
    /// governance view intact: the registered source still reads as loaded,
    /// never as an unreadable registry (`QUAL-DELEG-001`).
    #[test]
    fn write_review_and_write_analysis_keep_the_unmeasured_resolution() {
        let dir = repo_with_config(Some(
            "[artifacts]\nread-size-bytes = 0\n\
             [constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"gov\"\n",
        ));
        checkout_of_size(&dir, "gov", 10);
        let feature = dir.path().join("specs/055-x");
        fs::create_dir_all(&feature).unwrap();
        fs::write(
            feature.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# 055-x\n",
        )
        .unwrap();
        assert!(measured_in(&dir).is_err(), "the measured entry halts here");

        crate::primitives::write_analysis::run(
            &crate::schema::primitives::WriteAnalysisArgs {
                feature: "055-x".into(),
                analyzed_at: "2026-10-29T00:00:00Z".into(),
                analyzed_against: "abc1234".into(),
                ..Default::default()
            },
            dir.path(),
        )
        .unwrap();
        let analysis = fs::read_to_string(feature.join("analysis.md")).unwrap();
        assert!(
            !analysis.contains("constitution-registry-unreadable"),
            "{analysis}"
        );

        crate::primitives::write_review::run(
            &crate::schema::primitives::WriteReviewArgs {
                feature: "055-x".into(),
                reviewed_at: "2026-10-29T00:00:00Z".into(),
                reviewed_against: "abc1234".into(),
                diff_base: "def5678".into(),
                ..Default::default()
            },
            dir.path(),
        )
        .unwrap();
        let review = fs::read_to_string(feature.join("review.md")).unwrap();
        assert!(
            review.contains("## Unexamined governance\n\n*None.*"),
            "{review}"
        );
    }

    /// Two aliases on one checkout measure one document, and both report its
    /// size, each warning naming its own repository.
    #[test]
    fn two_aliases_on_one_checkout_both_report_its_size() {
        let dir = repo_with_config(Some(
            "[artifacts]\nread-size-bytes = 10\n\
             [constitutions.acme]\nrepo = \"https://example.test/a\"\npath = \"gov\"\n\
             [constitutions.house]\nrepo = \"https://example.test/h\"\npath = \"gov\"\n",
        ));
        checkout_of_size(&dir, "gov", 11);

        let result = measured_in(&dir).unwrap();
        let sizes: Vec<(&str, Option<u64>, Option<bool>)> = result
            .loaded
            .iter()
            .map(|record| (record.alias.as_str(), record.bytes, record.over_limit))
            .collect();
        assert_eq!(
            sizes,
            [
                ("acme", Some(11), Some(true)),
                ("house", Some(11), Some(true))
            ]
        );
        assert!(
            result.loaded[1]
                .warning
                .as_deref()
                .is_some_and(|warning| warning.contains("https://example.test/h")),
            "{result:?}"
        );
    }
}
