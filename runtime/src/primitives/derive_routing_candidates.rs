//! `derive-routing-candidates` — the homes proposed work could belong to.
//!
//! `/ductus:groom` walks a five-route decision tree before anything is
//! written; `/ductus:specify` had no equivalent, so the two routing rules that
//! exist to *prevent* a new spec — add a rule to its surface's home spec, and
//! route runtime work to the spec that owns the runtime — bound only for work
//! arriving through the inbox. Work arriving through conversation bypassed the
//! tree entirely, at the one moment the rules matter.
//!
//! This primitive supplies the deterministic half: given the proposed work in
//! the requester's words, it derives candidate homes from three sources —
//!
//! - **runtime-work** — the description names a runtime artifact (a primitive,
//!   a path under `runtime/`), and some spec's plan claims `runtime/` under its
//!   Affected Files. That spec is the home, *derived from the corpus* rather
//!   than named here: an adopter's runtime-owning spec is not this repo's, and
//!   a project with none produces no candidate rather than a wrong one.
//! - **rule-surface** — a rule file whose category stem the description shares.
//! - **spec-corpus** — a spec whose slug the description shares vocabulary with.
//!
//! Matching is lexical and the result is advisory: the semantic judgment stays
//! at the `routeInboxItem` extension point (groom's tree, reused rather than
//! copied) and the decision stays with the operator. It reports, it does not
//! veto — a new spec remains creatable over any candidate.
//!
//! **No candidates and no candidates *derivable* are different answers.** Each
//! source lands in `sources-examined` or in `skipped`, never both and never
//! neither, so an empty `candidates` list means *examined and matched nothing*
//! only when `skipped` is empty (`QUAL-CLAIM-001`).
//!
//! Read-only. Defined by
//! `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`.

use std::collections::BTreeSet;
use std::path::Path;

use crate::primitives::{Result, list_feature_dirs, rel_path};
use crate::schema::paths;
use crate::schema::primitives::{
    DeriveRoutingCandidatesArgs, DeriveRoutingCandidatesResult, RoutingCandidate, RoutingSkip,
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

/// Execute the `derive-routing-candidates` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::InvalidArgument`] when
/// `description` is empty or whitespace-only — there is nothing to route.
/// Every other failure is reported as a skipped source rather than raised: a
/// derivation that cannot run must be distinguishable from one that found
/// nothing, and an error would collapse the two into "no gate".
pub fn run(
    args: &DeriveRoutingCandidatesArgs,
    repo: &Path,
) -> Result<DeriveRoutingCandidatesResult> {
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
        return Ok(DeriveRoutingCandidatesResult {
            description: args.description.clone(),
            candidates: Vec::new(),
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

    derive_runtime_work(
        &args.description,
        repo,
        &mut candidates,
        &mut examined,
        &mut skipped,
    );
    derive_rule_surfaces(&tokens, repo, &mut candidates, &mut examined, &mut skipped);
    derive_spec_corpus(&tokens, repo, &mut candidates, &mut examined, &mut skipped);

    let derivation_incomplete = !skipped.is_empty();
    Ok(DeriveRoutingCandidatesResult {
        description: args.description.clone(),
        candidates,
        sources_examined: examined,
        skipped,
        gate_required: true,
        derivation_incomplete,
    })
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
) {
    let Some(signal) = runtime_signal(description) else {
        examined.push("runtime-work".into());
        return;
    };
    let layout = paths::Paths::load(repo);
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
        return;
    }
    match runtime_home(&specs_dir) {
        Some(slug) => {
            let feature_dir = specs_dir.join(&slug);
            let status = read_status(&feature_dir);
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
fn runtime_home(specs_dir: &Path) -> Option<String> {
    list_feature_dirs(specs_dir).into_iter().find(|slug| {
        crate::primitives::compute_review_scope::read_plan_affected(&specs_dir.join(slug))
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
) {
    let (dir, dir_rel) = crate::primitives::discover_rule_files::resolve_rules_dir(repo);
    let Some(dir) = dir else {
        // No rule directory at all is a real, examined answer: this project
        // has no rule surfaces, so none can be a home. A fresh adopter is
        // exactly this case and must see no new friction.
        examined.push("rule-surface".into());
        return;
    };
    let files = match crate::primitives::discover_rule_files::list_rule_files(&dir) {
        Ok(files) => files,
        Err(err) => {
            skipped.push(RoutingSkip {
                source: "rule-surface".into(),
                reason: format!("rule-file directory could not be listed: {err}"),
                path: dir_rel,
            });
            return;
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
) {
    let layout = paths::Paths::load(repo);
    let specs_dir = repo.join(&layout.specs_root);
    if !specs_dir.is_dir() {
        skipped.push(RoutingSkip {
            source: "spec-corpus".into(),
            reason: "spec root is missing or unreadable".into(),
            path: layout.specs_root.clone(),
        });
        return;
    }
    for slug in list_feature_dirs(&specs_dir) {
        // Already offered by the runtime-work source, which is the more
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
        let status = read_status(&feature_dir);
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
}

/// A feature slug's words: everything after the `NNN-` prefix.
fn slug_words(slug: &str) -> &str {
    slug.get(4..).unwrap_or(slug)
}

/// A spec's frontmatter `status`, or empty when the file is absent or its
/// frontmatter unreadable. Empty is a truthful "not known" — it only softens
/// the confirmation prompt (no reopen is claimed), never fabricates one.
fn read_status(feature_dir: &Path) -> String {
    let Ok(content) = std::fs::read_to_string(feature_dir.join("spec.md")) else {
        return String::new();
    };
    let Ok((frontmatter, _)) =
        crate::primitives::split_frontmatter(&content, &feature_dir.join("spec.md"))
    else {
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
        assert!(result.skipped.is_empty());
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
        assert!(result.skipped.is_empty());
        assert!(!result.derivation_incomplete);
        assert_eq!(
            result.sources_examined,
            vec!["runtime-work", "rule-surface", "spec-corpus"],
            "every source must account for itself"
        );
        assert!(result.gate_required);
    }

    #[test]
    fn unreadable_spec_root_is_skipped_rather_than_read_as_no_candidates() {
        let tmp = tempdir().unwrap();
        let result = run(&args("webhook delivery retries"), tmp.path()).unwrap();
        assert!(result.candidates.is_empty());
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
            ("webhook delivery", tempdir().unwrap()),
        ] {
            let result = run(&args(description), repo.path()).unwrap();
            for source in ["runtime-work", "rule-surface", "spec-corpus"] {
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
        assert!(!result.candidates.is_empty());
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
        let description = "quality rules for the check-artifacts family and security review";
        let first = run(&args(description), tmp.path()).unwrap();
        let second = run(&args(description), tmp.path()).unwrap();
        assert_eq!(first, second, "the derivation must be deterministic");
        let sources: Vec<&str> = first.candidates.iter().map(|c| c.source.as_str()).collect();
        let mut sorted = sources.clone();
        sorted.sort_by_key(|s| match *s {
            "runtime-work" => 0,
            "rule-surface" => 1,
            _ => 2,
        });
        assert_eq!(sources, sorted, "{:?}", first.candidates);
    }
}
