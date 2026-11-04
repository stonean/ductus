//! Conformance: `/ductus:audit` Family 19 reports what the runtime answers,
//! and the rename-sweep exemption both gates share holds in both directions.
//!
//! "Is this review stale?" is enforced at two moments — `check-review-gate` on
//! the `in-progress → done` transition, and Family 19
//! (`scripts/audit/review-freshness.sh`) as a release gate. Since 2026-11-04
//! they are one implementation: Family 19 asks `check-review-freshness`, which
//! makes the gate's own comparison (066 plan, *Family 19 calls the runtime*).
//! Until then Family 19 carried a Python copy of the rule, kept because the
//! self-audit job had no Rust toolchain, and this file held the two together:
//! they disagreed once, silently, for **19 of this repo's 46 `done` specs**
//! (066 scenario `review-staleness-on-done-specs`).
//!
//! What is left to hold is the exemption itself, on repositories each test
//! builds with a known sweep and a known structural edit, asserted in both
//! directions, and that Family 19, run as the release gate runs it, turns each
//! answer into the right finding or none.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

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

/// A `done` spec under `root` with one scenario, whose review record
/// carries that scenario's digest when `digest` holds.
#[cfg(unix)]
fn seed_reviewed_done_spec(root: &Path, slug: &str, digest: bool) {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;
    let feature = root.join("specs").join(slug);
    std::fs::create_dir_all(feature.join("scenarios")).unwrap();
    std::fs::write(
        feature.join("spec.md"),
        "---\nstatus: done\ndependencies: []\n---\n\n# Fixture\n",
    )
    .unwrap();
    let scenario = "# Edge\n\nAs reviewed.\n";
    std::fs::write(feature.join("scenarios/edge.md"), scenario).unwrap();
    let mut record = format!(
        "---\nspec: {slug}\nlast-run: 2026-11-04T00:00:00Z\nreviewed-against: HEAD\n\
         must-violations: 0\nshould-violations: 0\nlow-confidence: 0\nblocking: false\n"
    );
    if digest {
        let mut hex = String::new();
        for byte in Sha256::digest(scenario.as_bytes()) {
            let _ = write!(hex, "{byte:02x}");
        }
        let _ = writeln!(record, "reviewed-digest:\n  scenarios/edge.md: {hex}");
    }
    record.push_str("---\n\n# Review\n");
    std::fs::write(feature.join("review.md"), record).unwrap();
}

/// Family 19, run as the release gate runs it — the whole script, the
/// runtime reached through `.ductus/bin/ductus` — over a corpus holding each
/// answer: a current review, a stale one, one with no digest, and a `done`
/// spec with no `review.md`. Returns whether it passed, its stdout (the
/// findings) and its stderr (the coverage line).
#[cfg(unix)]
fn run_family_19_over_each_answer() -> (bool, String, String) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();
    let audit = root.join("scripts/audit");
    std::fs::create_dir_all(&audit).unwrap();
    for script in ["review-freshness.sh", "lib.sh"] {
        std::fs::copy(
            repo_root().join("scripts/audit").join(script),
            audit.join(script),
        )
        .expect("copy Family 19");
    }
    std::fs::create_dir_all(root.join(".ductus/bin")).unwrap();
    std::os::unix::fs::symlink(
        env!("CARGO_BIN_EXE_ductus"),
        root.join(".ductus/bin/ductus"),
    )
    .unwrap();
    seed_reviewed_done_spec(root, "901-current", true);
    seed_reviewed_done_spec(root, "902-stale", true);
    seed_reviewed_done_spec(root, "903-no-digest", false);
    let grandfathered = root.join("specs/904-unreviewed");
    std::fs::create_dir_all(&grandfathered).unwrap();
    std::fs::write(
        grandfathered.join("spec.md"),
        "---\nstatus: done\ndependencies: []\n---\n\n# Fixture\n",
    )
    .unwrap();
    git2::Repository::init(root).expect("git init");
    commit_all(
        &git2::Repository::open(root).unwrap(),
        "fixture: reviewed",
        1_704_067_200,
    );
    std::fs::write(
        root.join("specs/902-stale/scenarios/edge.md"),
        "# Edge\n\nChanged after the review.\n",
    )
    .unwrap();

    let out = common::without_git_hook_variables(Command::new("bash"))
        .arg(audit.join("review-freshness.sh"))
        .current_dir(root)
        .output()
        .expect("run Family 19");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Family 19 reports a stale review and one it cannot judge as findings,
/// passes a current one, grandfathers a spec with no `review.md`, and says
/// how many of each it saw.
///
/// Unix-only: it runs the script under `bash` and reaches the runtime through
/// a symlink, as the framework-checks job on Ubuntu does.
#[cfg(unix)]
#[test]
fn family_19_reports_each_answer_the_runtime_gives() {
    let (passed, stdout, stderr) = run_family_19_over_each_answer();
    assert!(
        !passed,
        "a stale review must fail the family:\n{stdout}{stderr}"
    );
    assert!(
        stdout.contains("specs/902-stale | done spec's review is stale")
            && stdout.contains("specs/902-stale/scenarios/edge.md"),
        "the stale review is not reported:\n{stdout}"
    );
    assert!(
        stdout.contains("specs/903-no-digest | done spec's review freshness cannot be determined"),
        "the record with no digest is not reported:\n{stdout}"
    );
    assert!(
        !stdout.contains("901-current") && !stdout.contains("904-unreviewed"),
        "a current or grandfathered spec is reported:\n{stdout}"
    );
    assert!(
        stderr.contains(
            "examined 2 spec(s) at status: done by reviewed-digest; 1 grandfathered \
             (no review.md); 1 undeterminable; 0 never run; 0 not answered"
        ),
        "the coverage line does not account for every spec:\n{stderr}"
    );
}

/// The tests that build their own repository, each running git in it,
/// directly or through `review-freshness.sh`. The Family 19 test is compiled on
/// Unix alone, so it is re-run there alone: asking for it elsewhere runs
/// nothing.
fn fixture_tests() -> Vec<&'static str> {
    let mut tests = vec![
        "the_sweep_exemption_holds_on_a_built_sweep",
        "the_sweep_exemption_holds_on_a_renumbered_directory",
        "a_pure_rename_is_a_change",
        "an_over_limit_window_grants_no_exemption",
    ];
    if cfg!(unix) {
        tests.push("family_19_reports_each_answer_the_runtime_gives");
    }
    tests
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
