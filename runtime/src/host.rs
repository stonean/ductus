//! Host config loader — resolves the `{cli-config-dir}` and `{project}`
//! template variables used to locate slash-command files at `ductus exec`
//! time. See spec 022 scenario `commands-dir-parameterization`.
//!
//! The runtime resolves command files at two callsites
//! (`main::run_exec` and `interpreter::payload::locate_command_file`),
//! and since spec 080 also in `tool_loading::AlwaysLoaded::resolve` and
//! `render_help::guide`; the first two used to bake in Claude Code's config-dir name and
//! this repo's slash-command namespace. This module reads `project` from
//! the resolved config file's `[host]` block (team-shared — the
//! slash-command namespace is identical for every contributor) and
//! `cli-config-dir` from the resolved, gitignored, per-contributor session
//! file (teammates may each use a different agent, so the config-dir name
//! must never be committed). Both paths come from [`crate::schema::paths`],
//! which resolves a newest-wins ladder — `.ductus/config.toml` /
//! `.ductus/session.toml` post-049, the `.govern/` tier between 042 and 049,
//! and the legacy root `.govern.toml` / `.govern.session.toml` before that —
//! so this module never spells a tier itself. For adopters predating the
//! session relocation, `cli-config-dir` falls back to the config file's
//! `[host]` value, then to defaults that preserve the framework repo's
//! behavior (`.claude` and the repo directory basename).
//!
//! Each callsite resolves the installed command file via
//! [`Host::command_file_candidates`], which covers the two layouts:
//! `claude-style`'s `commands/` (Claude Code) and `pi`'s flat
//! project-hyphenated `prompts/` (spec 064 — `.pi/prompts/{project}-{name}.md`).

use std::path::Path;

use serde::Deserialize;

use crate::primitives::{PrimitiveError, Result, read_text_if_present};
use crate::schema::config_keys;
use crate::schema::paths;

/// Default `cli-config-dir` when the resolved config file's `[host]` block
/// is missing the key. Matches the framework repo's own layout, so this
/// repo's behavior is unchanged when no `[host]` block is declared.
const DEFAULT_CLI_CONFIG_DIR: &str = ".claude";

/// Last-resort `project` fallback when the repo path has no
/// extractable file-name component (UTF-8-invalid name, root path,
/// trailing `..`). The normal fallback is the repo's directory
/// basename; this constant only fires on the degenerate path shape.
const FALLBACK_PROJECT: &str = "ductus";

/// The `config_dir` of every agent in the bootstrap's Agent Registry
/// (`framework/bootstrap/ductus.md` §Agent Registry). A repository can commit
/// more than one agent's generated command copies — this one commits Claude's
/// and Pi's — so a check that skips generated copies skips every agent's, not
/// only the session's. `agent_config_dirs_match_the_registry` holds this list
/// to the registry.
pub const AGENT_CONFIG_DIRS: &[&str] = &[".claude", ".pi"];

/// Resolved host config — the values the command-resolution callsites
/// need at lookup time. `cli_config_dir` is the host's per-user
/// config-dir name (e.g., `.claude` for Claude Code, `.pi` for
/// Pi); `project` is the slash-command namespace under that dir
/// (e.g., `ductus` in this repo, `acme` for an adopter that set its own).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    /// Host's per-user config-dir name (e.g., `.claude`, `.pi`).
    pub cli_config_dir: String,
    /// Slash-command namespace under the config dir (e.g., `ductus`).
    pub project: String,
}

impl Host {
    /// Load `Host` for `repo`: `project` from the resolved config file's
    /// `[host]` block (team-shared), `cli-config-dir` from the resolved
    /// per-contributor session file, then the config file's `[host]` value.
    /// A value found in neither file takes its default (`.claude` / repo
    /// directory basename).
    ///
    /// A file that does not exist is absent, the ordinary state. A config or
    /// session file that exists but cannot be read or parsed halts with an
    /// error naming it, never a default in its place (`CFG-ENV-008`): a
    /// silently-wrong `cli_config_dir` would surface far away, as a "command
    /// file not found" for an agent the contributor does use. Mismatches
    /// between the resolved values and the on-disk layout surface as the
    /// existing "command file not found" error at lookup time.
    ///
    /// # Errors
    ///
    /// [`PrimitiveError::Io`] or [`PrimitiveError::Toml`], naming the config
    /// or session file that failed.
    pub fn load(repo: &Path) -> Result<Self> {
        let defaults = Self::defaults(repo);
        let host_block = Self::load_host_block(repo)?;
        // `project` is shared across the team — it names the slash-command
        // namespace and is identical for every contributor — so it stays in
        // the committed config file's `[host]` block.
        let project = host_block
            .as_ref()
            .and_then(|b| b.project.clone())
            .unwrap_or(defaults.project);
        // `cli-config-dir` is per-contributor: teammates on one project may
        // each use a different agent (`.claude` / `.pi`), so it must NOT live
        // in committed config. Prefer the gitignored session file; fall back
        // to the config file's `[host]` value for adopters predating the
        // relocation; then the default.
        let cli_config_dir = Self::load_session_cli_config_dir(repo)?
            .or_else(|| host_block.and_then(|b| b.cli_config_dir))
            .unwrap_or(defaults.cli_config_dir);
        Ok(Self {
            cli_config_dir,
            project,
        })
    }

    /// Read the `[host]` block from the resolved config file. `Ok(None)` when
    /// the file is missing or has no `[host]` block; a file that cannot be
    /// read or does not parse is an error naming it.
    fn load_host_block(repo: &Path) -> Result<Option<HostBlock>> {
        let toml_path = paths::config_path(repo);
        let Some(content) = read_text_if_present(&toml_path)? else {
            return Ok(None);
        };
        config_keys::warn_unknown_keys(&toml_path, &content, &["host"]);
        let parsed =
            toml::from_str::<HostFile>(&content).map_err(|source| PrimitiveError::Toml {
                path: toml_path,
                source,
            })?;
        Ok(parsed.host)
    }

    /// Read the per-contributor `cli-config-dir` from the resolved,
    /// gitignored session file. `Ok(None)` when the file is missing (the
    /// ordinary state) or carries no such key, so resolution falls through to
    /// the config file's `[host]` value and then the default. A file that
    /// exists but cannot be read or does not parse is an error naming it,
    /// matching [`Self::load_host_block`] (`CFG-ENV-008`); `write-session`
    /// replaces such a file, which is how `/target` repairs it.
    fn load_session_cli_config_dir(repo: &Path) -> Result<Option<String>> {
        let session_path = paths::session_path(repo);
        let Some(content) = read_text_if_present(&session_path)? else {
            return Ok(None);
        };
        let parsed =
            toml::from_str::<SessionHost>(&content).map_err(|source| PrimitiveError::Toml {
                path: session_path,
                source,
            })?;
        Ok(parsed.cli_config_dir)
    }

    /// Repo-relative paths where an installed slash-command file named
    /// `command_name` may live, in resolution order. Covers the two
    /// command layouts the runtime knows about:
    ///
    /// - `claude-style` (Claude Code) — `{dir}/commands/{project}/<name>.md`
    /// - `pi` — `{dir}/prompts/{project}-<name>.md` (flat project-hyphenated
    ///   prompt templates; spec 064)
    ///
    /// Each adopter installs into exactly one of these (selected by the
    /// agent's registry `layout`), and the directory names are agent-specific
    /// via `cli_config_dir` (`.claude` / `.pi`), so at most one candidate
    /// exists — trying both lets the runtime resolve any supported layout
    /// without knowing which agent wrote the file.
    #[must_use]
    pub fn command_file_candidates(&self, command_name: &str) -> Vec<String> {
        vec![
            format!(
                "{}/commands/{}/{command_name}.md",
                self.cli_config_dir, self.project
            ),
            format!(
                "{}/prompts/{}-{command_name}.md",
                self.cli_config_dir, self.project
            ),
        ]
    }

    fn defaults(repo: &Path) -> Self {
        let project = repo
            .file_name()
            .and_then(|s| s.to_str())
            .map_or_else(|| FALLBACK_PROJECT.to_owned(), str::to_owned);
        Self {
            cli_config_dir: DEFAULT_CLI_CONFIG_DIR.to_owned(),
            project,
        }
    }
}

#[derive(Deserialize, Default)]
struct HostFile {
    #[serde(default)]
    host: Option<HostBlock>,
}

#[derive(Deserialize, Default)]
struct HostBlock {
    #[serde(default, rename = "cli-config-dir")]
    cli_config_dir: Option<String>,
    #[serde(default)]
    project: Option<String>,
}

/// The per-contributor slice of the resolved session file this module reads
/// — the flat top-level `cli-config-dir` key. Other session keys (`feature`,
/// `path`, `set-at`, …) are ignored here; serde drops unknown fields.
#[derive(Deserialize)]
struct SessionHost {
    #[serde(default, rename = "cli-config-dir")]
    cli_config_dir: Option<String>,
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
            unrecognized(&[], fields_of::<HostFile>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["host"], fields_of::<HostBlock>()),
            [] as [String; 0]
        );
    }

    fn tmp_repo(name: &str) -> TempDir {
        tempfile::Builder::new().prefix(name).tempdir().unwrap()
    }

    #[test]
    fn agent_config_dirs_match_the_registry() {
        let bootstrap = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/bootstrap/ductus.md"),
        )
        .unwrap();
        let registry = bootstrap
            .split("\n## Agent Registry\n")
            .nth(1)
            .and_then(|rest| rest.split("\n## ").next())
            .unwrap();
        // The section's first table is the registry (its later subsections
        // carry tables of their own). Data rows only: the header row starts
        // `| `key``, and the delimiter row `| ---`.
        let dirs: Vec<&str> = registry
            .lines()
            .skip_while(|line| !line.starts_with('|'))
            .take_while(|line| line.starts_with('|'))
            .filter(|line| line.starts_with("| `") && !line.starts_with("| `key`"))
            .filter_map(|line| line.split('|').nth(3))
            .map(|cell| cell.trim().trim_matches('`'))
            .collect();
        assert_eq!(dirs, AGENT_CONFIG_DIRS, "§Agent Registry config_dir column");
    }

    #[test]
    fn missing_file_returns_defaults() {
        let repo = tmp_repo("ductus-fixture");
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".claude");
        assert_eq!(
            host.project,
            repo.path().file_name().unwrap().to_str().unwrap()
        );
    }

    #[test]
    fn empty_file_returns_defaults() {
        let repo = tmp_repo("ductus-fixture");
        std::fs::write(repo.path().join(".govern.toml"), "# empty\n").unwrap();
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".claude");
    }

    #[test]
    fn host_block_absent_returns_defaults() {
        let repo = tmp_repo("ductus-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[pins]\n\"foo\" = \"v1\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".claude");
    }

    #[test]
    fn host_block_full_overrides_defaults() {
        let repo = tmp_repo("ductus-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[host]\ncli-config-dir = \".pi\"\nproject = \"acme\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".pi");
        assert_eq!(host.project, "acme");
    }

    #[test]
    fn host_block_partial_uses_defaults_for_missing() {
        let repo = tmp_repo("acme-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[host]\nproject = \"acme\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".claude");
        assert_eq!(host.project, "acme");
    }

    #[test]
    fn session_cli_config_dir_overrides_legacy_ductus_toml() {
        // Per-contributor session file wins for `cli-config-dir`; `project`
        // still comes from the committed `.govern.toml`. This is the team
        // case: the committed config may say `.claude` (or nothing), but a
        // contributor using Pi resolves their own `.pi`.
        let repo = tmp_repo("acme-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[host]\ncli-config-dir = \".claude\"\nproject = \"acme\"\n",
        )
        .unwrap();
        std::fs::write(
            repo.path().join(".govern.session.toml"),
            "feature = \"001-x\"\npath = \"specs/001-x\"\nset-at = \"2026-06-20T00:00:00Z\"\ncli-config-dir = \".pi\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".pi");
        assert_eq!(host.project, "acme");
    }

    #[test]
    fn session_cli_config_dir_used_when_no_legacy_block() {
        let repo = tmp_repo("acme-fixture");
        std::fs::write(
            repo.path().join(".govern.session.toml"),
            "cli-config-dir = \".pi\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".pi");
    }

    #[test]
    fn a_malformed_session_file_halts_naming_it_rather_than_reading_the_config_value() {
        let repo = tmp_repo("acme-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[host]\ncli-config-dir = \".pi\"\n",
        )
        .unwrap();
        std::fs::write(
            repo.path().join(".govern.session.toml"),
            "cli-config-dir = [broken\n",
        )
        .unwrap();
        let err = Host::load(repo.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }), "{err}");
        assert!(err.to_string().contains(".govern.session.toml"), "{err}");
    }

    #[test]
    fn command_file_candidates_cover_claude_style_then_pi() {
        let host = Host {
            cli_config_dir: ".pi".to_owned(),
            project: "acme".to_owned(),
        };
        assert_eq!(
            host.command_file_candidates("specify"),
            vec![
                ".pi/commands/acme/specify.md".to_owned(),
                ".pi/prompts/acme-specify.md".to_owned(),
            ],
            "claude-style `commands/` tried first, then the pi flat shape"
        );
    }

    #[test]
    fn command_file_candidates_pi_shape_from_session_cli_config_dir() {
        let repo = tmp_repo("ductus-pi-fixture");
        std::fs::create_dir_all(repo.path().join(".ductus")).unwrap();
        std::fs::write(
            repo.path().join(".ductus/session.toml"),
            "cli-config-dir = \".pi\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path()).unwrap();
        assert_eq!(host.cli_config_dir, ".pi");
        // The project falls back to the repo directory basename, so the pi
        // candidate names the flat project-hyphenated prompt-template form.
        let project = repo
            .path()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap()
            .to_owned();
        assert_eq!(
            host.command_file_candidates("specify"),
            vec![
                format!(".pi/commands/{project}/specify.md"),
                format!(".pi/prompts/{project}-specify.md"),
            ],
            "pi session identity resolves the flat prompts/ candidate last"
        );
    }

    #[test]
    fn a_malformed_config_halts_naming_the_file_rather_than_using_defaults() {
        let repo = tmp_repo("ductus-fixture");
        std::fs::write(repo.path().join(".govern.toml"), "[host\nbroken").unwrap();
        let err = Host::load(repo.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }), "{err}");
        assert!(err.to_string().contains(".govern.toml"), "{err}");
    }

    /// A file that exists but cannot be read is not an absent one: a directory
    /// where the session file belongs halts naming it.
    #[test]
    fn an_unreadable_session_file_halts_naming_it_rather_than_reading_as_absent() {
        let repo = tmp_repo("ductus-fixture");
        std::fs::create_dir_all(repo.path().join(".ductus/session.toml")).unwrap();
        let err = Host::load(repo.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Io { .. }), "{err}");
        assert!(err.to_string().contains("session.toml"), "{err}");
    }
}
