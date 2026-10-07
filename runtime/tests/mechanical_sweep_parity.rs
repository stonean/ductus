//! Conformance: the Rust staleness rule and `/ductus:audit` Family 19 agree.
//!
//! "Is this review stale?" is enforced at two moments — `check-review-gate` on
//! the `in-progress → done` transition, and Family 19
//! (`scripts/audit/review-freshness.sh`) as a release gate. One rule, two
//! implementations, in two languages.
//!
//! That is not the shape anyone would choose, and it is worth saying why it
//! stands: the CI job that runs the self-audit checks out the repo with no
//! Rust toolchain and no runtime build (`.github/workflows/runtime-release.yml`,
//! job `audit`), because it gates the build. Making Family 19 call the runtime
//! would make the gate depend on compiling the artifact it gates. So the two
//! implementations stay, and this test is what keeps them honest.
//!
//! They disagreed once, silently, and it was expensive: until 2026-08-16 the
//! Rust side had no mechanical-sweep exemption while Family 19 did, and the two
//! answered differently for **19 of this repo's 46 `done` specs** — every one a
//! consequence of 049's `govern → ductus` rename. Nothing compared them, so
//! nothing said so.
//!
//! Two tests carry that. `rust_and_family_19_agree_on_a_built_sweep` builds
//! its own repository with a known sweep and a known structural edit, so the
//! parity rule is asserted deterministically and in **both** directions —
//! exemption granted and exemption refused. `rust_and_family_19_agree_on_every_done_spec`
//! then runs both over the real corpus and fails on the first spec where they
//! differ; it asserts agreement, not a particular verdict, so it stays valid
//! as the corpus changes.
//!
//! The corpus pass used to carry the vacuity guard itself, and that was wrong
//! in a way that took until 2026-09-15 to surface: its subject is the set of
//! `done` specs whose contracts differ from their `reviewed-against`, and for
//! most of this repo's history **exactly one spec** supplied it — 020's
//! `data-model.md`, stale but sweep-exempt. Re-reviewing that one spec emptied
//! the set and turned the guard red, so the test was asserting that an
//! *unhealthy* corpus must exist for it to mean anything. An empty set there
//! is a healthy corpus, not a broken check; the guard belongs on the built
//! fixture, which always has a subject.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Every `done` spec's `(slug, reviewed-against)`, skipping the grandfathered
/// ones (no `review.md`) and any whose sha does not resolve — the two cases
/// both implementations decline to judge.
///
/// The status comes from `spec.md` and the sha from `review.md`, because spec
/// 057 moved the record to the artifact that owns it. Reading the retired
/// `spec.md` block would build an **empty** subject over a migrated corpus,
/// which this file's own vacuity guard then reports as a check that could not
/// run — correctly, and that is how the relocation surfaced here.
fn reviewed_done_specs(root: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let specs = root.join("specs");
    let Ok(entries) = std::fs::read_dir(&specs) else {
        return out;
    };
    let mut dirs: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    for dir in dirs {
        let spec = dir.join("spec.md");
        let Ok(text) = std::fs::read_to_string(&spec) else {
            continue;
        };
        let Some(fm) = text
            .strip_prefix("---\n")
            .and_then(|r| r.split("\n---").next())
        else {
            continue;
        };
        if !fm.lines().any(|l| l.trim() == "status: done") {
            continue;
        }
        let Ok(review) = std::fs::read_to_string(dir.join("review.md")) else {
            continue; // grandfathered: the absent artifact is never-reviewed
        };
        let Some(review_fm) = review
            .strip_prefix("---\n")
            .and_then(|r| r.split("\n---").next())
        else {
            continue;
        };
        let Some(base) = review_fm
            .lines()
            .find_map(|l| l.trim().strip_prefix("reviewed-against:"))
            .map(|v| v.trim().trim_matches('"').to_string())
            .filter(|v| !v.is_empty())
        else {
            continue;
        };
        let resolves = Command::new("git")
            .args(["-C", root.to_str().unwrap(), "cat-file", "-e"])
            .arg(format!("{base}^{{commit}}"))
            .status()
            .is_ok_and(|s| s.success());
        if !resolves {
            continue;
        }
        out.push((
            dir.file_name().unwrap().to_string_lossy().into_owned(),
            base,
        ));
    }
    out
}

/// Durable contracts under `slug` that changed since `base`, before any
/// exemption — the candidate set both implementations start from.
fn changed_contracts(root: &Path, slug: &str, base: &str) -> BTreeSet<String> {
    let out = Command::new("git")
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
            // Mirrors both implementations' `is_durable_contract`, which
            // compare the extension case-insensitively.
            (rest.starts_with("scenarios/")
                && std::path::Path::new(rest)
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("md")))
                || *rest == "data-model.md"
        })
        .map(|(p, _)| p.to_string())
        .collect()
}

/// Family 19's verdict for one path, obtained by running its own
/// `changed_beyond_spelling` — the Python side, unmodified.
fn family_19_verdict(
    script_root: &Path,
    git_root: &Path,
    base: &str,
    paths: &BTreeSet<String>,
) -> BTreeSet<String> {
    family_19_verdict_with(script_root, git_root, base, paths, None)
}

/// [`family_19_verdict`], optionally with Family 19's `RENAME_LIMIT`
/// rebound — the Python counterpart of `SweepIndex::build_with_rename_limit`.
fn family_19_verdict_with(
    script_root: &Path,
    git_root: &Path,
    base: &str,
    paths: &BTreeSet<String>,
    rename_limit: Option<usize>,
) -> BTreeSet<String> {
    if paths.is_empty() {
        return BTreeSet::new();
    }
    let rebind = rename_limit.map_or_else(String::new, |n| format!("RENAME_LIMIT = {n}\n"));
    let script = std::fs::read_to_string(script_root.join("scripts/audit/review-freshness.sh"))
        .expect("Family 19 source");
    // Reuse the family's own definitions rather than restating them: take the
    // python block between its heredoc markers, drop the driver loop at the
    // end, and call `changed_beyond_spelling` directly.
    let body = script
        .split_once("python3 - \"$ROOT\" \"$SPECS_ROOT\" <<'PY'\n")
        .expect("python block start")
        .1
        .split_once("\nspecs_dir = root / specs_root")
        .expect("driver loop start")
        .0
        .to_string();
    let harness = format!(
        "{body}\n\
         {rebind}\
         import json, sys as _s\n\
         _base = _s.argv[3]\n\
         _paths = json.loads(_s.argv[4])\n\
         print(json.dumps([p for p in _paths if changed_beyond_spelling(_base, p)]))\n"
    );
    let out = Command::new("python3")
        .arg("-")
        .arg(git_root.to_str().unwrap())
        .arg("specs")
        .arg(base)
        .arg(serde_json::to_string(&paths.iter().collect::<Vec<_>>()).unwrap())
        .current_dir(git_root)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(harness.as_bytes())?;
            child.wait_with_output()
        })
        .expect("run Family 19's rule");
    assert!(
        out.status.success(),
        "Family 19 harness failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice::<Vec<String>>(&out.stdout)
        .expect("Family 19 verdict json")
        .into_iter()
        .collect()
}

#[test]
fn rust_and_family_19_agree_on_every_done_spec() {
    let root = repo_root();
    if !root.join(".git").exists() {
        eprintln!("skipping: not a git checkout");
        return;
    }
    let repo = git2::Repository::open(&root).expect("open repo");
    let head = repo
        .head()
        .expect("HEAD")
        .peel_to_commit()
        .expect("HEAD commit")
        .tree()
        .expect("HEAD tree");

    let specs = reviewed_done_specs(&root);
    assert!(
        !specs.is_empty(),
        "no done spec's reviewed-against sha resolved, so there is nothing to \
         compare — and a green result here would mean the check could not run. \
         On CI this is almost always a shallow checkout: this job needs \
         `fetch-depth: 0`, the same requirement Family 19's job carries. \
         Locally it means the recorded shas are absent from your history."
    );

    let mut compared = 0usize;
    for (slug, base) in &specs {
        let candidates = changed_contracts(&root, slug, base);
        if candidates.is_empty() {
            continue;
        }
        // revparse, not Oid::from_str — one spec records an abbreviated sha.
        let base_tree = repo
            .revparse_single(base)
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .tree()
            .unwrap();
        let index =
            ductus::primitives::mechanical_sweep::SweepIndex::build(&repo, &base_tree, &head);
        let rust: BTreeSet<String> = candidates
            .iter()
            .filter(|p| index.changed_beyond_spelling(p))
            .cloned()
            .collect();
        let python = family_19_verdict(&root, &root, base, &candidates);
        assert_eq!(
            rust, python,
            "staleness verdicts disagree for {slug} (reviewed-against {base}); \
             the transition gate and the release gate must answer the same question \
             the same way"
        );
        compared += 1;
    }
    // An empty set here means every `done` spec's contracts match its
    // `reviewed-against` — a healthy corpus, not a check that could not run.
    // §design-principles forbids rendering those two alike, so say which this
    // is. The parity rule's own vacuity guard lives on the built fixture
    // above, which cannot go empty.
    if compared == 0 {
        eprintln!(
            "no done spec carries a contract changed since its reviewed-against \
             — every review is current, so the corpus pass had no subject. The \
             rule itself is asserted by rust_and_family_19_agree_on_a_built_sweep."
        );
        return;
    }
    eprintln!(
        "compared {compared} spec(s) with changed contracts across {} done spec(s)",
        specs.len()
    );
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

/// The parity assertion with a subject it builds itself, in both directions.
///
/// This is where the vacuity guard belongs: the corpus pass can legitimately
/// find nothing to compare, but this one always has three changed contracts,
/// so a green result here always means the two implementations were actually
/// run against each other.
#[test]
fn rust_and_family_19_agree_on_a_built_sweep() {
    // (contracts receiving a structural edit, contracts expected to read stale)
    let cases: [(&[&str], &[&str]); 2] = [
        // Widened: a pure repo-wide sweep exempts every contract it touched.
        (&[], &[]),
        // Narrowed: one structural edit is stale, and does not cost the other
        // two their exemption — the pair is still repo-wide.
        (&["beta.md"], &["specs/900-fixture/scenarios/beta.md"]),
    ];

    for (structural, expected) in cases {
        let (tmp, base) = build_sweep_fixture(structural);
        let root = tmp.path();
        let repo = git2::Repository::open(root).expect("open fixture");
        let head = repo
            .head()
            .expect("HEAD")
            .peel_to_commit()
            .expect("HEAD commit")
            .tree()
            .expect("HEAD tree");
        let base_tree = repo
            .revparse_single(&base)
            .expect("base rev")
            .peel_to_commit()
            .expect("base commit")
            .tree()
            .expect("base tree");

        let candidates = changed_contracts(root, "900-fixture", &base);
        assert_eq!(
            candidates.len(),
            3,
            "the fixture must present all three contracts as changed, or this \
             test is measuring something other than it claims"
        );

        let index =
            ductus::primitives::mechanical_sweep::SweepIndex::build(&repo, &base_tree, &head);
        let rust: BTreeSet<String> = candidates
            .iter()
            .filter(|p| index.changed_beyond_spelling(p))
            .cloned()
            .collect();
        let python = family_19_verdict(&repo_root(), root, &base, &candidates);
        let want: BTreeSet<String> = expected.iter().map(|s| (*s).to_string()).collect();

        assert_eq!(
            rust, want,
            "Rust verdict wrong for structural={structural:?}"
        );
        assert_eq!(
            python, want,
            "Family 19 verdict wrong for structural={structural:?}"
        );
        assert_eq!(
            rust, python,
            "the transition gate and the release gate must answer the same \
             question the same way for structural={structural:?}"
        );
    }
}

/// A throwaway repository whose `base..HEAD` window **renumbers** a spec
/// directory — `063-fixture` → `064-fixture` — and rewrites the references to
/// it inside the three contracts it carries, with a structural edit added to
/// whichever contracts `structural` names (spec 022 scenario
/// `a-renamed-file-contributes-its-rewrites`).
///
/// Every rewrite lands in a renamed file, so the pairs are repo-wide only if a
/// half diffs each renamed file against its old path. Read as a delete plus an
/// add, the window holds no substitution at all and every contract reads
/// stale — which is what the fixture is built to catch.
///
/// The repository's own config is set against both halves: rename detection
/// off, a rename limit of one, no diff prefixes, colour forced on. A half that
/// inherited any of it would read this window differently from CI.
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

/// Both halves' verdicts over the renumber fixture's changed contracts.
fn renumber_verdicts(
    structural: &[&str],
    rename_limit: Option<usize>,
) -> (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>) {
    let (tmp, base) = build_renumber_fixture(structural);
    let root = tmp.path();
    let repo = git2::Repository::open(root).expect("open fixture");
    let head = repo
        .head()
        .expect("HEAD")
        .peel_to_commit()
        .expect("HEAD commit")
        .tree()
        .expect("HEAD tree");
    let base_tree = repo
        .revparse_single(&base)
        .expect("base rev")
        .peel_to_commit()
        .expect("base commit")
        .tree()
        .expect("base tree");

    let candidates = changed_contracts(root, "064-fixture", &base);
    assert_eq!(
        candidates.len(),
        3,
        "the fixture must present all three renamed contracts as changed, or \
         this test is measuring something other than it claims: {candidates:?}"
    );

    let index = rename_limit.map_or_else(
        || ductus::primitives::mechanical_sweep::SweepIndex::build(&repo, &base_tree, &head),
        |limit| {
            ductus::primitives::mechanical_sweep::SweepIndex::build_with_rename_limit(
                &repo, &base_tree, &head, limit,
            )
        },
    );
    let rust: BTreeSet<String> = candidates
        .iter()
        .filter(|p| index.changed_beyond_spelling(p))
        .cloned()
        .collect();
    let python = family_19_verdict_with(&repo_root(), root, &base, &candidates, rename_limit);
    (candidates, rust, python)
}

/// A renamed file contributes its rewrites, in both halves, in both
/// directions — and neither half takes its rename settings from the
/// repository it reads.
#[test]
fn rust_and_family_19_agree_on_a_renumbered_directory() {
    // (contracts receiving a structural edit, contracts expected to read stale)
    let cases: [(&[&str], &[&str]); 2] = [
        // A pure renumber exempts every renamed contract. Under a
        // delete-plus-add reading all three would be stale.
        (&[], &[]),
        // One structural edit is stale and costs the others nothing.
        (&["beta.md"], &["specs/064-fixture/scenarios/beta.md"]),
    ];
    for (structural, expected) in cases {
        let (_, rust, python) = renumber_verdicts(structural, None);
        let want: BTreeSet<String> = expected.iter().map(|s| (*s).to_string()).collect();
        assert_eq!(
            rust, want,
            "Rust verdict wrong for structural={structural:?}"
        );
        assert_eq!(
            python, want,
            "Family 19 verdict wrong for structural={structural:?}"
        );
    }
}

/// A file renamed with its content unchanged has no hunk, so neither half
/// indexes it and both read it as changed. The fixture's `spec.md` is one:
/// opening the entry at the file header, as the Rust half once did, made it an
/// empty and therefore exempt entry there while Family 19 — which opens it on
/// the `+++ b/` line such a file never carries — called it changed.
#[test]
fn a_pure_rename_reads_alike_in_both_halves() {
    let (tmp, base) = build_renumber_fixture(&[]);
    let root = tmp.path();
    let repo = git2::Repository::open(root).expect("open fixture");
    let head = repo
        .head()
        .unwrap()
        .peel_to_commit()
        .unwrap()
        .tree()
        .unwrap();
    let base_tree = repo
        .revparse_single(&base)
        .unwrap()
        .peel_to_commit()
        .unwrap()
        .tree()
        .unwrap();
    let path = "specs/064-fixture/spec.md".to_string();
    let index = ductus::primitives::mechanical_sweep::SweepIndex::build(&repo, &base_tree, &head);
    assert!(
        index.changed_beyond_spelling(&path),
        "Rust must not exempt a rename it read no hunk for"
    );
    let python = family_19_verdict(&repo_root(), root, &base, &BTreeSet::from([path.clone()]));
    assert!(
        python.contains(&path),
        "Family 19 must read the same rename as changed"
    );
}

/// Over the window guard, both halves refuse the window alike: no exemption,
/// so every changed contract reads stale. The fixture's window holds eight
/// paths before renames pair them (four files, each deleted and added), so a
/// limit of seven is the first one it exceeds — and eight still reads it.
#[test]
fn rust_and_family_19_refuse_an_over_limit_window_alike() {
    let (candidates, rust, python) = renumber_verdicts(&[], Some(7));
    assert_eq!(rust, candidates, "Rust must grant nothing over the limit");
    assert_eq!(
        python, candidates,
        "Family 19 must grant nothing over the limit"
    );

    let (_, rust, python) = renumber_verdicts(&[], Some(8));
    assert!(rust.is_empty(), "at the limit the window is read: {rust:?}");
    assert!(
        python.is_empty(),
        "at the limit the window is read: {python:?}"
    );
}

/// The rename settings are stated once per half and must match: the Rust
/// constants are the reference, and Family 19's source has to carry the same
/// values where its diff reads them.
#[test]
fn the_rename_settings_are_stated_alike_in_both_halves() {
    use ductus::primitives::mechanical_sweep::{RENAME_LIMIT, RENAME_THRESHOLD};
    let script = std::fs::read_to_string(repo_root().join("scripts/audit/review-freshness.sh"))
        .expect("Family 19 source");
    for needle in [
        format!("\nRENAME_THRESHOLD = {RENAME_THRESHOLD}\n"),
        format!("\nRENAME_LIMIT = {RENAME_LIMIT}\n"),
        "f\"--find-renames={RENAME_THRESHOLD}%\"".to_string(),
        "f\"-l{RENAME_LIMIT}\"".to_string(),
    ] {
        assert!(
            script.contains(&needle),
            "Family 19 must carry {needle:?} — the halves' rename settings have drifted"
        );
    }
}

/// Run Family 19 — the whole script, as the release gate runs it — over a
/// fixture whose one `done` spec carries a **pre-digest** review record, so
/// its proxy arm judges it. Since the review, a sweep renamed one contract
/// within the spec (`old.md` → `new.md`) and rewrote a token in it and in a
/// sibling contract. `renames` sets the fixture repository's `diff.renames`.
/// Returns whether the script passed, and its stdout.
#[cfg(unix)]
fn run_family_19_over_a_moved_contract(renames: bool) -> (bool, String) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();
    let audit = root.join("scripts/audit");
    std::fs::create_dir_all(&audit).expect("audit dir");
    for script in ["review-freshness.sh", "lib.sh"] {
        std::fs::copy(
            repo_root().join("scripts/audit").join(script),
            audit.join(script),
        )
        .expect("copy Family 19");
    }
    let feature = root.join("specs/900-proxy");
    let scenarios = feature.join("scenarios");
    std::fs::create_dir_all(&scenarios).expect("fixture dirs");
    std::fs::write(
        feature.join("spec.md"),
        "---\nstatus: done\n---\n\n# Proxy\n",
    )
    .expect("write spec.md");
    let body = |token: &str| {
        format!(
            "# Contract\n\nThe `{token}` runtime reads its config.\n\n## Context\n\n\
             Enough unchanged prose that one rewritten line leaves the file well\n\
             above any rename threshold.\n\n## Behavior\n\nUnchanged.\n"
        )
    };
    std::fs::write(scenarios.join("old.md"), body("govern")).expect("write old.md");
    std::fs::write(scenarios.join("other.md"), body("govern")).expect("write other.md");

    let repo = git2::Repository::init(root).expect("git init");
    git2::Config::open(&repo.path().join("config"))
        .expect("fixture repo config")
        .set_bool("diff.renames", renames)
        .unwrap();
    let base = commit_all(&repo, "fixture: base", 1_704_067_200);

    // A pre-digest record: `reviewed-against` and no `reviewed-digest`.
    std::fs::write(
        feature.join("review.md"),
        format!(
            "---\nspec: 900-proxy\nlast-run: 2024-01-01T00:00:00Z\n\
             reviewed-against: {base}\n---\n\n# Review\n"
        ),
    )
    .expect("write review.md");
    std::fs::remove_file(scenarios.join("old.md")).expect("move old.md away");
    std::fs::write(scenarios.join("new.md"), body("ductus")).expect("write new.md");
    std::fs::write(scenarios.join("other.md"), body("ductus")).expect("sweep other.md");
    commit_all_with_removals(&repo, "fixture: sweep and move", 1_704_067_201);

    let out = Command::new("bash")
        .arg(audit.join("review-freshness.sh"))
        .current_dir(root)
        .output()
        .expect("run Family 19");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// The proxy arm's verdict does not depend on who runs it. A contract that
/// left its path is a change — the verdict the digest arm reaches on the same
/// move, whose old key has vanished — so the moved contract reads stale with
/// `diff.renames` on and off alike. Left to config, renames-on listed only the
/// new path, which the sweep exemption cleared, and the spec read clean on a
/// contributor's machine while CI could call it stale.
///
/// Unix-only: it runs the script under `bash`, as the toolchain-free
/// framework-checks job on Ubuntu does, which is the only place it gates.
#[cfg(unix)]
#[test]
fn family_19s_proxy_arm_lists_both_sides_of_a_move_under_any_config() {
    for renames in [true, false] {
        let (passed, stdout) = run_family_19_over_a_moved_contract(renames);
        assert!(
            !passed && stdout.contains("specs/900-proxy/scenarios/old.md"),
            "with diff.renames={renames} the proxy arm must report the moved \
             contract's old path as changed; stdout:\n{stdout}"
        );
    }
}
