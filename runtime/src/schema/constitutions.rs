//! `[constitutions]` registry schema from `.ductus/config.toml`.
//!
//! A shared constitution (spec 055) is a governance document an organization
//! owns, registered here so every one of its projects loads the same rules
//! instead of retyping them. The registry mirrors [`crate::schema::services`]
//! deliberately: `repo` is identity and navigation only and is **never
//! fetched**, and the local `path` is the only state read. This module is the
//! pure shape plus parser; file IO and outcome classification live in the
//! `resolve-constitutions` primitive.
//!
//! Schema is canonical in `specs/055-shared-constitution/data-model.md`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// One `[constitutions.<alias>]` entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConstitutionEntry {
    /// Canonical repository URL — identity and navigation only. Recorded
    /// verbatim and never fetched; see the spec's Non-Goals.
    pub repo: String,
    /// Local checkout location (relative to the repo root or absolute). The
    /// only state read. `..` is permitted — a sibling checkout is the normal
    /// case, and this is machine-local config, not LLM-supplied input.
    pub path: String,
    /// Optional human/agent-facing note on what the source governs.
    ///
    /// No *resolution* behavior depends on it — it never changes which
    /// documents load, which checkout is read, or how an entry is classified.
    /// It is **not** unread, though: `resolve-constitutions` carries it onto
    /// every [`crate::schema::primitives::ConstitutionRecord`], so it reaches
    /// the surfaces that name a source (`/{project}:target`'s loaded and
    /// skipped reports, `write-review`'s `## Unexamined governance` section).
    /// An alias is a config key someone chose and does not say what a document
    /// governs; this is the field that does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// The `[constitutions]` table: alias → entry. Empty when the table is absent.
///
/// A `BTreeMap` rather than an insertion-ordered map on purpose: iteration is
/// alias order, which is stable across machines and TOML rewrites. AC7 requires
/// two projects with the same entries to load the same documents in the same
/// order anywhere, and alias order delivers that; config order would not
/// survive a reformat.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constitutions(pub BTreeMap<String, ConstitutionEntry>);

/// Wrapper for extracting just the `[constitutions]` table from the config.
/// Unknown top-level tables are accepted and ignored.
#[derive(Debug, Default, Deserialize)]
struct ConstitutionsConfig {
    #[serde(default)]
    constitutions: BTreeMap<String, ConstitutionEntry>,
}

impl Constitutions {
    /// Parse the `[constitutions]` table from config contents. An absent table
    /// or an empty document yields an empty registry — never an error, which is
    /// what makes AC1's "absent and present-but-empty are indistinguishable"
    /// true at the parse layer rather than by a caller's check.
    ///
    /// # Errors
    ///
    /// Returns the underlying [`toml::de::Error`] when the document is not
    /// valid TOML or an entry is missing a required field.
    pub fn from_toml_str(content: &str) -> std::result::Result<Self, toml::de::Error> {
        let parsed: ConstitutionsConfig = toml::from_str(content)?;
        Ok(Self(parsed.constitutions))
    }

    /// True when no constitutions are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Aliases that share a `path`, grouped by path. Two entries naming the
    /// same checkout resolve to the same document, so the caller warns and
    /// loads it once — the spec's Edge Cases settle this as warn-and-allow,
    /// matching `/ductus:link`'s duplicate-`repo` posture. Returns one
    /// `(path, aliases)` group per path used by two or more aliases; output is
    /// sorted for determinism.
    #[must_use]
    pub fn duplicate_paths(&self) -> Vec<(String, Vec<String>)> {
        let mut by_path: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for (alias, entry) in &self.0 {
            by_path
                .entry(entry.path.as_str())
                .or_default()
                .push(alias.as_str());
        }
        by_path
            .into_iter()
            .filter(|(_, aliases)| aliases.len() > 1)
            .map(|(path, aliases)| {
                (
                    path.to_string(),
                    aliases.into_iter().map(String::from).collect(),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn parses_present_entries() {
        let toml = r#"
[constitutions.acme]
repo = "https://github.com/acme/governance"
path = "../governance"
description = "Acme engineering house rules"

[constitutions.platform]
repo = "https://github.com/acme/platform-governance"
path = "../platform-governance"
"#;
        let registry = Constitutions::from_toml_str(toml).unwrap();
        assert_eq!(registry.0.len(), 2);

        let acme = registry.0.get("acme").expect("acme entry");
        assert_eq!(acme.repo, "https://github.com/acme/governance");
        assert_eq!(acme.path, "../governance");
        assert_eq!(
            acme.description.as_deref(),
            Some("Acme engineering house rules")
        );

        let platform = registry.0.get("platform").expect("platform entry");
        assert_eq!(platform.path, "../platform-governance");
        assert!(platform.description.is_none());
    }

    #[test]
    fn absent_table_is_empty() {
        // A config with other tables but no `[constitutions]`.
        let toml = "[review]\ntech-stack-verified = true\n";
        let registry = Constitutions::from_toml_str(toml).unwrap();
        assert!(registry.is_empty());
        assert!(registry.duplicate_paths().is_empty());
    }

    #[test]
    fn empty_document_is_empty() {
        let registry = Constitutions::from_toml_str("").unwrap();
        assert!(registry.is_empty());
    }

    #[test]
    fn empty_table_is_indistinguishable_from_absent() {
        // AC1: the key absent and the table present-but-empty are the same
        // input. Both parse to an empty registry, so no caller can tell them
        // apart — which is what makes "no resolution attempted" uniform.
        let absent =
            Constitutions::from_toml_str("[review]\ntech-stack-verified = true\n").unwrap();
        let empty = Constitutions::from_toml_str("[constitutions]\n").unwrap();
        assert_eq!(absent, empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn iteration_is_alias_order_not_document_order() {
        // AC7: order must not depend on how the TOML happened to be written.
        let written_zyx = r#"
[constitutions.zulu]
repo = "https://example.test/z"
path = "../z"

[constitutions.alpha]
repo = "https://example.test/a"
path = "../a"
"#;
        let registry = Constitutions::from_toml_str(written_zyx).unwrap();
        let order: Vec<&str> = registry.0.keys().map(String::as_str).collect();
        assert_eq!(order, vec!["alpha", "zulu"]);
    }

    #[test]
    fn duplicate_paths_detected() {
        let toml = r#"
[constitutions.acme]
repo = "https://github.com/acme/governance"
path = "../governance"

[constitutions.house]
repo = "https://github.com/acme/governance-mirror"
path = "../governance"

[constitutions.platform]
repo = "https://github.com/acme/platform-governance"
path = "../platform-governance"
"#;
        let registry = Constitutions::from_toml_str(toml).unwrap();
        let dups = registry.duplicate_paths();
        assert_eq!(dups.len(), 1, "exactly one path is shared");
        let (path, aliases) = &dups[0];
        assert_eq!(path, "../governance");
        assert_eq!(aliases, &vec!["acme".to_string(), "house".to_string()]);
    }

    #[test]
    fn distinct_paths_have_no_duplicates() {
        let toml = r#"
[constitutions.acme]
repo = "https://github.com/acme/governance"
path = "../governance"

[constitutions.platform]
repo = "https://github.com/acme/platform-governance"
path = "../platform-governance"
"#;
        let registry = Constitutions::from_toml_str(toml).unwrap();
        assert!(registry.duplicate_paths().is_empty());
    }

    #[test]
    fn missing_required_field_is_error() {
        // `path` omitted — a malformed entry surfaces as a parse error rather
        // than a silently half-resolved entry.
        let toml = "[constitutions.acme]\nrepo = \"https://github.com/acme/governance\"\n";
        assert!(Constitutions::from_toml_str(toml).is_err());
    }
}
