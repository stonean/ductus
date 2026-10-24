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
//! The project, for this check, is the outermost git work tree that holds
//! the working directory, or the working directory itself when no work tree
//! does: a caller of the CLI chooses the working directory, so it alone does
//! not bound what is judged. A Claude Code worktree under
//! `.claude/worktrees/<name>/` is judged as a project of its own. A destination that resolves outside the project is
//! not judged here; `apply-manifest`'s roots are bounded by the working
//! directory separately (spec 022 task 17, [`super::resolve_root`]).

use std::path::{Component, Path, PathBuf};

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
/// - the path as given, under the project root however the root is spelled
///   ([`given_names`]), since a host reads through any link on it
///   (`.claude/commands` linked elsewhere is still where Claude Code reads
///   commands);
/// - where the write lands, the parent resolved and the final name as given,
///   since a tempfile-and-rename write replaces a link there rather than
///   following it;
/// - where the path leads, every link on it followed, unless the destination
///   is itself a link, which the write replaces. A file system resolves a
///   name an existing file answers to, such as a Windows 8.3 short name, to
///   the file's own name here.
///
/// Each is judged inside the project only, and from two places: the project
/// root `repo`, and above it ([`outer_root`]): from the root of the
/// outermost git work tree that holds it, every destination inside that work
/// tree, or, when no work tree holds it, a destination inside the project
/// with the project root's own names in front. The project root is the working directory,
/// which a caller of the CLI chooses, so a run from inside `.claude/` would
/// otherwise see no config directory above it.
/// Returns `Ok(None)` when none lies in a host's configuration, and
/// `Ok(Some(row))` when one is a row a `/ductus` step writes:
/// [`HostRow::CommandOrSkill`] when any judgment finds one, so its
/// frontmatter is judged however the others read.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`], naming `shown`, for any other
/// destination in a host's configuration:
///
/// - under a config directory at any depth (a host reads the one where its
///   session starts, and a manifest's target root may be a subdirectory), a
///   row being matched under the first config directory on the path, or
///   under a Claude Code worktree the first one below `.claude/worktrees/`
///   ([`judge_under`]);
/// - an MCP or settings file at any depth, or the file the project root's
///   `.mcp.json` or `opencode.json(c)` finally resolves to (a link between
///   them in a chain is not followed back, and a file the config-directory
///   step names a row comes back as that row first:
///   `.mcp.json -> .claude/commands/acme/g.md` leaves `g.md` writable as a
///   command);
/// - under the directory the project root's config directory links to
///   (`.claude -> cfg` makes `cfg/settings.local.json` the host's), unless
///   that directory holds the project itself.
///
/// Names and the paths links lead to compare after [`fold`], since a
/// case-insensitive file system finds `.mcp.json` under `.MCP.JSON` and, on
/// macOS, `.mcp.jſon`. Unicode normalization is not folded: APFS finds a name
/// under either of its composed and decomposed spellings, so a link to a
/// missing target spelled one way lets a write spelled the other create the
/// file the link resolves to.
///
/// Not judged, when named by its own path: a project file that a link
/// *inside* a config directory points at (`.claude/settings.local.json ->
/// shared/s.json` leaves `shared/s.json` writable), and one that the config
/// directory or a root file of any directory other than the project root
/// and the work tree's root links to: one below the project root
/// (`sub/.claude -> ../elsewhere` leaves `elsewhere/settings.json`
/// writable), a sibling of it, or a subdirectory project's or a Claude Code
/// worktree's own when the CLI runs from below that directory, since only
/// the project root's and the work tree root's links are followed back.
/// [`PrimitiveError::Io`] when the project root or the destination cannot be
/// resolved.
pub(crate) fn check_destination(repo: &Path, dest: &Path, shown: &str) -> Result<Option<HostRow>> {
    let resolve = |p: &Path| {
        resolve_symlinks(p).map_err(|source| PrimitiveError::Io {
            path: p.into(),
            source,
        })
    };
    let canonical_repo = resolve(repo)?;
    let landing = match (dest.parent(), dest.file_name()) {
        (Some(parent), Some(name)) => resolve(parent)?.join(name),
        _ => resolve(dest)?,
    };
    let leads = if std::fs::symlink_metadata(dest).is_ok_and(|meta| meta.file_type().is_symlink()) {
        None
    } else {
        Some(resolve(dest)?)
    };
    let mut rows = Vec::new();
    let mut bases = vec![(repo.to_path_buf(), canonical_repo.clone())];
    let mut above = Vec::new();
    match outer_root(&canonical_repo) {
        Some(Outer::WorkTree(root)) => bases.push((root.clone(), root)),
        Some(Outer::Ancestors(names)) => above = names,
        None => {}
    }
    for (base, canonical_base) in &bases {
        if let Some(names) = given_names(base, canonical_base, dest) {
            rows.extend(judge_names(&names, shown)?);
        }
        rows.extend(judge(base, canonical_base, &landing, shown)?);
        if let Some(leads) = &leads {
            rows.extend(judge(base, canonical_base, leads, shown)?);
        }
    }
    if !above.is_empty() {
        let under_root = [
            given_names(repo, &canonical_repo, dest),
            strip_folded(&landing, &canonical_repo),
            leads
                .as_deref()
                .and_then(|leads| strip_folded(leads, &canonical_repo)),
        ];
        for names in under_root.into_iter().flatten() {
            rows.extend(judge_names(&[above.as_slice(), &names].concat(), shown)?);
        }
    }
    Ok(rows
        .iter()
        .copied()
        .find(|row| *row == HostRow::CommandOrSkill)
        .or_else(|| rows.first().copied()))
}

/// What lies above the project root for [`check_destination`] to judge.
enum Outer {
    /// The root of the outermost git work tree that holds the project root,
    /// above it: every destination inside that work tree is judged from
    /// there.
    WorkTree(PathBuf),
    /// The names of the project root's own path, when no work tree holds
    /// it: a destination inside the project is judged with them
    /// in front of its own, and one outside the project is not judged here
    /// (`apply-manifest`'s roots are bounded separately, spec 022 task 17).
    Ancestors(Vec<String>),
}

/// The second place [`check_destination`] judges from: the root of the
/// outermost git work tree that holds `canonical_repo`, the project root
/// resolved, when it lies above it, or the project root's own names when no
/// work tree holds it. `None` when the project root is that root. A work tree
/// is a directory holding a `.git` entry of any kind, a file, a directory or
/// a link (one leading nowhere included): found that way
/// rather than through libgit2, which refuses to open some repositories
/// (`extensions.refstorage`, for one) and would leave their work tree
/// unseen. The outermost one is taken (spec 022 task 11, the owner's answer
/// (a) to its question of 2026-10-22), so a repository nested inside a
/// config directory hides nothing, and a Claude Code worktree under
/// `.claude/worktrees/` is judged as a project of its own ([`judge_under`]).
fn outer_root(canonical_repo: &Path) -> Option<Outer> {
    let work_tree = canonical_repo
        .ancestors()
        .filter(|dir| std::fs::symlink_metadata(dir.join(".git")).is_ok())
        .last();
    match work_tree {
        Some(root) if root == canonical_repo => None,
        Some(root) => Some(Outer::WorkTree(root.to_path_buf())),
        None => Some(Outer::Ancestors(names_of(canonical_repo))),
    }
}

/// [`check_destination`] for a primitive that writes no row: every
/// destination in a host's configuration is refused, the rows included.
/// `merge-managed-block` is one, since no `/ductus` step writes a block into a
/// config directory, and a block written into a command file's frontmatter
/// would grant what the file's own check refused.
///
/// # Errors
///
/// As [`check_destination`], and [`PrimitiveError::InvalidPath`], naming
/// `shown`, for a row.
pub(crate) fn check_no_destination(repo: &Path, dest: &Path, shown: &str) -> Result<()> {
    match check_destination(repo, dest, shown)? {
        None => Ok(()),
        Some(_) => Err(PrimitiveError::InvalidPath {
            path: shown.into(),
            reason: "is a command, skill, rules-mirror or bridge file in an agent host's \
                     configuration, which only `apply-manifest` writes (spec 022 task 11)"
                .into(),
        }),
    }
}

/// `dest`'s names under the project root as written: what follows the
/// outermost ancestor of `dest` that resolves to the project root, with no
/// `..` among them. The root may be spelled otherwise than `repo`: through
/// the links above it (`/tmp` for macOS's `/private/tmp`), in another letter
/// case on a file system that is not case-sensitive, or with a `..` that
/// climbs back to it (`sub/..`), all of which an `apply-manifest` root may
/// carry. When every such remainder holds a `..` that does not climb back
/// to the root, as `.claude/commands/acme/..` does, the outermost one whose
/// `..`s stay below the root is read lexically, each `..` taking back the
/// name before it: through a linked `.claude/commands` the file system's
/// `..` leaves the config directory, while the host still reads the file
/// under its config name. A `..` read lexically can name a config directory
/// a write does not reach, which refuses more, never less. `None` when no
/// ancestor resolves to the root, as for a destination outside the project,
/// or when every lexical reading climbs above it.
fn given_names(repo: &Path, canonical_repo: &Path, dest: &Path) -> Option<Vec<String>> {
    let no_parent = |rel: &Path| !rel.components().any(|c| matches!(c, Component::ParentDir));
    if let Ok(rel) = dest
        .strip_prefix(repo)
        .or_else(|_| dest.strip_prefix(canonical_repo))
        && no_parent(rel)
    {
        return Some(names_of(rel));
    }
    let mut ancestors: Vec<&Path> = dest.ancestors().skip(1).collect();
    ancestors.reverse();
    let mut lexical = None;
    for ancestor in ancestors {
        let Ok(rel) = dest.strip_prefix(ancestor) else {
            continue;
        };
        let at_root = resolve_symlinks(ancestor).is_ok_and(|resolved| {
            strip_folded(&resolved, canonical_repo).is_some_and(|rest| rest.is_empty())
        });
        if !at_root {
            continue;
        }
        if no_parent(rel) {
            return Some(names_of(rel));
        }
        if lexical.is_none() {
            lexical = lexical_names(rel);
        }
    }
    lexical
}

/// `rel`'s names with each `..` taking back the name before it, or `None`
/// when a `..` has no name to take back.
fn lexical_names(rel: &Path) -> Option<Vec<String>> {
    let mut names = Vec::new();
    for component in rel.components() {
        match component {
            Component::Normal(name) => names.push(name.to_string_lossy().into_owned()),
            Component::ParentDir => {
                names.pop()?;
            }
            _ => {}
        }
    }
    Some(names)
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
            return judge_under(dir, layout, &rest, shown);
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
            return judge_under(dir, layout, &names[depth + 1..], shown);
        }
    }
    Ok(None)
}

/// The verdict on `rest`, a path under the config directory `dir`: the row
/// it is, or a refusal. The one exception is Claude Code's own worktrees,
/// which it creates "under `.claude/worktrees/<name>/` at your repository
/// root" (code.claude.com/docs/en/worktrees) and which its protected-path
/// list exempts from `.claude`: the names below a worktree's directory are a
/// checkout's, judged afresh as from a project root (spec 022 task 11, the
/// owner's answer (a) to its question of 2026-10-22). The worktree's own name
/// is judged with them, as a name under that root, so a directory there named
/// for a config directory (`.claude/worktrees/.claude/`) is one: a session
/// started above a nested `.claude/skills/` loads its skills "once Claude
/// works on files there" (code.claude.com/docs/en/skills). A name that is
/// not a config directory's changes no verdict, since a config directory is
/// matched at any depth and a config file only as the last name.
fn judge_under(dir: &str, layout: Layout, rest: &[String], shown: &str) -> Result<Option<HostRow>> {
    if dir == ".claude" && rest.len() > 2 && fold(&rest[0]) == "worktrees" {
        return judge_names(&rest[1..], shown);
    }
    row(layout, rest)
        .map(Some)
        .ok_or_else(|| refuse(shown, dir))
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

/// The row `rest`, the path under a config directory, is, if any. Names
/// compare after [`fold`], as config names do: `/ductus` writes a row in one
/// case, but a file system that is not case-sensitive finds it under an
/// existing file or directory of another case, and a path resolved through
/// that one carries its case. A path deeper than its row is not one.
fn row(layout: Layout, rest: &[String]) -> Option<HostRow> {
    let folded: Vec<String> = rest.iter().map(|name| fold(name)).collect();
    let parts: Vec<&str> = folded.iter().map(String::as_str).collect();
    let md = |name: &str| Path::new(name).extension().is_some_and(|ext| ext == "md");
    match (layout, parts.as_slice()) {
        (Layout::ClaudeStyle, ["commands", "ductus.md"])
        | (Layout::OpenCode, ["command", "ductus.md"])
        | (Layout::Antigravity, ["skills", _, "skill.md"]) => Some(HostRow::CommandOrSkill),
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

    // What each judgment of [`check_destination`] does alone, and the test
    // that fails when that judgment alone loses it (spec 022 task 11,
    // re-reads 7 to 13). The cells are the product of four dimensions:
    //
    // - base: P, the project root; W, the outermost work tree's root, when the
    //   project root lies below it; A, with no work tree, the project root's
    //   own ancestors;
    // - step: the path as given, judged by its names (G); and where the write
    //   lands, judged through the root's config-directory links (D), then its
    //   root files' links (F), then by its names (N), each returning before
    //   the next, and where its links lead (L), judged the same way;
    // - output: a refusal (R); a command row (C), whose frontmatter the
    //   caller judges; a row naming no command (X), the rules mirror or Pi's
    //   bridge, which `merge-managed-block` refuses ([`check_no_destination`]);
    // - name: the five config directories and the three root files.
    //
    // It collapses, each for a reason the code gives: A runs no link step,
    // judging the ancestors' names alone, so A has G and the landing by its
    // names only. F and the root files have no row: a root file is never one.
    // C lies only under a config directory, and X only under `.agents` (the
    // rules mirror) and `.pi` (the bridge). G, N and A's landing are one
    // function, `judge_names`, which reads names and no base, so each name is
    // held once, at P, while each base's G and N are held per output. L has
    // no test of its own: it differs from the landing only through a Windows
    // 8.3 short name, a link behind a missing directory, or on macOS a name in
    // another Unicode normalization, each refusing more, not less (M28). And
    // F alone refuses only through a name the file system folds, so F is held
    // jointly with L (M114, M115). A cell's fixture turns the others off: a
    // destination that is itself a link turns off L; a repository at the
    // project root turns off A; a project root below the work tree's root
    // leaves P nothing to see; and a landing meant for N lies under a nested
    // config directory, since D resolves each of the root's own config
    // directories, link or not.
    //
    // | Base | Step | Output | Test |
    // | --- | --- | --- | --- |
    // | P | G | R | `the_path_as_given_is_judged` |
    // | P | G | C | `the_project_root_judges_the_path_as_given` |
    // | P | D | R | `the_landing_follows_the_root_config_directory_link` |
    // | P | D | C | `the_landing_names_a_command_through_the_root_config_directory` |
    // | P | F | R | `links_are_resolved_before_judging`, with L |
    // | P | N | R | `the_landing_is_judged_by_its_names_from_each_base` |
    // | P | N | C | `the_landing_names_a_command_by_its_names_from_each_base` |
    // | W | G | R | `the_path_as_given_is_judged` |
    // | W | G | C | `the_work_tree_root_judges_the_path_as_given` |
    // | W | D | R | `the_landing_follows_the_root_config_directory_link` |
    // | W | D | C | `the_landing_names_a_command_through_the_root_config_directory` |
    // | W | F | R | `the_work_tree_root_follows_its_root_files_links_back`, with L |
    // | W | N | R | `the_landing_is_judged_by_its_names_from_each_base` |
    // | W | N | C | `the_landing_names_a_command_by_its_names_from_each_base` |
    // | A | G | R | `with_no_repository_the_ancestors_refuse_the_path_as_given` |
    // | A | G | C | `with_no_repository_the_ancestors_judge_the_path_as_given` |
    // | A | landing | R | `with_no_repository_the_ancestors_judge_where_the_write_lands` |
    // | A | landing | C | `with_no_repository_the_ancestors_name_a_command_where_the_write_lands` |
    // | P, W | G, D, N | X | `a_block_refuses_the_rows_that_name_no_command`, each kind |
    // | A | G, landing | X | `a_block_refuses_the_rows_that_name_no_command`, each kind |
    //
    // By name: `judge_names`' R for every name and C for every directory,
    // held once, by `every_config_name_is_judged_by_its_names`; D's R per
    // directory, the landing alone, and F's R per root file, with L, each
    // from P and from W, by `every_root_config_link_is_followed_back`; and D's
    // C per directory, from P and from W, by
    // `every_config_directory_names_a_row_through_its_root`.
    //
    // Two branches inside the shared steps are held per base too (re-read
    // 14, the owner's answer (b) to its question): the Claude Code worktree
    // exception's judgment of the names below a worktree, from P
    // (`a_claude_worktree_is_judged_from_the_main_checkout`), from W and from
    // A (`the_worktree_exception_judges_from_each_base`); and the path as
    // given under another spelling of the project root and with a `..` read
    // lexically, from P, W and A (`a_spelled_path_is_judged_as_given_from_each_base`).
    // `judge`'s own internal branches, the skip of a config directory that
    // links to an ancestor and the folded comparison, are not split per base.

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
    /// neither is one outside the project (`apply-manifest`'s roots are
    /// bounded separately, spec 022 task 17).
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
            // Each row's shape, one constraint at a time: its directory, its
            // file name or extension, and its depth.
            ".claude/commands/evil.md",
            ".claude/command/ductus.md",
            ".claude/command/acme/status.md",
            ".opencode/command/evil.md",
            ".opencode/commands/ductus.md",
            ".opencode/command/acme/sub/status.md",
            ".opencode/command/acme/status.txt",
            ".agents/skills/acme/sub/SKILL.md",
            ".agents/skills/SKILL.md",
            ".agents/skill/acme-status/SKILL.md",
            ".agents/rules/security.json",
            ".agents/rules/sub/security.md",
            ".agents/rule/security.md",
            ".pi/prompts/sub/acme-status.md",
            ".pi/prompts/acme-status.txt",
            ".pi/prompt/acme-status.md",
            ".pi/extensions/sub/ductus.ts",
            ".pi/extension/ductus.ts",
            // Each row's layout, and the depth of each `ductus` file.
            ".agents/commands/ductus.md",
            ".agents/commands/acme/status.md",
            ".claude/prompts/acme-status.md",
            ".claude/rules/security.md",
            ".claude/extensions/ductus.ts",
            ".claude/commands/acme/sub/ductus.md",
            ".opencode/command/acme/sub/ductus.md",
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
    /// commands directory are judged where the host reads them. With a
    /// repository at the project root, the path as given alone refuses the
    /// nested settings file, from the project root and from below the work
    /// tree's root.
    #[cfg(unix)]
    #[test]
    fn the_path_as_given_is_judged() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        git2::Repository::init(repo).unwrap();
        std::fs::create_dir_all(repo.join("elsewhere")).unwrap();
        std::fs::create_dir_all(repo.join("sub")).unwrap();
        std::fs::create_dir_all(repo.join("specs")).unwrap();
        std::os::unix::fs::symlink(repo.join("elsewhere"), repo.join("sub/.claude")).unwrap();
        assert!(refused(&judge(repo, "sub/.claude/settings.json")));
        let below = repo.join("specs");
        let dest = below.join("../sub/.claude/settings.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
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

    /// A destination spelled through a `..` back to the project root is
    /// judged by its names under the root, so a linked commands directory is
    /// still where the host reads commands.
    #[cfg(unix)]
    #[test]
    fn a_root_spelled_another_way_is_judged_as_given() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join(".claude")).unwrap();
        std::fs::create_dir_all(repo.join("sub")).unwrap();
        std::fs::create_dir_all(repo.join("shared/cmds")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", repo.join(".claude/commands")).unwrap();
        let dest = repo.join("sub/../.claude/commands/acme/g.md");
        assert_eq!(
            check_destination(repo, &dest, "g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// A `..` after a linked config directory leaves it on the file system,
    /// so the path as given is read lexically and the command row is still
    /// seen.
    #[cfg(unix)]
    #[test]
    fn a_parent_step_through_a_linked_config_directory_is_read_lexically() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join(".claude")).unwrap();
        std::fs::create_dir_all(repo.join("shared/cmds/acme")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", repo.join(".claude/commands")).unwrap();
        let dest = repo.join(".claude/commands/acme/../acme/g.md");
        assert_eq!(
            check_destination(repo, &dest, "g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
        assert_eq!(lexical_names(Path::new("a/../../b")), None);
        assert_eq!(
            lexical_names(Path::new("a/b/../c")),
            Some(vec!["a".to_owned(), "c".to_owned()])
        );
    }

    /// A project root inside a config directory, as a CLI run from
    /// `.claude/` makes it, is judged from the work tree's root, and so is a
    /// destination above the project root: the config directory is still
    /// seen.
    #[test]
    fn a_project_root_inside_a_config_directory_is_judged_from_its_work_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join(".claude")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        let inside = tree.join(".claude");
        assert!(refused(&judge(&inside, "settings.local.json")));
        assert_eq!(
            judge(&inside, "commands/acme/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
        let below = tree.join("specs");
        let dest = below.join("../.claude/settings.local.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
    }

    /// With no repository, the whole path is judged, so a project root
    /// inside a config directory still sees it.
    #[test]
    fn a_project_root_outside_any_repository_is_judged_whole() {
        let tmp = tempfile::tempdir().unwrap();
        let inside = tmp.path().join("proj/.agents");
        std::fs::create_dir_all(&inside).unwrap();
        assert!(refused(&judge(&inside, "settings.json")));
    }

    /// A Claude Code worktree under `.claude/worktrees/<name>/`, whose `.git`
    /// is a file, is judged as a project of its own from the outermost work
    /// tree, so its writes pass; a path out of it into the main checkout's
    /// `.claude/` is still refused.
    #[test]
    fn a_claude_worktree_is_judged_as_a_project_of_its_own() {
        let tmp = tempfile::tempdir().unwrap();
        let main = tmp.path();
        git2::Repository::init(main).unwrap();
        let worktree = main.join(".claude/worktrees/wt");
        std::fs::create_dir_all(&worktree).unwrap();
        std::fs::write(
            worktree.join(".git"),
            "gitdir: ../../../.git/worktrees/wt\n",
        )
        .unwrap();
        assert_eq!(judge(&worktree, "specs/001-x/spec.md").unwrap(), None);
        assert_eq!(
            judge(&worktree, ".claude/commands/acme/status.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
        assert!(refused(&judge(&worktree, ".claude/settings.local.json")));
        let out = worktree.join("../../settings.local.json");
        assert!(refused(&check_destination(&worktree, &out, "x")));
    }

    /// Judged from the main checkout's root, where only the exception sees a
    /// worktree, the names below it are still judged: a worktree's settings
    /// file is refused there, while its rows and its other files pass.
    #[test]
    fn a_claude_worktree_is_judged_from_the_main_checkout() {
        let tmp = tempfile::tempdir().unwrap();
        let main = tmp.path();
        git2::Repository::init(main).unwrap();
        std::fs::create_dir_all(main.join(".claude/worktrees/wt")).unwrap();
        assert!(refused(&judge(
            main,
            ".claude/worktrees/wt/.claude/settings.local.json"
        )));
        assert_eq!(
            judge(main, ".claude/worktrees/wt/.claude/commands/acme/status.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
        assert_eq!(
            judge(main, ".claude/worktrees/wt/specs/001-x/spec.md").unwrap(),
            None
        );
        assert_eq!(judge(main, ".claude/worktrees/wt/CLAUDE.md").unwrap(), None);
    }

    /// Run from below the work tree's root, a command written through a
    /// linked commands directory is named a command only by the work tree's
    /// root, which judges the path as given, so its frontmatter is judged.
    #[cfg(unix)]
    #[test]
    fn the_work_tree_root_judges_the_path_as_given() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join("shared/cmds/acme")).unwrap();
        std::fs::create_dir_all(tree.join(".claude")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", tree.join(".claude/commands")).unwrap();
        let below = tree.join("specs");
        let dest = below.join("../.claude/commands/acme/g.md");
        assert_eq!(
            check_destination(&below, &dest, "x").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// Run from below the work tree's root, a file under the directory the
    /// root's config directory links to is refused only by the work tree's
    /// root, which follows its own links back.
    #[cfg(unix)]
    #[test]
    fn the_work_tree_root_follows_its_own_links_back() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join("cfg")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::os::unix::fs::symlink("cfg", tree.join(".claude")).unwrap();
        let below = tree.join("specs");
        let dest = below.join("../cfg/settings.local.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
    }

    /// Run from below the work tree's root, the file the root's `.mcp.json`
    /// links to is refused only by the work tree's root, which follows its
    /// own root files' links back too.
    #[cfg(unix)]
    #[test]
    fn the_work_tree_root_follows_its_root_files_links_back() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join("shared")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::fs::write(tree.join("shared/m.json"), "{}\n").unwrap();
        std::os::unix::fs::symlink("shared/m.json", tree.join(".mcp.json")).unwrap();
        let below = tree.join("specs");
        let dest = below.join("../shared/m.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
    }

    /// With a repository at the project root, the only base, a command
    /// written through a linked commands directory is named a command by the
    /// path as given alone.
    #[cfg(unix)]
    #[test]
    fn the_project_root_judges_the_path_as_given() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        git2::Repository::init(repo).unwrap();
        std::fs::create_dir_all(repo.join("shared/cmds/acme")).unwrap();
        std::fs::create_dir_all(repo.join(".claude")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", repo.join(".claude/commands")).unwrap();
        assert_eq!(
            judge(repo, ".claude/commands/acme/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// A destination that is itself a link, under a nested config directory
    /// reached through a link, is refused only by the landing's names, from
    /// the project root and from below the work tree's root: the root has no
    /// `.claude` of its own to catch it first.
    #[cfg(unix)]
    #[test]
    fn the_landing_is_judged_by_its_names_from_each_base() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join("sub/.claude")).unwrap();
        std::fs::create_dir_all(tree.join("shared")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::fs::write(tree.join("shared/o.json"), "{}\n").unwrap();
        std::os::unix::fs::symlink(
            "../../shared/o.json",
            tree.join("sub/.claude/settings.json"),
        )
        .unwrap();
        std::os::unix::fs::symlink("sub/.claude", tree.join("alias")).unwrap();
        assert!(refused(&judge(tree, "alias/settings.json")));
        let below = tree.join("specs");
        let dest = below.join("../alias/settings.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
    }

    /// A destination that is itself a link, under the directory the root's
    /// `.claude` links to, is refused only by the landing's judgment through
    /// that link, from the project root and from below the work tree's root.
    #[cfg(unix)]
    #[test]
    fn the_landing_follows_the_root_config_directory_link() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join("cfg")).unwrap();
        std::fs::create_dir_all(tree.join("shared")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::fs::write(tree.join("shared/s.json"), "{}\n").unwrap();
        std::os::unix::fs::symlink("cfg", tree.join(".claude")).unwrap();
        std::os::unix::fs::symlink("../shared/s.json", tree.join("cfg/settings.local.json"))
            .unwrap();
        assert!(refused(&judge(tree, "cfg/settings.local.json")));
        let below = tree.join("specs");
        let dest = below.join("../cfg/settings.local.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
    }

    /// With no repository and a project root inside `.claude`, a command
    /// written through a linked commands directory is named a command only
    /// by the ancestors' judgment of the path as given.
    #[cfg(unix)]
    #[test]
    fn with_no_repository_the_ancestors_judge_the_path_as_given() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        std::fs::create_dir_all(tree.join("shared/cmds/acme")).unwrap();
        std::fs::create_dir_all(tree.join(".claude")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", tree.join(".claude/commands")).unwrap();
        let inside = tree.join(".claude");
        let dest = inside.join("../.claude/commands/acme/g.md");
        assert_eq!(
            check_destination(&inside, &dest, "x").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// With no repository and a project root inside `.claude`, a destination
    /// that is itself a link, reached through a link into `.claude`, is
    /// refused only by the ancestors' judgment of where the write lands.
    #[cfg(unix)]
    #[test]
    fn with_no_repository_the_ancestors_judge_where_the_write_lands() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        std::fs::create_dir_all(tree.join(".claude/sub")).unwrap();
        std::fs::write(tree.join(".claude/sub/other.json"), "{}\n").unwrap();
        std::os::unix::fs::symlink("other.json", tree.join(".claude/sub/settings.json")).unwrap();
        std::os::unix::fs::symlink(".claude/sub", tree.join("alias")).unwrap();
        let inside = tree.join(".claude");
        let dest = inside.join("../alias/settings.json");
        assert!(refused(&check_destination(&inside, &dest, "x")));
    }

    /// With no repository and a project root inside `.claude`, a file the
    /// host would read as a command directly under the linked commands
    /// directory, not a row, is refused only by the ancestors' judgment of
    /// the path as given.
    #[cfg(unix)]
    #[test]
    fn with_no_repository_the_ancestors_refuse_the_path_as_given() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        std::fs::create_dir_all(tree.join("shared/cmds")).unwrap();
        std::fs::create_dir_all(tree.join(".claude")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", tree.join(".claude/commands")).unwrap();
        let inside = tree.join(".claude");
        let dest = inside.join("../.claude/commands/evil.md");
        assert!(refused(&check_destination(&inside, &dest, "x")));
    }

    /// With no repository and a project root inside `.claude`, a command
    /// file that is itself a link, reached through a link into `.claude`, is
    /// named a command only by the ancestors' judgment of where the write
    /// lands.
    #[cfg(unix)]
    #[test]
    fn with_no_repository_the_ancestors_name_a_command_where_the_write_lands() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        std::fs::create_dir_all(tree.join(".claude/commands/acme")).unwrap();
        std::fs::create_dir_all(tree.join("shared")).unwrap();
        std::fs::write(tree.join("shared/g.md"), "x\n").unwrap();
        std::os::unix::fs::symlink(
            "../../../shared/g.md",
            tree.join(".claude/commands/acme/g.md"),
        )
        .unwrap();
        std::os::unix::fs::symlink(".claude/commands/acme", tree.join("alias")).unwrap();
        let inside = tree.join(".claude");
        let dest = inside.join("../alias/g.md");
        assert_eq!(
            check_destination(&inside, &dest, "x").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// A command file that is itself a link, under a nested config directory
    /// reached through a link, is named a command only by the landing's
    /// names, from the project root and from below the work tree's root.
    #[cfg(unix)]
    #[test]
    fn the_landing_names_a_command_by_its_names_from_each_base() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join("sub/.claude/commands/acme")).unwrap();
        std::fs::create_dir_all(tree.join("shared")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::fs::write(tree.join("shared/g.md"), "x\n").unwrap();
        std::os::unix::fs::symlink(
            "../../../../shared/g.md",
            tree.join("sub/.claude/commands/acme/g.md"),
        )
        .unwrap();
        std::os::unix::fs::symlink("sub/.claude/commands/acme", tree.join("alias")).unwrap();
        assert_eq!(
            judge(tree, "alias/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
        let below = tree.join("specs");
        let dest = below.join("../alias/g.md");
        assert_eq!(
            check_destination(&below, &dest, "x").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// A command file that is itself a link, under the root's own `.claude`
    /// reached through a link, is named a command only by the landing's
    /// config-directory step, from the project root and from below the work
    /// tree's root. That step reads the root's `.claude` the same way whether
    /// it is a directory or a link, so one fixture holds both.
    #[cfg(unix)]
    #[test]
    fn the_landing_names_a_command_through_the_root_config_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join(".claude/commands/acme")).unwrap();
        std::fs::create_dir_all(tree.join("shared")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::fs::write(tree.join("shared/g.md"), "x\n").unwrap();
        std::os::unix::fs::symlink(
            "../../../shared/g.md",
            tree.join(".claude/commands/acme/g.md"),
        )
        .unwrap();
        std::os::unix::fs::symlink(".claude/commands/acme", tree.join("alias")).unwrap();
        assert_eq!(
            judge(tree, "alias/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
        let below = tree.join("specs");
        let dest = below.join("../alias/g.md");
        assert_eq!(
            check_destination(&below, &dest, "x").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// Every config directory's link and every root file's link at the root
    /// is followed back, each name on its own.
    #[cfg(unix)]
    #[test]
    fn every_root_config_link_is_followed_back() {
        for &(dir, _) in CONFIG_DIRS {
            let tmp = tempfile::tempdir().unwrap();
            let tree = tmp.path();
            git2::Repository::init(tree).unwrap();
            std::fs::create_dir_all(tree.join("cfg")).unwrap();
            std::fs::create_dir_all(tree.join("shared")).unwrap();
            std::fs::write(tree.join("shared/s.json"), "{}\n").unwrap();
            std::os::unix::fs::symlink("../shared/s.json", tree.join("cfg/settings.json")).unwrap();
            std::os::unix::fs::symlink("cfg", tree.join(dir)).unwrap();
            assert!(refused(&judge(tree, "cfg/settings.json")), "{dir}");
            std::fs::create_dir_all(tree.join("specs")).unwrap();
            let below = tree.join("specs");
            let dest = below.join("../cfg/settings.json");
            assert!(refused(&check_destination(&below, &dest, "x")), "{dir}");
        }
        for &file in CONFIG_FILES {
            let tmp = tempfile::tempdir().unwrap();
            let tree = tmp.path();
            git2::Repository::init(tree).unwrap();
            std::fs::create_dir_all(tree.join("shared")).unwrap();
            std::fs::write(tree.join("shared/x"), "{}\n").unwrap();
            std::os::unix::fs::symlink("shared/x", tree.join(file)).unwrap();
            assert!(refused(&judge(tree, "shared/x")), "{file}");
            std::fs::create_dir_all(tree.join("specs")).unwrap();
            let below = tree.join("specs");
            let dest = below.join("../shared/x");
            assert!(refused(&check_destination(&below, &dest, "x")), "{file}");
        }
    }

    /// The rows of each layout, the path to one row's file from its config
    /// directory, and the directory that file sits in.
    const LAYOUT_ROWS: &[(&str, &str, &str)] = &[
        (".claude", "commands/acme/g.md", "commands/acme"),
        (".augment", "commands/acme/g.md", "commands/acme"),
        (".agents", "skills/acme-g/SKILL.md", "skills/acme-g"),
        (".opencode", "command/acme/g.md", "command/acme"),
        (".pi", "prompts/acme-g.md", "prompts"),
    ];

    /// Every config directory and root file is judged by its names, each on
    /// its own, refused and, for a directory, naming its layout's command
    /// row: under a nested `sub/`, which the root's own link steps never
    /// see. The names step is one function at every base, so one base holds
    /// each name.
    #[test]
    fn every_config_name_is_judged_by_its_names() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        git2::Repository::init(repo).unwrap();
        for &(dir, row, _) in LAYOUT_ROWS {
            assert!(
                refused(&judge(repo, &format!("sub/{dir}/settings.json"))),
                "{dir}"
            );
            assert_eq!(
                judge(repo, &format!("sub/{dir}/{row}")).unwrap(),
                Some(HostRow::CommandOrSkill),
                "{dir}"
            );
        }
        for &file in CONFIG_FILES {
            assert!(refused(&judge(repo, &format!("sub/{file}"))), "{file}");
        }
    }

    /// Every config directory's step names its layout's command row on its
    /// own: a row file that is itself a link, reached through a link into
    /// the root's config directory, is named by that step alone.
    #[cfg(unix)]
    #[test]
    fn every_config_directory_names_a_row_through_its_root() {
        for &(dir, row, parent) in LAYOUT_ROWS {
            let tmp = tempfile::tempdir().unwrap();
            let tree = tmp.path();
            git2::Repository::init(tree).unwrap();
            std::fs::create_dir_all(tree.join(dir).join(parent)).unwrap();
            std::fs::create_dir_all(tree.join("shared")).unwrap();
            std::fs::write(tree.join("shared/g.md"), "x\n").unwrap();
            std::os::unix::fs::symlink(tree.join("shared/g.md"), tree.join(dir).join(row)).unwrap();
            std::os::unix::fs::symlink(tree.join(dir).join(parent), tree.join("alias")).unwrap();
            let file = Path::new(row).file_name().unwrap().to_string_lossy();
            assert_eq!(
                judge(tree, &format!("alias/{file}")).unwrap(),
                Some(HostRow::CommandOrSkill),
                "{dir}"
            );
            std::fs::create_dir_all(tree.join("specs")).unwrap();
            let below = tree.join("specs");
            let dest = below.join(format!("../alias/{file}"));
            assert_eq!(
                check_destination(&below, &dest, "x").unwrap(),
                Some(HostRow::CommandOrSkill),
                "{dir}"
            );
        }
    }

    /// `merge-managed-block` refuses the rows that name no command too, the
    /// rules mirror and Pi's bridge, and each step names them on its own from
    /// each base it runs at: the path as given through a linked nested config
    /// directory, the landing through the root's config directory and through
    /// a nested one (each destination itself a link), from the project root
    /// and from below the work tree's root; and with no repository and a
    /// project root inside the config directory, the ancestors' path as given
    /// and landing.
    #[cfg(unix)]
    #[test]
    fn a_block_refuses_the_rows_that_name_no_command() {
        for (dir, sub, file) in [
            (".agents", "rules", "x.md"),
            (".pi", "extensions", "ductus.ts"),
        ] {
            let tmp = tempfile::tempdir().unwrap();
            let tree = tmp.path();
            git2::Repository::init(tree).unwrap();
            let link = |target: &str, at: &str| {
                std::os::unix::fs::symlink(tree.join(target), tree.join(at)).unwrap();
            };
            for d in ["specs", "shared", "elsewhere", "sub", "n"] {
                std::fs::create_dir_all(tree.join(d)).unwrap();
            }
            std::fs::create_dir_all(tree.join("elsewhere").join(sub)).unwrap();
            std::fs::create_dir_all(tree.join(dir).join(sub)).unwrap();
            std::fs::create_dir_all(tree.join("n").join(dir).join(sub)).unwrap();
            std::fs::write(tree.join("shared/f"), "x\n").unwrap();
            link("elsewhere", &format!("sub/{dir}"));
            link("shared/f", &format!("{dir}/{sub}/{file}"));
            link(&format!("{dir}/{sub}"), "root_alias");
            link("shared/f", &format!("n/{dir}/{sub}/{file}"));
            link(&format!("n/{dir}/{sub}"), "nested_alias");
            let below = tree.join("specs");
            for rel in [
                format!("sub/{dir}/{sub}/{file}"),
                format!("root_alias/{file}"),
                format!("nested_alias/{file}"),
            ] {
                assert!(
                    check_no_destination(tree, &tree.join(&rel), &rel).is_err(),
                    "{rel}"
                );
                let dest = below.join("..").join(&rel);
                assert!(
                    check_no_destination(&below, &dest, &rel).is_err(),
                    "{rel} from below"
                );
            }

            let tmp = tempfile::tempdir().unwrap();
            let bare = tmp.path();
            std::fs::create_dir_all(bare.join(dir)).unwrap();
            std::fs::create_dir_all(bare.join("elsewhere")).unwrap();
            std::fs::create_dir_all(bare.join("shared")).unwrap();
            std::fs::write(bare.join("shared/f"), "x\n").unwrap();
            std::os::unix::fs::symlink(bare.join("elsewhere"), bare.join(dir).join(sub)).unwrap();
            let inside = bare.join(dir);
            let given = inside.join(sub).join(file);
            assert!(
                check_no_destination(&inside, &given, file).is_err(),
                "{dir} as given"
            );
            let tmp = tempfile::tempdir().unwrap();
            let bare = tmp.path();
            std::fs::create_dir_all(bare.join(dir).join(sub)).unwrap();
            std::fs::create_dir_all(bare.join("shared")).unwrap();
            std::fs::write(bare.join("shared/f"), "x\n").unwrap();
            std::os::unix::fs::symlink(bare.join("shared/f"), bare.join(dir).join(sub).join(file))
                .unwrap();
            std::os::unix::fs::symlink(bare.join(dir).join(sub), bare.join("alias")).unwrap();
            let inside = bare.join(dir);
            let landing = inside.join("../alias").join(file);
            assert!(
                check_no_destination(&inside, &landing, file).is_err(),
                "{dir} landing"
            );
        }
    }

    /// The worktree exception's own judgment of the names below a worktree
    /// is made from every base: from below the work tree's root, run from the
    /// worktree's `specs/` and from the main checkout's, and with no
    /// repository from a project root inside a worktree's own `.claude`.
    #[test]
    fn the_worktree_exception_judges_from_each_base() {
        let tmp = tempfile::tempdir().unwrap();
        let main = tmp.path();
        git2::Repository::init(main).unwrap();
        let worktree = main.join(".claude/worktrees/wt");
        std::fs::create_dir_all(worktree.join(".claude")).unwrap();
        std::fs::create_dir_all(worktree.join("specs")).unwrap();
        std::fs::create_dir_all(main.join("specs")).unwrap();
        std::fs::write(
            worktree.join(".git"),
            "gitdir: ../../../.git/worktrees/wt\n",
        )
        .unwrap();
        let below = worktree.join("specs");
        let dest = below.join("../.claude/settings.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
        let below = main.join("specs");
        let dest = below.join("../.claude/worktrees/wt/.claude/settings.json");
        assert!(refused(&check_destination(&below, &dest, "x")));

        let tmp = tempfile::tempdir().unwrap();
        let inside = tmp.path().join(".claude/worktrees/wt/.claude");
        std::fs::create_dir_all(&inside).unwrap();
        assert!(refused(&judge(&inside, "settings.json")));
    }

    /// A path spelled otherwise than the project root spells it is judged as
    /// given from every base: through `sub/..` and through a `..` out of a
    /// linked commands directory, read lexically, with a repository at the
    /// project root and from below the work tree's root, and with no
    /// repository from a project root inside `.claude`.
    #[cfg(unix)]
    #[test]
    fn a_spelled_path_is_judged_as_given_from_each_base() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join(".claude")).unwrap();
        std::fs::create_dir_all(tree.join("sub")).unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::fs::create_dir_all(tree.join("shared/cmds/acme")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", tree.join(".claude/commands")).unwrap();
        for rel in [
            "sub/../.claude/commands/acme/g.md",
            ".claude/commands/acme/../acme/g.md",
        ] {
            assert_eq!(
                check_destination(tree, &tree.join(rel), rel).unwrap(),
                Some(HostRow::CommandOrSkill),
                "{rel}"
            );
        }
        let below = tree.join("specs");
        let dest = below.join("../.claude/commands/acme/../acme/g.md");
        assert_eq!(
            check_destination(&below, &dest, "x").unwrap(),
            Some(HostRow::CommandOrSkill)
        );

        let tmp = tempfile::tempdir().unwrap();
        let bare = tmp.path();
        std::fs::create_dir_all(bare.join(".claude")).unwrap();
        std::fs::create_dir_all(bare.join("shared/cmds/acme")).unwrap();
        std::os::unix::fs::symlink("../shared/cmds", bare.join(".claude/commands")).unwrap();
        let inside = bare.join(".claude");
        let dest = inside.join("commands/acme/../acme/g.md");
        assert_eq!(
            check_destination(&inside, &dest, "x").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// Through the root's link to another config directory, a `worktrees`
    /// directory there is that directory's, not a Claude Code worktree.
    #[cfg(unix)]
    #[test]
    fn the_worktree_exception_is_narrow_through_a_linked_config_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        git2::Repository::init(tree).unwrap();
        std::fs::create_dir_all(tree.join("cfg")).unwrap();
        std::os::unix::fs::symlink("cfg", tree.join(".agents")).unwrap();
        assert!(refused(&judge(tree, "cfg/worktrees/wt/settings.json")));
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        let below = tree.join("specs");
        let dest = below.join("../cfg/worktrees/wt/settings.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
    }

    /// A worktree's own name is judged as a name under the worktree's root,
    /// so a directory under `.claude/worktrees/` named for a config directory
    /// is one: its settings file and a skill Claude's layout has no row for
    /// are refused, and a command is a row, whose frontmatter the caller
    /// judges.
    #[test]
    fn a_worktree_name_is_judged_as_a_name() {
        let tmp = tempfile::tempdir().unwrap();
        let main = tmp.path();
        git2::Repository::init(main).unwrap();
        std::fs::create_dir_all(main.join(".claude/worktrees")).unwrap();
        assert!(refused(&judge(
            main,
            ".claude/worktrees/.claude/settings.json"
        )));
        assert!(refused(&judge(
            main,
            ".claude/worktrees/.agents/settings.json"
        )));
        assert!(refused(&judge(
            main,
            ".claude/worktrees/.claude/skills/x/SKILL.md"
        )));
        assert_eq!(
            judge(main, ".claude/worktrees/.claude/commands/acme/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// The exception is Claude Code's alone and holds a worktree's contents,
    /// not a file beside the worktrees: `worktrees` under another config
    /// directory is that directory's, and a file directly in
    /// `.claude/worktrees/` is `.claude`'s.
    #[test]
    fn the_worktree_exception_is_narrow() {
        let tmp = tempfile::tempdir().unwrap();
        let main = tmp.path();
        git2::Repository::init(main).unwrap();
        assert!(refused(&judge(main, ".agents/worktrees/wt/notes.md")));
        assert!(refused(&judge(main, ".claude/worktrees/notes.md")));
    }

    /// The same holds with no repository at all: the working directory's own
    /// names carry the worktree directory, and the names below it are judged
    /// afresh.
    #[test]
    fn a_claude_worktree_outside_any_repository_is_its_own_project() {
        let tmp = tempfile::tempdir().unwrap();
        let worktree = tmp.path().join(".claude/worktrees/wt");
        std::fs::create_dir_all(&worktree).unwrap();
        assert_eq!(judge(&worktree, "specs/001-x/spec.md").unwrap(), None);
        assert!(refused(&judge(&worktree, ".claude/settings.json")));
    }

    /// A repository nested inside a config directory hides nothing: `.claude`
    /// itself a repository, or a submodule whose `.git` is a file, is judged
    /// from the outermost work tree.
    #[test]
    fn a_repository_inside_a_config_directory_hides_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let main = tmp.path();
        git2::Repository::init(main).unwrap();
        let claude = main.join(".claude");
        std::fs::create_dir_all(&claude).unwrap();
        git2::Repository::init(&claude).unwrap();
        assert!(refused(&judge(&claude, "settings.local.json")));
        let agents = main.join(".agents");
        std::fs::create_dir_all(&agents).unwrap();
        std::fs::write(agents.join(".git"), "gitdir: ../.git/modules/agents\n").unwrap();
        assert!(refused(&judge(&agents, "settings.json")));
    }

    /// A repository's work tree is found by its `.git` entry, whether or not
    /// it can be opened: a `.git` file pointing nowhere still marks the work
    /// tree a run below it is judged from.
    #[test]
    fn a_work_tree_is_found_whether_or_not_it_opens() {
        let tmp = tempfile::tempdir().unwrap();
        let tree = tmp.path();
        std::fs::write(tree.join(".git"), "gitdir: /nonexistent\n").unwrap();
        std::fs::create_dir_all(tree.join("specs")).unwrap();
        std::fs::create_dir_all(tree.join(".claude")).unwrap();
        let below = tree.join("specs");
        let dest = below.join("../.claude/settings.local.json");
        assert!(refused(&check_destination(&below, &dest, "x")));
    }

    /// When the judgments disagree on a row, a command row wins, so a file
    /// the host reads as a command has its frontmatter judged.
    #[cfg(unix)]
    #[test]
    fn a_command_verdict_is_preferred() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join(".agents")).unwrap();
        std::fs::create_dir_all(repo.join(".claude/commands/acme")).unwrap();
        std::os::unix::fs::symlink("../.claude/commands/acme", repo.join(".agents/rules")).unwrap();
        assert_eq!(
            judge(repo, ".agents/rules/g.md").unwrap(),
            Some(HostRow::CommandOrSkill)
        );
    }

    /// A row is matched whatever the case of its names, since a file system
    /// that is not case-sensitive resolves a path through an existing file
    /// or directory to that one's case.
    #[test]
    fn rows_match_in_any_case() {
        let names = |parts: &[&str]| parts.iter().map(|p| (*p).to_owned()).collect::<Vec<_>>();
        for (layout, parts, kind) in [
            (
                Layout::ClaudeStyle,
                names(&["commands", "DUCTUS.md"]),
                HostRow::CommandOrSkill,
            ),
            (
                Layout::ClaudeStyle,
                names(&["Commands", "acme", "Status.MD"]),
                HostRow::CommandOrSkill,
            ),
            (
                Layout::Antigravity,
                names(&["skills", "acme-status", "skill.md"]),
                HostRow::CommandOrSkill,
            ),
            (
                Layout::Antigravity,
                names(&["Rules", "r.MD"]),
                HostRow::RulesMirror,
            ),
            (
                Layout::Pi,
                names(&["prompts", "acme-status.MD"]),
                HostRow::CommandOrSkill,
            ),
            (
                Layout::Pi,
                names(&["Extensions", "DUCTUS.ts"]),
                HostRow::PiBridge,
            ),
        ] {
            assert_eq!(row(layout, &parts), Some(kind), "{parts:?}");
        }
    }
}
