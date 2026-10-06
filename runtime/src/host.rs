//! Host config loader — resolves the `{cli-config-dir}` and `{project}`
//! template variables used to locate slash-command files at `ductus exec`
//! time. See spec 022 scenario `commands-dir-parameterization`.
//!
//! The runtime resolves command files at two callsites
//! (`main::run_exec` and `interpreter::payload::locate_command_file`),
//! both of which used to bake in Claude Code's config-dir name and
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
//! The two callsites resolve the installed command file via
//! [`Host::command_file_candidates`], which covers the three flat-namespaced
//! layouts: `claude-style`'s `commands/` (Claude, Auggie), `opencode`'s
//! singular `command/`, and `pi`'s flat project-hyphenated `prompts/`
//! (spec 064 — `.pi/prompts/{project}-{name}.md`).

use std::path::Path;

use serde::Deserialize;

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
pub const AGENT_CONFIG_DIRS: &[&str] = &[".claude", ".augment", ".agents", ".opencode", ".pi"];

/// Resolved host config — the values both command-resolution callsites
/// need at lookup time. `cli_config_dir` is the host's per-user
/// config-dir name (e.g., `.claude` for Claude Code, `.augment` for
/// Auggie); `project` is the slash-command namespace under that dir
/// (e.g., `ductus` in this repo, `acme` for an adopter that set its own).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    /// Host's per-user config-dir name (e.g., `.claude`, `.augment`).
    pub cli_config_dir: String,
    /// Slash-command namespace under the config dir (e.g., `ductus`).
    pub project: String,
}

impl Host {
    /// Load `Host` for `repo`: `project` from the resolved config file's
    /// `[host]` block (team-shared), `cli-config-dir` from the resolved
    /// per-contributor session file with a config-file `[host]` fallback.
    /// Returns defaults (`.claude` / repo directory basename) for any value
    /// not found in those sources.
    ///
    /// A malformed config or session file is treated as absent, and **both
    /// cases log a warning to stderr** — command resolution should not fail
    /// because of an unrelated config error, but it must not resolve a
    /// silently-wrong value either.
    /// Mismatches between the resolved values and the on-disk layout surface
    /// as the existing "command file not found" error at lookup time.
    #[must_use]
    pub fn load(repo: &Path) -> Self {
        let defaults = Self::defaults(repo);
        let host_block = Self::load_host_block(repo);
        // `project` is shared across the team — it names the slash-command
        // namespace and is identical for every contributor — so it stays in
        // the committed config file's `[host]` block.
        let project = host_block
            .as_ref()
            .and_then(|b| b.project.clone())
            .unwrap_or(defaults.project);
        // `cli-config-dir` is per-contributor: teammates on one project may
        // each use a different agent (`.claude` / `.augment` / `.opencode` /
        // `.agents`), so it must NOT live in committed config. Prefer the
        // gitignored session file; fall back to the config file's `[host]`
        // value for adopters predating the relocation; then the default.
        let cli_config_dir = Self::load_session_cli_config_dir(repo)
            .or_else(|| host_block.and_then(|b| b.cli_config_dir))
            .unwrap_or(defaults.cli_config_dir);
        Self {
            cli_config_dir,
            project,
        }
    }

    /// Read the `[host]` block from the resolved config file. Returns `None`
    /// when the file is missing, has no `[host]` block, or fails to parse
    /// (a parse error logs to stderr and yields `None` — command resolution
    /// should not fail because of an unrelated config error).
    fn load_host_block(repo: &Path) -> Option<HostBlock> {
        let toml_path = paths::config_path(repo);
        let content = std::fs::read_to_string(&toml_path).ok()?;
        match toml::from_str::<HostFile>(&content) {
            Ok(parsed) => parsed.host,
            Err(err) => {
                eprintln!(
                    "ductus: failed to parse {} for [host] block: {err}; using defaults",
                    toml_path.display()
                );
                None
            }
        }
    }

    /// Read the per-contributor `cli-config-dir` from the resolved,
    /// gitignored session file. Best-effort: a missing or malformed session
    /// file yields `None` so resolution falls through to the config file's
    /// `[host]` value and then the default.
    ///
    /// A **missing** file is silent — not having one is the ordinary state.
    /// A file that exists but does not parse logs to stderr, matching
    /// [`Self::load_host_block`]: both are the same failure class, and
    /// swallowing one of them made a corrupt session file indistinguishable
    /// from an absent one (`QUAL-CLAIM-001`). The caller then resolved a
    /// silently-wrong `cli_config_dir` and the mistake surfaced far away, as
    /// a "command file not found" for an agent the contributor does use.
    fn load_session_cli_config_dir(repo: &Path) -> Option<String> {
        let session_path = paths::session_path(repo);
        let content = std::fs::read_to_string(&session_path).ok()?;
        match toml::from_str::<SessionHost>(&content) {
            Ok(parsed) => parsed.cli_config_dir,
            Err(err) => {
                eprintln!(
                    "ductus: failed to parse {} for cli-config-dir: {err}; using defaults",
                    session_path.display()
                );
                None
            }
        }
    }

    /// Repo-relative paths where an installed slash-command file named
    /// `command_name` may live, in resolution order. Covers the three
    /// flat-namespaced command layouts the runtime knows about:
    ///
    /// - `claude-style` (Claude Code, Auggie) — `{dir}/commands/{project}/<name>.md`
    /// - `opencode` — `{dir}/command/{project}/<name>.md` (singular `command/`)
    /// - `pi` — `{dir}/prompts/{project}-<name>.md` (flat project-hyphenated
    ///   prompt templates; spec 064)
    ///
    /// Each adopter installs into exactly one of these (selected by the
    /// agent's registry `layout`), and the directory names are agent-specific
    /// via `cli_config_dir` (`.claude` / `.augment` / `.opencode` / `.pi`), so
    /// the candidates never more than one exist per layout — trying all three
    /// lets the runtime resolve any supported layout without knowing which
    /// agent wrote the file. The plural form is tried first, then the
    /// singular, so existing claude-style and opencode adopters resolve
    /// exactly as before; the pi shape is appended last, keeping both
    /// pre-existing candidates' relative order untouched (spec 064).
    #[must_use]
    pub fn command_file_candidates(&self, command_name: &str) -> Vec<String> {
        let mut candidates = ["commands", "command"]
            .iter()
            .map(|subdir| {
                format!(
                    "{}/{subdir}/{}/{command_name}.md",
                    self.cli_config_dir, self.project
                )
            })
            .collect::<Vec<_>>();
        candidates.push(format!(
            "{}/prompts/{}-{command_name}.md",
            self.cli_config_dir, self.project
        ));
        candidates
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
        let host = Host::load(repo.path());
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
        let host = Host::load(repo.path());
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
        let host = Host::load(repo.path());
        assert_eq!(host.cli_config_dir, ".claude");
    }

    #[test]
    fn host_block_full_overrides_defaults() {
        let repo = tmp_repo("ductus-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[host]\ncli-config-dir = \".augment\"\nproject = \"acme\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path());
        assert_eq!(host.cli_config_dir, ".augment");
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
        let host = Host::load(repo.path());
        assert_eq!(host.cli_config_dir, ".claude");
        assert_eq!(host.project, "acme");
    }

    #[test]
    fn session_cli_config_dir_overrides_legacy_ductus_toml() {
        // Per-contributor session file wins for `cli-config-dir`; `project`
        // still comes from the committed `.govern.toml`. This is the team
        // case: the committed config may say `.claude` (or nothing), but a
        // contributor using OpenCode resolves their own `.opencode`.
        let repo = tmp_repo("acme-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[host]\ncli-config-dir = \".claude\"\nproject = \"acme\"\n",
        )
        .unwrap();
        std::fs::write(
            repo.path().join(".govern.session.toml"),
            "feature = \"001-x\"\npath = \"specs/001-x\"\nset-at = \"2026-06-20T00:00:00Z\"\ncli-config-dir = \".opencode\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path());
        assert_eq!(host.cli_config_dir, ".opencode");
        assert_eq!(host.project, "acme");
    }

    #[test]
    fn session_cli_config_dir_used_when_no_legacy_block() {
        let repo = tmp_repo("acme-fixture");
        std::fs::write(
            repo.path().join(".govern.session.toml"),
            "cli-config-dir = \".opencode\"\n",
        )
        .unwrap();
        let host = Host::load(repo.path());
        assert_eq!(host.cli_config_dir, ".opencode");
    }

    #[test]
    fn malformed_session_falls_back_to_legacy_then_default() {
        let repo = tmp_repo("acme-fixture");
        std::fs::write(
            repo.path().join(".govern.toml"),
            "[host]\ncli-config-dir = \".augment\"\n",
        )
        .unwrap();
        std::fs::write(
            repo.path().join(".govern.session.toml"),
            "cli-config-dir = [broken\n",
        )
        .unwrap();
        let host = Host::load(repo.path());
        assert_eq!(host.cli_config_dir, ".augment");
    }

    #[test]
    fn command_file_candidates_cover_both_layouts_plural_first() {
        let host = Host {
            cli_config_dir: ".opencode".to_owned(),
            project: "acme".to_owned(),
        };
        assert_eq!(
            host.command_file_candidates("specify"),
            vec![
                ".opencode/commands/acme/specify.md".to_owned(),
                ".opencode/command/acme/specify.md".to_owned(),
                ".opencode/prompts/acme-specify.md".to_owned(),
            ],
            "plural (claude-style) tried first, then singular (opencode), then the pi flat shape appended last"
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
        let host = Host::load(repo.path());
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
                format!(".pi/command/{project}/specify.md"),
                format!(".pi/prompts/{project}-specify.md"),
            ],
            "pi session identity resolves the flat prompts/ candidate last"
        );
    }

    #[test]
    fn malformed_toml_falls_back_to_defaults() {
        let repo = tmp_repo("ductus-fixture");
        std::fs::write(repo.path().join(".govern.toml"), "[host\nbroken").unwrap();
        let host = Host::load(repo.path());
        assert_eq!(host.cli_config_dir, ".claude");
    }
}
