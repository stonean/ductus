//! End-to-end golden tests for `derive-dependencies`.
//!
//! Every fixture is built **deliberately out of sync**: each spec's
//! `dependencies:` starts wrong, and the primitive must rewrite it to the
//! bytes in `tests/golden/derive-generators/deps-*.md`. Asserting on an
//! already-in-sync corpus would be very nearly vacuous — it passes even if the
//! harvester finds nothing, because there is no change to find either way.
//!
//! **Provenance of the goldens.** These files were not hand-authored. Each was
//! blessed from a run of `.ductus/scripts/gen-spec-deps.sh` — the shell
//! generator this primitive replaced — over the same fixture, and written only
//! after a byte-equality assertion against the primitive passed. So the
//! expected output encodes the behavior of the implementation that ran on
//! every adopter's machine for the life of spec 017, and it keeps encoding it
//! after that script is gone. Re-bless with `BLESS=1` only for a deliberate
//! behavior change.

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use git2::{IndexAddOption, Repository, Signature};

mod common;

/// One fixture spec: directory slug, and the full file contents.
struct SpecFile {
    slug: &'static str,
    body: &'static str,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn write(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

/// Stand up a git repo containing `specs` under `root`, all tracked and
/// committed. Returns the repo directory.
fn build_fixture(dir: &Path, specs_root: &str, specs: &[SpecFile], config: Option<&str>) {
    let repository = Repository::init(dir).unwrap();
    if let Some(config_body) = config {
        write(&dir.join(".ductus/config.toml"), config_body);
    }
    for spec in specs {
        write(
            &dir.join(specs_root).join(spec.slug).join("spec.md"),
            spec.body,
        );
    }
    let mut index = repository.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repository.find_tree(tree_id).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    repository
        .commit(Some("HEAD"), &sig, &sig, "fixture", &tree, &[])
        .unwrap();
}

/// Run the primitive against `dir` via the real CLI surface.
fn run_primitive(dir: &Path) -> serde_json::Value {
    run_primitive_with(dir, &["--write"]).1
}

/// Run the primitive with extra flags. Returns `(exit code, payload)` — the
/// exit code carries the CLI's blocking contract (non-zero on a cycle, or on
/// drift under `--dry-run`), which is what a pre-commit hook and CI read.
fn run_primitive_with(dir: &Path, flags: &[&str]) -> (i32, serde_json::Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_ductus"))
        .arg("derive-dependencies")
        .args(flags)
        .current_dir(dir)
        .output()
        .expect("binary runs");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("primitive emitted non-JSON: {e}\n--- stdout ---\n{stdout}"));
    (output.status.code().unwrap_or(-1), value)
}

/// Stage `path` (repo-relative) into the index.
fn stage(dir: &Path, rel: &str) {
    let repository = Repository::open(dir).unwrap();
    let mut index = repository.index().unwrap();
    index.add_path(Path::new(rel)).unwrap();
    index.write().unwrap();
}

/// Every spec file's contents under `root`, in sorted slug order.
fn read_specs(dir: &Path, specs_root: &str, specs: &[SpecFile]) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = specs
        .iter()
        .map(|spec| {
            let path = dir.join(specs_root).join(spec.slug).join("spec.md");
            (spec.slug.to_string(), fs::read_to_string(path).unwrap())
        })
        .collect();
    out.sort();
    out
}

/// Build the fixture, run the primitive, and assert the resulting spec files
/// match the golden byte for byte.
///
/// Returns `(exit code, payload)` so a caller can additionally assert on the
/// blocking contract and the cycle verdict.
fn assert_golden(
    name: &str,
    specs_root: &str,
    specs: &[SpecFile],
    config: Option<&str>,
) -> (i32, serde_json::Value) {
    let dir = tempfile::tempdir().unwrap();
    build_fixture(dir.path(), specs_root, specs, config);
    let (code, result) = run_primitive_with(dir.path(), &["--write"]);
    let produced = read_specs(dir.path(), specs_root, specs);
    common::maybe_bless(
        &repo_root(),
        "derive-generators",
        &format!("deps-{name}"),
        &produced,
    );
    common::compare_golden(
        &repo_root(),
        "derive-generators",
        &format!("deps-{name}"),
        &produced,
    );
    (code, result)
}

/// A spec whose `dependencies:` is deliberately wrong on disk, so both
/// implementations must rewrite it.
fn out_of_sync(slug: &'static str, body: &'static str) -> SpecFile {
    SpecFile { slug, body }
}

#[test]
fn relative_and_rooted_links_agree() {
    let specs = [
        out_of_sync(
            "001-a",
            "---\nstatus: done\ndependencies: [999-wrong]\n---\n\n\
             Links [b](../002-b/spec.md) and [c](specs/003-c/spec.md).\n",
        ),
        out_of_sync(
            "002-b",
            "---\nstatus: done\ndependencies: [999-wrong]\n---\n\nNo links.\n",
        ),
        out_of_sync(
            "003-c",
            "---\nstatus: done\ndependencies: []\n---\n\n[b](../002-b/spec.md)\n",
        ),
    ];
    let (code, result) = assert_golden("relative-and-rooted", "specs", &specs, None);
    assert_eq!(code, 0, "shell reported a cycle it should not have");
    assert_eq!(result["cycles"].as_array().unwrap().len(), 0);
    assert_eq!(result["drift"], true, "fixture was built out of sync");
}

#[test]
fn the_four_exclusions_agree() {
    // Each excluded context holds a link that must NOT become an edge, and
    // `## References` holds one that must. A harvester that gets any of the
    // five wrong diverges from the shell here.
    let specs = [out_of_sync(
        "001-a",
        "---\nstatus: done\ndependencies: [999-wrong]\n---\n\n\
         Real: [b](../002-b/spec.md)\n\n\
         ```\n[fenced](../003-c/spec.md)\n```\n\n\
         > [quoted](../004-d/spec.md)\n\n\
         ## See also\n\n\
         - [nav](../005-e/spec.md)\n\n\
         ### Nested under see also\n\n\
         - [also nav](../006-f/spec.md)\n\n\
         ## References\n\n\
         - [formal](../007-g/spec.md)\n",
    )];
    let (_, result) = assert_golden("four-exclusions", "specs", &specs, None);
    assert_eq!(result["drift"], true, "fixture was built out of sync");
}

#[test]
fn exclusions_derive_exactly_b_and_g() {
    // The parity assertion above proves the two agree; this proves what they
    // agree *on*. Both checks are needed — agreement alone would survive both
    // implementations being wrong in the same direction.
    let dir = tempfile::tempdir().unwrap();
    let specs = [out_of_sync(
        "001-a",
        "---\nstatus: done\ndependencies: [999-wrong]\n---\n\n\
         Real: [b](../002-b/spec.md)\n\n\
         ```\n[fenced](../003-c/spec.md)\n```\n\n\
         > [quoted](../004-d/spec.md)\n\n\
         ## See also\n\n\
         - [nav](../005-e/spec.md)\n\n\
         ### Nested under see also\n\n\
         - [also nav](../006-f/spec.md)\n\n\
         ## References\n\n\
         - [formal](../007-g/spec.md)\n",
    )];
    build_fixture(dir.path(), "specs", &specs, None);
    run_primitive(dir.path());
    let body = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();
    assert!(
        body.contains("dependencies: [002-b, 007-g]"),
        "unexpected derivation:\n{body}"
    );
}

#[test]
fn block_form_dependencies_agree() {
    // The two shell generators diverged on exactly this splice; the block form
    // must be replaced wholesale, not left with orphaned continuation lines.
    let specs = [out_of_sync(
        "001-a",
        "---\nstatus: done\ndependencies:\n  - 000-stale\n  - 111-also-stale\nnext-criterion: 4\n---\n\n\
         [b](../002-b/spec.md)\n",
    )];
    let (_, _) = assert_golden("block-form", "specs", &specs, None);
    // And confirm the following key survived on both sides.
    let dir = tempfile::tempdir().unwrap();
    build_fixture(dir.path(), "specs", &specs, None);
    run_primitive(dir.path());
    let body = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();
    assert!(body.contains("dependencies: [002-b]"), "{body}");
    assert!(!body.contains("000-stale"), "orphaned continuation: {body}");
    assert!(body.contains("next-criterion: 4"), "key consumed: {body}");
}

#[test]
fn empty_derivation_agrees() {
    let specs = [out_of_sync(
        "001-a",
        "---\nstatus: done\ndependencies: [999-wrong]\n---\n\nNo links at all.\n",
    )];
    assert_golden("empty", "specs", &specs, None);
}

#[test]
fn self_link_and_cycle_verdicts_agree() {
    let specs = [out_of_sync(
        "001-a",
        "---\nstatus: done\ndependencies: []\n---\n\n[self](../001-a/spec.md)\n",
    )];
    let (code, result) = assert_golden("self-link", "specs", &specs, None);
    assert_ne!(code, 0, "shell should exit non-zero on a self-cycle");
    assert_eq!(
        result["cycles"].as_array().unwrap().len(),
        1,
        "primitive missed the self-cycle: {result}"
    );
    assert_eq!(result["cycles"][0][0], "001-a");
}

#[test]
fn two_node_cycle_verdicts_agree() {
    let specs = [
        out_of_sync(
            "001-a",
            "---\nstatus: done\ndependencies: []\n---\n\n[b](../002-b/spec.md)\n",
        ),
        out_of_sync(
            "002-b",
            "---\nstatus: done\ndependencies: []\n---\n\n[a](../001-a/spec.md)\n",
        ),
    ];
    let (code, result) = assert_golden("two-node-cycle", "specs", &specs, None);
    assert_ne!(code, 0, "shell should exit non-zero on a cycle");
    let cycles = result["cycles"].as_array().unwrap();
    assert_eq!(cycles.len(), 1, "{result}");
    assert_eq!(cycles[0][0], "001-a");
    assert_eq!(cycles[0][1], "002-b");
}

#[test]
fn non_default_specs_root_agrees() {
    // Spec 040. The third of the three defects that reached adopters on
    // 2026-08-17 was a hardcoded `specs`, so this case is not hypothetical.
    let specs = [
        out_of_sync(
            "001-a",
            "---\nstatus: done\ndependencies: [999-wrong]\n---\n\n\
             [b](../002-b/spec.md) and [b again](governance/002-b/plan.md)\n",
        ),
        out_of_sync(
            "002-b",
            "---\nstatus: done\ndependencies: [999-wrong]\n---\n\nnothing\n",
        ),
    ];
    let config = "[paths]\nspecs-root = \"governance\"\n";
    let (code, result) = assert_golden("non-default-root", "governance", &specs, Some(config));
    assert_eq!(code, 0);
    assert_eq!(result["specs-root"], "governance");
    assert_eq!(result["examined"], 2, "wrong tree enumerated: {result}");
}

#[test]
fn duplicate_links_dedupe_identically() {
    let specs = [
        out_of_sync(
            "001-a",
            "---\nstatus: done\ndependencies: []\n---\n\n\
             [b](../002-b/spec.md) [b](../002-b/plan.md) [b](specs/002-b/tasks.md)\n",
        ),
        out_of_sync(
            "002-b",
            "---\nstatus: done\ndependencies: []\n---\n\nnothing\n",
        ),
    ];
    assert_golden("dedupe", "specs", &specs, None);
}

#[test]
fn an_in_sync_corpus_is_left_byte_identical_by_both() {
    // The idempotence direction: a second run must be a no-op on both sides.
    let specs = [
        out_of_sync(
            "001-a",
            "---\nstatus: done\ndependencies: [002-b]\n---\n\n[b](../002-b/spec.md)\n",
        ),
        out_of_sync(
            "002-b",
            "---\nstatus: done\ndependencies: []\n---\n\nnothing\n",
        ),
    ];
    let (code, result) = assert_golden("in-sync", "specs", &specs, None);
    assert_eq!(code, 0);
    assert_eq!(result["drift"], false, "no-op run reported drift: {result}");
    assert_eq!(result["updated"].as_array().unwrap().len(), 0);
}

/// Both specs start in sync and are then edited on disk so each needs a
/// rewrite; only `001-a` is staged. Under `--staged` the rewrite must land on
/// `001-a` alone — committing one spec never rewrites another.
fn build_staged_fixture(dir: &Path) -> [SpecFile; 2] {
    let specs = [
        SpecFile {
            slug: "001-a",
            body: "---\nstatus: done\ndependencies: []\n---\n\nnothing yet\n",
        },
        SpecFile {
            slug: "002-b",
            body: "---\nstatus: done\ndependencies: []\n---\n\nnothing yet\n",
        },
    ];
    build_fixture(dir, "specs", &specs, None);
    // Now edit both so both are out of sync, and stage only the first.
    write(
        &dir.join("specs/001-a/spec.md"),
        "---\nstatus: done\ndependencies: []\n---\n\n[b](../002-b/spec.md)\n",
    );
    write(
        &dir.join("specs/002-b/spec.md"),
        "---\nstatus: done\ndependencies: []\n---\n\n[a](../001-a/spec.md)\n",
    );
    stage(dir, "specs/001-a/spec.md");
    specs
}

#[test]
fn staged_scoping_agrees() {
    // The pre-commit path. The third of the three defects that reached
    // adopters on 2026-08-17 was a staging bug that left every rewrite
    // uncaptured, so this is the case with the worst blast radius.
    let rust_dir = tempfile::tempdir().unwrap();
    let specs = build_staged_fixture(rust_dir.path());

    let (_, result) = run_primitive_with(rust_dir.path(), &["--write", "--staged"]);
    let rust_specs = read_specs(rust_dir.path(), "specs", &specs);

    // The staged spec was rewritten...
    assert!(
        rust_specs[0].1.contains("dependencies: [002-b]"),
        "staged spec was not rewritten: {}",
        rust_specs[0].1
    );
    // ...and the unstaged one was left alone on disk, though it was examined
    // and found drifted.
    assert!(
        rust_specs[1].1.contains("dependencies: []"),
        "unstaged spec was rewritten: {}",
        rust_specs[1].1
    );
    assert_eq!(result["updated"].as_array().unwrap().len(), 1, "{result}");
    assert_eq!(
        result["unwritten"][0], "specs/002-b/spec.md",
        "a drifted-but-unstaged spec must be reported, not counted as in sync: {result}"
    );
}

#[test]
fn report_only_is_the_default_and_never_writes() {
    let dir = tempfile::tempdir().unwrap();
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: [999-wrong]\n---\n\n[b](../002-b/spec.md)\n",
    }];
    build_fixture(dir.path(), "specs", &specs, None);
    let before = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();

    let (code, result) = run_primitive_with(dir.path(), &[]);
    let after = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();

    assert_eq!(before, after, "a run without --write touched the tree");
    assert_ne!(
        code, 0,
        "a report-only run with drift must fail the CI check"
    );
    assert_eq!(
        result["drift"], true,
        "report-only run failed to surface drift"
    );
    assert_eq!(result["wrote"], false);
    assert_eq!(result["updated"].as_array().unwrap().len(), 1);
}

#[test]
fn untracked_specs_are_never_rewritten_and_are_reported() {
    // Spec 017 / tracked-specs-not-worktree: an untracked draft is excluded
    // by design. Reporting it is what keeps an empty `updated` from reading
    // as "everything is in sync".
    let dir = tempfile::tempdir().unwrap();
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\nnothing\n",
    }];
    build_fixture(dir.path(), "specs", &specs, None);
    let draft = "---\nstatus: draft\ndependencies: [999-wrong]\n---\n\n[a](../001-a/spec.md)\n";
    write(&dir.path().join("specs/002-draft/spec.md"), draft);

    let result = run_primitive(dir.path());

    assert_eq!(
        fs::read_to_string(dir.path().join("specs/002-draft/spec.md")).unwrap(),
        draft,
        "untracked draft was rewritten"
    );
    assert_eq!(result["examined"], 1, "untracked draft entered the walk");
    assert_eq!(
        result["untracked-skipped"][0], "specs/002-draft/spec.md",
        "untracked draft was not reported: {result}"
    );
}

#[test]
fn a_second_run_is_a_no_op_on_both_sides() {
    // Idempotence on the write path: run once to converge, then assert the
    // second run changes nothing and claims nothing.
    let dir = tempfile::tempdir().unwrap();
    let specs = [
        SpecFile {
            slug: "001-a",
            body: "---\nstatus: done\ndependencies: [999-wrong]\n---\n\n[b](../002-b/spec.md)\n",
        },
        SpecFile {
            slug: "002-b",
            body: "---\nstatus: done\ndependencies: [888-wrong]\n---\n\nnothing\n",
        },
    ];
    build_fixture(dir.path(), "specs", &specs, None);

    let first = run_primitive(dir.path());
    assert_eq!(first["drift"], true);
    let converged = read_specs(dir.path(), "specs", &specs);

    let second = run_primitive(dir.path());
    assert_eq!(
        second["drift"], false,
        "second run reported drift: {second}"
    );
    assert_eq!(read_specs(dir.path(), "specs", &specs), converged);
}
