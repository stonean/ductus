//! Destinations in an agent host's own configuration, which the primitives
//! that write or delete project files refuse (spec 022 task 11, the owner's
//! answer (a)).
//!
//! A host auto-approves the runtime's tools, and the seeded shell allow
//! reaches its CLI, so a primitive that writes a host's settings, MCP
//! registration, or a command that grants tools would let an agent change
//! what it may do without the host's prompt. Claude Code protects the same
//! files from the agent's own writes ("Writes to a small set of paths are
//! never auto-approved", code.claude.com/docs/en/permission-modes, Protected
//! paths). 022's plan, Agent reach, states the posture.
//!
//! [`check_destination`] refuses a destination that lies under a host's
//! config directory (the `config_dir` column of the Agent Registry in
//! `framework/bootstrap/ductus.md`) or is an MCP or settings file, at any
//! depth, judged as given, where the write lands and where its links lead,
//! unless it is one of the paths a `/ductus` step writes there: a command or
//! skill file, the `ductus` install file, Antigravity's rules mirror, or Pi's
//! bridge (the §Derived values rows). The project name a row carries is not
//! known here, so a row is matched by its shape.
//!
//! A destination that resolves outside the project is not judged: bounding
//! `apply-manifest`'s roots is task 17's question.

use std::path::{Component, Path};

use super::{PrimitiveError, Result, resolve_symlinks};

/// How a host lays out the files `/ductus` writes in its config directory
/// (the `layout` column of the Agent Registry).
#[derive(Clone, Copy)]
enum Layout {
    /// `commands/<project>/<name>.md` and `commands/ductus.md`.
    ClaudeStyle,
    /// `skills/<project>-<name>/SKILL.md`, `skills/ductus/SKILL.md`, and the
    /// rules mirror `rules/<name>.md`.
    Antigravity,
    /// `command/<project>/<name>.md` and `command/ductus.md`.
    OpenCode,
    /// `prompts/<project>-<name>.md`, `prompts/ductus.md`, and the bridge
    /// `extensions/ductus.ts`.
    Pi,
}

/// Every agent's `config_dir` and its layout.
const CONFIG_DIRS: &[(&str, Layout)] = &[
    (".claude", Layout::ClaudeStyle),
    (".augment", Layout::ClaudeStyle),
    (".agents", Layout::Antigravity),
    (".opencode", Layout::OpenCode),
    (".pi", Layout::Pi),
];

/// Files that register an MCP server or carry a host's permissions outside a
/// config directory: Claude Code's `.mcp.json`, and the root config `OpenCode` reads,
/// which holds both its `mcp` block and its `permission` map.
const CONFIG_FILES: &[&str] = &[".mcp.json", "opencode.json", "opencode.jsonc"];

/// Which of the permitted rows a destination is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostRow {
    /// A command, skill or prompt file, or the `ductus` install file. Its
    /// frontmatter is what a host reads to grant tools, so `apply-manifest`
    /// checks it ([`check_frontmatter`]).
    CommandOrSkill,
    /// Antigravity's mirror of a rule file.
    RulesMirror,
    /// Pi's extension bridge.
    PiBridge,
}

/// The frontmatter keys the shipped command, skill and installer files carry:
/// `description`, `argument-hint` and `parity` in `framework/commands/`,
/// `framework/bootstrap/configure/` and `ductus.md`, and `name` in the skill
/// wrapper Antigravity's layout writes. A key such as `allowed-tools` or
/// `hooks` would make the file grant tools or run commands
/// (code.claude.com/docs/en/skills), so no other key is written.
const SHIPPED_FRONTMATTER_KEYS: &[&str] = &["description", "argument-hint", "parity", "name"];

/// Judge `dest`, an absolute path a primitive is about to write or delete.
///
/// Three paths are judged, and each must pass:
///
/// - the path as given, under the project root, since a host reads through
///   any link on it (`.claude/commands` linked elsewhere is still where
///   Claude Code reads commands);
/// - where the write lands, the parent resolved and the final name as given,
///   since a tempfile-and-rename write replaces a link there rather than
///   following it;
/// - where the path leads, every link on it followed, unless the destination
///   is itself a link, which the write replaces. A file system resolves a
///   name an existing file answers to, such as a Windows 8.3 short name, to
///   the file's own name here.
///
/// Each is judged inside the project only. Returns `Ok(None)` when none lies
/// in a host's configuration, and `Ok(Some(row))` when one is a row a
/// `/ductus` step writes.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`], naming `shown`, for any other
/// destination in a host's configuration:
///
/// - under a config directory at any depth (a host reads the one where its
///   session starts, and a manifest's target root may be a subdirectory), a
///   row being matched under the first config directory on the path;
/// - an MCP or settings file at any depth, or the file the project root's
///   `.mcp.json` or `opencode.json(c)` links to;
/// - under the directory the project root's config directory links to
///   (`.claude -> cfg` makes `cfg/settings.local.json` the host's), unless
///   that directory holds the project itself.
///
/// Names and the paths links lead to compare after [`fold`], since a
/// case-insensitive file system finds `.mcp.json` under `.MCP.JSON` and, on
/// macOS, `.mcp.jſon`.
///
/// Not judged: a project file that a link *inside* a config directory points
/// at, when it is named by its own path (`.claude/settings.local.json ->
/// shared/s.json` leaves `shared/s.json` writable). [`PrimitiveError::Io`]
/// when the project root or the destination cannot be resolved.
pub(crate) fn check_destination(repo: &Path, dest: &Path, shown: &str) -> Result<Option<HostRow>> {
    let resolve = |p: &Path| {
        resolve_symlinks(p).map_err(|source| PrimitiveError::Io {
            path: p.into(),
            source,
        })
    };
    let canonical_repo = resolve(repo)?;
    let mut verdicts = Vec::with_capacity(3);
    if let Some(names) = given_names(repo, &canonical_repo, dest) {
        verdicts.push(judge_names(&names, shown)?);
    }
    let landing = match (dest.parent(), dest.file_name()) {
        (Some(parent), Some(name)) => resolve(parent)?.join(name),
        _ => resolve(dest)?,
    };
    verdicts.push(judge(repo, &canonical_repo, &landing, shown)?);
    if !std::fs::symlink_metadata(dest).is_ok_and(|meta| meta.file_type().is_symlink()) {
        verdicts.push(judge(repo, &canonical_repo, &resolve(dest)?, shown)?);
    }
    Ok(verdicts.into_iter().flatten().next())
}

/// `dest`'s names under the project root as written, when it is spelled
/// under the root (as given or canonical) with no `..`.
fn given_names(repo: &Path, canonical_repo: &Path, dest: &Path) -> Option<Vec<String>> {
    let rel = dest
        .strip_prefix(repo)
        .or_else(|_| dest.strip_prefix(canonical_repo))
        .ok()?;
    if rel.components().any(|c| matches!(c, Component::ParentDir)) {
        return None;
    }
    Some(names_of(rel))
}

/// A resolved path's verdict, as [`check_destination`] describes.
fn judge(repo: &Path, canonical_repo: &Path, path: &Path, shown: &str) -> Result<Option<HostRow>> {
    let Some(names) = strip_folded(path, canonical_repo) else {
        return Ok(None);
    };
    // A config directory a link redirects: `.claude -> cfg` makes
    // `cfg/settings.local.json` the file the host reads.
    for &(dir, layout) in CONFIG_DIRS {
        let Ok(linked) = resolve_symlinks(&repo.join(dir)) else {
            continue;
        };
        if canonical_repo.starts_with(&linked) {
            continue;
        }
        if let Some(rest) = strip_folded(path, &linked) {
            return row(layout, &rest)
                .map(Some)
                .ok_or_else(|| refuse(shown, dir));
        }
    }
    for &file in CONFIG_FILES {
        if let Ok(linked) = resolve_symlinks(&repo.join(file))
            && strip_folded(path, &linked).is_some_and(|rest| rest.is_empty())
        {
            return Err(refuse_file(shown, file));
        }
    }
    judge_names(&names, shown)
}

/// The verdict on a path's names under the project root: an MCP or settings
/// file, or a config directory with the row under it.
fn judge_names(names: &[String], shown: &str) -> Result<Option<HostRow>> {
    if let Some(last) = names.last()
        && let Some(&file) = CONFIG_FILES.iter().find(|f| fold(last) == **f)
    {
        return Err(refuse_file(shown, file));
    }
    for (depth, name) in names.iter().enumerate() {
        if let Some(&(dir, layout)) = CONFIG_DIRS.iter().find(|(dir, _)| fold(name) == *dir) {
            return row(layout, &names[depth + 1..])
                .map(Some)
                .ok_or_else(|| refuse(shown, dir));
        }
    }
    Ok(None)
}

/// `path` less `prefix`, compared component by component after [`fold`]:
/// the rest of `path`'s names, or `None` when `prefix` does not begin it. A
/// link to a directory that does not exist yet keeps the case it was written
/// in, while the file system finds it under any case.
fn strip_folded(path: &Path, prefix: &Path) -> Option<Vec<String>> {
    let mut rest = path.components();
    for want in prefix.components() {
        let got = rest.next()?;
        if fold(&got.as_os_str().to_string_lossy()) != fold(&want.as_os_str().to_string_lossy()) {
            return None;
        }
    }
    Some(names_of(rest.as_path()))
}

/// The normal components of `path`, as strings.
fn names_of(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|c| match c {
            Component::Normal(n) => Some(n.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
}

/// A name as a file system that is not case-sensitive may treat it, for
/// comparing with the lowercase ASCII config names: the part before any `:`
/// (an NTFS stream, `.mcp.json::$DATA`), less trailing dots and spaces, which
/// Windows drops, then case-converted to upper case and back. That maps `ſ`
/// to `s`, `K` (the Kelvin sign) to `k` and `ﬁ` to `fi`; of the characters
/// APFS's case-insensitive lookup treats as an ASCII letter, `ſ` is the one a
/// config name can hold (`.mcp.jſon`). It also maps `ı` to `i`, which APFS
/// does not, so such a name is refused without being an alias there.
fn fold(name: &str) -> String {
    let base = name.split(':').next().unwrap_or(name);
    base.trim_end_matches(['.', ' '])
        .to_uppercase()
        .to_lowercase()
}

/// The row `rest`, the path under a config directory, is, if any. Rows match
/// exactly, letter case included, since `/ductus` writes them in one case: a
/// path deeper than its row is not one.
fn row(layout: Layout, rest: &[String]) -> Option<HostRow> {
    let parts: Vec<&str> = rest.iter().map(String::as_str).collect();
    let md = |name: &str| Path::new(name).extension().is_some_and(|ext| ext == "md");
    match (layout, parts.as_slice()) {
        (Layout::ClaudeStyle, ["commands", "ductus.md"])
        | (Layout::OpenCode, ["command", "ductus.md"])
        | (Layout::Antigravity, ["skills", _, "SKILL.md"]) => Some(HostRow::CommandOrSkill),
        (Layout::ClaudeStyle, ["commands", _, name])
        | (Layout::OpenCode, ["command", _, name])
        | (Layout::Pi, ["prompts", name])
            if md(name) =>
        {
            Some(HostRow::CommandOrSkill)
        }
        (Layout::Antigravity, ["rules", name]) if md(name) => Some(HostRow::RulesMirror),
        (Layout::Pi, ["extensions", "ductus.ts"]) => Some(HostRow::PiBridge),
        _ => None,
    }
}

/// Refuse a `migrate-session-file` `legacy-path` that is not
/// `<config-dir>/<name>-session.json`, the file the `session-file-consolidate`
/// migration names (`{config_dir}/{project}-session.json`). The primitive
/// deletes the file, so this is its exception to [`check_destination`]: the
/// legacy file lies in a config directory, and nothing else may. Judged on
/// the path as given; `remove_file` on a link removes the link.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] naming `legacy`.
pub(crate) fn check_legacy_session_path(legacy: &str) -> Result<()> {
    const SUFFIX: &str = "-session.json";
    let parts: Vec<&str> = legacy.split(['/', '\\']).collect();
    let shaped = matches!(
        parts.as_slice(),
        [dir, name] if CONFIG_DIRS.iter().any(|(d, _)| d == dir)
            && name.len() > SUFFIX.len()
            && name.ends_with(SUFFIX)
    );
    if shaped {
        Ok(())
    } else {
        Err(PrimitiveError::InvalidPath {
            path: legacy.into(),
            reason: format!(
                "is not a legacy session file `<config-dir>/<name>{SUFFIX}`, the only file this \
                 primitive deletes (spec 022 task 11)"
            ),
        })
    }
}

fn refuse(shown: &str, dir: &str) -> PrimitiveError {
    PrimitiveError::InvalidPath {
        path: shown.into(),
        reason: format!(
            "resolves into the agent host configuration directory `{dir}`, where only the \
             command, skill, rules-mirror and bridge files `/ductus` writes are permitted \
             (spec 022 task 11)"
        ),
    }
}

fn refuse_file(shown: &str, file: &str) -> PrimitiveError {
    PrimitiveError::InvalidPath {
        path: shown.into(),
        reason: format!(
            "resolves to `{file}`, an agent host's MCP or permission configuration, which a \
             ductus primitive does not write (spec 022 task 11)"
        ),
    }
}

/// Refuse `bytes`, the content about to be written to a command, skill or
/// prompt file, when its frontmatter carries a key the shipped files do not
/// ([`SHIPPED_FRONTMATTER_KEYS`]).
///
/// Frontmatter is a block whose first line, after an optional byte-order
/// mark, is `---`, closed by a later `---` line; a host reads it "only when
/// the opening `---` is the file's first line" (code.claude.com/docs/en/skills),
/// so a file that does not open that way carries none. A block that is not
/// closed, does not parse as YAML, is not a mapping, or is in a file that is
/// not UTF-8 is refused: the check cannot say what a host would read there.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] naming `primitive`, the `entries`
/// argument and `shown`.
pub(crate) fn check_frontmatter(primitive: &str, shown: &str, bytes: &[u8]) -> Result<()> {
    let refuse = |why: String| PrimitiveError::InvalidArgument {
        primitive: primitive.into(),
        argument: "entries".into(),
        reason: format!("{shown}: {why} (spec 022 task 11)"),
    };
    let Ok(text) = std::str::from_utf8(bytes) else {
        return Err(refuse(
            "a command or skill file must be UTF-8 so its frontmatter can be checked".into(),
        ));
    };
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = text.split('\n');
    let is_fence = |line: &str| line.trim_end_matches('\r').trim_end() == "---";
    if !lines.next().is_some_and(is_fence) {
        return Ok(());
    }
    let mut block = Vec::new();
    let mut closed = false;
    for line in lines {
        if is_fence(line) {
            closed = true;
            break;
        }
        block.push(line.trim_end_matches('\r'));
    }
    if !closed {
        return Err(refuse("its frontmatter is not closed".into()));
    }
    let value: serde_norway::Value = serde_norway::from_str(&block.join("\n"))
        .map_err(|err| refuse(format!("its frontmatter does not parse as YAML: {err}")))?;
    let mapping = match value {
        serde_norway::Value::Null => return Ok(()),
        serde_norway::Value::Mapping(mapping) => mapping,
        _ => return Err(refuse("its frontmatter is not a mapping".into())),
    };
    for key in mapping.keys() {
        let name = key.as_str().unwrap_or_default();
        if !SHIPPED_FRONTMATTER_KEYS.contains(&name) {
            return Err(refuse(format!(
                "its frontmatter carries `{}`, and a command or skill file ductus writes \
                 carries only {}",
                key.as_str()
                    .map_or_else(|| format!("{key:?}"), str::to_string),
                SHIPPED_FRONTMATTER_KEYS
                    .iter()
                    .map(|k| format!("`{k}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn judge(repo: &Path, rel: &str) -> Result<Option<HostRow>> {
        check_destination(repo, &repo.join(rel), rel)
    }

    fn refused(result: &Result<Option<HostRow>>) -> bool {
        matches!(result, Err(PrimitiveError::InvalidPath { reason, .. }) if reason.contains("task 11"))
    }

    /// Every row a `/ductus` step writes under a config directory is
    /// permitted, and names its kind.
    #[test]
    fn the_rows_ductus_writes_are_permitted() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        for (rel, kind) in [
            (".claude/commands/acme/status.md", HostRow::CommandOrSkill),
            (".claude/commands/ductus.md", HostRow::CommandOrSkill),
            (".augment/commands/acme/status.md", HostRow::CommandOrSkill),
            (".augment/commands/ductus.md", HostRow::CommandOrSkill),
            (
                ".agents/skills/acme-status/SKILL.md",
                HostRow::CommandOrSkill,
            ),
            (".agents/skills/ductus/SKILL.md", HostRow::CommandOrSkill),
            (".agents/rules/security-backend.md", HostRow::RulesMirror),
            (".opencode/command/acme/status.md", HostRow::CommandOrSkill),
            (".opencode/command/ductus.md", HostRow::CommandOrSkill),
            (".pi/prompts/acme-status.md", HostRow::CommandOrSkill),
            (".pi/prompts/ductus.md", HostRow::CommandOrSkill),
            (".pi/extensions/ductus.ts", HostRow::PiBridge),
            // Under a manifest target root below the project root.
            (
                "project/.claude/commands/ductus.md",
                HostRow::CommandOrSkill,
            ),
            (
                "sub/.claude/commands/acme/status.md",
                HostRow::CommandOrSkill,
            ),
        ] {
            assert_eq!(judge(repo, rel).unwrap(), Some(kind), "{rel}");
        }
    }

    /// A destination outside every host's configuration is not judged, and
    /// neither is one outside the project (task 17's question).
    #[test]
    fn other_destinations_are_not_judged() {
        let tmp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        for rel in [
            "specs/001-x/spec.md",
            ".githooks/pre-commit",
            ".ductus/config.toml",
            "CLAUDE.md",
            "AGENTS.md",
            ".gitignore",
            "claude/settings.json",
        ] {
            assert_eq!(judge(repo, rel).unwrap(), None, "{rel}");
        }
        let elsewhere = outside.path().join(".claude/settings.json");
        assert_eq!(
            check_destination(repo, &elsewhere, "x").unwrap(),
            None,
            "outside the project"
        );
    }

    /// Settings, MCP registrations, and any other file under a config
    /// directory are refused: at the top level, nested (a host started in a
    /// subdirectory reads that one), in any letter case, and at a row's
    /// depth plus one.
    #[test]
    fn host_configuration_outside_the_rows_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        for rel in [
            ".claude/settings.local.json",
            ".claude/settings.json",
            ".claude/agents/reviewer.md",
            ".claude/skills/deploy/SKILL.md",
            ".claude/commands/acme/nested/status.md",
            ".claude/commands/acme/status.txt",
            ".claude/commands/acme/.md",
            ".augment/settings.json",
            ".agents/settings.json",
            ".agents/skills/acme-status/helper.sh",
            ".opencode/opencode.json",
            ".opencode/commands/acme/status.md",
            ".pi/settings.json",
            ".pi/extensions/other.ts",
            ".mcp.json",
            "opencode.json",
            "opencode.jsonc",
            "sub/.mcp.json",
            "sub/.claude/settings.json",
            "sub/.claude/commands/acme/.claude/settings.json",
            ".claude/commands/acme/.agents/rules/r.md",
            ".CLAUDE/settings.local.json",
            ".Mcp.Json",
        ] {
            assert!(refused(&judge(repo, rel)), "{rel}: {:?}", judge(repo, rel));
        }
    }

    /// A link is resolved before judging: a config directory that links
    /// elsewhere in the project makes that directory the host's, a root file
    /// that links elsewhere makes its target the host's, and a path through
    /// another link into a config directory is judged where it lands.
    #[cfg(unix)]
    #[test]
    fn links_are_resolved_before_judging() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join("cfg")).unwrap();
        std::os::unix::fs::symlink(repo.join("cfg"), repo.join(".claude")).unwrap();
        assert!(refused(&judge(repo, "cfg/settings.local.json")));
        assert_eq!(
            judge(repo, "cfg/commands/acme/status.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );

        std::fs::write(repo.join("cfg-mcp.json"), "{}").unwrap();
        std::os::unix::fs::symlink(repo.join("cfg-mcp.json"), repo.join(".mcp.json")).unwrap();
        assert!(refused(&judge(repo, "cfg-mcp.json")));

        std::fs::create_dir_all(repo.join(".augment")).unwrap();
        std::os::unix::fs::symlink(repo.join(".augment"), repo.join("aside")).unwrap();
        assert!(refused(&judge(repo, "aside/settings.json")));
    }

    #[test]
    fn shipped_frontmatter_passes() {
        for text in [
            "---\ndescription: Status.\nargument-hint: \"[--all]\"\n---\n\nBody\n",
            "---\nname: acme-status\ndescription: Status.\n---\nBody\n",
            "---\ndescription: Analyze.\nparity:\n  semantic-fields:\n    - \"findings[].message\"\n---\n",
            "---\n---\nEmpty block\n",
            "No frontmatter at all\n",
            "\n---\nallowed-tools: Bash\n---\nNot on the first line, so no host reads it\n",
        ] {
            check_frontmatter("apply-manifest", "x.md", text.as_bytes()).unwrap();
        }
    }

    /// A key the shipped files do not carry is refused however it is
    /// spelled: block or flow style, after a byte-order mark, with CRLF
    /// endings, or as a merge key.
    #[test]
    fn granting_frontmatter_is_refused() {
        for text in [
            "---\ndescription: x\nallowed-tools: Bash(*)\n---\n",
            "---\nhooks:\n  PreToolUse: []\n---\n",
            "---\n{description: x, allowed-tools: Bash}\n---\n",
            "\u{feff}---\nallowed-tools: Bash\n---\n",
            "---\r\nallowed-tools: Bash\r\n---\r\n",
            "--- \nallowed-tools: Bash\n---\n",
            "---\n<<: {allowed-tools: Bash}\n---\n",
            "---\ndisable-model-invocation: false\n---\n",
        ] {
            let err = check_frontmatter("apply-manifest", "x.md", text.as_bytes()).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidArgument { reason, .. } if reason.contains("x.md")),
                "{text:?}: {err}"
            );
        }
    }

    /// What the check cannot read is refused rather than passed.
    #[test]
    fn unreadable_frontmatter_is_refused() {
        for bytes in [
            b"---\ndescription: x\n".as_slice(),
            b"---\n- a list\n---\n".as_slice(),
            b"---\n: : :\n---\n".as_slice(),
            b"---\ndescription: \xff\n---\n".as_slice(),
        ] {
            assert!(
                check_frontmatter("apply-manifest", "x.md", bytes).is_err(),
                "{bytes:?}"
            );
        }
    }

    #[test]
    fn only_a_legacy_session_file_may_be_migrated() {
        for ok in [
            ".claude/gov-session.json",
            ".augment/acme-session.json",
            ".claude\\gov-session.json",
        ] {
            check_legacy_session_path(ok).unwrap();
        }
        for bad in [
            ".claude/settings.local.json",
            ".claude/-session.json",
            ".claude/sub/gov-session.json",
            "gov-session.json",
            "link/gov-session.json",
            ".claude/gov-session.json.bak",
            "",
        ] {
            assert!(
                matches!(
                    check_legacy_session_path(bad),
                    Err(PrimitiveError::InvalidPath { .. })
                ),
                "{bad}"
            );
        }
    }

    /// A write replaces a link rather than following it, so a destination
    /// that is a link is judged where the write lands as well as where the
    /// link leads: a settings file linked to a project file is refused, and a
    /// command file linked out is still a command row.
    #[cfg(unix)]
    #[test]
    fn a_link_is_judged_where_the_write_lands() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join(".claude/commands/acme")).unwrap();
        std::fs::create_dir_all(repo.join("shared")).unwrap();
        std::fs::write(repo.join("shared/s.json"), "{}").unwrap();
        std::fs::write(repo.join("shared/cmd.md"), "x").unwrap();
        std::os::unix::fs::symlink(
            repo.join("shared/s.json"),
            repo.join(".claude/settings.local.json"),
        )
        .unwrap();
        std::os::unix::fs::symlink(
            repo.join("shared/cmd.md"),
            repo.join(".claude/commands/acme/cmd.md"),
        )
        .unwrap();
        assert!(refused(&judge(repo, ".claude/settings.local.json")));
        assert_eq!(
            judge(repo, ".claude/commands/acme/cmd.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// A name a case-insensitive file system resolves to a config name is
    /// judged as that name: Unicode case folding (APFS finds `.mcp.json`
    /// under `.mcp.jſon`), and Windows' dropped trailing dots and spaces and
    /// its stream suffix.
    #[test]
    fn folded_names_are_judged_as_the_host_reads_them() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        for rel in [
            ".mcp.j\u{17f}on",
            "opencode.j\u{17f}on",
            "sub/.mcp.j\u{17f}on",
            ".agent\u{17f}/settings.json",
            ".p\u{131}/settings.json",
            ".mcp.json.",
            ".mcp.json ",
            ".mcp.json::$DATA",
            ".claude./settings.json",
            // Nested, so only the name fold sees them, not the comparison
            // with where the project root's config directory leads.
            "sub/.CLAUDE/settings.json",
            "sub/.agent\u{17f}/settings.json",
        ] {
            assert!(
                refused(&judge(repo, rel)),
                "{rel:?}: {:?}",
                judge(repo, rel)
            );
        }
        // A stream suffix on a directory is refused too: by the fold where
        // the name is legal, and on Windows, where it is not, by the path
        // failing to resolve, so nothing is written either way.
        let streamed = judge(repo, ".claude:x/settings.json");
        assert!(
            refused(&streamed)
                || (cfg!(windows) && matches!(streamed, Err(PrimitiveError::Io { .. }))),
            "{streamed:?}"
        );
        assert_eq!(fold(".MCP.J\u{17f}ON. "), ".mcp.json");
        assert_eq!(fold("\u{212a}"), "k");
    }

    /// A root link to a path that does not exist yet keeps the case it was
    /// written in, while a case-insensitive file system finds the target
    /// under any case, so the comparison folds both sides.
    #[cfg(unix)]
    #[test]
    fn a_dangling_root_link_is_compared_folded() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::os::unix::fs::symlink(repo.join("cfg"), repo.join(".claude")).unwrap();
        assert!(refused(&judge(repo, "CFG/settings.local.json")));
        std::fs::create_dir_all(repo.join("shared")).unwrap();
        std::os::unix::fs::symlink(repo.join("shared/m.json"), repo.join(".mcp.json")).unwrap();
        assert!(refused(&judge(repo, "shared/M.json")));
    }

    /// The path as given is judged by its names, since a host reads through
    /// any link on it: a linked nested config directory and a linked
    /// commands directory are judged where the host reads them.
    #[cfg(unix)]
    #[test]
    fn the_path_as_given_is_judged() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join("elsewhere")).unwrap();
        std::fs::create_dir_all(repo.join("sub")).unwrap();
        std::os::unix::fs::symlink(repo.join("elsewhere"), repo.join("sub/.claude")).unwrap();
        assert!(refused(&judge(repo, "sub/.claude/settings.json")));
        assert_eq!(
            judge(repo, "sub/.claude/commands/acme/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );

        std::fs::create_dir_all(repo.join(".claude")).unwrap();
        std::fs::create_dir_all(repo.join("shared/cmds")).unwrap();
        std::os::unix::fs::symlink(repo.join("shared/cmds"), repo.join(".claude/commands"))
            .unwrap();
        assert_eq!(
            judge(repo, ".claude/commands/acme/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// The landing path resolves the destination's parent: a write through a
    /// link to a config directory lands there even when the file it replaces
    /// is a link out.
    #[cfg(unix)]
    #[test]
    fn the_landing_parent_is_resolved() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join(".claude")).unwrap();
        std::fs::create_dir_all(repo.join("shared")).unwrap();
        std::fs::write(repo.join("shared/s.json"), "{}").unwrap();
        std::os::unix::fs::symlink(
            repo.join("shared/s.json"),
            repo.join(".claude/settings.local.json"),
        )
        .unwrap();
        std::os::unix::fs::symlink(repo.join(".claude"), repo.join("alias")).unwrap();
        assert!(refused(&judge(repo, "alias/settings.local.json")));
    }

    /// A destination that is a link is not judged where it leads, since the
    /// write replaces it: a root `CLAUDE.md` linked into `.claude/` takes
    /// `/ductus`'s block, as does a `specs/rules` file that `.claude/rules`
    /// links to. And a config directory linked to a directory holding the
    /// project names no configuration of its own.
    #[cfg(unix)]
    #[test]
    fn legitimate_writes_through_links_are_not_refused() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join(".claude")).unwrap();
        std::fs::write(repo.join(".claude/CLAUDE.md"), "x").unwrap();
        std::os::unix::fs::symlink(repo.join(".claude/CLAUDE.md"), repo.join("CLAUDE.md")).unwrap();
        assert_eq!(judge(repo, "CLAUDE.md").unwrap(), None);
        std::fs::create_dir_all(repo.join("specs/rules")).unwrap();
        std::os::unix::fs::symlink(repo.join("specs/rules"), repo.join(".claude/rules")).unwrap();
        assert_eq!(
            judge(repo, "specs/rules/security-backend.md").unwrap(),
            None
        );

        let outer = tempfile::tempdir().unwrap();
        let project = outer.path().join("project");
        std::fs::create_dir_all(&project).unwrap();
        std::os::unix::fs::symlink(outer.path(), project.join(".claude")).unwrap();
        assert_eq!(judge(&project, "specs/001-x/spec.md").unwrap(), None);
    }
}
