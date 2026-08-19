//! Shared helpers for integration test crates under `runtime/tests/`.
//!
//! Each `tests/*.rs` file compiles as its own integration-test binary,
//! so plain top-level helpers are not directly shareable. The
//! `tests/common/mod.rs` shape is the idiomatic Rust workaround: a
//! sub-module path that cargo does NOT auto-build as a test binary
//! (the `tests/foo.rs` shape would). Each integration test that needs
//! a helper does `mod common;` at the top of its file and imports the
//! symbols it uses.

#![allow(dead_code, clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

/// Recursively copy `src` into `dst`. Creates `dst` (and any missing
/// parents for nested files) as needed. Used to stage fixtures from
/// `runtime/tests/fixtures/<name>/` into a tempdir for write-side
/// integration tests.
pub fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to);
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&from, &to).unwrap();
        }
    }
}

/// Render a fixture's spec files into the golden-comparison text form.
fn render_specs(specs: &[(String, String)]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    for (slug, body) in specs {
        let _ = write!(out, "=== {slug} ===\n{body}");
    }
    out
}

/// Compare rendered fixture output against `runtime/tests/golden/<dir>/<file>.md`.
///
/// Shared by the two `derive-*` golden suites, which produce the same shape and
/// previously carried byte-identical copies of this helper.
pub fn compare_golden(repo_root: &Path, dir: &str, file: &str, specs: &[(String, String)]) {
    let path = repo_root
        .join("runtime/tests/golden")
        .join(dir)
        .join(format!("{file}.md"));
    let actual = render_specs(specs);
    let expected = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing golden {}; re-bless with BLESS=1", path.display()));
    assert_eq!(
        expected, actual,
        "golden mismatch for {file}\n--- expected ---\n{expected}\n--- actual ---\n{actual}"
    );
}

/// With `BLESS=1`, rewrite the golden from the current output. Deliberate
/// behavior changes only — see each suite's provenance note.
pub fn maybe_bless(repo_root: &Path, dir: &str, file: &str, specs: &[(String, String)]) {
    if std::env::var("BLESS").as_deref() != Ok("1") {
        return;
    }
    let out_dir = repo_root.join("runtime/tests/golden").join(dir);
    fs::create_dir_all(&out_dir).unwrap();
    fs::write(out_dir.join(format!("{file}.md")), render_specs(specs)).unwrap();
}
