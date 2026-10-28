"""Mutation list for spec 022's scenario installed-files-are-not-group-writable:
`extract-archive` masks each extracted file's mode to `rwxr-xr-x`, so no file
`apply-manifest` installs carries group or other write; and `apply-manifest`
re-applies the source's mode to an `unchanged` destination whose bits
differ, so a file installed group-writable before the mask loses it too.

Run with `python3 scripts/mutation/run.py scripts/mutation/installed-file-modes.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing tests are `tar_gz_drops_group_and_other_write` and
`zip_drops_group_and_other_write` in runtime/src/primitives/extract_archive.rs
for M1-M3, and in runtime/src/primitives/apply_manifest.rs
`an_unchanged_destination_takes_the_source_mode` (M4),
`an_unchanged_destination_whose_mode_matches_is_not_touched` (M5, which
compares ctime after a 50 ms wait, so its kill depends on the file system's
ctime clock being finer than that) and
`an_unchanged_destination_that_is_a_link_is_not_chmodded_through` (M6), and
`an_unchanged_destination_that_refuses_a_chmod_is_named_and_the_walk_goes_on`
(M7, M8; macOS only, since `chflags uchg` is the one way an unprivileged test
can make a chmod fail, so on another platform M7 and M8 survive).
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
    (
        'M4 an unchanged destination keeps a mode that differs from the source\'s',
        'src/primitives/apply_manifest.rs',
        '            if !reapply_source_mode(&source, &dest)? {\n                result.modes_kept.push(entry.dest.clone());\n            }\n',
        '            let _ = (&source, &dest);\n',
        'apply_manifest',
    ),
    (
        'M5 the source mode re-applied to an unchanged destination whose mode already matches',
        'src/primitives/apply_manifest.rs',
        '    if source_mode & 0o7777 == dest_meta.permissions().mode() & 0o7777 {\n        return Ok(true);\n    }\n',
        '',
        'apply_manifest',
    ),
    (
        'M6 an unchanged destination that is a link chmodded through',
        'src/primitives/apply_manifest.rs',
        '    if dest_meta.file_type().is_symlink() {\n        return Ok(true);\n    }\n',
        '',
        'apply_manifest',
    ),
    (
        'M7 a chmod the file system refuses stops the walk',
        'src/primitives/apply_manifest.rs',
        '        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => Ok(false),\n',
        '',
        'apply_manifest',
    ),
    (
        'M8 a refused chmod is not named in modes-kept',
        'src/primitives/apply_manifest.rs',
        '                result.modes_kept.push(entry.dest.clone());\n',
        '',
        'apply_manifest',
    ),
]
