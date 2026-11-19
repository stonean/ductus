"""Mutation list for spec 066's scenario `record-paths-are-quoted-only-when-they-need-it`:
`write-review` and `write-analysis` write each digest key and each unreadable
item through `yaml_string`, so a path is quoted exactly when its plain form
would not read back as itself, and an ordinary path is written byte for byte
as before (a bare path is what the analysis digest of `review.md` was taken
over). M5 holds the second half: quoting every key rewrites every stored record.

Run with `python3 scripts/mutation/run.py scripts/mutation/record-path-quoting-066.py
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
        'M1 a review digest key is written bare',
        'src/primitives/write_review.rs',
        '        for (path, digest) in &contracts.digests {\n            let _ = writeln!(fm, "  {}: {digest}", yaml_string(path));\n',
        '        for (path, digest) in &contracts.digests {\n            let _ = writeln!(fm, "  {path}: {digest}");\n',
        'write_review::tests::a_contract_named_with_a_comment_or_a_colon_round_trips',
    ),
    (
        'M2 a review unreadable item is written bare',
        'src/primitives/write_review.rs',
        '        let _ = writeln!(fm, "reviewed-unreadable:");\n        for path in &contracts.unreadable {\n            let _ = writeln!(fm, "  - {}", yaml_string(path));\n',
        '        let _ = writeln!(fm, "reviewed-unreadable:");\n        for path in &contracts.unreadable {\n            let _ = writeln!(fm, "  - {path}");\n',
        'write_review::tests::an_unreadable_contract_named_with_a_comment_round_trips',
    ),
    (
        'M3 an analysis digest key is written bare',
        'src/primitives/write_analysis.rs',
        '        for (path, digest) in &subjects.digests {\n            let _ = writeln!(out, "  {}: {digest}", yaml_string(path));\n',
        '        for (path, digest) in &subjects.digests {\n            let _ = writeln!(out, "  {path}: {digest}");\n',
        'write_analysis::tests::a_subject_named_with_a_colon_and_a_comment_reads_back_fresh',
    ),
    (
        'M4 an analysis unreadable item is written bare',
        'src/primitives/write_analysis.rs',
        '        let _ = writeln!(out, "analyzed-unreadable:");\n        for path in &subjects.unreadable {\n            let _ = writeln!(out, "  - {}", yaml_string(path));\n',
        '        let _ = writeln!(out, "analyzed-unreadable:");\n        for path in &subjects.unreadable {\n            let _ = writeln!(out, "  - {path}");\n',
        'write_analysis::tests::an_unreadable_subject_named_with_a_comment_round_trips',
    ),
    (
        'M5 a review digest key is always quoted',
        'src/primitives/write_review.rs',
        '        for (path, digest) in &contracts.digests {\n            let _ = writeln!(fm, "  {}: {digest}", yaml_string(path));\n',
        '        for (path, digest) in &contracts.digests {\n            let _ = writeln!(fm, "  {}: {digest}", yaml_quoted(path));\n',
        'write_review::tests::a_contract_named_with_a_comment_or_a_colon_round_trips',
    ),
]
