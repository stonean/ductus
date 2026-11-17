//! Test support: drive a primitive's write to fail by making the directory it
//! writes into unwritable, so a test asserts what an interrupted write leaves
//! behind (`QUAL-TEST-001`: the test calls the primitive it names).

#![allow(clippy::unwrap_used)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// The directory's entry names, sorted.
pub(crate) fn entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

struct Restore(PathBuf);

impl Drop for Restore {
    fn drop(&mut self) {
        let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755));
    }
}

/// Run `body` with `dir` made unwritable (mode `0555`), restoring `0755`
/// afterwards, even when `body` panics. Panics instead of running `body` when
/// the permission does not bind (running as root), since a test that cannot
/// make its subject must fail, never pass unrun.
pub(crate) fn while_unwritable<R>(dir: &Path, body: impl FnOnce() -> R) -> R {
    let _restore = Restore(dir.to_path_buf());
    fs::set_permissions(dir, fs::Permissions::from_mode(0o555)).unwrap();
    assert!(
        fs::write(dir.join(".probe"), b"x").is_err(),
        "permissions do not bind (running as root?): no directory can be made unwritable"
    );
    body()
}
