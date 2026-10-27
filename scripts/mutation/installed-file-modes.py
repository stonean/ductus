"""Mutation list for spec 022's scenario installed-files-are-not-group-writable:
`extract-archive` masks each extracted file's mode to `rwxr-xr-x`, so no file
`apply-manifest` installs carries group or other write.

Run with `python3 scripts/mutation/run.py scripts/mutation/installed-file-modes.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing tests are `tar_gz_drops_group_and_other_write` and
`zip_drops_group_and_other_write` in runtime/src/primitives/extract_archive.rs.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the mask keeps group and other write',
        'src/primitives/extract_archive.rs',
        'const PERMISSION_MASK: u32 = 0o755;',
        'const PERMISSION_MASK: u32 = 0o777;',
        'extract_archive',
    ),
    (
        'M2 the mask keeps group write',
        'src/primitives/extract_archive.rs',
        'const PERMISSION_MASK: u32 = 0o755;',
        'const PERMISSION_MASK: u32 = 0o775;',
        'extract_archive',
    ),
    (
        'M3 the mask drops the executable bit',
        'src/primitives/extract_archive.rs',
        'const PERMISSION_MASK: u32 = 0o755;',
        'const PERMISSION_MASK: u32 = 0o644;',
        'extract_archive',
    ),
]
