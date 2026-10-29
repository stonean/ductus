"""Mutation list for spec 048 task 19: every `fetch-archive` request carries a
User-Agent naming ductus, its version and the repository URL, `USER_AGENT` in
`runtime/src/primitives/fetch_archive.rs`.

Run with `python3 scripts/mutation/run.py scripts/mutation/fetch-archive-user-agent.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

Two judgments, one mutation each, both killed by
`every_request_names_ductus_its_version_and_the_repository`, which builds its
expected value from `CARGO_PKG_VERSION` and the URL rather than from the
constant:

  M1  the client sets the User-Agent: without it reqwest sends none.
  M2  the User-Agent names the repository URL, not some other contact.

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 User-Agent not set (reqwest sends none)',
        'src/primitives/fetch_archive.rs',
        '        .timeout(timeout)\n        .user_agent(USER_AGENT)\n',
        '        .timeout(timeout)\n',
        'every_request_names_ductus_its_version_and_the_repository',
    ),
    (
        'M2 User-Agent names another contact than the repository URL',
        'src/primitives/fetch_archive.rs',
        '    " (https://github.com/stonean/ductus)"\n);',
        '    " (https://example.invalid)"\n);',
        'every_request_names_ductus_its_version_and_the_repository',
    ),
]
