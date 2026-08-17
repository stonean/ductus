//! `check-orphaned-references` — adopter-owned files pointing at paths that
//! no longer exist.
//!
//! A migration that moves a framework-owned path can re-point the files it
//! owns. It cannot re-point the ones it does not: an adopter-owned file is
//! `create`-strategy, so the manifest never overwrites it, and unpinned, so the
//! pinned-invoker warning never fires. A migration is the only mechanism that
//! can follow a move into one, and each migration knows only its own hop.
//!
//! Two real instances surfaced in the `papur` bootstrap (048 AC10), and both
//! were silent in the way this project pays most for — nothing errored. A
//! dangling `@import` yields a constitution that is simply not loaded, and a
//! hook calling a missing script fails at commit time, far from the run that
//! broke it.
//!
//! Read-only, and it **reports rather than repairs**: the adopter may have
//! hand-edited the reference, and a rewrite that guessed wrong is worse than a
//! report that is precise.
//!
//! Defined by `specs/022-deterministic-runtime/scenarios/orphaned-reference-check.md`,
//! required by `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md`.

use std::collections::BTreeSet;
use std::path::Path;

use crate::primitives::Result;
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
const REFERRERS: &[&str] = &[
    "CLAUDE.md",
    "AGENTS.md",
    "README.md",
    ".githooks/pre-commit",
];

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
    let roots = managed_roots(repo);
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

    for referrer in REFERRERS {
        let full = repo.join(referrer);
        if !full.exists() {
            // Not every project has every referrer. An absent file is not an
            // unexamined one — there is nothing there to examine — so it is
            // neither a finding nor a skip.
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&full) else {
            skipped.push(OrphanedReferenceSkip {
                path: (*referrer).to_string(),
                reason: "file exists but could not be read as UTF-8 text".into(),
            });
            continue;
        };
        examined.push((*referrer).to_string());
        for (idx, line) in text.lines().enumerate() {
            for target in managed_paths_in(line, &roots) {
                if repo.join(target.trim_end_matches('/')).exists() {
                    continue;
                }
                if super::check_artifacts::ships_to_adopter(&ships_elsewhere, &target) {
                    continue;
                }
                findings.push(OrphanedReference {
                    referrer: (*referrer).to_string(),
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
fn managed_roots(repo: &Path) -> Vec<String> {
    let layout = paths::Paths::load(repo);
    let mut roots = vec![".ductus/".to_string(), ".githooks/".to_string()];
    roots.push(format!("{}/", layout.specs_root.trim_end_matches('/')));
    roots.sort();
    roots.dedup();
    roots
}

/// Every managed path named on one line, deduplicated, in first-appearance
/// order of the sorted set.
///
/// Scans for a managed-root prefix and takes the path-character run that
/// follows, rather than parsing markdown link / code-span / shell grammars
/// separately: the same reference appears in all three forms across these four
/// files (`@import .ductus/constitution.md`, `[c](.ductus/constitution.md)`,
/// `bash .ductus/scripts/gen-spec-deps.sh`), and one scan handles them all
/// without a grammar per host format.
fn managed_paths_in(line: &str, roots: &[String]) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for root in roots {
        let mut from = 0usize;
        while let Some(rel) = line[from..].find(root.as_str()) {
            let start = from + rel;
            // A prefix must start a token: `x.ductus/y` is not a reference to
            // `.ductus/y`, and matching inside one would invent paths.
            let preceded_by_path_char = start > 0
                && line[..start]
                    .chars()
                    .next_back()
                    .is_some_and(|c| is_path_char(c) || c == '@');
            from = start + root.len();
            if preceded_by_path_char {
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
            if candidate.len() > root.len() && !is_pattern(candidate) {
                found.insert(candidate.to_string());
            }
        }
    }
    found
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
/// unparseable one both land here, but they are distinguished for the caller by
/// the `skipped` entry the unparseable case adds — a caller at the bootstrap
/// site expects attribution and must be able to tell "not installed" from
/// "broken".
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
    let dir = claimed.trim_end_matches('/');
    target.starts_with(&format!("{dir}/")) || claimed.ends_with('/') && target.starts_with(claimed)
}

// -- watermark -----------------------------------------------------------------

/// `[migrations].last_applied` from the active config file, or empty.
///
/// Empty means *no migration has been applied* — the absent-section case the
/// bootstrap treats as null. It is never rendered as a migration name; the
/// caller distinguishes it from an id by its emptiness.
fn read_last_applied(repo: &Path) -> String {
    let Ok(text) = std::fs::read_to_string(paths::config_path(repo)) else {
        return String::new();
    };
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
    fn the_papur_defect_is_reported() {
        // The real instance: `constitution-relocate` wrote a `.govern/`
        // reference into CLAUDE.md and `ductus-rename` moved the file, leaving
        // an @import that resolves to nothing and loads no constitution.
        let tmp = adopter();
        write(
            &tmp.path().join("CLAUDE.md"),
            "# CLAUDE.md\n\n@import .govern/constitution.md\n",
        );
        // `.govern/` is not a managed root, so the stale reference must be
        // caught by naming a path under one that does not resolve.
        write(
            &tmp.path().join("AGENTS.md"),
            "See `.ductus/constitution.md` and `.ductus/scripts/gen-spec-deps.sh`.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let targets: Vec<&str> = result.findings.iter().map(|f| f.target.as_str()).collect();
        assert_eq!(
            targets,
            vec![".ductus/scripts/gen-spec-deps.sh"],
            "the constitution resolves; the generator does not: {:?}",
            result.findings
        );
        assert_eq!(result.findings[0].referrer, "AGENTS.md");
        assert_eq!(result.findings[0].line, 1);
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
        // At the bootstrap call site the registry is in staging. Two entries
        // name the constitution; the *rename* is the one that left the
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
        assert!(result.skipped.is_empty());
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
        assert!(result.findings.is_empty());
        assert!(result.skipped.is_empty());
        assert!(result.examined.is_empty());
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
