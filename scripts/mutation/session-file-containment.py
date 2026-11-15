"""Mutation list for spec 022 task 34: every open of a session file goes
through canonical containment (`BE-INPUT-004`), in runtime/src/session.rs and the
other readers of the shared default (M12 `host.rs`, M14 `migrate_session_file.rs`). Each
mutation drops one `contained` call, which is one `validate_contained` of the
file or its directory against the project root. The cell table above the
tests' `outside` helper in session.rs names the test that kills each.

`DefaultRecord::store` has no guard of its own, and so no mutation: every
caller has refused the path before it, and a store replaces a link rather than
following it. M3 was removed with it. M13, the `ductus exec` seed in
runtime/src/main.rs, is not here: run.py runs `--lib` tests and its test is in
the binary; it was proved by hand with `cargo test --bin ductus`.

Run with `python3 scripts/mutation/run.py scripts/mutation/session-file-containment.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 ProcessRecord::load reads a session file linked outside the project',
        'src/session.rs',
        '    pub fn load(repo: &Path, path: &Path) -> Result<Self> {\n        contained(repo, path)?;\n',
        '    pub fn load(repo: &Path, path: &Path) -> Result<Self> {\n',
        'session::tests::',
    ),
    (
        'M2 ProcessRecord::store writes through a session file linked outside the project',
        'src/session.rs',
        '    pub fn store(&self, repo: &Path, path: &Path) -> Result<()> {\n        contained(repo, path)?;\n',
        '    pub fn store(&self, repo: &Path, path: &Path) -> Result<()> {\n',
        'session::tests::',
    ),
    (
        'M4 read_default_strict reads a default linked outside the project',
        'src/session.rs',
        '    let path = paths::session_path(repo);\n    contained(repo, &path)?;\n',
        '    let path = paths::session_path(repo);\n',
        'session::tests::',
    ),
    (
        'M5 write checks the caller\'s own file only after it has written the default',
        'src/session.rs',
        '        contained(repo, &own)?;\n        if own.exists() {',
        '        if own.exists() {',
        'session::tests::',
    ),
    (
        'M6 the lock creates and fills a sessions directory linked outside the project',
        'src/session.rs',
        '    contained(repo, dir)?;\n    std::fs::create_dir_all(dir)',
        '    std::fs::create_dir_all(dir)',
        'session::tests::',
    ),
    (
        'M7 the lock opens a lock file linked outside the project',
        'src/session.rs',
        '    contained(repo, path)?;\n    let io = ',
        '    let io = ',
        'session::tests::',
    ),
    (
        'M8 session_files lists a sessions directory linked outside the project',
        'src/session.rs',
        '    contained(repo, &dir)?;\n    let entries = match',
        '    let entries = match',
        'session::tests::',
    ),
    (
        'M9 retarget does not refuse a shared default linked outside the project',
        'src/session.rs',
        '    contained(repo, &default_path)?;\n    if let Some(identity) = identity {',
        '    if let Some(identity) = identity {',
        'session::tests::',
    ),
    (
        'M10 retarget does not refuse the caller\'s own file linked outside the project',
        'src/session.rs',
        '    if let Some(identity) = identity {\n        contained(repo, &own_path(repo, identity))?;\n    }\n',
        '',
        'session::tests::',
    ),
    (
        'M11 write reads and replaces a shared default linked outside the project',
        'src/session.rs',
        '    contained(repo, path)?;\n    Ok(match std::fs::read_to_string(path) {',
        '    Ok(match std::fs::read_to_string(path) {',
        'session::tests::',
    ),
    (
        'M12 Host::load reads a session file linked outside the project',
        'src/host.rs',
        '        crate::session::contained(repo, &session_path)?;\n',
        '',
        'host::tests::a_session_file_linked_outside',
    ),
    (
        'M14 migrate-session-file writes through a directory linked outside the project',
        'src/primitives/migrate_session_file.rs',
        '    crate::session::contained(repo, &dest_path)?;\n',
        '',
        'migrate_session_file::tests::a_destination_under',
    ),
]
