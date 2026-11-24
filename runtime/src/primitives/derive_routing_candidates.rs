//! `derive-routing-candidates` — the homes proposed work could belong to.
//!
//! `/ductus:groom` walks a five-route decision tree before anything is
//! written; `/ductus:specify` had no equivalent, so the routing that exists to
//! *prevent* a new spec — a rule added to its surface's home spec, and runtime
//! work offered the spec that owns the runtime as one candidate — bound only for
//! work arriving through the inbox. Work arriving through conversation bypassed the
//! tree entirely, at the one moment the rules matter.
//!
//! This primitive supplies the deterministic half: given the proposed work in
//! the requester's words, it derives candidate homes from four sources —
//!
//! - **path-claim** — each path the description names, read from its inline
//!   code spans by `check-artifacts`' path grammar, offered to every spec whose
//!   plan claims it under Affected Files, read as `compute-review-scope` reads
//!   one: an exact path first, then a glob, then a directory, then by spec
//!   number. A claim records that a plan *changed* the path, not that its spec
//!   owns it, so the reason says so. A named path no plan claims is reported
//!   in `unclaimed-paths`, not left to read as an empty list.
//! - **runtime-work** — the description names a runtime artifact (a primitive,
//!   a path under `runtime/`), and some spec's plan claims `runtime/` under its
//!   Affected Files. That spec is the home, *derived from the corpus* rather
//!   than named here: an adopter's runtime-owning spec is not this repo's, and
//!   a project with none produces no candidate rather than a wrong one.
//! - **rule-surface** — a rule file whose category stem the description shares.
//! - **spec-corpus** — a spec whose slug the description shares vocabulary with.
//!
//! A spec an earlier source offered is not offered again by a later one.
//! Matching is lexical and the result is advisory: the semantic judgment stays
//! at the `routeInboxItem` extension point (groom's tree, reused rather than
//! copied) and the decision stays with the operator. It reports, it does not
//! veto — a new spec remains creatable over any candidate.
//!
//! **No candidates and no candidates *derivable* are different answers.** Each
//! source lands in `sources-examined` or in `skipped`, never both and never
//! neither, so a zero `candidates-count` means *examined and matched nothing*
//! only when `skipped` is empty (`QUAL-CLAIM-001`).
//!
//! Read-only but for the list file past the read size. Defined by
//! `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
//! and, for `path-claim`, `routing-ranks-by-claimed-path.md` beside it.

use std::collections::BTreeSet;
use std::path::Path;

use crate::primitives::{Result, list_feature_dirs, rel_path};
use crate::schema::paths;
use crate::schema::primitives::{
    DeriveRoutingCandidatesArgs, DeriveRoutingCandidatesResult, ListsForm, RoutingCandidate,
    RoutingSkip,
};

/// Tokens too common to carry routing signal. Kept deliberately short: a
/// stopword list that grows into a tuning surface is a second routing rule by
/// another name, and the operator's judgment is the backstop for a weak match.
const STOPWORDS: &[&str] = &[
    "spec", "specs", "feature", "features", "ductus", "with", "that", "this", "from", "into",
    "when", "then", "than", "have", "make", "must", "should", "would", "could", "does", "each",
    "also", "only", "some", "such", "they", "them", "will", "your", "about", "which", "there",
    "their", "where", "while", "these", "those", "other", "after", "before", "under", "over",
    "runtime",
];

/// Minimum token length that can contribute a match. Three-letter tokens carry
/// too little signal to distinguish one spec slug from another.
const MIN_TOKEN_LEN: usize = 4;

/// Path prefix that marks a description as runtime work. The *home* for that
/// work is derived from the corpus (see [`runtime_home`]), never named here.
const RUNTIME_PATH_PREFIX: &str = "runtime/";

/// Execute the `derive-routing-candidates` primitive: [`compute`], returned
/// inline while it fits `[artifacts] read-size-bytes`, and past it with
/// `candidates`, which grows with the specs a description shares vocabulary
/// with, in the file it names and only its count inline
/// (`bounded_lists::place`; spec 080, AC7).
///
/// # Errors
///
/// Every error [`compute`] returns, and those `bounded_lists::place` returns
/// for a malformed read size or a list file that cannot be written.
pub fn run(
    args: &DeriveRoutingCandidatesArgs,
    repo: &Path,
) -> Result<DeriveRoutingCandidatesResult> {
    let derived = compute(args, repo)?;
    let lists = serde_json::json!({ "candidates": derived.candidates });
    let mut result = DeriveRoutingCandidatesResult {
        description: derived.description,
        candidates_count: derived.candidates.len(),
        candidates: Some(derived.candidates),
        named_paths: derived.named_paths,
        unclaimed_paths: derived.unclaimed_paths,
        sources_examined: derived.sources_examined,
        skipped: derived.skipped,
        gate_required: derived.gate_required,
        derivation_incomplete: derived.derivation_incomplete,
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) = super::bounded_lists::place(
        repo,
        "derive-routing-candidates",
        super::bounded_lists::ALL_SPECS,
        &result,
        &lists,
    )? {
        result.form = ListsForm::File;
        result.candidates = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// One derivation, whole: what `derive-routing-candidates` returns before its
/// result is bounded. Each field is [`DeriveRoutingCandidatesResult`]'s of
/// the same name.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Derived {
    pub(crate) description: String,
    pub(crate) candidates: Vec<RoutingCandidate>,
    pub(crate) named_paths: Vec<String>,
    pub(crate) unclaimed_paths: Vec<String>,
    pub(crate) sources_examined: Vec<String>,
    pub(crate) skipped: Vec<RoutingSkip>,
    pub(crate) gate_required: bool,
    pub(crate) derivation_incomplete: bool,
}

/// Derive the routing candidates for `args`' description.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::InvalidArgument`] when
/// `description` is empty or whitespace-only — there is nothing to route,
/// and the error `Paths::load` returns for a project config that does not
/// parse or names an invalid spec root. Every other failure is reported as a
/// skipped source rather than raised: a
/// derivation that cannot run must be distinguishable from one that found
/// nothing, and an error would collapse the two into "no gate".
pub(crate) fn compute(args: &DeriveRoutingCandidatesArgs, repo: &Path) -> Result<Derived> {
    if args.description.trim().is_empty() {
        return Err(crate::primitives::PrimitiveError::InvalidArgument {
            primitive: "derive-routing-candidates".into(),
            argument: "description".into(),
            reason: "description is empty; there is no work to route".into(),
        });
    }

    // A tree that already ran is not re-run. The routing decision is the
    // operator's and it has been made — asking again is friction, not rigor.
    if let Some(routed_by) = args
        .routed_by
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Ok(Derived {
            description: args.description.clone(),
            candidates: Vec::new(),
            named_paths: Vec::new(),
            unclaimed_paths: Vec::new(),
            sources_examined: vec![format!("routed-by:{routed_by}")],
            skipped: Vec::new(),
            gate_required: false,
            derivation_incomplete: false,
        });
    }

    let tokens = tokenize(&args.description);
    let mut candidates: Vec<RoutingCandidate> = Vec::new();
    let mut examined: Vec<String> = Vec::new();
    let mut skipped: Vec<RoutingSkip> = Vec::new();

    let claims = derive_path_claims(
        &args.description,
        repo,
        &mut candidates,
        &mut examined,
        &mut skipped,
    )?;
    derive_runtime_work(
        &args.description,
        repo,
        &mut candidates,
        &mut examined,
        &mut skipped,
    )?;
    derive_rule_surfaces(&tokens, repo, &mut candidates, &mut examined, &mut skipped)?;
    derive_spec_corpus(&tokens, repo, &mut candidates, &mut examined, &mut skipped)?;

    let derivation_incomplete = !skipped.is_empty();
    Ok(Derived {
        description: args.description.clone(),
        candidates,
        named_paths: claims.named,
        unclaimed_paths: claims.unclaimed,
        sources_examined: examined,
        skipped,
        gate_required: true,
        derivation_incomplete,
    })
}

// -- source: path claims -----------------------------------------------------

/// How a plan's Affected Files entry claims a named path, most specific first:
/// the order `path-claim` candidates are offered in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ClaimRank {
    /// The entry is the path.
    Exact,
    /// A `*`, `?` or `{a, b}` pattern matching it.
    Glob,
    /// A directory, ending in `/`, holding it.
    Directory,
}

/// The paths a description names, and those of them no plan claims.
#[derive(Debug, Default)]
struct PathClaims {
    named: Vec<String>,
    unclaimed: Vec<String>,
}

/// The path-claim source: each path the description names, offered to every
/// spec whose plan's Affected Files claims it.
///
/// A plan that cannot be read is a skip naming it, and while one is, no path
/// is reported unclaimed: that plan may claim it (`QUAL-CLAIM-001`). A
/// description naming no path reads no plan and is examined.
fn derive_path_claims(
    description: &str,
    repo: &Path,
    candidates: &mut Vec<RoutingCandidate>,
    examined: &mut Vec<String>,
    skipped: &mut Vec<RoutingSkip>,
) -> Result<PathClaims> {
    let named = crate::primitives::check_artifacts::candidate_paths(
        &crate::primitives::line_citations::strip(description),
    );
    if named.is_empty() {
        examined.push("path-claim".into());
        return Ok(PathClaims::default());
    }
    let layout = paths::Paths::load(repo)?;
    let specs_dir = repo.join(&layout.specs_root);
    // A spec root that leaves the project is not listed (`BE-INPUT-004`).
    let listed = list_feature_dirs(repo, &layout.specs_root)
        .ok()
        .filter(|_| specs_dir.is_dir());
    let Some(listed) = listed else {
        skipped.push(RoutingSkip {
            source: "path-claim".into(),
            reason: "spec root is missing or unreadable, so no plan's claims can be read".into(),
            path: layout.specs_root.clone(),
        });
        return Ok(PathClaims {
            named,
            unclaimed: Vec::new(),
        });
    };

    // Each spec's most specific claim on any named path, with the path it
    // claims; then how many specs claim each named path at all.
    let mut claims: Vec<(ClaimRank, String, String, String)> = Vec::new();
    let mut claimants = vec![0_usize; named.len()];
    let mut unread = false;
    for slug in listed {
        let plan_rel = format!("{}/{slug}/plan.md", layout.specs_root);
        // A plan that is, or lies under, a link leaving the project is not
        // read (`BE-INPUT-004`); like an unreadable one, it is named.
        let entries = if crate::primitives::leaves_project(repo, &plan_rel) {
            None
        } else {
            crate::primitives::compute_review_scope::read_plan_affected(&specs_dir.join(&slug)).ok()
        };
        let Some(entries) = entries else {
            unread = true;
            skipped.push(RoutingSkip {
                source: "path-claim".into(),
                reason: "plan could not be read, so a path it may claim is not reported unclaimed"
                    .into(),
                path: plan_rel,
            });
            continue;
        };
        let mut best: Option<(ClaimRank, usize, String)> = None;
        for (index, path) in named.iter().enumerate() {
            let Some((rank, entry)) = entries
                .iter()
                .filter_map(|entry| claim_rank(entry, path).map(|rank| (rank, entry)))
                .min_by_key(|(rank, _)| *rank)
            else {
                continue;
            };
            claimants[index] += 1;
            if best.as_ref().is_none_or(|(held, _, _)| rank < *held) {
                best = Some((rank, index, entry.clone()));
            }
        }
        if let Some((rank, index, entry)) = best {
            claims.push((rank, slug, named[index].clone(), entry));
        }
    }
    // Stable: specs keep their listing's number order within a rank.
    claims.sort_by_key(|(rank, ..)| *rank);
    for (rank, slug, path, entry) in claims {
        let index = named.iter().position(|named| *named == path).unwrap_or(0);
        let feature_dir = specs_dir.join(&slug);
        let status = read_status(repo, &layout.specs_root, &slug);
        candidates.push(RoutingCandidate {
            route: "scenario".into(),
            source: "path-claim".into(),
            target: slug,
            path: rel_path(&feature_dir, repo),
            reopens: status == "done",
            status,
            reason: claim_reason(rank, &path, &entry, claimants[index]),
        });
    }
    let unclaimed = if unread {
        Vec::new()
    } else {
        examined.push("path-claim".into());
        named
            .iter()
            .zip(&claimants)
            .filter(|(_, count)| **count == 0)
            .map(|(path, _)| path.clone())
            .collect()
    };
    Ok(PathClaims { named, unclaimed })
}

/// How `entry`, one of a plan's Affected Files entries, claims `path`, or
/// `None` when it does not. An entry is matched as `compute-review-scope`
/// matches one ([`entry_pattern`](crate::primitives::compute_review_scope::entry_pattern)),
/// and an entry that is the path, a trailing `/` aside, is exact.
fn claim_rank(entry: &str, path: &str) -> Option<ClaimRank> {
    if entry.trim_end_matches('/') == path.trim_end_matches('/') {
        return Some(ClaimRank::Exact);
    }
    let pattern = crate::primitives::compute_review_scope::entry_pattern(entry)?;
    if !pattern.is_match(path) {
        return None;
    }
    Some(if entry.contains(['*', '?', '{']) {
        ClaimRank::Glob
    } else {
        ClaimRank::Directory
    })
}

/// A path-claim candidate's reason: what its plan changed, never that its spec
/// owns the path, with how many plans claim it, since that count is the
/// evidence a claim is history.
fn claim_reason(rank: ClaimRank, path: &str, entry: &str, claimants: usize) -> String {
    let of = if claimants == 1 {
        "the only plan that did".to_string()
    } else {
        format!("1 of {claimants} plans that did")
    };
    match rank {
        ClaimRank::Exact => format!("its plan changed `{path}` ({of})"),
        ClaimRank::Glob => {
            format!("its plan changed paths matching `{entry}`, which holds `{path}` ({of})")
        }
        ClaimRank::Directory => {
            format!("its plan changed files under `{entry}`, which holds `{path}` ({of})")
        }
    }
}

// -- source: runtime work ----------------------------------------------------

/// The runtime-work signal: the description names a runtime artifact, and some
/// spec's plan claims `runtime/` under its Affected Files.
///
/// The signal firing with no derivable home is a **skip**, not silence: it is
/// precisely the case where the operator most needs to be told the check could
/// not answer, since the work is runtime work either way.
fn derive_runtime_work(
    description: &str,
    repo: &Path,
    candidates: &mut Vec<RoutingCandidate>,
    examined: &mut Vec<String>,
    skipped: &mut Vec<RoutingSkip>,
) -> Result<()> {
    let Some(signal) = runtime_signal(description) else {
        examined.push("runtime-work".into());
        return Ok(());
    };
    let layout = paths::Paths::load(repo)?;
    let specs_dir = repo.join(&layout.specs_root);
    if !specs_dir.is_dir() {
        skipped.push(RoutingSkip {
            source: "runtime-work".into(),
            reason: format!(
                "description names {signal}, but the spec root is missing or unreadable so the \
                 owning spec cannot be derived"
            ),
            path: layout.specs_root.clone(),
        });
        return Ok(());
    }
    match runtime_home(repo, &layout.specs_root) {
        // Already offered by `path-claim`, the more specific claim: a second
        // entry for the same spec would read as two independent matches.
        Some(slug) if candidates.iter().any(|c| c.target == slug) => {
            examined.push("runtime-work".into());
        }
        Some(slug) => {
            let feature_dir = specs_dir.join(&slug);
            let status = read_status(repo, &layout.specs_root, &slug);
            candidates.push(RoutingCandidate {
                route: "scenario".into(),
                source: "runtime-work".into(),
                target: slug.clone(),
                path: rel_path(&feature_dir, repo),
                reopens: status == "done",
                status,
                reason: format!("names {signal}; this spec's plan claims `runtime/`"),
            });
            examined.push("runtime-work".into());
        }
        None => skipped.push(RoutingSkip {
            source: "runtime-work".into(),
            reason: format!(
                "description names {signal}, but no spec's plan lists a `runtime/` path under \
                 Affected Files, so the owning spec cannot be derived"
            ),
            path: layout.specs_root.clone(),
        }),
    }
    Ok(())
}

/// The runtime artifact the description names, phrased for the gate, or `None`
/// when it names none. Checked against the shipped primitive vocabulary rather
/// than a keyword list, so the signal tracks the runtime instead of drifting
/// from it.
fn runtime_signal(description: &str) -> Option<String> {
    let lower = description.to_ascii_lowercase();
    if lower.contains(RUNTIME_PATH_PREFIX) {
        return Some(format!("a path under `{RUNTIME_PATH_PREFIX}`"));
    }
    crate::parser::PRIMITIVE_NAMES
        .iter()
        .find(|name| lower.contains(&name.to_ascii_lowercase()))
        .map(|name| format!("the primitive `{name}`"))
}

/// The feature whose `plan.md` claims a `runtime/` path under Affected Files —
/// the spec that owns the runtime in *this* corpus. The lowest-numbered match
/// wins so the answer is stable; `None` when no spec claims it.
fn runtime_home(repo: &Path, specs_root: &str) -> Option<String> {
    let specs_dir = repo.join(specs_root);
    // A spec root that leaves the project is not listed: it claims nothing
    // here, and `derive_spec_corpus` reports it unread (`BE-INPUT-004`).
    let Ok(slugs) = list_feature_dirs(repo, specs_root) else {
        return None;
    };
    slugs.into_iter().find(|slug| {
        // A plan that is, or lies under, a link leaving the project is not
        // read (`BE-INPUT-004`). An unreadable plan claims nothing here, as it
        // always has: routing candidates are advice, and this caller's
        // contract predates the error `compute-review-scope` now raises for
        // it.
        !crate::primitives::leaves_project(repo, &format!("{specs_root}/{slug}/plan.md"))
            && crate::primitives::compute_review_scope::read_plan_affected(&specs_dir.join(slug))
                .unwrap_or_default()
                .iter()
                .any(|path| path.starts_with(RUNTIME_PATH_PREFIX))
    })
}

// -- source: rule surfaces ---------------------------------------------------

/// Rule files whose category stem the description shares. A rule belonging to
/// an existing surface is amended on that surface, not spawned as a new spec.
fn derive_rule_surfaces(
    tokens: &BTreeSet<String>,
    repo: &Path,
    candidates: &mut Vec<RoutingCandidate>,
    examined: &mut Vec<String>,
    skipped: &mut Vec<RoutingSkip>,
) -> Result<()> {
    // A rule directory that leaves the project is refused by the shared
    // resolution; routing is advice, so it is a skip that names why, never an
    // empty surface list (`BE-INPUT-004`).
    let (dir, dir_rel) = match crate::primitives::discover_rule_files::resolve_rules_dir(repo) {
        Ok(resolved) => resolved,
        Err(crate::primitives::PrimitiveError::InvalidPath { reason, .. }) => {
            skipped.push(RoutingSkip {
                source: "rule-surface".into(),
                reason: format!("rule-file directory leaves the project: {reason}"),
                path: "rules".into(),
            });
            return Ok(());
        }
        Err(other) => return Err(other),
    };
    let Some(dir) = dir else {
        // No rule directory at all is a real, examined answer: this project
        // has no rule surfaces, so none can be a home. A fresh adopter is
        // exactly this case and must see no new friction.
        examined.push("rule-surface".into());
        return Ok(());
    };
    let files = match crate::primitives::discover_rule_files::list_rule_files(&dir) {
        Ok(files) => files,
        Err(err) => {
            skipped.push(RoutingSkip {
                source: "rule-surface".into(),
                reason: format!("rule-file directory could not be listed: {err}"),
                path: dir_rel,
            });
            return Ok(());
        }
    };
    for name in files {
        let shared = shared_tokens(tokens, &tokenize(&rule_stem(&name)));
        if shared.is_empty() {
            continue;
        }
        candidates.push(RoutingCandidate {
            route: "rule".into(),
            source: "rule-surface".into(),
            target: name.clone(),
            path: format!("{dir_rel}/{name}"),
            status: String::new(),
            reopens: false,
            reason: format!("shares {} with this rule surface", quoted_list(&shared)),
        });
    }
    examined.push("rule-surface".into());
    Ok(())
}

/// A rule file's category stem: the basename with its `.md` extension and its
/// `-backend` / `-frontend` / `-cross` surface suffix removed, so
/// `security-backend.md` matches on `security` rather than on `backend`.
fn rule_stem(name: &str) -> String {
    let stem = name.strip_suffix(".md").unwrap_or(name);
    for suffix in ["-backend", "-frontend", "-cross"] {
        if let Some(base) = stem.strip_suffix(suffix) {
            return base.to_string();
        }
    }
    stem.to_string()
}

// -- source: spec corpus -----------------------------------------------------

/// Specs whose slug the description shares vocabulary with. A match means the
/// work may be a scenario on that spec rather than a spec of its own.
fn derive_spec_corpus(
    tokens: &BTreeSet<String>,
    repo: &Path,
    candidates: &mut Vec<RoutingCandidate>,
    examined: &mut Vec<String>,
    skipped: &mut Vec<RoutingSkip>,
) -> Result<()> {
    let layout = paths::Paths::load(repo)?;
    let specs_dir = repo.join(&layout.specs_root);
    // A spec root that leaves the project is not listed, and reads as an
    // unreadable one (`BE-INPUT-004`).
    let listed = list_feature_dirs(repo, &layout.specs_root)
        .ok()
        .filter(|_| specs_dir.is_dir());
    let Some(listed) = listed else {
        skipped.push(RoutingSkip {
            source: "spec-corpus".into(),
            reason: "spec root is missing or unreadable".into(),
            path: layout.specs_root.clone(),
        });
        return Ok(());
    };
    for slug in listed {
        // Already offered by `path-claim` or `runtime-work`, each a more
        // specific claim; a second entry for the same spec would read as two
        // independent matches.
        if candidates.iter().any(|c| c.target == slug) {
            continue;
        }
        let shared = shared_tokens(tokens, &tokenize(slug_words(&slug)));
        if shared.is_empty() {
            continue;
        }
        let feature_dir = specs_dir.join(&slug);
        let status = read_status(repo, &layout.specs_root, &slug);
        candidates.push(RoutingCandidate {
            route: "scenario".into(),
            source: "spec-corpus".into(),
            target: slug.clone(),
            path: rel_path(&feature_dir, repo),
            reopens: status == "done",
            status,
            reason: format!("shares {} with this spec's subject", quoted_list(&shared)),
        });
    }
    examined.push("spec-corpus".into());
    Ok(())
}

/// A feature slug's words: everything after the `NNN-` prefix.
fn slug_words(slug: &str) -> &str {
    slug.get(4..).unwrap_or(slug)
}

/// A spec's frontmatter `status`, or empty when the file is absent or its
/// frontmatter unreadable. Empty is a truthful "not known" — it only softens
/// the confirmation prompt (no reopen is claimed), never fabricates one.
fn read_status(repo: &Path, specs_root: &str, slug: &str) -> String {
    let spec_rel = format!("{specs_root}/{slug}/spec.md");
    // A spec that is, or lies under, a link leaving the project is not read:
    // its status is not known here (`BE-INPUT-004`).
    if crate::primitives::leaves_project(repo, &spec_rel) {
        return String::new();
    }
    let spec = repo.join(&spec_rel);
    let Ok(content) = std::fs::read_to_string(&spec) else {
        return String::new();
    };
    let Ok((frontmatter, _)) = crate::primitives::split_frontmatter(&content, &spec) else {
        return String::new();
    };
    frontmatter
        .lines()
        .find_map(|line| line.strip_prefix("status:"))
        .map(|value| value.trim().to_string())
        .unwrap_or_default()
}

// -- lexical matching --------------------------------------------------------

/// Lowercase alphanumeric tokens of at least [`MIN_TOKEN_LEN`] characters,
/// minus [`STOPWORDS`]. A `BTreeSet` keeps the shared-token report ordered.
fn tokenize(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .filter(|token| token.len() >= MIN_TOKEN_LEN)
        .filter(|token| !STOPWORDS.contains(&token.as_str()))
        .collect()
}

/// Tokens present in both sets, in sorted order.
fn shared_tokens(a: &BTreeSet<String>, b: &BTreeSet<String>) -> Vec<String> {
    a.intersection(b).cloned().collect()
}

/// Render shared tokens for the `reason` prose: ``"`a`, `b`"``.
fn quoted_list(tokens: &[String]) -> String {
    tokens
        .iter()
        .map(|token| format!("`{token}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    /// The tests read the whole derivation: [`super::compute`], which
    /// [`super::run`] bounds.
    fn run(args: &DeriveRoutingCandidatesArgs, repo: &Path) -> Result<Derived> {
        compute(args, repo)
    }

    fn args(description: &str) -> DeriveRoutingCandidatesArgs {
        DeriveRoutingCandidatesArgs {
            description: description.into(),
            routed_by: None,
        }
    }

    fn write(path: &Path, body: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }

    /// A corpus with one spec whose plan claims `runtime/` (the runtime home),
    /// one ordinary spec, and two rule surfaces.
    fn corpus() -> TempDir {
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join("specs/022-deterministic-runtime/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# 022\n",
        );
        write(
            &tmp.path().join("specs/022-deterministic-runtime/plan.md"),
            "# Plan\n\n## Affected Files\n\n| Path | Change |\n| --- | --- |\n| `runtime/src/lib.rs` | edit |\n",
        );
        write(
            &tmp.path().join("specs/036-quality-cross-rules/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# 036\n",
        );
        write(
            &tmp.path().join("framework/rules/security-backend.md"),
            "# Security\n",
        );
        write(
            &tmp.path().join("framework/rules/quality-cross.md"),
            "# Quality\n",
        );
        tmp
    }

    #[test]
    fn runtime_work_routes_to_the_spec_whose_plan_claims_runtime() {
        let tmp = corpus();
        let result = run(
            &args("report a stale review from the check-artifacts family"),
            tmp.path(),
        )
        .unwrap();
        let runtime: Vec<_> = result
            .candidates
            .iter()
            .filter(|c| c.source == "runtime-work")
            .collect();
        assert_eq!(runtime.len(), 1, "{:?}", result.candidates);
        assert_eq!(runtime[0].target, "022-deterministic-runtime");
        assert_eq!(runtime[0].route, "scenario");
        assert!(runtime[0].reason.contains("check-artifacts"));
        assert!(!runtime[0].reopens, "022 is in-progress, no back-edge");
        assert_eq!(result.skipped, [] as [RoutingSkip; 0]);
        assert!(result.gate_required);
    }

    #[test]
    fn runtime_home_is_derived_from_the_corpus_not_named_in_code() {
        // The same description in a corpus whose runtime-owning spec has a
        // different slug must route to *that* spec — an adopter's runtime home
        // is not this repo's, and a hardcoded slug would silently miss it.
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join("specs/004-our-own-engine/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# 004\n",
        );
        write(
            &tmp.path().join("specs/004-our-own-engine/plan.md"),
            "# Plan\n\n## Affected Files\n\n| Path | Change |\n| --- | --- |\n| `runtime/src/a.rs` | edit |\n",
        );
        let result = run(&args("a new field on the read-spec result"), tmp.path()).unwrap();
        let runtime: Vec<_> = result
            .candidates
            .iter()
            .filter(|c| c.source == "runtime-work")
            .collect();
        assert_eq!(runtime.len(), 1);
        assert_eq!(runtime[0].target, "004-our-own-engine");
        assert!(
            runtime[0].reopens,
            "a done candidate must name the back-edge it implies"
        );
        assert_eq!(runtime[0].status, "done");
    }

    #[test]
    fn runtime_signal_without_a_derivable_home_is_skipped_not_silent() {
        // The description is runtime work, but no spec claims `runtime/`. That
        // is "could not derive", not "no candidate" — reporting it as the
        // latter would tell the operator a new spec is correct when the check
        // never actually ran.
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# 001\n",
        );
        let result = run(&args("change a shared parser in runtime/src/"), tmp.path()).unwrap();
        assert!(result.candidates.iter().all(|c| c.source != "runtime-work"));
        assert_eq!(result.skipped.len(), 1, "{:?}", result.skipped);
        assert_eq!(result.skipped[0].source, "runtime-work");
        assert!(result.derivation_incomplete);
        assert!(
            !result
                .sources_examined
                .contains(&"runtime-work".to_string())
        );
    }

    #[test]
    fn rule_surface_matches_on_the_category_stem_not_the_surface_suffix() {
        let tmp = corpus();
        let result = run(&args("a new security rule about token logging"), tmp.path()).unwrap();
        let rules: Vec<_> = result
            .candidates
            .iter()
            .filter(|c| c.source == "rule-surface")
            .collect();
        assert_eq!(rules.len(), 1, "{:?}", result.candidates);
        assert_eq!(rules[0].target, "security-backend.md");
        assert_eq!(rules[0].route, "rule");
        assert_eq!(rules[0].path, "framework/rules/security-backend.md");

        // `backend` is the surface suffix, not the category: a description
        // about backends must not match every backend rule file.
        let backend = run(&args("a backend deployment checklist"), tmp.path()).unwrap();
        assert!(
            backend
                .candidates
                .iter()
                .all(|c| c.source != "rule-surface"),
            "{:?}",
            backend.candidates
        );
    }

    #[test]
    fn spec_corpus_match_on_a_done_spec_names_the_back_edge() {
        let tmp = corpus();
        let result = run(
            &args("more quality rules for the review passes"),
            tmp.path(),
        )
        .unwrap();
        let spec: Vec<_> = result
            .candidates
            .iter()
            .filter(|c| c.target == "036-quality-cross-rules")
            .collect();
        assert_eq!(spec.len(), 1, "{:?}", result.candidates);
        assert_eq!(spec[0].route, "scenario");
        assert_eq!(spec[0].status, "done");
        assert!(
            spec[0].reopens,
            "naming a done candidate must name the reopen it implies"
        );
    }

    #[test]
    fn one_spec_is_offered_once_even_when_two_sources_match_it() {
        // The runtime-work claim is the more specific one; a second entry for
        // the same spec would read as two independent matches.
        let tmp = corpus();
        let result = run(
            &args("deterministic runtime work on the read-spec primitive"),
            tmp.path(),
        )
        .unwrap();
        let hits = result
            .candidates
            .iter()
            .filter(|c| c.target == "022-deterministic-runtime")
            .count();
        assert_eq!(hits, 1, "{:?}", result.candidates);
    }

    #[test]
    fn fresh_adopter_with_no_matches_reports_examined_and_empty() {
        // No rule directory, a single-spec corpus, nothing shared: every source
        // ran and matched nothing. `skipped` empty is what lets the caller say
        // "no candidate found" rather than "could not derive".
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join("specs/001-checkout-flow/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# 001\n",
        );
        let result = run(&args("webhook delivery retries"), tmp.path()).unwrap();
        assert!(result.candidates.is_empty(), "{:?}", result.candidates);
        assert_eq!(result.skipped, [] as [RoutingSkip; 0]);
        assert!(!result.derivation_incomplete);
        assert_eq!(
            result.sources_examined,
            vec!["path-claim", "runtime-work", "rule-surface", "spec-corpus"],
            "every source must account for itself"
        );
        assert!(result.named_paths.is_empty(), "{:?}", result.named_paths);
        assert!(result.gate_required);
    }

    #[test]
    fn unreadable_spec_root_is_skipped_rather_than_read_as_no_candidates() {
        let tmp = tempdir().unwrap();
        let result = run(&args("webhook delivery retries"), tmp.path()).unwrap();
        assert_eq!(result.candidates, [] as [RoutingCandidate; 0]);
        assert!(result.derivation_incomplete);
        assert!(result.skipped.iter().any(|s| s.source == "spec-corpus"));
        assert!(
            !result.sources_examined.contains(&"spec-corpus".to_string()),
            "a source is examined or skipped, never both"
        );
    }

    #[test]
    fn every_source_lands_in_exactly_one_of_examined_or_skipped() {
        // The invariant the QUAL-CLAIM-001 distinction rests on.
        for (description, repo) in [
            ("a security rule about tokens", corpus()),
            ("check-artifacts family work", corpus()),
            ("a fix in `runtime/src/lib.rs`", corpus()),
            ("webhook delivery", tempdir().unwrap()),
            (
                "webhook delivery in `src/hooks/send.rs`",
                tempdir().unwrap(),
            ),
        ] {
            let result = run(&args(description), repo.path()).unwrap();
            for source in ["path-claim", "runtime-work", "rule-surface", "spec-corpus"] {
                let examined = result.sources_examined.iter().any(|s| s == source);
                let skipped = result.skipped.iter().any(|s| s.source == source);
                assert!(
                    examined ^ skipped,
                    "source `{source}` must be examined XOR skipped for {description:?}: {result:?}"
                );
            }
        }
    }

    #[test]
    fn already_routed_skips_the_gate_without_asking_twice() {
        let tmp = corpus();
        let mut a = args("a new security rule about token logging");
        a.routed_by = Some("groom".into());
        let result = run(&a, tmp.path()).unwrap();
        assert!(!result.gate_required);
        assert!(
            result.candidates.is_empty(),
            "no second decision is offered"
        );
        assert!(result.skipped.is_empty(), "nothing failed to run");
        assert!(!result.derivation_incomplete);
        assert_eq!(result.sources_examined, vec!["routed-by:groom"]);
    }

    #[test]
    fn blank_routed_by_still_runs_the_derivation() {
        // An empty string is not a routing tree that ran; treating it as one
        // would silently suppress the gate.
        let tmp = corpus();
        let mut a = args("a new security rule about token logging");
        a.routed_by = Some("   ".into());
        let result = run(&a, tmp.path()).unwrap();
        assert!(result.gate_required);
        assert_ne!(result.candidates, [] as [RoutingCandidate; 0]);
    }

    #[test]
    fn empty_description_is_an_operational_error() {
        let tmp = corpus();
        let err = run(&args("   "), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, crate::primitives::PrimitiveError::InvalidArgument { argument, .. }
                if argument == "description"),
            "{err:?}"
        );
    }

    #[test]
    fn candidates_are_ordered_by_source_precedence_and_are_stable() {
        let tmp = corpus();
        write(
            &tmp.path().join("specs/036-quality-cross-rules/plan.md"),
            "# Plan\n\n## Affected Files\n\n| Path | Change |\n| --- | --- |\n| `framework/rules/quality-cross.md` | edit |\n",
        );
        let description = "quality rules for the check-artifacts family and security review \
                           in `framework/rules/quality-cross.md`";
        let first = run(&args(description), tmp.path()).unwrap();
        let second = run(&args(description), tmp.path()).unwrap();
        assert_eq!(first, second, "the derivation must be deterministic");
        let sources: Vec<&str> = first.candidates.iter().map(|c| c.source.as_str()).collect();
        let mut sorted = sources.clone();
        sorted.sort_by_key(|s| match *s {
            "path-claim" => 0,
            "runtime-work" => 1,
            "rule-surface" => 2,
            _ => 3,
        });
        assert_eq!(sources, sorted, "{:?}", first.candidates);
        assert_eq!(sources.first(), Some(&"path-claim"), "{sources:?}");
    }

    /// A spec directory that is a link leaving the project is not read for
    /// its status or its plan: the status is not known and the plan claims
    /// nothing (`BE-INPUT-004`; 022 task 42).
    #[cfg(unix)]
    #[test]
    fn a_spec_linked_outside_the_project_is_not_read() {
        let repo = tempdir().unwrap();
        let _outside = crate::primitives::outward::dir(repo.path(), "specs/001-a", |dir| {
            fs::write(
                dir.join("spec.md"),
                "---\nstatus: done\ndependencies: []\n---\n\n# a\n",
            )
            .unwrap();
            fs::write(
                dir.join("plan.md"),
                "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n| `runtime/src/x.rs` | Edit |\n",
            )
            .unwrap();
        });
        assert_eq!(read_status(repo.path(), "specs", "001-a"), "");
        assert_eq!(runtime_home(repo.path(), "specs"), None);

        let repo = tempdir().unwrap();
        write(
            &repo.path().join("specs/001-a/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# a\n",
        );
        write(
            &repo.path().join("specs/001-a/plan.md"),
            "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n| `runtime/src/x.rs` | Edit |\n",
        );
        assert_eq!(read_status(repo.path(), "specs", "001-a"), "done");
        assert_eq!(runtime_home(repo.path(), "specs").as_deref(), Some("001-a"));
    }

    /// A rule directory that is a link leaving the project is skipped by name,
    /// not listed and not mistaken for a project with no rule surfaces
    /// (`BE-INPUT-004`; 022 task 42).
    #[cfg(unix)]
    #[test]
    fn a_rule_directory_linked_outside_the_project_is_skipped_by_name() {
        let repo = tempdir().unwrap();
        let _outside = crate::primitives::outward::dir(repo.path(), "specs/rules", |dir| {
            fs::write(dir.join("quality-cross.md"), "# rules\n").unwrap();
        });
        let derived = run(&args("quality"), repo.path()).unwrap();
        assert!(
            derived
                .skipped
                .iter()
                .any(|skip| skip.source == "rule-surface" && skip.reason.contains("leaves")),
            "{:?}",
            derived.skipped
        );
        assert!(derived.candidates.is_empty(), "{:?}", derived.candidates);
    }

    /// A spec root that is a link out of the project is not listed: the
    /// corpus source is skipped as unreadable, so the derivation is
    /// incomplete, and no spec outside is offered (`BE-INPUT-004`; 022
    /// task 48).
    #[cfg(unix)]
    #[test]
    fn a_spec_root_linked_outside_the_project_is_skipped_and_not_listed() {
        let repo = tempdir().unwrap();
        let _outside = crate::primitives::outward::dir(repo.path(), "specs", |dir| {
            write(
                &dir.join("001-token-logging/spec.md"),
                "---\nstatus: done\ndependencies: []\n---\n\n# 001\n",
            );
        });
        let result = run(&args("rework the token logging"), repo.path()).unwrap();
        assert!(
            result
                .candidates
                .iter()
                .all(|c| c.target != "001-token-logging"),
            "{:?}",
            result.candidates
        );
        assert!(
            result
                .skipped
                .iter()
                .any(|skip| skip.source == "spec-corpus"),
            "{:?}",
            result.skipped
        );
        assert!(result.derivation_incomplete);
    }

    // -- path claims (scenario `routing-ranks-by-claimed-path`) --------------

    /// Plans claiming `src/api/send.rs` every way: two exactly, one by a glob,
    /// one by its directory, and one plan claiming something else.
    fn claims_corpus() -> TempDir {
        let tmp = tempdir().unwrap();
        for (slug, entry) in [
            ("001-sweep", "`src/**`"),
            ("002-layout", "`src/api/`"),
            ("010-retries", "`src/api/send.rs`"),
            ("020-backoff", "`src/api/send.rs`"),
            ("030-guide", "`docs/guide.md`"),
        ] {
            write(
                &tmp.path().join(format!("specs/{slug}/spec.md")),
                "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
            );
            write(
                &tmp.path().join(format!("specs/{slug}/plan.md")),
                &format!(
                    "# Plan\n\n## Affected Files\n\n| Path | Change |\n| --- | --- |\n| {entry} | edit |\n"
                ),
            );
        }
        tmp
    }

    fn path_claims(result: &Derived) -> Vec<&str> {
        result
            .candidates
            .iter()
            .filter(|c| c.source == "path-claim")
            .map(|c| c.target.as_str())
            .collect()
    }

    #[test]
    fn path_claims_rank_exact_then_glob_then_directory_then_by_number() {
        let tmp = claims_corpus();
        let result = run(&args("retries never stop in `src/api/send.rs`"), tmp.path()).unwrap();
        assert_eq!(
            path_claims(&result),
            ["010-retries", "020-backoff", "001-sweep", "002-layout"],
            "{:?}",
            result.candidates
        );
        assert_eq!(result.named_paths, ["src/api/send.rs"]);
        assert!(
            result.unclaimed_paths.is_empty(),
            "{:?}",
            result.unclaimed_paths
        );
        assert!(result.sources_examined.iter().any(|s| s == "path-claim"));
        // `retries` names 010's slug too: the claim precedes the word match,
        // and the spec is offered once.
        assert_eq!(result.candidates[0].source, "path-claim");
        assert_eq!(
            result
                .candidates
                .iter()
                .filter(|c| c.target == "010-retries")
                .count(),
            1,
            "{:?}",
            result.candidates
        );
    }

    #[test]
    fn a_path_claims_reason_says_its_plan_changed_the_path_never_that_it_owns_it() {
        let tmp = claims_corpus();
        let result = run(&args("`src/api/send.rs`"), tmp.path()).unwrap();
        let claims: Vec<_> = result
            .candidates
            .iter()
            .filter(|c| c.source == "path-claim")
            .collect();
        assert_eq!(claims.len(), 4, "{claims:?}");
        for candidate in claims {
            assert!(
                candidate.reason.starts_with("its plan changed"),
                "{}",
                candidate.reason
            );
            assert!(
                candidate.reason.contains("1 of 4 plans that did"),
                "{}",
                candidate.reason
            );
            assert!(!candidate.reason.contains("own"), "{}", candidate.reason);
        }
    }

    #[test]
    fn a_named_path_no_plan_claims_is_its_own_result_not_an_empty_list() {
        let tmp = claims_corpus();
        let result = run(&args("a new route in `lib/routes/hooks.rs`"), tmp.path()).unwrap();
        assert_eq!(result.named_paths, ["lib/routes/hooks.rs"]);
        assert_eq!(result.unclaimed_paths, ["lib/routes/hooks.rs"]);
        assert!(path_claims(&result).is_empty(), "{:?}", result.candidates);
        assert!(!result.derivation_incomplete);

        // A description naming no path is a different answer: nothing named.
        let none = run(&args("a new route for hooks"), tmp.path()).unwrap();
        assert!(none.named_paths.is_empty(), "{:?}", none.named_paths);
        assert!(
            none.unclaimed_paths.is_empty(),
            "{:?}",
            none.unclaimed_paths
        );
        assert!(none.sources_examined.iter().any(|s| s == "path-claim"));
    }

    #[test]
    fn a_path_is_read_from_a_code_span_with_its_line_citation_stripped() {
        let tmp = claims_corpus();
        let cited = run(
            &args("the loop at `src/api/send.rs:42` never stops"),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(cited.named_paths, ["src/api/send.rs"]);
        assert_eq!(cited.candidates[0].target, "010-retries");

        // Outside a code span a slash names no path: neither `and/or` nor a
        // bare path, as the acceptance-criterion path grammar reads prose.
        let bare = run(
            &args("retries and/or backoff in src/api/send.rs"),
            tmp.path(),
        )
        .unwrap();
        assert!(bare.named_paths.is_empty(), "{:?}", bare.named_paths);
        assert!(path_claims(&bare).is_empty(), "{:?}", bare.candidates);
    }

    #[test]
    fn a_path_named_only_in_a_plans_prose_claims_nothing() {
        let tmp = tempdir().unwrap();
        write(
            &tmp.path().join("specs/001-a/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# a\n",
        );
        write(
            &tmp.path().join("specs/001-a/plan.md"),
            "# Plan\n\n## Affected Files\n\nNot changed: `src/api/send.rs`.\n\n| Path | Change |\n| --- | --- |\n| `src/other.rs` | edit |\n",
        );
        let result = run(&args("`src/api/send.rs`"), tmp.path()).unwrap();
        assert!(path_claims(&result).is_empty(), "{:?}", result.candidates);
        assert_eq!(result.unclaimed_paths, ["src/api/send.rs"]);
    }

    #[test]
    fn runtime_work_does_not_offer_again_a_spec_a_path_claim_offered() {
        let tmp = corpus();
        let result = run(&args("a fix in `runtime/src/lib.rs`"), tmp.path()).unwrap();
        let hits: Vec<_> = result
            .candidates
            .iter()
            .filter(|c| c.target == "022-deterministic-runtime")
            .collect();
        assert_eq!(hits.len(), 1, "{:?}", result.candidates);
        assert_eq!(hits[0].source, "path-claim");
        assert!(result.sources_examined.iter().any(|s| s == "runtime-work"));
    }

    /// A plan that cannot be read is named in `skipped`, and while one is, no
    /// named path is reported unclaimed: that plan may claim it.
    #[test]
    fn an_unreadable_plan_is_skipped_by_name_and_holds_back_unclaimed() {
        let tmp = claims_corpus();
        // Present where the plan belongs, but not a file that reads.
        fs::create_dir_all(tmp.path().join("specs/040-broken/plan.md")).unwrap();
        write(
            &tmp.path().join("specs/040-broken/spec.md"),
            "---\nstatus: draft\ndependencies: []\n---\n\n# b\n",
        );
        let result = run(
            &args("`lib/routes/hooks.rs` and `src/api/send.rs`"),
            tmp.path(),
        )
        .unwrap();
        assert!(
            result
                .skipped
                .iter()
                .any(|s| s.source == "path-claim" && s.path == "specs/040-broken/plan.md"),
            "{:?}",
            result.skipped
        );
        assert!(
            result.unclaimed_paths.is_empty(),
            "{:?}",
            result.unclaimed_paths
        );
        assert_eq!(
            result.named_paths,
            ["lib/routes/hooks.rs", "src/api/send.rs"]
        );
        assert!(result.derivation_incomplete);
        assert!(!result.sources_examined.iter().any(|s| s == "path-claim"));
        // The plans that were read still offer what they claim.
        assert_eq!(result.candidates[0].target, "010-retries");
    }

    /// The bounded result `derive-routing-candidates` returns carries both
    /// path lists, not only the derivation the other tests read.
    #[test]
    fn the_returned_result_carries_the_named_and_the_unclaimed_paths() {
        let tmp = claims_corpus();
        let result = super::run(
            &args("`src/api/send.rs` and `lib/routes/hooks.rs`"),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            result.named_paths,
            ["src/api/send.rs", "lib/routes/hooks.rs"]
        );
        assert_eq!(result.unclaimed_paths, ["lib/routes/hooks.rs"]);
        let value = serde_json::to_value(&result).unwrap();
        assert_eq!(
            value["unclaimed-paths"],
            serde_json::json!(["lib/routes/hooks.rs"])
        );
    }

    /// A plan that is a link leaving the project is not read: it is skipped by
    /// name and claims nothing (`BE-INPUT-004`).
    #[cfg(unix)]
    #[test]
    fn a_plan_linked_outside_the_project_is_skipped_by_name_not_read() {
        let repo = tempdir().unwrap();
        write(
            &repo.path().join("specs/001-a/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# a\n",
        );
        let _outside = crate::primitives::outward::file(
            repo.path(),
            "specs/001-a/plan.md",
            "# Plan\n\n## Affected Files\n\n| Path | Change |\n| --- | --- |\n| `src/x.rs` | edit |\n",
        );
        let result = run(&args("`src/x.rs`"), repo.path()).unwrap();
        assert!(path_claims(&result).is_empty(), "{:?}", result.candidates);
        assert!(
            result
                .skipped
                .iter()
                .any(|s| s.source == "path-claim" && s.path == "specs/001-a/plan.md"),
            "{:?}",
            result.skipped
        );
        assert!(
            result.unclaimed_paths.is_empty(),
            "{:?}",
            result.unclaimed_paths
        );
    }
}
