//! `check-orphaned-references` — adopter-owned files pointing at paths that
//! no longer exist.
//!
//! A migration that moves a framework-owned path can re-point the files it
//! owns. It cannot re-point the ones it does not: an adopter-owned file is
//! `create`-strategy, so the manifest never overwrites it, and unpinned, so the
//! pinned-invoker warning never fires. A migration is the only mechanism that
//! can follow a move into one, and each migration knows only its own hop.
//!
//! Two real instances surfaced in the adopter bootstrap (048 AC10), and both
//! were silent in the way this project pays most for — nothing errored. A
//! dangling `@import` yields a constitution that is simply not loaded, and a
//! hook calling a missing script fails at commit time, far from the run that
//! broke it.
//!
//! Read-only, and it **reports rather than repairs**: the adopter may have
//! hand-edited the reference, and a rewrite that guessed wrong is worse than a
//! report that is precise. Repair happens elsewhere, where an endpoint is
//! known: a migration re-points the references it authored, and
//! `rewrite-spec-links` re-points any reference to a feature directory that a
//! fold or a consolidation is retiring, since that command knows both ends.
//! The rewrite walks this check's own referrer list with this check's own
//! token scanner — [`referrers`] and [`managed_path_spans`] — so a reference
//! to the retiring directory that this check would report is one the rewrite
//! re-points, short of a referrer the rewrite lists in its `skipped` and
//! leaves as written: a symbolic link, which it does not follow and this
//! check reads through; one reached through a directory that resolves
//! outside the project, which this check also reads; and one that is not
//! UTF-8 text, which this check skips too, in the same words
//! ([`UNREADABLE_AS_TEXT`]).
//!
//! Defined by `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`,
//! required by `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md`.

use std::collections::BTreeSet;
use std::ops::Range;
use std::path::Path;

use crate::primitives::Result;
use crate::schema::config_keys;
use crate::schema::paths;
use crate::schema::primitives::{
    CheckOrphanedReferencesArgs, CheckOrphanedReferencesResult, OrphanedReference,
    OrphanedReferenceSkip,
};

/// Adopter-owned files that carry references into ductus-managed paths.
///
/// Enumerated here rather than derived from the **Shared Files** manifest
/// because that manifest lives in `framework/bootstrap/ductus.md`, which is a
/// *this-repo* artifact — it is fetched into staging during a run and never
/// installed, so `check_artifacts::adopter_destinations` returns an empty set
/// in the adopter checkout where this check does its work. These four are the
/// `create`-strategy destinations a migration has actually orphaned:
/// `constitution-relocate` wrote the constitution reference into the first
/// three, and `.githooks/pre-commit` is the adopter-owned invoker that
/// `govern-dir-consolidate` and `ductus-rename` moved generators out from
/// under.
///
/// A fifth referrer — the spec root's `system.md` — is appended by
/// [`referrers`], because it is the one that must be resolved against config
/// rather than spelled literally.
const FIXED_REFERRERS: &[&str] = &[
    "CLAUDE.md",
    "AGENTS.md",
    "README.md",
    ".githooks/pre-commit",
];

/// The referrer set for this repo, with the spec root resolved.
///
/// `system.md` is `create`-strategy and adopter-owned exactly like the four
/// above, but nothing had it in scope on either side: no migration step names
/// it (`ductus-rename` step 3 re-points the constitution reference in
/// `CLAUDE.md`, `AGENTS.md` and `README.md` only), and it was absent from this
/// list — so a run completed clean while the file pointed at a directory that
/// no longer existed. Found on a live adopter bootstrap by a host reading the
/// file, which is the only thing that could have found it.
///
/// The asymmetry with the other four is deliberate and stays: a **migration**
/// repairs only references the framework authored, so an adopter's own
/// reference to a moved framework path is reported and left alone. Reporting
/// is the whole of the fix — it is what stops the run looking clean over a
/// reference already known to be broken.
///
/// One class is repaired whoever wrote it: a reference to a feature directory
/// that `/{project}:fold` or `/{project}:consolidate` is retiring. That command
/// knows both endpoints, so `rewrite-spec-links` re-points the reference in
/// every file this list names, `system.md` included, short of one it lists in
/// its `skipped`: a symbolic link, which it does not follow; one reached
/// through a directory that resolves outside the project, which it does not
/// write; and one that is not UTF-8 text. That is why the list is shared
/// rather than copied: a referrer added here is one the rewrite covers too,
/// and a copy would let the two drift apart silently.
///
/// Resolved through `[paths] specs-root` rather than hardcoded. A literal
/// `specs/system.md` would silently examine nothing on a project that renamed
/// its root — the same exit-0-over-a-wrong-path failure `config_path_of`
/// produced in the shipped `specs-root.sh`.
pub(crate) fn referrers(layout: &paths::Paths) -> Vec<String> {
    let mut list: Vec<String> = FIXED_REFERRERS
        .iter()
        .map(|referrer| (*referrer).to_string())
        .collect();
    list.push(format!("{}system.md", spec_root_prefix(layout)));
    list
}

/// The prefixes that name the project root from `referrer`, a project-relative
/// path from [`referrers`], so that a managed path behind one is the same
/// reference as the path written bare (`051-branch-scoped-spec-numbering`
/// task 30).
///
/// A markdown file at the project root reaches the root with `./`, and with a
/// leading `/`, which is how a repository host resolves a link; `../` from
/// there leaves the project. `system.md`, one level down under the spec root,
/// reaches it with `../` and a leading `/`; its `./` names the spec root
/// itself. The hook takes `./`, since git runs a hook from the working tree's
/// root, where its bare paths resolve too; a leading `/` there is an absolute
/// file-system path. Every referrer is one of those three kinds.
///
/// A host resolves a leading `/` from the repository's root, so it names the
/// project root only when the project is its work tree's root
/// (`at_work_tree_root`, from [`project_is_work_tree_root`]); in a project in a
/// repository subdirectory (spec 059) it is no prefix.
pub(crate) fn root_prefixes(referrer: &str, at_work_tree_root: bool) -> Vec<&'static str> {
    let markdown = Path::new(referrer)
        .extension()
        .is_some_and(|ext| ext == "md");
    let prefixes: &[&str] = match (markdown, referrer.matches('/').count()) {
        (true, 0) => &["./", "/"],
        (true, 1) => &["../", "/"],
        _ => &["./"],
    };
    prefixes
        .iter()
        .copied()
        .filter(|prefix| at_work_tree_root || *prefix != "/")
        .collect()
}

/// Whether `repo`, the project root, is its git work tree's root. A project
/// in no repository has no host to resolve a leading `/` from another root,
/// so it counts as one.
pub(crate) fn project_is_work_tree_root(repo: &Path) -> bool {
    super::ProjectRepository::discover(repo).map_or(true, |project| project.is_work_tree_root())
}

/// The configured spec root as a path prefix, with its trailing `/` —
/// `specs/` by default. One spelling for the managed root this check matches,
/// the `system.md` referrer it reads, and the prefix `rewrite-spec-links`
/// re-points behind.
pub(crate) fn spec_root_prefix(layout: &paths::Paths) -> String {
    format!("{}/", layout.specs_root.trim_end_matches('/'))
}

/// The `skipped` reason for a referrer that exists but could not be read as
/// UTF-8 text — here, any failure to read it. `rewrite-spec-links` records the
/// same reason for a referrer whose bytes are not UTF-8, so the two
/// primitives name that file in the same words; any other read failure is an
/// error there, raised before it writes anything.
pub(crate) const UNREADABLE_AS_TEXT: &str = "file exists but could not be read as UTF-8 text";

/// Attribution modes, reported so a caller can tell one from the other.
const ATTRIBUTION_REGISTRY: &str = "registry";
const ATTRIBUTION_WATERMARK: &str = "watermark";

/// Execute the `check-orphaned-references` primitive.
///
/// # Errors
///
/// None — every failure is a recorded `skipped` entry rather than a raised
/// error. A check that dies on an unreadable referrer tells the caller nothing
/// about the referrers it *could* read, and this one runs inside `/ductus`'s
/// migration batch, which must not be aborted by a file it could not open.
pub fn run(
    _args: &CheckOrphanedReferencesArgs,
    repo: &Path,
) -> Result<CheckOrphanedReferencesResult> {
    // Loaded once and shared: both the referrer list and the managed roots are
    // derived from the configured spec root, and reading the config twice to
    // answer one question is work with no second answer behind it.
    let layout = paths::Paths::load(repo);
    let roots = managed_roots(&layout);
    let at_work_tree_root = project_is_work_tree_root(repo);
    let registry = read_registry(repo);
    // Paths this repo declares it *ships into* an adopter rather than holds.
    // In an adopter checkout the manifest is not installed, so this is empty
    // and every managed path is checked for real. In the framework repo it is
    // populated, and without it every destination row — `.ductus/constitution.md`,
    // `specs/system.md` — would be reported missing here, which is true and
    // completely uninteresting: those live at their source paths and are
    // written on the way out. Same distinction `criterion-path-existence`
    // draws, using the same two helpers rather than a second copy of the rule.
    let ships_elsewhere = super::check_artifacts::adopter_destinations(repo);
    let mut findings = Vec::new();
    let mut examined = Vec::new();
    let mut skipped = Vec::new();

    for referrer in referrers(&layout) {
        let full = repo.join(&referrer);
        if !full.exists() {
            // Not every project has every referrer. An absent file is not an
            // unexamined one — there is nothing there to examine — so it is
            // neither a finding nor a skip.
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&full) else {
            skipped.push(OrphanedReferenceSkip {
                path: referrer.clone(),
                reason: UNREADABLE_AS_TEXT.into(),
            });
            continue;
        };
        examined.push(referrer.clone());
        let prefixes = root_prefixes(&referrer, at_work_tree_root);
        for (idx, line) in text.lines().enumerate() {
            for target in managed_paths_in(line, &roots, &prefixes) {
                if repo.join(target.trim_end_matches('/')).exists() {
                    continue;
                }
                if super::check_artifacts::ships_to_adopter(&ships_elsewhere, &target) {
                    continue;
                }
                findings.push(OrphanedReference {
                    referrer: referrer.clone(),
                    line: u32::try_from(idx + 1).unwrap_or(u32::MAX),
                    migration: registry
                        .as_ref()
                        .and_then(|entries| attribute(entries, &target))
                        .unwrap_or_default(),
                    target,
                });
            }
        }
    }

    Ok(CheckOrphanedReferencesResult {
        findings,
        examined,
        skipped,
        matched_prefixes: roots.clone(),
        attribution: if registry.is_some() {
            ATTRIBUTION_REGISTRY.to_string()
        } else {
            ATTRIBUTION_WATERMARK.to_string()
        },
        last_applied: read_last_applied(repo),
    })
}

// -- managed roots -------------------------------------------------------------

/// Path prefixes that mark a reference as framework-owned.
///
/// Scoped deliberately: a broken link to `docs/design.md` is the adopter's
/// business, and flagging it would make the check noise. The spec root is read
/// from config rather than assumed, so a project that configured one is checked
/// against its own layout.
///
/// **Historical roots are included, and that is the point.** The orphan a
/// migration chain leaves behind is a reference to the path *before* the move,
/// so it carries the *old* root — and a list of only current roots is blind to
/// exactly the class this check exists to catch. Observed on the adopter
/// bootstrap (048 AC10): after the generators moved root `scripts/` →
/// `.govern/scripts/` → `.ductus/scripts/`, the adopter's `AGENTS.md` still
/// named `scripts/gen-spec-deps.sh` and this check reported clean, because
/// `scripts/` matched no root. The operator found it by reading the file.
///
/// The pre-042 generator prefixes are deliberately narrower than `scripts/`:
/// the framework owned `scripts/gen-*.sh` and `scripts/lib/`, never the whole
/// directory, so a root of `scripts/` would flag an adopter's own
/// `scripts/build.sh` — noise this check must not produce.
fn managed_roots(layout: &paths::Paths) -> Vec<String> {
    let mut roots = vec![
        ".ductus/".to_string(),
        ".githooks/".to_string(),
        // Historical: the 042-era per-project directory, retired by `049`.
        ".govern/".to_string(),
        // Historical: the pre-042 generator locations, by prefix rather than
        // by directory, so adopter-owned scripts stay out of scope.
        "scripts/gen-".to_string(),
        "scripts/lib/".to_string(),
    ];
    roots.push(spec_root_prefix(layout));
    roots.sort();
    roots.dedup();
    roots
}

/// Every managed path named on one line, deduplicated, in the sorted set's
/// order.
fn managed_paths_in(line: &str, roots: &[String], prefixes: &[&str]) -> BTreeSet<String> {
    managed_path_spans(line, roots, prefixes)
        .into_iter()
        .map(|span| line[span].to_string())
        .collect()
}

/// The byte range of every managed path named on one line, ordered by where
/// it starts.
///
/// Scans for a managed-root prefix and takes the path-character run that
/// follows, rather than parsing markdown link / code-span / shell grammars
/// separately: the same reference appears in all three forms across the
/// referrers (`@.ductus/constitution.md`, `[c](.ductus/constitution.md)`,
/// `bash .ductus/scripts/gen-spec-deps.sh`), and one scan handles them all
/// without a grammar per host format.
///
/// A range rather than the text, so `rewrite-spec-links` can splice a
/// replacement into the line at the place this check found it — the only way
/// it can rewrite exactly what this check would report. Replacing the text
/// instead would lose the token-start rules below and re-point
/// `specs/1234.1-a-cache/x` on a line that also names `specs/1234.1-a/`.
/// Ranges from one root never overlap, because a second occurrence inside a
/// token is preceded by a path character and so is never a reference; two
/// roots can yield one range twice, which [`managed_paths_in`] deduplicates.
///
/// `prefixes` are the ones naming the project root from the referrer the
/// line comes from ([`root_prefixes`]). A root behind one starts a reference
/// when the prefix itself starts a token — when it does not continue another
/// path ([`continues_another_path`]) — so the rules below are applied where
/// the prefix begins; the range still starts at the root, so the existence
/// test sees the path as if written bare and `rewrite-spec-links` keeps the
/// prefix as written. Any other text glued before a root — another prefix,
/// or a word — keeps the root from starting a reference.
pub(crate) fn managed_path_spans(
    line: &str,
    roots: &[String],
    prefixes: &[&str],
) -> Vec<Range<usize>> {
    let mut found = Vec::new();
    for root in roots {
        let mut from = 0usize;
        while let Some(rel) = line[from..].find(root.as_str()) {
            let start = from + rel;
            let token = start - root_prefix_before(&line[..start], prefixes);
            // A reference must start a token: `x.ductus/y` is not one to
            // `.ductus/y`, and matching inside one would invent paths. An `@`
            // that itself starts a token is the `@path` import form, so it is
            // a reference; one glued to a word (`x@.ductus/y`) is not. A git
            // object name is not one either — see `names_a_git_object`.
            let previous = line[..token].chars().next_back();
            let not_a_reference = previous.is_some_and(is_path_char)
                || (token < start && continues_another_path(&line[..token]))
                || (previous == Some('@')
                    && line[..token - 1]
                        .chars()
                        .next_back()
                        .is_some_and(|c| !c.is_whitespace()))
                || (previous == Some(':') && names_a_git_object(&line[..token - 1]));
            from = start + root.len();
            if not_a_reference {
                continue;
            }
            let end = start
                + line[start..]
                    .char_indices()
                    .take_while(|(_, c)| is_path_char(*c))
                    .map(|(i, c)| i + c.len_utf8())
                    .last()
                    .unwrap_or(0);
            let candidate = trim_trailing_punctuation(&line[start..end]);
            if candidate.len() > root.len() && !is_pattern(candidate) && stays_in_repo(candidate) {
                found.push(start..start + candidate.len());
            }
        }
    }
    found.sort_by_key(|span| span.start);
    found
}

/// Whether `before`, the text ahead of a root prefix, ends another path, so
/// the prefix continues it rather than naming the project root: a path
/// character, or the end of a home or variable directory — `~` in `~/`, `}`
/// in `${HOME}/`, `%` in `%USERPROFILE%/`, `)` in `$(pwd)/` — written bare or
/// closed by a quote or backtick (`"$HOME"/`, `` `pwd`/ ``). Anything else,
/// markup and shell operators among it (`|a|./x`, `<td>/x`, `2>./x`), leaves
/// the prefix starting a token.
fn continues_another_path(before: &str) -> bool {
    let ends_a_path = |c: char| is_path_char(c) || matches!(c, '~' | '}' | '%' | ')');
    let mut back = before.chars().rev();
    match back.next() {
        Some('"' | '\'' | '`' | '\u{201d}' | '\u{2019}') => back.next().is_some_and(ends_a_path),
        Some(c) => ends_a_path(c),
        None => false,
    }
}

/// The length of the longest of `prefixes` that `before` ends with, or zero.
/// The longest, because `../` ends with `./` and `/` — a shorter one would
/// leave a `.` before it, a path character, and the token unrecognised.
fn root_prefix_before(before: &str, prefixes: &[&str]) -> usize {
    prefixes
        .iter()
        .filter(|prefix| before.ends_with(**prefix))
        .map(|prefix| prefix.len())
        .max()
        .unwrap_or(0)
}

/// Whether the colon that ends `before` joins a git revision to the path after
/// it, making `<rev>:<path>` (or `:<path>`, the index) an object name rather
/// than a working-tree reference.
///
/// An object name resolves through git's object store, where no migration can
/// move it, so reporting its path as missing from the working tree is a false
/// finding. A colon alone does not decide it: a colon-joined path list —
/// `PATH="$PATH:.ductus/bin"` in a hook — names a working-tree directory, and
/// skipping it would hide exactly the orphan this check exists to report. So
/// the token before the colon decides. Empty is the index form. A `$` or `%`
/// marks a variable, and a leading `/` or `.` marks a path, since a refname
/// cannot start with either; those are lists, and their entries are checked.
/// A trailing `]` is a markdown link label — the `CommonMark` spec lets a
/// reference definition omit the space, `[old]:specs/x.md` — and a label's
/// target is a working-tree path, so it is checked too. Anything else is read
/// as a revision — `abc1234`, `HEAD~2`, `origin/main`, `v0.58.0`. A list entry
/// after a bare word (`bin:.ductus/x`) is therefore skipped, because a branch
/// may be named `bin` and nothing on the line says otherwise.
fn names_a_git_object(before: &str) -> bool {
    let token_start = before
        .char_indices()
        .rev()
        .find(|(_, c)| {
            c.is_whitespace() || matches!(c, '`' | '\'' | '"' | '=' | '(' | '[' | ',' | ';')
        })
        .map_or(0, |(i, c)| i + c.len_utf8());
    let token = &before[token_start..];
    token.is_empty()
        || !(token.contains(['$', '%']) || token.starts_with(['/', '.']) || token.ends_with(']'))
}

/// Characters that continue a path token. `*` is included so a glob is
/// captured *whole* and can then be rejected by [`is_pattern`] — dropping it
/// from the token instead would leave `.ductus/scripts/gen-` behind, a
/// prose fragment reported as a missing file.
fn is_path_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/' | '*')
}

/// Whether a candidate is a *pattern* rather than a reference to one file.
///
/// Documentation names shapes as often as it names files —
/// `` `specs/NNN-*/spec.md` ``, `` `.ductus/scripts/gen-*.sh` ``,
/// `specs/NNN-feature/scenarios/slug.md`. None of those is expected to exist,
/// so testing them against the filesystem manufactures findings out of prose.
/// Both markers earned their place against real lines in this repo's own
/// `AGENTS.md` and `README.md`, which is where the false positives showed up.
fn is_pattern(candidate: &str) -> bool {
    candidate.contains('*') || candidate.contains("NNN")
}

/// Whether a candidate stays inside the repo.
///
/// The candidate comes from a file's *contents*, and `.` and `/` are both path
/// characters, so `.ductus/../../elsewhere` matches a managed root and would be
/// joined onto the repo root unexamined. Nothing is opened — the check is
/// existence-only — so this is not an exploit so much as a category error: a
/// path outside the repo is not a ductus-managed path, and reporting it as a
/// broken managed reference would be a finding about something this check does
/// not govern. `apply-manifest` already refuses a destination that escapes the
/// repo root; same rule, applied where paths enter rather than where they are
/// written.
fn stays_in_repo(candidate: &str) -> bool {
    !candidate.split('/').any(|segment| segment == "..")
}

/// Strip sentence and markup punctuation a path can pick up from prose.
///
/// Only trailing, and only characters that cannot end a real path: a `.md`
/// suffix survives because `d` is not punctuation, while
/// `` `.ductus/constitution.md`. `` loses both the fence and the full stop.
fn trim_trailing_punctuation(candidate: &str) -> &str {
    candidate.trim_end_matches(['.', ',', ';', ':', ')', '`', '"', '\'', '*'])
}

// -- attribution ---------------------------------------------------------------

/// `framework/migrations.toml`, reduced to what attribution needs.
///
/// Typed deserialization rather than a `toml::Value` walk, matching
/// `schema::paths`' idiom: unknown keys on either the file or an entry are
/// ignored, so the registry can grow fields without breaking this reader.
#[derive(serde::Deserialize)]
struct RegistryFile {
    #[serde(default)]
    migrations: Vec<RegistryEntry>,
}

/// One registry entry reduced to what attribution needs.
#[derive(serde::Deserialize)]
struct RegistryEntry {
    id: String,
    #[serde(default)]
    target_paths: Vec<String>,
}

/// The active config file, reduced to the migration watermark.
#[derive(serde::Deserialize)]
struct ConfigFile {
    migrations: Option<MigrationsSection>,
}

/// `[migrations]` — absent entirely for an adopter who has never migrated.
#[derive(serde::Deserialize)]
struct MigrationsSection {
    #[serde(default)]
    last_applied: String,
}

/// Parse `framework/migrations.toml` into `(id, target_paths)` pairs, or `None`
/// when it is absent or will not parse.
///
/// `None` is what drives `attribution: "watermark"`. A missing registry and an
/// unparseable one both land here, and nothing distinguishes them for the
/// caller: both read `watermark`, and neither adds a `skipped` entry.
fn read_registry(repo: &Path) -> Option<Vec<RegistryEntry>> {
    let text = std::fs::read_to_string(repo.join("framework/migrations.toml")).ok()?;
    let parsed: RegistryFile = toml::from_str(&text).ok()?;
    Some(parsed.migrations)
}

/// The entry whose `target_paths` covers `target`, if any.
///
/// The **last** match wins: entries are registry-ordered, oldest first, and
/// when several hops touched a path it is the most recent one that left the
/// reference dangling. `constitution-relocate` and `ductus-rename` both name
/// the constitution; the rename is the one to report.
/// An empty return under `attribution: "registry"` means *the registry was
/// read and no entry claims this path* — a different statement from the empty
/// `migration` under `watermark`, which means *no registry was available to
/// ask*. `attribution` is what separates them; neither is a migration named "".
fn attribute(entries: &[RegistryEntry], target: &str) -> Option<String> {
    entries
        .iter()
        .rfind(|entry| {
            entry
                .target_paths
                .iter()
                .any(|claimed| covers(claimed, target))
        })
        .map(|entry| entry.id.clone())
}

/// Whether a registry `target_paths` entry covers a referenced path — exact
/// match, or a directory/glob prefix (`\.ductus/scripts/` covers
/// `.ductus/scripts/gen-spec-deps.sh`).
fn covers(claimed: &str, target: &str) -> bool {
    let claimed = claimed.trim_end_matches('*');
    if claimed == target {
        return true;
    }
    // One prefix test, not two: the trailing-slash form (`.ductus/`) and the
    // bare form (`.ductus`) both normalize to the same directory here, so the
    // second disjunct this used to carry could never fire on an input the
    // first did not already match.
    let dir = claimed.trim_end_matches('/');
    target.starts_with(&format!("{dir}/"))
}

// -- watermark -----------------------------------------------------------------

/// `[migrations].last_applied` from the active config file, or empty.
///
/// Empty means *no migration has been applied* — the absent-section case the
/// bootstrap treats as null. It is never rendered as a migration name; the
/// caller distinguishes it from an id by its emptiness.
fn read_last_applied(repo: &Path) -> String {
    let path = paths::config_path(repo);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return String::new();
    };
    config_keys::warn_unknown_keys(&path, &text, &["migrations"]);
    let Ok(parsed) = toml::from_str::<ConfigFile>(&text) else {
        return String::new();
    };
    parsed
        .migrations
        .map(|section| section.last_applied)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    /// The shared registry declares every key this reader reads, so none of
    /// them is reported as unknown (spec 019).
    #[test]
    fn the_registry_recognizes_every_key_this_reader_reads() {
        use crate::schema::config_keys::{fields_of, unrecognized};
        assert_eq!(
            unrecognized(&[], fields_of::<ConfigFile>()),
            [] as [String; 0]
        );
        assert_eq!(
            unrecognized(&["migrations"], fields_of::<MigrationsSection>()),
            [] as [String; 0]
        );
    }

    fn write(path: &Path, body: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }

    fn args() -> CheckOrphanedReferencesArgs {
        CheckOrphanedReferencesArgs {}
    }

    /// An adopter checkout: no registry (it is never installed), a config
    /// carrying a migration watermark, and a constitution that exists.
    fn adopter() -> TempDir {
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[host]\nproject = \"ductus\"\n\n[migrations]\nlast_applied = \"ductus-rename\"\n",
        );
        write(
            &tmp.path().join(".ductus/constitution.md"),
            "# Constitution\n",
        );
        tmp
    }

    #[test]
    fn the_real_adopter_defect_is_reported() {
        // The real instance: `constitution-relocate` wrote a `.govern/`
        // reference into CLAUDE.md and `ductus-rename` moved the file, leaving
        // an @import that resolves to nothing and loads no constitution.
        let tmp = adopter();
        write(
            &tmp.path().join("CLAUDE.md"),
            "# CLAUDE.md\n\n@import .govern/constitution.md\n",
        );
        // `.govern/` is a *historical* managed root and is matched, so the
        // `@import` above is reported directly. It did not used to be, and this
        // test worked around that by relying on a `.ductus/` path instead — the
        // workaround was the blind spot, found on the 048 AC10 adopter run.
        write(
            &tmp.path().join("AGENTS.md"),
            "See `.ductus/constitution.md` and `.ductus/scripts/gen-spec-deps.sh`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(
            targets,
            vec![
                ".govern/constitution.md",
                ".ductus/scripts/gen-spec-deps.sh"
            ],
            "the retired-root @import and the missing generator both report; \
             the current constitution resolves: {:?}",
            result.findings
        );
        assert_eq!(result.findings[0].referrer, "CLAUDE.md");
        assert_eq!(result.findings[1].referrer, "AGENTS.md");
        assert_eq!(result.findings[1].line, 1);
    }

    #[test]
    fn the_pre_042_generator_path_a_chain_orphans_is_reported() {
        // The defect the AC10 adopter run surfaced: the generators moved root
        // `scripts/` -> `.govern/scripts/` -> `.ductus/scripts/`, so the stale
        // reference carries the *pre-migration* root. A current-roots-only list
        // cannot see it, which is why the run reported clean over a real orphan.
        let tmp = adopter();
        write(
            &tmp.path().join("AGENTS.md"),
            "Run `scripts/gen-spec-deps.sh` before committing.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(
            targets,
            vec!["scripts/gen-spec-deps.sh"],
            "{:?}",
            result.findings
        );
    }

    #[test]
    fn an_adopters_own_script_is_not_flagged_by_the_historical_prefixes() {
        // The reason the pre-042 roots are `scripts/gen-` and `scripts/lib/`
        // rather than `scripts/`: the framework owned those, never the whole
        // directory. A bare `scripts/` root would turn every adopter script
        // that fails to resolve into a finding — noise this must not produce.
        let tmp = adopter();
        write(
            &tmp.path().join("README.md"),
            "Build with `scripts/build.sh`, release with `scripts/release.sh`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn a_clean_result_declares_the_prefixes_it_matched() {
        // `examined` bounds the claim by subject; `matched_prefixes` bounds it
        // by scope. A reference carrying no listed prefix is not reported as
        // skipped, because nothing recognized it as a reference — so a clean
        // verdict has to say which forms it looked for.
        let tmp = adopter();
        write(&tmp.path().join("AGENTS.md"), "Nothing managed here.\n");
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.findings, [] as [OrphanedReference; 0]);
        for expected in [
            ".ductus/",
            ".githooks/",
            ".govern/",
            "scripts/gen-",
            "scripts/lib/",
            "specs/",
        ] {
            assert!(
                result.matched_prefixes.iter().any(|p| p == expected),
                "{expected} missing from {:?}",
                result.matched_prefixes
            );
        }
    }

    #[test]
    fn an_adopter_checkout_reports_watermark_attribution_not_a_blank_migration() {
        // No registry is installed adopter-side, so attribution is impossible.
        // Saying so is the point: a blank `migration` rendered like an
        // attributed one is QUAL-CLAIM-001 inside the check.
        let tmp = adopter();
        write(
            &tmp.path().join("AGENTS.md"),
            "Run `.ductus/scripts/missing.sh`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.attribution, "watermark");
        assert_eq!(result.last_applied, "ductus-rename");
        assert_eq!(result.findings[0].migration, "");
    }

    #[test]
    fn the_registry_attributes_the_most_recent_hop() {
        // A registry readable in the project tree (a framework checkout).
        // Two entries name the constitution; the *rename* is the one that left the
        // reference dangling, so the last match wins.
        let tmp = adopter();
        write(
            &tmp.path().join("framework/migrations.toml"),
            "[[migrations]]\nid = \"constitution-relocate\"\ntarget_paths = [\".ductus/constitution.md\"]\n\n\
             [[migrations]]\nid = \"ductus-rename\"\ntarget_paths = [\".ductus/\"]\n",
        );
        write(
            &tmp.path().join("AGENTS.md"),
            "See `.ductus/constitution-old.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.attribution, "registry");
        assert_eq!(result.findings[0].migration, "ductus-rename");
    }

    #[test]
    fn a_clean_tree_is_distinguishable_from_an_unexamined_one() {
        // findings empty AND skipped empty is the only shape that means
        // "examined and clean"; `examined` quantifies the subject.
        let tmp = adopter();
        write(
            &tmp.path().join("AGENTS.md"),
            "See `.ductus/constitution.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
        assert_eq!(result.skipped, [] as [OrphanedReferenceSkip; 0]);
        assert_eq!(result.examined, vec!["AGENTS.md".to_string()]);
    }

    #[test]
    fn an_unreadable_referrer_is_skipped_not_silently_passed() {
        let tmp = adopter();
        // A directory at the referrer path: exists, cannot be read as text, on
        // every platform and without depending on file modes.
        fs::create_dir_all(tmp.path().join("README.md")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.skipped.len(), 1, "{:?}", result.skipped);
        assert_eq!(result.skipped[0].path, "README.md");
        assert!(!result.examined.contains(&"README.md".to_string()));
    }

    #[test]
    fn an_absent_referrer_is_neither_a_finding_nor_a_skip() {
        // Not every project has every referrer; nothing there is not the same
        // as something unexamined.
        let tmp = adopter();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.findings, [] as [OrphanedReference; 0]);
        assert_eq!(result.skipped, [] as [OrphanedReferenceSkip; 0]);
        assert_eq!(result.examined, [] as [String; 0]);
    }

    #[test]
    fn no_migrations_section_yields_an_empty_watermark_not_a_named_migration() {
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[host]\nproject = \"ductus\"\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.last_applied, "");
        assert_eq!(result.attribution, "watermark");
    }

    #[test]
    fn only_managed_roots_are_checked() {
        // A broken link into the adopter's own docs is their business; the
        // check would be noise if it flagged everything that fails to resolve.
        let tmp = adopter();
        write(
            &tmp.path().join("README.md"),
            "See [design](docs/design.md) and [notes](../elsewhere/notes.md).\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
        assert_eq!(result.examined, vec!["README.md".to_string()]);
    }

    #[test]
    fn the_configured_spec_root_is_honored() {
        // An adopter who configured a spec root gets checked against their own
        // layout, not an assumed `specs/`.
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[paths]\nspecs-root = \"governance\"\n",
        );
        write(
            &tmp.path().join("AGENTS.md"),
            "See `governance/inbox.md` and `specs/inbox.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(
            targets,
            vec!["governance/inbox.md"],
            "`specs/` is not this project's root, so it is not a managed path"
        );
    }

    #[test]
    fn documentation_patterns_are_not_reported_as_missing_files() {
        // Found by running this primitive against ductus's own repo: prose
        // names shapes as often as files, and testing a shape against the
        // filesystem manufactures findings out of documentation.
        let tmp = adopter();
        write(
            &tmp.path().join("AGENTS.md"),
            "Stage `specs/NNN-*/spec.md`, run `.ductus/scripts/gen-*.sh`, \
             see `specs/NNN-feature/scenarios/slug.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
        assert_eq!(result.examined, vec!["AGENTS.md".to_string()]);
    }

    #[test]
    fn a_path_this_repo_ships_elsewhere_is_not_reported_as_missing_here() {
        // Also found against ductus's own repo. `.ductus/constitution.md` is a
        // manifest *destination* — it is written into an adopter on the way
        // out and correctly absent from the framework checkout, so reporting
        // it here would be true and useless. In an adopter the manifest is not
        // installed, so this exemption is empty and real breakage still fires.
        let tmp = adopter();
        fs::remove_file(tmp.path().join(".ductus/constitution.md")).unwrap();
        write(
            &tmp.path().join("framework/bootstrap/ductus.md"),
            "| Source Path | Destination Path |\n| --- | --- |\n\
             | `framework/constitution.md` | `.ductus/constitution.md` |\n",
        );
        write(
            &tmp.path().join("AGENTS.md"),
            "@import .ductus/constitution.md\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            result.findings.is_empty(),
            "a declared adopter destination is not local breakage: {:?}",
            result.findings
        );
    }

    #[test]
    fn a_reference_that_escapes_the_repo_is_not_a_managed_path() {
        // `.` and `/` are both path characters, so a traversal matches a
        // managed-root prefix. A path outside the repo is not a ductus-managed
        // path, so it is not this check's business either way.
        let tmp = adopter();
        write(
            &tmp.path().join("AGENTS.md"),
            "See `.ductus/../../elsewhere/gone.md` and `.ductus/still-gone.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(
            targets,
            vec![".ductus/still-gone.md"],
            "only the in-repo reference is reported: {:?}",
            result.findings
        );
    }

    #[test]
    fn covers_matches_a_directory_entry_in_either_written_form() {
        // The registry writes `.ductus/` and `.ductus` interchangeably; both
        // must cover a file beneath it, and neither may cover a sibling whose
        // name merely starts with the same characters.
        assert!(covers(".ductus/", ".ductus/constitution.md"));
        assert!(covers(".ductus", ".ductus/constitution.md"));
        assert!(covers(".ductus/*", ".ductus/constitution.md"));
        assert!(covers(".ductus/constitution.md", ".ductus/constitution.md"));
        assert!(!covers(".ductus", ".ductus-cache/x.md"));
    }

    #[test]
    fn a_prefix_inside_a_longer_token_is_not_a_reference() {
        let tmp = adopter();
        write(
            &tmp.path().join("AGENTS.md"),
            "The path x.ductus/nope.md is not a reference.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn an_at_path_import_is_a_reference() {
        // Claude Code's import form is `@path`, at line start or after a
        // space; a dangling one loads nothing, so it must be reported.
        let tmp = adopter();
        write(
            &tmp.path().join("CLAUDE.md"),
            "# CLAUDE.md\n\n@.ductus/missing.md\nSee also @.ductus/gone.md\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let mut targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        targets.sort_unstable();
        assert_eq!(targets, vec![".ductus/gone.md", ".ductus/missing.md"]);
    }

    #[test]
    fn an_at_inside_a_word_is_not_an_import() {
        let tmp = adopter();
        write(
            &tmp.path().join("CLAUDE.md"),
            "Mail x@.ductus/y about it.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn a_revision_qualified_path_is_not_a_working_tree_reference() {
        // The real shape: a retired spec cited at the commit that last held
        // it. The bare spelling of the same missing path, on its own line,
        // is the control — it must still report, so the exclusion is the
        // glued colon and not the path.
        let tmp = adopter();
        write(
            &tmp.path().join("README.md"),
            "Recover it with `git show abc1234:specs/002-auth/spec.md`.\n\
             The index copy: `git show :specs/002-auth/plan.md`.\n\
             At a ref: `git show HEAD~2:.ductus/gone.md`, or \
             origin/main:specs/002-auth/tasks.md, or v0.58.0:specs/002-auth/data-model.md.\n\
             See: specs/002-auth/spec.md\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found: Vec<(&str, u32)> = result
            .findings
            .iter()
            .map(|f| (f.target.as_str(), f.line))
            .collect();
        assert_eq!(
            found,
            vec![("specs/002-auth/spec.md", 4)],
            "only the bare reference after `: ` is a working-tree claim"
        );
    }

    #[test]
    fn a_path_after_a_non_revision_colon_is_still_checked() {
        // The shape a bare-colon rule would hide: a hook extending PATH with a
        // managed directory. Each entry is a working-tree reference, and
        // skipping it would turn a moved directory back into a silent orphan.
        // Every line's token before the colon is one a revision cannot be.
        let tmp = adopter();
        write(
            &tmp.path().join(".githooks/pre-commit"),
            "#!/usr/bin/env bash\n\
             PATH=\"$PATH:.ductus/gone-a\"\n\
             PATH=${PATH}:.ductus/gone-b\n\
             PATH=/usr/local/bin:.ductus/gone-c\n\
             PATH=./bin:.ductus/gone-d\n",
        );
        // A link reference definition with no space after its colon is the
        // same claim in markdown: its target is a working-tree path.
        write(
            &tmp.path().join("README.md"),
            "[old]:specs/gone-e/spec.md\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(
            found,
            vec![
                "specs/gone-e/spec.md",
                ".ductus/gone-a",
                ".ductus/gone-b",
                ".ductus/gone-c",
                ".ductus/gone-d"
            ],
            "a variable, a path or a link label before the colon is not a revision"
        );
    }

    #[test]
    fn a_path_behind_a_prefix_naming_the_project_root_is_checked() {
        // 051 task 30: `./` or a leading `/` in a root markdown file, `./` in
        // the hook, and `../` or a leading `/` in `system.md` each name the
        // project root, so the path behind one is the same reference as the
        // path written bare, and is reported as that path.
        let tmp = adopter();
        write(
            &tmp.path().join("README.md"),
            "See [a](./specs/gone-a/spec.md) and [b](/specs/gone-b/spec.md).\n",
        );
        write(
            &tmp.path().join(".githooks/pre-commit"),
            "#!/usr/bin/env bash\ngit add ./specs/gone-c/spec.md\n",
        );
        write(
            &tmp.path().join("specs/system.md"),
            "# System\n\nSee `../specs/gone-d/spec.md` and `/specs/gone-e/spec.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found: Vec<(&str, &str)> = result
            .findings
            .iter()
            .map(|f| (f.referrer.as_str(), f.target.as_str()))
            .collect();
        assert_eq!(
            found,
            vec![
                ("README.md", "specs/gone-a/spec.md"),
                ("README.md", "specs/gone-b/spec.md"),
                (".githooks/pre-commit", "specs/gone-c/spec.md"),
                ("specs/system.md", "specs/gone-d/spec.md"),
                ("specs/system.md", "specs/gone-e/spec.md"),
            ]
        );
    }

    #[test]
    fn a_path_behind_any_other_prefix_is_not_a_reference() {
        // `../` from a root file leaves the project, a leading `/` in the hook
        // is an absolute file-system path, `./` in `system.md` names the spec
        // root itself, and a word glued before a prefix — a directory, a
        // URL's host — makes the path part of another token, as the end of a
        // home or variable path does (`~/`, `${HOME}/`, `%USERPROFILE%/`,
        // `$(pwd)/`), bare or closed by a quote or backtick.
        let tmp = adopter();
        write(
            &tmp.path().join("README.md"),
            "See ../specs/gone-a/spec.md, docs/specs/gone-b/spec.md \
             and https://example.com/specs/gone-c/spec.md.\n\
             The pointer is ~/.ductus/bin/ductus, ${HOME}/.ductus/gone-g \
             or %USERPROFILE%/.ductus/gone-h, or $(pwd)/.ductus/gone-i.\n\
             Quoted: \"$HOME\"/.ductus/gone-j, '${HOME}'/.ductus/gone-k, \
             `pwd`/.ductus/gone-l and \"$(pwd)\"/.ductus/gone-m.\n",
        );
        write(
            &tmp.path().join(".githooks/pre-commit"),
            "#!/usr/bin/env bash\ncat /specs/gone-d/spec.md ../specs/gone-e/spec.md\n",
        );
        write(
            &tmp.path().join("specs/system.md"),
            "# System\n\nSee `./specs/gone-f/spec.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
        assert_eq!(result.examined.len(), 3, "{:?}", result.examined);
    }

    #[test]
    fn a_prefix_after_markup_or_a_shell_operator_is_a_reference() {
        // A prefix counts wherever it starts a token, so a path behind one is
        // found wherever its bare spelling is: each line carries both, after
        // a table bar, an HTML tag, a blockquote marker, a typographic quote,
        // a redirection, a pipe and `&&`.
        let tmp = adopter();
        write(
            &tmp.path().join("README.md"),
            "|a|specs/gone-a/x.md|./specs/gone-b/x.md|\n\
             <td>specs/gone-c/x.md</td><td>/specs/gone-d/x.md</td>\n\
             >specs/gone-e/x.md\n\
             >./specs/gone-f/x.md\n\
             \u{201c}specs/gone-g/x.md\u{201d} and \u{201c}./specs/gone-h/x.md\u{201d}\n",
        );
        write(
            &tmp.path().join(".githooks/pre-commit"),
            "#!/usr/bin/env bash\n\
             echo x >.ductus/gone-i 2>./.ductus/gone-j\n\
             cat a|.ductus/gone-k|./.ductus/gone-l\n\
             true&&.ductus/gone-m&&./.ductus/gone-n\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let mut targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        targets.sort_unstable();
        let expected: Vec<String> = ('a'..='h')
            .map(|c| format!("specs/gone-{c}/x.md"))
            .chain(('i'..='n').map(|c| format!(".ductus/gone-{c}")))
            .collect();
        let mut expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        expected.sort_unstable();
        assert_eq!(targets, expected);
    }

    #[test]
    fn a_leading_slash_is_no_prefix_below_the_repository_root() {
        // Spec 059: a host resolves a leading `/` from the repository's root,
        // so in a project one directory down it names a `specs/` beside the
        // project, not the project's own. `./` still names the project root.
        let tmp = tempdir().unwrap();
        let (_repository, project) =
            crate::primitives::git_fixture::subdirectory_project(tmp.path());
        write(
            &project.join("README.md"),
            "See [a](/specs/gone-a/spec.md) and [b](./specs/gone-b/spec.md).\n",
        );
        let result = run(&args(), &project).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(targets, vec!["specs/gone-b/spec.md"]);
    }

    #[test]
    fn the_import_and_revision_rules_apply_where_the_prefix_begins() {
        // The character before the root is the prefix's own `/`, so the
        // token-start rules read the one before the prefix: an `@` starting
        // a token is an import and one glued to a word is not, and a
        // revision before a colon names a git object while a link label
        // before one names a working-tree path.
        let tmp = adopter();
        write(
            &tmp.path().join("CLAUDE.md"),
            "# CLAUDE.md\n\n@./specs/gone-a/spec.md\nMail x@./specs/gone-b/spec.md\n",
        );
        write(
            &tmp.path().join("README.md"),
            "Recover it with `git show HEAD:./specs/gone-c/spec.md`.\n\
             [old]:./specs/gone-d/spec.md\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found: Vec<(&str, u32)> = result
            .findings
            .iter()
            .map(|f| (f.target.as_str(), f.line))
            .collect();
        assert_eq!(
            found,
            vec![("specs/gone-a/spec.md", 3), ("specs/gone-d/spec.md", 2)]
        );
    }

    #[test]
    fn an_adopter_authored_reference_in_system_md_is_reported() {
        // The defect a live adopter bootstrap surfaced. `specs/system.md` is
        // `create`-strategy and adopter-owned: nothing rescaffolds it, no
        // migration step names it, and it was absent from the referrer list —
        // so the run completed clean while the file pointed at a directory
        // that no longer existed.
        let tmp = adopter();
        write(
            &tmp.path().join("specs/system.md"),
            "# System\n\nSee [the constitution](.govern/constitution.md).\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(
            targets,
            vec![".govern/constitution.md"],
            "{:?}",
            result.findings
        );
        assert_eq!(result.findings[0].referrer, "specs/system.md");
        assert!(result.examined.contains(&"specs/system.md".to_string()));
    }

    #[test]
    fn the_system_md_referrer_follows_the_configured_spec_root() {
        // A literal `specs/system.md` would examine nothing on a project that
        // renamed its root, and report clean for the wrong reason — the
        // exit-0-over-a-wrong-path shape `config_path_of` produced in the
        // shipped specs-root.sh.
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[paths]\nspecs-root = \"governance\"\n",
        );
        write(
            &tmp.path().join("governance/system.md"),
            "# System\n\nSee `.ductus/gone.md`.\n",
        );
        // Same reference under the default root, which this project does not
        // use: it must not be examined at all.
        write(
            &tmp.path().join("specs/system.md"),
            "# System\n\nSee `.ductus/also-gone.md`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(targets, vec![".ductus/gone.md"], "{:?}", result.findings);
        assert!(
            result
                .examined
                .contains(&"governance/system.md".to_string())
        );
        assert!(!result.examined.contains(&"specs/system.md".to_string()));
    }

    #[test]
    fn an_absent_system_md_is_neither_a_finding_nor_a_skip() {
        // The adopter who deleted or never received it. Nothing there is not
        // the same as something examined and found clean — `examined` is what
        // carries that distinction.
        let tmp = adopter();
        write(&tmp.path().join("AGENTS.md"), "Nothing managed here.\n");
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
        assert_eq!(result.skipped, [] as [OrphanedReferenceSkip; 0]);
        assert!(!result.examined.contains(&"specs/system.md".to_string()));
    }

    #[test]
    fn markdown_and_shell_forms_are_both_found() {
        // One scan, three host formats — link target, code span, shell arg.
        let tmp = adopter();
        write(
            &tmp.path().join("README.md"),
            "[c](.ductus/gone-a.md) and `.ductus/gone-b.md`.\n",
        );
        write(
            &tmp.path().join(".githooks/pre-commit"),
            "#!/usr/bin/env bash\nbash .ductus/scripts/gone-c.sh --staged\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let mut targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        targets.sort_unstable();
        assert_eq!(
            targets,
            vec![
                ".ductus/gone-a.md",
                ".ductus/gone-b.md",
                ".ductus/scripts/gone-c.sh"
            ],
            "trailing `)` and backticks must not survive into the path"
        );
    }
}
