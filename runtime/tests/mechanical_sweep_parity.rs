//! Conformance: the rename-sweep exemption holds in both directions.
//!
//! The exemption (`runtime/src/primitives/mechanical_sweep.rs`) decides two
//! things: whether a uniform rename sweep leaves a record's digest current, as
//! the analysis record's freshness reads it at the gate, and whether an edit
//! under a `done` spec is mechanical, as `classify-spec-edit` reads it. The
//! review record's freshness check, and Family 19 with `check-review-freshness`
//! that asked it at release, were retired by spec 081, since a review is owed
//! for code alone.
//!
//! What this holds is the exemption itself, on repositories each test builds
//! with a known sweep and a known structural edit, asserted in both directions.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

/// The exemption's verdict over `candidates`: the paths that changed beyond a
/// uniform rename sweep between `base` and `HEAD`.
fn rust_verdict(
    root: &Path,
    base: &str,
    candidates: &BTreeSet<String>,
    rename_limit: Option<usize>,
) -> BTreeSet<String> {
    let repo = git2::Repository::open(root).expect("open fixture");
    let head = repo
        .head()
        .expect("HEAD")
        .peel_to_commit()
        .expect("HEAD commit")
        .tree()
        .expect("HEAD tree");
    let base_tree = repo
        .revparse_single(base)
        .expect("base rev")
        .peel_to_commit()
        .expect("base commit")
        .tree()
        .expect("base tree");
    let index = rename_limit.map_or_else(
        || ductus::primitives::mechanical_sweep::SweepIndex::build(&repo, &base_tree, &head),
        |limit| {
            ductus::primitives::mechanical_sweep::SweepIndex::build_with_rename_limit(
                &repo, &base_tree, &head, limit,
            )
        },
    );
    candidates
        .iter()
        .filter(|p| index.changed_beyond_spelling(p))
        .cloned()
        .collect()
}

/// Durable contracts under `slug` that changed since `base`, before any
/// exemption — the candidate set the exemption starts from.
fn changed_contracts(root: &Path, slug: &str, base: &str) -> BTreeSet<String> {
    let out = common::git_command()
        .args([
            "-C",
            root.to_str().unwrap(),
            "diff",
            "--name-only",
            "--no-color",
        ])
        .arg(format!("{base}..HEAD"))
        .output()
        .expect("git diff");
    let prefix = format!("specs/{slug}/");
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|p| p.strip_prefix(&prefix).map(|rest| (p, rest)))
        .filter(|(_, rest)| {
            // Mirrors `analyze_subjects::is_review_contract`, which compares
            // the extension case-insensitively.
            (rest.starts_with("scenarios/")
                && std::path::Path::new(rest)
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("md")))
                || *rest == "data-model.md"
        })
        .map(|(p, _)| p.to_string())
        .collect()
}

/// Commit everything staged in `repo`, on top of HEAD when one exists.
///
/// Fixed-time signature so the fixture's shas are stable run to run; nothing
/// here reaches a golden, but a stable sha makes a failure reproducible from
/// the message alone.
fn commit_all(repo: &git2::Repository, message: &str, seconds: i64) -> String {
    use git2::{IndexAddOption, Signature, Time};
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"], IndexAddOption::DEFAULT, None)
        .expect("git add");
    index.write().expect("index write");
    let tree_id = index.write_tree().expect("write tree");
    let tree = repo.find_tree(tree_id).unwrap();
    let when = Time::new(seconds, 0);
    let sig = Signature::new("Sweep Fixture", "sweep@example.com", &when).expect("signature");
    let parents = match repo.head() {
        Ok(head) => vec![head.peel_to_commit().expect("HEAD commit")],
        Err(_) => Vec::new(),
    };
    let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parent_refs)
        .expect("commit")
        .to_string()
}

/// A throwaway repository whose `base..HEAD` window is a uniform
/// `govern` → `ductus` substitution across three durable contracts, with a
/// structural edit added to whichever contracts `structural` names.
///
/// Three files rather than two because the exemption turns on a pair being
/// **repo-wide** — a substitution appearing in only one file is not one, so a
/// two-file fixture with one structural edit would leave the remaining
/// substitution unexplained and prove nothing about the exempt path.
///
/// Returns the temp dir (the caller keeps it alive) and the base sha.
fn build_sweep_fixture(structural: &[&str]) -> (tempfile::TempDir, String) {
    const CONTRACTS: [&str; 3] = ["alpha.md", "beta.md", "gamma.md"];

    let tmp = tempfile::tempdir().expect("tempdir");
    let feature = tmp.path().join("specs").join("900-fixture");
    std::fs::create_dir_all(feature.join("scenarios")).expect("fixture dirs");
    std::fs::write(
        feature.join("spec.md"),
        "---\nstatus: done\n---\n\n# Fixture\n",
    )
    .expect("write spec.md");
    for name in CONTRACTS {
        std::fs::write(
            feature.join("scenarios").join(name),
            format!("# {name}\n\nThe `govern` runtime reads `govern` config.\n"),
        )
        .expect("write base contract");
    }

    let repo = git2::Repository::init(tmp.path()).expect("git init");
    let base = commit_all(&repo, "fixture: base", 1_704_067_200);

    for name in CONTRACTS {
        let mut body = format!("# {name}\n\nThe `ductus` runtime reads `ductus` config.\n");
        if structural.contains(&name) {
            body.push_str("\nAn added sentence that no substitution explains.\n");
        }
        std::fs::write(feature.join("scenarios").join(name), body).expect("write swept contract");
    }
    commit_all(&repo, "fixture: sweep", 1_704_067_201);

    (tmp, base)
}

/// The exemption with a subject this test builds, in both directions: a pure
/// repo-wide sweep exempts every contract it touched, and one structural edit
/// is stale without costing the others their exemption.
#[test]
fn the_sweep_exemption_holds_on_a_built_sweep() {
    // (contracts receiving a structural edit, contracts expected to read stale)
    let cases: [(&[&str], &[&str]); 2] = [
        (&[], &[]),
        (&["beta.md"], &["specs/900-fixture/scenarios/beta.md"]),
    ];
    for (structural, expected) in cases {
        let (tmp, base) = build_sweep_fixture(structural);
        let candidates = changed_contracts(tmp.path(), "900-fixture", &base);
        assert_eq!(
            candidates.len(),
            3,
            "the fixture must present all three contracts as changed, or this \
             test is measuring something other than it claims"
        );
        let want: BTreeSet<String> = expected.iter().map(|s| (*s).to_string()).collect();
        assert_eq!(
            rust_verdict(tmp.path(), &base, &candidates, None),
            want,
            "verdict wrong for structural={structural:?}"
        );
    }
}

/// A throwaway repository whose `base..HEAD` window **renumbers** a spec
/// directory — `063-fixture` → `064-fixture` — and rewrites the references to
/// it inside the three contracts it carries, with a structural edit added to
/// whichever contracts `structural` names (spec 066 scenario
/// `a-renamed-file-contributes-its-rewrites`).
///
/// Every rewrite lands in a renamed file, so the pairs are repo-wide only if a
/// half diffs each renamed file against its old path. Read as a delete plus an
/// add, the window holds no substitution at all and every contract reads
/// stale — which is what the fixture is built to catch.
///
/// The repository's own config is set against the exemption: rename
/// detection off, a rename limit of one, no diff prefixes, colour forced on.
/// An exemption that inherited any of it would read this window differently
/// from CI.
fn build_renumber_fixture(structural: &[&str]) -> (tempfile::TempDir, String) {
    const CONTRACTS: [&str; 3] = ["alpha.md", "beta.md", "gamma.md"];
    const STABLE: &str = "## Context\n\nA contract long enough that one rewritten line leaves it \
         well above the rename threshold.\n\n## Behavior\n\nThe behavior this scenario \
         describes, unchanged by the renumber.\n\n## Edge Cases\n\n- None.\n";

    let tmp = tempfile::tempdir().expect("tempdir");
    let specs = tmp.path().join("specs");
    let write_feature = |slug: &str, number: &str, extra: &[&str]| {
        let feature = specs.join(slug);
        std::fs::create_dir_all(feature.join("scenarios")).expect("fixture dirs");
        std::fs::write(
            feature.join("spec.md"),
            "---\nstatus: done\n---\n\n# Fixture\n",
        )
        .expect("write spec.md");
        for name in CONTRACTS {
            let mut body = format!(
                "# {name}\n\nPart of `{number}-fixture`; see [{number}](../../{number}-fixture/spec.md).\n\n{STABLE}"
            );
            if extra.contains(&name) {
                body.push_str("\nAn added sentence that no substitution explains.\n");
            }
            std::fs::write(feature.join("scenarios").join(name), body).expect("write contract");
        }
    };

    write_feature("063-fixture", "063", &[]);
    let repo = git2::Repository::init(tmp.path()).expect("git init");
    {
        // The fixture's own `.git/config`, opened by path, so these writes
        // can never land in the contributor's global config.
        let mut config =
            git2::Config::open(&repo.path().join("config")).expect("fixture repo config");
        config.set_bool("diff.renames", false).unwrap();
        config.set_i32("diff.renameLimit", 1).unwrap();
        config.set_bool("diff.noprefix", true).unwrap();
        config.set_str("color.ui", "always").unwrap();
    }
    let base = commit_all(&repo, "fixture: base", 1_704_067_200);

    std::fs::remove_dir_all(specs.join("063-fixture")).expect("remove old dir");
    write_feature("064-fixture", "064", structural);
    commit_all_with_removals(&repo, "fixture: renumber", 1_704_067_201);

    (tmp, base)
}

/// [`commit_all`] for a window that also deletes paths: `add_all` stages
/// new and modified files only, so the removed directory is dropped from the
/// index by `update_all` first.
fn commit_all_with_removals(repo: &git2::Repository, message: &str, seconds: i64) -> String {
    let mut index = repo.index().unwrap();
    index.update_all(["*"], None).expect("git add -u");
    index.write().expect("index write");
    commit_all(repo, message, seconds)
}

/// The renumber fixture's changed contracts and the exemption's verdict.
fn renumber_verdict(
    structural: &[&str],
    rename_limit: Option<usize>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let (tmp, base) = build_renumber_fixture(structural);
    let candidates = changed_contracts(tmp.path(), "064-fixture", &base);
    assert_eq!(
        candidates.len(),
        3,
        "the fixture must present all three renamed contracts as changed, or \
         this test is measuring something other than it claims: {candidates:?}"
    );
    let verdict = rust_verdict(tmp.path(), &base, &candidates, rename_limit);
    (candidates, verdict)
}

/// A renamed file contributes its rewrites, in both directions, whatever
/// rename settings the repository it reads carries.
#[test]
fn the_sweep_exemption_holds_on_a_renumbered_directory() {
    let cases: [(&[&str], &[&str]); 2] = [
        // A pure renumber exempts every renamed contract. Under a
        // delete-plus-add reading all three would be stale.
        (&[], &[]),
        (&["beta.md"], &["specs/064-fixture/scenarios/beta.md"]),
    ];
    for (structural, expected) in cases {
        let (_, verdict) = renumber_verdict(structural, None);
        let want: BTreeSet<String> = expected.iter().map(|s| (*s).to_string()).collect();
        assert_eq!(verdict, want, "verdict wrong for structural={structural:?}");
    }
}

/// A file renamed with its content unchanged has no hunk, so it is not
/// indexed and reads as changed, never as an empty and therefore exempt
/// entry. The fixture's `spec.md` is one.
#[test]
fn a_pure_rename_is_a_change() {
    let (tmp, base) = build_renumber_fixture(&[]);
    let path = "specs/064-fixture/spec.md".to_string();
    let verdict = rust_verdict(tmp.path(), &base, &BTreeSet::from([path.clone()]), None);
    assert!(
        verdict.contains(&path),
        "a rename read with no hunk must not be exempt"
    );
}

/// Over the window guard no exemption is granted, so every changed contract
/// reads stale. The window holds eight paths before renames pair them (four
/// files, each deleted and added), so a limit of seven is the first it
/// exceeds — and eight still reads it.
#[test]
fn an_over_limit_window_grants_no_exemption() {
    let (candidates, verdict) = renumber_verdict(&[], Some(7));
    assert_eq!(verdict, candidates, "nothing is granted over the limit");
    let (_, verdict) = renumber_verdict(&[], Some(8));
    assert!(
        verdict.is_empty(),
        "at the limit the window is read: {verdict:?}"
    );
}

/// The tests that build their own repository, each running git in it.
fn fixture_tests() -> Vec<&'static str> {
    vec![
        "the_sweep_exemption_holds_on_a_built_sweep",
        "the_sweep_exemption_holds_on_a_renumbered_directory",
        "a_pure_rename_is_a_change",
        "an_over_limit_window_grants_no_exemption",
    ]
}

/// A commit from a linked worktree hands its pre-commit hook an absolute
/// `GIT_DIR`, `GIT_WORK_TREE` and `GIT_INDEX_FILE`, and the hook runs this
/// suite, so each fixture test must act on its own repository with all three
/// naming another one (066 task 8). Each is re-run in a child of this binary
/// with them set to a repository it never builds, and must pass, and must
/// have run: a filter that matches no test exits 0 too.
#[test]
fn the_fixture_tests_pass_under_a_hooks_git_variables() {
    let other = tempfile::tempdir().unwrap();
    git2::Repository::init(other.path()).unwrap();
    let git_dir = other.path().join(".git");
    for name in fixture_tests() {
        let out = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", name])
            .env("GIT_DIR", &git_dir)
            .env("GIT_WORK_TREE", other.path())
            .env("GIT_INDEX_FILE", git_dir.join("index"))
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "{name} fails with a hook's git variables set:\n{stdout}{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(stdout.contains("1 passed"), "{name} did not run:\n{stdout}");
    }
}
