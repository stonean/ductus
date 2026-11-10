//! `[paths]` block schema from the project config — the configurable spec-root
//! directory name (spec 040).
//!
//! The top-level directory that holds every ductus artifact defaults to
//! `specs`, but an adopter may rename it via the project config:
//!
//! ```toml
//! [paths]
//! specs-root = "governance"
//! ```
//!
//! to avoid collisions with sibling-framework directories (the motivating
//! case is `RSpec`'s `spec/`, one character away from `specs/`). This module is
//! the single source of truth the runtime uses to resolve that name: every
//! primitive that today hardcodes `repo.join("specs")` calls [`specs_dir`]
//! instead, and the default keeps an adopter who never sets the key on exactly
//! today's behavior (`specs`). Per [§runtime-boundary], the project config is
//! the git-tracked source of truth — the runtime reads the resolved name, it does
//! not own it.
//!
//! A missing file, an absent block/key, or an empty value resolves to the
//! default `specs`; a file that exists but does not parse, or a value that
//! [`validate_specs_root`] rejects, halts with an error naming the file
//! (`CFG-ENV-008`), as [`crate::host::Host::load`] does. [`validate_specs_root`]
//! is the shared predicate the `/ductus` configuration prompt and the runtime
//! agree on.
//!
//! [§runtime-boundary]: `framework/constitution.md` §runtime-boundary

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::primitives::{PrimitiveError, Result, read_text_if_present};
use crate::schema::config_keys;

/// Default spec-root directory name when the project config does not configure
/// one. Matches the framework's historical hardcoded value, so an adopter who
/// never sets `[paths] specs-root` sees byte-for-byte identical behavior.
pub const DEFAULT_SPECS_ROOT: &str = "specs";

/// Project config file under the `.ductus/` directory (spec 049).
pub(crate) const CONFIG_FILE: &str = ".ductus/config.toml";

/// Config file under the pre-rename `.govern/` directory (spec 042). Read as a
/// fallback when [`CONFIG_FILE`] is absent.
pub(crate) const LEGACY_DIR_CONFIG_FILE: &str = ".govern/config.toml";

/// Legacy repo-root config filename (pre-042). Read as a fallback when neither
/// directory-scoped file is present, so an adopter who upgrades the binary
/// before re-running the bootstrap is never broken.
pub(crate) const LEGACY_CONFIG_FILE: &str = ".govern.toml";

/// Number of resolution tiers: the current directory, the 042-era one, and
/// the pre-042 repo-root file. Fixed-size so an empty chain cannot be built.
pub(crate) const TIERS: usize = 3;

/// Config locations in resolution order, newest first. Every resolver below
/// walks this one list, so the read path, the write path, and the provenance
/// tag can never disagree about the precedence rule (spec 049).
pub(crate) const CONFIG_CHAIN: [&str; 3] =
    [CONFIG_FILE, LEGACY_DIR_CONFIG_FILE, LEGACY_CONFIG_FILE];

/// Per-contributor session file under `.ductus/` (spec 049). Gitignored. This
/// is the single source of truth for the session path; the `write-session` and
/// `migrate-session-file` primitives and `host.rs` all resolve through the
/// helpers below rather than joining a literal.
pub(crate) const SESSION_FILE: &str = ".ductus/session.toml";

/// Session file under the pre-rename `.govern/` directory (spec 042).
pub(crate) const LEGACY_DIR_SESSION_FILE: &str = ".govern/session.toml";

/// Legacy repo-root session filename (pre-042). Read as a fallback when neither
/// directory-scoped file is present.
pub(crate) const LEGACY_SESSION_FILE: &str = ".govern.session.toml";

/// Directory holding one per-process session target per identity (spec 062).
/// Gitignored, and self-ignoring: the session core writes a `.gitignore`
/// containing `*` into it when it creates it.
pub(crate) const SESSIONS_DIR: &str = ".ductus/sessions";

/// The advisory lock every session-state read-modify-write holds (spec 062,
/// `BE-RACE-001`). The file carries no content; the lock is on its handle.
pub(crate) const SESSIONS_LOCK: &str = ".ductus/sessions/.lock";

/// Session locations in resolution order, newest first. See [`CONFIG_CHAIN`].
pub(crate) const SESSION_CHAIN: [&str; 3] =
    [SESSION_FILE, LEGACY_DIR_SESSION_FILE, LEGACY_SESSION_FILE];

/// Where a project's constitution lives, in resolution order: the copy an
/// adopter's bootstrap installs, then ductus's own source, which ships as that
/// copy. A repository holds one or the other.
pub(crate) const CONSTITUTION_CHAIN: [&str; 2] =
    [".ductus/constitution.md", "framework/constitution.md"];

/// The project's constitution, repo-relative: the first of
/// [`CONSTITUTION_CHAIN`] that is a file, or `None` when neither is. The one
/// resolver for every runtime reader of the constitution, so an adopter and
/// this repository resolve by the same rule.
#[must_use]
pub(crate) fn constitution_path(repo: &Path) -> Option<&'static str> {
    CONSTITUTION_CHAIN
        .into_iter()
        .find(|candidate| repo.join(candidate).is_file())
}

/// Resolve the project config file to *read* for `repo`: the newest location
/// in [`CONFIG_CHAIN`] that exists. New-wins across all three tiers, so a
/// split layout never reads stale content. When none exists the oldest path is
/// returned — a missing file the caller already treats as "config absent" →
/// defaults.
#[must_use]
pub fn config_path(repo: &Path) -> PathBuf {
    // Derived from `config_display_name` so the new-wins choice lives once
    // — the read path and the provenance tag can never disagree on the rule.
    repo.join(config_display_name(repo))
}

/// Repo-relative display name of the resolved config file — the same
/// new-wins choice [`config_path`] makes, as the literal provenance tags
/// render (`.ductus/config.toml` post-rename, `.govern/config.toml` between
/// 042 and 049, the legacy root `.govern.toml` before that). Display-only;
/// readers resolve through [`config_path`].
///
/// A caller that needs **both** the path and the name must call
/// [`resolve_config`] once instead of calling this and [`config_path`]
/// separately — see that function for why.
#[must_use]
pub(crate) fn config_display_name(repo: &Path) -> &'static str {
    newest_existing(repo, &CONFIG_CHAIN)
}

/// The newest entry in `chain` that exists under `repo`, falling back to the
/// oldest when none does. Shared by the config and session read resolvers so
/// the precedence rule is stated once (spec 049).
fn newest_existing(repo: &Path, chain: &[&'static str; TIERS]) -> &'static str {
    // Fixed-size, so "no tiers" is unrepresentable rather than guarded: an
    // `unwrap_or_default()` here would resolve an empty chain to `""`, and
    // `repo.join("")` is the repo root — the worst possible answer to give
    // silently for a state that cannot occur.
    chain
        .iter()
        .find(|rel| repo.join(rel).exists())
        .copied()
        .unwrap_or(chain[TIERS - 1])
}

/// The resolved project config file from a **single** existence probe: the
/// path to read, and the repo-relative name to render as its provenance tag.
///
/// Callers that both read the config and name it must resolve once and reuse
/// the answer. Calling [`config_path`] for the read and [`config_display_name`]
/// for the tag probes the filesystem twice, and a `/ductus` migration landing
/// between the two probes would let them disagree — the primitive would read
/// one file and attribute its contents to the other. One probe cannot
/// straddle the migration (spec 042 review, `BE-RACE-001`).
#[must_use]
pub(crate) fn resolve_config(repo: &Path) -> (PathBuf, &'static str) {
    let display_name = config_display_name(repo);
    (repo.join(display_name), display_name)
}

/// Resolve the session file to *read*: the newest location in
/// [`SESSION_CHAIN`] that exists. New-wins on a split.
#[must_use]
pub fn session_path(repo: &Path) -> PathBuf {
    repo.join(newest_existing(repo, &SESSION_CHAIN))
}

/// Resolve the session file to *write*: the active file — the newest tier that
/// exists, else the newest tier for a fresh project. The bootstrap migration is
/// the sole cutover, so a write never creates a `.ductus/` file while an older
/// one still lingers (spec 042 §Transition and fallback, extended to the third
/// tier by spec 049).
#[must_use]
pub(crate) fn session_path_for_write(repo: &Path) -> PathBuf {
    active_path(repo, &SESSION_CHAIN)
}

/// Resolve the config file to *write*: the active file, same rule as
/// [`session_path_for_write`]. No runtime primitive writes the config file
/// itself — config writes are host-driven (the bootstrap's `[migrations]` /
/// `[project]` writes, `/ductus:review`'s `[review]` flag, and
/// the `merge-managed-block` host-block call, whose target path the caller
/// supplies) — so this resolver is the canonical statement of the rule those
/// callers mirror: a pre-migration write lands on the older file rather than
/// creating a partial `.ductus/config.toml` that new-wins-on-read would let
/// strand the older file's other sections (spec 042 §Transition and
/// fallback).
#[must_use]
pub fn config_path_for_write(repo: &Path) -> PathBuf {
    active_path(repo, &CONFIG_CHAIN)
}

/// Shared active-file resolution for writes: the newest tier in `chain` that
/// exists, defaulting to the newest tier for a fresh project (none present).
/// Identical to [`newest_existing`] except for the empty-chain fallback — a
/// write with nothing on disk targets the *newest* location, while a read with
/// nothing on disk names the *oldest* (the caller treats it as absent).
fn active_path(repo: &Path, chain: &[&'static str; TIERS]) -> PathBuf {
    let existing = chain.iter().find(|rel| repo.join(rel).exists()).copied();
    repo.join(existing.unwrap_or(chain[0]))
}

/// Validate a configured spec-root directory name for well-formedness.
///
/// A well-formed name is a single directory-name segment using only the
/// conservative charset `[A-Za-z0-9_-]` (letters, digits, hyphen, underscore)
/// and is non-empty. This is deliberately stricter than "no separators / no
/// `..`", and the reason it was originally *this* strict is now historical:
/// the shipped bash generators interpolated the name **unescaped** into
/// `grep -E` / awk regexes, where a `.`, `+`, `*`, `(`, … acted as a regex
/// metacharacter (over-matching, or a syntax error that silently dropped a
/// spec). Those generators were promoted to the `derive-dependencies` /
/// `derive-references` primitives (spec 067) and the adopter pre-commit hook
/// now matches the spec root by *shape* rather than interpolating the
/// configured name, so no shell consumer interpolates it any more.
///
/// The strictness is kept rather than relaxed, on the reasons that outlived
/// that one: it rejects a lone `.` (which would resolve the spec-root to the
/// repo root), it keeps the name safe as a literal path component on every
/// platform, and relaxing a validator is a one-way door — an adopter who
/// configures a newly-permitted name cannot be un-configured by a later
/// tightening. See spec 040's review. The predicate is shared so the
/// runtime's resolver ([`Paths::load`]) and the `/ductus`
/// configuration prompt apply the same rule.
///
/// # Errors
///
/// Returns a human-readable reason (suitable for an error message or a
/// configuration-time rejection message) when `name` is malformed.
pub fn validate_specs_root(name: &str) -> std::result::Result<(), String> {
    if name.is_empty() {
        return Err("spec-root name must not be empty".to_owned());
    }
    // A path separator is the most common mistake — give it a specific message
    // before the general charset check.
    if name.contains('/') || name.contains('\\') {
        return Err(format!(
            "spec-root name must not contain a path separator: {name:?}"
        ));
    }
    if let Some(bad) = name
        .chars()
        .find(|c| !matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_'))
    {
        return Err(format!(
            "spec-root name must use only letters, digits, '-', or '_': {name:?} (offending character {bad:?})"
        ));
    }
    Ok(())
}

/// Resolved `[paths]` configuration — currently just the spec-root directory
/// name. Loaded once per primitive invocation via [`Paths::load`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paths {
    /// The spec-root directory name (e.g., `specs` by default, or a
    /// configured value like `governance`). Always well-formed: a malformed
    /// configured value makes [`Paths::load`] return an error.
    pub specs_root: String,
}

impl Paths {
    /// Resolve `[paths]` for `base` by reading `base`'s project config.
    ///
    /// `base` is the directory whose project config governs the layout — the
    /// repo root in the common case, or a cross-service checkout root for
    /// `resolve-references` (each service may configure its own spec-root).
    ///
    /// A missing file, absent `[paths]` table, or absent or empty
    /// `specs-root` yields the default `specs`.
    ///
    /// # Errors
    ///
    /// [`PrimitiveError::Io`] when the config file exists but cannot be read,
    /// [`PrimitiveError::Toml`] when it does not parse, and
    /// [`PrimitiveError::InvalidSpecsRoot`] when `specs-root` fails
    /// [`validate_specs_root`]. Each names the file; none falls back to the
    /// default.
    pub fn load(base: &Path) -> Result<Self> {
        let specs_root =
            Self::load_configured(base)?.unwrap_or_else(|| DEFAULT_SPECS_ROOT.to_owned());
        Ok(Self { specs_root })
    }

    /// Read and validate `[paths] specs-root` from `base`'s project config.
    /// Returns `Ok(None)` (→ caller uses the default) when the file is
    /// missing or the key is absent or empty; a file that does not parse or a
    /// value that fails validation is an error naming the file.
    fn load_configured(base: &Path) -> Result<Option<String>> {
        let toml_path = config_path(base);
        let Some(content) = read_text_if_present(&toml_path)? else {
            return Ok(None);
        };
        config_keys::warn_unknown_keys(&toml_path, &content, &["paths"]);
        let parsed: PathsFile =
            toml::from_str(&content).map_err(|source| PrimitiveError::Toml {
                path: toml_path.clone(),
                source,
            })?;
        let Some(raw) = parsed.paths.and_then(|block| block.specs_root) else {
            return Ok(None);
        };
        let name = raw.trim();
        if name.is_empty() {
            // Empty/whitespace value is treated as unset → default, no error.
            return Ok(None);
        }
        validate_specs_root(name).map_err(|reason| PrimitiveError::InvalidSpecsRoot {
            path: toml_path,
            value: name.to_owned(),
            reason,
        })?;
        Ok(Some(name.to_owned()))
    }
}

/// Resolve the absolute spec-root directory under `base`.
///
/// This is the single replacement for the historical `base.join("specs")`:
/// every runtime primitive that joins a bare feature name (or enumerates the
/// tree) under the spec root calls `specs_dir(base).join(feature)` instead, so
/// the configured root is honored uniformly. With no configuration it returns
/// `base/specs`, preserving today's behavior exactly.
///
/// # Errors
///
/// Whatever [`Paths::load`] returns for a config file that cannot be read or
/// parsed, or a `specs-root` it rejects.
pub fn specs_dir(base: &Path) -> Result<PathBuf> {
    Ok(base.join(Paths::load(base)?.specs_root))
}

#[derive(Deserialize, Default)]
struct PathsFile {
    #[serde(default)]
    paths: Option<PathsBlock>,
}

#[derive(Deserialize, Default)]
struct PathsBlock {
    #[serde(default, rename = "specs-root")]
    specs_root: Option<String>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use tempfile::TempDir;

    /// The shared registry declares every key this reader reads, so none of
    /// them is reported as unknown (spec 019).
    #[test]
    fn the_registry_recognizes_every_key_this_reader_reads() {
        use crate::schema::config_keys::{fields_of, unrecognized};
        assert_eq!(
            unrecognized(&[], fields_of::<PathsFile>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["paths"], fields_of::<PathsBlock>()),
            [] as [String; 0]
        );
    }

    fn tmp_repo() -> TempDir {
        tempfile::Builder::new()
            .prefix("ductus-paths-fixture")
            .tempdir()
            .unwrap()
    }

    fn write_toml(dir: &Path, body: &str) {
        let path = dir.join(CONFIG_FILE);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }

    // --- constitution_path ---------------------------------------------------

    /// An adopter's installed copy wins over a framework source, and a project
    /// holding neither resolves to nothing rather than to a path that is not
    /// there.
    #[test]
    fn constitution_resolves_the_adopter_copy_then_the_framework_source() {
        let dir = tmp_repo();
        assert_eq!(constitution_path(dir.path()), None);
        std::fs::create_dir_all(dir.path().join("framework")).unwrap();
        std::fs::write(dir.path().join("framework/constitution.md"), "# c\n").unwrap();
        assert_eq!(
            constitution_path(dir.path()),
            Some("framework/constitution.md")
        );
        std::fs::create_dir_all(dir.path().join(".ductus")).unwrap();
        std::fs::write(dir.path().join(".ductus/constitution.md"), "# c\n").unwrap();
        assert_eq!(
            constitution_path(dir.path()),
            Some(".ductus/constitution.md")
        );
    }

    // --- validate_specs_root -------------------------------------------------

    #[test]
    fn validate_accepts_well_formed_names() {
        for name in [
            "specs",
            "governance",
            "gov-specs",
            "design",
            "specs_v2",
            "s",
        ] {
            assert!(
                validate_specs_root(name).is_ok(),
                "{name:?} should be accepted"
            );
        }
    }

    #[test]
    fn validate_rejects_empty() {
        assert!(validate_specs_root("").is_err());
    }

    #[test]
    fn validate_rejects_leading_slash() {
        assert!(validate_specs_root("/abs").is_err());
        assert!(validate_specs_root("/").is_err());
    }

    #[test]
    fn validate_rejects_path_separator() {
        assert!(validate_specs_root("a/b").is_err());
        assert!(validate_specs_root("nested/specs").is_err());
        assert!(validate_specs_root("a\\b").is_err());
    }

    #[test]
    fn validate_rejects_dot_dot() {
        assert!(validate_specs_root("..").is_err());
        assert!(validate_specs_root("../escape").is_err());
        assert!(validate_specs_root("a..b").is_err());
    }

    #[test]
    fn validate_rejects_regex_metachars_and_dot() {
        // Characters outside [A-Za-z0-9_-] are rejected. The original reason
        // was the shipped bash generators interpolating the name unescaped
        // into regexes (spec 040 review); those are runtime primitives now
        // (spec 022), and the constraint is retained on the reasons given on
        // `validate_specs_root`. A lone `.` (repo-root) is rejected too.
        for name in [
            ".", "v1.0", "a.b", "spec+s", "spec(s", "spec*s", "a b", "a[b",
        ] {
            assert!(
                validate_specs_root(name).is_err(),
                "{name:?} should be rejected"
            );
        }
    }

    // --- Paths::load / specs_dir: default fallbacks --------------------------

    #[test]
    fn missing_file_defaults_to_specs() {
        let repo = tmp_repo();
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "specs");
        assert_eq!(specs_dir(repo.path()).unwrap(), repo.path().join("specs"));
    }

    #[test]
    fn empty_file_defaults_to_specs() {
        let repo = tmp_repo();
        write_toml(repo.path(), "# empty\n");
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "specs");
    }

    #[test]
    fn paths_block_absent_defaults_to_specs() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[review]\ntech-stack-verified = true\n");
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "specs");
    }

    #[test]
    fn specs_root_key_absent_defaults_to_specs() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths]\nother = \"x\"\n");
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "specs");
    }

    #[test]
    fn empty_value_defaults_to_specs() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths]\nspecs-root = \"\"\n");
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "specs");
    }

    #[test]
    fn whitespace_only_value_defaults_to_specs() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths]\nspecs-root = \"   \"\n");
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "specs");
    }

    // --- Paths::load / specs_dir: configured override ------------------------

    #[test]
    fn configured_value_overrides_default() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths]\nspecs-root = \"governance\"\n");
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "governance");
        assert_eq!(
            specs_dir(repo.path()).unwrap(),
            repo.path().join("governance")
        );
    }

    #[test]
    fn configured_value_is_trimmed() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths]\nspecs-root = \"  governance  \"\n");
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "governance");
    }

    #[test]
    fn coexists_with_other_tables() {
        let repo = tmp_repo();
        write_toml(
            repo.path(),
            "[host]\nproject = \"acme\"\n\n[paths]\nspecs-root = \"design\"\n\n[review]\ntech-stack-verified = true\n",
        );
        assert_eq!(Paths::load(repo.path()).unwrap().specs_root, "design");
    }

    // --- Paths::load: a malformed config halts, naming the file --------------

    #[test]
    fn a_value_outside_the_charset_is_rejected_naming_the_setting_value_and_file() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths]\nspecs-root = \"../escape\"\n");
        let err = Paths::load(repo.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidSpecsRoot { .. }));
        let text = err.to_string();
        assert!(text.contains("[paths] specs-root"), "{text}");
        assert!(text.contains("../escape"), "{text}");
        assert!(text.contains("config.toml"), "{text}");
        assert!(text.contains("remove the key"), "{text}");
    }

    #[test]
    fn a_value_with_a_separator_is_rejected_not_defaulted() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths]\nspecs-root = \"nested/specs\"\n");
        let text = Paths::load(repo.path()).unwrap_err().to_string();
        assert!(text.contains("nested/specs"), "{text}");
        assert!(text.contains("path separator"), "{text}");
        assert!(specs_dir(repo.path()).is_err());
    }

    #[test]
    fn a_document_that_does_not_parse_halts_naming_the_file() {
        let repo = tmp_repo();
        write_toml(repo.path(), "[paths\nbroken");
        let err = Paths::load(repo.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }));
        assert!(err.to_string().contains("config.toml"), "{err}");
    }

    // --- config_path / session_path resolvers (specs 042, 049) --------------

    fn touch(dir: &Path, rel: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(p, "x").unwrap();
    }

    /// Every subset of a three-tier chain, as (present tiers, expected winner).
    /// Read resolvers name the *oldest* tier when nothing exists (the caller
    /// treats a missing file as "absent"); write resolvers name the *newest*
    /// (a fresh project cuts over immediately). Everything else agrees.
    fn subsets(chain: &[&'static str]) -> Vec<(Vec<&'static str>, &'static str)> {
        let mut cases = Vec::new();
        for mask in 1u8..(1 << chain.len()) {
            let present: Vec<&'static str> = chain
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, rel)| *rel)
                .collect();
            // Newest present tier wins: chain is ordered newest-first, so it is
            // the first entry of `present`.
            let expected = present[0];
            cases.push((present, expected));
        }
        cases
    }

    #[test]
    fn config_read_resolves_newest_existing_tier() {
        // nothing present → oldest tier, which the caller reads as "absent"
        let repo = tmp_repo();
        assert_eq!(
            config_path(repo.path()),
            repo.path().join(LEGACY_CONFIG_FILE)
        );

        for (present, expected) in subsets(&CONFIG_CHAIN) {
            let repo = tmp_repo();
            for rel in &present {
                touch(repo.path(), rel);
            }
            assert_eq!(
                config_path(repo.path()),
                repo.path().join(expected),
                "present: {present:?}"
            );
            // The provenance tag must never disagree with the path read.
            assert_eq!(config_display_name(repo.path()), expected);
            let (path, name) = resolve_config(repo.path());
            assert_eq!((path, name), (repo.path().join(expected), expected));
        }
    }

    #[test]
    fn session_read_resolves_newest_existing_tier() {
        let repo = tmp_repo();
        assert_eq!(
            session_path(repo.path()),
            repo.path().join(LEGACY_SESSION_FILE)
        );

        for (present, expected) in subsets(&SESSION_CHAIN) {
            let repo = tmp_repo();
            for rel in &present {
                touch(repo.path(), rel);
            }
            assert_eq!(
                session_path(repo.path()),
                repo.path().join(expected),
                "present: {present:?}"
            );
        }
    }

    #[test]
    fn config_write_targets_active_tier_defaulting_newest() {
        // fresh project (nothing present) → newest, so the migration is the
        // sole cutover and a write never strands a populated older file
        let repo = tmp_repo();
        assert_eq!(
            config_path_for_write(repo.path()),
            repo.path().join(CONFIG_FILE)
        );

        for (present, expected) in subsets(&CONFIG_CHAIN) {
            let repo = tmp_repo();
            for rel in &present {
                touch(repo.path(), rel);
            }
            assert_eq!(
                config_path_for_write(repo.path()),
                repo.path().join(expected),
                "present: {present:?}"
            );
        }
    }

    #[test]
    fn session_write_targets_active_tier_defaulting_newest() {
        let repo = tmp_repo();
        assert_eq!(
            session_path_for_write(repo.path()),
            repo.path().join(SESSION_FILE)
        );

        for (present, expected) in subsets(&SESSION_CHAIN) {
            let repo = tmp_repo();
            for rel in &present {
                touch(repo.path(), rel);
            }
            assert_eq!(
                session_path_for_write(repo.path()),
                repo.path().join(expected),
                "present: {present:?}"
            );
        }
    }

    #[test]
    fn pre_rename_adopter_still_resolves_through_the_middle_tier() {
        // The 049 guarantee: an adopter who upgrades the binary before
        // re-running the bootstrap sits on the 042 layout and must keep
        // working — nothing here may resolve to the un-migrated `.ductus/`.
        let repo = tmp_repo();
        touch(repo.path(), LEGACY_DIR_CONFIG_FILE);
        touch(repo.path(), LEGACY_DIR_SESSION_FILE);

        assert_eq!(
            config_path(repo.path()),
            repo.path().join(LEGACY_DIR_CONFIG_FILE)
        );
        assert_eq!(
            config_path_for_write(repo.path()),
            repo.path().join(LEGACY_DIR_CONFIG_FILE)
        );
        assert_eq!(
            session_path(repo.path()),
            repo.path().join(LEGACY_DIR_SESSION_FILE)
        );
        assert_eq!(
            session_path_for_write(repo.path()),
            repo.path().join(LEGACY_DIR_SESSION_FILE)
        );
    }
}
