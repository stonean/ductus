//! End-to-end golden tests for `derive-references`.
//!
//! Companion to `derive_dependencies_golden.rs`; the provenance note there
//! applies here too — every golden was blessed from a run of
//! `.ductus/scripts/gen-cross-service-refs.sh` over the same fixture and
//! written only after a byte-equality assertion against the primitive passed.
//!
//! This generator has more surface than the dependency one — a `[services]`
//! registry, two root-matching tiers, branch-ref stripping, repo
//! normalization, an inline-code exclusion the other does not have, and an
//! absent-when-empty write rule. Each is a place behavior could silently
//! shift, so each has a case.

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use git2::{IndexAddOption, Repository, Signature};

mod common;

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

/// Stand up a git repo with `specs`, an optional config, and optional
/// sibling checkouts (each `(relative path, its own specs-root)`).
fn build_fixture(
    dir: &Path,
    specs_root: &str,
    specs: &[SpecFile],
    config: Option<&str>,
    checkouts: &[(&str, &str)],
) {
    let repository = Repository::init(dir).unwrap();
    if let Some(config_body) = config {
        write(&dir.join(".ductus/config.toml"), config_body);
    }
    for (rel, root) in checkouts {
        write(
            &dir.join(rel).join(".ductus/config.toml"),
            &format!("[paths]\nspecs-root = \"{root}\"\n"),
        );
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

fn run_primitive(dir: &Path, flags: &[&str]) -> serde_json::Value {
    run_primitive_checked(dir, flags).1
}

/// Returns `(exit code, payload)`. The exit code carries the CLI's blocking
/// contract — non-zero on drift under `--dry-run`, which is the CI check.
fn run_primitive_checked(dir: &Path, flags: &[&str]) -> (i32, serde_json::Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_ductus"))
        .arg("derive-references")
        .args(flags)
        .current_dir(dir)
        .output()
        .expect("binary runs");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("primitive emitted non-JSON: {e}\n--- stdout ---\n{stdout}"));
    (output.status.code().unwrap_or(-1), value)
}

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

/// Build the fixture, run the primitive, and assert the output matches the
/// golden byte for byte.
fn assert_golden(
    name: &str,
    specs: &[SpecFile],
    config: Option<&str>,
    checkouts: &[(&str, &str)],
) -> (serde_json::Value, String) {
    let dir = tempfile::tempdir().unwrap();
    build_fixture(dir.path(), "specs", specs, config, checkouts);
    let result = run_primitive(dir.path(), &["--write"]);
    let produced = read_specs(dir.path(), "specs", specs);
    common::maybe_bless(
        &repo_root(),
        "derive-generators",
        &format!("refs-{name}"),
        &produced,
    );
    common::compare_golden(
        &repo_root(),
        "derive-generators",
        &format!("refs-{name}"),
        &produced,
    );
    (result, produced[0].1.clone())
}

const CONFIG_CHECKED_OUT: &str = "\
[services.api]
repo = \"https://github.com/acme/api\"
path = \"checkouts/api\"
";

const CONFIG_NOT_CHECKED_OUT: &str = "\
[services.api]
repo = \"https://github.com/acme/api\"
path = \"checkouts/absent\"
";

fn spec_with(body: &str) -> String {
    format!("---\nstatus: done\ndependencies: []\n---\n\n{body}\n")
}

#[test]
fn registered_service_with_matching_root_agrees() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [u](https://github.com/acme/api/specs/003-user/spec.md)\n",
    }];
    let (result, body) = assert_golden(
        "registered-matching-root",
        &specs,
        Some(CONFIG_CHECKED_OUT),
        &[("checkouts/api", "specs")],
    );
    assert_eq!(result["registered-services"], 1);
    assert!(body.contains("service: api"), "{body}");
    assert!(body.contains("spec: 003-user"), "{body}");
}

#[test]
fn reachable_checkout_rejects_the_wrong_root_segment() {
    // The referenced service renamed its root to `governance` (spec 040), so
    // a `specs/` URL for it is not a reference to that service.
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [wrong](https://github.com/acme/api/specs/003-user/spec.md)\n\
               [right](https://github.com/acme/api/governance/004-order/spec.md)\n",
    }];
    let (_, body) = assert_golden(
        "wrong-root-rejected",
        &specs,
        Some(CONFIG_CHECKED_OUT),
        &[("checkouts/api", "governance")],
    );
    assert!(body.contains("spec: 004-order"), "{body}");
    // Scoped to the harvested block: the rejected link's slug still appears
    // in the body prose, which is not evidence it became a reference.
    assert!(
        !body.contains("spec: 003-user"),
        "wrong-root link harvested: {body}"
    );
}

#[test]
fn unreachable_checkout_accepts_any_root_segment() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [a](https://github.com/acme/api/anything/003-user/spec.md)\n",
    }];
    let (_, body) = assert_golden(
        "unreachable-permissive",
        &specs,
        Some(CONFIG_NOT_CHECKED_OUT),
        &[],
    );
    assert!(body.contains("service: api"), "{body}");
    assert!(body.contains("spec: 003-user"), "{body}");
}

#[test]
fn unregistered_repo_records_a_null_service() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [x](https://github.com/other/thing/specs/003-user/spec.md)\n",
    }];
    let (_, body) = assert_golden("unregistered", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    assert!(body.contains("service: null"), "{body}");
    assert!(body.contains("spec: 003-user"), "{body}");
}

#[test]
fn branch_refs_collapse_to_one_reference() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [a](https://github.com/acme/api/blob/main/specs/003-user/spec.md)\n\
               [b](https://github.com/acme/api/tree/v2/specs/003-user/spec.md)\n\
               [c](https://github.com/acme/api/specs/003-user/spec.md)\n",
    }];
    let (_, body) = assert_golden(
        "branch-refs",
        &specs,
        Some(CONFIG_CHECKED_OUT),
        &[("checkouts/api", "specs")],
    );
    assert_eq!(body.matches("spec: 003-user").count(), 1, "{body}");
    assert!(body.contains("service: api"), "{body}");
}

#[test]
fn a_backticked_link_is_not_a_reference() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               Example: `[x](https://github.com/acme/api/specs/003-user/spec.md)`\n",
    }];
    let (_, body) = assert_golden(
        "backticked",
        &specs,
        Some(CONFIG_CHECKED_OUT),
        &[("checkouts/api", "specs")],
    );
    assert!(
        !body.contains("references:"),
        "backticked link harvested: {body}"
    );
}

#[test]
fn relative_sibling_links_are_never_references() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [b](../002-b/spec.md) and [c](specs/003-c/spec.md)\n",
    }];
    let (_, body) = assert_golden("relative", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    assert!(!body.contains("references:"), "{body}");
}

#[test]
fn the_shared_exclusions_apply() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               ```\n[f](https://github.com/acme/api/specs/003-a/spec.md)\n```\n\n\
               > [q](https://github.com/acme/api/specs/004-b/spec.md)\n\n\
               ## See also\n\n\
               [n](https://github.com/acme/api/specs/005-c/spec.md)\n",
    }];
    let (_, body) = assert_golden(
        "exclusions",
        &specs,
        Some(CONFIG_CHECKED_OUT),
        &[("checkouts/api", "specs")],
    );
    assert!(
        !body.contains("references:"),
        "an excluded link was harvested: {body}"
    );
}

#[test]
fn a_stale_block_is_removed_when_the_last_link_goes() {
    // Absent-when-empty. The spec starts carrying a block its body no longer
    // justifies; both implementations must delete it outright.
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\nreferences:\n  - service: api\n    spec: 003-user\nnext-criterion: 2\n---\n\nno links now\n",
    }];
    let (_, body) = assert_golden("stale-removed", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    assert!(
        !body.contains("references:"),
        "stale block survived: {body}"
    );
    assert!(body.contains("next-criterion: 2"), "key consumed: {body}");
}

#[test]
fn block_placement_without_a_dependencies_key_agrees() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\n---\n\n\
               [x](https://github.com/other/thing/specs/003-user/spec.md)\n",
    }];
    let (_, body) = assert_golden("no-deps-key", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    assert!(body.contains("references:"), "{body}");
    assert!(body.contains("spec: 003-user"), "{body}");
}

#[test]
fn repo_normalization_agrees_on_git_suffix_and_trailing_slash() {
    let config = "\
[services.api]
repo = \"https://github.com/acme/api.git\"
path = \"checkouts/absent\"
";
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [x](https://github.com/acme/api/specs/003-user/spec.md)\n",
    }];
    let (_, body) = assert_golden("normalization", &specs, Some(config), &[]);
    assert!(
        body.contains("service: api"),
        "a .git-suffixed repo failed to match: {body}"
    );
}

#[test]
fn ordering_and_dedup_agree() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [d](https://github.com/acme/api/specs/009-z/spec.md)\n\
               [b](https://github.com/other/x/specs/004-b/spec.md)\n\
               [a](https://github.com/acme/api/specs/003-a/spec.md)\n\
               [dup](https://github.com/acme/api/specs/003-a/spec.md)\n",
    }];
    let (_, body) = assert_golden("ordering", &specs, Some(CONFIG_NOT_CHECKED_OUT), &[]);
    let first = body.find("003-a").unwrap();
    let second = body.find("004-b").unwrap();
    let third = body.find("009-z").unwrap();
    assert!(first < second && second < third, "wrong order:\n{body}");
    assert_eq!(body.matches("spec: 003-a").count(), 1, "{body}");
}

#[test]
fn no_config_means_every_reference_is_null_and_the_registry_is_reported_empty() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [x](https://github.com/acme/api/specs/003-user/spec.md)\n",
    }];
    let (result, body) = assert_golden("no-config", &specs, None, &[]);
    assert_eq!(
        result["registered-services"], 0,
        "an absent config must be distinguishable from a populated one: {result}"
    );
    assert!(body.contains("service: null"), "{body}");
}

#[test]
fn spec_and_plan_files_are_matched() {
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [x](https://github.com/other/t/specs/003-user/spec-and-plan.md)\n",
    }];
    let (_, body) = assert_golden("spec-and-plan", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    assert!(body.contains("spec: 003-user"), "{body}");
}

#[test]
fn staged_scoping_agrees() {
    let specs = [
        SpecFile {
            slug: "001-a",
            body: "---\nstatus: done\ndependencies: []\n---\n\nnothing\n",
        },
        SpecFile {
            slug: "002-b",
            body: "---\nstatus: done\ndependencies: []\n---\n\nnothing\n",
        },
    ];
    let build = |dir: &Path| {
        build_fixture(dir, "specs", &specs, Some(CONFIG_CHECKED_OUT), &[]);
        for slug in ["001-a", "002-b"] {
            write(
                &dir.join("specs").join(slug).join("spec.md"),
                "---\nstatus: done\ndependencies: []\n---\n\n\
                 [x](https://github.com/other/t/specs/003-user/spec.md)\n",
            );
        }
        let repository = Repository::open(dir).unwrap();
        let mut index = repository.index().unwrap();
        index.add_path(Path::new("specs/001-a/spec.md")).unwrap();
        index.write().unwrap();
    };

    let rust_dir = tempfile::tempdir().unwrap();
    build(rust_dir.path());

    let result = run_primitive(rust_dir.path(), &["--write", "--staged"]);
    let rust_specs = read_specs(rust_dir.path(), "specs", &specs);
    assert!(
        rust_specs[0].1.contains("references:"),
        "staged spec not rewritten"
    );
    assert!(
        !rust_specs[1].1.contains("references:"),
        "unstaged spec was rewritten: {}",
        rust_specs[1].1
    );
    assert_eq!(result["examined"], 1, "{result}");
}

#[test]
fn report_only_is_the_default_and_never_writes() {
    let dir = tempfile::tempdir().unwrap();
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [x](https://github.com/other/t/specs/003-user/spec.md)\n",
    }];
    build_fixture(dir.path(), "specs", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    let before = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();

    let (code, result) = run_primitive_checked(dir.path(), &[]);
    assert_ne!(
        code, 0,
        "a report-only run with drift must fail the CI check"
    );

    assert_eq!(
        fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap(),
        before,
        "a run without --write touched the tree"
    );
    assert_eq!(result["drift"], true);
    assert_eq!(result["wrote"], false);
}

#[test]
fn a_second_run_is_a_no_op() {
    let dir = tempfile::tempdir().unwrap();
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [x](https://github.com/other/t/specs/003-user/spec.md)\n",
    }];
    build_fixture(dir.path(), "specs", &specs, Some(CONFIG_CHECKED_OUT), &[]);

    assert_eq!(run_primitive(dir.path(), &["--write"])["drift"], true);
    let converged = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();
    let second = run_primitive(dir.path(), &["--write"]);
    assert_eq!(
        second["drift"], false,
        "second run reported drift: {second}"
    );
    assert_eq!(
        fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap(),
        converged
    );
}

#[test]
fn dependencies_frontmatter_is_never_touched() {
    // The two indexes are strictly distinct (spec 030). This primitive must
    // leave `dependencies:` byte-identical even while rewriting around it.
    let dir = tempfile::tempdir().unwrap();
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: [002-b, 003-c]\n---\n\n\
               [x](https://github.com/other/t/specs/003-user/spec.md)\n",
    }];
    build_fixture(dir.path(), "specs", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    run_primitive(dir.path(), &["--write"]);
    let body = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();
    assert!(body.contains("dependencies: [002-b, 003-c]"), "{body}");
    let _ = spec_with("");
}

#[test]
fn an_unterminated_frontmatter_block_is_reported_not_silently_skipped() {
    // QUAL-CLAIM-001: the spec is reachable and still goes underived, so an
    // empty `updated` must not read as examined-and-clean.
    let dir = tempfile::tempdir().unwrap();
    let specs = [SpecFile {
        slug: "001-a",
        // Frontmatter opened, never closed, and no `dependencies:` key — the
        // splice has neither anchor.
        body: "---\nstatus: done\n\n[x](https://github.com/other/t/specs/003-user/spec.md)\n",
    }];
    build_fixture(dir.path(), "specs", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    let before = fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap();

    let result = run_primitive(dir.path(), &["--write"]);

    assert_eq!(
        fs::read_to_string(dir.path().join("specs/001-a/spec.md")).unwrap(),
        before,
        "an unparseable spec must not be rewritten"
    );
    assert_eq!(
        result["unparseable"][0], "specs/001-a/spec.md",
        "unparseable spec was silently skipped: {result}"
    );
    assert_eq!(result["updated"].as_array().unwrap().len(), 0, "{result}");
    assert_eq!(result["drift"], false);
}

#[test]
fn a_well_formed_corpus_reports_nothing_unparseable() {
    // The other half of the pair: empty `unparseable` has to mean something,
    // so it must be reachable on a clean corpus.
    let dir = tempfile::tempdir().unwrap();
    let specs = [SpecFile {
        slug: "001-a",
        body: "---\nstatus: done\ndependencies: []\n---\n\n\
               [x](https://github.com/other/t/specs/003-user/spec.md)\n",
    }];
    build_fixture(dir.path(), "specs", &specs, Some(CONFIG_CHECKED_OUT), &[]);
    let result = run_primitive(dir.path(), &["--write"]);
    assert_eq!(
        result["unparseable"].as_array().unwrap().len(),
        0,
        "{result}"
    );
    assert_eq!(result["drift"], true);
}
