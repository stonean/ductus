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
//! This test runs both over the real corpus and fails on the first spec where
//! they differ. It asserts agreement, not a particular verdict, so it stays
//! valid as the corpus changes.

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
/// ones (no `review:` block) and any whose sha does not resolve — the two
/// cases both implementations decline to judge.
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
        let Some(base) = fm
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
        .args(["-C", root.to_str().unwrap(), "diff", "--name-only"])
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
fn family_19_verdict(root: &Path, base: &str, paths: &BTreeSet<String>) -> BTreeSet<String> {
    if paths.is_empty() {
        return BTreeSet::new();
    }
    let script = std::fs::read_to_string(root.join("scripts/audit/review-freshness.sh"))
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
         import json, sys as _s\n\
         _base = _s.argv[3]\n\
         _paths = json.loads(_s.argv[4])\n\
         print(json.dumps([p for p in _paths if changed_beyond_spelling(_base, p)]))\n"
    );
    let out = Command::new("python3")
        .arg("-")
        .arg(root.to_str().unwrap())
        .arg("specs")
        .arg(base)
        .arg(serde_json::to_string(&paths.iter().collect::<Vec<_>>()).unwrap())
        .current_dir(root)
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
        "no reviewed done specs found — the comparison would pass vacuously, \
         which is exactly the shape this repo treats as a defect"
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
        let python = family_19_verdict(&root, base, &candidates);
        assert_eq!(
            rust, python,
            "staleness verdicts disagree for {slug} (reviewed-against {base}); \
             the transition gate and the release gate must answer the same question \
             the same way"
        );
        compared += 1;
    }
    assert!(
        compared > 0,
        "every done spec's contracts were unchanged, so nothing was actually \
         compared — a green result here would mean the check could not run"
    );
    eprintln!(
        "compared {compared} spec(s) with changed contracts across {} done spec(s)",
        specs.len()
    );
}
