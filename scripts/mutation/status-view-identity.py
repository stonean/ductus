"""Mutation list for spec 062's AC8: the pipeline view `render-status` renders
names the target of the process that runs it, labelled with its session,
rather than the shared default's (`runtime/src/primitives/render_status.rs`).
Found by 062's review (2026-11-10): no test failed when `run` passed no
identity.

Run with `python3 scripts/mutation/run.py scripts/mutation/status-view-identity.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The same property through the built binary is
held by `a_write_in_one_process_never_moves_the_other` in
`runtime/tests/concurrent_sessions.rs`, which fails under M1, run by hand with
`cargo test --locked --test concurrent_sessions`.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the view is rendered for no process',
        'src/primitives/render_status.rs',
        '    let (model, project) = dashboard::view_model(repo, identity.as_ref(), SystemTime::now())?;\n',
        '    let (model, project) = dashboard::view_model(repo, None, SystemTime::now())?;\n',
        'render_status::tests::the_view_names_this_process_target_and_session',
    ),
]
