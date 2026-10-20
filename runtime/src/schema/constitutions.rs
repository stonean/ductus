//! `[constitutions]` registry schema from `.ductus/config.toml`.
//!
//! A shared constitution (spec 055) is a governance document an organization
//! owns, registered here so every one of its projects loads the same rules
//! instead of retyping them. The registry mirrors [`crate::schema::services`]
//! deliberately: `repo` is identity and navigation only and is **never
//! fetched**, and the local `path` is the only state read. This module is the
//! pure shape, its parser, and the value validation the registry's schema
//! declares; file IO and outcome classification live in the
//! `resolve-constitutions` primitive.
//!
//! **Validation runs inside [`Constitutions::from_toml_str`], not beside it.**
//! A separate `validate()` a caller had to remember to invoke would be the
//! diligence dependency the constitution's §design-principles rejects — and
//! the failure would be silent, since an unvalidated registry resolves
//! perfectly well and simply answers wrongly. Routing it through the one
//! constructor makes a `Constitutions` built from a config document validated
//! by construction.
//!
//! Schema is canonical in `specs/055-shared-constitution/data-model.md`; the
//! severities are canonical in `framework/bootstrap/ductus.md` §Validating the
//! registry, which this module mirrors rather than restates.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Why the `[constitutions]` registry could not be read.
///
/// Two kinds, kept apart because a caller renders them differently and they
/// are different operator mistakes. A [`Self::Parse`] failure is a document
/// that is not TOML, or an entry missing a required field — serde's message
/// already locates it. A [`Self::Invalid`] failure is an entry that parsed
/// cleanly and carries a value the registry's schema forbids, so nothing but
/// this check would ever name it.
#[derive(Debug, thiserror::Error)]
pub enum ConstitutionsError {
    /// The config document is not valid TOML, or an entry omits `repo` or
    /// `path`. Carried through verbatim so the caller can attach the path.
    #[error(transparent)]
    Parse(#[from] toml::de::Error),
    /// An entry parsed but holds a value the schema forbids. Names the alias
    /// and the field, which is what `framework/bootstrap/ductus.md`
    /// §Validating the registry requires of the halt.
    ///
    /// The alias is **escaped** for display. A TOML quoted key may carry a
    /// newline (`[constitutions."a\nb"]`), and that alias is exactly the one
    /// this error reports, since a non-bare key is what the first check
    /// rejects — so rendering it raw would let a config value forge a second
    /// line in an operator-facing message (`BE-INPUT-011`). The `alias` field
    /// itself stays verbatim for callers; only the rendering is escaped, so a
    /// bare alias reads unchanged. This module already takes the same posture
    /// one field over, where `resolve-constitutions` collapses a multi-line
    /// `description` before it can break a single-line report.
    #[error("[constitutions.{}] `{field}` {reason}", .alias.escape_debug())]
    Invalid {
        /// The offending entry's registry alias, verbatim.
        alias: String,
        /// Which field was rejected — `alias`, `repo`, or `path`.
        field: String,
        /// One-line description of why, phrased to follow the field name.
        reason: String,
    },
}

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
    /// Returns [`ConstitutionsError::Parse`] when the document is not valid
    /// TOML or an entry is missing a required field, and
    /// [`ConstitutionsError::Invalid`] when an entry parses but carries a
    /// value the registry's schema forbids — see [`Self::validate`].
    pub fn from_toml_str(content: &str) -> std::result::Result<Self, ConstitutionsError> {
        let parsed: ConstitutionsConfig = toml::from_str(content)?;
        let registry = Self(parsed.constitutions);
        registry.validate()?;
        Ok(registry)
    }

    /// Check every entry's values against the registry's declared schema,
    /// returning the **first** violation in a deterministic order: entries in
    /// alias order, and within an entry `alias`, then `repo`, then `path`.
    /// One message rather than a list, because a malformed entry halts — the
    /// operator fixes it and re-runs, and the next violation surfaces then.
    ///
    /// Scope is exactly what `framework/bootstrap/ductus.md` §Validating the
    /// registry declares malformed, and deliberately **nothing else**. In
    /// particular a `path` that does not *resolve* is not checked here: that
    /// is the `not-checked-out` state, which warns rather than halting,
    /// because it is a machine-local condition that is correct for any
    /// contributor who has not cloned the governance repository yet. The
    /// asymmetry is the point — a malformed entry is a mistake in the
    /// project's own committed config and is always wrong, while an
    /// unresolved checkout is not. Validation is therefore pure value
    /// inspection and touches no filesystem.
    fn validate(&self) -> std::result::Result<(), ConstitutionsError> {
        for (alias, entry) in &self.0 {
            if !is_bare_toml_key(alias) {
                return Err(ConstitutionsError::Invalid {
                    alias: alias.clone(),
                    field: "alias".to_string(),
                    reason: "is not a bare TOML key: letters, digits, hyphens and underscores \
                             only, with no whitespace, dots or quotes"
                        .to_string(),
                });
            }
            if !is_url_shaped(&entry.repo) {
                return Err(ConstitutionsError::Invalid {
                    alias: alias.clone(),
                    field: "repo".to_string(),
                    reason: format!(
                        "is not URL-shaped: a scheme and a host are required, found {:?} \
                         (e.g. https://github.com/acme/governance)",
                        entry.repo
                    ),
                });
            }
            if entry.path.trim().is_empty() {
                return Err(ConstitutionsError::Invalid {
                    alias: alias.clone(),
                    field: "path".to_string(),
                    reason: "is empty: it names the local checkout directory, and an empty \
                             value resolves to the repository root, which then reports the \
                             operator's own config mistake as a missing document"
                        .to_string(),
                });
            }
        }
        Ok(())
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

/// True when `alias` is spellable as a bare TOML key.
///
/// TOML's bare-key grammar is ASCII letters, digits, hyphens and underscores,
/// which is exactly what the registry's schema declares. A quoted key
/// (`[constitutions."my org"]`, `[constitutions."a.b"]`) parses fine and lands
/// here carrying whitespace, a dot, or anything else — legal TOML, and not a
/// legal alias, because the alias is what every report names the source by and
/// a quoted one reads as two fields in a single-line report.
///
/// Empty is rejected: `[constitutions.""]` is a key nothing can refer to.
fn is_bare_toml_key(alias: &str) -> bool {
    !alias.is_empty()
        && alias
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// True when `repo` is URL-shaped — a scheme and a host, which is the whole
/// requirement the schema states.
///
/// Delegates to the URL parser the crate already depends on and already uses
/// for exactly this question (`fetch_archive`'s `validate_fetch_url`), rather
/// than hand-rolling a second one. A hand-rolled scheme/authority split is a
/// fresh copy of a parse the tree already owns and has tested, and the copies
/// drift — the same reasoning §runtime-boundary gives for not reimplementing a
/// primitive in shell. It is also where the subtle cases live: an authority of
/// `:8080` is a port with no host, and a bracketed IPv6 literal carries colons
/// of its own.
///
/// Shape only, never reachability: `repo` is identity and navigation and is
/// **never fetched**, so there is nothing to resolve and nothing that could
/// fail at a distance. `host()` is what carries the *host* half — a URL the
/// parser accepts as cannot-be-a-base (`mailto:someone@example.test`) has a
/// scheme and no host, and is rejected.
///
/// An scp-style git address (`git@github.com:acme/gov.git`) has no scheme and
/// is therefore rejected. That is the schema as declared, not an oversight:
/// the URL form (`ssh://git@github.com/acme/gov.git`) says the same thing and
/// is navigable, which is the field's only job.
fn is_url_shaped(repo: &str) -> bool {
    reqwest::Url::parse(repo).is_ok_and(|parsed| parsed.host().is_some())
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
            unrecognized(&[], fields_of::<ConstitutionsConfig>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["constitutions", "acme"], fields_of::<ConstitutionEntry>()),
            [] as [String; 0]
        );
    }

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
        assert_eq!(registry.duplicate_paths(), [] as [(String, Vec<String>); 0]);
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
        assert_eq!(registry.duplicate_paths(), [] as [(String, Vec<String>); 0]);
    }

    #[test]
    fn missing_required_field_is_error() {
        // `path` omitted — a malformed entry surfaces as a parse error rather
        // than a silently half-resolved entry.
        let toml = "[constitutions.acme]\nrepo = \"https://github.com/acme/governance\"\n";
        let err = Constitutions::from_toml_str(toml).expect_err("missing path");
        assert!(
            matches!(err, ConstitutionsError::Parse(_)),
            "an omitted field is serde's to report, not the value check's: {err}"
        );
    }

    /// Reduce a rejection to `(alias, field)` so a test asserts what the halt
    /// names rather than how it is worded.
    fn reject(toml: &str) -> (String, String) {
        match Constitutions::from_toml_str(toml).expect_err("entry should be rejected") {
            ConstitutionsError::Invalid { alias, field, .. } => (alias, field),
            ConstitutionsError::Parse(err) => {
                panic!("expected a value rejection, got a parse error: {err}")
            }
        }
    }

    /// The first half of the bug reproduced against `ductus-v0.49.2`:
    /// `repo = "not-a-url-at-all"` resolved `loaded` with no complaint.
    #[test]
    fn a_repo_that_is_not_url_shaped_is_rejected() {
        let toml = "[constitutions.acme]\nrepo = \"not-a-url-at-all\"\npath = \"../gov\"\n";
        assert_eq!(reject(toml), ("acme".to_string(), "repo".to_string()));
    }

    /// The second half: `path = ""` resolved to the repository root and was
    /// reported `no-constitution-document` — a confidently wrong reason,
    /// naming the operator's checkout when the mistake is in their config.
    #[test]
    fn an_empty_path_is_rejected() {
        let toml = "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"\"\n";
        assert_eq!(reject(toml), ("acme".to_string(), "path".to_string()));
    }

    /// Whitespace-only is empty for this check. It is the same operator
    /// mistake, and `framework/bootstrap/ductus.md` §Validating the registry
    /// says so in as many words rather than leaving it to be inferred.
    #[test]
    fn a_whitespace_only_path_is_rejected() {
        let toml = "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"   \"\n";
        assert_eq!(reject(toml), ("acme".to_string(), "path".to_string()));
    }

    /// A quoted key is legal TOML and lands here carrying whitespace. The
    /// alias is what every report names the source by, so one carrying a space
    /// reads as two fields in a single-line report.
    #[test]
    fn a_quoted_alias_is_rejected_when_it_is_not_a_bare_key() {
        for alias in [r#""my org""#, r#""a.b""#, r#""acme!""#, r#""""#] {
            let toml = format!(
                "[constitutions.{alias}]\nrepo = \"https://example.test/g\"\npath = \"../gov\"\n"
            );
            let (_, field) = reject(&toml);
            assert_eq!(field, "alias", "alias {alias} should be rejected");
        }
    }

    /// Hyphens and underscores are bare-key characters and stay accepted —
    /// the check refuses what TOML would have to quote, nothing more.
    #[test]
    fn a_bare_alias_with_hyphens_and_underscores_is_accepted() {
        let toml = "[constitutions.acme-platform_2]\n\
                    repo = \"https://example.test/g\"\npath = \"../gov\"\n";
        let registry = Constitutions::from_toml_str(toml).expect("bare key accepted");
        assert!(registry.0.contains_key("acme-platform_2"));
    }

    /// The asymmetry the whole feature turns on: validation is pure value
    /// inspection, so a `path` that does not resolve parses fine and stays the
    /// warning-level `not-checked-out` state the primitive reports.
    #[test]
    fn a_path_that_does_not_resolve_is_not_a_validation_failure() {
        let toml = "[constitutions.acme]\n\
                    repo = \"https://example.test/g\"\npath = \"../nowhere-at-all\"\n";
        assert!(Constitutions::from_toml_str(toml).is_ok());
    }

    #[test]
    fn url_shape_accepts_a_scheme_and_a_host() {
        for repo in [
            "https://github.com/acme/governance",
            "http://example.test",
            "ssh://git@github.com/acme/gov.git",
            "git+ssh://git@example.test/g",
            "https://user:pw@example.test:8443/g",
            "https://[::1]:8080/g",
            "https://[::1]/g",
            // The parser skips the extra slash in an authority position for a
            // special scheme, so this resolves a host of `acme` rather than an
            // empty one. Verified against the parser rather than assumed.
            "https:///acme/gov",
        ] {
            assert!(is_url_shaped(repo), "{repo} is URL-shaped");
        }
    }

    #[test]
    fn url_shape_rejects_what_has_no_scheme_or_no_host() {
        for repo in [
            "not-a-url-at-all",
            "",
            "github.com/acme/governance",
            // scp-style git: no scheme. The URL form says the same thing.
            "git@github.com:acme/gov.git",
            "://example.test",
            "1http://example.test",
            "https://",
            // a port and no host
            "https://:8080/g",
            // a scheme the parser accepts as cannot-be-a-base: no host
            "mailto:someone@example.test",
            // a port with no host, and a non-numeric variant of it
            "https://:abc",
            // `file://` normalises `localhost` away, leaving no host. `repo`
            // is the remote identity; the local half is `path`.
            "file://localhost/srv/gov",
            "file:///srv/gov",
        ] {
            assert!(!is_url_shaped(repo), "{repo} is not URL-shaped");
        }
    }

    /// One message, not a list — a malformed entry halts, so the operator
    /// fixes it and re-runs. Which one is reported must therefore be stable:
    /// alias order, and `alias` before `repo` before `path` within an entry.
    #[test]
    fn the_first_violation_is_reported_in_alias_order() {
        let toml = "[constitutions.zulu]\nrepo = \"nope\"\npath = \"../z\"\n\
                    [constitutions.alpha]\nrepo = \"also-nope\"\npath = \"../a\"\n";
        assert_eq!(reject(toml).0, "alpha", "alias order, not document order");
    }

    /// A TOML quoted key may carry a newline, and a non-bare key is exactly
    /// what the first check rejects — so the offending alias reaches the
    /// message. Rendering it raw would let committed config forge a second
    /// line in an operator-facing error (`BE-INPUT-011`).
    #[test]
    fn a_newline_in_an_alias_cannot_forge_a_line_in_the_message() {
        let toml = "[constitutions.\"a\\nb\"]\n\
                    repo = \"https://example.test/g\"\npath = \"../gov\"\n";
        let err = Constitutions::from_toml_str(toml).expect_err("rejected");
        let ConstitutionsError::Invalid { alias, .. } = &err else {
            panic!("expected a value rejection: {err}");
        };
        assert!(alias.contains('\n'), "the field stays verbatim for callers");
        let message = err.to_string();
        assert!(
            !message.contains('\n'),
            "the rendering is single-line: {message:?}"
        );
        assert!(
            message.contains("a\\nb"),
            "escaped, not dropped: {message:?}"
        );
    }

    #[test]
    fn a_rejection_names_the_alias_and_the_field_in_its_message() {
        let toml = "[constitutions.acme]\nrepo = \"not-a-url-at-all\"\npath = \"../gov\"\n";
        let message = Constitutions::from_toml_str(toml)
            .expect_err("rejected")
            .to_string();
        assert!(message.contains("acme"), "names the alias: {message}");
        assert!(message.contains("repo"), "names the field: {message}");
        assert!(
            message.contains("not-a-url-at-all"),
            "quotes the offending value: {message}"
        );
    }
}
