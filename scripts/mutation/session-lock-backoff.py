"""Mutation list for spec 062 task 28: the session-lock poll backs off
exponentially with full jitter (`BE-RETRY-001`) and retries only a lock another
process holds (`BE-RETRY-003`), in runtime/src/session.rs.

Run with `python3 scripts/mutation/run.py scripts/mutation/session-lock-backoff.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing tests are `the_retry_bound_doubles_from_the_first_wait_to_the_cap`,
`a_wait_is_drawn_within_its_bound_and_varies` and
`the_lock_loop_waits_by_the_jittered_backoff` in runtime/src/session.rs.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound does not double',
        'src/session.rs',
        '        .saturating_mul(1_u32 << attempt.min(16))\n',
        '        .saturating_mul(1)\n',
        'session::',
    ),
    (
        'M2 the bound has no cap',
        'src/session.rs',
        '        .saturating_mul(1_u32 << attempt.min(16))\n        .min(LOCK_RETRY_CAP)\n',
        '        .saturating_mul(1_u32 << attempt.min(16))\n',
        'session::',
    ),
    (
        'M3 no jitter: every wait is its whole bound',
        'src/session.rs',
        '    Duration::from_nanos(seed % nanos.saturating_add(1))',
        '    Duration::from_nanos(nanos.min(seed | nanos))',
        'session::',
    ),
    (
        'M4 a wait can pass its bound',
        'src/session.rs',
        '    Duration::from_nanos(seed % nanos.saturating_add(1))',
        '    Duration::from_nanos(seed % nanos.saturating_mul(2).saturating_add(1))',
        'session::',
    ),
    (
        'M5 every seed is the same',
        'src/session.rs',
        '    std::collections::hash_map::RandomState::new().hash_one(0_u8)',
        '    let _ = std::collections::hash_map::RandomState::new().hash_one(0_u8);\n    7',
        'session::',
    ),
    (
        'M6 the loop waits a fixed interval',
        'src/session.rs',
        '                pause(wait.min(deadline.saturating_duration_since(Instant::now())));\n',
        '                let _ = wait;\n                pause(LOCK_RETRY_FIRST);\n',
        'session::',
    ),
    (
        'M7 the attempt count never advances',
        'src/session.rs',
        '                attempt = attempt.saturating_add(1);\n',
        '',
        'session::',
    ),
]
