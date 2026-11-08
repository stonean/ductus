//! `[services]` registry schema from the project config.
//!
//! The config file is whichever tier [`crate::schema::paths`]'s `CONFIG_CHAIN`
//! resolves — `.ductus/config.toml`, `.govern/config.toml`, or the legacy root
//! `.govern.toml`, newest-wins. This module never spells a tier itself.
//!
//! Cross-service references (spec 030) resolve a linked spec's lifecycle
//! status by matching a reference link's repository URL against a registered
//! service's `repo`, then reading the linked spec from that service's local
//! `path`. This module is the pure shape plus parser; file IO and outcome
//! classification live in the `resolve-references` primitive.
//!
//! Schema is canonical in
//! `specs/030-cross-service-references/data-model.md`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Why the `[services]` registry could not be read.
///
/// Kept apart as `ConstitutionsError` keeps its two: a [`Self::Parse`] failure
/// is a document that is not TOML, located by serde's message, and an
/// [`Self::Invalid`] one is an entry that parsed and lacks a value it needs,
/// which nothing but [`Services::validate`] would ever name.
#[derive(Debug, thiserror::Error)]
pub enum ServicesError {
    /// The config document is not valid TOML, or an entry's field holds the
    /// wrong type. Carried through verbatim so the caller can attach the path.
    #[error(transparent)]
    Parse(#[from] toml::de::Error),
    /// An entry's `repo` or `path` is absent or empty. The alias is escaped
    /// for display, as `ConstitutionsError::Invalid`'s is, since a quoted key
    /// may carry a newline (`BE-INPUT-011`).
    #[error("[services.{}] `{field}` {reason}", .alias.escape_debug())]
    Invalid {
        /// The offending entry's registry alias, verbatim.
        alias: String,
        /// Which field was rejected: `repo` or `path`.
        field: String,
        /// One-line description of why, phrased to follow the field name.
        reason: String,
    },
}

/// One `[services.<alias>]` entry.
///
/// `repo` and `path` default to empty on the wire so an absent one reaches
/// [`Services::validate`], which names the alias with the field, where serde's
/// missing-field error names neither the entry nor the reason (`CFG-ENV-003`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceEntry {
    /// Canonical repository URL — the identity matched against a reference
    /// link's href to decide registration.
    #[serde(default)]
    pub repo: String,
    /// Local checkout location (relative to the repo root or absolute) read
    /// for status resolution.
    #[serde(default)]
    pub path: String,
    /// Optional human/agent-facing note on the service's purpose.
    /// Informational only — no runtime behavior depends on it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// The `[services]` table: alias → entry. Empty when the table is absent.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Services(pub BTreeMap<String, ServiceEntry>);

/// Canonical repo identity: trailing slashes and a trailing `.git` removed, so
/// `https://host/o/r/`, `https://host/o/r.git`, and `https://host/o/r` are one
/// service.
///
/// Lives here, beside the registry it identifies, because both the reference
/// harvester and [`Services::duplicate_repos`] must agree on what "the same
/// repo" means. They did not: the harvester normalized and the duplicate
/// detector compared raw strings, so a registry that was ambiguous to one was
/// clean to the other.
#[must_use]
pub fn normalize_repo(repo: &str) -> String {
    let trimmed = repo.trim_end_matches('/');
    trimmed.strip_suffix(".git").unwrap_or(trimmed).to_string()
}

/// Wrapper for extracting just the `[services]` table from the project config.
/// Unknown top-level tables are accepted and ignored.
#[derive(Debug, Default, Deserialize)]
struct ServicesConfig {
    #[serde(default)]
    services: BTreeMap<String, ServiceEntry>,
}

impl Services {
    /// Parse the `[services]` table from project-config contents. An absent
    /// table or an empty document yields an empty registry — never an error.
    ///
    /// # Errors
    ///
    /// Returns [`ServicesError::Parse`] when the document is not valid TOML or
    /// a field holds the wrong type, and [`ServicesError::Invalid`] when an
    /// entry's `repo` or `path` is absent or empty (see [`Self::validate`]).
    pub fn from_toml_str(content: &str) -> std::result::Result<Self, ServicesError> {
        let parsed: ServicesConfig = toml::from_str(content)?;
        let registry = Self(parsed.services);
        registry.validate()?;
        Ok(registry)
    }

    /// Check that every entry names a `repo` and a `path`, returning the
    /// first entry, in alias order, that does not, `repo` before `path`.
    ///
    /// An empty value counts as absent (`CFG-ENV-003`): an empty `repo`
    /// matches no link, so every reference to the service would read as
    /// unregistered, and an empty `path` resolves to the consumer's own
    /// repository, whose specs would then answer for the service's. A `path`
    /// that does not resolve is not checked: that is the `not-checked-out`
    /// outcome, correct for any contributor who has not cloned the service.
    fn validate(&self) -> std::result::Result<(), ServicesError> {
        for (alias, entry) in &self.0 {
            if entry.repo.trim().is_empty() {
                return Err(ServicesError::Invalid {
                    alias: alias.clone(),
                    field: "repo".to_string(),
                    reason: "is absent or empty: it is the identity a reference link's repo \
                             is matched against"
                        .to_string(),
                });
            }
            if entry.path.trim().is_empty() {
                return Err(ServicesError::Invalid {
                    alias: alias.clone(),
                    field: "path".to_string(),
                    reason: "is absent or empty: it names the service's local checkout, and an \
                             empty value resolves to this repository"
                        .to_string(),
                });
            }
        }
        Ok(())
    }

    /// True when no services are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Aliases that share a `repo`, grouped by normalized repo. A duplicate
    /// repo makes link→service matching ambiguous, so callers surface it as a
    /// finding. Returns one `(repo, aliases)` group per repo used by two or
    /// more aliases; output is sorted for determinism.
    ///
    /// Grouping is by [`normalize_repo`] — the same identity the reference
    /// harvester keys its registry on — not by the raw string. Two aliases
    /// whose `repo` values differ only by a trailing `/` or `.git` are **one**
    /// service to the harvester, which silently keeps whichever sorts last, and
    /// were **two** to this detector, which therefore reported no duplicate.
    /// An ambiguous registry then resolved to a confidently wrong status with
    /// no signal anywhere. One identity function is what keeps the two halves
    /// from disagreeing.
    #[must_use]
    pub fn duplicate_repos(&self) -> Vec<(String, Vec<String>)> {
        let mut by_repo: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for (alias, entry) in &self.0 {
            by_repo
                .entry(normalize_repo(&entry.repo))
                .or_default()
                .push(alias.as_str());
        }
        by_repo
            .into_iter()
            .filter(|(_, aliases)| aliases.len() > 1)
            .map(|(repo, aliases)| (repo, aliases.into_iter().map(String::from).collect()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// The shared registry declares every key this reader reads, so none of
    /// them is reported as unknown (spec 019).
    #[test]
    fn the_registry_recognizes_every_key_this_reader_reads() {
        use crate::schema::config_keys::{fields_of, unrecognized};
        assert_eq!(
            unrecognized(&[], fields_of::<ServicesConfig>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["services", "api"], fields_of::<ServiceEntry>()),
            [] as [String; 0]
        );
    }

    #[test]
    fn parses_present_entries() {
        let toml = r#"
[services.api]
repo = "https://github.com/acme/api"
path = "../api"
description = "owns all data models; the system of record"

[services.frontend]
repo = "https://github.com/acme/frontend"
path = "../frontend"
"#;
        let services = Services::from_toml_str(toml).unwrap();
        assert_eq!(services.0.len(), 2);

        let api = services.0.get("api").expect("api entry");
        assert_eq!(api.repo, "https://github.com/acme/api");
        assert_eq!(api.path, "../api");
        assert_eq!(
            api.description.as_deref(),
            Some("owns all data models; the system of record")
        );

        let frontend = services.0.get("frontend").expect("frontend entry");
        assert_eq!(frontend.path, "../frontend");
        assert!(frontend.description.is_none());
    }

    #[test]
    fn absent_table_is_empty() {
        // A `.govern.toml` with other tables but no `[services]`.
        let toml = "[review]\ntech-stack-verified = true\n";
        let services = Services::from_toml_str(toml).unwrap();
        assert!(services.is_empty());
        assert_eq!(services.duplicate_repos(), [] as [(String, Vec<String>); 0]);
    }

    #[test]
    fn empty_document_is_empty() {
        let services = Services::from_toml_str("").unwrap();
        assert!(services.is_empty());
    }

    #[test]
    fn duplicate_repos_detected() {
        let toml = r#"
[services.api]
repo = "https://github.com/acme/api"
path = "../api"

[services.backend]
repo = "https://github.com/acme/api"
path = "../backend"

[services.frontend]
repo = "https://github.com/acme/frontend"
path = "../frontend"
"#;
        let services = Services::from_toml_str(toml).unwrap();
        let dups = services.duplicate_repos();
        assert_eq!(dups.len(), 1, "exactly one repo is shared");
        let (repo, aliases) = &dups[0];
        assert_eq!(repo, "https://github.com/acme/api");
        assert_eq!(aliases, &vec!["api".to_string(), "backend".to_string()]);
    }

    /// Two aliases whose `repo` differs only by a trailing `/` or `.git` are
    /// one service to the reference harvester, which keys its registry on the
    /// normalized URL. Before this grouping normalized too, they were two
    /// entries here and the registry reported no duplicate at all — so an
    /// ambiguous registry resolved to whichever alias sorted last, with
    /// nothing anywhere saying the choice had been made.
    #[test]
    fn duplicate_repos_normalizes_trailing_slash_and_git_suffix() {
        let toml = r#"
[services.api]
repo = "https://github.com/acme/api"
path = "../api"

[services.mirror]
repo = "https://github.com/acme/api/"
path = "../mirror"

[services.zzz]
repo = "https://github.com/acme/api.git"
path = "../zzz"
"#;
        let services = Services::from_toml_str(toml).unwrap();
        let dups = services.duplicate_repos();
        assert_eq!(dups.len(), 1, "all three spellings are one repo");
        let (repo, aliases) = &dups[0];
        assert_eq!(
            repo, "https://github.com/acme/api",
            "grouped under the normalized identity"
        );
        assert_eq!(
            aliases,
            &vec!["api".to_string(), "mirror".to_string(), "zzz".to_string()]
        );
    }

    /// The normalizer is the registry's identity function and is shared with
    /// the reference harvester; both halves must answer the same way.
    #[test]
    fn normalize_repo_collapses_slash_and_git_spellings() {
        assert_eq!(normalize_repo("https://h/o/r/"), "https://h/o/r");
        assert_eq!(normalize_repo("https://h/o/r.git"), "https://h/o/r");
        assert_eq!(normalize_repo("https://h/o/r"), "https://h/o/r");
        assert_eq!(normalize_repo("https://h/o/r.git/"), "https://h/o/r");
    }

    #[test]
    fn distinct_repos_have_no_duplicates() {
        let toml = r#"
[services.api]
repo = "https://github.com/acme/api"
path = "../api"

[services.frontend]
repo = "https://github.com/acme/frontend"
path = "../frontend"
"#;
        let services = Services::from_toml_str(toml).unwrap();
        assert_eq!(services.duplicate_repos(), [] as [(String, Vec<String>); 0]);
    }

    /// An absent or empty `repo` or `path` names the alias and the field
    /// (`CFG-ENV-003`), each of the four cases on its own.
    #[test]
    fn an_absent_or_empty_repo_or_path_names_the_alias_and_the_field() {
        let cases = [
            ("[services.api]\npath = \"../api\"\n", "repo"),
            ("[services.api]\nrepo = \"  \"\npath = \"../api\"\n", "repo"),
            (
                "[services.api]\nrepo = \"https://github.com/acme/api\"\n",
                "path",
            ),
            (
                "[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"\"\n",
                "path",
            ),
        ];
        for (toml, expected) in cases {
            let err = Services::from_toml_str(toml).unwrap_err();
            let ServicesError::Invalid { alias, field, .. } = &err else {
                panic!("expected a value rejection for {toml:?}, got {err}");
            };
            assert_eq!(
                (alias.as_str(), field.as_str()),
                ("api", expected),
                "{toml:?}"
            );
            assert!(err.to_string().starts_with("[services.api] `"), "{err}");
        }
    }

    /// A field of the wrong type is still serde's parse error.
    #[test]
    fn a_field_of_the_wrong_type_is_a_parse_error() {
        let toml = "[services.api]\nrepo = 7\npath = \"../api\"\n";
        assert!(matches!(
            Services::from_toml_str(toml),
            Err(ServicesError::Parse(_))
        ));
    }
}
